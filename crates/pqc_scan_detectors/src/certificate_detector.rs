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
