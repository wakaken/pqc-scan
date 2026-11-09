use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn cli(input: &Path, rules: &Path, out: &Path, extra: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(input)
        .args(["--format", "all", "--threads", "1", "--rules-dir"])
        .arg(rules)
        .arg("--out-dir")
        .arg(out)
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn inventory_is_explicit_and_legacy_outputs_are_a_shadow_not_a_replacement() {
    let f = Fixture(std::env::temp_dir().join(format!("pqc025-cli-{}", std::process::id())));
    let input = f.0.join("input");
    let rules = f.0.join("rules");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(input.join("app.txt"), "ML-KEM-768\nML-KEM-768\n").unwrap();
    std::fs::write(
        rules.join("rule.yml"),
        serde_json::json!([{
            "id":"TEST_NATIVE", "kind":"regex", "category":"test", "severity":"medium",
            "risk":"quantum-uncertain", "confidence":0.5, "migration_hint":"Review fixture.",
            "pattern":"ML-KEM-768", "scope":"code"
        }])
        .to_string(),
    )
    .unwrap();
    let old = f.0.join("old");
    let new = f.0.join("new");
    assert!(cli(&input, &rules, &old, &[]).status.success());
    assert!(!old.join("scan.inventory.v1.json").exists());
    assert!(!old.join("scan.native.v1.json").exists());
    let result = cli(
        &input,
        &rules,
        &new,
        &["--inventory-json", "--repository-scope", "test/project"],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let native: Value =
        serde_json::from_slice(&std::fs::read(new.join("scan.inventory.v1.json")).unwrap())
            .unwrap();
    let schema: Value =
        serde_json::from_str(include_str!("../../../schemas/native-scan-v1.schema.json")).unwrap();
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .should_validate_formats(true)
        .build(&schema)
        .unwrap();
    assert!(
        validator.is_valid(&native),
        "{:?}",
        validator.iter_errors(&native).collect::<Vec<_>>()
    );
    let mut semantic = native.clone();
    let mut replacements = std::collections::BTreeMap::new();
    replacements.insert(
        native["run"]["id"].as_str().unwrap().to_string(),
        "RUN".to_string(),
    );
    for (i, o) in native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        replacements.insert(
            o["id"].as_str().unwrap().to_string(),
            format!("OBSERVATION-{i}"),
        );
    }
    for (i, l) in native["legacy_associations"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        replacements.insert(
            l["legacy_id"].as_str().unwrap().to_string(),
            format!("LEGACY-{i}"),
        );
    }
    fn normalize(v: &mut Value, map: &std::collections::BTreeMap<String, String>) {
        match v {
            Value::String(s) => {
                if let Some(r) = map.get(s) {
                    *s = r.clone();
                }
            }
            Value::Array(a) => {
                for x in a {
                    normalize(x, map)
                }
            }
            Value::Object(o) => {
                for x in o.values_mut() {
                    normalize(x, map)
                }
            }
            _ => {}
        }
    }
    semantic["run"]["generated_at"] = serde_json::json!("TIME");
    normalize(&mut semantic, &replacements);
    let golden: Value =
        serde_json::from_str(include_str!("fixtures/native-v1.golden.json")).unwrap();
    assert_eq!(semantic,golden,"reviewed semantic golden; only run/time and legacy root-sensitive IDs normalized, references preserved");
    assert_eq!(native["schema_version"], 1);
    assert_eq!(native["observations"].as_array().unwrap().len(), 2);
    assert_eq!(native["repository_scope"], "test/project");
    assert_eq!(native["assessment_status"], "not_evaluated");
    for file in [
        "report.json",
        "cbom.json",
        "dependency-sbom.json",
        "report.sarif",
    ] {
        let mut a: Value = serde_json::from_slice(&std::fs::read(old.join(file)).unwrap()).unwrap();
        let mut b: Value = serde_json::from_slice(&std::fs::read(new.join(file)).unwrap()).unwrap();
        if file == "report.json" {
            a.as_object_mut().unwrap().remove("generated_at");
            b.as_object_mut().unwrap().remove("generated_at");
        }
        assert_eq!(a, b, "{file}");
    }
    let no_scope = cli(
        &input,
        &rules,
        &f.0.join("bad"),
        &["--repository-scope", "project"],
    );
    assert!(!no_scope.status.success());
}

#[test]
fn native_private_markers_do_not_export_payload_and_remain_language_independent() {
    let f = Fixture(std::env::temp_dir().join(format!("pqc025-private-{}", std::process::id())));
    let input = f.0.join("input");
    std::fs::create_dir_all(&input).unwrap();
    let kinds = [
        "RSA PRIVATE KEY",
        "EC PRIVATE KEY",
        "DSA PRIVATE KEY",
        "ENCRYPTED PRIVATE KEY",
        "PRIVATE KEY",
        "OPENSSH PRIVATE KEY",
    ];
    for (i, kind) in kinds.iter().enumerate() {
        std::fs::write(
            input.join(format!("marker-{i}.pem")),
            format!("-----BEGIN {kind}-----\nPRIVATE_PAYLOAD_CANARY_{i}\n-----END {kind}-----\n"),
        )
        .unwrap();
    }
    let rules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules");
    for language in ["en", "ja"] {
        let out = f.0.join(language);
        let result = cli(
            &input,
            &rules,
            &out,
            &[
                "--native-json",
                "--repository-scope",
                "test/private",
                "--lang",
                language,
            ],
        );
        assert!(result.status.success());
        assert!(!String::from_utf8_lossy(&result.stderr).contains("CANARY"));
        assert!(!String::from_utf8_lossy(&result.stdout).contains("CANARY"));
        for name in [
            "report.json",
            "cbom.json",
            "dependency-sbom.json",
            "report.sarif",
            "report.html",
            "report.md",
            "scan.native.v1.json",
        ] {
            assert!(
                !std::fs::read_to_string(out.join(name))
                    .unwrap()
                    .contains("PRIVATE_PAYLOAD_CANARY"),
                "{name}"
            );
        }
        let n: Value =
            serde_json::from_slice(&std::fs::read(out.join("scan.native.v1.json")).unwrap())
                .unwrap();
        assert!(n["observations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["method"] == "private_key_marker"));
        assert!(!n["assets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["kind"] == "public_key"));
    }
}

#[test]
fn native_schema_and_references_cover_public_certificates_dependencies_and_rejected_package_paths()
{
    let f = Fixture(std::env::temp_dir().join(format!("pqc025-mixed-{}", std::process::id())));
    let input = f.0.join("input");
    std::fs::create_dir_all(&input).unwrap();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::copy(
        workspace.join("crates/pqc_scan_detectors/tests/fixtures/certificates/p256-rsa-signed.der"),
        input.join("public.der"),
    )
    .unwrap();
    std::fs::write(input.join("package.json"),r#"{"dependencies":{"test-package":"1.2.3-beta.1","/SECRET_CANARY/absolute":"1.0.0","foo//bar":"1.0.0"}}"#).unwrap();
    let out = f.0.join("out");
    let result = cli(&input, &workspace.join("rules"), &out, &["--native-json"]);
    assert!(result.status.success());
    let raw = std::fs::read_to_string(out.join("scan.native.v1.json")).unwrap();
    assert!(!raw.contains("CANARY"));
    assert!(!raw.contains("foo//bar"));
    let native: Value = serde_json::from_str(&raw).unwrap();
    let schema: Value =
        serde_json::from_str(include_str!("../../../schemas/native-scan-v1.schema.json")).unwrap();
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .build(&schema)
        .unwrap();
    assert!(
        validator.is_valid(&native),
        "{:?}",
        validator.iter_errors(&native).collect::<Vec<_>>()
    );
    serde_json::from_str::<pqc_scan_core::NativeScan>(&raw)
        .unwrap()
        .validate()
        .unwrap();
    assert_eq!(
        native["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["kind"] == "certificate")
            .count(),
        1
    );
    assert!(native["assets"].as_array().unwrap().iter().any(
        |a| a["kind"] == "dependency" && a["attributes"]["reported_version"] == "1.2.3-beta.1"
    ));
    let algorithms: Vec<_> = native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "algorithm")
        .collect();
    assert!(algorithms
        .iter()
        .any(|a| a["attributes"]["identifier"] == "1.2.840.113549.1.1.11"));
    assert!(algorithms
        .iter()
        .any(|a| a["attributes"]["identifier"] == "1.2.840.10045.2.1"));
}

#[test]
fn inventory_and_native_options_preserve_filenames_and_share_one_capture_when_combined() {
    let f = Fixture(std::env::temp_dir().join(format!("pqc025-names-{}", std::process::id())));
    let input = f.0.join("input");
    let rules = f.0.join("rules");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(input.join("app.txt"), "RSA\n").unwrap();
    std::fs::write(rules.join("rule.yml"),r#"[{"id":"TEST_RENAME","kind":"regex","category":"test","severity":"high","risk":"quantum-vulnerable","confidence":0.5,"migration_hint":"Review fixture.","pattern":"RSA","scope":"code"}]"#).unwrap();
    for (i, (flags, inventory, native)) in [
        (vec!["--inventory-json"], true, false),
        (vec!["--native-json"], false, true),
        (vec!["--inventory-json", "--native-json"], true, true),
    ]
    .into_iter()
    .enumerate()
    {
        for (j, (exit_flags, exit)) in [
            (vec![], 0),
            (vec!["--fail-on", "high"], 2),
            (vec!["--require-complete"], 3),
            (vec!["--require-complete", "--fail-on", "high"], 3),
        ]
        .into_iter()
        .enumerate()
        {
            let out = f.0.join(format!("{i}-{j}"));
            let mut args = flags.clone();
            args.extend(["--repository-scope", "test/project"]);
            args.extend(exit_flags);
            let result = cli(&input, &rules, &out, &args);
            assert_eq!(
                result.status.code(),
                Some(exit),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert_eq!(out.join("scan.inventory.v1.json").exists(), inventory);
            assert_eq!(out.join("scan.native.v1.json").exists(), native);
            if inventory && native {
                assert_eq!(
                    std::fs::read(out.join("scan.inventory.v1.json")).unwrap(),
                    std::fs::read(out.join("scan.native.v1.json")).unwrap(),
                    "same run and observation IDs, captured once"
                );
            }
            let filename = if inventory {
                "scan.inventory.v1.json"
            } else {
                "scan.native.v1.json"
            };
            let value: Value =
                serde_json::from_slice(&std::fs::read(out.join(filename)).unwrap()).unwrap();
            assert_eq!(value["repository_scope"], "test/project");
            assert_eq!(value["observations"].as_array().unwrap().len(), 1);
        }
    }
    let out = f.0.join("no-scope");
    let result = cli(&input, &rules, &out, &["--inventory-json"]);
    assert!(result.status.success());
    let value: Value =
        serde_json::from_slice(&std::fs::read(out.join("scan.inventory.v1.json")).unwrap())
            .unwrap();
    assert!(value["repository_scope"].is_null());
    let help = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .args(["scan", "--help"])
        .output()
        .unwrap();
    let help = format!(
        "{}{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    assert!(help.contains("--inventory-json"));
    assert!(help.contains("Cryptographic Inventory and Evidence Report"));
}
