//! Test-only observations for TASK-055; no serialized product contract is added.
use super::*;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc055-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("input")).unwrap();
        fs::create_dir(root.join("rules")).unwrap();
        fs::write(root.join("rules/test.yml"),"id: TEST\nkind: regex\ncategory: Control\nseverity: low\nrisk: quantum-uncertain\nconfidence: 0.5\nmigration_hint: Review\npattern: HIT\nscope: code\n").unwrap();
        Self(root)
    }
    fn config(&self) -> ScanConfig {
        ScanConfig {
            root: self.0.join("input"),
            threads: 1,
            max_file_size_bytes: 64,
        }
    }
    fn rules(&self) -> RuleSet {
        RuleSet::load_from_dir(&self.0.join("rules")).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct States;
impl Detector for States {
    fn name(&self) -> &'static str {
        "controlled_states"
    }
    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(file, rules)?.into_detections())
    }
    fn detect_detailed(&self, file: &ScannableFile, _: &RuleSet) -> Result<crate::DetectorOutcome> {
        let name = file.path.file_stem().unwrap().to_str().unwrap();
        let status = match name {
            "complete" => DetectorStatus::Complete,
            "partial" => DetectorStatus::Partial,
            "truncated" => DetectorStatus::Truncated,
            "unsupported" => DetectorStatus::Unsupported,
            "notapplicable" => DetectorStatus::NotApplicable,
            "unknown" => DetectorStatus::NotReported,
            "failed" => anyhow::bail!("SYNTHETIC_055_ERROR_MUST_NOT_ESCAPE"),
            _ => DetectorStatus::Complete,
        };
        let hits = if matches!(name, "complete" | "partial" | "truncated") {
            vec![Detection {
                rule_id: "TEST".into(),
                location: crate::Location {
                    file: file.path.to_string_lossy().into(),
                    line: 1,
                    column: 1,
                },
                evidence: crate::Evidence {
                    r#type: "code".into(),
                    r#match: "HIT".into(),
                    snippet_preview: "HIT".into(),
                    metadata: BTreeMap::new(),
                },
            }]
        } else {
            vec![]
        };
        Ok(crate::DetectorOutcome::new(hits, status))
    }
}
fn read(path: &Path) -> std::io::Result<Vec<u8>> {
    if path.ends_with("read.txt") {
        Err(std::io::Error::other("SYNTHETIC_055_READ"))
    } else {
        fs::read(path)
    }
}
fn partition(c: &CoverageSummary) {
    assert_eq!(
        c.selected_files(),
        c.read_ok_files() + c.read_failed_files()
    );
    assert_eq!(
        c.read_ok_files(),
        c.analyzed_files()
            + c.partial_files()
            + c.analysis_failed_files()
            + c.unsupported_files()
            + c.not_applicable_files()
            + c.unknown_files()
    );
}
fn save(name: &str, value: &Value) {
    if let Some(dir) = std::env::var_os("PQC_QUALITY_EVIDENCE_DIR") {
        let dir = PathBuf::from(dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), serde_json::to_vec_pretty(value).unwrap()).unwrap();
    }
}

