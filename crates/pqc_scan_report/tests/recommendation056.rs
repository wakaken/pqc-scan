//! Real writer regressions and retained synthetic reports for task056 remediation.
use pqc_scan_core::{recommendation, Finding, ScanResult};
use pqc_scan_report::{write_reports_with_language, ReportFormat, ReportLanguage};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn corrected_recommendations_reach_both_languages_without_changing_machine_facts() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("pqc056-copy-{}-{unique}", std::process::id());
    let root = Scratch(std::env::temp_dir().join(name));
    let mut findings = vec![];
    for (id, category, file, java) in [
        ("JAVA_UNKNOWN056", "CryptoAPI", "unknown/Example.java", None),
        (
            "JAVA_CURRENT056",
            "CryptoAPI",
            "current/Example.java",
            Some("24"),
        ),
        ("PKI056", "PKI", "certificate.pem", None),
        ("ENVOY056", "TLS", "envoy.yaml", None),
    ] {
        let f: Finding = serde_json::from_value(json!({
            "finding_id":id,"rule_id":id,"category":category,"risk":"quantum-vulnerable",
            "severity":"medium","confidence":0.8,"description":"Synthetic migration reference",
            "migration_hint":"Review actual use before migration",
            "location":{"file":file,"line":1,"column":1},
            "evidence":{"type":"configuration_match","match":"synthetic reference",
                "snippet_preview":"synthetic reference","metadata":{"scope":"config"}}
        }))
        .unwrap();
        let mut one = vec![f];
        let mut profile = recommendation::RuntimeProfile::default();
        profile.java_version = java.map(str::to_string);
        recommendation::annotate_findings(&mut one, &profile);
        findings.extend(one);
    }
    let result: ScanResult = serde_json::from_value(json!({
        "generated_at":"2026-11-15T00:00:00Z", "findings":findings,"cbom":[],"dependency_sbom":[],
        "summary":{"total_findings":4,"by_severity":{"medium":4},
        "by_risk":{"quantum-vulnerable":4},"scanned_files":4,"skipped_files":0}
    }))
    .unwrap();
    let mut reports: BTreeMap<String, String> = BTreeMap::new();
    for (language, name) in [(ReportLanguage::En, "en"), (ReportLanguage::Ja, "ja")] {
        write_reports_with_language(&result, &root.0.join(name), ReportFormat::All, language)
            .unwrap();
        for file in ["report.html", "report.md", "report.json", "report.sarif"] {
            reports.insert(
                format!("{name}/{file}"),
                fs::read_to_string(root.0.join(name).join(file)).unwrap(),
            );
        }
    }
    for file in ["report.json", "report.sarif"] {
        assert_eq!(
            reports[&format!("en/{file}")],
            reports[&format!("ja/{file}")]
        );
    }
    for file in ["report.html", "report.md"] {
        let en = &reports[&format!("en/{file}")];
        let ja = &reports[&format!("ja/{file}")];
        for text in [
            "version is unknown",
            "new ML-DSA key pair",
            "construction",
            "encoding",
            "ECDHE_RSA",
            "not TLS 1.3",
        ] {
            assert!(
                en.contains(text),
                "missing English prerequisite {text} in {file}"
            );
        }
        for text in [
            "バージョンが不明",
            "新しいML-DSA鍵ペア",
            "BC",
            "構成と符号化形式",
            "ECDHE_RSA",
            "TLS 1.3には適用",
        ] {
            assert!(
                ja.contains(text),
                "missing Japanese prerequisite {text} in {file}"
            );
        }
        assert!(!en.contains("Remove RSA-dependent suites"));
        assert!(!ja.contains("ハイブリッド証明書チェーンを試してください"));
    }
    let machine: Value = serde_json::from_str(&reports["en/report.json"]).unwrap();
    assert_eq!(machine["findings"][0]["confidence"], json!(0.8));
    println!(
        "REMEDIATION056_REPORTS:{}",
        serde_json::to_string(&reports).unwrap()
    );
}
