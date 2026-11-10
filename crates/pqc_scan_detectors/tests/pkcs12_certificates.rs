use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::certificate_detector::CertificateDetector;
use pqc_scan_rules::RuleSet;
use std::path::PathBuf;

const CERT: &[u8] = include_bytes!("fixtures/certificates/rsa2048.der");
const DATA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 1];
const ENCRYPTED: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 6];
fn tlv(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    if body.len() < 128 {
        out.push(body.len() as u8);
    } else {
        let n = body.len().to_be_bytes();
        let n = &n[n.iter().position(|b| *b != 0).unwrap()..];
        out.push(0x80 | n.len() as u8);
        out.extend(n);
    }
    out.extend(body);
    out
}
fn seq(parts: &[Vec<u8>]) -> Vec<u8> {
    tlv(0x30, &parts.concat())
}
fn oid(v: &[u8]) -> Vec<u8> {
    tlv(6, v)
}
fn content(kind: &[u8], body: &[u8]) -> Vec<u8> {
    seq(&[oid(kind), tlv(0xa0, body)])
}
fn data(body: &[u8]) -> Vec<u8> {
    content(DATA, &tlv(4, body))
}
fn bag(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut id = vec![0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 12, 10, 1];
    id.push(kind);
    seq(&[oid(&id), tlv(0xa0, body)])
}
fn cert() -> Vec<u8> {
    bag(
        3,
        &seq(&[
            oid(&[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 9, 22, 1]),
            tlv(0xa0, &tlv(4, CERT)),
        ]),
    )
}
fn pfx(contents: &[Vec<u8>]) -> Vec<u8> {
    seq(&[tlv(2, &[3]), data(&seq(contents))])
}
fn inspect(bytes: Vec<u8>) -> pqc_scan_core::DetectorOutcome {
    let rules =
        RuleSet::load_from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .unwrap();
    CertificateDetector::at(1_780_000_000)
        .detect_observed(
            &ScannableFile::from_bytes(PathBuf::from("public.p12"), bytes),
            &rules,
        )
        .unwrap()
}
#[test]
fn extracts_public_certificates_and_retains_one_legacy_container_signal() {
    let out = inspect(pfx(&[data(&seq(&[cert(), cert()]))]));
    assert_eq!(out.status(), DetectorStatus::Complete);
    assert_eq!(
        out.detections()
            .iter()
            .filter(|h| h.rule_id == "CERT_RSA_SIGNATURE")
            .count(),
        2
    );
    assert_eq!(
        out.detections()
            .iter()
            .filter(|h| h.rule_id == "CERT_P12_CONTAINER")
            .count(),
        1
    );
    let certs: Vec<_> = out
        .detections()
        .iter()
        .filter(|h| h.rule_id == "CERT_RSA_SIGNATURE")
        .collect();
    assert_eq!(
        certs[0].evidence.metadata["certificate_sha256"],
        certs[1].evidence.metadata["certificate_sha256"]
    );
    assert_ne!(
        certs[0].evidence.metadata["object_index"],
        certs[1].evidence.metadata["object_index"]
    );
    for hit in certs {
        assert_eq!(hit.location.line, 1);
        assert_eq!(hit.location.column, 1);
        assert_eq!(hit.evidence.metadata["mac_verification"], "not_performed");
        assert_eq!(hit.evidence.metadata["location_kind"], "binary_offset");
    }
}
#[test]
fn mixed_encrypted_and_key_bags_keep_public_certificates_without_payload_leaks() {
    let canary = b"PRIVATE_CANARY_do_not_export";
    let out = inspect(pfx(&[
        data(&seq(&[cert(), bag(1, canary), bag(2, canary)])),
        content(ENCRYPTED, &tlv(0x30, canary)),
    ]));
    assert_eq!(out.status(), DetectorStatus::Partial);
    assert!(out
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
    assert!(!format!("{out:?}").contains("PRIVATE_CANARY"));
    let container = out
        .detections()
        .iter()
        .find(|h| h.rule_id == "CERT_P12_CONTAINER")
        .unwrap();
    assert!(container.evidence.metadata["container_inspection"].contains("unsupported"));
}
#[test]
fn encrypted_only_is_unsupported_and_malformed_input_is_partial() {
    assert_eq!(
        inspect(pfx(&[content(ENCRYPTED, &tlv(0x30, &[]))])).status(),
        DetectorStatus::Unsupported
    );
    assert_eq!(
        inspect(vec![0x30, 0x84, 0xff, 0xff, 0xff, 0xff]).status(),
        DetectorStatus::Partial
    );
    let out = inspect(pfx(&[data(&seq(&[cert(), tlv(0x30, &[6])]))]));
    assert_eq!(out.status(), DetectorStatus::Partial);
    assert!(out
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
}
#[test]
fn bounded_nested_contents_are_supported_but_deep_or_oversized_inputs_are_incomplete() {
    assert_eq!(
        inspect(pfx(&[data(&seq(&[bag(6, &seq(&[cert()]))]))])).status(),
        DetectorStatus::Complete
    );
    let mut nested = cert();
    for _ in 0..12 {
        nested = bag(6, &seq(&[nested]));
    }
    assert_ne!(
        inspect(pfx(&[data(&seq(&[nested]))])).status(),
        DetectorStatus::Complete
    );
    let mut big = pfx(&[data(&seq(&[cert()]))]);
    big.resize(2 * 1024 * 1024 + 1, 0);
    assert_eq!(inspect(big).status(), DetectorStatus::Truncated);
    assert_eq!(
        inspect(pfx(&[data(&seq(&vec![cert(); 257]))])).status(),
        DetectorStatus::Truncated
    );
}
#[test]
fn wrong_inner_tags_cannot_smuggle_public_certificates() {
    // A cert bag requires an explicit X.509 OCTET STRING, not arbitrary nested ASN.1.
    let bad = bag(
        3,
        &seq(&[
            oid(&[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 9, 22, 1]),
            tlv(0xa0, &tlv(0x30, CERT)),
        ]),
    );
    let out = inspect(pfx(&[data(&seq(&[bad]))]));
    assert_eq!(out.status(), DetectorStatus::Partial);
    assert!(!out
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
}
#[test]
fn mac_presence_never_claims_verification_and_bad_mac_retains_certificates() {
    let contents = data(&seq(&[data(&seq(&[cert()]))]));
    let mac = seq(&[
        seq(&[
            seq(&[oid(&[0x60, 0x86, 0x48, 1, 0x65, 3, 4, 2, 1]), tlv(5, &[])]),
            tlv(4, &[7; 32]),
        ]),
        tlv(4, b"public_salt"),
        tlv(2, &[1]),
    ]);
    let out = inspect(seq(&[tlv(2, &[3]), contents.clone(), mac]));
    assert_eq!(out.status(), DetectorStatus::Complete);
    assert!(out
        .detections()
        .iter()
        .all(|h| h.evidence.metadata["mac_verification"] == "not_performed"));
    let out = inspect(seq(&[tlv(2, &[3]), contents, seq(&[tlv(4, b"bad_mac")])]));
    assert_eq!(out.status(), DetectorStatus::Partial);
    assert!(out
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
}
#[test]
fn malformed_attributes_do_not_turn_a_certificate_container_complete() {
    let mut id = vec![0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 12, 10, 1];
    id.push(3);
    let value = seq(&[
        oid(&[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 9, 22, 1]),
        tlv(0xa0, &tlv(4, CERT)),
    ]);
    let bad = seq(&[oid(&id), tlv(0xa0, &value), tlv(0x31, &[0xff])]);
    assert_eq!(
        inspect(pfx(&[data(&seq(&[cert(), bad]))])).status(),
        DetectorStatus::Partial
    );
}
#[test]
fn recovered_certificate_offsets_reference_original_public_der() {
    let bytes = pfx(&[data(&seq(&[bag(1, b"private_canary"), cert(), cert()]))]);
    let out = inspect(bytes.clone());
    for hit in out
        .detections()
        .iter()
        .filter(|h| h.rule_id == "CERT_RSA_SIGNATURE")
    {
        let offset: usize = hit.evidence.metadata["byte_offset"].parse().unwrap();
        assert_eq!(&bytes[offset..offset + CERT.len()], CERT);
    }
}

#[test]
fn reviewed_public_example_is_fully_inspected_within_supported_scope() {
    let bytes = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/pki/certs/legacy.p12"),
    )
    .unwrap();
    let out = inspect(bytes);
    assert_eq!(out.status(), DetectorStatus::Complete);
    assert!(out
        .detections()
        .iter()
        .any(|h| h.rule_id == "CERT_RSA_SIGNATURE"));
}

#[test]
fn truncated_bag_oid_prefix_cannot_panic_or_become_complete() {
    let unknown = seq(&[
        oid(&[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 12, 10, 1]),
        tlv(0xa0, b"opaque"),
    ]);
    assert_eq!(
        inspect(pfx(&[data(&seq(&[unknown]))])).status(),
        DetectorStatus::Unsupported
    );
}

#[test]
fn binary_certificate_observations_do_not_invent_text_positions() {
    use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
    use std::sync::Arc;
    let root = std::env::temp_dir().join(format!("pqc-pkcs12-native-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("public.p12"),
        pfx(&[data(&seq(&[cert(), cert()]))]),
    )
    .unwrap();
    let rules =
        RuleSet::load_from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .unwrap();
    let config = ScanConfig {
        root: root.clone(),
        threads: 1,
        ..ScanConfig::default()
    };
    let detectors: Vec<Arc<dyn Detector>> = vec![Arc::new(CertificateDetector::at(1_780_000_000))];
    let result =
        scan_repository_native(&config, &rules, &detectors, &NativeOptions::default()).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
    let native = serde_json::to_value(result.native().unwrap()).unwrap();
    let observations: Vec<_> = native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["method"] == "certificate_parse")
        .collect();
    assert_eq!(observations.len(), 2);
    for o in observations {
        assert!(o["target"]["line"].is_null());
        assert!(o["target"]["column"].is_null());
        assert!(o["target"]["object_index"].is_number());
    }
}

#[test]
fn inspection_limits_remain_truncated_when_retained_certificate_attributes_are_partial() {
    let mut der = CERT.to_vec();
    let rsa_oid = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 1];
    let i = der
        .windows(rsa_oid.len())
        .position(|v| v == rsa_oid)
        .unwrap();
    der[i + 8] = 99; // Unsupported public-key algorithm, no private material.
    let unknown = bag(
        3,
        &seq(&[
            oid(&[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 9, 22, 1]),
            tlv(0xa0, &tlv(4, &der)),
        ]),
    );
    let mut bags = vec![unknown];
    bags.extend(vec![cert(); 256]);
    assert_eq!(
        inspect(pfx(&[data(&seq(&bags))])).status(),
        DetectorStatus::Truncated
    );
}
