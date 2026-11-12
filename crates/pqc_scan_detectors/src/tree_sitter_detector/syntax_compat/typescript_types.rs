//! Bounded, parse-only recognition of import types when the first tree recovers
//! them as expressions. The probe tree never supplies evidence or output bytes.
use tree_sitter::{Node, Parser, Tree};

fn charge(remaining: &mut usize, cost: usize) -> bool {
    if let Some(next) = remaining.checked_sub(cost) {
        *remaining = next;
        true
    } else {
        false
    }
}
fn adapt(bytes: &mut [u8], start: usize, open: usize, end: usize, qualified_end: usize) {
    bytes[start + 1] = b'_';
    bytes[open] = b'<';
    bytes[end - 1] = b'>';
    for b in &mut bytes[end..qualified_end] {
        if *b == b'.' {
            *b = b'&';
        }
    }
}
fn type_context(
    mut node: Node<'_>,
    start: usize,
    end: usize,
    remaining: &mut usize,
) -> Option<bool> {
    loop {
        if !charge(remaining, 1) {
            return None;
        }
        match node.kind() {
            "type_query"
            | "type_annotation"
            | "type_arguments"
            | "type_alias_declaration"
            | "generic_type"
            | "type_parameter"
            | "function_type"
            | "constructor_type" => return Some(true),
            "as_expression" | "satisfies_expression" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if !charge(remaining, 1) {
                        return None;
                    }
                    if matches!(child.kind(), "as" | "satisfies") && child.end_byte() <= start {
                        return Some(true);
                    }
                }
                return Some(false);
            }
            // A containing type declaration does not turn its runtime expressions
            // (e.g. computed property names) into import types.
            "arrow_function"
            | "statement_block"
            | "computed_property_name"
            | "arguments"
            | "expression_statement"
            | "pair"
            | "object" => return Some(false),
            _ => {}
        }
        let Some(parent) = node.parent() else {
            return Some(false);
        };
        if parent.start_byte() > start || parent.end_byte() < end {
            return Some(false);
        }
        node = parent;
    }
}

pub(super) fn normalize(
    text: &str,
    tree: &Tree,
    protected: &[bool],
    bytes: &mut [u8],
    remaining: &mut usize,
) -> bool {
    static IMPORT: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let import = IMPORT.get_or_init(|| {
        regex::Regex::new(r#"\bimport\s*\(\s*(?:'[^'"\r\n]+'|"[^'"\r\n]+")\s*\)"#).unwrap()
    });
    let mut candidates = Vec::new();
    for m in import.find_iter(text) {
        if !charge(remaining, m.len()) {
            return false;
        }
        if protected[m.start()] {
            continue;
        }
        let mut qualified_end = m.end();
        while text.as_bytes().get(qualified_end) == Some(&b'.') {
            let begin = qualified_end + 1;
            if !text
                .as_bytes()
                .get(begin)
                .is_some_and(|b| b.is_ascii_alphabetic() || matches!(*b, b'_' | b'$'))
            {
                break;
            }
            let mut end = begin;
            while text
                .as_bytes()
                .get(end)
                .is_some_and(|b| b.is_ascii_alphanumeric() || matches!(*b, b'_' | b'$'))
            {
                end += 1;
            }
            if !charge(remaining, end - qualified_end) {
                return false;
            }
            if end == begin {
                break;
            }
            qualified_end = end;
        }
        candidates.push((
            m.start(),
            m.start() + m.as_str().find('(').unwrap(),
            m.end(),
            qualified_end,
        ));
    }
    if candidates.is_empty() {
        return true;
    }
    if !charge(remaining, bytes.len()) {
        return false;
    }
    let mut probe = bytes.to_vec();
    for &(start, open, end, last) in &candidates {
        adapt(&mut probe, start, open, end, last);
    }
    let mut parser = Parser::new();
    if parser.set_language(&tree.language()).is_err() {
        return true;
    }
    let Some(probe_tree) = parser.parse(&probe, None) else {
        return true;
    };
    for (start, open, end, last) in candidates {
        let Some(node) = probe_tree.root_node().descendant_for_byte_range(start, end) else {
            continue;
        };
        match type_context(node, start, end, remaining) {
            Some(true) => adapt(bytes, start, open, end, last),
            Some(false) => {}
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn runtime_import_bytes_remain_original_beside_recovered_type_imports() {
        let text = "type View = () => import('types').Node[];\nconst load = () => import('runtime').then(f);\nasync function run() { await import('runtime2'); }\nconst x = (import('runtime3')) as unknown;";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(text, None).unwrap();
        let mut bytes = text.as_bytes().to_vec();
        assert!(super::normalize(
            text,
            &tree,
            &vec![false; text.len()],
            &mut bytes,
            &mut 100000
        ));
        assert_ne!(
            &bytes[..text.find('\n').unwrap()],
            &text.as_bytes()[..text.find('\n').unwrap()]
        );
        let start = text.find('\n').unwrap();
        assert_eq!(&bytes[start..], &text.as_bytes()[start..]);
    }
}
