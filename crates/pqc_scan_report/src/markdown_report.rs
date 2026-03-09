use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};

use crate::grouping::{group_findings, summarize_lines};
use pqc_scan_core::ScanResult;

pub fn write_markdown_report(result: &ScanResult, path: &Path) -> Result<()> {
    let mut out = String::new();
    let groups = group_findings(&result.findings);

    writeln!(&mut out, "# PQC Migration Scan Report")?;
    writeln!(&mut out)?;
    writeln!(&mut out, "- Generated: {}", result.generated_at)?;
    writeln!(
        &mut out,
        "- Total findings: {}",
        result.summary.total_findings
    )?;
    writeln!(&mut out, "- Grouped findings: {}", groups.len())?;
    writeln!(
        &mut out,
        "- Scanned files: {}",
        result.summary.scanned_files
    )?;
    writeln!(
        &mut out,
        "- Skipped files: {}",
        result.summary.skipped_files
    )?;
    writeln!(
        &mut out,
        "- Dependency SBOM entries: {}",
        result.dependency_sbom.len()
    )?;
    writeln!(&mut out)?;

    writeln!(&mut out, "## Severity Summary")?;
    writeln!(&mut out)?;
    writeln!(&mut out, "| Severity | Count |")?;
    writeln!(&mut out, "|---|---:|")?;
    for (severity, count) in &result.summary.by_severity {
        writeln!(&mut out, "| {} | {} |", severity, count)?;
    }
    writeln!(&mut out)?;

    writeln!(&mut out, "## Findings (Grouped by Rule)")?;
    writeln!(&mut out)?;
    for group in &groups {
        if group.files.is_empty() {
            continue;
        }
        writeln!(
            &mut out,
            "### {} ({}) - {} hits",
            group.rule_id, group.severity, group.total_hits
        )?;
        writeln!(&mut out, "- Category: {}", group.category)?;
        writeln!(&mut out, "- Risk: {}", group.risk)?;
        writeln!(&mut out, "- Affected files: {}", group.files.len())?;
        writeln!(&mut out, "- Migration hint: {}", group.migration_hint)?;
        if let Some(sample) = group.sample_evidence.first() {
            writeln!(&mut out, "- Sample evidence: `{}`", sample)?;
        }

        writeln!(&mut out)?;
        writeln!(&mut out, "| File | Hits | Lines | Sample Evidence |")?;
        writeln!(&mut out, "|---|---:|---|---|")?;
        for file in &group.files {
            let sample = file.sample_evidence.first().copied().unwrap_or("-");
            writeln!(
                &mut out,
                "| `{}` | {} | {} | `{}` |",
                escape_md_table_cell(file.file),
                file.hits,
                summarize_lines(&file.lines, 20),
                escape_md_table_cell(sample)
            )?;
        }

        if !group.recommended_actions.is_empty() {
            writeln!(&mut out)?;
            writeln!(&mut out, "- Recommended actions:")?;
            for action in &group.recommended_actions {
                writeln!(
                    &mut out,
                    "  - [{}] {} ({})",
                    action.priority, action.title, action.action_id
                )?;
                writeln!(&mut out, "    - Why: {}", action.rationale)?;
                for step in &action.steps {
                    writeln!(&mut out, "    - Step: {}", step)?;
                }
                for reference in &action.references {
                    writeln!(&mut out, "    - Ref: {}", reference)?;
                }
                for example in &action.code_examples {
                    writeln!(&mut out, "    - Example ({}) before:", example.language)?;
                    writeln!(&mut out, "```{}", example.language)?;
                    writeln!(&mut out, "{}", example.before)?;
                    writeln!(&mut out, "```")?;
                    writeln!(&mut out, "    - Example ({}) after:", example.language)?;
                    writeln!(&mut out, "```{}", example.language)?;
                    writeln!(&mut out, "{}", example.after)?;
                    writeln!(&mut out, "```")?;
                }
            }
        }
        writeln!(&mut out)?;
    }

    writeln!(&mut out, "## CBOM")?;
    writeln!(&mut out)?;
    writeln!(
        &mut out,
        "| Component | Algorithm | Usage | Location | Quantum Risk | Migration Hint |"
    )?;
    writeln!(&mut out, "|---|---|---|---|---|---|")?;
    for item in &result.cbom {
        writeln!(
            &mut out,
            "| {} | {} | {} | {} | {} | {} |",
            item.component,
            item.algorithm,
            item.usage_type,
            item.location,
            item.quantum_risk,
            item.migration_hint
        )?;
    }

    writeln!(&mut out)?;
    writeln!(&mut out, "## Dependency SBOM")?;
    writeln!(&mut out)?;
    writeln!(
        &mut out,
        "| Ecosystem | Name | Version | Source Type | Source File | PURL |"
    )?;
    writeln!(&mut out, "|---|---|---|---|---|---|")?;
    for dep in &result.dependency_sbom {
        writeln!(
            &mut out,
            "| {} | {} | {} | {} | {} | {} |",
            dep.ecosystem, dep.name, dep.version, dep.source_type, dep.source_file, dep.purl
        )?;
    }

    std::fs::write(path, out).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn escape_md_table_cell(input: &str) -> String {
    input.replace('|', "\\|").replace('\n', " ")
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

        assert!(rendered.contains("### TLS_RSA_DEPRECATED (high) - 2 hits"));
        assert!(rendered.contains("- Affected files: 1"));
        assert!(rendered.contains(
            "| `service\\|prod.yaml` | 2 | 8, 9 | `[masked-sensitive-content]\\|wrapped line` |"
        ));
        assert!(rendered.contains("`[masked-sensitive-content]|wrapped"));
        assert!(rendered.contains("- Recommended actions:"));
        assert!(rendered.contains("  - [p1] Migrate to ML-KEM (tls-doc-1)"));
        assert!(rendered.contains("    - Why: Remove RSA fallback before rollout"));
        assert!(rendered.contains("    - Ref: https://example.com/spec"));
        assert!(rendered.contains("```yaml"));
        assert!(rendered.contains("cipher: RSA|PKCS1"));
        assert!(rendered.contains("cipher: ML-KEM"));

        let _ = fs::remove_file(path);
    }
}
