//! Human report projection. Machine records and stable identities remain untouched.
use std::collections::BTreeMap;

use pqc_scan_core::{Finding, ScanResult};

pub(crate) struct View<'a> {
    pub(crate) result: ScanResult,
    pub(crate) originals: BTreeMap<String, Vec<&'a Finding>>,
    location_counts: BTreeMap<(String, usize, usize), usize>,
}
impl View<'_> {
    pub(crate) fn coincident_records(&self, f: &Finding) -> usize {
        if f.location.line == 0 || f.location.column == 0 {
            return 0;
        }
        self.location_counts
            .get(&(f.location.file.clone(), f.location.line, f.location.column))
            .copied()
            .unwrap_or(0)
    }
}

fn builtin_copy(id: &str) -> Option<(String, String, String)> {
    let jwt_hint = "Confirm whether this reference belongs to active signing, verification, tests or documentation; assess JOSE formats and peer/library support before selecting PQC signatures.";
    let (category, description, hint) = match id {
        "JWT_RS256" | "JWT_RS384" | "JWT_RS512" | "JWT_PS256" | "JWT_PS384" | "JWT_PS512" => ("JWT", format!("{} algorithm name referenced; active signing and effective policy are not established.", &id[4..]), jwt_hint),
        "TS_JWT_RSA_ALGS" => ("JWT", "RSA-family JWT signing algorithms detected in AST nodes.".into(), jwt_hint),
        "API_JOSE_RSA_ALG" => ("CryptoAPI", "RSA-family JOSE algorithm detected.".into(), "Update JOSE algorithms to PQC-ready alternatives."),
        "SSH_RSA_ALGO" => ("SSH", "ssh-rsa algorithm usage detected.".into(), "Plan PQC host authentication separately from hybrid key exchange; ssh-ed25519 remains a classical signature algorithm."),
        "TS_SSH_RSA_STRING" => ("SSH", "SSH RSA-family algorithm strings detected in AST nodes.".into(), "Plan phased retirement of ssh-rsa with hybrid key operations."),
        _ => return None,
    };
    Some((category.into(), description, hint.into()))
}

fn issue(f: &Finding) -> String {
    if f.evidence.metadata.get("reference_kind").is_some_and(|s| s == "display_text")
        && f.description == "An algorithm name appears in non-executable text or a symbol declaration."
        && f.migration_hint == "This text alone does not establish cryptographic use. Inspect the implementation and effective configuration separately."
    { return "display-algorithm-reference".into(); }
    // Only explicitly equivalent built-in rule families cross description/category
    // boundaries. Other rules require identical descriptions and guidance.
    let copy = (&f.category, &f.description, &f.migration_hint);
    if builtin_copy(&f.rule_id)
        .as_ref()
        .map(|c| (&c.0, &c.1, &c.2))
        != Some(copy)
    {
        return serde_json::to_string(&copy).unwrap();
    }
    let family = match f.rule_id.as_str() {
        "JWT_RS256" | "JWT_RS384" | "JWT_RS512" | "JWT_PS256" | "JWT_PS384" | "JWT_PS512"
        | "TS_JWT_RSA_ALGS" | "API_JOSE_RSA_ALG" => "jose-rsa-reference",
        "SSH_RSA_ALGO" | "TS_SSH_RSA_STRING" => "ssh-rsa-reference",
        _ => {
            return serde_json::to_string(&(&f.category, &f.description, &f.migration_hint))
                .unwrap()
        }
    };
    family.into()
}

