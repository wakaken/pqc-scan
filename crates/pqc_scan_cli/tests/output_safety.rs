#![cfg(unix)]
use std::{
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-output-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("input")).unwrap();
        fs::write(root.join("input/a.rs"), "fn main() {}\n").unwrap();
        Self(root)
    }
    fn scan(&self, output: Option<&Path>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
        command
            .current_dir(&self.0)
            .args([
                "scan",
                "input",
                "--format",
                "all",
                "--inventory-json",
                "--repository-scope",
                "test/output",
                "--diagnostics",
                "--cyclonedx-17",
                "--policy-profile",
                "migration-review-v1",
                "--threads",
                "1",
                "--rules-dir",
            ])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"));
        if let Some(path) = output {
            command.arg("--out-dir").arg(path);
        }
        command.output().unwrap()
    }
    fn diff(&self, inventory: &Path, output: &Path) -> Output {
        Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args(["diff"])
            .arg(inventory)
            .arg(inventory)
            .args(["--repository-scope", "test/output", "--out-dir"])
            .arg(output)
            .output()
            .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn real_scan_default_and_explicit_outputs_cannot_redirect_writes() {
    let fixture = Fixture::new();
    let outside = fixture.0.join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("report.json"), b"EXTERNAL_CANARY").unwrap();
    symlink(&outside, fixture.0.join("pqc-report")).unwrap();
    assert!(!fixture.scan(None).status.success());
    assert_eq!(
        fs::read(outside.join("report.json")).unwrap(),
        b"EXTERNAL_CANARY"
    );
    assert_eq!(fs::read_dir(&outside).unwrap().count(), 1);
    fs::remove_file(fixture.0.join("pqc-report")).unwrap();

    symlink(&outside, fixture.0.join("parent-link")).unwrap();
    assert!(!fixture
        .scan(Some(&fixture.0.join("parent-link/new")))
        .status
        .success());
    assert!(!outside.join("new").exists());
    let output = fixture.0.join("normal");
    fs::create_dir(&output).unwrap();
    symlink(outside.join("report.json"), output.join("report.json")).unwrap();
    let attack = fixture.scan(Some(&output));
    assert!(!attack.status.success());
    assert_eq!(
        fs::read(outside.join("report.json")).unwrap(),
        b"EXTERNAL_CANARY"
    );
    fs::remove_file(output.join("report.json")).unwrap();
    fs::hard_link(outside.join("report.json"), output.join("report.json")).unwrap();
    let ordinary = fixture.scan(Some(&output));
    assert!(
        ordinary.status.success(),
        "{}",
        String::from_utf8_lossy(&ordinary.stderr)
    );
    assert_eq!(
        fs::read(outside.join("report.json")).unwrap(),
        b"EXTERNAL_CANARY"
    );
    assert_eq!(fs::read_dir(&output).unwrap().count(), 11);
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(output.join("report.json")).unwrap()).unwrap();
    assert!(report["findings"].as_array().unwrap().is_empty());
    assert!(fixture.scan(Some(&output)).status.success());
}

#[test]
fn real_diff_rejects_both_output_links_and_preserves_valid_repeated_output() {
    let fixture = Fixture::new();
    let normal = fixture.0.join("scan");
    assert!(fixture.scan(Some(&normal)).status.success());
    let inventory = normal.join("scan.inventory.v1.json");
    let victim = fixture.0.join("outside");
    fs::write(&victim, b"EXTERNAL_CANARY").unwrap();
    for name in ["scan.diff.v1.json", "scan.diff.v1.md"] {
        let root = fixture.0.join(name);
        fs::create_dir(&root).unwrap();
        symlink(&victim, root.join(name)).unwrap();
        assert!(!fixture.diff(&inventory, &root).status.success());
        assert_eq!(fs::read(&victim).unwrap(), b"EXTERNAL_CANARY");
    }
    let output = fixture.0.join("diff");
    assert!(fixture.diff(&inventory, &output).status.success());
    let first = fs::read(output.join("scan.diff.v1.json")).unwrap();
    assert!(fixture.diff(&inventory, &output).status.success());
    assert_eq!(first, fs::read(output.join("scan.diff.v1.json")).unwrap());
    assert_eq!(fs::read_dir(&output).unwrap().count(), 2);
}
