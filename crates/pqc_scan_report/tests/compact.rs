use chrono::Utc;
use pqc_scan_core::{Evidence, Finding, Location, RecommendedAction, ScanResult, ScanSummary};
use pqc_scan_report::{write_reports_with_language, ReportFormat, ReportLanguage};
use pqc_scan_rules::{Risk, Severity};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn result(count: usize) -> ScanResult {
    let findings: Vec<_> = (0..count)
        .map(|i| Finding {
            finding_id: format!("occurrence-{i}"),
            rule_id: "JWT_RS256".into(),
            category: "JWT".into(),
            risk: Risk::QuantumVulnerable,
            severity: Severity::High,
            confidence: 0.8,
            description: "RSA JWT signature".into(),
            migration_hint: "Verify issuer and verifier compatibility".into(),
            location: Location {
                file: if i % 2 == 0 { "app.java" } else { "app.py" }.into(),
                line: i,
                column: 1,
            },
            evidence: Evidence {
                r#type: "regex_match".into(),
                r#match: "RS256".into(),
                snippet_preview: "RS256".into(),
                metadata: Default::default(),
            },
            source_snippet: None,
            recommended_actions: vec![
                RecommendedAction {
                    action_id: "shared".into(),
                    title: "Shared planning".into(),
                    priority: "planned".into(),
                    rationale: "shared-rationale".into(),
                    steps: vec!["shared-step".into()],
                    references: vec![],
                    code_examples: vec![],
                },
                RecommendedAction {
                    action_id: "same-id-different-payload".into(),
                    title: if i % 2 == 0 {
                        "Java only"
                    } else {
                        "Python only"
                    }
                    .into(),
                    priority: "near-term".into(),
                    rationale: "local".into(),
                    steps: vec![format!("local-step-{i}")],
                    references: vec![],
                    code_examples: vec![],
                },
            ],
        })
        .collect();
    ScanResult {
        generated_at: Utc::now(),
        findings,
        cbom: vec![],
        dependency_sbom: vec![],
        summary: ScanSummary {
            total_findings: count,
            by_severity: [("high".into(), count)].into(),
            by_risk: [("quantum-vulnerable".into(), count)].into(),
            scanned_files: 2,
            skipped_files: 0,
        },
    }
}

#[test]
fn compact_reports_keep_all_locations_and_only_share_identical_actions() {
    let root = std::env::temp_dir().join(format!(
        "pqc-compact-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    for (name, lang) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        write_reports_with_language(&result(25), &root.join(name), ReportFormat::All, lang)
            .unwrap();
        let html = fs::read_to_string(root.join(name).join("report.html")).unwrap();
        assert!(html.contains("data-report-controls"));
        assert!(html.contains("data-occurrence-list"));
        assert!(html.contains("data-page-size=\"20\""));
        assert_eq!(html.matches("data-finding-id=").count(), 25);
        assert_eq!(html.matches("shared-rationale").count(), 1);
        for i in 0..25 {
            assert!(html.contains(&format!("local-step-{i}")));
        }
        let md = fs::read_to_string(root.join(name).join("report.md")).unwrap();
        assert!(md.contains("#finding-0"));
        assert_eq!(md.matches("shared-rationale").count(), 1);
        for i in 0..25 {
            assert!(md.contains(&format!("local-step-{i}")));
        }
    }
    let many = result(1000);
    write_reports_with_language(
        &many,
        &root.join("many"),
        ReportFormat::Html,
        ReportLanguage::En,
    )
    .unwrap();
    let html = fs::read_to_string(root.join("many/report.html")).unwrap();
    assert_eq!(html.matches("data-finding-id=").count(), 1000);
    assert!(html.contains("occurrence-999"));
    let mut tls = result(1);
    tls.findings[0].rule_id = "ENVOY_TLS_MIN_VERSION_LEGACY".into();
    tls.findings[0].migration_hint = "Use TLS 1.2 or later".into();
    tls.findings[0].recommended_actions[0].action_id = "tls.signature-readiness-note".into();
    tls.findings[0].recommended_actions[0].title = "Signature readiness note".into();
    for (name, lang, expected) in [
        ("tls-en", ReportLanguage::En, "Prefer TLS 1.3"),
        ("tls-ja", ReportLanguage::Ja, "TLSv1_3"),
    ] {
        write_reports_with_language(&tls, &root.join(name), ReportFormat::All, lang).unwrap();
        for filename in ["report.html", "report.md"] {
            let report = fs::read_to_string(root.join(name).join(filename)).unwrap();
            assert!(report.contains(expected));
            assert!(!report.contains("Use TLS 1.2 or later"));
        }
    }
    fs::remove_dir_all(root).unwrap();
}
