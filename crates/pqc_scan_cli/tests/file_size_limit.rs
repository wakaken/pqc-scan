use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn run(root: &Path, out: &Path, limit: Option<&str>) -> std::process::Output {
    let mut c = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
    c.arg("scan")
        .arg(root)
        .args(["--format", "json", "--diagnostics", "--rules-dir"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .arg("--out-dir")
        .arg(out)
        .args(["--threads", "1"]);
    if let Some(limit) = limit {
        c.args(["--max-file-size-mib", limit]);
    }
    c.output().unwrap()
}
#[test]
fn size_option_changes_real_selection_and_keeps_existing_findings() {
    let f = Fixture(std::env::temp_dir().join(format!("pqc-size-{}", std::process::id())));
    fs::create_dir_all(f.0.join("input")).unwrap();
    fs::write(f.0.join("input/small.py"), "algorithm = 'RS256'\n").unwrap();
    let mut large = b"algorithm = 'RS512'\n#".to_vec();
    large.resize(2 * 1024 * 1024 + 1, b'x');
    fs::write(f.0.join("input/large.py"), large).unwrap();
    let mut results = Vec::new();
    for (name, limit, count) in [
        ("default", None, 1),
        ("explicit", Some("2"), 1),
        ("raised", Some("3"), 2),
    ] {
        let out = f.0.join(name);
        let r = run(&f.0.join("input"), &out, limit);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        let d: Value =
            serde_json::from_slice(&fs::read(out.join("diagnostics.json")).unwrap()).unwrap();
        assert_eq!(d["coverage"]["selected_files"], count);
        assert_eq!(
            d["scope"]["max_file_size_bytes"],
            if name == "raised" {
                3 * 1024 * 1024
            } else {
                2 * 1024 * 1024
            }
        );
        let report: Value =
            serde_json::from_slice(&fs::read(out.join("report.json")).unwrap()).unwrap();
        results.push(report);
    }
    let ids = |v: &Value| {
        v["findings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["finding_id"].as_str().unwrap().to_owned())
            .collect::<std::collections::BTreeSet<_>>()
    };
    assert_eq!(ids(&results[0]), ids(&results[1]));
    assert!(ids(&results[0]).is_subset(&ids(&results[2])));
    assert!(results[2]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["location"]["file"] == "large.py" && f["evidence"]["match"] == "RS512"));
}
#[test]
fn invalid_limits_fail_at_argument_parsing_without_creating_outputs() {
    let out = std::env::temp_dir().join(format!("pqc-size-invalid-{}", std::process::id()));
    for value in [
        "0",
        "65",
        "-1",
        "+2",
        "2.5",
        "2MiB",
        "999999999999999999999",
    ] {
        let r = run(
            Path::new("missing-source-must-not-be-read"),
            &out,
            Some(value),
        );
        assert_eq!(r.status.code(), Some(1), "{value}");
        assert!(!out.exists());
    }
    for value in ["1", "2", "16", "64"] {
        let r = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args(["serve", "--max-file-size-mib", value, "--help"])
            .output()
            .unwrap();
        assert!(r.status.success());
    }
}
