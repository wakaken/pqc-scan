use std::fmt;

use pqc_scan_core::DetectorStatus;
use sha2::{Digest, Sha256};
use x509_parser::asn1_rs::{Any, Class, FromDer, Tag};
use x509_parser::der_parser::der::{parse_der_integer, parse_der_sequence_defined_g};
use x509_parser::der_parser::error::{BerError, BerResult};
use x509_parser::prelude::{SubjectPublicKeyInfo, X509Certificate};

/// Public facts only. No raw key/parameter bytes, names or private material.
#[derive(Debug, Clone)]
pub struct PublicKeyAttributes {
    pub algorithm_oid: String,
    pub algorithm: String,
    /// RSA modulus bits or known EC field bits; never encoded size/security strength.
    pub key_bits: Option<usize>,
    pub key_size_kind: &'static str,
    pub named_curve_oid: Option<String>,
    pub named_curve: Option<&'static str>,
    pub parameters_kind: &'static str,
    pub parameters_oid: Option<String>,
    /// Digest of parameter content, excluding its ASN.1 tag/length.
    pub parameters_content_sha256: Option<String>,
    pub encoded_bytes: usize,
    pub spki_der_bytes: usize,
    pub spki_sha256: String,
    /// Completeness of supported attribute extraction, not cryptographic validation.
    pub status: DetectorStatus,
}

#[derive(Debug, Clone)]
pub struct CertificateAttributes {
    pub signature_oid: String,
    pub signature_algorithm: String,
    pub public_key: PublicKeyAttributes,
    pub certificate_der_bytes: usize,
    pub certificate_sha256: String,
    pub not_before_unix: i64,
    pub not_after_unix: i64,
    /// Existing representation retained for legacy certificate rule inputs.
    pub expires_at: String,
    pub status: DetectorStatus,
}

/// Fixed safe error: never includes input bytes or upstream parser diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicDerParseError;

impl fmt::Display for PublicDerParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Public DER object could not be parsed completely.")
    }
}

impl std::error::Error for PublicDerParseError {}

/// Parse exactly one certificate. Does not verify its signature or trust chain.
pub fn parse_certificate_der(der: &[u8]) -> Result<CertificateAttributes, PublicDerParseError> {
    let (rest, cert) = x509_parser::parse_x509_certificate(der).map_err(|_| PublicDerParseError)?;
    if !rest.is_empty() {
        return Err(PublicDerParseError);
    }
    certificate_attributes(&cert, der)
}

/// Parse exactly one SubjectPublicKeyInfo, not PKCS#1/private-key/container DER.
/// Attribute status must be checked: a parsed OID does not validate key material.
pub fn parse_public_key_der(der: &[u8]) -> Result<PublicKeyAttributes, PublicDerParseError> {
    validate_spki_layout(der)?;
    let (rest, spki) = SubjectPublicKeyInfo::from_der(der).map_err(|_| PublicDerParseError)?;
    if !rest.is_empty() || spki.raw != der {
        return Err(PublicDerParseError);
    }
    Ok(public_key_attributes(&spki))
}

pub(super) fn certificate_attributes(
    cert: &X509Certificate<'_>,
    der: &[u8],
) -> Result<CertificateAttributes, PublicDerParseError> {
    let spki_der = validate_certificate_layout(der)?;
    if cert.public_key().raw != spki_der {
        return Err(PublicDerParseError);
    }
    let signature_oid = cert.signature_algorithm.algorithm.to_id_string();
    let public_key = public_key_attributes(cert.public_key());
    let status = if public_key.status != DetectorStatus::Complete
        || cert.signature_algorithm != cert.tbs_certificate.signature
        || cert.signature_value.unused_bits != 0
        || pqc_signature_size(&signature_oid).is_some_and(|size| {
            cert.signature_algorithm.parameters.is_some() || cert.signature_value.data.len() != size
        }) {
        DetectorStatus::Partial
    } else {
        DetectorStatus::Complete
    };
    Ok(CertificateAttributes {
        signature_algorithm: super::signature_name(&signature_oid),
        signature_oid,
        public_key,
        certificate_der_bytes: der.len(),
        certificate_sha256: fingerprint(der),
        not_before_unix: cert.validity().not_before.timestamp(),
        not_after_unix: cert.validity().not_after.timestamp(),
        expires_at: format!("{:?}", cert.validity().not_after),
        status,
    })
}

