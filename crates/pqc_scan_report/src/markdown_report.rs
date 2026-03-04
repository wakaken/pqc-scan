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
