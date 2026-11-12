//! Ruby declaration inventory, never DSL execution or installation resolution.
use super::parse::*;
use pqc_scan_core::RepositoryContext;
use tree_sitter::{Node, Parser};
fn literal<'a>(node: Node<'_>, text: &'a str) -> Option<&'a str> {
    if node.kind() != "string" {
        return None;
    }
    let raw = text.get(node.byte_range())?;
    let q = *raw.as_bytes().first()?;
    if !matches!(q, b'\'' | b'"') || raw.len() < 2 || raw.as_bytes().last() != Some(&q) {
        return None;
    }
    let value = &raw[1..raw.len() - 1];
    (!value.contains("#{") && !value.contains(['\\', '\n'])).then_some(value)
}
fn method<'a>(node: Node<'_>, text: &'a str) -> Option<&'a str> {
    node.child_by_field_name("method")
        .and_then(|n| text.get(n.byte_range()))
}
fn bound_spec(node: Node<'_>, receiver: &str, text: &str) -> bool {
    for n in std::iter::successors(Some(node), |n| n.parent()).take(64) {
        if matches!(n.kind(), "method" | "singleton_method") {
            return false;
        }
        if !matches!(n.kind(), "do_block" | "block") {
            continue;
        }
        let Some(parameters) = n.child_by_field_name("parameters") else {
            continue;
        };
        let raw = text[parameters.byte_range()].trim_matches('|').trim();
        if !raw
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|part| part == receiver)
        {
            continue;
        }
        let Some(call) = n.parent() else { return false };
        return raw == receiver
            && method(call, text) == Some("new")
            && call
                .child_by_field_name("receiver")
                .and_then(|r| text.get(r.byte_range()))
                == Some("Gem::Specification");
    }
    false
}

