use anyhow::Result;
use x509_parser::parse_x509_certificate;

use pqc_scan_core::classifier;
use pqc_scan_core::model::mask_preview;
use pqc_scan_core::{Detection, Detector, Evidence, Location, ScannableFile};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug, Default)]
pub struct CertificateDetector;

#[derive(Debug)]
struct CertInfo {
    signature_algorithm: String,
    signature_oid: String,
    public_key_oid: String,
    key_bits: usize,
    expires_at: String,
}

impl Detector for CertificateDetector {
    fn name(&self) -> &'static str {
        "certificate_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        if !classifier::is_certificate_file(&file.path) {
            return Ok(Vec::new());
        }

        let mut infos = Vec::new();
        let ext = file.ext().unwrap_or_default().to_ascii_lowercase();

        if ext == "pem" || ext == "crt" || ext == "cer" {
            if let Some(text) = file.text.as_ref() {
                if let Ok(blocks) = pem::parse_many(text) {
                    for block in blocks {
                        if !block.tag().contains("CERTIFICATE") {
                            continue;
                        }
                        if let Ok((_, cert)) = parse_x509_certificate(block.contents()) {
                            infos.push(CertInfo {
                                signature_algorithm: signature_name(
                                    cert.signature_algorithm.algorithm.to_id_string().as_str(),
                                ),
                                signature_oid: cert.signature_algorithm.algorithm.to_id_string(),
                                public_key_oid: cert
                                    .public_key()
                                    .algorithm
                                    .algorithm
                                    .to_id_string(),
                                key_bits: cert.public_key().subject_public_key.data.len() * 8,
                                expires_at: format!("{:?}", cert.validity().not_after),
                            });
                        }
                    }
                }
            }
        } else if ext == "der" {
            if let Ok((_, cert)) = parse_x509_certificate(&file.bytes) {
                infos.push(CertInfo {
                    signature_algorithm: signature_name(
                        cert.signature_algorithm.algorithm.to_id_string().as_str(),
                    ),
                    signature_oid: cert.signature_algorithm.algorithm.to_id_string(),
                    public_key_oid: cert.public_key().algorithm.algorithm.to_id_string(),
                    key_bits: cert.public_key().subject_public_key.data.len() * 8,
                    expires_at: format!("{:?}", cert.validity().not_after),
                });
            }
        }

        let mut out = Vec::new();
        let rule_iter = rules.by_kind(RuleKind::Certificate);
        let file_path = file.path.to_string_lossy().into_owned();

        if infos.is_empty() {
            for rule in rule_iter {
                if matches!(rule.scope.as_str(), "signature_oid" | "public_key_oid") {
                    continue;
                }
                let regex = match rule.compiled_pattern() {
                    Some(v) => v,
                    None => continue,
                };
                if regex.is_match(&file_path) {
                    out.push(Detection {
                        rule_id: rule.id.clone(),
                        location: Location {
                            file: file_path.clone(),
                            line: 1,
                            column: 1,
                        },
                        evidence: Evidence {
                            r#type: "certificate".to_string(),
                            r#match: mask_preview(file.file_name().unwrap_or("certificate")),
                            snippet_preview: "certificate container detected".to_string(),
                            metadata: std::collections::BTreeMap::from([
                                ("detector".to_string(), self.name().to_string()),
                                ("extension".to_string(), ext.clone()),
                            ]),
                        },
                    });
                }
            }
            return Ok(out);
        }

        for info in infos {
            // Preserve legacy rule inputs: new PQC display names contain "DSA",
            // which broad historical patterns would mistake for classical DSA.
            let legacy_signature = if info.signature_algorithm.starts_with("ML-DSA")
                || info.signature_algorithm.starts_with("SLH-DSA")
            {
                &info.signature_oid
            } else {
                &info.signature_algorithm
            };
            let target = format!(
                "{} {} {} {} {}",
                file_path, legacy_signature, info.signature_oid, info.key_bits, info.expires_at
            );

            for rule in rules.by_kind(RuleKind::Certificate) {
                let regex = match rule.compiled_pattern() {
                    Some(v) => v,
                    None => continue,
                };
                let scoped_target = match rule.scope.as_str() {
                    "signature_oid" => &info.signature_oid,
                    "public_key_oid" => &info.public_key_oid,
                    _ => &target,
                };
                if !regex.is_match(scoped_target) {
                    continue;
                }

                out.push(Detection {
                    rule_id: rule.id.clone(),
                    location: Location {
                        file: file_path.clone(),
                        line: 1,
                        column: 1,
                    },
                    evidence: Evidence {
                        r#type: "certificate".to_string(),
                        r#match: if rule.scope == "public_key_oid" {
                            signature_name(&info.public_key_oid)
                        } else if rule.scope == "signature_oid" {
                            info.signature_algorithm.clone()
                        } else {
                            mask_preview(&format!(
                                "{} {}-bit",
                                info.signature_algorithm, info.key_bits
                            ))
                        },
                        snippet_preview: format!(
                            "sig={}, key_bits={}, expires={}",
                            info.signature_algorithm, info.key_bits, info.expires_at
                        ),
                        metadata: std::collections::BTreeMap::from([
                            ("detector".to_string(), self.name().to_string()),
                            ("signature_oid".to_string(), info.signature_oid.clone()),
                            ("public_key_oid".to_string(), info.public_key_oid.clone()),
                            ("expires_at".to_string(), info.expires_at.clone()),
                        ]),
                    },
                });
            }
        }

        Ok(out)
    }
}

