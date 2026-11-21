//! Test-only syntax regions; never evaluates cfg expressions or expands macros.
use std::ops::Range;
use tree_sitter::Node;

pub(super) fn rust_test_ranges(root: Node<'_>, text: &str, max_nodes: usize) -> Vec<Range<usize>> {
    if root.has_error() {
        return Vec::new();
    }
    let mut stack = vec![root];
    let mut scheduled = 1;
    let mut ranges = Vec::new();
    let mut opaque = Vec::new();
    let mut builtin_test = true;
    while let Some(node) = stack.pop() {
        // An attribute macro on this item or an enclosing module can remove or
        // rewrite a nested #[test]. Only builtin-only syntax establishes it.
        if matches!(node.kind(), "attribute_item" | "inner_attribute_item") {
            let raw: String = text[node.byte_range()]
                .chars()
                .filter(|c| !c.is_ascii_whitespace())
                .collect();
            if raw != "#[test]" && !(raw.starts_with("#[cfg(") && raw.ends_with(")]")) {
                builtin_test = false;
                if node.kind() == "inner_attribute_item" {
                    if let Some(parent) = node.parent() {
                        opaque.push(parent.byte_range());
                    }
                }
            }
        }
        if node.kind() == "extern_crate_declaration"
            || node.kind() == "use_declaration" && {
                let source = &text[node.byte_range()];
                source.contains('*')
                    || source
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .any(|name| name == "test")
            }
            || node.kind() == "macro_definition"
                && node
                    .child_by_field_name("name")
                    .is_some_and(|n| &text[n.byte_range()] == "test")
        {
            builtin_test = false;
        }
        if !matches!(node.kind(), "attribute_item" | "inner_attribute_item") {
            let mut previous = node.prev_named_sibling();
            let (mut cfg_test, mut test, mut uncertain, mut complete) =
                (false, false, false, false);
            for _ in 0..64 {
                let Some(attribute) = previous else {
                    complete = true;
                    break;
                };
                if attribute.kind() == "line_comment" || attribute.kind() == "block_comment" {
                    previous = attribute.prev_named_sibling();
                    continue;
                }
                if attribute.kind() != "attribute_item" {
                    complete = true;
                    break;
                }
                let raw: String = text[attribute.byte_range()]
                    .chars()
                    .filter(|c| !c.is_ascii_whitespace())
                    .collect();
                cfg_test |= raw == "#[cfg(test)]";
                test |= raw == "#[test]" && node.kind() == "function_item";
                uncertain |=
                    raw != "#[test]" && !(raw.starts_with("#[cfg(") && raw.ends_with(")]"));
                previous = attribute.prev_named_sibling();
            }
            if uncertain || !complete {
                opaque.push(node.byte_range());
            }
            if cfg_test {
                ranges.push((node.byte_range(), false));
            } else if test && !uncertain && complete {
                ranges.push((node.byte_range(), true));
            }
        }
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if scheduled >= max_nodes {
                return Vec::new();
            }
            scheduled += 1;
            stack.push(child);
        }
    }
    opaque.sort_by_key(|r| (r.start, std::cmp::Reverse(r.end)));
    let mut merged: Vec<Range<usize>> = Vec::new();
    for range in opaque {
        if let Some(last) = merged.last_mut().filter(|last| range.start < last.end) {
            last.end = last.end.max(range.end);
        } else {
            merged.push(range);
        }
    }
    ranges.retain(|(range, test_attribute)| {
        if *test_attribute && !builtin_test {
            return false;
        }
        let index = merged.partition_point(|r| r.start <= range.start);
        // Direct builtin cfg runs before an attribute on the same item, but
        // an outer attribute macro can rewrite a nested cfg before it is seen.
        index == 0 || {
            let outer = &merged[index - 1];
            outer.end < range.end || outer == range
        }
    });
    ranges.sort_by_key(|(r, _)| (r.start, std::cmp::Reverse(r.end)));
    let mut disjoint: Vec<Range<usize>> = Vec::new();
    for (range, _) in ranges {
        if disjoint.last().is_none_or(|outer| range.start >= outer.end) {
            disjoint.push(range);
        }
    }
    disjoint
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ranges(text: &str, budget: usize) -> Vec<Range<usize>> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(text, None).unwrap();
        rust_test_ranges(tree.root_node(), text, budget)
    }
    #[test]
    fn exact_test_syntax_only_and_errors_and_limits_retain_source() {
        for text in [
            "#[cfg(any(test, feature = \"prod\"))] mod shared { fn f() {} }",
            "#[cfg(not(test))] fn production() {}",
            "#[tokio::test] async fn custom() {}",
            "use kept::test; #[test] fn production() {}",
            "use kept::identity as test; #[test] fn production() {}",
            "use kept::*; #[test] fn production() {}",
            "#[macro_use] extern crate kept; #[test] fn production() {}",
            "#[kept::prod] #[test] fn production() {}",
            "#[kept::prod] mod app { #[test] fn production() {} }",
            "#![kept::prod] #[test] fn production() {}",
            "#[kept::prod] mod app { #[cfg(test)] pub fn production() {} }",
            "#[kept::prod] fn outer() { #[cfg(test)] fn production() {} }",
            "#![kept::prod] #[cfg(test)] fn production() {}",
            "fn text() { let _ = \"#[cfg(test)]\"; }",
            "#[cfg(test)] mod broken {",
        ] {
            assert!(ranges(text, 2000).is_empty(), "{text}");
        }
        let text =
            "#[cfg(test)] /* comment */ mod checks { #[test] fn unit() {} } fn production() {}";
        let found = ranges(text, 2000);
        assert!(found
            .iter()
            .any(|r| text[r.clone()].starts_with("mod checks")));
        assert!(!found
            .iter()
            .any(|r| text[r.clone()].contains("fn production")));
        assert!(ranges(text, 2).is_empty());
        let guarded = "use kept::test; #[cfg(test)] mod checks { fn only_test() {} } #[test] fn production() {}";
        let result = ranges(guarded, 2000);
        assert_eq!(result.len(), 1);
        assert!(guarded[result[0].clone()].starts_with("mod checks"));
        let direct = "#[kept::prod] #[cfg(test)] fn only_test() {}";
        assert_eq!(ranges(direct, 2000).len(), 1);
    }
}
