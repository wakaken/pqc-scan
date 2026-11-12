use super::{
    context::{children, pattern_names, Context, MAX_DEPTH},
    Facts,
};
use tree_sitter::Node;
fn canonical(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> Option<String> {
    if depth > MAX_DEPTH {
        ctx.truncated.set(true);
        return None;
    }
    if node.kind() == "attribute" {
        let object = canonical(ctx, node.child_by_field_name("object")?, depth + 1)?;
        return Some(format!(
            "{object}.{}",
            ctx.source(node.child_by_field_name("attribute")?)
        ));
    }
    if node.kind() != "identifier" {
        return None;
    }
    let name = ctx.source(node);
    if let Some(b) = ctx.lookup(node, name) {
        if !ctx.unchanged(b) {
            return None;
        }
        return b
            .imported
            .as_deref()?
            .strip_prefix("python:")
            .map(str::to_string);
    }
    Some(name.into())
}
fn generation(name: &str, pattern: &regex::Regex) -> bool {
    super::exact_match(pattern, &format!("{name}("))
}
pub(super) fn inspect<'a>(
    ctx: &mut Context<'a, '_>,
    facts: &mut Facts,
    rules: &pqc_scan_rules::RuleSet,
) {
    let Some(pattern) = rules
        .get("PQC_PYTHON_API")
        .and_then(|r| r.compiled_pattern())
    else {
        return;
    };
    let nodes = ctx.nodes.clone();
    for node in &nodes {
        let scope = ctx.scope(*node);
        match node.kind() {
            "import_from_statement" => {
                let Some(module) = node.child_by_field_name("module_name") else {
                    continue;
                };
                let module = ctx.source(module);
                let trusted = module == "cryptography.hazmat.primitives.asymmetric"
                    || module == "cryptography.hazmat.primitives.asymmetric.mlkem"
                    || module == "cryptography.hazmat.primitives.asymmetric.mldsa";
                let mut recognized = false;
                for name in children(*node).into_iter().filter(|n| {
                    n.start_byte() > node.child_by_field_name("module_name").unwrap().end_byte()
                }) {
                    let (original, alias) = if name.kind() == "aliased_import" {
                        let Some(original) = name.child_by_field_name("name") else {
                            continue;
                        };
                        (
                            original,
                            name.child_by_field_name("alias").unwrap_or(original),
                        )
                    } else {
                        (name, name)
                    };
                    let original_name = ctx.source(original);
                    let canonical = if module.ends_with(".mlkem") {
                        format!("mlkem.{original_name}")
                    } else if module.ends_with(".mldsa") {
                        format!("mldsa.{original_name}")
                    } else {
                        original_name.into()
                    };
                    let known = trusted
                        && (matches!(canonical.as_str(), "mlkem" | "mldsa")
                            || generation(&format!("{canonical}.generate"), pattern));
                    recognized |= known;
                    ctx.add(
                        scope,
                        ctx.source(alias),
                        None,
                        None,
                        known.then(|| format!("python:{canonical}")),
                        alias.id(),
                    );
                }
                if recognized {
                    facts.node("PQC_PYTHON_API", *node, "explicit_python_pqc_import");
                }
            }
            "assignment" | "augmented_assignment" | "for_statement" => {
                if let Some(left) = node.child_by_field_name("left") {
                    for (name, _) in pattern_names(left, ctx.text, 0) {
                        ctx.add(
                            scope,
                            &name,
                            node.child_by_field_name("right"),
                            None,
                            None,
                            left.id(),
                        );
                    }
                }
            }
            "import_statement" => {
                for import in children(*node) {
                    let name = import
                        .child_by_field_name("alias")
                        .or_else(|| import.named_child(0))
                        .unwrap_or(import);
                    ctx.add(scope, ctx.source(name), None, None, None, name.id());
                }
            }
            "as_pattern" | "except_clause" => {
                if let Some(alias) = node.child_by_field_name("alias") {
                    for (name, _) in pattern_names(alias, ctx.text, 0) {
                        ctx.add(scope, &name, None, None, None, alias.id());
                    }
                }
            }
            "parameters" | "lambda_parameters" => {
                for (name, _) in pattern_names(*node, ctx.text, 0) {
                    ctx.add(scope, &name, None, None, None, node.id());
                }
            }
            "function_definition" => {
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
            "class_definition" => {
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
            _ => {}
        }
    }
    let mut examined = 0;
    for node in nodes {
        if !matches!(node.kind(), "call" | "ERROR") {
            continue;
        }
        if examined == 4096 {
            facts.truncated = true;
            break;
        }
        examined += 1;
        if node.kind() == "call" {
            if !ctx.unambiguous_string_context(node.start_byte()) {
                continue;
            }
            if node
                .child_by_field_name("function")
                .and_then(|n| canonical(ctx, n, 0))
                .is_some_and(|name| generation(&name, pattern))
            {
                facts.node(
                    "PQC_PYTHON_API",
                    node,
                    "explicit_python_generation_reference",
                );
            }
        } else {
            let raw = ctx.source(node);
            let boundary = raw.char_indices().nth(2000).map_or(raw.len(), |(i, _)| i);
            facts.truncated |= boundary < raw.len();
            for hit in pattern.find_iter(&raw[..boundary]).take(4096) {
                let start = node.start_byte() + hit.start();
                let root = hit.as_str().split('.').next().unwrap_or("");
                let shadowed = ctx.lookup(node, root).is_some_and(|b| b.imported.is_none());
                if ctx.code_at(start) && !shadowed {
                    facts.push(
                        "PQC_PYTHON_API",
                        start,
                        node.start_byte() + hit.end(),
                        "bounded_syntax_recovery",
                    );
                }
            }
        }
    }
}
