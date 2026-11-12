//! Bounded static import binding; no imports, target code or package managers run.
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::{Node, Tree};

pub(super) struct ResolvedCall {
    pub start: usize,
    pub end: usize,
    pub canonical: String,
}
struct Work(usize);
impl Work {
    fn spend(&mut self) -> Option<()> {
        self.0 = self.0.checked_sub(1)?;
        Some(())
    }
}

pub(super) fn aliases(
    text: &str,
    tree: &Tree,
    language: &str,
    max_nodes: usize,
    max_calls: usize,
) -> (Vec<ResolvedCall>, bool) {
    if language != "python" {
        return (Vec::new(), false);
    }
    let mut work = Work(max_nodes.saturating_mul(8));
    match resolve(text, tree, &mut work, max_calls) {
        Some(calls) => (calls, false),
        None => (Vec::new(), true),
    }
}
fn resolve(
    text: &str,
    tree: &Tree,
    work: &mut Work,
    max_calls: usize,
) -> Option<Vec<ResolvedCall>> {
    let mut imports = BTreeMap::new();
    let mut rebound = BTreeSet::new();
    let mut ambiguous = BTreeSet::new();
    let mut wildcard_scopes = BTreeSet::new();
    let mut calls = Vec::new();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        work.spend()?;
        match node.kind() {
            "import_from_statement" if !node.has_error() => {
                let module = node
                    .child_by_field_name("module_name")
                    .map(|n| text[n.byte_range()].to_string())
                    .unwrap_or_default();
                let scope = scopes(node, work, false)?[0];
                let mut c = node.walk();
                if node
                    .named_children(&mut c)
                    .any(|n| n.kind() == "wildcard_import")
                {
                    wildcard_scopes.insert(scope);
                }
                let mut c = node.walk();
                for item in node.children_by_field_name("name", &mut c) {
                    if let Some((name, alias)) = import_name(item, text) {
                        let local = alias.unwrap_or_else(|| name.clone());
                        remember(
                            &mut imports,
                            &mut rebound,
                            scope,
                            local,
                            supported_module(&module).then(|| format!("{module}.{name}")),
                        );
                    }
                }
            }
            "import_statement" if !node.has_error() => {
                let scope = scopes(node, work, false)?[0];
                let mut c = node.walk();
                for item in node.children_by_field_name("name", &mut c) {
                    if let Some((name, alias)) = import_name(item, text) {
                        let supported = supported_module(&name);
                        let (local, canonical) = if let Some(alias) = alias {
                            (alias, name)
                        } else {
                            let root = name.split('.').next()?.to_string();
                            (root.clone(), root)
                        };
                        remember(
                            &mut imports,
                            &mut rebound,
                            scope,
                            local,
                            supported.then_some(canonical),
                        );
                    }
                }
            }
            "assignment"
            | "augmented_assignment"
            | "named_expression"
            | "for_statement"
            | "for_in_clause"
            | "as_pattern" => {
                if let Some(left) = node
                    .child_by_field_name("left")
                    .or_else(|| node.child_by_field_name("name"))
                    .or_else(|| node.child_by_field_name("alias"))
                {
                    bindings(
                        left,
                        text,
                        scopes(node, work, false)?[0],
                        &mut rebound,
                        work,
                    )?;
                }
            }
            "function_definition" | "class_definition" => {
                if let Some(name) = node.child_by_field_name("name") {
                    rebound.insert((
                        scopes(node.parent()?, work, false)?[0],
                        text[name.byte_range()].to_string(),
                    ));
                }
            }
            "parameters" | "lambda_parameters" => bindings(
                node,
                text,
                scopes(node, work, false)?[0],
                &mut rebound,
                work,
            )?,
            "global_statement" | "nonlocal_statement" => {
                let mut c = node.walk();
                for child in node.named_children(&mut c) {
                    if child.kind() == "identifier" {
                        ambiguous.insert(text[child.byte_range()].to_string());
                    }
                }
            }
            "call" => {
                if calls.len() >= max_calls {
                    return None;
                }
                if let Some(function) = node.child_by_field_name("function") {
                    calls.push(function);
                }
            }
            _ => {}
        }
        let mut c = node.walk();
        for child in node.named_children(&mut c) {
            stack.push(child);
        }
    }
    let mut out = Vec::new();
    for function in calls {
        let raw = &text[function.byte_range()];
        // Supported qualified call names are short; do not repeatedly split
        // arbitrarily large computed receivers or copy them into evidence.
        if raw.len() > 512 {
            continue;
        }
        work.spend()?;
        let parts = raw.split('.').map(str::trim).collect::<Vec<_>>();
        if !parts.iter().all(|p| identifier(p)) {
            continue;
        }
        let first = parts[0];
        if ambiguous.contains(first) {
            continue;
        }
        let mut resolved = None;
        for scope in scopes(function, work, true)? {
            if wildcard_scopes.contains(&scope) || rebound.contains(&(scope, first.to_string())) {
                break;
            }
            if let Some(binding) = imports.get(&(scope, first.to_string())) {
                resolved = Some(binding);
                break;
            }
        }
        let Some(binding) = resolved else {
            continue;
        };
        let canonical = if parts.len() == 1 {
            binding.clone()
        } else {
            format!("{binding}.{}", parts[1..].join("."))
        };
        if canonical.len() > 256 {
            continue;
        }
        out.push(ResolvedCall {
            start: function.start_byte(),
            end: function.end_byte(),
            canonical,
        });
    }
    Some(out)
}
fn import_name(node: Node<'_>, text: &str) -> Option<(String, Option<String>)> {
    let (name, alias) = if node.kind() == "aliased_import" {
        (
            node.child_by_field_name("name")?,
            node.child_by_field_name("alias"),
        )
    } else {
        (node, None)
    };
    let name = text[name.byte_range()]
        .split('.')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(".");
    if !name.split('.').all(identifier) {
        return None;
    }
    Some((name, alias.map(|a| text[a.byte_range()].to_string())))
}
fn supported_module(module: &str) -> bool {
    module.len() <= 128
        && (module == "cryptography"
            || module == "Crypto"
            || module == "Crypto.PublicKey"
            || module.starts_with("Crypto.PublicKey.")
            || module == "cryptography.hazmat.primitives.asymmetric"
            || module.starts_with("cryptography.hazmat.primitives.asymmetric."))
}
fn identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
        && chars.all(|c| c == '_' || c.is_alphanumeric())
}
fn scopes(mut node: Node<'_>, work: &mut Work, evaluation: bool) -> Option<Vec<usize>> {
    let origin = (node.start_byte(), node.end_byte());
    let mut out = Vec::new();
    let mut in_function = false;
    loop {
        work.spend()?;
        match node.kind() {
            "function_definition" | "lambda" => {
                let in_body = node.child_by_field_name("body").is_some_and(|body| {
                    body.start_byte() <= origin.0 && body.end_byte() >= origin.1
                });
                if !evaluation || in_body {
                    out.push(node.start_byte() + 1);
                    in_function = true;
                }
            }
            "list_comprehension"
            | "set_comprehension"
            | "dictionary_comprehension"
            | "generator_expression" => out.push(node.start_byte() + 1),
            "class_definition" if !in_function => {
                let in_body = node.child_by_field_name("body").is_some_and(|body| {
                    body.start_byte() <= origin.0 && body.end_byte() >= origin.1
                });
                if !evaluation || in_body {
                    out.push(node.start_byte() + 1);
                }
            }
            _ => {}
        }
        if let Some(parent) = node.parent() {
            node = parent;
        } else {
            break;
        }
    }
    out.push(0);
    Some(out)
}
fn bindings(
    node: Node<'_>,
    text: &str,
    scope: usize,
    out: &mut BTreeSet<(usize, String)>,
    work: &mut Work,
) -> Option<()> {
    let mut pending = vec![node];
    while let Some(node) = pending.pop() {
        work.spend()?;
        if node.kind() == "identifier" {
            out.insert((scope, text[node.byte_range()].to_string()));
        }
        let selected = match node.kind() {
            "default_parameter" | "typed_default_parameter" => node.child_by_field_name("name"),
            "attribute" => node.child_by_field_name("object"),
            "subscript" => node.child_by_field_name("value"),
            _ => None,
        };
        if let Some(name) = selected {
            pending.push(name);
            continue;
        }
        let annotation = node.child_by_field_name("type");
        let mut c = node.walk();
        for child in node.named_children(&mut c) {
            if annotation.is_none_or(|n| n.id() != child.id()) {
                pending.push(child);
            }
        }
    }
    Some(())
}

fn remember(
    imports: &mut BTreeMap<(usize, String), String>,
    rebound: &mut BTreeSet<(usize, String)>,
    scope: usize,
    name: String,
    canonical: Option<String>,
) {
    let key = (scope, name);
    if let Some(canonical) = canonical {
        if imports
            .insert(key.clone(), canonical.clone())
            .is_some_and(|previous| previous != canonical)
        {
            rebound.insert(key);
        }
    } else {
        rebound.insert(key);
    }
}
