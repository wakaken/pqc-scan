use super::{
    assess_primitive, bounded, schema, target, technical, Error, InputFormat, InputReference,
    ObservationFacts, RegistryFact, Result, SavedEvidence, SourceReference, Target,
};
use crate::{
    assessment::{AlgorithmAssessment, Purpose},
    target_policy::{ProbeHost, TargetRequest},
};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    net::{IpAddr, SocketAddr},
};

pub(super) fn project(value: Value, format: InputFormat, hash: String) -> Result<SavedEvidence> {
    use std::sync::OnceLock;
    static TLS: OnceLock<Result<jsonschema::Validator>> = OnceLock::new();
    static OFFERS: OnceLock<Result<jsonschema::Validator>> = OnceLock::new();
    static KEX: OnceLock<Result<jsonschema::Validator>> = OnceLock::new();
    let (body, cache) = match format {
        InputFormat::TlsV1 => (
            include_str!("../../../../schemas/probe-tls-v1.schema.json"),
            &TLS,
        ),
        InputFormat::SshOffersV1 => (
            include_str!("../../../../schemas/probe-ssh-offers-v1.schema.json"),
            &OFFERS,
        ),
        InputFormat::SshKexV1 => (
            include_str!("../../../../schemas/probe-ssh-kex-v1.schema.json"),
            &KEX,
        ),
        _ => return Err(Error::UnsupportedFormat),
    };
    // Detect contradictory success before shape rejection for a fixed error code.
    contradiction_preflight(&value, format)?;
    smaller_limits(&value, format)?;
    schema(&value, body, cache)?;
    envelope(&value)?;
    let collection_time = string(&value["generated_at"])?;
    bounded::collection_timestamp(collection_time)?;
    let input = InputReference {
        format,
        sha256: hash,
        schema_version: 1,
        collection_time: collection_time.into(),
        run_id: None,
        attempt_id: Some(1),
        protocol: Some(string(&value["protocol"])?.into()),
    };
    let targets = match format {
        InputFormat::TlsV1 => tls(&value, &input)?,
        InputFormat::SshOffersV1 => offers(&value, &input)?,
        InputFormat::SshKexV1 => kex(&value, &input)?,
        _ => return Err(Error::UnsupportedFormat),
    };
    Ok(SavedEvidence {
        input,
        targets,
        collection_outcome: string(&value["outcome"])?.into(),
        file: None,
        path: None,
    })
}

fn string(value: &Value) -> Result<&str> {
    value.as_str().ok_or(Error::InvalidInput)
}

fn smaller_limits(v: &Value, format: InputFormat) -> Result<()> {
    let mut limits = vec![("/diagnostics", 32)];
    if format == InputFormat::TlsV1 {
        limits.extend([
            ("/details/peer_certificates", 16),
            ("/details/client_profile/protocol_versions", 2),
            ("/details/client_profile/cipher_suites", 256),
            ("/details/client_profile/key_exchange_groups", 64),
        ]);
    }
    if limits.iter().any(|(path, limit)| {
        v.pointer(path)
            .and_then(Value::as_array)
            .is_some_and(|items| items.len() > *limit)
    }) {
        return Err(Error::ResourceLimit);
    }
    if format == InputFormat::SshOffersV1 {
        for offers in [
            &v["details"]["offers"],
            &v["details"]["client_profile"]["offers"],
        ] {
            if let Some(lists) = offers.as_object() {
                for list in lists.values().filter_map(Value::as_array) {
                    if list.len() > 256
                        || list.iter().filter_map(Value::as_str).any(|s| s.len() > 64)
                    {
                        return Err(Error::ResourceLimit);
                    }
                }
            }
        }
    }
    Ok(())
}
fn contradiction_preflight(v: &Value, format: InputFormat) -> Result<()> {
    let d = &v["details"];
    let contradiction = match format {
        InputFormat::TlsV1 => {
            (!d["negotiated"].is_null() || d["handshake_complete"] == true)
                && (v["outcome"] == "failed"
                    || d["handshake_complete"] != true
                    || d["verification"]["certificate_chain_and_name"] != "verified"
                    || !matches!(
                        d["handshake_kind"].as_str(),
                        Some("full" | "full_with_hello_retry_request")
                    ))
        }
        InputFormat::SshKexV1 => {
            (!d["negotiated_kex"].is_null()
                || !d["host_signature"].is_null()
                || d["handshake_complete"] == true
                || d["host_identity_verified"] == true)
                && (v["outcome"] != "completed"
                    || d["handshake_complete"] != true
                    || d["host_identity_verified"] != true
                    || d["user_authentication_attempted"] != false
                    || d["host_authentication_pqc"] != false)
        }
        _ => false,
    };
    if contradiction {
        return Err(Error::ContradictorySavedFacts);
    }
    Ok(())
}

