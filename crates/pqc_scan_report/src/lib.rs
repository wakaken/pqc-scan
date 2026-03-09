mod grouping;
mod html_report;
mod json_report;
mod markdown_report;
mod sarif_report;

use std::path::{Path, PathBuf};

use anyhow::Result;

use pqc_scan_core::ScanResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Json,
    Html,
    Markdown,
    Sarif,
    All,
}

impl ReportFormat {
    pub fn expand(self) -> Vec<ReportFormat> {
        match self {
            ReportFormat::All => vec![
                ReportFormat::Json,
                ReportFormat::Html,
                ReportFormat::Markdown,
                ReportFormat::Sarif,
            ],
            value => vec![value],
        }
    }
}

pub fn write_reports(
    result: &ScanResult,
    out_dir: &Path,
    format: ReportFormat,
) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(out_dir)?;
    let mut sanitized = result.clone();
    sanitize_scan_result(&mut sanitized);

    let mut outputs = Vec::new();
    for selected in format.expand() {
        let path = match selected {
            ReportFormat::Json => {
                let path = out_dir.join("report.json");
                json_report::write_json_report(&sanitized, &path)?;
                path
            }
            ReportFormat::Html => {
                let path = out_dir.join("report.html");
                html_report::write_html_report(&sanitized, &path)?;
                path
            }
            ReportFormat::Markdown => {
                let path = out_dir.join("report.md");
                markdown_report::write_markdown_report(&sanitized, &path)?;
                path
            }
            ReportFormat::Sarif => {
                let path = out_dir.join("report.sarif");
                sarif_report::write_sarif_report(&sanitized, &path)?;
                path
            }
            ReportFormat::All => continue,
        };
        outputs.push(path);
    }

    let cbom_path = out_dir.join("cbom.json");
    json_report::write_cbom(&sanitized, &cbom_path)?;
    outputs.push(cbom_path);

    let dep_sbom_path = out_dir.join("dependency-sbom.json");
    json_report::write_dependency_sbom(&sanitized, &dep_sbom_path)?;
    outputs.push(dep_sbom_path);

    Ok(outputs)
}

fn sanitize_scan_result(result: &mut ScanResult) {
    for finding in &mut result.findings {
        finding.evidence.r#match = sanitize_text_fragment(&finding.evidence.r#match);
        finding.evidence.snippet_preview =
            sanitize_text_fragment(&finding.evidence.snippet_preview);

        if let Some(snippet) = finding.source_snippet.as_mut() {
            for line in &mut snippet.lines {
                line.text = sanitize_text_fragment(&line.text);
            }
        }
    }
}

fn sanitize_text_fragment(input: &str) -> String {
    if looks_like_private_key_text(input) {
        return "[masked-private-key-material]".to_string();
    }
    if looks_like_base64_secret(input) {
        return "[masked-sensitive-content]".to_string();
    }
    input.to_string()
}

fn looks_like_private_key_text(text: &str) -> bool {
    let upper = text.to_ascii_uppercase();
    upper.contains("PRIVATE KEY")
}

