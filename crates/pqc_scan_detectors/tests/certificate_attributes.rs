use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use pqc_scan_core::{
    scan_repository_detailed, Detection, Detector, DetectorStatus, DiagnosticCode, ScanConfig,
    ScannableFile,
};
use pqc_scan_detectors::certificate_detector::CertificateDetector;
use pqc_scan_rules::RuleSet;

const RSA1024: &[u8] = include_bytes!("fixtures/certificates/rsa1024.der");
const RSA2048: &[u8] = include_bytes!("fixtures/certificates/rsa2048.der");
const RSA3072: &[u8] = include_bytes!("fixtures/certificates/rsa3072.der");
const RSA_SECOND: &[u8] = include_bytes!("fixtures/certificates/rsa2048-second.der");
const P256: &[u8] = include_bytes!("fixtures/certificates/p256.der");
const P384: &[u8] = include_bytes!("fixtures/certificates/p384.der");
const P521: &[u8] = include_bytes!("fixtures/certificates/p521.der");
const ED25519: &[u8] = include_bytes!("fixtures/certificates/ed25519.der");
const MIXED: &[u8] = include_bytes!("fixtures/certificates/p256-rsa-signed.der");

fn rules() -> RuleSet {
    RuleSet::load_from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap()
}

fn file(name: &str, bytes: &[u8]) -> ScannableFile {
    ScannableFile::from_bytes(PathBuf::from(name), bytes.to_vec())
}

fn pem(der: &[u8]) -> String {
    pem::encode(&pem::Pem::new("CERTIFICATE", der))
}

fn metadata<'a>(hit: &'a Detection, name: &str) -> &'a str {
    hit.evidence
        .metadata
        .get(name)
        .map(String::as_str)
        .unwrap_or("")
}

#[test]
fn rsa_sizes_are_modulus_bits_and_public_fingerprints_match_openssl() {
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/certificates/manifest.json")).unwrap();
    for (name, der, bits) in [
        ("rsa1024.der", RSA1024, "1024"),
        ("rsa2048.der", RSA2048, "2048"),
        ("rsa3072.der", RSA3072, "3072"),
    ] {
        let hits = CertificateDetector
            .detect(&file(name, der), &rules())
            .unwrap();
        let hit = hits
            .iter()
            .find(|h| h.rule_id == "CERT_RSA_SIGNATURE")
            .unwrap();
        assert_eq!(metadata(hit, "key_bits"), bits, "{name}");
        assert_eq!(metadata(hit, "key_size_kind"), "rsa_modulus");
        assert_eq!(metadata(hit, "public_key_algorithm"), "RSA");
        assert_eq!(
            metadata(hit, "certificate_sha256"),
            expected[name]["certificate_sha256"].as_str().unwrap()
        );
        assert_eq!(
            metadata(hit, "spki_sha256"),
            expected[name]["spki_sha256"].as_str().unwrap()
        );
        assert_eq!(
            metadata(hit, "certificate_der_bytes"),
            der.len().to_string()
        );
        assert_eq!(
            metadata(hit, "spki_der_bytes"),
            expected[name]["spki_der_bytes"]
                .as_u64()
                .unwrap()
                .to_string()
        );
        assert!(hits.iter().any(|h| h.rule_id == format!("CERT_RSA_{bits}")));
    }
}

