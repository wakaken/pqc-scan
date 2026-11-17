//! Real CLI classification and native provenance for the F054-01 regression.
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn non_rsa_inventory_is_retained_without_a_false_quantum_classification() {
    let temp = Scratch(std::env::temp_dir().join(format!("pqc-f054-cli-{}", std::process::id())));
    let input = temp.0.join("input");
    let output = temp.0.join("output");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::write(input.join("Cargo.toml"), "[dependencies]\nrsa = '^0.9'\njsonwebtoken = '9'\nuniversal-hash = '^0.6'\nrsa-helper = '1'\n").unwrap();
    std::fs::write(input.join("Cargo.lock"), "version = 4\n[[package]]\nname = 'rsa'\nversion = '0.9.8'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n[[package]]\nname = 'jsonwebtoken'\nversion = '9.3.1'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n[[package]]\nname = 'universal-hash'\nversion = '0.6.1'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n[[package]]\nname = 'rsa-helper'\nversion = '1.0.0'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n").unwrap();
    let components: Vec<Value> = ["rsa", "jsonwebtoken", "universal-hash", "rsa-helper"].iter().map(|name|json!({"type":"library","name":name,"version":"0.6.1","purl":format!("pkg:cargo/{name}@0.6.1")})).collect();
    std::fs::write(
        input.join("bom.cdx.json"),
        json!({"bomFormat":"CycloneDX","specVersion":"1.7","components":components}).to_string(),
    )
    .unwrap();
    let packages: Vec<Value> = ["rsa", "jsonwebtoken", "universal-hash", "rsa-helper"].iter().map(|name|json!({"name":name,"versionInfo":"0.6.1","externalRefs":[{"referenceType":"purl","referenceLocator":format!("pkg:cargo/{name}@0.6.1")}]})).collect();
    std::fs::write(
        input.join("sbom.spdx.json"),
        json!({"spdxVersion":"SPDX-2.3","packages":packages}).to_string(),
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(&input)
        .args([
            "--format",
            "all",
            "--inventory-json",
            "--policy-profile",
            "migration-review-v1",
            "--threads",
            "1",
            "--rules-dir",
        ])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
        .arg("--out-dir")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let read = |file: &str| -> Value {
        serde_json::from_slice(&std::fs::read(output.join(file)).unwrap()).unwrap()
    };
    let report = read("report.json");
    let hits: Vec<_> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["rule_id"] == "DEP_RUST_RSA_CRATE")
        .collect();
    assert_eq!(hits.len(), 8);
    for hit in hits {
        assert_eq!(hit["risk"], "quantum-vulnerable");
        assert_eq!(hit["severity"], "medium");
        let purl = hit["evidence"]["metadata"]["dep_purl"].as_str().unwrap();
        assert!(
            purl == "pkg:cargo/rsa"
                || purl.starts_with("pkg:cargo/rsa@")
                || purl == "pkg:cargo/jsonwebtoken"
                || purl.starts_with("pkg:cargo/jsonwebtoken@"),
            "{purl}"
        );
    }
    let native = read("scan.inventory.v1.json");
    let assets: Vec<_> = native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "dependency" && a["attributes"]["name"] == "universal-hash")
        .collect();
    assert!(!assets.is_empty(), "dependency inventory was lost");
    assert!(assets
        .iter()
        .any(|a| a["attributes"]["declared_constraint"] == "^0.6"));
    assert!(assets
        .iter()
        .any(|a| a["attributes"]["resolved_version"] == "0.6.1"));
    assert!(assets
        .iter()
        .any(|a| a["attributes"]["reported_version"] == "0.6.1"));
    for observation in native["observations"].as_array().unwrap() {
        assert_ne!(observation["usage"], "executed");
        assert_ne!(observation["usage"], "active");
        assert_ne!(observation["assurance"], "cryptographically_verified");
    }
    let assessment = read("scan.assessment.v1.json");
    let ids: Vec<_> = assets.iter().filter_map(|a| a["id"].as_str()).collect();
    assert!(!assessment["assessments"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| ids.contains(&a["asset_id"].as_str().unwrap_or("")) && a["decision"] == "pass"));
    for file in [
        "report.html",
        "report.md",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
    ] {
        assert!(output.join(file).is_file());
    }
}
