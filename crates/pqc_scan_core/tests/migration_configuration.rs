use pqc_scan_core::migration::{evaluate, InputFormat, MigrationContext, Readiness};
use serde_json::{json, Value};
#[path = "support/migration/context.rs"]
mod ctx;
#[path = "support/migration/mod.rs"]
#[allow(dead_code)]
mod fixture;
fn configuration() -> Value {
    ctx::claim(json!({"use_case":"signature","algorithm":"ML-DSA","parameters":"ML-DSA-65"}))
}
fn schema(name: &str) -> jsonschema::Validator {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas")
        .join(name);
    let value: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    jsonschema::validator_for(&value).unwrap()
}
#[test]
fn configuration_roundtrip_and_evidence_keep_exact_operator_choice_without_verifying_it() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut claims = ctx::critical_signing();
    claims["support"] = ctx::support();
    claims["migration_target"] = configuration();
    let body = ctx::context(&e, claims);
    assert!(schema("migration-context-v1.schema.json").is_valid(&body));
    let c = ctx::parse(&body);
    let exported: Value = serde_json::from_slice(&c.to_bytes().unwrap()).unwrap();
    assert_eq!(
        exported["bindings"][0]["claims"]["migration_target"]["value"]["parameters"],
        "ML-DSA-65"
    );
    let result = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
    result.validate_for(&e, Some(&c)).unwrap();
    assert_eq!(result.decisions()[0].readiness, Readiness::Ready);
    let json = serde_json::to_value(&result).unwrap();
    assert!(schema("migration-assessment-v1.schema.json").is_valid(&json));
    assert!(json["claims"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c["field"] == "migration_target" && c["value"]["parameters"] == "ML-DSA-65"));
    // Selection remains an unverified declaration, not a validated parameter recommendation.
    assert_eq!(
        json["candidates"][0]["parameter_selection"],
        "explicit_parameter_selection_required"
    );
}
#[test]
fn unknown_or_stale_configuration_never_establishes_specific_known_support() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    for target in [ctx::unknown(), {
        let mut value = configuration();
        value["provenance"]["valid_until"] = json!("2026-11-17T12:00:00Z");
        value
    }] {
        let mut claims = ctx::critical_signing();
        claims["support"] = ctx::support();
        claims["migration_target"] = target;
        let c = ctx::parse(&ctx::context(&e, claims));
        let result = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
        assert_eq!(result.decisions()[0].readiness, Readiness::Unknown);
        assert!(result.decisions()[0]
            .missing_inputs
            .contains(&"support.runtime".to_owned()));
    }
}
#[test]
fn mismatched_purpose_and_inherited_generic_support_are_rejected_not_rebound() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut claims = ctx::critical_signing();
    claims["migration_target"] = configuration();
    let mut body = ctx::context(&e, claims);
    body["organization_defaults"] = json!({"support":ctx::support()});
    assert!(ctx::parse(&body).bind(&e, ctx::AS_OF).is_err());
    body.as_object_mut()
        .unwrap()
        .remove("organization_defaults");
    body["bindings"][0]["claims"]["migration_target"]["value"]["use_case"] = json!("jwt");
    assert!(ctx::parse(&body).bind(&e, ctx::AS_OF).is_err());
}
#[test]
fn old_generic_contexts_keep_their_meaning_and_invalid_configurations_are_rejected() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut claims = ctx::critical_signing();
    claims["support"] = ctx::support();
    let old = ctx::parse(&ctx::context(&e, claims.clone()));
    assert_eq!(
        evaluate(&e, Some(&old), ctx::AS_OF).unwrap().decisions()[0].readiness,
        Readiness::Ready
    );
    for invalid in [
        json!({"use_case":"signature","algorithm":"ML-KEM","parameters":"ML-KEM-768"}),
        json!({"use_case":"signature","algorithm":"ML-DSA","parameters":" "}),
        json!({"use_case":"signature","algorithm":"ML-DSA","parameters":"x\ny"}),
        json!({"use_case":"signature","algorithm":"ML-DSA","parameters":"x".repeat(81)}),
    ] {
        claims["migration_target"] = ctx::claim(invalid);
        assert!(MigrationContext::from_bytes(
            &serde_json::to_vec(&ctx::context(&e, claims.clone())).unwrap()
        )
        .is_err());
    }
}

#[test]
fn explicit_unknown_configuration_override_suppresses_inherited_known_support() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let mut defaults = ctx::critical_signing();
    defaults["support"] = ctx::support();
    defaults["migration_target"] = configuration();
    let mut body = ctx::context(&e, json!({"migration_target":ctx::unknown()}));
    body["organization_defaults"] = defaults;
    let c = ctx::parse(&body);
    let result = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
    assert_eq!(result.decisions()[0].readiness, Readiness::Unknown);
    assert!(result.decisions()[0]
        .missing_inputs
        .contains(&"support.runtime".to_owned()));
}

