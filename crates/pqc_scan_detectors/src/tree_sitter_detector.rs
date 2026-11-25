mod display_references;
mod header;
mod javascript_validation;
mod module_syntax;
mod positions;
mod source_api;
mod syntax_compat;
mod test_scope;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use anyhow::Result;
use tree_sitter::{Language, Parser};

use crate::comment_filter::should_skip_comment_only_match_for_language;
use pqc_scan_core::model::mask_preview;
use pqc_scan_core::{
    Detection, Detector, DetectorOutcome, DetectorStatus, Evidence, Location, ScannableFile,
};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug, Default)]
pub struct TreeSitterDetector;

#[derive(Debug, Clone)]
struct CandidateNode<'a> {
    kind: &'static str,
    text: &'a str,
    line: usize,
    column: usize,
    byte_start: usize,
    canonical: Option<String>,
}

impl Detector for TreeSitterDetector {
    fn name(&self) -> &'static str {
        "tree_sitter_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(file, rules)?.into_detections())
    }

    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.detect_internal(file, rules, false, crate::DetectorBudget::default())
    }
    fn detect_in_repository(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        repository: Option<&pqc_scan_core::RepositoryContext>,
        observe: bool,
    ) -> Result<DetectorOutcome> {
        let budget = crate::DetectorBudget::default();
        self.detect_source(file, rules, observe, budget, repository)
            .map(|outcome| {
                outcome
                    .with_display_ranges(display_references::ranges(file, budget.max_ast_nodes))
                    .with_sensitive_source_context(
                        file.text
                            .as_deref()
                            .and_then(|text| resolve_language(&file.path, text))
                            .is_some_and(|(_, name)| {
                                matches!(name, "php" | "csharp" | "c" | "cpp")
                                    && rules.all().iter().any(|r| {
                                        scope_matches(&r.scope, name)
                                            && crate::source_facts::builtin(r)
                                    })
                            }),
                    )
            })
    }
    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.detect_internal(file, rules, true, crate::DetectorBudget::default())
    }
}
impl TreeSitterDetector {
    fn detect_internal(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        observe: bool,
        budget: crate::DetectorBudget,
    ) -> Result<DetectorOutcome> {
        self.detect_source(file, rules, observe, budget, None)
            .map(|outcome| {
                outcome
                    .with_display_ranges(display_references::ranges(file, budget.max_ast_nodes))
                    .with_sensitive_source_context(
                        file.text
                            .as_deref()
                            .and_then(|text| resolve_language(&file.path, text))
                            .is_some_and(|(_, name)| {
                                matches!(name, "php" | "csharp" | "c" | "cpp")
                                    && rules.all().iter().any(|r| {
                                        scope_matches(&r.scope, name)
                                            && crate::source_facts::builtin(r)
                                    })
                            }),
                    )
            })
    }
    fn detect_source(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        observe: bool,
        budget: crate::DetectorBudget,
        repository: Option<&pqc_scan_core::RepositoryContext>,
    ) -> Result<DetectorOutcome> {
        if file.text.is_none()
            && !crate::source_kind::is_dockerfile(&file.path, "")
            && file.ext().is_some_and(|e| {
                matches!(
                    e.to_ascii_lowercase().as_str(),
                    "php"
                        | "phtml"
                        | "cs"
                        | "c"
                        | "cc"
                        | "cpp"
                        | "cxx"
                        | "h"
                        | "hpp"
                        | "hh"
                        | "hxx"
                )
            })
        {
            return Ok(DetectorOutcome::new(Vec::new(), DetectorStatus::Partial)
                .with_explanation("Source text could not be decoded; syntax and cryptographic references remain uninspected."));
        }
        if file.path.extension().is_some_and(|e| e == "h") {
            return header::inspect(self, file, rules, observe, budget, repository);
        }
        self.detect_candidate(file, rules, observe, budget, repository, None)
    }
    fn detect_candidate(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        observe: bool,
        budget: crate::DetectorBudget,
        repository: Option<&pqc_scan_core::RepositoryContext>,
        selected: Option<(Language, &'static str)>,
    ) -> Result<DetectorOutcome> {
        let text = match file.text.as_ref() {
            Some(v) => v,
            None => {
                return Ok(DetectorOutcome::new(
                    Vec::new(),
                    DetectorStatus::NotApplicable,
                ))
            }
        };

        let (language, language_name) =
            match selected.or_else(|| resolve_language(&file.path, text)) {
                Some(v) => v,
                None => {
                    return Ok(DetectorOutcome::new(
                        Vec::new(),
                        DetectorStatus::NotApplicable,
                    ))
                }
            };

        if !rules
            .by_kind(RuleKind::TreeSitter)
            .any(|rule| scope_matches(&rule.scope, language_name))
        {
            return Ok(DetectorOutcome::new(
                Vec::new(),
                DetectorStatus::NotApplicable,
            ));
        }

        let ParsedSource {
            tree,
            original_tree,
            compatibility_limited,
            template_pending,
            template_limited,
        } = source_tree(text, &file.path, language, language_name, budget)?;
        let rsa_constructors = language_name == "java"
            && rules
                .get("TS_RSA_KEY_SIZE_LITERAL")
                .is_some_and(|r| r.kind == RuleKind::TreeSitter && scope_matches(&r.scope, "java"));
        let positions = is_new_language(language_name)
            .then(|| positions::Positions::new(text, budget.max_ast_nodes));
        let (mut candidates, mut status, mut constructors) = collect_candidate_nodes(
            text,
            &tree,
            budget,
            rsa_constructors,
            language_name,
            file.path.extension().is_some_and(|e| e == "h") && language_name == "cpp",
            positions.as_ref(),
        );
        if compatibility_limited {
            status = DetectorStatus::Truncated;
        }
        if tree.root_node().id() != original_tree.root_node().id() {
            let (original, original_status, extra_constructors) = collect_candidate_nodes(
                text,
                &original_tree,
                budget,
                rsa_constructors,
                language_name,
                file.path.extension().is_some_and(|e| e == "h") && language_name == "cpp",
                positions.as_ref(),
            );
            for candidate in extra_constructors {
                if !constructors.iter().any(|old| {
                    old.byte_start == candidate.byte_start && old.text.len() == candidate.text.len()
                }) {
                    if constructors.len() == budget.max_ast_candidates {
                        status = DetectorStatus::Truncated;
                        break;
                    }
                    constructors.push(candidate);
                }
            }
            if original_status == DetectorStatus::Truncated {
                status = DetectorStatus::Truncated;
            }
            let mut seen: HashSet<_> = candidates
                .iter()
                .map(|n| (n.byte_start, n.text.len(), n.kind))
                .collect();
            for candidate in original {
                if seen.insert((candidate.byte_start, candidate.text.len(), candidate.kind)) {
                    if candidates.len() >= budget.max_ast_candidates {
                        status = DetectorStatus::Truncated;
                        break;
                    }
                    candidates.push(candidate);
                }
            }
        }
        if positions.as_ref().is_some_and(|p| !p.complete) {
            status = DetectorStatus::Truncated;
        }
        let (resolved_calls, binding_limited) = source_api::aliases(
            text,
            &tree,
            language_name,
            budget.max_ast_nodes,
            budget.max_ast_candidates,
        );
        if binding_limited {
            status = DetectorStatus::Truncated;
        }
        for call in resolved_calls {
            if candidates.len() >= budget.max_ast_candidates {
                status = DetectorStatus::Truncated;
                break;
            }
            let Some((line, column)) = source_position(text, call.start, positions.as_ref()) else {
                status = DetectorStatus::Truncated;
                continue;
            };
            candidates.push(CandidateNode {
                kind: "resolved_call",
                text: &text[call.start..call.end],
                line,
                column,
                byte_start: call.start,
                canonical: Some(call.canonical),
            });
        }
        if language_name == "javascript" {
            let (invalid, limited) = javascript_validation::strict_yield_binding(
                text,
                &original_tree,
                file.ext().is_some_and(|e| e.eq_ignore_ascii_case("mjs")),
                budget.max_ast_nodes,
            );
            if limited {
                status = DetectorStatus::Truncated;
            } else if invalid && status != DetectorStatus::Truncated {
                status = DetectorStatus::Partial;
            }
        }
        if tree.root_node().has_error() && status != DetectorStatus::Truncated {
            status = DetectorStatus::Partial;
        }
        let facts = crate::source_facts::inspect_in_repository(
            tree.root_node(),
            text,
            language_name,
            rules,
            budget,
            Some(&file.path),
            repository,
        );
        if facts.truncated {
            status = DetectorStatus::Truncated;
        } else if is_new_language(language_name)
            && facts.partial
            && status == DetectorStatus::Complete
        {
            status = DetectorStatus::Partial;
        }

        let mut observations = Vec::new();
        let mut out = Vec::new();
        let file_path = file.path.to_string_lossy().into_owned();

        let mut remaining_match_bytes = budget.max_ast_match_bytes;
        // Preserve the complete legacy candidate/alias/rule pass before spending
        // residual budgets on the previously omitted Java RSA constructor arm.
        let remaining_candidates = budget.max_ast_candidates.saturating_sub(candidates.len());
        if constructors.len() > remaining_candidates {
            status = DetectorStatus::Truncated;
        }
        constructors.truncate(remaining_candidates);
        let mut rsa_dedup = HashSet::new();
        let mut rsa_examined = 0;
        for constructor_pass in [false, true] {
            let pass_candidates = if constructor_pass {
                &constructors
            } else {
                &candidates
            };
            for rule in rules
                .by_kind(RuleKind::TreeSitter)
                .filter(|r| !constructor_pass || r.id == "TS_RSA_KEY_SIZE_LITERAL")
            {
                if !scope_matches(&rule.scope, language_name) {
                    continue;
                }

                let regex = match rule.compiled_pattern() {
                    Some(v) => v,
                    None => {
                        if status != DetectorStatus::Truncated {
                            status = DetectorStatus::Partial;
                        }
                        continue;
                    }
                };

                let mut dedup = if constructor_pass {
                    rsa_dedup.clone()
                } else {
                    HashSet::new()
                };
                let mut rule_hits = if constructor_pass {
                    out.iter()
                        .filter(|d: &&Detection| d.rule_id == rule.id)
                        .count()
                } else {
                    0
                };
                let mut examined = if constructor_pass { rsa_examined } else { 0 };
                'candidates: for candidate in pass_candidates {
                    // Java constructors are included for the existing RSA parameter
                    // rule only; other rules retain their original candidate scope.
                    if candidate.kind == "object_creation_expression"
                        && language_name == "java"
                        && rule.id != "TS_RSA_KEY_SIZE_LITERAL"
                    {
                        continue;
                    }
                    if facts.replaces(rule) {
                        break;
                    }
                    if language_name == "python"
                        && matches!(
                            rule.id.as_str(),
                            "TS_PY_RSA_USAGE" | "TS_CLASSICAL_EC_API" | "TS_CLASSICAL_EDDSA_API"
                        )
                        && candidate.kind != "resolved_call"
                    {
                        continue;
                    }
                    let match_text = candidate.canonical.as_deref().unwrap_or(candidate.text);
                    if match_text.len() > remaining_match_bytes {
                        status = DetectorStatus::Truncated;
                        break;
                    }
                    remaining_match_bytes -= match_text.len();
                    for hit in regex.find_iter(match_text) {
                        if examined >= budget.max_match_candidates {
                            status = DetectorStatus::Truncated;
                            break 'candidates;
                        }
                        examined += 1;
                        if should_skip_comment_only_match_for_language(
                            language_name,
                            match_text,
                            hit.start(),
                        ) {
                            continue;
                        }
                        if is_embedded_identifier_submatch(match_text, hit.start(), hit.end()) {
                            continue;
                        }

                        let (relative_line, relative_col) = if is_new_language(language_name) {
                            (1, 1)
                        } else {
                            pqc_scan_core::model::line_col_for_offset(
                                candidate.text,
                                if candidate.canonical.is_some() {
                                    0
                                } else {
                                    hit.start()
                                },
                            )
                        };
                        let line = candidate.line + relative_line.saturating_sub(1);
                        let column = if relative_line == 1 {
                            candidate.column + relative_col.saturating_sub(1)
                        } else {
                            relative_col
                        };

                        // Parent/child candidates can expose the same token with
                        // different quote wrappers. Normalize only its boundaries;
                        // distinct occurrences must never share a line/text key.
                        let raw = hit.as_str();
                        let leading = raw.len() - raw.trim_start_matches(is_match_boundary).len();
                        let token = raw.trim_matches(is_match_boundary);
                        let source_start = candidate.byte_start
                            + if candidate.canonical.is_some() {
                                0
                            } else {
                                hit.start() + leading
                            };
                        let source_end = if candidate.canonical.is_some() {
                            candidate.byte_start + candidate.text.len()
                        } else {
                            source_start + token.len()
                        };
                        let Some((line, column)) = (if positions.is_some() {
                            source_position(text, source_start, positions.as_ref())
                        } else {
                            Some((line, column))
                        }) else {
                            status = DetectorStatus::Truncated;
                            continue;
                        };
                        let dedup_key = (source_start, source_end);
                        if dedup.contains(&dedup_key) {
                            continue;
                        }
                        if rule_hits == budget.max_hits_per_rule {
                            status = DetectorStatus::Truncated;
                            break 'candidates;
                        }
                        dedup.insert(dedup_key);

                        let mut metadata = BTreeMap::new();
                        metadata.insert("detector".to_string(), self.name().to_string());
                        metadata.insert("language".to_string(), language_name.to_string());
                        metadata.insert("node_kind".to_string(), candidate.kind.to_string());
                        if rule.id == "PQC_STANDARD_REFERENCE" {
                            let kind = if candidate.kind.contains("string") {
                                "name_literal"
                            } else if candidate.kind.contains("call") {
                                "call_reference"
                            } else {
                                "symbol_reference"
                            };
                            metadata.insert("reference_kind".into(), kind.into());
                        }
                        if let Some(fact) = facts
                            .rule_matches(rule)
                            .iter()
                            .find(|f| f.start == source_start && f.end == source_end)
                        {
                            metadata.insert("source_resolution".into(), fact.reason.into());
                        }
                        metadata.insert("scope".to_string(), rule.scope.clone());
                        if candidate.canonical.is_some() {
                            metadata
                                .insert("resolution_method".into(), "static_import_binding".into());
                        }

                        out.push(Detection {
                            rule_id: rule.id.clone(),
                            location: Location {
                                file: file_path.clone(),
                                line,
                                column,
                            },
                            evidence: Evidence {
                                r#type: "tree_sitter_match".to_string(),
                                r#match: mask_preview(if candidate.canonical.is_some() {
                                    candidate.text
                                } else {
                                    hit.as_str()
                                }),
                                snippet_preview: mask_preview(
                                    &clip_text(candidate.text.trim(), 240).0,
                                ),
                                metadata,
                            },
                        });
                        if observe {
                            let draft = pqc_scan_core::ObservationDraft::source(
                                out.last().expect("accepted detection"),
                                pqc_scan_core::ObservationMethod::AstPattern,
                                hit.as_str(),
                                Some((source_start, source_end)),
                            )
                            .with_algorithm_references(
                                facts.references_at(rule, source_start, source_end),
                            );
                            observations.push(if is_new_language(language_name) {
                                draft.with_code_point_columns()
                            } else {
                                draft
                            });
                        }
                        rule_hits += 1;
                    }
                }
                for found in facts
                    .rule_matches(rule)
                    .iter()
                    .filter(|_| !constructor_pass)
                {
                    if dedup.iter().any(|&(start, end)| {
                        if is_new_language(language_name) {
                            (found.start, found.end) == (start, end)
                        } else {
                            found.start <= start && end <= found.end
                        }
                    }) {
                        continue;
                    }
                    if rule_hits >= budget.max_hits_per_rule
                        || examined >= budget.max_match_candidates
                    {
                        status = DetectorStatus::Truncated;
                        break;
                    }
                    let Some((line, column)) =
                        source_position(text, found.start, positions.as_ref())
                    else {
                        status = DetectorStatus::Truncated;
                        continue;
                    };
                    let raw = &text[found.start..found.end];
                    if found.budget_bytes() > remaining_match_bytes {
                        status = DetectorStatus::Truncated;
                        break;
                    }
                    remaining_match_bytes -= found.budget_bytes();
                    examined += 1;
                    let detection = Detection {
                        rule_id: rule.id.clone(),
                        location: Location {
                            file: file_path.clone(),
                            line,
                            column,
                        },
                        evidence: Evidence {
                            r#type: "tree_sitter_match".into(),
                            r#match: found
                                .safe_preview
                                .clone()
                                .unwrap_or_else(|| mask_preview(raw)),
                            snippet_preview: found.safe_preview.clone().unwrap_or_else(|| {
                                pqc_scan_core::model::snippet_around(
                                    text,
                                    found.start,
                                    found.end,
                                    64,
                                )
                            }),
                            metadata: BTreeMap::from([
                                ("detector".into(), self.name().into()),
                                ("language".into(), language_name.into()),
                                ("node_kind".into(), "bounded_source_fact".into()),
                                ("scope".into(), rule.scope.clone()),
                                ("source_resolution".into(), found.reason.into()),
                            ]),
                        },
                    };
                    if observe {
                        let draft = pqc_scan_core::ObservationDraft::source(
                            &detection,
                            pqc_scan_core::ObservationMethod::AstPattern,
                            raw,
                            Some((found.start, found.end)),
                        )
                        .with_algorithm_references(&found.algorithm_references);
                        observations.push(if is_new_language(language_name) {
                            let draft =
                                draft.with_source_api_references(&found.algorithm_references);
                            let draft = if let Some(reference) = found.source_library {
                                draft.with_source_library_reference(reference)
                            } else {
                                draft
                            };
                            let draft = if let Some(reference) = &found.source_configuration {
                                draft.with_source_configuration_reference(reference.clone())
                            } else {
                                draft
                            };
                            draft.with_code_point_columns()
                        } else {
                            draft
                        });
                    }
                    dedup.insert((found.start, found.end));
                    out.push(detection);
                    rule_hits += 1;
                }
                if !constructor_pass && rule.id == "TS_RSA_KEY_SIZE_LITERAL" {
                    rsa_dedup = dedup;
                    rsa_examined = examined;
                }
            }
        }

        if template_limited {
            status = DetectorStatus::Truncated;
        } else if template_pending && status != DetectorStatus::Truncated {
            status = DetectorStatus::Partial;
        }
        let test_ranges = if language_name == "rust" {
            test_scope::rust_test_ranges(original_tree.root_node(), text, budget.max_ast_nodes)
        } else {
            Vec::new()
        };
        let outcome = DetectorOutcome::new(out, status)
            .with_observations(observations)
            .with_test_ranges(test_ranges)
            .with_sensitive_ranges(facts.sensitive_ranges.clone());
        Ok(if template_pending {
            outcome.with_explanation("Unrendered template: static source inspected; generated values and branches remain unknown.")
        } else if positions.as_ref().is_some_and(|p| !p.complete) {
            outcome.with_limit(
                "source_position_entries",
                budget.max_ast_nodes,
                positions.as_ref().map_or(0, |p| p.entries()),
            )
        } else if let Some((reason, limit, processed)) = facts.limit_reason {
            outcome.with_limit(reason, limit, processed)
        } else if status == DetectorStatus::Truncated {
            outcome.with_explanation("AST inspection limit: node, candidate, alias or retained-match work was bounded. Retained findings remain available; omitted findings are unknown. Inspect the remaining source separately.")
        } else if status == DetectorStatus::Partial {
            outcome.with_explanation("Source syntax could not be fully interpreted. Parsed regions were inspected; unresolved syntax remains unknown. Check generated syntax, language version and the source file separately.")
        } else {
            outcome
        })
    }
}

