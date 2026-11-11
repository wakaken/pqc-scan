mod attributes;
mod pkcs12;

pub use attributes::{
    parse_certificate_der, parse_public_key_der, CertificateAttributes, PublicDerParseError,
    PublicKeyAttributes,
};

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use x509_parser::parse_x509_certificate;

use pqc_scan_core::classifier;
use pqc_scan_core::model::mask_preview;
use pqc_scan_core::{
    Detection, Detector, DetectorOutcome, DetectorStatus, Evidence, Location, ScannableFile,
};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug, Default)]
pub struct CertificateDetector;

struct LocatedCertificate {
    attributes: CertificateAttributes,
    line: usize,
    column: usize,
    object_index: usize,
    byte_offset: usize,
    pkcs12: bool,
}

impl Detector for CertificateDetector {
    fn name(&self) -> &'static str {
        "certificate_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(file, rules)?.into_detections())
    }

    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.inspect_with_time(file, rules, unix_seconds(SystemTime::now()), true)
    }

    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.detect_with_time(file, rules, unix_seconds(SystemTime::now()))
    }
}

impl CertificateDetector {
    /// A fixed Unix-second assessment clock, reusable across files/scans.
    /// This does not validate signatures, chains or actual certificate deployment.
    pub fn at(assessment_time_unix: i64) -> impl Detector {
        ClockedCertificateDetector {
            assessment_time: Some(assessment_time_unix),
        }
    }

    fn detect_with_time(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        assessment_time: Option<i64>,
    ) -> Result<DetectorOutcome> {
        self.inspect_with_time(file, rules, assessment_time, false)
    }

