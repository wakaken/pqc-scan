use super::*;
use std::collections::{BTreeMap, BTreeSet};

fn xml(text: &str) -> Result<roxmltree::Document<'_>, Failure> {
    if text.contains("<!DOCTYPE") || text.contains("<!ENTITY") {
        return Err(Failure::Malformed);
    }
    let doc = roxmltree::Document::parse_with_options(
        text,
        roxmltree::ParsingOptions {
            nodes_limit: 50000,
            ..Default::default()
        },
    )
    .map_err(|e| {
        if matches!(e, roxmltree::Error::NodesLimitReached) {
            Failure::Limit
        } else {
            Failure::Malformed
        }
    })?;
    for n in doc.descendants().filter(|n| n.is_element()) {
        if n.ancestors().take(66).count() > 65 {
            return Err(Failure::Limit);
        }
        if n.tag_name()
            .namespace()
            .is_some_and(|s| s != "http://schemas.microsoft.com/developer/msbuild/2003")
        {
            return Err(Failure::Unsupported);
        }
    }
    Ok(doc)
}
fn conditional(n: roxmltree::Node<'_, '_>) -> bool {
    n.ancestors().any(|a| a.attribute("Condition").is_some())
        || n.children().any(|c| {
            (c.has_tag_name("Version") || c.has_tag_name("VersionOverride"))
                && c.attribute("Condition").is_some()
        })
}
fn literal_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 100
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
        && !name.contains("..")
}
fn version<'a>(n: roxmltree::Node<'a, '_>) -> Result<Option<&'a str>, ()> {
    let attr = n
        .attribute("Version")
        .or_else(|| n.attribute("VersionOverride"));
    let mut children = n
        .children()
        .filter(|c| c.has_tag_name("Version") || c.has_tag_name("VersionOverride"));
    let child = children.next();
    if children.next().is_some()
        || (attr.is_some() && child.is_some())
        || (n.attribute("Version").is_some() && n.attribute("VersionOverride").is_some())
    {
        return Err(());
    }
    Ok(attr.or_else(|| child.and_then(|c| c.text())).map(str::trim))
}
fn trusted_central(text: &str) -> Option<BTreeMap<String, Option<String>>> {
    let doc = xml(text).ok()?;
    let root = doc.root_element();
    if !root.has_tag_name("Project")
        || root
            .descendants()
            .any(|n| n.has_tag_name("Import") || n.has_tag_name("Choose"))
    {
        return None;
    }
    let mut out = BTreeMap::new();
    for n in root
        .children()
        .filter(|n| n.has_tag_name("ItemGroup"))
        .flat_map(|n| n.children().filter(|n| n.has_tag_name("PackageVersion")))
    {
        let id = n
            .attribute("Include")
            .or_else(|| n.attribute("Update"))?
            .to_ascii_lowercase();
        if !literal_name(&id) {
            return None;
        }
        let value = version(n)
            .ok()?
            .filter(|v| pqc_scan_core::observation::safe_dependency_constraint("nuget", v));
        let value = if conditional(n)
            || n.attribute("Include").is_none()
            || n.attribute("VersionOverride").is_some()
            || n.children().any(|n| n.has_tag_name("VersionOverride"))
        {
            None
        } else {
            value.map(str::to_string)
        };
        out.entry(id).and_modify(|v| *v = None).or_insert(value);
    }
    Some(out)
}
fn central(
    file: &ScannableFile,
    repo: Option<&RepositoryContext>,
) -> BTreeMap<String, Option<String>> {
    let Some(repo) = repo else {
        return BTreeMap::new();
    };
    for depth in 0..32 {
        let relative = format!("{}Directory.Packages.props", "../".repeat(depth));
        if let Some(f) = repo.read_relative(&file.path, std::path::Path::new(&relative)) {
            let Some(text) = f.text.as_deref() else {
                return BTreeMap::new();
            };
            // Props use the same record/status limits as their own inventory.
            // repo=None cannot recurse into a project/context lookup.
            let checked = inspect(&f, &SourceText::new(text), None);
            if !checked.is_ok_and(|p| p.status == DetectorStatus::Complete) {
                return BTreeMap::new();
            }
            return trusted_central(text).unwrap_or_default();
        }
    }
    BTreeMap::new()
}
fn frameworks(b: &mut Build, value: &str, line: usize, conditional: bool) {
    if conditional {
        b.status = DetectorStatus::Partial;
    }
    for tfm in value.split(';').take(129) {
        if !literal_name(tfm) {
            b.status = DetectorStatus::Partial;
            continue;
        }
        add(
            b,
            "dotnet-framework",
            "target-framework",
            Some(tfm),
            Origin::Manifest,
            line,
            Some(if conditional {
                Context::Conditional
            } else {
                Context::FrameworkDeclared
            }),
        );
    }
    if value.split(';').count() > 128 {
        b.status = DetectorStatus::Truncated;
    }
}
pub(super) fn inspect(
    file: &ScannableFile,
    text: &SourceText<'_>,
    repo: Option<&RepositoryContext>,
) -> anyhow::Result<Parsed> {
    if matches!(file.file_name(), Some("packages.lock.json" | "global.json")) {
        return json_file(file, text);
    }
    let doc = match xml(text) {
        Ok(d) => d,
        Err(e) => return parse::failure(e),
    };
    let root = doc.root_element();
    let mut b = Build::new(DetectorStatus::Complete);
    if file.file_name() == Some("packages.config") {
        if !root.has_tag_name("packages") {
            anyhow::bail!("invalid legacy package root");
        }
        for n in root.children().filter(|n| n.is_element()) {
            if !n.has_tag_name("package") {
                b.reject();
                continue;
            }
            let (Some(id), Some(v)) = (n.attribute("id"), n.attribute("version")) else {
                b.reject();
                continue;
            };
            if !literal_name(id) {
                b.reject();
                continue;
            }
            add(
                &mut b,
                "nuget",
                &id.to_ascii_lowercase(),
                Some(v),
                Origin::Manifest,
                text.line_at(n.range().start),
                Some(Context::LegacyDeclaration),
            );
            if let Some(tfm) = n.attribute("targetFramework") {
                frameworks(&mut b, tfm, text.line_at(n.range().start), false);
            }
        }
        return b.finish();
    }
    if !root.has_tag_name("Project") {
        anyhow::bail!("invalid MSBuild project root");
    }
    let inherited = if file.ext() == Some("csproj") {
        central(file, repo)
    } else {
        BTreeMap::new()
    };
    if root
        .descendants()
        .any(|n| n.has_tag_name("Import") || n.has_tag_name("Choose"))
    {
        b.status = DetectorStatus::Partial;
    }
    for group in root.children().filter(|n| n.is_element()) {
        if group.has_tag_name("PropertyGroup") {
            for n in group
                .children()
                .filter(|n| n.has_tag_name("TargetFramework") || n.has_tag_name("TargetFrameworks"))
            {
                frameworks(
                    &mut b,
                    n.text().unwrap_or(""),
                    text.line_at(n.range().start),
                    conditional(n),
                );
            }
        }
        if !group.has_tag_name("ItemGroup") {
            continue;
        }
        for n in group
            .children()
            .filter(|n| n.has_tag_name("PackageReference") || n.has_tag_name("PackageVersion"))
        {
            let Some(id) = n.attribute("Include").or_else(|| n.attribute("Update")) else {
                b.reject();
                continue;
            };
            if !literal_name(id) {
                b.status = DetectorStatus::Partial;
                continue;
            }
            let id = id.to_ascii_lowercase();
            let mut origin = Origin::Manifest;
            let mut context = None;
            let mut malformed_version = false;
            let mut v = match version(n) {
                Ok(v) => v,
                Err(_) => {
                    b.reject();
                    malformed_version = true;
                    None
                }
            };
            let is_conditional = malformed_version
                || conditional(n)
                || v.is_some_and(|v| {
                    !pqc_scan_core::observation::safe_dependency_constraint("nuget", v)
                });
            if is_conditional {
                b.status = DetectorStatus::Partial;
                v = None;
                context = Some(Context::Conditional);
            } else if v.is_none() && n.has_tag_name("PackageReference") {
                origin = Origin::WorkspaceInherited;
                v = inherited.get(&id).and_then(|v| v.as_deref());
                context = Some(Context::CentralInherited);
                if v.is_none() {
                    b.status = DetectorStatus::Partial;
                }
            }
            add(
                &mut b,
                "nuget",
                &id,
                v,
                origin,
                text.line_at(n.range().start),
                context,
            );
        }
    }
    b.finish()
}
fn json_file(file: &ScannableFile, text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    let value = match json(text) {
        Ok(v) => v,
        Err(e) => return parse::failure(e),
    };
    let root = object(&value)?;
    let mut b = Build::new(DetectorStatus::Complete);
    if file.file_name() == Some("global.json") {
        if let Some(sdk) = root.get("sdk") {
            let sdk = object(sdk)?;
            if let Some(v) = field(sdk, "version")? {
                add(
                    &mut b,
                    "dotnet-sdk",
                    "dotnet-sdk",
                    Some(v),
                    Origin::Manifest,
                    line(text, "version"),
                    Some(Context::SdkDeclared),
                );
            }
        }
        return b.finish();
    }
    match root.get("version").and_then(Value::as_u64) {
        Some(1 | 2) => {}
        Some(_) => return parse::failure(Failure::Unsupported),
        None => anyhow::bail!("missing NuGet lock version"),
    }
    let buckets = root
        .get("dependencies")
        .ok_or_else(|| anyhow::anyhow!("missing NuGet dependencies"))?;
    for (tfm, packages) in object(buckets)? {
        let parts: Vec<_> = tfm.split('/').collect();
        if parts.len() > 2 || !parts.iter().all(|s| literal_name(s)) {
            b.reject();
            continue;
        }
        let Some(packages) = packages.as_object() else {
            b.reject();
            continue;
        };
        for (i, part) in parts.iter().enumerate() {
            add(
                &mut b,
                "dotnet-framework",
                if i == 0 {
                    "target-framework"
                } else {
                    "runtime-identifier"
                },
                Some(part),
                Origin::LockUnverified,
                line(text, tfm),
                Some(Context::FrameworkLockReported),
            );
        }
        let mut seen = BTreeSet::new();
        for (id, record) in packages {
            if !literal_name(id) {
                b.reject();
                continue;
            }
            let at = line(text, id);
            let id = id.to_ascii_lowercase();
            if !seen.insert(id.clone()) {
                b.status = DetectorStatus::Partial;
            }
            let Some(record) = record.as_object() else {
                b.reject();
                continue;
            };
            let kind = record.get("type").and_then(Value::as_str);
            if !matches!(kind, Some("Direct" | "Transitive" | "Project")) {
                b.reject();
                continue;
            }
            let requested = record.get("requested").and_then(Value::as_str);
            let resolved = record.get("resolved").and_then(Value::as_str);
            if record.get("requested").is_some_and(|v| !v.is_string()) {
                b.reject();
            }
            let origin = if kind == Some("Project") {
                Origin::WorkspaceInherited
            } else {
                Origin::LockResolved
            };
            b.add(
                "nuget",
                &id,
                resolved,
                requested,
                if kind == Some("Project") {
                    None
                } else {
                    resolved
                },
                origin,
                at,
                false,
            );
        }
    }
    b.finish()
}
