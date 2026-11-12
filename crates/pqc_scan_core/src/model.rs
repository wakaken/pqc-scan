use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use pqc_scan_rules::{Risk, Severity};

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub root: PathBuf,
    pub max_file_size_bytes: usize,
    pub threads: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            max_file_size_bytes: 2 * 1024 * 1024,
            threads: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub r#type: String,
    pub r#match: String,
    pub snippet_preview: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub rule_id: String,
    pub location: Location,
    pub evidence: Evidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub finding_id: String,
    pub rule_id: String,
    pub category: String,
    pub risk: Risk,
    pub severity: Severity,
    pub confidence: f32,
    pub description: String,
    pub migration_hint: String,
    pub location: Location,
    pub evidence: Evidence,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_actions: Vec<RecommendedAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_snippet: Option<SourceSnippet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    pub action_id: String,
    pub title: String,
    pub priority: String,
    pub rationale: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_examples: Vec<CodeExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSnippet {
    pub lines: Vec<SourceSnippetLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSnippetLine {
    pub line: usize,
    pub text: String,
    pub highlighted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CbomEntry {
    pub component: String,
    pub algorithm: String,
    pub usage_type: String,
    pub location: String,
    pub quantum_risk: Risk,
    pub migration_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySbomEntry {
    pub name: String,
    pub version: String,
    pub ecosystem: String,
    pub source_file: String,
    pub source_type: String,
    pub purl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanSummary {
    pub total_findings: usize,
    pub by_severity: BTreeMap<String, usize>,
    pub by_risk: BTreeMap<String, usize>,
    pub scanned_files: usize,
    pub skipped_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub generated_at: DateTime<Utc>,
    pub findings: Vec<Finding>,
    pub cbom: Vec<CbomEntry>,
    pub dependency_sbom: Vec<DependencySbomEntry>,
    pub summary: ScanSummary,
}

#[derive(Debug, Clone)]
pub struct ScannableFile {
    pub path: PathBuf,
    pub size: usize,
    pub bytes: Vec<u8>,
    pub text: Option<String>,
}

impl ScannableFile {
    pub fn from_bytes(path: PathBuf, bytes: Vec<u8>) -> Self {
        // PKCS#12 is an opaque container even when hostile or malformed bytes
        // happen to look like text; never feed key-bag contents to text detectors.
        let opaque = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("p12") || e.eq_ignore_ascii_case("pfx"));
        let text = if opaque {
            None
        } else {
            crate::text_encoding::decode(&bytes)
        };

        Self {
            path,
            size: bytes.len(),
            bytes,
            text,
        }
    }

    /// Map decoded UTF-8 character boundaries to original file byte offsets.
    /// None means ordinary UTF-8 needs no remapping, or there is no decoded text.
    pub fn decoded_source_offsets(&self) -> Option<Vec<(usize, usize)>> {
        self.text
            .as_deref()
            .and_then(|text| crate::text_encoding::source_offsets(&self.bytes, text))
    }

    pub fn ext(&self) -> Option<&str> {
        self.path.extension().and_then(|x| x.to_str())
    }

    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|x| x.to_str())
    }

    pub fn as_relative_path<'a>(&'a self, root: &'a Path) -> String {
        self.path
            .strip_prefix(root)
            .unwrap_or(&self.path)
            .to_string_lossy()
            .into_owned()
    }
}

