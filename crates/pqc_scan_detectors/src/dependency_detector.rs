use std::collections::{BTreeMap, HashSet};

use anyhow::Result;
use regex::Regex;
use serde_json::Value as JsonValue;

use pqc_scan_core::classifier;
use pqc_scan_core::model::{line_col_for_offset, mask_preview};
use pqc_scan_core::{Detection, Detector, Evidence, Location, ScannableFile};
use pqc_scan_rules::{RuleKind, RuleSet};

const INVENTORY_RULE_ID: &str = "__DEPENDENCY_INVENTORY__";

#[derive(Debug, Default)]
pub struct DependencyDetector;

#[derive(Debug, Clone)]
struct DependencyItem {
    name: String,
    version: String,
    ecosystem: String,
    purl: String,
    line: usize,
    source_type: String,
}

impl Detector for DependencyDetector {
    fn name(&self) -> &'static str {
        "dependency_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        let is_manifest = classifier::is_dependency_manifest(&file.path);
        let is_sbom = classifier::is_sbom_file(&file.path);
        if !is_manifest && !is_sbom {
            return Ok(Vec::new());
        }

        let text = match file.text.as_ref() {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        let file_name = file.file_name().unwrap_or("unknown");
        let mut items = if is_sbom {
            parse_sbom_dependencies(text)
        } else {
            parse_manifest_dependencies(file_name, text)
        };
        dedup_items(&mut items);

        if items.is_empty() {
            return Ok(Vec::new());
        }

        let mut out = Vec::new();
        let file_path = file.path.to_string_lossy().into_owned();

        for item in items {
            out.push(inventory_detection(&file_path, &item, self.name()));

            let target = format!(
                "{} {} {} {}",
                item.name, item.version, item.ecosystem, item.purl
            );

            for rule in rules.by_kind(RuleKind::Dependency) {
                let regex = match rule.compiled_pattern() {
                    Some(v) => v,
                    None => continue,
                };

                if !regex.is_match(&target) {
                    continue;
                }

                let mut metadata = BTreeMap::new();
                metadata.insert("detector".to_string(), self.name().to_string());
                metadata.insert("dep_name".to_string(), item.name.clone());
                metadata.insert("dep_version".to_string(), item.version.clone());
                metadata.insert("dep_ecosystem".to_string(), item.ecosystem.clone());
                metadata.insert("dep_purl".to_string(), item.purl.clone());
                metadata.insert("dep_source_type".to_string(), item.source_type.clone());
                metadata.insert("manifest".to_string(), file_name.to_string());

                out.push(Detection {
                    rule_id: rule.id.clone(),
                    location: Location {
                        file: file_path.clone(),
                        line: item.line,
                        column: if item.line == 0 { 0 } else { 1 },
                    },
                    evidence: Evidence {
                        r#type: if is_sbom {
                            "sbom_dependency".to_string()
                        } else {
                            "dependency".to_string()
                        },
                        r#match: mask_preview(&format!("{}@{}", item.name, item.version)),
                        snippet_preview: mask_preview(&format!(
                            "{} {} {}",
                            item.ecosystem, item.name, item.version
                        )),
                        metadata,
                    },
                });
            }
        }

        Ok(out)
    }
}

fn inventory_detection(file_path: &str, item: &DependencyItem, detector_name: &str) -> Detection {
    let mut metadata = BTreeMap::new();
    metadata.insert("detector".to_string(), detector_name.to_string());
    metadata.insert("inventory_type".to_string(), "dependency".to_string());
    metadata.insert("dep_name".to_string(), item.name.clone());
    metadata.insert("dep_version".to_string(), item.version.clone());
    metadata.insert("dep_ecosystem".to_string(), item.ecosystem.clone());
    metadata.insert("dep_purl".to_string(), item.purl.clone());
    metadata.insert("dep_source_type".to_string(), item.source_type.clone());

    Detection {
        rule_id: INVENTORY_RULE_ID.to_string(),
        location: Location {
            file: file_path.to_string(),
            line: item.line,
            column: if item.line == 0 { 0 } else { 1 },
        },
        evidence: Evidence {
            r#type: "dependency_inventory".to_string(),
            r#match: mask_preview(&format!("{}@{}", item.name, item.version)),
            snippet_preview: mask_preview(&format!("{} {}", item.ecosystem, item.name)),
            metadata,
        },
    }
}

