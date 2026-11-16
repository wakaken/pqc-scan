//! Regression cases derived from task056's five source-backed content findings.
use pqc_scan_core::{
    recommendation::{annotate_findings, RuntimeProfile},
    Finding, RecommendedAction,
};
use serde_json::json;

fn actions(category: &str, file: &str, java: Option<&str>) -> Vec<RecommendedAction> {
    let mut findings: Vec<Finding> = serde_json::from_value(json!([{
        "finding_id":"synthetic056", "rule_id":"SYNTHETIC056", "category":category,
        "risk":"quantum-vulnerable", "severity":"medium", "confidence":0.8,
        "description":"Synthetic reference", "migration_hint":"Review use",
        "location":{"file":file,"line":1,"column":1},
        "evidence":{"type":"regex_match","match":"reference","snippet_preview":"reference",
                    "metadata":{"scope":"config"}}
    }]))
    .unwrap();
    let mut profile = RuntimeProfile::default();
    profile.java_version = java.map(str::to_string);
    annotate_findings(&mut findings, &profile);
    findings.remove(0).recommended_actions
}
fn action<'a>(actions: &'a [RecommendedAction], id: &str) -> &'a RecommendedAction {
    actions.iter().find(|a| a.action_id == id).unwrap()
}

#[test]
fn unknown_java_is_not_reported_as_a_detected_old_version() {
    for version in [None, Some("unresolved")] {
        let a = actions("CryptoAPI", "Example.java", version);
        let a = action(&a, "java.runtime-upgrade-for-pqc");
        assert!(a.rationale.to_lowercase().contains("unknown"));
        assert!(!a.rationale.contains("Detected Java version appears older"));
    }
    let old = actions("CryptoAPI", "Example.java", Some("23"));
    assert!(action(&old, "java.runtime-upgrade-for-pqc")
        .rationale
        .contains("older"));
    let current = actions("CryptoAPI", "Example.java", Some("24"));
    assert!(!current
        .iter()
        .any(|a| a.action_id == "java.runtime-upgrade-for-pqc"));
    assert!(current
        .iter()
        .any(|a| a.action_id == "java.jca-ml-dsa-ml-kem"));
}

#[test]
fn bc_example_names_supported_provider_and_new_signature_key_material() {
    let a = actions("CryptoAPI", "Example.java", Some("21"));
    let a = action(&a, "java.bouncycastle-pqc");
    let after = &a.code_examples[0].after;
    assert!(after.contains("new BouncyCastleProvider()"));
    assert!(after.contains("getInstance(\"ML-DSA-65\", \"BC\")"));
    assert!(!after.contains("BCPQC"));
    assert!(after.contains("initSign(mlDsaPrivateKey)"));
    let steps = a.steps.join(" ");
    assert!(steps.contains("1.85") && steps.contains("Verify"));
}

#[test]
fn jca_signature_migration_requires_new_keys_and_verifier_provisioning() {
    let a = actions("CryptoAPI", "Example.java", Some("24"));
    let a = action(&a, "java.jca-ml-dsa-ml-kem");
    assert!(a.code_examples[0]
        .after
        .contains("initSign(mlDsaPrivateKey)"));
    assert!(!a.code_examples[0].after.contains("initSign(privateKey)"));
    let steps = a.steps.join(" ");
    assert!(steps.contains("new ML-DSA key pair"));
    assert!(steps.contains("parameter set") && steps.contains("verifiers"));
}

#[test]
fn certificate_pilot_requires_a_supported_construction_and_encoding() {
    let a = actions("PKI", "certificate.pem", None);
    let a = action(&a, "pki.hybrid-certificate-roadmap");
    let text = format!("{} {}", a.rationale, a.steps.join(" "));
    assert!(text.contains("construction") && text.contains("encoding"));
    assert!(text.contains("FIPS 204/205") && text.contains("not"));
    assert!(text.contains("CA") && text.contains("trust") && text.contains("provider"));
}

#[test]
fn envoy_advice_separates_static_rsa_exchange_from_rsa_authentication() {
    let a = actions("TLS", "envoy.yaml", None);
    for id in ["tls.envoy-hardening", "envoy.pqc-signature-readiness-note"] {
        let a = action(&a, id);
        let text = a.steps.join(" ");
        assert!(text.contains("static RSA") && text.contains("ECDHE_RSA"));
        assert!(text.contains("authentication"));
        assert!(text.contains("cipher_suites") && text.contains("TLS 1.0-1.2"));
        assert!(text.contains("not TLS 1.3"));
    }
}
