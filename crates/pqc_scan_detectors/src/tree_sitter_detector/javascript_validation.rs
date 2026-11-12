//! A narrow guard for the sloppy-yield compatibility, not a JS compiler.
use tree_sitter::{Node, Tree};
fn directive(node: Node<'_>, text: &str) -> bool {
    let mut cursor = node.walk();
    for n in node.named_children(&mut cursor) {
        if n.kind() == "comment" {
            continue;
        }
        if n.kind() != "expression_statement" {
            break;
        }
        let Some(value) = n.named_child(0).filter(|n| n.kind() == "string") else {
            break;
        };
        if matches!(&text[value.byte_range()], "'use strict'" | "\"use strict\"") {
            return true;
        }
    }
    false
}
pub(super) fn strict_yield_binding(
    text: &str,
    tree: &Tree,
    module: bool,
    max: usize,
) -> (bool, bool) {
    let root = tree.root_node();
    let mut c = root.walk();
    let strict = module
        || directive(root, text)
        || root
            .named_children(&mut c)
            .any(|n| matches!(n.kind(), "import_statement" | "export_statement"))
        || text
            .lines()
            .take(10)
            .any(|l| l.trim_start().starts_with("// Flags:") && l.contains("--use-strict"));
    let mut stack = vec![(root, strict)];
    let mut examined = 0;
    while let Some((n, inherited)) = stack.pop() {
        if examined >= max {
            return (false, true);
        }
        examined += 1;
        let strict = inherited
            || matches!(n.kind(), "class" | "class_declaration")
            || matches!(
                n.kind(),
                "function_declaration"
                    | "function_expression"
                    | "generator_function_declaration"
                    | "generator_function"
                    | "arrow_function"
                    | "method_definition"
            ) && n
                .child_by_field_name("body")
                .is_some_and(|b| b.kind() == "statement_block" && directive(b, text));
        if strict
            && n.kind() == "identifier"
            && &text[n.byte_range()] == "yield"
            && n.parent().is_some_and(|p| {
                matches!(
                    p.kind(),
                    "variable_declarator"
                        | "formal_parameters"
                        | "required_parameter"
                        | "optional_parameter"
                        | "function_declaration"
                        | "function_expression"
                )
            })
        {
            return (true, false);
        }
        let mut c = n.walk();
        stack.extend(n.named_children(&mut c).map(|n| (n, strict)));
    }
    (false, false)
}
