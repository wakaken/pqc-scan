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
    is_sensitive_name(name) || NAMES.iter().any(|key| n.ends_with(&format!("_{key}")))
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
    let replaced = assignment().replace_all(&queries, |c: &regex::Captures<'_>| {
        let value = &c["value"];
        if !literal(value) {
            return c[0].to_string();
        }
        format!("{}{}{}", &c["prefix"], masked_value(value), &c["end"])
    });
    match replaced {
        Cow::Owned(text) => Cow::Owned(text),
        Cow::Borrowed(_) => match queries {
            Cow::Owned(text) => Cow::Owned(text),
            Cow::Borrowed(_) => urls,
        },
    }
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
    for (key, value) in &mut finding.evidence.metadata {
        if secret_name(key) {
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
        if let Some(ranges) = self.credentials.get(&number) {
            for &(_, _, a, b) in ranges {
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
        self.masks_source_context() || !self.credentials.is_empty()
    }
    pub(crate) fn sensitive_match(
        &self,
        line: usize,
        column: usize,
        mixed: bool,
        value: &str,
    ) -> bool {
        self.sensitive(line, column, mixed)
            || self.credentials.get(&line).is_some_and(|ranges| {
                value.contains("***")
                    || ranges.iter().any(|&(start, _, _, _)| {
                        column <= start && start < column.saturating_add(value.chars().count())
                    })
            })
    }
    pub(crate) fn is_empty(&self) -> bool {
        !self.masks_source_context() && self.credentials.is_empty() && self.source_lines.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
