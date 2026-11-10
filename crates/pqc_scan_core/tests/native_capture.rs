use pqc_scan_core::{
    scan_repository_detailed, scan_repository_native, Detection, Detector, Evidence, Location,
    NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_rules::RuleSet;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
struct Fixture(PathBuf);
impl Fixture {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!("pqc025-core-{}-{tag}", std::process::id()));
        std::fs::create_dir_all(path.join("input")).unwrap();
        std::fs::create_dir_all(path.join("rules")).unwrap();
        std::fs::write(path.join("input/app.txt"), "line\n".repeat(12)).unwrap();
        std::fs::write(path.join("rules/test.yml"),serde_json::json!([{"id":"TEST","kind":"regex","category":"test","severity":"medium","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"TOKEN","scope":"code"}]).to_string()).unwrap();
        Self(path)
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
struct OldConsumer(Arc<AtomicUsize>);
impl Detector for OldConsumer {
    fn name(&self) -> &'static str {
        "old_consumer"
    }
    fn detect(&self, file: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok([(1, 11), (11, 1)]
            .into_iter()
            .map(|(line, column)| Detection {
                rule_id: "TEST".into(),
                location: Location {
                    file: file.path.to_string_lossy().into(),
                    line,
                    column,
                },
                evidence: Evidence {
                    r#type: "certificate".into(),
                    r#match: "RAW_BODY_CANARY_025".into(),
                    snippet_preview: "RAW_PREVIEW_CANARY_025".into(),
                    metadata: BTreeMap::from([
                        ("certificate_sha256".into(), "a".repeat(64)),
                        ("spki_sha256".into(), "b".repeat(64)),
                        ("untrusted".into(), "RAW_METADATA_CANARY_025".into()),
                    ]),
                },
            })
            .collect())
    }
}
#[test]
fn old_consumer_runs_once_and_native_preserves_real_legacy_hash_collision_indices() {
    let fixture = Fixture::new("adapter");
    let count = Arc::new(AtomicUsize::new(0));
    let detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(OldConsumer(count.clone()))];
    let old = scan_repository_detailed(&fixture.config(), &fixture.rules(), &detectors).unwrap();
    assert_eq!(count.swap(0, Ordering::SeqCst), 1);
    assert!(old.native().is_none());
    let doc = scan_repository_native(
        &fixture.config(),
        &fixture.rules(),
        &detectors,
        &NativeOptions::new(Some("test/project")).unwrap(),
    )
    .unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    let mut a = serde_json::to_value(old.legacy()).unwrap();
    let mut b = serde_json::to_value(doc.legacy()).unwrap();
    a.as_object_mut().unwrap().remove("generated_at");
    b.as_object_mut().unwrap().remove("generated_at");
    assert_eq!(a, b);
    let findings = &doc.legacy().findings;
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].finding_id, findings[1].finding_id);
    let n = serde_json::to_value(doc.native().unwrap()).unwrap();
    assert_eq!(n["observations"].as_array().unwrap().len(), 2);
    for o in n["observations"].as_array().unwrap() {
        assert_eq!(o["assurance"], "unknown");
        assert_eq!(o["usage"], "unknown");
        assert_eq!(o["producer"], "old_consumer");
    }
    for (i, association) in n["legacy_associations"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        assert_eq!(association["legacy_index"], i);
        assert_eq!(association["observation_ids"].as_array().unwrap().len(), 1);
    }
    let raw = serde_json::to_string(doc.native().unwrap()).unwrap();
    assert!(!raw.contains("CANARY"));
    assert!(!n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["kind"] == "certificate" || a["kind"] == "public_key"));
}

struct Spoof;
impl Detector for Spoof {
    fn name(&self) -> &'static str {
        "spoof"
    }
    fn detect(&self, file: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        Ok(vec![Detection {
            rule_id: "TEST".into(),
            location: Location {
                file: file
                    .path
                    .with_file_name("forged.txt")
                    .to_string_lossy()
                    .into(),
                line: 1,
                column: 1,
            },
            evidence: Evidence {
                r#type: "test".into(),
                r#match: "TOKEN".into(),
                snippet_preview: String::new(),
                metadata: BTreeMap::new(),
            },
        }])
    }
}
#[test]
fn a_custom_detection_cannot_attribute_native_facts_to_another_scanned_relative_path() {
    let f = Fixture::new("spoof");
    let detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(Spoof)];
    let doc = scan_repository_native(
        &f.config(),
        &f.rules(),
        &detectors,
        &NativeOptions::default(),
    )
    .unwrap();
    assert_eq!(doc.legacy().findings[0].location.file, "forged.txt");
    let n = serde_json::to_value(doc.native().unwrap()).unwrap();
    assert!(n["observations"][0]["target"]["path"].is_null());
    assert_eq!(
        n["observations"][0]["evidence"]["capture_status"],
        "target_mismatch"
    );
}

struct Lossy;
impl Detector for Lossy {
    fn name(&self) -> &'static str {
        "lossy"
    }
    fn detect(&self, file: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        let mut hits = OldConsumer(Arc::new(AtomicUsize::new(0))).detect(
            file,
            &RuleSet::load_from_dir(&file.path.parent().unwrap().parent().unwrap().join("rules"))?,
        )?;
        hits[0].location.line = 1;
        hits[0].location.column = 2;
        hits[0].evidence.r#match = "3:4".into();
        hits[1].location.file.push_str(":1");
        hits[1].location.line = 2;
        hits[1].location.column = 3;
        hits[1].evidence.r#match = "4".into();
        let mut unknown = hits[0].clone();
        unknown.rule_id = "UNKNOWN_RULE".into();
        hits.push(unknown);
        let mut inventory = hits[0].clone();
        inventory.rule_id = "__DEPENDENCY_INVENTORY__".into();
        hits.push(inventory);
        Ok(hits)
    }
}
#[test]
fn native_loss_ledger_separates_unknown_rule_inventory_and_legacy_delimiter_collapse() {
    let f = Fixture::new("losses");
    let doc = scan_repository_native(
        &f.config(),
        &f.rules(),
        &[Arc::new(Lossy)],
        &NativeOptions::default(),
    )
    .unwrap();
    assert_eq!(doc.legacy().findings.len(), 1);
    let n = serde_json::to_value(doc.native().unwrap()).unwrap();
    let reasons: Vec<_> = n["legacy_loss_records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["reason"].as_str().unwrap())
        .collect();
    assert_eq!(reasons.len(), 3);
    for reason in [
        "unknown_rule_dropped_by_legacy_normalization",
        "dependency_inventory_not_a_legacy_finding",
        "legacy_normalization_collapsed_distinct_binding",
    ] {
        assert!(reasons.contains(&reason));
    }
    doc.native().unwrap().validate().unwrap();
}
