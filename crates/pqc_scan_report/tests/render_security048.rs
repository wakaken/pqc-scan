//! Synthetic adversarial writer inputs; retained artifacts are real writer output.
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport, ProbeTarget},
    target_policy::{ProbeLimits, TargetRequest},
    ScanResult,
};
use pqc_scan_report::{
    write_probe_reports, write_reports_with_language, ReportFormat, ReportLanguage,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const ATTACK: &str = "</script><script>document.title='EXECUTED048'</script><img src=x onerror=\"document.title='EXECUTED048'\"><svg onload=\"document.title='EXECUTED048'\"> [click](javascript:document.title='EXECUTED048') | ` ``` \" & 日😀\r\n<img src=x onerror=\"document.title='EXECUTED048'\">\r<script>document.title='EXECUTED048'</script>";
fn retain(name: &str, path: &Path) {
    if let Some(out) = std::env::var_os("PQC048_EVIDENCE") {
        let out = PathBuf::from(out).join(name);
        fs::create_dir_all(&out).unwrap();
        for e in fs::read_dir(path).unwrap() {
            let e = e.unwrap();
            fs::copy(e.path(), out.join(e.file_name())).unwrap();
        }
    }
}
fn scan() -> ScanResult {
    let findings:Vec<_>=(0..25).map(|i|json!({
        "finding_id":format!("synthetic048-{i}"),"rule_id":"SYNTHETIC048","category":"TLS","risk":"quantum-vulnerable","severity":"high","confidence":0.8,
        "description":ATTACK,"migration_hint":ATTACK,"location":{"file":format!("日本/unsafe-\"<{i}>.py"),"line":i+1,"column":3},
        "evidence":{"type":"regex_match","match":ATTACK,"snippet_preview":ATTACK,"metadata":{}},
        "recommended_actions":[{"action_id":"attack048","title":ATTACK,"priority":"high","rationale":ATTACK,"steps":[ATTACK],"references":["javascript:document.title='EXECUTED048'","data:text/html,<script>document.title='EXECUTED048'</script>","https://example.invalid/\" onmouseover=\"document.title='EXECUTED048'"],"code_examples":[{"language":"python","before":ATTACK,"after":"safe"}]}],
        "source_snippet":{"lines":[{"line":i+1,"text":format!("{ATTACK} password='CANARY048_SECRET'"),"highlighted":true}]}
    })).collect();
    serde_json::from_value(json!({"generated_at":"2026-11-14T00:00:00Z","findings":findings,"cbom":[],"dependency_sbom":[],"summary":{"total_findings":25,"by_severity":{"high":25},"by_risk":{"quantum-vulnerable":25},"scanned_files":25,"skipped_files":0}})).unwrap()
}
fn check(dir: &Path, machine: &str, count: usize) {
    let html = fs::read_to_string(dir.join("report.html")).unwrap();
    let md = fs::read_to_string(dir.join("report.md")).unwrap();
    assert!(!html.contains("<script>document.title"));
    assert!(!html.contains("<img src=x"));
    assert!(!html.contains("<svg onload="));
    assert!(!html.contains("href=\"javascript:"));
    assert!(!html.contains("href=\"data:"));
    for line in md.lines().filter(|line| !line.starts_with("    ")) {
        assert!(
            !line.contains("<script>document.title"),
            "active Markdown HTML: {line}"
        );
        assert!(
            !line.contains("<img src=x"),
            "active Markdown image: {line}"
        );
    }
    let value: Value = serde_json::from_slice(&fs::read(dir.join(machine)).unwrap()).unwrap();
    assert!(
        value.to_string().contains("EXECUTED048"),
        "payload must reach the writer"
    );
    if count > 0 {
        assert_eq!(value["findings"].as_array().unwrap().len(), count);
        assert_eq!(html.matches("class=\"occurrence\"").count(), count);
        assert_eq!(md.matches("<a id=\"finding-").count(), count);
    }
    for e in fs::read_dir(dir).unwrap() {
        assert!(!fs::read_to_string(e.unwrap().path())
            .unwrap()
            .contains("CANARY048_SECRET"));
    }
}
#[test]
fn adversarial_static_and_probe_fields_remain_data_in_both_languages() {
    let root = std::env::temp_dir().join(format!("pqc048-render-{}", std::process::id()));
    assert!(!root.exists());
    for (name, lang) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        let dir = root.join(name);
        write_reports_with_language(&scan(), &dir, ReportFormat::All, lang).unwrap();
        check(&dir, "report.json", 25);
        let sarif: Value =
            serde_json::from_slice(&fs::read(dir.join("report.sarif")).unwrap()).unwrap();
        assert_eq!(sarif["runs"][0]["results"].as_array().unwrap().len(), 25);
        retain(&format!("attack-static-{name}"), &dir);
        for (tag, protocol) in [
            ("tls", ProbeProtocol::Tls),
            ("ssh-offers", ProbeProtocol::SshOffers),
            ("ssh-kex", ProbeProtocol::SshKex),
        ] {
            let target = ProbeTarget::requested(&TargetRequest::new("localhost", 443).unwrap());
            let mut report = ProbeReport::new(
                protocol,
                target,
                &ProbeLimits::default(),
                json!({"untrusted":ATTACK,"credential":"password='CANARY048_SECRET'"}),
            );
            report.set_outcome(ProbeOutcome::Failed, vec![ProbeDiagnostic::AddressDenied]);
            let dir = root.join(format!("{tag}-{name}"));
            write_probe_reports(&report, &dir, lang).unwrap();
            check(&dir, &format!("probe.{tag}.v1.json"), 0);
            retain(&format!("attack-{tag}-{name}"), &dir);
        }
    }
    fs::remove_dir_all(root).unwrap();
}
