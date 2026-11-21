//! Inspect an initial require_relative of constant-only modules, without loading Ruby.
use super::{literal, method};
use pqc_scan_core::{RepositoryContext, ScannableFile};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
use tree_sitter::{Node, Parser};

pub(super) struct Loads {
    approved: BTreeSet<usize>,
    pub limited: bool,
}
impl Loads {
    pub fn new(
        root: Node<'_>,
        text: &str,
        file: &ScannableFile,
        repo: Option<&RepositoryContext>,
    ) -> Self {
        let mut result = Self {
            approved: BTreeSet::new(),
            limited: false,
        };
        let mut cache = BTreeMap::new();
        let mut cursor = root.walk();
        for node in root.named_children(&mut cursor) {
            if node.kind() == "comment" {
                continue;
            }
            if node.kind() != "call"
                || method(node, text) != Some("require_relative")
                || node.child_by_field_name("receiver").is_some()
                || node.child_by_field_name("block").is_some()
            {
                break;
            }
            let Some(args) = node
                .child_by_field_name("arguments")
                .filter(|a| a.named_child_count() == 1)
            else {
                break;
            };
            let Some(path) = args.named_child(0).and_then(|n| literal(n, text)) else {
                break;
            };
            if path.len() > 256 || path.contains(['\0', '\\']) || Path::new(path).is_absolute() {
                break;
            }
            let target = match Path::new(path).extension().and_then(|s| s.to_str()) {
                None => format!("{path}.rb"),
                Some("rb") => path.to_owned(),
                _ => break,
            };
            let pure = if let Some(cached) = cache.get(&target) {
                *cached
            } else {
                if cache.len() == 8 {
                    result.limited = true;
                    break;
                }
                let pure = repo
                    .and_then(|r| r.read_relative(&file.path, Path::new(&target)))
                    .is_some_and(|f| {
                        if f.bytes.len() > 16384 {
                            result.limited = true;
                            return false;
                        }
                        let Some(source) = f.text.as_deref() else {
                            return false;
                        };
                        let mut parser = Parser::new();
                        if parser
                            .set_language(&tree_sitter_ruby::LANGUAGE.into())
                            .is_err()
                        {
                            return false;
                        }
                        let Some(tree) = parser.parse(source, None) else {
                            result.limited = true;
                            return false;
                        };
                        if tree.root_node().has_error() {
                            return false;
                        }
                        let mut stack = vec![tree.root_node()];
                        let mut scheduled = 1;
                        while let Some(n) = stack.pop() {
                            let mut c = n.walk();
                            for child in n.named_children(&mut c) {
                                if scheduled == 4096 {
                                    result.limited = true;
                                    return false;
                                }
                                scheduled += 1;
                                stack.push(child);
                            }
                        }
                        pure_module(tree.root_node(), source, 0, &mut result.limited)
                    });
                cache.insert(target, pure);
                pure
            };
            if !pure {
                break;
            }
            result.approved.insert(node.id());
        }
        result
    }
    pub fn approved(&self, node: Node<'_>) -> bool {
        self.approved.contains(&node.id())
    }
}
fn pure_module(node: Node<'_>, text: &str, depth: usize, limited: &mut bool) -> bool {
    if depth > 8 {
        *limited = true;
        return false;
    }
    match node.kind() {
        "program" | "body_statement" => {
            let mut cursor = node.walk();
            let pure = node
                .named_children(&mut cursor)
                .all(|n| pure_module(n, text, depth, limited));
            pure
        }
        "comment" => true,
        "module" => {
            node.child_by_field_name("name").is_some_and(|n| {
                n.kind() == "constant"
                    && !matches!(
                        &text[n.byte_range()],
                        "Gem" | "File" | "Kernel" | "Object" | "BasicObject" | "Module" | "Class"
                    )
            }) && node
                .child_by_field_name("body")
                .is_none_or(|n| pure_module(n, text, depth + 1, limited))
        }
        "assignment" if depth > 0 => {
            node.child_by_field_name("left")
                .is_some_and(|n| n.kind() == "constant" && &text[n.byte_range()] == "VERSION")
                && node
                    .child_by_field_name("right")
                    .and_then(|n| literal(n, text))
                    .is_some_and(|s| s.len() <= 128 && s.is_ascii())
        }
        _ => false,
    }
}
