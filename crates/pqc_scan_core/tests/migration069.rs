#[path = "support/migration/context.rs"]
mod ctx;
#[path = "support/migration/mod.rs"]
mod fixture;
use fixture::*;
use pqc_scan_core::{
    identity::native_id,
    migration::{
        evaluate, evaluate_with_versions, ActionKind, Error, Information, InputFormat,
        MigrationAssessment, Readiness, Urgency, VersionSelection,
    },
};
use serde_json::{json, Value};
fn rsa() -> Value {
    let mut v = inventory("RSA");
    v["assets"][0]["attributes"]["parameters"] = json!({"key_bits":"2048"});
    v
}
fn assessed(v: &Value, format: InputFormat, claims: Option<Value>) -> MigrationAssessment {
    let e = ingest(v, format).unwrap();
    let c = claims.map(|claims| ctx::parse(&ctx::context(&e, claims)));
    let report = evaluate(&e, c.as_ref(), ctx::AS_OF).unwrap();
    report.validate_for(&e, c.as_ref()).unwrap();
    report
}
fn result(
    r: &MigrationAssessment,
    urgency: Urgency,
    information: Information,
    readiness: Readiness,
    action: ActionKind,
    reason: &str,
) {
    let d = &r.decisions()[0];
    assert_eq!(
        (d.urgency, d.information, d.readiness, d.action.kind),
        (urgency, information, readiness, action)
    );
    assert_eq!(d.reason_codes[0], reason);
}

