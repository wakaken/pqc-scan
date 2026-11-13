//! TASK-048: real offline exports, independent source facts and destructive controls.
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

struct Workspace(PathBuf);
static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
impl Workspace {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "pqc048-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        assert!(!p.exists());
        fs::create_dir_all(p.join("input")).unwrap();
        Self(p)
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn rows(v: &Value) -> &[Value] {
    v.as_array().unwrap()
}
fn validator(name: &str) -> jsonschema::Validator {
    let mut options = jsonschema::options().should_validate_formats(true);
    if name.contains("sarif") {
        options = options.with_draft(jsonschema::Draft::Draft4);
    }
    if name.contains("bom-1.7") {
        for n in [
            "spdx.schema.json",
            "jsf-0.82.schema.json",
            "cryptography-defs.schema.json",
        ] {
            options = options.with_resource(
                format!("http://cyclonedx.org/schema/{n}"),
                jsonschema::Resource::from_contents(read(
                    &root().join("schemas/cyclonedx/1.7").join(n),
                ))
                .unwrap(),
            );
        }
    }
    options.build(&read(&root().join(name))).unwrap()
}
fn check(file: &Path, schema: &str, pointer: &str, invalid: Value) -> Value {
    let value = read(file);
    let validator = validator(schema);
    assert!(
        validator.is_valid(&value),
        "{}: {:?}",
        file.display(),
        validator.iter_errors(&value).collect::<Vec<_>>()
    );
    let mut broken = value.clone();
    *broken.pointer_mut(pointer).unwrap() = invalid;
    assert!(
        !validator.is_valid(&broken),
        "negative control accepted: {} {pointer}",
        file.display()
    );
    json!({"file":file.file_name().unwrap().to_str(),"schema":schema,"positive":true,"negativePointer":pointer,"negativeRejected":true})
}
fn retain(name: &str, dir: &Path) {
    if let Some(out) = std::env::var_os("PQC048_EVIDENCE") {
        let out = PathBuf::from(out).join(name);
        fs::create_dir_all(&out).unwrap();
        for e in fs::read_dir(dir).unwrap() {
            let e = e.unwrap();
            if e.file_type().unwrap().is_file() {
                fs::copy(e.path(), out.join(e.file_name())).unwrap();
            }
        }
    }
}

#[test]
fn real_exports_preserve_facts_and_validate_offline_with_negative_controls() {
    let w = Workspace::new();
    let input = w.0.join("input");
    fs::write(
        input.join("app.py"),
        "from Crypto.PublicKey import RSA\na=RSA.generate(2048)\nb=RSA.generate(2048)\n",
    )
    .unwrap();
    fs::write(input.join("marker.txt"), "-----BEGIN RSA PRIVATE KEY-----\nSYNTHETIC048_NOT_PRIVATE_KEY_BYTES\n-----END RSA PRIVATE KEY-----\n").unwrap();
    fs::write(
        input.join("Cargo.toml"),
        "[dependencies]\nopenssl = \"^0.10\"\n",
    )
    .unwrap();
    fs::write(input.join("Cargo.lock"), "version = 3\n[[package]]\nname = \"openssl\"\nversion = \"0.10.75\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n").unwrap();
    fs::copy(
        root().join("crates/pqc_scan_detectors/tests/fixtures/certificates/rsa2048.der"),
        input.join("public.der"),
    )
    .unwrap();
    let mut schema_cases = vec![];
    let mut scans = vec![];
    for lang in ["en", "ja"] {
        let relocated = w.0.join(format!("input-{lang}"));
        fs::create_dir_all(&relocated).unwrap();
        for e in fs::read_dir(&input).unwrap() {
            let e = e.unwrap();
            fs::copy(e.path(), relocated.join(e.file_name())).unwrap();
        }
        let out = w.0.join(lang);
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(&relocated)
            .args([
                "--format",
                "all",
                "--inventory-json",
                "--diagnostics",
                "--cyclonedx-17",
                "--policy-profile",
                "migration-review-v1",
                "--repository-scope",
                "quality/048",
                "--lang",
                lang,
                "--rules-dir",
            ])
            .arg(root().join("rules"))
            .arg("--out-dir")
            .arg(&out)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        for (file, schema, pointer, invalid) in [
            (
                "scan.inventory.v1.json",
                "schemas/native-scan-v1.schema.json",
                "/schema_version",
                json!(2),
            ),
            (
                "scan.assessment.v1.json",
                "schemas/assessment-v1.schema.json",
                "/schema_version",
                json!(2),
            ),
            (
                "report.sarif",
                "crates/pqc_scan_report/tests/schema/sarif-schema-2.1.0.json",
                "/version",
                json!("9.0"),
            ),
            (
                "cbom.cdx-1.7.json",
                "schemas/cyclonedx/1.7/bom-1.7.schema.json",
                "/components",
                json!("not-an-array"),
            ),
            (
                "dependency-sbom.cdx-1.7.json",
                "schemas/cyclonedx/1.7/bom-1.7.schema.json",
                "/bomFormat",
                json!("legacy"),
            ),
        ] {
            schema_cases.push(check(&out.join(file), schema, pointer, invalid));
        }
        let legacy = read(&out.join("report.json"));
        let native = read(&out.join("scan.inventory.v1.json"));
        let sarif = read(&out.join("report.sarif"));
        let cbom = read(&out.join("cbom.json"));
        assert!(cbom.is_array());
        assert!(!validator("schemas/cyclonedx/1.7/bom-1.7.schema.json").is_valid(&cbom));
        assert_eq!(cbom, legacy["cbom"]);
        assert_eq!(
            read(&out.join("dependency-sbom.json")),
            legacy["dependency_sbom"]
        );
        let findings = rows(&legacy["findings"]);
        assert!(!findings.is_empty());
        assert_eq!(rows(&sarif["runs"][0]["results"]).len(), findings.len());
        let html = fs::read_to_string(out.join("report.html")).unwrap();
        let md = fs::read_to_string(out.join("report.md")).unwrap();
        let ids: BTreeSet<_> = findings
            .iter()
            .map(|f| f["finding_id"].as_str().unwrap())
            .collect();
        assert_eq!(ids.len(), findings.len());
        assert_eq!(md.matches("<a id=\"finding-").count(), findings.len());
        for f in findings {
            let id = f["finding_id"].as_str().unwrap();
            assert_eq!(
                html.matches(&format!("data-finding-id=\"{id}\"")).count(),
                1,
                "HTML omission/duplication {id}"
            );
            assert!(md.contains(f["location"]["file"].as_str().unwrap()));
            assert!(rows(&sarif["runs"][0]["results"])
                .iter()
                .any(|r| r["ruleId"] == f["rule_id"]
                    && r["locations"][0]["physicalLocation"]["artifactLocation"]["uri"]
                        == f["location"]["file"]
                    && r["locations"][0]["physicalLocation"]["region"]["startLine"]
                        == f["location"]["line"]));
        }
        for line in [2, 3] {
            assert_eq!(
                findings
                    .iter()
                    .filter(|f| f["rule_id"] == "TS_PY_RSA_USAGE"
                        && f["location"]["file"] == "app.py"
                        && f["location"]["line"] == line)
                    .count(),
                1
            );
        }
        let assets = rows(&native["assets"]);
        for kind in [
            "algorithm",
            "certificate",
            "public_key",
            "usage",
            "dependency",
        ] {
            assert!(assets.iter().any(|a| a["kind"] == kind));
        }
        let certs: Vec<_> = assets
            .iter()
            .filter(|a| a["kind"] == "certificate")
            .collect();
        assert_eq!(certs.len(), 1);
        assert_eq!(
            certs[0]["attributes"]["certificate_sha256"],
            "ca58745a1bda90cc3368a37ae4a97d9022c57f8b82da22ca41b4416e08586772"
        );
        assert!(certs[0]["uncertainty"]
            .to_string()
            .contains("trust_and_signature_not_verified"));
        let keys: Vec<_> = assets
            .iter()
            .filter(|a| a["kind"] == "public_key")
            .collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0]["attributes"]["key_bits"], 2048);
        assert!(!assets.iter().any(|a| a["kind"] == "private_key"));
        let observations = rows(&native["observations"]);
        let marker = observations
            .iter()
            .find(|o| o["method"] == "private_key_marker")
            .unwrap();
        assert_eq!(marker["usage"], "unknown");
        assert_eq!(marker["assurance"], "lexical");
        for (provenance, declared, resolved) in [
            ("manifest_declared", json!("^0.10"), Value::Null),
            ("lock_resolved", Value::Null, json!("0.10.75")),
        ] {
            assert!(assets.iter().any(|a| a["kind"] == "dependency"
                && a["attributes"]["version_provenance"] == provenance
                && a["attributes"]["declared_constraint"] == declared
                && a["attributes"]["resolved_version"] == resolved));
        }
        assert_eq!(rows(&native["legacy_associations"]).len(), findings.len());
        for a in rows(&native["legacy_associations"]) {
            assert!(ids.contains(a["legacy_id"].as_str().unwrap()));
            assert!(!rows(&a["observation_ids"]).is_empty());
            for id in rows(&a["observation_ids"]) {
                assert!(observations.iter().any(|o| o["id"] == *id));
            }
        }
        let cdx = read(&out.join("cbom.cdx-1.7.json"));
        assert!(cdx.to_string().contains("export-loss"));
        assert!(!cdx.to_string().contains("certificateState"));
        for c in rows(&cdx["components"]) {
            assert!(assets
                .iter()
                .any(|a| a["id"] == c["bom-ref"] && a["kind"] != "usage"));
        }
        let software = read(&out.join("dependency-sbom.cdx-1.7.json"));
        for c in rows(&software["components"]) {
            let a = assets.iter().find(|a| a["id"] == c["bom-ref"]).unwrap();
            if a["attributes"]["version_provenance"] == "manifest_declared" {
                assert!(c.get("version").is_none());
            } else {
                assert_eq!(c["version"], "0.10.75");
            }
        }
        for entry in fs::read_dir(&out).unwrap() {
            assert!(!fs::read_to_string(entry.unwrap().path())
                .unwrap()
                .contains("SYNTHETIC048_NOT_PRIVATE_KEY_BYTES"));
        }
        retain(&format!("scan-{lang}"), &out);
        scans.push((legacy, native));
    }
    retain("inputs", &input);
    assert_eq!(scans[0].0["findings"], scans[1].0["findings"]);
    assert_eq!(scans[0].1["assets"], scans[1].1["assets"]);
    let before = w.0.join("en/scan.inventory.v1.json");
    let after = w.0.join("ja/scan.inventory.v1.json");
    let out = w.0.join("diff");
    assert!(Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("diff")
        .arg(before)
        .arg(after)
        .args(["--repository-scope", "quality/048", "--out-dir"])
        .arg(&out)
        .status()
        .unwrap()
        .success());
    schema_cases.push(check(
        &out.join("scan.diff.v1.json"),
        "schemas/scan-diff-v1.schema.json",
        "/schema_version",
        json!(2),
    ));
    let comparison = read(&out.join("scan.diff.v1.json"));
    assert!(rows(&comparison["occurrences"])
        .iter()
        .any(|o| o["status"] == "unchanged"));
    assert!(rows(&comparison["occurrences"])
        .iter()
        .any(|o| o["status"] == "unresolved"));
    assert!(rows(&comparison["occurrences"])
        .iter()
        .all(|o| o["status"] == "unchanged" || o["status"] == "unresolved"));
    retain("diff", &out);
    if let Some(out) = std::env::var_os("PQC048_EVIDENCE") {
        fs::write(
            PathBuf::from(out).join("schema-scan.json"),
            serde_json::to_vec_pretty(&schema_cases).unwrap(),
        )
        .unwrap();
    }
}

