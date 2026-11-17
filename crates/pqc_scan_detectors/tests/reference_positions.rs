use pqc_scan_core::{Detector, ScannableFile};
use pqc_scan_detectors::{regex_detector::RegexDetector, tree_sitter_detector::TreeSitterDetector};
use pqc_scan_rules::RuleSet;
use std::path::Path;

fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap()
}

#[test]
fn explicit_python_imports_start_at_the_module_and_retain_imported_parameters() {
    let rules = rules();
    for (source, line, column, expected) in [
        ("from cryptography.hazmat.primitives.asymmetric.mldsa import MLDSA87PrivateKey\n", 1, 6, "cryptography.hazmat.primitives.asymmetric.mldsa import MLDSA87PrivateKey"),
        ("# 日本語😀\r\ndef key_type():\r\n    from cryptography.hazmat.primitives.asymmetric.mldsa import (\r\n        MLDSA87PrivateKey,\r\n    )\r\n",3,10,"cryptography.hazmat.primitives.asymmetric.mldsa import (\r\n        MLDSA87PrivateKey,\r\n    )"),
    ] {
        {
            let detector = &TreeSitterDetector as &dyn Detector;
            let file = ScannableFile::from_bytes("example.py".into(), source.as_bytes().to_vec());
            let output = detector.detect_observed(&file, &rules).unwrap();
            let found: Vec<_> = output.detections().iter().filter(|d| d.rule_id == "PQC_PYTHON_API").collect();
            assert_eq!(found.len(), 1, "{}", detector.name());
            assert_eq!((found[0].location.line, found[0].location.column), (line, column));
            assert_eq!(found[0].evidence.r#match, pqc_scan_core::model::mask_preview(expected));
        }
    }
}

#[test]
fn configuration_ranges_exclude_layout_and_count_unicode_columns() {
    let rules = rules();
    for (source, line, column) in [
        (
            "<entry key=\"jdk.tls.namedGroups\">secp384r1,secp521r1</entry>",
            1,
            1,
        ),
        (
            "\r\n\r\n\t <entry key=\"jdk.tls.namedGroups\">secp384r1,secp521r1</entry>",
            3,
            3,
        ),
        (
            "\u{2003}<entry key=\"jdk.tls.namedGroups\">secp384r1,secp521r1</entry>",
            1,
            2,
        ),
    ] {
        let file = ScannableFile::from_bytes("settings.xml".into(), source.as_bytes().to_vec());
        let output = RegexDetector.detect_observed(&file, &rules).unwrap();
        let found: Vec<_> = output
            .detections()
            .iter()
            .filter(|d| d.rule_id == "PQC_JAVA_CLASSICAL_GROUPS")
            .collect();
        assert_eq!(found.len(), 1);
        assert_eq!(
            (found[0].location.line, found[0].location.column),
            (line, column)
        );
        assert_eq!(
            found[0].evidence.r#match,
            pqc_scan_core::model::mask_preview(
                "<entry key=\"jdk.tls.namedGroups\">secp384r1,secp521r1</entry>"
            )
        );
    }
}

#[test]
fn deliberately_whitespace_only_custom_configuration_matches_are_preserved() {
    let root = std::env::temp_dir().join(format!("pqc-layout-only-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    std::fs::write(root.join("rule.yml"), "id: WHITESPACE\nkind: regex\ncategory: custom\nseverity: info\nrisk: quantum-uncertain\nconfidence: 0.3\nmigration_hint: Inspect layout.\npattern: '^\\s+$'\nscope: config\n").unwrap();
    let rules = RuleSet::load_from_dir(&root).unwrap();
    let file = ScannableFile::from_bytes("layout.conf".into(), b" \t ".to_vec());
    let output = RegexDetector.detect_observed(&file, &rules).unwrap();
    assert_eq!(output.detections().len(), 1);
    assert_eq!(
        output.detections()[0].evidence.r#match,
        pqc_scan_core::model::mask_preview(" \t ")
    );
    assert_eq!(
        (
            output.detections()[0].location.line,
            output.detections()[0].location.column
        ),
        (1, 1)
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn native_targets_bind_the_complete_original_source_reference_range() {
    use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
    use std::sync::Arc;
    for (index, filename, source, id, expected) in [
        (0,"input.py", "# 日本語😀\r\nfrom cryptography.hazmat.primitives.asymmetric.mldsa import (\r\n    MLDSA87PrivateKey,\r\n)\r\n", "PQC_PYTHON_API", "cryptography.hazmat.primitives.asymmetric.mldsa import (\r\n    MLDSA87PrivateKey,\r\n)"),
        (1,"input.xml", "\r\n\u{2003}<entry key=\"jdk.tls.namedGroups\">secp384r1,secp521r1</entry>", "PQC_JAVA_CLASSICAL_GROUPS", "<entry key=\"jdk.tls.namedGroups\">secp384r1,secp521r1</entry>"),
    ] {
        let root = std::env::temp_dir().join(format!("pqc-position-native-{}-{index}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join(filename), source).unwrap();
        let document = scan_repository_native(&ScanConfig {root:root.clone(),threads:1,..Default::default()}, &rules(), &[Arc::new(RegexDetector),Arc::new(TreeSitterDetector)], &NativeOptions::new(Some("test/positions")).unwrap()).unwrap();
        let value = serde_json::to_value(document.native().unwrap()).unwrap();
        let observations: Vec<_> = value["observations"].as_array().unwrap().iter().filter(|o| o["rule_id"]==id).collect();
        assert!(!observations.is_empty());
        let start=source.find(expected).unwrap();
        for observation in observations {
            assert_eq!(observation["target"]["byte_start"], start);
            assert_eq!(observation["target"]["byte_end"], start+expected.len());
            assert_eq!(&source[start..start+expected.len()], expected);
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn location_refinement_does_not_relax_the_original_statement_byte_budget() {
    let source = "from cryptography.hazmat.primitives.asymmetric.mldsa import MLDSA87PrivateKey";
    let file = ScannableFile::from_bytes("input.py".into(), source.as_bytes().to_vec());
    let root = std::env::temp_dir().join(format!("pqc-position-budget-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let builtin = rules();
    std::fs::write(
        root.join("one.yml"),
        serde_json::to_vec(&[builtin.get("PQC_PYTHON_API").unwrap()]).unwrap(),
    )
    .unwrap();
    let rules = RuleSet::load_from_dir(&root).unwrap();
    for (limit, expected) in [(source.len() - 1, 0), (source.len(), 1)] {
        let detector = TreeSitterDetector::with_budget(pqc_scan_detectors::DetectorBudget {
            max_ast_match_bytes: limit,
            ..Default::default()
        });
        let output = detector.detect_detailed(&file, &rules).unwrap();
        assert_eq!(
            output
                .detections()
                .iter()
                .filter(|d| d.rule_id == "PQC_PYTHON_API")
                .count(),
            expected
        );
        if expected == 0 {
            assert_eq!(output.status(), pqc_scan_core::DetectorStatus::Truncated);
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
