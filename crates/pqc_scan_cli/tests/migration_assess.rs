#[path = "../../pqc_scan_core/tests/support/migration/context.rs"]
mod ctx;
#[path = "../../pqc_scan_core/tests/support/migration/mod.rs"]
mod fixture;
use pqc_scan_core::migration::{InputFormat, SavedEvidence};
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let p = std::env::temp_dir().join(format!(
            "pqc-assess069-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn put(&self, name: &str, value: &Value) -> PathBuf {
        let p = self.0.join(name);
        std::fs::write(&p, serde_json::to_vec(value).unwrap()).unwrap();
        p
    }
    fn run(
        &self,
        input: &Path,
        format: &str,
        context: Option<&Path>,
        out: &Path,
        clock: &str,
    ) -> Output {
        let mut c = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
        c.arg("assess")
            .arg(input)
            .args(["--input-format", format, "--as-of", clock, "--out-dir"])
            .arg(out);
        if let Some(context) = context {
            c.arg("--context").arg(context);
        }
        c.output().unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn rsa() -> Value {
    let mut v = fixture::inventory("RSA");
    v["assets"][0]["attributes"]["parameters"] = json!({"key_bits":"2048"});
    v
}
fn report(out: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(out.join("migration.assessment.v1.json")).unwrap())
        .unwrap()
}
fn assert_ok(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
}
fn assert_error(output: &Output, code: &str) {
    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(code),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn actual_cli_reassesses_same_input_with_paired_contexts_and_validates_full_schema() {
    let f = Files::new();
    let input = f.put("saved.inventory.json", &rsa());
    let bytes = std::fs::read(&input).unwrap();
    let evidence = SavedEvidence::from_bytes(&bytes, InputFormat::InventoryV1).unwrap();
    let mut critical = ctx::critical_signing();
    critical["support"] = ctx::support();
    let mut ordinary = ctx::ordinary_signing();
    ordinary["support"] = ctx::support();
    let a = f.put("context-a.json", &ctx::context(&evidence, critical));
    let b = f.put("context-b.json", &ctx::context(&evidence, ordinary));
    for (context, name, urgency, action) in [
        (&a, "a", "urgent", "migrate"),
        (&b, "b", "plan", "plan"),
        (&a, "repeat", "urgent", "migrate"),
    ] {
        let out = f.0.join(name);
        assert_ok(&f.run(&input, "inventory-v1", Some(context), &out, ctx::AS_OF));
        let value = report(&out);
        assert_eq!(value["decisions"][0]["urgency"], urgency);
        assert_eq!(value["decisions"][0]["action"]["kind"], action);
        assert_eq!(value["input"]["sha256"], evidence.input().sha256);
        let schema: Value = serde_json::from_str(include_str!(
            "../../../schemas/migration-assessment-v1.schema.json"
        ))
        .unwrap();
        let validator = jsonschema::validator_for(&schema).unwrap();
        let errors = validator
            .iter_errors(&value)
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        assert!(errors.is_empty(), "{errors:?}");
        assert!(!out.join("report.json").exists());
        assert!(!out.join("report.html").exists());
        assert!(!out.join("cbom.json").exists());
        let mut bad = value.clone();
        bad["decisions"][0]["readiness"] = "verified_safe".into();
        assert!(!validator.is_valid(&bad));
        let mut bad = value.clone();
        bad["policy"]["version"] = "2.0.0".into();
        assert!(!validator.is_valid(&bad));
        let mut bad = value;
        bad["claims"][0]["SECRET_CANARY_069"] = true.into();
        assert!(!validator.is_valid(&bad));
    }
    assert_eq!(report(&f.0.join("a")), report(&f.0.join("repeat")));
    assert_ne!(
        report(&f.0.join("a"))["context"]["sha256"],
        report(&f.0.join("b"))["context"]["sha256"]
    );
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
}
#[test]
fn each_saved_format_works_in_the_actual_static_or_featured_binary_without_probing() {
    let f = Files::new();
    for (i, value, format) in [
        (0, fixture::inventory("RSA"), "inventory-v1"),
        (1, fixture::tls(), "tls-v1"),
        (2, fixture::failed_tls(), "tls-v1"),
        (3, fixture::offers(), "ssh-offers-v1"),
        (4, fixture::kex(), "ssh-kex-v1"),
    ] {
        let input = f.put(&format!("input{i}.json"), &value);
        let out = f.0.join(format!("out{i}"));
        assert_ok(&f.run(&input, format, None, &out, ctx::AS_OF));
        assert_eq!(report(&out)["input"]["format"], format);
        assert!(std::fs::read_to_string(out.join("migration.assessment.md"))
            .unwrap()
            .starts_with("# Organization migration review"));
        let expected = fixture::ingest(&value, format.parse().unwrap()).unwrap();
        assert_eq!(report(&out)["input"]["sha256"], expected.input().sha256);
    }
}
#[test]
fn invalid_context_versions_references_content_and_clock_fail_before_new_reports() {
    let f = Files::new();
    let input = f.put("input.json", &rsa());
    let e = SavedEvidence::read(&input, InputFormat::InventoryV1).unwrap();
    let original = ctx::context(&e, ctx::critical_signing());
    let mut wrong_digest = original.clone();
    wrong_digest["input"]["sha256"] = "a".repeat(64).into();
    let mut unknown = original.clone();
    unknown["bindings"][0]["target_id"] =
        pqc_scan_core::identity::native_id("migration_target", &["missing"]).into();
    let mut duplicate = original.clone();
    let row = duplicate["bindings"][0].clone();
    duplicate["bindings"].as_array_mut().unwrap().push(row);
    let mut invalid = original.clone();
    invalid["SECRET_CANARY_069"] = "SECRET_CANARY_069".into();
    let mut lead = original.clone();
    lead["bindings"][0]["claims"]["migration_lead_seconds"] = ctx::claim(json!(315576001));
    for (i, value, code) in [
        (0, wrong_digest, "input_binding_mismatch"),
        (1, unknown, "invalid_reference"),
        (2, duplicate, "ambiguous_binding"),
        (3, invalid, "invalid_context"),
        (4, lead, "invalid_context"),
    ] {
        let context = f.put(&format!("context{i}-SECRET_CANARY_069.json"), &value);
        let out = f.0.join(format!("bad{i}"));
        let output = f.run(&input, "inventory-v1", Some(&context), &out, ctx::AS_OF);
        assert_error(&output, code);
        assert!(!out.exists());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("CANARY"));
    }
    let out = f.0.join("clock");
    assert_error(
        &f.run(&input, "inventory-v1", None, &out, "2026-11-16T00:00:00Z"),
        "invalid_time",
    );
    assert!(!out.exists());
    let out = f.0.join("format");
    assert_error(
        &f.run(&input, "SECRET_CANARY_069", None, &out, ctx::AS_OF),
        "unsupported_format",
    );
    assert!(!out.exists());
}
#[test]
fn input_context_and_hardlink_aliases_are_rejected_without_mutation() {
    let f = Files::new();
    let out = f.0.join("alias");
    std::fs::create_dir_all(&out).unwrap();
    let input = out.join("migration.assessment.v1.json");
    let bytes = serde_json::to_vec(&rsa()).unwrap();
    std::fs::write(&input, &bytes).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", None, &out, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    let input = f.put("input.json", &rsa());
    let e = SavedEvidence::read(&input, InputFormat::InventoryV1).unwrap();
    let context = out.join("migration.assessment.v1.json");
    let context_bytes = serde_json::to_vec(&ctx::context(&e, ctx::critical_signing())).unwrap();
    std::fs::write(&context, &context_bytes).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", Some(&context), &out, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&context).unwrap(), context_bytes);
    std::fs::remove_file(&context).unwrap();
    std::fs::hard_link(&input, &context).unwrap();
    let bytes = std::fs::read(&input).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", None, &out, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    assert_eq!(std::fs::read(&context).unwrap(), bytes);
}
#[test]
fn no_endpoint_connection_no_input_execution_and_no_raw_metadata_output() {
    let f = Files::new();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let mut value = fixture::tls();
    value["target"]["host"] = "127.0.0.1".into();
    value["target"]["port"] = address.port().into();
    value["target"]["resolved_addresses"] = json!(["127.0.0.1"]);
    value["target"]["pinned_address"] = address.to_string().into();
    value["target"]["connected_peer"] = address.to_string().into();
    let input = f.put("saved-tls.json", &value);
    let out = f.0.join("tls-out");
    let e = SavedEvidence::read(&input, InputFormat::TlsV1).unwrap();
    let mut c = ctx::context(
        &e,
        json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment"))}),
    );
    c["references"][0]["https_uri"] = format!("https://{address}/never-fetch").into();
    c["bindings"][0]["claims"]["applicability"]["provenance"]["reference_id"] =
        "org-deadline".into();
    let context = f.put("offline-context.json", &c);
    assert_ok(&f.run(&input, "tls-v1", Some(&context), &out, ctx::AS_OF));
    assert!(report(&out)["sources"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s["id"] == "context:org-deadline"));
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    let mut v = rsa();
    let marker = f.0.join("NEVER_EXECUTE_069");
    v["arbitrary_metadata"] = json!({"secret":"SECRET_CANARY_069","command":format!("/usr/bin/touch {}", marker.display())});
    let input = f.put("untrusted.json", &v);
    let out = f.0.join("untrusted-out");
    let output = f.run(&input, "inventory-v1", None, &out, ctx::AS_OF);
    assert_ok(&output);
    for name in [
        "migration.assessment.v1.json",
        "migration.assessment.md",
        "migration.assessment.html",
    ] {
        assert!(!std::fs::read_to_string(out.join(name))
            .unwrap()
            .contains("CANARY"));
    }
    assert!(!f.0.join("NEVER_EXECUTE_069").exists());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("CANARY"));
}

