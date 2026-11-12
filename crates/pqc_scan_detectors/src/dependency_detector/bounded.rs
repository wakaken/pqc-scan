//! Fully consumed, deliberately small declaration grammars. No evaluation or resolution.
use super::parse::*;

fn finish(mut b: Build, incomplete: bool) -> anyhow::Result<Parsed> {
    if incomplete && b.status != DetectorStatus::Truncated {
        b.status = if b.items.is_empty() {
            DetectorStatus::NotReported
        } else {
            DetectorStatus::Partial
        };
    }
    b.finish()
}
fn comment(line: &str, marker: char, whitespace_before: bool) -> &str {
    let mut quote = None;
    for (i, c) in line.char_indices() {
        if let Some(q) = quote {
            if c == q {
                quote = None;
            }
        } else if c == '\'' || c == '"' {
            quote = Some(c);
        } else if c == marker
            && (!whitespace_before
                || i == 0
                || line[..i].chars().last().is_some_and(char::is_whitespace))
        {
            return &line[..i];
        }
    }
    line
}
fn logical_lines(text: &str, requirement_comments: bool) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut pending = String::new();
    let mut start = 1;
    for (i, raw) in text.lines().enumerate() {
        if pending.is_empty() {
            start = i + 1;
        }
        let line = comment(raw, '#', requirement_comments).trim();
        if let Some(part) = line.strip_suffix('\\') {
            pending.push_str(part);
            pending.push(' ');
        } else {
            pending.push_str(line);
            out.push((start, std::mem::take(&mut pending)));
        }
    }
    if !pending.is_empty() {
        pending.push('\\');
        out.push((start, pending));
    }
    out
}
pub(super) fn inspect(name: &str, text: &str) -> Option<anyhow::Result<Parsed>> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    if !matches!(
        name,
        "go.mod"
            | "requirements.txt"
            | "Gemfile"
            | "gems.rb"
            | "Gemfile.lock"
            | "gems.locked"
            | "pom.xml"
    ) {
        return None;
    }
    if text.len() > 2 * 1024 * 1024 || text.lines().take(50001).count() > 50000 {
        return Some(failure(Failure::Limit));
    }
    Some(match name {
        "go.mod" => go(text),
        "requirements.txt" => requirements(text),
        "Gemfile" | "gems.rb" => gemfile(text),
        "pom.xml" => pom(text),
        _ => gemlock(text),
    })
}
fn preserve(b: &mut Build, name: &str, raw: &str, line: usize) {
    if let Ok(parsed) = heuristic(name, raw) {
        for mut item in parsed.items {
            if b.items.len() >= 10000 {
                b.status = DetectorStatus::Truncated;
                break;
            }
            item.ordinal = b.items.len();
            item.line = line;
            b.items.push(item);
        }
    }
}
fn go(text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    let mut block = false;
    let mut metadata_block: Option<&str> = None;
    let mut directives = std::collections::BTreeSet::new();
    let mut incomplete = false;
    let go_version = Regex::new(r"^[0-9]+\.[0-9]+(?:\.[0-9]+)?$")?;
    for (i, raw) in text.lines().enumerate() {
        let l = raw.split_once("//").map_or(raw, |(l, _)| l).trim();
        if l.is_empty() {
            continue;
        }
        if let Some(kind) = metadata_block {
            if l == ")" {
                metadata_block = None;
                continue;
            }
            if !valid_go_metadata(kind, l) {
                incomplete = true;
            }
            continue;
        }
        if !block {
            let (kind, body) = l.split_once(char::is_whitespace).unwrap_or((l, ""));
            if [
                "godebug",
                "tool",
                "replace",
                "exclude",
                "retract",
                "toolchain",
                "ignore",
            ]
            .contains(&kind)
            {
                if body.trim() == "(" {
                    metadata_block = Some(kind);
                } else if !valid_go_metadata(kind, body.trim()) {
                    incomplete = true;
                }
                continue;
            }
        }
        if l == "require (" && !block {
            block = true;
            continue;
        }
        if l == ")" && block {
            block = false;
            continue;
        }
        let parts: Vec<_> = l.split_whitespace().collect();
        if !block
            && parts.len() == 2
            && parts[0] == "module"
            && valid_dependency_name(parts[1], "golang")
        {
            incomplete |= !directives.insert("module");
            continue;
        }
        if !block && parts.len() == 2 && parts[0] == "go" && go_version.is_match(parts[1]) {
            incomplete |= !directives.insert("go");
            continue;
        }
        let fields = if block {
            parts.as_slice()
        } else if parts.first() == Some(&"require") {
            &parts[1..]
        } else {
            incomplete = true;
            continue;
        };
        if fields.len() == 2 && valid_go_module_version(fields[1]) {
            b.add(
                "golang",
                fields[0],
                Some(fields[1]),
                Some(fields[1]),
                None,
                Origin::Manifest,
                i + 1,
                false,
            );
        } else {
            incomplete = true;
        }
    }
    finish(b, incomplete || block || metadata_block.is_some())
}
fn valid_go_module_version(value: &str) -> bool {
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let Some(c) = RE.get_or_init(|| Regex::new(r"^v(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$").unwrap()).captures(value) else { return false; };
    c.get(1).is_none_or(|pre| {
        pre.as_str().split('.').all(|part| {
            !part.bytes().all(|b| b.is_ascii_digit()) || part.len() == 1 || !part.starts_with('0')
        })
    })
}