    fn inspect_with_time(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        assessment_time: Option<i64>,
        observe: bool,
    ) -> Result<DetectorOutcome> {
        let applicable = rules.by_kind(RuleKind::Certificate).next().is_some();
        if !classifier::is_certificate_file(&file.path) || (!applicable && !observe) {
            return Ok(DetectorOutcome::new(
                Vec::new(),
                DetectorStatus::NotApplicable,
            ));
        }
        let ext = file.ext().unwrap_or_default().to_ascii_lowercase();
        let p12 = matches!(ext.as_str(), "p12" | "pfx").then(|| pkcs12::inspect(&file.bytes));
        let p12_reason = p12.as_ref().map(|p| p.reason);
        let (objects, mut status) = if let Some(p12) = p12 {
            (p12.objects, p12.status)
        } else {
            // A DER name/extension may itself contain PEM-looking text. Prefer
            // actual DER parsing rather than a substring-based format guess.
            let der = parse_der_object(&file.bytes);
            if der.0.is_empty() {
                parse_pem_objects(&file.bytes)
            } else {
                der
            }
        };
        let file_path = file.path.to_string_lossy().into_owned();
        let mut container_detections = Vec::new();
        if objects.is_empty() || p12_reason.is_some() {
            // Compatibility: a filename/container signal is not a parsed certificate.
            let mut out = Vec::new();
            for rule in rules.by_kind(RuleKind::Certificate) {
                if matches!(
                    rule.scope.as_str(),
                    "signature_oid"
                        | "public_key_oid"
                        | "rsa_modulus_bits"
                        | "certificate_validity"
                        | "legacy_not_after_2026"
                ) {
                    continue;
                }
                if rule
                    .compiled_pattern()
                    .is_some_and(|r| r.is_match(&file_path))
                {
                    out.push(Detection {
                        rule_id: rule.id.clone(),
                        location: Location {
                            file: file_path.clone(),
                            line: 1,
                            column: 1,
                        },
                        evidence: Evidence {
                            r#type: "certificate".into(),
                            r#match: mask_preview(file.file_name().unwrap_or("certificate")),
                            snippet_preview: "certificate container detected".into(),
                            metadata: BTreeMap::from([
                                ("detector".into(), self.name().into()),
                                ("extension".into(), ext.clone()),
                            ]),
                        },
                    });
                }
            }
            if let Some(reason) = p12_reason {
                for hit in &mut out {
                    hit.evidence
                        .metadata
                        .insert("container_inspection".into(), reason.into());
                    hit.evidence
                        .metadata
                        .insert("mac_verification".into(), "not_performed".into());
                    hit.evidence
                        .metadata
                        .insert("trust_verification".into(), "not_performed".into());
                }
            }
            if !objects.is_empty() {
                container_detections = out;
            } else {
                let mut drafts = Vec::new();
                if observe {
                    drafts.push(pqc_scan_core::ObservationDraft::container(
                        &file_path, status, &out,
                    ));
                }
                return Ok(DetectorOutcome::new(
                    out,
                    if applicable {
                        status
                    } else {
                        DetectorStatus::NotApplicable
                    },
                )
                .with_observations(drafts));
            }
        }
        let mut out = container_detections;
        let mut observations = Vec::new();
        if observe && (status != DetectorStatus::Complete || p12_reason.is_some()) {
            observations.push(pqc_scan_core::ObservationDraft::container(
                &file_path, status, &out,
            ));
        }
        for object in objects {
            let start = out.len();
            let info = &object.attributes;
            if info.status != DetectorStatus::Complete {
                mark_partial(&mut status);
            }
            let interval_valid = info.not_before_unix <= info.not_after_unix;
            let validity = assessment_time.filter(|_| interval_valid).map(|now| {
                if now < info.not_before_unix {
                    "not_yet_valid"
                } else if now > info.not_after_unix {
                    "expired"
                } else {
                    "valid"
                }
            });
            if !interval_valid
                || (assessment_time.is_none()
                    && rules
                        .by_kind(RuleKind::Certificate)
                        .any(|r| r.scope == "certificate_validity"))
            {
                mark_partial(&mut status);
            }
            // Preserve composite input for legacy/custom scopes only.
            let legacy_signature = if info.signature_algorithm.starts_with("ML-DSA")
                || info.signature_algorithm.starts_with("SLH-DSA")
            {
                &info.signature_oid
            } else {
                &info.signature_algorithm
            };
            let key_bits = info
                .public_key
                .key_bits
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unknown".into());
            let target = format!(
                "{} {} {} {} {}",
                file_path, legacy_signature, info.signature_oid, key_bits, info.expires_at
            );
            for rule in rules.by_kind(RuleKind::Certificate) {
                if object.pkcs12 && rule.id == "CERT_P12_CONTAINER" {
                    continue;
                }
                let Some(regex) = rule.compiled_pattern() else {
                    mark_partial(&mut status);
                    continue;
                };
                let typed_target;
                let scoped_target = match rule.scope.as_str() {
                    "signature_oid" => &info.signature_oid,
                    "public_key_oid" => &info.public_key.algorithm_oid,
                    "rsa_modulus_bits" => {
                        if !matches!(
                            info.public_key.algorithm_oid.as_str(),
                            "1.2.840.113549.1.1.1" | "1.2.840.113549.1.1.10"
                        ) || info.public_key.key_size_kind != "rsa_modulus"
                        {
                            continue;
                        }
                        let Some(bits) = info.public_key.key_bits else {
                            continue;
                        };
                        typed_target = bits.to_string();
                        &typed_target
                    }
                    "certificate_validity" => {
                        let Some(state) = validity else {
                            continue;
                        };
                        state
                    }
                    "legacy_not_after_2026" => {
                        if !interval_valid {
                            continue;
                        }
                        if info.not_after_unix < 1_798_761_600 {
                            "before_2027"
                        } else {
                            "after_2026"
                        }
                    }
                    _ => &target,
                };
                if !regex.is_match(scoped_target) {
                    continue;
                }
                let mut metadata = object_metadata(&object);
                let snippet_preview = match (rule.scope.as_str(), assessment_time) {
                    ("certificate_validity", Some(now)) => {
                        metadata.insert("assessment_time_unix".into(), now.to_string());
                        metadata.insert("validity_status".into(), scoped_target.to_string());
                        format!("validity={}, assessment_time_unix={}, not_before_unix={}, not_after_unix={}",
                            scoped_target, now, info.not_before_unix, info.not_after_unix)
                    }
                    _ => format!(
                        "sig={}, key_bits={}, expires={}",
                        info.signature_algorithm, key_bits, info.expires_at
                    ),
                };
                out.push(Detection {
                    rule_id: rule.id.clone(),
                    location: Location {
                        file: file_path.clone(),
                        line: object.line,
                        column: object.column,
                    },
                    evidence: Evidence {
                        r#type: "certificate".into(),
                        r#match: match rule.scope.as_str() {
                            "public_key_oid" => info.public_key.algorithm.clone(),
                            "signature_oid" => info.signature_algorithm.clone(),
                            "rsa_modulus_bits" => {
                                format!("{} {}-bit", info.public_key.algorithm, scoped_target)
                            }
                            "certificate_validity" => scoped_target.to_string(),
                            "legacy_not_after_2026" => {
                                if scoped_target == "before_2027" {
                                    "notAfter < 2027".into()
                                } else {
                                    "notAfter >= 2027".into()
                                }
                            }
                            _ => mask_preview(&format!(
                                "{} {}-bit",
                                info.signature_algorithm, key_bits
                            )),
                        },
                        snippet_preview,
                        metadata,
                    },
                });
            }
            if observe {
                if let Some(facts) = pqc_scan_core::PublicCertificateFacts::from_parser_metadata(
                    &object_metadata(&object),
                ) {
                    observations.push(pqc_scan_core::ObservationDraft::certificate(
                        &file_path,
                        if object.pkcs12 { 0 } else { object.line },
                        if object.pkcs12 { 0 } else { object.column },
                        object.object_index,
                        facts,
                        info.status,
                        &out[start..],
                    ));
                } else {
                    observations.push(pqc_scan_core::ObservationDraft::container(
                        &file_path,
                        DetectorStatus::Partial,
                        &out[start..],
                    ));
                }
            }
        }
        Ok(DetectorOutcome::new(
            out,
            if applicable {
                status
            } else {
                DetectorStatus::NotApplicable
            },
        )
        .with_observations(observations))
    }
}

