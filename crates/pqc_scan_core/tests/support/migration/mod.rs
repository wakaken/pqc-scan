use pqc_scan_core::{
    identity::native_id,
    migration::{InputFormat, SavedEvidence},
};
use serde_json::{json, Value};

pub fn inventory(identifier: &str) -> Value {
    let mut v: Value = serde_json::from_str(include_str!(
        "../../../../pqc_scan_cli/tests/fixtures/native-v1.golden.json"
    ))
    .unwrap();
    v["run"]["id"] = native_id("scan", &["migration-fixture"]).into();
    v["run"]["generated_at"] = "2026-11-17T00:00:00Z".into();
    let asset = native_id("asset", &[identifier]);
    v["assets"] = json!([{"id":asset,"kind":"algorithm","identity_basis":"normalized_identifier_and_reported_parameters","attributes":{"identifier":identifier,"namespace":"named","purpose":"reference","parameters":{}},"relations":[],"uncertainty":[]}]);
    let mut observation = v["observations"][0].clone();
    observation["id"] = native_id("observation", &["fixture-1"]).into();
    observation["asset_ids"] = json!([asset]);
    observation["identity_confidence"] = "weak_structural_anchor".into();
    observation["method"] = "ast_pattern".into();
    observation["assurance"] = "syntactic".into();
    v["observations"] = json!([observation]);
    for key in [
        "legacy_associations",
        "unassociated_observation_ids",
        "legacy_loss_records",
    ] {
        v[key] = json!([]);
    }
    v
}

pub fn envelope(protocol: &str, details: Value) -> Value {
    json!({"schema_version":1,"scanner_version":"0.3.0","generated_at":"2026-11-17T00:00:00Z","protocol":protocol,"attempt_id":1,
        "target":{"host":"example.test","port":443,"resolved_addresses":["192.0.2.1"],"pinned_address":"192.0.2.1:443","connected_peer":"192.0.2.1:443"},
        "limits":{"max_dns_answers":8,"dns_timeout_ns":5000000000u64,"tcp_timeout_ns":3000000000u64,"protocol_timeout_ns":5000000000u64,"overall_timeout_ns":15000000000u64,"max_received_bytes":524288,"max_sent_bytes":65536},
        "traffic":{"received_bytes":1200,"sent_bytes":300},"outcome":"completed","diagnostics":[],"redaction_applied":false,"details":details})
}
pub fn tls() -> Value {
    let version = json!({"id":772,"name":"TLSv1_3"});
    let suite = json!({"id":4865,"name":"TLS13_AES_128_GCM_SHA256"});
    let group = json!({"id":4588,"name":"X25519MLKEM768"});
    envelope(
        "tls",
        json!({
        "client_profile":{"name":"default_secure_fresh","provider":"aws_lc_rs","provider_version":"1.18.0","rustls_version":"0.23.46","protocol_versions":[version],"cipher_suites":[suite],"key_exchange_groups":[group]},
        "verification":{"verification_name":"example.test","sent_sni":"example.test","trust_source":"bundled_mozilla","root_set_sha256":"a".repeat(64),"root_set_digest_basis":"sorted_length_prefixed_trust_anchor_der_fields_sha256_v1","certificate_chain_and_name":"verified","revocation":"not_checked","trust_bundle_version":"1.0.11"},
        "handshake_complete":true,"handshake_kind":"full","negotiated":{"version":version,"cipher_suite":suite,"key_exchange_group":group,"group_classification":"hybrid"},
        "peer_certificates":[{"certificate_der_sha256":"b".repeat(64),"spki_sha256":"c".repeat(64),"signature_oid":"1.2.840.113549.1.1.11","public_key_oid":"1.2.840.113549.1.1.1","key_bits":2048,"named_curve_oid":null,"not_before_unix":1700000000,"not_after_unix":1900000000,"attribute_status":"complete"}],
        "certificate_collection":{"status":"complete","presented_count":1,"presented_bytes":900,"collected_count":1}}),
    )
}
pub fn failed_tls() -> Value {
    let mut v = tls();
    v["outcome"] = "failed".into();
    v["diagnostics"] = json!(["certificate_validation_failed"]);
    v["details"]["handshake_complete"] = false.into();
    v["details"]["handshake_kind"] = Value::Null;
    v["details"]["negotiated"] = Value::Null;
    v["details"]["verification"]["certificate_chain_and_name"] = "failed".into();
    v
}
pub fn ssh_lists() -> Value {
    json!({"kex_algorithms":["mlkem768x25519-sha256","curve25519-sha256","ext-info-s"],"server_host_key_algorithms":["ssh-ed25519"],
        "encryption_algorithms_client_to_server":["aes256-ctr"],"encryption_algorithms_server_to_client":["aes256-ctr"],
        "mac_algorithms_client_to_server":["hmac-sha2-256"],"mac_algorithms_server_to_client":["hmac-sha2-256"],
        "compression_algorithms_client_to_server":["none"],"compression_algorithms_server_to_client":["none"],"languages_client_to_server":[],"languages_server_to_client":[]})
}
pub fn offers() -> Value {
    envelope(
        "ssh_offers",
        json!({"client_profile":{"identification":"SSH-2.0-pqc_scan_0.3.0","purpose":"diagnostic_proposal_only","offers":ssh_lists(),"first_kex_packet_follows":false,"reserved":0},
        "assurance":"unauthenticated_advertisement","server_identification":"SSH-2.0-fixture","offers":ssh_lists(),"first_kex_packet_follows":false,
        "kex_candidates":[],"host_key_candidates":[],"extension_markers":["ext-info-s"],"pre_identification_lines":0,"early_messages":0}),
    )
}
pub fn kex() -> Value {
    envelope(
        "ssh_kex",
        json!({"russh_version":"0.65.1","provider_version":"1.18.0","offered_kex":"mlkem768x25519-sha256","offered_host_key":"ssh-ed25519","trust_source":"explicit_endpoint_public_key",
        "host_identity_verified":true,"handshake_complete":true,"negotiated_kex":"mlkem768x25519-sha256","host_signature":"ssh-ed25519","host_authentication_pqc":false,"user_authentication_attempted":false}),
    )
}
pub fn ingest(v: &Value, format: InputFormat) -> pqc_scan_core::migration::Result<SavedEvidence> {
    SavedEvidence::from_bytes(&serde_json::to_vec(v).unwrap(), format)
}
