use super::*;

pub(super) fn inspect(file: &ScannableFile, text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    match file.file_name().unwrap_or("") {
        "conanfile.txt" => conan_text(text),
        "CMakeLists.txt" => cmake(text),
        _ if file.ext() == Some("cmake") => cmake(text),
        "conan.lock" => conan_lock(text),
        _ => vcpkg(file, text),
    }
}
fn port_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 100
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
fn features(b: &mut Build, text: &SourceText<'_>, port: &str, value: &Value) {
    let Some(values) = value.as_array() else {
        b.reject();
        return;
    };
    for feature in values {
        let Some(feature) = feature.as_str().filter(|f| port_name(f)) else {
            b.reject();
            continue;
        };
        add(
            b,
            "vcpkg-feature",
            &format!("{port}/{feature}"),
            None,
            Origin::Manifest,
            line(text, feature),
            Some(Context::FeatureDeclared),
        );
    }
}
fn ports(b: &mut Build, text: &SourceText<'_>, value: &Value, optional: bool) {
    let Some(values) = value.as_array() else {
        b.reject();
        return;
    };
    for port in values {
        let name = port
            .as_str()
            .or_else(|| port.get("name").and_then(Value::as_str));
        let Some(name) = name.filter(|n| port_name(n)) else {
            b.reject();
            continue;
        };
        let mut version = None;
        let conditional = optional || port.get("platform").is_some();
        if let Some(o) = port.as_object() {
            version = match field(o, "version>=") {
                Ok(v) => v,
                Err(_) => {
                    b.reject();
                    None
                }
            };
            if let Some(v) = o.get("features") {
                features(b, text, name, v);
            }
            if o.get("host").is_some_and(|v| !v.is_boolean())
                || o.get("default-features").is_some_and(|v| !v.is_boolean())
            {
                b.reject();
            }
        }
        let version = version.map(|v| format!(">={v}"));
        add(
            b,
            "vcpkg",
            name,
            version.as_deref(),
            Origin::Manifest,
            line(text, name),
            conditional.then_some(Context::Conditional),
        );
    }
}
fn baseline(b: &mut Build, text: &SourceText<'_>, name: &str, value: &Value) {
    let Some(hash) = value
        .as_str()
        .filter(|s| s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit()))
    else {
        b.reject();
        return;
    };
    add(
        b,
        "vcpkg-baseline",
        name,
        Some(hash),
        Origin::Manifest,
        line(text, hash),
        Some(Context::BaselineDeclared),
    );
}
fn registry(b: &mut Build, text: &SourceText<'_>, value: &Value, ordinal: usize) {
    let Some(o) = value.as_object() else {
        if !value.is_null() {
            b.reject();
        }
        return;
    };
    let Some(kind) = o.get("kind").and_then(Value::as_str) else {
        b.reject();
        return;
    };
    if !matches!(kind, "builtin" | "git" | "filesystem" | "artifact") {
        b.status = DetectorStatus::Partial;
        return;
    }
    if let Some(v) = o.get("baseline") {
        baseline(b, text, &format!("{kind}-registry-{ordinal}"), v);
    }
    if let Some(v) = o.get("packages") {
        let Some(names) = v.as_array() else {
            b.reject();
            return;
        };
        for name in names {
            if let Some(name) = name.as_str().filter(|s| port_name(s)) {
                add(
                    b,
                    "vcpkg",
                    name,
                    None,
                    Origin::Unknown,
                    line(text, name),
                    None,
                );
            } else {
                b.status = DetectorStatus::Partial;
            }
        }
    }
}
fn configuration(b: &mut Build, text: &SourceText<'_>, o: &Map<String, Value>) {
    if let Some(v) = o.get("default-registry") {
        registry(b, text, v, 0);
    }
    if let Some(v) = o.get("registries") {
        if let Some(values) = v.as_array() {
            for (i, v) in values.iter().enumerate() {
                registry(b, text, v, i + 1);
            }
        } else {
            b.reject();
        }
    }
    // Overlay ports are local/custom, never installed registry assertions.
    if o.contains_key("overlay-ports") || o.contains_key("overlay-triplets") {
        b.status = DetectorStatus::Partial;
    }
}
fn vcpkg(file: &ScannableFile, text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    let value = match json(text) {
        Ok(v) => v,
        Err(e) => return parse::failure(e),
    };
    let o = object(&value)?;
    let mut b = Build::new(DetectorStatus::Complete);
    if file.file_name() == Some("vcpkg-configuration.json") {
        configuration(&mut b, text, o);
        return b.finish();
    }
    if let Some(v) = o.get("dependencies") {
        ports(&mut b, text, v, false);
    }
    if let Some(v) = o.get("builtin-baseline") {
        baseline(&mut b, text, "builtin", v);
    }
    if let Some(v) = o.get("vcpkg-configuration") {
        if let Some(o) = v.as_object() {
            configuration(&mut b, text, o);
        } else {
            b.reject();
        }
    }
    if let Some(v) = o.get("features") {
        if let Some(features_map) = v.as_object() {
            for (feature, spec) in features_map {
                if !port_name(feature) {
                    b.reject();
                    continue;
                }
                let Some(spec) = spec.as_object() else {
                    b.reject();
                    continue;
                };
                let project = o
                    .get("name")
                    .and_then(Value::as_str)
                    .filter(|n| port_name(n))
                    .unwrap_or("manifest");
                add(
                    &mut b,
                    "vcpkg-feature",
                    &format!("{project}/{feature}"),
                    None,
                    Origin::ManifestOptional,
                    line(text, feature),
                    Some(Context::FeatureDeclared),
                );
                if let Some(v) = spec.get("dependencies") {
                    ports(&mut b, text, v, true);
                }
            }
        } else {
            b.reject();
        }
    }
    if let Some(v) = o.get("default-features") {
        features(
            &mut b,
            text,
            o.get("name")
                .and_then(Value::as_str)
                .filter(|n| port_name(n))
                .unwrap_or("manifest"),
            v,
        );
    }
    if let Some(v) = o.get("overrides") {
        if let Some(values) = v.as_array() {
            for v in values {
                let Some(o) = v.as_object() else {
                    b.reject();
                    continue;
                };
                let Some(name) = o
                    .get("name")
                    .and_then(Value::as_str)
                    .filter(|n| port_name(n))
                else {
                    b.reject();
                    continue;
                };
                let versions: Vec<_> = [
                    "version",
                    "version-string",
                    "version-semver",
                    "version-date",
                ]
                .iter()
                .filter_map(|k| o.get(*k))
                .collect();
                if versions.len() != 1 || !versions[0].is_string() {
                    b.reject();
                    continue;
                }
                let mut version = versions[0].as_str().unwrap().to_string();
                if let Some(p) = o.get("port-version") {
                    let Some(p) = p.as_u64() else {
                        b.reject();
                        continue;
                    };
                    version.push_str(&format!("#{p}"));
                }
                add(
                    &mut b,
                    "vcpkg",
                    name,
                    Some(&version),
                    Origin::Manifest,
                    line(text, name),
                    Some(Context::OverrideDeclared),
                );
            }
        } else {
            b.reject();
        }
    }
    b.finish()
}
fn conan_reference(b: &mut Build, reference: &str, line: usize, locked: bool, build: bool) {
    let Some((name, version)) = reference.split_once('/') else {
        b.reject();
        return;
    };
    if !pqc_scan_core::observation::valid_dependency_name(name, "conan") {
        b.reject();
        return;
    }
    // User/channel sources retain only a safe version claim, never a registry identity.
    let (version, custom) = version
        .split_once('@')
        .map(|(v, source)| {
            (
                format!(
                    "{v}{}",
                    source
                        .split_once('#')
                        .map(|(_, rev)| format!("#{rev}"))
                        .unwrap_or_default()
                ),
                true,
            )
        })
        .unwrap_or((version.to_string(), false));
    if let Some((_, revision)) = version.split_once('#') {
        let (hash, timestamp) = revision
            .split_once('%')
            .map(|(a, b)| (a, Some(b)))
            .unwrap_or((revision, None));
        if hash.is_empty()
            || hash.len() > 64
            || !hash.bytes().all(|b| b.is_ascii_hexdigit())
            || timestamp.is_some_and(|s| {
                s.is_empty()
                    || s.len() > 64
                    || s.matches('.').count() > 1
                    || !s.bytes().all(|b| b.is_ascii_digit() || b == b'.')
            })
        {
            b.status = DetectorStatus::Partial;
            return;
        }
    }
    if !pqc_scan_core::observation::safe_dependency_constraint("conan", &version) {
        b.status = DetectorStatus::Partial;
        return;
    }
    let before = b.items.len();
    b.add(
        "conan",
        name,
        Some(&version),
        if locked { None } else { Some(&version) },
        None,
        if custom {
            Origin::Unknown
        } else if locked {
            Origin::LockUnverified
        } else {
            Origin::Manifest
        },
        line,
        false,
    );
    if build {
        b.context(before, Context::BuildRequirement);
    }
}
fn conan_text(text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    let mut b = Build::new(DetectorStatus::Complete);
    let mut section = "";
    if text.lines().take(50001).count() > 50000 {
        return parse::failure(Failure::Limit);
    }
    for (i, raw) in text.lines().enumerate() {
        let s = raw.trim();
        if s.is_empty() || s.starts_with('#') {
            continue;
        }
        if s.starts_with('[') && s.ends_with(']') {
            section = &s[1..s.len() - 1];
            if !matches!(
                section,
                "requires"
                    | "tool_requires"
                    | "build_requires"
                    | "generators"
                    | "options"
                    | "layout"
                    | "conf"
                    | "test_requires"
            ) {
                b.status = DetectorStatus::Partial;
            }
            continue;
        }
        if matches!(
            section,
            "requires" | "tool_requires" | "build_requires" | "test_requires"
        ) {
            conan_reference(&mut b, s, i + 1, false, section != "requires");
        } else if section.is_empty() {
            b.reject();
        }
    }
    b.finish()
}
fn conan_lock(text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    let value = match json(text) {
        Ok(v) => v,
        Err(e) => return parse::failure(e),
    };
    let o = object(&value)?;
    match o.get("version").and_then(Value::as_str) {
        Some("0.5") => {}
        Some(_) => return parse::failure(Failure::Unsupported),
        None => anyhow::bail!("missing Conan lock version"),
    }
    let mut b = Build::new(DetectorStatus::Complete);
    for section in [
        "requires",
        "build_requires",
        "python_requires",
        "config_requires",
    ] {
        if let Some(v) = o.get(section) {
            let Some(values) = v.as_array() else {
                b.reject();
                continue;
            };
            for v in values {
                if let Some(r) = v.as_str() {
                    conan_reference(&mut b, r, line(text, r), true, section != "requires");
                } else {
                    b.reject();
                }
            }
        }
    }
    b.finish()
}

