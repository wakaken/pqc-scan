use std::collections::{BTreeMap, BTreeSet};

use pqc_scan_core::{Finding, RecommendedAction};

pub(crate) struct FileOccurrence<'a> {
    pub file: &'a str,
    pub hits: usize,
    pub lines: Vec<usize>,
    pub sample_evidence: Vec<&'a str>,
    pub sample_findings: Vec<&'a Finding>,
}

pub(crate) struct GroupedFinding<'a> {
    pub rule_id: &'a str,
    pub category: &'a str,
    pub risk: String,
    pub severity: String,
    pub description: &'a str,
    pub migration_hint: &'a str,
    pub total_hits: usize,
    pub files: Vec<FileOccurrence<'a>>,
    pub sample_evidence: Vec<&'a str>,
    pub recommended_actions: Vec<&'a RecommendedAction>,
}

pub(crate) fn group_findings(findings: &[Finding]) -> Vec<GroupedFinding<'_>> {
    let mut buckets: BTreeMap<String, Vec<&Finding>> = BTreeMap::new();
    for finding in findings {
        buckets
            .entry(finding.rule_id.clone())
            .or_default()
            .push(finding);
    }

    let mut groups = Vec::new();
    for (_rule_id, mut bucket) in buckets {
        if bucket.is_empty() {
            continue;
        }
        bucket.sort_by(|a, b| {
            a.location
                .file
                .cmp(&b.location.file)
                .then(a.location.line.cmp(&b.location.line))
                .then(a.location.column.cmp(&b.location.column))
        });

        let representative = bucket[0];
        let severity = bucket
            .iter()
            .fold(representative.severity.to_string(), |acc, f| {
                if severity_rank(&f.severity.to_string()) > severity_rank(&acc) {
                    f.severity.to_string()
                } else {
                    acc
                }
            });

        let mut by_file: BTreeMap<&str, Vec<&Finding>> = BTreeMap::new();
        for finding in &bucket {
            by_file
                .entry(finding.location.file.as_str())
                .or_default()
                .push(*finding);
        }

        let mut file_occurrences = Vec::new();
        for (file, mut file_findings) in by_file {
            file_findings.sort_by(|a, b| {
                a.location
                    .line
                    .cmp(&b.location.line)
                    .then(a.location.column.cmp(&b.location.column))
            });

            let hits = file_findings.len();
            let mut lines = BTreeSet::new();
            let mut sample_evidence = Vec::new();
            let mut sample_findings = Vec::new();
            for finding in &file_findings {
                if finding.location.line > 0 {
                    lines.insert(finding.location.line);
                }
                let evidence = finding.evidence.r#match.as_str();
                if !evidence.is_empty() && !sample_evidence.contains(&evidence) {
                    sample_evidence.push(evidence);
                }
                if sample_evidence.len() >= 3 {
                    // keep looping to find snippet samples
                }
                if finding.source_snippet.is_some() && sample_findings.len() < 3 {
                    sample_findings.push(*finding);
                }
            }

            file_occurrences.push(FileOccurrence {
                file,
                hits,
                lines: lines.into_iter().collect(),
                sample_evidence,
                sample_findings,
            });
        }

        let mut action_ids = BTreeSet::new();
        let mut actions = Vec::new();
        for finding in &bucket {
            for action in &finding.recommended_actions {
                if action_ids.insert(action.action_id.clone()) {
                    actions.push(action);
                }
            }
        }

        let mut sample_evidence = Vec::new();
        for finding in &bucket {
            let evidence = finding.evidence.r#match.as_str();
            if !evidence.is_empty() && !sample_evidence.contains(&evidence) {
                sample_evidence.push(evidence);
            }
            if sample_evidence.len() >= 3 {
                break;
            }
        }

        groups.push(GroupedFinding {
            rule_id: &representative.rule_id,
            category: &representative.category,
            risk: representative.risk.to_string(),
            severity,
            description: &representative.description,
            migration_hint: &representative.migration_hint,
            total_hits: bucket.len(),
            files: file_occurrences,
            sample_evidence,
            recommended_actions: actions,
        });
    }

    groups.sort_by(|a, b| {
        severity_rank(&b.severity)
            .cmp(&severity_rank(&a.severity))
            .then(b.total_hits.cmp(&a.total_hits))
            .then(a.rule_id.cmp(b.rule_id))
    });

    groups
}

pub(crate) fn summarize_lines(lines: &[usize], max_lines: usize) -> String {
    if lines.is_empty() {
        return "unknown".to_string();
    }

    let mut shown = lines
        .iter()
        .take(max_lines)
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    if lines.len() > max_lines {
        shown.push(format!("... +{} more", lines.len() - max_lines));
    }
    shown.join(", ")
}

fn severity_rank(severity: &str) -> u8 {
    match severity {
        "critical" => 5,
        "high" => 4,
        "medium" => 3,
        "low" => 2,
        "info" => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::{Evidence, Finding, Location};
    use pqc_scan_rules::{Risk, Severity};
    use std::collections::BTreeMap;

    fn finding(rule: &str, file: &str, line: usize) -> Finding {
        Finding {
            finding_id: format!("{rule}-{file}-{line}"),
            rule_id: rule.to_string(),
            category: "JWT".to_string(),
            risk: Risk::QuantumVulnerable,
            severity: Severity::High,
            confidence: 0.8,
            description: "desc".to_string(),
            migration_hint: "hint".to_string(),
            location: Location {
                file: file.to_string(),
                line,
                column: 1,
            },
            evidence: Evidence {
                r#type: "regex_match".to_string(),
                r#match: "RS256".to_string(),
                snippet_preview: "RS256".to_string(),
                metadata: BTreeMap::new(),
            },
            recommended_actions: Vec::new(),
            source_snippet: None,
        }
    }

    #[test]
    fn groups_by_rule_id_and_splits_files() {
        let findings = vec![
            finding("TS_JWT_RSA_ALGS", "a.ts", 10),
            finding("TS_JWT_RSA_ALGS", "a.ts", 12),
            finding("TS_JWT_RSA_ALGS", "b.ts", 20),
            finding("JWT_RS256", "c.ts", 30),
        ];
        let groups = group_findings(&findings);

        assert_eq!(groups.len(), 2);
        let ts_group = groups
            .iter()
            .find(|g| g.rule_id == "TS_JWT_RSA_ALGS")
            .expect("ts group");
        assert_eq!(ts_group.total_hits, 3);
        assert_eq!(ts_group.files.len(), 2);
    }
}
