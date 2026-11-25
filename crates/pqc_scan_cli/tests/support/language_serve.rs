//! Actual language scan -> held inventory -> scoped web context -> CLI comparison.
use super::*;
#[path = "language_input.rs"]
mod input;
use pqc_scan_core::migration::{InputFormat, SavedEvidence};

#[test]
fn new_language_scan_template_scoped_web_and_bilingual_offline_cli_agree() {
    let s = Session::start_with(true);
    let source = s.root.join("project & 日");
    fs::remove_file(source.join("main.py")).unwrap();
    input::write(&source);
    let route = format!("{}api/scan?path=project%20%26%20%E6%97%A5", s.prefix);
    assert_eq!(
        s.request("POST", &route, &s.host, Some(&format!("http://{}", s.host)))
            .0,
        202
    );
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let jobs: Value = serde_json::from_str(&s.get("api/jobs").1).unwrap();
        if jobs["jobs"][0]["status"] != "running" {
            assert_eq!(jobs["jobs"][0]["status"], "ready", "{jobs}");
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    let native = s.reports.join("1/scan.inventory.v1.json");
    let bytes = fs::read(&native).unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains(input::CANARY));
    let evidence = SavedEvidence::from_bytes(&bytes, InputFormat::InventoryV1).unwrap();
    // Assert all source families and dependencies before selecting a target.
    for (path, algorithm) in [
        ("kem.php", "ML-KEM-768"),
        ("signature.cs", "ML-DSA-65"),
        ("signature.c", "SLH-DSA-SHA2-128s"),
        ("signature.cpp", "ML-DSA-65"),
        ("rsa.php", "RSA"),
        ("unknown.c", "unknown"),
    ] {
        let ts: Vec<_> = evidence
            .targets()
            .iter()
            .filter(|t| t.source_ref.path.as_deref() == Some(path))
            .collect();
        assert_eq!(ts.len(), 1, "{path}: {ts:?}");
        assert_eq!(ts[0].technical.algorithm.identifier, algorithm);
        assert_eq!(ts[0].observation.assurance, "syntactic");
        assert_eq!(ts[0].observation.usage, "referenced");
    }
    for provenance in [
        "platform_required",
        "platform_emulation_disabled",
        "manifest_dev_declared",
        "lock_resolved",
        "manifest_declared",
    ] {
        assert!(
            evidence
                .targets()
                .iter()
                .any(|t| t.source_ref.dependency_provenance.as_deref() == Some(provenance)),
            "{provenance}"
        );
    }
    let scan: pqc_scan_core::model::ScanResult =
        serde_json::from_slice(&fs::read(s.reports.join("1/report.json")).unwrap()).unwrap();
    let as_of = (scan.generated_at + Duration::from_secs(2))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    let expiry = format!("{}-01-01T00:00:00Z", as_of[..4].parse::<u32>().unwrap() + 1);
    let list = targets(&s, 1);
    let index = evidence
        .targets()
        .iter()
        .position(|t| t.source_ref.path.as_deref() == Some("rsa.php"))
        .unwrap()
        + 1;
    // The real headless helper lists and binds only the explicitly selected target.
    let listed = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("context-template")
        .arg(&native)
        .args(["--input-format", "inventory-v1", "--list-targets"])
        .output()
        .unwrap();
    assert!(listed.status.success());
    let lines = String::from_utf8(listed.stdout).unwrap();
    assert_eq!(lines.lines().count(), evidence.targets().len());
    for p in [
        "kem.php",
        "signature.cs",
        "signature.c",
        "signature.cpp",
        "rsa.php",
    ] {
        assert!(lines.contains(p));
    }
    let template = s.root.join("sparse-context.json");
    let made = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("context-template")
        .arg(&native)
        .args(["--input-format", "inventory-v1", "--out"])
        .arg(&template)
        .args([
            "--as-of",
            &as_of,
            "--target",
            &index.to_string(),
            "--set",
            "usage_state=\"active\"",
            "--unknown",
            "support.runtime",
            "--actor",
            "owner",
            "--asserted-at",
            &as_of,
            "--valid-until",
            &expiry,
        ])
        .output()
        .unwrap();
    assert!(
        made.status.success(),
        "{}",
        String::from_utf8_lossy(&made.stderr)
    );
    let sparse: Value = serde_json::from_slice(&fs::read(&template).unwrap()).unwrap();
    assert_eq!(sparse["bindings"].as_array().unwrap().len(), 1);
    assert_eq!(
        sparse["bindings"][0]["claims"]["support"]["runtime"]["state"],
        "unknown"
    );
    assert!(sparse["bindings"][0]["claims"].get("use_case").is_none());
    let mut request = generated_request(list["input"]["sha256"].as_str().unwrap());
    request["as_of"] = as_of.clone().into();
    request["asserted_at"] = as_of.clone().into();
    request["valid_until"] = expiry.into();
    request["selected"] = json!([index]);
    request["answers"][2]["value"] = true.into();
    request["answers"]
        .as_array_mut()
        .unwrap()
        .push(json!({"field":"applicability","state":"known","value":"applicable"}));
    request["answers"]
        .as_array_mut()
        .unwrap()
        .push(json!({"field":"support.runtime","state":"known","value":false}));
    let p = preview(&s, 1, &request);
    assert_eq!(p["scope"].as_array().unwrap().len(), 1);
    assert_eq!(p["scope"][0]["number"], index);
    let context = s.root.join("context.json");
    fs::write(&context, p["context"].as_str().unwrap()).unwrap();
    // No source input remains available to either assessment path.
    fs::remove_dir_all(&source).unwrap();
    let result = confirm(&s, 1, p["token"].as_str().unwrap());
    let (status, body) = s.get(result["json"].as_str().unwrap());
    assert_eq!(status, 200);
    let web: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(
        web["decisions"].as_array().unwrap().len(),
        evidence.targets().len()
    );
    let focal = &web["decisions"][index - 1];
    assert_eq!(focal["readiness"], "blocked");
    assert_eq!(focal["action"]["kind"], "unblock");
    for (i, d) in web["decisions"].as_array().unwrap().iter().enumerate() {
        if i != index - 1 {
            assert_ne!(d["readiness"], "ready");
            assert!(d["system_id"].is_null());
        }
    }
    assert_eq!(
        s.get("reports/1/migration-context.json").1,
        p["context"].as_str().unwrap()
    );
    for language in ["en", "ja"] {
        let out = s.root.join(language);
        let cli = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("assess")
            .arg(&native)
            .args(["--input-format", "inventory-v1", "--context"])
            .arg(&context)
            .args(["--as-of", &as_of, "--lang", language, "--out-dir"])
            .arg(&out)
            .output()
            .unwrap();
        assert!(
            cli.status.success(),
            "{}",
            String::from_utf8_lossy(&cli.stderr)
        );
        let actual: Value =
            serde_json::from_slice(&fs::read(out.join("migration.assessment.v1.json")).unwrap())
                .unwrap();
        assert_eq!(actual, web);
        for file in [
            "migration.assessment.v1.json",
            "migration.assessment.md",
            "migration.assessment.html",
        ] {
            let text = fs::read_to_string(out.join(file)).unwrap();
            assert!(!text.contains(input::CANARY));
            assert!(
                text.replace("&#46;", ".").contains("signature.cpp"),
                "{language}/{file}"
            );
            assert!(
                text.replace("&#46;", ".").contains("kem.php"),
                "{language}/{file}"
            );
        }
        let md = fs::read_to_string(out.join("migration.assessment.md")).unwrap();
        assert!(md.contains(if language == "ja" {
            "プラットフォームの仮想設定を明示的に無効化"
        } else {
            "Platform emulation explicitly disabled"
        }));
    }
    assert_eq!(fs::read(&native).unwrap(), bytes);
    // A new job with even whitespace-only raw replacement cannot inherit this binding.
    let mut replaced = bytes.clone();
    replaced.push(b'\n');
    let id = import(&s, &serde_json::from_slice::<Value>(&replaced).unwrap());
    let replaced_list = targets(&s, id);
    assert_ne!(replaced_list["input"]["sha256"], list["input"]["sha256"]);
    let stale = json!({"mode":"import_context","input_sha256":replaced_list["input"]["sha256"],"as_of":as_of,"context":p["context"]});
    assert_eq!(
        s.post(
            &format!("api/migration/preview?job={id}"),
            &stale.to_string()
        )
        .0,
        400
    );
    // CLI binds the exact raw file too, not a target's name or position.
    let changed = s.root.join("changed.inventory.json");
    fs::write(&changed, &replaced).unwrap();
    let rejected = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("assess")
        .arg(&changed)
        .args(["--input-format", "inventory-v1", "--context"])
        .arg(&context)
        .args(["--as-of", &as_of, "--out-dir"])
        .arg(s.root.join("rejected"))
        .output()
        .unwrap();
    assert_eq!(rejected.status.code(), Some(1));
    assert!(!String::from_utf8_lossy(&rejected.stderr).contains(input::CANARY));
}