#[test]
fn paired_markdown_language_preserves_json_and_default_reports_and_fixed_errors() {
    let f = Files::new();
    let input = f.put("input070.json", &rsa());
    let e = SavedEvidence::read(&input, InputFormat::InventoryV1).unwrap();
    let mut claims = ctx::critical_signing();
    claims["support"] = ctx::support();
    claims["support"]["peer"] = ctx::claim(json!(false));
    let context = f.put("context070.json", &ctx::context(&e, claims));
    let mut results = Vec::new();
    for lang in ["en", "ja"] {
        let out = f.0.join(lang);
        let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("assess")
            .arg(&input)
            .args([
                "--input-format",
                "inventory-v1",
                "--as-of",
                ctx::AS_OF,
                "--context",
            ])
            .arg(&context)
            .arg("--out-dir")
            .arg(&out)
            .args(["--lang", lang])
            .output()
            .unwrap();
        assert_ok(&output);
        let md = std::fs::read_to_string(out.join("migration.assessment.md")).unwrap();
        if lang == "ja" {
            assert!(md.starts_with("# 組織の暗号移行レポート"));
            assert!(md.contains("早急に対応"));
            assert!(md.contains("阻害条件を解消する"));
            assert!(md.contains("接続相手の対応"));
        } else {
            assert!(md.starts_with("# Organization migration review"));
            assert!(md.contains("Urgent"));
            assert!(md.contains("Resolve blocking prerequisites"));
        }
        assert!(!out.join("report.html").exists());
        assert!(!out.join("report.rules.html").exists());
        results.push(report(&out));
    }
    assert_eq!(results[0], results[1]);
    let out = f.0.join("default070");
    assert_ok(&f.run(&input, "inventory-v1", Some(&context), &out, ctx::AS_OF));
    assert_eq!(
        std::fs::read(out.join("migration.assessment.md")).unwrap(),
        std::fs::read(f.0.join("en/migration.assessment.md")).unwrap()
    );
    let invalid = f.0.join("bad-lang070");
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("assess")
        .arg(&input)
        .args([
            "--input-format",
            "inventory-v1",
            "--as-of",
            ctx::AS_OF,
            "--out-dir",
        ])
        .arg(&invalid)
        .args(["--lang", "SECRET_CANARY_070"])
        .output()
        .unwrap();
    assert_error(&output, "unsupported_language");
    assert!(!invalid.exists());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("CANARY"));
}
#[test]
fn markdown_input_and_context_aliases_reject_before_either_report_write() {
    let f = Files::new();
    let out = f.0.join("md-alias070");
    std::fs::create_dir_all(&out).unwrap();
    let input = out.join("migration.assessment.md");
    let bytes = serde_json::to_vec(&rsa()).unwrap();
    std::fs::write(&input, &bytes).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", None, &out, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    assert!(!out.join("migration.assessment.v1.json").exists());
    let input = f.put("normal070.json", &rsa());
    let e = SavedEvidence::read(&input, InputFormat::InventoryV1).unwrap();
    let context = out.join("migration.assessment.md");
    let bytes = serde_json::to_vec(&ctx::context(&e, ctx::critical_signing())).unwrap();
    std::fs::write(&context, &bytes).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", Some(&context), &out, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&context).unwrap(), bytes);
    assert!(!out.join("migration.assessment.v1.json").exists());
    std::fs::remove_file(&context).unwrap();
    std::fs::hard_link(&input, &context).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", None, &out, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&context).unwrap()
    );
    assert!(!out.join("migration.assessment.v1.json").exists());
}

