//! Task084-only metadata observer. Target repositories are data, never executed.
//! Compile against the separately bound scanner source; never ship this binary.
use anyhow::{ensure, Result};
use pqc_scan_core::{
    scan_repository_native, Detection, Detector, DetectorOutcome, NativeOptions, RepositoryContext,
    ScanConfig, ScanDocument, ScannableFile,
};
use pqc_scan_rules::RuleSet;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

struct Observed {
    inner: Arc<dyn Detector>,
    rows: Arc<Mutex<Vec<Value>>>,
}
impl Detector for Observed {
    fn name(&self) -> &'static str {
        self.inner.name()
    }
    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        self.inner.detect(file, rules)
    }
    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.inner.detect_detailed(file, rules)
    }
    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.inner.detect_observed(file, rules)
    }
    fn detect_in_repository(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        context: Option<&RepositoryContext>,
        observed: bool,
    ) -> Result<DetectorOutcome> {
        let result = self
            .inner
            .detect_in_repository(file, rules, context, observed);
        let status = result
            .as_ref()
            .map(|outcome| format!("{:?}", outcome.status()))
            .unwrap_or_else(|_| "Failed".to_owned());
        self.rows.lock().unwrap().push(json!({
            "file": file.path.to_string_lossy(),
            "detector": self.inner.name(),
            "status": status,
        }));
        result
    }
}

fn config(root: &Path) -> ScanConfig {
    ScanConfig {
        root: root.to_path_buf(),
        threads: 1,
        max_file_size_bytes: 2 * 1024 * 1024,
    }
}
fn run(root: &Path, rules: &RuleSet, scope: &str) -> Result<(ScanDocument, Vec<Value>)> {
    let rows = Arc::new(Mutex::new(Vec::new()));
    let detectors: Vec<Arc<dyn Detector>> = pqc_scan_detectors::default_detectors()
        .into_iter()
        .map(|inner| {
            Arc::new(Observed {
                inner,
                rows: Arc::clone(&rows),
            }) as Arc<dyn Detector>
        })
        .collect();
    let document = scan_repository_native(
        &config(root),
        rules,
        &detectors,
        &NativeOptions::new(Some(scope))?,
    )?;
    let captured = rows.lock().unwrap().clone();
    Ok((document, captured))
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    ensure!(
        args.len() == 5,
        "expected root rules repository-scope output.json"
    );
    let root = PathBuf::from(&args[1]).canonicalize()?;
    let output = PathBuf::from(&args[4]);
    ensure!(!output.exists(), "refusing to overwrite observer output");
    let rules = RuleSet::load_from_dir(Path::new(&args[2]))?;
    let (selected, _) = pqc_scan_core::walker::walk_repository(&root, 2 * 1024 * 1024)?;
    let (document, mut rows) = run(&root, &rules, &args[3])?;
    rows.sort_by_key(Value::to_string);
    let inventory = serde_json::to_value(document.native().unwrap())?;
    let selected: Vec<_> = selected
        .iter()
        .map(|p| {
            p.strip_prefix(&root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    let value = json!({
        "observer": "task084-default-detector-forwarder",
        "repositoryScope": args[3],
        "selectedPaths": selected,
        "detectorRows": rows,
        "coverage": inventory["coverage"],
        "rulesDigest": inventory["rules_digest"],
        "scannerVersion": inventory["scanner_version"],
        "nativeObservationCount": inventory["observations"].as_array().unwrap().len(),
        "findingCount": document.legacy().findings.len(),
        "purpose": "Per-file detector states only; not a performance sample or a new crypto matcher"
    });
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer(file, &value)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir()
                .join(format!("pqc084-observer-control-{}", std::process::id()));
            fs::create_dir(&root).unwrap();
            for (name, source) in [
                ("a.php", "<?php openssl_sign($message,$sig,$key);"),
                ("a.c", "void f(){EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,0);}"),
                ("a.cpp", "void f(){OQS_KEM_new(\"ML-KEM-768\");}"),
                ("a.cs", "using System.Security.Cryptography; class C {void M(){RSA.Create();}}"),
                ("Directory.Packages.props", "<Project><PropertyGroup><ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally></PropertyGroup><ItemGroup><PackageVersion Include=\"BouncyCastle.Cryptography\" Version=\"2.6.2\"/></ItemGroup></Project>"),
                ("a.csproj", "<Project><ItemGroup><PackageReference Include=\"BouncyCastle.Cryptography\"/></ItemGroup></Project>"),
            ] {
                fs::write(root.join(name), source).unwrap();
            }
            Self(root)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn rules() -> RuleSet {
        RuleSet::load_from_dir(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../rules")
                .as_path(),
        )
        .unwrap()
    }
    fn legacy(document: &ScanDocument) -> Value {
        let mut value = serde_json::to_value(document.legacy()).unwrap();
        value.as_object_mut().unwrap().remove("generated_at");
        value
    }
    #[test]
    fn records_every_file_detector_without_changing_results_or_repository_context() {
        let fixture = Fixture::new();
        let rules = rules();
        let plain = scan_repository_native(
            &config(&fixture.0),
            &rules,
            &pqc_scan_detectors::default_detectors(),
            &NativeOptions::new(Some("oss084/control")).unwrap(),
        )
        .unwrap();
        let (captured, rows) = run(&fixture.0, &rules, "oss084/control").unwrap();
        assert_eq!(legacy(&plain), legacy(&captured));
        let plain_native = serde_json::to_value(plain.native().unwrap()).unwrap();
        let captured_native = serde_json::to_value(captured.native().unwrap()).unwrap();
        assert_eq!(plain_native["coverage"], captured_native["coverage"]);
        assert!(!plain.legacy().findings.is_empty());
        assert_eq!(
            rows.len(),
            6 * 5,
            "all six selected files must have every default detector state"
        );
        let unique: std::collections::HashSet<_> = rows
            .iter()
            .map(|r| (r["file"].as_str().unwrap(), r["detector"].as_str().unwrap()))
            .collect();
        assert_eq!(unique.len(), rows.len());
        assert!(rows
            .iter()
            .any(|r| r["detector"] == "tree_sitter_detector" && r["status"] == "Complete"));
    }
    struct Failed;
    impl Detector for Failed {
        fn name(&self) -> &'static str {
            "controlled_failure"
        }
        fn detect(&self, _: &ScannableFile, _: &RuleSet) -> Result<Vec<Detection>> {
            anyhow::bail!("PRIVATE084_OBSERVER_ERROR_CANARY")
        }
    }
    #[test]
    fn failed_status_is_recorded_without_logging_error_payload() {
        let rows = Arc::new(Mutex::new(Vec::new()));
        let detector = Observed {
            inner: Arc::new(Failed),
            rows: Arc::clone(&rows),
        };
        let file = ScannableFile::from_bytes("a.cs".into(), b"class C{}".to_vec());
        assert!(detector
            .detect_in_repository(&file, &rules(), None, true)
            .is_err());
        let rows = rows.lock().unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["status"], "Failed");
        assert!(!serde_json::to_string(&*rows)
            .unwrap()
            .contains("PRIVATE084_OBSERVER_ERROR_CANARY"));
    }
}
