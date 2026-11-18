//! Pre-observation envelope validation; no technical or policy evaluator is called.
use pqc_scan_core::{inventory::NativeScan, migration::MigrationContext};
use serde_json::Value;
use std::{fs, path::PathBuf};

#[test]
fn frozen_synthetic_inputs_and_contexts_have_adopted_shapes_and_native_references() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let study = root.join("tasks/071/evidence/study-v2");
    let manifest: Value =
        serde_json::from_slice(&fs::read(study.join("manifest.json")).unwrap()).unwrap();
    let rows = manifest["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 24);
    let context_schema: Value = serde_json::from_slice(
        &fs::read(root.join("schemas/migration-context-v1.schema.json")).unwrap(),
    )
    .unwrap();
    let context_validator = jsonschema::validator_for(&context_schema).unwrap();
    for row in rows {
        let path = root.join(row["input"].as_str().unwrap());
        let bytes = fs::read(path).unwrap();
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        let format = row["format"].as_str().unwrap();
        let schema_name = match format {
            "inventory-v1" => "native-scan-v1.schema.json",
            "tls-v1" => "probe-tls-v1.schema.json",
            "ssh-offers-v1" => "probe-ssh-offers-v1.schema.json",
            _ => panic!("unselected study format"),
        };
        let schema: Value =
            serde_json::from_slice(&fs::read(root.join("schemas").join(schema_name)).unwrap())
                .unwrap();
        let validator = jsonschema::validator_for(&schema).unwrap();
        let errors: Vec<_> = validator
            .iter_errors(&value)
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{}: {errors:?}", row["id"]);
        if format == "inventory-v1" {
            let native: NativeScan = serde_json::from_slice(&bytes).unwrap();
            native.validate().unwrap();
        }
        let context = fs::read(root.join(row["context"].as_str().unwrap())).unwrap();
        let value: Value = serde_json::from_slice(&context).unwrap();
        let errors: Vec<_> = context_validator
            .iter_errors(&value)
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{} context: {errors:?}", row["id"]);
        MigrationContext::from_bytes(&context).unwrap();
    }
}