#[test]
fn a01_a04_a06_context_free_references_and_declarations_never_establish_deployment() {
    for id in ["RSA", "ML-KEM-768", "Kyber768"] {
        let r = assessed(&inventory(id), InputFormat::InventoryV1, None);
        result(
            &r,
            Urgency::Undetermined,
            Information::Incomplete,
            Readiness::Unknown,
            ActionKind::Investigate,
            "insufficient_information",
        );
    }
    let mut v = inventory("Kyber768");
    v["assets"][0]["attributes"]["namespace"] = "draft".into();
    let r = assessed(&v, InputFormat::InventoryV1, None);
    assert_eq!(r.targets()[0].technical.algorithm.standardization, "draft");
    let mut v = inventory("RSA");
    v["assets"][0]["kind"] = "dependency".into();
    v["assets"][0]["attributes"] = json!({"name":"rustls","ecosystem":"cargo","reported_version":"0.23.46","declared_constraint":"0.23","resolved_version":"0.23.46","version_provenance":"lock_resolved"});
    let r = assessed(&v, InputFormat::InventoryV1, None);
    assert_eq!(r.targets()[0].technical.algorithm.identifier, "unknown");
    assert!(r.candidates().is_empty());
}
#[test]
fn a02_a09_urgent_ready_blocked_unknown_and_false_plus_missing_keep_independent_axes() {
    let base = ctx::critical_signing();
    result(
        &assessed(&rsa(), InputFormat::InventoryV1, Some(base.clone())),
        Urgency::Urgent,
        Information::Incomplete,
        Readiness::Unknown,
        ActionKind::Investigate,
        "critical_use_declared",
    );
    let mut claims = base;
    claims["support"] = ctx::support();
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(claims.clone()));
    result(
        &r,
        Urgency::Urgent,
        Information::Sufficient,
        Readiness::Ready,
        ActionKind::Migrate,
        "critical_use_declared",
    );
    assert!(r.decisions()[0].missing_inputs.is_empty());
    assert!(r.claims().iter().all(|c| !matches!(
        c.field.as_str(),
        "confidential" | "deadline" | "support.jose" | "support.verifiers" | "support.pki"
    )));
    claims["support"]["peer"] = ctx::claim(json!(false));
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(claims.clone()));
    result(
        &r,
        Urgency::Urgent,
        Information::Sufficient,
        Readiness::Blocked,
        ActionKind::Unblock,
        "critical_use_declared",
    );
    assert_eq!(r.decisions()[0].action.blocking_prerequisites, ["peer"]);
    claims["support"].as_object_mut().unwrap().remove("peer");
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(claims.clone()));
    result(
        &r,
        Urgency::Urgent,
        Information::Incomplete,
        Readiness::Unknown,
        ActionKind::Investigate,
        "critical_use_declared",
    );
    assert!(r.decisions()[0]
        .missing_inputs
        .contains(&"support.peer".into()));
    claims["support"]["runtime"] = ctx::claim(json!(false));
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(claims));
    result(
        &r,
        Urgency::Urgent,
        Information::Incomplete,
        Readiness::Blocked,
        ActionKind::Unblock,
        "critical_use_declared",
    );
    assert_eq!(r.decisions()[0].action.blocking_prerequisites, ["runtime"]);
    assert!(r.decisions()[0]
        .missing_inputs
        .contains(&"support.peer".into()));
}
#[test]
fn a03_protection_and_historical_ciphertext_are_independent_of_criticality_and_current_use() {
    let mut claims = json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment")),"usage_state":ctx::claim(json!("active")),"confidential":ctx::claim(json!(true)),"confidentiality_until":ctx::claim(json!("2036-11-18T00:00:00Z"))});
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(claims.clone()));
    result(
        &r,
        Urgency::Urgent,
        Information::Incomplete,
        Readiness::Unknown,
        ActionKind::Investigate,
        "long_protection_or_historical_ciphertext",
    );
    assert!(!r.decisions()[0]
        .missing_inputs
        .contains(&"business_critical".into()));
    claims["usage_state"] = ctx::claim(json!("unused"));
    claims["historical_ciphertext"] = ctx::claim(json!(true));
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(claims));
    assert_eq!(r.decisions()[0].urgency, Urgency::Urgent);
    assert!(r
        .claims()
        .iter()
        .any(|c| c.field == "historical_ciphertext" && c.value == Some(json!(true))));
}
#[test]
fn a08_deadline_boundaries_none_missing_lead_and_irrelevant_fields_are_exact() {
    for (at, expected) in [
        ("2026-11-19T00:00:00Z", Urgency::Urgent),
        ("2026-11-18T23:59:59Z", Urgency::Urgent),
        ("2026-11-19T00:00:01Z", Urgency::Plan),
    ] {
        let mut c = ctx::ordinary_signing();
        c["deadline"] = ctx::claim(
            json!({"kind":"at","at":at,"reference_id":"org-deadline","scope_reason":"organization_policy"}),
        );
        c["migration_lead_seconds"] = ctx::claim(json!(86400));
        let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c.clone()));
        assert_eq!(r.decisions()[0].urgency, expected);
        assert_eq!(r.decisions()[0].action.kind, ActionKind::Investigate);
        c.as_object_mut().unwrap().remove("migration_lead_seconds");
        let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c));
        assert_eq!(r.decisions()[0].urgency, Urgency::Undetermined);
        assert!(r.decisions()[0]
            .missing_inputs
            .contains(&"migration_lead_seconds".into()));
    }
    for at in ["2026-11-18T00:00:00Z", "2026-11-17T00:00:00Z"] {
        let mut c = ctx::ordinary_signing();
        c["deadline"] = ctx::claim(
            json!({"kind":"at","at":at,"reference_id":"org-deadline","scope_reason":"contractual_requirement"}),
        );
        c["migration_lead_seconds"] = ctx::claim(json!(0));
        assert_eq!(
            assessed(&rsa(), InputFormat::InventoryV1, Some(c)).decisions()[0].urgency,
            Urgency::Urgent
        );
    }
    let mut c = ctx::ordinary_signing();
    c["support"] = ctx::support();
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c));
    result(
        &r,
        Urgency::Plan,
        Information::Sufficient,
        Readiness::Ready,
        ActionKind::Plan,
        "known_concern_plan",
    );
}
#[test]
fn a05_a10_a11_tls_roles_partial_unknowns_and_failed_verification_do_not_merge() {
    let claims = json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment"))});
    let r = assessed(&tls(), InputFormat::TlsV1, Some(claims.clone()));
    result(
        &r,
        Urgency::Monitor,
        Information::Sufficient,
        Readiness::NotApplicable,
        ActionKind::Monitor,
        "bounded_role_no_known_concern",
    );
    assert_eq!(r.decisions().len(), 3);
    assert!(r.decisions()[1..]
        .iter()
        .all(|d| d.urgency == Urgency::Undetermined));
    assert!(r.targets()[1..]
        .iter()
        .all(|t| t.technical.algorithm.quantum
            == pqc_scan_core::assessment::QuantumExposure::Vulnerable));
    let mut v = tls();
    v["outcome"] = "partial".into();
    v["diagnostics"] = json!(["certificate_parse_incomplete"]);
    v["details"]["certificate_collection"]["status"] = "partial".into();
    v["details"]["peer_certificates"][0]["attribute_status"] = "partial".into();
    let r = assessed(&v, InputFormat::TlsV1, Some(claims.clone()));
    result(
        &r,
        Urgency::Monitor,
        Information::Sufficient,
        Readiness::NotApplicable,
        ActionKind::Monitor,
        "bounded_role_no_known_concern",
    );
    v["details"]["negotiated"]["key_exchange_group"] = Value::Null;
    v["details"]["negotiated"]["group_classification"] = "unknown".into();
    v["diagnostics"] = json!(["group_unavailable", "certificate_parse_incomplete"]);
    assert_eq!(
        assessed(&v, InputFormat::TlsV1, Some(claims)).decisions()[0].urgency,
        Urgency::Undetermined
    );
    let mut v = failed_tls();
    v["details"]["peer_certificates"][0]["signature_oid"] = "1.2.840.113549.1.1.4".into();
    let r = assessed(&v, InputFormat::TlsV1, None);
    assert_eq!(r.decisions()[1].urgency, Urgency::Urgent);
    assert_eq!(r.decisions()[1].reason_codes[0], "known_classical_concern");
    assert_eq!(r.decisions()[1].action.kind, ActionKind::Investigate);
}
#[test]
fn a15_stale_due_claim_is_urgent_investigation_and_never_current_authority() {
    let mut c = ctx::ordinary_signing();
    let mut deadline = ctx::claim(
        json!({"kind":"at","at":"2026-11-16T00:00:00Z","reference_id":"org-deadline","scope_reason":"organization_policy"}),
    );
    deadline["provenance"]["source_kind"] = "external_review_claim".into();
    deadline["provenance"]["asserted_at"] = "2026-11-16T00:00:00Z".into();
    deadline["provenance"]["valid_until"] = "2026-11-17T00:00:00Z".into();
    c["deadline"] = deadline;
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c));
    result(
        &r,
        Urgency::Urgent,
        Information::Incomplete,
        Readiness::Unknown,
        ActionKind::Investigate,
        "stale_urgent_claim_requires_refresh",
    );
    let claim = r.claims().iter().find(|c| c.field == "deadline").unwrap();
    assert_eq!(
        claim.freshness,
        pqc_scan_core::migration::ClaimFreshness::Stale
    );
    assert_eq!(claim.provenance.source_kind(), "external_review_claim");
    assert!(claim.active_value().is_none());
}
#[test]
fn a07_purpose_conflict_and_purpose_specific_readiness_prevent_kem_signature_substitution() {
    let mut c = ctx::critical_signing();
    c["support"] = ctx::support();
    let r = assessed(&inventory("ML-KEM-768"), InputFormat::InventoryV1, Some(c));
    result(
        &r,
        Urgency::Undetermined,
        Information::Conflicting,
        Readiness::Unknown,
        ActionKind::Investigate,
        "insufficient_information",
    );
    assert!(r.candidates().is_empty());
    let mut c = json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment")),"usage_state":ctx::claim(json!("active")),"confidential":ctx::claim(json!(true)),"confidentiality_until":ctx::claim(json!("2036-11-18T00:00:00Z")),"support":ctx::support()});
    c["support"]["jose"] = ctx::claim(json!(false));
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c));
    assert_eq!(r.decisions()[0].readiness, Readiness::Ready);
    assert!(r.candidates()[0]
        .prerequisites
        .iter()
        .all(|p| p.name != "jose" && p.name != "verifiers"));
    assert!(r.claims().iter().all(|c| c.field != "support.jose"));
    let mut c = ctx::critical_signing();
    c["use_case"] = ctx::claim(json!("jwt"));
    c["support"] = ctx::support();
    c["support"].as_object_mut().unwrap().remove("jose");
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c));
    assert_eq!(r.decisions()[0].readiness, Readiness::Unknown);
    assert!(r.decisions()[0]
        .missing_inputs
        .contains(&"support.jose".into()));
    assert!(r
        .candidates()
        .iter()
        .all(|c| c.purpose == pqc_scan_core::assessment::Purpose::Signature));
}
#[test]
fn a12_a13_same_rsa_targets_and_context_reassessment_have_stable_source_but_distinct_decisions() {
    let mut v = rsa();
    let mut other = v["observations"][0].clone();
    other["id"] = native_id("observation", &["system-b"]).into();
    other["occurrence_key"] = native_id("occurrence", &["system-b"]).into();
    v["observations"].as_array_mut().unwrap().push(other);
    let e = ingest(&v, InputFormat::InventoryV1).unwrap();
    let mut c = ctx::context(&e, ctx::critical_signing());
    c["systems"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"system-b"}));
    c["bindings"].as_array_mut().unwrap().push(json!({"target_id":e.targets()[1].id,"system_id":"system-b","claims":ctx::ordinary_signing()}));
    let c = ctx::parse(&c);
    let r = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
    assert_eq!(r.decisions()[0].urgency, Urgency::Urgent);
    assert_eq!(r.decisions()[1].urgency, Urgency::Plan);
    assert_ne!(
        r.decisions()[0].applied_claim_ids,
        r.decisions()[1].applied_claim_ids
    );
    assert_eq!(
        r.targets()[0].source_ref.root_asset_id,
        r.targets()[1].source_ref.root_asset_id
    );
    let ca = ctx::parse(&ctx::context(&e, ctx::critical_signing()));
    let cb = ctx::parse(&ctx::context(&e, ctx::ordinary_signing()));
    let a = evaluate(&e, Some(&ca), ctx::AS_OF).unwrap();
    let b = evaluate(&e, Some(&cb), ctx::AS_OF).unwrap();
    let repeat = evaluate(&e, Some(&ca), ctx::AS_OF).unwrap();
    assert_eq!(
        serde_json::to_value(&a).unwrap(),
        serde_json::to_value(repeat).unwrap()
    );
    assert_eq!(a.targets()[0].id, b.targets()[0].id);
    assert_eq!(a.input().sha256, b.input().sha256);
    assert_ne!(a.decisions()[0].id, b.decisions()[0].id);
    assert_ne!(a.candidates()[0].id, b.candidates()[0].id);
    assert!(a.validate_for(&e, Some(&cb)).is_err());
}
#[test]
fn a16_ssh_offers_and_negotiation_remain_distinct_and_no_user_authentication_is_invented() {
    let a = assessed(&offers(), InputFormat::SshOffersV1, None);
    assert!(a
        .decisions()
        .iter()
        .all(|d| d.urgency == Urgency::Undetermined));
    assert!(a
        .targets()
        .iter()
        .all(|t| t.observation.assurance == "unauthenticated_advertisement"));
    let b = assessed(&kex(), InputFormat::SshKexV1, None);
    assert!(b
        .decisions()
        .iter()
        .all(|d| d.urgency == Urgency::Undetermined));
    assert!(b
        .targets()
        .iter()
        .all(|t| t.observation.assurance == "collector_reported_verified_session"));
    assert_eq!(b.targets()[1].technical.algorithm.identifier, "Ed25519");
}
#[test]
fn a18_not_applicable_preserves_known_concern_and_exact_versions_and_clock_reject() {
    let r = assessed(
        &rsa(),
        InputFormat::InventoryV1,
        Some(json!({"applicability":ctx::claim(json!("not_applicable"))})),
    );
    result(
        &r,
        Urgency::NotApplicable,
        Information::Sufficient,
        Readiness::NotApplicable,
        ActionKind::None,
        "outside_declared_scope",
    );
    assert_eq!(
        r.targets()[0].technical.algorithm.quantum,
        pqc_scan_core::assessment::QuantumExposure::Vulnerable
    );
    let e = ingest(&rsa(), InputFormat::InventoryV1).unwrap();
    assert_eq!(
        evaluate(&e, None, "2026-11-16T00:00:00Z").unwrap_err(),
        Error::InvalidTime
    );
    for selection in [
        VersionSelection {
            policy_version: "unsupported",
            ..Default::default()
        },
        VersionSelection {
            catalog_id: "local-assessment-1",
            ..Default::default()
        },
        VersionSelection {
            model_version: "2",
            ..Default::default()
        },
    ] {
        assert_eq!(
            evaluate_with_versions(&e, None, ctx::AS_OF, selection).unwrap_err(),
            Error::UnsupportedVersion
        );
    }
}

