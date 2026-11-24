#[path = "support/migration/mod.rs"]
mod fixture;
use fixture::*;
use pqc_scan_core::{
    assessment::{Purpose, QuantumExposure},
    identity::native_id,
    migration::{
        assess_primitive, source, Error, InputFormat, SavedEvidence, CATALOG_ID, CATALOG_VERSION,
    },
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[test]
fn a01_a04_native_reference_assurance_and_exact_primitive_are_separate() {
    for (id, exposure, purpose) in [
        ("RSA", QuantumExposure::Vulnerable, Purpose::Unknown),
        (
            "ML-KEM-768",
            QuantumExposure::DesignedResistant,
            Purpose::KeyEstablishment,
        ),
        (
            "ML-DSA-65",
            QuantumExposure::DesignedResistant,
            Purpose::Signature,
        ),
    ] {
        let result = ingest(&inventory(id), InputFormat::InventoryV1).unwrap();
        let t = &result.targets()[0];
        assert_eq!(t.technical.algorithm.quantum, exposure);
        assert_eq!(t.technical.algorithm.purpose, purpose);
        assert_eq!(t.observation.assurance, "syntactic");
        assert_eq!(t.observation.usage, "referenced");
        assert_eq!(t.observation.collection_outcome, "complete");
        assert_eq!(t.source_ref.path.as_deref(), Some("app.txt"));
        assert_eq!(
            t.source_ref.observation_id.as_deref(),
            Some(native_id("observation", &["fixture-1"]).as_str())
        );
    }
}
#[test]
fn a04_exact_drafts_namespaces_prehash_and_malformed_parameters_do_not_alias() {
    for (id, ns, standard, purpose) in [
        ("Kyber768", "draft", "draft", Purpose::KeyEstablishment),
        ("ML-KEM-768", "draft", "unknown", Purpose::Unknown),
        (
            "2.16.840.1.101.3.4.3.17",
            "oid",
            "nist_final",
            Purpose::Signature,
        ),
        (
            "2.16.840.1.101.3.4.3.32",
            "oid",
            "unknown",
            Purpose::Unknown,
        ),
    ] {
        let mut v = inventory(id);
        v["assets"][0]["attributes"]["namespace"] = ns.into();
        let r = ingest(&v, InputFormat::InventoryV1).unwrap();
        assert_eq!(r.targets()[0].technical.algorithm.standardization, standard);
        assert_eq!(r.targets()[0].technical.algorithm.purpose, purpose);
    }
    for bits in ["not-an-integer", "-1", "+2048", "65537", "0", "2.5"] {
        let a = assess_primitive("RSA", &BTreeMap::from([("key_bits".into(), bits.into())]));
        assert!(!a.parameters_known, "{bits}");
        assert_eq!(a.quantum, QuantumExposure::Vulnerable);
    }
    for kind in ["null", "sequence", "unknown"] {
        let a = assess_primitive(
            "2.16.840.1.101.3.4.3.17",
            &BTreeMap::from([("parameters_kind".into(), kind.into())]),
        );
        assert!(!a.parameters_known);
        assert_eq!(a.quantum, QuantumExposure::Unknown);
    }
}
#[test]
fn technical_axes_classical_parameters_kem_signatures_and_hash_models_remain_distinct() {
    for id in ["RSA", "DH", "DSA", "ECDH", "ECDSA"] {
        let weak = assess_primitive(id, &BTreeMap::from([("key_bits".into(), "128".into())]));
        assert_eq!(weak.quantum, QuantumExposure::Vulnerable);
        assert!(!weak.classical_concerns.is_empty());
        assert!(!assess_primitive(id, &BTreeMap::new()).parameters_known);
    }
    assert_eq!(
        assess_primitive("RSA-PSS", &BTreeMap::new()).purpose,
        Purpose::Signature
    );
    for (id, purpose) in [
        ("AES-128", Purpose::Encryption),
        ("SHA-256", Purpose::Hash),
        ("ML-KEM-512", Purpose::KeyEstablishment),
        ("SLH-DSA-SHAKE-256f", Purpose::Signature),
    ] {
        let a = assess_primitive(id, &BTreeMap::new());
        assert_eq!(a.purpose, purpose);
        assert!(a.classical_concerns.is_empty());
    }
    let a = assess_primitive("SHA-256", &BTreeMap::new());
    assert_eq!(a.attack_models.len(), 2);
    assert_eq!(a.attack_models[0].exponent_denominator, 2);
    assert_eq!(a.attack_models[1].exponent_denominator, 3);
}
#[test]
fn a06_dependency_declaration_resolution_never_selects_a_primitive_or_provider() {
    for provenance in ["manifest_declared", "lock_resolved", "source_unresolved"] {
        let mut v = inventory("RSA");
        v["assets"][0]["kind"] = "dependency".into();
        v["assets"][0]["attributes"] = json!({"name":"rustls","ecosystem":"cargo","reported_version":"0.23.46","declared_constraint":"0.23","resolved_version":"0.23.46","version_provenance":provenance});
        v["observations"][0]["method"] = "dependency_report".into();
        v["observations"][0]["assurance"] = "parsed".into();
        let r = ingest(&v, InputFormat::InventoryV1).unwrap();
        let t = &r.targets()[0];
        assert_eq!(
            t.source_ref.dependency_provenance.as_deref(),
            Some(provenance)
        );
        assert_eq!(t.technical.algorithm.identifier, "unknown");
        assert_eq!(t.observation.assurance, "parsed");
    }
}
#[test]
fn a05_a10_verified_tls_has_separate_kex_certificate_roles_and_no_reverification() {
    let r = ingest(&tls(), InputFormat::TlsV1).unwrap();
    assert_eq!(r.targets().len(), 3);
    let kx = &r.targets()[0];
    assert_eq!(
        kx.observation.assurance,
        "collector_reported_verified_session"
    );
    assert_eq!(
        kx.technical
            .algorithm
            .components
            .iter()
            .map(|c| c.assessment.identifier.as_str())
            .collect::<Vec<_>>(),
        ["X25519", "ML-KEM-768"]
    );
    assert_eq!(kx.technical.algorithm.quantum, QuantumExposure::Unknown);
    assert_eq!(
        kx.technical.algorithm.combiner_assessment.as_deref(),
        Some("standard_specified_implementation_unverified")
    );
    assert_eq!(kx.source_ref.protocol_version.as_ref().unwrap().id, 772);
    for cert in &r.targets()[1..] {
        assert_eq!(
            cert.technical.algorithm.quantum,
            QuantumExposure::Vulnerable
        );
        assert_eq!(cert.observation.assurance, "parsed");
        assert_eq!(cert.technical.encoding_assurance, "unknown");
    }
    assert_eq!(
        r.targets()[1].technical.algorithm.purpose,
        Purpose::Signature
    );
    assert_ne!(r.targets()[1].id, r.targets()[2].id);
    assert_eq!(
        r.targets()[1].source_ref.certificate_sha256.as_deref(),
        Some("b".repeat(64).as_str())
    );
}
#[test]
fn a05_all_final_draft_and_unknown_groups_preserve_exact_components() {
    for (id, name, class, standard, components) in [
        (4587, "SecP256r1MLKEM768", "hybrid", "rfc_standard", 2),
        (4589, "SecP384r1MLKEM1024", "hybrid", "rfc_standard", 2),
        (
            25497,
            "X25519Kyber768Draft00",
            "unknown",
            "obsolete_pre_standard",
            2,
        ),
        (
            25498,
            "SecP256r1Kyber768Draft00",
            "unknown",
            "obsolete_pre_standard",
            2,
        ),
        (65535, "unknown", "unknown", "unknown", 0),
        (29, "X25519", "classical", "classical_standard", 0),
    ] {
        let mut v = tls();
        let group = json!({"id":id,"name":name});
        v["details"]["negotiated"]["key_exchange_group"] = group.clone();
        v["details"]["client_profile"]["key_exchange_groups"] = json!([group]);
        v["details"]["negotiated"]["group_classification"] = class.into();
        let r = ingest(&v, InputFormat::TlsV1).unwrap();
        let a = &r.targets()[0].technical.algorithm;
        assert_eq!(a.standardization, standard);
        assert_eq!(a.components.len(), components);
        if id == 4589 {
            assert_eq!(a.components[1].assessment.identifier, "ML-KEM-1024");
        }
    }
}
#[test]
fn a11_partial_certificate_and_missing_group_keep_independently_verified_facts() {
    let mut v = tls();
    v["outcome"] = "partial".into();
    v["diagnostics"] = json!(["certificate_parse_incomplete"]);
    v["details"]["certificate_collection"]["status"] = "partial".into();
    v["details"]["peer_certificates"][0]["attribute_status"] = "partial".into();
    let r = ingest(&v, InputFormat::TlsV1).unwrap();
    assert_eq!(
        r.targets()[0]
            .source_ref
            .key_exchange_group
            .as_ref()
            .unwrap()
            .id,
        4588
    );
    assert_eq!(
        r.targets()[0].observation.assurance,
        "collector_reported_verified_session"
    );
    assert_eq!(r.targets()[0].observation.collection_outcome, "partial");
    v["details"]["negotiated"]["key_exchange_group"] = Value::Null;
    v["details"]["negotiated"]["group_classification"] = "unknown".into();
    v["diagnostics"] = json!(["group_unavailable", "certificate_parse_incomplete"]);
    let r = ingest(&v, InputFormat::TlsV1).unwrap();
    assert_eq!(r.targets()[0].technical.algorithm.identifier, "unknown");
    assert!(r.targets()[0].source_ref.protocol_version.is_some());
    assert!(r.targets()[0].source_ref.cipher_suite.is_some());
    assert_eq!(
        r.targets()[0].observation.assurance,
        "collector_reported_verified_session"
    );
}
#[test]
fn a11_failed_tls_keeps_safe_classical_concerns_without_negotiation() {
    let mut v = failed_tls();
    v["details"]["peer_certificates"][0]["signature_oid"] = "1.2.840.113549.1.1.4".into();
    let r = ingest(&v, InputFormat::TlsV1).unwrap();
    assert_eq!(
        r.targets()[0].observation.assurance,
        "unverified_or_incomplete"
    );
    assert_eq!(r.targets()[0].technical.algorithm.identifier, "unknown");
    assert!(r.targets()[1]
        .technical
        .algorithm
        .classical_concerns
        .contains(&"signature_uses_weak_collision_digest".into()));
    v["outcome"] = "partial".into();
    v["details"]["verification"]["certificate_chain_and_name"] = "not_completed".into();
    assert!(ingest(&v, InputFormat::TlsV1).is_ok());
}
#[test]
fn a05_a11_contradictory_tls_verification_binding_id_name_and_suite_reject() {
    let base = tls();
    for (pointer, replacement) in [
        (
            "/details/verification/certificate_chain_and_name",
            json!("failed"),
        ),
        ("/details/handshake_complete", json!(false)),
        (
            "/details/negotiated/key_exchange_group/name",
            json!("secp256r1"),
        ),
        (
            "/details/negotiated/group_classification",
            json!("classical"),
        ),
        ("/target/connected_peer", json!("192.0.2.2:443")),
        ("/details/certificate_collection/collected_count", json!(0)),
        ("/details/negotiated/version/id", json!(771)),
    ] {
        let mut v = base.clone();
        *v.pointer_mut(pointer).unwrap() = replacement;
        assert!(ingest(&v, InputFormat::TlsV1).is_err(), "{pointer}");
    }
    let mut v = failed_tls();
    v["details"]["negotiated"] = base["details"]["negotiated"].clone();
    assert_eq!(
        ingest(&v, InputFormat::TlsV1).unwrap_err(),
        Error::ContradictorySavedFacts
    );
}
#[test]
fn a16_ssh_offers_keep_order_skip_markers_and_ignore_stored_classifications() {
    let mut v = offers();
    v["details"]["kex_candidates"] = json!([{ "name":"curve25519-sha256","construction":"hybrid","namespace":"unqualified","provenance":"rfc_defined","reference":null }]);
    let r = ingest(&v, InputFormat::SshOffersV1).unwrap();
    assert_eq!(r.targets().len(), 3);
    assert_eq!(
        r.targets()[0].technical.algorithm.identifier,
        "mlkem768x25519-sha256"
    );
    assert_eq!(r.targets()[1].technical.algorithm.identifier, "X25519");
    for t in r.targets() {
        assert_eq!(t.observation.assurance, "unauthenticated_advertisement");
        assert_eq!(t.observation.usage, "unknown");
    }
    assert!(r
        .targets()
        .iter()
        .all(|t| t.technical.algorithm.identifier != "ext-info-s"));
}
#[test]
fn a16_ssh_kex_requires_pinned_success_and_separates_host_from_user_authentication() {
    let base = kex();
    let r = ingest(&base, InputFormat::SshKexV1).unwrap();
    assert_eq!(r.targets().len(), 2);
    assert_eq!(r.targets()[0].technical.algorithm.components.len(), 2);
    assert_eq!(r.targets()[1].technical.algorithm.identifier, "Ed25519");
    assert_eq!(
        r.targets()[1].technical.algorithm.quantum,
        QuantumExposure::Vulnerable
    );
    for field in [
        "host_identity_verified",
        "handshake_complete",
        "user_authentication_attempted",
        "host_authentication_pqc",
    ] {
        let mut v = base.clone();
        v["details"][field] = (!base["details"][field].as_bool().unwrap()).into();
        assert!(ingest(&v, InputFormat::SshKexV1).is_err(), "{field}");
    }
    let mut v = base;
    v["outcome"] = "failed".into();
    v["diagnostics"] = json!(["host_key_untrusted"]);
    for field in ["host_identity_verified", "handshake_complete"] {
        v["details"][field] = false.into();
    }
    for field in ["negotiated_kex", "host_signature"] {
        v["details"][field] = Value::Null;
    }
    let r = ingest(&v, InputFormat::SshKexV1).unwrap();
    assert!(r
        .targets()
        .iter()
        .all(|t| t.technical.algorithm.identifier == "unknown"));
}
#[test]
fn content_hash_ids_and_versions_are_explicit_and_deterministic() {
    let v = inventory("RSA");
    let compact = serde_json::to_vec(&v).unwrap();
    let pretty = serde_json::to_vec_pretty(&v).unwrap();
    let a = SavedEvidence::from_bytes(&compact, InputFormat::InventoryV1).unwrap();
    let b = SavedEvidence::from_bytes(&compact, InputFormat::InventoryV1).unwrap();
    let c = SavedEvidence::from_bytes(&pretty, InputFormat::InventoryV1).unwrap();
    assert_eq!(a.targets()[0].id, b.targets()[0].id);
    assert_ne!(a.targets()[0].id, c.targets()[0].id);
    assert_ne!(a.input().sha256, c.input().sha256);
    assert_eq!(
        a.targets()[0].source_ref.observation_id,
        c.targets()[0].source_ref.observation_id
    );
    assert_eq!(
        SavedEvidence::from_bytes_with_catalog(
            &compact,
            InputFormat::InventoryV1,
            "local-assessment-1",
            CATALOG_VERSION
        )
        .unwrap_err(),
        Error::UnsupportedVersion
    );
    assert_eq!(
        SavedEvidence::from_bytes_with_catalog(
            &compact,
            InputFormat::InventoryV1,
            CATALOG_ID,
            "2.0.0"
        )
        .unwrap_err(),
        Error::UnsupportedVersion
    );
    assert_eq!(
        "unknown".parse::<InputFormat>().unwrap_err(),
        Error::UnsupportedFormat
    );
}
#[test]
fn native_fixed_role_links_do_not_follow_arbitrary_cycles_and_bad_references_reject() {
    let mut v = inventory("RSA");
    let id = v["assets"][0]["id"].clone();
    v["assets"][0]["relations"] = json!([{"relationship":"future-cycle","target":id}]);
    assert!(ingest(&v, InputFormat::InventoryV1).is_ok());
    v["assets"][0]["relations"][0]["target"] = native_id("asset", &["missing"]).into();
    assert_eq!(
        ingest(&v, InputFormat::InventoryV1).unwrap_err(),
        Error::InvalidReference
    );
    let mut v = inventory("RSA");
    v["observations"][0]["asset_ids"] = json!([native_id("asset", &["missing"])]);
    assert_eq!(
        ingest(&v, InputFormat::InventoryV1).unwrap_err(),
        Error::InvalidReference
    );
}
#[test]
fn safe_extensions_and_unknown_vocabulary_never_leak_raw_input() {
    let mut v = inventory("SECRET_CANARY_067");
    v["SECRET_CANARY_067"] = json!({"raw":"SECRET_CANARY_067"});
    v["assets"][0]["attributes"]["SECRET_CANARY_067"] = "SECRET_CANARY_067".into();
    v["observations"][0]["assurance"] = "SECRET_CANARY_067".into();
    let r = ingest(&v, InputFormat::InventoryV1).unwrap();
    let out = serde_json::to_string(r.targets()).unwrap();
    assert!(!out.contains("CANARY"));
    assert_eq!(r.targets()[0].observation.assurance, "unknown");
    for (mut v, format) in [
        (tls(), InputFormat::TlsV1),
        (offers(), InputFormat::SshOffersV1),
        (kex(), InputFormat::SshKexV1),
    ] {
        v["details"]["SECRET_CANARY_067"] = "SECRET_CANARY_067".into();
        let error = ingest(&v, format).unwrap_err();
        assert!(!error.to_string().contains("CANARY"));
    }
}
#[test]
fn bounded_json_rejects_duplicates_utf8_bom_trailing_null_and_oversized_values() {
    for bytes in [
        b"{\"schema_version\":1,\"schema_version\":1}".as_slice(),
        b"{\"x\":{\"a\":1,\"a\":2}}",
        b"\xef\xbb\xbf{}",
        b"{} {}",
        b"\xff",
        b"null",
    ] {
        assert!(SavedEvidence::from_bytes(bytes, InputFormat::TlsV1).is_err());
    }
    let mut v = tls();
    v["details"]["peer_certificates"][0]["signature_oid"] = Value::Null;
    v["details"]["peer_certificates"][0]["attribute_status"] = "partial".into();
    v["details"]["certificate_collection"]["status"] = "partial".into();
    v["outcome"] = "partial".into();
    v["diagnostics"] = json!(["certificate_parse_incomplete"]);
    assert!(ingest(&v, InputFormat::TlsV1).is_ok());
    v["details"]["handshake_complete"] = Value::Null;
    assert!(ingest(&v, InputFormat::TlsV1).is_err());
    let mut v = tls();
    v["schema_version"] = 2.into();
    assert_eq!(
        ingest(&v, InputFormat::TlsV1).unwrap_err(),
        Error::UnsupportedVersion
    );
    let mut v = inventory("RSA");
    v["extra"] = "x".repeat(1025).into();
    assert_eq!(
        ingest(&v, InputFormat::InventoryV1).unwrap_err(),
        Error::ResourceLimit
    );
    assert_eq!(
        SavedEvidence::from_bytes(&vec![b' '; 1024 * 1024 + 1], InputFormat::TlsV1).unwrap_err(),
        Error::ResourceLimit
    );
}
#[test]
fn every_emitted_technical_source_resolves_to_an_edition_and_clause() {
    fn check(a: &pqc_scan_core::assessment::AlgorithmAssessment) {
        for id in &a.source_ids {
            let s = source(id).unwrap_or_else(|| panic!("unresolved {id}"));
            assert!(!s.edition.is_empty());
            assert!(!s.clause.is_empty());
        }
        for c in &a.components {
            check(&c.assessment);
        }
    }
    for (v, format) in [
        (inventory("ML-DSA-65"), InputFormat::InventoryV1),
        (tls(), InputFormat::TlsV1),
        (offers(), InputFormat::SshOffersV1),
        (kex(), InputFormat::SshKexV1),
    ] {
        let r = ingest(&v, format).unwrap();
        for t in r.targets() {
            check(&t.technical.algorithm);
            if let Some(id) = &t.technical.protocol_ref {
                assert!(source(id).is_some());
            }
        }
    }
}

#[test]
fn native_certificate_subject_signature_and_ambiguous_links_keep_distinct_roles() {
    let mut v = inventory("RSA");
    let alg = v["assets"][0].clone();
    let key_id = native_id("asset", &["key"]);
    let cert_id = native_id("asset", &["cert"]);
    let key = json!({"id":key_id,"kind":"public_key","identity_basis":"public_spki_sha256","attributes":{"spki_sha256":"c".repeat(64),"key_bits":1024,"named_curve_oid":null},"relations":[{"relationship":"algorithm","target":alg["id"]}],"uncertainty":[]});
    let cert = json!({"id":cert_id,"kind":"certificate","identity_basis":"public_certificate_der_sha256","attributes":{"certificate_sha256":"b".repeat(64),"not_before_unix":1700000000,"not_after_unix":1900000000},"relations":[{"relationship":"subject_key","target":key_id},{"relationship":"signature_algorithm","target":alg["id"]}],"uncertainty":[]});
    v["assets"] = json!([alg, key, cert]);
    v["observations"][0]["asset_ids"] = json!([cert_id]);
    let r = ingest(&v, InputFormat::InventoryV1).unwrap();
    assert_eq!(r.targets().len(), 2);
    let key = r
        .targets()
        .iter()
        .find(|t| t.source_ref.role == "certificate.subject_key.algorithm")
        .unwrap();
    assert!(key
        .technical
        .algorithm
        .classical_concerns
        .contains(&"public_key_below_local_minimum".into()));
    let sig = r
        .targets()
        .iter()
        .find(|t| t.source_ref.role == "certificate.signature_algorithm")
        .unwrap();
    assert_eq!(sig.technical.algorithm.purpose, Purpose::Signature);
    let relation = v["assets"][2]["relations"][0].clone();
    v["assets"][2]["relations"]
        .as_array_mut()
        .unwrap()
        .push(relation);
    let r = ingest(&v, InputFormat::InventoryV1).unwrap();
    let key = r
        .targets()
        .iter()
        .find(|t| t.source_ref.role == "certificate.subject_key.algorithm")
        .unwrap();
    assert_eq!(key.technical.algorithm.identifier, "unknown");
    assert!(key.limitations.contains(&"unresolved_role".into()));
}

#[test]
fn json_resource_limits_accept_below_and_at_and_reject_above() {
    // Every generated value is otherwise a valid Native v1 document. Future
    // extension fields isolate the generic resource boundary from product facts.
    for n in [1023, 1024, 1025] {
        let mut v = inventory("RSA");
        v["extension"] = "x".repeat(n).into();
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 1024,
            "string {n}"
        );
    }
    for n in [63, 64, 65] {
        let mut v = inventory("RSA");
        let base = v.as_object().unwrap().len();
        for i in base..n {
            v[format!("extension{i}")] = true.into();
        }
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 64,
            "fields {n}"
        );
    }
    for n in [15, 16, 17] {
        let mut v = inventory("RSA");
        let mut extra = json!(true);
        for _ in 1..n {
            extra = json!({"x":extra});
        }
        v["extension"] = extra;
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 16,
            "depth {n}"
        );
    }
    for n in [65535, 65536, 65537] {
        let mut v = inventory("RSA");
        v["extension"] = json!(vec![false; n]);
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 65536,
            "array {n}"
        );
    }
    for format in [InputFormat::InventoryV1, InputFormat::TlsV1] {
        let value = if format == InputFormat::InventoryV1 {
            inventory("RSA")
        } else {
            tls()
        };
        let base = serde_json::to_vec(&value).unwrap();
        for size in [
            format.byte_limit() - 1,
            format.byte_limit(),
            format.byte_limit() + 1,
        ] {
            let mut bytes = base.clone();
            bytes.resize(size, b' ');
            let result = SavedEvidence::from_bytes(&bytes, format);
            if size <= format.byte_limit() {
                assert!(result.is_ok(), "bytes {size}");
            } else {
                assert_eq!(result.unwrap_err(), Error::ResourceLimit);
            }
        }
    }
}

