use pqc_scan_rules::RuleSet;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
#[test]
fn every_builtin_has_bound_review_axes_sources_and_honest_pattern_controls() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let metadata: Value = serde_json::from_slice(
        &fs::read(root.join("docs/quality/RULE-QUALITY-METADATA.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(metadata["schema_version"], 1);
    assert_eq!(metadata["fixture_layer"], "compiled_pattern_target_only");
    for source in metadata["source_files"].as_array().unwrap() {
        let bytes = fs::read(root.join(source["path"].as_str().unwrap())).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(bytes)),
            source["sha256"].as_str().unwrap()
        );
    }
    let rules = RuleSet::load_from_dir(&root.join("rules/default")).unwrap();
    assert_eq!(rules.len(), 122);
    let rows = metadata["rules"].as_array().unwrap();
    assert_eq!(rows.len(), rules.len());
    let mut seen = BTreeSet::new();
    for row in rows {
        let id = row["id"].as_str().unwrap();
        assert!(seen.insert(id));
        let r = rules.get(id).unwrap();
        let mut snapshot = row["rule_snapshot"].clone();
        // Confidence has the existing public f32 domain; other fields compare exactly.
        assert_eq!(
            snapshot["confidence"].as_f64().unwrap() as f32,
            r.confidence
        );
        let mut actual = serde_json::to_value(r).unwrap();
        snapshot.as_object_mut().unwrap().remove("confidence");
        actual.as_object_mut().unwrap().remove("confidence");
        assert_eq!(snapshot, actual, "{id} snapshot");
        for field in ["purpose", "quantum", "classical", "review_status"] {
            assert!(!row[field].as_str().unwrap().is_empty(), "{id} {field}");
        }
        assert!(!row["method_limits"].as_array().unwrap().is_empty());
        for source in row["reference_ids"].as_array().unwrap() {
            let reference = &metadata["references"][source.as_str().unwrap()];
            assert!(!reference["edition"].as_str().unwrap().is_empty());
            assert!(!reference["scope_limit"].as_str().unwrap().is_empty());
        }
        assert!(!row["reference_ids"].as_array().unwrap().is_empty());
        let examples = row["fixtures"].as_array().unwrap();
        assert!(examples.iter().any(|e| e["desired_positive"] == true));
        assert!(examples.iter().any(|e| e["desired_positive"] == false));
        for fixture in examples {
            let actual = r
                .compiled_pattern()
                .unwrap()
                .is_match(fixture["target"].as_str().unwrap());
            assert_eq!(
                actual,
                fixture["expected_pattern_match"].as_bool().unwrap(),
                "{id}: {fixture}"
            );
            if fixture["desired_positive"] != fixture["expected_pattern_match"] {
                assert!(
                    !fixture["known_gap"].as_str().unwrap().is_empty(),
                    "{id} undocumented disagreement"
                );
            }
        }
    }
}
