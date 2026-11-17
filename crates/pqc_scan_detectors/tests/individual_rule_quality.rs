//! New nominal controls for previously empty/one-sided rule strata.
//! Expected facts are hand-authored and frozen before execution, not learned from hits.
use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::{RuleKind, RuleSet};
use serde_json::Value;
use std::path::Path;

#[test]
fn individually_nominated_rule_references_have_positive_and_negative_controls() {
    let rules =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap();
    let cases: Value =
        serde_json::from_str(include_str!("fixtures/quality-completion/text-cases.json")).unwrap();
    let mut disagreements = vec![];
    let mut seen = std::collections::BTreeSet::new();
    for case in cases["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(seen.insert(id));
        let detector: &dyn Detector = match rules.get(id).unwrap().kind {
            RuleKind::Regex => &RegexDetector,
            RuleKind::TreeSitter => &TreeSitterDetector,
            RuleKind::Dependency => &DependencyDetector,
            _ => panic!("certificate controls have a separate parsed-facts oracle"),
        };
        for polarity in ["positive", "negative"] {
            let file = ScannableFile::from_bytes(
                case["path"].as_str().unwrap().into(),
                case[polarity].as_str().unwrap().as_bytes().to_vec(),
            );
            let result = detector.detect_detailed(&file, &rules).unwrap();
            let hits: Vec<_> = result
                .detections()
                .iter()
                .filter(|d| d.rule_id == id)
                .collect();
            let expected = case[format!("{polarity}_expected")].as_u64().unwrap() as usize;
            let correct = hits.len() == expected && result.status() == DetectorStatus::Complete;
            println!(
                "RULE_CASE {}",
                serde_json::json!({"id":id,"polarity":polarity,"expected":expected,"observed":hits.len(),"status":format!("{:?}",result.status()),"correct":correct})
            );
            if !correct {
                disagreements.push(format!(
                    "{id}/{polarity}: expected {expected} complete, actual {} {:?}",
                    hits.len(),
                    result.status()
                ));
            }
            for hit in hits {
                assert!(hit.location.line > 0 && hit.location.column > 0, "{id}");
            }
        }
    }
    assert_eq!(seen.len(), 83);
    assert!(disagreements.is_empty(), "{}", disagreements.join("\n"));
}

#[test]
fn java_rsa_parameter_constructors_are_inspected_without_broadening_other_rule_candidates() {
    let rules =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap();
    for size in [1024, 2048, 3072] {
        let text=format!("class Use {{ Object spec = new RSAKeyGenParameterSpec({size}, RSAKeyGenParameterSpec.F4); }}");
        let file = ScannableFile::from_bytes("Use.java".into(), text.as_bytes().to_vec());
        let result = TreeSitterDetector.detect_observed(&file, &rules).unwrap();
        assert_eq!(result.status(), DetectorStatus::Complete);
        assert_eq!(
            result
                .detections()
                .iter()
                .filter(|d| d.rule_id == "TS_RSA_KEY_SIZE_LITERAL")
                .count(),
            1
        );
        assert!(result
            .detections()
            .iter()
            .all(|d| d.rule_id == "TS_RSA_KEY_SIZE_LITERAL"));
    }
    for source in [
        "class Use { Object spec = new ECGenParameterSpec(\"secp256r1\"); }",
        "class Use { Object spec = new RSAKeyGenParameterSpec(4096, RSAKeyGenParameterSpec.F4); }",
        "class Use { // new RSAKeyGenParameterSpec(2048, 1);\n }",
    ] {
        let file = ScannableFile::from_bytes("Use.java".into(), source.as_bytes().to_vec());
        assert!(TreeSitterDetector
            .detect(&file, &rules)
            .unwrap()
            .iter()
            .all(|d| d.rule_id != "TS_RSA_KEY_SIZE_LITERAL"));
    }
}

#[test]
fn constructor_inspection_cannot_displace_prior_candidates_or_match_byte_work() {
    use pqc_scan_detectors::DetectorBudget;
    let rules =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap();
    let root = std::env::temp_dir().join(format!("pqc-constructor-budget-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let old: Vec<_> = rules
        .all()
        .iter()
        .filter(|r| r.id != "TS_RSA_KEY_SIZE_LITERAL")
        .collect();
    std::fs::write(root.join("rules.yml"), serde_json::to_vec(&old).unwrap()).unwrap();
    let without = RuleSet::load_from_dir(&root).unwrap();
    for text in [
        "class Use { void f() { String alg = \"RS256\"; new Object(); } }".to_string(),
        format!(
            "class Use {{ void f() {{ String alg = \"RS256\"; {} }} }}",
            "new RSAKeyGenParameterSpec(2048, RSAKeyGenParameterSpec.F4);".repeat(25)
        ),
    ] {
        let file = ScannableFile::from_bytes("Use.java".into(), text.into_bytes());
        for candidates in [2, 8, 128] {
            for bytes in [16, 512, 256 * 1024 * 1024] {
                let detector = TreeSitterDetector::with_budget(DetectorBudget {
                    max_ast_candidates: candidates,
                    max_ast_match_bytes: bytes,
                    ..Default::default()
                });
                let before = detector.detect_detailed(&file, &without).unwrap();
                let after = detector.detect_detailed(&file, &rules).unwrap();
                let comparable = |v: &pqc_scan_core::DetectorOutcome| {
                    v.detections()
                        .iter()
                        .filter(|d| d.rule_id != "TS_RSA_KEY_SIZE_LITERAL")
                        .map(|d| {
                            (
                                d.rule_id.clone(),
                                d.location.file.clone(),
                                d.location.line,
                                d.location.column,
                                serde_json::to_value(&d.evidence).unwrap(),
                            )
                        })
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    comparable(&before),
                    comparable(&after),
                    "candidate cap {candidates}, byte cap {bytes}"
                );
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
