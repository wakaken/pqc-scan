use chrono::Utc;
use pqc_scan_core::{
    CbomEntry, Evidence, Finding, Location, ScanResult, ScanSummary, SourceSnippet,
    SourceSnippetLine,
};
use pqc_scan_report::{write_reports, write_reports_with_language, ReportFormat, ReportLanguage};
use pqc_scan_rules::{Risk, Severity};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn japanese_human_reports_preserve_machine_output_and_source_evidence() {
    let root = std::env::temp_dir().join(format!(
        "pqc-ja-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut result = ScanResult {
        generated_at: Utc::now(),
        findings: vec![Finding {
            finding_id: "same-id".into(),
            rule_id: "PQC_GO_TLS_DISABLED".into(),
            category: "TLS".into(),
            risk: Risk::QuantumUncertain,
            severity: Severity::Medium,
            confidence: 0.8,
            description: "Go hybrid TLS default explicitly disabled".into(),
            migration_hint: "Review tlsmlkem=0".into(),
            location: Location {
                file: "<prod>&.conf".into(),
                line: 1,
                column: 1,
            },
            evidence: Evidence {
                r#type: "regex_match".into(),
                r#match: "GODEBUG=tlsmlkem=0".into(),
                snippet_preview: "Sample Evidence".into(),
                metadata: Default::default(),
            },
            recommended_actions: vec![],
            source_snippet: Some(SourceSnippet {
                lines: vec![SourceSnippetLine {
                    line: 1,
                    text: "Total Findings <script>alert(1)</script>".into(),
                    highlighted: true,
                }],
            }),
        }],
        cbom: vec![CbomEntry {
            component: "<prod>&.conf".into(),
            algorithm: "GODEBUG=tlsmlkem=0".into(),
            usage_type: "TLS".into(),
            location: "<prod>&.conf:1".into(),
            quantum_risk: Risk::QuantumUncertain,
            migration_hint: "Review tlsmlkem=0".into(),
        }],
        dependency_sbom: vec![],
        summary: ScanSummary {
            total_findings: 1,
            by_severity: [("medium".into(), 1)].into(),
            by_risk: [("quantum-uncertain".into(), 1)].into(),
            scanned_files: 1,
            skipped_files: 0,
        },
    };
    write_reports_with_language(
        &result,
        &root.join("ja"),
        ReportFormat::All,
        ReportLanguage::Ja,
    )
    .unwrap();
    write_reports_with_language(
        &result,
        &root.join("en"),
        ReportFormat::All,
        ReportLanguage::En,
    )
    .unwrap();
    for file in [
        "report.json",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
    ] {
        assert_eq!(
            fs::read(root.join("ja").join(file)).unwrap(),
            fs::read(root.join("en").join(file)).unwrap()
        );
    }
    let html = fs::read_to_string(root.join("ja/report.html")).unwrap();
    assert!(html.contains("lang=\"ja\""));
    assert!(html.contains("耐量子暗号への移行調査レポート"));
    assert!(html.contains("Goのハイブリッド鍵共有を無効にする設定"));
    assert!(html.contains("要確認"));
    assert!(html.contains("確認すること"));
    assert!(!html.contains("確認してほしいこと"));
    assert!(html.contains("GODEBUG=tlsmlkem=0"));
    assert!(html.contains("&lt;prod&gt;&amp;.conf"));
    assert!(!html.contains("<prod>"));
    assert!(html.contains("Total Findings &lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(!html.contains("<script>alert(1)</script>"));
    let md = fs::read_to_string(root.join("ja/report.md")).unwrap();
    assert!(md.contains("検出箇所"));
    assert!(md.contains("確認すること"));
    assert!(!md.contains("確認してほしいこと"));
    assert!(md.contains("PQC_GO_TLS_DISABLED"));
    assert!(!md.contains("Review tlsmlkem=0"));
    write_reports(&result, &root.join("default"), ReportFormat::All).unwrap();
    for file in [
        "report.html",
        "report.md",
        "report.json",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
    ] {
        assert_eq!(
            fs::read(root.join("default").join(file)).unwrap(),
            fs::read(root.join("en").join(file)).unwrap()
        );
    }
    result.findings[0].rule_id = "CUSTOM_RULE".into();
    result.findings[0].description = "Custom <unsafe> description".into();
    result.findings[0].evidence.r#match = "-----BEGIN PRIVATE KEY-----".into();
    result.findings[0].source_snippet.as_mut().unwrap().lines[0].text =
        "-----BEGIN PRIVATE KEY-----".into();
    write_reports_with_language(
        &result,
        &root.join("custom"),
        ReportFormat::All,
        ReportLanguage::Ja,
    )
    .unwrap();
    let custom = fs::read_to_string(root.join("custom/report.html")).unwrap();
    assert!(custom.contains("日本語の説明が未登録"));
    assert!(custom.contains("Custom &lt;unsafe&gt; description"));
    assert!(!custom.contains("BEGIN PRIVATE KEY"));
    assert!(custom.contains("[masked-private-key-material]"));
    assert!(fs::read_to_string(root.join("custom/report.md"))
        .unwrap()
        .contains("日本語の説明が未登録"));
    result.findings.clear();
    result.cbom.clear();
    result.summary.total_findings = 0;
    result.summary.by_severity.clear();
    result.summary.by_risk.clear();
    write_reports_with_language(
        &result,
        &root.join("empty"),
        ReportFormat::Html,
        ReportLanguage::Ja,
    )
    .unwrap();
    assert!(fs::read_to_string(root.join("empty/report.html"))
        .unwrap()
        .contains("該当する検出はありませんでした"));
    assert!(!root.join("empty/report.md").exists());
    assert!(root.join("empty/cbom.json").exists());
    fs::remove_dir_all(root).unwrap();
}