pub fn line_col_for_offset(text: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;

    for (idx, ch) in text.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

pub fn snippet_around(text: &str, start: usize, end: usize, radius: usize) -> String {
    let left = floor_char_boundary(text, start.saturating_sub(radius));
    let right = ceil_char_boundary(text, usize::min(end.saturating_add(radius), text.len()));
    let mut snippet = text[left..right].replace('\n', " ");
    if snippet.chars().count() > 180 {
        snippet = snippet.chars().take(180).collect::<String>();
        snippet.push_str("...");
    }
    mask_preview(&snippet)
}

pub fn mask_preview(raw: &str) -> String {
    let text = raw.trim();
    if text.contains("PRIVATE KEY") {
        return "[masked-private-key]".to_string();
    }
    if text.chars().count() <= 20 {
        return text.to_string();
    }
    let head: String = text.chars().take(8).collect();
    let tail: String = text
        .chars()
        .rev()
        .take(6)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{}***{}", head, tail)
}

fn floor_char_boundary(text: &str, mut idx: usize) -> usize {
    idx = usize::min(idx, text.len());
    while idx > 0 && !text.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

fn ceil_char_boundary(text: &str, mut idx: usize) -> usize {
    idx = usize::min(idx, text.len());
    while idx < text.len() && !text.is_char_boundary(idx) {
        idx += 1;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn mask_preview_masks_private_key_markers() {
        assert_eq!(
            mask_preview("-----BEGIN PRIVATE KEY-----"),
            "[masked-private-key]"
        );
    }

    #[test]
    fn mask_preview_truncates_long_text() {
        assert_eq!(
            mask_preview("abcdefghijklmnopqrstuvwxyz"),
            "abcdefgh***uvwxyz"
        );
    }

    #[test]
    fn line_col_for_offset_handles_multiline_text() {
        let text = "alpha\nbeta\ngamma";
        let offset = text.find("gamma").expect("gamma");
        assert_eq!(line_col_for_offset(text, offset), (3, 1));
    }

    #[test]
    fn snippet_around_clips_and_masks_sensitive_content() {
        let text = "prefix -----BEGIN PRIVATE KEY----- suffix";
        let start = text.find("PRIVATE").expect("private");
        let end = start + "PRIVATE KEY".len();
        assert_eq!(snippet_around(text, start, end, 10), "[masked-private-key]");
    }

    #[test]
    fn scannable_file_detects_binary_and_relative_paths() {
        let file =
            ScannableFile::from_bytes(PathBuf::from("/repo/src/app.bin"), vec![0, 159, 146, 150]);
        assert!(file.text.is_none());
        assert_eq!(
            file.as_relative_path(std::path::Path::new("/repo")),
            "src/app.bin"
        );
    }
}

#[cfg(test)]
mod opaque_container_tests {
    use super::*;
    #[test]
    fn pkcs12_payloads_are_never_exposed_as_source_text() {
        for ext in ["p12", "pfx", "P12", "PFX"] {
            let file = ScannableFile::from_bytes(
                PathBuf::from(format!("container.{ext}")),
                b"opaque synthetic container payload without NUL".to_vec(),
            );
            assert!(file.text.is_none());
        }
    }
}

#[cfg(test)]
mod wide_text_repairs {
    use super::*;
    #[test]
    fn utf16_source_is_decoded_with_and_without_bom() {
        let source = "const key = 'RS256';\n";
        for bom in [true, false] {
            for little in [true, false] {
                let mut bytes = if bom {
                    if little {
                        vec![0xff, 0xfe]
                    } else {
                        vec![0xfe, 0xff]
                    }
                } else {
                    vec![]
                };
                for c in source.encode_utf16() {
                    bytes.extend(if little {
                        c.to_le_bytes()
                    } else {
                        c.to_be_bytes()
                    });
                }
                let file = ScannableFile::from_bytes("code.js".into(), bytes);
                assert_eq!(file.text.as_deref(), Some(source));
            }
        }
    }
    #[test]
    fn utf32_source_is_decoded_with_and_without_bom() {
        let source = "const 日 = 'RS256';\n";
        for bom in [true, false] {
            for little in [true, false] {
                let mut bytes = if bom {
                    if little {
                        vec![0xff, 0xfe, 0, 0]
                    } else {
                        vec![0, 0, 0xfe, 0xff]
                    }
                } else {
                    vec![]
                };
                for c in source.chars() {
                    bytes.extend(if little {
                        (c as u32).to_le_bytes()
                    } else {
                        (c as u32).to_be_bytes()
                    });
                }
                let file = ScannableFile::from_bytes("code.js".into(), bytes);
                assert_eq!(file.text.as_deref(), Some(source));
            }
        }
    }
    #[test]
    fn encrypted_containers_stay_opaque_even_with_unicode_bom() {
        let f = ScannableFile::from_bytes("opaque.p12".into(), vec![0xff, 0xfe, b'a', 0]);
        assert!(f.text.is_none());
    }
}
