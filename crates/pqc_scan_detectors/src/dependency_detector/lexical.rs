use super::super::DependencyItem;
use super::*;
use std::sync::LazyLock;
pub(super) fn parse_gemfile_lock(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let spec_re = match Regex::new(r"(?m)^ {4}([A-Za-z0-9_.-]+)(?: \(([^)]+)\))?") {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in spec_re.captures_iter(text).take(10001) {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;
        out.push(DependencyItem {
            context: None,
            name: name.to_string(),
            version: sanitize_version(version),
            ecosystem: "gem".to_string(),
            purl: make_purl("gem", name, version),
            line,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
    }

    out
}

pub(super) fn parse_requirements(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    static REQUIREMENT_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)^\s*([a-z0-9_.-]+)\s*(?:==|>=|<=|~=|>|<)?\s*([a-z0-9*_.+-]+)?")
            .expect("constant requirement syntax")
    });
    let line_re = &*REQUIREMENT_RE;

    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
            continue;
        }
        let caps = match line_re.captures(line) {
            Some(v) => v,
            None => continue,
        };
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        out.push(DependencyItem {
            context: None,
            name: name.to_string(),
            version: sanitize_version(version),
            ecosystem: "pypi".to_string(),
            purl: make_purl("pypi", name, version),
            line: idx + 1,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
    }

    out
}

