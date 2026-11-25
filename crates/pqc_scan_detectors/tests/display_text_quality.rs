//! Classification quality against explicit labels, including UTF-8 locations.
use pqc_scan_core::{scan_repository_detailed, ScanConfig};
use pqc_scan_detectors::default_detectors;
use pqc_scan_rules::RuleSet;
use std::path::PathBuf;
#[test]
fn recorded_protocol_text_and_comments_do_not_turn_into_actionable_tls_usage() {
    let root = std::env::temp_dir().join(format!("pqc-recorded-protocol-{}", std::process::id()));
    std::fs::create_dir_all(root.join("tests")).unwrap();
    let suite = "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384";
    for (name, source) in [
        (
            "record.cs",
            format!("class C {{string[] headers={{\"Received: from relay with {suite}\"}};}}"),
        ),
        (
            "comment.cs",
            format!("/// Registry entry {suite}\nclass C {{}}"),
        ),
        (
            "config.cs",
            format!("class C {{void F(){{ ConfigureTls(\"{suite}\");}}}}"),
        ),
        (
            "tests/crypto.cs",
            "using System.Security.Cryptography; class C{void F(){RSA.Create();}}".into(),
        ),
    ] {
        std::fs::write(root.join(name), source).unwrap();
    }
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let doc = scan_repository_detailed(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
    )
    .unwrap();
    for name in ["record.cs", "comment.cs", "config.cs"] {
        let findings: Vec<_> = doc
            .legacy()
            .findings
            .iter()
            .filter(|f| f.location.file == name && f.rule_id.starts_with("TLS_"))
            .collect();
        assert!(!findings.is_empty(), "retain the lexical reference: {name}");
        for finding in findings {
            assert_eq!(
                finding.severity.as_str() == "info",
                name != "config.cs",
                "{name}"
            );
            assert_eq!(
                finding
                    .evidence
                    .metadata
                    .get("reference_kind")
                    .map(String::as_str),
                if name == "config.cs" {
                    None
                } else {
                    Some("display_text")
                }
            );
            if name != "config.cs" {
                assert!(finding.recommended_actions.is_empty());
            }
        }
    }
    assert!(doc
        .legacy()
        .findings
        .iter()
        .any(|f| f.location.file == "tests/crypto.cs"
            && f.rule_id == "CSHARP_CRYPTO_API"
            && f.severity.as_str() != "info"));
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn suite_registry_declarations_do_not_demote_an_independent_configuration_value() {
    let root = std::env::temp_dir().join(format!("pqc-suite-registry-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("registry.cs"),"class C {\npublic const int TLS_ECDHE_RSA_WITH_NULL_SHA = 123;\nvoid F(){ConfigureTls(\"TLS_ECDHE_RSA_WITH_NULL_SHA\");}\n}").unwrap();
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let doc = scan_repository_detailed(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
    )
    .unwrap();
    for line in [2, 3] {
        let rows: Vec<_> = doc
            .legacy()
            .findings
            .iter()
            .filter(|f| f.location.line == line && f.rule_id.starts_with("TLS_"))
            .collect();
        assert!(!rows.is_empty());
        for row in rows {
            assert_eq!(row.severity.as_str() == "info", line == 2, "line{line}");
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn recorded_header_values_in_valid_regions_survive_unrelated_parse_errors_without_secrets() {
    let root = std::env::temp_dir().join(format!("pqc-header-value-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("record.cs"),"class C { string[] values={\" from relay\\r\\n\\tby mx with ESMTP (cipher=TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384); date PRIVATE085_RECEIVED_CANARY\"}; }\nclass Broken { void }\n").unwrap();
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let doc = scan_repository_detailed(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
    )
    .unwrap();
    let rows: Vec<_> = doc
        .legacy()
        .findings
        .iter()
        .filter(|f| f.rule_id.starts_with("TLS_"))
        .collect();
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|f| f.severity.as_str() == "info"));
    assert!(!serde_json::to_string(doc.legacy())
        .unwrap()
        .contains("PRIVATE085_RECEIVED_CANARY"));
    assert_eq!(doc.coverage().partial_files(), 1);
    std::fs::remove_dir_all(root).unwrap();
}

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