fn collect_candidate_nodes<'a>(
    text: &'a str,
    tree: &tree_sitter::Tree,
    budget: crate::DetectorBudget,
    rsa_constructors: bool,
    language: &str,
    ambiguous_cpp: bool,
    positions: Option<&positions::Positions>,
) -> (
    Vec<CandidateNode<'a>>,
    DetectorStatus,
    Vec<CandidateNode<'a>>,
) {
    let mut out = Vec::new();
    let mut constructors = Vec::new();
    let mut stack = vec![tree.root_node()];
    let mut scheduled = 1;
    let mut status = DetectorStatus::Complete;

    'walk: while let Some(node) = stack.pop() {
        if node.kind() == "import_attribute" {
            let mut keys = HashSet::new();
            let mut cursor = node.walk();
            for object in node.named_children(&mut cursor) {
                let mut c = object.walk();
                for pair in object.named_children(&mut c) {
                    if let (Some(key), Some(value)) = (
                        pair.child_by_field_name("key"),
                        pair.child_by_field_name("value"),
                    ) {
                        if value.kind() != "string"
                            || !matches!(
                                key.kind(),
                                "property_identifier" | "identifier" | "string"
                            )
                            || !keys.insert(text[key.byte_range()].trim_matches(['\'', '"']))
                        {
                            status = DetectorStatus::Partial;
                        }
                    } else {
                        status = DetectorStatus::Partial;
                    }
                }
            }
        }
        if node.kind() == "jsx_element" {
            if let (Some(open), Some(close)) = (
                node.child_by_field_name("open_tag"),
                node.child_by_field_name("close_tag"),
            ) {
                let a = open.child_by_field_name("name");
                let b = close.child_by_field_name("name");
                if a.map(|n| &text[n.byte_range()]) != b.map(|n| &text[n.byte_range()]) {
                    status = DetectorStatus::Partial;
                }
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if scheduled >= budget.max_ast_nodes {
                status = DetectorStatus::Truncated;
                break 'walk;
            }
            stack.push(child);
            scheduled += 1;
        }

        if !node.is_named() {
            continue;
        }

        let kind = node.kind();
        if ambiguous_cpp && !header::c_kind_known(kind) && status == DetectorStatus::Complete {
            // C's permissive macro/declaration recovery can accept C++ scope
            // syntax as a different, error-free structure. Parser success alone
            // must not resolve the header's configured language.
            status = DetectorStatus::Partial;
        }
        let constructor = rsa_constructors
            && kind == "object_creation_expression"
            && node
                .child_by_field_name("type")
                .and_then(|n| n.utf8_text(text.as_bytes()).ok())
                .is_some_and(|name| name.rsplit('.').next() == Some("RSAKeyGenParameterSpec"));
        if !is_semantic_node_kind(kind) && !new_semantic_node(language, kind) && !constructor {
            continue;
        }

        let raw_text = match node.utf8_text(text.as_bytes()) {
            Ok(v) => v,
            Err(_) => {
                if status != DetectorStatus::Truncated {
                    status = DetectorStatus::Partial;
                }
                continue;
            }
        };
        if raw_text.trim().is_empty() {
            continue;
        }

        let target = if constructor {
            &mut constructors
        } else {
            &mut out
        };
        if target.len() >= budget.max_ast_candidates {
            status = DetectorStatus::Truncated;
            if constructor {
                continue;
            }
            break;
        }
        let point = node.start_position();
        let Some((line, column)) = (if positions.is_some() {
            source_position(text, node.start_byte(), positions)
        } else {
            Some((point.row + 1, point.column + 1))
        }) else {
            status = DetectorStatus::Truncated;
            continue;
        };
        target.push(CandidateNode {
            kind,
            text: raw_text,
            line,
            column,
            byte_start: node.start_byte(),
            canonical: None,
        });
    }

    (out, status, constructors)
}