#[test]
fn format_selection_uses_bytes_within_all_supported_certificate_suffixes() {
    for name in [
        "server.der",
        "server.crt",
        "server.cer",
        "server.pem",
        "server.CRT",
    ] {
        for bytes in [RSA2048.to_vec(), pem(RSA2048).into_bytes()] {
            let result = CertificateDetector
                .detect_detailed(&file(name, &bytes), &rules())
                .unwrap();
            assert_eq!(result.status(), DetectorStatus::Complete, "{name}");
            assert!(result
                .detections()
                .iter()
                .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
        }
    }
}

#[test]
fn ec_sizes_come_from_named_curve_including_p521_not_point_encoding() {
    for (der, bits, oid, name) in [
        (P256, "256", "1.2.840.10045.3.1.7", "P-256"),
        (P384, "384", "1.3.132.0.34", "P-384"),
        (P521, "521", "1.3.132.0.35", "P-521"),
    ] {
        let result = CertificateDetector
            .detect_detailed(&file("ec.der", der), &rules())
            .unwrap();
        assert_eq!(result.status(), DetectorStatus::Complete);
        let hit = result
            .detections()
            .iter()
            .find(|h| h.rule_id == "CERT_ECDSA_SIGNATURE")
            .unwrap();
        assert_eq!(metadata(hit, "key_bits"), bits);
        assert_eq!(metadata(hit, "key_size_kind"), "ec_field");
        assert_eq!(metadata(hit, "named_curve_oid"), oid);
        assert_eq!(metadata(hit, "named_curve"), name);
    }
}

#[test]
fn rsa_signature_does_not_turn_ec_subject_into_rsa_key() {
    let result = CertificateDetector
        .detect_detailed(&file("mixed.der", MIXED), &rules())
        .unwrap();
    let hit = result
        .detections()
        .iter()
        .find(|h| h.rule_id == "CERT_RSA_SIGNATURE")
        .unwrap();
    assert_eq!(metadata(hit, "signature_oid"), "1.2.840.113549.1.1.11");
    assert_eq!(metadata(hit, "public_key_oid"), "1.2.840.10045.2.1");
    assert_eq!(metadata(hit, "public_key_algorithm"), "EC");
    assert_eq!(metadata(hit, "key_size_kind"), "ec_field");
    assert_eq!(metadata(hit, "key_bits"), "256");
}

#[test]
fn ed25519_retains_encoded_size_without_inventing_rsa_or_security_bits() {
    // No built-in Ed25519 certificate inventory rule exists. Use an explicit
    // test-only OID rule; parser completeness must not depend on a finding.
    let ed_rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/certificates"),
    )
    .unwrap();
    let result = CertificateDetector
        .detect_detailed(&file("ed.der", ED25519), &ed_rules)
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Complete);
    let hit = &result.detections()[0];
    assert_eq!(metadata(hit, "public_key_algorithm"), "Ed25519");
    assert_eq!(metadata(hit, "public_key_encoded_bytes"), "32");
    assert_eq!(metadata(hit, "key_bits"), "");
    assert_eq!(metadata(hit, "parameters_kind"), "absent");
}

#[test]
fn pqc_and_unknown_spki_oids_are_not_measured_as_rsa() {
    let rsa_oid = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 1];
    for replacement in [
        [0x60, 0x86, 0x48, 1, 0x65, 3, 4, 3, 17],
        [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 99],
    ] {
        let mut der = RSA2048.to_vec();
        let offset = der.windows(9).position(|s| s == rsa_oid).unwrap();
        der[offset..offset + 9].copy_from_slice(&replacement);
        let result = CertificateDetector
            .detect_detailed(&file("unknown.der", &der), &rules())
            .unwrap();
        assert_eq!(result.status(), DetectorStatus::Partial);
        let hit = result
            .detections()
            .iter()
            .find(|h| h.rule_id == "CERT_RSA_SIGNATURE")
            .unwrap();
        assert_eq!(metadata(hit, "key_bits"), "");
        assert_eq!(metadata(hit, "key_size_kind"), "unknown");
        assert_eq!(metadata(hit, "spki_sha256").len(), 64);
        assert!(!result
            .detections()
            .iter()
            .any(|h| h.rule_id == "CERT_RSA_2048"));
    }
}