fn valid_go_metadata(kind: &str, body: &str) -> bool {
    let fields: Vec<_> = body.split_whitespace().collect();
    let version = valid_go_module_version;
    match kind {
        "tool" => fields.len() == 1 && valid_dependency_name(fields[0], "golang"),
        "ignore" => fields.len() == 1 && !fields[0].contains(['\0', '"']),
        "toolchain" => {
            fields.len() == 1
                && (fields[0] == "default"
                    || regex::Regex::new(r"^go[0-9]+\.[0-9]+(?:\.[0-9]+)?$")
                        .unwrap()
                        .is_match(fields[0]))
        }
        "godebug" => {
            fields.len() == 1
                && fields[0].split_once('=').is_some_and(|(k, v)| {
                    !k.is_empty()
                        && !v.is_empty()
                        && k.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
                        && v.bytes()
                            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
                })
        }
        "replace" => fields.iter().position(|f| *f == "=>").is_some_and(|i| {
            (i == 1 || i == 2)
                && fields.len() > i + 1
                && fields.len() <= i + 3
                && valid_dependency_name(fields[0], "golang")
                && (i == 1 || version(fields[1]))
                && if fields[i + 1].starts_with('.') || fields[i + 1].starts_with('/') {
                    fields.len() == i + 2 && !fields[i + 1].contains(['\0', '"'])
                } else {
                    fields.len() == i + 3
                        && valid_dependency_name(fields[i + 1], "golang")
                        && version(fields[i + 2])
                }
        }),
        "exclude" => {
            fields.len() == 2 && valid_dependency_name(fields[0], "golang") && version(fields[1])
        }
        "retract" => {
            if let Some(range) = body.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                range
                    .split_once(',')
                    .is_some_and(|(lo, hi)| version(lo.trim()) && version(hi.trim()))
            } else {
                version(body)
            }
        }

        _ => false,
    }
}
pub(super) fn requirements(text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    let mut incomplete = false;
    let re = Regex::new(
        r"^([A-Za-z0-9][A-Za-z0-9_.-]*)\s*((?:(?:===|==|!=|~=|>=|<=|>|<)\s*[0-9][A-Za-z0-9*_.+!-]*\s*)(?:,\s*(?:===|==|!=|~=|>=|<=|>|<)\s*[0-9][A-Za-z0-9*_.+!-]*\s*)*)?$",
    )?;
    let constraint = Regex::new(
        r"(?i)^(?:===|==|!=|~=|>=|<=|>|<)\s*(?:[0-9]+!)?[0-9]+(?:\.[0-9]+)*(?:(?:a|b|rc)[0-9]+)?(?:\.post[0-9]+)?(?:\.dev[0-9]+)?(?:\+[a-z0-9]+(?:[._-][a-z0-9]+)*)?(?:\.\*)?$",
    )?;
    let hash = Regex::new(
        r"(?:^|[ \t]+)--hash=(?:sha256:[A-Fa-f0-9]{64}|sha384:[A-Fa-f0-9]{96}|sha512:[A-Fa-f0-9]{128})(?:$|[ \t]+)",
    )?;
    for (line, l) in logical_lines(text, true) {
        let mut stripped = l.clone();
        while hash.is_match(&stripped) {
            stripped = hash.replace_all(&stripped, " ").into_owned();
        }
        let l = stripped.trim().to_string();
        if l.is_empty() {
            continue;
        }
        if let Some(c) = re.captures(&l) {
            let spec = c.get(2).map(|m| m.as_str().trim());
            if spec.is_some_and(|spec| {
                spec.split(',').any(|part| {
                    let part = part.trim();
                    !constraint.is_match(part)
                        || (part.contains('*')
                            && !part.starts_with("==")
                            && !part.starts_with("!="))
                })
            }) {
                incomplete = true;
                preserve(&mut b, "requirements.txt", &l, line);
                continue;
            }
            let count = b.items.len();
            b.add(
                "pypi",
                &c[1],
                spec,
                spec,
                None,
                Origin::Manifest,
                line,
                false,
            );
            // Preserve the flat inventory's historical first-version projection;
            // the full comparator list remains in the declaration facts.
            if b.items.len() > count {
                if let Some(spec) = spec {
                    let version = spec
                        .trim_start_matches(['=', '>', '<', '!', '~'])
                        .trim()
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim();
                    b.items.last_mut().unwrap().version = version.to_string();
                }
            }
        } else {
            incomplete = true;
            preserve(&mut b, "requirements.txt", &l, line);
        }
    }
    finish(b, incomplete)
}
fn gemfile(text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    let mut incomplete = false;
    let gem = Regex::new(
        r#"^gem\s+(['"])([A-Za-z0-9_.-]+)['"]\s*((?:,\s*['"][A-Za-z0-9~><=!*., +_-]+['"]\s*)*)$"#,
    )?;
    let quoted = Regex::new(r#"['"]([^'"]*)['"]"#)?;
    let constraint = Regex::new(
        r"^(?:(?:~>|>=|<=|!=|==|=|>|<)\s*)?[0-9]+(?:\.[A-Za-z0-9]+)*(?:-[A-Za-z0-9.-]+)?$",
    )?;
    let option = Regex::new(
        r",\s*(?:(?:require|platform)\s*:|:(?:require|platform)\s*=>)\s*(?:false|true|:[A-Za-z_][A-Za-z0-9_]*)\s*$",
    )?;
    for (line, l) in logical_lines(text, false) {
        let mut l = l;
        while let Some(m) = option.find(&l) {
            l.truncate(m.start());
        }
        if l.is_empty()
            || l == "source 'https://rubygems.org'"
            || l == "source \"https://rubygems.org\""
        {
            continue;
        }
        if let Some(c) = gem.captures(&l) {
            // Check matching quote pairs, prohibit interpolation and executable arguments.
            let strings: Vec<_> = quoted.captures_iter(&l).collect();
            if strings.is_empty()
                || strings[0][1] != c[2]
                || strings.iter().any(|s| {
                    let raw = s.get(0).unwrap().as_str().as_bytes();
                    raw.first() != raw.last()
                })
            {
                incomplete = true;
                continue;
            }
            let spec = strings
                .iter()
                .skip(1)
                .map(|s| s[1].to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if !spec.is_empty()
                && spec
                    .split(',')
                    .any(|part| !constraint.is_match(part.trim()))
            {
                incomplete = true;
                preserve(&mut b, "Gemfile", &l, line);
                continue;
            }
            b.add(
                "gem",
                &c[2],
                (!spec.is_empty()).then_some(spec.as_str()),
                (!spec.is_empty()).then_some(spec.as_str()),
                None,
                Origin::Manifest,
                line,
                false,
            );
        } else {
            incomplete = true;
            preserve(&mut b, "Gemfile", &l, line);
        }
    }
    finish(b, incomplete)
}
fn gemlock(text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    let mut incomplete = false;
    let mut section = "";
    let mut specs = false;
    let mut source = Origin::LockUnverified;
    let mut remote = false;
    let mut revision = false;
    // A linear metadata pass provides corroborating platform labels before specs.
    let mut in_platforms = false;
    let mut platforms = std::collections::BTreeSet::new();
    for line in text.lines() {
        if !line.starts_with(' ') && !line.trim().is_empty() {
            in_platforms = line == "PLATFORMS";
        } else if in_platforms {
            let value = line.trim();
            if !value.is_empty()
                && value.len() <= 128
                && value
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
            {
                platforms.insert(value);
            }
        }
    }
    let spec = Regex::new(r"^    ([A-Za-z0-9_.-]+) \(([A-Za-z0-9_.+-]+)\)$")?;
    let nested = Regex::new(r"^      [A-Za-z0-9_.-]+(?: \([A-Za-z0-9~><=!*., +_-]+\))?$")?;
    let declaration = Regex::new(r"^  [A-Za-z0-9_.-]+(?: \([A-Za-z0-9~><=!*., +_-]+\))?!?$")?;
    for (i, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        if !raw.starts_with(' ') {
            incomplete |=
                section == "GIT" && (!remote || !revision) || section == "PATH" && !remote;
            remote = false;
            revision = false;
            section = raw;
            specs = false;
            source = Origin::LockUnverified;
            if !matches!(
                section,
                "GEM"
                    | "GIT"
                    | "PATH"
                    | "PLATFORMS"
                    | "DEPENDENCIES"
                    | "BUNDLED WITH"
                    | "RUBY VERSION"
                    | "CHECKSUMS"
            ) {
                incomplete = true;
            }
            continue;
        }
        if matches!(section, "GEM" | "GIT" | "PATH") {
            if raw == "  specs:" {
                specs = true;
                continue;
            }
            if raw == "  remote: https://rubygems.org/" && section == "GEM" && !specs {
                source = Origin::LockResolved;
                continue;
            }
            if !specs && raw.starts_with("  remote:") {
                remote = raw.len() > "  remote: ".len();
            }
            if !specs && raw.starts_with("  revision:") {
                revision = raw.trim_start().strip_prefix("revision:").is_some_and(|s| {
                    let s = s.trim();
                    (7..=64).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_hexdigit())
                });
            }
            if !specs
                && matches!(section, "GIT" | "PATH")
                && [
                    "  remote:",
                    "  revision:",
                    "  branch:",
                    "  tag:",
                    "  ref:",
                    "  glob:",
                    "  submodules:",
                ]
                .iter()
                .any(|prefix| raw.starts_with(prefix) && raw.len() > prefix.len())
            {
                continue;
            }
            if let Some(c) = spec.captures(raw).filter(|_| specs) {
                let origin = match section {
                    "GIT" => Origin::Git,
                    "PATH" => Origin::Path,
                    _ => source,
                };
                let resolved = matches!(origin, Origin::LockResolved)
                    .then(|| gemlock_registry_version(&c[2], &platforms))
                    .flatten();
                b.add(
                    "gem",
                    &c[1],
                    Some(&c[2]),
                    None,
                    resolved,
                    origin,
                    i + 1,
                    false,
                );
                continue;
            }
            if specs && nested.is_match(raw) {
                continue;
            }
            incomplete = true;
        } else if !(match section {
            "DEPENDENCIES" => declaration.is_match(raw),
            "PLATFORMS" => {
                raw.starts_with("  ")
                    && raw
                        .trim()
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
            }
            "BUNDLED WITH" => raw.starts_with("  ") && exact_dependency_version(raw.trim()),
            "RUBY VERSION" => raw.trim().starts_with("ruby "),
            "CHECKSUMS" => raw.starts_with("  ") && !raw.contains('\0'),
            _ => false,
        }) {
            incomplete = true;
        }
    }
    incomplete |= section == "GIT" && (!remote || !revision) || section == "PATH" && !remote;
    finish(b, incomplete)
}
// Keep the original platform-qualified record as reported_version. Only a
// corroborated platform suffix may be removed for the resolved registry version.
fn gemlock_registry_version<'a>(
    record: &'a str,
    platforms: &std::collections::BTreeSet<&str>,
) -> Option<&'a str> {
    if resolved_dependency_version("gem", record) {
        return Some(record);
    }
    if record.len() > 256 {
        return None;
    }
    for (offset, byte) in record.bytes().enumerate() {
        if byte != b'-' {
            continue;
        }
        let suffix = &record[offset + 1..];
        let declared = suffix != "ruby"
            && (platforms.contains(suffix)
                || suffix
                    .strip_suffix("-gnu")
                    .filter(|base| base.ends_with("-linux"))
                    .is_some_and(|base| platforms.contains(base)));
        let version = &record[..offset];
        if declared && resolved_dependency_version("gem", version) {
            return Some(version);
        }
    }
    None
}

