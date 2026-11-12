use super::{
    context::{children, literal, pattern_names, unwrap_expression, Context, MAX_DEPTH},
    Facts,
};
use tree_sitter::Node;

fn api(name: &str) -> bool {
    matches!(
        name,
        "generateKeyPair"
            | "generateKeyPairSync"
            | "createSign"
            | "createVerify"
            | "privateEncrypt"
            | "publicDecrypt"
    )
}
fn crypto_module(ctx: &Context<'_, '_>, node: Node<'_>) -> bool {
    let node = unwrap_expression(node);
    if node.kind() != "call_expression" {
        return false;
    }
    let Some(function) = node.child_by_field_name("function") else {
        return false;
    };
    let name = ctx.source(function);
    if !matches!(name, "require" | "import")
        || (name == "require" && ctx.lookup(function, name).is_some())
    {
        return false;
    }
    node.child_by_field_name("arguments")
        .and_then(|n| n.named_child(0))
        .and_then(|n| literal(ctx.text, n))
        .is_some_and(|s| matches!(s, "crypto" | "node:crypto"))
}
fn namespace(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return false;
    }
    let node = unwrap_expression(node);
    if node.kind() != "identifier" {
        return crypto_module(ctx, node);
    }
    let Some(b) = ctx.lookup(node, ctx.source(node)) else {
        return false;
    };
    if !ctx.unchanged(b) {
        return false;
    }
    if b.imported.as_deref() == Some("@crypto") {
        return true;
    }
    b.value.is_some_and(|v| {
        v.start_byte() < node.start_byte()
            && (crypto_module(ctx, v) || namespace(ctx, v, depth + 1))
    })
}
fn canonical(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> Option<String> {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return None;
    }
    let node = unwrap_expression(node);
    if node.kind() == "member_expression" {
        let object = node.child_by_field_name("object")?;
        let name = ctx.source(node.child_by_field_name("property")?);
        return (namespace(ctx, object, depth + 1) && api(name)).then(|| name.to_string());
    }
    if node.kind() != "identifier" {
        return None;
    }
    let name = ctx.source(node);
    let Some(b) = ctx.lookup(node, name) else {
        return api(name).then(|| name.to_string());
    };
    if !ctx.unchanged(b) {
        return None;
    }
    if let Some(imported) = &b.imported {
        return imported
            .strip_prefix("api:")
            .filter(|n| api(n))
            .map(str::to_string);
    }
    let value = b.value?;
    if value.start_byte() >= node.start_byte() {
        return None;
    }
    if crypto_module(ctx, value) {
        return b
            .projection
            .as_deref()
            .filter(|n| api(n))
            .map(str::to_string);
    }
    if b.projection.is_some() {
        return None;
    }
    canonical(ctx, value, depth + 1)
}
fn scalar_values(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> Option<Vec<String>> {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return None;
    }
    let node = unwrap_expression(node);
    if let Some(value) = literal(ctx.text, node) {
        if value.len() > 256 {
            return None;
        }
        return Some(vec![value.into()]);
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
    if b.imported.as_deref() == Some("@loop") {
        let values = unwrap_expression(value);
        let values = if values.kind() == "identifier" {
            let bound = ctx.lookup(values, ctx.source(values))?;
            if !ctx.unchanged(bound) {
                return None;
            }
            unwrap_expression(bound.value?)
        } else {
            values
        };
        if values.kind() != "array" {
            return None;
        }
        let items = array_elements(values)?;
        if items.len() > 32 {
            ctx.truncated.set(true);
            return None;
        }
        let projection = b
            .projection
            .as_deref()
            .map(str::parse::<usize>)
            .transpose()
            .ok()?;
        let mut result = Vec::new();
        for item in items {
            let item = if let Some(i) = projection {
                if item.kind() != "array" {
                    return None;
                }
                *array_elements(item)?.get(i)?
            } else {
                item
            };
            if literal(ctx.text, item)?.len() > 256 {
                return None;
            }
            result.push(literal(ctx.text, item)?.to_string());
        }
        return (!result.is_empty()).then_some(result);
    }
    if b.projection.is_some() {
        return None;
    }
    scalar_values(ctx, value, depth + 1)
}
fn array_elements(node: Node<'_>) -> Option<Vec<Node<'_>>> {
    let mut result = Vec::new();
    let mut present = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor).take(4096) {
        if child.kind() == "," {
            if !present {
                return None;
            }
            present = false;
        }
        if child.is_named() && !child.kind().contains("comment") {
            result.push(child);
            present = true;
        }
    }
    Some(result)
}
fn selected_argument(rule: Option<&pqc_scan_rules::Rule>, callee: &str, value: &str) -> bool {
    let Some(pattern) = rule.and_then(|r| r.compiled_pattern()) else {
        return false;
    };
    ['\'', '"']
        .into_iter()
        .any(|quote| super::exact_match(pattern, &format!("{callee}({quote}{value}{quote}")))
}
fn arguments(node: Node<'_>) -> Vec<Node<'_>> {
    node.child_by_field_name("arguments")
        .map_or_else(Vec::new, children)
}
fn generated_rsa(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return false;
    }
    let node = unwrap_expression(node);
    if node.kind() != "call_expression" {
        return false;
    }
    let Some(function) = node.child_by_field_name("function") else {
        return false;
    };
    let Some(name) = canonical(ctx, function, depth + 1) else {
        return false;
    };
    name == "generateKeyPairSync"
        && arguments(node)
            .first()
            .and_then(|n| scalar_values(ctx, *n, depth + 1))
            .is_some_and(|v| v == ["rsa"])
}
fn rsa_key(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return false;
    }
    let node = unwrap_expression(node);
    if node.kind() == "member_expression" {
        let Some(property) = node.child_by_field_name("property") else {
            return false;
        };
        if !matches!(ctx.source(property), "privateKey" | "publicKey") {
            return false;
        }
        return node
            .child_by_field_name("object")
            .and_then(|o| ctx.lookup(o, ctx.source(o)))
            .filter(|b| ctx.unchanged(b))
            .and_then(|b| b.value)
            .is_some_and(|v| generated_rsa(ctx, v, depth + 1));
    }
    if node.kind() != "identifier" {
        return false;
    }
    let Some(b) = ctx.lookup(node, ctx.source(node)) else {
        return false;
    };
    if !ctx.unchanged(b) {
        return false;
    }
    let Some(value) = b.value.filter(|v| v.start_byte() < node.start_byte()) else {
        return false;
    };
    if b.projection
        .as_deref()
        .is_some_and(|p| matches!(p, "privateKey" | "publicKey"))
    {
        return generated_rsa(ctx, value, depth + 1);
    }
    if b.projection.is_none() {
        return rsa_key(ctx, value, depth + 1);
    }
    false
}
fn signer<'a>(
    ctx: &Context<'a, '_>,
    node: Node<'a>,
    method: &str,
    depth: usize,
) -> Option<Node<'a>> {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return None;
    }
    let b = ctx.lookup(node, ctx.source(node))?;
    if !ctx.unchanged(b) {
        return None;
    }
    let value = unwrap_expression(b.value?);
    if value.start_byte() >= node.start_byte() {
        return None;
    }
    if value.kind() == "identifier" {
        return signer(ctx, value, method, depth + 1);
    }
    if value.kind() != "call_expression" {
        return None;
    }
    let function = value.child_by_field_name("function")?;
    let name = canonical(ctx, function, depth + 1)?;
    ((method == "sign" && name == "createSign") || (method == "verify" && name == "createVerify"))
        .then_some(value)
}
pub(super) fn inspect<'a>(
    ctx: &mut Context<'a, '_>,
    facts: &mut Facts,
    rules: &pqc_scan_rules::RuleSet,
) {
    let nodes = ctx.nodes.clone();
    for node in &nodes {
        let scope = ctx.scope(*node);
        match node.kind() {
            "import_statement" => {
                let trusted = node
                    .child_by_field_name("source")
                    .and_then(|n| literal(ctx.text, n))
                    .is_some_and(|s| matches!(s, "crypto" | "node:crypto"));
                let first = nodes.partition_point(|n| n.start_byte() < node.start_byte());
                let last = nodes.partition_point(|n| n.start_byte() < node.end_byte());
                for n in nodes[first..last]
                    .iter()
                    .filter(|n| n.end_byte() <= node.end_byte())
                {
                    match n.kind() {
                        "import_specifier" => {
                            let Some(name) = n.child_by_field_name("name") else {
                                continue;
                            };
                            let alias = n.child_by_field_name("alias").unwrap_or(name);
                            ctx.add(
                                scope,
                                ctx.source(alias),
                                None,
                                None,
                                trusted.then(|| format!("api:{}", ctx.source(name))),
                                alias.id(),
                            );
                        }
                        "namespace_import" => {
                            if let Some(name) = n.named_child(0) {
                                ctx.add(
                                    scope,
                                    ctx.source(name),
                                    None,
                                    None,
                                    trusted.then(|| "@crypto".into()),
                                    name.id(),
                                );
                            }
                        }
                        "import_clause" => {
                            for name in children(*n)
                                .into_iter()
                                .filter(|n| n.kind() == "identifier")
                            {
                                ctx.add(
                                    scope,
                                    ctx.source(name),
                                    None,
                                    None,
                                    trusted.then(|| "@crypto".into()),
                                    name.id(),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            "variable_declarator" => {
                let Some(pattern) = node.child_by_field_name("name") else {
                    continue;
                };
                let value = node.child_by_field_name("value");
                let scope = if node
                    .parent()
                    .is_some_and(|p| p.kind() == "variable_declaration")
                {
                    ctx.function_scope(scope)
                } else {
                    scope
                };
                for (name, projection) in pattern_names(pattern, ctx.text, 0) {
                    let projection = if pattern.kind() == "object_pattern" {
                        Some(projection.unwrap_or_else(|| name.clone()))
                    } else {
                        projection
                    };
                    ctx.add(scope, &name, value, projection, None, pattern.id());
                }
            }
            "function_declaration" | "generator_function_declaration" => {
                if let Some(name) = node.child_by_field_name("name") {
                    ctx.add(
                        ctx.parent_scope(*node),
                        ctx.source(name),
                        None,
                        None,
                        None,
                        name.id(),
                    );
                }
            }
            "function_expression" | "generator_function" | "class_declaration" => {
                if let Some(name) = node.child_by_field_name("name") {
                    ctx.add(scope, ctx.source(name), None, None, None, name.id());
                }
            }
            "formal_parameters" => {
                for (name, _) in pattern_names(*node, ctx.text, 0) {
                    ctx.add(scope, &name, None, None, None, node.id());
                }
            }
            "arrow_function" => {
                if let Some(name) = node.child_by_field_name("parameter") {
                    ctx.add(scope, ctx.source(name), None, None, None, name.id());
                }
            }
            "catch_clause" => {
                if let Some(name) = node.child_by_field_name("parameter") {
                    for (name, _) in pattern_names(name, ctx.text, 0) {
                        ctx.add(scope, &name, None, None, None, node.id());
                    }
                }
            }
            "for_in_statement" => {
                if let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) {
                    let is_of = node
                        .child_by_field_name("operator")
                        .is_some_and(|op| ctx.source(op) == "of");
                    for (name, projection) in pattern_names(left, ctx.text, 0) {
                        ctx.add(
                            scope,
                            &name,
                            is_of.then_some(right),
                            projection,
                            is_of.then(|| "@loop".into()),
                            left.id(),
                        );
                    }
                }
            }
            _ => {}
        }
    }
    for node in &nodes {
        if matches!(
            node.kind(),
            "assignment_expression" | "augmented_assignment_expression" | "update_expression"
        ) {
            if let Some(left) = node
                .child_by_field_name("left")
                .or_else(|| node.child_by_field_name("argument"))
            {
                for (name, _) in pattern_names(left, ctx.text, 0) {
                    ctx.mark_write(left, &name);
                }
                if left.kind() == "member_expression" {
                    if let Some(object) = left.child_by_field_name("object") {
                        ctx.mark_write(object, ctx.source(object));
                    }
                }
            }
        }
    }
    for (examined, node) in nodes
        .iter()
        .filter(|n| n.kind() == "call_expression")
        .enumerate()
    {
        if examined == 4096 {
            facts.truncated = true;
            break;
        }
        let Some(function) = node.child_by_field_name("function") else {
            continue;
        };
        if let Some(name) = canonical(ctx, function, 0) {
            let args = arguments(*node);
            if matches!(name.as_str(), "generateKeyPair" | "generateKeyPairSync") {
                if let Some(values) = args.first().and_then(|n| scalar_values(ctx, *n, 0)) {
                    if values
                        .iter()
                        .any(|v| selected_argument(rules.get("PQC_NODE_API"), &name, v))
                    {
                        facts.call(
                            "PQC_NODE_API",
                            *node,
                            "bounded_import_scope_and_literal_set",
                        );
                    }
                    if values == ["rsa"] {
                        facts.call("TS_NODE_RSA_USAGE", *node, "explicit_rsa_generation");
                    }
                }
            }
            if matches!(name.as_str(), "createSign" | "createVerify")
                && args
                    .first()
                    .and_then(|n| literal(ctx.text, *n))
                    .is_some_and(|s| s.starts_with("RSA"))
            {
                facts.call("API_NODE_CRYPTO_SIGN_RSA", *node, "explicit_rsa_factory");
            }
        }
        if function.kind() != "member_expression" {
            continue;
        }
        let (Some(object), Some(property)) = (
            function.child_by_field_name("object"),
            function.child_by_field_name("property"),
        ) else {
            continue;
        };
        let method = ctx.source(property);
        if !matches!(method, "sign" | "verify") {
            continue;
        }
        let Some(factory) = signer(ctx, object, method, 0) else {
            continue;
        };
        if arguments(*node)
            .first()
            .is_some_and(|key| rsa_key(ctx, *key, 0))
        {
            facts.call("TS_NODE_RSA_USAGE", factory, "local_rsa_key_provenance");
            facts.call(
                "API_NODE_CRYPTO_SIGN_RSA",
                factory,
                "local_rsa_key_provenance",
            );
            facts.call("TS_NODE_RSA_USAGE", *node, "local_rsa_key_provenance");
            facts.call(
                "API_NODE_CRYPTO_SIGN_RSA",
                *node,
                "local_rsa_key_provenance",
            );
        }
    }
}
