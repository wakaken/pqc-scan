#[path = "support/migration/mod.rs"]
mod fixture;
use fixture::*;
use pqc_scan_core::{
    identity::native_id,
    migration::{parse_as_of, ClaimFreshness, Error, InputFormat, MigrationContext, SavedEvidence},
};
use serde_json::{json, Value};
const NOW: &str = "2026-11-18T00:00:00Z";
fn provenance() -> Value {
    json!({"source_kind":"operator_claim","actor_id":"owner","asserted_at":"2026-11-17T00:00:00Z","valid_until":"2027-11-17T00:00:00Z"})
}
fn known(value: Value) -> Value {
    json!({"state":"known","value":value,"provenance":provenance()})
}
fn unknown() -> Value {
    json!({"state":"unknown","provenance":provenance()})
}
fn context(e: &SavedEvidence) -> Value {
    json!({"format":"pqc-scan.migration-context","version":1,"input":{"format":e.input().format,"sha256":e.input().sha256},"organization_id":"example-org","systems":[{"id":"system-a"},{"id":"system-b"}],"bindings":[{"target_id":e.targets()[0].id,"system_id":"system-a"}]})
}
fn parse(v: &Value) -> pqc_scan_core::migration::Result<MigrationContext> {
    MigrationContext::from_bytes(&serde_json::to_vec(v).unwrap())
}
fn reference() -> Value {
    json!({"id":"policy-1","issuer_id":"example-org","document_id":"migration-policy","edition":"v1","clause":"section-1","scope_id":"example-org","https_uri":"https://example.test/policy#section1"})
}

