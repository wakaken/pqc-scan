use pqc_scan_core::probe::{
    ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport, ProbeTarget,
};
use pqc_scan_core::target_policy::{ProbeLimits, TargetRequest};

#[test]
fn positive_submillisecond_limits_remain_exact_valid_report_values() {
    let limits = ProbeLimits {
        dns_timeout: std::time::Duration::from_nanos(1),
        ..Default::default()
    };
    assert!(pqc_scan_core::target_policy::ProbeBudget::new(
        limits.clone(),
        pqc_scan_core::target_policy::Cancellation::new()
    )
    .is_ok());
    let report = ProbeReport::new(
        ProbeProtocol::Tls,
        ProbeTarget::requested(&TargetRequest::new("localhost", 443).unwrap()),
        &limits,
        (),
    );
    assert!(report.validate().is_ok());
    assert_eq!(
        serde_json::to_value(report).unwrap()["limits"]["dns_timeout_ns"],
        1
    );
}

#[test]
fn report_preserves_unknown_addresses_and_checks_completion_and_traffic() {
    let request = TargetRequest::new("localhost", 443).unwrap();
    let mut report = ProbeReport::new(
        ProbeProtocol::Tls,
        ProbeTarget::requested(&request),
        &ProbeLimits::default(),
        (),
    );
    assert_eq!(report.outcome, ProbeOutcome::Failed);
    assert!(report.validate().is_ok());
    report.set_outcome(ProbeOutcome::Completed, vec![]);
    assert!(
        report.validate().is_err(),
        "completed inspection needs a bound connected peer"
    );
    report.set_outcome(ProbeOutcome::Failed, vec![ProbeDiagnostic::AddressDenied]);
    let cap = report.limits.max_dns_answers;
    report.limits.max_dns_answers = cap + 1;
    assert!(
        report.validate().is_err(),
        "report cannot widen reviewed hard caps"
    );
    report.limits.max_dns_answers = cap;
    let value = serde_json::to_value(&report).unwrap();
    assert!(value["target"]["resolved_addresses"].is_null());
    assert_eq!(value["schema_version"], 1);
    report.set_outcome(
        ProbeOutcome::Completed,
        vec![ProbeDiagnostic::GroupUnavailable],
    );
    assert!(report.validate().is_err());
    report.set_outcome(
        ProbeOutcome::Partial,
        vec![ProbeDiagnostic::GroupUnavailable],
    );
    assert!(report.validate().is_ok());
    report.traffic.received_bytes = report.limits.max_received_bytes + 1;
    assert!(report.validate().is_err());
}