struct ClockedCertificateDetector {
    assessment_time: Option<i64>,
}

impl Detector for ClockedCertificateDetector {
    fn name(&self) -> &'static str {
        "certificate_detector"
    }
    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(file, rules)?.into_detections())
    }
    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        CertificateDetector.detect_with_time(file, rules, self.assessment_time)
    }
    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        CertificateDetector.inspect_with_time(file, rules, self.assessment_time, true)
    }
}

// Registry construction is the snapshot boundary. The CLI creates a new registry
// per command; library callers can reuse an explicit assessment instant or rebuild.
pub(crate) fn clock_snapshot() -> impl Detector {
    ClockedCertificateDetector {
        assessment_time: unix_seconds(SystemTime::now()),
    }
}

fn unix_seconds(now: SystemTime) -> Option<i64> {
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => i64::try_from(duration.as_secs()).ok(),
        Err(error) => {
            let duration = error.duration();
            i64::try_from(duration.as_secs())
                .ok()?
                .checked_neg()?
                .checked_sub(i64::from(duration.subsec_nanos() != 0))
        }
    }
}

// Limit exhaustion remains visible even when retained public facts are partial.
fn mark_partial(status: &mut DetectorStatus) {
    if *status != DetectorStatus::Truncated {
        *status = DetectorStatus::Partial;
    }
}

fn object_metadata(object: &LocatedCertificate) -> BTreeMap<String, String> {
    let info = &object.attributes;
    let key = &info.public_key;
    let mut m = BTreeMap::from([
        ("detector".into(), "certificate_detector".into()),
        ("signature_oid".into(), info.signature_oid.clone()),
        (
            "signature_algorithm".into(),
            info.signature_algorithm.clone(),
        ),
        ("public_key_oid".into(), key.algorithm_oid.clone()),
        ("public_key_algorithm".into(), key.algorithm.clone()),
        ("key_size_kind".into(), key.key_size_kind.into()),
        ("parameters_kind".into(), key.parameters_kind.into()),
        (
            "public_key_encoded_bytes".into(),
            key.encoded_bytes.to_string(),
        ),
        ("spki_der_bytes".into(), key.spki_der_bytes.to_string()),
        (
            "certificate_der_bytes".into(),
            info.certificate_der_bytes.to_string(),
        ),
        ("certificate_sha256".into(), info.certificate_sha256.clone()),
        ("spki_sha256".into(), key.spki_sha256.clone()),
        ("object_index".into(), object.object_index.to_string()),
        ("byte_offset".into(), object.byte_offset.to_string()),
        ("expires_at".into(), info.expires_at.clone()),
        ("not_before_unix".into(), info.not_before_unix.to_string()),
        ("not_after_unix".into(), info.not_after_unix.to_string()),
    ]);
    if object.pkcs12 {
        m.insert("source_format".into(), "pkcs12".into());
        m.insert("location_kind".into(), "binary_offset".into());
        m.insert("mac_verification".into(), "not_performed".into());
        m.insert("trust_verification".into(), "not_performed".into());
    }
    for (name, value) in [
        ("key_bits", key.key_bits.map(|n| n.to_string())),
        ("named_curve_oid", key.named_curve_oid.clone()),
        ("named_curve", key.named_curve.map(str::to_string)),
        ("parameters_oid", key.parameters_oid.clone()),
        (
            "parameters_content_sha256",
            key.parameters_content_sha256.clone(),
        ),
    ] {
        if let Some(value) = value {
            m.insert(name.into(), value);
        }
    }
    m
}

fn parse_der_object(bytes: &[u8]) -> (Vec<LocatedCertificate>, DetectorStatus) {
    let Ok((rest, cert)) = parse_x509_certificate(bytes) else {
        return (Vec::new(), DetectorStatus::Partial);
    };
    let consumed = bytes.len() - rest.len();
    let Ok(attributes) = attributes::certificate_attributes(&cert, &bytes[..consumed]) else {
        return (Vec::new(), DetectorStatus::Partial);
    };
    let status = if rest.is_empty() {
        attributes.status
    } else {
        DetectorStatus::Partial
    };
    (
        vec![LocatedCertificate {
            attributes,
            line: 1,
            column: 1,
            object_index: 1,
            byte_offset: 0,
            pkcs12: false,
        }],
        status,
    )
}

