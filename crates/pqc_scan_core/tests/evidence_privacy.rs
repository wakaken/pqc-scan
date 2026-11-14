use pqc_scan_core::{
    scan_repository_native, Detection, Detector, Evidence, Location, NativeOptions, ScanConfig,
    ScannableFile,
};
use pqc_scan_rules::RuleSet;
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};
struct Fixture(PathBuf);
impl Fixture {
    fn new(tag: &str) -> Self {
        let p = std::env::temp_dir().join(format!("pqc029-core-{}-{tag}", std::process::id()));
        fs::create_dir_all(p.join("input")).unwrap();
        fs::create_dir_all(p.join("rules")).unwrap();
        fs::write(p.join("rules/control.yml"),"id: CONTROL\nkind: regex\ncategory: Control\nseverity: low\nrisk: quantum-uncertain\nconfidence: 0.5\nmigration_hint: Review\npattern: CONTROL\nscope: code\n").unwrap();
        Self(p)
    }
    fn scan(&self, erase: bool, fail: bool) -> pqc_scan_core::ScanDocument {
        let rules = RuleSet::load_from_dir(&self.0.join("rules")).unwrap();
        let c = ScanConfig {
            root: self.0.join("input"),
            threads: 1,
            ..Default::default()
        };
        let mut detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(Body { erase })];
        if fail {
            detectors.push(Arc::new(Failure));
        }
        scan_repository_native(
            &c,
            &rules,
            &detectors,
            &NativeOptions::new(Some("synthetic/privacy")).unwrap(),
        )
        .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct Body {
    erase: bool,
}
impl Detector for Body {
    fn name(&self) -> &'static str {
        "synthetic_body"
    }
    fn detect(&self, file: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        if self.erase {
            fs::write(&file.path, "header removed after inspection\npublic text\n")?;
        }
        Ok(vec![Detection {
            rule_id: "CONTROL".into(),
            location: Location {
                file: file.path.to_string_lossy().into(),
                line: 2,
                column: 1,
            },
            evidence: Evidence {
                r#type: "regex_match".into(),
                r#match: "LEAK029A_OPAQUE_BODY_Z029ZZ".into(),
                snippet_preview: "opaque preview".into(),
                metadata: BTreeMap::from([
                    ("key_bits".into(), "2048".into()),
                    ("signature_oid".into(), "1.2.840.113549.1.1.11".into()),
                ]),
            },
        }])
    }
}
struct Failure;
impl Detector for Failure {
    fn name(&self) -> &'static str {
        "synthetic_failure"
    }
    fn detect(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        anyhow::bail!("LEAK029E_SYNTHETIC_ERROR_NOT_SECRET_Z029ZZ")
    }
}
#[test]
fn redaction_preserves_legacy_ids_native_associations_cbom_rows_and_technical_metadata() {
    let f = Fixture::new("links");
    let input = f.0.join("input/body.txt");
    fs::write(
        &input,
        "public opaque control\nLEAK029A_OPAQUE_BODY_Z029ZZ\n",
    )
    .unwrap();
    let before = f.scan(false, false);
    fs::write(&input,"-----BEGIN EC PRIVATE KEY-----\nLEAK029A_OPAQUE_BODY_Z029ZZ\n-----END EC PRIVATE KEY-----\n").unwrap();
    let after = f.scan(false, false);
    assert_eq!(
        before.legacy().findings[0].finding_id,
        after.legacy().findings[0].finding_id
    );
    assert_eq!(
        before.legacy().findings.len(),
        after.legacy().findings.len()
    );
    assert_eq!(before.legacy().cbom.len(), after.legacy().cbom.len());
    assert!(!serde_json::to_string(after.legacy())
        .unwrap()
        .contains("LEAK029"));
    assert_eq!(
        after.legacy().findings[0].evidence.metadata["key_bits"],
        "2048"
    );
    assert_eq!(
        after.legacy().findings[0].evidence.metadata["signature_oid"],
        "1.2.840.113549.1.1.11"
    );
    let native = serde_json::to_value(after.native().unwrap()).unwrap();
    assert_eq!(
        native["legacy_associations"][0]["legacy_id"],
        after.legacy().findings[0].finding_id
    );
    assert_eq!(
        native["legacy_associations"][0]["observation_ids"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(after.native().unwrap().validate().is_ok());
}
#[test]
fn initial_source_snapshot_masks_body_even_if_a_detector_changes_the_file() {
    let f = Fixture::new("snapshot");
    fs::write(f.0.join("input/body.txt"),"-----BEGIN DSA PRIVATE KEY-----\nLEAK029A_OPAQUE_BODY_Z029ZZ\n-----END DSA PRIVATE KEY-----\n").unwrap();
    let document = f.scan(true, false);
    assert!(!serde_json::to_string(document.legacy())
        .unwrap()
        .contains("LEAK029"));
    assert!(!serde_json::to_string(document.native().unwrap())
        .unwrap()
        .contains("LEAK029"));
}
#[test]
fn detector_error_content_is_not_exported_by_diagnostics_or_inventory() {
    let f = Fixture::new("diagnostics");
    fs::write(
        f.0.join("input/body.txt"),
        "-----BEGIN ENCRYPTED PRIVATE KEY-----\nLEAK029A_OPAQUE_BODY_Z029ZZ\n",
    )
    .unwrap();
    let d = f.scan(false, true);
    assert!(!serde_json::to_string(&d.diagnostics_report())
        .unwrap()
        .contains("LEAK029"));
    assert!(!serde_json::to_string(d.native().unwrap())
        .unwrap()
        .contains("LEAK029"));
    assert!(!serde_json::to_string(d.legacy())
        .unwrap()
        .contains("LEAK029"));
}

#[test]
fn quality057_unknown_line_or_column_preserves_conservative_redaction() {
    struct Position(usize, usize);
    impl Detector for Position {
        fn name(&self) -> &'static str {
            "position057"
        }
        fn detect(&self, file: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
            Ok(vec![Detection {
                rule_id: "CONTROL".into(),
                location: Location {
                    file: file.path.to_string_lossy().into(),
                    line: self.0,
                    column: self.1,
                },
                evidence: Evidence {
                    r#type: "regex_match".into(),
                    r#match: "MISSING057".into(),
                    snippet_preview: "MISSING057".into(),
                    metadata: BTreeMap::new(),
                },
            }])
        }
    }
    for (line, column, tag) in [(0, 1, "line-zero"), (1, 0, "column-zero")] {
        let f = Fixture::new(tag);
        let input = f.0.join("input/body.txt");
        let rules = RuleSet::load_from_dir(&f.0.join("rules")).unwrap();
        let config = ScanConfig {
            root: f.0.join("input"),
            threads: 1,
            ..Default::default()
        };
        let scan = || {
            scan_repository_native(
                &config,
                &rules,
                &[Arc::new(Position(line, column))],
                &NativeOptions::new(Some("synthetic/057")).unwrap(),
            )
            .unwrap()
        };
        fs::write(&input, "public MISSING057").unwrap();
        let public = scan();
        assert!(serde_json::to_string(public.legacy())
            .unwrap()
            .contains("MISSING057"));
        fs::write(
            &input,
            format!(
                "{} password='MISSING057_SYNTHETIC_VALUE'",
                "public ".repeat(12)
            ),
        )
        .unwrap();
        let protected = scan();
        assert_eq!(protected.legacy().findings.len(), 1);
        for value in [
            serde_json::to_value(protected.legacy()).unwrap(),
            serde_json::to_value(protected.native().unwrap()).unwrap(),
            serde_json::to_value(protected.diagnostics_report()).unwrap(),
        ] {
            assert!(!value.to_string().contains("MISSING057"));
        }
    }
}
