//! Bounded, known-syntax export redaction. This is not a universal secret detector.
use crate::{Finding, ScanResult};
use regex::Regex;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    sync::OnceLock,
};
const PRIVATE: &str = "[masked-private-key-material]";
const SENSITIVE: &str = "[masked-sensitive-content]";
const VALUE: &str = "[masked-sensitive-value]";
const JWK_PRIVATE_FIELDS: &[&str] = &["d", "p", "q", "dp", "dq", "qi", "k", "oth"];
const LABELS: &[&str] = &[
    "RSA PRIVATE KEY",
    "PRIVATE KEY",
    "OPENSSH PRIVATE KEY",
    "EC PRIVATE KEY",
    "DSA PRIVATE KEY",
    "ENCRYPTED PRIVATE KEY",
];
const NAMES: &[&str] = &[
    "password",
    "passwd",
    "secret",
    "token",
    "api_key",
    "apikey",
    "api_token",
    "access_token",
    "client_secret",
    "private_key",
    "key_material",
    "authorization",
    "secret_key",
    "secret_access_key",
];
fn assignment() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let names=NAMES.iter().map(|n|regex::escape(n).replace('_',"[_-]?")).collect::<Vec<_>>().join("|");
        let field=format!(r#"\b(?:[a-z0-9]+[_-])*(?:{names})\b"#);
        let pattern=format!(r#"(?i)(?P<prefix>(?:["']{field}["']\s*=>|["']?{field}["']?\s*[:=])\s*)(?P<value>"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|(?:Bearer|Basic)\s+[a-z0-9._~+/=-]+|[a-z0-9_+/=-]+)(?P<end>$|[\s,;&<>\]\}}\)])"#);
        Regex::new(&pattern).expect("fixed credential redaction regex")
    })
}
fn query_value() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let names = NAMES
            .iter()
            .map(|n| regex::escape(n).replace('_', "[_-]?"))
            .collect::<Vec<_>>()
            .join("|");
        Regex::new(&format!(
            r#"(?i)(?P<prefix>[?&](?:[a-z0-9]+[_-])*(?:{names})=)(?P<value>[^&#\s"'<>]+)"#
        ))
        .expect("fixed query redaction regex")
    })
}
fn userinfo() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)(?P<scheme>https?://)(?P<userinfo>[^/\s"'<>?#]+)@"#)
            .expect("fixed URL redaction regex")
    })
}
/// Whether a field name denotes a recognized credential/key value. This is
/// the same finite name policy used by export masking, including camel case.
pub fn is_sensitive_name(name: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let names = NAMES
            .iter()
            .map(|n| regex::escape(n).replace('_', "[_-]?"))
            .collect::<Vec<_>>()
            .join("|");
        Regex::new(&format!(r"(?i)^(?:[a-z0-9]+[_-])*(?:{names})$"))
            .expect("fixed sensitive name regex")
    })
    .is_match(name)
}
fn secret_name(name: &str) -> bool {
    // Preserve the legacy arbitrary-prefix metadata suffix policy, including
    // dotted and Unicode prefixes; source identifiers use the finite matcher.
    let n = name.to_ascii_lowercase().replace('-', "_");
    private_material_name(name) || NAMES.iter().any(|key| n.ends_with(&format!("_{key}")))
}

fn literal(value: &str) -> bool {
    // Preserve code expressions/keywords: these are not observed credential values.
    !matches!(
        value.to_ascii_lowercase().as_str(),
        "await" | "new" | "true" | "false" | "null" | "none" | "undefined"
    )
}
fn masked_value(value: &str) -> String {
    if value.starts_with('"') && value.ends_with('"') {
        format!("\"{VALUE}\"")
    } else if value.starts_with('\'') && value.ends_with('\'') {
        format!("'{VALUE}'")
    } else if value.to_ascii_lowercase().starts_with("bearer ")
        || value.to_ascii_lowercase().starts_with("basic ")
    {
        format!("{} {VALUE}", value.split_whitespace().next().unwrap_or(""))
    } else {
        VALUE.into()
    }
}
pub(crate) fn has_private_marker(text: &str) -> bool {
    if !text
        .as_bytes()
        .windows(11)
        .any(|s| s.eq_ignore_ascii_case(b"PRIVATE KEY"))
    {
        return false;
    }
    let upper = text.to_ascii_uppercase();
    LABELS.iter().any(|label| {
        upper.contains(&format!("BEGIN {label}")) || upper.contains(&format!("END {label}"))
    })
}
fn base64_like(text: &str) -> bool {
    let t = text.trim();
    t.len() >= 64
        && t.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"+/=-_".contains(&b))
        && (t.starts_with("MII") || t.ends_with('='))
}
/// Redact recognized markers/credential literals while retaining benign text.
/// Operator identifiers/paths and arbitrary opaque strings are outside this policy.
pub fn redact_text(input: &str) -> String {
    redact(input).into_owned()
}
fn redact(input: &str) -> Cow<'_, str> {
    if has_private_marker(input) {
        return Cow::Borrowed(PRIVATE);
    }
    if base64_like(input) {
        return Cow::Borrowed(SENSITIVE);
    }
    if !input.contains(['=', ':']) {
        return Cow::Borrowed(input);
    }
    let urls = if input.contains('@') && input.contains("://") {
        userinfo().replace_all(input, "${scheme}[masked-userinfo]@")
    } else {
        Cow::Borrowed(input)
    };
    let queries = query_value().replace_all(&urls, "${prefix}[masked-sensitive-value]");
    let structured = redact_structured(&queries);
    let replaced = assignment().replace_all(&structured, |c: &regex::Captures<'_>| {
        let value = &c["value"];
        if !literal(value) {
            return c[0].to_string();
        }
        format!("{}{}{}", &c["prefix"], masked_value(value), &c["end"])
    });
    match replaced {
        Cow::Owned(text) => Cow::Owned(text),
        Cow::Borrowed(_) => match structured {
            Cow::Owned(text) => Cow::Owned(text),
            Cow::Borrowed(_) => match queries {
                Cow::Owned(text) => Cow::Owned(text),
                Cow::Borrowed(_) => urls,
            },
        },
    }
}
fn quoted_end(text: &str, start: usize) -> usize {
    let bytes = text.as_bytes();
    let quote = bytes[start];
    let triple = bytes.get(start..start + 3).is_some_and(|s| s == [quote; 3]);
    let width = if triple { 3 } else { 1 };
    let mut i = start + width;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i = (i + 2).min(bytes.len());
        } else if bytes[i] == quote
            && (!triple || bytes.get(i..i + 3).is_some_and(|s| s == [quote; 3]))
        {
            return (i + width).min(bytes.len());
        } else {
            i += 1;
        }
    }
    bytes.len()
}