fn signature_name(oid: &str) -> String {
    match oid {
        "1.2.840.113549.1.1.5" => "sha1WithRSAEncryption".to_string(),
        "1.2.840.113549.1.1.11" => "sha256WithRSAEncryption".to_string(),
        "1.2.840.113549.1.1.12" => "sha384WithRSAEncryption".to_string(),
        "1.2.840.113549.1.1.13" => "sha512WithRSAEncryption".to_string(),
        "1.2.840.10045.4.3.2" => "ecdsa-with-SHA256".to_string(),
        "1.2.840.10045.4.3.3" => "ecdsa-with-SHA384".to_string(),
        "1.2.840.10045.4.3.4" => "ecdsa-with-SHA512".to_string(),
        "2.16.840.1.101.3.4.3.17" => "ML-DSA-44".to_string(),
        "2.16.840.1.101.3.4.3.18" => "ML-DSA-65".to_string(),
        "2.16.840.1.101.3.4.3.19" => "ML-DSA-87".to_string(),
        value
            if value
                .strip_prefix("2.16.840.1.101.3.4.3.")
                .and_then(|v| v.parse::<u8>().ok())
                .is_some_and(|v| (20..=31).contains(&v)) =>
        {
            format!("SLH-DSA ({value})")
        }
        _ => oid.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const VALID_RSA_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----\n\
MIIDETCCAfmgAwIBAgIUY/Qll3SQZ80iLq9sOCMCJDDxxHowDQYJKoZIhvcNAQEL\n\
BQAwGDEWMBQGA1UEAwwNcHFjLXNjYW4tdGVzdDAeFw0yNjAzMDkwNjMyMzJaFw0y\n\
NzAzMDkwNjMyMzJaMBgxFjAUBgNVBAMMDXBxYy1zY2FuLXRlc3QwggEiMA0GCSqG\n\
SIb3DQEBAQUAA4IBDwAwggEKAoIBAQC3aW8C4+2yqCv/5qf+8Uw44lABc3OAFx+C\n\
Bo2yd1ONUQK89n+vEYu4nH2CiphB9evkUejXFP6zQr191Tn0KLeB4ugSuJBForPs\n\
UvPiiyF27iyUAKOrcsTuac05xmRtVcGrAKV7YauVFMWGePaCkh+C56dH+vcHI+/A\n\
VrulRDBGEduQh/3tLzwsFmLYxKoHHRbns48A0PBN1Ugk8T7rRroSarcyUwzQwRu7\n\
ZzzVOICE8KGDqWrzGygQ1YT1M0ED9Sy3bkGBCsFaKcaQq0zp23MT/QVUxfMAoPJ8\n\
3RPZe+0SDasCF1SUzL6a+77anApzaSkAqDFqqWdxqUssaOHsQRO/AgMBAAGjUzBR\n\
MB0GA1UdDgQWBBSU8+L0gcCDCDXScvjncW9QKb7FHDAfBgNVHSMEGDAWgBSU8+L0\n\
gcCDCDXScvjncW9QKb7FHDAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUA\n\
A4IBAQBuKXCbD4RdYAircgtA/N8RdUkmW/xZKmEesWF3dDsH3EOCY6sgql4DWGQ2\n\
gQRnz/IJIoe5htjwxvRxxsiB3COr9eip4tCHSJ7bJlNomjYTwX9TcN/U/8VaxPj/\n\
aA5ABtG3Rh1/v2w5lSKSQFhCpTze11NmMdvoJssrJswtvUUL+JwpbwIkcj8q3bw3\n\
V4rbUZ6U4htW/EQrafwTopsnT6D9EltghIBrT5gJ7oE2FHQAxnzbgWMTwf+2iaZr\n\
6PYiUKam9H4m8e0sHBdPLJOOEu2GlG3pf05NpOUwIKbyedic5GnI140MSh8nl2fW\n\
YaIFrdcmPh3KlSLTCZod7SfAVUCY\n\
-----END CERTIFICATE-----\n";

    fn rules_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../rules/default")
            .canonicalize()
            .expect("rules dir")
    }

    #[test]
    fn recognizes_pqc_oids_without_confusing_signature_and_public_key() {
        let rules = RuleSet::load_from_dir(&rules_dir()).unwrap();
        let original = pem::parse(VALID_RSA_CERT_PEM).unwrap();
        let rsa_signature = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 11];
        let rsa_key = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 1];
        for last in 17..=31 {
            let (signature_id, key_id) = if last <= 19 {
                ("CERT_ML_DSA_SIGNATURE", "CERT_ML_DSA_PUBLIC_KEY")
            } else {
                ("CERT_SLH_DSA_SIGNATURE", "CERT_SLH_DSA_PUBLIC_KEY")
            };
            for (replace, expected, unexpected) in [
                (rsa_signature, signature_id, key_id),
                (rsa_key, key_id, signature_id),
            ] {
                // Parser-only synthetic certificate: signature bytes are not verified.
                let mut der = original.contents().to_vec();
                let offsets: Vec<_> = der
                    .windows(9)
                    .enumerate()
                    .filter_map(|(i, bytes)| (bytes == replace).then_some(i))
                    .collect();
                assert!(!offsets.is_empty());
                for i in offsets {
                    der[i..i + 9].copy_from_slice(&[0x60, 0x86, 0x48, 1, 0x65, 3, 4, 3, last]);
                }
                let file = ScannableFile::from_bytes(PathBuf::from("synthetic.der"), der);
                let hits = CertificateDetector.detect(&file, &rules).unwrap();
                assert!(hits.iter().any(|h| h.rule_id == expected), "{expected}");
                assert!(!hits.iter().any(|h| h.rule_id == unexpected));
                if replace == rsa_signature {
                    assert!(
                        hits.iter().all(|h| h.rule_id != "CERT_DSA_SIGNATURE"),
                        "PQC must not be mistaken for classical DSA"
                    );
                }
            }
        }
        let spoof = ScannableFile::from_bytes(
            PathBuf::from("ML-DSA-44-2.16.840.1.101.3.4.3.17.pem"),
            VALID_RSA_CERT_PEM.as_bytes().to_vec(),
        );
        assert!(CertificateDetector
            .detect(&spoof, &rules)
            .unwrap()
            .iter()
            .all(|h| !h.rule_id.starts_with("CERT_ML_DSA")));
        let unparsed = ScannableFile::from_bytes(
            PathBuf::from("2.16.840.1.101.3.4.3.20.pem"),
            b"not a certificate".to_vec(),
        );
        assert!(CertificateDetector
            .detect(&unparsed, &rules)
            .unwrap()
            .iter()
            .all(
                |h| !h.rule_id.starts_with("CERT_ML_DSA") && !h.rule_id.starts_with("CERT_SLH_DSA")
            ));
        assert_eq!(
            signature_name("2.16.840.1.101.3.4.3.32"),
            "2.16.840.1.101.3.4.3.32"
        );
    }

    #[test]
    fn falls_back_to_container_match_for_unparseable_pem() {
        let detector = CertificateDetector;
        let rules = RuleSet::load_from_dir(&rules_dir()).expect("load rules");
        let file = ScannableFile::from_bytes(
            PathBuf::from("fixtures/broken-cert.pem"),
            b"-----BEGIN CERTIFICATE-----\nnot-a-real-cert\n-----END CERTIFICATE-----\n".to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 1);
        let detection = &detections[0];
        assert_eq!(detection.rule_id, "CERT_PEM_CONTAINER");
        assert_eq!(detection.evidence.r#match, "broken-cert.pem");
        assert_eq!(
            detection.evidence.snippet_preview,
            "certificate container detected"
        );
        assert_eq!(
            detection.evidence.metadata.get("extension"),
            Some(&"pem".to_string())
        );
    }

    #[test]
    fn matches_certificate_metadata_for_valid_rsa_pem() {
        let detector = CertificateDetector;
        let rules = RuleSet::load_from_dir(&rules_dir()).expect("load rules");
        let file = ScannableFile::from_bytes(
            PathBuf::from("fixtures/valid-cert.pem"),
            VALID_RSA_CERT_PEM.as_bytes().to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");
        let ids = detections
            .iter()
            .map(|d| d.rule_id.as_str())
            .collect::<Vec<_>>();

        assert!(ids.contains(&"CERT_RSA_SIGNATURE"));
        assert!(detections
            .iter()
            .all(|d| d.evidence.r#type == "certificate"));
        assert!(detections
            .iter()
            .any(|d| d.evidence.metadata.contains_key("signature_oid")));
        assert!(detections.iter().any(|d| d
            .evidence
            .snippet_preview
            .contains("sig=sha256WithRSAEncryption")));
        assert!(detections.iter().any(|d| {
            d.evidence
                .metadata
                .get("signature_oid")
                .is_some_and(|oid| oid == "1.2.840.113549.1.1.11")
        }));
    }
}
