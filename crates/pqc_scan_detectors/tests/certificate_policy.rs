use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::certificate_detector::{parse_certificate_der, CertificateDetector};
use pqc_scan_rules::{Risk, RuleSet, Severity};
use serde_json::Value;
use x509_parser::asn1_rs::{Any, FromDer};

fn corpus() -> Value {
    serde_json::from_str(include_str!("fixtures/certificate-policy/cases.json")).unwrap()
}
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap()
}
fn der(tag: u8, content: &[u8]) -> Vec<u8> {
    let mut bytes = vec![tag];
    if content.len() < 128 {
        bytes.push(content.len() as u8);
    } else {
        let len = u16::try_from(content.len()).unwrap();
        bytes.extend_from_slice(&[0x82, (len >> 8) as u8, len as u8]);
    }
    bytes.extend_from_slice(content);
    bytes
}
fn fields(encoded: &[u8]) -> Vec<Vec<u8>> {
    let (rest, sequence) = Any::from_der(encoded).unwrap();
    assert!(rest.is_empty());
    let mut input = sequence.as_bytes();
    let mut values = Vec::new();
    while !input.is_empty() {
        let (rest, _) = Any::from_der(input).unwrap();
        values.push(input[..input.len() - rest.len()].to_vec());
        input = rest;
    }
    values
}
fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
        .collect()
}
fn time(s: &str) -> Vec<u8> {
    der(if s.len() == 13 { 0x17 } else { 0x18 }, s.as_bytes())
}
fn case_bytes(case: &Value) -> Vec<u8> {
    if case["malformed"].as_bool() == Some(true) {
        return b"not a certificate".to_vec();
    }
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(case["source"].as_str().unwrap());
    let mut outer = fields(&std::fs::read(source).unwrap());
    let mut tbs = fields(&outer[0]);
    // All pinned public fixtures are v3: version, serial, signature, issuer,
    // validity, subject, SPKI, optional extensions.
    tbs[4] = der(
        0x30,
        &[
            time(case["before"].as_str().unwrap()),
            time(case["after"].as_str().unwrap()),
        ]
        .concat(),
    );
    if let Some(oid) = case["signature_oid_hex"].as_str() {
        let mut algorithm = der(6, &unhex(oid));
        match case["signature_parameters"].as_str() {
            Some("null") => algorithm.extend_from_slice(&[5, 0]),
            Some("pss_sha256") => {
                let hash = der(
                    0x30,
                    &[der(6, &unhex("608648016503040201")), vec![5, 0]].concat(),
                );
                let mgf = der(
                    0x30,
                    &[der(6, &unhex("2a864886f70d010108")), hash.clone()].concat(),
                );
                let parameters = der(
                    0x30,
                    &[der(0xa0, &hash), der(0xa1, &mgf), der(0xa2, &der(2, &[32]))].concat(),
                );
                algorithm.extend_from_slice(&parameters);
            }
            _ => {}
        }
        let algorithm = der(0x30, &algorithm);
        tbs[2] = algorithm.clone();
        outer[1] = algorithm;
    }
    if case["public_key_pss"].as_bool() == Some(true) || case["public_key_oid_hex"].is_string() {
        let mut spki = fields(&tbs[6]);
        let oid = case["public_key_oid_hex"]
            .as_str()
            .unwrap_or("2a864886f70d01010a");
        spki[0] = der(0x30, &der(6, &unhex(oid)));
        tbs[6] = der(0x30, &spki.concat());
    }
    outer[0] = der(0x30, &tbs.concat());
    der(0x30, &outer.concat())
}
fn input(case: &Value) -> ScannableFile {
    ScannableFile::from_bytes(
        PathBuf::from(case["path"].as_str().unwrap()),
        case_bytes(case),
    )
}
fn ids(detections: &[pqc_scan_core::Detection]) -> Vec<String> {
    let mut ids: Vec<_> = detections.iter().map(|d| d.rule_id.clone()).collect();
    ids.sort();
    ids
}
fn named(name: &str) -> Value {
    corpus()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == name)
        .unwrap()
        .clone()
}