// The pinned x509/DER container parser discards inner remainders. Check the
// bounded, expected envelope fields using its existing shallow ASN.1 decoder;
// otherwise an ignored field can also shorten SPKI::raw and its fingerprint.
fn sequence_fields(der: &[u8], max: usize) -> Result<Vec<&[u8]>, PublicDerParseError> {
    let (rest, sequence) = Any::from_der(der).map_err(|_| PublicDerParseError)?;
    if !rest.is_empty()
        || sequence.class() != Class::Universal
        || sequence.tag() != Tag::Sequence
        || !sequence.header.is_constructed()
    {
        return Err(PublicDerParseError);
    }
    let mut fields = Vec::new();
    let mut input = sequence.as_bytes();
    while !input.is_empty() {
        if fields.len() == max {
            return Err(PublicDerParseError);
        }
        let (rest, _) = Any::from_der(input).map_err(|_| PublicDerParseError)?;
        fields.push(&input[..input.len() - rest.len()]);
        input = rest;
    }
    Ok(fields)
}

fn validate_algorithm_identifier(der: &[u8]) -> Result<(), PublicDerParseError> {
    let fields = sequence_fields(der, 2)?;
    let first = fields.first().ok_or(PublicDerParseError)?;
    let (_, oid) = Any::from_der(first).map_err(|_| PublicDerParseError)?;
    oid.as_oid().map_err(|_| PublicDerParseError)?;
    Ok(())
}

fn validate_spki_layout(der: &[u8]) -> Result<(), PublicDerParseError> {
    let fields = sequence_fields(der, 2)?;
    if fields.len() != 2 {
        return Err(PublicDerParseError);
    }
    validate_algorithm_identifier(fields[0])
}

fn validate_certificate_layout(der: &[u8]) -> Result<&[u8], PublicDerParseError> {
    let certificate = sequence_fields(der, 3)?;
    if certificate.len() != 3 {
        return Err(PublicDerParseError);
    }
    validate_algorithm_identifier(certificate[1])?;
    let tbs = sequence_fields(certificate[0], 10)?;
    let base = usize::from(tbs.first().is_some_and(|v| v.first() == Some(&0xa0)));
    if tbs.len() < base + 6 {
        return Err(PublicDerParseError);
    }
    if base == 1 {
        let (_, version) = Any::from_der(tbs[0]).map_err(|_| PublicDerParseError)?;
        let (rest, _) = Any::from_der(version.as_bytes()).map_err(|_| PublicDerParseError)?;
        if !rest.is_empty() {
            return Err(PublicDerParseError);
        }
    }
    validate_algorithm_identifier(tbs[base + 1])?;
    if sequence_fields(tbs[base + 3], 2)?.len() != 2 {
        return Err(PublicDerParseError);
    }
    let spki = tbs[base + 5];
    validate_spki_layout(spki)?;
    let mut previous = 0;
    for optional in &tbs[base + 6..] {
        let tag = match optional.first() {
            Some(0x81) => 1,
            Some(0x82) => 2,
            Some(0xa3) => 3,
            _ => return Err(PublicDerParseError),
        };
        if tag <= previous {
            return Err(PublicDerParseError);
        }
        previous = tag;
    }
    Ok(spki)
}

