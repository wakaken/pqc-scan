//! One bounded parse per Go file, shared by the built-in import indicators.
use pqc_scan_core::DetectorStatus;

pub(super) struct Imports {
    pub spans: Vec<(usize, usize)>,
    pub status: DetectorStatus,
}
pub(super) fn collect(text: &str, max_nodes: usize) -> Imports {
    let mut result = Imports {
        spans: Vec::new(),
        status: DetectorStatus::Complete,
    };
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
        .is_err()
    {
        result.status = DetectorStatus::Partial;
        return result;
    }
    let Some(tree) = parser.parse(text, None) else {
        result.status = DetectorStatus::Partial;
        return result;
    };
    let mut remaining = max_nodes;
    let mut cursor = tree.root_node().walk();
    'declarations: for declaration in tree.root_node().named_children(&mut cursor) {
        let Some(next) = remaining.checked_sub(1) else {
            result.status = DetectorStatus::Truncated;
            break;
        };
        remaining = next;
        if declaration.kind() != "import_declaration" {
            continue;
        }
        if declaration.has_error() {
            result.status = DetectorStatus::Partial;
        }
        let mut stack = vec![declaration];
        while let Some(node) = stack.pop() {
            let Some(next) = remaining.checked_sub(1) else {
                result.status = DetectorStatus::Truncated;
                break 'declarations;
            };
            remaining = next;
            if node.kind() == "import_spec" && !node.has_error() {
                if let Some(path) = node.child_by_field_name("path") {
                    let start = path.start_byte() + 1;
                    let end = path.end_byte().saturating_sub(1);
                    if start <= end && text.get(start..end) == Some("crypto/rsa") {
                        result.spans.push((start, end));
                    }
                }
            }
            let mut c = node.walk();
            for child in node.named_children(&mut c) {
                stack.push(child);
            }
        }
    }
    result.spans.sort_unstable();
    result
}
