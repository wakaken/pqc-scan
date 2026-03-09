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
            let target = format!(
                "{} {} {} {} {}",
                file_path,
                info.signature_algorithm,
                info.signature_oid,
                info.key_bits,
                info.expires_at
            );

            for rule in rules.by_kind(RuleKind::Certificate) {
                let regex = match rule.compiled_pattern() {
                    Some(v) => v,
                    None => continue,
                };
                if !regex.is_match(&target) {
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
                        r#match: mask_preview(&format!(
                            "{} {}-bit",
                            info.signature_algorithm, info.key_bits
                        )),
                        snippet_preview: format!(
                            "sig={}, key_bits={}, expires={}",
                            info.signature_algorithm, info.key_bits, info.expires_at
                        ),
                        metadata: std::collections::BTreeMap::from([
                            ("detector".to_string(), self.name().to_string()),
                            ("signature_oid".to_string(), info.signature_oid.clone()),
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
