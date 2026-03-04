use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use anyhow::Result;
use tree_sitter::{Language, Parser};

use crate::comment_filter::should_skip_comment_only_match_for_language;
use pqc_scan_core::model::mask_preview;
use pqc_scan_core::{Detection, Detector, Evidence, Location, ScannableFile};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug, Default)]
pub struct TreeSitterDetector;

#[derive(Debug, Clone)]
struct CandidateNode {
    kind: String,
    text: String,
    snippet_preview: String,
    line: usize,
    column: usize,
}

impl Detector for TreeSitterDetector {
    fn name(&self) -> &'static str {
        "tree_sitter_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        let text = match file.text.as_ref() {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        let (language, language_name) = match resolve_language(&file.path) {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        let mut parser = Parser::new();
        if parser.set_language(&language).is_err() {
            return Ok(Vec::new());
        }

        let tree = match parser.parse(text, None) {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        let candidates = collect_candidate_nodes(text, &tree);
        if candidates.is_empty() {
            return Ok(Vec::new());
        }

        let mut out = Vec::new();
        let mut dedup = HashSet::new();
        let file_path = file.path.to_string_lossy().into_owned();

        for rule in rules.by_kind(RuleKind::TreeSitter) {
            if !scope_matches(&rule.scope, language_name) {
                continue;
            }

            let regex = match rule.compiled_pattern() {
                Some(v) => v,
                None => continue,
            };

            let mut rule_hits = 0usize;
            for candidate in &candidates {
                if rule_hits >= 32 {
                    break;
                }

                for hit in regex.find_iter(&candidate.text) {
                    if rule_hits >= 32 {
                        break;
                    }
                    if should_skip_comment_only_match_for_language(
                        language_name,
                        &candidate.text,
                        hit.start(),
                    ) {
                        continue;
                    }
                    if is_embedded_identifier_submatch(&candidate.text, hit.start(), hit.end()) {
                        continue;
                    }

                    let (relative_line, relative_col) =
                        pqc_scan_core::model::line_col_for_offset(&candidate.text, hit.start());
                    let line = candidate.line + relative_line.saturating_sub(1);
                    let column = if relative_line == 1 {
                        candidate.column + relative_col.saturating_sub(1)
                    } else {
                        relative_col
                    };

                    let dedup_key = format!(
                        "{}:{}:{}:{}",
                        rule.id,
                        file_path,
                        line,
                        normalize_match_for_dedup(hit.as_str())
                    );
                    if !dedup.insert(dedup_key) {
                        continue;
                    }

                    let mut metadata = BTreeMap::new();
                    metadata.insert("detector".to_string(), self.name().to_string());
                    metadata.insert("language".to_string(), language_name.to_string());
                    metadata.insert("node_kind".to_string(), candidate.kind.clone());
                    metadata.insert("scope".to_string(), rule.scope.clone());

                    out.push(Detection {
                        rule_id: rule.id.clone(),
                        location: Location {
                            file: file_path.clone(),
                            line,
                            column,
                        },
                        evidence: Evidence {
                            r#type: "tree_sitter_match".to_string(),
                            r#match: mask_preview(hit.as_str()),
                            snippet_preview: mask_preview(&candidate.snippet_preview),
                            metadata,
                        },
                    });
                    rule_hits += 1;
                }
            }
        }

        Ok(out)
    }
}

fn collect_candidate_nodes(text: &str, tree: &tree_sitter::Tree) -> Vec<CandidateNode> {
    let mut out = Vec::new();
    let mut stack = vec![tree.root_node()];

    while let Some(node) = stack.pop() {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            stack.push(child);
        }

        if !node.is_named() {
            continue;
        }

        let kind = node.kind();
        if !is_semantic_node_kind(kind) {
            continue;
        }

        let raw_text = match node.utf8_text(text.as_bytes()) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if raw_text.trim().is_empty() {
            continue;
        }

        let clipped = clip_text(raw_text, 2000);
        let point = node.start_position();
        out.push(CandidateNode {
            kind: kind.to_string(),
            text: clipped,
            snippet_preview: clip_text(raw_text.trim(), 240),
            line: point.row + 1,
            column: point.column + 1,
        });
    }

    out
}

fn clip_text(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    let mut out: String = input.chars().take(max_chars).collect();
    out.push_str("...");
    out
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

fn resolve_language(path: &Path) -> Option<(Language, &'static str)> {
    let ext = path
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match ext.as_str() {
        "java" => Some((tree_sitter_java::LANGUAGE.into(), "java")),
        "go" => Some((tree_sitter_go::LANGUAGE.into(), "go")),
        "js" | "mjs" | "cjs" | "jsx" => {
            Some((tree_sitter_javascript::LANGUAGE.into(), "javascript"))
        }
        "ts" | "tsx" => Some((tree_sitter_javascript::LANGUAGE.into(), "typescript")),
        "py" => Some((tree_sitter_python::LANGUAGE.into(), "python")),
        "rs" => Some((tree_sitter_rust::LANGUAGE.into(), "rust")),
        "rb" => Some((tree_sitter_ruby::LANGUAGE.into(), "ruby")),
        _ => None,
    }
}

fn normalize_match_for_dedup(raw: &str) -> String {
    raw.trim_matches(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '`'))
        .to_ascii_lowercase()
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
