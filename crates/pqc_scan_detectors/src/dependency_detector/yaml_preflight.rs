//! Reject actual YAML node properties before deserialization, without treating
//! punctuation inside plain/quoted/block scalars as anchors, aliases or tags.
//! Grammar reference: https://yaml.org/spec/1.2.2/#733-plain-style
use super::decode::Failure;
pub(super) fn inspect(text: &str) -> Result<(), Failure> {
    let mut quote = None;
    let mut escaped = false;
    let mut flow = 0usize;
    let mut block_indent = None;
    for line in text.lines() {
        let indent = line.len() - line.trim_start().len();
        if indent > 256 {
            return Err(Failure::Limit);
        }
        if let Some(parent) = block_indent {
            if line.trim().is_empty() || indent > parent {
                continue;
            }
            block_indent = None;
        }
        // A document marker is not a plain scalar. Properties on the same
        // line belong to the new root node and must receive the same guard.
        let content = if quote.is_none()
            && (line.starts_with("---") || line.starts_with("..."))
            && line.as_bytes().get(3).is_none_or(u8::is_ascii_whitespace)
        {
            &line[3..]
        } else {
            line
        };
        let mut plain = false;
        let mut quoted = false;
        let mut previous = None;
        let mut chars = content.chars().peekable();
        while let Some(c) = chars.next() {
            if let Some(q) = quote {
                if escaped {
                    escaped = false;
                } else if c == '\\' && q == '"' {
                    escaped = true;
                } else if c == q {
                    // YAML single quotes escape themselves by doubling.
                    if q == '\'' && chars.peek() == Some(&'\'') {
                        chars.next();
                    } else {
                        quote = None;
                        quoted = true;
                    }
                }
                continue;
            }
            if c == '#' && previous.is_none_or(char::is_whitespace) {
                break;
            }
            if c.is_whitespace() {
                previous = Some(c);
                continue;
            }
            if c == ':'
                && (quoted
                    || chars
                        .peek()
                        .is_none_or(|n| n.is_whitespace() || "[]{},".contains(*n)))
            {
                plain = false;
                quoted = false;
                previous = Some(c);
                continue;
            }
            if matches!(c, '[' | '{') && (!plain || flow > 0) {
                flow += 1;
                if flow > 64 {
                    return Err(Failure::Limit);
                }
                plain = false;
                quoted = false;
                previous = Some(c);
                continue;
            }
            if flow > 0 && matches!(c, ']' | '}' | ',') {
                if c != ',' {
                    flow = flow.saturating_sub(1);
                }
                plain = false;
                quoted = false;
                previous = Some(c);
                continue;
            }
            if !plain {
                if matches!(c, '&' | '*' | '!') {
                    return Err(Failure::Unsupported);
                }
                if matches!(c, '\'' | '"') {
                    quote = Some(c);
                    quoted = false;
                    previous = Some(c);
                    continue;
                }
                if matches!(c, '|' | '>') && flow == 0 {
                    block_indent = Some(indent);
                    break;
                }
                if matches!(c, '-' | '?') && chars.peek().is_none_or(|n| n.is_whitespace()) {
                    previous = Some(c);
                    continue;
                }
                plain = true;
                quoted = false;
            }
            previous = Some(c);
        }
    }
    Ok(())
}
