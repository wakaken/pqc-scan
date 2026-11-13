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

pub(super) fn parse_build_gradle(text: &str) -> Vec<DependencyItem> {
    static DECL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
        r"^\s*(?:implementation|api|compileOnly|runtimeOnly|testImplementation|testRuntimeOnly)\s+(.+)"
    ).expect("Gradle declaration")
    });
    static COORD: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^["']([^:"']+):([^:"']+):([^"']+)["']"#).expect("Gradle coordinate")
    });
    static FIELD: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^\s*(group|name|version)\s*:\s*(?:'([^']*)'|"([^"\n]*)"|(.+?))\s*$"#)
            .expect("Gradle map field")
    });
    let mut out = Vec::new();
    // Single-line lexical subset. Scripts, closures, interpolation and variables
    // are never evaluated; the existing heuristic outcome stays NotReported.
    for (idx, raw) in text.lines().enumerate() {
        let Some(decl) = DECL.captures(raw) else {
            continue;
        };
        let body = &decl[1];
        let (group, artifact, version) = if let Some(c) = COORD.captures(body) {
            (c[1].to_string(), c[2].to_string(), c[3].to_string())
        } else {
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
        };
        let name = format!("{group}:{artifact}");
        out.push(DependencyItem {
            context: None,
            name: name.clone(),
            version: sanitize_version(&version),
            ecosystem: "maven".to_string(),
            purl: make_purl("maven", &name, &version),
            line: idx + 1,
            source_type: "manifest".to_string(),
            ..Default::default()
        });
        if out.len() == 10001 {
            break;
        }
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