#[test]
fn minimal_context_needs_no_irrelevant_business_fields_and_preserves_false_unknown_absent() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut v = context(&e);
    let c = parse(&v).unwrap().bind(&e, NOW).unwrap();
    assert_eq!(c.target(&e.targets()[0].id).unwrap().claims().count(), 0);
    v["bindings"][0]["claims"] = json!({"business_critical":known(json!(false)),"confidential":unknown(),"applicability":known(json!("not_applicable"))});
    let c = parse(&v).unwrap().bind(&e, NOW).unwrap();
    let t = c.target(&e.targets()[0].id).unwrap();
    assert_eq!(
        t.claim("business_critical").unwrap().active_value(),
        Some(&json!(false))
    );
    assert_eq!(t.claim("confidential").unwrap().state, "unknown");
    assert!(t.claim("confidential").unwrap().active_value().is_none());
    assert!(t.claim("usage_state").is_none());
    assert_eq!(
        t.claim("applicability").unwrap().active_value(),
        Some(&json!("not_applicable"))
    );
}
#[test]
fn a12_same_algorithm_separate_systems_defaults_and_whole_claim_overrides_do_not_leak() {
    let mut value = inventory("RSA");
    let mut second = value["observations"][0].clone();
    second["id"] = native_id("observation", &["other-system"]).into();
    second["occurrence_key"] = native_id("occurrence", &["other-system"]).into();
    value["observations"].as_array_mut().unwrap().push(second);
    let e = ingest(&value, InputFormat::InventoryV1).unwrap();
    assert_eq!(e.targets().len(), 2);
    assert_eq!(
        e.targets()[0].source_ref.root_asset_id,
        e.targets()[1].source_ref.root_asset_id
    );
    let mut v = context(&e);
    v["organization_defaults"] = json!({"business_critical":known(json!(false)),"support":{"runtime":known(json!(false)),"peer":known(json!(true))}});
    v["systems"][0]["defaults"] =
        json!({"business_critical":known(json!(true)),"support":{"runtime":known(json!(true))}});
    v["bindings"] = json!([{"target_id":e.targets()[0].id,"system_id":"system-a","claims":{"support":{"peer":unknown()}}},{"target_id":e.targets()[1].id,"system_id":"system-b"}]);
    let bound = parse(&v).unwrap().bind(&e, NOW).unwrap();
    let a = bound.target(&e.targets()[0].id).unwrap();
    let b = bound.target(&e.targets()[1].id).unwrap();
    assert_eq!(a.system_id, "system-a");
    assert_eq!(b.system_id, "system-b");
    assert_eq!(
        a.claim("business_critical").unwrap().active_value(),
        Some(&json!(true))
    );
    assert_eq!(
        b.claim("business_critical").unwrap().active_value(),
        Some(&json!(false))
    );
    assert_eq!(
        a.claim("support.runtime").unwrap().active_value(),
        Some(&json!(true))
    );
    assert_eq!(
        b.claim("support.runtime").unwrap().active_value(),
        Some(&json!(false))
    );
    assert_eq!(a.claim("support.peer").unwrap().state, "unknown");
    assert_eq!(
        b.claim("support.peer").unwrap().active_value(),
        Some(&json!(true))
    );
    assert!(a
        .claim("business_critical")
        .unwrap()
        .context_pointer
        .starts_with("/systems/0/defaults"));
    assert_eq!(
        a.claim("support.peer").unwrap().context_pointer,
        "/bindings/0/claims/support/peer"
    );
    v["bindings"].as_array_mut().unwrap().pop();
    let bound = parse(&v).unwrap().bind(&e, NOW).unwrap();
    assert!(
        bound.target(&e.targets()[1].id).is_none(),
        "unbound target must not receive organization defaults"
    );
}
#[test]
fn a13_a14_content_format_missing_duplicate_and_ambiguous_bindings_reject() {
    let value = inventory("RSA");
    let e = ingest(&value, InputFormat::InventoryV1).unwrap();
    let original = context(&e);
    let pretty = SavedEvidence::from_bytes(
        &serde_json::to_vec_pretty(&value).unwrap(),
        InputFormat::InventoryV1,
    )
    .unwrap();
    assert_eq!(
        parse(&original).unwrap().bind(&pretty, NOW).unwrap_err(),
        Error::InputBindingMismatch
    );
    for (pointer, replacement) in [
        ("/input/format", json!("tls-v1")),
        ("/input/sha256", json!("b".repeat(64))),
        (
            "/bindings/0/target_id",
            json!(native_id("migration_target", &["missing"])),
        ),
    ] {
        let mut v = original.clone();
        *v.pointer_mut(pointer).unwrap() = replacement;
        assert_eq!(
            parse(&v).unwrap().bind(&e, NOW).unwrap_err(),
            if pointer == "/bindings/0/target_id" {
                Error::InvalidReference
            } else {
                Error::InputBindingMismatch
            }
        );
    }
    for duplicate in ["systems", "bindings"] {
        let mut v = original.clone();
        let row = v[duplicate][0].clone();
        v[duplicate].as_array_mut().unwrap().push(row);
        assert_eq!(
            parse(&v).unwrap_err(),
            if duplicate == "bindings" {
                Error::AmbiguousContextBinding
            } else {
                Error::InvalidContextBinding
            }
        );
    }
    let mut v = original.clone();
    v["bindings"][0]["system_id"] = "missing-system".into();
    assert_eq!(parse(&v).unwrap_err(), Error::InvalidReference);
    let mut ambiguous = value;
    ambiguous["observations"][0]["identity_confidence"] = "ambiguous_duplicate_anchor".into();
    let e = ingest(&ambiguous, InputFormat::InventoryV1).unwrap();
    assert_eq!(
        parse(&context(&e)).unwrap().bind(&e, NOW).unwrap_err(),
        Error::AmbiguousContextBinding
    );
}
#[test]
fn a15_provenance_freshness_boundaries_unknown_expiry_and_external_label_are_explicit() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    for (asserted, until, expected) in [
        (
            "2026-11-18T00:00:00Z",
            Some("2026-11-19T00:00:00Z"),
            ClaimFreshness::Active,
        ),
        ("2026-11-17T00:00:00Z", Some(NOW), ClaimFreshness::Stale),
        (
            "2026-11-19T00:00:00Z",
            Some("2026-11-20T00:00:00Z"),
            ClaimFreshness::NotYetEffective,
        ),
        ("2026-11-17T00:00:00Z", None, ClaimFreshness::Unknown),
    ] {
        let mut v = context(&e);
        let mut p = provenance();
        p["asserted_at"] = asserted.into();
        if let Some(until) = until {
            p["valid_until"] = until.into();
        } else {
            p.as_object_mut().unwrap().remove("valid_until");
        }
        p["source_kind"] = "external_review_claim".into();
        v["bindings"][0]["claims"] =
            json!({"business_critical":{"state":"known","value":true,"provenance":p}});
        let b = parse(&v).unwrap().bind(&e, NOW).unwrap();
        let claim = b
            .target(&e.targets()[0].id)
            .unwrap()
            .claim("business_critical")
            .unwrap();
        assert_eq!(claim.freshness, expected);
        assert_eq!(
            claim.active_value().is_some(),
            expected == ClaimFreshness::Active
        );
        let record = serde_json::to_value(claim).unwrap();
        assert_eq!(record["provenance"]["source_kind"], "external_review_claim");
        assert_eq!(record["provenance"]["actor_id"], "owner");
        assert!(!serde_json::to_string(&record)
            .unwrap()
            .contains("scanner_verified"));
    }
}
#[test]
fn a07_incompatible_purpose_is_retained_as_conflict_and_never_rewrites_primitive() {
    for (id, usage, conflicting) in [
        ("ML-KEM-768", "signature", true),
        ("ML-DSA-65", "key_establishment", true),
        ("RSA", "hashing", true),
        ("RSA", "signature", false),
        ("RSA-PSS", "key_establishment", true),
    ] {
        let e = ingest(&inventory(id), InputFormat::InventoryV1).unwrap();
        let mut v = context(&e);
        v["bindings"][0]["claims"] = json!({"use_case":known(json!(usage))});
        let b = parse(&v).unwrap().bind(&e, NOW).unwrap();
        assert_eq!(
            b.target(&e.targets()[0].id).unwrap().purpose_conflicting,
            conflicting
        );
        assert_eq!(e.targets()[0].technical.algorithm.identifier, id);
    }
}
#[test]
fn references_deadlines_and_all_typed_fields_validate_without_fetching() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut v = context(&e);
    v["references"] = json!([reference()]);
    let mut p = provenance();
    p["reference_id"] = "policy-1".into();
    v["bindings"][0]["claims"] = json!({"applicability":known(json!("applicable")),"use_case":known(json!("signature")),"usage_state":known(json!("planned")),"business_critical":known(json!(true)),"confidential":known(json!(false)),"confidentiality_until":known(json!("2036-11-18T09:00:00+09:00")),"signature_verification_until":known(json!("2030-11-18T00:00:00Z")),"historical_ciphertext":known(json!(true)),"migration_lead_seconds":known(json!(86400)),"deadline":{"state":"known","value":{"kind":"at","at":"2026-11-19T00:00:00Z","reference_id":"policy-1","scope_reason":"organization_policy"},"provenance":p},"support":{"runtime":known(json!(true)),"provider":known(json!(true)),"peer":known(json!(false)),"protocol":unknown(),"parameters":known(json!(true)),"jose":known(json!(false)),"verifiers":known(json!(true)),"pki":unknown()}});
    let b = parse(&v).unwrap().bind(&e, NOW).unwrap();
    let t = b.target(&e.targets()[0].id).unwrap();
    assert_eq!(
        t.claim("confidentiality_until").unwrap().active_value(),
        Some(&json!("2036-11-18T00:00:00Z"))
    );
    assert_eq!(b.references()[0].id, "policy-1");
    let mut missing = v.clone();
    missing["references"] = json!([]);
    assert_eq!(parse(&missing).unwrap_err(), Error::InvalidReference);
    let mut duplicate = v.clone();
    duplicate["references"]
        .as_array_mut()
        .unwrap()
        .push(reference());
    assert_eq!(parse(&duplicate).unwrap_err(), Error::InvalidReference);
    for uri in [
        "file:///etc/passwd",
        "http://example.test/policy",
        "https://user:password@example.test/policy",
        "https://example.test/policy?token=SECRET_CANARY_068",
        "https://example.test/\nvalue",
        "https://example.test:0/path",
    ] {
        let mut invalid = v.clone();
        invalid["references"][0]["https_uri"] = uri.into();
        let error = parse(&invalid).unwrap_err();
        assert!(!error.to_string().contains("CANARY"));
    }
}
#[test]
fn strict_claims_reject_null_unknown_duplicate_fields_types_and_verification_labels() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let base = context(&e);
    for claims in [
        json!({"business_critical":null}),
        json!({"business_critical":known(json!("false"))}),
        json!({"business_critical":{"state":"known","value":false,"provenance":provenance(),"verified":true}}),
        json!({"business_critical":{"state":"unknown","value":false,"provenance":provenance()}}),
        json!({"use_case":known(json!("unknown"))}),
        json!({"support":{"runtime":null}}),
        json!({"SECRET_CANARY_068":"SECRET_CANARY_068"}),
    ] {
        let mut v = base.clone();
        v["bindings"][0]["claims"] = claims;
        let error = parse(&v).unwrap_err();
        assert!(!error.to_string().contains("CANARY"));
    }
    let mut v = base.clone();
    v["bindings"][0]["claims"] = json!({"business_critical":known(json!(true))});
    v["bindings"][0]["claims"]["business_critical"]["provenance"]["source_kind"] =
        "scanner_verified".into();
    assert!(parse(&v).is_err());
    let text = serde_json::to_string(&base)
        .unwrap()
        .replace("\"version\":1", "\"version\":1,\"version\":1");
    assert!(MigrationContext::from_bytes(text.as_bytes()).is_err());
    let nested = serde_json::to_string(&base).unwrap().replace(
        "\"id\":\"system-a\"",
        "\"id\":\"system-a\",\"id\":\"system-a\"",
    );
    assert!(MigrationContext::from_bytes(nested.as_bytes()).is_err());
}
#[test]
fn explicit_clock_dates_intervals_and_lead_bounds_are_checked() {
    assert_eq!(parse_as_of("2026-11-18T09:00:00+09:00").unwrap(), NOW);
    for date in [
        "2026-11-18",
        "2026-11-18T00:00:00.1Z",
        "2026-11-18T00:00:60Z",
        "2026-02-30T00:00:00Z",
        "1969-12-31T23:59:59Z",
    ] {
        assert!(parse_as_of(date).is_err(), "{date}");
    }
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let base = context(&e);
    assert!(parse(&base)
        .unwrap()
        .bind(&e, "2026-11-16T00:00:00Z")
        .is_err());
    for lead in [
        json!(0),
        json!(315576000),
        json!(-1),
        json!(315576001),
        json!(1.5),
    ] {
        let mut v = base.clone();
        v["bindings"][0]["claims"] = json!({"migration_lead_seconds":known(lead.clone())});
        assert_eq!(parse(&v).is_ok(), lead == 0 || lead == 315576000);
    }
    for until in ["2026-11-17T00:00:00Z", "2026-11-16T00:00:00Z"] {
        let mut v = base.clone();
        v["bindings"][0]["claims"] = json!({"business_critical":known(json!(false))});
        v["bindings"][0]["claims"]["business_critical"]["provenance"]["valid_until"] = until.into();
        assert!(parse(&v).is_err());
    }
}
#[test]
fn context_and_claim_identity_bind_exact_bytes_and_pointers() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut v = context(&e);
    v["organization_defaults"] = json!({"business_critical":known(json!(true))});
    let a = parse(&v).unwrap();
    let b = parse(&v).unwrap();
    let c = MigrationContext::from_bytes(&serde_json::to_vec_pretty(&v).unwrap()).unwrap();
    assert_eq!(a.sha256(), b.sha256());
    assert_ne!(a.sha256(), c.sha256());
    let a = a.bind(&e, NOW).unwrap();
    let b = b.bind(&e, NOW).unwrap();
    let c = c.bind(&e, NOW).unwrap();
    let field = |b: &pqc_scan_core::migration::BoundContext| {
        b.target(&e.targets()[0].id)
            .unwrap()
            .claim("business_critical")
            .unwrap()
            .id
            .clone()
    };
    assert_eq!(field(&a), field(&b));
    assert_ne!(field(&a), field(&c));
}
#[test]
fn context_boundaries_accept_systems_bindings_alias_reference_and_bytes_maxima() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let base = context(&e);
    for n in [255, 256, 257] {
        let mut v = base.clone();
        v["systems"] = (0..n)
            .map(|i| json!({"id":if i==0{"system-a".to_string()}else{format!("system-{i}")}}))
            .collect::<Vec<_>>()
            .into();
        assert_eq!(parse(&v).is_ok(), n <= 256, "systems {n}");
    }
    for n in [4095, 4096, 4097] {
        let mut v = base.clone();
        v["bindings"]=(0..n).map(|i|json!({"target_id":native_id("migration_target",&[&i.to_string()]),"system_id":"system-a"})).collect::<Vec<_>>().into();
        assert_eq!(parse(&v).is_ok(), n <= 4096, "bindings {n}");
    }
    for n in [63, 64, 65] {
        let mut v = base.clone();
        v["organization_id"] = "a".repeat(n).into();
        assert_eq!(parse(&v).is_ok(), n <= 64, "alias {n}");
    }
    for n in [127, 128, 129] {
        let mut v = base.clone();
        let mut r = reference();
        r["id"] = "r".repeat(n).into();
        v["references"] = json!([r]);
        assert_eq!(parse(&v).is_ok(), n <= 128, "reference {n}");
    }
    let bytes = serde_json::to_vec(&base).unwrap();
    for n in [1024 * 1024 - 1, 1024 * 1024, 1024 * 1024 + 1] {
        let mut b = bytes.clone();
        b.resize(n, b' ');
        assert_eq!(
            MigrationContext::from_bytes(&b).is_ok(),
            n <= 1024 * 1024,
            "bytes {n}"
        );
    }
}
#[test]
fn new_context_schema_and_parser_agree_on_structural_controls_and_legacy_stays_separate() {
    let schema: Value = serde_json::from_str(include_str!(
        "../../../schemas/migration-context-v1.schema.json"
    ))
    .unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut v = context(&e);
    assert!(validator.is_valid(&v));
    assert!(parse(&v).is_ok());
    v["bindings"][0]["claims"] =
        json!({"business_critical":known(json!(false)),"use_case":unknown()});
    assert!(validator.is_valid(&v));
    assert!(parse(&v).is_ok());
    v["bindings"][0]["claims"]["business_critical"] = Value::Null;
    assert!(!validator.is_valid(&v));
    assert!(parse(&v).is_err());
    assert!(MigrationContext::from_bytes(br#"{"version":1,"use_case":"signature"}"#).is_err());
    assert!(pqc_scan_core::policy::AssessmentContext::from_json(
        r#"{"version":1,"use_case":"signature"}"#
    )
    .is_ok());
}
#[test]
fn explicit_context_binding_works_for_each_selected_saved_format_without_collection() {
    for (value, format) in [
        (inventory("RSA"), InputFormat::InventoryV1),
        (tls(), InputFormat::TlsV1),
        (failed_tls(), InputFormat::TlsV1),
        (offers(), InputFormat::SshOffersV1),
        (kex(), InputFormat::SshKexV1),
    ] {
        let e = ingest(&value, format).unwrap();
        let c = parse(&context(&e)).unwrap().bind(&e, NOW).unwrap();
        assert_eq!(c.target(&e.targets()[0].id).unwrap().system_id, "system-a");
    }
}

#[test]
fn maximal_binding_counts_resolve_real_manifest_targets() {
    let mut value = inventory("RSA");
    let base = value["observations"][0].clone();
    value["observations"] = (0..4097)
        .map(|i| {
            let mut o = base.clone();
            o["id"] = native_id("observation", &[&i.to_string()]).into();
            o["occurrence_key"] = native_id("occurrence", &[&i.to_string()]).into();
            o
        })
        .collect::<Vec<_>>()
        .into();
    let evidence = ingest(&value, InputFormat::InventoryV1).unwrap();
    for n in [4095, 4096, 4097] {
        let mut v = context(&evidence);
        v["bindings"] = evidence.targets()[..n]
            .iter()
            .map(|t| json!({"target_id":t.id,"system_id":"system-a"}))
            .collect::<Vec<_>>()
            .into();
        let parsed = parse(&v);
        if n <= 4096 {
            let bound = parsed.unwrap().bind(&evidence, NOW).unwrap();
            assert!(bound.target(&evidence.targets()[n - 1].id).is_some());
        } else {
            assert_eq!(parsed.unwrap_err(), Error::ResourceLimit);
        }
    }
}

#[test]
fn shared_resource_preflight_and_context_privacy_apply_before_typed_parsing() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let base = context(&e);
    let mut deep = json!(true);
    for _ in 0..17 {
        deep = json!({"x":deep});
    }
    let mut v = base.clone();
    v["unrecognized"] = deep;
    assert_eq!(parse(&v).unwrap_err(), Error::ResourceLimit);
    let mut v = base.clone();
    v["unrecognized"] = "SECRET_CANARY_068".repeat(100).into();
    let error = parse(&v).unwrap_err();
    assert_eq!(error, Error::ResourceLimit);
    assert!(!error.to_string().contains("CANARY"));
    let mut v = base.clone();
    for i in 0..65 {
        v[format!("extra{i}")] = false.into();
    }
    assert_eq!(parse(&v).unwrap_err(), Error::ResourceLimit);
    for bytes in [b"\xff".as_slice(), b"\xef\xbb\xbf{}", b"{} {}"] {
        assert!(MigrationContext::from_bytes(bytes).is_err());
    }
    let mut v = base;
    v["bindings"][0]["claims"] =
        json!({"deadline":known(json!({"kind":"none","at":"2026-11-19T00:00:00Z"}))});
    assert!(parse(&v).is_err());
}

#[test]
fn context_file_read_is_regular_immutable_and_does_not_echo_rejected_contents() {
    let e = ingest(&inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let dir = std::env::temp_dir().join(format!("pqc-context068-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("context.json");
    let bytes = serde_json::to_vec(&context(&e)).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    assert!(MigrationContext::read(&path).unwrap().bind(&e, NOW).is_ok());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    assert_eq!(
        MigrationContext::read(&dir).unwrap_err(),
        Error::InputUnavailable
    );
    std::fs::write(&path, b"SECRET_CANARY_068").unwrap();
    assert!(!MigrationContext::read(&path)
        .unwrap_err()
        .to_string()
        .contains("CANARY"));
    std::fs::remove_dir_all(dir).unwrap();
}
