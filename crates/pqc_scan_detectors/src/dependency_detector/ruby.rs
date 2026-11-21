//! Ruby declaration inventory, never DSL execution or installation resolution.
mod loads;
mod versions;
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
    inspect_level(file, repo, true)
}
fn inspect_level(
    file: &ScannableFile,
    repo: Option<&RepositoryContext>,
    allow_delegation: bool,
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
    let mut visited = Vec::new();
    while let Some(n) = nodes.pop() {
        if preflight == 0 {
            return failure(Failure::Limit);
        }
        preflight -= 1;
        visited.push(n);
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
    let mut version_values = versions::Versions::new(&visited, text, file, repo);
    let loads = loads::Loads::new(tree.root_node(), text, file, repo);
    let mut delegated_status = None;
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
        if matches!(n.kind(), "alias" | "undef") {
            incomplete = true;
        }
        // A no-argument Ruby DSL directive is an identifier in this grammar.
        // Model only the direct top-level gemspec form, never a bound local.
        let bare_gemspec = n.kind() == "identifier"
            && &text[n.byte_range()] == "gemspec"
            && n.parent().is_some_and(|p| {
                matches!(p.kind(), "program" | "body_statement" | "then" | "else")
            });
        if n.kind() == "call" || bare_gemspec {
            let name = if bare_gemspec {
                "gemspec"
            } else {
                method(n, text).unwrap_or("")
            };
            if matches!(name, "require" | "require_relative") && !loads.approved(n) {
                incomplete = true;
            }
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
                let simple = n.child_by_field_name("arguments").is_none()
                    && !rebound.contains("gemspec")
                    && n.parent().is_some_and(|p| p.kind() == "program");
                if !allow_delegation || !simple {
                    incomplete = true;
                } else {
                    let status = memoized_status(&mut delegated_status, || {
                        let candidates = repo
                            .map(|r| r.sibling_files(&file.path, "gemspec"))
                            .unwrap_or_default();
                        if candidates.len() != 1 {
                            return Ok(DetectorStatus::Partial);
                        }
                        let Some(child) = repo.and_then(|r| {
                            r.read_relative(
                                &file.path,
                                Path::new(candidates[0].file_name().unwrap()),
                            )
                        }) else {
                            return Ok(DetectorStatus::Partial);
                        };
                        // The child's declarations keep their own separately selected
                        // source path. One child parse per parent; never delegate again.
                        Ok(inspect_level(&child, repo, false)?.status)
                    })?;
                    incomplete |= status != DetectorStatus::Complete;
                    if status == DetectorStatus::Truncated {
                        b.status = status;
                    }
                }
            }
            if n.child_by_field_name("receiver").is_none()
                && !loads.approved(n)
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
                    if let Some(value) = literal(*arg, text)
                        .map(str::to_string)
                        .or_else(|| version_values.value(*arg))
                    {
                        versions.push(value);
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
    if version_values.limited || loads.limited {
        b.status = DetectorStatus::Truncated;
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

fn memoized_status(
    cache: &mut Option<DetectorStatus>,
    load: impl FnOnce() -> anyhow::Result<DetectorStatus>,
) -> anyhow::Result<DetectorStatus> {
    if let Some(status) = *cache {
        return Ok(status);
    }
    let status = load()?;
    *cache = Some(status);
    Ok(status)
}
#[cfg(test)]
mod cache_tests {
    use super::*;
    #[test]
    fn repeated_delegation_spends_one_child_inspection_even_when_partial() {
        for result in [
            DetectorStatus::Complete,
            DetectorStatus::Partial,
            DetectorStatus::Truncated,
        ] {
            let mut cache = None;
            let mut calls = 0;
            for _ in 0..1000 {
                assert_eq!(
                    memoized_status(&mut cache, || {
                        calls += 1;
                        Ok(result)
                    })
                    .unwrap(),
                    result
                );
            }
            assert_eq!(calls, 1);
        }
    }
}
