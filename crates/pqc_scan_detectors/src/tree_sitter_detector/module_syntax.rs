//! Parse-only compatibility for module modifiers absent from the pinned grammars.
//! The original source is always used for matching, evidence and coordinates.
use anyhow::{anyhow, Result};
use tree_sitter::{Parser, Tree};

fn token(text: &str, mut i: usize) -> Option<(&str, usize, usize)> {
    let b = text.as_bytes();
    loop {
        while b.get(i).is_some_and(u8::is_ascii_whitespace) {
            i += 1;
        }
        if b.get(i..i + 2) == Some(b"/*") {
            i += 2;
            i += text.get(i..)?.find("*/")? + 2;
        } else if b.get(i..i + 2) == Some(b"//") {
            i += text.get(i..)?.find('\n')? + 1;
        } else {
            break;
        }
    }
    let start = i;
    let c = *b.get(i)?;
    if c.is_ascii_alphabetic() || c == b'_' || c == b'$' {
        i += 1;
        while b
            .get(i)
            .is_some_and(|v| v.is_ascii_alphanumeric() || *v == b'_' || *v == b'$')
        {
            i += 1;
        }
    } else {
        i += 1;
    }
    Some((text.get(start..i)?, start, i))
}
pub(super) fn compatible_tree(
    parser: &mut Parser,
    text: &str,
    tree: Tree,
    language: &str,
    work_budget: usize,
    module: bool,
) -> Result<(Tree, bool)> {
    if !tree.root_node().has_error() {
        return Ok((tree, false));
    }
    let flow = language == "javascript"
        && text
            .lines()
            .take(5)
            .any(|l| l.trim_start().starts_with("//") && l.contains("@flow"));
    if flow {
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
            .map_err(|_| anyhow!("failed to initialize Flow-compatible parser"))?;
    }
    let mut changes = std::collections::BTreeSet::new();
    // Only actual import/export statements (or top-level error spans) qualify.
    // Strings, comments, templates and nested arbitrary error fragments do not.
    let root = tree.root_node();
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        if !matches!(
            node.kind(),
            "export_statement" | "import_statement" | "ERROR"
        ) {
            continue;
        }
        let start = node.start_byte();
        let fragment = &text[start..node.end_byte()];
        let Some((first, _, end)) = token(fragment, 0) else {
            continue;
        };
        let Some((modifier, a, end)) = token(fragment, end) else {
            continue;
        };
        let Some((star, _, _)) = token(fragment, end) else {
            continue;
        };
        let supported = (language == "typescript" && first == "export" && modifier == "type")
            || (first == "import" && modifier == "defer");
        if supported && star == "*" {
            changes.insert((start + a, start + end));
        }
    }
    drop(cursor);
    let (mut bytes, limited) =
        super::syntax_compat::normalize(text, &tree, language, work_budget, module);
    if limited {
        return Ok((tree, true));
    }
    if changes.is_empty() && bytes == text.as_bytes() && !flow {
        return Ok((tree, false));
    }
    for (a, b) in changes {
        bytes[a..b].fill(b' ');
    }
    // Replacing ASCII modifiers with spaces preserves every byte, line and column.
    parser
        .parse(&bytes, None)
        .map(|tree| (tree, false))
        .ok_or_else(|| anyhow!("source parser produced no tree"))
}