#[test]
fn contradictory_exact_parameters_cannot_enter_favorable_decision_branches() {
    for (id, parameters) in [
        ("AES-128", json!({"key_bits":"64"})),
        (
            "ECDH",
            json!({"key_bits":"384","named_curve_oid":"1.2.840.10045.3.1.7"}),
        ),
    ] {
        let mut v = inventory(id);
        v["assets"][0]["attributes"]["parameters"] = parameters;
        assert_eq!(
            ingest(&v, InputFormat::InventoryV1).unwrap_err(),
            Error::ContradictorySavedFacts
        );
    }
    for (id, parameters) in [
        ("AES-128", json!({"key_bits":"128"})),
        ("AES-192", json!({"key_bits":"192"})),
        ("AES-256", json!({"key_bits":"256"})),
        (
            "ECDH",
            json!({"key_bits":"256","named_curve_oid":"1.2.840.10045.3.1.7"}),
        ),
        (
            "ECDH",
            json!({"key_bits":"384","named_curve_oid":"1.3.132.0.34"}),
        ),
        (
            "ECDH",
            json!({"key_bits":"521","named_curve_oid":"1.3.132.0.35"}),
        ),
    ] {
        let mut v = inventory(id);
        v["assets"][0]["attributes"]["parameters"] = parameters;
        assert!(ingest(&v, InputFormat::InventoryV1).is_ok());
    }
    let mut v = tls();
    v["details"]["peer_certificates"][0]["public_key_oid"] = "1.2.840.10045.2.1".into();
    v["details"]["peer_certificates"][0]["key_bits"] = 384.into();
    v["details"]["peer_certificates"][0]["named_curve_oid"] = "1.2.840.10045.3.1.7".into();
    assert_eq!(
        ingest(&v, InputFormat::TlsV1).unwrap_err(),
        Error::ContradictorySavedFacts
    );
    let mut v = inventory("ECDH");
    v["assets"][0]["attributes"]["parameters"] =
        json!({"key_bits":"384","named_curve_oid":"1.2.3.999"});
    let e = ingest(&v, InputFormat::InventoryV1).unwrap();
    assert!(!e.targets()[0].technical.algorithm.parameters_known);
}

