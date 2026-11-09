//! Typed, allowlisted native observations. Raw legacy bindings are never serialized.
use crate::compat::LegacyBinding;
use crate::identity::safe_token;
use crate::{Detection, DetectorStatus};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlgorithmSpec {
    pub(crate) namespace: String,
    pub(crate) identifier: String,
    pub(crate) purpose: String,
    pub(crate) parameters: BTreeMap<String, String>,
}

impl AlgorithmSpec {
    /// Exact recognized vocabulary only. No arbitrary scanned identifier is retained.
    pub fn recognized(raw: &str) -> Option<Self> {
        let canonical = match raw {
            "ml-kem-512" | "ML-KEM-512" => "ML-KEM-512",
            "ml-kem-768" | "ML-KEM-768" | "MlKem768" | "MLKEM768PrivateKey" => "ML-KEM-768",
            "ml-kem-1024" | "ML-KEM-1024" | "MlKem1024" | "MLKEM1024PrivateKey" => "ML-KEM-1024",
            "ML-KEM" | "mlkem" | "ml_kem" => "ML-KEM",
            "ML-DSA" | "mldsa" | "ml_dsa" => "ML-DSA",
            "ML-DSA-44" | "ml-dsa-44" | "MLDSA44PrivateKey" => "ML-DSA-44",
            "ML-DSA-65" | "ml-dsa-65" | "MLDSA65PrivateKey" => "ML-DSA-65",
            "ML-DSA-87" | "ml-dsa-87" | "MLDSA87PrivateKey" => "ML-DSA-87",
            "SLH-DSA" | "slh_dsa" => "SLH-DSA",
            "RSA" | "RS256" | "RS384" | "RS512" | "ES256" | "Ed25519" => raw,
            "Kyber512"
            | "Kyber768"
            | "Kyber1024"
            | "Dilithium2"
            | "Dilithium3"
            | "Dilithium5"
            | "X25519Kyber768Draft00" => raw,
            _ => return None,
        };
        let namespace = if canonical.starts_with("Kyber")
            || canonical.starts_with("Dilithium")
            || canonical.contains("Draft")
        {
            "draft"
        } else {
            "named"
        };
        Some(Self {
            namespace: namespace.into(),
            identifier: canonical.into(),
            purpose: "reference".into(),
            parameters: BTreeMap::new(),
        })
    }
    pub(crate) fn oid(value: &str, purpose: &str, parameters: BTreeMap<String, String>) -> Self {
        Self {
            namespace: "oid".into(),
            identifier: value.into(),
            purpose: purpose.into(),
            parameters,
        }
    }
    pub(crate) fn key(&self) -> String {
        serde_json::to_string(self).expect("typed algorithm serialization")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObservationMethod {
    LexicalMatch,
    AstPattern,
    Configuration,
    Dependency,
    PrivateKeyMarker,
    CertificateParse,
    LegacyAdapter,
}
impl ObservationMethod {
    pub(crate) fn values(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::LexicalMatch => ("lexical_match", "lexical", "referenced"),
            Self::AstPattern => ("ast_pattern", "syntactic", "referenced"),
            Self::Configuration => ("configuration_match", "lexical", "configured"),
            Self::Dependency => ("dependency_report", "unknown", "referenced"),
            Self::PrivateKeyMarker => ("private_key_marker", "lexical", "unknown"),
            Self::CertificateParse => ("certificate_parse", "parsed", "unknown"),
            Self::LegacyAdapter => ("legacy_adapter", "unknown", "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeTarget {
    pub(crate) path: Option<String>,
    pub(crate) line: Option<usize>,
    pub(crate) column: Option<usize>,
    pub(crate) column_kind: String,
    pub(crate) byte_start: Option<usize>,
    pub(crate) byte_end: Option<usize>,
    pub(crate) object_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeEvidence {
    pub(crate) marker_kind: Option<String>,
    pub(crate) capture_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub(crate) id: String,
    pub(crate) occurrence_key: Option<String>,
    pub(crate) identity_confidence: String,
    pub(crate) asset_ids: Vec<String>,
    pub(crate) target: NativeTarget,
    pub(crate) method: String,
    pub(crate) producer: String,
    pub(crate) assurance: String,
    pub(crate) usage: String,
    pub(crate) rule_id: Option<String>,
    pub(crate) evidence: SafeEvidence,
    pub(crate) uncertainty: Vec<String>,
}
impl Observation {
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicCertificateFacts {
    pub(crate) certificate_sha256: String,
    pub(crate) spki_sha256: String,
    pub(crate) signature_oid: String,
    pub(crate) public_key_oid: String,
    pub(crate) key_bits: Option<u64>,
    pub(crate) named_curve_oid: Option<String>,
    pub(crate) parameters_kind: String,
    pub(crate) parameters_content_sha256: Option<String>,
    pub(crate) not_before_unix: i64,
    pub(crate) not_after_unix: i64,
}
fn sha(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn oid(value: &str) -> bool {
    value.len() <= 128
        && value.contains('.')
        && value
            .split('.')
            .all(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
}
impl PublicCertificateFacts {
    /// For explicit parser adapters only; legacy metadata is not a trust source.
    pub fn from_parser_metadata(m: &BTreeMap<String, String>) -> Option<Self> {
        let get = |key: &str| m.get(key).map(String::as_str);
        let cert = get("certificate_sha256")?;
        let key = get("spki_sha256")?;
        let signature = get("signature_oid")?;
        let public = get("public_key_oid")?;
        if !sha(cert) || !sha(key) || !oid(signature) || !oid(public) {
            return None;
        }
        let parameters_kind = match get("parameters_kind") {
            Some("absent") => "absent",
            Some("null") => "null",
            Some("oid") => "oid",
            Some("sequence") => "sequence",
            _ => "unknown",
        };
        Some(Self {
            certificate_sha256: cert.into(),
            spki_sha256: key.into(),
            signature_oid: signature.into(),
            public_key_oid: public.into(),
            key_bits: get("key_bits")
                .and_then(|v| v.parse().ok())
                .filter(|v| *v > 0),
            named_curve_oid: get("named_curve_oid")
                .filter(|v| oid(v))
                .map(str::to_string),
            parameters_kind: parameters_kind.into(),
            parameters_content_sha256: get("parameters_content_sha256")
                .filter(|v| sha(v))
                .map(str::to_string),
            not_before_unix: get("not_before_unix")?.parse().ok()?,
            not_after_unix: get("not_after_unix")?.parse().ok()?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyFacts {
    pub(crate) name: String,
    pub(crate) ecosystem: String,
    pub(crate) reported_version: Option<String>,
    pub(crate) declared_constraint: Option<String>,
    pub(crate) resolved_version: Option<String>,
    pub(crate) version_provenance: String,
}
impl DependencyFacts {
    pub(crate) fn from_detection(d: &Detection) -> Option<Self> {
        let m = &d.evidence.metadata;
        let name = m.get("dep_name")?;
        if name.is_empty()
            || name.len() > 160
            || name.contains("://")
            || name.contains("..")
            || !name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_.-@/:".contains(&b))
        {
            return None;
        }
        if name.contains('@')
            && !(name.starts_with('@')
                && name[1..].split('/').count() == 2
                && name[1..].split('/').all(safe_token))
        {
            return None;
        }
        if name.contains(':') && !(name.split(':').count() == 2 && name.split(':').all(safe_token))
        {
            return None;
        }
        let ecosystem = m
            .get("dep_ecosystem")
            .filter(|v| {
                ["cargo", "npm", "pypi", "gem", "maven", "golang", "unknown"].contains(&v.as_str())
            })
            .cloned()
            .unwrap_or("unknown".into());
        let package_valid = match ecosystem.as_str() {
            "npm" => {
                (safe_token(name) && name != ".")
                    || (name.starts_with('@')
                        && name[1..].split('/').count() == 2
                        && name[1..]
                            .split('/')
                            .all(|s| safe_token(s) && s != "." && s != ".."))
            }
            "maven" => {
                name.split(':').count() == 2
                    && name
                        .split(':')
                        .all(|s| safe_token(s) && s != "." && s != "..")
            }
            "golang" => {
                !name.starts_with('/')
                    && name
                        .split('/')
                        .all(|s| safe_token(s) && s != "." && s != "..")
            }
            _ => safe_token(name) && name != "." && name != "..",
        };
        if !package_valid {
            return None;
        }
        let version = m
            .get("dep_version")
            .filter(|v| {
                !v.is_empty()
                    && v.len() <= 64
                    && v.split(['|', ','])
                        .filter(|s| !s.trim().is_empty())
                        .all(|s| {
                            let t = s
                                .trim()
                                .trim_start_matches(['v', '^', '~', '<', '>', '=', ' ']);
                            t.starts_with(|c: char| c.is_ascii_digit() || c == '*')
                                && t.bytes()
                                    .all(|b| b.is_ascii_alphanumeric() || b".*+-_".contains(&b))
                        })
            })
            .cloned();
        Some(Self {
            name: name.clone(),
            ecosystem,
            reported_version: version.clone(),
            declared_constraint: None,
            resolved_version: None,
            version_provenance: if version.is_some() {
                "legacy_unclassified"
            } else {
                "unavailable_or_unsafe_version_omitted"
            }
            .into(),
        })
    }
}

/// Opaque draft: raw legacy equality material is transient and never serialized.
#[derive(Clone)]
pub struct ObservationDraft {
    pub(crate) raw_path: String,
    pub(crate) target: NativeTarget,
    pub(crate) method: String,
    pub(crate) producer: String,
    pub(crate) assurance: String,
    pub(crate) usage: String,
    pub(crate) rule_id: Option<String>,
    pub(crate) algorithms: Vec<AlgorithmSpec>,
    pub(crate) certificate: Option<PublicCertificateFacts>,
    pub(crate) dependency: Option<DependencyFacts>,
    pub(crate) evidence: SafeEvidence,
    pub(crate) bindings: Vec<LegacyBinding>,
}
impl ObservationDraft {
    pub fn source(
        d: &Detection,
        method: ObservationMethod,
        raw: &str,
        span: Option<(usize, usize)>,
    ) -> Self {
        let (method_name, assurance, usage) = method.values();
        let algorithms = if matches!(
            method,
            ObservationMethod::LegacyAdapter | ObservationMethod::PrivateKeyMarker
        ) {
            Vec::new()
        } else {
            let mut found: Vec<_> = raw
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
                .filter_map(AlgorithmSpec::recognized)
                .collect();
            found.sort_by_key(AlgorithmSpec::key);
            found.dedup();
            found
        };
        Self {
            raw_path: d.location.file.clone(),
            target: NativeTarget {
                path: None,
                line: (d.location.line > 0).then_some(d.location.line),
                column: (d.location.line > 0 && d.location.column > 0).then_some(d.location.column),
                column_kind: match method {
                    ObservationMethod::AstPattern => "legacy_mixed",
                    ObservationMethod::LegacyAdapter => "unknown",
                    _ => "unicode_code_points",
                }
                .into(),
                byte_start: span.filter(|s| s.0 <= s.1).map(|s| s.0),
                byte_end: span.filter(|s| s.0 <= s.1).map(|s| s.1),
                object_index: None,
            },
            producer: match method {
                ObservationMethod::LexicalMatch | ObservationMethod::Configuration => {
                    "regex_detector"
                }
                ObservationMethod::AstPattern => "tree_sitter_detector",
                ObservationMethod::Dependency => "dependency_detector",
                ObservationMethod::PrivateKeyMarker => "key_detector",
                ObservationMethod::CertificateParse => "certificate_detector",
                ObservationMethod::LegacyAdapter => "legacy_adapter",
            }
            .into(),
            method: method_name.into(),
            assurance: assurance.into(),
            usage: usage.into(),
            rule_id: safe_token(&d.rule_id).then(|| d.rule_id.clone()),
            algorithms,
            certificate: None,
            dependency: None,
            evidence: SafeEvidence {
                marker_kind: matches!(method, ObservationMethod::PrivateKeyMarker)
                    .then_some("private_marker".into()),
                capture_status: "legacy_reported".into(),
            },
            bindings: vec![LegacyBinding::from_detection(d)],
        }
    }
    pub(crate) fn bind_to_inspected_file(
        mut self,
        actual: &std::path::Path,
        relative: &str,
    ) -> Self {
        if self.raw_path == actual.to_string_lossy() || self.raw_path == relative {
            self.raw_path = actual.to_string_lossy().into_owned();
        } else {
            self.raw_path.clear();
            self.assurance = "unknown".into();
            self.usage = "unknown".into();
            self.algorithms.clear();
            self.certificate = None;
            self.dependency = None;
            self.evidence.capture_status = "target_mismatch".into();
        }
        self
    }
    pub fn with_producer(mut self, producer: &str) -> Self {
        self.producer = if safe_token(producer) {
            producer.into()
        } else {
            "unavailable".into()
        };
        self
    }
    pub(crate) fn fact_order(&self) -> String {
        serde_json::json!({"algorithms":self.algorithms,"certificate":self.certificate,"dependency":self.dependency,"evidence":self.evidence}).to_string()
    }
    pub fn legacy(d: &Detection) -> Self {
        Self::source(d, ObservationMethod::LegacyAdapter, "", None)
    }
    pub fn dependency(d: &Detection) -> Self {
        let mut draft = Self::source(d, ObservationMethod::Dependency, "", None);
        draft.dependency = DependencyFacts::from_detection(d);
        if draft.dependency.is_none() {
            draft.evidence.capture_status =
                "unsafe_or_unavailable_dependency_identity_omitted".into();
        }
        draft
    }
    pub fn certificate(
        path: &str,
        line: usize,
        column: usize,
        index: usize,
        facts: PublicCertificateFacts,
        status: DetectorStatus,
        bindings: &[Detection],
    ) -> Self {
        let dummy = Detection {
            rule_id: String::new(),
            location: crate::Location {
                file: path.into(),
                line,
                column,
            },
            evidence: crate::Evidence {
                r#type: String::new(),
                r#match: String::new(),
                snippet_preview: String::new(),
                metadata: BTreeMap::new(),
            },
        };
        let mut draft = Self::source(&dummy, ObservationMethod::CertificateParse, "", None);
        draft.target.object_index = Some(index);
        draft.certificate = Some(facts);
        draft.evidence.capture_status = format!("{status:?}").to_ascii_lowercase();
        draft.bindings = bindings.iter().map(LegacyBinding::from_detection).collect();
        draft
    }
    pub fn container(path: &str, status: DetectorStatus, bindings: &[Detection]) -> Self {
        let dummy = Detection {
            rule_id: String::new(),
            location: crate::Location {
                file: path.into(),
                line: 0,
                column: 0,
            },
            evidence: crate::Evidence {
                r#type: String::new(),
                r#match: String::new(),
                snippet_preview: String::new(),
                metadata: BTreeMap::new(),
            },
        };
        let mut draft = Self::legacy(&dummy).with_producer("certificate_detector");
        draft.method = "certificate_container".into();
        draft.evidence.capture_status = format!("{status:?}").to_ascii_lowercase();
        draft.bindings = bindings.iter().map(LegacyBinding::from_detection).collect();
        draft
    }
    pub(crate) fn anchor(&self) -> String {
        let facts = serde_json::json!({"rule":self.rule_id,"method":self.method,"producer":self.producer,"algorithms":self.algorithms,"certificate":self.certificate.as_ref().map(|c|&c.certificate_sha256),"dependency":self.dependency,"marker":self.evidence.marker_kind});
        crate::identity::native_id("anchor", &[&facts.to_string()])
    }
}

impl std::fmt::Debug for ObservationDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObservationDraft")
            .field("method", &self.method)
            .field("assurance", &self.assurance)
            .finish_non_exhaustive()
    }
}
impl PublicCertificateFacts {
    pub(crate) fn valid(&self) -> bool {
        sha(&self.certificate_sha256)
            && sha(&self.spki_sha256)
            && oid(&self.signature_oid)
            && oid(&self.public_key_oid)
            && self.named_curve_oid.as_deref().is_none_or(oid)
            && self.parameters_content_sha256.as_deref().is_none_or(sha)
            && ["absent", "null", "oid", "sequence", "unknown"]
                .contains(&self.parameters_kind.as_str())
            && self.key_bits.is_none_or(|v| v > 0)
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;
    #[test]
    fn package_grammar_rejects_paths_and_credential_shapes_before_identity() {
        for (ecosystem, name, valid) in [
            ("npm", "/SECRET_CANARY/absolute", false),
            ("npm", "foo//bar", false),
            ("npm", ".", false),
            ("npm", "../foo", false),
            ("npm", "@scope/name", true),
            ("npm", "scope/name", false),
            ("golang", "example.invalid/team/module", true),
            ("golang", "/example.invalid/module", false),
            ("maven", "org.example:module", true),
            ("cargo", "test-package", true),
            ("unknown", "foo/bar", false),
        ] {
            let d = Detection {
                rule_id: "TEST".into(),
                location: crate::Location {
                    file: "package.json".into(),
                    line: 1,
                    column: 1,
                },
                evidence: crate::Evidence {
                    r#type: "test".into(),
                    r#match: String::new(),
                    snippet_preview: String::new(),
                    metadata: BTreeMap::from([
                        ("dep_name".into(), name.into()),
                        ("dep_ecosystem".into(), ecosystem.into()),
                    ]),
                },
            };
            if !valid {
                assert_eq!(
                    ObservationDraft::dependency(&d).evidence.capture_status,
                    "unsafe_or_unavailable_dependency_identity_omitted"
                );
            }
            assert_eq!(
                DependencyFacts::from_detection(&d).is_some(),
                valid,
                "{ecosystem}:{name}"
            );
        }
    }
    #[test]
    fn bounded_prereleases_are_reported_but_urls_and_credentials_are_not() {
        for (version, safe) in [
            ("1.2.3-beta.1", true),
            ("3.0.0rc1", true),
            ("^1.2.3", true),
            ("https://token@host/1", false),
            ("1.2:password", false),
        ] {
            let d = Detection {
                rule_id: "TEST".into(),
                location: crate::Location {
                    file: "package.json".into(),
                    line: 1,
                    column: 1,
                },
                evidence: crate::Evidence {
                    r#type: "test".into(),
                    r#match: String::new(),
                    snippet_preview: String::new(),
                    metadata: BTreeMap::from([
                        ("dep_name".into(), "test-package".into()),
                        ("dep_version".into(), version.into()),
                        ("dep_ecosystem".into(), "npm".into()),
                    ]),
                },
            };
            let fact = DependencyFacts::from_detection(&d).unwrap();
            assert_eq!(fact.reported_version.is_some(), safe, "{version}");
            if !safe {
                assert_eq!(
                    fact.version_provenance,
                    "unavailable_or_unsafe_version_omitted"
                );
            }
        }
    }
}
