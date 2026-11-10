use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeTarget},
    target_policy::{ProbeLimits, TargetRequest},
};
use pqc_scan_probe_ssh::SshProbe;
use serde_json::Value;

#[test]
fn advertised_only_document_has_a_self_contained_offline_schema_and_no_negotiated_claim() {
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schemas/probe-ssh-offers-v1.schema.json"
        ))
        .unwrap(),
    )
    .unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let report = SshProbe::new().failed(
        ProbeTarget::requested(&TargetRequest::new("localhost", 22).unwrap()),
        &ProbeLimits::default(),
        ProbeDiagnostic::DnsFailed,
    );
    let value = serde_json::to_value(report).unwrap();
    assert!(
        validator.is_valid(&value),
        "{:?}",
        validator.iter_errors(&value).collect::<Vec<_>>()
    );
    let mut forged = value.clone();
    forged["details"]["negotiated"] = "curve25519-sha256".into();
    assert!(!validator.is_valid(&forged));
    let mut forged = value.clone();
    forged["details"]["client_profile"]["first_kex_packet_follows"] = true.into();
    assert!(!validator.is_valid(&forged));
    let mut masked = value;
    masked["details"]["server_identification"] = "[masked-sensitive-content]".into();
    masked["redaction_applied"] = true.into();
    assert!(validator.is_valid(&masked));
    for key in [
        "max_dns_answers",
        "max_received_bytes",
        "max_sent_bytes",
        "dns_timeout_ns",
        "tcp_timeout_ns",
        "protocol_timeout_ns",
        "overall_timeout_ns",
    ] {
        let mut invalid = masked.clone();
        invalid["limits"][key] = 0.into();
        assert!(!validator.is_valid(&invalid), "zero limit {key} must fail");
    }
}