#[test]
fn planning_goal_month_is_lossless_provenanced_and_does_not_change_policy() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let original = ctx::critical_signing();
    let baseline = ctx::parse(&ctx::context(&e, original.clone()));
    let before = serde_json::to_value(evaluate(&e, Some(&baseline), ctx::AS_OF).unwrap()).unwrap();
    for month in [json!("2030-09"), json!("0001-01"), json!("9999-12")] {
        let mut claims = original.clone();
        claims["migration_goal_month"] = ctx::claim(month.clone());
        let body = ctx::context(&e, claims);
        assert!(schema("migration-context-v1.schema.json").is_valid(&body));
        let c = ctx::parse(&body);
        let saved: Value = serde_json::from_slice(&c.to_bytes().unwrap()).unwrap();
        assert_eq!(
            saved["bindings"][0]["claims"]["migration_goal_month"]["value"],
            month
        );
        let report = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
        report.validate_for(&e, Some(&c)).unwrap();
        let after = serde_json::to_value(&report).unwrap();
        assert!(schema("migration-assessment-v1.schema.json").is_valid(&after));
        for key in [
            "urgency",
            "information",
            "readiness",
            "reason_codes",
            "missing_inputs",
        ] {
            assert_eq!(
                before["decisions"][0][key], after["decisions"][0][key],
                "planning month changed {key}"
            );
        }
        // IDs bind the exact context digest, so a planning edit changes identities.
        for key in ["kind", "blocking_prerequisites", "validation_steps"] {
            assert_eq!(
                before["decisions"][0]["action"][key],
                after["decisions"][0]["action"][key]
            );
        }
        let claim = after["claims"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["field"] == "migration_goal_month")
            .unwrap();
        assert_eq!(claim["value"], month);
        assert_eq!(claim["provenance"]["actor_id"], "owner");
    }
    for invalid in [
        json!("0000-01"),
        json!("2030-00"),
        json!("2030-13"),
        json!("2030-9"),
        json!("2030-09-01"),
        json!("２０３０-09"),
        json!(203009),
    ] {
        let mut claims = original.clone();
        claims["migration_goal_month"] = ctx::claim(invalid);
        let body = ctx::context(&e, claims);
        assert!(!schema("migration-context-v1.schema.json").is_valid(&body));
        assert!(MigrationContext::from_bytes(&serde_json::to_vec(&body).unwrap()).is_err());
    }
    let mut claims = original;
    claims["migration_goal_month"] = ctx::unknown();
    let c = ctx::parse(&ctx::context(&e, claims));
    assert!(
        serde_json::to_value(evaluate(&e, Some(&c), ctx::AS_OF).unwrap()).unwrap()["claims"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["field"] == "migration_goal_month" && c["state"] == "unknown")
    );
}

#[test]
fn planning_goal_freshness_never_enters_any_decision_branch() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    for mut claims in [
        ctx::critical_signing(),
        ctx::ordinary_signing(),
        json!({"applicability":ctx::claim(json!("not_applicable"))}),
    ] {
        claims["support"] = ctx::support();
        let baseline = ctx::parse(&ctx::context(&e, claims.clone()));
        let before =
            serde_json::to_value(evaluate(&e, Some(&baseline), ctx::AS_OF).unwrap()).unwrap();
        for state in [
            "active",
            "unknown",
            "stale",
            "not_yet_effective",
            "no_expiry",
        ] {
            let mut goal = if state == "unknown" {
                ctx::unknown()
            } else {
                ctx::claim(json!("2030-09"))
            };
            match state {
                "stale" => goal["provenance"]["valid_until"] = json!("2026-11-17T12:00:00Z"),
                "not_yet_effective" => {
                    goal["provenance"]["asserted_at"] = json!("2026-11-19T00:00:00Z")
                }
                "no_expiry" => {
                    goal["provenance"]
                        .as_object_mut()
                        .unwrap()
                        .remove("valid_until");
                }
                _ => {}
            }
            claims["migration_goal_month"] = goal;
            let c = ctx::parse(&ctx::context(&e, claims.clone()));
            let result = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
            result.validate_for(&e, Some(&c)).unwrap();
            let after = serde_json::to_value(&result).unwrap();
            for key in [
                "urgency",
                "information",
                "readiness",
                "reason_codes",
                "missing_inputs",
                "source_ids",
            ] {
                assert_eq!(
                    before["decisions"][0][key], after["decisions"][0][key],
                    "{state} changed {key}"
                );
            }
            for key in ["kind", "blocking_prerequisites", "validation_steps"] {
                assert_eq!(
                    before["decisions"][0]["action"][key], after["decisions"][0]["action"][key],
                    "{state} changed action.{key}"
                );
            }
            assert!(after["claims"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["field"] == "migration_goal_month"));
            claims
                .as_object_mut()
                .unwrap()
                .remove("migration_goal_month");
        }
    }
}