// A finite CMake token subset. Comments, bracket strings and quoted command text
// cannot introduce directives. Variables, nesting and malformed syntax stay incomplete.
#[derive(Debug)]
struct Token {
    value: String,
    line: usize,
    quoted: bool,
}
fn tokens(text: &SourceText<'_>) -> Result<Vec<Token>, Failure> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let (mut i, mut line) = (0, 1);
    while i < bytes.len() {
        if out.len() >= 50000 {
            return Err(Failure::Limit);
        }
        let start = i;
        let at = line;
        if bytes[i] == b'[' {
            if let Some(end) = bracket_end(bytes, i)? {
                line += bytes[i..end].iter().filter(|b| **b == b'\n').count();
                out.push(Token {
                    value: String::new(),
                    line: at,
                    quoted: true,
                });
                i = end;
                continue;
            }
        }
        match bytes[i] {
            b'\n' => {
                line += 1;
                i += 1;
            }
            c if c.is_ascii_whitespace() => i += 1,
            b'#' => {
                i += 1;
                if bytes.get(i) == Some(&b'[') {
                    if let Some(end) = bracket_end(bytes, i)? {
                        line += bytes[i..end].iter().filter(|b| **b == b'\n').count();
                        i = end;
                        continue;
                    }
                }
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'"' => {
                i += 1;
                let begin = i;
                let mut escaped = false;
                while i < bytes.len() {
                    let c = bytes[i];
                    if c == b'\n' {
                        line += 1;
                    }
                    if !escaped && c == b'"' {
                        break;
                    }
                    escaped = !escaped && c == b'\\';
                    i += 1;
                }
                if i == bytes.len() {
                    return Err(Failure::Malformed);
                }
                out.push(Token {
                    value: text[begin..i].into(),
                    line: at,
                    quoted: true,
                });
                i += 1;
            }
            b'(' | b')' => {
                out.push(Token {
                    value: text[i..i + 1].into(),
                    line: at,
                    quoted: false,
                });
                i += 1;
            }
            _ => {
                while i < bytes.len()
                    && !bytes[i].is_ascii_whitespace()
                    && !b"()#\"".contains(&bytes[i])
                {
                    i += 1;
                }
                if i == start {
                    i += 1;
                    continue;
                }
                out.push(Token {
                    value: text[start..i].into(),
                    line: at,
                    quoted: false,
                });
            }
        }
    }
    Ok(out)
}
fn bracket_end(bytes: &[u8], start: usize) -> Result<Option<usize>, Failure> {
    let mut i = start + 1;
    while bytes.get(i) == Some(&b'=') {
        i += 1;
        if i - start > 65 {
            return Err(Failure::Limit);
        }
    }
    if bytes.get(i) != Some(&b'[') {
        return Ok(None);
    }
    let equals = i - start - 1;
    let mut j = i + 1;
    while j < bytes.len() {
        if bytes[j] == b']'
            && bytes
                .get(j + 1..j + 1 + equals)
                .is_some_and(|s| s.iter().all(|b| *b == b'='))
            && bytes.get(j + 1 + equals) == Some(&b']')
        {
            return Ok(Some(j + equals + 2));
        }
        j += 1;
    }
    Err(Failure::Malformed)
}
fn cmake(text: &SourceText<'_>) -> anyhow::Result<Parsed> {
    let tokens = match tokens(text) {
        Ok(t) => t,
        Err(e) => return parse::failure(e),
    };
    let mut b = Build::new(DetectorStatus::Partial);
    let mut i = 0;
    while i < tokens.len() {
        let command = &tokens[i];
        if command.quoted || tokens.get(i + 1).is_none_or(|t| t.quoted || t.value != "(") {
            i += 1;
            continue;
        }
        let mut end = i + 2;
        let mut depth = 1usize;
        let mut nested = false;
        while end < tokens.len() {
            let t = &tokens[end];
            if !t.quoted && t.value == "(" {
                depth += 1;
                nested = true;
            }
            if !t.quoted && t.value == ")" {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            if depth > 64 {
                return parse::failure(Failure::Limit);
            }
            end += 1;
        }
        if end == tokens.len() {
            anyhow::bail!("invalid CMake reference syntax");
        }
        let args = &tokens[i + 2..end];
        if !nested {
            match command.value.to_ascii_lowercase().as_str() {
                "find_package" | "fetchcontent_declare" => {
                    if let Some(name) = args.first() {
                        let canonical = canonical_build_name(&name.value);
                        if let Some(name) = canonical {
                            let version = if command.value.eq_ignore_ascii_case("find_package") {
                                args.get(1).map(|t| t.value.as_str()).filter(|s| {
                                    pqc_scan_core::observation::exact_dependency_version(s)
                                })
                            } else {
                                None
                            };
                            add(
                                &mut b,
                                "cmake",
                                &name,
                                version,
                                Origin::HeuristicManifest,
                                command.line,
                                Some(Context::LexicalReference),
                            );
                        }
                    }
                }
                "target_link_libraries" => {
                    for arg in args.iter().skip(1) {
                        if matches!(
                            arg.value.as_str(),
                            "PUBLIC" | "PRIVATE" | "INTERFACE" | "debug" | "optimized" | "general"
                        ) {
                            continue;
                        }
                        if let Some(name) = canonical_build_name(&arg.value) {
                            add(
                                &mut b,
                                "cmake",
                                &name,
                                None,
                                Origin::HeuristicManifest,
                                command.line,
                                Some(Context::LexicalReference),
                            );
                        }
                    }
                }
                _ => {}
            }
        }
        i = end + 1;
    }
    b.finish()
}
fn canonical_build_name(value: &str) -> Option<String> {
    let lower = value.to_ascii_lowercase();
    let value = match lower.as_str() {
        "openssl::crypto" | "openssl::ssl" => "openssl",
        "oqs" | "oqs::oqs" | "liboqs::oqs" => "liboqs",
        _ => lower.as_str(),
    };
    if !value.is_empty()
        && value.len() <= 100
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        && !value.contains("..")
    {
        Some(value.into())
    } else {
        None
    }
}