fn looks_like_base64_secret(text: &str) -> bool {
    let trimmed = text.trim();
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
    use chrono::Utc;
    use pqc_scan_core::{
        CbomEntry, DependencySbomEntry, Evidence, Finding, Location, ScanResult, ScanSummary,
        SourceSnippet, SourceSnippetLine,
    };
    use pqc_scan_rules::{Risk, Severity};
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const LONG_BASE64_SECRET: &str =
        "MIIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

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

    fn sample_result() -> ScanResult {
        let finding = Finding {
            finding_id: "f1".to_string(),
            rule_id: "K8S_INGRESS_RSA_CIPHERS".to_string(),
            category: "Middleware".to_string(),
            risk: Risk::QuantumVulnerable,
            severity: Severity::High,
            confidence: 0.8,
            description: "test finding".to_string(),
            migration_hint: "remove rsa".to_string(),
            location: Location {
                file: "ingress.yaml".to_string(),
                line: 3,
                column: 5,
            },
            evidence: Evidence {
                r#type: "regex_match".to_string(),
                r#match: "-----BEGIN PRIVATE KEY-----".to_string(),
                snippet_preview: LONG_BASE64_SECRET.to_string(),
                metadata: BTreeMap::new(),
            },
            recommended_actions: Vec::new(),
            source_snippet: Some(SourceSnippet {
                lines: vec![SourceSnippetLine {
                    line: 3,
                    text: LONG_BASE64_SECRET.to_string(),
                    highlighted: true,
                }],
            }),
        };

        ScanResult {
            generated_at: Utc::now(),
            findings: vec![finding],
            cbom: vec![CbomEntry {
                component: "ingress.yaml".to_string(),
                algorithm: "TLS_RSA".to_string(),
                usage_type: "TLS".to_string(),
                location: "ingress.yaml:3".to_string(),
                quantum_risk: Risk::QuantumVulnerable,
                migration_hint: "remove rsa".to_string(),
            }],
            dependency_sbom: vec![DependencySbomEntry {
                name: "openssl".to_string(),
                version: "1.1.1".to_string(),
                ecosystem: "system".to_string(),
                source_file: "Dockerfile".to_string(),
                source_type: "manifest".to_string(),
                purl: "pkg:generic/openssl@1.1.1".to_string(),
            }],
            summary: ScanSummary {
                total_findings: 1,
                by_severity: BTreeMap::from([("high".to_string(), 1)]),
                by_risk: BTreeMap::from([("quantum-vulnerable".to_string(), 1)]),
                scanned_files: 1,
                skipped_files: 0,
            },
        }
    }

    #[test]
    fn write_reports_masks_sensitive_fragments() {
        let out_dir = temp_dir("report-mask");
        let result = sample_result();
        write_reports(&result, &out_dir, ReportFormat::All).expect("write reports");

        let json_report =
            fs::read_to_string(out_dir.join("report.json")).expect("read report.json");
        let html_report =
            fs::read_to_string(out_dir.join("report.html")).expect("read report.html");

        assert!(!json_report.contains(LONG_BASE64_SECRET));
        assert!(!html_report.contains(LONG_BASE64_SECRET));
        assert!(json_report.contains("[masked-private-key-material]"));
    }

    #[test]
    fn write_reports_preserves_json_inventory_contracts() {
        let out_dir = temp_dir("report-json-contract");
        let result = sample_result();
        write_reports(&result, &out_dir, ReportFormat::Json).expect("write json reports");

        let json_report =
            fs::read_to_string(out_dir.join("report.json")).expect("read report.json");
        let cbom = fs::read_to_string(out_dir.join("cbom.json")).expect("read cbom.json");
        let dependency_sbom = fs::read_to_string(out_dir.join("dependency-sbom.json"))
            .expect("read dependency-sbom.json");

        let json_value: Value = serde_json::from_str(&json_report).expect("parse report.json");
        let cbom_value: Value = serde_json::from_str(&cbom).expect("parse cbom.json");
        let dependency_value: Value =
            serde_json::from_str(&dependency_sbom).expect("parse dependency-sbom.json");

        assert_eq!(
            json_value["findings"][0]["evidence"]["match"],
            "[masked-private-key-material]"
        );
        assert_eq!(
            json_value["findings"][0]["evidence"]["snippet_preview"],
            "[masked-sensitive-content]"
        );
        assert_eq!(cbom_value[0]["algorithm"], "TLS_RSA");
        assert_eq!(dependency_value[0]["source_file"], "Dockerfile");
        assert!(dependency_value[0].get("sourceFile").is_none());
    }

    #[test]
    fn sarif_output_uses_camel_case_fields() {
        let out_dir = temp_dir("report-sarif");
        let result = sample_result();
        write_reports(&result, &out_dir, ReportFormat::Sarif).expect("write sarif report");

        let raw = fs::read_to_string(out_dir.join("report.sarif")).expect("read report.sarif");
        let v: Value = serde_json::from_str(&raw).expect("parse sarif");

        let rule = &v["runs"][0]["tool"]["driver"]["rules"][0];
        let result_item = &v["runs"][0]["results"][0];
        let location = &result_item["locations"][0];

        assert!(result_item.get("ruleId").is_some());
        assert!(result_item.get("rule_id").is_none());
        assert!(rule.get("shortDescription").is_some());
        assert!(rule.get("short_description").is_none());
        assert!(location.get("physicalLocation").is_some());
        assert!(location.get("physical_location").is_none());
    }
}
