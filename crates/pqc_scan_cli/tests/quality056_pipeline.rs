//! End-to-end source-fact observations. Fixture programs are never executed.
use serde_json::{json, Value};
use std::{
    path::{Component, Path},
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Scratch(std::path::PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
#[test]
fn source_and_dependency_facts_never_become_execution_or_deployment_proof() {
    let oracle: Value = serde_json::from_str(include_str!(
        "../../../tests/quality/judgment/pipeline.json"
    ))
    .unwrap();
    let mut rows = vec![];
    for case in oracle["cases"].as_array().unwrap() {
        let temp = Scratch(std::env::temp_dir().join(format!(
            "pqc056-pipeline-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )));
        std::fs::create_dir(&temp.0).unwrap();
        let input = temp.0.join("input");
        std::fs::create_dir(&input).unwrap();
        for (name, text) in case["input"]["files"].as_object().unwrap() {
            assert!(Path::new(name)
                .components()
                .all(|p| matches!(p, Component::Normal(_))));
            let path = input.join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .and_then(|mut f| {
                    use std::io::Write;
                    f.write_all(text.as_str().unwrap().as_bytes())
                })
                .unwrap();
        }
        let out = temp.0.join("output");
        let rules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules");
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(&input)
            .args([
                "--format",
                "json",
                "--inventory-json",
                "--policy-profile",
                "migration-review-v1",
                "--threads",
                "2",
                "--rules-dir",
            ])
            .arg(rules)
            .arg("--out-dir")
            .arg(&out)
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "scanner fixture failed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let native = read(&out.join("scan.inventory.v1.json"));
        let assessment = read(&out.join("scan.assessment.v1.json"));
        let mut declared = vec![];
        let mut resolved = vec![];
        let mut dependency_ids = std::collections::BTreeSet::new();
        for a in native["assets"].as_array().unwrap() {
            if a["kind"] != "dependency" {
                continue;
            }
            dependency_ids.insert(a["id"].as_str().unwrap());
            let attrs = &a["attributes"];
            let name = attrs["name"].as_str().unwrap();
            if let Some(version) = attrs["declared_constraint"].as_str() {
                declared.push(format!("{name}:{version}"));
            }
            if let Some(version) = attrs["resolved_version"].as_str() {
                resolved.push(format!("{name}:{version}"));
            }
        }
        declared.sort();
        resolved.sort();
        let observations = native["observations"].as_array().unwrap();
        let active = observations
            .iter()
            .filter(|o| o["usage"] == "executed" || o["usage"] == "active")
            .count();
        let trust = observations
            .iter()
            .filter(|o| {
                o["assurance"] == "cryptographically_verified" || o["assurance"] == "authenticated"
            })
            .count();
        let passes = assessment["asset_policies"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| {
                dependency_ids.contains(p["asset_id"].as_str().unwrap()) && p["decision"] == "pass"
            })
            .count();
        rows.push(json!({"id":case["id"],"actual":{"native":native,"assessment":assessment,"summary":{"declared":declared,"resolved":resolved,"activeUseClaims":active,"verifiedTrustClaims":trust,"dependencyPasses":passes},"commandExit":result.status.code()}}));
    }
    println!(
        "QUALITY056_PIPELINE:{}",
        serde_json::to_string(&rows).unwrap()
    );
}
