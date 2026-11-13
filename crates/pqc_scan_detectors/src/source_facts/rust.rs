use super::{
    context::{children, pattern_names, Context, MAX_DEPTH},
    Facts,
};
use tree_sitter::Node;
const CRATES: [&str; 3] = ["ml_kem", "ml_dsa", "slh_dsa"];
static INCOMPLETE_USE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    let crates = CRATES.map(regex::escape).join("|");
    regex::Regex::new(&format!(r"^\s*use\s+({crates})::\s*$")).expect("known Rust crate names")
});
fn root_name(ctx: &Context<'_, '_>, mut node: Node<'_>) -> String {
    for _ in 0..MAX_DEPTH {
        if let Some(path) = node.child_by_field_name("path") {
            node = path;
            continue;
        }
        if matches!(node.kind(), "use_wildcard" | "use_as_clause") {
            if let Some(child) = node.named_child(0) {
                node = child;
                continue;
            }
        }
        return ctx.source(node).trim_start_matches("::").to_string();
    }
    ctx.truncated.set(true);
    String::new()
}
fn external(ctx: &Context<'_, '_>, node: Node<'_>) -> bool {
    let name = root_name(ctx, node);
    CRATES.contains(&name.as_str())
        && (ctx.source(node).starts_with("::") || ctx.lookup(node, &name).is_none())
}
fn known_type(name: &str, krate: &str) -> bool {
    match krate {
        "ml_kem" => matches!(name, "MlKem512" | "MlKem768" | "MlKem1024"),
        "ml_dsa" => matches!(name, "MlDsa44" | "MlDsa65" | "MlDsa87"),
        _ => false,
    }
}
fn type_family(ctx: &Context<'_, '_>, node: Node<'_>) -> Option<&'static str> {
    if matches!(node.kind(), "scoped_identifier" | "scoped_type_identifier") {
        return external(ctx, node)
            .then(|| root_name(ctx, node))
            .and_then(|name| CRATES.into_iter().find(|k| *k == name));
    }
    if !matches!(node.kind(), "identifier" | "type_identifier") {
        return None;
    }
    let name = ctx.source(node);
    if let Some(b) = ctx.lookup(node, name) {
        return ctx
            .unchanged(b)
            .then_some(b)
            .and_then(|b| b.imported.as_deref())
            .and_then(|s| s.strip_prefix("rust:"))
            .and_then(|name| {
                CRATES
                    .into_iter()
                    .find(|k| name.split("::").next() == Some(*k))
            });
    }
    CRATES
        .into_iter()
        .find(|krate| known_type(name, krate) && ctx.lookup(node, &format!("*{krate}")).is_some())
}
fn generation(ctx: &Context<'_, '_>, node: Node<'_>) -> Option<&'static str> {
    if node.kind() != "call_expression" {
        return None;
    }
    let function = node.child_by_field_name("function")?;
    if function.kind() != "scoped_identifier"
        || function
            .child_by_field_name("name")
            .is_none_or(|n| ctx.source(n) != "generate")
    {
        return None;
    }
    type_family(ctx, function.child_by_field_name("path")?)
}
fn role(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> Option<usize> {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return None;
    }
    if node.kind() != "identifier" {
        return None;
    }
    let b = ctx.lookup(node, ctx.source(node))?;
    if !ctx.unchanged(b) {
        return None;
    }
    let value = b.value?;
    if value.start_byte() >= node.start_byte() {
        return None;
    }
    if generation(ctx, value) == Some("ml_kem") {
        return b.projection.as_deref()?.parse().ok();
    }
    if b.projection.is_none() {
        return role(ctx, value, depth + 1);
    }
    None
}
pub(super) fn inspect<'a>(ctx: &mut Context<'a, '_>, facts: &mut Facts) {
    let nodes = ctx.nodes.clone();
    for node in &nodes {
        let scope = ctx.scope(*node);
        match node.kind() {
            "mod_item" | "struct_item" | "enum_item" | "function_item" => {
                if let Some(name) = node.child_by_field_name("name") {
                    let scope = if node.kind() == "function_item" {
                        ctx.parent_scope(*node)
                    } else {
                        scope
                    };
                    ctx.add_hoisted(scope, ctx.source(name), None, name.id());
                }
            }
            "let_declaration" => {
                if let Some(pattern) = node.child_by_field_name("pattern") {
                    for (name, projection) in pattern_names(pattern, ctx.text, 0) {
                        ctx.add(
                            scope,
                            &name,
                            node.child_by_field_name("value"),
                            projection,
                            None,
                            pattern.id(),
                        );
                    }
                }
            }
            "parameter" => {
                if let Some(pattern) = node.child_by_field_name("pattern") {
                    for (name, _) in pattern_names(pattern, ctx.text, 0) {
                        ctx.add(scope, &name, None, None, None, pattern.id());
                    }
                }
            }
            "closure_parameters" => {
                for (name, _) in pattern_names(*node, ctx.text, 0) {
                    ctx.add(scope, &name, None, None, None, node.id());
                }
            }
            "type_parameter" => {
                let mut ancestor = node.parent();
                let mut applicable = false;
                for _ in 0..64 {
                    let Some(parent) = ancestor else {
                        break;
                    };
                    if matches!(parent.kind(), "struct_item" | "enum_item") {
                        break;
                    }
                    if matches!(parent.kind(), "function_item" | "impl_item" | "trait_item") {
                        applicable = true;
                        break;
                    }
                    ancestor = parent.parent();
                }
                if applicable {
                    if let Some(name) = node.child_by_field_name("name") {
                        ctx.add(scope, ctx.source(name), None, None, None, name.id());
                    }
                }
            }
            _ => {}
        }
    }
    for node in &nodes {
        if node.kind() != "use_declaration" {
            continue;
        }
        let Some(argument) = node.child_by_field_name("argument") else {
            continue;
        };
        if !external(ctx, argument) {
            continue;
        }
        let krate = root_name(ctx, argument);
        facts.node("PQC_RUST_API", argument, "explicit_rust_pqc_import");
        if argument.kind() == "use_wildcard" {
            ctx.add_hoisted(
                ctx.scope(*node),
                &format!("*{krate}"),
                Some(format!("rust:{krate}::*")),
                argument.id(),
            );
            continue;
        }
        if argument.kind() == "use_as_clause" {
            if let Some(alias) = argument.child_by_field_name("alias") {
                ctx.add_hoisted(
                    ctx.scope(*node),
                    ctx.source(alias),
                    Some(format!("rust:{krate}")),
                    alias.id(),
                );
            }
        } else if let Some(name) = argument.child_by_field_name("name") {
            ctx.add_hoisted(
                ctx.scope(*node),
                ctx.source(name),
                Some(format!("rust:{krate}")),
                name.id(),
            );
        } else if let Some(list) = argument.child_by_field_name("list") {
            for name in children(list) {
                if matches!(name.kind(), "identifier" | "type_identifier") {
                    ctx.add_hoisted(
                        ctx.scope(*node),
                        ctx.source(name),
                        Some(format!("rust:{krate}")),
                        name.id(),
                    );
                }
            }
        }
    }
    for node in &nodes {
        if node.kind() == "assignment_expression" {
            if let Some(left) = node.child_by_field_name("left") {
                for (name, _) in pattern_names(left, ctx.text, 0) {
                    ctx.mark_write(left, &name);
                }
            }
        }
    }
    let incomplete = &*INCOMPLETE_USE;
    let mut examined = 0;
    for node in nodes {
        if examined >= ctx.budget.max_match_candidates {
            facts.truncated = true;
            break;
        }
        match node.kind() {
            "call_expression" => {
                examined += 1;
                if generation(ctx, node).is_some() {
                    facts.callee("PQC_RUST_API", node, "locally_imported_rust_type");
                    continue;
                }
                let Some(function) = node.child_by_field_name("function") else {
                    continue;
                };
                if function.kind() == "field_expression" {
                    let (Some(value), Some(field)) = (
                        function.child_by_field_name("value"),
                        function.child_by_field_name("field"),
                    ) else {
                        continue;
                    };
                    let method = ctx.source(field);
                    let role = role(ctx, value, 0);
                    if (method == "encapsulate" && role == Some(1))
                        || (method == "decapsulate" && role == Some(0))
                    {
                        facts.callee("PQC_RUST_API", node, "local_ml_kem_receiver_provenance");
                    }
                }
            }
            "scoped_identifier" | "scoped_type_identifier" => {
                examined += 1;
                let mut parent = node.parent();
                let mut in_import = false;
                let mut nested = false;
                for _ in 0..MAX_DEPTH {
                    let Some(p) = parent else {
                        break;
                    };
                    if p.kind() == "use_declaration" {
                        in_import = true;
                        break;
                    }
                    if p.kind() == "scoped_identifier" || p.kind() == "scoped_type_identifier" {
                        nested = true;
                    }
                    parent = p.parent();
                }
                if !in_import && !nested && external(ctx, node) {
                    facts.node("PQC_RUST_API", node, "explicit_rust_crate_symbol");
                }
            }
            "ERROR" => {
                examined += 1;
                let raw = ctx.source(node);
                if raw.len() > ctx.budget.max_ast_match_bytes {
                    facts.truncated = true;
                    continue;
                }
                if let Some(c) = incomplete.captures(raw) {
                    let name = c.get(1).unwrap();
                    if ctx.code_at(node.start_byte() + name.start())
                        && ctx.lookup(node, name.as_str()).is_none()
                    {
                        facts.push(
                            "PQC_RUST_API",
                            node.start_byte() + name.start(),
                            node.end_byte(),
                            "bounded_incomplete_rust_import",
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
