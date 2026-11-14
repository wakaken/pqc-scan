use pqc_scan_core::{NativeScan, ScanResult};
use pqc_scan_report::{cyclonedx, write_reports_with_language, ReportFormat, ReportLanguage};
use serde_json::{json, Value};
use std::{fs, path::Path};
fn schema() -> jsonschema::Validator {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/cyclonedx/1.7");
    let mut options = jsonschema::options();
    for name in [
        "spdx.schema.json",
        "jsf-0.82.schema.json",
        "cryptography-defs.schema.json",
    ] {
        options = options.with_resource(
            format!("http://cyclonedx.org/schema/{name}"),
            jsonschema::Resource::from_contents(
                serde_json::from_slice(&fs::read(root.join(name)).unwrap()).unwrap(),
            )
            .unwrap(),
        );
    }
    options
        .build(
            &serde_json::from_slice::<Value>(&fs::read(root.join("bom-1.7.schema.json")).unwrap())
                .unwrap(),
        )
        .unwrap()
}
fn inspect(bytes: &[u8], out: &Path) -> Value {
    let Ok(value) = serde_json::from_slice::<Value>(bytes) else {
        return json!({"output":"rejected_json"});
    };
    if value.get("schema_version").is_some() {
        let Ok(native) = serde_json::from_value::<NativeScan>(value) else {
            return json!({"output":"rejected_native"});
        };
        if native.validate().is_err() {
            return json!({"output":"rejected_references"});
        }
        let serialized = serde_json::to_vec(&native).unwrap();
        let again: NativeScan = serde_json::from_slice(&serialized).unwrap();
        again.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&native).unwrap(),
            serde_json::to_value(again).unwrap()
        );
        let (crypto, software) = cyclonedx::documents(&native).unwrap();
        let validator = schema();
        for doc in [crypto, software] {
            assert!(validator.is_valid(&doc), "CycloneDX schema violation");
        }
        return json!({"output":"native_roundtrip_and_cyclonedx17"});
    }
    let Ok(mut result) = serde_json::from_value::<ScanResult>(value) else {
        return json!({"output":"rejected_legacy"});
    };
    // Independently inserted, recognized secret syntax. Paths and opaque values are not an oracle.
    for finding in &mut result.findings {
        finding.description.push_str(" password='CANARY047_EXPORT'");
    }
    for (tag, language) in [("en", ReportLanguage::En), ("ja", ReportLanguage::Ja)] {
        let paths =
            write_reports_with_language(&result, &out.join(tag), ReportFormat::All, language)
                .unwrap();
        assert_eq!(paths.len(), 6);
        for path in paths {
            let text = fs::read_to_string(&path).unwrap();
            assert!(!text.contains("CANARY047_EXPORT"));
            if path.extension().is_some_and(|x| x == "json") {
                let _: Value = serde_json::from_str(&text).unwrap();
            }
        }
    }
    json!({"output":"legacy_six_formats_both_languages","findings":result.findings.len()})
}
#[test]
fn invalid_json_and_valid_empty_export_are_distinct() {
    let root = std::env::temp_dir().join(format!("pqc047-report-smoke-{}", std::process::id()));
    assert_eq!(inspect(b"{", &root)["output"], "rejected_json");
    let seed=br#"{"generated_at":"2026-11-13T00:00:00Z","findings":[],"cbom":[],"dependency_sbom":[],"summary":{"total_findings":0,"by_severity":{},"by_risk":{},"scanned_files":0,"skipped_files":0}}"#;
    assert_eq!(
        inspect(seed, &root)["output"],
        "legacy_six_formats_both_languages"
    );
    fs::remove_dir_all(root).unwrap();
}
#[test]
#[ignore = "invoked per input by the bounded TASK-047 supervisor"]
fn quality047_worker() {
    let root = std::env::temp_dir().join(format!("pqc047-report-{}", std::process::id()));
    let result = std::panic::catch_unwind(|| {
        let bytes = fs::read(std::env::var("PQC_FUZZ_INPUT").unwrap()).unwrap();
        assert!(bytes.len() <= 1024 * 1024);
        inspect(&bytes, &root)
    });
    let _ = fs::remove_dir_all(root);
    match result {
        Ok(value) => println!("PQC047:{value}"),
        Err(_) => std::process::exit(42),
    }
}
