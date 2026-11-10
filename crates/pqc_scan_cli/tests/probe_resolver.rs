#![cfg(feature = "probe-transport")]
#[path = "../src/probe_resolver.rs"]
mod probe_resolver;
use pqc_scan_core::target_policy::{
    Cancellation, ProbeBudget, ProbeHost, ProbeLimits, TargetPolicy, TargetRequest,
};

#[test]
fn self_executable_resolves_once_and_authorizes_all_local_answers() {
    let mut resolver =
        probe_resolver::ProcessResolver::for_executable(env!("CARGO_BIN_EXE_pqc-scan").into());
    let policy = TargetPolicy::new(&["127.0.0.0/8".into(), "::1/128".into()]).unwrap();
    let budget = ProbeBudget::new(ProbeLimits::default(), Cancellation::new()).unwrap();
    let target = policy
        .resolve(
            TargetRequest::new("localhost", 443).unwrap(),
            &mut resolver,
            &budget,
        )
        .unwrap();
    assert!(matches!(target.host(), ProbeHost::Dns(name) if name == "localhost"));
    assert!(!target.answers().is_empty());
    assert!(target.answers().iter().all(|ip| ip.is_loopback()));
    assert_eq!(target.socket_addr().port(), 443);
    assert!(policy
        .resolve(
            TargetRequest::new("localhost", 443).unwrap(),
            &mut resolver,
            &budget
        )
        .is_err());
}

#[test]
fn helper_is_absent_from_help_and_rejects_non_hostname_input() {
    let exe = env!("CARGO_BIN_EXE_pqc-scan");
    let help = std::process::Command::new(exe)
        .arg("--help")
        .output()
        .unwrap();
    assert!(!String::from_utf8_lossy(&help.stdout).contains("__resolve-host"));
    let invalid = std::process::Command::new(exe)
        .args(["__resolve-host", "https://localhost/"])
        .output()
        .unwrap();
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
}
