//! Explicit jsonwebtoken signing options, never inferred from nearby literals.
use super::{
    context::{children, literal, unwrap_expression, Context, MAX_DEPTH},
    Facts,
};
use std::collections::BTreeSet;
use tree_sitter::Node;
const ID: &str = "TS_NODE_JOSE_USAGE";

fn namespace(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> bool {
    namespace_origin(ctx, node, depth, true)
}
fn namespace_origin(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize, unchanged: bool) -> bool {
    if depth > MAX_DEPTH {
        ctx.limit("source_alias_depth", MAX_DEPTH, MAX_DEPTH);
        return false;
    }
    let node = unwrap_expression(node);
    if node.kind() == "identifier" {
        let Some(b) = ctx
            .lookup(node, ctx.source(node))
            .filter(|b| !unchanged || ctx.unchanged(b))
        else {
            return false;
        };
        return b.imported.as_deref() == Some("@jsonwebtoken")
            || b.projection.is_none()
                && b.value.is_some_and(|v| {
                    v.start_byte() < node.start_byte()
                        && namespace_origin(ctx, v, depth + 1, unchanged)
                });
    }
    if node.kind() != "call_expression" {
        return false;
    }
    let Some(function) = node.child_by_field_name("function") else {
        return false;
    };
    ctx.source(function) == "require"
        && ctx.lookup(function, "require").is_none()
        && node.child_by_field_name("arguments").is_some_and(|args| {
            args.named_child_count() == 1
                && args.named_child(0).and_then(|n| literal(ctx.text, n)) == Some("jsonwebtoken")
        })
}
// An alias write must not disappear merely because the write itself made that
// alias "changed". Follow possible origins without the unchanged filter here.
// Namespace escape into containers/calls/returns is conservatively unresolved.
pub(super) fn namespace_may_change(ctx: &Context<'_, '_>) -> bool {
    for &node in &ctx.nodes {
        if !matches!(node.kind(), "identifier" | "call_expression")
            || !namespace_origin(ctx, node, 0, false)
        {
            continue;
        }
        let Some(parent) = node.parent() else {
            return true;
        };
        if matches!(parent.kind(), "namespace_import" | "import_clause") {
            continue;
        }
        if parent.kind() == "variable_declarator"
            && parent
                .child_by_field_name("value")
                .is_some_and(|n| n.id() == node.id())
        {
            continue;
        }
        if !matches!(parent.kind(), "member_expression" | "subscript_expression")
            || parent
                .child_by_field_name("object")
                .is_none_or(|n| n.id() != node.id())
        {
            return true;
        }
        let mut access = parent;
        for depth in 0..=MAX_DEPTH {
            let Some(owner) = access.parent() else {
                return true;
            };
            if matches!(owner.kind(), "member_expression" | "subscript_expression")
                && owner
                    .child_by_field_name("object")
                    .is_some_and(|n| n.id() == access.id())
            {
                if depth == MAX_DEPTH {
                    ctx.limit("source_alias_depth", MAX_DEPTH, MAX_DEPTH);
                    return true;
                }
                access = owner;
                continue;
            }
            if matches!(
                owner.kind(),
                "assignment_expression" | "augmented_assignment_expression"
            ) && owner
                .child_by_field_name("left")
                .is_some_and(|n| n.id() == access.id())
                || owner.kind() == "update_expression"
                || owner.kind() == "unary_expression"
                    && owner
                        .child_by_field_name("operator")
                        .is_some_and(|n| ctx.source(n) == "delete")
            {
                return true;
            }
            break;
        }
    }
    ctx.truncated.get()
}
pub(super) fn signing(ctx: &Context<'_, '_>, node: Node<'_>, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        ctx.limit("source_alias_depth", MAX_DEPTH, MAX_DEPTH);
        return false;
    }
    let node = unwrap_expression(node);
    if node.kind() == "member_expression" {
        return node
            .child_by_field_name("property")
            .is_some_and(|n| ctx.source(n) == "sign")
            && node
                .child_by_field_name("object")
                .is_some_and(|n| namespace(ctx, n, depth + 1));
    }
    if node.kind() != "identifier" {
        return false;
    }
    let Some(b) = ctx
        .lookup(node, ctx.source(node))
        .filter(|b| ctx.unchanged(b))
    else {
        return false;
    };
    b.imported.as_deref() == Some("jwt:sign")
        || b.projection.is_none()
            && b.value
                .is_some_and(|v| v.start_byte() < node.start_byte() && signing(ctx, v, depth + 1))
}
pub(super) fn property<'a, 's>(
    ctx: &Context<'a, 's>,
    object: Node<'a>,
    wanted: &str,
) -> Option<Option<Node<'a>>> {
    if object.kind() != "object" {
        return None;
    }
    let mut seen = BTreeSet::new();
    let mut result = None;
    for pair in children(object) {
        if pair.kind() == "comment" {
            continue;
        }
        // Spreads, computed keys, accessors and shorthand can overwrite a
        // property or hide executable behavior. Do not guess their effects.
        if pair.kind() != "pair" {
            return None;
        }
        let key = pair.child_by_field_name("key")?;
        let name = if key.kind() == "property_identifier" {
            ctx.source(key)
        } else {
            literal(ctx.text, key)?
        };
        if !seen.insert(name) {
            return None;
        }
        if name == wanted {
            result = Some(pair.child_by_field_name("value")?);
        }
    }
    Some(result)
}
fn algorithm(ctx: &Context<'_, '_>, options: Node<'_>) -> Option<String> {
    let name = literal(ctx.text, property(ctx, options, "algorithm")??)?;
    if !matches!(
        name,
        "RS256" | "RS384" | "RS512" | "ES256" | "ES384" | "ES512" | "EdDSA"
    ) {
        return None;
    }
    // jsonwebtoken's header can override the algorithm option. Require its
    // explicit value to agree and reject an unresolved or mutating header.
    if let Some(header) = property(ctx, options, "header")? {
        if let Some(alg) = property(ctx, header, "alg")? {
            if literal(ctx.text, alg) != Some(name) {
                return None;
            }
        }
    }
    Some(name.into())
}
pub(super) fn inspect(ctx: &Context<'_, '_>, facts: &mut Facts) {
    if namespace_may_change(ctx) {
        return;
    }
    let mut remaining_bytes = ctx.budget.max_ast_match_bytes;
    for (examined, &node) in ctx
        .nodes
        .iter()
        .filter(|n| n.kind() == "call_expression")
        .enumerate()
    {
        if examined >= ctx.budget.max_match_candidates {
            facts.truncated = true;
            break;
        }
        let Some(function) = node.child_by_field_name("function") else {
            continue;
        };
        if !signing(ctx, function, 0) {
            continue;
        }
        let Some(args) = node.child_by_field_name("arguments") else {
            continue;
        };
        if args.named_child_count() != 3 {
            continue;
        }
        let cost = args.end_byte() - args.start_byte();
        if cost > remaining_bytes {
            facts.truncated = true;
            break;
        }
        remaining_bytes -= cost;
        let Some(name) = args.named_child(2).and_then(|n| algorithm(ctx, n)) else {
            continue;
        };
        let next = facts.hits.get(ID).map_or(0, Vec::len);
        facts.callee(ID, node, "explicit_jsonwebtoken_signing_options");
        if let Some(hit) = facts.hits.get_mut(ID).and_then(|hits| hits.get_mut(next)) {
            hit.algorithm_references.push(name);
        }
    }
}
