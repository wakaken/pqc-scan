//! An external legacy-style client uses only public crate interfaces.
use pqc_scan_core::{
    scan_repository, scan_repository_detailed, scan_repository_native, Detection, Detector,
    DetectorStatus, Evidence, Location, NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_report::{write_reports, ReportFormat};
use pqc_scan_rules::RuleSet;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct OldDetector(AtomicUsize);
impl Detector for OldDetector {
    fn name(&self) -> &'static str {
        "old-public-client"
    }
    fn detect(&self, _file: &ScannableFile, _rules: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(vec![Detection {
            rule_id: "COMPAT058".into(),
            location: Location {
                file: "app.js".into(),
                line: 1,
                column: 1,
            },
            evidence: Evidence {
                r#type: "regex_match".into(),
                r#match: "RS256".into(),
                snippet_preview: "RS256".into(),
                metadata: BTreeMap::new(),
            },
        }])
    }
}
#[test]
fn old_detector_and_scan_writer_calls_keep_results_and_unknown_assurance() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("pqc058-public-{}-{unique}", std::process::id());
    let root = Scratch(std::env::temp_dir().join(name));
    fs::create_dir_all(root.0.join("input")).unwrap();
    fs::create_dir(root.0.join("rules")).unwrap();
    fs::write(root.0.join("input/app.js"), "const algorithm = 'RS256';\n").unwrap();
    fs::write(root.0.join("rules/test.yml"),r#"[{"id":"COMPAT058","kind":"regex","category":"cryptoapi","severity":"medium","risk":"quantum-uncertain","confidence":0.8,"migration_hint":"Review use","pattern":"RS256","scope":"code"}]"#).unwrap();
    let rules = RuleSet::load_from_dir(&root.0.join("rules")).unwrap();
    let config = ScanConfig {
        root: root.0.join("input"),
        ..ScanConfig::default()
    };
    let detector = Arc::new(OldDetector(AtomicUsize::new(0)));
    let detectors: Vec<Arc<dyn Detector>> = vec![detector.clone()];
    let legacy = scan_repository(&config, &rules, &detectors).unwrap();
    assert_eq!(detector.0.load(Ordering::SeqCst), 1);
    let detailed = scan_repository_detailed(&config, &rules, &detectors).unwrap();
    assert_eq!(detector.0.load(Ordering::SeqCst), 2);
    let native = scan_repository_native(
        &config,
        &rules,
        &detectors,
        &NativeOptions::new(Some("compat058")).unwrap(),
    )
    .unwrap();
    assert_eq!(detector.0.load(Ordering::SeqCst), 3);
    assert_eq!(detailed.coverage().unknown_files(), 1);
    assert_eq!(native.coverage().unknown_files(), 1);
    let normalize = |result: &pqc_scan_core::ScanResult| {
        let mut value = serde_json::to_value(result).unwrap();
        value.as_object_mut().unwrap().remove("generated_at");
        value
    };
    assert_eq!(normalize(&legacy), normalize(detailed.legacy()));
    assert_eq!(normalize(&legacy), normalize(native.legacy()));
    let native_value = serde_json::to_value(native.native().unwrap()).unwrap();
    assert_eq!(native_value["schema_version"], 1);
    for o in native_value["observations"].as_array().unwrap() {
        assert_ne!(o["assurance"], "cryptographically_verified");
        assert_ne!(o["usage"], "executed");
    }
    let files = write_reports(&legacy, &root.0.join("reports"), ReportFormat::All).unwrap();
    let actual: BTreeSet<_> = files
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap())
        .collect();
    assert_eq!(
        actual,
        BTreeSet::from([
            "report.json",
            "report.html",
            "report.rules.html",
            "report.md",
            "report.sarif",
            "cbom.json",
            "dependency-sbom.json"
        ])
    );
    assert_eq!(legacy.findings.len(), 1);
    // An old implementer needs only name/detect; the default adapter remains NotReported.
    let file = ScannableFile {
        path: config.root.join("app.js"),
        size: 5,
        bytes: b"RS256".to_vec(),
        text: Some("RS256".into()),
    };
    assert_eq!(
        detector.detect_detailed(&file, &rules).unwrap().status(),
        DetectorStatus::NotReported
    );
}
