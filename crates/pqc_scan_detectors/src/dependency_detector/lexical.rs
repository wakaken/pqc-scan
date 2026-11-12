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

pub(super) fn parse_build_gradle(text: &str) -> Vec<DependencyItem> {
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
        ) || call.args.len() != 1
        {
            continue;
        }
        let parts: Vec<_> = call.args[0].split(':').collect();
        if parts.len() != 3 {
            continue;
        }
        let (group, artifact, version) = (parts[0], parts[1], parts[2]);
        let name = format!("{}:{}", group, artifact);
        let line = line_col_for_offset(text, call.start).0;

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

struct StaticDeclaration<'a> {
    name: &'a str,
    args: Vec<String>,
    start: usize,
}

// A lexical subset only: skip comments/strings and accept literal-only calls at
// statement boundaries. Unknown DSL expressions never become resolved versions.
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
        let mut args = Vec::new();
        loop {
            while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                cursor += 1;
            }
            let Some((value, end)) = quoted_literal(text, cursor) else {
                args.clear();
                break;
            };
            if value.contains(['$', '\\']) || value.contains("#{") {
                args.clear();
                break;
            }
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
            out.push(StaticDeclaration { name, args, start });
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
