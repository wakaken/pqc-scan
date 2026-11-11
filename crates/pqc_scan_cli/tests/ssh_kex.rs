#![cfg(feature = "probe-ssh-kex")]
use std::{fs, process::Command};
#[test]
fn key_is_required_and_private_or_invalid_key_fails_before_connection() {
    let dir = std::env::temp_dir().join(format!("pqc-kex-cli-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let base = [
        "probe",
        "ssh-kex",
        "127.0.0.1",
        "--port",
        &port,
        "--allow-cidr",
        "127.0.0.1/32",
    ];
    let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .args(base)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    for content in [
        "PRIVATE-CANARY-----BEGIN OPENSSH PRIVATE KEY-----",
        "not a key",
        &"x".repeat(4097),
    ] {
        let pin = dir.join("bad.pub");
        fs::write(&pin, content).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args(base)
            .arg("--host-key-file")
            .arg(pin)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(1));
        assert!(!String::from_utf8_lossy(&result.stderr).contains("PRIVATE-CANARY"));
        assert!(result.stdout.is_empty());
    }
    assert!(matches!(listener.accept(),Err(e) if e.kind()==std::io::ErrorKind::WouldBlock));
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn denied_target_has_no_verified_facts_and_bilingual_schema_valid_outputs() {
    let dir = std::env::temp_dir().join(format!("pqc-kex-denied-{}", std::process::id()));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("schemas/probe-ssh-kex-v1.schema.json")).unwrap(),
    )
    .unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let mut records = vec![];
    for language in ["ja", "en"] {
        let out = dir.join(language);
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args([
                "probe",
                "ssh-kex",
                "127.0.0.1",
                "--port",
                "22",
                "--lang",
                language,
                "--host-key-file",
            ])
            .arg(root.join("tests/fixtures/ssh-kex-host.pub"))
            .arg("--out-dir")
            .arg(&out)
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(3),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let mut record: serde_json::Value =
            serde_json::from_slice(&fs::read(out.join("probe.ssh-kex.v1.json")).unwrap()).unwrap();
        assert!(validator.is_valid(&record));
        assert_eq!(record["diagnostics"][0], "address_denied");
        assert_eq!(record["traffic"]["sent_bytes"], 0);
        assert!(record["details"]["negotiated_kex"].is_null());
        assert_eq!(record["details"]["host_identity_verified"], false);
        assert_eq!(fs::read_dir(out).unwrap().count(), 3);
        record.as_object_mut().unwrap().remove("generated_at");
        records.push(record);
    }
    assert_eq!(records[0], records[1]);
    fs::remove_dir_all(dir).unwrap();
}