fn public_key_attributes(spki: &SubjectPublicKeyInfo<'_>) -> PublicKeyAttributes {
    let oid = spki.algorithm.algorithm.to_id_string();
    let parameters = spki.algorithm.parameters.as_ref();
    let parameters_oid = parameters
        .and_then(|p| p.as_oid().ok())
        .map(|o| o.to_id_string());
    let mut out = PublicKeyAttributes {
        algorithm: match oid.as_str() {
            "1.2.840.113549.1.1.1" => "RSA".into(),
            "1.2.840.113549.1.1.10" => "RSA-PSS".into(),
            "1.2.840.10045.2.1" => "EC".into(),
            "1.2.840.10040.4.1" | "1.3.14.3.2.12" => "DSA".into(),
            "1.3.101.112" => "Ed25519".into(),
            "1.3.101.113" => "Ed448".into(),
            "1.3.101.110" => "X25519".into(),
            "1.3.101.111" => "X448".into(),
            _ => super::signature_name(&oid),
        },
        algorithm_oid: oid.clone(),
        key_bits: None,
        key_size_kind: "unknown",
        named_curve_oid: None,
        named_curve: None,
        parameters_kind: match parameters {
            None => "absent",
            Some(p) if p.as_null().is_ok() => "null",
            Some(_) if parameters_oid.is_some() => "oid",
            Some(_) => "other",
        },
        parameters_oid,
        parameters_content_sha256: parameters.map(|p| fingerprint(p.as_bytes())),
        encoded_bytes: spki.subject_public_key.data.len(),
        spki_der_bytes: spki.raw.len(),
        spki_sha256: fingerprint(spki.raw),
        status: DetectorStatus::Unsupported,
    };
    if spki.subject_public_key.unused_bits != 0 {
        out.status = DetectorStatus::Partial;
        return out;
    }
    let data = spki.subject_public_key.data.as_ref();
    match oid.as_str() {
        "1.2.840.113549.1.1.1" | "1.2.840.113549.1.1.10" => {
            let parameters_ok = if oid == "1.2.840.113549.1.1.1" {
                parameters.is_none() || out.parameters_kind == "null"
            } else {
                parameters.is_none() || parameters.is_some_and(|p| p.as_sequence().is_ok())
            };
            out.status = DetectorStatus::Partial;
            if parameters_ok {
                if let Ok((rest, bits)) = parse_rsa_modulus_bits(data) {
                    if rest.is_empty() {
                        out.key_bits = Some(bits);
                        out.key_size_kind = "rsa_modulus";
                        out.status = DetectorStatus::Complete;
                    }
                }
            }
        }
        "1.2.840.10045.2.1" => {
            out.named_curve_oid = out.parameters_oid.clone();
            let known = match out.named_curve_oid.as_deref() {
                Some("1.2.840.10045.3.1.7") => Some(("P-256", 256_usize)),
                Some("1.3.132.0.34") => Some(("P-384", 384)),
                Some("1.3.132.0.35") => Some(("P-521", 521)),
                _ => None,
            };
            if let Some((name, bits)) = known {
                out.named_curve = Some(name);
                let coordinate_bytes = bits.div_ceil(8);
                let shape_ok = match data.first() {
                    Some(4) => data.len() == 1 + 2 * coordinate_bytes,
                    Some(2 | 3) => data.len() == 1 + coordinate_bytes,
                    _ => false,
                };
                out.status = DetectorStatus::Partial;
                if shape_ok {
                    out.key_bits = Some(bits);
                    out.key_size_kind = "ec_field";
                    out.status = DetectorStatus::Complete;
                }
            }
        }
        "1.3.101.110" | "1.3.101.111" | "1.3.101.112" | "1.3.101.113" => {
            let expected = match oid.as_str() {
                "1.3.101.110" | "1.3.101.112" => 32,
                "1.3.101.111" => 56,
                _ => 57,
            };
            out.status = if parameters.is_none() && data.len() == expected {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Partial
            };
        }
        // RFC 9881 / RFC 9909: raw public-key octets and absent parameters.
        // Shape checks establish extraction completeness, never key validity,
        // signature verification, security strength, or trust.
        "2.16.840.1.101.3.4.3.17" | "2.16.840.1.101.3.4.3.18" | "2.16.840.1.101.3.4.3.19" => {
            let expected = match oid.as_str() {
                "2.16.840.1.101.3.4.3.17" => 1312,
                "2.16.840.1.101.3.4.3.18" => 1952,
                _ => 2592,
            };
            out.status = if parameters.is_none() && data.len() == expected {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Partial
            };
        }
        _ if oid
            .strip_prefix("2.16.840.1.101.3.4.3.")
            .and_then(|s| s.parse::<u8>().ok())
            .is_some_and(|n| (20..=31).contains(&n)) =>
        {
            let n = oid.rsplit('.').next().unwrap().parse::<u8>().unwrap();
            let expected = match (n - 20) % 6 {
                0 | 1 => 32,
                2 | 3 => 48,
                _ => 64,
            };
            out.status = if parameters.is_none() && data.len() == expected {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Partial
            };
        }
        "1.2.840.10040.4.1" | "1.3.14.3.2.12" => {
            out.status = DetectorStatus::Partial;
            if let Some(p) = parameters {
                if let Ok(fields) = integer_parameter_fields(p) {
                    let valid = fields.len() == 3
                        && fields.iter().all(|f| {
                            Any::from_der(f).ok().is_some_and(|(rest, v)| {
                                rest.is_empty()
                                    && v.tag() == Tag::Integer
                                    && positive_integer_bits(v.as_bytes()).is_some()
                            })
                        });
                    let y = Any::from_der(data).ok();
                    if valid
                        && y.is_some_and(|(rest, y)| {
                            rest.is_empty()
                                && y.tag() == Tag::Integer
                                && positive_integer_bits(y.as_bytes()).is_some()
                        })
                    {
                        out.status = DetectorStatus::Complete;
                    }
                }
            }
        }
        _ => {}
    }
    out
}