fn envelope(v: &Value) -> Result<()> {
    let t = &v["target"];
    let port = t["port"].as_u64().ok_or(Error::InvalidInput)? as u16;
    TargetRequest::new(string(&t["host"])?, port).map_err(|_| Error::InvalidInput)?;
    let answers = if let Some(values) = t["resolved_addresses"].as_array() {
        values
            .iter()
            .map(|v| {
                string(v)?
                    .parse::<IpAddr>()
                    .map_err(|_| Error::InvalidInput)
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        vec![]
    };
    if answers.len()
        > v["limits"]["max_dns_answers"]
            .as_u64()
            .ok_or(Error::InvalidInput)? as usize
    {
        return Err(Error::ContradictorySavedFacts);
    }
    let socket = |v: &Value| -> Result<Option<SocketAddr>> {
        if v.is_null() {
            Ok(None)
        } else {
            string(v)?
                .parse()
                .map(Some)
                .map_err(|_| Error::InvalidInput)
        }
    };
    let pinned = socket(&t["pinned_address"])?;
    let peer = socket(&t["connected_peer"])?;
    if pinned.is_some_and(|p| p.port() != port || !answers.contains(&p.ip()))
        || peer.is_some_and(|p| Some(p) != pinned)
        || (v["outcome"] == "completed" && peer.is_none())
        || ((v["outcome"] == "completed")
            != (v["diagnostics"].as_array().is_some_and(Vec::is_empty)))
    {
        return Err(Error::ContradictorySavedFacts);
    }
    for (traffic, cap) in [
        ("received_bytes", "max_received_bytes"),
        ("sent_bytes", "max_sent_bytes"),
    ] {
        let count = v["traffic"][traffic].as_u64().ok_or(Error::InvalidInput)?;
        let limit = v["limits"][cap].as_u64().ok_or(Error::InvalidInput)?;
        if count > limit || (count != 0 && peer.is_none()) {
            return Err(Error::ContradictorySavedFacts);
        }
    }
    Ok(())
}

const VERSIONS: &[(u16, &str)] = &[(771, "TLSv1_2"), (772, "TLSv1_3")];
const SUITES: &[(u16, &str)] = &[
    (4865, "TLS13_AES_128_GCM_SHA256"),
    (4866, "TLS13_AES_256_GCM_SHA384"),
    (4867, "TLS13_CHACHA20_POLY1305_SHA256"),
    (0xc02b, "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256"),
    (0xc02c, "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"),
    (0xc02f, "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"),
    (0xc030, "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384"),
    (0xcca8, "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256"),
    (0xcca9, "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256"),
];
fn registry(v: &Value, table: &[(u16, &str)]) -> Result<RegistryFact> {
    let id = v["id"]
        .as_u64()
        .filter(|n| *n <= 65535)
        .ok_or(Error::InvalidInput)? as u16;
    let name = string(&v["name"])?;
    let expected = table.iter().find(|(n, _)| *n == id).map(|(_, name)| *name);
    if expected.is_some_and(|expected| name != expected)
        || table.iter().any(|(n, known)| *known == name && *n != id)
    {
        return Err(Error::ContradictorySavedFacts);
    }
    Ok(RegistryFact {
        id,
        name: expected.unwrap_or("unknown").into(),
    })
}
fn group(v: &Value) -> Result<(RegistryFact, &'static str, AlgorithmAssessment)> {
    let id = v["id"]
        .as_u64()
        .filter(|n| *n <= 65535)
        .ok_or(Error::InvalidInput)?;
    let name = string(&v["name"])?;
    let (expected, class, algorithm) = technical::tls_group(id);
    if expected.is_some_and(|s| s != name)
        || [23, 24, 29, 4587, 4588, 4589, 25497, 25498]
            .iter()
            .any(|n| *n != id && technical::tls_group(*n).0 == Some(name))
    {
        return Err(Error::ContradictorySavedFacts);
    }
    Ok((
        RegistryFact {
            id: id as u16,
            name: expected.unwrap_or("unknown").into(),
        },
        class,
        algorithm,
    ))
}

fn tls(v: &Value, input: &InputReference) -> Result<Vec<Target>> {
    let d = &v["details"];
    let profile = &d["client_profile"];
    let verification = &d["verification"];
    let identity = TargetRequest::new(string(&verification["verification_name"])?, 1)
        .map_err(|_| Error::InvalidInput)?;
    let normalized = crate::probe::host_name(identity.host());
    let expected_sni = if matches!(identity.host(), ProbeHost::Dns(_)) {
        Some(normalized.as_str())
    } else {
        None
    };
    if verification["verification_name"] != normalized
        || verification["sent_sni"].as_str() != expected_sni
    {
        return Err(Error::ContradictorySavedFacts);
    }
    if (verification["trust_source"] == "bundled_mozilla")
        != (verification["trust_bundle_version"] == "1.0.11")
    {
        return Err(Error::ContradictorySavedFacts);
    }

    for item in profile["protocol_versions"]
        .as_array()
        .ok_or(Error::InvalidInput)?
    {
        registry(item, VERSIONS)?;
    }
    for item in profile["cipher_suites"]
        .as_array()
        .ok_or(Error::InvalidInput)?
    {
        registry(item, SUITES)?;
    }
    for item in profile["key_exchange_groups"]
        .as_array()
        .ok_or(Error::InvalidInput)?
    {
        group(item)?;
    }
    let verified = d["handshake_complete"] == true
        && d["verification"]["certificate_chain_and_name"] == "verified"
        && matches!(
            d["handshake_kind"].as_str(),
            Some("full" | "full_with_hello_retry_request")
        )
        && !d["negotiated"].is_null()
        && v["target"]["connected_peer"].is_string()
        && matches!(v["outcome"].as_str(), Some("completed" | "partial"));
    if (d["handshake_complete"] == true
        || !d["negotiated"].is_null()
        || d["verification"]["certificate_chain_and_name"] == "verified")
        && !verified
    {
        return Err(Error::ContradictorySavedFacts);
    }
    if verified
        && v["diagnostics"]
            .as_array()
            .ok_or(Error::InvalidInput)?
            .iter()
            .any(|d| {
                !matches!(
                    d.as_str(),
                    Some(
                        "group_unavailable"
                            | "certificate_parse_incomplete"
                            | "certificate_collection_limit"
                    )
                )
            })
    {
        return Err(Error::ContradictorySavedFacts);
    }
    let mut reference = SourceReference {
        attempt_id: Some(1),
        role: "negotiated_key_establishment".into(),
        detail_path: Some("/details/negotiated/key_exchange_group".into()),
        ..Default::default()
    };
    let mut algorithm = AlgorithmAssessment::unknown();
    if verified {
        let n = &d["negotiated"];
        let version = registry(&n["version"], VERSIONS)?;
        let suite = registry(&n["cipher_suite"], SUITES)?;
        if (version.id == 772
            && SUITES.iter().any(|(id, _)| *id == suite.id)
            && !(4865..=4867).contains(&suite.id))
            || (version.id == 771 && (4865..=4867).contains(&suite.id))
        {
            return Err(Error::ContradictorySavedFacts);
        }
        for (selected, offered) in [
            ("version", "protocol_versions"),
            ("cipher_suite", "cipher_suites"),
            ("key_exchange_group", "key_exchange_groups"),
        ] {
            if !n[selected].is_null()
                && !profile[offered]
                    .as_array()
                    .ok_or(Error::InvalidInput)?
                    .contains(&n[selected])
            {
                return Err(Error::ContradictorySavedFacts);
            }
        }
        reference.protocol_version = Some(version);
        reference.cipher_suite = Some(suite);
        if !n["key_exchange_group"].is_null() {
            let (fact, class, a) = group(&n["key_exchange_group"])?;
            // Original collectors classified obsolete IDs as unknown. Preserve
            // that saved vocabulary while the new mapping identifies the draft.
            let legacy_draft_unknown =
                matches!(fact.id, 25497 | 25498) && n["group_classification"] == "unknown";
            if n["group_classification"] != class && !legacy_draft_unknown {
                return Err(Error::ContradictorySavedFacts);
            }
            if reference
                .protocol_version
                .as_ref()
                .is_some_and(|v| v.id != 772)
                && class == "hybrid"
            {
                return Err(Error::ContradictorySavedFacts);
            }
            reference.key_exchange_group = Some(fact);
            algorithm = a;
        } else if n["group_classification"] != "unknown" {
            return Err(Error::ContradictorySavedFacts);
        }
    }
    let observation = ObservationFacts {
        method: "tls_saved_observation".into(),
        assurance: if verified {
            "collector_reported_verified_session"
        } else {
            "unverified_or_incomplete"
        }
        .into(),
        usage: if verified { "negotiated" } else { "unknown" }.into(),
        collection_outcome: string(&v["outcome"])?.into(),
    };
    let mut targets = vec![target(
        input,
        reference,
        algorithm,
        observation.clone(),
        "unknown",
        Some("F07"),
        true,
    )];
    let certificates = d["peer_certificates"]
        .as_array()
        .ok_or(Error::InvalidInput)?;
    let collection = &d["certificate_collection"];
    let count = certificates.len() as u64;
    if collection["collected_count"] != count
        || collection["presented_count"]
            .as_u64()
            .is_some_and(|n| n < count)
        || (collection["status"] == "complete"
            && (collection["presented_count"] != count || count == 0))
        || (collection["status"] == "not_available" && count != 0)
        || (collection["status"] == "complete"
            && certificates
                .iter()
                .any(|c| c["attribute_status"] != "complete"))
    {
        return Err(Error::ContradictorySavedFacts);
    }
    for (i, cert) in certificates.iter().enumerate() {
        for name in ["not_before_unix", "not_after_unix"] {
            if !cert[name].is_null() && cert[name].as_i64().is_none() {
                return Err(Error::InvalidInput);
            }
        }
        if !cert["key_bits"].is_null() && cert["key_bits"].as_u64().is_none() {
            return Err(Error::InvalidInput);
        }
        if cert["attribute_status"] == "complete"
            && [
                "signature_oid",
                "public_key_oid",
                "spki_sha256",
                "not_before_unix",
                "not_after_unix",
            ]
            .iter()
            .any(|k| cert[k].is_null())
        {
            return Err(Error::ContradictorySavedFacts);
        }
        let start = cert["not_before_unix"].as_i64();
        let end = cert["not_after_unix"].as_i64();
        if start.zip(end).is_some_and(|(s, e)| s > e) {
            return Err(Error::ContradictorySavedFacts);
        }
        if cert["attribute_status"] == "unsupported_or_malformed"
            && [
                "signature_oid",
                "public_key_oid",
                "spki_sha256",
                "key_bits",
                "named_curve_oid",
                "not_before_unix",
                "not_after_unix",
            ]
            .iter()
            .any(|k| !cert[k].is_null())
        {
            return Err(Error::ContradictorySavedFacts);
        }
        for (role, field) in [
            ("certificate_signature", "signature_oid"),
            ("certificate_subject_key", "public_key_oid"),
        ] {
            let mut parameters = BTreeMap::new();
            if field == "public_key_oid" {
                if let Some(bits) = cert["key_bits"].as_u64() {
                    parameters.insert("key_bits".into(), bits.to_string());
                }
                if let Some(curve) = cert["named_curve_oid"].as_str() {
                    parameters.insert("named_curve_oid".into(), curve.into());
                }
            }
            if !technical::parameters_consistent(cert[field].as_str().unwrap_or(""), &parameters) {
                return Err(Error::ContradictorySavedFacts);
            }
            let mut a = assess_primitive(cert[field].as_str().unwrap_or(""), &parameters);
            if field == "signature_oid" && matches!(a.identifier.as_str(), "RSA" | "EC") {
                a.purpose = Purpose::Signature;
            }
            if field == "signature_oid"
                && !matches!(a.purpose, Purpose::Signature | Purpose::Unknown)
            {
                a.classical_concerns
                    .push("certificate_signature_purpose_mismatch".into());
            }
            a.source_ids.push("F08".into());
            let reference = SourceReference {
                attempt_id: Some(1),
                role: role.into(),
                detail_path: Some(format!("/details/peer_certificates/{i}/{field}")),
                certificate_sha256: cert["certificate_der_sha256"].as_str().map(str::to_owned),
                spki_sha256: cert["spki_sha256"].as_str().map(str::to_owned),
                not_before_unix: start,
                not_after_unix: end,
                ..Default::default()
            };
            let mut facts = observation.clone();
            facts.method = "certificate_parse".into();
            facts.assurance = if cert["attribute_status"] == "unsupported_or_malformed" {
                "unknown"
            } else {
                "parsed"
            }
            .into();
            facts.usage = "unknown".into();
            if cert["attribute_status"] != "complete" {
                facts.collection_outcome = "partial".into();
            }
            targets.push(target(
                input,
                reference,
                a,
                facts,
                "unknown",
                Some("F08"),
                true,
            ));
        }
    }
    Ok(targets)
}

fn offers(v: &Value, input: &InputReference) -> Result<Vec<Target>> {
    let d = &v["details"];
    let present = !d["offers"].is_null();
    if present != (d["assurance"] == "unauthenticated_advertisement")
        || (present && v["target"]["connected_peer"].is_null())
    {
        return Err(Error::ContradictorySavedFacts);
    }
    let observation = ObservationFacts {
        method: "ssh_saved_offers".into(),
        assurance: if present {
            "unauthenticated_advertisement"
        } else {
            "unverified_or_incomplete"
        }
        .into(),
        usage: "unknown".into(),
        collection_outcome: string(&v["outcome"])?.into(),
    };
    let mut targets = vec![];
    if present {
        for (field, host) in [
            ("kex_algorithms", false),
            ("server_host_key_algorithms", true),
        ] {
            for (i, name) in d["offers"][field]
                .as_array()
                .ok_or(Error::InvalidInput)?
                .iter()
                .enumerate()
            {
                let name = string(name)?;
                if technical::marker(name) {
                    continue;
                }
                let reference = SourceReference {
                    attempt_id: Some(1),
                    role: if host {
                        "advertised_host_signature"
                    } else {
                        "advertised_key_establishment"
                    }
                    .into(),
                    detail_path: Some(format!("/details/offers/{field}/{i}")),
                    ..Default::default()
                };
                targets.push(target(
                    input,
                    reference,
                    technical::ssh(name, host),
                    observation.clone(),
                    "unknown",
                    Some("F09"),
                    true,
                ));
            }
        }
    }
    if targets.is_empty() {
        targets.push(target(
            input,
            SourceReference {
                attempt_id: Some(1),
                role: "reference-only".into(),
                detail_path: Some("/details/offers".into()),
                ..Default::default()
            },
            AlgorithmAssessment::unknown(),
            observation,
            "unknown",
            Some("F09"),
            true,
        ));
    }
    Ok(targets)
}

fn kex(v: &Value, input: &InputReference) -> Result<Vec<Target>> {
    let completed = v["outcome"] == "completed";
    let observation = ObservationFacts {
        method: "ssh_saved_kex".into(),
        assurance: if completed {
            "collector_reported_verified_session"
        } else {
            "unverified_or_incomplete"
        }
        .into(),
        usage: if completed { "negotiated" } else { "unknown" }.into(),
        collection_outcome: string(&v["outcome"])?.into(),
    };
    let mut targets = vec![];
    for (field, role, host) in [
        ("negotiated_kex", "negotiated_key_establishment", false),
        ("host_signature", "host_signature", true),
    ] {
        let a = if completed {
            technical::ssh(string(&v["details"][field])?, host)
        } else {
            AlgorithmAssessment::unknown()
        };
        let reference = SourceReference {
            attempt_id: Some(1),
            role: role.into(),
            detail_path: Some(format!("/details/{field}")),
            ..Default::default()
        };
        targets.push(target(
            input,
            reference,
            a,
            observation.clone(),
            "unknown",
            Some(if host { "F09" } else { "F10" }),
            true,
        ));
    }
    Ok(targets)
}