fn declaration_block(n: Node<'_>, text: &str) -> bool {
    match method(n, text) {
        Some("group" | "platform" | "platforms" | "source") => {
            n.child_by_field_name("receiver").is_none()
        }
        Some("new") => {
            n.child_by_field_name("receiver")
                .and_then(|r| text.get(r.byte_range()))
                == Some("Gem::Specification")
        }
        _ => false,
    }
}
pub(super) fn inspect(
    file: &ScannableFile,
    repo: Option<&RepositoryContext>,
) -> anyhow::Result<Parsed> {
    let Some(text) = file.text.as_deref() else {
        return failure(Failure::Unsupported);
    };
    if text.len() > 2 * 1024 * 1024 {
        return failure(Failure::Limit);
    }
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_ruby::LANGUAGE.into())?;
    let Some(tree) = parser.parse(text, None) else {
        return failure(Failure::Limit);
    };
    let constraint = Regex::new(
        r"^(?:(?:~>|>=|<=|!=|==|=|>|<)\s*)?[0-9]+(?:\.[A-Za-z0-9]+)*(?:-[A-Za-z0-9.-]+)?$",
    )?;
    let mut rebound = std::collections::BTreeSet::new();
    let mut nodes = vec![tree.root_node()];
    let mut preflight = 200000usize;
    while let Some(n) = nodes.pop() {
        if preflight == 0 {
            return failure(Failure::Limit);
        }
        preflight -= 1;
        if matches!(
            n.kind(),
            "assignment" | "operator_assignment" | "multiple_assignment"
        ) {
            if let Some(left) = n.child_by_field_name("left") {
                let mut bindings = vec![left];
                while let Some(binding) = bindings.pop() {
                    if preflight == 0 {
                        return failure(Failure::Limit);
                    }
                    preflight -= 1;
                    if binding.kind() == "identifier" {
                        rebound.insert(text[binding.byte_range()].to_string());
                    } else if !matches!(binding.kind(), "call" | "element_reference") {
                        let mut c = binding.walk();
                        bindings.extend(binding.named_children(&mut c));
                    }
                }
            }
        }
        let mut c = n.walk();
        nodes.extend(n.named_children(&mut c));
    }
    let mut b = Build::new(DetectorStatus::Complete);
    let mut incomplete = tree.root_node().has_error();
    let mut stack = vec![tree.root_node()];
    let mut work = 200000usize;
    while let Some(n) = stack.pop() {
        if work == 0 {
            b.status = DetectorStatus::Truncated;
            break;
        }
        work -= 1;
        if n.kind() == "call" {
            let name = method(n, text).unwrap_or("");
            if matches!(
                name,
                "eval"
                    | "instance_eval"
                    | "class_eval"
                    | "send"
                    | "public_send"
                    | "eval_gemfile"
                    | "load"
            ) {
                incomplete = true;
            }
            if name == "gemspec" && n.child_by_field_name("receiver").is_none() {
                let simple = n.child_by_field_name("arguments").is_none();
                let candidates = repo
                    .map(|r| r.sibling_files(&file.path, "gemspec"))
                    .unwrap_or_default();
                // Delegated dependencies retain the gemspec's own source path/positions in
                // its separately selected scan; this directive must not relabel them here.
                if !simple || candidates.len() != 1 {
                    incomplete = true;
                } else if let Some(repo) = repo {
                    if let Some(child) = repo
                        .read_relative(&file.path, Path::new(candidates[0].file_name().unwrap()))
                    {
                        let child = inspect(&child, None)?;
                        if child.status != DetectorStatus::Complete {
                            incomplete = true;
                        }
                        if child.status == DetectorStatus::Truncated {
                            b.status = DetectorStatus::Truncated;
                        }
                    } else {
                        incomplete = true;
                    }
                }
            }
            if n.child_by_field_name("receiver").is_none()
                && !matches!(
                    name,
                    "gem" | "gemspec" | "group" | "platforms" | "platform" | "source" | "ruby"
                )
            {
                incomplete = true;
            }
            let gem = name == "gem" && n.child_by_field_name("receiver").is_none();
            let spec = matches!(
                name,
                "add_dependency" | "add_runtime_dependency" | "add_development_dependency"
            ) && n
                .child_by_field_name("receiver")
                .and_then(|r| text.get(r.byte_range()))
                .is_some_and(|r| !rebound.contains(r) && bound_spec(n, r, text));
            if !spec
                && matches!(
                    name,
                    "add_dependency"
                        | "add_runtime_dependency"
                        | "add_development_dependency"
                        | "dependencies"
                        | "runtime_dependencies"
                        | "development_dependencies"
                )
            {
                incomplete = true;
            }
            if gem || spec {
                let Some(args) = n.child_by_field_name("arguments") else {
                    incomplete = true;
                    continue;
                };
                let mut c = args.walk();
                let args: Vec<_> = args.named_children(&mut c).collect();
                let Some(dep) = args.first().and_then(|n| literal(*n, text)) else {
                    incomplete = true;
                    continue;
                };
                let mut conditional =
                    std::iter::successors(Some(n), |n| n.parent())
                        .take(64)
                        .any(|p| {
                            matches!(
                                p.kind(),
                                "if" | "unless" | "if_modifier" | "unless_modifier" | "case"
                            ) || p.kind() == "call"
                                && matches!(
                                    method(p, text),
                                    Some("group" | "platforms" | "platform")
                                )
                        });
                let deferred = std::iter::successors(Some(n), |n| n.parent())
                    .take(64)
                    .any(|p| {
                        matches!(
                            p.kind(),
                            "method"
                                | "singleton_method"
                                | "lambda"
                                | "while"
                                | "until"
                                | "while_modifier"
                                | "until_modifier"
                                | "for"
                        ) || p.kind() == "call"
                            && p.child_by_field_name("block").is_some()
                            && !declaration_block(p, text)
                    });
                conditional |= deferred;
                incomplete |= deferred;
                let mut origin = if conditional {
                    Origin::ManifestOptional
                } else {
                    Origin::Manifest
                };
                let mut versions = vec![];
                let mut dynamic = false;
                for arg in args.iter().skip(1) {
                    if let Some(value) = literal(*arg, text) {
                        versions.push(value.to_string());
                    } else if arg.kind() == "pair" {
                        if let Some(key) = arg
                            .child_by_field_name("key")
                            .and_then(|k| text.get(k.byte_range()))
                        {
                            match key.trim_start_matches(':') {
                                "platform" | "platforms" => {
                                    conditional = true;
                                    if matches!(origin, Origin::Manifest) {
                                        origin = Origin::ManifestOptional;
                                    }
                                }
                                "git" => origin = Origin::Git,
                                "path" => origin = Origin::Path,
                                _ => {}
                            }
                        }
                    } else {
                        dynamic = true;
                    }
                }
                dynamic |= versions.iter().any(|v| !constraint.is_match(v.trim()));
                incomplete |= dynamic;
                let constraint = versions.join(", ");
                b.add(
                    "gem",
                    dep,
                    (!dynamic && !constraint.is_empty()).then_some(constraint.as_str()),
                    (!dynamic && !constraint.is_empty()).then_some(constraint.as_str()),
                    None,
                    if dynamic { Origin::Unknown } else { origin },
                    n.start_position().row + 1,
                    false,
                );
                if let Some(item) = b.items.last_mut() {
                    item.context = if deferred {
                        Some("deferred_declaration_not_evaluated")
                    } else if dynamic {
                        Some("dynamic_version_not_evaluated")
                    } else if conditional {
                        Some("conditional_declaration_not_effective")
                    } else {
                        None
                    };
                }
            }
        }
        let mut cursor = n.walk();
        for child in n.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    b.items.sort_by_key(|i| i.line);
    for (ordinal, item) in b.items.iter_mut().enumerate() {
        item.ordinal = ordinal;
    }
    if incomplete && b.status != DetectorStatus::Truncated {
        b.status = DetectorStatus::Partial;
    }
    b.finish()
}
use std::path::Path;