#[test]
fn pem_bundle_retains_real_positions_and_distinct_certificates_sharing_spki() {
    let first = pem(RSA2048);
    let second_line = first.lines().count() + 3;
    let text = format!("preamble\r\n{first}\r\n{}", pem(RSA_SECOND));
    let result = CertificateDetector
        .detect_detailed(&file("bundle.pem", text.as_bytes()), &rules())
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Complete);
    let hits: Vec<_> = result
        .detections()
        .iter()
        .filter(|h| h.rule_id == "CERT_RSA_SIGNATURE")
        .collect();
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].location.line, 2);
    assert_eq!(hits[1].location.line, second_line);
    assert_eq!(metadata(hits[0], "object_index"), "1");
    assert_eq!(metadata(hits[1], "object_index"), "2");
    assert_eq!(metadata(hits[0], "byte_offset"), "10");
    assert_ne!(
        metadata(hits[0], "certificate_sha256"),
        metadata(hits[1], "certificate_sha256")
    );
    assert_eq!(
        metadata(hits[0], "spki_sha256"),
        metadata(hits[1], "spki_sha256")
    );
}

#[test]
fn damaged_or_unsupported_pem_block_preserves_neighboring_valid_certificates() {
    for middle in [
        "-----BEGIN CERTIFICATE-----\ninvalid base64!\n-----END CERTIFICATE-----\n",
        "-----BEGIN CERTIFICATE-----\nmissing end marker\n",
        "-----BEGIN PRIVATE KEY-----\nSYNTHETIC_PRIVATE_CANARY_020\n-----END PRIVATE KEY-----\n",
        "-----BEGIN FUTURE OBJECT-----\nopaque\n-----END FUTURE OBJECT-----\n",
    ] {
        let text = pem(RSA2048) + middle + &pem(RSA_SECOND);
        let result = CertificateDetector
            .detect_detailed(&file("bundle.pem", text.as_bytes()), &rules())
            .unwrap();
        assert_eq!(
            result.status(),
            if middle.starts_with("-----BEGIN PRIVATE KEY-----") {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Partial
            }
        );
        let hits: Vec<_> = result
            .detections()
            .iter()
            .filter(|h| h.rule_id == "CERT_RSA_SIGNATURE")
            .collect();
        assert_eq!(hits.len(), 2);
        assert_eq!(metadata(hits[1], "object_index"), "3");
        let evidence: Vec<_> = result.detections().iter().map(|h| &h.evidence).collect();
        assert!(!serde_json::to_string(&evidence)
            .unwrap()
            .contains("SYNTHETIC_PRIVATE_CANARY_020"));
    }
}

#[test]
fn malformed_and_unsupported_containers_keep_signals_but_do_not_complete() {
    for (name, bytes, expected, id) in [
        (
            "bad.pem",
            b"not a certificate".as_slice(),
            DetectorStatus::Partial,
            "CERT_PEM_CONTAINER",
        ),
        (
            "bundle.p12",
            b"unsupported".as_slice(),
            DetectorStatus::Partial,
            "CERT_P12_CONTAINER",
        ),
        (
            "bundle.pfx",
            b"unsupported".as_slice(),
            DetectorStatus::Partial,
            "CERT_P12_CONTAINER",
        ),
    ] {
        let result = CertificateDetector
            .detect_detailed(&file(name, bytes), &rules())
            .unwrap();
        assert_eq!(result.status(), expected);
        assert!(result.detections().iter().any(|h| h.rule_id == id));
        assert_eq!(
            CertificateDetector
                .detect(&file(name, bytes), &rules())
                .unwrap()
                .len(),
            result.detections().len()
        );
    }
}

#[test]
fn der_trailing_bytes_keep_parsed_facts_but_make_inspection_partial() {
    let mut bytes = RSA2048.to_vec();
    bytes.extend_from_slice(b"trailing data");
    let result = CertificateDetector
        .detect_detailed(&file("trailing.der", &bytes), &rules())
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Partial);
    assert!(result
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
}

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

