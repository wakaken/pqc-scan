use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
fn documented_examples_complete_supported_inspection_and_keep_all_requested_reports() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::env::temp_dir().join(format!("pqc-format-examples-{}", std::process::id()));
    assert!(!out.exists());
    let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(root.join("examples"))
        .args([
            "--format",
            "all",
            "--lang",
            "ja",
            "--diagnostics",
            "--require-complete",
            "--inventory-json",
            "--repository-scope",
            "pqc-scan/examples",
            "--policy-profile",
            "migration-review-v1",
            "--rules-dir",
        ])
        .arg(root.join("rules"))
        .arg("--out-dir")
        .arg(&out)
        .output()
        .unwrap();
    let diagnostics: Value =
        serde_json::from_slice(&fs::read(out.join("diagnostics.json")).unwrap()).unwrap();
    let files = fs::read_dir(&out).unwrap().count();
    let schemas = [
        (
            "scan.inventory.v1.json",
            include_str!("../../../schemas/native-scan-v1.schema.json"),
            jsonschema::Draft::Draft7,
        ),
        (
            "scan.assessment.v1.json",
            include_str!("../../../schemas/assessment-v1.schema.json"),
            jsonschema::Draft::Draft7,
        ),
        (
            "report.sarif",
            include_str!("../../pqc_scan_report/tests/schema/sarif-schema-2.1.0.json"),
            jsonschema::Draft::Draft4,
        ),
    ];
    let payloads: Vec<Value> = schemas
        .iter()
        .map(|(file, _, _)| serde_json::from_slice(&fs::read(out.join(file)).unwrap()).unwrap())
        .collect();
    fs::remove_dir_all(&out).unwrap();
    for ((file, schema, draft), payload) in schemas.into_iter().zip(payloads) {
        let schema: Value = serde_json::from_str(schema).unwrap();
        let validator = jsonschema::options()
            .with_draft(draft)
            .build(&schema)
            .unwrap();
        assert!(validator.is_valid(&payload), "invalid {file}");
    }
    assert!(
        result.status.success(),
        "incomplete supported example inspection: {}",
        diagnostics["coverage"]
    );
    assert_eq!(files, 9);
    let c = &diagnostics["coverage"];
    assert_eq!(c["status"], "complete_within_scope");
    assert_eq!(c["selected_files"], 55);
    assert_eq!(c["analyzed_files"], 55);
    for key in [
        "read_failed_files",
        "partial_files",
        "analysis_failed_files",
        "unsupported_files",
        "unknown_files",
    ] {
        assert_eq!(c[key], 0, "{key}");
    }
}
