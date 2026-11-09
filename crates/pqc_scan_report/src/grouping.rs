use pqc_scan_core::{Finding, RecommendedAction, ScanResult};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) struct RuleGroup<'a> {
    pub(crate) id: &'a str,
    pub(crate) occurrences: Vec<&'a Finding>,
    pub(crate) shared: Vec<&'a RecommendedAction>,
    pub(crate) shared_keys: BTreeSet<String>,
    pub(crate) rank: u8,
}

pub(crate) fn rank(value: &str) -> u8 {
    match value {
        "critical" => 5,
        "high" => 4,
        "medium" => 3,
        "low" => 2,
        "info" => 1,
        _ => 0,
    }
}

pub(crate) fn key(action: &RecommendedAction) -> String {
    // All action fields participate: a shared ID does not imply shared advice.
    serde_json::to_string(action).expect("action contains only serializable text")
}

pub(crate) fn group_findings(result: &ScanResult) -> Vec<RuleGroup<'_>> {
    let mut buckets: BTreeMap<&str, Vec<&Finding>> = BTreeMap::new();
    for finding in &result.findings {
        buckets.entry(&finding.rule_id).or_default().push(finding);
    }
    let mut groups = Vec::new();
    for (id, mut occurrences) in buckets {
        occurrences.sort_by(|a, b| {
            (
                &a.location.file,
                a.location.line,
                a.location.column,
                &a.finding_id,
            )
                .cmp(&(
                    &b.location.file,
                    b.location.line,
                    b.location.column,
                    &b.finding_id,
                ))
        });
        let mut common: BTreeSet<_> = occurrences[0].recommended_actions.iter().map(key).collect();
        for finding in &occurrences[1..] {
            let keys: BTreeSet<_> = finding.recommended_actions.iter().map(key).collect();
            common.retain(|candidate| keys.contains(candidate));
        }
        if occurrences.len() == 1 {
            common.retain(|candidate| {
                occurrences[0]
                    .recommended_actions
                    .iter()
                    .any(|a| key(a) == *candidate && general(a))
            });
        }
        let mut shared = Vec::new();
        let mut seen = BTreeSet::new();
        for action in &occurrences[0].recommended_actions {
            let k = key(action);
            if common.contains(&k) && seen.insert(k) {
                shared.push(action);
            }
        }
        let rank = occurrences
            .iter()
            .map(|f| rank(f.severity.as_str()))
            .max()
            .unwrap_or(0);
        groups.push(RuleGroup {
            id,
            occurrences,
            shared,
            shared_keys: common,
            rank,
        });
    }
    groups.sort_by(|a, b| b.rank.cmp(&a.rank).then(a.id.cmp(b.id)));
    groups
}

fn general(action: &RecommendedAction) -> bool {
    matches!(
        action.action_id.as_str(),
        "plan.hybrid-migration"
            | "dependency.track-pqc-readiness"
            | "runtime.verify-pqc-capabilities"
            | "pki.hybrid-certificate-roadmap"
            | "cryptoapi.abstraction-layer"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::{Evidence, Location};
    use pqc_scan_rules::{Risk, Severity};
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
    fn groups_retain_all_occurrences_and_sort_by_severity_then_rule() {
        let mut findings = vec![
            finding("Z", "a.rs", 5),
            finding("A", "a.rs", 1),
            finding("Z", "a.rs", 4),
            finding("Z", "a.rs", 3),
            finding("Z", "a.rs", 2),
            finding("Z", "a.rs", 1),
        ];
        findings[0].severity = Severity::Critical;
        let result:ScanResult=serde_json::from_value(serde_json::json!({"generated_at":"2026-11-09T00:00:00Z","findings":findings,"cbom":[],"dependency_sbom":[],"summary":{"total_findings":6,"by_severity":{},"by_risk":{},"scanned_files":1,"skipped_files":0}})).unwrap();
        let groups = group_findings(&result);
        assert_eq!(groups[0].id, "Z");
        assert_eq!(groups[0].occurrences.len(), 5);
        assert_eq!(groups[0].occurrences[0].location.line, 1);
        assert_eq!(groups[1].id, "A");
    }
}
