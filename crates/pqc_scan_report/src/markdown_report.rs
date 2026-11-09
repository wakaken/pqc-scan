use anyhow::Result;
use pqc_scan_core::ScanResult;
use std::path::Path;

pub fn write_markdown_report(result: &ScanResult, path: &Path) -> Result<()> {
    crate::compact::markdown_report(result, path, crate::ReportLanguage::En)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use pqc_scan_core::{
        CbomEntry, CodeExample, Evidence, Finding, Location, RecommendedAction, ScanResult,
        ScanSummary,
    };
    use pqc_scan_rules::{Risk, Severity};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(prefix: &str, extension: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        path.push(format!(
            "pqc-scan-{prefix}-{}-{nanos}.{extension}",
            std::process::id()
        ));
        path
    }

    fn sample_result() -> ScanResult {
        let action = RecommendedAction {
            action_id: "tls-doc-1".to_string(),
            title: "Migrate to ML-KEM".to_string(),
            priority: "p1".to_string(),
            rationale: "Remove RSA fallback before rollout".to_string(),
            steps: vec![
                "Update ingress policy".to_string(),
                "Re-issue certificates".to_string(),
            ],
            references: vec!["https://example.com/spec".to_string()],
            code_examples: vec![CodeExample {
                language: "yaml".to_string(),
                before: "cipher: RSA|PKCS1".to_string(),
                after: "cipher: ML-KEM".to_string(),
            }],
        };

        let finding = |id: &str, file: &str, line: usize, evidence: &str| Finding {
            finding_id: id.to_string(),
            rule_id: "TLS_RSA_DEPRECATED".to_string(),
            category: "TLS".to_string(),
            risk: Risk::QuantumVulnerable,
            severity: Severity::High,
            confidence: 0.84,
            description: "Legacy TLS rule".to_string(),
            migration_hint: "Use PQC-safe transport".to_string(),
            location: Location {
                file: file.to_string(),
                line,
                column: 2,
            },
            evidence: Evidence {
                r#type: "regex_match".to_string(),
                r#match: evidence.to_string(),
                snippet_preview: evidence.to_string(),
                metadata: BTreeMap::new(),
            },
            recommended_actions: vec![action.clone()],
            source_snippet: None,
        };

        ScanResult {
            generated_at: Utc
                .with_ymd_and_hms(2025, 2, 3, 4, 5, 6)
                .single()
                .expect("valid timestamp"),
            findings: vec![
                finding(
                    "f1",
                    "service|prod.yaml",
                    8,
                    "[masked-sensitive-content]|wrapped\nline",
                ),
                finding(
                    "f2",
                    "service|prod.yaml",
                    9,
                    "[masked-private-key-material]",
                ),
            ],
            cbom: vec![CbomEntry {
                component: "service|prod.yaml".to_string(),
                algorithm: "TLS_RSA".to_string(),
                usage_type: "TLS".to_string(),
                location: "service|prod.yaml:8".to_string(),
                quantum_risk: Risk::QuantumVulnerable,
                migration_hint: "Use PQC-safe transport".to_string(),
            }],
            dependency_sbom: Vec::new(),
            summary: ScanSummary {
                total_findings: 2,
                by_severity: BTreeMap::from([("high".to_string(), 2)]),
                by_risk: BTreeMap::from([("quantum-vulnerable".to_string(), 2)]),
                scanned_files: 1,
                skipped_files: 0,
            },
        }
    }

    #[test]
    fn markdown_report_preserves_grouping_and_escapes_table_cells() {
        let path = temp_file("markdown-report-contract", "md");
        let result = sample_result();

        write_markdown_report(&result, &path).expect("write markdown report");

        let rendered = fs::read_to_string(&path).expect("read markdown report");

        assert!(rendered.contains("[TLS_RSA_DEPRECATED](#rule-0)"));
        assert!(rendered.contains("## TLS_RSA_DEPRECATED"));
        assert!(rendered.contains("service\\|prod.yaml:8:2"));
        assert!(rendered.contains("service\\|prod.yaml:9:2"));
        assert!(rendered.contains("\\[masked-sensitive-content\\]\\|wrapped line"));
        assert!(rendered.contains("Shared checks for these locations"));
        assert!(rendered.contains("Migrate to ML-KEM"));
        assert!(rendered.contains("Remove RSA fallback before rollout"));
        assert!(rendered.contains("https://example.com/spec"));
        assert!(rendered.contains("Before (yaml)"));
        assert!(rendered.contains("cipher: RSA|PKCS1"));
        assert!(rendered.contains("cipher: ML-KEM"));

        let _ = fs::remove_file(path);
    }
}
