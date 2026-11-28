use pqc_scan_core::{Detector, DetectorOutcome, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{tree_sitter_detector::TreeSitterDetector, DetectorBudget};
use pqc_scan_rules::RuleSet;
use std::path::Path;

fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .unwrap()
}
fn inspect(name: &str, text: &str) -> DetectorOutcome {
    TreeSitterDetector
        .detect_observed(
            &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap()
}
fn parser_flags(text: &str) -> (bool, bool, bool) {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(text, None).unwrap();
    let mut stack = vec![tree.root_node()];
    let (mut error, mut missing) = (false, false);
    let mut count = 0;
    while let Some(node) = stack.pop() {
        count += 1;
        assert!(count < 1000);
        error |= node.is_error();
        missing |= node.is_missing();
        let mut cursor = node.walk();
        stack.extend(node.children(&mut cursor));
    }
    (tree.root_node().has_error(), error, missing)
}
#[test]
fn error_and_missing_only_syntax_get_parser_explanations_and_keep_references() {
    for (source, error, missing) in [
        ("void f() { RSA_new(); @ }", true, false),
        ("void f() { RSA_new();", false, true),
    ] {
        assert_eq!(parser_flags(source), (true, error, missing));
        let result = inspect("case.c", source);
        assert_eq!(result.status(), DetectorStatus::Partial);
        assert!(result
            .explanation()
            .unwrap()
            .contains("Parser recovery or missing syntax tokens:"));
        assert!(result
            .detections()
            .iter()
            .any(|d| d.rule_id == "C_FAMILY_CRYPTO_API"));
    }
}
#[test]
fn clean_conditional_source_is_not_described_as_parser_recovery() {
    let source = "#if FEATURE\nvoid f() { RSA_new(); }\n#endif\n";
    assert_eq!(parser_flags(source), (false, false, false));
    let result = inspect("conditional.c", source);
    assert_eq!(result.status(), DetectorStatus::Partial);
    let explanation = result.explanation().unwrap();
    assert!(explanation.contains("Conditional source:"));
    assert!(!explanation.contains("Parser recovery"));
    assert!(result
        .detections()
        .iter()
        .any(|d| d.rule_id == "C_FAMILY_CRYPTO_API"));
}
#[test]
fn unresolved_supported_crypto_consumers_do_not_claim_parser_failure() {
    let source = "class C { void F(dynamic key) { key.SignData(data); } }";
    let result = inspect("unknown.cs", source);
    assert_eq!(result.status(), DetectorStatus::Partial);
    let explanation = result.explanation().unwrap();
    assert!(explanation.contains("Unresolved supported crypto binding:"));
    assert!(!explanation.contains("Parser recovery"));
}
#[test]
fn source_limits_keep_precedence_over_parser_and_conditional_causes() {
    let budget = DetectorBudget {
        max_ast_nodes: 2,
        ..Default::default()
    };
    let detector = TreeSitterDetector::with_budget(budget);
    let result = detector
        .detect_observed(
            &ScannableFile::from_bytes(
                "limited.c".into(),
                b"#if FEATURE\nvoid f() { RSA_new();\n#endif\n".to_vec(),
            ),
            &rules(),
        )
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Truncated);
    assert!(result
        .diagnostic_message()
        .unwrap()
        .starts_with("Inspection limit:"));
    assert!(!result
        .diagnostic_message()
        .unwrap()
        .contains("Parser recovery"));
}
#[test]
fn clean_supported_source_keeps_complete_status_without_an_explanation() {
    let result = inspect("complete.c", "void f() { RSA_new(); }");
    assert_eq!(result.status(), DetectorStatus::Complete);
    assert!(result.explanation().is_none());
}
#[test]
fn diagnostics_never_embed_source_secrets_or_parser_debug_text() {
    let secret = "DIAGNOSTIC_SECRET_FIXTURE_7a2a";
    let source = format!("void f() {{ consume(\"{secret}\"); RSA_new(); @ }}");
    let result = inspect("private.c", &source);
    assert_eq!(result.status(), DetectorStatus::Partial);
    let explanation = result.explanation().unwrap();
    assert!(!explanation.contains(secret));
    assert!(!explanation.contains("consume"));
    assert!(!explanation.contains("ERROR"));
}