#[test]
fn assess_emits_a_real_styled_html_report_and_guards_its_input_context_aliases() {
    let f = Files::new();
    let input = f.put("input-html.json", &rsa());
    let out = f.0.join("html-output");
    assert_ok(&f.run(&input, "inventory-v1", None, &out, ctx::AS_OF));
    let html = std::fs::read_to_string(out.join("migration.assessment.html")).unwrap();
    assert!(html.starts_with("<!doctype html>"));
    assert!(html.contains("Migration response work items"));
    assert!(html.contains("data-report-translation"));
    assert!(html.contains("--card:"));
    assert!(html.contains("decision-card"));
    assert!(!html.contains("\"claimed_support\":"));
    let alias = f.0.join("html-alias");
    std::fs::create_dir_all(&alias).unwrap();
    let saved = alias.join("migration.assessment.html");
    let bytes = serde_json::to_vec(&rsa()).unwrap();
    std::fs::write(&saved, &bytes).unwrap();
    assert_error(
        &f.run(&saved, "inventory-v1", None, &alias, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&saved).unwrap(), bytes);
    assert!(!alias.join("migration.assessment.md").exists());
    assert!(!alias.join("migration.assessment.v1.json").exists());
    let e = SavedEvidence::read(&input, InputFormat::InventoryV1).unwrap();
    let bytes = serde_json::to_vec(&ctx::context(&e, ctx::critical_signing())).unwrap();
    std::fs::write(&saved, &bytes).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", Some(&saved), &alias, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(std::fs::read(&saved).unwrap(), bytes);
    assert!(!alias.join("migration.assessment.v1.json").exists());
    std::fs::remove_file(&saved).unwrap();
    std::fs::hard_link(&input, &saved).unwrap();
    assert_error(
        &f.run(&input, "inventory-v1", None, &alias, ctx::AS_OF),
        "unsafe_output",
    );
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&saved).unwrap()
    );
    assert!(!alias.join("migration.assessment.v1.json").exists());
}