#[test]
fn native_record_limits_accept_below_and_at_and_reject_above() {
    for n in [63, 64, 65] {
        let mut v = inventory("RSA");
        v["assets"][0]["relations"] = json!(vec![
            json!({"relationship":"future","target":v["assets"][0]["id"]});
            n
        ]);
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 64,
            "relations {n}"
        );
        let mut v = inventory("RSA");
        v["observations"][0]["asset_ids"] = json!(vec![v["assets"][0]["id"].clone(); n]);
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 64,
            "asset refs {n}"
        );
    }
    for n in [16383, 16384, 16385] {
        let mut v = inventory("RSA");
        let base = v["assets"][0].clone();
        let mut assets = vec![base.clone()];
        for i in 1..n {
            let mut asset = base.clone();
            asset["id"] = native_id("asset", &["extra", &i.to_string()]).into();
            assets.push(asset);
        }
        v["assets"] = assets.into();
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 16384,
            "assets {n}"
        );
    }
    for n in [32767, 32768, 32769] {
        let mut v = inventory("RSA");
        let base = v["observations"][0].clone();
        let mut observations = vec![];
        for i in 0..n {
            let mut o = base.clone();
            o["id"] = native_id("observation", &[&i.to_string()]).into();
            observations.push(o);
        }
        v["observations"] = observations.into();
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 32768,
            "observations {n}"
        );
    }
    for n in [65535, 65536, 65537] {
        let mut v = inventory("RSA");
        let a = v["assets"][0].clone();
        let mut b = a.clone();
        b["id"] = native_id("asset", &["second"]).into();
        let mut c = a.clone();
        c["id"] = native_id("asset", &["third"]).into();
        v["assets"] = json!([a, b, c]);
        let base = v["observations"][0].clone();
        let mut remaining = n;
        let mut observations = vec![];
        for i in 0..32768 {
            let mut o = base.clone();
            o["id"] = native_id("observation", &[&i.to_string()]).into();
            let count = if i == 0 && n == 65537 {
                3
            } else {
                remaining.min(2)
            };
            o["asset_ids"] = json!(v["assets"].as_array().unwrap()[..count]
                .iter()
                .map(|a| a["id"].clone())
                .collect::<Vec<_>>());
            remaining -= count;
            observations.push(o);
        }
        assert_eq!(remaining, 0);
        v["observations"] = observations.into();
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).is_ok(),
            n <= 65536,
            "targets {n}"
        );
    }
}

