use pqc_scan_core::observation::AlgorithmSpec;
use pqc_scan_core::{
    scan_repository_native, Detection, Detector, DetectorOutcome, DetectorStatus, Evidence,
    Location, NativeOptions, ObservationDraft, ObservationMethod, ScanConfig, ScannableFile,
};
use pqc_scan_rules::RuleSet;
use std::{collections::BTreeMap, sync::Arc};
#[test]
fn source_vocabulary_is_finite_and_does_not_change_legacy_lexical_meaning() {
    for name in [
        "DSA",
        "DH",
        "EC",
        "ECDH",
        "ECDSA",
        "X25519",
        "X-Wing",
        "CompositeMLKem",
        "CompositeMLDsa",
        "HPKE",
        "SLH-DSA-SHA2-128s",
        "SLH-DSA-SHAKE-256f",
    ] {
        assert!(
            AlgorithmSpec::source_api(name, &BTreeMap::new()).is_some(),
            "{name}"
        );
        // The additive source API path must not broaden legacy token matching.
        assert!(AlgorithmSpec::recognized(name).is_none(), "{name}");
    }
    for name in [
        "ML-KEM-513",
        "ML-DSA-45",
        "SLH-DSA-SHA2-128",
        "X25519MLKEM768",
        "SECRET_DO_NOT_EXPORT",
        "https://user:secret@invalid.example",
    ] {
        assert!(
            AlgorithmSpec::source_api(name, &BTreeMap::new()).is_none(),
            "{name}"
        );
    }
    let public_parameters = BTreeMap::from([("key_bits".into(), "2048".into())]);
    let rsa = serde_json::to_value(AlgorithmSpec::source_api("RSA", &public_parameters).unwrap())
        .unwrap();
    assert_eq!(rsa["parameters"]["key_bits"], "2048");
    assert_eq!(rsa["purpose"], "reference");
    let curve = BTreeMap::from([
        ("key_bits".into(), "384".into()),
        ("named_curve_oid".into(), "1.3.132.0.34".into()),
    ]);
    let ec = serde_json::to_value(AlgorithmSpec::source_api("ECDSA", &curve).unwrap()).unwrap();
    assert_eq!(ec["parameters"]["named_curve_oid"], "1.3.132.0.34");
    assert_eq!(ec["parameters"]["key_bits"], "384");
    for name in ["RSA", "ML-KEM-768", "ML-DSA-65", "Kyber768", "Dilithium3"] {
        let legacy = AlgorithmSpec::recognized(name).unwrap();
        let source = AlgorithmSpec::source_api(name, &BTreeMap::new()).unwrap();
        assert_eq!(legacy, source);
    }
    assert!(AlgorithmSpec::source_api(
        "ML-KEM-768",
        &BTreeMap::from([("key_bits".into(), "1024".into())])
    )
    .is_none());
    assert!(AlgorithmSpec::source_api(
        "RSA",
        &BTreeMap::from([("key_bits".into(), "SECRET_DO_NOT_EXPORT".into())])
    )
    .is_none());
    assert!(AlgorithmSpec::source_api(
        "EC",
        &BTreeMap::from([("named_curve_oid".into(), "1.2.3.4.5".into())])
    )
    .is_none());
    assert!(AlgorithmSpec::source_api(
        "RSA",
        &BTreeMap::from([("private_key".into(), "SECRET_DO_NOT_EXPORT".into())])
    )
    .is_none());
}
struct Capture(ObservationMethod);
impl Detector for Capture {
    fn name(&self) -> &'static str {
        "source_api_fixture"
    }
    fn detect(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        Ok(Vec::new())
    }
    fn detect_observed(
        &self,
        file: &ScannableFile,
        _: &RuleSet,
    ) -> anyhow::Result<DetectorOutcome> {
        let d = Detection {
            rule_id: "CONTROL".into(),
            location: Location {
                file: file.path.to_string_lossy().into(),
                line: 1,
                column: 1,
            },
            evidence: Evidence {
                r#type: "tree_sitter_match".into(),
                r#match: "SECRET_DO_NOT_EXPORT".into(),
                snippet_preview: "SECRET_DO_NOT_EXPORT".into(),
                metadata: BTreeMap::new(),
            },
        };
        let o =
            ObservationDraft::source(&d, self.0, "SECRET_DO_NOT_EXPORT ML-KEM-512", Some((0, 5)))
                .with_source_api_references(&["X-Wing".into(), "SECRET_DO_NOT_EXPORT".into()]);
        Ok(DetectorOutcome::new(vec![d], DetectorStatus::Complete).with_observations(vec![o]))
    }
}
#[test]
fn typed_source_capture_omits_arguments_and_stays_a_syntactic_reference() {
    let dir = std::env::temp_dir().join(format!("pqc-v05-core-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("input")).unwrap();
    std::fs::create_dir_all(dir.join("rules")).unwrap();
    std::fs::write(dir.join("input/a.php"), "TOKEN\n").unwrap();
    std::fs::write(dir.join("rules/r.yml"),serde_json::json!([{"id":"CONTROL","kind":"tree_sitter","scope":"php","pattern":"TOKEN","category":"test","severity":"info","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review"}]).to_string()).unwrap();
    let rules = RuleSet::load_from_dir(&dir.join("rules")).unwrap();
    for (method, expected, forbidden, method_name, assurance) in [
        (
            ObservationMethod::AstPattern,
            "X-Wing",
            "ML-KEM-512",
            "ast_pattern",
            "syntactic",
        ),
        (
            ObservationMethod::LexicalMatch,
            "ML-KEM-512",
            "X-Wing",
            "lexical_match",
            "lexical",
        ),
    ] {
        let ds: Vec<Arc<dyn Detector>> = vec![Arc::new(Capture(method))];
        let doc = scan_repository_native(
            &ScanConfig {
                root: dir.join("input"),
                threads: 1,
                ..Default::default()
            },
            &rules,
            &ds,
            &NativeOptions::new(Some("v05/capture")).unwrap(),
        )
        .unwrap();
        let value = serde_json::to_value(doc.native().unwrap()).unwrap();
        let raw = serde_json::to_string(&value).unwrap();
        assert!(!raw.contains("SECRET_DO_NOT_EXPORT"));
        assert!(!raw.contains(forbidden));
        let algorithms: Vec<_> = value["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["kind"] == "algorithm")
            .collect();
        assert_eq!(algorithms.len(), 1);
        assert_eq!(algorithms[0]["attributes"]["identifier"], expected);
        assert_eq!(value["observations"].as_array().unwrap().len(), 1);
        let obs = &value["observations"][0];
        assert_eq!(obs["method"], method_name);
        assert_eq!(obs["assurance"], assurance);
        assert_eq!(obs["usage"], "referenced");
        assert_eq!(obs["target"]["byte_start"], 0);
        assert_eq!(obs["target"]["byte_end"], 5);
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn header_equivalence_requires_the_same_span_and_exact_algorithm_facts() {
    let d = Detection {
        rule_id: "CONTROL".into(),
        location: Location {
            file: "a.h".into(),
            line: 1,
            column: 1,
        },
        evidence: Evidence {
            r#type: "tree_sitter_match".into(),
            r#match: "API".into(),
            snippet_preview: "API".into(),
            metadata: BTreeMap::new(),
        },
    };
    let draft = |method, span, names: Vec<String>| {
        ObservationDraft::source(&d, method, "", span).with_source_api_references(&names)
    };
    let a = draft(
        ObservationMethod::AstPattern,
        Some((0, 8)),
        vec!["ML-KEM-768".into()],
    );
    assert!(a.equivalent_source_reference(&a.clone()));
    assert!(!a.equivalent_source_reference(&draft(
        ObservationMethod::AstPattern,
        Some((0, 8)),
        vec!["ML-KEM-1024".into()]
    )));
    assert!(!a.equivalent_source_reference(&draft(
        ObservationMethod::AstPattern,
        Some((8, 16)),
        vec!["ML-KEM-768".into()]
    )));
    assert!(!a.equivalent_source_reference(&draft(
        ObservationMethod::AstPattern,
        Some((0, 8)),
        vec![]
    )));
    assert!(!a.equivalent_source_reference(&draft(
        ObservationMethod::LexicalMatch,
        Some((0, 8)),
        vec!["ML-KEM-768".into()]
    )));
    assert!(!draft(ObservationMethod::AstPattern, None, vec![])
        .equivalent_source_reference(&draft(ObservationMethod::AstPattern, None, vec![])));
    assert!(AlgorithmSpec::source_api(
        "EC",
        &BTreeMap::from([
            ("key_bits".into(), "384".into()),
            ("named_curve_oid".into(), "1.2.840.10045.3.1.7".into())
        ])
    )
    .is_none());
    assert!(AlgorithmSpec::source_api(
        "RSA",
        &BTreeMap::from([("key_bits".into(), "0".repeat(1000))])
    )
    .is_none());
}

#[test]
fn opaque_header_equivalence_requires_identical_public_project_and_configuration_facts() {
    use pqc_scan_core::observation::{
        SourceConfigurationReference as C, SourceLibraryReference as L,
    };
    let d = Detection {
        rule_id: "CONTROL".into(),
        location: Location {
            file: "a.h".into(),
            line: 1,
            column: 1,
        },
        evidence: Evidence {
            r#type: "tree_sitter_match".into(),
            r#match: "same safe label".into(),
            snippet_preview: "same safe label".into(),
            metadata: BTreeMap::new(),
        },
    };
    let opaque = || {
        ObservationDraft::source(&d, ObservationMethod::AstPattern, "", Some((0, 10)))
            .with_source_api_references(&[])
    };
    let library = opaque().with_source_library_reference(L::CLibOqsInclude);
    assert!(library.equivalent_source_reference(&library.clone()));
    assert!(!library
        .equivalent_source_reference(&opaque().with_source_library_reference(L::COpenSslInclude)));
    let config =
        |group| opaque().with_source_configuration_reference(C::tls_groups(group).unwrap());
    assert!(config("MLKEM768").equivalent_source_reference(&config("MLKEM768")));
    assert!(!config("MLKEM768").equivalent_source_reference(&config("X25519MLKEM768")));
    assert!(!config("X25519MLKEM768").equivalent_source_reference(&config("X25519Kyber768Draft00")));
    assert!(!config("P-256").equivalent_source_reference(&config("secp256r1")));
    for value in [
        "private=PRIVATE080",
        "X-Wing",
        "https://user:secret@invalid.example",
    ] {
        assert!(C::tls_groups(value).is_none());
        assert!(C::provider(value).is_none());
        assert!(C::property(value).is_none());
    }
}
