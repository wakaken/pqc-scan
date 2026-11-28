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
        assert_eq!(paths.len(), 7);
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

#[test]
fn structured_caller_results_are_sanitized_in_all_formats_without_mutation() {
    let mut input = result();
    input.findings[0].evidence.snippet_preview =
        "/* Example: \" { examples aren't code.\n{\n kty: 'RSA', d: { /* } */ value: 'SYNTH098_CALLER_D' }, n: 'PUBLIC_MODULUS', e: 'AQAB'\n} */\n// Note: \" scalar = 'SYNTH098_CALLER_SCALAR';".into();
    input.findings[0].source_snippet.as_mut().unwrap().lines[0].text =
        "private = [ /* outer /* inner */ ] */ 'SYNTH098_CALLER_PRIVATE' ]; scalar = 98217364198217364u64; seed = 98217364198217364M; private = ('PUBLIC_VALUE' + Lookup('PUBLIC_ENV'));".into();
    let before = serde_json::to_value(&input).unwrap();
    for (name, language) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        let root =
            std::env::temp_dir().join(format!("pqc098-report-{}-{name}", std::process::id()));
        let paths =
            write_reports_with_language(&input, &root, ReportFormat::All, language).unwrap();
        for path in paths {
            let text = fs::read_to_string(path).unwrap();
            assert!(!text.contains("SYNTH098"));
            assert!(!text.contains("98217364198217364"));
        }
        let report: Value =
            serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
        assert_eq!(report["findings"][0]["finding_id"], "stable-id");
        assert_eq!(report["findings"][0]["evidence"]["match"], "RS256");
        assert!(report["findings"][0]["evidence"]["snippet_preview"]
            .as_str()
            .unwrap()
            .contains("PUBLIC_MODULUS"));
        fs::remove_dir_all(root).unwrap();
    }
    assert_eq!(serde_json::to_value(input).unwrap(), before);
}

#[test]
fn caller_jwk_other_primes_are_masked_in_all_formats_and_both_languages() {
    let mut input = result();
    input.findings[0].evidence.metadata = serde_json::from_value(json!({
        "kty":"RSA",
        "oth":r#"[{"r":"SYNTH_V8_REPORT_PRIVATE_R","d":"SYNTH_V8_REPORT_PRIVATE_D","t":"SYNTH_V8_REPORT_PRIVATE_T"}]"#,
        "n":"PUBLIC_N", "e":"AQAB", "r":"PUBLIC_R", "t":"PUBLIC_T"
    })).unwrap();
    let mut ordinary = input.findings[0].clone();
    ordinary.finding_id = "ordinary-metadata".into();
    ordinary.evidence.metadata.remove("kty");
    ordinary.evidence.metadata.insert(
        "oth".into(),
        r#"[{"r":"PUBLIC_R","d":"PUBLIC_D","t":"PUBLIC_T"}]"#.into(),
    );
    let ordinary_metadata = serde_json::to_value(&ordinary.evidence.metadata).unwrap();
    input.findings.push(ordinary);
    let before = serde_json::to_value(&input).unwrap();
    for (name, language) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        let root =
            std::env::temp_dir().join(format!("pqc-v8-jwk-report-{}-{name}", std::process::id()));
        let paths =
            write_reports_with_language(&input, &root, ReportFormat::All, language).unwrap();
        assert_eq!(paths.len(), 7);
        for path in paths {
            let text = fs::read_to_string(path).unwrap();
            assert!(!text.contains("SYNTH_V8_REPORT_PRIVATE"));
        }
        let report: Value =
            serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
        let findings = report["findings"].as_array().unwrap();
        assert_eq!(findings.len(), 2);
        let metadata = &findings[0]["evidence"]["metadata"];
        assert_eq!(metadata["oth"], "[masked-sensitive-value]");
        for field in ["kty", "n", "e", "r", "t"] {
            assert_eq!(
                metadata[field],
                before["findings"][0]["evidence"]["metadata"][field]
            );
        }
        assert_eq!(findings[1]["evidence"]["metadata"], ordinary_metadata);
        for (index, finding) in findings.iter().enumerate() {
            for field in [
                "finding_id",
                "rule_id",
                "category",
                "risk",
                "severity",
                "confidence",
                "location",
            ] {
                assert_eq!(finding[field], before["findings"][index][field]);
            }
            assert_eq!(finding["evidence"]["match"], "RS256");
        }
        assert_eq!(serde_json::to_value(&input).unwrap(), before);
        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn caller_printed_private_dump_metadata_and_actions_are_hidden_in_every_format() {
    let dump = "ED25519 Private-Key:\npriv:\n  11:22:33:44:55:66:77:88\npub:\n  04:ca:fe:00";
    let public = "ED25519 Public-Key:\npub:\n  ab:cd:ef:09";
    let mut input = result();
    input.findings[0].evidence.metadata.clear();
    input.findings[0]
        .evidence
        .metadata
        .insert("dump".into(), dump.into());
    input.findings[0]
        .evidence
        .metadata
        .insert("public_dump".into(), public.into());
    input.findings[0].description = format!("Expected output\n{dump}");
    input.findings[0].recommended_actions[0].code_examples[0].before =
        format!("expected = r#\"{dump}\"#; PUBLIC_TAIL();");
    let before = serde_json::to_value(&input).unwrap();
    for (name, language) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        let root = std::env::temp_dir().join(format!(
            "pqc-printed-dump-report-{}-{name}",
            std::process::id()
        ));
        let paths =
            write_reports_with_language(&input, &root, ReportFormat::All, language).unwrap();
        assert_eq!(paths.len(), 7);
        for path in paths {
            let text = fs::read_to_string(path).unwrap();
            assert!(!text.contains("11:22:33:44"));
            assert!(!text.contains("55:66:77:88"));
        }
        let report: Value =
            serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
        assert_eq!(
            report["findings"][0]["evidence"]["metadata"]["public_dump"],
            public
        );
        assert!(
            report["findings"][0]["recommended_actions"][0]["code_examples"][0]["before"]
                .as_str()
                .unwrap()
                .contains("PUBLIC_TAIL")
        );
        for field in [
            "finding_id",
            "rule_id",
            "category",
            "risk",
            "severity",
            "confidence",
            "location",
        ] {
            assert_eq!(report["findings"][0][field], before["findings"][0][field]);
        }
        fs::remove_dir_all(root).unwrap();
    }
    assert_eq!(serde_json::to_value(&input).unwrap(), before);
}
