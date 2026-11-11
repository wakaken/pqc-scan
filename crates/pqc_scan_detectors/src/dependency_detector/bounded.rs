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
    let mut directives = std::collections::BTreeSet::new();
    let mut incomplete = false;
    let version = Regex::new(r"^v[0-9]+\.[0-9]+\.[0-9]+(?:[-+][A-Za-z0-9.+-]+)?$")?;
    let go_version = Regex::new(r"^[0-9]+\.[0-9]+(?:\.[0-9]+)?$")?;
    for (i, raw) in text.lines().enumerate() {
        let l = raw.split_once("//").map_or(raw, |(l, _)| l).trim();
        if l.is_empty() {
            continue;
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
        if fields.len() == 2 && version.is_match(fields[1]) {
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
    finish(b, incomplete || block)
}
fn requirements(text: &str) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    let mut incomplete = false;
    let re = Regex::new(
        r"^([A-Za-z0-9][A-Za-z0-9_.-]*)\s*((?:(?:===|==|!=|~=|>=|<=|>|<)\s*[0-9][A-Za-z0-9*_.+!-]*\s*)(?:,\s*(?:===|==|!=|~=|>=|<=|>|<)\s*[0-9][A-Za-z0-9*_.+!-]*\s*)*)?$",
    )?;
    let constraint = Regex::new(
        r"(?i)^(?:===|==|!=|~=|>=|<=|>|<)\s*(?:[0-9]+!)?[0-9]+(?:\.[0-9]+)*(?:(?:a|b|rc)[0-9]+)?(?:\.post[0-9]+)?(?:\.dev[0-9]+)?(?:\+[a-z0-9]+(?:[._-][a-z0-9]+)*)?(?:\.\*)?$",
    )?;
    for (line, l) in logical_lines(text, true) {
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
    for (line, l) in logical_lines(text, false) {
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
    let spec = Regex::new(r"^    ([A-Za-z0-9_.-]+) \(([A-Za-z0-9_.+-]+)\)$")?;
    let nested = Regex::new(r"^      [A-Za-z0-9_.-]+(?: \([A-Za-z0-9~><=!*., +_-]+\))?$")?;
    let declaration = Regex::new(r"^  [A-Za-z0-9_.-]+(?: \([A-Za-z0-9~><=!*., +_-]+\))?!?$")?;
    for (i, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        if !raw.starts_with(' ') {
            section = raw;
            specs = false;
            source = Origin::LockUnverified;
            if !matches!(
                section,
                "GEM" | "PLATFORMS" | "DEPENDENCIES" | "BUNDLED WITH" | "RUBY VERSION"
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
            if let Some(c) = spec.captures(raw).filter(|_| specs) {
                let origin = match section {
                    "GIT" => Origin::Git,
                    "PATH" => Origin::Path,
                    _ => source,
                };
                let resolved = matches!(origin, Origin::LockResolved).then_some(&c[2]);
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
            "BUNDLED WITH" => raw.starts_with("   ") && exact_dependency_version(raw.trim()),
            _ => false,
        }) {
            incomplete = true;
        }
    }
    finish(b, incomplete)
}
fn pom(text: &str) -> anyhow::Result<Parsed> {
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
    let mut incomplete = doc.root_element().tag_name().name() != "project";
    for n in doc.descendants().filter(|n| n.is_element()) {
        let tag = n.tag_name().name();
        if n.tag_name()
            .namespace()
            .is_some_and(|ns| ns != "http://maven.apache.org/POM/4.0.0")
        {
            incomplete = true;
        }
        if matches!(tag, "project" | "dependencies" | "dependency")
            && n.children()
                .any(|c| c.is_text() && c.text().is_some_and(|t| !t.trim().is_empty()))
        {
            incomplete = true;
        }
        if n.attributes().any(|a| {
            !(tag == "project"
                && a.name() == "schemaLocation"
                && a.namespace() == Some("http://www.w3.org/2001/XMLSchema-instance"))
        }) {
            incomplete = true;
        }
        let mut fields = std::collections::BTreeSet::new();
        for child in n.children().filter(|c| c.is_element()) {
            if !matches!(tag, "dependencies") && !fields.insert(child.tag_name().name()) {
                incomplete = true;
            }
        }
        let parent = n
            .parent_element()
            .map(|p| p.tag_name().name())
            .unwrap_or("");
        let supported = match parent {
            "" => tag == "project",
            "project" => matches!(
                tag,
                "modelVersion"
                    | "groupId"
                    | "artifactId"
                    | "version"
                    | "packaging"
                    | "name"
                    | "description"
                    | "dependencies"
            ),
            "dependencies" => {
                tag == "dependency"
                    && n.parent_element()
                        .and_then(|p| p.parent_element())
                        .is_some_and(|p| p.tag_name().name() == "project")
            }
            "dependency" => matches!(
                tag,
                "groupId" | "artifactId" | "version" | "scope" | "optional" | "type" | "classifier"
            ),
            _ => false,
        };
        if !supported {
            incomplete = true;
        }
        if tag != "dependency" {
            continue;
        }
        let field = |key| {
            let mut nodes = n
                .children()
                .filter(|c| c.is_element() && c.tag_name().name() == key);
            let first = nodes.next();
            if nodes.next().is_some() {
                None
            } else {
                first.and_then(|c| c.text()).map(str::trim)
            }
        };
        let (Some(group), Some(artifact)) = (field("groupId"), field("artifactId")) else {
            incomplete = true;
            continue;
        };
        let version = field("version");
        let dynamic = [Some(group), Some(artifact), version]
            .into_iter()
            .flatten()
            .any(|s| s.contains('$'));
        if dynamic {
            incomplete = true;
        }
        let name = format!("{group}:{artifact}");
        let version = version.filter(|v| !v.contains('$'));
        b.add(
            "maven",
            &name,
            version,
            version,
            None,
            Origin::Manifest,
            line_col_for_offset(text, n.range().start).0,
            false,
        );
    }
    finish(b, incomplete)
}