#[test]
fn private_pem_payload_is_not_attached_to_certificate_container_findings() {
    for label in [
        "EC PRIVATE KEY",
        "DSA PRIVATE KEY",
        "ENCRYPTED PRIVATE KEY",
        "PRIVATE KEY",
        "OPENSSH PRIVATE KEY",
    ] {
        let root = std::env::temp_dir().join(format!(
            "pqc-task020-private-{}-{}",
            std::process::id(),
            NEXT_ROOT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        let text =
            format!("-----BEGIN {label}-----\nCANARY_020_SECRET_PAYLOAD\n-----END {label}-----\n");
        std::fs::write(root.join("container.pem"), text).unwrap();
        let config = ScanConfig {
            root: root.clone(),
            threads: 1,
            ..ScanConfig::default()
        };
        let detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(CertificateDetector)];
        let result = scan_repository_detailed(&config, &rules(), &detectors).unwrap();
        std::fs::remove_dir_all(root).unwrap();
        assert!(
            result.legacy().findings.is_empty(),
            "a known private object is not a certificate"
        );
        assert!(
            !serde_json::to_string(result.legacy())
                .unwrap()
                .contains("CANARY_020_SECRET_PAYLOAD"),
            "{label}"
        );
    }
}

#[test]
fn distinct_bundle_objects_survive_real_pipeline_normalization() {
    let root = std::env::temp_dir().join(format!(
        "pqc-task020-{}-{}",
        std::process::id(),
        NEXT_ROOT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("bundle.pem"), pem(RSA2048) + &pem(RSA_SECOND)).unwrap();
    let config = ScanConfig {
        root: root.clone(),
        threads: 1,
        ..ScanConfig::default()
    };
    let detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(CertificateDetector)];
    let result = scan_repository_detailed(&config, &rules(), &detectors).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    let hits: Vec<_> = result
        .legacy()
        .findings
        .iter()
        .filter(|h| h.rule_id == "CERT_RSA_SIGNATURE")
        .collect();
    assert_eq!(hits.len(), 2);
    assert_ne!(hits[0].finding_id, hits[1].finding_id);
    assert!(result.coverage().is_complete());
    assert!(!result
        .diagnostics()
        .iter()
        .any(|d| d.code() == DiagnosticCode::DetectorNotReported));
}

#[test]
fn noncandidate_and_no_certificate_rules_are_explicitly_inapplicable() {
    for (input, rules) in [
        (file("app.rs", b"fn main() {}"), rules()),
        (file("server.der", RSA2048), RuleSet::default()),
    ] {
        let result = CertificateDetector.detect_detailed(&input, &rules).unwrap();
        assert_eq!(result.status(), DetectorStatus::NotApplicable);
        assert!(result.detections().is_empty());
    }
}

fn der(tag: u8, contents: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    if contents.len() < 128 {
        out.push(contents.len() as u8);
    } else {
        let length = u16::try_from(contents.len()).unwrap();
        out.extend_from_slice(&[0x82, (length >> 8) as u8, length as u8]);
    }
    out.extend_from_slice(contents);
    out
}

fn spki(algorithm: &[u8], parameters: &[u8], payload: &[u8], unused_bits: u8) -> Vec<u8> {
    let mut alg = der(6, algorithm);
    alg.extend_from_slice(parameters);
    let mut content = der(0x30, &alg);
    let mut bit_string = vec![unused_bits];
    bit_string.extend_from_slice(payload);
    content.extend_from_slice(&der(3, &bit_string));
    der(0x30, &content)
}

const RSA_OID: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 1];

fn rsa_spki(modulus: &[u8], exponent: &[u8], extra_integer: bool) -> Vec<u8> {
    let mut values = der(2, modulus);
    values.extend_from_slice(&der(2, exponent));
    if extra_integer {
        values.extend_from_slice(&der(2, &[1]));
    }
    spki(RSA_OID, &[5, 0], &der(0x30, &values), 0)
}

#[test]
fn public_der_helpers_require_exact_objects_and_return_safe_errors() {
    use pqc_scan_detectors::certificate_detector::{parse_certificate_der, parse_public_key_der};
    let cert = parse_certificate_der(RSA2048).unwrap();
    assert_eq!(cert.public_key.key_bits, Some(2048));
    assert!(cert.not_before_unix < cert.not_after_unix);
    let (_, parsed) = x509_parser::parse_x509_certificate(RSA2048).unwrap();
    let key = parse_public_key_der(parsed.public_key().raw).unwrap();
    assert_eq!(key.spki_sha256, cert.public_key.spki_sha256);
    assert_eq!(key.status, DetectorStatus::Complete);
    for bad in [
        b"CANARY_020_BAD_DER".to_vec(),
        [RSA2048, b"trailing"].concat(),
    ] {
        let error = parse_certificate_der(&bad).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Public DER object could not be parsed completely."
        );
    }
    assert!(parse_public_key_der(RSA2048).is_err());
    let mut trailing = parsed.public_key().raw.to_vec();
    trailing.push(0);
    assert!(parse_public_key_der(&trailing).is_err());
}

