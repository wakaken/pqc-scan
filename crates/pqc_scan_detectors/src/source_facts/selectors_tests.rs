//! Changed YAML selectors, not copied implementation lists, define allowed references.
use super::{context::Context, javascript, python, Facts};
use pqc_scan_rules::RuleSet;
use std::sync::atomic::{AtomicU64, Ordering};
fn inspect(language: &str, pattern: &str, text: &str) -> Facts {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "pqc043-selector-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    let id = if language == "python" {
        "PQC_PYTHON_API"
    } else {
        "PQC_NODE_API"
    };
    let raw = serde_json::json!([{"id":id,"kind":"tree_sitter","category":"test","severity":"info","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Synthetic selector control, not a supported real algorithm","pattern":pattern,"scope":language}]);
    std::fs::write(root.join("test.yml"), raw.to_string()).unwrap();
    let rules = RuleSet::load_from_dir(&root).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
    let mut parser = tree_sitter::Parser::new();
    let grammar = if language == "python" {
        tree_sitter_python::LANGUAGE
    } else {
        tree_sitter_javascript::LANGUAGE
    };
    parser.set_language(&grammar.into()).unwrap();
    let tree = parser.parse(text, None).unwrap();
    let mut ctx = Context::new(
        tree.root_node(),
        text,
        language,
        crate::DetectorBudget::default(),
    );
    let mut facts = Facts {
        partial_causes: if tree.root_node().has_error() {
            super::PartialCauses::from(super::PartialCause::ParserRecovery)
        } else {
            super::PartialCauses::default()
        },
        ..Facts::default()
    };
    if language == "python" {
        python::inspect(&mut ctx, &mut facts, &rules);
    } else {
        javascript::inspect(&mut ctx, &mut facts, &rules);
    }
    facts
}
fn count(facts: &Facts, id: &str) -> usize {
    facts.hits.get(id).map_or(0, Vec::len)
}
#[test]
fn jwt_option_analysis_charges_argument_bytes_and_keeps_truncation_visible() {
    let rules = RuleSet::load_from_dir(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let text = format!(
        "import * as jwt from 'jsonwebtoken'; jwt.sign({{data:'{}'}}, key, {{algorithm:'RS256'}});",
        "x".repeat(1024)
    );
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(&text, None).unwrap();
    let full = super::inspect(
        tree.root_node(),
        &text,
        "javascript",
        &rules,
        crate::DetectorBudget::default(),
    );
    assert_eq!(count(&full, "TS_NODE_JOSE_USAGE"), 1);
    assert!(!full.truncated);
    let limited = super::inspect(
        tree.root_node(),
        &text,
        "javascript",
        &rules,
        crate::DetectorBudget {
            max_ast_match_bytes: 256,
            ..Default::default()
        },
    );
    assert!(limited.truncated);
    assert_eq!(count(&limited, "TS_NODE_JOSE_USAGE"), 0);
}
#[test]
fn node_alias_and_literal_loop_follow_the_supplied_selector() {
    // This hypothetical name deliberately appears only in the test YAML.
    let pattern = r#"\bgenerateKeyPairSync\(\s*['"]future-kem['"]"#;
    let facts=inspect("javascript",pattern,"import {generateKeyPairSync as build} from 'node:crypto'; build('future-kem'); for(const choice of ['future-kem','ordinary']) { build(choice); }");
    assert_eq!(count(&facts, "PQC_NODE_API"), 2);
    assert_eq!(
        count(
            &inspect("javascript", pattern, "generateKeyPairSync('ml-kem-768');"),
            "PQC_NODE_API"
        ),
        0
    );
}
#[test]
fn python_import_alias_and_recovery_follow_the_supplied_selector() {
    let pattern = r"\b(?:mlkem\.)?FuturePrivateKey\.generate\(";
    for text in ["from cryptography.hazmat.primitives.asymmetric.mlkem import FuturePrivateKey as Key\nk = Key.generate()\n", "from cryptography.hazmat.primitives.asymmetric.mlkem import FuturePrivateKey\nk = FuturePrivateKey.generate(\n"] {
        assert_eq!(count(&inspect("python",pattern,text),"PQC_PYTHON_API"),2);
    }
    assert_eq!(count(&inspect("python",pattern,"from cryptography.hazmat.primitives.asymmetric.mlkem import MLKEM768PrivateKey\nk = MLKEM768PrivateKey.generate()\n"),"PQC_PYTHON_API"),0);
}
#[test]
fn quoting_and_partial_algorithm_names_do_not_forge_a_selected_argument() {
    let pattern = r#"\bgenerateKeyPairSync\(\s*['"]ml-kem-768['"]"#;
    for text in [
        r#"generateKeyPairSync("ml-kem-768' + unknown");"#,
        r#"generateKeyPairSync('ml-kem-768" + unknown');"#,
        "generateKeyPairSync('ml-kem-768-extra');",
    ] {
        assert_eq!(
            count(&inspect("javascript", pattern, text), "PQC_NODE_API"),
            0
        );
    }
}

#[test]
fn rust_tls_only_selection_does_not_consume_disabled_pqc_candidate_budget() {
    let rules = RuleSet::load_from_dir(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let root = std::env::temp_dir().join(format!("pqc-tls-budget-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("tls.yml"),
        serde_json::to_vec(&vec![rules.get("API_RUST_OPENSSL_TLS").unwrap()]).unwrap(),
    )
    .unwrap();
    let rules = RuleSet::load_from_dir(&root).unwrap();
    let text = "use ml_kem::MlKem768; fn setup() { openssl::ssl::SslMethod::tls(); }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(text, None).unwrap();
    let facts = super::inspect(
        tree.root_node(),
        text,
        "rust",
        &rules,
        crate::DetectorBudget {
            max_ast_candidates: 1,
            ..Default::default()
        },
    );
    assert!(!facts.truncated);
    assert_eq!(count(&facts, "API_RUST_OPENSSL_TLS"), 1);
    assert_eq!(count(&facts, "PQC_RUST_API"), 0);
    std::fs::remove_dir_all(root).unwrap();
}