fn clip_text(input: &str, max_chars: usize) -> (String, bool) {
    if let Some((boundary, _)) = input.char_indices().nth(max_chars) {
        let mut out = input[..boundary].to_string();
        out.push_str("...");
        (out, true)
    } else {
        (input.to_string(), false)
    }
}

fn is_semantic_node_kind(kind: &str) -> bool {
    kind.ends_with("identifier")
        || kind.contains("string")
        || kind == "import_path"
        || kind == "scoped_identifier"
        || kind == "qualified_identifier"
        || kind == "selector_expression"
        || kind == "call_expression"
        || kind == "method_invocation"
        || kind == "call"
        || kind == "attribute"
        || kind == "scope_resolution"
}

fn is_new_language(language: &str) -> bool {
    matches!(language, "php" | "csharp" | "c" | "cpp")
}
fn source_position(
    text: &str,
    offset: usize,
    positions: Option<&positions::Positions>,
) -> Option<(usize, usize)> {
    positions.map_or_else(
        || Some(pqc_scan_core::model::line_col_for_offset(text, offset)),
        |p| p.at(offset),
    )
}
fn new_semantic_node(language: &str, kind: &str) -> bool {
    match language {
        "php" => matches!(
            kind,
            "name"
                | "qualified_name"
                | "variable_name"
                | "function_call_expression"
                | "member_call_expression"
                | "scoped_call_expression"
                | "object_creation_expression"
        ),
        "csharp" => matches!(
            kind,
            "invocation_expression"
                | "member_access_expression"
                | "qualified_name"
                | "generic_name"
                | "object_creation_expression"
        ),
        "c" | "cpp" => matches!(kind, "field_expression" | "new_expression"),
        _ => false,
    }
}

