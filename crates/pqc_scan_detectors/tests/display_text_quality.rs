//! Classification quality against explicit labels, including UTF-8 locations.
use pqc_scan_core::{scan_repository_detailed, ScanConfig};
use pqc_scan_detectors::default_detectors;
use pqc_scan_rules::RuleSet;
use std::path::PathBuf;
#[test]
fn labelled_seven_language_corpus_preserves_references_and_configuration_candidates() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/display-text/cases.json")).unwrap();
    let root = std::env::temp_dir().join(format!("pqc-display-quality-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        std::fs::write(
            root.join(format!(
                "{}.{}",
                case["id"].as_str().unwrap(),
                case["extension"].as_str().unwrap()
            )),
            case["source"].as_str().unwrap(),
        )
        .unwrap();
    }
    let document = scan_repository_detailed(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
    )
    .unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let file = format!(
            "{}.{}",
            case["id"].as_str().unwrap(),
            case["extension"].as_str().unwrap()
        );
        let refs: Vec<_> = document
            .legacy()
            .findings
            .iter()
            .filter(|f| f.location.file == file && f.evidence.r#match == "RS256")
            .collect();
        assert!(!refs.is_empty(), "reference was lost: {file}");
        for finding in refs {
            let display = case["expected"] == "display_text";
            assert_eq!(
                finding.severity.as_str() == "info",
                display,
                "{file}: {}",
                finding.rule_id
            );
            assert_eq!(
                finding
                    .evidence
                    .metadata
                    .get("reference_kind")
                    .is_some_and(|s| s == "display_text"),
                display,
                "{file}"
            );
            if display {
                assert!(finding.recommended_actions.is_empty(), "{file}");
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn interpolation_does_not_demote_executable_signing_arguments() {
    let root =
        std::env::temp_dir().join(format!("pqc-display-interpolation-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    for (file, text) in [
        (
            "app.py",
            "raise ValueError(f\"Invalid token {jwt.encode({}, key, algorithm='RS256')}\")",
        ),
        (
            "app.rb",
            "raise \"Invalid token #{JWT.encode(payload,key,algorithm: 'RS256')}\"",
        ),
    ] {
        std::fs::write(root.join(file), text).unwrap();
    }
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let scan = scan_repository_detailed(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
    )
    .unwrap();
    for file in ["app.py", "app.rb"] {
        let matches: Vec<_> = scan
            .legacy()
            .findings
            .iter()
            .filter(|f| f.location.file == file && f.evidence.r#match == "RS256")
            .collect();
        assert!(!matches.is_empty(), "{file}");
        assert!(
            matches.iter().all(|f| f.severity.as_str() != "info"
                && !f.evidence.metadata.contains_key("reference_kind")),
            "{file}"
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}