#[test]
fn a07_unknown_use_preserves_concern_and_a08_same_context_clock_changes_only_decision() {
    let r = assessed(
        &rsa(),
        InputFormat::InventoryV1,
        Some(json!({"applicability":ctx::claim(json!("applicable"))})),
    );
    result(
        &r,
        Urgency::Undetermined,
        Information::Incomplete,
        Readiness::Unknown,
        ActionKind::Investigate,
        "insufficient_information",
    );
    assert_eq!(
        r.targets()[0].technical.algorithm.quantum,
        pqc_scan_core::assessment::QuantumExposure::Vulnerable
    );
    assert!(r.candidates().is_empty());
    let e = ingest(&rsa(), InputFormat::InventoryV1).unwrap();
    let mut claims = ctx::ordinary_signing();
    claims["deadline"] = ctx::claim(
        json!({"kind":"at","at":"2026-11-19T00:00:01Z","reference_id":"org-deadline","scope_reason":"organization_policy"}),
    );
    claims["migration_lead_seconds"] = ctx::claim(json!(86400));
    claims["support"] = ctx::support();
    let c = ctx::parse(&ctx::context(&e, claims));
    let before = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
    let at = evaluate(&e, Some(&c), "2026-11-18T00:00:01Z").unwrap();
    let offset = evaluate(&e, Some(&c), "2026-11-18T09:00:00+09:00").unwrap();
    assert_eq!(before.decisions()[0].urgency, Urgency::Plan);
    assert_eq!(at.decisions()[0].urgency, Urgency::Urgent);
    assert_ne!(before.decisions()[0].id, at.decisions()[0].id);
    assert_eq!(before.candidates()[0].id, at.candidates()[0].id);
    assert_eq!(before.targets()[0].id, at.targets()[0].id);
    assert_eq!(
        serde_json::to_value(&before).unwrap(),
        serde_json::to_value(offset).unwrap()
    );
}

