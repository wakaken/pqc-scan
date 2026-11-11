//! Offline format validation using the existing pinned development validator.
#[path = "../src/diff_command.rs"]
mod diff_command;
use pqc_scan_core::{diff::compare, scan_repository_native, NativeOptions, ScanConfig};
use pqc_scan_rules::RuleSet;
use serde_json::{json, Value};

#[test]
fn binary_diff_compares_real_inventory_outputs_and_requires_scope() {
    use std::process::Command;
    let binary = env!("CARGO_BIN_EXE_pqc-scan");
    let root = std::env::temp_dir().join(format!("pqc035-binary-{}", std::process::id()));
    std::fs::create_dir_all(root.join("input")).unwrap();
    let checkout = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let source = "from Crypto.PublicKey import RSA\nkey = RSA.generate(2048)\n";
    let source_path = root.join("input/app.py");
    std::fs::write(&source_path, source).unwrap();
    for (name, text) in [
        ("before", source.to_owned()),
        ("after", format!("\n{source}")),
    ] {
        std::fs::write(&source_path, text).unwrap();
        let output = Command::new(binary)
            .arg("scan")
            .arg(root.join("input"))
            .args([
                "--inventory-json",
                "--repository-scope",
                "test/project",
                "--format",
                "json",
                "--out-dir",
            ])
            .arg(root.join(name))
            .arg("--rules-dir")
            .arg(checkout.join("rules"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let before = root.join("before/scan.inventory.v1.json");
    let after = root.join("after/scan.inventory.v1.json");
    let output_dir = root.join("comparison");
    let output = Command::new(binary)
        .arg("diff")
        .arg(&before)
        .arg(&after)
        .args(["--repository-scope", "test/project", "--out-dir"])
        .arg(&output_dir)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value =
        serde_json::from_slice(&std::fs::read(output_dir.join("scan.diff.v1.json")).unwrap())
            .unwrap();
    let schema: Value =
        serde_json::from_str(include_str!("../../../schemas/scan-diff-v1.schema.json")).unwrap();
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .build(&schema)
        .unwrap();
    assert!(validator.is_valid(&value));
    assert!(!value["occurrences"].as_array().unwrap().is_empty());
    assert!(value["occurrences"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| row["status"] == "unchanged"));
    assert!(output_dir.join("scan.diff.v1.md").is_file());
    assert!(!output_dir.join("report.json").exists());

    let missing_scope = Command::new(binary)
        .arg("diff")
        .arg(&before)
        .arg(&after)
        .arg("--out-dir")
        .arg(root.join("missing-scope"))
        .output()
        .unwrap();
    assert_eq!(missing_scope.status.code(), Some(1));
    assert!(!root.join("missing-scope").exists());
    let wrong_scope = Command::new(binary)
        .arg("diff")
        .arg(&before)
        .arg(&after)
        .args(["--repository-scope", "another/project", "--out-dir"])
        .arg(root.join("wrong-scope"))
        .output()
        .unwrap();
    assert_eq!(wrong_scope.status.code(), Some(1));
    assert!(!root.join("wrong-scope").exists());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn comparison_output_and_semantic_golden_validate_offline() {
    let schema: Value =
        serde_json::from_str(include_str!("../../../schemas/scan-diff-v1.schema.json")).unwrap();
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .build(&schema)
        .unwrap();
    let golden: Value = serde_json::from_str(include_str!(
        "../../pqc_scan_core/tests/fixtures/diff/unchanged.v1.json"
    ))
    .unwrap();
    assert!(validator.is_valid(&golden));

    let root = std::env::temp_dir().join(format!("pqc035-schema-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let checkout = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rules = RuleSet::load_from_dir(&checkout.join("rules")).unwrap();
    let document = scan_repository_native(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &[],
        &NativeOptions::new(Some("test/project")).unwrap(),
    )
    .unwrap();
    let inventory = document.native().unwrap();
    let output =
        serde_json::to_value(compare(inventory, inventory, "test/project").unwrap()).unwrap();
    assert!(validator.is_valid(&output));
    assert_eq!(output["assets"], json!([]));
    assert_eq!(output["occurrences"], json!([]));
    assert!(output["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v == "absence_is_not_risk_resolution"));
    std::fs::remove_dir_all(root).unwrap();

    let mut bad = golden.clone();
    bad["occurrences"][0]["status"] = json!("risk_resolved");
    assert!(!validator.is_valid(&bad));
    bad = golden;
    bad.as_object_mut().unwrap().remove("limitations");
    assert!(!validator.is_valid(&bad));
}
