use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

use pqc_scan_core::{Finding, ScanResult};

#[derive(Debug, Serialize)]
struct SarifReport {
    version: String,
    #[serde(rename = "$schema")]
    schema: String,
    runs: Vec<SarifRun>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRun {
    column_kind: &'static str,
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Debug, Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Debug, Serialize)]
struct SarifDriver {
    name: String,
    rules: Vec<SarifRule>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRule {
    id: String,
    short_description: SarifMessage,
    full_description: SarifMessage,
    default_configuration: SarifDefaultConfiguration,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifDefaultConfiguration {
    level: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResult {
    rule_id: String,
    level: String,
    message: SarifMessage,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    locations: Vec<SarifLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<SarifResultProperties>,
}

#[derive(Debug, Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifLocation {
    physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifPhysicalLocation {
    artifact_location: SarifArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<SarifRegion>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRegion {
    start_line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_column: Option<usize>,
}

#[derive(Debug, Serialize)]
struct SarifResultProperties {
    migration_hint: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    recommended_actions: Vec<String>,
}

pub fn write_sarif_report(result: &ScanResult, path: &Path) -> Result<()> {
    let rules = collect_rules(&result.findings);
    let sarif = SarifReport {
        version: "2.1.0".to_string(),
        schema: "https://json.schemastore.org/sarif-2.1.0.json".to_string(),
        runs: vec![SarifRun {
            column_kind: "unicodeCodePoints",
            tool: SarifTool {
                driver: SarifDriver {
                    name: "pqc-scan".to_string(),
                    rules,
                },
            },
            results: result.findings.iter().map(to_result).collect(),
        }],
    };

    let json = serde_json::to_string_pretty(&sarif)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn collect_rules(findings: &[Finding]) -> Vec<SarifRule> {
    let mut uniq = BTreeMap::new();
    for finding in findings {
        uniq.entry(finding.rule_id.clone())
            .or_insert_with(|| SarifRule {
                id: finding.rule_id.clone(),
                short_description: SarifMessage {
                    text: finding.category.clone(),
                },
                full_description: SarifMessage {
                    text: finding.description.clone(),
                },
                default_configuration: SarifDefaultConfiguration {
                    level: severity_to_level(&finding.severity.to_string()).to_string(),
                },
            });
    }
    uniq.into_values().collect()
}

fn to_result(finding: &Finding) -> SarifResult {
    let action_titles: Vec<String> = finding
        .recommended_actions
        .iter()
        .take(4)
        .map(|action| format!("{} ({})", action.title, action.priority))
        .collect();
    let next_actions = if action_titles.is_empty() {
        String::new()
    } else {
        format!(" | next_actions={}", action_titles.join("; "))
    };

    SarifResult {
        rule_id: finding.rule_id.clone(),
        level: severity_to_level(&finding.severity.to_string()).to_string(),
        message: SarifMessage {
            text: format!(
                "{} | migration_hint={}{}",
                finding.description, finding.migration_hint, next_actions
            ),
        },
        locations: if finding.location.file.is_empty() {
            Vec::new()
        } else {
            vec![SarifLocation {
                physical_location: SarifPhysicalLocation {
                    artifact_location: SarifArtifactLocation {
                        uri: path_to_uri(&finding.location.file),
                    },
                    region: (finding.location.line > 0).then(|| SarifRegion {
                        start_line: finding.location.line,
                        start_column: sarif_column(finding),
                    }),
                },
            }]
        },
        properties: Some(SarifResultProperties {
            migration_hint: finding.migration_hint.clone(),
            recommended_actions: action_titles,
        }),
    }
}

fn severity_to_level(severity: &str) -> &'static str {
    match severity {
        "critical" | "high" => "error",
        "medium" => "warning",
        _ => "note",
    }
}

// Native AST columns mix a byte-based node start with a code-point match offset.
// Only an unmasked ASCII prefix can prove that the legacy value is also a
// code-point column. Never reread files or guess from a clipped/masked preview.
fn sarif_column(finding: &Finding) -> Option<usize> {
    let column = finding.location.column;
    if column == 0 {
        return None;
    }
    if finding.evidence.r#type == "tree_sitter_match" {
        let line = finding
            .source_snippet
            .as_ref()?
            .lines
            .iter()
            .find(|line| line.line == finding.location.line)?;
        if line.text.contains("[masked-") {
            return None;
        }
        let text = line.text.strip_suffix("...").unwrap_or(&line.text);
        if !text.get(..column - 1).is_some_and(str::is_ascii) {
            return None;
        }
    }
    Some(column)
}

// Locations contain filesystem paths, never already-encoded URIs. Encode UTF-8
// bytes once, including '%' and URI delimiters. Unix backslashes are filenames.
fn path_to_uri(path: &str) -> String {
    #[cfg(windows)]
    let normalized = path.replace('\\', "/");
    #[cfg(windows)]
    let path = normalized.as_str();

    let mut encoded = String::new();
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
            encoded.push(char::from(byte));
        } else {
            use std::fmt::Write;
            write!(encoded, "%{byte:02X}").expect("writing to String cannot fail");
        }
    }
    #[cfg(windows)]
    {
        if path.starts_with("//") {
            return format!("file:{encoded}");
        }
        if path.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
            && path.as_bytes().get(1..3) == Some(b":/")
        {
            return format!("file:///{}:{}", &encoded[..1], &encoded[4..]);
        }
    }
    if path.starts_with('/') {
        format!("file://{encoded}")
    } else {
        encoded
    }
}
