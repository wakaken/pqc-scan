use std::path::Path;

use anyhow::{Context, Result};

use pqc_scan_core::ScanResult;

pub fn write_json_report(result: &ScanResult, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn write_cbom(result: &ScanResult, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(&result.cbom)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn write_dependency_sbom(result: &ScanResult, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(&result.dependency_sbom)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use pqc_scan_core::{
        CbomEntry, DependencySbomEntry, Evidence, Finding, Location, RecommendedAction, ScanResult,
        ScanSummary, SourceSnippet, SourceSnippetLine,
    };
    use pqc_scan_rules::{Risk, Severity};
    use serde_json::Value;
    use std::collections::BTreeMap;
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

    fn sample_result() -> ScanResult {
        ScanResult {
            generated_at: Utc::now(),
            findings: vec![Finding {
                finding_id: "finding-1".to_string(),
                rule_id: "RULE_001".to_string(),
                category: "TLS".to_string(),
                risk: Risk::QuantumVulnerable,
                severity: Severity::High,
                confidence: 0.95,
                description: "RSA key exchange detected".to_string(),
                migration_hint: "Prefer hybrid or PQC-ready TLS".to_string(),
                location: Location {
                    file: "nginx.conf".to_string(),
                    line: 8,
                    column: 12,
                },
                evidence: Evidence {
                    r#type: "regex_match".to_string(),
                    r#match: "TLS_RSA_WITH_AES_128_CBC_SHA".to_string(),
                    snippet_preview: "ssl_ciphers TLS_RSA_WITH_AES_128_CBC_SHA".to_string(),
                    metadata: BTreeMap::from([(
                        "detector".to_string(),
                        "regex_detector".to_string(),
                    )]),
                },
                recommended_actions: vec![RecommendedAction {
                    action_id: "tls-1".to_string(),
                    title: "Update TLS configuration".to_string(),
                    priority: "high".to_string(),
                    rationale: "Legacy RSA key exchange is quantum-vulnerable.".to_string(),
                    steps: vec![
                        "Inventory exposed endpoints".to_string(),
                        "Disable RSA key exchange suites".to_string(),
                    ],
                    references: vec!["https://example.invalid/pqc".to_string()],
                    code_examples: Vec::new(),
                }],
                source_snippet: Some(SourceSnippet {
                    lines: vec![SourceSnippetLine {
                        line: 8,
                        text: "ssl_ciphers TLS_RSA_WITH_AES_128_CBC_SHA;".to_string(),
                        highlighted: true,
                    }],
                }),
            }],
            cbom: vec![CbomEntry {
                component: "nginx.conf".to_string(),
                algorithm: "TLS_RSA_WITH_AES_128_CBC_SHA".to_string(),
                usage_type: "TLS".to_string(),
                location: "nginx.conf:8".to_string(),
                quantum_risk: Risk::QuantumVulnerable,
                migration_hint: "Prefer hybrid or PQC-ready TLS".to_string(),
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
                scanned_files: 4,
                skipped_files: 1,
            },
        }
    }

    #[test]
    fn write_json_report_serializes_expected_sections_and_fields() {
        let out_dir = temp_dir("json-report");
        let path = out_dir.join("report.json");

        write_json_report(&sample_result(), &path).expect("write report.json");

        let raw = fs::read_to_string(path).expect("read report.json");
        let value: Value = serde_json::from_str(&raw).expect("parse report.json");

        assert!(value.get("generated_at").is_some());
        assert!(value.get("dependency_sbom").is_some());
        assert!(value.get("dependencySbom").is_none());
        assert_eq!(value["summary"]["total_findings"], 1);

        let finding = &value["findings"][0];
        assert_eq!(finding["rule_id"], "RULE_001");
        assert_eq!(finding["severity"], "high");
        assert_eq!(finding["risk"], "quantum-vulnerable");
        assert_eq!(finding["recommended_actions"][0]["action_id"], "tls-1");
        assert_eq!(finding["source_snippet"]["lines"][0]["line"], 8);
        assert_eq!(finding["source_snippet"]["lines"][0]["highlighted"], true);
    }

    #[test]
    fn write_cbom_serializes_expected_fields() {
        let out_dir = temp_dir("cbom-report");
        let path = out_dir.join("cbom.json");

        write_cbom(&sample_result(), &path).expect("write cbom.json");

        let raw = fs::read_to_string(path).expect("read cbom.json");
        let value: Value = serde_json::from_str(&raw).expect("parse cbom.json");

        assert_eq!(value.as_array().map(Vec::len), Some(1));
        assert_eq!(value[0]["component"], "nginx.conf");
        assert_eq!(value[0]["usage_type"], "TLS");
        assert_eq!(value[0]["quantum_risk"], "quantum-vulnerable");
        assert!(value[0].get("usageType").is_none());
        assert!(value[0].get("quantumRisk").is_none());
    }

    #[test]
    fn write_dependency_sbom_serializes_expected_fields() {
        let out_dir = temp_dir("dependency-sbom");
        let path = out_dir.join("dependency-sbom.json");

        write_dependency_sbom(&sample_result(), &path).expect("write dependency-sbom.json");

        let raw = fs::read_to_string(path).expect("read dependency-sbom.json");
        let value: Value = serde_json::from_str(&raw).expect("parse dependency-sbom.json");

        assert_eq!(value.as_array().map(Vec::len), Some(1));
        assert_eq!(value[0]["name"], "openssl");
        assert_eq!(value[0]["source_file"], "Dockerfile");
        assert_eq!(value[0]["source_type"], "manifest");
        assert_eq!(value[0]["purl"], "pkg:generic/openssl@1.1.1");
        assert!(value[0].get("sourceFile").is_none());
        assert!(value[0].get("sourceType").is_none());
    }
}