fn scope_matches(scope: &str, language: &str) -> bool {
    let scope_lower = scope.to_ascii_lowercase();
    if scope_lower == "*" || scope_lower == "any" {
        return true;
    }

    for token in scope_lower
        .split(|c| [',', '|', ';', ' '].contains(&c))
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
    {
        if token == language {
            return true;
        }
        if language == "typescript" && token == "javascript" {
            return true;
        }
    }

    false
}

pub(crate) fn resolve_language(path: &Path, text: &str) -> Option<(Language, &'static str)> {
    if crate::source_kind::is_dockerfile(path, text) {
        return None;
    }
    if path.extension().is_some_and(|e| e == "C") {
        return Some((tree_sitter_cpp::LANGUAGE.into(), "cpp"));
    }
    let ext = path
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match ext.as_str() {
        "php" | "phtml" => Some((tree_sitter_php::LANGUAGE_PHP.into(), "php")),
        "inc" if has_php_tag(text) => Some((tree_sitter_php::LANGUAGE_PHP.into(), "php")),
        "inc"
            if path.file_stem().and_then(|n| n.to_str()).is_some_and(|n| {
                n.ends_with(".cc") || n.ends_with(".cpp") || n.ends_with(".cxx")
            }) =>
        {
            Some((tree_sitter_cpp::LANGUAGE.into(), "cpp"))
        }
        "inc"
            if path
                .file_stem()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".c")) =>
        {
            Some((tree_sitter_c::LANGUAGE.into(), "c"))
        }
        "cs" => Some((tree_sitter_c_sharp::LANGUAGE.into(), "csharp")),
        "c" | "h" => Some((tree_sitter_c::LANGUAGE.into(), "c")),
        "cc" | "cpp" | "cxx" | "hpp" | "hh" | "hxx" => {
            Some((tree_sitter_cpp::LANGUAGE.into(), "cpp"))
        }
        "java" => Some((tree_sitter_java::LANGUAGE.into(), "java")),
        "go" => Some((tree_sitter_go::LANGUAGE.into(), "go")),
        "js" | "mjs" | "cjs" | "jsx" => {
            Some((tree_sitter_javascript::LANGUAGE.into(), "javascript"))
        }
        "ts" => Some((
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            "typescript",
        )),
        "tsx" => Some((tree_sitter_typescript::LANGUAGE_TSX.into(), "typescript")),
        "py" => Some((tree_sitter_python::LANGUAGE.into(), "python")),
        "rs" => Some((tree_sitter_rust::LANGUAGE.into(), "rust")),
        "rb" => Some((tree_sitter_ruby::LANGUAGE.into(), "ruby")),
        _ => None,
    }
}