#[test]
fn rsa_modulus_bits_handle_sign_padding_and_non_byte_aligned_integers() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    for (modulus, want) in [(&[0x7f][..], 7), (&[1, 1][..], 9), (&[0, 0x80][..], 8)] {
        let key = parse_public_key_der(&rsa_spki(modulus, &[3], false)).unwrap();
        assert_eq!(key.status, DetectorStatus::Complete);
        assert_eq!(key.key_bits, Some(want));
    }
    // PSS public-key restrictions are distinct from the certificate signature OID.
    let mut pss = RSA_OID.to_vec();
    pss[8] = 10;
    let payload = der(0x30, &[2, 2, 1, 1, 2, 1, 3]);
    let key = parse_public_key_der(&spki(&pss, &[], &payload, 0)).unwrap();
    assert_eq!(key.algorithm, "RSA-PSS");
    assert_eq!(key.key_bits, Some(9));
}

#[test]
fn malformed_rsa_integers_or_extra_fields_do_not_become_measured_keys() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    for input in [
        rsa_spki(&[], &[3], false),
        rsa_spki(&[0], &[3], false),
        rsa_spki(&[0x80], &[3], false),
        rsa_spki(&[0, 1], &[3], false),
        rsa_spki(&[1], &[0], false),
        rsa_spki(&[1], &[3], true),
    ] {
        // Structurally invalid inner RSA can either be rejected by the decoder or
        // retained as an explicitly incomplete attribute set, never Complete.
        if let Ok(key) = parse_public_key_der(&input) {
            assert_ne!(key.status, DetectorStatus::Complete);
            assert_eq!(key.key_bits, None);
        }
    }
}

#[test]
fn unknown_ec_parameters_and_invalid_points_have_no_invented_curve_size() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    let ec_oid = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 2, 1];
    let p256_oid = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 3, 1, 7];
    let unknown_oid = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 3, 1, 8];
    for (params, point) in [
        (Vec::new(), vec![4; 65]),
        (der(6, unknown_oid), vec![4; 65]),
        (der(6, p256_oid), vec![4, 0]),
        (der(6, p256_oid), vec![0; 65]),
    ] {
        let key = parse_public_key_der(&spki(ec_oid, &params, &point, 0)).unwrap();
        assert_ne!(key.status, DetectorStatus::Complete);
        assert_eq!(key.key_bits, None);
    }
}

#[test]
fn ed_and_x_keys_check_oid_specific_lengths_parameters_and_bit_alignment() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    for (suffix, name, length) in [
        (110, "X25519", 32),
        (111, "X448", 56),
        (112, "Ed25519", 32),
        (113, "Ed448", 57),
    ] {
        let oid = [0x2b, 0x65, suffix];
        let key = parse_public_key_der(&spki(&oid, &[], &vec![1; length], 0)).unwrap();
        assert_eq!(key.algorithm, name);
        assert_eq!(key.status, DetectorStatus::Complete);
        assert_eq!(key.key_bits, None);
        for bad in [
            spki(&oid, &[5, 0], &vec![1; length], 0),
            spki(&oid, &[], &vec![1; length - 1], 0),
            spki(&oid, &[], &vec![0; length], 1),
        ] {
            let key = parse_public_key_der(&bad).unwrap();
            assert_eq!(key.status, DetectorStatus::Partial);
            assert_eq!(key.key_bits, None);
        }
    }
}

