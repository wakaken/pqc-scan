#[path = "support/migration/context.rs"]
mod ctx;
#[path = "support/migration/mod.rs"]
#[allow(dead_code)]
mod fixture;
use pqc_scan_core::migration::*;
use serde_json::{json, Value};
fn evidence() -> SavedEvidence {
    fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap()
}
fn selection(claims: ContextClaims) -> ContextSelection {
    ContextSelection {
        target_index: 0,
        system_id: "system-1".into(),
        claims,
    }
}
fn answer(field: &str, value: Option<Value>) -> ClaimAnswer {
    ClaimAnswer {
        field: field.into(),
        value,
    }
}
#[test]
fn empty_sparse_false_unknown_and_na_preserve_meaning_and_schema() {
    let e = evidence();
    let empty = generate_context(&e, "organization", &[], ctx::AS_OF).unwrap();
    let c = MigrationContext::from_bytes(&empty).unwrap();
    assert!(c
        .bind(&e, ctx::AS_OF)
        .unwrap()
        .target(&e.targets()[0].id)
        .is_none());
    for value in [
        json!({}),
        json!({"usage_state":ctx::unknown(),"confidential":ctx::claim(json!(false)),"applicability":ctx::claim(json!("not_applicable"))}),
    ] {
        let claims = ContextClaims::from_bytes(&serde_json::to_vec(&value).unwrap()).unwrap();
        let bytes = generate_context(&e, "organization", &[selection(claims)], ctx::AS_OF).unwrap();
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        let schema: Value = serde_json::from_str(include_str!(
            "../../../schemas/migration-context-v1.schema.json"
        ))
        .unwrap();
        assert!(jsonschema::validator_for(&schema).unwrap().is_valid(&v));
        assert_eq!(v["bindings"][0]["claims"], value);
        assert_eq!(v["input"]["sha256"], e.input().sha256);
        assert!(!String::from_utf8(bytes.clone()).unwrap().contains("null"));
        let imported = MigrationContext::from_bytes(&bytes).unwrap();
        assert_eq!(imported.to_bytes().unwrap(), bytes);
        if value != json!({}) {
            let bound = imported.bind(&e, ctx::AS_OF).unwrap();
            let claims = bound.target(&e.targets()[0].id).unwrap();
            assert_eq!(
                claims.claim("confidential").unwrap().active_value(),
                Some(&json!(false))
            );
            assert!(claims.claim("usage_state").unwrap().state == "unknown");
            assert!(claims.claim("business_critical").is_none());
            assert_eq!(
                evaluate(&e, Some(&imported), ctx::AS_OF)
                    .unwrap()
                    .decisions()[0]
                    .urgency,
                Urgency::NotApplicable
            );
        }
    }
}
#[test]
fn explicit_scope_duplicates_ambiguity_substitution_and_time_are_checked() {
    let e = evidence();
    let s = selection(ContextClaims::default());
    assert_eq!(
        generate_context(&e, "organization", &[s.clone(), s.clone()], ctx::AS_OF).unwrap_err(),
        Error::AmbiguousContextBinding
    );
    let mut invalid = s.clone();
    invalid.target_index = 1;
    assert_eq!(
        generate_context(&e, "organization", &[invalid], ctx::AS_OF).unwrap_err(),
        Error::InvalidReference
    );
    assert_eq!(
        generate_context(&e, "INVALID alias", std::slice::from_ref(&s), ctx::AS_OF).unwrap_err(),
        Error::InvalidInput
    );
    assert_eq!(
        generate_context(
            &e,
            "organization",
            std::slice::from_ref(&s),
            "2000-01-01T00:00:00Z"
        )
        .unwrap_err(),
        Error::InvalidTime
    );
    let bytes = generate_context(&e, "organization", std::slice::from_ref(&s), ctx::AS_OF).unwrap();
    let mut other = fixture::inventory("RSA");
    other["producer"]["version"] = "changed".into();
    let other = fixture::ingest(&other, InputFormat::InventoryV1).unwrap();
    assert_eq!(
        MigrationContext::from_bytes(&bytes)
            .unwrap()
            .bind(&other, ctx::AS_OF)
            .unwrap_err(),
        Error::InputBindingMismatch
    );
    let mut ambiguous = fixture::inventory("RSA");
    ambiguous["observations"][0]["identity_confidence"] = "ambiguous_duplicate_anchor".into();
    let ambiguous = fixture::ingest(&ambiguous, InputFormat::InventoryV1).unwrap();
    assert!(!ambiguous.targets()[0].is_bindable());
    assert_eq!(
        generate_context(
            &ambiguous,
            "organization",
            std::slice::from_ref(&s),
            ctx::AS_OF
        )
        .unwrap_err(),
        Error::AmbiguousContextBinding
    );
    assert_eq!(
        generate_context(&e, "organization", &vec![s; 4097], ctx::AS_OF).unwrap_err(),
        Error::ResourceLimit
    );
}
#[test]
fn selected_target_does_not_apply_to_other_occurrences() {
    let mut input = fixture::inventory("RSA");
    let mut second = input["observations"][0].clone();
    second["id"] = pqc_scan_core::identity::native_id("observation", &["second"]).into();
    second["occurrence_key"] = pqc_scan_core::identity::native_id("occurrence", &["second"]).into();
    second["target"]["path"] = "other.rs".into();
    input["observations"].as_array_mut().unwrap().push(second);
    let e = fixture::ingest(&input, InputFormat::InventoryV1).unwrap();
    assert_eq!(e.targets().len(), 2);
    let c = MigrationContext::from_bytes(
        &generate_context(
            &e,
            "organization",
            &[selection(ContextClaims::default())],
            ctx::AS_OF,
        )
        .unwrap(),
    )
    .unwrap();
    let b = c.bind(&e, ctx::AS_OF).unwrap();
    assert!(b.target(&e.targets()[0].id).is_some());
    assert!(b.target(&e.targets()[1].id).is_none());
}
#[test]
fn answers_use_existing_types_and_never_invent_expiry() {
    let p = Provenance::operator("owner", "2026-11-17T09:00:00+09:00", None).unwrap();
    let claims = ContextClaims::from_answers(
        &[
            answer("usage_state", Some(json!("active"))),
            answer("support.peer", Some(json!(false))),
            answer("confidential", None),
        ],
        p.clone(),
    )
    .unwrap();
    let e = evidence();
    let bytes = generate_context(&e, "organization", &[selection(claims)], ctx::AS_OF).unwrap();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        v["bindings"][0]["claims"]["support"]["peer"]["value"],
        false
    );
    assert_eq!(
        v["bindings"][0]["claims"]["usage_state"]["provenance"]["asserted_at"],
        "2026-11-17T00:00:00Z"
    );
    assert!(v["bindings"][0]["claims"]["usage_state"]["provenance"]
        .get("valid_until")
        .is_none());
    for a in [
        vec![answer("business_critical", Some(json!("false")))],
        vec![answer("usage_state", Some(json!("guessed")))],
        vec![answer("support.missing", Some(json!(true)))],
        vec![answer("confidential", Some(Value::Null))],
        vec![
            answer("usage_state", None),
            answer("usage_state", Some(json!("active"))),
        ],
    ] {
        assert!(ContextClaims::from_answers(&a, p.clone()).is_err());
    }
    assert!(Provenance::operator("owner", ctx::AS_OF, Some("2026-01-01T00:00:00Z")).is_err());
}
#[test]
fn imported_all_supported_fields_references_and_stale_provenance_roundtrip() {
    let e = evidence();
    let mut v = ctx::context(&e, ctx::ordinary_signing());
    v["organization_defaults"] = json!({"historical_ciphertext":ctx::unknown(),"confidentiality_until":ctx::claim(json!("2030-01-01T00:00:00Z")),"migration_lead_seconds":ctx::claim(json!(123))});
    v["bindings"][0]["claims"]["deadline"] = ctx::claim(
        json!({"kind":"at","at":"2030-01-01T00:00:00Z","reference_id":"org-deadline","scope_reason":"contractual_requirement"}),
    );
    v["bindings"][0]["claims"]["business_critical"]["provenance"]["valid_until"] =
        "2026-11-17T12:00:00Z".into();
    let c = ctx::parse(&v);
    let exported = c.to_bytes().unwrap();
    assert_eq!(serde_json::from_slice::<Value>(&exported).unwrap(), v);
    let restored = MigrationContext::from_bytes(&exported).unwrap();
    let bound = restored.bind(&e, ctx::AS_OF).unwrap();
    assert_eq!(
        bound
            .target(&e.targets()[0].id)
            .unwrap()
            .claim("business_critical")
            .unwrap()
            .freshness,
        ClaimFreshness::Stale
    );
    assert_eq!(
        bound.references()[0].https_uri.as_deref(),
        Some("https://synthetic.invalid/never-fetch")
    );
}
#[test]
fn full_4096_sparse_bindings_fit_the_actual_context_limit() {
    let mut input = fixture::inventory("RSA");
    let original = input["observations"][0].clone();
    let observations = input["observations"].as_array_mut().unwrap();
    observations.clear();
    for n in 0..4096 {
        let mut row = original.clone();
        let name = n.to_string();
        row["id"] = pqc_scan_core::identity::native_id("observation", &[&name]).into();
        row["occurrence_key"] = pqc_scan_core::identity::native_id("occurrence", &[&name]).into();
        observations.push(row);
    }
    let e = fixture::ingest(&input, InputFormat::InventoryV1).unwrap();
    assert_eq!(e.targets().len(), 4096);
    let claims = ContextClaims::default();
    let selections = (0..4096)
        .map(|target_index| ContextSelection {
            target_index,
            system_id: "system-1".into(),
            claims: claims.clone(),
        })
        .collect::<Vec<_>>();
    let bytes = generate_context(&e, "organization", &selections, ctx::AS_OF).unwrap();
    assert!(bytes.len() < 1024 * 1024);
    let context = MigrationContext::from_bytes(&bytes).unwrap();
    let bound = context.bind(&e, ctx::AS_OF).unwrap();
    assert!(e.targets().iter().all(|t| bound.target(&t.id).is_some()));
    let large = ContextClaims::from_bytes(&serde_json::to_vec(&ctx::support()).unwrap()).err();
    assert!(
        large.is_some(),
        "support is nested, never accepted as untyped root claims"
    );
    let claims =
        ContextClaims::from_bytes(&serde_json::to_vec(&json!({"support":ctx::support()})).unwrap())
            .unwrap();
    let selections = (0..4096)
        .map(|target_index| ContextSelection {
            target_index,
            system_id: "system-1".into(),
            claims: claims.clone(),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        generate_context(&e, "organization", &selections, ctx::AS_OF).unwrap_err(),
        Error::ResourceLimit
    );
}
#[test]
fn compact_supported_context_is_not_rejected_by_pretty_expansion() {
    let e = evidence();
    let mut value = ctx::context(&e, json!({}));
    value["references"]=Value::Array((0..7500).map(|n|json!({"id":format!("r{n}"),"issuer_id":"a","document_id":"a","edition":"a","clause":"a","scope_id":"a"})).collect());
    let compact = serde_json::to_vec(&value).unwrap();
    assert!(compact.len() < 1024 * 1024);
    assert!(serde_json::to_vec_pretty(&value).unwrap().len() > 1024 * 1024);
    let c = MigrationContext::from_bytes(&compact).unwrap();
    let exported = c.to_bytes().unwrap();
    assert!(exported.len() <= 1024 * 1024);
    assert_eq!(serde_json::from_slice::<Value>(&exported).unwrap(), value);
    MigrationContext::from_bytes(&exported)
        .unwrap()
        .bind(&e, ctx::AS_OF)
        .unwrap();
}
#[test]
fn raw_claim_values_reject_nested_duplicates_before_value_conversion() {
    for raw in [
        r#"{"kind":"at","kind":"none"}"#,
        r#"{"nested":{"a":1,"a":2}}"#,
        r#"null"#,
    ] {
        assert!(ClaimValue::from_bytes(raw.as_bytes()).is_err());
        assert!(serde_json::from_str::<ClaimValue>(raw).is_err());
    }
    assert_eq!(
        ClaimValue::from_bytes(b"false").unwrap().into_value(),
        json!(false)
    );
}
#[test]
fn input_support_hints_match_actual_signing_encryption_and_certificate_requirements() {
    let rsa = evidence();
    let t = &rsa.targets()[0];
    assert_eq!(
        input_support_fields(t, Some("signature")).unwrap(),
        vec!["runtime", "provider", "parameters", "peer", "protocol"]
    );
    assert_eq!(
        input_support_fields(t, Some("jwt")).unwrap(),
        vec![
            "runtime",
            "provider",
            "parameters",
            "peer",
            "protocol",
            "jose",
            "verifiers"
        ]
    );
    let aes = fixture::ingest(&fixture::inventory("AES-256"), InputFormat::InventoryV1).unwrap();
    assert_eq!(
        input_support_fields(&aes.targets()[0], Some("encryption")).unwrap(),
        vec!["runtime", "provider", "parameters", "protocol"]
    );
    let cert = fixture::ingest(&fixture::tls(), InputFormat::TlsV1).unwrap();
    let cert = cert
        .targets()
        .iter()
        .find(|t| t.source_ref.role == "certificate_signature")
        .unwrap();
    assert!(input_support_fields(cert, None).unwrap().contains(&"pki"));
    assert_eq!(
        input_support_fields(t, Some("invented")).unwrap_err(),
        Error::InvalidInput
    );
}
