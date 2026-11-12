//! Parse-only view of unrendered template source. Never chooses a branch/value.
pub(crate) struct View {
    pub bytes: Vec<u8>,
    pub found: bool,
    pub limited: bool,
    pub conditional_ranges: Vec<(usize, usize)>,
}
pub(crate) fn view(text: &str, tree: &tree_sitter::Tree) -> View {
    view_with_comments(text, false, Some(tree))
}
pub(crate) fn requirements_view(text: &str) -> View {
    view_with_comments(text, true, None)
}
fn view_with_comments(text: &str, hash_comments: bool, tree: Option<&tree_sitter::Tree>) -> View {
    if !text.contains("{{") && !text.contains("{%") {
        return View {
            bytes: vec![],
            found: false,
            limited: false,
            conditional_ranges: vec![],
        };
    }
    let mut strong = false;
    let line_offsets: Vec<_> = text.match_indices('\n').map(|(i, _)| i).collect();
    let mut blocks: Vec<(&str, usize)> = vec![];
    let mut outer_start = 0;
    let s = text.as_bytes();
    let mut out = View {
        bytes: s.to_vec(),
        found: false,
        limited: false,
        conditional_ranges: vec![],
    };
    let mut i = 0;
    let mut count = 0;
    'scan: while i < s.len() {
        if s[i] == b'/' {
            if let Some(mut node) =
                tree.and_then(|t| t.root_node().descendant_for_byte_range(i, i + 1))
            {
                for _ in 0..64 {
                    if node.kind() == "regex" {
                        i = node.end_byte();
                        continue 'scan;
                    }
                    if let Some(parent) = node.parent() {
                        node = parent;
                    } else {
                        break;
                    }
                }
            }
        }
        if hash_comments && s[i] == b'#' {
            while i < s.len() && s[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if matches!(s[i], b'\'' | b'"' | b'`') {
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
        let directive = s[i..].starts_with(b"{%");
        let expression = s[i..].starts_with(b"{{");
        if !directive && !expression {
            i += 1;
            continue;
        }
        count += 1;
        if count > 10000 {
            out.limited = strong;
            break;
        }
        let closing = if directive { b"%}" } else { b"}}" };
        let Some(end) = s[i + 2..]
            .windows(2)
            .position(|w| w == closing)
            .map(|n| i + 2 + n + 2)
        else {
            break;
        };
        let body = text.get(i + 2..end - 2).unwrap_or("").trim();
        let recognized = directive
            || body.starts_with('$')
            || (!body.is_empty()
                && body
                    .chars()
                    .all(|c| c.is_alphanumeric() || "_ .|".contains(c)));
        if !recognized {
            i = end;
            continue;
        }
        if hash_comments && directive {
            let tag = body
                .trim_matches(['-', '+'])
                .split_whitespace()
                .next()
                .unwrap_or("");
            let starts = matches!(
                tag,
                "if" | "for" | "with" | "block" | "macro" | "filter" | "call"
            );
            if starts {
                if blocks.len() >= 64 {
                    out.limited = true;
                    break;
                }
                if blocks.is_empty() {
                    outer_start = 1 + line_offsets.partition_point(|n| *n < i);
                }
                blocks.push((tag, outer_start));
            } else if matches!(
                tag,
                "endif" | "endfor" | "endwith" | "endblock" | "endmacro" | "endfilter" | "endcall"
            ) {
                blocks.pop();
                if blocks.is_empty() {
                    out.conditional_ranges
                        .push((outer_start, 1 + line_offsets.partition_point(|n| *n < end)));
                }
            }
        }
        strong |= directive || body.starts_with('$') && body.contains(":=");
        out.found = true;
        for byte in &mut out.bytes[i..end] {
            if !matches!(*byte, b'\n' | b'\r') {
                *byte = b' ';
            }
        }
        if expression && !body.contains(":=") {
            out.bytes[i] = b'_';
        }
        i = end;
    }
    if !blocks.is_empty() {
        out.conditional_ranges
            .push((outer_start, line_offsets.len() + 1));
    }
    out.found &= strong;
    out
}
