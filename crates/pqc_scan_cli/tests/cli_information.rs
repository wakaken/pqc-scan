use std::process::Command;

#[test]
fn informational_flags_succeed_on_stdout_without_error_output() {
    for args in [
        vec!["--help"],
        vec!["--version"],
        vec!["scan", "--help"],
        vec!["rules", "--help"],
        vec!["diff", "--help"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args(&args)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!output.stdout.is_empty(), "{args:?}");
        assert!(output.stderr.is_empty(), "{args:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        if args == ["--version"] {
            assert_eq!(
                text.trim(),
                format!("pqc-scan {}", env!("CARGO_PKG_VERSION"))
            );
        } else {
            assert!(text.contains("Usage:"), "{args:?}: {text}");
        }
    }
}

#[test]
fn invalid_arguments_keep_the_existing_failure_exit_contract() {
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("--not-a-valid-option")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}
