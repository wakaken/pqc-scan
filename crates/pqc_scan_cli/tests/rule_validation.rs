use std::{fs, process::Command};
#[test]
fn invalid_rule_metadata_is_an_operational_error_before_any_report_is_written() {
    let root = std::env::temp_dir().join(format!("pqc028-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("input/app.js"), "const value='CONTROL';").unwrap();
    for (id, confidence) in [("''", "0.5"), ("VALID", ".nan"), ("VALID", "1.00000001")] {
        fs::write(root.join("rules/invalid.yml"),format!("id: {id}\nkind: regex\ncategory: custom\nseverity: low\nrisk: quantum-uncertain\nconfidence: {confidence}\nmigration_hint: Review\npattern: CONTROL\nscope: code\n")).unwrap();
        for command in [
            vec!["rules", "list"],
            vec!["scan", root.join("input").to_str().unwrap()],
        ] {
            let scanning = command[0] == "scan";
            let mut process = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
            process
                .args(command)
                .arg("--rules-dir")
                .arg(root.join("rules"));
            if scanning {
                process.arg("--out-dir").arg(root.join("out"));
            }
            let output = process.output().unwrap();
            assert_eq!(output.status.code(), Some(1));
            assert!(String::from_utf8_lossy(&output.stderr).contains("failed to load rules"));
            assert!(!root.join("out").exists());
        }
    }
    let _ = fs::remove_dir_all(root);
}