#[test]
fn certificate_truth_table_matches_exact_parsed_facts_and_safe_container_fallback() {
    let rules = rules();
    let data = corpus();
    let clock = CertificateDetector::at(data["assessment_time_unix"].as_i64().unwrap());
    for case in data["cases"].as_array().unwrap() {
        let result = clock.detect_detailed(&input(case), &rules).unwrap();
        let expected: Vec<String> = case["expected_rules"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert_eq!(ids(result.detections()), expected, "case {}", case["name"]);
        match case["expected_status"].as_str() {
            Some("partial") => assert_eq!(result.status(), DetectorStatus::Partial),
            Some("unsupported") => assert_eq!(result.status(), DetectorStatus::Unsupported),
            _ => {}
        }
    }
}

#[test]
fn ecdsa_and_dsa_have_distinct_oid_predicates_and_quantum_classification() {
    let rules = rules();
    for (name, id, base, final_severity) in [
        (
            "ecdsa",
            "CERT_ECDSA_SIGNATURE",
            Severity::Medium,
            Severity::High,
        ),
        (
            "dsa-sha256",
            "CERT_DSA_SIGNATURE",
            Severity::High,
            Severity::Critical,
        ),
    ] {
        let hits = CertificateDetector
            .detect(&input(&named(name)), &rules)
            .unwrap();
        let hit = hits.iter().find(|h| h.rule_id == id).unwrap();
        let rule = rules.get(id).unwrap();
        assert_eq!(rule.risk, Risk::QuantumVulnerable);
        assert_eq!(rule.severity, base);
        assert_eq!(
            pqc_scan_core::risk::adjusted_severity(rule, hit),
            final_severity
        );
        if name == "ecdsa" {
            assert!(!hits.iter().any(|h| h.rule_id == "CERT_DSA_SIGNATURE"));
        }
    }
}

#[test]
fn rsa_size_requires_subject_modulus_and_pss_scope_requires_signature_oid() {
    let rules = rules();
    for name in ["ecdsa", "spoofed-ec", "spoofed-ed"] {
        let hits = CertificateDetector
            .detect(&input(&named(name)), &rules)
            .unwrap();
        assert!(!hits
            .iter()
            .any(|h| ["CERT_RSA_1024", "CERT_RSA_2048", "CERT_RSA_3072"]
                .contains(&h.rule_id.as_str())));
    }
    let hits = CertificateDetector
        .detect(&input(&named("pss-subject-only")), &rules)
        .unwrap();
    assert!(hits.iter().any(|h| h.rule_id == "CERT_RSA_2048"));
    assert!(!hits.iter().any(|h| h.rule_id == "CERT_OID_RSA_PSS"));
}

#[test]
fn invalid_time_ranges_do_not_receive_assertive_expiry_or_cutoff_findings() {
    let rules = rules();
    let outcome = CertificateDetector
        .detect_detailed(&input(&named("reversed-interval")), &rules)
        .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Partial);
    assert!(outcome
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
    assert!(!outcome
        .detections()
        .iter()
        .any(|h| h.rule_id.contains("EXPIR") || h.rule_id == "CERT_NOT_YET_VALID"));
}

static NEXT_RULES: AtomicU64 = AtomicU64::new(0);
#[test]
fn custom_legacy_same_id_and_unknown_scope_keep_their_composite_text_behavior() {
    let root = std::env::temp_dir().join(format!(
        "pqc021-custom-{}-{}",
        std::process::id(),
        NEXT_RULES.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    let text = r#"
- id: CERT_RSA_1024
  kind: certificate
  category: PKI
  severity: low
  risk: quantum-uncertain
  confidence: 0.3
  migration_hint: Synthetic compatibility check.
  scope: certificate
  pattern: '1024'
- id: CUSTOM_UNKNOWN
  kind: certificate
  category: PKI
  severity: low
  risk: quantum-uncertain
  confidence: 0.3
  migration_hint: Synthetic compatibility check.
  scope: custom_legacy_label
  pattern: '1024'
"#;
    std::fs::write(root.join("rules.yml"), text).unwrap();
    let custom = RuleSet::load_from_dir(&root).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
    let actual = CertificateDetector
        .detect(&input(&named("spoofed-ec")), &custom)
        .unwrap();
    assert_eq!(ids(&actual), ["CERT_RSA_1024", "CUSTOM_UNKNOWN"]);
}

#[test]
fn reused_task020_attribute_parser_is_independent_of_rule_policy_changes() {
    let attributes = parse_certificate_der(&case_bytes(&named("ecdsa"))).unwrap();
    assert_eq!(attributes.public_key.algorithm, "EC");
    assert_eq!(attributes.public_key.key_bits, Some(256));
    assert_eq!(attributes.signature_oid, "1.2.840.10045.4.3.2");
}

#[test]
fn fixed_clock_observes_inclusive_validity_endpoints() {
    let rules = rules();
    let file = input(&named("rsa-2048"));
    for (now, expected) in [
        (1_577_836_799, Some("CERT_NOT_YET_VALID")),
        (1_577_836_800, None),
        (1_794_182_400, None),
        (2_208_988_800, None),
        (2_208_988_801, Some("CERT_EXPIRED")),
    ] {
        let outcome = CertificateDetector::at(now)
            .detect_detailed(&file, &rules)
            .unwrap();
        let temporal: Vec<_> = outcome
            .detections()
            .iter()
            .filter(|h| ["CERT_EXPIRED", "CERT_NOT_YET_VALID"].contains(&h.rule_id.as_str()))
            .map(|h| h.rule_id.as_str())
            .collect();
        assert_eq!(temporal, expected.into_iter().collect::<Vec<_>>(), "{now}");
    }
}

#[test]
fn current_time_metadata_is_explicit_and_does_not_change_static_certificate_facts() {
    let rules = rules();
    let outcome = CertificateDetector::at(1_794_182_400)
        .detect_detailed(&input(&named("expired")), &rules)
        .unwrap();
    let expired = outcome
        .detections()
        .iter()
        .find(|h| h.rule_id == "CERT_EXPIRED")
        .unwrap();
    assert_eq!(
        expired.evidence.metadata["assessment_time_unix"],
        "1794182400"
    );
    assert_eq!(expired.evidence.metadata["validity_status"], "expired");
    let signature = outcome
        .detections()
        .iter()
        .find(|h| h.rule_id == "CERT_RSA_SIGNATURE")
        .unwrap();
    assert!(!signature
        .evidence
        .metadata
        .contains_key("assessment_time_unix"));
    assert_eq!(
        signature.evidence.metadata["signature_oid"],
        "1.2.840.113549.1.1.11"
    );
}

#[test]
fn frozen_detector_registry_reuse_retains_its_explicit_assessment_instant() {
    use pqc_scan_core::{scan_repository_detailed, ScanConfig};
    use std::sync::Arc;
    let root = std::env::temp_dir().join(format!(
        "pqc021-clock-{}-{}",
        std::process::id(),
        NEXT_RULES.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    for name in ["one.der", "two.der"] {
        std::fs::write(root.join(name), case_bytes(&named("rsa-2048"))).unwrap();
    }
    let config = ScanConfig {
        root: root.clone(),
        threads: 2,
        ..ScanConfig::default()
    };
    let detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(CertificateDetector::at(1_420_070_400))];
    let a = scan_repository_detailed(&config, &rules(), &detectors).unwrap();
    let b = scan_repository_detailed(&config, &rules(), &detectors).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert!(a.coverage().is_complete());
    for report in [a.legacy(), b.legacy()] {
        let future: Vec<_> = report
            .findings
            .iter()
            .filter(|f| f.rule_id == "CERT_NOT_YET_VALID")
            .collect();
        assert_eq!(future.len(), 2);
        assert!(future
            .iter()
            .all(|f| f.evidence.metadata["assessment_time_unix"] == "1420070400"));
    }
    assert_eq!(
        a.legacy()
            .findings
            .iter()
            .map(|f| &f.finding_id)
            .collect::<Vec<_>>(),
        b.legacy()
            .findings
            .iter()
            .map(|f| &f.finding_id)
            .collect::<Vec<_>>()
    );
}

#[test]
fn legacy_cutoff_stays_fixed_instead_of_becoming_a_current_expiry_alias() {
    let rules = rules();
    let clock = CertificateDetector::at(1_893_456_000); // 2030-01-01 UTC
    for (name, legacy) in [("legacy-cutoff", true), ("after-legacy-cutoff", false)] {
        let out = clock.detect_detailed(&input(&named(name)), &rules).unwrap();
        assert_eq!(
            out.detections()
                .iter()
                .any(|h| h.rule_id == "CERT_EXPIRY_2026_OR_BEFORE"),
            legacy
        );
        assert!(out.detections().iter().any(|h| h.rule_id == "CERT_EXPIRED"));
    }
}

#[test]
fn opted_in_typed_scopes_never_claim_attributes_from_an_unparsed_filename() {
    let root = std::env::temp_dir().join(format!(
        "pqc021-scopes-{}-{}",
        std::process::id(),
        NEXT_RULES.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    let scopes = [
        "signature_oid",
        "public_key_oid",
        "rsa_modulus_bits",
        "certificate_validity",
        "legacy_not_after_2026",
    ];
    let rows: Vec<_> = scopes.iter().enumerate().map(|(i, scope)| serde_json::json!({
        "id": format!("CUSTOM_{i}"), "kind":"certificate", "category":"PKI", "severity":"low",
        "risk":"quantum-uncertain", "confidence":0.3, "migration_hint":"Synthetic scope check.",
        "scope":scope, "pattern":".*"
    })).collect();
    std::fs::write(root.join("rules.yml"), serde_json::to_vec(&rows).unwrap()).unwrap();
    let custom = RuleSet::load_from_dir(&root).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    let out = CertificateDetector::at(1_794_182_400)
        .detect_detailed(&input(&named("malformed")), &custom)
        .unwrap();
    assert!(out.detections().is_empty());
    assert_eq!(out.status(), DetectorStatus::Partial);
}

#[test]
fn custom_legacy_cutoff_scope_evidence_describes_both_possible_predicate_results() {
    let root = std::env::temp_dir().join(format!(
        "pqc021-evidence-{}-{}",
        std::process::id(),
        NEXT_RULES.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    let row = serde_json::json!([{ "id":"CUSTOM_CUTOFF", "kind":"certificate", "category":"PKI",
        "severity":"info", "risk":"quantum-uncertain", "confidence":0.3,
        "migration_hint":"Synthetic cutoff evidence check.", "scope":"legacy_not_after_2026", "pattern":".*" }]);
    std::fs::write(root.join("rules.yml"), serde_json::to_vec(&row).unwrap()).unwrap();
    let custom = RuleSet::load_from_dir(&root).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    for (name, expected) in [
        ("legacy-cutoff", "notAfter < 2027"),
        ("after-legacy-cutoff", "notAfter >= 2027"),
    ] {
        let outcome = CertificateDetector::at(1_794_182_400)
            .detect_detailed(&input(&named(name)), &custom)
            .unwrap();
        assert_eq!(outcome.detections().len(), 1);
        assert_eq!(outcome.detections()[0].evidence.r#match, expected);
    }
}
