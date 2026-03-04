use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use rayon::prelude::*;

use pqc_scan_rules::{RuleSet, Severity};

use crate::model::{
    CbomEntry, DependencySbomEntry, Detection, Finding, ScanConfig, ScanResult, ScanSummary,
    ScannableFile, SourceSnippet, SourceSnippetLine,
};
use crate::{recommendation, risk, walker, Detector};

pub fn scan_repository(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
) -> Result<ScanResult> {
    let (files, walk_stats) = walker::walk_repository(&config.root, config.max_file_size_bytes)?;

    let thread_count = if config.threads == 0 {
        num_cpus::get()
    } else {
        config.threads
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()?;

    let detections: Vec<Detection> = pool.install(|| {
        files
            .par_iter()
            .flat_map(|path| {
                let bytes = match fs::read(path) {
                    Ok(v) => v,
                    Err(_) => return Vec::new(),
                };

                let file = ScannableFile::from_bytes(path.clone(), bytes);
                detectors
                    .iter()
                    .flat_map(|detector| detector.detect(&file, rules).unwrap_or_default())
                    .collect::<Vec<_>>()
            })
            .collect()
    });

    let dependency_sbom = build_dependency_sbom(&config.root, &detections);
    let runtime_profile =
        recommendation::RuntimeProfile::from_repository(&config.root, &files, &dependency_sbom);
    let mut findings = normalize_findings(&config.root, detections, rules);
    recommendation::annotate_findings(&mut findings, &runtime_profile);
    attach_source_snippets(&config.root, &mut findings);
    findings.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then(a.location.file.cmp(&b.location.file))
    });

    let cbom = build_cbom(&findings);

    let mut summary = ScanSummary {
        total_findings: findings.len(),
        by_severity: BTreeMap::new(),
        by_risk: BTreeMap::new(),
        scanned_files: walk_stats.scanned,
        skipped_files: walk_stats.skipped,
    };

    for finding in &findings {
        *summary
            .by_severity
            .entry(finding.severity.as_str().to_string())
            .or_insert(0) += 1;
        *summary
            .by_risk
            .entry(finding.risk.as_str().to_string())
            .or_insert(0) += 1;
    }

    Ok(ScanResult {
        generated_at: Utc::now(),
        findings,
        cbom,
        dependency_sbom,
        summary,
    })
}

fn normalize_findings(
    root: &std::path::Path,
    detections: Vec<Detection>,
    rules: &RuleSet,
) -> Vec<Finding> {
    let mut dedup = HashSet::new();
    let mut findings = Vec::new();

    for detection in detections {
        let rule = match rules.get(&detection.rule_id) {
            Some(v) => v,
            None => continue,
        };

        let dedup_key = format!(
            "{}:{}:{}:{}:{}",
            rule.id,
            detection.location.file,
            detection.location.line,
            detection.location.column,
            detection.evidence.r#match
        );
        if !dedup.insert(dedup_key) {
            continue;
        }

        let severity = risk::adjusted_severity(rule, &detection);
        let finding_id = make_finding_id(
            &rule.id,
            &detection.location.file,
            detection.location.line,
            detection.location.column,
            &detection.evidence.r#match,
        );

        let description = rule
            .description
            .clone()
            .unwrap_or_else(|| format!("Rule {} matched a quantum-relevant crypto usage", rule.id));

        findings.push(Finding {
            finding_id,
            rule_id: rule.id.clone(),
            category: rule.category.clone(),
            risk: rule.risk,
            severity,
            confidence: rule.confidence,
            description,
            migration_hint: rule.migration_hint.clone(),
            location: crate::model::Location {
                file: relativize_path(root, &detection.location.file),
                line: detection.location.line,
                column: detection.location.column,
            },
            evidence: detection.evidence,
            recommended_actions: Vec::new(),
            source_snippet: None,
        });
    }

    findings
}

fn make_finding_id(
    rule_id: &str,
    file: &str,
    line: usize,
    column: usize,
    evidence: &str,
) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(rule_id.as_bytes());
    hasher.update(file.as_bytes());
    hasher.update(line.to_string().as_bytes());
    hasher.update(column.to_string().as_bytes());
    hasher.update(evidence.as_bytes());
    let digest = hasher.finalize();
    hex::encode(&digest[..12])
}

fn relativize_path(root: &std::path::Path, path: &str) -> String {
    let path_obj = std::path::Path::new(path);
    path_obj
        .strip_prefix(root)
        .unwrap_or(path_obj)
        .to_string_lossy()
        .into_owned()
}

