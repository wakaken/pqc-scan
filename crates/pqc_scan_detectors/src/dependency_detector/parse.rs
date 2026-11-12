//! Provenance capture over supported structured subsets; lexical fallbacks never Complete.
#[path = "lexical.rs"]
mod lexical;
pub(super) use super::{
    decode::{self, Failure, Format},
    DependencyItem,
};
pub(super) use pqc_scan_core::model::line_col_for_offset;
pub(super) use pqc_scan_core::observation::{
    exact_dependency_version, resolved_dependency_version, safe_dependency_version,
    valid_dependency_name, DependencyOrigin as Origin,
};
pub(super) use pqc_scan_core::{classifier, DetectorStatus, ScannableFile};
pub(super) use regex::Regex;
use serde_json::Value;
pub(super) struct Parsed {
    pub items: Vec<DependencyItem>,
    pub status: DetectorStatus,
}
pub(super) struct Build {
    pub(super) items: Vec<DependencyItem>,
    bad: usize,
    pub(super) status: DetectorStatus,
}
impl Build {
    pub(super) fn new(status: DetectorStatus) -> Self {
        Self {
            items: vec![],
            bad: 0,
            status,
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(super) fn add(
        &mut self,
        ecosystem: &str,
        name: &str,
        reported: Option<&str>,
        declared: Option<&str>,
        resolved: Option<&str>,
        mut origin: Origin,
        line: usize,
        sbom: bool,
    ) {
        if self.items.len() >= 10000 {
            self.status = DetectorStatus::Truncated;
            return;
        }
        if !valid_dependency_name(name, ecosystem) {
            self.bad += 1;
            return;
        }
        if (declared.is_some_and(|s| !safe_dependency_version(s))
            || reported.is_some_and(|s| !safe_dependency_version(s)))
            && !matches!(
                origin,
                Origin::Path
                    | Origin::Git
                    | Origin::Archive
                    | Origin::WorkspaceInherited
                    | Origin::Unknown
            )
        {
            self.bad += 1;
            origin = Origin::Unknown;
        }
        let declared = declared
            .filter(|s| safe_dependency_version(s))
            .map(str::to_string);
        let resolved = resolved
            .filter(|s| resolved_dependency_version(ecosystem, s))
            .map(str::to_string);
        if matches!(origin, Origin::LockResolved) && resolved.is_none() {
            self.bad += 1;
            origin = Origin::LockUnverified;
        }
        let reported = reported
            .filter(|s| safe_dependency_version(s))
            .map(str::to_string);
        let version = reported
            .as_deref()
            .map(lexical::sanitize_version)
            .unwrap_or_else(|| "unknown".into());
        self.items.push(DependencyItem {
            name: name.into(),
            version,
            ecosystem: ecosystem.into(),
            purl: if matches!(
                origin,
                Origin::Path
                    | Origin::Git
                    | Origin::Archive
                    | Origin::WorkspaceInherited
                    | Origin::Unknown
            ) {
                String::new()
            } else {
                purl(ecosystem, name, resolved.as_deref())
            },
            line,
            source_type: if sbom { "sbom" } else { "manifest" }.into(),
            declared,
            resolved,
            reported,
            origin,
            ordinal: self.items.len(),
            parsed: true,
        });
    }
    pub(super) fn finish(self) -> anyhow::Result<Parsed> {
        let status = if self.status == DetectorStatus::Truncated {
            DetectorStatus::Truncated
        } else if self.bad > 0 {
            if self.items.is_empty() {
                anyhow::bail!("invalid dependency records");
            }
            DetectorStatus::Partial
        } else {
            self.status
        };
        Ok(Parsed {
            items: self.items,
            status,
        })
    }
}
pub(super) fn failure(f: Failure) -> anyhow::Result<Parsed> {
    match f {
        Failure::Malformed => anyhow::bail!("invalid dependency document"),
        Failure::Limit => Ok(Parsed {
            items: vec![],
            status: DetectorStatus::Truncated,
        }),
        Failure::Unsupported => Ok(Parsed {
            items: vec![],
            status: DetectorStatus::Unsupported,
        }),
    }
}
fn object(value: &Value) -> anyhow::Result<&serde_json::Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("invalid dependency document shape"))
}
fn line(text: &str, key: &str) -> usize {
    lexical::find_json_key_line(text, key).unwrap_or(0)
}
fn located(text: &str, name: &str) -> usize {
    lexical::find_json_name_value_line(text, name).unwrap_or(0)
}
fn locator(value: &str) -> Option<Origin> {
    let lower = value.to_ascii_lowercase();
    if lower.starts_with("workspace:") {
        Some(Origin::WorkspaceInherited)
    } else if lower.starts_with("file:")
        || lower.starts_with("link:")
        || lower.starts_with('.')
        || lower.starts_with('/')
        || lower.starts_with("path:")
    {
        Some(Origin::Path)
    } else if lower.starts_with("git://")
        || lower.starts_with("git+")
        || lower.starts_with("git@")
        || lower.starts_with("github:")
        || lower.starts_with("gitlab:")
        || lower.starts_with("bitbucket:")
    {
        Some(Origin::Git)
    } else if lower.contains("://") {
        Some(Origin::Archive)
    } else if lower.starts_with("npm:") {
        Some(Origin::Unknown)
    } else {
        None
    }
}
fn lock_locator(value: &str) -> Option<Origin> {
    let lower = value.to_ascii_lowercase();
    if lower.starts_with("https://registry.npmjs.org/") {
        None
    } else if lower.starts_with("https:") || lower.starts_with("http:") {
        Some(Origin::Archive)
    } else {
        locator(value)
    }
}
fn valid_fields(value: &Value, strings: &[&str], booleans: &[&str]) -> bool {
    value.is_object()
        && strings
            .iter()
            .all(|k| value.get(*k).is_none_or(Value::is_string))
        && booleans
            .iter()
            .all(|k| value.get(*k).is_none_or(Value::is_boolean))
}
fn yaml_line(text: &str, key: &str) -> usize {
    let mut lines = text.lines().enumerate().filter(|(_, l)| {
        let t = l.trim_start();
        let prefix = serde_json::to_string(key).unwrap_or_default();
        t.starts_with(&format!("{key}:"))
            || t.starts_with(&format!("'{key}':"))
            || t.starts_with(&format!("{prefix}:"))
    });
    let first = lines.next();
    if lines.next().is_some() {
        0
    } else {
        first.map(|(i, _)| i + 1).unwrap_or(0)
    }
}
fn toml_name_line(text: &str, name: &str) -> usize {
    let lines: Vec<_> = text
        .lines()
        .enumerate()
        .filter(|(_, l)| l.trim() == format!("name = \"{name}\""))
        .collect();
    if lines.len() == 1 {
        lines[0].0 + 1
    } else {
        0
    }
}
fn component(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"._-~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn purl(ecosystem: &str, name: &str, version: Option<&str>) -> String {
    if !["cargo", "npm", "pypi", "gem", "maven", "golang", "generic"].contains(&ecosystem) {
        return String::new();
    }
    let path = if ecosystem == "maven" {
        name.split(':').map(component).collect::<Vec<_>>().join("/")
    } else {
        name.split('/').map(component).collect::<Vec<_>>().join("/")
    };
    let mut p = format!("pkg:{ecosystem}/{path}");
    if let Some(v) = version.filter(|s| exact_dependency_version(s)) {
        p.push('@');
        p.push_str(&component(v));
    }
    p
}
fn make_purl(ecosystem: &str, name: &str, _version: &str) -> String {
    purl(ecosystem, name, None)
}
pub(super) fn inspect(file: &ScannableFile) -> anyhow::Result<Parsed> {
    let name = file.file_name().unwrap_or("");
    if !classifier::is_dependency_manifest(&file.path) && !classifier::is_sbom_file(&file.path) {
        return Ok(Parsed {
            items: vec![],
            status: DetectorStatus::NotApplicable,
        });
    }
    let Some(text) = file.text.as_deref() else {
        return Ok(Parsed {
            items: vec![],
            status: DetectorStatus::Unsupported,
        });
    };
    let format = match name {
        "package.json" | "package-lock.json" | "Pipfile.lock" => Some(Format::Json),
        "Cargo.toml" | "Cargo.lock" | "poetry.lock" => Some(Format::Toml),
        "pnpm-lock.yaml" => Some(Format::Yaml),
        _ if classifier::is_sbom_file(&file.path) => Some(Format::Json),
        _ => None,
    };
    if let Some(format) = format {
        let value = match decode::decode(text, format) {
            Ok(v) => v,
            Err(e) => return failure(e),
        };
        object(&value)?;
        let field = match name {
            "package-lock.json" => Some(("lockfileVersion", false)),
            "Cargo.lock" => Some(("version", false)),
            _ => None,
        };
        if field.is_some_and(|(k, _)| value.get(k).is_some_and(|v| v.as_u64().is_none())) {
            anyhow::bail!("invalid dependency schema version");
        }

        match name {
            "package.json" => npm_manifest(&value, text),
            "package-lock.json" => npm_lock(&value, text),
            "Cargo.toml" => cargo_manifest(&value, text),
            "Cargo.lock" => cargo_lock(&value, text),
            "Pipfile.lock" => pipfile(&value, text),
            "poetry.lock" => poetry(&value, text),
            "pnpm-lock.yaml" => pnpm(&value, text),
            _ => sbom(&value, text),
        }
    } else if let Some(result) = super::bounded::inspect(name, text) {
        result
    } else {
        heuristic(name, text)
    }
}
fn npm_manifest(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    for (section, origin) in [
        ("dependencies", Origin::Manifest),
        ("devDependencies", Origin::ManifestDev),
        ("peerDependencies", Origin::ManifestPeer),
        ("optionalDependencies", Origin::ManifestOptional),
    ] {
        let Some(v) = value.get(section) else {
            continue;
        };
        let Some(deps) = v.as_object() else {
            b.bad += 1;
            continue;
        };
        for (name, v) in deps {
            let Some(spec) = v.as_str() else {
                b.bad += 1;
                continue;
            };
            let source_line = line(text, name);
            let (name, spec) = if let Some(alias) = spec.strip_prefix("npm:") {
                match alias.rsplit_once('@') {
                    Some((target, constraint)) if valid_dependency_name(target, "npm") => {
                        (target, constraint)
                    }
                    _ => {
                        b.bad += 1;
                        continue;
                    }
                }
            } else {
                (name.as_str(), spec)
            };
            let origin = locator(spec).unwrap_or(origin);
            b.add(
                "npm",
                name,
                Some(spec),
                Some(spec),
                None,
                origin,
                source_line,
                false,
            );
        }
    }
    b.finish()
}
fn npm_lock(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let version = value["lockfileVersion"].as_u64();
    let known = matches!(version, Some(1..=3));
    let mut b = Build::new(if known {
        DetectorStatus::Complete
    } else if version.is_some() {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::NotReported
    });
    if version != Some(1) && value.get("packages").is_some() {
        if let Some(packages) = value["packages"].as_object() {
            for (path, v) in packages {
                if path.is_empty() {
                    continue;
                }
                let Some(v) = v.as_object() else {
                    b.bad += 1;
                    continue;
                };
                let name = v
                    .get("name")
                    .and_then(Value::as_str)
                    .or_else(|| path.rsplit_once("node_modules/").map(|(_, n)| n))
                    .unwrap_or("");
                if !valid_fields(
                    &Value::Object(v.clone()),
                    &["name", "version", "resolved"],
                    &["link"],
                ) {
                    b.bad += 1;
                    continue;
                }
                let reported = v.get("version").and_then(Value::as_str);
                let origin = if v.get("link").and_then(Value::as_bool) == Some(true) {
                    Origin::WorkspaceInherited
                } else {
                    reported
                        .and_then(locator)
                        .or_else(|| {
                            v.get("resolved")
                                .and_then(Value::as_str)
                                .and_then(lock_locator)
                        })
                        .unwrap_or(if known {
                            Origin::LockResolved
                        } else {
                            Origin::LockUnverified
                        })
                };
                let resolved = if matches!(origin, Origin::LockResolved) {
                    reported.filter(|s| exact_dependency_version(s))
                } else {
                    None
                };
                b.add(
                    "npm",
                    name,
                    reported,
                    None,
                    resolved,
                    origin,
                    line(text, path),
                    false,
                );
            }
        } else {
            b.bad += 1;
        }
    } else if let Some(v) = value.get("dependencies") {
        if known && version != Some(1) {
            b.bad += 1;
        }
        npm_v1(v, text, known && version == Some(1), &mut b);
    } else if known && version == Some(1) {
    } else if known {
        b.bad += 1;
    }
    b.finish()
}
fn npm_v1(value: &Value, text: &str, known: bool, b: &mut Build) {
    let Some(deps) = value.as_object() else {
        b.bad += 1;
        return;
    };
    for (name, v) in deps {
        if !valid_fields(v, &["version", "resolved"], &[]) {
            b.bad += 1;
            continue;
        }
        let reported = v["version"].as_str();
        let origin = reported
            .and_then(locator)
            .or_else(|| {
                v.get("resolved")
                    .and_then(Value::as_str)
                    .and_then(lock_locator)
            })
            .unwrap_or(if known {
                Origin::LockResolved
            } else {
                Origin::LockUnverified
            });
        let resolved = if matches!(origin, Origin::LockResolved) {
            reported.filter(|s| exact_dependency_version(s))
        } else {
            None
        };
        b.add(
            "npm",
            name,
            reported,
            None,
            resolved,
            origin,
            line(text, name),
            false,
        );
        if let Some(child) = v.get("dependencies") {
            npm_v1(child, text, known, b);
        }
    }
}
fn cargo_manifest(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    cargo_tables(value, text, false, &mut b);
    b.finish()
}
fn cargo_tables(value: &Value, text: &str, workspace: bool, b: &mut Build) {
    for (section, base) in [
        ("dependencies", Origin::Manifest),
        ("dev-dependencies", Origin::ManifestDev),
        ("build-dependencies", Origin::ManifestDev),
    ] {
        let Some(v) = value.get(section) else {
            continue;
        };
        let Some(deps) = v.as_object() else {
            b.bad += 1;
            continue;
        };
        for (alias, v) in deps {
            let (name, spec, origin) = if let Some(spec) = v.as_str() {
                (
                    alias.as_str(),
                    Some(spec),
                    if workspace {
                        Origin::WorkspaceDeclaration
                    } else {
                        base
                    },
                )
            } else if let Some(v) = v.as_object() {
                let table = Value::Object(v.clone());
                if !valid_fields(
                    &table,
                    &[
                        "package", "version", "path", "git", "branch", "tag", "rev", "registry",
                    ],
                    &["workspace", "optional", "default-features"],
                ) || table
                    .get("features")
                    .is_some_and(|v| !v.as_array().is_some_and(|a| a.iter().all(Value::is_string)))
                    || (!v.contains_key("version")
                        && !v.contains_key("path")
                        && !v.contains_key("git")
                        && v.get("workspace").and_then(Value::as_bool) != Some(true))
                {
                    b.bad += 1;
                    continue;
                }

                let name = v.get("package").and_then(Value::as_str).unwrap_or(alias);
                let spec = v.get("version").and_then(Value::as_str);
                let origin = if v.get("workspace").and_then(Value::as_bool) == Some(true) {
                    Origin::WorkspaceInherited
                } else if v.contains_key("git") {
                    Origin::Git
                } else if v.contains_key("path") {
                    Origin::Path
                } else if workspace {
                    Origin::WorkspaceDeclaration
                } else {
                    base
                };
                if v.get("version").is_some_and(|s| !s.is_string()) {
                    b.bad += 1;
                    continue;
                }
                (name, spec, origin)
            } else {
                b.bad += 1;
                continue;
            };
            let lines: Vec<_> = text
                .lines()
                .enumerate()
                .filter(|(_, l)| {
                    l.split_once('=')
                        .is_some_and(|(k, _)| k.trim().trim_matches(['"', '\'']) == alias)
                })
                .collect();
            let l = if lines.len() == 1 { lines[0].0 + 1 } else { 0 };
            b.add("cargo", name, spec, spec, None, origin, l, false);
        }
    }
    if let Some(w) = value.get("workspace") {
        if w.is_object() {
            cargo_tables(w, text, true, b);
        } else {
            b.bad += 1;
        }
    }
    if let Some(targets) = value.get("target") {
        if let Some(targets) = targets.as_object() {
            for target in targets.values() {
                if target.is_object() {
                    cargo_tables(target, text, false, b);
                } else {
                    b.bad += 1;
                }
            }
        } else {
            b.bad += 1;
        }
    }
}
fn cargo_lock(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let known = matches!(value["version"].as_u64(), Some(1..=4));
    let mut b = Build::new(if known {
        DetectorStatus::Complete
    } else if value.get("version").is_some() {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::NotReported
    });
    if let Some(v) = value.get("package") {
        if let Some(packages) = v.as_array() {
            for p in packages {
                if !valid_fields(p, &["name", "version", "source"], &[]) {
                    b.bad += 1;
                    continue;
                }
                let (Some(name), Some(version)) = (p["name"].as_str(), p["version"].as_str())
                else {
                    b.bad += 1;
                    continue;
                };
                let origin = match p["source"].as_str() {
                    Some(s) if s.starts_with("registry+") && known => Origin::LockResolved,
                    Some(s) if s.starts_with("git+") => Origin::Git,
                    None => Origin::Path,
                    _ => Origin::LockUnverified,
                };
                if known && matches!(origin, Origin::LockUnverified) {
                    b.bad += 1;
                }
                let resolved = if matches!(origin, Origin::LockResolved) {
                    Some(version)
                } else {
                    None
                };
                let l = toml_name_line(text, name);
                b.add(
                    "cargo",
                    name,
                    Some(version),
                    None,
                    resolved,
                    origin,
                    l,
                    false,
                );
            }
        } else {
            b.bad += 1;
        }
    }
    b.finish()
}
fn pipfile(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let spec = value["_meta"]["pipfile-spec"].as_u64();
    let known = spec == Some(6);
    let mut b = Build::new(if known {
        DetectorStatus::Complete
    } else if spec.is_some() {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::NotReported
    });
    for section in ["default", "develop"] {
        let Some(v) = value.get(section) else {
            continue;
        };
        let Some(deps) = v.as_object() else {
            b.bad += 1;
            continue;
        };
        for (name, v) in deps {
            if !valid_fields(
                v,
                &["version", "git", "path", "file", "ref", "index"],
                &["editable"],
            ) || v
                .get("hashes")
                .is_some_and(|v| !v.as_array().is_some_and(|a| a.iter().all(Value::is_string)))
            {
                b.bad += 1;
                continue;
            }
            let reported = v["version"].as_str();
            let origin = if v.get("git").is_some() {
                Origin::Git
            } else if v.get("path").is_some() || v.get("file").is_some() {
                Origin::Path
            } else if known {
                Origin::LockResolved
            } else {
                Origin::LockUnverified
            };
            let resolved = if matches!(origin, Origin::LockResolved) {
                reported
                    .and_then(|v| v.strip_prefix("=="))
                    .filter(|v| exact_dependency_version(v))
            } else {
                None
            };
            b.add(
                "pypi",
                name,
                reported,
                None,
                resolved,
                origin,
                line(text, name),
                false,
            );
        }
    }
    b.finish()
}
fn poetry(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let version = value["metadata"]["lock-version"].as_str();
    let known = matches!(version, Some("1.1" | "2.0" | "2.1"));
    let mut b = Build::new(if known {
        DetectorStatus::Complete
    } else if version.is_some() {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::NotReported
    });
    if let Some(v) = value.get("package") {
        if let Some(packages) = v.as_array() {
            for v in packages {
                if !valid_fields(v, &["name", "version"], &[])
                    || v.get("source").is_some_and(|source| {
                        !valid_fields(
                            source,
                            &["type", "url", "reference", "resolved_reference"],
                            &[],
                        )
                    })
                {
                    b.bad += 1;
                    continue;
                }
                let (Some(name), Some(version)) = (v["name"].as_str(), v["version"].as_str())
                else {
                    b.bad += 1;
                    continue;
                };
                let origin = match v["source"]["type"].as_str() {
                    Some("git") => Origin::Git,
                    Some("directory" | "file") => Origin::Path,
                    Some("url") => Origin::Archive,
                    Some("legacy" | "pypi") if known => Origin::LockResolved,
                    None if known && v.get("source").is_none() => Origin::LockResolved,
                    _ => Origin::LockUnverified,
                };
                if known && matches!(origin, Origin::LockUnverified) {
                    b.bad += 1;
                }
                let resolved = if matches!(origin, Origin::LockResolved) {
                    Some(version)
                } else {
                    None
                };
                let l = toml_name_line(text, name);
                b.add(
                    "pypi",
                    name,
                    Some(version),
                    None,
                    resolved,
                    origin,
                    l,
                    false,
                );
            }
        } else {
            b.bad += 1;
        }
    }
    b.finish()
}
fn pnpm(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let version = value["lockfileVersion"]
        .as_str()
        .map(str::to_string)
        .or_else(|| value["lockfileVersion"].as_f64().map(|n| n.to_string()));
    let known = version
        .as_deref()
        .is_some_and(|s| matches!(s, "5.3" | "5.4" | "6" | "6.0" | "9" | "9.0"));
    let mut b = Build::new(if known {
        DetectorStatus::Complete
    } else if version.is_some() {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::NotReported
    });
    if let Some(v) = value.get("packages") {
        if let Some(packages) = v.as_object() {
            for (raw_key, v) in packages {
                if !v.is_object()
                    || v.get("resolution").is_some_and(|r| {
                        !valid_fields(
                            r,
                            &[
                                "tarball",
                                "directory",
                                "repo",
                                "type",
                                "commit",
                                "integrity",
                            ],
                            &[],
                        )
                    })
                {
                    b.bad += 1;
                    continue;
                }
                let key = raw_key
                    .trim_start_matches('/')
                    .split('(')
                    .next()
                    .unwrap_or("");
                let pair = if version.as_deref().is_some_and(|s| s.starts_with("5.")) {
                    key.rsplit_once('/')
                } else {
                    key.rsplit_once('@')
                };
                let Some((name, version)) = pair else {
                    b.bad += 1;
                    continue;
                };
                let version = version.split('_').next().unwrap_or(version);
                let origin = if v["resolution"].get("directory").is_some() || key.contains("file:")
                {
                    Origin::Path
                } else if v["resolution"].get("repo").is_some() || v["resolution"]["type"] == "git"
                {
                    Origin::Git
                } else {
                    v["resolution"]["tarball"]
                        .as_str()
                        .and_then(lock_locator)
                        .unwrap_or(if known {
                            Origin::LockResolved
                        } else {
                            Origin::LockUnverified
                        })
                };
                let resolved = matches!(origin, Origin::LockResolved).then_some(version);
                b.add(
                    "npm",
                    name,
                    Some(version),
                    None,
                    resolved,
                    origin,
                    yaml_line(text, raw_key),
                    false,
                );
            }
        } else {
            b.bad += 1;
        }
    }
    pnpm_declarations(value, text, &mut b);
    if let Some(v) = value.get("importers") {
        if let Some(importers) = v.as_object() {
            for importer in importers.values() {
                if importer.is_object() {
                    pnpm_declarations(importer, text, &mut b);
                } else {
                    b.bad += 1;
                }
            }
        } else {
            b.bad += 1;
        }
    }
    b.finish()
}
fn pnpm_declarations(value: &Value, text: &str, b: &mut Build) {
    for (section, base) in [
        ("dependencies", Origin::Manifest),
        ("devDependencies", Origin::ManifestDev),
        ("optionalDependencies", Origin::ManifestOptional),
    ] {
        if let Some(v) = value.get(section) {
            if let Some(deps) = v.as_object() {
                for (name, v) in deps {
                    let spec = value["specifiers"][name]
                        .as_str()
                        .or_else(|| v["specifier"].as_str())
                        .or_else(|| v.as_str());
                    if let Some(spec) = spec {
                        b.add(
                            "npm",
                            name,
                            Some(spec),
                            Some(spec),
                            None,
                            locator(spec).unwrap_or(base),
                            yaml_line(text, name),
                            false,
                        );
                    } else {
                        b.bad += 1;
                    }
                }
            } else {
                b.bad += 1;
            }
        }
    }
}