fn has_php_tag(text: &str) -> bool {
    text.match_indices("<?").any(|(i, _)| {
        let rest = &text[i + 2..];
        rest.starts_with('=')
            || (rest.get(..3).is_some_and(|s| s.eq_ignore_ascii_case("php"))
                && rest
                    .get(3..)
                    .is_some_and(|s| s.is_empty() || s.starts_with(char::is_whitespace)))
    })
}

fn is_match_boundary(c: char) -> bool {
    c.is_whitespace() || matches!(c, '"' | '\'' | '`')
}

fn is_embedded_identifier_submatch(text: &str, start: usize, end: usize) -> bool {
    if start >= end || end > text.len() {
        return false;
    }

    let m = &text[start..end];
    if m.is_empty()
        || !m
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_'))
    {
        return false;
    }

    let prev = text[..start].chars().next_back();
    let next = text[end..].chars().next();

    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    prev.is_some_and(is_ident) || next.is_some_and(is_ident)
}

pub struct BoundedTreeSitterDetector(crate::DetectorBudget);
impl TreeSitterDetector {
    pub fn with_budget(budget: crate::DetectorBudget) -> BoundedTreeSitterDetector {
        BoundedTreeSitterDetector(budget)
    }
}
impl Detector for BoundedTreeSitterDetector {
    fn name(&self) -> &'static str {
        "tree_sitter_detector"
    }
    fn detect(&self, f: &ScannableFile, r: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(f, r)?.into_detections())
    }
    fn detect_detailed(&self, f: &ScannableFile, r: &RuleSet) -> Result<DetectorOutcome> {
        TreeSitterDetector.detect_internal(f, r, false, self.0)
    }
    fn detect_observed(&self, f: &ScannableFile, r: &RuleSet) -> Result<DetectorOutcome> {
        TreeSitterDetector.detect_internal(f, r, true, self.0)
    }
}