fn dedup_items(items: &mut Vec<DependencyItem>) {
    let mut seen = HashSet::new();
    items.retain(|item| {
        let key = format!(
            "{}:{}:{}:{}",
            item.ecosystem, item.name, item.version, item.source_type
        );
        seen.insert(key)
    });
}

fn parse_manifest_dependencies(file_name: &str, text: &str) -> Vec<DependencyItem> {
    match file_name {
        "Cargo.toml" => parse_cargo_toml(text),
        "Cargo.lock" => parse_cargo_lock(text),
        "Gemfile" | "gems.rb" => parse_gemfile(text),
        "Gemfile.lock" | "gems.locked" => parse_gemfile_lock(text),
        name if name.ends_with(".gemspec") => parse_gemspec(text),
        "package.json" => parse_package_json(text),
        "package-lock.json" => parse_package_lock_json(text),
        "pnpm-lock.yaml" => parse_pnpm_lock(text),
        "requirements.txt" => parse_requirements(text),
        "go.mod" => parse_go_mod(text),
        "pom.xml" => parse_pom_xml(text),
        "build.gradle" | "build.gradle.kts" => parse_build_gradle(text),
        "Pipfile.lock" => parse_pipfile_lock(text),
        "poetry.lock" => parse_poetry_lock(text),
        "gradle.lockfile" => parse_gradle_lockfile(text),
        _ => Vec::new(),
    }
}

fn parse_cargo_toml(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let mut section = String::new();
    let version_re = Regex::new(r#"version\s*=\s*["']([^"']+)["']"#).ok();

    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(|c| c == '[' || c == ']').to_string();
            continue;
        }
        if !is_cargo_dependency_section(&section) {
            continue;
        }
        if line.starts_with("workspace = true") || line.starts_with("default-features =") {
            continue;
        }

        let (name, rhs) = match line.split_once('=') {
            Some((n, v)) => (n.trim(), v.trim()),
            None => continue,
        };
        if name.is_empty() {
            continue;
        }

        let version = if rhs.starts_with('"') || rhs.starts_with('\'') {
            rhs.trim_matches('"').trim_matches('\'').to_string()
        } else if rhs.starts_with('{') {
            version_re
                .as_ref()
                .and_then(|re| re.captures(rhs))
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        } else {
            "unknown".to_string()
        };

        out.push(DependencyItem {
            name: name.to_string(),
            version: sanitize_version(&version),
            ecosystem: "cargo".to_string(),
            purl: make_purl("cargo", name, &version),
            line: idx + 1,
            source_type: "manifest".to_string(),
        });
    }

    out
}

fn parse_cargo_lock(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let package_re =
        match Regex::new(r#"(?s)\[\[package\]\]\s*name\s*=\s*"([^"]+)"\s*version\s*=\s*"([^"]+)""#)
        {
            Ok(v) => v,
            Err(_) => return out,
        };

    for caps in package_re.captures_iter(text) {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;

        out.push(DependencyItem {
            name: name.to_string(),
            version: sanitize_version(version),
            ecosystem: "cargo".to_string(),
            purl: make_purl("cargo", name, version),
            line,
            source_type: "manifest".to_string(),
        });
    }

    out
}

fn is_cargo_dependency_section(section: &str) -> bool {
    section == "dependencies"
        || section == "dev-dependencies"
        || section == "build-dependencies"
        || section == "workspace.dependencies"
        || section.ends_with(".dependencies")
        || section.ends_with(".dev-dependencies")
        || section.ends_with(".build-dependencies")
}

fn parse_gemfile(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let gem_re = match Regex::new(r#"(?m)^\s*gem\s+['"]([^'"]+)['"]\s*(?:,\s*['"]([^'"]+)['"])?"#) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in gem_re.captures_iter(text) {
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
        });
    }

    out
}

