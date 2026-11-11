use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn cyclonedx_flag_only_adds_two_documents_and_keeps_legacy_machine_content() {
    let root = Fixture(std::env::temp_dir().join(format!(
            "pqc034-cli-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )));
    fs::create_dir_all(root.0.join("input")).unwrap();
    fs::write(
        root.0.join("input/main.rs"),
        "fn main() { let algorithm = \"RS256\"; }\n",
    )
    .unwrap();
    let rules = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules");
    for extra in [false, true] {
        let output = root.0.join(if extra { "cdx" } else { "plain" });
        let mut command = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
        command
            .arg("scan")
            .arg(root.0.join("input"))
            .arg("--rules-dir")
            .arg(&rules)
            .arg("--out-dir")
            .arg(&output);
        if extra {
            command.arg("--cyclonedx-17");
        }
        let result = command.output().unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            fs::read_dir(&output).unwrap().count(),
            if extra { 8 } else { 6 }
        );
        assert!(!output.join("scan.inventory.v1.json").exists());
        for name in ["cbom.cdx-1.7.json", "dependency-sbom.cdx-1.7.json"] {
            assert_eq!(output.join(name).exists(), extra);
            if extra {
                let v: Value =
                    serde_json::from_slice(&fs::read(output.join(name)).unwrap()).unwrap();
                assert_eq!(v["bomFormat"], "CycloneDX");
                assert_eq!(v["specVersion"], "1.7");
            }
        }
    }
    for name in [
        "report.json",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
    ] {
        let read = |variant: &str| {
            let mut v: Value =
                serde_json::from_slice(&fs::read(root.0.join(variant).join(name)).unwrap())
                    .unwrap();
            if let Some(object) = v.as_object_mut() {
                object.remove("generated_at");
            }
            v
        };
        assert_eq!(read("plain"), read("cdx"), "{name}");
    }
}