fn raw_quoted_region(text: &str, start: usize) -> Option<(usize, std::ops::Range<usize>)> {
    let bytes = text.as_bytes();
    let prefix = if bytes.get(start) == Some(&b'r') {
        1
    } else if matches!(bytes.get(start..start + 2), Some(b"br" | b"cr")) {
        2
    } else {
        return None;
    };
    let mut quote = start + prefix;
    while bytes.get(quote) == Some(&b'#') {
        quote += 1;
    }
    if bytes.get(quote) != Some(&b'"') {
        return None;
    }
    let hashes = quote - start - prefix;
    if hashes == 0 {
        return None;
    }
    let body = quote + 1;
    let mut end = body;
    while end < bytes.len() {
        if bytes[end] == b'"'
            && bytes
                .get(end + 1..end + 1 + hashes)
                .is_some_and(|tail| tail.iter().all(|&b| b == b'#'))
        {
            return Some((end + 1 + hashes, body..end));
        }
        end += 1;
    }
    Some((bytes.len(), body..bytes.len()))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SourceTokenKind {
    Word,
    Quoted,
    Number,
    CommentOpen,
    CommentBoundary,
    Symbol(u8),
}
struct SourceToken {
    kind: SourceTokenKind,
    span: std::ops::Range<usize>,
    close: Option<usize>,
    quoted_body: Option<std::ops::Range<usize>>,
}
fn mark_comment_literal_start(text: &str, mut value: usize, starts: &mut HashSet<usize>) {
    let tail = text[value..].trim_start_matches(char::is_whitespace);
    value = text.len() - tail.len();
    starts.insert(value);
    for wrapper in ["Buffer.from", "bytearray", "Uint8Array", "bytes", "b"] {
        if let Some(rest) = tail.strip_prefix(wrapper) {
            let trimmed = rest.trim_start_matches(char::is_whitespace);
            if trimmed.starts_with(['(', '\'', '"']) {
                starts.insert(value + tail.len() - trimmed.len());
                break;
            }
        }
    }
}
fn comment_literal_starts(text: &str) -> HashSet<usize> {
    static HEADERS: OnceLock<Regex> = OnceLock::new();
    let headers = HEADERS.get_or_init(|| {
        Regex::new(r#"(?P<name>"[a-zA-Z_][a-zA-Z0-9_-]*"|'[a-zA-Z_][a-zA-Z0-9_-]*'|\b[a-zA-Z_][a-zA-Z0-9_-]*)\s*[:=]\s*"#)
            .expect("fixed comment field grammar")
    });
    let mut starts = HashSet::new();
    for captures in headers.captures_iter(text) {
        let header = captures.get(0).unwrap();
        let name = captures["name"].trim_matches(['\'', '"']);
        if private_material_name(name)
            || JWK_PRIVATE_FIELDS.contains(&name)
            || matches!(name, "kty" | "n" | "e" | "x" | "y")
        {
            starts.insert(header.start());
            mark_comment_literal_start(text, header.end(), &mut starts);
        }
    }
    starts
}

struct SourceTokens {
    tokens: Vec<SourceToken>,
    comments: Vec<std::ops::Range<usize>>,
}
impl SourceTokens {
    fn new(text: &str, comments: bool) -> Self {
        let bytes = text.as_bytes();
        let mut result = Self {
            tokens: Vec::new(),
            comments: Vec::new(),
        };
        let mut stack = Vec::<(u8, usize, bool)>::new();
        let mut comment_literals = (!comments).then(|| comment_literal_starts(text));
        let mut comment_stacks = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i].is_ascii_whitespace() {
                i += 1;
                continue;
            }
            let start = i;
            let literal_context = comments
                || comment_literals.as_ref().is_some_and(|s| s.contains(&i))
                || stack.last().is_some_and(|(_, _, literal)| *literal);
            if let Some((end, body)) = literal_context
                .then(|| raw_quoted_region(text, i))
                .flatten()
            {
                result.tokens.push(SourceToken {
                    kind: SourceTokenKind::Quoted,
                    span: start..end,
                    close: None,
                    quoted_body: Some(body),
                });
                i = end;
                continue;
            }
            if comments && (bytes.get(i..i + 2) == Some(b"//") || bytes[i] == b'#') {
                i = bytes[i..]
                    .iter()
                    .position(|b| *b == b'\n')
                    .map_or(bytes.len(), |n| i + n);
                result.comments.push(start..i);
                continue;
            }
            if comments && bytes.get(i..i + 2) == Some(b"/*") {
                let mut depth = 1usize;
                i += 2;
                while depth > 0 && i < bytes.len() {
                    if bytes.get(i..i + 2) == Some(b"/*") {
                        depth += 1;
                        i += 2;
                    } else if bytes.get(i..i + 2) == Some(b"*/") {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                result.comments.push(start..i);
                continue;
            }
            let kind = match bytes[i] {
                b'/' if !comments && bytes.get(i..i + 2) == Some(b"/*") => {
                    i += 2;
                    SourceTokenKind::CommentOpen
                }
                b'*' if !comments && bytes.get(i..i + 2) == Some(b"*/") => {
                    i += 2;
                    SourceTokenKind::CommentBoundary
                }
                b'\'' | b'"' | b'`'
                    if comments
                        || comment_literals.as_ref().is_some_and(|s| s.contains(&i))
                        || stack.last().is_some_and(|(_, _, literal)| *literal) =>
                {
                    i = quoted_end(text, i);
                    SourceTokenKind::Quoted
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    i += 1;
                    while bytes
                        .get(i)
                        .is_some_and(|b| b.is_ascii_alphanumeric() || b"_-".contains(b))
                    {
                        i += 1;
                    }
                    SourceTokenKind::Word
                }
                b'0'..=b'9' | b'.'
                    if bytes[i].is_ascii_digit()
                        || bytes.get(i + 1).is_some_and(u8::is_ascii_digit) =>
                {
                    i += 1;
                    while bytes.get(i).is_some_and(|b| {
                        b.is_ascii_alphanumeric()
                            || b"_.".contains(b)
                            || (b"+-".contains(b) && matches!(bytes.get(i - 1), Some(b'e' | b'E')))
                    }) {
                        i += 1;
                    }
                    SourceTokenKind::Number
                }
                symbol => {
                    i += 1;
                    SourceTokenKind::Symbol(symbol)
                }
            };
            let index = result.tokens.len();
            result.tokens.push(SourceToken {
                kind,
                span: start..i,
                close: None,
                quoted_body: (kind == SourceTokenKind::Quoted).then(|| {
                    let quote = bytes[start];
                    let width = if bytes.get(start..start + 3) == Some(&[quote; 3]) {
                        3
                    } else {
                        1
                    };
                    let closed = i >= start + width * 2
                        && bytes.get(i - width..i) == Some(&[quote; 3][..width]);
                    start + width..i - if closed { width } else { 0 }
                }),
            });
            match kind {
                SourceTokenKind::CommentOpen => {
                    comment_stacks.push((index, std::mem::take(&mut stack)));
                }
                SourceTokenKind::CommentBoundary => {
                    if let Some((open, saved)) = comment_stacks.pop() {
                        result.tokens[open].close = Some(index);
                        if let Some(starts) = &mut comment_literals {
                            if starts.contains(&result.tokens[open].span.start) {
                                mark_comment_literal_start(text, i, starts);
                            }
                        }
                        stack = saved;
                    }
                }
                SourceTokenKind::Symbol(open @ (b'{' | b'[' | b'(')) => {
                    let literal = stack.last().is_some_and(|(_, _, literal)| *literal)
                        || comment_literals
                            .as_ref()
                            .is_some_and(|s| s.contains(&start));
                    stack.push((open, index, literal));
                }
                SourceTokenKind::Symbol(close @ (b'}' | b']' | b')')) => {
                    let open = match close {
                        b'}' => b'{',
                        b']' => b'[',
                        _ => b'(',
                    };
                    if stack.last().is_some_and(|(symbol, _, _)| *symbol == open) {
                        let (_, index, _) = stack.pop().unwrap();
                        result.tokens[index].close = Some(result.tokens.len() - 1);
                    } else {
                        stack.clear();
                    }
                }
                _ => {}
            }
        }
        result
    }
    fn symbol(&self, index: usize, symbol: u8) -> bool {
        self.tokens
            .get(index)
            .is_some_and(|t| t.kind == SourceTokenKind::Symbol(symbol))
    }
    fn skip_comments(&self, mut index: usize) -> usize {
        while self
            .tokens
            .get(index)
            .is_some_and(|t| t.kind == SourceTokenKind::CommentOpen)
        {
            index = self.tokens[index]
                .close
                .map_or(self.tokens.len(), |n| n + 1);
        }
        index
    }
    fn atom_end(&self, text: &str, start: usize) -> Option<usize> {
        let start = self.skip_comments(start);
        let token = self.tokens.get(start)?;
        match token.kind {
            SourceTokenKind::Quoted => Some(start + 1),
            SourceTokenKind::Number => {
                numeric_literal(&text[token.span.clone()]).then_some(start + 1)
            }
            SourceTokenKind::Symbol(b'-' | b'+') => {
                let next = self.tokens.get(start + 1)?;
                (next.kind == SourceTokenKind::Number
                    && token.span.end == next.span.start
                    && numeric_literal(&text[next.span.clone()]))
                .then_some(start + 2)
            }
            SourceTokenKind::Symbol(b'[' | b'{') => {
                Some(token.close.map_or(self.tokens.len(), |n| n + 1))
            }
            SourceTokenKind::Word if &text[token.span.clone()] == "b" => {
                let next = self.tokens.get(start + 1)?;
                (next.kind == SourceTokenKind::Quoted && token.span.end == next.span.start)
                    .then_some(start + 2)
            }
            _ => None,
        }
    }
    fn value_end(&self, text: &str, start: usize) -> Option<usize> {
        let start = self.skip_comments(start);
        let mut i = start;
        if self.tokens.get(i)?.kind == SourceTokenKind::Word {
            let wrapper = &text[self.tokens[i].span.clone()];
            if matches!(wrapper, "bytes" | "bytearray" | "Uint8Array") {
                i += 1;
            } else if wrapper == "Buffer"
                && self.symbol(i + 1, b'.')
                && self
                    .tokens
                    .get(i + 2)
                    .is_some_and(|t| &text[t.span.clone()] == "from")
            {
                i += 3;
            }
        }
        let end = if self.symbol(i, b'(') {
            let mut parentheses = 1usize;
            let mut expects_value = true;
            i += 1;
            loop {
                i = self.skip_comments(i);
                if i == self.tokens.len() {
                    return Some(text.len());
                }
                if self.symbol(i, b')') {
                    parentheses -= 1;
                    i += 1;
                    if parentheses == 0 {
                        break i;
                    }
                    expects_value = false;
                } else if expects_value && self.symbol(i, b'(') {
                    parentheses += 1;
                    i += 1;
                } else if expects_value {
                    i = self.atom_end(text, i)?;
                    expects_value = false;
                } else if self.symbol(i, b',') {
                    i += 1;
                    expects_value = true;
                } else {
                    return None;
                }
            }
        } else {
            if i != start {
                return None;
            }
            self.atom_end(text, start)?
        };
        let last = self.tokens.get(end.checked_sub(1)?)?;
        let first = &self.tokens[start];
        let offset = if end == self.tokens.len()
            && matches!(first.kind, SourceTokenKind::Symbol(b'{' | b'['))
            && first.close.is_none()
        {
            text.len()
        } else {
            last.span.end
        };
        let boundary = self.tokens.get(self.skip_comments(end)).is_none_or(|next| {
            text[offset..next.span.start].contains(['\n', '\r'])
                || matches!(
                    next.kind,
                    SourceTokenKind::CommentBoundary
                        | SourceTokenKind::Symbol(b',' | b';' | b'}' | b']' | b')')
                )
        });
        boundary.then_some(offset)
    }
}
fn numeric_literal(text: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(concat!(
            r"^(?:(?:",
            r"0[xX][0-9a-fA-F](?:_?[0-9a-fA-F])*|",
            r"0[bB][01](?:_?[01])*|0[oO][0-7](?:_?[0-7])*|",
            r"(?:[0-9](?:_?[0-9])*(?:\.[0-9](?:_?[0-9])*)?|\.[0-9](?:_?[0-9])*)(?:[eE][+-]?[0-9](?:_?[0-9])*)?|",
            r"[0-9](?:_?[0-9])*\.[eE][+-]?[0-9](?:_?[0-9])*",
            r")(?:[a-zA-Z_][a-zA-Z0-9_]*)?|[0-9](?:_?[0-9])*\.)$"
        ))
        .expect("fixed number-like literal token grammar")
    })
    .is_match(text)
}

fn private_material_name(name: &str) -> bool {
    let normalized: String = name
        .bytes()
        .filter(|b| !b"_-".contains(b))
        .map(|b| b.to_ascii_lowercase() as char)
        .collect();
    matches!(
        normalized.as_str(),
        "private"
            | "privatekey"
            | "privatekeyjwk"
            | "privatekeypem"
            | "privatekeypkcs8"
            | "privateexponent"
            | "privatekeybytes"
            | "privatescalar"
            | "privatescalarbytes"
            | "seedbytes"
            | "shared"
            | "sharedsecret"
            | "seed"
            | "scalar"
            | "secret"
    ) || is_sensitive_name(name)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InspectionRegion {
    Source,
    LiteralData,
    CommentData,
}

fn structured_private_ranges(text: &str) -> Vec<std::ops::Range<usize>> {
    let mut remaining_bytes = text.len().saturating_mul(8);
    structured_ranges(text, 0, InspectionRegion::Source, &mut remaining_bytes)
}
fn structured_ranges(
    text: &str,
    nesting: usize,
    region: InspectionRegion,
    remaining_bytes: &mut usize,
) -> Vec<std::ops::Range<usize>> {
    if text.len() > *remaining_bytes {
        return std::iter::once(0..text.len()).collect();
    }
    *remaining_bytes -= text.len();
    if region != InspectionRegion::Source {
        static PRINTED_PRIVATE_HEADER: OnceLock<Regex> = OnceLock::new();
        let header = PRINTED_PRIVATE_HEADER.get_or_init(|| {
            Regex::new(r"(?i)\bPrivate-Key[ \t]*:[^\r\n]*(?:[\r\n]+|\\[rn])+[ \t]*(?://|#|\*)?[ \t]*(?:priv|privateExponent|prime1|prime2|exponent1|exponent2|coefficient)[ \t]*:").expect("fixed printed private-key header")
        });
        static PRIVATE_HEADER: OnceLock<Regex> = OnceLock::new();
        static PRIVATE_COMPONENT_LINE: OnceLock<Regex> = OnceLock::new();
        let private_header = PRIVATE_HEADER.get_or_init(|| {
            Regex::new(r"(?i)\bPrivate-Key[ \t]*:").expect("fixed private-key header")
        });
        let private_component = PRIVATE_COMPONENT_LINE.get_or_init(|| {
            Regex::new(r"(?i)[\r\n][ \t]*(?:(?://|#|\*)[ \t]*)?(?:priv|privateExponent|prime1|prime2|exponent1|exponent2|coefficient)[ \t]*:").expect("fixed printed private component line")
        });
        if header.is_match(text)
            || private_header
                .find(text)
                .is_some_and(|header| private_component.is_match(&text[header.end()..]))
        {
            return std::iter::once(0..text.len()).collect();
        }
    }
    let scan_comments = region != InspectionRegion::CommentData;
    let lexed = SourceTokens::new(text, scan_comments);
    let tokens = &lexed.tokens;
    let mut ranges = Vec::new();
    let mut objects = Vec::<bool>::new();
    let mut stack = Vec::<Option<usize>>::new();
    let mut comment_objects = Vec::new();
    let mut short_fields = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        let name = match token.kind {
            SourceTokenKind::CommentOpen => {
                comment_objects.push(std::mem::take(&mut stack));
                i += 1;
                continue;
            }
            SourceTokenKind::CommentBoundary => {
                stack = comment_objects.pop().unwrap_or_default();
                i += 1;
                continue;
            }
            SourceTokenKind::Symbol(b'{') => {
                stack.push(Some(objects.len()));
                objects.push(false);
                i += 1;
                continue;
            }
            SourceTokenKind::Symbol(b'[' | b'(') => {
                stack.push(None);
                i += 1;
                continue;
            }
            SourceTokenKind::Symbol(b'}' | b']' | b')') => {
                stack.pop();
                i += 1;
                continue;
            }
            SourceTokenKind::Quoted => {
                let quoted = &text[token.span.clone()];
                if quoted.contains([':', '=']) {
                    if nesting >= 8 {
                        ranges.push(token.span.clone());
                    } else {
                        let body = token.quoted_body.as_ref().unwrap();
                        let (body_start, body_end) = (body.start, body.end);
                        ranges.extend(
                            structured_ranges(
                                &text[body_start..body_end],
                                nesting + 1,
                                InspectionRegion::LiteralData,
                                remaining_bytes,
                            )
                            .into_iter()
                            .map(|r| body_start + r.start..body_start + r.end),
                        );
                        if nesting < 2 && quoted.contains("\\\"") {
                            if let Ok(decoded) = serde_json::from_str::<String>(quoted) {
                                if !structured_ranges(
                                    &decoded,
                                    nesting + 1,
                                    InspectionRegion::LiteralData,
                                    remaining_bytes,
                                )
                                .is_empty()
                                {
                                    ranges.push(token.span.clone());
                                }
                            }
                        }
                    }
                }
                &text[token.quoted_body.as_ref().unwrap().clone()]
            }
            SourceTokenKind::Word => {
                let name = &text[token.span.clone()];
                if name.eq_ignore_ascii_case("Private-Key") && lexed.symbol(i + 1, b':') {
                    // A textual dump has no generic end marker. The enclosing
                    // quote/comment bounds this region; unbounded text ends at EOF.
                    ranges.push(token.span.start..text.len());
                    break;
                }
                name
            }
            _ => {
                i += 1;
                continue;
            }
        };
        i += 1;
        if !(lexed.symbol(i, b':') || lexed.symbol(i, b'='))
            || lexed.symbol(i + 1, b'=')
            || lexed.symbol(i + 1, b'>')
        {
            continue;
        }
        let value = i + 1;
        let object = stack.last().copied().flatten();
        if name == "kty" {
            if let Some(object) = object {
                objects[object] = true;
            }
        }
        let private = private_material_name(name);
        let short = JWK_PRIVATE_FIELDS.contains(&name);
        if private || (short && object.is_some()) {
            if let Some(end) = lexed.value_end(text, value) {
                let range = tokens[value].span.start..end;
                if private {
                    ranges.push(range);
                } else if let Some(object) = object {
                    short_fields.push((object, range));
                }
            }
        }
    }
    ranges.extend(
        short_fields
            .into_iter()
            .filter_map(|(object, range)| objects[object].then_some(range)),
    );
    if scan_comments {
        let mut comments = lexed.comments.into_iter().peekable();
        while let Some(mut region) = comments.next() {
            while comments.peek().is_some_and(|next| {
                text[region.end..next.start]
                    .bytes()
                    .all(|b| b.is_ascii_whitespace())
            }) {
                region.end = comments.next().unwrap().end;
            }
            ranges.extend(
                structured_ranges(
                    &text[region.clone()],
                    nesting,
                    InspectionRegion::CommentData,
                    remaining_bytes,
                )
                .into_iter()
                .map(|r| region.start + r.start..region.start + r.end),
            );
        }
    }
    ranges.sort_unstable_by_key(|r| (r.start, r.end));
    ranges
}