fn parse_gemfile_lock(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let spec_re = match Regex::new(r"(?m)^ {4}([A-Za-z0-9_.-]+)(?: \(([^)]+)\))?") {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in spec_re.captures_iter(text) {
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
        });
    }

    out
}

fn parse_gemspec(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let re = match Regex::new(
        r#"(?m)^\s*spec\.(?:add_dependency|add_runtime_dependency|add_development_dependency)\s+['"]([^'"]+)['"](?:\s*,\s*['"]([^'"]+)['"])?"#,
    ) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in re.captures_iter(text) {
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
        });
    }

    out
}

fn parse_package_json(text: &str) -> Vec<DependencyItem> {
    let value = match serde_json::from_str::<JsonValue>(text) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();
    let sections = [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ];

    for section in &sections {
        let deps = match value.get(*section).and_then(|v| v.as_object()) {
            Some(v) => v,
            None => continue,
        };
        for (name, version_value) in deps {
            let version = version_value.as_str().unwrap_or("unknown").to_string();
            let line = find_json_key_line(text, name).unwrap_or(0);
            out.push(DependencyItem {
                name: name.clone(),
                version: sanitize_version(&version),
                ecosystem: "npm".to_string(),
                purl: make_purl("npm", name, &version),
                line,
                source_type: "manifest".to_string(),
            });
        }
    }

    out
}

fn parse_package_lock_json(text: &str) -> Vec<DependencyItem> {
    let value = match serde_json::from_str::<JsonValue>(text) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();

    if let Some(deps) = value.get("dependencies").and_then(|v| v.as_object()) {
        for (name, dep_value) in deps {
            let version = dep_value
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let line = find_json_key_line(text, name).unwrap_or(0);
            out.push(DependencyItem {
                name: name.clone(),
                version: sanitize_version(version),
                ecosystem: "npm".to_string(),
                purl: make_purl("npm", name, version),
                line,
                source_type: "manifest".to_string(),
            });
        }
    }

    if let Some(packages) = value.get("packages").and_then(|v| v.as_object()) {
        for (pkg_path, pkg_value) in packages {
            if pkg_path.is_empty() {
                continue;
            }
            let name = pkg_value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| pkg_path.trim_start_matches("node_modules/"));
            let version = pkg_value
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let line = find_json_name_value_line(text, name).unwrap_or(0);
            out.push(DependencyItem {
                name: name.to_string(),
                version: sanitize_version(version),
                ecosystem: "npm".to_string(),
                purl: make_purl("npm", name, version),
                line,
                source_type: "manifest".to_string(),
            });
        }
    }

    out
}

fn parse_pnpm_lock(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();

    let package_re = match Regex::new(r"(?m)^\s{2,}/(@?[^/\s:]+(?:/[^/\s:]+)?)/([^:\s]+):") {
        Ok(v) => v,
        Err(_) => return out,
    };
    for caps in package_re.captures_iter(text) {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;
        out.push(DependencyItem {
            name: name.to_string(),
            version: sanitize_version(version),
            ecosystem: "npm".to_string(),
            purl: make_purl("npm", name, version),
            line,
            source_type: "manifest".to_string(),
        });
    }

    let dep_re = match Regex::new(r"(?m)^\s{2}([@A-Za-z0-9._/-]+):\s+([^\s#]+)\s*$") {
        Ok(v) => v,
        Err(_) => return out,
    };
    for caps in dep_re.captures_iter(text) {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        if name == "dependencies" || name == "devDependencies" {
            continue;
        }
        let version = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        let line = line_col_for_offset(text, caps.get(0).map(|m| m.start()).unwrap_or(0)).0;
        out.push(DependencyItem {
            name: name.to_string(),
            version: sanitize_version(version),
            ecosystem: "npm".to_string(),
            purl: make_purl("npm", name, version),
            line,
            source_type: "manifest".to_string(),
        });
    }

    out
}