#[test]
fn a08_signature_verification_lifetime_is_not_replaced_by_certificate_expiry() {
    let mut v = tls();
    v["details"]["peer_certificates"][0]["not_after_unix"] = json!(1795046400i64);
    let e = ingest(&v, InputFormat::TlsV1).unwrap();
    let mut cv = ctx::context(
        &e,
        json!({"signature_verification_until":ctx::claim(json!("2036-11-18T00:00:00Z"))}),
    );
    cv["bindings"][0]["target_id"] = e.targets()[1].id.clone().into();
    let c = ctx::parse(&cv);
    let r = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
    let d = &r.decisions()[1];
    assert_eq!(
        (d.urgency, d.action.kind),
        (Urgency::Undetermined, ActionKind::Investigate)
    );
    assert!(d
        .applied_claim_ids
        .iter()
        .any(|id| r.claims().iter().any(|c| &c.id == id
            && c.field == "signature_verification_until"
            && c.value == Some(json!("2036-11-18T00:00:00Z")))));
    assert_eq!(r.targets()[1].source_ref.not_after_unix, Some(1795046400));
}

#[test]
fn a05_unknown_and_draft_groups_never_inherit_final_monitor_decisions() {
    for (id, name, classification) in [
        (65535, "unknown", "unknown"),
        (25497, "X25519Kyber768Draft00", "unknown"),
    ] {
        let mut v = tls();
        v["details"]["negotiated"]["key_exchange_group"] = json!({"id":id,"name":name});
        v["details"]["client_profile"]["key_exchange_groups"] = json!([{ "id":id,"name":name }]);
        v["details"]["negotiated"]["group_classification"] = classification.into();
        let r = assessed(
            &v,
            InputFormat::TlsV1,
            Some(
                json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment"))}),
            ),
        );
        result(
            &r,
            Urgency::Undetermined,
            Information::Incomplete,
            Readiness::Unknown,
            ActionKind::Investigate,
            "insufficient_information",
        );
    }
}