#[test]
#[cfg(all(
    feature = "probe-tls",
    feature = "probe-ssh",
    feature = "probe-ssh-kex"
))]
fn all_probe_cli_exports_reject_schema_corruption_without_connecting() {
    let w = Workspace::new();
    let mut cases = vec![];
    for (command, tag, schema) in [
        ("tls", "tls", "schemas/probe-tls-v1.schema.json"),
        (
            "ssh",
            "ssh-offers",
            "schemas/probe-ssh-offers-v1.schema.json",
        ),
        ("ssh-kex", "ssh-kex", "schemas/probe-ssh-kex-v1.schema.json"),
    ] {
        for lang in ["en", "ja"] {
            let out = w.0.join(format!("{tag}-{lang}"));
            let mut c = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
            c.args([
                "probe",
                command,
                "127.0.0.1",
                "--port",
                "443",
                "--lang",
                lang,
                "--out-dir",
            ])
            .arg(&out);
            if command == "ssh-kex" {
                c.arg("--host-key-file")
                    .arg(root().join("tests/fixtures/ssh-kex-host.pub"));
            }
            let result = c.output().unwrap();
            assert_eq!(
                result.status.code(),
                Some(3),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            let file = out.join(format!("probe.{tag}.v1.json"));
            cases.push(check(&file, schema, "/schema_version", json!(2)));
            cases.push(check(&file, schema, "/limits/overall_timeout_ns", json!(0)));
            let value = read(&file);
            assert_eq!(value["diagnostics"][0], "address_denied");
            assert_eq!(value["traffic"]["sent_bytes"], 0);
            assert_eq!(value["traffic"]["received_bytes"], 0);
            let mut forged = value;
            forged["outcome"] = json!("completed");
            assert!(
                !validator(schema).is_valid(&forged),
                "failed probe promoted to completion: {tag}"
            );
            retain(&format!("probe-{tag}-{lang}"), &out);
        }
    }
    if let Some(out) = std::env::var_os("PQC048_EVIDENCE") {
        fs::write(
            PathBuf::from(out).join("schema-probe.json"),
            serde_json::to_vec_pretty(&cases).unwrap(),
        )
        .unwrap();
    }
}