fn parse_requirements(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let line_re =
        match Regex::new(r"(?i)^\s*([a-z0-9_.-]+)\s*(?:==|>=|<=|~=|>|<)?\s*([a-z0-9*_.+-]+)?") {
            Ok(v) => v,
            Err(_) => return out,
        };

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
        });
    }

    out
}

fn parse_go_mod(text: &str) -> Vec<DependencyItem> {
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
        });
    }

    out
}

fn parse_pom_xml(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let dep_re = match Regex::new(
        r"(?s)<dependency>.*?<groupId>\s*([^<\s]+)\s*</groupId>.*?<artifactId>\s*([^<\s]+)\s*</artifactId>.*?(?:<version>\s*([^<\s]+)\s*</version>)?.*?</dependency>",
    ) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in dep_re.captures_iter(text) {
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
        });
    }

    out
}

fn parse_build_gradle(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let re = match Regex::new(
        r#"(?m)^\s*(implementation|api|compileOnly|runtimeOnly|testImplementation|testRuntimeOnly)\s+['"]([^:'"]+):([^:'"]+):([^'"]+)['"]"#,
    ) {
        Ok(v) => v,
        Err(_) => return out,
    };

    for caps in re.captures_iter(text) {
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
        });
    }

    out
}

fn parse_pipfile_lock(text: &str) -> Vec<DependencyItem> {
    let value = match serde_json::from_str::<JsonValue>(text) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();
    let sections = ["default", "develop"];
    for section in &sections {
        let deps = match value.get(*section).and_then(|v| v.as_object()) {
            Some(v) => v,
            None => continue,
        };

        for (name, dep_value) in deps {
            let version = dep_value
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let line = find_json_key_line(text, name).unwrap_or(0);
            out.push(DependencyItem {
                name: name.clone(),
                version: sanitize_version(version),
                ecosystem: "pypi".to_string(),
                purl: make_purl("pypi", name, version),
                line,
                source_type: "manifest".to_string(),
            });
        }
    }

    out
}

fn parse_poetry_lock(text: &str) -> Vec<DependencyItem> {
    let mut out = Vec::new();
    let mut name = String::new();
    let mut version = String::new();
    let mut current_line = 1usize;

    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line == "[[package]]" {
            if !name.is_empty() {
                out.push(DependencyItem {
                    name: name.clone(),
                    version: sanitize_version(&version),
                    ecosystem: "pypi".to_string(),
                    purl: make_purl("pypi", &name, &version),
                    line: current_line,
                    source_type: "manifest".to_string(),
                });
            }
            name.clear();
            version.clear();
            current_line = idx + 1;
            continue;
        }
        if line.starts_with("name = ") {
            name = line
                .trim_start_matches("name = ")
                .trim_matches('"')
                .to_string();
        } else if line.starts_with("version = ") {
            version = line
                .trim_start_matches("version = ")
                .trim_matches('"')
                .to_string();
        }
    }

    if !name.is_empty() {
        out.push(DependencyItem {
            name: name.clone(),
            version: sanitize_version(&version),
            ecosystem: "pypi".to_string(),
            purl: make_purl("pypi", &name, &version),
            line: current_line,
            source_type: "manifest".to_string(),
        });
    }

    out
}

fn parse_gradle_lockfile(text: &str) -> Vec<DependencyItem> {
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
        });
    }

    out
}