#[test]
fn pki_support_is_required_but_hashing_does_not_require_peer_or_protocol() {
    let mut c = ctx::critical_signing();
    c["use_case"] = ctx::claim(json!("pki"));
    c["support"] = ctx::support();
    c["support"]["pki"] = ctx::claim(json!(false));
    let r = assessed(&rsa(), InputFormat::InventoryV1, Some(c));
    assert_eq!(r.decisions()[0].action.blocking_prerequisites, ["pki"]);
    assert_eq!(r.decisions()[0].action.kind, ActionKind::Unblock);
    let r = assessed(
        &inventory("SHA-256"),
        InputFormat::InventoryV1,
        Some(
            json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("hashing")),"usage_state":ctx::claim(json!("active"))}),
        ),
    );
    assert_eq!(r.decisions()[0].urgency, Urgency::Monitor);
    assert!(r.decisions()[0].missing_inputs.is_empty());
}

#[test]
fn saved_certificates_reject_unrepresentable_times_bits_and_reversed_intervals() {
    for (field, value, expected) in [
        (
            "not_before_unix",
            json!(18446744073709551615u64),
            Error::InvalidInput,
        ),
        (
            "not_after_unix",
            json!(18446744073709551615u64),
            Error::InvalidInput,
        ),
        ("key_bits", json!(-1), Error::InvalidInput),
        (
            "not_before_unix",
            json!(2000000000i64),
            Error::ContradictorySavedFacts,
        ),
    ] {
        let mut v = tls();
        v["details"]["peer_certificates"][0][field] = value;
        assert_eq!(ingest(&v, InputFormat::TlsV1).unwrap_err(), expected);
    }
}

