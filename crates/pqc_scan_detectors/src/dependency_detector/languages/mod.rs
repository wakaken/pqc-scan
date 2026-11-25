//! Offline source assertions for v0.5 formats; never a resolver or build evaluator.
mod c_family;
mod composer;
mod nuget;
use super::parse::{self, decode, Build, DetectorStatus, Failure, Format, Origin, Parsed};
use pqc_scan_core::{observation::DependencyContext as Context, RepositoryContext, ScannableFile};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
struct SourceText<'a> {
    text: &'a str,
    lines: BTreeMap<String, usize>,
    newlines: Vec<usize>,
}
impl std::ops::Deref for SourceText<'_> {
    type Target = str;
    fn deref(&self) -> &str {
        self.text
    }
}
impl<'a> SourceText<'a> {
    fn new(text: &'a str) -> Self {
        let mut lines = BTreeMap::new();
        let bytes = text.as_bytes();
        let (mut i, mut line) = (0, 1);
        // One bounded pass, rather than rescanning the whole input per record.
        while i < bytes.len() && lines.len() < 50000 {
            if bytes[i] == b'\n' {
                line += 1;
            }
            if bytes[i] != b'"' {
                i += 1;
                continue;
            }
            let start = i;
            let at = line;
            i += 1;
            let mut escaped = false;
            while i < bytes.len() {
                let c = bytes[i];
                if c == b'\n' {
                    line += 1;
                }
                if !escaped && c == b'"' {
                    i += 1;
                    break;
                }
                escaped = !escaped && c == b'\\';
                i += 1;
            }
            if let Ok(value) = serde_json::from_str::<String>(&text[start..i]) {
                lines
                    .entry(value)
                    .and_modify(|line| *line = 0)
                    .or_insert(at);
            }
        }
        Self {
            text,
            lines,
            newlines: text.match_indices('\n').map(|(i, _)| i).collect(),
        }
    }
    fn line_at(&self, offset: usize) -> usize {
        1 + self.newlines.partition_point(|i| *i < offset)
    }
}

pub(super) fn accepts(file: &ScannableFile) -> bool {
    matches!(
        file.file_name(),
        Some(
            "composer.json"
                | "composer.lock"
                | "Directory.Packages.props"
                | "Directory.Build.props"
                | "packages.config"
                | "packages.lock.json"
                | "global.json"
                | "vcpkg.json"
                | "vcpkg-configuration.json"
                | "conanfile.txt"
                | "conan.lock"
                | "CMakeLists.txt"
        )
    ) || matches!(file.ext(), Some("csproj" | "cmake"))
}
pub(super) fn inspect(
    file: &ScannableFile,
    repo: Option<&RepositoryContext>,
) -> anyhow::Result<Parsed> {
    let Some(text) = file.text.as_deref() else {
        return parse::failure(Failure::Unsupported);
    };
    if text.len() > 2 * 1024 * 1024 {
        return parse::failure(Failure::Limit);
    }
    let source = SourceText::new(text);
    let text = &source;
    match file.file_name().unwrap_or("") {
        "composer.json" | "composer.lock" => composer::inspect(file, text),
        "vcpkg.json"
        | "vcpkg-configuration.json"
        | "conanfile.txt"
        | "conan.lock"
        | "CMakeLists.txt" => c_family::inspect(file, text),
        _ if file.ext() == Some("cmake") => c_family::inspect(file, text),
        _ => nuget::inspect(file, text, repo),
    }
}
fn json(text: &str) -> Result<Value, Failure> {
    // Enforce the new-format budget during construction, not after allocation.
    decode::decode_limited(text, Format::Json, 50_000)
}
fn object(value: &Value) -> anyhow::Result<&Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("invalid dependency document shape"))
}
fn field<'a>(o: &'a Map<String, Value>, key: &str) -> anyhow::Result<Option<&'a str>> {
    o.get(key)
        .map(|v| {
            v.as_str()
                .ok_or_else(|| anyhow::anyhow!("invalid dependency field"))
        })
        .transpose()
}
fn line(text: &SourceText<'_>, value: &str) -> usize {
    text.lines.get(value).copied().unwrap_or(0)
}
fn add(
    b: &mut Build,
    ecosystem: &str,
    name: &str,
    version: Option<&str>,
    origin: Origin,
    line: usize,
    context: Option<Context>,
) {
    let before = b.items.len();
    b.add(ecosystem, name, version, version, None, origin, line, false);
    if let Some(context) = context {
        b.context(before, context);
    }
}
