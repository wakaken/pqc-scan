//! Conservative display-text classification. No evaluation of program values.
use super::resolve_language;
use pqc_scan_core::ScannableFile;
use std::ops::Range;
use tree_sitter::{Node, Parser};

pub(super) fn ranges(file: &ScannableFile, max_nodes: usize) -> Vec<Range<usize>> {
    let Some(text) = file.text.as_deref() else {
        return vec![];
    };
    if ![
        "RS256", "RS384", "RS512", "RSA", "ES256", "ES384", "ES512", "EdDSA", "AES", "TLS",
    ]
    .iter()
    .any(|s| text.contains(s))
    {
        return vec![];
    }
    let mut owned = None;
    let (language, language_name) = if file.ext().is_some_and(|e| e.eq_ignore_ascii_case("vue")) {
        // One inline script only; retain original byte positions. External and
        // multiple scripts are intentionally left unclassified.
        let Some(start) = text.find("<script") else {
            return vec![];
        };
        let Some(open) = text[start..].find('>').map(|i| start + i + 1) else {
            return vec![];
        };
        let Some(end) = text[open..].find("</script>").map(|i| open + i) else {
            return vec![];
        };
        if text[open..].contains("<script") || text[start..open].contains("src=") {
            return vec![];
        }
        let mut bytes = text.as_bytes().to_vec();
        for (i, b) in bytes.iter_mut().enumerate() {
            if (i < open || i >= end) && *b != b'\n' && *b != b'\r' {
                *b = b' ';
            }
        }
        owned = String::from_utf8(bytes).ok();
        (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            "typescript",
        )
    } else {
        let Some((lang, name)) = resolve_language(&file.path, text) else {
            return vec![];
        };
        (lang, name)
    };
    let source = owned.as_deref().unwrap_or(text);
    let mut parser = Parser::new();
    if parser.set_language(&language).is_err() {
        return vec![];
    }
    let Some(tree) = parser.parse(source, None) else {
        return vec![];
    };
    let partial_tree = tree.root_node().has_error();
    let mut stack = vec![tree.root_node()];
    let mut count = 0;
    let mut out = vec![];
    let mut shadowed = std::collections::BTreeSet::new();
    let mut trusted_go = std::collections::BTreeSet::new();
    let mut strings = Vec::new();
    while let Some(node) = stack.pop() {
        count += 1;
        if count > max_nodes.min(200_000) {
            return vec![];
        }
        if node.kind() == "ERROR" || node.is_missing() {
            continue;
        }
        if node.kind().contains("comment") {
            out.push(node.byte_range());
            continue;
        }
        if matches!(
            node.kind(),
            "variable_declarator" | "init_declarator" | "enumerator"
        ) {
            if let Some(name) = node
                .child_by_field_name("name")
                .or_else(|| node.child_by_field_name("declarator"))
                .filter(|n| matches!(n.kind(), "identifier" | "field_identifier"))
            {
                let token = &source[name.byte_range()];
                if token.starts_with("TLS_")
                    && token.contains("_WITH_")
                    && token.len() <= 128
                    && token
                        .bytes()
                        .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
                {
                    out.push(name.byte_range());
                }
            }
        }
        // Java imports/throws name exception types, not callable local values.
        // The Java case below still requires a literal directly in `throw new`.
        if language_name == "java" && matches!(node.kind(), "import_declaration" | "throws") {
            continue;
        }
        // Any visible non-callee use may introduce, rename, shadow or escape
        // an error name. This deliberately over-approximates bindings instead
        // of assuming that an unlisted loop/pattern/alias syntax is unbound.
        if (node.kind().contains("identifier") || node.kind() == "constant")
            && !callee_reference(node)
        {
            shadowed.insert(source[node.byte_range()].to_string());
        }
        if node.kind() == "import_spec" && node.child_by_field_name("name").is_none() {
            if let Some(path) = node.child_by_field_name("path") {
                let name = source[path.byte_range()].trim_matches(['"', '`']);
                if matches!(name, "errors" | "fmt") {
                    trusted_go.insert(name.to_string());
                }
            }
        }
        if node.kind() == "attribute_item" && source[node.byte_range()].contains("macro_use") {
            shadowed.insert("*".into());
        }
        if matches!(
            node.kind(),
            "variable_declarator"
                | "function_declaration"
                | "function_definition"
                | "method"
                | "singleton_method"
                | "function_item"
                | "let_declaration"
                | "short_var_declaration"
                | "var_spec"
                | "assignment_statement"
                | "enum_declaration"
                | "catch_clause"
                | "class_declaration"
                | "class_definition"
                | "macro_definition"
                | "parameter"
                | "required_parameter"
                | "parameter_declaration"
                | "variadic_parameter_declaration"
                | "type_spec"
                | "mod_item"
                | "assignment"
                | "assignment_expression"
        ) {
            if let Some(binding) = node
                .child_by_field_name("name")
                .or_else(|| node.child_by_field_name("left"))
                .or_else(|| node.child_by_field_name("pattern"))
                .or_else(|| node.child_by_field_name("parameter"))
            {
                if !binding_names(binding, source, &mut shadowed) {
                    return vec![];
                }
            }
        }
        if matches!(
            node.kind(),
            "parameters"
                | "method_parameters"
                | "formal_parameters"
                | "import_clause"
                | "import_specifier"
                | "import_from_statement"
                | "import_statement"
                | "import_declaration"
                | "import_spec"
                | "use_declaration"
        ) && !binding_names(node, source, &mut shadowed)
        {
            return vec![];
        }
        if matches!(
            node.kind(),
            "string" | "string_literal" | "interpreted_string_literal" | "raw_string_literal"
        ) {
            let value = &source[node.byte_range()];
            let lower = value.to_ascii_lowercase();
            let phrase = [
                "must be ",
                "not supported",
                "unsupported ",
                "invalid ",
                "unable to ",
                "failed ",
                "expected ",
            ]
            .iter()
            .any(|s| lower.contains(s));
            let plain = value
                .as_bytes()
                .first()
                .is_some_and(|b| matches!(b, b'\'' | b'"' | b'`'))
                && !value.contains("#{")
                && !value.contains("${");
            // An RFC-style Received header records transport history. The
            // literal itself does not select a cipher, even inside test code.
            if plain && value.len() <= 4096 {
                let body = value.trim_start_matches(['\'', '"', '`']).trim_start();
                let unfolded = body
                    .replace("\\r", " ")
                    .replace("\\n", " ")
                    .replace("\\t", " ");
                let normalized = unfolded.split_whitespace().collect::<Vec<_>>().join(" ");
                if body.starts_with("Received:")
                    || (normalized.starts_with("from ")
                        && normalized.contains(" by ")
                        && normalized.contains(" with ")
                        && normalized.contains(';'))
                {
                    out.push(node.byte_range());
                }
            }
            if !partial_tree && value.len() <= 512 && phrase && plain {
                strings.push(node);
            }
            if plain {
                continue;
            }
            // Interpolations can execute assignments or redefine error names.
            // Keep error-call classification conservative and still inspect
            // their children for visible bindings; never treat them as text.
            shadowed.insert("*".into());
        }
        if stack.len().saturating_add(node.named_child_count()) > max_nodes.min(200_000) {
            return vec![];
        }
        let mut cursor = node.walk();
        stack.extend(node.named_children(&mut cursor));
    }
    for node in strings {
        if error_context(node, source, &shadowed, language_name, &trusted_go) {
            out.push(node.byte_range());
        }
    }
    out
}
fn callee_reference(mut node: Node<'_>) -> bool {
    for _ in 0..8 {
        let Some(parent) = node.parent() else {
            return false;
        };
        if matches!(
            parent.kind(),
            "member_expression"
                | "selector_expression"
                | "scoped_identifier"
                | "scoped_type_identifier"
                | "qualified_identifier"
                | "attribute"
        ) {
            node = parent;
            continue;
        }
        if !matches!(
            parent.kind(),
            "new_expression"
                | "object_creation_expression"
                | "call_expression"
                | "call"
                | "method_invocation"
                | "macro_invocation"
        ) {
            return false;
        }
        return ["function", "constructor", "type", "method", "name", "macro"]
            .iter()
            .any(|field| {
                parent
                    .child_by_field_name(field)
                    .is_some_and(|n| n.id() == node.id())
            });
    }
    false
}
fn binding_names(
    root: Node<'_>,
    text: &str,
    names: &mut std::collections::BTreeSet<String>,
) -> bool {
    let mut stack = vec![root];
    let mut count = 0;
    while let Some(n) = stack.pop() {
        count += 1;
        if count > 512 || stack.len().saturating_add(n.named_child_count()) > 512 {
            return false;
        }
        if n.kind().contains("wildcard") {
            names.insert("*".into());
        }
        if matches!(
            n.kind(),
            "identifier"
                | "package_identifier"
                | "field_identifier"
                | "type_identifier"
                | "property_identifier"
                | "shorthand_property_identifier_pattern"
        ) {
            names.insert(text[n.byte_range()].into());
        }
        let mut cursor = n.walk();
        stack.extend(n.named_children(&mut cursor));
    }
    true
}
fn error_context(
    mut node: Node<'_>,
    text: &str,
    shadowed: &std::collections::BTreeSet<String>,
    language: &str,
    trusted_go: &std::collections::BTreeSet<String>,
) -> bool {
    for _ in 0..8 {
        let Some(parent) = node.parent() else {
            break;
        };
        match parent.kind() {
            "variable_declarator" => {
                return parent.child_by_field_name("name").is_some_and(|name| {
                    let name = text[name.byte_range()].to_ascii_uppercase();
                    name.contains("ERROR") && (name.contains("MESSAGE") || name.contains("TEXT"))
                });
            }
            "new_expression"
            | "object_creation_expression"
            | "call_expression"
            | "call"
            | "method_invocation"
            | "macro_invocation" => {
                let callee = parent
                    .child_by_field_name("function")
                    .or_else(|| parent.child_by_field_name("constructor"))
                    .or_else(|| parent.child_by_field_name("type"))
                    .or_else(|| parent.child_by_field_name("name"))
                    .or_else(|| parent.child_by_field_name("method"))
                    .or_else(|| parent.child_by_field_name("macro"));
                let Some(callee) = callee else {
                    return false;
                };
                let value = &text[callee.byte_range()];
                if shadowed.contains("*")
                    || shadowed.contains(value.split('.').next().unwrap_or(value))
                {
                    return false;
                }
                return match language {
                    "javascript" | "typescript" => matches!(
                        value,
                        "Error"
                            | "TypeError"
                            | "RangeError"
                            | "SyntaxError"
                            | "ReferenceError"
                            | "URIError"
                            | "EvalError"
                            | "AggregateError"
                    ),
                    "python" => matches!(
                        value,
                        "ValueError"
                            | "RuntimeError"
                            | "TypeError"
                            | "Exception"
                            | "AssertionError"
                    ),
                    "ruby" => value == "raise",
                    "go" => {
                        (value == "errors.New" && trusted_go.contains("errors"))
                            || (value == "fmt.Errorf" && trusted_go.contains("fmt"))
                    }
                    "rust" => parent.kind() == "macro_invocation" && value == "panic",
                    "java" => {
                        parent.kind() == "object_creation_expression"
                            && (value.ends_with("Exception") || value == "Error")
                            && parent
                                .parent()
                                .is_some_and(|n| n.kind() == "throw_statement")
                    }
                    _ => false,
                };
            }
            // Never classify an argument of an intervening arbitrary call.
            "function_declaration" | "function_definition" | "method" | "assignment" => {
                return false
            }
            _ => {}
        }
        node = parent;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_and_configuration_are_distinct() {
        for (extension, source) in [
            ("ts", "throw new Error('Unsupported RS256 algorithm');"),
            ("java", "import example.ComponentValidationException; class A { void f() throws ComponentValidationException { throw new ComponentValidationException(\"Unsupported RS256 algorithm\"); } }"),
            ("py", "raise ValueError('Unsupported RS256 algorithm')"),
            ("java", "class A { void f() { throw new IllegalArgumentException(\"Unsupported RS256 algorithm\"); } }"),
            ("go", "package p\nimport \"errors\"\nfunc f() error { return errors.New(\"Unsupported RS256 algorithm\") }"),
            ("rb", "raise ArgumentError, 'Unsupported RS256 algorithm'"),
            ("rs", "fn main() { panic!(\"Unsupported RS256 algorithm\"); }"),
            ("vue", "<script setup lang=\"ts\">const NODE_API_AUTH_ERROR_MESSAGES = ['secretOrPrivateKey must be an asymmetric key when using RS256'];</script><template></template>"),
        ] {
            let file = ScannableFile::from_bytes(format!("display.{extension}").into(), source.as_bytes().to_vec());
            assert!(!ranges(&file, 10000).is_empty(), "{extension}");
        }
        for (extension, source) in [
            ("go", "package p\nfunc f(){ errors.New(\"Unsupported RS256 algorithm\") }"),
            ("ts", "for (const Error of builders) { new Error('Unsupported RS256 algorithm'); }"),
            ("py", "for ValueError in builders:\n    ValueError('Unsupported RS256 algorithm')"),
            ("py", "with builder() as ValueError:\n    ValueError('Unsupported RS256 algorithm')"),
            ("py", "try:\n    f()\nexcept Exception as ValueError:\n    ValueError('Unsupported RS256 algorithm')"),
            ("go", "package p\nimport . \"example.com/crypto\"\nfunc f(){ Error(\"Unsupported RS256 algorithm\") }"),
            ("java", "class A { static class RuntimeException {} void f(){ throw new RuntimeException(\"Unsupported RS256 algorithm\"); } }"),
            ("py", "f\"{(ValueError := crypto_builder)}\"\nValueError('Unsupported RS256 algorithm')"),
            ("rb", "text = \"#{def raise(message); encrypt(message); end}\"\nraise 'Unsupported RS256 algorithm'"),
            ("py", "Error('Unsupported RS256 algorithm')"),
            ("rs", "fn main(){Error(\"Unsupported RS256 algorithm\");}"),
            ("go", "package p\nfunc f(){ errors := cryptoBuilder; errors.New(\"Unsupported RS256 algorithm\") }"),
            ("go", "package p\nimport errors \"example.com/crypto\"\nfunc f(){ errors.New(\"Unsupported RS256 algorithm\") }"),
            ("go", "package p\nvar errors = cryptoBuilder\nfunc f(){ errors.New(\"Unsupported RS256 algorithm\") }"),
            ("rs", "use custom::panic; fn main(){panic!(\"Unsupported RS256 algorithm\");}"),
            ("rs", "fn Error(x:&str){configure(x);} fn main(){Error(\"Unsupported RS256 algorithm\");}"),
            ("rs", "fn main(){let Error=configure; Error(\"Unsupported RS256 algorithm\");}"),
            ("py", "def f(ValueError):\n    raise ValueError('Unsupported RS256 algorithm')"),
        ] {
            let file = ScannableFile::from_bytes(format!("shadow.{extension}").into(), source.as_bytes().to_vec());
            assert!(ranges(&file,10000).is_empty(), "{extension}: {source}");
        }
        for source in [
            "def raise(message); encrypt(message); end; raise 'Unsupported RS256 algorithm'",
            "def f(raise); raise('Unsupported RS256 algorithm'); end",
        ] {
            let file = ScannableFile::from_bytes("custom.rb".into(), source.as_bytes().to_vec());
            assert!(ranges(&file, 10000).is_empty(), "{source}");
        }
        for source in [
            "configureException('Unsupported RS256 algorithm');",
            "function f(Error) { throw new Error('Unsupported RS256 algorithm'); }",
            "const {Error} = custom; throw new Error('Unsupported RS256 algorithm');",
            "function Error(x) { return configure(x); } throw new Error('Unsupported RS256 algorithm');",
            "const algorithm = 'RS256';",
            "jwt.sign(p, k, {algorithm:'RS256'});",
            "configure('Unsupported RS256 algorithm');",
            "const ERROR_MESSAGES = [configure('Unsupported RS256 algorithm')];",
            "const value = `Unsupported RS256 ${fn()}`;",
        ] {
            let file = ScannableFile::from_bytes("config.ts".into(), source.as_bytes().to_vec());
            assert!(ranges(&file, 10000).is_empty(), "{source}");
        }
    }
}
