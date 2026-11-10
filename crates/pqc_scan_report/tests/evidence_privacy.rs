use pqc_scan_core::ScanResult;
use pqc_scan_report::{write_reports_with_language, ReportFormat, ReportLanguage};
use serde_json::{json, Value};
use std::fs;
fn result() -> ScanResult {
    let secret = "LEAK029A_SYNTHETIC_NOT_A_SECRET_Z029ZZ";
    serde_json::from_value(json!({
      "generated_at":"2026-11-10T00:00:00Z",
      "findings":[{"finding_id":"stable-id","rule_id":"SYNTHETIC","category":"TLS","risk":"quantum-vulnerable","severity":"high","confidence":0.75,
        "description":"Technical RS256 / 2048-bit reference",
        "migration_hint":format!("Preserve 2048 and version1.2.3; client_secret='{secret}'"),
        "location":{"file":"安全.conf","line":4,"column":2},
        "evidence":{"type":"regex_match","match":"RS256","snippet_preview":format!("password=\"{secret}\""),"metadata":{"key_bits":"2048","signature_oid":"1.2.840.113549.1.1.11","access_token":secret,"context":format!("-----BEGIN EC PRIVATE KEY-----\n{secret}\n-----END EC PRIVATE KEY-----")}},
        "recommended_actions":[{"action_id":"same-action","title":"Keep technical guidance","priority":"high","rationale":format!("password='{secret}'"),"steps":["Keep RSA2048 as observed, not a PQC fix",format!("Authorization: Bearer {secret}")],"references":[format!("https://user:{secret}@example.invalid/path?token={secret}&mode=read"),"javascript:<img src=x onerror=alert(1)>"],"code_examples":[{"language":"javascript","before":format!("const api_key=\"{secret}\"; const alg='RS256';"),"after":"const alg='RS256';"}]}],
        "source_snippet":{"lines":[{"line":4,"text":format!("api_key='{secret}'"),"highlighted":true}]}}],
      "cbom":[{"component":"service","algorithm":format!("password={secret}"),"usage_type":"TLS","location":"安全.conf:4","quantum_risk":"quantum-vulnerable","migration_hint":"Preserve technical policy"}],
      "dependency_sbom":[{"name":"public-package","version":"1.2.3","ecosystem":"generic","source_file":"package.json","source_type":"manifest","purl":format!("pkg:generic/public-package@1.2.3?token={secret}")}],
      "summary":{"total_findings":1,"by_severity":{"high":1},"by_risk":{"quantum-vulnerable":1},"scanned_files":1,"skipped_files":0}})).unwrap()
}
#[test]
fn known_secret_syntax_is_omitted_in_all_legacy_formats_and_both_languages() {
    for (name, lang) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        let root =
            std::env::temp_dir().join(format!("pqc029-report-{}-{name}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let input = result();
        let before = serde_json::to_value(&input).unwrap();
        let paths = write_reports_with_language(&input, &root, ReportFormat::All, lang).unwrap();
        assert_eq!(paths.len(), 6);
        for path in paths {
            let text = fs::read_to_string(&path).unwrap();
            assert!(
                !text.contains("LEAK029"),
                "canary survived in {}",
                path.file_name().unwrap().to_string_lossy()
            );
        }
        assert_eq!(
            serde_json::to_value(&input).unwrap(),
            before,
            "writer must not mutate its caller"
        );
        let out: Value =
            serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
        assert_eq!(out["findings"][0]["finding_id"], "stable-id");
        assert_eq!(out["findings"][0]["evidence"]["match"], "RS256");
        assert_eq!(
            out["findings"][0]["evidence"]["metadata"]["key_bits"],
            "2048"
        );
        assert_eq!(
            out["findings"][0]["evidence"]["metadata"]["signature_oid"],
            "1.2.840.113549.1.1.11"
        );
        assert_eq!(out["dependency_sbom"][0]["version"], "1.2.3");
        let html = fs::read_to_string(root.join("report.html")).unwrap();
        assert!(!html.contains("<img src=x"));
        assert!(!html.contains("href=\"javascript:"));
        fs::remove_dir_all(root).unwrap();
    }
}