fn sequence_fields(bytes: &[u8]) -> Vec<Vec<u8>> {
    use x509_parser::asn1_rs::{Any, FromDer};
    let (rest, sequence) = Any::from_der(bytes).unwrap();
    assert!(rest.is_empty());
    let mut fields = Vec::new();
    let mut input = sequence.as_bytes();
    while !input.is_empty() {
        let (rest, _) = Any::from_der(input).unwrap();
        fields.push(input[..input.len() - rest.len()].to_vec());
        input = rest;
    }
    fields
}

fn with_extra_sequence_field(bytes: &[u8]) -> Vec<u8> {
    let mut fields = sequence_fields(bytes);
    fields.push(vec![2, 1, 1]);
    der(0x30, &fields.concat())
}

#[test]
fn nested_asn1_remainders_are_rejected_in_helpers_and_not_complete_in_detector() {
    use pqc_scan_detectors::certificate_detector::{parse_certificate_der, parse_public_key_der};
    let good_key = spki(&[0x2b, 0x65, 112], &[], &[1; 32], 0);
    assert!(parse_public_key_der(&with_extra_sequence_field(&good_key)).is_err());
    let mut key_fields = sequence_fields(&good_key);
    // AlgorithmIdentifier may have at most OID + one parameter.
    let mut alg_fields = sequence_fields(&key_fields[0]);
    alg_fields.extend([vec![5, 0], vec![2, 1, 1]]);
    key_fields[0] = der(0x30, &alg_fields.concat());
    assert!(parse_public_key_der(&der(0x30, &key_fields.concat())).is_err());
    let mut altered = vec![with_extra_sequence_field(RSA2048)];
    let cert_fields = sequence_fields(RSA2048);
    // The generated v3 fixture has version, serial, signature, issuer, validity,
    // subject, SPKI. Inject extra inner elements while keeping outer lengths valid.
    for index in [2, 6] {
        let mut outer = cert_fields.clone();
        let mut tbs = sequence_fields(&outer[0]);
        tbs[index] = with_extra_sequence_field(&tbs[index]);
        outer[0] = der(0x30, &tbs.concat());
        altered.push(der(0x30, &outer.concat()));
    }
    let mut outer = cert_fields;
    outer[1] = with_extra_sequence_field(&outer[1]);
    altered.push(der(0x30, &outer.concat()));
    for bytes in altered {
        assert!(parse_certificate_der(&bytes).is_err());
        assert_eq!(
            CertificateDetector
                .detect_detailed(&file("bad.der", &bytes), &rules())
                .unwrap()
                .status(),
            DetectorStatus::Partial
        );
    }
}

#[test]
fn a_pem_marker_inside_der_name_does_not_change_format_selection() {
    let mut bytes = RSA2048.to_vec();
    let text = b"pqc-scan-public-test";
    let positions: Vec<_> = bytes
        .windows(text.len())
        .enumerate()
        .filter_map(|(i, v)| (v == text).then_some(i))
        .collect();
    assert!(!positions.is_empty());
    for position in positions {
        bytes[position..position + 11].copy_from_slice(b"-----BEGIN ");
    }
    let result = CertificateDetector
        .detect_detailed(&file("embedded.crt", &bytes), &rules())
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Complete);
    assert!(result
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
}

#[test]
fn legacy_dsa_oid_requires_the_same_well_formed_public_parameters() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    let params = der(0x30, &[2, 1, 23, 2, 1, 11, 2, 1, 2]);
    for oid in [
        &[0x2b, 0x0e, 3, 2, 12][..],
        &[0x2a, 0x86, 0x48, 0xce, 0x38, 4, 1][..],
    ] {
        let key = parse_public_key_der(&spki(oid, &params, &[2, 1, 8], 0)).unwrap();
        assert_eq!(key.algorithm, "DSA");
        assert_eq!(key.status, DetectorStatus::Complete);
        for (p, y) in [
            (&params[..], &[2, 1, 0][..]),
            (&params[..], &[2, 1, 0x80][..]),
            (&[0x30, 3, 2, 1, 23][..], &[2, 1, 8][..]),
        ] {
            assert_eq!(
                parse_public_key_der(&spki(oid, p, y, 0)).unwrap().status,
                DetectorStatus::Partial
            );
        }
    }
}
