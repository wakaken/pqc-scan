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
fn overlapping_non_equivalent_records_are_explained_without_merging_machine_evidence() {
    let root = std::env::temp_dir().join(format!("pqc-overlap-{}", std::process::id()));
    let mut raw = result(2);
    raw.findings[1].location = raw.findings[0].location.clone();
    raw.findings[0].location.line = 1;
    raw.findings[1].location.line = 1;
    raw.findings[1].rule_id = "OTHER_REFERENCE".into();
    raw.findings[1].description = "Distinct source evidence".into();
    raw.findings[1].risk = Risk::QuantumUncertain;
    for (name, language, phrase) in [
        ("ja", ReportLanguage::Ja, "この位置の検出記録"),
        (
            "en",
            ReportLanguage::En,
            "Detection records at this location",
        ),
    ] {
        write_reports_with_language(&raw, &root.join(name), ReportFormat::All, language).unwrap();
        let html = fs::read_to_string(root.join(name).join("report.html")).unwrap();
        let md = fs::read_to_string(root.join(name).join("report.md")).unwrap();
        assert!(html.contains(phrase));
        assert!(md.contains(phrase));
        assert!(html.contains("Distinct source evidence"));
        let machine: serde_json::Value =
            serde_json::from_slice(&fs::read(root.join(name).join("report.json")).unwrap())
                .unwrap();
        assert_eq!(machine["findings"].as_array().unwrap().len(), 2);
        assert_eq!(machine["findings"][1]["risk"], "quantum-uncertain");
    }
    fs::remove_dir_all(root).unwrap();
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
        let heading = html
            .split("class=\"rule-title\"")
            .nth(1)
            .unwrap()
            .split("</summary>")
            .next()
            .unwrap();
        assert!(
            !heading.contains("JWT_RS256"),
            "rule IDs must not appear in result headings"
        );
        let matched = html.split("class=\"matched-rules\"").nth(1).unwrap();
        let section = matched
            .split("<section id=\"scan-match-occurrence-")
            .nth(1)
            .unwrap()
            .split("</section>")
            .next()
            .unwrap();
        let (anchor, section) = section.split_once('>').unwrap();
        assert!(anchor.contains("tabindex=\"-1\""));
        assert!(section.starts_with("<h4>"));
        assert!(!section.split("</h4>").next().unwrap().contains("JWT_RS256"));
        assert!(
            section.contains("<p><code>JWT_RS256</code>"),
            "matched rule detail retains its ID"
        );
        assert!(html.contains("data-page-size=\"20\""));
        assert_eq!(
            html.split("<template")
                .next()
                .unwrap()
                .matches("data-finding-id=")
                .count(),
            25
        );
        assert!(html.contains("data-match-count>25 / 25</span>"));
        assert_eq!(
            html.split("<template")
                .next()
                .unwrap()
                .matches("shared-rationale")
                .count(),
            1
        );
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
    assert_eq!(
        html.split("<template")
            .next()
            .unwrap()
            .matches("data-finding-id=")
            .count(),
        1000
    );
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

#[test]
fn dashboard_keeps_exact_counts_escaped_filters_and_machine_outputs() {
    let root = std::env::temp_dir().join(format!(
        "pqc-dashboard-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut scan = result(5);
    for (finding, severity) in scan.findings.iter_mut().zip([
        Severity::Critical,
        Severity::High,
        Severity::Medium,
        Severity::Low,
        Severity::Info,
    ]) {
        finding.severity = severity;
    }
    scan.findings[4].risk = Risk::QuantumSafe;
    scan.findings[4].rule_id = "PQC_STANDARD_REFERENCE".into();
    scan.findings[0].location.file = "src/<name>\"&.py".into();
    // The chart must describe actual records, even if a library caller supplies
    // stale summary counters. Rendering must not rewrite the machine report.
    scan.summary.by_severity.clear();
    scan.findings[0].description = format!(
        "{} <full explanation>",
        "Long finding description ".repeat(8)
    );
    for (name, lang) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        write_reports_with_language(&scan, &root.join(name), ReportFormat::All, lang).unwrap();
        let html = fs::read_to_string(root.join(name).join("report.html")).unwrap();
        assert_eq!(
            html.split("<template")
                .next()
                .unwrap()
                .matches("data-finding-id=")
                .count(),
            5
        );
        assert_eq!(
            html.split("<template")
                .next()
                .unwrap()
                .matches("style=\"width:20.0000%\"")
                .count(),
            5
        );
        assert!(html.contains("data-file=\"src/&lt;name&gt;&quot;&amp;.py\""));
        assert!(html.contains("<option value=\"src/&lt;name&gt;&quot;&amp;.py\">"));
        assert!(html.contains("data-rule-filter"));
        if lang == ReportLanguage::En {
            assert!(html.contains(&format!(
                "<p class=\"finding-description\">{} &lt;full explanation&gt;</p>",
                "Long finding description ".repeat(8)
            )));
        }
        assert!(html.contains("data-file-filter"));
        assert!(html.contains("href=\"#findings\""));
        assert!(html.contains("aria-atomic=\"true\""));
        assert!(html.contains("data-report-controls hidden"));
        assert!(!html.contains("<script src="));
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(root.join(name).join("report.json")).unwrap())
                .unwrap();
        let expected: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&scan).unwrap()).unwrap();
        assert_eq!(json, expected);
    }
    for file in [
        "report.json",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
    ] {
        assert_eq!(
            fs::read(root.join("en").join(file)).unwrap(),
            fs::read(root.join("ja").join(file)).unwrap()
        );
    }
    write_reports_with_language(
        &result(0),
        &root.join("empty"),
        ReportFormat::Html,
        ReportLanguage::En,
    )
    .unwrap();
    let empty = fs::read_to_string(root.join("empty/report.html")).unwrap();
    assert!(empty.contains("No findings in the scanned files."));
    assert!(!empty.contains("width:NaN"));
    assert!(!empty.contains("width:inf"));
    assert_eq!(empty.matches("data-finding-id=").count(), 0);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn bilingual_report_and_companion_explain_rules_without_changing_machine_data() {
    let root = std::env::temp_dir().join(format!(
        "pqc-presentation-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut scan = result(1);
    scan.findings[0].rule_id = "CUSTOM_<rule>\"".into();
    scan.findings[0].description = "Explanation <untrusted> & detail".into();
    let paths =
        write_reports_with_language(&scan, &root, ReportFormat::All, ReportLanguage::Ja).unwrap();
    assert_eq!(paths.len(), 7);
    let html = fs::read_to_string(root.join("report.html")).unwrap();
    let guide = fs::read_to_string(root.join("report.rules.html")).unwrap();
    let (active, alternate) = html
        .split_once("<template data-report-translation")
        .unwrap();
    assert!(active.contains("data-report-view lang=\"ja\""));
    assert!(alternate.contains("data-report-view lang=\"en\""));
    assert_eq!(active.matches("data-finding-id=").count(), 1);
    assert_eq!(alternate.matches("data-finding-id=").count(), 1);
    for disclosure in html.split("<details").skip(1) {
        assert!(disclosure.split('>').next().unwrap().contains("id=\""));
    }
    assert!(html.contains("data-language"));
    assert!(html.contains("data-theme-select"));
    assert!(html.contains("datetime=\""));
    assert!(html.contains("2026-"));
    assert!(html.contains("report.rules.html?lang=ja#rule-0"));
    assert!(!html.contains("コードや設定の手掛かりをまとめた調査結果です。"));
    assert!(!html.contains("<untrusted>"));
    assert!(guide.contains("Explanation &lt;untrusted&gt; &amp; detail"));
    assert!(guide.contains("CUSTOM_&lt;rule&gt;&quot;"));
    assert!(guide.contains("report.html?lang=en#rule-0"));
    assert!(guide.contains("data-report-translation"));
    assert!(!guide.contains("<script src="));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tree_uses_all_occurrences_escapes_paths_and_clarifies_certificate_cutoff_only_for_people() {
    let root = std::env::temp_dir().join(format!(
        "pqc-tree-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut scan = result(3);
    scan.findings[0].location.file = "src/a.py".into();
    scan.findings[1].location.file = "src/a.py".into();
    scan.findings[2].location.file = "tests/<example>&.pem".into();
    scan.findings[2].rule_id = "CERT_EXPIRY_2026_OR_BEFORE".into();
    scan.findings[2].description = "Original fixed-cutoff machine description".into();
    scan.findings[2].migration_hint = "Original machine hint".into();
    write_reports_with_language(&scan, &root, ReportFormat::All, ReportLanguage::Ja).unwrap();
    let html = fs::read_to_string(root.join("report.html")).unwrap();
    let active = html.split("<template").next().unwrap();
    assert!(active.contains("src/a.py、全検出2件"));
    assert!(active.contains("data-tree-file=\"tests/&lt;example&gt;&amp;.pem\""));
    assert!(!active.contains("data-toggle-filters"));
    assert!(active.contains("<details id=\"report-filters\""));
    assert!(!active.contains("data-toggle-tree"));
    assert!(active.contains("<details id=\"report-file-tree\""));
    assert!(!html.contains("従来の固定基準"));
    assert!(html.contains("解析した証明書の有効期限が2026年以前です。"));
    assert!(html.contains("2027年1月1日UTCより前"));
    assert!(html.contains("January 1, 2027 UTC"));
    let machine: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join("report.json")).unwrap()).unwrap();
    assert_eq!(
        machine["findings"][2]["description"],
        "Original fixed-cutoff machine description"
    );
    assert_eq!(
        machine["findings"][2]["migration_hint"],
        "Original machine hint"
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn deep_tree_hover_paths_are_complete_and_escape_attribute_text() {
    let root = std::env::temp_dir().join(format!(
        "pqc-deep-hover-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut scan = result(1);
    scan.findings[0].location.file="test-framework/oauth/src/main/java/org/keycloak/testframework/oauth/<unsafe>\"&/OAuthIdentityProvider.java".into();
    write_reports_with_language(&scan, &root, ReportFormat::All, ReportLanguage::Ja).unwrap();
    let html = fs::read_to_string(root.join("report.html")).unwrap();
    assert!(html.contains("<summary title=\"test-framework/oauth/src/main/java/org/keycloak/testframework/oauth/&lt;unsafe&gt;&quot;&amp;\">"));
    assert!(html.contains("title=\"test-framework/oauth/src/main/java/org/keycloak/testframework/oauth/&lt;unsafe&gt;&quot;&amp;/OAuthIdentityProvider.java\""));
    assert!(html.contains("data-tree-file=\"test-framework/oauth/src/main/java/org/keycloak/testframework/oauth/&lt;unsafe&gt;&quot;&amp;/OAuthIdentityProvider.java\""));
    fs::remove_dir_all(root).unwrap();
}
