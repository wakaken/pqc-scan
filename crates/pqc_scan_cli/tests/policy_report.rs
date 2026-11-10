use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
struct Fixture(PathBuf);
impl Fixture {
    fn new(tag: &str) -> Self {
        let p = std::env::temp_dir().join(format!("pqc026-{}-{tag}", std::process::id()));
        std::fs::create_dir_all(p.join("input")).unwrap();
        std::fs::create_dir_all(p.join("rules")).unwrap();
        std::fs::write(p.join("input/app.txt"), "ML-KEM-768\nML-DSA-65\nRS256\n").unwrap();
        std::fs::write(p.join("rules/test.yml"),r#"[{"id":"POLICY_TEST","kind":"regex","category":"test","severity":"medium","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"ML-KEM-768|ML-DSA-65|RS256","scope":"code"}]"#).unwrap();
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        std::fs::copy(
            workspace
                .join("crates/pqc_scan_detectors/tests/fixtures/certificates/p256-rsa-signed.der"),
            p.join("input/public.der"),
        )
        .unwrap();
        Self(p)
    }
    fn run(&self, out: &str, extra: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(self.0.join("input"))
            .args(["--format", "all", "--threads", "1", "--out-dir"])
            .arg(self.0.join(out))
            .arg("--rules-dir")
            .arg(self.0.join("rules"))
            .args(extra)
            .output()
            .unwrap()
    }
    fn read(&self, out: &str, file: &str) -> Value {
        serde_json::from_slice(&std::fs::read(self.0.join(out).join(file)).unwrap()).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn explicit_policy_writes_a_separate_linked_report_without_changing_inventory_or_legacy() {
    let f = Fixture::new("report");
    assert!(f.run("plain", &["--inventory-json"]).status.success());
    assert!(!f.0.join("plain/scan.assessment.v1.json").exists());
    let r = f.run(
        "policy",
        &[
            "--inventory-json",
            "--policy-profile",
            "migration-review-v1",
            "--repository-scope",
            "test",
        ],
    );
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let n = f.read("policy", "scan.inventory.v1.json");
    assert_eq!(n["assessment_status"], "not_evaluated");
    assert_eq!(n["assessments"], serde_json::json!([]));
    let a = f.read("policy", "scan.assessment.v1.json");
    assert_eq!(a["evaluation"]["decision"], "fail");
    assert_eq!(a["inventory"]["run_id"], n["run"]["id"]);
    assert_eq!(a["profile"]["id"], "migration-review-v1");
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/assessment-v1.schema.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let v = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .build(&schema)
        .unwrap();
    assert!(
        v.is_valid(&a),
        "{:?}",
        v.iter_errors(&a).collect::<Vec<_>>()
    );
    let ids: std::collections::BTreeSet<_> = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    let obs: std::collections::BTreeSet<_> = n["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    for item in a["assessments"].as_array().unwrap() {
        assert!(ids.contains(item["asset_id"].as_str().unwrap()));
        for o in item["observations"].as_array().unwrap() {
            assert!(obs.contains(o["id"].as_str().unwrap()));
        }
    }
    let cert = a["assessments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["asset_kind"] == "certificate")
        .unwrap();
    assert_eq!(cert["algorithm"]["components"].as_array().unwrap().len(), 2);
    assert_eq!(cert["algorithm"]["quantum"], "vulnerable");
    for file in [
        "report.json",
        "cbom.json",
        "dependency-sbom.json",
        "report.sarif",
    ] {
        let mut x = f.read("plain", file);
        let mut y = f.read("policy", file);
        if file == "report.json" {
            x.as_object_mut().unwrap().remove("generated_at");
            y.as_object_mut().unwrap().remove("generated_at");
        }
        assert_eq!(x, y);
    }
    for flags in [
        vec!["--policy-profile", "migration-review-v1"],
        vec!["--inventory-json", "--policy-profile", "invalid"],
    ] {
        assert!(!f.run("rejected", &flags).status.success());
        assert!(!f.0.join("rejected").exists());
    }
}
#[test]
fn strict_context_cannot_echo_sensitive_values_or_create_reports_on_error() {
    let f = Fixture::new("context");
    let path = f.0.join("context.json");
    for content in [
        b"{\"version\":1,\"SECRET_CANARY\":true}".to_vec(),
        b"{\"version\":1,\"version\":1}".to_vec(),
        vec![b' '; 16385],
        vec![255u8],
    ] {
        std::fs::write(&path, content).unwrap();
        let r = f.run(
            "bad",
            &[
                "--inventory-json",
                "--policy-profile",
                "migration-review-v1",
                "--assessment-context",
                path.to_str().unwrap(),
            ],
        );
        assert!(!r.status.success());
        assert!(!String::from_utf8_lossy(&r.stderr).contains("CANARY"));
        assert!(!f.0.join("bad").exists());
    }
    assert!(!f
        .run(
            "without-policy",
            &["--assessment-context", path.to_str().unwrap()]
        )
        .status
        .success());
    std::fs::write(&path,r#"{"version":1,"use_case":"jwt","support":{"runtime":true,"provider":true,"peer":true,"protocol":true,"parameters":true,"jose":true,"verifiers":true}}"#).unwrap();
    let r = f.run(
        "ready",
        &[
            "--inventory-json",
            "--policy-profile",
            "migration-review-v1",
            "--assessment-context",
            path.to_str().unwrap(),
        ],
    );
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let a = f.read("ready", "scan.assessment.v1.json");
    assert_eq!(a["context"]["provenance"], "operator_supplied_unverified");
    for (algorithm, decision) in [("ML-DSA-65", "pass"), ("ML-KEM-768", "indeterminate")] {
        let target = a["assessments"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["asset_kind"] == "algorithm" && a["algorithm"]["identifier"] == algorithm)
            .unwrap();
        let p = a["asset_policies"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["asset_id"] == target["asset_id"])
            .unwrap();
        assert_eq!(p["decision"], decision);
    }
}

#[test]
fn advisory_golden_is_repeatable_and_does_not_pass_with_incomplete_scope() {
    let f = Fixture::new("golden");
    std::fs::write(f.0.join("input/app.txt"), "ML-DSA-65\n").unwrap();
    std::fs::remove_file(f.0.join("input/public.der")).unwrap();
    let golden: Value =
        serde_json::from_str(include_str!("fixtures/assessment-v1.golden.json")).unwrap();
    for out in ["one", "two"] {
        let result = f.run(
            out,
            &[
                "--inventory-json",
                "--policy-profile",
                "migration-review-v1",
                "--repository-scope",
                "golden",
            ],
        );
        assert!(result.status.success());
        let mut report = f.read(out, "scan.assessment.v1.json");
        assert_eq!(report["evaluation"]["decision"], "indeterminate");
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            report["inventory"]["run_id"].as_str().unwrap().to_string(),
            "RUN".to_string(),
        );
        for (i, a) in report["assessments"].as_array().unwrap().iter().enumerate() {
            map.insert(
                a["id"].as_str().unwrap().to_string(),
                format!("ASSESSMENT-{i}"),
            );
        }
        for (i, o) in report["assessments"][0]["observations"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            map.insert(
                o["id"].as_str().unwrap().to_string(),
                format!("OBSERVATION-{i}"),
            );
        }
        fn normalize(v: &mut Value, map: &std::collections::BTreeMap<String, String>) {
            match v {
                Value::String(s) => {
                    if let Some(r) = map.get(s) {
                        *s = r.clone()
                    }
                }
                Value::Array(a) => {
                    for i in a {
                        normalize(i, map)
                    }
                }
                Value::Object(o) => {
                    for i in o.values_mut() {
                        normalize(i, map)
                    }
                }
                _ => {}
            }
        }
        normalize(&mut report, &map);
        assert_eq!(
            report, golden,
            "full report except explicit run-scoped identities; stable asset IDs retained"
        );
    }
}
