#[path = "../../pqc_scan_core/tests/support/migration/mod.rs"]
#[allow(dead_code)]
mod fixture;
use pqc_scan_core::migration::*;
use serde_json::{json, Value};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
const NOW: &str = "2026-11-18T00:00:00Z";
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let p = std::env::temp_dir().join(format!(
            "pqc-template073-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn input(&self, value: &Value) -> PathBuf {
        let p = self.0.join("input.json");
        std::fs::write(&p, serde_json::to_vec(value).unwrap()).unwrap();
        p
    }
    fn run(&self, input: &std::path::Path, format: &str, extra: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("context-template")
            .arg(input)
            .args(["--input-format", format])
            .args(extra)
            .output()
            .unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn ok(o: &Output) {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(o.stderr.is_empty());
}
#[test]
fn actual_headless_cli_list_sparse_template_and_assess_roundtrip() {
    let f = Files::new();
    let input = f.input(&fixture::inventory("RSA"));
    let original = std::fs::read(&input).unwrap();
    let list = f.run(&input, "inventory-v1", &["--list-targets"]);
    ok(&list);
    let text = String::from_utf8(list.stdout).unwrap();
    assert!(text.starts_with("1\tRSA\t"));
    assert!(text.contains("app.txt"));
    assert!(text.contains("selectable"));
    let out = f.0.join("context.json");
    ok(&f.run(
        &input,
        "inventory-v1",
        &[
            "--out",
            out.to_str().unwrap(),
            "--as-of",
            NOW,
            "--target",
            "1",
            "--set",
            "usage_state=\"active\"",
            "--set",
            "business_critical=false",
            "--unknown",
            "confidential",
            "--actor",
            "owner",
            "--asserted-at",
            "2026-11-17T00:00:00Z",
        ],
    ));
    let bytes = std::fs::read(&out).unwrap();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["bindings"].as_array().unwrap().len(), 1);
    assert_eq!(
        v["bindings"][0]["claims"]["business_critical"]["value"],
        false
    );
    assert_eq!(
        v["bindings"][0]["claims"]["confidential"]["state"],
        "unknown"
    );
    assert!(v["bindings"][0]["claims"].get("use_case").is_none());
    let e = SavedEvidence::read(&input, InputFormat::InventoryV1).unwrap();
    let c = MigrationContext::read(&out).unwrap();
    let expected = evaluate(&e, Some(&c), NOW).unwrap();
    let report = f.0.join("report");
    let run = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("assess")
        .arg(&input)
        .args(["--input-format", "inventory-v1", "--context"])
        .arg(&out)
        .args(["--as-of", NOW, "--out-dir"])
        .arg(&report)
        .output()
        .unwrap();
    ok(&run);
    let actual: Value = serde_json::from_slice(
        &std::fs::read(report.join("migration.assessment.v1.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(actual, serde_json::to_value(expected).unwrap());
    assert_eq!(std::fs::read(input).unwrap(), original);
}
#[test]
fn no_selection_is_unbound_and_each_adopted_format_works() {
    let f = Files::new();
    for (i, (v, format)) in [
        (fixture::inventory("RSA"), "inventory-v1"),
        (fixture::tls(), "tls-v1"),
        (fixture::offers(), "ssh-offers-v1"),
        (fixture::kex(), "ssh-kex-v1"),
    ]
    .into_iter()
    .enumerate()
    {
        let input = f.input(&v);
        let out = f.0.join(format!("{i}.json"));
        ok(&f.run(
            &input,
            format,
            &["--out", out.to_str().unwrap(), "--as-of", NOW],
        ));
        let c: Value = serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
        assert_eq!(c["bindings"], json!([]));
        assert_eq!(c["systems"], json!([]));
        ok(&f.run(
            &input,
            format,
            &[
                "--out",
                out.to_str().unwrap(),
                "--as-of",
                NOW,
                "--target",
                "1",
            ],
        ));
        let e = SavedEvidence::read(&input, format.parse().unwrap()).unwrap();
        MigrationContext::read(&out).unwrap().bind(&e, NOW).unwrap();
    }
}
#[test]
fn invalid_answers_targets_and_times_leave_existing_output_untouched() {
    let f = Files::new();
    let input = f.input(&fixture::inventory("RSA"));
    let out = f.0.join("keep.json");
    std::fs::write(&out, b"KEEP").unwrap();
    for args in [
        vec!["--target", "0"],
        vec!["--target", "999"],
        vec!["--target", "1", "--target", "1"],
        vec![
            "--target",
            "1",
            "--set",
            "business_critical=\"SECRET_CANARY_073\"",
            "--actor",
            "owner",
            "--asserted-at",
            NOW,
        ],
        vec!["--target", "1", "--unknown", "usage_state"],
        vec!["--set", "usage_state=\"active\""],
        vec![
            "--target",
            "1",
            "--set",
            "confidential=null",
            "--actor",
            "owner",
            "--asserted-at",
            NOW,
        ],
    ] {
        let mut extra = vec!["--out", out.to_str().unwrap(), "--as-of", NOW];
        extra.extend(args);
        let o = f.run(&input, "inventory-v1", &extra);
        assert_eq!(o.status.code(), Some(1));
        assert!(!String::from_utf8_lossy(&o.stderr).contains("SECRET_CANARY_073"));
        assert_eq!(std::fs::read(&out).unwrap(), b"KEEP");
    }
}
#[test]
fn output_name_inode_and_link_boundaries_preserve_input() {
    let f = Files::new();
    let input = f.input(&fixture::inventory("RSA"));
    let original = std::fs::read(&input).unwrap();
    let hard = f.0.join("hard.json");
    std::fs::hard_link(&input, &hard).unwrap();
    for out in [&input, &hard] {
        let o = f.run(
            &input,
            "inventory-v1",
            &[
                "--out",
                out.to_str().unwrap(),
                "--as-of",
                NOW,
                "--target",
                "1",
            ],
        );
        assert_eq!(o.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&o.stderr).contains("unsafe_output"));
    }
    #[cfg(unix)]
    {
        let linked = f.0.join("link.json");
        std::os::unix::fs::symlink(&input, &linked).unwrap();
        assert!(!f
            .run(
                &input,
                "inventory-v1",
                &["--out", linked.to_str().unwrap(), "--as-of", NOW]
            )
            .status
            .success());
    }
    assert_eq!(std::fs::read(&input).unwrap(), original);
    assert_eq!(std::fs::read(hard).unwrap(), original);
}
#[test]
fn raw_nested_duplicate_cli_answers_are_rejected_before_any_write() {
    let f = Files::new();
    let input = f.input(&fixture::inventory("RSA"));
    let out = f.0.join("keep.json");
    std::fs::write(&out, b"KEEP").unwrap();
    let result = f.run(
        &input,
        "inventory-v1",
        &[
            "--out",
            out.to_str().unwrap(),
            "--as-of",
            NOW,
            "--target",
            "1",
            "--set",
            r#"deadline={"kind":"at","kind":"none"}"#,
            "--actor",
            "owner",
            "--asserted-at",
            NOW,
        ],
    );
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(std::fs::read(&out).unwrap(), b"KEEP");
}
