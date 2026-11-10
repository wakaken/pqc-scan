use pqc_scan_core::diff::{compare, parse_inventory, MAX_INPUT_BYTES};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub(crate) fn run(
    before: &Path,
    after: &Path,
    scope: &str,
    out_dir: &Path,
    stdout: &mut impl Write,
) -> anyhow::Result<()> {
    let read = |path: &Path| -> anyhow::Result<_> {
        let file =
            File::open(path).map_err(|_| anyhow::anyhow!("failed to read comparison input"))?;
        let mut bytes = Vec::new();
        file.take(MAX_INPUT_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("failed to read comparison input"))?;
        parse_inventory(&bytes)
    };
    let before = read(before)?;
    let after = read(after)?;
    let report = compare(&before, &after, scope)?;
    let json = serde_json::to_vec_pretty(&report)?;
    let summary = report.summary_markdown();
    fs::create_dir_all(out_dir)?;
    let json_path = out_dir.join("scan.diff.v1.json");
    let summary_path = out_dir.join("scan.diff.v1.md");
    fs::write(&json_path, json)?;
    fs::write(&summary_path, summary)?;
    writeln!(stdout, "written {}", json_path.display())?;
    writeln!(stdout, "written {}", summary_path.display())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
    use pqc_scan_rules::RuleSet;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "pqc035-adapter-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(root.join("input")).unwrap();
            let checkout = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
            let rules = RuleSet::load_from_dir(&checkout.join("rules")).unwrap();
            let document = scan_repository_native(
                &ScanConfig {
                    root: root.join("input"),
                    threads: 1,
                    ..Default::default()
                },
                &rules,
                &[],
                &NativeOptions::new(Some("test/project")).unwrap(),
            )
            .unwrap();
            let bytes = serde_json::to_vec(document.native().unwrap()).unwrap();
            fs::write(root.join("before.json"), &bytes).unwrap();
            fs::write(root.join("after.json"), &bytes).unwrap();
            Self(root)
        }
        fn run(&self, scope: &str) -> anyhow::Result<Vec<u8>> {
            let mut stdout = Vec::new();
            run(
                &self.0.join("before.json"),
                &self.0.join("after.json"),
                scope,
                &self.0.join("output"),
                &mut stdout,
            )?;
            Ok(stdout)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn valid_inputs_write_only_separate_diff_artifacts() {
        let fixture = Fixture::new();
        let stdout = String::from_utf8(fixture.run("test/project").unwrap()).unwrap();
        let names: std::collections::BTreeSet<_> = fs::read_dir(fixture.0.join("output"))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            std::collections::BTreeSet::from([
                "scan.diff.v1.json".to_owned(),
                "scan.diff.v1.md".to_owned()
            ])
        );
        assert!(stdout.contains("scan.diff.v1.json"));
        let value: serde_json::Value =
            serde_json::from_slice(&fs::read(fixture.0.join("output/scan.diff.v1.json")).unwrap())
                .unwrap();
        assert_eq!(value["repository_scope"], "test/project");
        assert!(fs::read_to_string(fixture.0.join("output/scan.diff.v1.md"))
            .unwrap()
            .contains("Missing-after records remain unresolved"));
    }

    #[test]
    fn malformed_or_wrong_scope_input_creates_no_output_directory() {
        let fixture = Fixture::new();
        assert!(fixture.run("wrong/project").is_err());
        assert!(!fixture.0.join("output").exists());
        fs::write(fixture.0.join("after.json"), b"SECRET_MALFORMED_CANARY").unwrap();
        let err = fixture.run("test/project").unwrap_err().to_string();
        assert!(!err.contains("SECRET_MALFORMED_CANARY"));
        assert!(!fixture.0.join("output").exists());
    }

    #[test]
    fn oversized_input_is_rejected_before_output() {
        let fixture = Fixture::new();
        fs::File::create(fixture.0.join("after.json"))
            .unwrap()
            .set_len(pqc_scan_core::diff::MAX_INPUT_BYTES as u64 + 1)
            .unwrap();
        assert!(fixture
            .run("test/project")
            .unwrap_err()
            .to_string()
            .contains("limit"));
        assert!(!fixture.0.join("output").exists());
    }

    #[test]
    fn missing_input_and_output_failure_propagate() {
        let fixture = Fixture::new();
        fs::write(fixture.0.join("output"), b"not a directory").unwrap();
        assert!(fixture.run("test/project").is_err());
        fs::remove_file(fixture.0.join("output")).unwrap();
        fs::remove_file(fixture.0.join("before.json")).unwrap();
        assert!(fixture
            .run("test/project")
            .unwrap_err()
            .to_string()
            .contains("failed to read comparison input"));
        assert!(!fixture.0.join("output").exists());
    }
}