fn build_cbom(findings: &[Finding]) -> Vec<CbomEntry> {
    let mut seen = HashSet::new();
    let mut cbom = Vec::new();

    for finding in findings {
        let location = format!("{}:{}", finding.location.file, finding.location.line);
        let key = format!(
            "{}:{}:{}",
            finding.rule_id, location, finding.evidence.r#match
        );
        if !seen.insert(key) {
            continue;
        }

        cbom.push(CbomEntry {
            component: finding
                .location
                .file
                .split('/')
                .next()
                .unwrap_or("repo")
                .to_string(),
            algorithm: finding.evidence.r#match.clone(),
            usage_type: finding.category.clone(),
            location,
            quantum_risk: finding.risk,
            migration_hint: finding.migration_hint.clone(),
        });
    }

    cbom
}

fn build_dependency_sbom(
    root: &std::path::Path,
    detections: &[Detection],
) -> Vec<DependencySbomEntry> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for detection in detections {
        let meta = &detection.evidence.metadata;
        let marker = meta
            .get("inventory_type")
            .map(|x| x.as_str())
            .unwrap_or_default();
        if marker != "dependency" {
            continue;
        }

        let name = meta.get("dep_name").cloned().unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        let version = meta
            .get("dep_version")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let ecosystem = meta
            .get("dep_ecosystem")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let source_type = meta
            .get("dep_source_type")
            .cloned()
            .unwrap_or_else(|| "manifest".to_string());
        let purl = meta.get("dep_purl").cloned().unwrap_or_default();
        let source_file = relativize_path(root, &detection.location.file);

        let dedup_key = format!("{}:{}:{}:{}", ecosystem, name, version, source_file);
        if !seen.insert(dedup_key) {
            continue;
        }

        out.push(DependencySbomEntry {
            name,
            version,
            ecosystem,
            source_file,
            source_type,
            purl,
        });
    }

    out.sort_by(|a, b| {
        a.ecosystem
            .cmp(&b.ecosystem)
            .then(a.name.cmp(&b.name))
            .then(a.version.cmp(&b.version))
            .then(a.source_file.cmp(&b.source_file))
    });
    out
}

pub fn exceeds_fail_threshold(findings: &[Finding], threshold: Severity) -> bool {
    findings.iter().any(|f| f.severity >= threshold)
}

fn attach_source_snippets(root: &std::path::Path, findings: &mut [Finding]) {
    let mut text_cache: HashMap<String, CachedSnippetSource> = HashMap::new();

    for finding in findings {
        if finding.evidence.r#type == "private_key" {
            finding.source_snippet = Some(SourceSnippet {
                lines: vec![SourceSnippetLine {
                    line: finding.location.line,
                    text: "[masked-private-key-material]".to_string(),
                    highlighted: true,
                }],
            });
            continue;
        }

        let rel_file = finding.location.file.clone();
        let source = text_cache
            .entry(rel_file.clone())
            .or_insert_with(|| load_snippet_source(&root.join(&rel_file)));

        finding.source_snippet = match source {
            CachedSnippetSource::MaskedSensitiveFile => {
                finding.evidence.snippet_preview = "[masked-sensitive-file-content]".to_string();
                Some(SourceSnippet {
                    lines: vec![SourceSnippetLine {
                        line: finding.location.line,
                        text: "[masked-sensitive-file-content]".to_string(),
                        highlighted: true,
                    }],
                })
            }
            CachedSnippetSource::TextLines(all_lines) => {
                build_source_snippet(all_lines, finding.location.line, 2, 2)
            }
            CachedSnippetSource::Unavailable => None,
        };
    }
}

enum CachedSnippetSource {
    MaskedSensitiveFile,
    TextLines(Vec<String>),
    Unavailable,
}

fn load_snippet_source(path: &std::path::Path) -> CachedSnippetSource {
    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(_) => return CachedSnippetSource::Unavailable,
    };
    let sample_len = usize::min(bytes.len(), 1024);
    if bytes[..sample_len].contains(&0) {
        return CachedSnippetSource::Unavailable;
    }

    let raw = String::from_utf8_lossy(&bytes).into_owned();
    if contains_private_key_marker(&raw) {
        return CachedSnippetSource::MaskedSensitiveFile;
    }

    let sanitized = mask_private_key_blocks(&raw);
    CachedSnippetSource::TextLines(sanitized.lines().map(sanitize_source_line).collect())
}

