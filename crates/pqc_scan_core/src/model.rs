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
        let text = if is_probably_binary(&bytes) {
            None
        } else {
            Some(String::from_utf8_lossy(&bytes).into_owned())
        };

        Self {
            path,
            size: bytes.len(),
            bytes,
            text,
        }
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

fn is_probably_binary(bytes: &[u8]) -> bool {
    let sample_len = usize::min(1024, bytes.len());
    bytes[..sample_len].contains(&0)
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
