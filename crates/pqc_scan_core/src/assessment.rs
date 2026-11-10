//! Pure, versioned advisory facts. No legacy risk or cryptographic operation changes.
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const CATALOG_VERSION: &str = "local-assessment-1";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuantumExposure {
    Vulnerable,
    DesignedResistant,
    GenericSearch,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Purpose {
    KeyEstablishment,
    Signature,
    Encryption,
    Hash,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AttackModel {
    pub attack: String,
    pub exponent_numerator: u32,
    pub exponent_denominator: u32,
    pub assumption: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComponentAssessment {
    pub role: String,
    pub assessment: AlgorithmAssessment,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlgorithmAssessment {
    pub identifier: String,
    pub construction: String,
    pub standardization: String,
    pub quantum: QuantumExposure,
    pub purpose: Purpose,
    pub parameters_known: bool,
    pub classical_concerns: Vec<String>,
    pub attack_models: Vec<AttackModel>,
    pub components: Vec<ComponentAssessment>,
    pub combiner_assessment: Option<String>,
    pub source_ids: Vec<String>,
}
impl AlgorithmAssessment {
    pub(crate) fn unknown() -> Self {
        Self {
            identifier: "unknown".into(),
            construction: "unknown".into(),
            standardization: "unknown".into(),
            quantum: QuantumExposure::Unknown,
            purpose: Purpose::Unknown,
            parameters_known: false,
            classical_concerns: vec![],
            attack_models: vec![],
            components: vec![],
            combiner_assessment: None,
            source_ids: vec!["LOCAL026".into()],
        }
    }
}
fn query(attack: &str, bits: u32, denominator: u32) -> AttackModel {
    AttackModel{attack:attack.into(),exponent_numerator:bits,exponent_denominator:denominator,assumption:"ideal_oracle_queries_not_operational_security_bits; memory_parallelism_and_circuit_cost_not_estimated".into()}
}
fn number(p: &BTreeMap<String, String>, name: &str, max: u32) -> Option<u32> {
    p.get(name)?
        .parse::<u32>()
        .ok()
        .filter(|n| *n > 0 && *n <= max)
}
/// Exact finite vocabulary. Unknown input is never echoed or interpreted as an alias.
pub fn assess_algorithm(
    identifier: &str,
    parameters: &BTreeMap<String, String>,
) -> AlgorithmAssessment {
    let id = match identifier {
        "2.16.840.1.101.3.4.4.1" => "ML-KEM-512",
        "2.16.840.1.101.3.4.4.2" => "ML-KEM-768",
        "2.16.840.1.101.3.4.4.3" => "ML-KEM-1024",
        "2.16.840.1.101.3.4.3.17" => "ML-DSA-44",
        "2.16.840.1.101.3.4.3.18" => "ML-DSA-65",
        "2.16.840.1.101.3.4.3.19" => "ML-DSA-87",
        "2.16.840.1.101.3.4.3.20" => "SLH-DSA-SHA2-128s",
        "2.16.840.1.101.3.4.3.21" => "SLH-DSA-SHA2-128f",
        "2.16.840.1.101.3.4.3.22" => "SLH-DSA-SHA2-192s",
        "2.16.840.1.101.3.4.3.23" => "SLH-DSA-SHA2-192f",
        "2.16.840.1.101.3.4.3.24" => "SLH-DSA-SHA2-256s",
        "2.16.840.1.101.3.4.3.25" => "SLH-DSA-SHA2-256f",
        "2.16.840.1.101.3.4.3.26" => "SLH-DSA-SHAKE-128s",
        "2.16.840.1.101.3.4.3.27" => "SLH-DSA-SHAKE-128f",
        "2.16.840.1.101.3.4.3.28" => "SLH-DSA-SHAKE-192s",
        "2.16.840.1.101.3.4.3.29" => "SLH-DSA-SHAKE-192f",
        "2.16.840.1.101.3.4.3.30" => "SLH-DSA-SHAKE-256s",
        "2.16.840.1.101.3.4.3.31" => "SLH-DSA-SHAKE-256f",

        "1.2.840.113549.1.1.1" => "RSA",
        "1.2.840.113549.1.1.10" => "RSA-PSS",
        "1.2.840.113549.1.1.11" => "RSA-SHA256",
        "1.2.840.113549.1.1.12" => "RSA-SHA384",
        "1.2.840.113549.1.1.13" => "RSA-SHA512",
        "1.2.840.113549.1.1.5" => "RSA-SHA1",
        "1.2.840.113549.1.1.4" => "RSA-MD5",
        "1.2.840.10045.2.1" => "EC",
        "1.2.840.10045.4.3.2" | "1.2.840.10045.4.3.3" | "1.2.840.10045.4.3.4" => "ECDSA",
        "1.2.840.10045.4.1" => "ECDSA-SHA1",
        "1.2.840.10040.4.1" => "DSA",
        "1.2.840.10040.4.3" => "DSA-SHA1",
        "1.3.101.112" => "Ed25519",
        "1.3.101.110" => "X25519",
        "RSA"
        | "RSA-PSS"
        | "RS256"
        | "RS384"
        | "RS512"
        | "RSA-SHA256"
        | "RSA-SHA384"
        | "RSA-SHA512"
        | "RSA-SHA1"
        | "RSA-MD5"
        | "EC"
        | "ECDH"
        | "ECDSA"
        | "ECDSA-SHA1"
        | "ES256"
        | "DSA"
        | "DSA-SHA1"
        | "DH"
        | "Ed25519"
        | "X25519"
        | "ML-KEM"
        | "ML-KEM-512"
        | "ML-KEM-768"
        | "ML-KEM-1024"
        | "ML-DSA"
        | "ML-DSA-44"
        | "ML-DSA-65"
        | "ML-DSA-87"
        | "SLH-DSA"
        | "SLH-DSA-SHA2-128s"
        | "SLH-DSA-SHA2-128f"
        | "SLH-DSA-SHA2-192s"
        | "SLH-DSA-SHA2-192f"
        | "SLH-DSA-SHA2-256s"
        | "SLH-DSA-SHA2-256f"
        | "SLH-DSA-SHAKE-128s"
        | "SLH-DSA-SHAKE-128f"
        | "SLH-DSA-SHAKE-192s"
        | "SLH-DSA-SHAKE-192f"
        | "SLH-DSA-SHAKE-256s"
        | "SLH-DSA-SHAKE-256f"
        | "Kyber512"
        | "Kyber768"
        | "Kyber1024"
        | "Dilithium2"
        | "Dilithium3"
        | "Dilithium5"
        | "X25519Kyber768Draft00"
        | "AES-128"
        | "AES-192"
        | "AES-256"
        | "SHA-1"
        | "SHA-256"
        | "SHA-384"
        | "SHA-512"
        | "MD5" => identifier,
        _ => return AlgorithmAssessment::unknown(),
    };
    let mut a = AlgorithmAssessment::unknown();
    a.identifier = id.into();
    if id == "X25519Kyber768Draft00" {
        a.construction = "hybrid".into();
        a.standardization = "draft".into();
        a.purpose = Purpose::KeyEstablishment;
        a.components = [
            ("classical_component", "X25519"),
            ("post_quantum_component", "Kyber768"),
        ]
        .into_iter()
        .map(|(role, id)| ComponentAssessment {
            role: role.into(),
            assessment: assess_algorithm(id, &BTreeMap::new()),
        })
        .collect();
        a.combiner_assessment = Some("unverified_combiner_no_resistance_inference".into());
        return a;
    }
    if id.starts_with("ML-") || id.starts_with("SLH-DSA") {
        a.construction = "post_quantum".into();
        a.standardization = "nist_final".into();
        a.purpose = if id.starts_with("ML-KEM") {
            Purpose::KeyEstablishment
        } else {
            Purpose::Signature
        };
        a.parameters_known = !["ML-KEM", "ML-DSA", "SLH-DSA"].contains(&id);
        if a.parameters_known {
            a.quantum = QuantumExposure::DesignedResistant;
        }
        a.source_ids.push(
            if a.purpose == Purpose::KeyEstablishment {
                "S203"
            } else if id.starts_with("ML-DSA") {
                "S204"
            } else {
                "S205"
            }
            .into(),
        );
    } else if id.starts_with("Kyber") || id.starts_with("Dilithium") {
        a.construction = "post_quantum".into();
        a.standardization = "draft".into();
        a.purpose = if id.starts_with("Kyber") {
            Purpose::KeyEstablishment
        } else {
            Purpose::Signature
        };
    } else if id.starts_with("AES-") {
        let bits = match id {
            "AES-128" => 128,
            "AES-192" => 192,
            _ => 256,
        };
        a.construction = "symmetric".into();
        a.standardization = "classical_standard".into();
        a.quantum = QuantumExposure::GenericSearch;
        a.purpose = Purpose::Encryption;
        a.parameters_known = true;
        a.attack_models.push(query("generic_key_search", bits, 2));
        a.source_ids.push("NIST-PQ-FAQ".into());
    } else if id.starts_with("SHA-") || id == "MD5" {
        let bits = match id {
            "SHA-1" => 160,
            "SHA-256" => 256,
            "SHA-384" => 384,
            "SHA-512" => 512,
            _ => 128,
        };
        a.construction = "hash".into();
        a.standardization = "classical_standard".into();
        a.quantum = QuantumExposure::GenericSearch;
        a.purpose = Purpose::Hash;
        a.parameters_known = true;
        a.attack_models = vec![
            query("generic_preimage", bits, 2),
            query("generic_collision", bits, 3),
        ];
        a.source_ids
            .extend(["NIST-PQ-FAQ".into(), "BHT1997".into()]);
        if id == "MD5" || id == "SHA-1" {
            a.classical_concerns
                .push("weak_collision_resistance".into());
            a.source_ids
                .push(if id == "MD5" { "R6151" } else { "S131A2" }.into());
        }
    } else {
        a.construction = "classical_public_key".into();
        a.standardization = "classical_standard".into();
        a.quantum = QuantumExposure::Vulnerable;
        a.source_ids.push("NIST-PQ-FAQ".into());
        a.purpose = if ["X25519", "ECDH", "DH"].contains(&id) {
            Purpose::KeyEstablishment
        } else if ["RSA", "EC"].contains(&id) {
            Purpose::Unknown
        } else {
            Purpose::Signature
        };
        if ["Ed25519", "X25519", "ES256"].contains(&id) {
            a.parameters_known = true;
        }
        if id == "RSA" || id == "RSA-PSS" {
            if let Some(bits) = number(parameters, "key_bits", 65536) {
                a.parameters_known = true;
                if bits < 2048 {
                    a.classical_concerns
                        .push("public_key_below_local_minimum".into());
                    a.source_ids.push("S131A2".into());
                }
            }
        }
        if ["EC", "ECDH", "ECDSA"].contains(&id) {
            let bits = number(parameters, "key_bits", 1024).or_else(|| {
                match parameters.get("named_curve_oid").map(String::as_str) {
                    Some("1.2.840.10045.3.1.7") => Some(256),
                    Some("1.3.132.0.34") => Some(384),
                    Some("1.3.132.0.35") => Some(521),
                    _ => None,
                }
            });
            if let Some(bits) = bits {
                a.parameters_known = true;
                if bits < 224 {
                    a.classical_concerns
                        .push("public_key_below_local_minimum".into());
                    a.source_ids.push("S131A2".into());
                }
            }
        }
        if id.ends_with("SHA1") || id.ends_with("MD5") {
            a.classical_concerns
                .push("signature_uses_weak_collision_digest".into());
            a.source_ids.push(
                if id.ends_with("MD5") {
                    "R6151"
                } else {
                    "S131A2"
                }
                .into(),
            );
        }
    }
    if a.standardization == "nist_final"
        && parameters
            .get("parameters_kind")
            .is_some_and(|k| k != "absent")
    {
        a.parameters_known = false;
        a.classical_concerns
            .push("unexpected_parameters_for_parameterless_algorithm".into());
    }
    if parameters
        .get("parameter_interpretation")
        .is_some_and(|v| v == "not_available")
    {
        a.parameters_known = false;
    }
    if id == "RSA-PSS" {
        a.source_ids.push("R4055".into());
    }
    if identifier.starts_with("2.16.840.1.101.3.4.") {
        a.source_ids.push("CSOR".into());
    }
    a.source_ids.sort();
    a.source_ids.dedup();
    a
}

#[derive(Debug, Clone, Serialize)]
pub struct ObservationSupport {
    pub id: String,
    pub method: String,
    pub assurance: String,
    pub usage: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct AssetAssessment {
    pub id: String,
    pub asset_id: String,
    pub asset_kind: String,
    pub algorithm: AlgorithmAssessment,
    pub certainty: String,
    pub observations: Vec<ObservationSupport>,
    pub limitations: Vec<String>,
}
pub(crate) fn valid_id(value: &str, domain: &str) -> bool {
    value
        .strip_prefix(&format!("{domain}-v1-"))
        .is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
}
fn vocabulary(value: &str, allowed: &[&str]) -> String {
    if allowed.contains(&value) {
        value
    } else {
        "unknown"
    }
    .into()
}
fn from_asset(
    a: &crate::inventory::CryptoAsset,
    extra: Option<&serde_json::Value>,
) -> AlgorithmAssessment {
    let id = a.attributes["identifier"].as_str().unwrap_or("");
    let namespace = a.attributes["namespace"].as_str().unwrap_or("");
    let draft =
        id.starts_with("Kyber") || id.starts_with("Dilithium") || id == "X25519Kyber768Draft00";
    let numeric = id.contains('.') && id.bytes().all(|b| b.is_ascii_digit() || b == b'.');
    if !((namespace == "oid" && numeric)
        || (namespace == "draft" && draft)
        || (namespace == "named" && !numeric && !draft))
    {
        return AlgorithmAssessment::unknown();
    }
    let mut p: BTreeMap<String, String> = a.attributes["parameters"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.iter())
        .filter(|(k, _)| {
            [
                "key_bits",
                "named_curve_oid",
                "parameter_interpretation",
                "parameters_kind",
            ]
            .contains(&k.as_str())
        })
        .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.into())))
        .collect();
    if let Some(extra) = extra {
        if let Some(bits) = extra["key_bits"].as_u64().filter(|b| *b > 0 && *b <= 65536) {
            p.insert("key_bits".into(), bits.to_string());
        }
        if let Some(curve) = extra["named_curve_oid"]
            .as_str()
            .filter(|c| ["1.2.840.10045.3.1.7", "1.3.132.0.34", "1.3.132.0.35"].contains(c))
        {
            p.insert("named_curve_oid".into(), curve.into());
        }
    }
    assess_algorithm(a.attributes["identifier"].as_str().unwrap_or(""), &p)
}
fn related<'a>(
    a: &crate::inventory::CryptoAsset,
    relationship: &str,
    kind: &str,
    index: &BTreeMap<&str, &'a crate::inventory::CryptoAsset>,
) -> Option<&'a crate::inventory::CryptoAsset> {
    let targets: Vec<_> = a
        .relations
        .iter()
        .filter(|r| r.relationship == relationship)
        .collect();
    if targets.len() != 1 {
        return None;
    }
    index
        .get(targets[0].target.as_str())
        .copied()
        .filter(|a| a.kind == kind)
}
fn key_assessment(
    a: &crate::inventory::CryptoAsset,
    index: &BTreeMap<&str, &crate::inventory::CryptoAsset>,
) -> AlgorithmAssessment {
    related(a, "algorithm", "algorithm", index)
        .map(|algorithm| from_asset(algorithm, Some(&a.attributes)))
        .unwrap_or_else(AlgorithmAssessment::unknown)
}
/// Evaluate captured facts only, with fixed output vocabulary and bounded kind-specific links.
pub fn evaluate_inventory(
    native: &crate::inventory::NativeScan,
) -> anyhow::Result<Vec<AssetAssessment>> {
    native
        .validate()
        .map_err(|_| anyhow::anyhow!("invalid inventory references"))?;
    anyhow::ensure!(
        valid_id(&native.run.id, "scan")
            && native.assets.iter().all(|a| valid_id(&a.id, "asset"))
            && native
                .observations
                .iter()
                .all(|o| valid_id(&o.id, "observation")),
        "invalid inventory identities"
    );
    let index: BTreeMap<_, _> = native.assets.iter().map(|a| (a.id.as_str(), a)).collect();
    let mut support: BTreeMap<&str, Vec<&crate::observation::Observation>> = BTreeMap::new();
    for observation in &native.observations {
        for asset_id in observation.asset_ids.iter().collect::<BTreeSet<_>>() {
            support
                .entry(asset_id.as_str())
                .or_default()
                .push(observation);
        }
    }
    let now = chrono::DateTime::parse_from_rfc3339(&native.run.generated_at)
        .ok()
        .map(|t| t.timestamp());
    let mut results = vec![];
    for asset in &native.assets {
        let kind = vocabulary(
            &asset.kind,
            &[
                "algorithm",
                "public_key",
                "certificate",
                "usage",
                "dependency",
            ],
        );
        let algorithm = match kind.as_str() {
            "algorithm" => from_asset(asset, None),
            "public_key" => key_assessment(asset, &index),
            "certificate" => {
                let mut result = AlgorithmAssessment::unknown();
                result.identifier = "certificate_roles".into();
                result.construction = "certificate".into();
                result.purpose = Purpose::Signature;
                if let Some(sig) = related(asset, "signature_algorithm", "algorithm", &index) {
                    let mut a = from_asset(sig, None);
                    if a.purpose == Purpose::Unknown
                        && ["RSA", "EC"].contains(&a.identifier.as_str())
                    {
                        a.purpose = Purpose::Signature;
                    } else if a.purpose != Purpose::Unknown && a.purpose != Purpose::Signature {
                        a.classical_concerns
                            .push("certificate_signature_purpose_mismatch".into());
                        a.parameters_known = false;
                    }
                    result.components.push(ComponentAssessment {
                        role: "certificate_signature".into(),
                        assessment: a,
                    });
                }
                if let Some(key) = related(asset, "subject_key", "public_key", &index) {
                    result.components.push(ComponentAssessment {
                        role: "subject_public_key".into(),
                        assessment: key_assessment(key, &index),
                    });
                }
                result.classical_concerns = result
                    .components
                    .iter()
                    .flat_map(|c| c.assessment.classical_concerns.iter().cloned())
                    .collect();
                result.quantum = if result
                    .components
                    .iter()
                    .any(|c| c.assessment.quantum == QuantumExposure::Vulnerable)
                {
                    QuantumExposure::Vulnerable
                } else if result.components.len() == 2
                    && result
                        .components
                        .iter()
                        .all(|c| c.assessment.quantum == QuantumExposure::DesignedResistant)
                {
                    QuantumExposure::DesignedResistant
                } else {
                    QuantumExposure::Unknown
                };
                if result
                    .classical_concerns
                    .iter()
                    .any(|c| c == "certificate_signature_purpose_mismatch")
                {
                    result.quantum = QuantumExposure::Unknown;
                }
                result.parameters_known = result.components.len() == 2
                    && result
                        .components
                        .iter()
                        .all(|c| c.assessment.parameters_known);
                match (
                    now,
                    asset.attributes["not_before_unix"].as_i64(),
                    asset.attributes["not_after_unix"].as_i64(),
                ) {
                    (Some(now), Some(start), Some(end)) if start <= end => {
                        if now < start {
                            result
                                .classical_concerns
                                .push("certificate_not_yet_valid".into());
                        }
                        if now > end {
                            result.classical_concerns.push("certificate_expired".into());
                        }
                    }
                    (_, Some(start), Some(end)) if start > end => result
                        .classical_concerns
                        .push("invalid_certificate_validity_interval".into()),
                    _ => {
                        result
                            .classical_concerns
                            .push("certificate_validity_unknown".into());
                        result.parameters_known = false;
                    }
                }
                result.source_ids.extend(
                    result
                        .components
                        .iter()
                        .flat_map(|c| c.assessment.source_ids.iter().cloned()),
                );
                result.source_ids.sort();
                result.source_ids.dedup();
                result.classical_concerns.sort();
                result.classical_concerns.dedup();
                result
            }
            _ => AlgorithmAssessment::unknown(),
        };
        let observations: Vec<_> = support
            .get(asset.id.as_str())
            .into_iter()
            .flatten()
            .map(|o| ObservationSupport {
                id: o.id.clone(),
                method: vocabulary(
                    &o.method,
                    &[
                        "lexical_match",
                        "ast_pattern",
                        "configuration_match",
                        "dependency_report",
                        "private_key_marker",
                        "certificate_parse",
                        "legacy_adapter",
                    ],
                ),
                assurance: vocabulary(&o.assurance, &["unknown", "lexical", "syntactic", "parsed"]),
                usage: vocabulary(&o.usage, &["unknown", "referenced", "configured"]),
            })
            .collect();
        results.push(AssetAssessment {
            id: crate::identity::native_id(
                "assessment",
                &[&native.run.id, &asset.id, CATALOG_VERSION],
            ),
            asset_id: asset.id.clone(),
            asset_kind: kind,
            algorithm,
            certainty: if observations.iter().all(|o| o.assurance == "unknown") {
                "unknown"
            } else {
                "uncalibrated_static_evidence"
            }
            .into(),
            observations,
            limitations: vec![
                "references_do_not_prove_active_use".into(),
                "not_a_full_classical_security_review".into(),
                "trust_signatures_and_implementation_not_verified".into(),
            ],
        });
    }
    Ok(results)
}