pub(crate) fn project(result: &ScanResult) -> View<'_> {
    let mut buckets = BTreeMap::new();
    let mut location_counts = BTreeMap::new();
    for f in &result.findings {
        if f.location.line > 0 && f.location.column > 0 {
            *location_counts
                .entry((f.location.file.clone(), f.location.line, f.location.column))
                .or_insert(0) += 1;
        }
        let evidence = &f.evidence.r#match;
        // Missing positions and redacted/empty evidence cannot establish equivalence.
        let isolated = if f.location.line == 0
            || f.location.column == 0
            || evidence.is_empty()
            || evidence.contains("[masked")
            || evidence.contains("***")
        {
            f.finding_id.as_str()
        } else {
            ""
        };
        let k = serde_json::to_string(&(
            &f.location.file,
            f.location.line,
            f.location.column,
            evidence,
            issue(f),
            f.risk.as_str(),
            isolated,
        ))
        .unwrap();
        buckets.entry(k).or_insert_with(Vec::new).push(f);
    }
    let mut view = View {
        result: result.clone(),
        originals: BTreeMap::new(),
        location_counts,
    };
    view.result.findings.clear();
    for mut originals in buckets.into_values() {
        originals.sort_by(|a, b| {
            crate::grouping::rank(b.severity.as_str())
                .cmp(&crate::grouping::rank(a.severity.as_str()))
                .then(a.rule_id.cmp(&b.rule_id))
                .then(a.finding_id.cmp(&b.finding_id))
        });
        let mut representative = originals[0].clone();
        for f in &originals[1..] {
            for a in &f.recommended_actions {
                if !representative
                    .recommended_actions
                    .iter()
                    .any(|b| crate::grouping::key(a) == crate::grouping::key(b))
                {
                    representative.recommended_actions.push(a.clone());
                }
            }
        }
        view.originals
            .insert(representative.finding_id.clone(), originals);
        view.result.findings.push(representative);
    }
    view.result.summary.total_findings = view.result.findings.len();
    view.result.summary.by_severity.clear();
    view.result.summary.by_risk.clear();
    for f in &view.result.findings {
        *view
            .result
            .summary
            .by_severity
            .entry(f.severity.as_str().into())
            .or_default() += 1;
        *view
            .result
            .summary
            .by_risk
            .entry(f.risk.as_str().into())
            .or_default() += 1;
    }
    view
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> ScanResult {
        let findings: Vec<_> = ["JWT_RS512", "TS_JWT_RSA_ALGS", "API_JOSE_RSA_ALG"]
            .into_iter().map(|id| serde_json::json!({
                "finding_id": id, "rule_id": id, "category": "JWT",
                "risk": "quantum-vulnerable", "severity": if id == "API_JOSE_RSA_ALG" {"high"} else {"critical"},
                "confidence": 0.7, "description": id, "migration_hint": id,
                "location": {"file": "tokens.py", "line": 96, "column": 17},
                "evidence": {"type": "regex_match", "match": "RS512", "snippet_preview": "", "metadata": {}},
                "recommended_actions": [], "source_snippet": null
            })).collect();
        let mut result: ScanResult = serde_json::from_value(serde_json::json!({"generated_at":"2026-11-21T00:00:00Z", "findings":findings,"cbom":[],"dependency_sbom":[],"summary":{"total_findings":3,"by_severity":{"critical":2,"high":1},"by_risk":{"quantum-vulnerable":3},"scanned_files":12,"skipped_files":0}})).unwrap();
        for f in &mut result.findings {
            let (category, description, hint) = builtin_copy(&f.rule_id).unwrap();
            f.category = category;
            f.description = description;
            f.migration_hint = hint;
        }
        result
    }
    #[test]
    fn equivalent_display_references_keep_all_matching_rules_once() {
        let mut raw = fixture();
        for f in &mut raw.findings {
            f.severity = pqc_scan_rules::Severity::Info;
            f.description =
                "An algorithm name appears in non-executable text or a symbol declaration.".into();
            f.migration_hint = "This text alone does not establish cryptographic use. Inspect the implementation and effective configuration separately.".into();
            f.evidence
                .metadata
                .insert("reference_kind".into(), "display_text".into());
        }
        let view = project(&raw);
        assert_eq!(view.result.findings.len(), 1);
        assert_eq!(view.originals.values().next().unwrap().len(), 3);
        assert_eq!(raw.findings.len(), 3);
    }
    #[test]
    fn equivalent_rules_collapse_without_mutating_machine_records() {
        let raw = fixture();
        let before = serde_json::to_value(&raw).unwrap();
        let view = project(&raw);
        assert_eq!(view.result.findings.len(), 1);
        assert_eq!(view.originals.values().next().unwrap().len(), 3);
        assert_eq!(view.result.findings[0].severity.as_str(), "critical");
        assert_eq!(view.result.summary.by_severity.get("critical"), Some(&1));
        assert_eq!(view.result.summary.scanned_files, 12);
        assert_eq!(serde_json::to_value(&raw).unwrap(), before);
        let mut reversed = raw.clone();
        reversed.findings.reverse();
        assert_eq!(
            serde_json::to_value(project(&reversed).result).unwrap(),
            serde_json::to_value(&view.result).unwrap()
        );
    }
    #[test]
    fn different_algorithm_issue_position_risk_and_unavailable_evidence_stay_separate() {
        for case in 0..9 {
            let mut raw = fixture();
            raw.findings.truncate(2);
            match case {
                0 => raw.findings[1].evidence.r#match = "RS256".into(),
                1 => raw.findings[1].rule_id = "CERT_RSA_SIGNATURE".into(),
                2 => raw.findings[1].location.column += 1,
                3 => raw.findings[1].location.file = "other.py".into(),
                4 => {
                    for f in &mut raw.findings {
                        f.evidence.r#match = "[masked-sensitive-content]".into();
                    }
                }
                5 => {
                    for f in &mut raw.findings {
                        f.location.line = 0;
                    }
                }
                6 => raw.findings[1].risk = pqc_scan_rules::Risk::NonQuantumRisk,
                7 => {
                    for f in &mut raw.findings {
                        f.evidence.r#match = "abcdefgh***uvwxyz".into();
                    }
                }
                _ => raw.findings[1].description = "Different custom issue".into(),
            }
            assert_eq!(project(&raw).result.findings.len(), 2, "case {case}");
        }
    }
    #[test]
    fn merged_actions_and_rendered_rule_details_remain_available() {
        let mut raw = fixture();
        raw.findings[2].recommended_actions = vec![serde_json::from_value(serde_json::json!({"action_id":"extra","title":"Extra check","priority":"planned","rationale":"reason","steps":["check"],"references":[]})).unwrap()];
        assert_eq!(
            project(&raw).result.findings[0].recommended_actions.len(),
            1
        );
        let dir = std::env::temp_dir().join(format!("pqc-dedup-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        crate::compact::html_report(&raw, &dir.join("report.html"), crate::ReportLanguage::Ja)
            .unwrap();
        crate::compact::markdown_report(&raw, &dir.join("report.md"), crate::ReportLanguage::Ja)
            .unwrap();
        let html = std::fs::read_to_string(dir.join("report.html")).unwrap();
        let md = std::fs::read_to_string(dir.join("report.md")).unwrap();
        // One occurrence per language, with every supporting rule in its details.
        assert_eq!(html.matches("class=\"occurrence\"").count(), 2);
        for id in ["JWT_RS512", "TS_JWT_RSA_ALGS", "API_JOSE_RSA_ALG"] {
            assert!(html.contains(id));
            assert!(md.contains(id));
        }
        assert!(html.contains("一致したルール (3)"));
        assert!(html.contains("確認が必要な指摘"));
        assert!(html.contains("<small class=\"metric-unit\">件</small>"));
        assert!(html.contains("Extra check"));
        let guide = std::fs::read_to_string(dir.join("report.rules.html")).unwrap();
        assert!(!guide.contains("report.html?lang=ja#rule-1"));
        assert!(!guide.contains("report.html?lang=en#rule-2"));
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn sidebar_test_paths_and_embedded_rule_help_preserve_offline_fallback() {
        let mut raw = fixture();
        raw.findings[0].location.file = "src/test/java/KeyTest.java".into();
        raw.findings[1].location.file = "src/main/Key.java".into();
        raw.findings[2].location.file = "testsuite/keys/example.pem".into();
        let dir = std::env::temp_dir().join(format!("pqc-sidebar-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        crate::compact::html_report(&raw, &dir.join("report.html"), crate::ReportLanguage::Ja)
            .unwrap();
        let html = std::fs::read_to_string(dir.join("report.html")).unwrap();
        assert_eq!(html.matches("data-test=\"true\"").count(), 4);
        assert_eq!(html.matches("data-test=\"false\"").count(), 2);
        assert!(html.contains("data-show-tests checked"));
        assert!(html.contains("data-rule-help=\"0\""));
        assert!(html.contains("<template data-rule-explanations>"));
        assert!(html.contains("<dialog class=\"rule-dialog\""));
        assert!(html.contains("report.rules.html?lang=ja#rule-0"));
        assert!(!html.contains("data-panel-toggles"));
        assert!(!html.contains("検出のあるファイル · 件数は全検出"));
        assert!(html.contains("data-tree-clear>すべての結果を表示</a>"));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