pub(super) fn parse_go_mod(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let mut in_require_block = false;

    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.starts_with("require (") {
            in_require_block = true;
            continue;
        }
        if in_require_block && line == ")" {
            in_require_block = false;
            continue;
        }

        let target = if line.starts_with("require ") {
            line.trim_start_matches("require ").trim()
        } else if in_require_block {
            line
        } else {
            continue;
        };

        let mut parts = target.split_whitespace();
        let name = match parts.next() {
            Some(v) => v,
            None => continue,
        };
        let version = parts.next().unwrap_or("unknown");

        out.push(DependencyItem {
            context: None,
            name: name.to_string(),
            version: sanitize_version(version),
            ecosystem: "golang".to_string(),
            purl: make_purl("golang", name, version),
            line: idx + 1,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
    }

    out
}

pub(super) fn parse_pom_xml(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let dep_re = match Regex::new(
        r"(?s)<dependency>.*?<groupId>\s*([^<\s]+)\s*</groupId>.*?<artifactId>\s*([^<\s]+)\s*</artifactId>.*?(?:<version>\s*([^<\s]+)\s*</version>)?.*?</dependency>",
    ) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in dep_re.captures_iter(text).take(10001) {
        let group = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let artifact = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(3).map(|m| m.as_str()).unwrap_or("unknown");
        let name = format!("{}:{}", group, artifact);
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;

        out.push(DependencyItem {
            context: None,
            name: name.clone(),
            version: sanitize_version(version),
            ecosystem: "maven".to_string(),
            purl: make_purl("maven", &name, version),
            line,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
    }

    out
}

// Split map fields outside quoted literals. Escapes keep a quoted comma from
// becoming syntax; literal contents are still validated by the shared policy.
fn gradle_fields(body: &str) -> impl Iterator<Item = &str> {
    let mut quote = None;
    let mut escaped = false;
    body.split(move |ch| {
        if escaped {
            escaped = false;
            return false;
        }
        if quote.is_some() && ch == '\\' {
            escaped = true;
            return false;
        }
        if quote == Some(ch) {
            quote = None;
            return false;
        }
        if quote.is_none() && matches!(ch, '\'' | '"') {
            quote = Some(ch);
            return false;
        }
        quote.is_none() && ch == ','
    })
}

fn gradle_map_body(text: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (at, ch) in text.char_indices() {
        if ch == '\n' {
            return &text[..at];
        }
        if escaped {
            escaped = false;
            continue;
        }
        if let Some(q) = quote {
            if ch == '\\' {
                escaped = true;
            } else if ch == q {
                quote = None;
            }
            continue;
        }
        if matches!(ch, '\'' | '"') {
            quote = Some(ch);
        } else if matches!(ch, ';' | '}')
            || text[at..].starts_with("//")
            || text[at..].starts_with("/*")
        {
            return &text[..at];
        }
    }
    text
}

pub(super) fn parse_build_gradle(text: &str) -> Vec<DependencyItem> {
    static FIELD: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^\s*(group|name|version)\s*:\s*(?:'([^']*)'|"([^"\n]*)"|(.+?))\s*$"#)
            .expect("Gradle map field")
    });
    let mut out = Vec::new();
    for call in static_declarations(text, false) {
        if !matches!(
            call.name,
            "implementation"
                | "api"
                | "compileOnly"
                | "runtimeOnly"
                | "testImplementation"
                | "testRuntimeOnly"
        ) {
            continue;
        }
        let (group, artifact, version) = if let Some(body) = call.map_body {
            let mut fields = std::collections::BTreeMap::new();
            let mut duplicate = false;
            for field in gradle_fields(body.trim_end_matches(';')) {
                let Some(c) = FIELD.captures(field) else {
                    duplicate = true;
                    break;
                };
                let value = c.get(2).or_else(|| c.get(3)).map(|m| m.as_str());
                if fields.insert(c[1].to_string(), value).is_some() {
                    duplicate = true;
                }
            }
            if duplicate {
                continue;
            }
            let (Some(Some(group)), Some(Some(name))) = (fields.get("group"), fields.get("name"))
            else {
                continue;
            };
            (
                group.to_string(),
                name.to_string(),
                fields
                    .get("version")
                    .and_then(|v| *v)
                    .unwrap_or("unknown")
                    .to_string(),
            )
        } else {
            if call.args.len() != 1 {
                continue;
            }
            let parts: Vec<_> = call.args[0].split(':').collect();
            if parts.len() != 3
                || parts[..2]
                    .iter()
                    .any(|v| v.contains(['$', '\\']) || v.contains("#{"))
            {
                continue;
            }
            (
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            )
        };
        let name = format!("{group}:{artifact}");
        out.push(DependencyItem {
            context: None,
            name: name.clone(),
            version: sanitize_version(&version),
            ecosystem: "maven".to_string(),
            purl: make_purl("maven", &name, &version),
            line: line_col_for_offset(text, call.start).0,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
        if out.len() == 10001 {
            break;
        }
    }
    out
}

struct StaticDeclaration<'a> {
    name: &'a str,
    args: Vec<String>,
    map_body: Option<&'a str>,
    start: usize,
}

// A lexical subset only: skip comments/strings and accept literal-only calls at
// statement boundaries. Gradle versions remain declared constraints; dynamic
// versions are filtered to Unknown by the shared manifest policy.
fn static_declarations(text: &str, hash_comments: bool) -> Vec<StaticDeclaration<'_>> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() && out.len() < 10001 {
        if bytes[i..].starts_with(b"//") || (hash_comments && bytes[i] == b'#') {
            i = text[i..].find('\n').map_or(bytes.len(), |n| i + n + 1);
            continue;
        }
        if bytes[i..].starts_with(b"/*") {
            i = text[i + 2..].find("*/").map_or(bytes.len(), |n| i + n + 4);
            continue;
        }
        if matches!(bytes[i], b'\'' | b'"') {
            i = quoted_literal(text, i).map_or(bytes.len(), |(_, end)| end);
            continue;
        }
        if !bytes[i].is_ascii_alphabetic() && bytes[i] != b'_' {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len()
            && (bytes[i].is_ascii_alphanumeric() || matches!(bytes[i], b'_' | b'.'))
        {
            i += 1;
        }
        let name = &text[start..i];
        let previous = text[..start]
            .chars()
            .rev()
            .find(|c| !matches!(c, ' ' | '\t' | '\r'));
        if previous.is_some_and(|c| !matches!(c, '\n' | '{' | ';')) {
            continue;
        }
        let mut cursor = i;
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        let parenthesized = bytes.get(cursor) == Some(&b'(');
        if parenthesized {
            cursor += 1;
        }
        // The audited Groovy map subset is single-line. Its head is found by
        // the same scanner, so comments and quoted lookalikes stay excluded.
        if !parenthesized
            && ["group", "name", "version"].iter().any(|key| {
                text[cursor..]
                    .strip_prefix(key)
                    .is_some_and(|rest| rest.trim_start_matches([' ', '\t']).starts_with(':'))
            })
        {
            out.push(StaticDeclaration {
                name,
                args: Vec::new(),
                map_body: Some(gradle_map_body(&text[cursor..])),
                start,
            });
            continue;
        }
        let mut args = Vec::new();
        loop {
            while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                cursor += 1;
            }
            let Some((value, end)) = quoted_literal(text, cursor) else {
                args.clear();
                break;
            };
            args.push(value.to_string());
            cursor = end;
            while bytes
                .get(cursor)
                .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r'))
            {
                cursor += 1;
            }
            if bytes.get(cursor) == Some(&b',') && args.len() < 16 {
                cursor += 1;
                continue;
            }
            if (parenthesized && bytes.get(cursor) != Some(&b')'))
                || (!parenthesized
                    && bytes
                        .get(cursor)
                        .is_some_and(|b| !matches!(b, b'\n' | b'}' | b';' | b'#'))
                    && !bytes[cursor..].starts_with(b"//"))
            {
                args.clear();
            }
            break;
        }
        if !args.is_empty() {
            out.push(StaticDeclaration {
                name,
                args,
                map_body: None,
                start,
            });
        }
    }
    out
}

