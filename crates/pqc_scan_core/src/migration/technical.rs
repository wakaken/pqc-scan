//! New-only finite mapping, pinned to task066 F01-F12. Legacy helpers are unchanged.
use crate::assessment::{
    assess_algorithm, AlgorithmAssessment, ComponentAssessment, Purpose, QuantumExposure,
};
use serde::Serialize;
use std::collections::BTreeMap;

pub const CATALOG_ID: &str = "migration-facts-v1";
pub const CATALOG_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize)]
pub struct TechnicalSource {
    pub id: &'static str,
    pub edition: &'static str,
    pub clause: &'static str,
    pub uri: &'static str,
    pub scope_limit: &'static str,
}
pub fn source(id: &str) -> Option<TechnicalSource> {
    let (id, edition, clause, uri, scope_limit) = match id {
        "LOCAL026" => ("LOCAL026", "local-assessment-1", "finite vocabulary and evidence limits", "docs/architecture/LOCAL-ASSESSMENT-V1.md", "advisory facts; no deployment or cryptographic verification"),
        "S203" => ("S203", "FIPS203 Final 2024-08-13", "1.1; 3.1; 7; 8 Table2", "https://csrc.nist.gov/pubs/fips/203/final", "KEM parameter names/purpose only"),
        "S204" => ("S204", "FIPS204 Final 2024-08-13", "1.1; 4 Table1", "https://csrc.nist.gov/pubs/fips/204/final", "pure signature names/purpose; no prehash equivalence"),
        "S205" => ("S205", "FIPS205 Final 2024-08-13", "10.2.1; 10.2.2; 11 Table2", "https://csrc.nist.gov/pubs/fips/205/final", "exact pure signature names/purpose only"),
        "CSOR" => ("CSOR", "task056 pinned CSOR snapshot", "pure PQC OID namespaces", "https://csrc.nist.gov/projects/computer-security-objects-register/algorithm-registration", "OID identity is not parameter-encoding validation"),
        "S131A2" => ("S131A2", "SP800-131A Rev2 2019", "3; 5; 6", "https://csrc.nist.gov/pubs/sp/800/131/a/r2/final", "local reference thresholds; not current legal compliance"),
        "NIST-PQ-FAQ" => ("NIST-PQ-FAQ", "task056 pinned FAQ snapshot", "quantum public-key and generic-search models", "https://csrc.nist.gov/projects/post-quantum-cryptography/faqs", "not quantum arrival, probability or operational security bits"),
        "BHT1997" => ("BHT1997", "1997", "ideal quantum collision-query model", "https://arxiv.org/abs/quant-ph/9705002", "oracle queries only; circuit and memory cost excluded"),
        "R6151" => ("R6151", "RFC6151 2011", "2", "https://www.rfc-editor.org/rfc/rfc6151.html", "MD5 collision-sensitive use; not every hash use"),
        "R4055" => ("R4055", "RFC4055 2005", "1.2; 3", "https://www.rfc-editor.org/rfc/rfc4055.html", "PSS is signature-only; saved attributes do not verify parameters"),
        "F06" => ("F06", "RFC10024 August2026; RFC9954 July2026", "10024:4.1-4.3,7.1-7.4; 9954:3.1-3.3,6", "https://www.rfc-editor.org/rfc/rfc10024.html", "specified components/combiner; implementation unverified; obsolete Kyber IDs are separate"),
        "F07" => ("F07", "RFC9846 July2026", "4.2; 4.3.7; 4.3.8; 4.5.1-4.5.3", "https://www.rfc-editor.org/rfc/rfc9846.html", "collector-reported full session only; suite is not KEX; CertificateVerify scheme absent"),
        "F08" => ("F08", "RFC5280 2008; RFC4055 2005", "5280:4.1.1.2,4.1.2.5,4.1.2.7; 4055:1.2,3", "https://www.rfc-editor.org/rfc/rfc5280.html", "parsed subject/signature/time facts; no offline trust, revocation or encoding re-verification"),
        "F09" => ("F09", "RFC4253 2006", "7.1", "https://www.rfc-editor.org/rfc/rfc4253.html", "ordered unauthenticated offers; markers are not algorithms"),
        "F10" => ("F10", "RFC10042 August2026", "2.3.1-2.3.3; 2.4; 2.5; 5", "https://www.rfc-editor.org/rfc/rfc10042.html", "specified hybrid SSH KEX, not user authentication or verified combiner implementation"),
        "R8731" => ("R8731", "RFC8731 2020", "3; 4", "https://www.rfc-editor.org/rfc/rfc8731.html", "SSH curve25519/448 key establishment only"),
        "R8332" => ("R8332", "RFC8332 2018", "3", "https://www.rfc-editor.org/rfc/rfc8332.html", "RSA host signatures; no selected key size inference"),
        "F12" => ("F12", "task066 adopted contract 2026-11-18", "technical dispatch and ingestion invariants; FACT-SOURCE-MAPPING F12", "docs/architecture/MIGRATION-ASSESSMENT-V1.md", "saved claims preserve assurance; digest does not attest collector truth"),
        "F13" => ("F13", "task066 adopted time contract 2026-11-18", "time and policy semantics; FACT-SOURCE-MAPPING F13", "docs/architecture/MIGRATION-ASSESSMENT-V1.md", "scoped declared lifetimes/deadlines; no quantum forecast or universal deadline"),
        "F14" => ("F14", "migration-decision-v1 1.0.0; adopted 2026-11-18", "P1-P8; readiness and action precedence", "docs/architecture/MIGRATION-ASSESSMENT-V1.md", "local advisory policy; conformance is not practitioner usefulness"),
        "R8725" => ("R8725", "RFC8725 2020", "3.1; 3.3; 3.4; 3.8", "https://www.rfc-editor.org/rfc/rfc8725.html", "JWT algorithm, input and key binding prerequisites; no invented PQC JOSE identifier"),
        _ => return None,
    };
    Some(TechnicalSource {
        id,
        edition,
        clause,
        uri,
        scope_limit,
    })
}