struct PemBlock<'a> {
    label: &'a [u8],
    byte_offset: usize,
    line: usize,
    column: usize,
    index: usize,
}

// Frame blocks independently so one bad block cannot discard good neighbors.
// Non-certificate bodies are never decoded or placed into evidence/errors.
fn parse_pem_objects(bytes: &[u8]) -> (Vec<LocatedCertificate>, DetectorStatus) {
    let mut objects = Vec::new();
    let mut block: Option<PemBlock<'_>> = None;
    let mut offset = 0;
    let mut index = 0;
    let mut malformed = false;
    let mut unsupported = false;
    for (line_index, line) in bytes.split_inclusive(|b| *b == b'\n').enumerate() {
        let trimmed = line.trim_ascii();
        if let Some(label) = pem_label(trimmed, b"-----BEGIN ") {
            if block.is_some() {
                malformed = true;
            }
            index += 1;
            let leading = line.len() - line.trim_ascii_start().len();
            block = Some(PemBlock {
                label,
                byte_offset: offset + leading,
                line: line_index + 1,
                column: leading + 1,
                index,
            });
        } else if let Some(label) = pem_label(trimmed, b"-----END ") {
            match block.take() {
                Some(b) if b.label == label => {
                    if label == b"CERTIFICATE" {
                        let decoded = pem::parse(&bytes[b.byte_offset..offset + line.len()]);
                        match decoded.and_then(|p| {
                            parse_certificate_der(p.contents())
                                .map_err(|_| pem::PemError::MalformedFraming)
                        }) {
                            Ok(attributes) => objects.push(LocatedCertificate {
                                attributes,
                                line: b.line,
                                column: b.column,
                                object_index: b.index,
                                byte_offset: b.byte_offset,
                                pkcs12: false,
                            }),
                            Err(_) => malformed = true,
                        }
                    } else {
                        unsupported = true;
                    }
                }
                _ => malformed = true,
            }
        } else if trimmed.starts_with(b"-----BEGIN") || trimmed.starts_with(b"-----END") {
            malformed = true;
        }
        offset += line.len();
    }
    malformed |= block.is_some();
    let incomplete_attributes = objects
        .iter()
        .any(|o| o.attributes.status != DetectorStatus::Complete);
    let status = if malformed || incomplete_attributes || (!objects.is_empty() && unsupported) {
        DetectorStatus::Partial
    } else if unsupported {
        DetectorStatus::Unsupported
    } else if objects.is_empty() {
        DetectorStatus::Partial
    } else {
        DetectorStatus::Complete
    };
    (objects, status)
}

fn pem_label<'a>(line: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    let label = line.strip_prefix(prefix)?.strip_suffix(b"-----")?;
    (!label.is_empty()).then_some(label)
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

    #[test]
    fn missing_assessment_clock_retains_static_facts_without_a_false_validity_verdict() {
        let rules = RuleSet::load_from_dir(&rules_dir()).unwrap();
        let file = ScannableFile::from_bytes(
            PathBuf::from("public.pem"),
            VALID_RSA_CERT_PEM.as_bytes().to_vec(),
        );
        let out = CertificateDetector
            .detect_with_time(&file, &rules, None)
            .unwrap();
        assert_eq!(out.status(), DetectorStatus::Partial);
        assert!(out
            .detections()
            .iter()
            .any(|d| d.rule_id == "CERT_RSA_SIGNATURE"));
        assert!(!out
            .detections()
            .iter()
            .any(|d| ["CERT_EXPIRED", "CERT_NOT_YET_VALID"].contains(&d.rule_id.as_str())));
        assert!(out
            .detections()
            .iter()
            .all(|d| !d.evidence.metadata.contains_key("assessment_time_unix")));
    }

    #[test]
    fn unix_clock_conversion_floors_seconds_on_both_sides_of_the_epoch() {
        use std::time::Duration;
        assert_eq!(unix_seconds(UNIX_EPOCH), Some(0));
        assert_eq!(
            unix_seconds(UNIX_EPOCH + Duration::new(1, 900_000_000)),
            Some(1)
        );
        assert_eq!(unix_seconds(UNIX_EPOCH - Duration::from_nanos(1)), Some(-1));
        assert_eq!(unix_seconds(UNIX_EPOCH - Duration::new(1, 1)), Some(-2));
    }
}