fn redact_structured(input: &str) -> Cow<'_, str> {
    let ranges = structured_private_ranges(input);
    if ranges.is_empty() {
        return Cow::Borrowed(input);
    }
    let mut out = String::new();
    let mut cursor = 0;
    for range in ranges {
        if range.start < cursor {
            continue;
        }
        out.push_str(&input[cursor..range.start]);
        out.push_str(VALUE);
        cursor = range.end;
    }
    out.push_str(&input[cursor..]);
    Cow::Owned(out)
}

/// Sanitize a complete inspected source snapshot before selecting report excerpts.
/// Line terminators are preserved; replacement text can change byte/column offsets.
/// This pure helper does not read or execute files.
pub fn redact_source_text(input: &str) -> String {
    let mut privacy = SourcePrivacy::from_text(input);
    let mut private_lines = std::mem::take(&mut privacy.private_lines)
        .into_iter()
        .peekable();
    let mut exact_private_lines = std::mem::take(&mut privacy.exact_private_lines)
        .into_iter()
        .peekable();
    let mut out = String::new();
    for (index, part) in input.split_inclusive('\n').enumerate() {
        let (line, terminator) = if let Some(line) = part.strip_suffix("\r\n") {
            (line, "\r\n")
        } else if let Some(line) = part.strip_suffix('\n') {
            (line, "\n")
        } else {
            (part, "")
        };
        let number = index + 1;
        for (lines, destination) in [
            (&mut private_lines, &mut privacy.private_lines),
            (&mut exact_private_lines, &mut privacy.exact_private_lines),
        ] {
            while lines.peek().is_some_and(|r| r.1 < number) {
                lines.next();
            }
            destination.clear();
            if let Some(range) = lines.peek().filter(|r| r.0 <= number) {
                destination.push(*range);
            }
        }
        let mut ranges = [
            (privacy.credentials.remove(&number).unwrap_or_default(), 0),
            (
                privacy
                    .export_credentials
                    .remove(&number)
                    .unwrap_or_default(),
                0,
            ),
        ];
        let whole_fragment = base64_like(line);
        let private_line =
            privacy.exact_private_lines.is_empty() && !privacy.private_lines.is_empty();
        let mut from = 0;
        for part in line
            .split_inclusive('\r')
            .chain(line.is_empty().then_some(""))
        {
            let content = part.strip_suffix('\r').unwrap_or(part);
            let to = from + content.len();
            for ((ranges, first), destination) in ranges
                .iter_mut()
                .zip([&mut privacy.credentials, &mut privacy.export_credentials])
            {
                while *first < ranges.len() && ranges[*first].3 - 1 <= from {
                    *first += 1;
                }
                let end = *first + ranges[*first..].iter().take_while(|r| r.2 - 1 < to).count();
                destination.insert(number, ranges[*first..end].to_vec());
            }
            if !whole_fragment
                && !private_line
                && privacy.credentials[&number].is_empty()
                && privacy.export_credentials[&number].is_empty()
                && base64_like(content)
            {
                out.push_str(content);
            } else {
                out.push_str(&privacy.redact_slice(number, line, from, to));
            }
            out.push_str(&part[content.len()..]);
            from += part.len();
        }
        out.push_str(terminator);
    }
    out
}

fn field(value: &mut String) {
    let replacement = redact(value);
    if replacement.as_ref() != value {
        *value = replacement.into_owned();
    }
}