/// Reuse pure primitive facts without changing local-assessment-1 semantics.
pub fn assess_primitive(
    identifier: &str,
    parameters: &BTreeMap<String, String>,
) -> AlgorithmAssessment {
    let mut a = assess_algorithm(identifier, parameters);
    if !parameters_consistent(identifier, parameters) {
        a.parameters_known = false;
        a.classical_concerns.push("contradictory_parameters".into());
    }
    // DSA/DH parameter capture is a new-dispatch gap; a family alone stays unknown.
    if matches!(a.identifier.as_str(), "DSA" | "DSA-SHA1" | "DH") {
        if let Some(bits) = parameters.get("key_bits").and_then(|s| decimal(s, 65536)) {
            a.parameters_known = parameters
                .get("parameter_interpretation")
                .is_none_or(|v| v != "not_available");
            if bits < 2048 {
                a.classical_concerns
                    .push("public_key_below_local_minimum".into());
            }
            a.source_ids.push("S131A2".into());
        }
    }
    if parameters
        .get("key_bits")
        .is_some_and(|s| decimal(s, 65536).is_none())
    {
        a.parameters_known = false;
    }
    if matches!(a.identifier.as_str(), "EC" | "ECDH" | "ECDSA")
        && parameters.get("named_curve_oid").is_some_and(|c| {
            !["1.2.840.10045.3.1.7", "1.3.132.0.34", "1.3.132.0.35"].contains(&c.as_str())
        })
    {
        a.parameters_known = false;
    }
    if a.standardization == "nist_final" && !a.parameters_known {
        a.quantum = QuantumExposure::Unknown;
    }
    a.source_ids.push("F12".into());
    a.source_ids.sort();
    a.source_ids.dedup();
    a
}
/// Compare only specified relationships; unknown encodings remain unknown.
pub(super) fn parameters_consistent(
    identifier: &str,
    parameters: &BTreeMap<String, String>,
) -> bool {
    let a = assess_algorithm(identifier, parameters);
    let Some(bits) = parameters.get("key_bits").and_then(|s| decimal(s, 65536)) else {
        return true;
    };
    let exact = match a.identifier.as_str() {
        "AES-128" => Some(128),
        "AES-192" => Some(192),
        "AES-256" => Some(256),
        "ES256" => Some(256),
        _ => None,
    };
    if exact.is_some_and(|expected| expected != bits) {
        return false;
    }
    if matches!(a.identifier.as_str(), "EC" | "ECDH" | "ECDSA") {
        let curve = match parameters.get("named_curve_oid").map(String::as_str) {
            Some("1.2.840.10045.3.1.7") => Some(256),
            Some("1.3.132.0.34") => Some(384),
            Some("1.3.132.0.35") => Some(521),
            _ => None,
        };
        if curve.is_some_and(|expected| expected != bits) {
            return false;
        }
    }
    true
}
fn decimal(value: &str, max: u32) -> Option<u32> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    value.parse::<u32>().ok().filter(|n| *n > 0 && *n <= max)
}

fn hybrid(
    id: &str,
    classical: &str,
    bits: Option<u32>,
    pq: &str,
    source_id: &str,
) -> AlgorithmAssessment {
    let p = bits
        .map(|n| BTreeMap::from([("key_bits".into(), n.to_string())]))
        .unwrap_or_default();
    let mut a = AlgorithmAssessment::unknown();
    a.identifier = id.into();
    a.construction = "hybrid".into();
    a.standardization = if source_id == "F10" {
        "rfc_informational"
    } else {
        "rfc_standard"
    }
    .into();
    a.purpose = Purpose::KeyEstablishment;
    a.components = vec![
        ComponentAssessment {
            role: "classical_component".into(),
            assessment: assess_primitive(classical, &p),
        },
        ComponentAssessment {
            role: "post_quantum_component".into(),
            assessment: assess_primitive(pq, &BTreeMap::new()),
        },
    ];
    a.parameters_known = true;
    // A specified combiner is not an executed/verified implementation.
    a.quantum = QuantumExposure::Unknown;
    a.combiner_assessment = Some("standard_specified_implementation_unverified".into());
    a.source_ids = vec![source_id.into(), "F12".into()];
    a
}

