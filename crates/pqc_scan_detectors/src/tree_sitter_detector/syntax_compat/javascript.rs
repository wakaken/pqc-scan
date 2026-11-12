//! Offset-preserving compatibility for supported script syntax.
use super::javascript_lexer::{identifier, spend, tokens, Token};
use tree_sitter::Tree;
fn blank(bytes: &mut [u8], a: usize, b: usize) {
    for c in &mut bytes[a..b] {
        if !matches!(*c, b'\n' | b'\r') {
            *c = b' ';
        }
    }
}
fn closing(
    ts: &[Token<'_>],
    start: usize,
    open: &str,
    close: &str,
    left: &mut usize,
) -> Option<usize> {
    let mut depth = 0;
    for (i, t) in ts.iter().enumerate().skip(start) {
        spend(left, 1)?;
        if t.value == open {
            depth += 1;
        }
        if t.value == close {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}
fn strict_directive(ts: &[Token<'_>]) -> bool {
    for t in ts {
        if t.value == ";" {
            continue;
        }
        if !t.value.starts_with(['\'', '"']) {
            break;
        }
        if matches!(t.value, "'use strict'" | "\"use strict\"") {
            return true;
        }
    }
    false
}
pub(super) fn normalize(
    text: &str,
    tree: &Tree,
    bytes: &mut [u8],
    left: &mut usize,
    module: bool,
) -> bool {
    let Some(ts) = tokens(text, tree, bytes, left) else {
        return false;
    };
    let mut depth = 0;
    let mut strict = module
        || strict_directive(&ts)
        || text
            .lines()
            .take(10)
            .any(|l| l.trim_start().starts_with("// Flags:") && l.contains("--use-strict"));
    for (i, t) in ts.iter().enumerate() {
        if depth == 0
            && (t.value == "export"
                || t.value == "import"
                    && ts.get(i + 1).is_some_and(|n| !matches!(n.value, "(" | ".")))
        {
            strict = true;
        }
        if t.value == "{" {
            depth += 1;
        }
        if t.value == "}" {
            depth -= 1;
        }
    }
    let mut functions = vec![];
    let mut classes = vec![];
    for (i, t) in ts.iter().enumerate() {
        if spend(left, 1).is_none() {
            return false;
        }
        if t.value == "class" {
            let mut opening = None;
            for (j, t) in ts.iter().enumerate().skip(i + 1) {
                if spend(left, 1).is_none() {
                    return false;
                }
                if t.value == "{" {
                    opening = Some(j);
                    break;
                }
            }
            if let Some(a) = opening {
                if let Some(b) = closing(&ts, a, "{", "}", left) {
                    classes.push((ts[a].start, ts[b].end));
                }
            }
        }
        if t.value != "function" {
            continue;
        }
        let mut a = i + 1;
        let generator = ts.get(a).is_some_and(|t| t.value == "*");
        if generator {
            a += 1;
        }
        if ts.get(a).is_some_and(|t| t.value != "(") {
            a += 1;
        }
        if ts.get(a).is_none_or(|t| t.value != "(") {
            continue;
        }
        let Some(b) = closing(&ts, a, "(", ")", left) else {
            continue;
        };
        if ts.get(b + 1).is_none_or(|t| t.value != "{") {
            continue;
        }
        let Some(end) = closing(&ts, b + 1, "{", "}", left) else {
            continue;
        };
        let own_strict = strict_directive(&ts[b + 2..end]);
        functions.push((ts[i].start, ts[end].end, generator, own_strict));
    }
    let mut block_ends = std::collections::BTreeSet::new();
    let mut nodes = vec![tree.root_node()];
    while let Some(n) = nodes.pop() {
        if spend(left, 1).is_none() {
            return false;
        }
        if n.kind() == "method_definition" {
            if let Some(body) = n.child_by_field_name("body") {
                let mut c = n.walk();
                let generator = n.children(&mut c).any(|c| c.kind() == "*");
                let start = ts.partition_point(|t| t.start <= body.start_byte());
                let end = ts.partition_point(|t| t.start < body.end_byte());
                functions.push((
                    n.start_byte(),
                    n.end_byte(),
                    generator,
                    strict_directive(&ts[start..end]),
                ));
            }
        }
        if n.kind() == "statement_block"
            && n.parent().is_some_and(|p| {
                matches!(
                    p.kind(),
                    "catch_clause"
                        | "try_statement"
                        | "if_statement"
                        | "for_statement"
                        | "for_in_statement"
                        | "while_statement"
                        | "function_declaration"
                        | "generator_function_declaration"
                        | "program"
                        | "statement_block"
                )
            })
        {
            block_ends.insert(n.end_byte());
        }
        let mut c = n.walk();
        nodes.extend(n.named_children(&mut c));
    }
    let mut control_ends = std::collections::BTreeSet::new();
    for (i, t) in ts.iter().enumerate() {
        if matches!(
            t.value,
            "if" | "while" | "for" | "with" | "catch" | "switch"
        ) && ts.get(i + 1).is_some_and(|t| t.value == "(")
        {
            if let Some(end) = closing(&ts, i + 1, "(", ")", left) {
                control_ends.insert(end);
                if ts.get(end + 1).is_some_and(|t| t.value == "{") {
                    if let Some(close) = closing(&ts, end + 1, "{", "}", left) {
                        block_ends.insert(ts[close].end);
                    }
                }
            }
        }
    }
    for (i, t) in ts.iter().enumerate() {
        if matches!(t.value, "else" | "try" | "finally" | "do")
            && ts.get(i + 1).is_some_and(|t| t.value == "{")
        {
            if let Some(close) = closing(&ts, i + 1, "{", "}", left) {
                block_ends.insert(ts[close].end);
            }
        }
    }
    for (i, t) in ts.iter().enumerate() {
        if spend(left, functions.len() + classes.len() + 1).is_none() {
            return false;
        }
        if t.value == "yield" && !strict {
            let scope = functions
                .iter()
                .filter(|f| f.0 <= t.start && t.end <= f.1)
                .min_by_key(|f| f.1 - f.0);
            let forbidden = scope.is_some_and(|f| f.2)
                || functions
                    .iter()
                    .any(|f| f.0 <= t.start && t.end <= f.1 && f.3)
                || classes.iter().any(|f| f.0 <= t.start && t.end <= f.1);
            if !forbidden {
                bytes[t.start + 1] = b'_';
            }
        }
        if t.value == "%"
            && ts
                .get(i + 1)
                .is_some_and(|n| n.value.as_bytes().first().is_some_and(|b| identifier(*b)))
            && ts.get(i + 2).is_some_and(|n| n.value == "(")
        {
            let previous = i.checked_sub(1).and_then(|j| ts.get(j));
            if previous.is_some_and(|p| p.value == "}" && block_ends.contains(&p.end))
                || i.checked_sub(1).is_some_and(|j| control_ends.contains(&j))
                || previous.is_none_or(|p| {
                    matches!(
                        p.value,
                        "(" | "["
                            | "{"
                            | "="
                            | ":"
                            | ","
                            | ";"
                            | "!"
                            | "?"
                            | "&&"
                            | "||"
                            | "return"
                            | "throw"
                            | "typeof"
                            | "void"
                            | "delete"
                            | "in"
                            | "of"
                            | "else"
                            | "+"
                            | "-"
                            | "*"
                            | "/"
                            | "**"
                            | "=="
                            | "==="
                            | "!="
                            | "!=="
                            | "<"
                            | ">"
                            | "<="
                            | ">="
                            | "~"
                            | "&"
                            | "|"
                            | "^"
                            | "=>"
                    )
                })
            {
                bytes[t.start] = b' ';
            }
        }
    }
    for (i, t) in ts.iter().enumerate() {
        if t.value != "[" {
            continue;
        }
        if i > 0
            && !matches!(
                ts[i - 1].value,
                ";" | "{" | "}" | "(" | "=" | "," | "return"
            )
        {
            continue;
        }
        let Some(end) = closing(&ts, i, "[", "]", left) else {
            continue;
        };
        if ts.get(end + 1).is_none_or(|t| t.value != "=") {
            continue;
        }
        // Only direct pattern elements qualify. Computed class fields may also
        // have [...] = initializers, with arbitrary nested function bodies.
        let mut depth = 0usize;
        for j in i + 1..end {
            if spend(left, 1).is_none() {
                return false;
            }
            if depth == 0
                && ts[j].value == "("
                && ts
                    .get(j + 1)
                    .is_some_and(|t| t.value.bytes().all(identifier))
                && ts.get(j + 2).is_some_and(|t| t.value == ")")
                && ts
                    .get(j + 3)
                    .is_some_and(|t| matches!(t.value, "," | "]" | "="))
            {
                bytes[ts[j].start] = b' ';
                bytes[ts[j + 2].start] = b' ';
            }
            match ts[j].value {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
    }
    // Call expressions as assignment/for-in targets are legacy runtime errors,
    // not successful assignable references. Never touch optional-chain targets.
    let mut pending = vec![tree.root_node()];
    while let Some(n) = pending.pop() {
        if spend(left, 1).is_none() {
            return false;
        }
        if n.kind() == "call_expression" && !n.has_error() && !text[n.byte_range()].contains("?.") {
            let after = text[n.end_byte()..].trim_start();
            if (after.starts_with('=') && !after.starts_with("==") && !after.starts_with("=>"))
                || after.starts_with("**=")
                || after.starts_with("in ")
            {
                blank(bytes, n.start_byte(), n.end_byte());
                bytes[n.start_byte()] = b'_';
            }
        }
        if n.kind() == "parenthesized_expression"
            && n.named_child_count() == 1
            && n.named_child(0).is_some_and(|c| c.kind() == "identifier")
            && n.parent()
                .is_some_and(|p| matches!(p.kind(), "array_pattern" | "assignment_pattern"))
        {
            bytes[n.start_byte()] = b' ';
            bytes[n.end_byte() - 1] = b' ';
        }
        let mut c = n.walk();
        for ch in n.named_children(&mut c) {
            pending.push(ch);
        }
    }
    *left > 0
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal_regex_and_remainder_percent_bytes_are_preserved() {
        for text in [
            "const x = value %Native(); const s='%Native(yield)';",
            r"if (x) /%Native\(yield\)/.test(s);",
            "const x = function() {} %Native();",
            "var yield; yield %Native();",
            "const x = class {} %Native();",
        ] {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_javascript::LANGUAGE.into())
                .unwrap();
            let tree = parser.parse(text, None).unwrap();
            let mut bytes = text.as_bytes().to_vec();
            assert!(super::normalize(
                text,
                &tree,
                &mut bytes,
                &mut 1000000,
                false
            ));
            for (offset, _) in text.match_indices('%') {
                assert_eq!(bytes[offset], b'%', "{text}");
            }
        }
    }
    #[test]
    fn computed_field_nested_calls_and_catch_keep_their_parentheses() {
        let text = "class C { [(exported = function access(r,v){try {r.m()} catch(e){} advance(v);}, 'x')] = 1; }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(text, None).unwrap();
        let mut bytes = text.as_bytes().to_vec();
        assert!(super::normalize(
            text,
            &tree,
            &mut bytes,
            &mut 1_000_000,
            false
        ));
        assert_eq!(bytes, text.as_bytes());
    }
    #[test]
    fn depleted_compatibility_work_is_reported_as_incomplete() {
        let text = "function f(yield){yield()}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(text, None).unwrap();
        assert!(!super::normalize(
            text,
            &tree,
            &mut text.as_bytes().to_vec(),
            &mut 1,
            false
        ));
    }
}