fn pom(text: &str) -> anyhow::Result<Parsed> {
    let file = ScannableFile::from_bytes("pom.xml".into(), text.as_bytes().to_vec());
    pom_with_context(text, &super::maven_context::load(&file, None))
}
pub(super) fn pom_with_context(
    text: &str,
    context: &super::maven_context::Context,
) -> anyhow::Result<Parsed> {
    if text.contains("<!DOCTYPE") || text.contains("<!ENTITY") {
        return Ok(Parsed {
            items: vec![],
            status: DetectorStatus::Unsupported,
        });
    }
    let doc = match roxmltree::Document::parse_with_options(
        text,
        roxmltree::ParsingOptions {
            nodes_limit: 50000,
            ..Default::default()
        },
    ) {
        Ok(doc) => doc,
        Err(roxmltree::Error::NodesLimitReached) => return failure(Failure::Limit),
        Err(_) => anyhow::bail!("invalid dependency XML document"),
    };
    if doc
        .descendants()
        .any(|n| n.ancestors().take(66).count() > 65)
    {
        return failure(Failure::Limit);
    }
    let mut b = Build::new(DetectorStatus::Complete);
    let root = doc.root_element();
    let mut incomplete = root.tag_name().name() != "project";
    let namespace = "http://maven.apache.org/POM/4.0.0";
    for n in doc.descendants().filter(|n| {
        n.is_element()
            && matches!(
                n.tag_name().name(),
                "project" | "dependencies" | "dependency"
            )
    }) {
        incomplete |= n
            .children()
            .any(|c| c.is_text() && c.text().is_some_and(|s| !s.trim().is_empty()));
        incomplete |= n.attributes().any(|a| {
            !(n.tag_name().name() == "project"
                && a.name() == "schemaLocation"
                && a.namespace() == Some("http://www.w3.org/2001/XMLSchema-instance"))
        });
    }
    incomplete |= doc
        .descendants()
        .filter(|n| n.is_element())
        .any(|n| n.tag_name().namespace().is_some_and(|ns| ns != namespace));
    let mut seen = std::collections::BTreeSet::new();
    let supported = [
        "modelVersion",
        "parent",
        "groupId",
        "artifactId",
        "version",
        "packaging",
        "name",
        "description",
        "url",
        "inceptionYear",
        "organization",
        "licenses",
        "developers",
        "contributors",
        "mailingLists",
        "prerequisites",
        "modules",
        "scm",
        "issueManagement",
        "ciManagement",
        "distributionManagement",
        "properties",
        "dependencyManagement",
        "dependencies",
        "repositories",
        "pluginRepositories",
        "build",
        "reporting",
        "profiles",
    ];
    for n in root.children().filter(|n| n.is_element()) {
        if !seen.insert(n.tag_name().name()) || !supported.contains(&n.tag_name().name()) {
            incomplete = true;
        }
    }
    let mut properties = context.properties.clone();
    let mut own_properties = std::collections::BTreeSet::new();
    if let Some(props) = root.children().find(|n| n.has_tag_name("properties")) {
        for n in props.children().filter(|n| n.is_element()) {
            if !own_properties.insert(n.tag_name().name())
                && properties
                    .get(n.tag_name().name())
                    .is_some_and(|v| v != n.text().unwrap_or("").trim())
            {
                incomplete = true;
                properties.insert(
                    n.tag_name().name().into(),
                    format!("${{{}}}", n.tag_name().name()),
                );
                continue;
            }
            properties.insert(
                n.tag_name().name().to_string(),
                n.text().unwrap_or("").trim().to_string(),
            );
        }
    }
    for key in ["groupId", "artifactId", "version"] {
        let value = root
            .children()
            .find(|n| n.has_tag_name(key))
            .and_then(|n| n.text())
            .or_else(|| {
                root.children()
                    .find(|n| n.has_tag_name("parent"))
                    .and_then(|n| n.children().find(|c| c.has_tag_name(key)))
                    .and_then(|n| n.text())
            });
        if let Some(value) = value {
            properties.insert(format!("project.{key}"), value.trim().to_string());
            properties.insert(format!("pom.{key}"), value.trim().to_string());
        }
    }
    for n in doc.descendants().filter(|n| {
        n.has_tag_name("dependency")
            && n.parent_element()
                .is_some_and(|p| p.has_tag_name("dependencies"))
    }) {
        let mut fields = std::collections::BTreeSet::new();
        for c in n.children().filter(|c| c.is_element()) {
            if !fields.insert(c.tag_name().name()) {
                incomplete = true;
            }
        }
        let field = |key| {
            n.children()
                .find(|c| c.has_tag_name(key))
                .and_then(|c| c.text())
                .map(str::trim)
        };
        let (Some(group), Some(artifact)) = (field("groupId"), field("artifactId")) else {
            incomplete = true;
            continue;
        };
        let own_profile = n.ancestors().find(|n| n.has_tag_name("profile"));
        let mut base = properties.clone();
        if let Some(profile) = own_profile {
            base.extend(super::maven_context::node_properties(profile));
        }
        let has_references = group.contains('$')
            || artifact.contains('$')
            || field("version").is_some_and(|v| v.contains('$'));
        let baseline_unresolved = expand_maven(group, &base).contains('$')
            || expand_maven(artifact, &base).contains('$')
            || field("version").is_some_and(|v| expand_maven(v, &base).contains('$'));
        let choices = std::iter::once(None)
            .chain(context.profiles.iter().filter(|_| has_references).map(Some));
        let mut emitted = std::collections::BTreeSet::new();
        for profile in choices {
            let mut props = base.clone();
            if let Some(profile) = profile {
                props.extend(profile.clone());
            }
            let g = expand_maven(group, &props);
            let a = expand_maven(artifact, &props);
            let declared = field("version").map(|v| expand_maven(v, &props));
            if g.contains('$')
                || a.contains('$')
                || declared.as_ref().is_some_and(|v| v.contains('$'))
            {
                continue;
            }
            emitted.insert((format!("{g}:{a}"), declared));
        }
        let any = !emitted.is_empty();
        let conditional = own_profile.is_some() || baseline_unresolved || emitted.len() > 1;
        for (name, declared) in emitted {
            b.add(
                "maven",
                &name,
                declared.as_deref(),
                declared.as_deref(),
                None,
                if conditional {
                    Origin::ManifestOptional
                } else {
                    Origin::Manifest
                },
                line_col_for_offset(text, n.range().start).0,
                false,
            );
            if conditional {
                if let Some(item) = b.items.last_mut() {
                    item.context = Some("conditional_profile_declaration_not_effective");
                }
            }
        }
        if !any {
            let group = expand_maven(group, &base);
            let artifact = expand_maven(artifact, &base);
            if !group.contains('$') && !artifact.contains('$') {
                b.add(
                    "maven",
                    &format!("{group}:{artifact}"),
                    None,
                    None,
                    None,
                    Origin::Unknown,
                    line_col_for_offset(text, n.range().start).0,
                    false,
                );
                if let Some(item) = b.items.last_mut() {
                    item.context = Some("unresolved_property_not_effective");
                }
            }
        }
        incomplete |= !any;
    }
    if context.limited {
        b.status = DetectorStatus::Truncated;
    }
    finish(b, incomplete)
}
fn expand_maven(value: &str, properties: &std::collections::BTreeMap<String, String>) -> String {
    let re = Regex::new(r"\$\{([A-Za-z0-9_.-]+)\}").unwrap();
    let mut value = value.to_string();
    for _ in 0..8 {
        let mut next = String::new();
        let mut end = 0;
        for c in re.captures_iter(&value) {
            let m = c.get(0).unwrap();
            let replacement = properties
                .get(&c[1])
                .map(String::as_str)
                .unwrap_or(m.as_str());
            if next
                .len()
                .saturating_add(m.start() - end)
                .saturating_add(replacement.len())
                > 4096
            {
                return value;
            }
            next.push_str(&value[end..m.start()]);
            next.push_str(replacement);
            end = m.end();
        }
        if next.len().saturating_add(value.len() - end) > 4096 {
            return value;
        }
        next.push_str(&value[end..]);
        if next == value {
            break;
        }
        value = next;
    }
    value
}
