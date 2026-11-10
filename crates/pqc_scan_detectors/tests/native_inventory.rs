use pqc_scan_core::{scan_repository_detailed, scan_repository_native, NativeOptions, ScanConfig};
use pqc_scan_detectors::default_detectors;
use pqc_scan_rules::RuleSet;
use std::path::{Path, PathBuf};
struct Fixture(PathBuf);
impl Fixture {
    fn new(tag: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("pqc025-inventory-{}-{tag}", std::process::id()));
        std::fs::create_dir_all(root.join("input")).unwrap();
        std::fs::create_dir_all(root.join("rules")).unwrap();
        std::fs::write(root.join("rules/test.yml"),r#"[{"id":"LEX","kind":"regex","category":"test","severity":"low","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"ML-KEM-768","scope":"code"}]"#).unwrap();
        Self(root)
    }
    fn rules(&self) -> RuleSet {
        RuleSet::load_from_dir(&self.0.join("rules")).unwrap()
    }
    fn config(&self) -> ScanConfig {
        ScanConfig {
            root: self.0.join("input"),
            threads: 1,
            ..Default::default()
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn public(name: &str) -> Vec<u8> {
    std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/certificates")
            .join(name),
    )
    .unwrap()
}
#[test]
fn parse_only_public_certificates_share_spki_without_replacing_legacy_or_claiming_trust() {
    let f = Fixture::new("public");
    std::fs::write(f.0.join("input/a.der"), public("rsa2048.der")).unwrap();
    std::fs::write(f.0.join("input/b.der"), public("rsa2048-second.der")).unwrap();
    let detectors = default_detectors();
    let old = scan_repository_detailed(&f.config(), &f.rules(), &detectors).unwrap();
    let native = scan_repository_native(
        &f.config(),
        &f.rules(),
        &detectors,
        &NativeOptions::new(Some("test")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(old.coverage()).unwrap(),
        serde_json::to_value(native.coverage()).unwrap()
    );
    assert!(native.legacy().findings.is_empty());
    let n = serde_json::to_value(native.native().unwrap()).unwrap();
    let assets = n["assets"].as_array().unwrap();
    assert_eq!(
        assets.iter().filter(|a| a["kind"] == "certificate").count(),
        2
    );
    assert_eq!(
        assets.iter().filter(|a| a["kind"] == "public_key").count(),
        1
    );
    assert_eq!(
        n["unassociated_observation_ids"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        n["legacy_loss_records"][0]["reason"],
        "certificate_parse_without_rule_finding"
    );
}
#[test]
fn trailing_or_malformed_neighbor_is_visible_even_without_certificate_rules() {
    let f = Fixture::new("partial");
    let mut der = public("rsa2048.der");
    der.extend_from_slice(b"TRAILING_CANARY");
    std::fs::write(f.0.join("input/partial.der"), der).unwrap();
    let mut pem = pem::encode(&pem::Pem::new("CERTIFICATE", public("rsa2048.der"))).into_bytes();
    pem.extend_from_slice(
        b"\n-----BEGIN CERTIFICATE-----\nMALFORMED_CANARY\n-----END CERTIFICATE-----\n",
    );
    std::fs::write(f.0.join("input/bundle.pem"), pem).unwrap();
    let detectors = default_detectors();
    let old = scan_repository_detailed(&f.config(), &f.rules(), &detectors).unwrap();
    let doc = scan_repository_native(
        &f.config(),
        &f.rules(),
        &detectors,
        &NativeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(old.coverage()).unwrap(),
        serde_json::to_value(doc.coverage()).unwrap()
    );
    let n = serde_json::to_value(doc.native().unwrap()).unwrap();
    assert!(n["observations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|o| o["evidence"]["capture_status"] == "partial"
            || o["evidence"]["capture_status"] == "unsupported"));
    for path in ["partial.der", "bundle.pem"] {
        let observations: Vec<_> = n["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|o| o["target"]["path"] == path)
            .collect();
        assert!(
            observations.iter().any(
                |o| o["assurance"] == "parsed" && o["evidence"]["capture_status"] == "complete"
            ),
            "good object retained in {path}"
        );
        assert!(
            observations
                .iter()
                .any(|o| o["evidence"]["capture_status"] == "partial"
                    || o["evidence"]["capture_status"] == "unsupported"),
            "container limitation retained in {path}"
        );
    }
    assert!(!serde_json::to_string(&n).unwrap().contains("CANARY"));
}
#[test]
fn exact_algorithm_is_classified_at_the_source_match_before_legacy_preview_masking() {
    let f = Fixture::new("premask");
    std::fs::write(f.0.join("rules/test.yml"),r#"[{"id":"AST","kind":"tree_sitter","category":"test","severity":"low","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"generateKeyPairSync\\('ml-kem-768'\\)","scope":"javascript"}]"#).unwrap();
    std::fs::write(
        f.0.join("input/app.js"),
        "generateKeyPairSync('ml-kem-768');\n",
    )
    .unwrap();
    let doc = scan_repository_native(
        &f.config(),
        &f.rules(),
        &default_detectors(),
        &NativeOptions::default(),
    )
    .unwrap();
    assert_eq!(doc.legacy().findings.len(), 1);
    assert!(!doc.legacy().findings[0]
        .evidence
        .r#match
        .contains("ml-kem-768"));
    let n = serde_json::to_value(doc.native().unwrap()).unwrap();
    assert!(n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["kind"] == "algorithm" && a["attributes"]["identifier"] == "ML-KEM-768"));
}