fn build_source_snippet(
    lines: &[String],
    target_line: usize,
    before: usize,
    after: usize,
) -> Option<SourceSnippet> {
    if lines.is_empty() || target_line == 0 {
        return None;
    }

    let target_idx = usize::min(target_line - 1, lines.len() - 1);
    let start = target_idx.saturating_sub(before);
    let end = usize::min(target_idx.saturating_add(after), lines.len() - 1);

    let mut snippet_lines = Vec::new();
    for (idx, text) in lines.iter().enumerate().take(end + 1).skip(start) {
        snippet_lines.push(SourceSnippetLine {
            line: idx + 1,
            text: sanitize_source_line(text),
            highlighted: idx == target_idx,
        });
    }

    if snippet_lines.is_empty() {
        None
    } else {
        Some(SourceSnippet {
            lines: snippet_lines,
        })
    }
}

fn sanitize_source_line(line: &str) -> String {
    if looks_like_private_key_line(line) {
        return "[masked-private-key-material]".to_string();
    }
    if looks_like_base64_secret(line) {
        return "[masked-sensitive-content]".to_string();
    }

    let mut out = line.replace('\r', "");
    if out.chars().count() > 240 {
        out = out.chars().take(240).collect::<String>();
        out.push_str("...");
    }
    out
}

fn looks_like_private_key_line(line: &str) -> bool {
    let upper = line.to_ascii_uppercase();
    upper.contains("BEGIN RSA PRIVATE KEY")
        || upper.contains("BEGIN PRIVATE KEY")
        || upper.contains("BEGIN OPENSSH PRIVATE KEY")
        || upper.contains("END RSA PRIVATE KEY")
        || upper.contains("END PRIVATE KEY")
        || upper.contains("END OPENSSH PRIVATE KEY")
}

fn contains_private_key_marker(text: &str) -> bool {
    let upper = text.to_ascii_uppercase();
    upper.contains("BEGIN RSA PRIVATE KEY")
        || upper.contains("BEGIN PRIVATE KEY")
        || upper.contains("BEGIN OPENSSH PRIVATE KEY")
}

fn mask_private_key_blocks(text: &str) -> String {
    let mut out = String::new();
    let mut in_private_key_block = false;

    for line in text.lines() {
        let upper = line.to_ascii_uppercase();
        let is_begin = upper.contains("BEGIN RSA PRIVATE KEY")
            || upper.contains("BEGIN PRIVATE KEY")
            || upper.contains("BEGIN OPENSSH PRIVATE KEY");
        let is_end = upper.contains("END RSA PRIVATE KEY")
            || upper.contains("END PRIVATE KEY")
            || upper.contains("END OPENSSH PRIVATE KEY");

        if is_begin {
            in_private_key_block = true;
        }

        if in_private_key_block {
            out.push_str("[masked-private-key-material]\n");
        } else {
            out.push_str(line);
            out.push('\n');
        }

        if is_end {
            in_private_key_block = false;
        }
    }

    out
}

fn looks_like_base64_secret(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 64 || trimmed.chars().any(char::is_whitespace) {
        return false;
    }

    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=' | '-' | '_'))
    {
        return false;
    }

    trimmed.starts_with("MII") || trimmed.ends_with('=')
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn masks_entire_private_key_block() {
        let text = "before\n-----BEGIN PRIVATE KEY-----\nSUPERSECRETBASE64PAYLOAD\n-----END PRIVATE KEY-----\nafter\n";
        let masked = mask_private_key_blocks(text);

        assert!(!masked.contains("SUPERSECRETBASE64PAYLOAD"));
        assert!(masked.contains("[masked-private-key-material]"));
        assert!(masked.contains("before"));
        assert!(masked.contains("after"));
    }

    #[test]
    fn marks_sensitive_file_for_snippet_masking() {
        let root = temp_dir("pipeline-mask");
        let file = root.join("leak.conf");
        fs::write(
            &file,
            "ssl_protocols TLSv1;\n-----BEGIN PRIVATE KEY-----\nMIIEV...SECRET...\n-----END PRIVATE KEY-----\n",
        )
        .expect("write file");

        match load_snippet_source(&file) {
            CachedSnippetSource::MaskedSensitiveFile => {}
            other => panic!(
                "expected masked sensitive file, got {:?}",
                type_name(&other)
            ),
        }
    }

    fn type_name(source: &CachedSnippetSource) -> &'static str {
        match source {
            CachedSnippetSource::MaskedSensitiveFile => "MaskedSensitiveFile",
            CachedSnippetSource::TextLines(_) => "TextLines",
            CachedSnippetSource::Unavailable => "Unavailable",
        }
    }
}