fn evidence_field(value: &mut String) {
    // Retain the legacy writer's broader protection for evidence fragments,
    // including markers whose BEGIN/END prefix was lost before export.
    if value.to_ascii_uppercase().contains("PRIVATE KEY") {
        *value = PRIVATE.into();
    } else {
        field(value);
    }
}
fn already_masked(value: &str) -> bool {
    matches!(
        value,
        "[masked-private-key]" | PRIVATE | SENSITIVE | VALUE | "[masked-sensitive-file-content]"
    )
}
fn sanitize_finding(finding: &mut Finding, sensitive: bool) {
    if sensitive && !already_masked(&finding.evidence.r#match) {
        finding.evidence.r#match = SENSITIVE.into();
    }
    evidence_field(&mut finding.evidence.r#match);
    if sensitive {
        finding.evidence.snippet_preview = SENSITIVE.into();
    }
    evidence_field(&mut finding.evidence.snippet_preview);
    let jwk_metadata = finding.evidence.metadata.contains_key("kty");
    for (key, value) in &mut finding.evidence.metadata {
        if secret_name(key) || (jwk_metadata && JWK_PRIVATE_FIELDS.contains(&key.as_str())) {
            *value = masked_value(value);
        } else {
            field(value);
        }
    }
    field(&mut finding.description);
    field(&mut finding.migration_hint);
    if let Some(snippet) = &mut finding.source_snippet {
        for line in &mut snippet.lines {
            evidence_field(&mut line.text);
        }
    }
    for action in &mut finding.recommended_actions {
        field(&mut action.title);
        field(&mut action.rationale);
        for s in &mut action.steps {
            field(s);
        }
        for s in &mut action.references {
            field(s);
        }
        for example in &mut action.code_examples {
            field(&mut example.before);
            field(&mut example.after);
        }
    }
}
/// Defense in depth for caller-provided legacy results; structural fields stay intact.
pub fn sanitize_scan_result(result: &mut ScanResult) {
    sanitize_with_context(result, &HashSet::new());
}
pub(crate) fn sanitize_with_context(result: &mut ScanResult, sensitive_ids: &HashSet<String>) {
    // Preserve original CBOM rows/order. A sensitive occurrence always wins a
    // column-less legacy collision; later benign occurrences cannot unmask it.
    let mut replacements = HashMap::new();
    for f in &mut result.findings {
        let sensitive = sensitive_ids.contains(&f.finding_id) || f.evidence.r#type == "private_key";
        let key = (
            format!("{}:{}", f.location.file, f.location.line),
            f.category.clone(),
            f.evidence.r#match.clone(),
        );
        sanitize_finding(f, sensitive);
        if sensitive {
            replacements.insert(key, f.evidence.r#match.clone());
        }
    }
    for row in &mut result.cbom {
        if let Some(replacement) = replacements.get(&(
            row.location.clone(),
            row.usage_type.clone(),
            row.algorithm.clone(),
        )) {
            row.algorithm = replacement.clone();
        }
        field(&mut row.algorithm);
        field(&mut row.migration_hint);
    }
    for row in &mut result.dependency_sbom {
        field(&mut row.name);
        field(&mut row.version);
        field(&mut row.ecosystem);
        field(&mut row.source_type);
        field(&mut row.purl);
    }
}
#[derive(Debug, Default)]
pub(crate) struct SourcePrivacy {
    pub(crate) private_marker: bool,
    syntax_sensitive_context: bool,
    restricted_source_matches: bool,
    private_lines: Vec<(usize, usize)>,
    exact_private_lines: Vec<(usize, usize)>,
    // Only positions survive collection. Never retain payloads or secret digests.
    credentials: BTreeMap<usize, Vec<(usize, usize, usize, usize)>>,
    export_credentials: BTreeMap<usize, Vec<(usize, usize, usize, usize)>>,
    line_offsets: Vec<usize>,
    pub(crate) source_lines: Option<Vec<String>>,
}
impl SourcePrivacy {
    pub(crate) fn from_text(text: &str) -> Self {
        let mut out = Self {
            line_offsets: std::iter::once(0)
                .chain(text.match_indices('\n').map(|(i, _)| i + 1))
                .collect(),
            ..Self::default()
        };
        let mut active: Option<(usize, &str)> = None;
        let mut last = 0;
        for (index, line) in text.lines().enumerate() {
            let number = index + 1;
            last = number;
            if has_private_marker(line) {
                let upper = line.to_ascii_uppercase();
                if active.is_none() {
                    for &label in LABELS {
                        if upper.contains(&format!("BEGIN {label}")) {
                            active = Some((number, label));
                            out.private_marker = true;
                            break;
                        }
                    }
                }
                let was_active = active.is_some();
                if let Some((start, label)) = active {
                    if upper.contains(&format!("END {label}")) {
                        out.private_lines.push((start, number));
                        active = None;
                    }
                }
                if LABELS
                    .iter()
                    .any(|label| upper.contains(&format!("END {label}")))
                {
                    out.private_marker = true;
                    if !was_active {
                        let start = out.private_lines.last().map_or(1, |r| r.1 + 1);
                        out.private_lines.push((start, number));
                    }
                }
            }
            let mut ranges: Vec<_> = assignment()
                .captures_iter(line)
                .filter(|c| literal(&c["value"]))
                .map(|c| {
                    let m = c.name("value").unwrap();
                    (m.start(), m.end())
                })
                .collect();
            ranges.extend(query_value().captures_iter(line).map(|c| {
                let m = c.name("value").unwrap();
                (m.start(), m.end())
            }));
            ranges.extend(userinfo().captures_iter(line).map(|c| {
                let m = c.name("userinfo").unwrap();
                (m.start(), m.end())
            }));
            ranges.sort_unstable();
            let mut merged: Vec<(usize, usize)> = Vec::new();
            for (a, b) in ranges {
                if let Some(last) = merged.last_mut() {
                    if a <= last.1 {
                        last.1 = last.1.max(b);
                        continue;
                    }
                }
                merged.push((a, b));
            }
            let (mut offset, mut column) = (0, 1);
            let mut columns = Vec::new();
            for (a, b) in merged {
                column += line[offset..a].chars().count();
                let start = column;
                column += line[a..b].chars().count();
                columns.push((start, column, a + 1, b + 1));
                offset = b;
            }
            if !columns.is_empty() {
                out.credentials.insert(number, columns);
            }
        }
        if let Some((start, _)) = active {
            out.private_lines.push((start, last));
        }
        // Exact PEM bounds let public code on the same line remain visible.
        let mut private_spans = Vec::new();
        let mut active: Option<(usize, String)> = None;
        static MARKERS: OnceLock<Regex> = OnceLock::new();
        let markers = MARKERS.get_or_init(|| {
            Regex::new(&format!(
                r"(?i)-{{2,}}(?:BEGIN|END) (?:{})-{{2,}}",
                LABELS.join("|")
            ))
            .expect("fixed private marker regex")
        });
        for marker in markers.find_iter(text) {
            let upper = marker.as_str().to_ascii_uppercase();
            if upper.contains("BEGIN ") {
                if active.is_none() {
                    let label = LABELS
                        .iter()
                        .find(|label| upper.contains(&format!("BEGIN {label}")))
                        .unwrap();
                    active = Some((marker.start(), (*label).into()));
                }
            } else if let Some((start, label)) = &active {
                if upper.contains(&format!("END {label}")) {
                    private_spans.push(*start..marker.end());
                    active = None;
                }
            } else {
                private_spans.push(
                    private_spans
                        .last()
                        .map_or(0, |r: &std::ops::Range<usize>| r.end)
                        ..marker.end(),
                );
            }
        }
        if let Some((start, _)) = active {
            private_spans.push(start..text.len());
        }
        out.exact_private_lines = private_spans
            .iter()
            .map(|r| {
                (
                    out.line_offsets.partition_point(|&o| o <= r.start),
                    out.line_offsets.partition_point(|&o| o < r.end),
                )
            })
            .collect();
        out.add_sensitive_ranges(text, &private_spans);
        // Collect whole-input quoted values too: a value may span source lines.
        let multiline: Vec<_> = assignment()
            .captures_iter(text)
            .filter(|c| literal(&c["value"]))
            .filter_map(|c| c.name("value").map(|v| v.range()))
            .collect();
        out.add_sensitive_ranges(text, &multiline);
        let mut presentation = Self::default();
        presentation.add_sensitive_ranges(text, &structured_private_ranges(text));
        out.export_credentials = presentation.credentials;
        out
    }
    /// Redact against the original inspected snapshot before any clipping. The
    /// cache contains sanitized text only; callers never reload changed files.
    pub(crate) fn capture_source(&mut self, text: &str) {
        if self.restricted_source_matches {
            return;
        }
        self.source_lines = Some(
            text.lines()
                .enumerate()
                .map(|(i, line)| self.redact_slice(i + 1, line, 0, line.len()))
                .collect(),
        );
    }
    fn redact_slice(&self, number: usize, line: &str, from: usize, to: usize) -> String {
        if !self
            .exact_private_lines
            .iter()
            .any(|&(a, b)| a <= number && number <= b)
            && self
                .private_lines
                .iter()
                .any(|&(a, b)| a <= number && number <= b)
        {
            return PRIVATE.into();
        }
        let mut out = String::new();
        let mut cursor = from;
        let mut ranges: Vec<_> = self
            .credentials
            .get(&number)
            .into_iter()
            .flatten()
            .chain(self.export_credentials.get(&number).into_iter().flatten())
            .copied()
            .collect();
        ranges.sort_unstable();
        {
            for (_, _, a, b) in ranges {
                let a = (a - 1).max(from);
                let b = (b - 1).min(to);
                if a >= b || b <= cursor {
                    continue;
                }
                out.push_str(&line[cursor..a.max(cursor)]);
                out.push_str(VALUE);
                cursor = b;
            }
        }
        out.push_str(&line[cursor..to]);
        redact_text(&out)
    }
    pub(crate) fn source_excerpt(&self, text: &str, start: usize, end: usize) -> Option<String> {
        if self.restricted_source_matches {
            return Some(SENSITIVE.into());
        }
        if start >= end
            || end > text.len()
            || !text.is_char_boundary(start)
            || !text.is_char_boundary(end)
        {
            return None;
        }
        let first = self
            .line_offsets
            .partition_point(|&o| o <= start)
            .saturating_sub(1);
        let last = self
            .line_offsets
            .partition_point(|&o| o < end)
            .saturating_sub(1);
        let mut lines = Vec::new();
        for i in first..=last {
            let offset = self.line_offsets[i];
            let limit = self.line_offsets.get(i + 1).copied().unwrap_or(text.len());
            let line = &text[offset..limit];
            lines.push(self.redact_slice(
                i + 1,
                line,
                start.max(offset) - offset,
                end.min(limit) - offset,
            ));
        }
        Some(lines.join("").chars().take(240).collect())
    }
    pub(crate) fn add_sensitive_ranges(&mut self, text: &str, ranges: &[std::ops::Range<usize>]) {
        if ranges.is_empty() {
            return;
        }
        let mut ranges: Vec<_> = ranges
            .iter()
            .filter(|r| {
                r.start < r.end
                    && r.end <= text.len()
                    && text.is_char_boundary(r.start)
                    && text.is_char_boundary(r.end)
            })
            .cloned()
            .collect();
        ranges.sort_unstable_by_key(|r| (r.start, r.end));
        // Merge first: nested calls must not multiply line/context work.
        let mut merged: Vec<std::ops::Range<usize>> = Vec::new();
        for range in ranges {
            if let Some(last) = merged.last_mut() {
                if range.start <= last.end {
                    last.end = last.end.max(range.end);
                    continue;
                }
            }
            merged.push(range);
        }
        self.syntax_sensitive_context |= !merged.is_empty();
        let (mut offset, mut first) = (0, 0);
        for (index, line) in text.split_inclusive('\n').enumerate() {
            let end = offset + line.len();
            while first < merged.len() && merged[first].end <= offset {
                first += 1;
            }
            for range in merged[first..].iter().take_while(|r| r.start < end) {
                let a = range.start.max(offset) - offset;
                let b = range.end.min(end) - offset;
                let start = line[..a].chars().count() + 1;
                let stop = start + line[a..b].chars().count();
                self.credentials
                    .entry(index + 1)
                    .or_default()
                    .push((start, stop, a + 1, b + 1));
            }
            offset = end;
        }
        for ranges in self.credentials.values_mut() {
            ranges.sort_unstable();
            let mut merged: Vec<(usize, usize, usize, usize)> = Vec::new();
            for &(a, b, c, d) in ranges.iter() {
                if let Some(last) = merged.last_mut() {
                    if a <= last.1 {
                        last.1 = last.1.max(b);
                        last.3 = last.3.max(d);
                        continue;
                    }
                }
                merged.push((a, b, c, d));
            }
            *ranges = merged;
        }
    }
    pub(crate) fn protect_source_context(&mut self) {
        self.syntax_sensitive_context = true;
        self.restricted_source_matches = true;
    }
    pub(crate) fn restricts_source_matches(&self) -> bool {
        self.restricted_source_matches
    }
    pub(crate) fn masks_source_context(&self) -> bool {
        self.private_marker || self.syntax_sensitive_context
    }
    pub(crate) fn sensitive(&self, line: usize, column: usize, mixed_ast: bool) -> bool {
        if self.restricted_source_matches {
            return true;
        }
        if line == 0 {
            return self.masks_source_context() || !self.credentials.is_empty();
        }
        let i = self.private_lines.partition_point(|r| r.1 < line);
        if self.private_lines.get(i).is_some_and(|r| r.0 <= line) {
            return true;
        }
        self.credentials.get(&line).is_some_and(|ranges| {
            if column == 0 {
                return true;
            }
            let i = ranges.partition_point(
                |&(_, end, _, byte_end)| if mixed_ast { byte_end } else { end } <= column,
            );
            ranges
                .get(i)
                .is_some_and(|&(start, _, _, _)| column >= start)
        })
    }
    pub(crate) fn protects_preview(&self) -> bool {
        self.masks_source_context()
            || !self.credentials.is_empty()
            || !self.export_credentials.is_empty()
    }
    pub(crate) fn sensitive_match(
        &self,
        line: usize,
        column: usize,
        mixed: bool,
        value: &str,
    ) -> bool {
        self.sensitive(line, column, mixed)
            || (line == 0 && !self.export_credentials.is_empty())
            || self.export_credentials.get(&line).is_some_and(|ranges| {
                column == 0
                    || ranges.iter().any(|&(start, end, byte_start, byte_end)| {
                        let (start, end) = if mixed {
                            (start.min(byte_start), end.max(byte_end))
                        } else {
                            (start, end)
                        };
                        column < end && start < column.saturating_add(value.chars().count())
                    })
            })
            || self.credentials.get(&line).is_some_and(|ranges| {
                value.contains("***")
                    || ranges.iter().any(|&(start, _, _, _)| {
                        column <= start && start < column.saturating_add(value.chars().count())
                    })
            })
    }
    pub(crate) fn is_empty(&self) -> bool {
        !self.masks_source_context()
            && self.credentials.is_empty()
            && self.export_credentials.is_empty()
            && self.source_lines.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_rsa_dump_public_components_cannot_split_private_owner() {
        for component in [
            "priv",
            "privateExponent",
            "prime1",
            "prime2",
            "exponent1",
            "exponent2",
            "coefficient",
        ] {
            for (open, close) in [
                ("r#\"", "\"#"),
                ("br###\"", "\"###"),
                ("cr#\"", "\"#"),
                ("\"\"\"", "\"\"\""),
                ("`", "`"),
                ("/*", "*/"),
            ] {
                for quote_before_label in [false, true] {
                    let (before, after) = if quote_before_label {
                        ("\"\r\n", "")
                    } else {
                        ("", "\"\r\n")
                    };
                    let body = format!("prose \"Private-Key: (2048 bit)\r\nmodulus:\r\n  00:11\r\npublicExponent: 65537\r\n{before}{component}:\r\n{after}  11:22:33:44\r\n  55:66:77:88\r\n");
                    let source = format!("公開 PREFIX; {open}{body}{close};\r\nPUBLIC_TAIL");
                    for output in [redact_text(&source), redact_source_text(&source)] {
                        for private in ["11:22:33:44", "55:66:77:88"] {
                            assert!(
                                !output.contains(private),
                                "{open}/{component}/{quote_before_label}: {output}"
                            );
                        }
                        assert!(output.contains("公開 PREFIX"));
                        assert!(output.contains("PUBLIC_TAIL"));
                        assert_eq!(redact_text(&output), output);
                        assert_eq!(redact_source_text(&output), output);
                    }
                    let output = redact_source_text(&source);
                    assert_eq!(
                        output.matches("\r\n").count(),
                        source.matches("\r\n").count()
                    );
                }
            }
        }
        let source = "/*\r\n * prose \"PRIVATE-KEY: (2048 bit)\r\n * modulus:\r\n *   00:11\r\n * publicExponent: 65537\r\n * \"\r\n * PRIME1:\r\n *   11:22:33:44\r\n *   55:66:77:88\r\n */\r\nPUBLIC_TAIL";
        for output in [redact_text(source), redact_source_text(source)] {
            assert!(!output.contains("11:22:33:44"));
            assert!(!output.contains("55:66:77:88"));
            assert!(output.contains("PUBLIC_TAIL"));
        }
    }

    #[test]
    fn owned_rsa_dump_requires_later_private_component_in_same_owner() {
        for body in [
            "OWNER_PREFIX prose \"Private-Key: (2048 bit)\nmodulus:\n  00:11\npublicExponent: 65537\n\"\nOWNER_SUFFIX",
            "OWNER_PREFIX\nprime1:\n  aa:bb:cc:dd\nprose \"Private-Key: (2048 bit)\nmodulus:\n  00:11\n\"\nOWNER_SUFFIX",
        ] {
            let source = format!("r#\"{body}\"#;\nPUBLIC_TAIL");
            for output in [redact_text(&source), redact_source_text(&source)] {
                assert!(output.contains("OWNER_PREFIX"));
                assert!(output.contains("OWNER_SUFFIX"));
                assert!(output.contains("PUBLIC_TAIL"));
                if body.contains("aa:bb:cc:dd") { assert!(output.contains("aa:bb:cc:dd")); }
            }
        }
        let separate = "first=r#\"OWNER_PREFIX prose 'Private-Key: (2048 bit)\nmodulus:\n  00:11\n'\nOWNER_SUFFIX\"#;\nsecond=r#\"prime1:\n  aa:bb:cc:dd\"#;\nPUBLIC_TAIL";
        for output in [redact_text(separate), redact_source_text(separate)] {
            assert!(output.contains("OWNER_PREFIX"));
            assert!(output.contains("OWNER_SUFFIX"));
            assert!(output.contains("aa:bb:cc:dd"));
            assert!(output.contains("PUBLIC_TAIL"));
        }
        let public = "r#\"prose \"Public-Key: (2048 bit)\nmodulus:\n  00:11\npublicExponent: 65537\n\"\nprime1:\n  aa:bb:cc:dd\n\"#; PUBLIC_TAIL";
        assert_eq!(redact_text(public), public);
        assert_eq!(redact_source_text(public), public);
    }

    #[test]
    fn owned_dump_data_keeps_its_outer_boundary_despite_inner_quotes() {
        let body = "prose \"Private-Key:\npriv:\n\"\n  11:22:33:44\n";
        for (open, close) in [
            ("r#\"", "\"#"),
            ("br###\"", "\"###"),
            ("cr#\"", "\"#"),
            ("\"\"\"", "\"\"\""),
            ("`", "`"),
            ("/*", "*/"),
        ] {
            for body in [body.to_owned(), body.replace('\n', "\\n")] {
                let source = format!("公開 PREFIX; {open}{body}{close};\nPUBLIC_TAIL");
                let privacy = SourcePrivacy::from_text(&source);
                assert!(privacy.credentials.is_empty());
                assert!(privacy.private_lines.is_empty());
                assert!(!privacy.private_marker);
                assert!(!privacy.sensitive(3, 1, false));
                for output in [redact_text(&source), redact_source_text(&source)] {
                    assert!(!output.contains("11:22:33:44"), "{open}");
                    assert!(output.contains("PUBLIC_TAIL"), "{open}");
                    assert!(output.contains("公開 PREFIX"), "{open}");
                    assert_eq!(redact_text(&output), output);
                    assert_eq!(redact_source_text(&output), output);
                }
                let public = source
                    .replace("Private-Key", "Public-Key")
                    .replace("priv:", "pub:");
                for output in [redact_text(&public), redact_source_text(&public)] {
                    assert!(output.contains("11:22:33:44"), "{open}");
                    assert!(output.contains("PUBLIC_TAIL"), "{open}");
                }
            }
        }
    }

    #[test]
    fn printed_private_dump_scopes_hide_bodies_without_changing_native_suppression() {
        let body = "Private-Key: (256 bit)\r\npriv:\r\n    11:22:33:44:55:66:77:88:99:aa:bb:cc:dd:ee:fa:01\r\npub:\r\n    04:ca:fe:00\r\n";
        for heading in [
            "Private-Key: (256 bit)",
            "ED25519 Private-Key:",
            "private-key: (256 bit)",
        ] {
            let dump = body.replace("Private-Key: (256 bit)", heading);
            for (open, close) in [
                ("\"", "\""),
                ("r#\"", "\"#"),
                ("br###\"", "\"###"),
                ("cr#\"", "\"#"),
                ("\"\"\"", "\"\"\""),
                ("'", "'"),
                ("`", "`"),
            ] {
                let source = format!("PUBLIC_PREFIX\r\nconst expected = {open}{dump}{close};\r\nPUBLIC_TAIL = '04:aa:bb:cc';\r\n");
                let privacy = SourcePrivacy::from_text(&source);
                assert!(
                    privacy.credentials.is_empty(),
                    "presentation must not change legacy credential ranges"
                );
                assert!(privacy.private_lines.is_empty());
                assert!(!privacy.private_marker);
                assert!(!privacy.sensitive(4, 5, false));
                assert!(!privacy.export_credentials.is_empty());
                for output in [redact_text(&source), redact_source_text(&source)] {
                    assert!(
                        !output.contains("11:22:33:44"),
                        "private dump survived for {heading}/{open}: {output}"
                    );
                    assert!(output.contains("PUBLIC_PREFIX"));
                    assert!(output.contains("PUBLIC_TAIL"));
                    assert!(output.contains("04:aa:bb:cc"));
                    assert_eq!(redact_text(&output), output);
                }
                let output = redact_source_text(&source);
                assert_eq!(
                    output.matches("\r\n").count(),
                    source.matches("\r\n").count()
                );
                assert_eq!(redact_source_text(&output), output);
            }
        }
        let escaped = r#"const expected = "ED25519 Private-Key:\npriv:\n    11:22:33:44\npub:\n    04:ca:fe"; PUBLIC_TAIL();"#;
        for output in [redact_text(escaped), redact_source_text(escaped)] {
            assert!(!output.contains("11:22:33:44"));
            assert!(output.contains("PUBLIC_TAIL"));
        }
        for source in [
            "PUBLIC_PREFIX\nPrivate-Key: (256 bit)\npriv:\n  11:22:33:44\n  55:66:77:88",
            "PUBLIC_PREFIX\n/* Note: \" prose\nPrivate-Key: (256 bit)\npriv:\n  11:22:33:44\n*/ PUBLIC_TAIL();",
            "// Private-Key: (256 bit)\n// priv:\n//   11:22:33:44\nPUBLIC_TAIL();",
            "/*\r\n * Private-Key: (256 bit)\r\n * priv:\r\n *   11:22:33:44\r\n */ PUBLIC_TAIL();",
        ] {
            let output = redact_source_text(source);
            assert!(!output.contains("11:22:33:44"));
            assert!(!output.contains("55:66:77:88"));
            if source.contains("PUBLIC_TAIL") { assert!(output.contains("PUBLIC_TAIL")); }
            assert_eq!(redact_source_text(&output), output);
        }
        for public in [
            "Public-Key: (256 bit)\npub:\n    11:22:33:44\nPUBLIC_TAIL",
            "ED25519 Public-Key:\npub:\n    11:22:33:44\nPUBLIC_TAIL",
            "Certificate:\n    Signature Value:\n        11:22:33:44",
            "let fingerprint = \"11:22:33:44\";",
        ] {
            assert_eq!(redact_text(public), public);
            assert_eq!(redact_source_text(public), public);
        }
    }
    #[test]
    fn caller_jwk_metadata_masks_complete_other_primes_only_in_its_own_context() {
        let oth =
            r#"[{"r":"SYNTH_V8_PRIVATE_R","d":"SYNTH_V8_PRIVATE_D","t":"SYNTH_V8_PRIVATE_T"}]"#;
        let ordinary = r#"[{"r":"PUBLIC_R","d":"PUBLIC_D","t":"PUBLIC_T"}]"#;
        let mut result: ScanResult = serde_json::from_value(serde_json::json!({
            "generated_at":"2026-11-10T00:00:00Z",
            "findings":[{"finding_id":"metadata", "rule_id":"CONTROL", "category":"Control", "risk":"quantum-uncertain", "severity":"low", "confidence":0.5, "description":"Control", "migration_hint":"Review", "location":{"file":"a.txt","line":1,"column":1}, "evidence":{"type":"regex_match", "match":"RS256", "snippet_preview":"public", "metadata":{"kty":"RSA", "oth":oth, "n":"PUBLIC_N", "e":"AQAB", "r":"PUBLIC_R", "t":"PUBLIC_T"}}}],
            "cbom":[], "dependency_sbom":[], "summary":{"total_findings":1,"by_severity":{},"by_risk":{},"scanned_files":1,"skipped_files":0}
        })).unwrap();
        let mut public = result.findings[0].clone();
        public.finding_id = "ordinary-metadata".into();
        public.evidence.metadata.remove("kty");
        public
            .evidence
            .metadata
            .insert("oth".into(), ordinary.into());
        let public_before = serde_json::to_value(&public).unwrap();
        result.findings.push(public);
        let mut expected = serde_json::to_value(&result).unwrap();
        expected["findings"][0]["evidence"]["metadata"]["oth"] = VALUE.into();
        sanitize_scan_result(&mut result);
        assert_eq!(result.findings[0].evidence.metadata["oth"], VALUE);
        assert!(!serde_json::to_string(&result)
            .unwrap()
            .contains("SYNTH_V8_PRIVATE"));
        assert_eq!(serde_json::to_value(&result).unwrap(), expected);
        assert_eq!(
            serde_json::to_value(&result.findings[1]).unwrap(),
            public_before
        );
        sanitize_scan_result(&mut result);
        assert_eq!(serde_json::to_value(&result).unwrap(), expected);
    }

    #[test]
    fn serialized_private_containers_protect_all_presentation_callers() {
        let body = r#"{"kty":"RSA","d":"SYNTH_V7_PRIVATE_D","oth":[{"r":"SYNTH_V7_PRIVATE_R","d":"SYNTH_V7_PRIVATE_OTHER_D","t":"SYNTH_V7_PRIVATE_T"}],"n":"PUBLIC_N","e":"AQAB"}"#;
        for quote in ["'", "`", "\"\"\"", "'''"] {
            for prefix in ["", "r", "b"] {
                let source =
                    format!("const jwk = {prefix}{quote}{body}{quote}; // 日本語\r\nPUBLIC_TAIL");
                for output in [redact_text(&source), redact_source_text(&source)] {
                    assert!(!output.contains("SYNTH_V7_PRIVATE"));
                    for public in ["PUBLIC_N", "AQAB", "PUBLIC_TAIL", "日本語"] {
                        assert!(output.contains(public));
                    }
                    assert_eq!(output.matches("\r\n").count(), 1);
                    assert_eq!(redact_source_text(&output), output);
                }
                let mut result: ScanResult = serde_json::from_value(serde_json::json!({
                    "generated_at":"2026-11-10T00:00:00Z", "findings":[{
                        "finding_id":"stable-id", "rule_id":"SYNTHETIC", "category":"TLS",
                        "risk":"quantum-vulnerable", "severity":"high", "confidence":0.75,
                        "description":source, "migration_hint":"public",
                        "location":{"file":"synthetic.js","line":1,"column":1},
                        "evidence":{"type":"regex_match","match":"RS256","snippet_preview":source,"metadata":{}},
                        "recommended_actions":[],
                        "source_snippet":{"lines":[{"line":1,"text":source,"highlighted":true}]}}],
                    "cbom":[], "dependency_sbom":[],
                    "summary":{"total_findings":1,"by_severity":{"high":1},"by_risk":{"quantum-vulnerable":1},"scanned_files":1,"skipped_files":0}
                })).unwrap();
                sanitize_scan_result(&mut result);
                let output = serde_json::to_string(&result).unwrap();
                assert!(!output.contains("SYNTH_V7_PRIVATE"));
                assert!(output.contains("PUBLIC_N"));
                assert_eq!(result.findings[0].finding_id, "stable-id");
                assert_eq!(result.findings[0].evidence.r#match, "RS256");
            }
        }
        for source in [
            r#"const jwk = '{"kty":"RSA","n":"PUBLIC_N","e":"AQAB"}';"#,
            r#"const key = `{kty:"EC",r:"PUBLIC_R",t:"PUBLIC_T",x:"PUBLIC_X",y:"PUBLIC_Y"}`;"#,
            r#"{metadata:{kty:"RSA"},oth:[{r:"PUBLIC_R",t:"PUBLIC_T"}]}"#,
            r#"[{kty:"RSA"},{oth:[{r:"PUBLIC_R",t:"PUBLIC_T"}]}]"#,
            r#"description = "scalar = a_public_reference";"#,
        ] {
            assert_eq!(redact_text(source), source);
            assert_eq!(redact_source_text(source), source);
        }
    }

    #[test]
    fn serialized_private_data_remains_bounded_and_fail_closed() {
        let mut source = r#"{"kty":"RSA","d":"SYNTH_V7_DEEP_PRIVATE"}"#.to_string();
        for _ in 0..12 {
            source = serde_json::to_string(&source).unwrap();
            let output = redact_source_text(&source);
            assert!(!output.contains("SYNTH_V7_DEEP_PRIVATE"));
            assert_eq!(redact_source_text(&output), output);
        }
    }

    #[test]
    fn structured_private_values_are_masked_without_changing_public_facts() {
        for source in [
            r#"{"kty":"RSA","n":"PUBLIC_MODULUS","e":"AQAB","d":"SYNTH098_PRIVATE_D","p":"SYNTH098_PRIVATE_P","q":"SYNTH098_PRIVATE_Q","dp":"SYNTH098_PRIVATE_DP","dq":"SYNTH098_PRIVATE_DQ","qi":"SYNTH098_PRIVATE_QI"}"#,
            r#"{"d":"SYNTH098_PRIVATE_D","kty":"EC","x":"PUBLIC_X","crv":"P-256"}"#,
            r#"{"kty":"oct","k":"SYNTH098_SHARED_K"}"#,
            "private = {'nested': ['SYNTH098_PRIVATE_NESTED', 98217364198217364]}\nRSA_new();",
            "shared = bytes([98, 21, 73, 64, 19])\nRSA_new();",
            "seed = [98, 21, 73, 64, 19]\nscalar = 'SYNTH098_SCALAR';",
            "privateExponent = 98217364198217364; RSA_new();",
            "privateKeyJwk = { nested: { scalar: 'SYNTH098_NESTED' } }; RSA_new();",
            r#"serialized = "{\"kty\":\"RSA\",\"d\":\"SYNTH098_ESCAPED_D\"}"; RSA_new();"#,
            "private = \"\"\"SYNTH098_FIRST\nSYNTH098_SECOND\"\"\"\nRSA_new();",
        ] {
            let output = redact_text(source);
            assert!(
                !output.contains("SYNTH098"),
                "synthetic private value escaped"
            );
            assert!(!output.contains("98217364198217364"));
            if source.starts_with("shared = bytes") {
                assert!(!output.contains("98, 21, 73, 64, 19"));
            }
        }
        for source in [
            "let d = 17; let p = 23; let q = 31;",
            "private = true; secret = false; shared = None;",
            r#"{"d":"PUBLIC_PARAMETER","p":23,"q":31}"#,
            r#"{"kty":"RSA","n":"PUBLIC_MODULUS","e":"AQAB","key_bits":2048}"#,
            "private = derive(public_input); shared = public_reference; seed = new Generator();",
            "password = (((Lookup(\"PUBLIC_ENV_NAME\"))));",
            "private_key_algorithm=RSA; key_bits=2048; signature_oid=1.2.840.113549.1.1.11;",
        ] {
            assert_eq!(redact_text(source), source);
        }
    }

    #[test]
    fn reviewed_lexical_privacy_and_computed_expression_cases() {
        for (id, source, private) in [
            (
                r###"jwk-public-contrast"###,
                r###"{"kty":"RSA","d":"SYNTH_REVIEW_PRIVATE","n":"PUBLIC_N","e":"AQAB"}"###,
                true,
            ),
            (
                r###"jwk-before-kty"###,
                r###"{"d":"SYNTH_REVIEW_PRIVATE","kty":"EC","x":"PUBLIC_X"}"###,
                true,
            ),
            (
                r###"unrelated-d"###,
                r###"let d="SYNTH_REVIEW_PUBLIC";"###,
                false,
            ),
            (
                r###"public-call"###,
                r###"password = (((Lookup("SYNTH_REVIEW_PUBLIC"))));"###,
                false,
            ),
            (
                r###"computed-numeric-first"###,
                r###"private = (17 + Lookup("SYNTH_REVIEW_PUBLIC"));"###,
                false,
            ),
            (
                r###"computed-string-first"###,
                r###"private = ("SYNTH_REVIEW_PUBLIC" + Lookup("ENV"));"###,
                false,
            ),
            (
                r###"comment-apostrophe"###,
                r###"// don't disclose values
scalar = 'SYNTH_REVIEW_PRIVATE';
"###,
                true,
            ),
            (
                r###"comment-doublequote"###,
                r###"// a "quote
scalar = "SYNTH_REVIEW_PRIVATE";
"###,
                true,
            ),
            (
                r###"comment-block"###,
                r###"scalar = /* comment */ "SYNTH_REVIEW_PRIVATE";"###,
                true,
            ),
            (
                r###"private-multiline"###,
                r###"private = {
"nested":["SYNTH_REVIEW_PRIVATE"]
}
PUBLIC"###,
                true,
            ),
            (
                r###"jwk-same-object"###,
                r###"{"metadata":{"kty":"RSA"},"d":"SYNTH_REVIEW_PUBLIC"}"###,
                false,
            ),
            (
                r###"jwk-sibling"###,
                r###"[{"kty":"RSA"},{"d":"SYNTH_REVIEW_PUBLIC"}]"###,
                false,
            ),
            (
                r###"number-java-long"###,
                r###"scalar = 98217364198217364L;"###,
                true,
            ),
            (
                r###"number-js-bigint"###,
                r###"scalar = 98217364198217364n;"###,
                true,
            ),
            (
                r###"comment-inside-container"###,
                r###"private = { /* } */ value: "SYNTH_REVIEW_PRIVATE" };"###,
                true,
            ),
            (
                r###"utf16"###,
                r###"scalar = "SYNTH_REVIEW_PRIVATE";
"###,
                true,
            ),
            (
                r###"truncated"###,
                r###"private = {"nested":["SYNTH_REVIEW_PRIVATE""###,
                true,
            ),
            (
                r###"jwk-comment-apostrophe"###,
                r###"// don't confuse quotes
{
"kty": "RSA",
"d": "SYNTH_REVIEW_PRIVATE"
}
"###,
                true,
            ),
            (
                r###"jwk-comment-brace"###,
                r###"{"kty":"RSA", "d": { /* } */ nested: "SYNTH_REVIEW_PRIVATE" }}"###,
                true,
            ),
            (
                r###"jwk-comment-doublequote"###,
                r###"// unpaired "
{
"kty":"RSA",
"d":"SYNTH_REVIEW_PRIVATE"
}"###,
                true,
            ),
            (
                r###"jwk-escaped"###,
                r###"x="{\"kty\":\"RSA\",\"d\":\"SYNTH_REVIEW_PRIVATE\"}""###,
                true,
            ),
            (
                r###"private-negative-number"###,
                r###"scalar = -98217364198217364;"###,
                true,
            ),
            (r###"private-hex"###, r###"scalar = 0x1234abcd;"###, true),
            (
                r###"shared-jwk-k"###,
                r###"{"kty":"oct","k":"SYNTH_REVIEW_PRIVATE"}"###,
                true,
            ),
        ] {
            for output in [redact_text(source), redact_source_text(source)] {
                if private {
                    assert!(!output.contains("SYNTH_REVIEW"), "{id}");
                    assert!(!output.contains("98217364198217364"), "{id}");
                    if id == "private-hex" {
                        assert!(!output.contains("0x1234abcd"), "{id}");
                    }
                } else {
                    assert_eq!(output, source, "{id}");
                }
            }
        }
    }
    #[test]
    fn comment_documents_and_complete_literals_keep_their_boundaries() {
        for source in [
            "/* don't { kty: 'RSA', d: 'SYNTH_LEX_PRIVATE', n: 'PUBLIC_N' } */",
            "// { kty: 'RSA',\n// d: 'SYNTH_LEX_PRIVATE', n: 'PUBLIC_N' }",
            "# don't quote\n{ kty: 'RSA', d: 'SYNTH_LEX_PRIVATE', n: 'PUBLIC_N' }",
            "private = { /* } ] ) ' \" */ nested: {value: 'SYNTH_LEX_PRIVATE'} }; PUBLIC",
            "private = ((17, ('SYNTH_LEX_PRIVATE',)), [98, 21]); PUBLIC",
            "private = ('SYNTH_LEX_PRIVATE', 98217364198217364L)\r\nPUBLIC",
            "private = 'SYNTH_LEX_PRIVATE\\\"quoted'; PUBLIC",
            "scalar = 98217364198217364n",
            "scalar = 98217364198217364.;",
            "scalar = .98217364198217364;",
            "scalar = -98217364198217364.e+2F;",
            "private = { /* unterminated SYNTH_LEX_PRIVATE",
        ] {
            let output = redact_source_text(source);
            assert!(!output.contains("SYNTH_LEX_PRIVATE"));
            assert!(!output.contains("98217364198217364"));
            assert_eq!(output.matches('\n').count(), source.matches('\n').count());
            assert_eq!(
                output.matches("\r\n").count(),
                source.matches("\r\n").count()
            );
            if source.contains("PUBLIC_N") {
                assert!(output.contains("PUBLIC_N"));
            }
            if source.ends_with("PUBLIC") {
                assert!(output.ends_with("PUBLIC"));
            }
        }
        for source in [
            "private = (17 + Lookup('PUBLIC_ENV'));",
            "private = ('PUBLIC_VALUE' + Lookup('PUBLIC_ENV'));",
            "private = ((17, 'PUBLIC_VALUE') + Lookup('PUBLIC_ENV'));",
            "private = 'PUBLIC_VALUE' + Lookup('PUBLIC_ENV');",
            "scalar = 17 + Lookup('PUBLIC_ENV');",
            "private = ([17] + Lookup('PUBLIC_ENV'));",
            "private = (17 /* public */ + Lookup('PUBLIC_ENV'));",
            "{ kty: 'RSA', n: 'PUBLIC_N', e: 'AQAB', x: 'PUBLIC_X', y: 'PUBLIC_Y', crv: 'P-256', alg: 'RS256' }",
        ] { assert_eq!(redact_source_text(source), source); }
    }

    #[test]
    fn reviewed_comment_prose_nested_blocks_and_numeric_suffixes_are_private() {
        for (id, source, forbidden) in [
            (
                r####"numeric-suffix-i8"####,
                r####"scalar = 98i8;"####,
                &[r####"98"####][..],
            ),
            (
                r####"numeric-suffix-u8"####,
                r####"scalar = 98u8;"####,
                &[r####"98"####][..],
            ),
            (
                r####"numeric-suffix-i16"####,
                r####"scalar = 98i16;"####,
                &[r####"98"####][..],
            ),
            (
                r####"numeric-suffix-u16"####,
                r####"scalar = 98u16;"####,
                &[r####"98"####][..],
            ),
            (
                r####"numeric-suffix-i32"####,
                r####"scalar = 98i32;"####,
                &[r####"98"####][..],
            ),
            (
                r####"numeric-suffix-u32"####,
                r####"scalar = 98u32;"####,
                &[r####"98"####][..],
            ),
            (
                r####"numeric-suffix-i64"####,
                r####"scalar = 98217364198217364i64;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-u64"####,
                r####"scalar = 98217364198217364u64;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-i128"####,
                r####"scalar = 98217364198217364i128;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-u128"####,
                r####"scalar = 98217364198217364u128;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-isize"####,
                r####"scalar = 98217364198217364isize;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-usize"####,
                r####"scalar = 98217364198217364usize;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-f32"####,
                r####"scalar = 98217364198217364f32;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-f64"####,
                r####"scalar = 98217364198217364f64;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-M"####,
                r####"scalar = 98217364198217364M;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-suffix-m"####,
                r####"scalar = 98217364198217364m;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"comment-jwk-prose-brace"####,
                r####"/* Describe { then don't leak { kty: "RSA", d: "SYNTH_REVIEW_PRIVATE", n: "PUBLIC_N" } */"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"comment-scalar-apostrophe"####,
                r####"// don't leak scalar = "SYNTH_REVIEW_PRIVATE";"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"comment-scalar-doublequote"####,
                r####"// unpaired " scalar = 'SYNTH_REVIEW_PRIVATE';"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"comment-block-nested-rust"####,
                r####"let private = [ /* outer /* inner */ ] */ "SYNTH_REVIEW_PRIVATE" ];"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"comment-block-nested-jwk"####,
                r####"let key = Jwk { kty: "RSA", d: [ /* outer /* inner */ ] */ "SYNTH_REVIEW_PRIVATE" ], n: "PUBLIC_N" };"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"numeric-rust-separator-suffix"####,
                r####"scalar=98_217_364_198_217_364_u64;"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"comment-jwk-multiline-earlier-brace"####,
                r####"/* { examples aren't code.
{ "kty": "RSA",
"d": "SYNTH_REVIEW_PRIVATE", "n": "PUBLIC_N" }
*/"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
            (
                r####"closed-private-quoted-nested-comment"####,
                r####"let scalar = /* outer /* inner */ /* two */ end */ "SYNTH_REVIEW_PRIVATE";"####,
                &[
                    r####"SYNTH_REVIEW"####,
                    r####"98217364198217364"####,
                    r####"98_217_364_198_217_364"####,
                ][..],
            ),
        ] {
            for output in [redact_text(source), redact_source_text(source)] {
                for value in forbidden {
                    assert!(!output.contains(value), "{id}");
                }
                for public in ["PUBLIC_N", "AQAB", "PUBLIC_X", "PUBLIC_Y"] {
                    assert!(!source.contains(public) || output.contains(public), "{id}");
                }
            }
        }
    }

    #[test]
    fn prose_containers_and_numeric_tokens_preserve_public_expressions() {
        for (case, source) in [
            "/* prose { isn't code. { kty: 'RSA', d: [']', 'SYNTH_V5_PRIVATE'], n: 'PUBLIC_N' } */",
            "// unpaired \" scalar = bytes(b'SYNTH_V5_PRIVATE'); PUBLIC",
            "/* prose { don't: quote { kty: 'RSA', d: { nested: [']', 'SYNTH_V5_PRIVATE'] }, n: 'PUBLIC_N' } */",
            "// { kty: 'RSA', d: Buffer.from('SYNTH_V5_PRIVATE'), n: 'PUBLIC_N' }",
            r#"/* don't leak scalar = "SYNTH_V5_PRIVATE" */"#,
            "/* don't scalar = [']', 'SYNTH_V5_PRIVATE'] */",
            "/* outer /* don't leak scalar = 'SYNTH_V5_PRIVATE' */ */",
            "/* prose { don't: quote { kty: 'RSA', d: { /* } */ nested: ['SYNTH_V5_PRIVATE'] }, n: 'PUBLIC_N' } */",
            "/* { kty: 'RSA', n: [ /* scalar = 'SYNTH_V5_PRIVATE' */ 'PUBLIC_N' ] } */",
            "/* don't leak scalar = /* { PUBLIC } */ 'SYNTH_V5_PRIVATE' */",
            "scalar = 98217364198217364opaque_suffix; PUBLIC",
            "scalar = -98217364198217364_custom123; PUBLIC",
        ].into_iter().enumerate() {
            let output = redact_source_text(source);
            assert!(!output.contains("SYNTH_V5_PRIVATE"), "case {case}");
            assert!(!output.contains("98217364198217364"));
            assert!(!source.contains("PUBLIC_N") || output.contains("PUBLIC_N"));
            assert_eq!(redact_source_text(&output), output);
        }
        for source in [
            "scalar = 17.0.real;",
            "scalar = 17.real;",
            "scalar = 17..real;",
            "scalar = 17u64.real;",
            "/* {kty: 'RSA', /* d: 'PUBLIC_D' */ n: 'PUBLIC_N'} */",
            "/* { /* kty: 'RSA' */ d: 'PUBLIC_D'} */",
            "/* scalar = (17 /* outer /* ) */ nested */ + Lookup('PUBLIC_ENV')); */",
            "scalar = 17.toString();",
            "scalar = 17u64 + Lookup('PUBLIC_ENV');",
            "scalar = 17u64('PUBLIC_ENV');",
            "// prose { isn't code. scalar = 17.0.real;",
            "// don't mask private = (17u64 + Lookup('PUBLIC_ENV'));",
            "// unpaired \" { kty: 'RSA', n: 'PUBLIC_N', e: 'AQAB', x: 'PUBLIC_X', y: 'PUBLIC_Y' }",
            "/* prose { don't mask { metadata: {kty: 'RSA'}, d: 'PUBLIC_D' } */",
            "/* prose { don't mask [{kty: 'RSA'}, {d: 'PUBLIC_D'}] */",
        ] {
            assert_eq!(redact_source_text(source), source);
            assert_eq!(redact_text(source), source);
        }
    }

    #[test]
    fn unknown_comment_prose_labels_cannot_shield_recognized_private_fields() {
        let check = |id: &str, source: &str, private: bool, forbidden: &[&str]| {
            for output in [redact_source_text(source), redact_text(source)] {
                if private {
                    for value in forbidden {
                        assert!(!output.contains(value), "{id}");
                    }
                } else {
                    assert_eq!(output, source, "{id}");
                }
                for public in ["PUBLIC_N", "PUBLIC_D", "AQAB", "PUBLIC_X", "PUBLIC_Y"] {
                    assert!(!source.contains(public) || output.contains(public), "{id}");
                }
                assert_eq!(redact_source_text(&output), output, "{id}");
                assert_eq!(
                    output.matches('\n').count(),
                    source.matches('\n').count(),
                    "{id}"
                );
                assert_eq!(
                    output.matches("\r\n").count(),
                    source.matches("\r\n").count(),
                    "{id}"
                );
            }
        };
        for (id, source, private, forbidden) in [
            (
                "comment-prose-label-doublequote-scalar",
                r######"/* Note: " scalar = 'SYNTH_NOVEL_PRIVATE'; */"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "line-comment-prose-label-doublequote-scalar",
                r######"// Note: " scalar = 'SYNTH_NOVEL_PRIVATE';"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "line-comment-prose-label-singlequote-jwk",
                r######"// NOTE: 'do not publish {kty:"RSA",d:"SYNTH_NOVEL_PRIVATE",n:"PUBLIC_N"}"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "prose-label-text-quote-control",
                r######"// Note: text " scalar = 'SYNTH_NOVEL_PRIVATE';"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "line-comment-label-with-next-comment-line",
                r######"// Note: "
// scalar = 'SYNTH_NOVEL_PRIVATE';"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "block-comment-label-with-next-line",
                r######"/* Note: "
 scalar = 'SYNTH_NOVEL_PRIVATE';
*/"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "comment-apostrophe-control",
                r######"/* Note: don't leak {kty:'RSA',d:'SYNTH_NOVEL_PRIVATE',n:'PUBLIC_N'} */"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "comment-label-closed-quote-control",
                r######"/* Note: "example" scalar = 'SYNTH_NOVEL_PRIVATE'; */"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "jwk-object-numeric-suffix",
                r######"{kty:"RSA",d:0xdeadbeefL,n:"PUBLIC_N"}"######,
                true,
                &["0xdeadbeefL"][..],
            ),
            (
                "decimal-exponent-separator",
                r######"scalar = 12_34.56_78e-2M; PUBLIC"######,
                true,
                &["12_34.56_78e-2M"][..],
            ),
            (
                "negative-public-numeric-member",
                r######"scalar = 0xdeadbeef.real;"######,
                false,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "negative-public-quoted-value",
                r######"note = "scalar = a_public_reference";"######,
                false,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "negative-public-jwk",
                r######"{kty:"RSA",n:"PUBLIC_N",e:"AQAB",x:"PUBLIC_X",y:"PUBLIC_Y"}"######,
                false,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "unicode-crlf-neighbour",
                r######"// 日本語
scalar="SYNTH_NOVEL_PRIVATE"; PUBLIC
"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "wrapper-two-literals",
                r######"private = Buffer.from("SYNTH_NOVEL_PRIVATE", "hex"); PUBLIC"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "comment-quoted-comment-markers",
                r######"/* scalar = ["/*", "*/", "SYNTH_NOVEL_PRIVATE"]; */"######,
                true,
                &["SYNTH_NOVEL_PRIVATE"][..],
            ),
            (
                "description-closed-private-example",
                r######"/* description: "scalar = 'SYNTH_V6_PRIVATE';" */"######,
                true,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "description-closed-jwk-example",
                r######"/* description: '{kty:"RSA",d:"SYNTH_V6_PRIVATE",n:"PUBLIC_N"}' */"######,
                true,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "unicode-crlf-description",
                r######"/* 説明 Note: "
scalar = 'SYNTH_V6_PRIVATE';
*/
PUBLIC"######,
                true,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "nested-comment-prose",
                r######"/* Example: " prose /* NB: ' scalar = "SYNTH_V6_PRIVATE"; */ */"######,
                true,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "unknown-equals-prose",
                r######"// usage_hint = ' scalar = "SYNTH_V6_PRIVATE";"######,
                true,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "metadata-kty",
                r######"/* Note: " {metadata:{kty:'RSA'},d:'PUBLIC_D'} */"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "sibling-kty",
                r######"/* Example: ' [{kty:'RSA'},{d:'PUBLIC_D'}] */"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "public-jwk",
                r######"// NB: " {kty:'RSA',n:'PUBLIC_N',e:'AQAB',x:'PUBLIC_X',y:'PUBLIC_Y'}"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "public-description",
                r######"/* description: "ordinary quoted prose" */"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "code-quoted-private-example",
                r######"description = "scalar = 'SYNTH_V7_PRIVATE';";"######,
                true,
                &["SYNTH_V7_PRIVATE"][..],
            ),
            (
                "code-quoted-jwk-example",
                r######"description = '{kty:"RSA",d:"SYNTH_V7_PRIVATE"}';"######,
                true,
                &["SYNTH_V7_PRIVATE"][..],
            ),
            (
                "public-member",
                r######"// Example: ' scalar = 17u64.real;"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "public-computed",
                r######"/* Notice: " private = (17u64 + Lookup('PUBLIC_ENV')); */"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
            (
                "public-nested-comment-kty",
                r######"/* NB: " { /* kty:'RSA' */ d:'PUBLIC_D'} */"######,
                false,
                &["SYNTH_V6_PRIVATE"][..],
            ),
        ] {
            check(id, source, private, forbidden);
        }
        for label in [
            "Note",
            "Example",
            "NB",
            "Notice",
            "usage_hint",
            "description",
            "ordinaryLabel42",
        ] {
            for (quote, value_quote) in [('"', '\''), ('\'', '"')] {
                for (prefix, suffix) in [("/*", " */"), ("//", ""), ("#", "")] {
                    for body in [
                        format!("scalar = {value_quote}SYNTH_V6_PRIVATE{value_quote};"),
                        format!("do not publish {{kty:{value_quote}RSA{value_quote},d:{value_quote}SYNTH_V6_PRIVATE{value_quote},n:{value_quote}PUBLIC_N{value_quote}}}"),
                    ] {
                        let source = format!("{prefix} {label}: {quote} {body}{suffix}");
                        check(label, &source, true, &["SYNTH_V6_PRIVATE"]);
                    }
                }
            }
        }
    }

    #[test]
    fn source_text_redaction_retains_bare_cr_and_mixed_terminators() {
        for source in ["\r", "\r\r", "\r\n\r\r\n\n\r", "公開\rPUBLIC\r\n\rTAIL\n\r"] {
            assert_eq!(redact_source_text(source), source);
        }
        for source in [
            "公開 password = 'SYNTH_CR_ONE'\rPUBLIC\r\nsecret='SYNTH_CR_TWO'\r\rTAIL\n",
            "公開\r{\"kty\":\"RSA\",\"d\":\"SYNTH_CR_ONE\rSYNTH_CR_TWO\",\"n\":\"PUBLIC\"}\r\nTAIL\n",
            "公開\r-----BEGIN PRIVATE KEY-----\rSYNTH_CR_ONE\r\rSYNTH_CR_TWO\r-----END PRIVATE KEY-----\r\nPUBLIC\nTAIL\r",
        ] {
            let output = redact_source_text(source);
            assert!(!output.contains("SYNTH_CR"));
            assert!(output.contains("公開"));
            assert!(output.contains("PUBLIC"));
            assert!(output.contains("TAIL"));
            assert_eq!(
                output.chars().filter(|&c| c == '\r' || c == '\n').collect::<String>(),
                source.chars().filter(|&c| c == '\r' || c == '\n').collect::<String>(),
            );
            assert_eq!(redact_source_text(&output), output);
        }
    }

    #[test]
    fn source_text_redaction_keeps_cr_inside_owned_private_dumps() {
        for component in [
            "priv",
            "privateExponent",
            "prime1",
            "prime2",
            "exponent1",
            "exponent2",
            "coefficient",
        ] {
            for (open, close) in [("r##\"", "\"##"), ("/*", "*/"), ("`", "`")] {
                for separator in ["\r", "\r\r", "\r\n", "\r\n\r\n\n\r"] {
                    let header = format!("{component}:");
                    let body = [
                        "intro \"Private-Key: (2048 bit)",
                        "modulus:",
                        "  00:01",
                        "publicExponent: 65537",
                        "public prose",
                        "\"",
                        &header,
                        "  SYNTH_CR_PRIVATE_ONE",
                        "  SYNTH_CR_PRIVATE_TWO",
                        "",
                    ]
                    .join(separator);
                    let source = format!("外側_PREFIX; {open}{body}{close};\nPUBLIC_AFTER\r\n\r");
                    let output = redact_source_text(&source);
                    assert!(!output.contains("SYNTH_CR_PRIVATE"), "{component}");
                    assert!(output.starts_with("外側_PREFIX; "), "{component}");
                    assert!(output.contains("PUBLIC_AFTER"), "{component}");
                    assert_eq!(
                        output
                            .chars()
                            .filter(|&c| c == '\r' || c == '\n')
                            .collect::<String>(),
                        source
                            .chars()
                            .filter(|&c| c == '\r' || c == '\n')
                            .collect::<String>(),
                        "{component}",
                    );
                    assert_eq!(redact_source_text(&output), output, "{component}");
                }
            }
        }
    }

    #[test]
    fn source_text_redaction_does_not_promote_cr_fragments_to_whole_base64_values() {
        let public = format!("MII{}=", "A".repeat(80));
        for source in [
            format!("{public}\rPUBLIC_TAIL"),
            format!("{public}\rdata = r##\"Private-Key: (256 bit)\rmodulus:\r  00:01\rprime1:\r  SYNTH_CR_PRIVATE\r\"##; PUBLIC_TAIL"),
        ] {
            let output = redact_source_text(&source);
            assert!(output.contains(&public));
            assert!(output.contains("PUBLIC_TAIL"));
            assert!(!output.contains("SYNTH_CR_PRIVATE"));
            assert_eq!(output.matches('\r').count(), source.matches('\r').count());
            assert_eq!(redact_source_text(&output), output);
        }
    }

    #[test]
    fn source_text_redaction_preserves_unicode_and_line_terminators() {
        for source in ["", "公開\r\n\r\nRSA_new();\n", "PUBLIC", "\n", "\r\n"] {
            assert_eq!(redact_source_text(source), source);
        }
        let source = "公開 private = 'SYNTH098_LINE'\r\nPUBLIC\r\n";
        assert_eq!(
            redact_source_text(source),
            "公開 private = [masked-sensitive-value]\r\nPUBLIC\r\n"
        );
        let source =
            "-----BEGIN PRIVATE KEY-----\r\nSYNTH098_BODY\r\n-----END PRIVATE KEY-----\r\nPUBLIC";
        let output = redact_source_text(source);
        assert!(!output.contains("SYNTH098"));
        assert_eq!(output.matches("\r\n").count(), 3);
        assert!(output.ends_with("\r\nPUBLIC"));
    }

    #[test]
    fn structured_source_redaction_precedes_clipping_and_preserves_native_sensitivity() {
        let source = "{\n\"d\": \"SYNTH098_PRIVATE_D\",\n\"kty\": \"RSA\",\n\"n\": \"PUBLIC_MODULUS\"\n}\nRSA_new();";
        let mut privacy = SourcePrivacy::from_text(source);
        assert!(!privacy.sensitive(2, 8, false));
        assert!(privacy.sensitive_match(2, 8, false, "SYNTH098_PRIVATE_D"));
        assert!(privacy.protects_preview());
        privacy.capture_source(source);
        let lines = privacy.source_lines.as_ref().unwrap();
        assert!(!lines.join("\n").contains("SYNTH098"));
        assert!(lines[3].contains("PUBLIC_MODULUS"));
        let start = source.find("PRIVATE_D").unwrap();
        assert!(!privacy
            .source_excerpt(source, start, start + 9)
            .unwrap()
            .contains("PRIVATE_D"));
        assert_eq!(lines[5], "RSA_new();");
    }

    #[test]
    fn source_window_retains_public_unicode_text_beside_credentials_before_clipping() {
        let source = "公開 password=\"SYNTH087_VALUE\"; int visible=7;\nRSA_new();";
        let mut privacy = SourcePrivacy::from_text(source);
        privacy.capture_source(source);
        assert_eq!(
            privacy.source_lines.as_ref().unwrap()[0],
            "公開 password=[masked-sensitive-value]; int visible=7;"
        );
        assert_eq!(
            privacy.source_excerpt(source, 0, source.len()).unwrap(),
            "公開 password=[masked-sensitive-value]; int visible=7;\nRSA_new();"
        );
        assert!(privacy.source_excerpt(source, 1, source.len()).is_none());
    }

    #[test]
    fn private_context_does_not_hide_public_prefix_and_suffix_or_orphan_followup() {
        let text = "public -----BEGIN EC PRIVATE KEY-----\nSYNTH087_BLOCK\n-----END EC PRIVATE KEY----- suffix\nRSA_new();";
        let mut privacy = SourcePrivacy::from_text(text);
        privacy.capture_source(text);
        let lines = privacy.source_lines.unwrap();
        assert_eq!(lines[0], "public [masked-sensitive-value]");
        assert_eq!(lines[2], "[masked-sensitive-value] suffix");
        assert_eq!(lines[3], "RSA_new();");
        assert!(!lines.join("\n").contains("SYNTH087_BLOCK"));
    }
    #[test]
    fn syntax_sensitive_ranges_merge_unicode_and_mask_only_bounded_context() {
        let text = "é(a,\nβ) tail";
        let mut p = SourcePrivacy::from_text(text);
        p.add_sensitive_ranges(text, &[2..9, 3..8, 1..2, 9..999, 7..7]);
        assert!(p.masks_source_context());
        assert!(!p.sensitive(1, 1, false));
        assert!(p.sensitive(1, 2, false));
        assert!(p.sensitive(2, 1, false));
        assert!(!p.sensitive(2, 3, false));
        assert!(p.sensitive_match(1, 1, false, "é(a"));
        let mut empty = SourcePrivacy::default();
        empty.add_sensitive_ranges(text, &[1..2, 99..100]);
        assert!(empty.is_empty());
    }
    #[test]
    fn known_literals_keys_urls_and_encoded_value_strings_are_redacted() {
        for text in [
            "['password' => 'LEAK029A', 'mode' => 'public']",
            "password='LEAK029A'",
            r#"api_key="LEAK029A\"TAIL""#,
            "Authorization: Bearer LEAK029A",
            "https://u:LEAK029A@example.invalid/path?token=LEAK029B&mode=read",
            "AWS_SECRET_ACCESS_KEY=LEAK029A",
            "https://example.invalid/?token=LEAK029A.jwt%21&mode=read",
        ] {
            assert!(!redact_text(text).contains("LEAK029"));
        }
        assert!(is_sensitive_name("privateKey"));
        assert!(is_sensitive_name("prefix_privateKey"));
        assert!(is_sensitive_name("prefix_private_key"));
        assert!(!is_sensitive_name("private_key_algorithm"));
        let mut fragment = "PRIVATE KEY legacy fragment".to_string();
        evidence_field(&mut fragment);
        assert_eq!(fragment, PRIVATE);
        for name in NAMES {
            assert!(secret_name(name));
            assert!(secret_name(&format!("PREFIX_{name}")));
        }
    }
    #[test]
    fn benign_technical_facts_expressions_and_public_urls_stay_exact() {
        for text in [
            "password => publicValue;",
            "const factory = password => derive(password);",
            "RS256",
            "1.2.840.113549.1.1.11",
            "key_bits=2048",
            "private_key_algorithm=RSA",
            r#"const token = jwt.sign(payload, privateKey, { algorithm: "RS256" });"#,
            "const token = await signWithPqc(payload);",
            "https://example.invalid/docs?version=1.2.3",
            "PUBLIC_OPAQUE_IDENTIFIER_abcdefghijklmnopqrstuvwxyz0123456789_ABCDEFGHIJKLMN",
        ] {
            assert_eq!(redact_text(text), text);
        }
    }
    #[test]
    fn private_formats_and_unicode_credential_positions_have_bounded_context() {
        for label in LABELS {
            let text =
                format!("RS256\n-----BEGIN {label}-----\nopaque\n-----END {label}-----\nRS256\n");
            let p = SourcePrivacy::from_text(&text);
            assert!(!p.sensitive(1, 1, false));
            assert!(p.sensitive(3, 1, false));
            assert!(!p.sensitive(5, 1, false));
        }
        let line = "🔐 password = \"LEAK029A\"; RS256";
        let p = SourcePrivacy::from_text(line);
        let start = line[..line.find("LEAK029A").unwrap()].chars().count() + 1;
        assert!(p.sensitive(1, start, false));
        assert!(!p.sensitive(1, 1, false));
        let public = line[..line.find("RS256").unwrap()].chars().count() + 1;
        assert!(!p.sensitive(1, public, false));
        let unterminated =
            SourcePrivacy::from_text("-----BEGIN EC PRIVATE KEY-----\nopaque\nother\n");
        assert!(unterminated.sensitive(3, 1, false));
    }
    #[test]
    fn caller_metadata_retains_legacy_arbitrary_prefix_secret_protection() {
        let mut result: ScanResult = serde_json::from_value(serde_json::json!({
            "generated_at":"2026-11-10T00:00:00Z",
            "findings":[{"finding_id":"metadata", "rule_id":"CONTROL", "category":"Control", "risk":"quantum-uncertain", "severity":"low", "confidence":0.5, "description":"Control", "migration_hint":"Review", "location":{"file":"a.txt","line":1,"column":1}, "evidence":{"type":"regex_match", "match":"RSA_new()", "snippet_preview":"public", "metadata":{"tls.auth_password":"SYNTH087_METADATA", "顧客_password":"SYNTH087_METADATA", "prefix_privateKey":"SYNTH087_METADATA", "private_key_algorithm":"RSA"}}}],
            "cbom":[], "dependency_sbom":[], "summary":{"total_findings":1,"by_severity":{},"by_risk":{},"scanned_files":1,"skipped_files":0}
        })).unwrap();
        sanitize_scan_result(&mut result);
        let metadata = &result.findings[0].evidence.metadata;
        assert_eq!(metadata["private_key_algorithm"], "RSA");
        assert_eq!(result.findings[0].evidence.r#match, "RSA_new()");
        assert!(!serde_json::to_string(&result)
            .unwrap()
            .contains("SYNTH087_METADATA"));
    }
    #[test]
    fn cbom_collision_cannot_unmask_a_sensitive_occurrence_or_collapse_rows() {
        fn finding(id: &str, col: usize) -> Finding {
            serde_json::from_value(serde_json::json!({"finding_id":id,"rule_id":"CONTROL","category":"Control","risk":"quantum-uncertain","severity":"low","confidence":0.5,"description":"Control","migration_hint":"Review","location":{"file":"a.txt","line":2,"column":col},"evidence":{"type":"regex_match","match":"opaque","snippet_preview":"public","metadata":{}}})).unwrap()
        }
        for reversed in [false, true] {
            let mut findings = vec![finding("safe", 1), finding("sensitive", 20)];
            if reversed {
                findings.reverse();
            }
            let mut result:ScanResult=serde_json::from_value(serde_json::json!({"generated_at":"2026-11-10T00:00:00Z","findings":findings,"cbom":[{"component":"service","algorithm":"opaque","usage_type":"Control","location":"a.txt:2","quantum_risk":"quantum-uncertain","migration_hint":"Review"},{"component":"service","algorithm":"RS256","usage_type":"Control","location":"a.txt:3","quantum_risk":"quantum-uncertain","migration_hint":"Review"}],"dependency_sbom":[],"summary":{"total_findings":2,"by_severity":{},"by_risk":{},"scanned_files":1,"skipped_files":0}})).unwrap();
            sanitize_with_context(&mut result, &HashSet::from(["sensitive".into()]));
            assert_eq!(result.cbom.len(), 2);
            assert_eq!(result.cbom[0].algorithm, SENSITIVE);
            assert_eq!(result.cbom[1].algorithm, "RS256");
        }
    }
    #[test]
    fn trailing_marker_without_header_is_conservatively_private_context() {
        let p = SourcePrivacy::from_text("opaque\n-----END RSA PRIVATE KEY-----\npublic\n");
        assert!(p.sensitive(1, 1, false));
        assert!(!p.sensitive(3, 1, false));
    }
}

#[cfg(test)]
#[path = "../tests/support/privacy_quality.rs"]
mod individual_quality;
