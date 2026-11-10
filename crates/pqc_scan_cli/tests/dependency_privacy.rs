use std::{path::Path, process::Command};
#[test]
fn dependency_locator_credentials_are_omitted_from_all_outputs_and_stderr() {
    let root = std::env::temp_dir().join(format!("pqc027-private-{}", std::process::id()));
    std::fs::create_dir_all(root.join("input")).unwrap();
    std::fs::write(root.join("input/Cargo.toml"),"[dependencies]\nopenssl = { git = \"https://TOKEN_CANARY@credentials.invalid/private\", version = \"1.2\" }\nring = { path = \"../LOCAL_PATH_CANARY\", version = \"0.17\" }\n").unwrap();
    std::fs::write(root.join("input/package.json"),
        r#"{"dependencies":{"openssl":"1.2.3"},"publishConfig":{"registry":"https:\u002f\u002fTOKEN_CANARY@credentials.invalid"}}"#).unwrap();
    let rules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules");
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(root.join("input"))
        .args([
            "--format",
            "all",
            "--inventory-json",
            "--policy-profile",
            "migration-review-v1",
            "--diagnostics",
            "--rules-dir",
        ])
        .arg(rules)
        .arg("--out-dir")
        .arg(root.join("out"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("CANARY"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("CANARY"));
    for entry in std::fs::read_dir(root.join("out")).unwrap() {
        let path = entry.unwrap().path();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(!text.contains("CANARY"), "{}", path.display());
        assert!(!text.contains("credentials.invalid"), "{}", path.display());
    }
    let _ = std::fs::remove_dir_all(root);
}