#[test]
fn a11_partial_without_negotiation_retains_safe_certificate_concerns() {
    let mut v = failed_tls();
    v["outcome"] = "partial".into();
    let r = assessed(&v, InputFormat::TlsV1, None);
    assert!(r
        .targets()
        .iter()
        .all(|t| t.observation.assurance != "collector_reported_verified_session"));
    assert_eq!(r.decisions()[1].urgency, Urgency::Undetermined);
    assert_eq!(r.decisions()[1].information, Information::Incomplete);
    assert_eq!(r.decisions()[1].action.kind, ActionKind::Investigate);
    assert_eq!(
        r.targets()[1].technical.algorithm.quantum,
        pqc_scan_core::assessment::QuantumExposure::Vulnerable
    );
}

#[test]
fn purpose_conflict_preserves_universal_false_blockers_but_not_jose_for_kem() {
    let mut c = ctx::critical_signing();
    c["support"] = ctx::support();
    c["support"]["jose"] = ctx::claim(json!(false));
    let irrelevant = assessed(
        &inventory("ML-KEM-768"),
        InputFormat::InventoryV1,
        Some(c.clone()),
    );
    assert_eq!(irrelevant.decisions()[0].readiness, Readiness::Unknown);
    assert!(irrelevant.decisions()[0]
        .action
        .blocking_prerequisites
        .is_empty());
    for prerequisite in ["runtime", "provider", "parameters"] {
        let mut c = c.clone();
        c["support"][prerequisite] = ctx::claim(json!(false));
        let r = assessed(&inventory("ML-KEM-768"), InputFormat::InventoryV1, Some(c));
        result(
            &r,
            Urgency::Undetermined,
            Information::Conflicting,
            Readiness::Blocked,
            ActionKind::Investigate,
            "insufficient_information",
        );
        assert_eq!(
            r.decisions()[0].action.blocking_prerequisites,
            [prerequisite]
        );
        assert!(r.candidates().is_empty());
        assert!(!r.decisions()[0]
            .missing_inputs
            .contains(&"support.jose".into()));
        assert!(r.claims().iter().all(|c| c.field != "support.jose"));
    }
}