pub(super) fn tls_group(id: u64) -> (Option<&'static str>, &'static str, AlgorithmAssessment) {
    let empty = BTreeMap::new();
    let (name, class, mut a) = match id {
        4588 => (
            Some("X25519MLKEM768"),
            "hybrid",
            hybrid("X25519MLKEM768", "X25519", None, "ML-KEM-768", "F06"),
        ),
        4587 => (
            Some("SecP256r1MLKEM768"),
            "hybrid",
            hybrid("SecP256r1MLKEM768", "ECDH", Some(256), "ML-KEM-768", "F06"),
        ),
        4589 => (
            Some("SecP384r1MLKEM1024"),
            "hybrid",
            hybrid(
                "SecP384r1MLKEM1024",
                "ECDH",
                Some(384),
                "ML-KEM-1024",
                "F06",
            ),
        ),
        25497 => (
            Some("X25519Kyber768Draft00"),
            "hybrid",
            assess_primitive("X25519Kyber768Draft00", &empty),
        ),
        25498 => (
            Some("SecP256r1Kyber768Draft00"),
            "hybrid",
            hybrid(
                "SecP256r1Kyber768Draft00",
                "ECDH",
                Some(256),
                "Kyber768",
                "F06",
            ),
        ),
        29 => (
            Some("X25519"),
            "classical",
            assess_primitive("X25519", &empty),
        ),
        23 => (
            Some("secp256r1"),
            "classical",
            assess_primitive("ECDH", &BTreeMap::from([("key_bits".into(), "256".into())])),
        ),
        24 => (
            Some("secp384r1"),
            "classical",
            assess_primitive("ECDH", &BTreeMap::from([("key_bits".into(), "384".into())])),
        ),
        _ => (None, "unknown", AlgorithmAssessment::unknown()),
    };
    if matches!(id, 25497 | 25498) {
        a.standardization = "obsolete_pre_standard".into();
        a.parameters_known = false;
        a.combiner_assessment = Some("unverified_combiner_no_resistance_inference".into());
    }
    a.source_ids.extend(["F06".into(), "F07".into()]);
    a.source_ids.sort();
    a.source_ids.dedup();
    (name, class, a)
}

pub(super) fn ssh(name: &str, host_key: bool) -> AlgorithmAssessment {
    let empty = BTreeMap::new();
    let mut a = if host_key {
        match name {
            "ssh-ed25519" => assess_primitive("Ed25519", &empty),
            "rsa-sha2-256" => assess_primitive("RSA-SHA256", &empty),
            "rsa-sha2-512" => assess_primitive("RSA-SHA512", &empty),
            "ssh-rsa" => assess_primitive("RSA-SHA1", &empty),
            "ssh-dss" => assess_primitive("DSA-SHA1", &empty),
            "ecdsa-sha2-nistp256" => assess_primitive(
                "ECDSA",
                &BTreeMap::from([("key_bits".into(), "256".into())]),
            ),
            "ecdsa-sha2-nistp384" => assess_primitive(
                "ECDSA",
                &BTreeMap::from([("key_bits".into(), "384".into())]),
            ),
            "ecdsa-sha2-nistp521" => assess_primitive(
                "ECDSA",
                &BTreeMap::from([("key_bits".into(), "521".into())]),
            ),
            _ => AlgorithmAssessment::unknown(),
        }
    } else {
        match name {
            "mlkem768x25519-sha256" => hybrid(name, "X25519", None, "ML-KEM-768", "F10"),
            "mlkem768nistp256-sha256" => hybrid(name, "ECDH", Some(256), "ML-KEM-768", "F10"),
            "mlkem1024nistp384-sha384" => hybrid(name, "ECDH", Some(384), "ML-KEM-1024", "F10"),
            "curve25519-sha256" | "curve25519-sha256@libssh.org" => {
                assess_primitive("X25519", &empty)
            }
            "curve448-sha512" => {
                assess_primitive("ECDH", &BTreeMap::from([("key_bits".into(), "448".into())]))
            }
            "sntrup761x25519-sha512" | "sntrup761x25519-sha512@openssh.com" => {
                let mut a = hybrid(name, "X25519", None, "unknown", "F10");
                a.standardization = "vendor_defined".into();
                a.parameters_known = false;
                a.combiner_assessment = Some("unverified_combiner_no_resistance_inference".into());
                a
            }
            _ => AlgorithmAssessment::unknown(),
        }
    };
    a.source_ids.push("F09".into());
    if name.starts_with("curve") {
        a.source_ids.push("R8731".into());
    }
    if name.starts_with("rsa-sha2") {
        a.source_ids.push("R8332".into());
    }
    a.source_ids.sort();
    a.source_ids.dedup();
    a
}

pub(super) fn marker(name: &str) -> bool {
    matches!(
        name,
        "ext-info-c"
            | "ext-info-s"
            | "kex-strict-c-v00@openssh.com"
            | "kex-strict-s-v00@openssh.com"
    )
}
