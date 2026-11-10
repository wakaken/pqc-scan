use super::super::DependencyItem;
use super::*;
use std::sync::LazyLock;
pub(super) fn parse_gemfile(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    static GEM_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?m)^\s*gem\s+['"]([^'"]+)['"]\s*(?:,\s*['"]([^'"]+)['"])?"#)
            .expect("constant gem syntax")
    });
    let gem_re = &*GEM_RE;

    for caps in gem_re.captures_iter(text).take(10001) {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;
        out.push(DependencyItem {
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

pub(super) fn parse_gemspec(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let re = match Regex::new(
        r#"(?m)^\s*spec\.(?:add_dependency|add_runtime_dependency|add_development_dependency)\s+['"]([^'"]+)['"](?:\s*,\s*['"]([^'"]+)['"])?"#,
    ) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in re.captures_iter(text).take(10001) {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;
        out.push(DependencyItem {
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
    let re = match Regex::new(
        r#"(?m)^\s*(implementation|api|compileOnly|runtimeOnly|testImplementation|testRuntimeOnly)\s+['"]([^:'"]+):([^:'"]+):([^'"]+)['"]"#,
    ) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in re.captures_iter(text).take(10001) {
        let group = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let artifact = caps.get(3).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(4).map(|m| m.as_str()).unwrap_or("unknown");
        let name = format!("{}:{}", group, artifact);
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;

        out.push(DependencyItem {
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