fn parse_rsa_modulus_bits(bytes: &[u8]) -> BerResult<'_, usize> {
    parse_der_sequence_defined_g(|input, _| {
        let (rest, modulus) = parse_der_integer(input)?;
        let (rest, exponent) = parse_der_integer(rest)?;
        let modulus = modulus.as_slice().map_err(x509_parser::nom::Err::Error)?;
        let exponent = exponent.as_slice().map_err(x509_parser::nom::Err::Error)?;
        let bits = positive_integer_bits(modulus);
        if !rest.is_empty() || bits.is_none() || positive_integer_bits(exponent).is_none() {
            return Err(x509_parser::nom::Err::Error(BerError::InvalidValue {
                tag: x509_parser::asn1_rs::Tag::Integer,
                msg: "Invalid RSA public integer structure".into(),
            }));
        }
        Ok((rest, bits.unwrap()))
    })(bytes)
}

fn positive_integer_bits(bytes: &[u8]) -> Option<usize> {
    let (&first, rest) = bytes.split_first()?;
    if first & 0x80 != 0 {
        return None;
    }
    let value = if first == 0 {
        if rest.first().is_none_or(|v| v & 0x80 == 0) {
            return None;
        }
        rest
    } else {
        bytes
    };
    Some(value.len() * 8 - value[0].leading_zeros() as usize)
}

fn fingerprint(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn integer_parameter_fields<'a>(p: &Any<'a>) -> Result<Vec<&'a [u8]>, PublicDerParseError> {
    if p.tag() != Tag::Sequence {
        return Err(PublicDerParseError);
    }
    let mut input = p.as_bytes();
    let mut fields = Vec::new();
    while !input.is_empty() {
        if fields.len() == 3 {
            return Err(PublicDerParseError);
        }
        let (rest, _) = Any::from_der(input).map_err(|_| PublicDerParseError)?;
        fields.push(&input[..input.len() - rest.len()]);
        input = rest;
    }
    Ok(fields)
}

// RFC 9881 and RFC 9909 fixed signature encodings; no signature verification.
fn pqc_signature_size(oid: &str) -> Option<usize> {
    let n = oid
        .strip_prefix("2.16.840.1.101.3.4.3.")?
        .parse::<u8>()
        .ok()?;
    match n {
        17 => Some(2420),
        18 => Some(3309),
        19 => Some(4627),
        20..=43 => Some(match (n - 20) % 6 {
            0 => 7856,
            1 => 17088,
            2 => 16224,
            3 => 35664,
            4 => 29792,
            _ => 49856,
        }),
        _ => None,
    }
}