fn quoted_literal(text: &str, start: usize) -> Option<(&str, usize)> {
    let bytes = text.as_bytes();
    let quote = *bytes.get(start)?;
    if !matches!(quote, b'\'' | b'"') {
        return None;
    }
    let mut i = start + 1;
    while i < bytes.len() {
        if bytes[i] == quote {
            return Some((&text[start + 1..i], i + 1));
        }
        if bytes[i] == b'\\' {
            i += 1;
        }
        if bytes.get(i) == Some(&b'\n') {
            return None;
        }
        i += 1;
    }
    None
}

pub(super) fn parse_gradle_lockfile(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();

    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || !line.contains(':') {
            continue;
        }
        let left = line.split('=').next().unwrap_or(line).trim();
        let parts: Vec<&str> = left.split(':').collect();
        if parts.len() < 3 {
            continue;
        }
        let name = format!("{}:{}", parts[0], parts[1]);
        let version = parts[2];

        out.push(DependencyItem {
            context: None,
            name: name.clone(),
            version: sanitize_version(version),
            ecosystem: "maven".to_string(),
            purl: make_purl("maven", &name, version),
            line: idx + 1,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
    }

    out
}

pub(super) fn sanitize_version(version: &str) -> String {
    let trimmed = version.trim().trim_matches('"').trim_matches('\'');
    let trimmed = trimmed.trim_start_matches('^').trim_start_matches('~');
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed.to_string()
    }
}

pub(super) fn find_json_key_line(text: &str, key: &str) -> Option<usize> {
    let escaped = regex::escape(key);
    let pattern = format!(r#""{}"\s*:"#, escaped);
    let re = Regex::new(&pattern).ok()?;
    let mut hits = re.find_iter(text);
    let hit = hits.next()?;
    if hits.next().is_some() {
        return None;
    }
    Some(line_col_for_offset(text, hit.start()).0)
}

pub(super) fn find_json_name_value_line(text: &str, name: &str) -> Option<usize> {
    let escaped = regex::escape(name);
    let pattern = format!(r#""name"\s*:\s*"{}""#, escaped);
    let re = Regex::new(&pattern).ok()?;
    let mut hits = re.find_iter(text);
    let hit = hits.next()?;
    if hits.next().is_some() {
        return None;
    }
    Some(line_col_for_offset(text, hit.start()).0)
}
