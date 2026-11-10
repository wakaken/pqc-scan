use pqc_scan_core::assessment::{assess_algorithm, Purpose, QuantumExposure};
use pqc_scan_core::policy::{assess_primitive_policy, candidates, AssessmentContext, Decision};
use std::collections::BTreeMap;
fn params(key: &str, value: &str) -> BTreeMap<String, String> {
    BTreeMap::from([(key.into(), value.into())])
}
#[test]
fn quantum_and_classical_axes_coexist_and_unknown_facts_are_not_guessed() {
    let a = assess_algorithm("RSA", &params("key_bits", "1024"));
    assert_eq!(a.quantum, QuantumExposure::Vulnerable);
    assert!(a
        .classical_concerns
        .contains(&"public_key_below_local_minimum".into()));
    let unknown = assess_algorithm("SECRET_UNKNOWN_CANARY", &BTreeMap::new());
    assert_eq!(unknown.quantum, QuantumExposure::Unknown);
    assert!(!serde_json::to_string(&unknown).unwrap().contains("CANARY"));
    let family = assess_algorithm("ML-KEM", &BTreeMap::new());
    assert_eq!(family.quantum, QuantumExposure::Unknown);
    assert!(!family.parameters_known);
    let exact = assess_algorithm("ML-KEM-768", &BTreeMap::new());
    assert_eq!(exact.quantum, QuantumExposure::DesignedResistant);
    assert_eq!(exact.purpose, Purpose::KeyEstablishment);
    let draft = assess_algorithm("Kyber768", &BTreeMap::new());
    assert_eq!(draft.standardization, "draft");
    assert_eq!(draft.quantum, QuantumExposure::Unknown);
}
#[test]
fn hybrid_never_inherits_resistance_and_symmetric_hash_models_are_parameter_specific() {
    let a = assess_algorithm("X25519Kyber768Draft00", &BTreeMap::new());
    assert_eq!(a.construction, "hybrid");
    assert_eq!(a.components.len(), 2);
    assert_eq!(a.quantum, QuantumExposure::Unknown);
    let aes = assess_algorithm("AES-256", &BTreeMap::new());
    assert_eq!(aes.quantum, QuantumExposure::GenericSearch);
    assert_eq!(aes.attack_models[0].exponent_numerator, 256);
    assert_eq!(aes.attack_models[0].exponent_denominator, 2);
    let hash = assess_algorithm("SHA-256", &BTreeMap::new());
    assert_eq!(hash.attack_models.len(), 2);
    assert!(hash
        .attack_models
        .iter()
        .any(|m| m.attack == "generic_collision" && m.exponent_denominator == 3));
    let old = assess_algorithm("SHA-1", &BTreeMap::new());
    assert_eq!(old.quantum, QuantumExposure::GenericSearch);
    assert!(!old.classical_concerns.is_empty());
}
#[test]
fn policy_tables_require_applicability_purpose_and_declared_support() {
    let rsa = assess_algorithm("RSA", &BTreeMap::new());
    for use_case in ["hashing", "encryption"] {
        let context =
            AssessmentContext::from_json(&format!(r#"{{"version":1,"use_case":"{use_case}"}}"#))
                .unwrap();
        assert!(candidates(&rsa, &context).is_empty());
    }

    let default = AssessmentContext::default();
    assert_eq!(
        assess_primitive_policy(&rsa, &default).decision,
        Decision::Fail
    );
    for (app, expected) in [
        ("unknown", Decision::Indeterminate),
        ("not_applicable", Decision::NotApplicable),
    ] {
        let ctx =
            AssessmentContext::from_json(&format!(r#"{{"version":1,"applicability":"{app}"}}"#))
                .unwrap();
        assert_eq!(assess_primitive_policy(&rsa, &ctx).decision, expected);
    }
    let pq = assess_algorithm("ML-DSA-65", &BTreeMap::new());
    assert_eq!(
        assess_primitive_policy(&pq, &default).decision,
        Decision::Indeterminate
    );
    let ready=AssessmentContext::from_json(r#"{"version":1,"use_case":"jwt","support":{"runtime":true,"provider":true,"peer":true,"protocol":true,"parameters":true,"jose":true,"verifiers":true}}"#).unwrap();
    assert_eq!(
        assess_primitive_policy(&pq, &ready).decision,
        Decision::Pass
    );
    let kem = assess_algorithm("ML-KEM-768", &BTreeMap::new());
    assert_eq!(
        assess_primitive_policy(&kem, &ready).decision,
        Decision::Indeterminate
    );
    let c = candidates(&assess_algorithm("RS256", &BTreeMap::new()), &ready);
    assert!(c.iter().all(|c| c.purpose == Purpose::Signature));
    assert!(c[0]
        .prerequisites
        .iter()
        .any(|p| p.name == "jose_identifier_and_encoding"));
    let pki = AssessmentContext::from_json(r#"{"version":1,"use_case":"pki"}"#).unwrap();
    assert!(candidates(&rsa, &pki).iter().any(|c| c
        .prerequisites
        .iter()
        .any(|p| p.name == "issuer_and_trust_store_support")));
}
#[test]
fn context_is_strict_bounded_and_does_not_echo_sensitive_values() {
    for input in [
        r#"{"version":2}"#,
        r#"{"version":1,"SECRET_CANARY":"bad"}"#,
        r#"{"version":1,"use_case":"SECRET_CANARY"}"#,
        r#"{"version":1,"version":1}"#,
    ] {
        let error = AssessmentContext::from_json(input)
            .err()
            .unwrap()
            .to_string();
        assert!(!error.contains("CANARY"));
    }
    assert!(AssessmentContext::from_json(&" ".repeat(16385)).is_err());
}

#[test]
fn aggregate_truth_table_never_treats_incomplete_empty_or_mixed_unknown_as_clean() {
    use pqc_scan_core::policy::summarize;
    let c = AssessmentContext::default();
    for (decisions, inspection, native, expected) in [
        (vec![], true, true, Decision::Indeterminate),
        (vec![Decision::Pass], false, true, Decision::Indeterminate),
        (vec![Decision::Pass], true, false, Decision::Indeterminate),
        (
            vec![Decision::Pass, Decision::Indeterminate],
            true,
            true,
            Decision::Indeterminate,
        ),
        (
            vec![Decision::Fail, Decision::Indeterminate],
            false,
            false,
            Decision::Fail,
        ),
        (vec![Decision::Pass], true, true, Decision::Pass),
        (
            vec![Decision::NotApplicable],
            true,
            true,
            Decision::Indeterminate,
        ),
    ] {
        assert_eq!(
            summarize(&decisions, inspection, native, &c).decision,
            expected
        );
    }
    let denied=AssessmentContext::from_json(r#"{"version":1,"use_case":"signature","support":{"runtime":true,"provider":true,"peer":true,"protocol":true,"parameters":false}}"#).unwrap();
    assert_eq!(
        assess_primitive_policy(&assess_algorithm("ML-DSA-65", &BTreeMap::new()), &denied).decision,
        Decision::Indeterminate
    );
}

#[test]
fn formal_pqc_oids_preserve_parameter_set_and_purpose() {
    for (oid, purpose) in [
        ("2.16.840.1.101.3.4.4.2", Purpose::KeyEstablishment),
        ("2.16.840.1.101.3.4.3.18", Purpose::Signature),
        ("2.16.840.1.101.3.4.3.20", Purpose::Signature),
    ] {
        let a = assess_algorithm(oid, &BTreeMap::new());
        assert_eq!(a.purpose, purpose);
        assert_eq!(a.quantum, QuantumExposure::DesignedResistant);
    }
    let unknown = assess_algorithm("UNKNOWN_CANARY", &BTreeMap::new());
    let ctx = AssessmentContext::from_json(r#"{"version":1,"use_case":"pki"}"#).unwrap();
    assert!(candidates(&unknown, &ctx).is_empty());
}
fn inventory_with(assets: serde_json::Value) -> pqc_scan_core::NativeScan {
    let mut v: serde_json::Value = serde_json::from_str(include_str!(
        "../../pqc_scan_cli/tests/fixtures/native-v1.golden.json"
    ))
    .unwrap();
    v["run"]["id"] = serde_json::json!(pqc_scan_core::identity::native_id("scan", &["test"]));
    v["run"]["generated_at"] = serde_json::json!("2036-01-01T00:00:00Z");
    v["assets"] = assets;
    for key in [
        "observations",
        "legacy_associations",
        "unassociated_observation_ids",
        "legacy_loss_records",
    ] {
        v[key] = serde_json::json!([]);
    }
    serde_json::from_value(v).unwrap()
}
fn asset_id(tag: &str) -> String {
    pqc_scan_core::identity::native_id("asset", &[tag])
}
fn asset(
    tag: &str,
    kind: &str,
    attributes: serde_json::Value,
    relations: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({"id":asset_id(tag),"kind":kind,"identity_basis":"test_only","attributes":attributes,"relations":relations,"uncertainty":[]})
}
#[test]
fn inventory_policy_retains_certificate_components_and_never_transfers_purpose_or_unknown_assurance(
) {
    use serde_json::json;
    let cert = inventory_with(json!([
        asset(
            "rsa",
            "algorithm",
            json!({"namespace":"oid","identifier":"1.2.840.113549.1.1.1","parameters":{}}),
            json!([])
        ),
        asset(
            "key",
            "public_key",
            json!({"key_bits":1024}),
            json!([{"relationship":"algorithm","target":asset_id("rsa")}])
        ),
        asset(
            "cert",
            "certificate",
            json!({"not_before_unix":1,"not_after_unix":10}),
            json!([{"relationship":"signature_algorithm","target":asset_id("rsa")},{"relationship":"subject_key","target":asset_id("key")}])
        )
    ]));
    let mut malformed = serde_json::to_value(&cert).unwrap();
    malformed["assets"][0]["attributes"] =
        json!({"namespace":"oid","identifier":"2.16.840.1.101.3.4.4.2","parameters":{}});
    let malformed: pqc_scan_core::NativeScan = serde_json::from_value(malformed).unwrap();
    let invalid = serde_json::to_value(
        pqc_scan_core::policy::evaluate(
            &malformed,
            &AssessmentContext::default(),
            "migration-review-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        invalid["assessments"][2]["algorithm"]["components"][0]["assessment"]["purpose"],
        "key_establishment"
    );
    assert!(invalid["assessments"][2]["algorithm"]["classical_concerns"]
        .as_array()
        .unwrap()
        .contains(&json!("certificate_signature_purpose_mismatch")));
    assert_eq!(invalid["assessments"][2]["algorithm"]["quantum"], "unknown");
    let context = AssessmentContext::default();
    let report = pqc_scan_core::policy::evaluate(&cert, &context, "migration-review-v1").unwrap();
    let j = serde_json::to_value(report).unwrap();
    let c = &j["assessments"][2];
    assert_eq!(c["algorithm"]["quantum"], "vulnerable");
    assert_eq!(c["algorithm"]["components"].as_array().unwrap().len(), 2);
    assert!(c["algorithm"]["classical_concerns"]
        .as_array()
        .unwrap()
        .contains(&json!("certificate_expired")));
    assert_eq!(j["evaluation"]["decision"], "fail");
    assert_eq!(
        j,
        serde_json::to_value(
            pqc_scan_core::policy::evaluate(&cert, &context, "migration-review-v1").unwrap()
        )
        .unwrap()
    );
    let pki=AssessmentContext::from_json(r#"{"version":1,"use_case":"pki","support":{"runtime":true,"provider":true,"peer":true,"protocol":true,"parameters":true,"pki":true}}"#).unwrap();
    let pq = inventory_with(json!([
        asset(
            "kem",
            "algorithm",
            json!({"namespace":"oid","identifier":"2.16.840.1.101.3.4.4.2","parameters":{}}),
            json!([])
        ),
        asset(
            "dsa",
            "algorithm",
            json!({"namespace":"named","identifier":"ML-DSA-65","parameters":{}}),
            json!([])
        ),
        asset(
            "wrong-ns",
            "algorithm",
            json!({"namespace":"draft","identifier":"ML-DSA-65","parameters":{}}),
            json!([])
        )
    ]));
    let r = serde_json::to_value(
        pqc_scan_core::policy::evaluate(&pq, &pki, "migration-review-v1").unwrap(),
    )
    .unwrap();
    assert_eq!(r["asset_policies"][0]["candidates"], json!([]));
    assert_eq!(r["asset_policies"][1]["decision"], "indeterminate");
    assert_eq!(r["assessments"][2]["algorithm"]["quantum"], "unknown");
    assert_ne!(r["evaluation"]["decision"], "pass");
    let empty = inventory_with(json!([]));
    assert_eq!(
        pqc_scan_core::policy::evaluate(&empty, &context, "migration-review-v1")
            .unwrap()
            .decision(),
        Decision::Indeterminate
    );
}

#[test]
fn opaque_or_unexpected_pqc_parameters_never_become_known_encodings() {
    let opaque = assess_algorithm(
        "2.16.840.1.101.3.4.3.18",
        &params("parameter_interpretation", "not_available"),
    );
    assert!(!opaque.parameters_known);
    let invalid = assess_algorithm("2.16.840.1.101.3.4.4.2", &params("parameters_kind", "null"));
    assert!(!invalid.parameters_known);
}

#[test]
fn rsa_pss_is_signature_only_even_when_operator_declares_establishment() {
    let ctx =
        AssessmentContext::from_json(r#"{"version":1,"use_case":"key_establishment"}"#).unwrap();
    for identifier in ["RSA-PSS", "1.2.840.113549.1.1.10"] {
        let a = assess_algorithm(identifier, &BTreeMap::new());
        assert_eq!(a.purpose, Purpose::Signature);
        assert!(candidates(&a, &ctx).is_empty());
    }
}
