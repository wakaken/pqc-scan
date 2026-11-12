//! Script grammar compatibility. Original source remains the evidence authority.
use tree_sitter::Tree;
#[derive(Clone)]
pub(super) struct Token<'a> {
    pub(super) value: &'a str,
    pub(super) start: usize,
    pub(super) end: usize,
}
pub(super) fn spend(left: &mut usize, cost: usize) -> Option<()> {
    *left = left.checked_sub(cost)?;
    Some(())
}
pub(super) fn identifier(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'_' | b'$')
}
// Quotes/comments are recognized without trusting an error-recovery tree.
pub(super) fn tokens<'a>(
    text: &'a str,
    tree: &Tree,
    bytes: &mut [u8],
    left: &mut usize,
) -> Option<Vec<Token<'a>>> {
    // Invalid raw escapes can corrupt the original recovery tree, obscuring the
    // tag itself. Use an escape-neutral discovery tree only to prove tag roles;
    // the final view still changes escapes exclusively in proven tagged text.
    let mut probe = text.as_bytes().to_vec();
    tokens_inner(text, tree, &mut probe, left, true)?;
    if probe == text.as_bytes() {
        return tokens_inner(text, tree, bytes, left, false);
    }
    spend(left, text.len().saturating_mul(4))?;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .ok()?;
    let discovery = parser.parse(&probe, None)?;
    tokens_inner(text, &discovery, bytes, left, false)
}
fn tokens_inner<'a>(
    text: &'a str,
    tree: &Tree,
    bytes: &mut [u8],
    left: &mut usize,
    discover_tags: bool,
) -> Option<Vec<Token<'a>>> {
    let mut regexes = std::collections::BTreeMap::new();
    let mut tags = std::collections::BTreeSet::new();
    let mut nodes = vec![tree.root_node()];
    while let Some(n) = nodes.pop() {
        spend(left, 1)?;
        if n.kind() == "call_expression"
            && n.child_by_field_name("function")
                .is_some_and(|f| !f.is_error())
        {
            if let Some(arg) = n
                .child_by_field_name("arguments")
                .filter(|a| a.kind() == "template_string")
            {
                tags.insert(arg.start_byte());
            }
        }
        if n.kind() == "regex" {
            regexes.insert(n.start_byte(), n.end_byte());
        }
        let mut c = n.walk();
        nodes.extend(n.named_children(&mut c));
    }
    let s = text.as_bytes();
    let mut i = 0;
    let mut out: Vec<Token<'a>> = vec![];
    while i < s.len() {
        spend(left, 1)?;
        let a = i;
        if let Some(end) = regexes.get(&i) {
            i = *end;
            out.push(Token {
                value: "regex",
                start: a,
                end: i,
            });
            continue;
        }
        if s[i].is_ascii_whitespace()
            || text[i..].chars().next().is_some_and(|c| {
                matches!(
                    c,
                    '\u{00a0}' | '\u{feff}' | '\u{1680}' | '\u{2000}'
                        ..='\u{200a}'
                            | '\u{2028}'
                            | '\u{2029}'
                            | '\u{202f}'
                            | '\u{205f}'
                            | '\u{3000}'
                )
            })
        {
            i += text[i..].chars().next()?.len_utf8();
            continue;
        }
        if s[i..].starts_with(b"//") {
            while i < s.len() && s[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if s[i..].starts_with(b"/*") {
            i += 2;
            while i < s.len() && !s[i..].starts_with(b"*/") {
                i += 1;
            }
            i = (i + 2).min(s.len());
            continue;
        }
        if s[i] == b'/'
            && out.last().is_none_or(|p| {
                matches!(
                    p.value,
                    "=" | "(" | "[" | "{" | "," | ":" | ";" | "return" | "throw" | "=>" | "case"
                )
            })
        {
            i += 1;
            let mut class = false;
            while i < s.len() {
                let c = s[i];
                i += 1;
                if c == b'\\' {
                    i = (i + 1).min(s.len());
                } else if c == b'[' {
                    class = true;
                } else if c == b']' {
                    class = false;
                } else if c == b'/' && !class || c == b'\n' {
                    break;
                }
            }
            while i < s.len() && s[i].is_ascii_alphabetic() {
                i += 1;
            }
            out.push(Token {
                value: "regex",
                start: a,
                end: i,
            });
            continue;
        }
        if matches!(s[i], b'\'' | b'"') {
            let q = s[i];
            i += 1;
            while i < s.len() {
                let c = s[i];
                i += 1;
                if c == b'\\' {
                    i = (i + 1).min(s.len());
                } else if c == q {
                    break;
                }
            }
        } else if s[i] == b'`' {
            let tagged = discover_tags || tags.contains(&a);
            i += 1;
            let mut interpolation = 0usize;
            while i < s.len() {
                spend(left, 1)?;
                if interpolation > 0 && regexes.contains_key(&i) {
                    i = regexes[&i];
                    continue;
                }
                if interpolation > 0 && s[i..].starts_with(b"//") {
                    while i < s.len() && s[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
                if interpolation > 0 && s[i..].starts_with(b"/*") {
                    i += 2;
                    while i < s.len() && !s[i..].starts_with(b"*/") {
                        i += 1;
                    }
                    i = (i + 2).min(s.len());
                    continue;
                }
                if s[i] == b'\\' {
                    if tagged
                        && interpolation == 0
                        && !s
                            .get(i + 1)
                            .is_some_and(|c| matches!(*c, b'`' | b'\\' | b'$'))
                    {
                        bytes[i] = b'x';
                    }
                    i = (i + 2).min(s.len());
                } else if interpolation > 0 && matches!(s[i], b'\'' | b'"') {
                    let q = s[i];
                    i += 1;
                    while i < s.len() {
                        let c = s[i];
                        i += 1;
                        if c == b'\\' {
                            i = (i + 1).min(s.len());
                        } else if c == q {
                            break;
                        }
                    }
                } else if s[i] == b'`' && interpolation == 0 {
                    i += 1;
                    break;
                } else if interpolation > 0 && s[i] == b'{' {
                    interpolation += 1;
                    i += 1;
                } else if interpolation > 0 && s[i] == b'`' {
                    // Nested template contents are left untouched. Incomplete
                    // nested syntax cannot be repaired by the outer tag.
                    i += 1;
                    while i < s.len() {
                        let c = s[i];
                        i += 1;
                        if c == b'\\' {
                            i = (i + 1).min(s.len());
                        } else if c == b'`' {
                            break;
                        }
                    }
                } else if s[i..].starts_with(b"${") {
                    interpolation += 1;
                    i += 2;
                } else if s[i] == b'}' && interpolation > 0 {
                    interpolation -= 1;
                    i += 1;
                } else {
                    i += 1;
                }
            }
            out.push(Token {
                value: "template",
                start: a,
                end: i,
            });
            continue;
        } else if identifier(s[i]) {
            i += 1;
            while i < s.len() && identifier(s[i]) {
                i += 1;
            }
        } else {
            i += text[a..].chars().next()?.len_utf8();
            for op in [
                "**=", "===", "!==", "=>", "==", "!=", "++", "--", "&&", "||", "?.", "**",
            ] {
                if text[a..].starts_with(op) {
                    i = a + op.len();
                    break;
                }
            }
        }
        if let Some(value) = text.get(a..i) {
            out.push(Token {
                value,
                start: a,
                end: i,
            });
        }
    }
    Some(out)
}
