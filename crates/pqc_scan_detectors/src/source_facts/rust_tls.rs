//! Standard OpenSSL Rust API references, independent of wrapper/product names.
use super::{
    context::{children, Context, MAX_DEPTH},
    Facts,
};
use tree_sitter::Node;
const ID: &str = "API_RUST_OPENSSL_TLS";
fn recognized(path: &str) -> bool {
    matches!(
        path,
        "openssl::ssl::SslConnector"
            | "openssl::ssl::SslAcceptor"
            | "openssl::ssl::SslContext"
            | "openssl::ssl::SslMethod"
    )
}
fn imports(ctx: &mut Context<'_, '_>, node: Node<'_>, prefix: &str, scope: usize, depth: usize) {
    if depth > MAX_DEPTH {
        ctx.limit("source_alias_depth", MAX_DEPTH, MAX_DEPTH);
        return;
    }
    let join = |name: &str| {
        if prefix.is_empty() {
            name.trim_start_matches("::").to_owned()
        } else {
            format!("{prefix}::{name}")
        }
    };
    match node.kind() {
        "scoped_use_list" => {
            let prefix = node
                .child_by_field_name("path")
                .map_or(prefix.to_owned(), |n| join(ctx.source(n)));
            if let Some(list) = node.child_by_field_name("list") {
                for n in children(list) {
                    imports(ctx, n, &prefix, scope, depth + 1);
                }
            }
        }
        "use_as_clause" => {
            if let (Some(path), Some(alias)) = (
                node.child_by_field_name("path"),
                node.child_by_field_name("alias"),
            ) {
                let path = join(ctx.source(path));
                ctx.add_hoisted(
                    scope,
                    ctx.source(alias),
                    Some(format!("tls:{path}")),
                    alias.id(),
                );
            }
        }
        "scoped_identifier" | "identifier" => {
            let path = join(ctx.source(node));
            let name = path.rsplit("::").next().unwrap();
            ctx.add_hoisted(scope, name, Some(format!("tls:{path}")), node.id());
        }
        _ => {}
    }
}
fn canonical(ctx: &Context<'_, '_>, node: Node<'_>) -> Option<String> {
    let raw = ctx.source(node);
    let absolute = raw.starts_with("::");
    let raw = raw.trim_start_matches("::");
    if recognized(raw) && (absolute || ctx.lookup(node, "openssl").is_none()) {
        return Some(raw.to_owned());
    }
    if absolute || !matches!(node.kind(), "identifier" | "scoped_identifier") {
        return None;
    }
    // `use openssl::ssl as tls; tls::SslMethod::tls()` is the same
    // statically imported API, not a product-specific wrapper heuristic.
    let (name, suffix) = raw.split_once("::").unwrap_or((raw, ""));
    let binding = ctx.lookup(node, name)?;
    let path = ctx
        .unchanged(binding)
        .then_some(binding)?
        .imported
        .as_deref()?
        .strip_prefix("tls:")?;
    let path = if suffix.is_empty() {
        path.to_owned()
    } else {
        format!("{path}::{suffix}")
    };
    recognized(&path).then_some(path)
}
pub(super) fn inspect(ctx: &mut Context<'_, '_>, facts: &mut Facts) {
    let nodes = ctx.nodes.clone();
    for &node in &nodes {
        if matches!(node.kind(), "type_item" | "const_item" | "static_item") {
            if let Some(name) = node.child_by_field_name("name") {
                ctx.add_hoisted(ctx.scope(node), ctx.source(name), None, name.id());
            }
        }
    }
    for &node in &nodes {
        if node.kind() == "use_declaration" {
            if let Some(arg) = node.child_by_field_name("argument") {
                let root = ctx
                    .source(arg)
                    .trim_start_matches("::")
                    .split("::")
                    .next()
                    .unwrap_or("");
                if ctx.source(arg).starts_with("::")
                    || root != "openssl"
                    || ctx.lookup(arg, "openssl").is_none()
                {
                    imports(ctx, arg, "", ctx.scope(node), 0);
                }
            }
        }
    }
    for &node in &nodes {
        if node.kind() != "call_expression" {
            continue;
        }
        let Some(function) = node.child_by_field_name("function") else {
            continue;
        };
        if function.kind() != "scoped_identifier" {
            continue;
        }
        let (Some(path), Some(name)) = (
            function.child_by_field_name("path"),
            function.child_by_field_name("name"),
        ) else {
            continue;
        };
        let Some(path) = canonical(ctx, path) else {
            continue;
        };
        let method = ctx.source(name);
        let selected = match path.as_str() {
            "openssl::ssl::SslConnector" => method == "builder",
            "openssl::ssl::SslAcceptor" => matches!(
                method,
                "mozilla_intermediate"
                    | "mozilla_modern"
                    | "mozilla_intermediate_v5"
                    | "mozilla_modern_v5"
            ),
            "openssl::ssl::SslContext" => method == "builder",
            "openssl::ssl::SslMethod" => matches!(method, "tls" | "tls_client" | "tls_server"),
            _ => false,
        };
        if selected {
            facts.push(
                ID,
                function.start_byte(),
                function.end_byte(),
                "rust_openssl_tls_api_reference_no_negotiated_algorithm",
            );
        }
    }
}
