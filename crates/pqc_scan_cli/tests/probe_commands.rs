#[cfg(feature = "probe-tls")]
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
            "pqc-probe-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )))
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(not(feature = "probe-tls"))]
#[test]
fn default_binary_has_no_probe_command() {
    let f = Fixture::new();
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .args(["probe", "tls", "localhost", "--port", "443", "--out-dir"])
        .arg(&f.0)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(!f.0.exists());
    let help = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(!String::from_utf8_lossy(&help.stdout).contains("probe"));
}

#[cfg(feature = "probe-tls")]
fn denied(protocol: &str, filename: &str) {
    let f = Fixture::new();
    for lang in ["en", "ja"] {
        let directory = f.0.join(lang);
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args([
                "probe",
                protocol,
                "127.0.0.1",
                "--port",
                "443",
                "--lang",
                lang,
                "--out-dir",
            ])
            .arg(&directory)
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(3),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 3);
        let report: Value =
            serde_json::from_slice(&fs::read(directory.join(filename)).unwrap()).unwrap();
        assert_eq!(report["outcome"], "failed");
        assert_eq!(report["diagnostics"][0], "address_denied");
        assert!(report["target"]["connected_peer"].is_null());
        assert_eq!(report["traffic"]["sent_bytes"], 0);
    }
    let load = |lang: &str| {
        let mut v: Value =
            serde_json::from_slice(&fs::read(f.0.join(lang).join(filename)).unwrap()).unwrap();
        v.as_object_mut().unwrap().remove("generated_at");
        v
    };
    assert_eq!(load("en"), load("ja"));
}
#[cfg(feature = "probe-tls")]
#[test]
fn tls_denial_writes_failed_observation_without_network_and_keeps_language_parity() {
    denied("tls", "probe.tls.v1.json");
}
#[cfg(feature = "probe-tls")]
#[test]
fn tls_ca_private_block_is_rejected_without_echoing_contents() {
    let f = Fixture::new();
    fs::create_dir_all(&f.0).unwrap();
    let ca = f.0.join("ca.pem");
    fs::write(
        &ca,
        "-----BEGIN PRIVATE KEY-----\nSYNTHETIC_PRIVATE_CANARY_032\n-----END PRIVATE KEY-----\n",
    )
    .unwrap();
    let out = f.0.join("output");
    let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .args(["probe", "tls", "localhost", "--port", "443", "--ca-file"])
        .arg(&ca)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    assert!(!out.exists());
    assert!(!String::from_utf8_lossy(&result.stderr).contains("SYNTHETIC_PRIVATE_CANARY_032"));
}