fn parse_sbom_dependencies(text: &str) -> Vec<DependencyItem> {
    let value = match serde_json::from_str::<JsonValue>(text) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();

    if let Some(components) = value.get("components").and_then(|v| v.as_array()) {
        for component in components {
            let name = component
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let version = component
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let purl = component.get("purl").and_then(|v| v.as_str()).unwrap_or("");
            let line = find_json_name_value_line(text, name).unwrap_or(0);

            out.push(DependencyItem {
                name: name.to_string(),
                version: sanitize_version(version),
                ecosystem: ecosystem_from_purl(purl).unwrap_or_else(|| "unknown".to_string()),
                purl: purl.to_string(),
                line,
                source_type: "sbom".to_string(),
            });
        }
    }

    if let Some(packages) = value.get("packages").and_then(|v| v.as_array()) {
        for package in packages {
            let name = package
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let version = package
                .get("versionInfo")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let line = find_json_name_value_line(text, name).unwrap_or(0);

            let mut purl = String::new();
            if let Some(ext_refs) = package.get("externalRefs").and_then(|v| v.as_array()) {
                for ext_ref in ext_refs {
                    let is_purl = ext_ref
                        .get("referenceType")
                        .and_then(|v| v.as_str())
                        .map(|v| v.eq_ignore_ascii_case("purl"))
                        .unwrap_or(false);
                    if is_purl {
                        purl = ext_ref
                            .get("referenceLocator")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        break;
                    }
                }
            }

            out.push(DependencyItem {
                name: name.to_string(),
                version: sanitize_version(version),
                ecosystem: ecosystem_from_purl(&purl).unwrap_or_else(|| "unknown".to_string()),
                purl,
                line,
                source_type: "sbom".to_string(),
            });
        }
    }

    out
}

fn make_purl(ecosystem: &str, name: &str, version: &str) -> String {
    let clean_version = sanitize_version(version);
    if clean_version == "unknown" {
        return format!("pkg:{}/{}", ecosystem, name);
    }
    format!("pkg:{}/{}@{}", ecosystem, name, clean_version)
}

fn sanitize_version(version: &str) -> String {
    let trimmed = version.trim().trim_matches('"').trim_matches('\'');
    let trimmed = trimmed.trim_start_matches('^').trim_start_matches('~');
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed.to_string()
    }
}

fn ecosystem_from_purl(purl: &str) -> Option<String> {
    if !purl.starts_with("pkg:") {
        return None;
    }
    let rest = &purl[4..];
    let ecosystem = rest.split('/').next().unwrap_or("").trim();
    if ecosystem.is_empty() {
        None
    } else {
        Some(ecosystem.to_string())
    }
}

fn find_json_key_line(text: &str, key: &str) -> Option<usize> {
    let escaped = regex::escape(key);
    let pattern = format!(r#""{}"\s*:"#, escaped);
    let re = Regex::new(&pattern).ok()?;
    let hit = re.find(text)?;
    Some(line_col_for_offset(text, hit.start()).0)
}

fn find_json_name_value_line(text: &str, name: &str) -> Option<usize> {
    let escaped = regex::escape(name);
    let pattern = format!(r#""name"\s*:\s*"{}""#, escaped);
    let re = Regex::new(&pattern).ok()?;
    let hit = re.find(text)?;
    Some(line_col_for_offset(text, hit.start()).0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::Detector;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        dir.push(format!("pqc-scan-{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn package_json_dependency_detection_reports_actual_line() {
        let root = temp_dir("dep-line");
        let rules_dir = root.join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(
            rules_dir.join("dependency.yml"),
            r#"
- id: DEP_OPENSSL
  kind: dependency
  category: Dependency
  severity: medium
  risk: quantum-vulnerable
  confidence: 0.8
  migration_hint: migrate
  pattern: "openssl"
  scope: code
"#,
        )
        .expect("write rule");
        let rules = RuleSet::load_from_dir(&rules_dir).expect("load rules");

        let manifest_path = root.join("package.json");
        let manifest = r#"{
  "name": "demo",
  "version": "1.0.0",
  "dependencies": {
    "openssl": "1.0.2"
  }
}"#;
        fs::write(&manifest_path, manifest).expect("write manifest");

        let file = ScannableFile::from_bytes(manifest_path.clone(), manifest.as_bytes().to_vec());
        let detector = DependencyDetector;
        let detections = detector.detect(&file, &rules).expect("run detector");

        let dep = detections
            .iter()
            .find(|d| d.rule_id == "DEP_OPENSSL")
            .expect("dependency finding exists");

        assert_eq!(dep.location.file, manifest_path.to_string_lossy());
        assert_eq!(dep.location.line, 5);
        assert_eq!(dep.location.column, 1);
    }
}