#[test]
fn saved_collection_times_preserve_fractional_precision_and_reject_invalid_dates() {
    for time in [
        "2026-11-17T00:00:00.123456789Z",
        "2026-11-17T09:00:00+09:00",
    ] {
        let mut v = tls();
        v["generated_at"] = time.into();
        let r = ingest(&v, InputFormat::TlsV1).unwrap();
        assert_eq!(r.input().collection_time, time);
    }
    for time in [
        "2026-11-17",
        "1969-12-31T23:59:59Z",
        "2026-02-30T00:00:00Z",
        "2026-11-17T00:00:60Z",
    ] {
        let mut v = tls();
        v["generated_at"] = time.into();
        assert!(ingest(&v, InputFormat::TlsV1).is_err());
    }
}

#[test]
fn file_ingestion_rejects_nonregular_inputs_and_performs_no_endpoint_connection() {
    let dir = std::env::temp_dir().join(format!("pqc-migration067-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("saved.json");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let addr = listener.local_addr().unwrap();
    let mut v = tls();
    v["target"]["host"] = "127.0.0.1".into();
    v["target"]["port"] = addr.port().into();
    v["target"]["resolved_addresses"] = json!(["127.0.0.1"]);
    v["target"]["pinned_address"] = addr.to_string().into();
    v["target"]["connected_peer"] = addr.to_string().into();
    let bytes = serde_json::to_vec(&v).unwrap();
    std::fs::write(&input, &bytes).unwrap();
    let r = SavedEvidence::read(&input, InputFormat::TlsV1).unwrap();
    assert_eq!(r.targets().len(), 3);
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert_eq!(
        SavedEvidence::read(&dir, InputFormat::TlsV1).unwrap_err(),
        Error::InputUnavailable
    );
    #[cfg(unix)]
    {
        let link = dir.join("symlink.json");
        std::os::unix::fs::symlink(&input, &link).unwrap();
        assert_eq!(
            SavedEvidence::read(&link, InputFormat::TlsV1).unwrap_err(),
            Error::InputUnavailable
        );
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn unavailable_dsa_dh_parameter_interpretation_preserves_concern_without_promotion() {
    for id in ["DSA", "DSA-SHA1", "DH"] {
        for bits in ["1024", "2048"] {
            let a = assess_primitive(
                id,
                &BTreeMap::from([
                    ("key_bits".into(), bits.into()),
                    ("parameter_interpretation".into(), "not_available".into()),
                ]),
            );
            assert!(!a.parameters_known, "{id}/{bits}");
            assert_eq!(a.quantum, QuantumExposure::Vulnerable);
            assert_eq!(
                a.classical_concerns
                    .contains(&"public_key_below_local_minimum".into()),
                bits == "1024"
            );
        }
    }
}

#[test]
fn tls_verification_identity_and_sni_match_saved_collector_semantics() {
    for (name, sni, valid) in [
        ("", json!(""), false),
        ("example.test", Value::Null, false),
        ("example.test", json!("different.test"), false),
        ("127.0.0.1", json!("127.0.0.1"), false),
        ("127.0.0.1", Value::Null, true),
        ("other.test", json!("other.test"), true),
    ] {
        let mut v = tls();
        v["details"]["verification"]["verification_name"] = name.into();
        v["details"]["verification"]["sent_sni"] = sni;
        assert_eq!(ingest(&v, InputFormat::TlsV1).is_ok(), valid, "{name}");
    }
    for (source, bundle) in [
        ("bundled_mozilla", Value::Null),
        ("local_ca_file", json!("1.0.11")),
    ] {
        let mut v = tls();
        v["details"]["verification"]["trust_source"] = source.into();
        v["details"]["verification"]["trust_bundle_version"] = bundle;
        assert!(ingest(&v, InputFormat::TlsV1).is_err());
    }
    let mut v = tls();
    v["details"]["verification"]["trust_source"] = "local_ca_file".into();
    v["details"]["verification"]["trust_bundle_version"] = Value::Null;
    assert!(ingest(&v, InputFormat::TlsV1).is_ok());
}

#[test]
fn existing_smaller_probe_limits_remain_enforced_at_boundaries() {
    for n in [15, 16, 17] {
        let mut v = tls();
        let certificate = v["details"]["peer_certificates"][0].clone();
        v["details"]["peer_certificates"] = json!(vec![certificate; n]);
        v["details"]["certificate_collection"]["collected_count"] = n.into();
        v["details"]["certificate_collection"]["presented_count"] = n.into();
        assert_eq!(
            ingest(&v, InputFormat::TlsV1).is_ok(),
            n <= 16,
            "certificates {n}"
        );
    }
    for (field, max) in [("key_exchange_groups", 64), ("cipher_suites", 256)] {
        for n in [max - 1, max, max + 1] {
            let mut v = tls();
            let item = v["details"]["client_profile"][field][0].clone();
            v["details"]["client_profile"][field] = json!(vec![item; n]);
            assert_eq!(
                ingest(&v, InputFormat::TlsV1).is_ok(),
                n <= max,
                "{field}/{n}"
            );
        }
    }
    for n in [1, 2, 3] {
        let mut v = tls();
        let item = v["details"]["client_profile"]["protocol_versions"][0].clone();
        v["details"]["client_profile"]["protocol_versions"] = json!(vec![item; n]);
        assert_eq!(
            ingest(&v, InputFormat::TlsV1).is_ok(),
            n <= 2,
            "versions {n}"
        );
    }
    for n in [31, 32, 33] {
        let mut v = failed_tls();
        v["diagnostics"] = json!(vec!["certificate_validation_failed"; n]);
        assert_eq!(
            ingest(&v, InputFormat::TlsV1).is_ok(),
            n <= 32,
            "diagnostics {n}"
        );
    }
    for n in [255, 256, 257] {
        let mut v = offers();
        v["details"]["offers"]["kex_algorithms"] = json!(vec!["curve25519-sha256"; n]);
        assert_eq!(
            ingest(&v, InputFormat::SshOffersV1).is_ok(),
            n <= 256,
            "ssh list {n}"
        );
    }
    for n in [63, 64, 65] {
        let mut v = offers();
        v["details"]["offers"]["kex_algorithms"] = json!(["x".repeat(n)]);
        assert_eq!(
            ingest(&v, InputFormat::SshOffersV1).is_ok(),
            n <= 64,
            "ssh name {n}"
        );
    }
}