#[test]
fn quality055_controlled_files_and_failures_preserve_partition_and_facts() {
    let cases = [
        ("complete", "analyzed_files"),
        ("partial", "partial_files"),
        ("truncated", "partial_files"),
        ("failed", "analysis_failed_files"),
        ("unsupported", "unsupported_files"),
        ("notapplicable", "not_applicable_files"),
        ("unknown", "unknown_files"),
        ("read", "read_failed_files"),
    ];
    let mut rows = vec![];
    for (name, terminal) in cases {
        let f = Fixture::new();
        let config = f.config();
        fs::write(config.root.join(format!("{name}.txt")), "HIT").unwrap();
        let doc = scan_with_reader(&config, &f.rules(), &[Arc::new(States)], &read).unwrap();
        partition(doc.coverage());
        let counts = serde_json::to_value(doc.coverage()).unwrap();
        assert_eq!(counts[terminal], 1, "{name}");
        assert_eq!(counts["selected_files"], 1);
        assert_eq!(
            doc.legacy().findings.len(),
            usize::from(matches!(name, "complete" | "partial" | "truncated"))
        );
        if name != "complete" && name != "notapplicable" {
            assert!(!doc.coverage().is_complete());
        }
        let diagnostics = serde_json::to_value(doc.diagnostics_report()).unwrap();
        assert!(!diagnostics.to_string().contains("SYNTHETIC_055"));
        rows.push(json!({"file":format!("{name}.txt"),"expected_terminal":terminal,"expectedApplicable":match name{"unknown"|"read"=>None,"unsupported"|"notapplicable"=>Some(false),_=>Some(true)},"coverage":counts,"diagnostics":diagnostics["diagnostics"],"findings":doc.legacy().findings.len()}));
    }
    let f = Fixture::new();
    for (name, _) in cases {
        fs::write(f.config().root.join(format!("{name}.txt")), "HIT").unwrap();
    }
    let batch = scan_with_reader(&f.config(), &f.rules(), &[Arc::new(States)], &read).unwrap();
    partition(batch.coverage());
    assert_eq!(batch.coverage().selected_files(), 8);
    assert_eq!(batch.coverage().partial_files(), 2);
    assert_eq!(batch.legacy().findings.len(), 3);
    // A failing detector must not erase a different detector's useful result.
    struct Failure;
    impl Detector for Failure {
        fn name(&self) -> &'static str {
            "failure"
        }
        fn detect(&self, _: &ScannableFile, _: &RuleSet) -> Result<Vec<Detection>> {
            anyhow::bail!("SYNTHETIC_055_FAILURE")
        }
    }
    let mixed = Fixture::new();
    fs::write(mixed.config().root.join("complete.txt"), "HIT").unwrap();
    let mixed = scan_with_reader(
        &mixed.config(),
        &mixed.rules(),
        &[Arc::new(Failure), Arc::new(States)],
        &read,
    )
    .unwrap();
    assert_eq!(mixed.legacy().findings.len(), 1);
    assert_eq!(mixed.coverage().partial_files(), 1);
    partition(mixed.coverage());
    assert!(mixed
        .diagnostics()
        .iter()
        .any(|d| d.code() == DiagnosticCode::DetectorFailed));
    save(
        "core055.json",
        &json!({"files":rows,"batch":batch.coverage(),"mixedFailure":{"coverage":mixed.coverage(),"retained_findings":mixed.legacy().findings.len(),"diagnostics":mixed.diagnostics_report()}}),
    );
}

#[test]
fn quality055_known_population_distinguishes_policy_exclusion_from_non_enumeration() {
    let f = Fixture::new();
    let root = f.config().root;
    let inputs = [
        (".gitignore", "ignored.txt\n".to_string()),
        ("good.txt", "ok".into()),
        ("edge.txt", "a".repeat(64)),
        ("large.txt", "a".repeat(65)),
        ("note.md", "note".into()),
        ("image.png", "not an image".into()),
        ("docs/inside.txt", "text".into()),
        ("ignored.txt", "ignored".into()),
        (".git/hidden", "hidden".into()),
    ];
    for (name, text) in &inputs {
        let p = root.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    }
    let walk = walker::walk_repository_detailed(&root, 64).unwrap();
    let mut selected: Vec<_> = walk
        .files
        .iter()
        .map(|p| walker::relative_path(&root, p))
        .collect();
    selected.sort();
    assert_eq!(selected, [".gitignore", "edge.txt", "good.txt"]);
    assert_eq!(walk.excluded, 5);
    let mut excluded: Vec<_> = walk
        .diagnostics
        .iter()
        .filter_map(|d| {
            serde_json::to_value(d).unwrap()["file"]
                .as_str()
                .map(str::to_string)
        })
        .collect();
    excluded.sort();
    assert_eq!(
        excluded,
        [
            ".git/hidden",
            "docs/inside.txt",
            "image.png",
            "large.txt",
            "note.md"
        ]
    );
    let doc = scan_repository_detailed(&f.config(), &f.rules(), &[Arc::new(States)]).unwrap();
    assert!(doc.coverage().not_enumerated_population());
    assert_eq!(doc.coverage().selected_files(), 3);
    partition(doc.coverage());
    let missing = Fixture::new();
    fs::write(
        missing.config().root.join(".gitignore"),
        "[z-a]SYNTHETIC_055_IGNORE\n*\n",
    )
    .unwrap();
    fs::write(missing.config().root.join("ignored.txt"), "hidden").unwrap();
    let soft =
        scan_repository_detailed(&missing.config(), &missing.rules(), &[Arc::new(States)]).unwrap();
    assert!(!soft.coverage().is_complete());
    assert_eq!(soft.coverage().enumeration_failed_entries(), 1);
    let mut absent = f.config();
    absent.root = root.join("absent");
    assert!(scan_repository_detailed(&absent, &f.rules(), &[Arc::new(States)]).is_err());
    save(
        "selection055.json",
        &json!({"knownInputs":inputs,"knownTotal":9,"selected":selected,"enumeratedExcluded":excluded,"notEnumerated":["ignored.txt"],"diagnostics":doc.diagnostics_report(),"softEnumerationFailure":soft.diagnostics_report(),"missingRoot":"fatal_error"}),
    );
}

#[path = "quality046.rs"]
mod quality046;
