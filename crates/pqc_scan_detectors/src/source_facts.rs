//! Local source facts for the explicitly supported built-in reference rules.
//! Unrelated/custom patterns retain the generic matching path.
mod bindings;
mod context;
mod javascript;
mod python;
mod rust;
#[cfg(test)]
mod selectors_tests;
use bindings::{handler, Handler};
use context::Context;
use pqc_scan_core::{DetectorStatus, ScannableFile};
use pqc_scan_rules::{Rule, RuleSet};
use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Clone, Debug)]
pub(crate) struct SourceMatch {
    pub start: usize,
    pub end: usize,
    pub reason: &'static str,
}
#[derive(Default)]
pub(crate) struct Facts {
    hits: HashMap<&'static str, Vec<SourceMatch>>,
    pub truncated: bool,
    pub partial: bool,
    remaining_candidates: Option<usize>,
    remaining_bytes: Option<usize>,
}
impl Facts {
    fn push(&mut self, id: &'static str, start: usize, end: usize, reason: &'static str) {
        if let Some(remaining) = &mut self.remaining_bytes {
            let bytes = end.saturating_sub(start);
            if bytes > *remaining {
                self.truncated = true;
                return;
            }
            *remaining -= bytes;
        }
        if let Some(remaining) = &mut self.remaining_candidates {
            if *remaining == 0 {
                self.truncated = true;
                return;
            }
            *remaining -= 1;
        }
        let hits = self.hits.entry(id).or_default();
        if start < end {
            hits.push(SourceMatch { start, end, reason });
        }
    }
    fn node(&mut self, id: &'static str, node: Node<'_>, reason: &'static str) {
        self.push(id, node.start_byte(), node.end_byte(), reason);
    }
    fn callee(&mut self, id: &'static str, node: Node<'_>, reason: &'static str) {
        if let Some(function) = node.child_by_field_name("function") {
            self.push(id, node.start_byte(), function.end_byte(), reason);
        }
    }
    fn call(&mut self, id: &'static str, node: Node<'_>, reason: &'static str) {
        let Some(function) = node.child_by_field_name("function") else {
            return;
        };
        let end = node
            .child_by_field_name("arguments")
            .and_then(|args| context::children(args).first().copied())
            .map_or(function.end_byte(), |arg| arg.end_byte());
        self.push(id, node.start_byte(), end, reason);
    }
    pub fn rule_matches(&self, rule: &Rule) -> &[SourceMatch] {
        if !builtin(rule) {
            return &[];
        }
        self.hits.get(rule.id.as_str()).map_or(&[], Vec::as_slice)
    }
    pub fn replaces(&self, rule: &Rule) -> bool {
        handler(rule).is_some_and(Handler::replaces_generic)
    }

    pub fn status(&self) -> DetectorStatus {
        if self.truncated {
            DetectorStatus::Truncated
        } else if self.partial {
            DetectorStatus::Partial
        } else {
            DetectorStatus::Complete
        }
    }
}

pub(crate) fn builtin(rule: &Rule) -> bool {
    handler(rule).is_some()
}

// Require the complete canonical request prefix: an embedded quote or an
// algorithm-name prefix must not turn an unrelated value into a selected API.
fn exact_match(pattern: &regex::Regex, target: &str) -> bool {
    pattern
        .find(target)
        .is_some_and(|m| m.start() == 0 && m.end() == target.len())
}
pub(crate) fn inspect(
    root: Node<'_>,
    text: &str,
    language: &str,
    rules: &RuleSet,
    budget: crate::DetectorBudget,
) -> Facts {
    let mut facts = Facts {
        partial: root.has_error(),
        remaining_candidates: Some(budget.max_ast_candidates),
        remaining_bytes: Some(budget.max_ast_match_bytes),
        ..Facts::default()
    };
    if !Handler::ALL
        .into_iter()
        .filter(|h| h.handles(language))
        .any(|h| rules.get(h.id()).is_some_and(builtin))
    {
        return facts;
    }
    let mut context = Context::new(root, text, language, budget);
    if context.truncated.get() {
        facts.truncated = true;
        return facts;
    }
    match language {
        "javascript" | "typescript" => javascript::inspect(&mut context, &mut facts, rules),
        "python" => python::inspect(&mut context, &mut facts, rules),
        "rust" => rust::inspect(&mut context, &mut facts),
        "java" => {
            if let Some(rule) = rules.get("PQC_JAVA_API").filter(|r| builtin(r)) {
                for node in &context.nodes {
                    if !matches!(node.kind(), "method_invocation" | "ERROR") {
                        continue;
                    }
                    let raw = context.bounded_source(*node);
                    for (examined, hit) in
                        rule.compiled_pattern().unwrap().find_iter(raw).enumerate()
                    {
                        if examined >= budget.max_match_candidates {
                            facts.truncated = true;
                            break;
                        }
                        let start = node.start_byte() + hit.start();
                        let embedded = text[..start]
                            .chars()
                            .next_back()
                            .is_some_and(|c| c.is_alphanumeric() || c == '_');
                        if context.code_at(start) && !embedded {
                            facts.push(
                                "PQC_JAVA_API",
                                start,
                                node.start_byte() + hit.end(),
                                if node.kind() == "ERROR" {
                                    "bounded_syntax_recovery"
                                } else {
                                    "explicit_api_request"
                                },
                            );
                        }
                    }
                }
            }
        }
        _ => {}
    }
    facts.truncated |= context.truncated.get();
    for (id, matches) in &mut facts.hits {
        if let Some(pattern) = rules
            .get(id)
            .filter(|r| builtin(r))
            .and_then(|r| r.compiled_pattern())
        {
            for hit in matches.iter_mut() {
                if let Some(original) = pattern
                    .find(&text[hit.start..hit.end])
                    .filter(|m| m.start() == 0)
                {
                    hit.end = hit.start + original.end();
                }
            }
        }
        matches.sort_by_key(|m| (m.start, m.end));
        matches.dedup_by_key(|m| m.start);
    }
    facts
}
pub(crate) fn for_regex(
    file: &ScannableFile,
    rules: &RuleSet,
    budget: crate::DetectorBudget,
) -> Option<Facts> {
    if !rules.get("API_NODE_CRYPTO_SIGN_RSA").is_some_and(builtin) {
        return None;
    }
    let (language, name) =
        crate::tree_sitter_detector::resolve_language(&file.path, file.text.as_deref()?)?;
    if !matches!(name, "javascript" | "typescript") {
        return None;
    }
    let text = file.text.as_deref()?;
    let Ok(parsed) =
        crate::tree_sitter_detector::source_tree(text, &file.path, language, name, budget)
    else {
        return Some(Facts {
            partial: true,
            ..Facts::default()
        });
    };
    let mut facts = inspect(parsed.tree.root_node(), text, name, rules, budget);
    facts.truncated |= parsed.compatibility_limited || parsed.template_limited;
    facts.partial |= parsed.template_pending;
    Some(facts)
}
