use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{
    regex_detector::RegexDetector, tree_sitter_detector::TreeSitterDetector, DetectorBudget,
};
use pqc_scan_rules::RuleSet;
use std::path::{Path, PathBuf};
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .unwrap()
}
#[test]
fn source_facts_obey_the_released_default_and_explicit_hit_budgets() {
    let f = ScannableFile::from_bytes(
        PathBuf::from("app.js"),
        "generateKeyPairSync('ml-kem-768');\n"
            .repeat(40)
            .into_bytes(),
    );
    let full = TreeSitterDetector.detect_detailed(&f, &rules()).unwrap();
    assert_eq!(full.status(), DetectorStatus::Complete);
    assert_eq!(
        full.detections()
            .iter()
            .filter(|d| d.rule_id == "PQC_NODE_API")
            .count(),
        40
    );
    let limited = TreeSitterDetector::with_budget(DetectorBudget {
        max_hits_per_rule: 2,
        ..Default::default()
    })
    .detect_detailed(&f, &rules())
    .unwrap();
    assert_eq!(limited.status(), DetectorStatus::Truncated);
    assert_eq!(
        limited
            .detections()
            .iter()
            .filter(|d| d.rule_id == "PQC_NODE_API")
            .count(),
        2
    );
    let f = ScannableFile::from_bytes(
        PathBuf::from("app.js"),
        "createSign('RSA-SHA256');\n".repeat(40).into_bytes(),
    );
    let full = RegexDetector.detect_detailed(&f, &rules()).unwrap();
    assert_eq!(
        full.detections()
            .iter()
            .filter(|d| d.rule_id == "API_NODE_CRYPTO_SIGN_RSA")
            .count(),
        40
    );
    let limited = RegexDetector::with_budget(DetectorBudget {
        max_hits_per_rule: 2,
        ..Default::default()
    })
    .detect_detailed(&f, &rules())
    .unwrap();
    assert_eq!(limited.status(), DetectorStatus::Truncated);
    assert_eq!(
        limited
            .detections()
            .iter()
            .filter(|d| d.rule_id == "API_NODE_CRYPTO_SIGN_RSA")
            .count(),
        2
    );
}

#[test]
fn source_fact_paths_respect_small_work_budgets() {
    let rules = rules();
    for budget in [
        DetectorBudget {
            max_ast_nodes: 1,
            ..Default::default()
        },
        DetectorBudget {
            max_ast_candidates: 0,
            ..Default::default()
        },
        DetectorBudget {
            max_match_candidates: 0,
            ..Default::default()
        },
        DetectorBudget {
            max_ast_match_bytes: 0,
            ..Default::default()
        },
    ] {
        for (text, tree) in [
            ("generateKeyPairSync('ml-kem-768');", true),
            ("createSign('RSA-SHA256');", false),
        ] {
            let file = ScannableFile::from_bytes(PathBuf::from("app.js"), text.as_bytes().to_vec());
            let outcome = if tree {
                TreeSitterDetector::with_budget(budget)
                    .detect_detailed(&file, &rules)
                    .unwrap()
            } else {
                RegexDetector::with_budget(budget)
                    .detect_detailed(&file, &rules)
                    .unwrap()
            };
            assert_eq!(outcome.status(), DetectorStatus::Truncated, "{budget:?}");
            assert!(outcome.detections().is_empty(), "{budget:?}");
        }
    }
}

#[test]
fn both_source_paths_share_compatible_syntax_and_original_spans() {
    for (name, prefix) in [
        (
            "view.tsx",
            "const 日 = <A href='/x?first-key=1&second-key=2' />;\n",
        ),
        ("types.ts", "type Shape<> = { v: true };\n"),
        ("module.mjs", "import defer * as ns from './source.js';\n"),
    ] {
        let text = format!("{prefix}generateKeyPairSync('ml-kem-768'); createSign('RSA-SHA256');");
        let file = ScannableFile::from_bytes(PathBuf::from(name), text.into_bytes());
        for (outcome, id) in [
            (
                TreeSitterDetector.detect_detailed(&file, &rules()).unwrap(),
                "PQC_NODE_API",
            ),
            (
                RegexDetector.detect_detailed(&file, &rules()).unwrap(),
                "API_NODE_CRYPTO_SIGN_RSA",
            ),
        ] {
            assert_eq!(outcome.status(), DetectorStatus::Complete, "{name} {id}");
            let hits: Vec<_> = outcome
                .detections()
                .iter()
                .filter(|d| d.rule_id == id)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].location.line, 2);
        }
    }
}

#[test]
fn source_facts_allow_explicitly_larger_hit_and_span_budgets() {
    let budget = DetectorBudget {
        max_hits_per_rule: 5000,
        ..Default::default()
    };
    let rules = rules();
    for (text, tree, id) in [
        ("generateKeyPairSync('ml-kem-768');", true, "PQC_NODE_API"),
        (
            "createSign('RSA-SHA256');",
            false,
            "API_NODE_CRYPTO_SIGN_RSA",
        ),
    ] {
        for source in [
            text.repeat(4100),
            text.replace('(', &format!("({}", " ".repeat(9000))),
        ] {
            let expected = if source.len() > 100_000 { 4100 } else { 1 };
            let file = ScannableFile::from_bytes(PathBuf::from("app.js"), source.into_bytes());
            let outcome = if tree {
                TreeSitterDetector::with_budget(budget)
                    .detect_detailed(&file, &rules)
                    .unwrap()
            } else {
                RegexDetector::with_budget(budget)
                    .detect_detailed(&file, &rules)
                    .unwrap()
            };
            assert_eq!(outcome.status(), DetectorStatus::Complete, "{id}");
            assert_eq!(
                outcome
                    .detections()
                    .iter()
                    .filter(|d| d.rule_id == id)
                    .count(),
                expected,
                "{id}"
            );
        }
    }
}

#[test]
fn java_python_and_rust_fact_paths_honor_released_work_budgets() {
    let rules = rules();
    let budget = DetectorBudget {
        max_hits_per_rule: 20000,
        ..Default::default()
    };
    let java = format!("Signature.getInstance({}\"ML-DSA-44\");", " ".repeat(3000));
    let python = format!(
        "from cryptography.hazmat.primitives.asymmetric import mlkem\n{}",
        "mlkem.MLKEM768PrivateKey.generate()\n".repeat(4100)
    );
    let rust = format!(
        "use ml_kem::*;\n{}",
        "MlKem768::generate(&mut rng);\n".repeat(4100)
    );
    for (name, text, id, last_line) in [
        ("Example.java", java, "PQC_JAVA_API", 1),
        ("app.py", python, "PQC_PYTHON_API", 4101),
        ("app.rs", rust, "PQC_RUST_API", 4101),
    ] {
        let file = ScannableFile::from_bytes(PathBuf::from(name), text.into_bytes());
        let outcome = TreeSitterDetector::with_budget(budget)
            .detect_detailed(&file, &rules)
            .unwrap();
        assert_eq!(outcome.status(), DetectorStatus::Complete, "{id}");
        assert!(
            outcome
                .detections()
                .iter()
                .any(|d| d.rule_id == id && d.location.line == last_line),
            "{id} final occurrence"
        );
    }
}