fn sbom(value: &Value, text: &str) -> anyhow::Result<Parsed> {
    let cdx = value["bomFormat"] == "CycloneDX";
    let spdx = value["spdxVersion"]
        .as_str()
        .is_some_and(|s| s.starts_with("SPDX-"));
    let known =
        (cdx && matches!(
            value["specVersion"].as_str(),
            Some("1.4" | "1.5" | "1.6" | "1.7")
        )) || (spdx && matches!(value["spdxVersion"].as_str(), Some("SPDX-2.2" | "SPDX-2.3")));
    let mut b = Build::new(if known {
        DetectorStatus::Complete
    } else if value.get("specVersion").is_some() || value.get("spdxVersion").is_some() {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::NotReported
    });
    if let Some(v) = value.get("components") {
        sbom_components(v, text, &mut b);
    }
    if let Some(v) = value.get("packages") {
        if let Some(packages) = v.as_array() {
            for p in packages {
                if p.get("versionInfo").is_some_and(|v| !v.is_string())
                    || p.get("externalRefs").is_some_and(|v| !v.is_array())
                {
                    b.bad += 1;
                }
                let Some(name) = p["name"].as_str() else {
                    b.bad += 1;
                    continue;
                };
                let provided_refs: Vec<_> = p["externalRefs"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter(|r| r["referenceType"] == "purl")
                    .collect();
                let provided = if provided_refs.len() == 1 {
                    provided_refs[0]["referenceLocator"].as_str().unwrap_or("")
                } else {
                    ""
                };
                if provided_refs.len() > 1
                    || provided_refs
                        .first()
                        .is_some_and(|r| !r["referenceLocator"].is_string())
                {
                    b.bad += 1;
                }
                sbom_add(
                    &mut b,
                    name,
                    None,
                    p["versionInfo"].as_str(),
                    provided,
                    located(text, name),
                );
            }
        } else {
            b.bad += 1;
        }
    }
    if !cdx && !spdx && value.get("components").is_none() && value.get("packages").is_none() {
        b.status = DetectorStatus::Unsupported;
    }
    b.finish()
}
// Purls are identity claims, never locators to fetch or evidence of installation.
fn purl_identity(provided: &str) -> Option<(&str, String, Option<String>)> {
    if provided.len() > 4096 || provided.contains(['?', '#', '\\']) {
        return None;
    }
    let value = provided.strip_prefix("pkg:")?;
    let (ecosystem, path) = value.split_once('/')?;
    if !["cargo", "npm", "pypi", "gem", "maven", "golang", "generic"].contains(&ecosystem) {
        return None;
    }
    // A raw npm scope @ precedes a slash and is not a version separator.
    let separator = path
        .rfind('@')
        .filter(|at| path.rfind('/').is_none_or(|slash| *at > slash));
    let (path, version) = match separator {
        Some(at) => (&path[..at], Some(decode_component(&path[at + 1..])?)),
        None => (path, None),
    };
    if version.as_deref().is_some_and(|v| {
        !safe_dependency_version(v)
            || if ecosystem == "golang" {
                !v.strip_prefix('v')
                    .is_some_and(|v| resolved_dependency_version("npm", v))
            } else {
                !exact_dependency_version(v)
            }
    }) {
        return None;
    }
    let parts = path
        .split('/')
        .map(decode_component)
        .collect::<Option<Vec<_>>>()?;
    if parts
        .iter()
        .any(|s| s.is_empty() || s == "." || s == ".." || s.contains(['/', ':', '@']))
    {
        // npm scope is the sole allowed embedded @ component.
        if ecosystem != "npm"
            || parts.len() != 2
            || !parts[0].starts_with('@')
            || parts[0][1..].contains(['/', ':', '@'])
            || parts[1].contains(['/', ':', '@'])
        {
            return None;
        }
    }
    let name = if ecosystem == "maven" {
        if parts.len() != 2 {
            return None;
        }
        format!("{}:{}", parts[0], parts[1])
    } else {
        if matches!(ecosystem, "cargo" | "pypi" | "gem") && parts.len() != 1 {
            return None;
        }
        parts.join("/")
    };
    valid_dependency_name(&name, ecosystem).then_some((ecosystem, name, version))
}
fn decode_component(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let pair = bytes.get(i + 1..i + 3)?;
            let hex = std::str::from_utf8(pair).ok()?;
            decoded.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            decoded.push(bytes[i]);
            i += 1;
        }
    }
    let value = String::from_utf8(decoded).ok()?;
    if value
        .bytes()
        .any(|c| c.is_ascii_control() || c.is_ascii_whitespace())
    {
        None
    } else {
        Some(value)
    }
}
fn sbom_add(
    b: &mut Build,
    label: &str,
    group: Option<&str>,
    reported: Option<&str>,
    provided: &str,
    line: usize,
) {
    let identity = purl_identity(provided).filter(|(ecosystem, name, version)| {
        let matching_name = if *ecosystem == "maven" {
            // Without group, an SBOM name may be a product/display label.
            group.map_or(!label.contains(':') || label == name, |g| {
                format!("{g}:{label}") == *name
            })
        } else if *ecosystem == "npm" {
            let declared = group.map_or_else(|| label.to_string(), |g| format!("{g}/{label}"));
            declared == *name
        } else {
            label == name
        };
        matching_name && version.as_deref().zip(reported).is_none_or(|(a, b)| a == b)
    });
    if let Some((ecosystem, name, version)) = identity {
        b.add(
            ecosystem,
            &name,
            reported.or(version.as_deref()),
            None,
            None,
            Origin::SbomReported,
            line,
            true,
        );
    } else {
        if !provided.is_empty() {
            b.bad += 1;
        }
        // A malformed identity claim must not discard an independently safe
        // namespaced source label, or turn that label into a verified purl.
        let ecosystem = provided
            .strip_prefix("pkg:")
            .and_then(|p| p.split_once('/'))
            .map(|(e, _)| e)
            .filter(|e| ["cargo", "npm", "pypi", "gem", "maven", "golang", "generic"].contains(e))
            .filter(|e| valid_dependency_name(label, e))
            .unwrap_or("unknown");
        let ecosystem = if valid_dependency_name(label, "unknown") {
            "unknown"
        } else {
            ecosystem
        };
        let count = b.items.len();
        b.add(
            ecosystem,
            label,
            reported,
            None,
            None,
            Origin::SbomReported,
            line,
            true,
        );
        if b.items.len() > count {
            b.items.last_mut().unwrap().purl.clear();
        }
    }
}
fn sbom_components(value: &Value, text: &str, b: &mut Build) {
    let Some(components) = value.as_array() else {
        b.bad += 1;
        return;
    };
    for c in components {
        if ["version", "group", "purl"]
            .iter()
            .any(|key| c.get(*key).is_some_and(|v| !v.is_string()))
        {
            b.bad += 1;
        }
        let Some(name) = c["name"].as_str() else {
            b.bad += 1;
            continue;
        };
        sbom_add(
            b,
            name,
            c["group"].as_str(),
            c["version"].as_str(),
            c["purl"].as_str().unwrap_or(""),
            located(text, name),
        );
        if let Some(child) = c.get("components") {
            sbom_components(child, text, b);
        }
    }
}
pub(super) fn heuristic(name: &str, text: &str) -> anyhow::Result<Parsed> {
    if text.len() > 2 * 1024 * 1024 {
        return failure(Failure::Limit);
    }
    let mut lexical_truncated = false;
    let mut items = match name {
        "Gemfile" | "gems.rb" => lexical::parse_gemfile(text),
        "Gemfile.lock" | "gems.locked" => lexical::parse_gemfile_lock(text),
        v if v.ends_with(".gemspec") => lexical::parse_gemspec(text, &mut lexical_truncated),
        "requirements.txt" => lexical::parse_requirements(text),
        "go.mod" => lexical::parse_go_mod(text),
        "pom.xml" => lexical::parse_pom_xml(text),
        "build.gradle" | "build.gradle.kts" => lexical::parse_build_gradle(text),
        "gradle.lockfile" => lexical::parse_gradle_lockfile(text),
        _ => vec![],
    };
    let mut b = Build::new(if lexical_truncated {
        DetectorStatus::Truncated
    } else {
        DetectorStatus::NotReported
    });
    for item in items.drain(..) {
        let raw = text.lines().nth(item.line.saturating_sub(1)).unwrap_or("");
        let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
        let source = if compact.contains("git:")
            || compact.contains("git=")
            || raw.contains("://")
            || raw.contains("git@")
        {
            Some(Origin::Git)
        } else if compact.contains("path:")
            || compact.contains("path=")
            || raw.contains("file:")
            || raw.contains("=>")
        {
            Some(Origin::Path)
        } else {
            None
        };
        let lock = name.ends_with(".lock") || name == "gems.locked" || name == "gradle.lockfile";
        let mut origin = source.unwrap_or(if lock {
            Origin::HeuristicLock
        } else {
            Origin::HeuristicManifest
        });
        if name == "Gemfile.lock" || name == "gems.locked" {
            let header = text
                .lines()
                .take(item.line)
                .filter(|l| !l.is_empty() && !l.starts_with(' '))
                .last()
                .unwrap_or("");
            if header == "GIT" {
                origin = Origin::Git;
            }
            if header == "PATH" {
                origin = Origin::Path;
            }
        }
        let mut declared = if lock {
            None
        } else if name == "requirements.txt" {
            raw.find(&item.name).map(|n| {
                raw[n + item.name.len()..]
                    .split(';')
                    .next()
                    .unwrap_or("")
                    .split('#')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string()
            })
        } else {
            let strings: Vec<_> = raw
                .split(['\'', '"'])
                .enumerate()
                .filter_map(|(i, s)| (i % 2 == 1).then_some(s))
                .collect();
            if strings.len() > 1 {
                Some(
                    strings[1..]
                        .iter()
                        .copied()
                        .take_while(|s| safe_dependency_version(s))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            } else if item.version != "unknown" {
                Some(item.version.clone())
            } else {
                None
            }
        };
        if name == "pom.xml" {
            let source = text
                .lines()
                .skip(item.line.saturating_sub(1))
                .collect::<Vec<_>>()
                .join("\n");
            let block = source.split("</dependency>").next().unwrap_or("");
            declared = block
                .split_once("<version>")
                .and_then(|(_, v)| v.split_once("</version>"))
                .map(|(v, _)| v.trim().to_string());
        }
        let proper_lock = if name == "Gemfile.lock" || name == "gems.locked" {
            text.lines()
                .take(item.line)
                .filter(|l| !l.is_empty() && !l.starts_with(' '))
                .last()
                == Some("GEM")
        } else if name == "gradle.lockfile" {
            raw.split_once('=')
                .is_some_and(|(l, r)| l.split(':').count() == 3 && !r.trim().is_empty())
        } else {
            false
        };
        let resolved = if lock
            && proper_lock
            && !matches!(origin, Origin::Git | Origin::Path)
            && resolved_dependency_version(&item.ecosystem, &item.version)
        {
            Some(item.version.as_str())
        } else {
            None
        };
        b.add(
            &item.ecosystem,
            &item.name,
            (item.version != "unknown").then_some(item.version.as_str()),
            declared.as_deref().filter(|s| !s.is_empty()),
            resolved,
            origin,
            item.line,
            false,
        );
        if let Some(last) = b.items.last_mut() {
            last.parsed = false;
            if !lock {
                last.reported = last.declared.clone().or(last.reported.take());
            }
        }
    }
    b.finish()
}
