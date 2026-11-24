use pqc_scan_core::migration::{
    generate_context_with_references, ClaimAnswer, ContextClaims, ContextReference,
    ContextSelection, InputFormat, MigrationContext, Provenance,
};
use serde_json::json;
#[path = "support/migration/context.rs"]
mod ctx;
#[path = "support/migration/mod.rs"]
#[allow(dead_code)]
mod fixture;
fn reference() -> ContextReference {
    ContextReference {
        id: "policy".into(),
        issuer_id: "organization".into(),
        document_id: "transition-policy".into(),
        edition: "2026".into(),
        clause: "sso".into(),
        scope_id: "system-1".into(),
        https_uri: None,
    }
}
#[test]
fn authored_deadline_requires_a_valid_reference_and_roundtrips_the_existing_schema() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let answers = [ClaimAnswer {
        field: "deadline".into(),
        value: Some(
            json!({"kind":"at","at":"2027-03-31T23:59:59+09:00","reference_id":"policy","scope_reason":"organization_policy"}),
        ),
    }];
    let provenance = Provenance::operator(
        "owner",
        "2026-11-17T00:00:00Z",
        Some("2027-06-30T00:00:00Z"),
    )
    .unwrap();
    assert!(ContextClaims::from_answers(&answers, provenance.clone()).is_err());
    assert!(ContextClaims::from_answers_with_references(
        &answers,
        provenance.clone(),
        &[reference(), reference()]
    )
    .is_err());
    let claims =
        ContextClaims::from_answers_with_references(&answers, provenance, &[reference()]).unwrap();
    let selections = [ContextSelection {
        target_index: 0,
        system_id: "system-1".into(),
        claims,
    }];
    assert!(
        generate_context_with_references(&e, "organization", &selections, ctx::AS_OF, &[]).is_err()
    );
    let bytes = generate_context_with_references(
        &e,
        "organization",
        &selections,
        ctx::AS_OF,
        &[reference()],
    )
    .unwrap();
    let c = MigrationContext::from_bytes(&bytes).unwrap();
    c.bind(&e, ctx::AS_OF).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&c.to_bytes().unwrap()).unwrap();
    assert_eq!(
        value["bindings"][0]["claims"]["deadline"]["value"]["at"],
        "2027-03-31T14:59:59Z"
    );
    assert_eq!(value["references"][0]["id"], "policy");
}