pub(crate) struct ParsedSource {
    pub tree: tree_sitter::Tree,
    pub original_tree: tree_sitter::Tree,
    pub compatibility_limited: bool,
    pub template_pending: bool,
    pub template_limited: bool,
}

// Both AST rules and source-aware regex rules must use the same grammar view.
pub(crate) fn source_tree(
    text: &str,
    path: &Path,
    language: Language,
    language_name: &str,
    budget: crate::DetectorBudget,
) -> Result<ParsedSource> {
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .map_err(|_| anyhow::anyhow!("failed to initialize source parser"))?;

    let tree = parser
        .parse(text, None)
        .ok_or_else(|| anyhow::anyhow!("source parser produced no tree"))?;

    let original_tree = tree.clone();
    let template = if language_name == "javascript" {
        Some(crate::template_source::view(text, &original_tree))
    } else {
        None
    };
    let parse_text = template
        .as_ref()
        .filter(|v| v.found)
        .and_then(|v| std::str::from_utf8(&v.bytes).ok())
        .unwrap_or(text);
    let tree = if parse_text != text {
        parser
            .parse(parse_text, None)
            .ok_or_else(|| anyhow::anyhow!("template parser produced no tree"))?
    } else {
        tree
    };
    let (tree, compatibility_limited) = module_syntax::compatible_tree(
        &mut parser,
        parse_text,
        tree,
        language_name,
        budget.max_ast_match_bytes,
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("mjs")),
    )?;
    Ok(ParsedSource {
        tree,
        original_tree,
        compatibility_limited,
        template_pending: template.as_ref().is_some_and(|v| v.found)
            || (language_name == "php"
                && path
                    .file_name()
                    .and_then(|p| p.to_str())
                    .is_some_and(|p| p.ends_with(".blade.php"))),
        template_limited: template.as_ref().is_some_and(|v| v.limited),
    })
}

#[cfg(test)]
mod tests {
    use super::is_embedded_identifier_submatch;

    #[test]
    fn rejects_substring_inside_identifier() {
        let text = "registerSAMLScriptMapper";
        let start = text
            .to_ascii_lowercase()
            .find("rsa")
            .expect("rsa should exist in text");
        let end = start + 3;
        assert!(is_embedded_identifier_submatch(text, start, end));
    }

    #[test]
    fn rejects_signature_inside_snake_case_identifier() {
        let text = "signature_required";
        let start = text.find("signature").expect("signature token");
        let end = start + "signature".len();
        assert!(is_embedded_identifier_submatch(text, start, end));
    }

    #[test]
    fn keeps_standalone_token_matches() {
        let text = "Signature.getInstance(\"SHA256withRSA\")";
        let start = text.find("Signature").expect("signature token");
        let end = start + "Signature".len();
        assert!(!is_embedded_identifier_submatch(text, start, end));
    }
}
