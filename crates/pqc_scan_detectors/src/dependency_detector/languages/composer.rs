use super::*;

fn platform(name: &str) -> bool {
    name == "php"
        || name.starts_with("php-")
        || name.starts_with("ext-")
        || name.starts_with("lib-")
        || matches!(
            name,
            "composer" | "composer-plugin-api" | "composer-runtime-api"
        )
}
fn declarations(b: &mut Build, text: &SourceText<'_>, value: &Value, dev: bool, emulated: bool) {
    let Some(entries) = value.as_object() else {
        b.reject();
        return;
    };
    for (name, value) in entries {
        let name = name.to_ascii_lowercase();
        let is_platform = platform(&name);
        if emulated && !is_platform {
            b.reject();
            continue;
        }
        let disabled = emulated && value == &Value::Bool(false);
        let version = value.as_str();
        if version.is_none() && !disabled {
            b.reject();
            continue;
        }
        let context = if disabled {
            Some(Context::PlatformDisabled)
        } else if emulated {
            Some(Context::PlatformEmulated)
        } else if is_platform {
            Some(if dev {
                Context::PlatformDevelopment
            } else {
                Context::PlatformRequirement
            })
        } else {
            None
        };
        add(
            b,
            if is_platform {
                "php-platform"
            } else {
                "composer"
            },
            &name,
            version,
            if dev {
                Origin::ManifestDev
            } else {
                Origin::Manifest
            },
            line(text, &name),
            context,
        );
    }
}
pub(super) fn inspect(file: &ScannableFile, text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    let value = match json(text) {
        Ok(v) => v,
        Err(e) => return parse::failure(e),
    };
    let root = object(&value)?;
    let mut b = Build::new(DetectorStatus::Complete);
    if file.file_name() == Some("composer.json") {
        for (section, dev) in [("require", false), ("require-dev", true)] {
            if let Some(v) = root.get(section) {
                declarations(&mut b, text, v, dev, false);
            }
        }
        if let Some(repositories) = root.get("repositories") {
            let custom = match repositories {
                Value::Array(a) => !a.is_empty(),
                Value::Object(o) => !o.is_empty(),
                _ => {
                    b.reject();
                    true
                }
            };
            if custom {
                for item in &mut b.items {
                    if item.ecosystem == "composer" {
                        if matches!(item.origin, Origin::ManifestDev) {
                            item.language_context = Some(Context::Development);
                        }
                        item.origin = Origin::Unknown;
                        item.purl.clear();
                    }
                }
            }
        }
        if let Some(config) = root.get("config") {
            if let Some(config) = config.as_object() {
                if let Some(v) = config.get("platform") {
                    declarations(&mut b, text, v, false, true);
                }
            } else {
                b.reject();
            }
        }
    } else {
        // Composer has no stable numeric lock schema edition. Inspect named arrays only.
        if !root.contains_key("packages") {
            anyhow::bail!("missing Composer package array");
        }
        for (section, dev) in [("packages", false), ("packages-dev", true)] {
            let Some(packages) = root.get(section) else {
                continue;
            };
            let Some(packages) = packages.as_array() else {
                b.reject();
                continue;
            };
            for p in packages {
                let Some(p) = p.as_object() else {
                    b.reject();
                    continue;
                };
                let (Some(name), Some(version)) = (
                    p.get("name").and_then(Value::as_str),
                    p.get("version").and_then(Value::as_str),
                ) else {
                    b.reject();
                    continue;
                };
                let name = name.to_ascii_lowercase();
                // A Git source or dist URL cannot authenticate a Packagist resolution.
                // Do not infer registry provenance from a package's version or name.
                let mut origin = Origin::LockUnverified;
                let mut malformed = false;
                for key in ["source", "dist"] {
                    if let Some(source) = p.get(key) {
                        if let Some(source) = source.as_object() {
                            match source.get("type").and_then(Value::as_str) {
                                Some("path") => origin = Origin::Path,
                                Some("git" | "hg" | "svn" | "fossil")
                                    if !matches!(origin, Origin::Path) =>
                                {
                                    origin = Origin::Git
                                }
                                Some("zip" | "tar") if matches!(origin, Origin::LockUnverified) => {
                                    origin = Origin::Archive
                                }
                                Some(_) => {}
                                None => malformed = true,
                            }
                        } else {
                            malformed = true;
                        }
                    }
                }
                if malformed {
                    b.reject();
                    origin = Origin::Unknown;
                }
                let before = b.items.len();
                b.add(
                    "composer",
                    &name,
                    Some(version),
                    None,
                    None,
                    origin,
                    line(text, &name),
                    false,
                );
                // Even unverified plain lock records have no authenticated registry identity.
                for item in &mut b.items[before..] {
                    item.purl.clear();
                }
                if dev {
                    b.context(before, Context::Development);
                }
            }
        }
        for (section, dev, emulated) in [
            ("platform", false, false),
            ("platform-dev", true, false),
            ("platform-overrides", false, true),
        ] {
            if let Some(v) = root.get(section) {
                declarations(&mut b, text, v, dev, emulated);
            }
        }
    }
    b.finish()
}
