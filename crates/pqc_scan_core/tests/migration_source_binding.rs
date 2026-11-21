#[path = "support/migration/context.rs"]
mod ctx;
#[path = "support/migration/mod.rs"]
#[allow(dead_code)]
mod fixture;
use pqc_scan_core::{identity::native_id, migration::*};
use serde_json::{json, Value};

fn source_pair(scoped: bool, algorithm: bool) -> Value {
    let mut v = fixture::inventory("RS256");
    v["repository_scope"] = if scoped {
        json!("source-test")
    } else {
        Value::Null
    };
    let root = v["assets"][0]["id"].clone();
    if !algorithm {
        v["assets"] = json!([]);
    }
    let template = v["observations"][0].clone();
    v["observations"] = json!([]);
    let anchor = native_id("anchor", &["same-source-rule", "RS256"]);
    for i in 0..2 {
        let usage = native_id("asset", &["usage", &i.to_string()]);
        let mut o = template.clone();
        o["id"] = json!(native_id("observation", &["source", &i.to_string()]));
        o["occurrence_key"] = if scoped {
            json!(native_id("occurrence", &["source", &i.to_string()]))
        } else {
            Value::Null
        };
        o["identity_confidence"] = json!("ambiguous_duplicate_anchor");
        o["usage"] = json!("referenced");
        o["target"] = json!({"path":"auth/token.ts","line":i+10,"column":1,"column_kind":"unicode_code_points","byte_start":i*100+10,"byte_end":i*100+15,"object_index":null});
        o["asset_ids"] = if algorithm {
            json!([root, usage])
        } else {
            json!([usage])
        };
        v["observations"].as_array_mut().unwrap().push(o);
        v["assets"].as_array_mut().unwrap().push(json!({"id":usage,"kind":"usage","identity_basis":if scoped {"scoped_weak_source_anchor"} else {"run_local_only"},"attributes":{"path":"auth/token.ts","method":"ast_pattern","local_ordinal":i,"ambiguous":true,"anchor":anchor},"relations":if algorithm {json!([{"relationship":"references","target":root}])} else {json!([])},"uncertainty":["static_reference_not_active_use"]}));
    }
    v
}
fn read(v: &Value) -> SavedEvidence {
    fixture::ingest(v, InputFormat::InventoryV1).unwrap()
}
fn claims() -> ContextClaims {
    ContextClaims::from_bytes(&serde_json::to_vec(&json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("jwt")),"usage_state":ctx::claim(json!("active"))})).unwrap()).unwrap()
}
fn one(_e: &SavedEvidence, target_index: usize) -> ContextSelection {
    ContextSelection {
        target_index,
        system_id: format!("system-{target_index}"),
        claims: claims(),
    }
}
#[test]
fn exact_saved_source_records_can_be_explicitly_bound_without_cross_input_transfer() {
    for scoped in [false, true] {
        let v = source_pair(scoped, true);
        let native: pqc_scan_core::NativeScan = serde_json::from_value(v.clone()).unwrap();
        let before = serde_json::to_value(&native).unwrap();
        let e = read(&v);
        assert!(e.targets().iter().all(Target::is_bindable));
        let ids: Vec<_> = e.targets().iter().map(|t| t.id.clone()).collect();
        let c = MigrationContext::from_bytes(
            &generate_context(&e, "source-test", &[one(&e, 0), one(&e, 1)], ctx::AS_OF).unwrap(),
        )
        .unwrap();
        let bound = c.bind(&e, ctx::AS_OF).unwrap();
        for (i, t) in e.targets().iter().enumerate() {
            assert_eq!(
                bound.target(&t.id).unwrap().system_id,
                format!("system-{i}")
            );
        }
        assert_eq!(
            before,
            serde_json::to_value(native).unwrap(),
            "binding never rewrites native IDs or ambiguity flags"
        );
        assert_eq!(
            ids,
            evaluate(&e, Some(&c), ctx::AS_OF)
                .unwrap()
                .targets()
                .iter()
                .map(|t| t.id.clone())
                .collect::<Vec<_>>()
        );
        let one_context = MigrationContext::from_bytes(
            &generate_context(&e, "source-test", &[one(&e, 0)], ctx::AS_OF).unwrap(),
        )
        .unwrap();
        assert!(one_context
            .bind(&e, ctx::AS_OF)
            .unwrap()
            .target(&e.targets()[1].id)
            .is_none());
        let pretty = SavedEvidence::from_bytes(
            &serde_json::to_vec_pretty(&v).unwrap(),
            InputFormat::InventoryV1,
        )
        .unwrap();
        assert_eq!(
            c.bind(&pretty, ctx::AS_OF).unwrap_err(),
            Error::InputBindingMismatch
        );
        assert!(
            generate_context(&e, "source-test", &[one(&e, 0), one(&e, 0)], ctx::AS_OF).is_err()
        );
    }
}
#[test]
fn reference_only_role_accepts_declared_purpose_but_retains_unknown_technical_algorithm() {
    let e = read(&source_pair(false, false));
    assert!(e.targets().iter().all(Target::is_bindable));
    assert!(e
        .targets()
        .iter()
        .all(|t| t.source_ref.role == "reference-only"
            && t.technical.algorithm.identifier == "unknown"));
    let c = MigrationContext::from_bytes(
        &generate_context(&e, "source-test", &[one(&e, 0)], ctx::AS_OF).unwrap(),
    )
    .unwrap();
    let r = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
    assert!(r
        .decisions()
        .iter()
        .all(|d| d.urgency == Urgency::Undetermined));
    assert!(r
        .targets()
        .iter()
        .all(|t| t.technical.algorithm.identifier == "unknown"));
}
#[test]
fn collisions_and_incomplete_neighbors_never_become_distinguishable_after_filtering() {
    for case in 0..18 {
        let mut v = source_pair(true, true);
        match case {
            0 => v["observations"][1]["target"]["byte_start"] = json!(10),
            1 => {
                v["observations"][1]["target"]["byte_start"] = json!(10);
                v["observations"][1]["target"]["byte_end"] = json!(15);
            }
            2 => v["observations"][1]["target"]["byte_start"] = Value::Null,
            3 => v["observations"][1]["target"]["column"] = Value::Null,
            4 => v["observations"][1]["target"]["line"] = Value::Null,
            5 => v["observations"][1]["target"]["column_kind"] = json!("unknown"),
            6 => v["observations"][1]["target"]["line"] = json!(10),
            7 => v["observations"][1]["target"]["path"] = json!("auth/other.ts"),
            8 => v["assets"][2]["attributes"]["method"] = json!("lexical_match"),
            9 => v["assets"][2]["identity_basis"] = json!("unknown"),
            10 => v["assets"][2]["relations"] = json!([]),
            11 => {
                v["observations"][1]["asset_ids"][1] = v["observations"][0]["asset_ids"][1].clone()
            }
            12 => v["assets"][0]["attributes"]["ambiguous"] = json!(true),
            13 => {
                v["observations"][1]["occurrence_key"] =
                    v["observations"][0]["occurrence_key"].clone()
            }
            14 => v["observations"][1]["identity_confidence"] = json!("future_unknown_confidence"),
            15 => {
                v["observations"][1]["method"] = json!("dependency_report");
                v["assets"][2]["attributes"]["method"] = json!("dependency_report");
            }
            16 => {
                v["observations"][1]["target"]["byte_start"] = json!(12);
                v["observations"][1]["target"]["byte_end"] = json!(18);
            }
            _ => {
                v["observations"].as_array_mut().unwrap().pop();
            }
        }
        let e = read(&v);
        assert!(e.targets().iter().all(|t| !t.is_bindable()), "case {case}");
        assert_eq!(
            generate_context(&e, "source-test", &[one(&e, 0)], ctx::AS_OF).unwrap_err(),
            Error::AmbiguousContextBinding,
            "case {case}"
        );
    }
}
#[test]
fn an_occurrence_collision_in_a_neighbor_rejects_the_entire_source_group() {
    let mut v = source_pair(true, true);
    let mut third = v["observations"][1].clone();
    third["id"] = json!(native_id("observation", &["third-occurrence"]));
    third["occurrence_key"] = json!(native_id("occurrence", &["third-occurrence"]));
    third["target"]["line"] = json!(30);
    third["target"]["byte_start"] = json!(300);
    third["target"]["byte_end"] = json!(305);
    let usage = native_id("asset", &["third-occurrence"]);
    third["asset_ids"][1] = json!(usage);
    let mut asset = v["assets"][2].clone();
    asset["id"] = json!(usage);
    v["observations"].as_array_mut().unwrap().push(third);
    v["assets"].as_array_mut().unwrap().push(asset);
    assert!(read(&v).targets().iter().all(Target::is_bindable));
    for external in [false, true] {
        let mut collision = v.clone();
        collision["observations"][2]["occurrence_key"] =
            collision["observations"][1]["occurrence_key"].clone();
        if external {
            collision["observations"][2]["target"]["path"] = json!("other/token.ts");
            collision["assets"][3]["attributes"]["path"] = json!("other/token.ts");
        }
        let e = read(&collision);
        assert!(
            e.targets().iter().all(|t| !t.is_bindable()),
            "external {external}"
        );
        assert_eq!(
            generate_context(&e, "source-test", &[one(&e, 0)], ctx::AS_OF).unwrap_err(),
            Error::AmbiguousContextBinding
        );
    }
}
#[test]
fn unsupported_algorithms_and_unresolved_component_roles_remain_unbindable() {
    for role in [false, true] {
        let mut v = source_pair(false, true);
        if role {
            v["assets"][0]["kind"] = json!("public_key");
            v["assets"][0]["attributes"] =
                json!({"spki_sha256":"a".repeat(64),"key_bits":null,"named_curve_oid":null});
        } else {
            v["assets"][0]["attributes"]["identifier"] = json!("UNSUPPORTED");
        }
        let e = read(&v);
        assert!(e.targets().iter().all(|t| !t.is_bindable()));
        assert!(e
            .targets()
            .iter()
            .all(|t| t.limitations.iter().any(|l| l == "unresolved_role")));
    }
}
#[test]
fn a_neighbor_without_group_metadata_does_not_disappear_from_uniqueness_checks() {
    for field in ["anchor", "path", "usage_kind"] {
        let mut v = source_pair(false, true);
        let mut third = v["observations"][1].clone();
        third["id"] = json!(native_id("observation", &["third"]));
        third["target"]["line"] = json!(30);
        third["target"]["byte_start"] = json!(300);
        third["target"]["byte_end"] = json!(305);
        let usage = native_id("asset", &["third"]);
        third["asset_ids"][1] = json!(usage);
        let mut asset = v["assets"][2].clone();
        asset["id"] = json!(usage);
        if field == "path" {
            asset["attributes"]["path"] = Value::Null;
        } else if field == "usage_kind" {
            asset["kind"] = json!("future_unknown_kind");
        } else {
            asset["attributes"].as_object_mut().unwrap().remove(field);
        }
        v["observations"].as_array_mut().unwrap().push(third);
        v["assets"].as_array_mut().unwrap().push(asset);
        if field == "anchor" {
            assert_eq!(
                fixture::ingest(&v, InputFormat::InventoryV1).unwrap_err(),
                Error::InvalidInput
            );
            continue;
        }
        let e = read(&v);
        assert!(e.targets().iter().all(|t| !t.is_bindable()), "{field}");
    }
}
#[test]
fn source_selection_does_not_promote_lexical_or_configured_evidence_assurance() {
    for (method, usage) in [
        ("lexical_match", "referenced"),
        ("configuration_match", "configured"),
    ] {
        let mut v = source_pair(false, true);
        for i in 0..2 {
            v["observations"][i]["method"] = json!(method);
            v["observations"][i]["assurance"] = json!("lexical");
            v["observations"][i]["usage"] = json!(usage);
            v["assets"][i + 1]["attributes"]["method"] = json!(method);
        }
        let e = read(&v);
        assert!(e.targets().iter().all(Target::is_bindable));
        let c = MigrationContext::from_bytes(
            &generate_context(&e, "source-test", &[one(&e, 0)], ctx::AS_OF).unwrap(),
        )
        .unwrap();
        let r = evaluate(&e, Some(&c), ctx::AS_OF).unwrap();
        assert!(r
            .targets()
            .iter()
            .all(|t| t.observation.assurance == "lexical" && t.observation.usage == usage));
    }
}
#[test]
fn a_pathless_ambiguous_neighbor_blocks_the_surviving_anchor_group() {
    for unknown_kind in [false, true] {
        let mut v = source_pair(false, true);
        let mut third = v["observations"][1].clone();
        third["id"] = json!(native_id("observation", &["pathless"]));
        third["target"]["path"] = Value::Null;
        let usage = native_id("asset", &["pathless"]);
        third["asset_ids"][1] = json!(usage);
        let mut asset = v["assets"][2].clone();
        asset["id"] = json!(usage);
        asset["attributes"]["path"] = Value::Null;
        if unknown_kind {
            asset["kind"] = json!("future_unknown_kind");
        }
        v["observations"].as_array_mut().unwrap().push(third);
        v["assets"].as_array_mut().unwrap().push(asset);
        let e = read(&v);
        assert!(e.targets().iter().all(|t| !t.is_bindable()));
        assert_eq!(
            generate_context(&e, "source-test", &[one(&e, 0)], ctx::AS_OF).unwrap_err(),
            Error::AmbiguousContextBinding
        );
    }
}
