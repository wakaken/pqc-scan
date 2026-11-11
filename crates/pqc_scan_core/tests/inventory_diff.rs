use pqc_scan_core::diff::{compare, parse_inventory};
use pqc_scan_core::identity::native_id;
use pqc_scan_core::NativeScan;
use serde_json::{json, Value};

fn fixture(path: &str, line: usize, run: &str) -> Value {
    let algorithm = native_id("asset", &["algorithm", "ML-KEM-768"]);
    let occurrence = native_id("occurrence", &["team/project", path, "anchor", "0"]);
    let usage = native_id("asset", &["usage", &occurrence]);
    let observation = native_id("observation", &[run, path]);
    json!({
        "schema_version":1,"identity_scheme":"pqc-scan-length-prefixed-sha256-v1",
        "scanner_version":"0.1.0","catalog_version":"native-exact-identifiers-1",
        "profile":"static-native-v1","rules_digest":"a".repeat(64),
        "run":{"id":native_id("scan", &[run]),"attempt_id":"static-1","generated_at":run},
        "repository_scope":"team/project","scope":{"max_file_size_bytes":2097152},
        "coverage":{"status":"complete_within_scope","selected_files":1},
        "diagnostics":[],"native_completeness":"not_exhaustive",
        "assets":[
            {"id":algorithm,"kind":"algorithm","identity_basis":"normalized_identifier_and_reported_parameters",
             "attributes":{"namespace":"named","identifier":"ML-KEM-768","purpose":"reference","parameters":{}},
             "relations":[],"uncertainty":[]},
            {"id":usage,"kind":"usage","identity_basis":"scoped_weak_source_anchor",
             "attributes":{"path":path,"method":"lexical_match","local_ordinal":0,"ambiguous":false,"anchor":"semantic-anchor"},
             "relations":[{"relationship":"references","target":algorithm}],"uncertainty":["static_reference_not_active_use"]}
        ],
        "observations":[{"id":observation,"occurrence_key":occurrence,"identity_confidence":"weak_structural_anchor",
            "asset_ids":[algorithm,usage],"target":{"path":path,"line":line,"column":1,"column_kind":"unicode_code_points",
            "byte_start":null,"byte_end":null,"object_index":null},"method":"lexical_match","producer":"regex_detector",
            "assurance":"lexical","usage":"referenced","rule_id":"TEST_RULE",
            "evidence":{"marker_kind":null,"capture_status":"typed_capture"},"uncertainty":[]}],
        "legacy_associations":[],"unassociated_observation_ids":[observation],"legacy_loss_records":[],
        "assessment_status":"not_evaluated","assessments":[],"export_loss_notes":[]
    })
}
fn inventory(value: &Value) -> NativeScan {
    parse_inventory(&serde_json::to_vec(value).unwrap()).unwrap()
}
fn report(before: &Value, after: &Value) -> Value {
    serde_json::to_value(compare(&inventory(before), &inventory(after), "team/project").unwrap())
        .unwrap()
}

#[test]
fn line_and_run_changes_preserve_exact_occurrence_correspondence() {
    let before = fixture("src/app.rs", 1, "first");
    let after = fixture("src/app.rs", 99, "second");
    let result = report(&before, &after);
    assert_eq!(result["schema_version"], 1);
    assert_eq!(result["occurrences"][0]["status"], "unchanged");
    assert!(result["assets"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["status"] == "unchanged"));
    assert_eq!(result["drift"], json!([]));
    let golden: Value =
        serde_json::from_str(include_str!("fixtures/diff/unchanged.v1.json")).unwrap();
    assert_eq!(result, golden);
}

#[test]
fn absent_after_is_unresolved_even_with_complete_legacy_coverage() {
    let before = fixture("src/app.rs", 1, "first");
    let mut after = fixture("src/app.rs", 1, "second");
    after["assets"] = json!([]);
    after["observations"] = json!([]);
    after["unassociated_observation_ids"] = json!([]);
    let result = report(&before, &after);
    assert_eq!(result["occurrences"][0]["status"], "unresolved");
    assert!(result["assets"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["status"] == "unresolved"));
    assert!(serde_json::to_string(&result)
        .unwrap()
        .contains("target_inspection_not_proven"));
    assert!(!serde_json::to_string(&result)
        .unwrap()
        .contains("resolved_risk"));
}

#[test]
fn unique_move_is_only_a_heuristic_candidate() {
    let result = report(&fixture("old.rs", 1, "a"), &fixture("new.rs", 1, "b"));
    assert_eq!(result["move_candidates"].as_array().unwrap().len(), 1);
    assert_eq!(result["move_candidates"][0]["confidence"], "heuristic_only");
    let statuses: Vec<_> = result["occurrences"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["status"].as_str().unwrap())
        .collect();
    assert!(statuses.contains(&"added") && statuses.contains(&"unresolved"));
}

#[test]
fn ambiguous_duplicate_anchor_never_corresponds_or_moves() {
    let mut before = fixture("old.rs", 1, "a");
    before["observations"][0]["identity_confidence"] = json!("ambiguous_duplicate_anchor");
    before["assets"][1]["attributes"]["ambiguous"] = json!(true);
    let mut after = before.clone();
    after["run"]["generated_at"] = json!("later");
    let same = report(&before, &after);
    assert_eq!(same["occurrences"][0]["status"], "unresolved");
    let moved = report(&before, &fixture("new.rs", 1, "b"));
    assert_eq!(moved["move_candidates"], json!([]));
}

#[test]
fn changed_observation_facts_and_metadata_drift_are_explicit() {
    let before = fixture("app.rs", 1, "a");
    let mut after = before.clone();
    after["observations"][0]["assurance"] = json!("parsed");
    after["rules_digest"] = json!("b".repeat(64));
    after["scope"]["max_file_size_bytes"] = json!(1024);
    let result = report(&before, &after);
    assert_eq!(result["occurrences"][0]["status"], "changed");
    assert_eq!(
        result["drift"],
        json!(["rules_digest_changed", "inspection_scope_changed"])
    );
}

#[test]
fn changed_coverage_is_disclosed_even_when_records_match() {
    let before = fixture("app.rs", 1, "a");
    let mut after = before.clone();
    after["coverage"]["status"] = json!("partial");
    let result = report(&before, &after);
    assert_eq!(result["occurrences"][0]["status"], "unchanged");
    assert_eq!(result["drift"], json!(["inspection_coverage_changed"]));
}

#[test]
fn scope_schema_and_reference_errors_are_rejected_without_echoing_input() {
    let original = fixture("app.rs", 1, "a");
    for (field, value) in [
        ("schema_version", json!(2)),
        ("identity_scheme", json!("wrong")),
        ("repository_scope", json!(null)),
        ("repository_scope", json!("other/project")),
    ] {
        let mut after = original.clone();
        after[field] = value;
        let result = parse_inventory(&serde_json::to_vec(&after).unwrap())
            .and_then(|n| compare(&inventory(&original), &n, "team/project"));
        assert!(result.is_err(), "field {field}");
    }
    let mut forged = original.clone();
    forged["observations"][0]["asset_ids"][0] = json!("SECRET_INPUT_CANARY");
    let err = parse_inventory(&serde_json::to_vec(&forged).unwrap())
        .unwrap_err()
        .to_string();
    assert!(!err.contains("SECRET_INPUT_CANARY"));
    let duplicate = forged["assets"][0].clone();
    forged["assets"].as_array_mut().unwrap().push(duplicate);
    assert!(parse_inventory(&serde_json::to_vec(&forged).unwrap()).is_err());
}

#[test]
fn unknown_assurance_stays_unresolved_and_future_fields_are_tolerated() {
    let mut before = fixture("app.rs", 1, "a");
    before["observations"][0]["identity_confidence"] = json!("future_confidence");
    before["future_field"] = json!({"ignored":true});
    let result = report(&before, &before);
    assert_eq!(result["occurrences"][0]["status"], "unresolved");
}

#[test]
fn comparison_order_is_independent_of_input_array_order() {
    let before = fixture("app.rs", 1, "a");
    let mut reversed = before.clone();
    reversed["assets"].as_array_mut().unwrap().reverse();
    assert_eq!(report(&before, &before), report(&reversed, &reversed));
}

#[test]
fn malformed_and_oversized_json_are_rejected() {
    assert!(parse_inventory(b"{malformed").is_err());
    assert!(parse_inventory(&vec![b' '; 16 * 1024 * 1024 + 1]).is_err());
}

#[test]
fn multiple_matching_move_anchors_abstain_instead_of_guessing() {
    let mut before = fixture("old-a.rs", 1, "a");
    let second = fixture("old-b.rs", 1, "b");
    before["assets"]
        .as_array_mut()
        .unwrap()
        .push(second["assets"][1].clone());
    before["observations"]
        .as_array_mut()
        .unwrap()
        .push(second["observations"][0].clone());
    before["unassociated_observation_ids"]
        .as_array_mut()
        .unwrap()
        .push(second["unassociated_observation_ids"][0].clone());
    let result = report(&before, &fixture("new.rs", 1, "c"));
    assert_eq!(result["move_candidates"], json!([]));
}

#[test]
fn a_matching_anchor_elsewhere_prevents_a_unique_move_claim() {
    let stable = fixture("stable.rs", 1, "stable");
    let add_stable = |value: &mut Value| {
        value["assets"]
            .as_array_mut()
            .unwrap()
            .push(stable["assets"][1].clone());
        value["observations"]
            .as_array_mut()
            .unwrap()
            .push(stable["observations"][0].clone());
        value["unassociated_observation_ids"]
            .as_array_mut()
            .unwrap()
            .push(stable["unassociated_observation_ids"][0].clone());
    };
    let mut before = fixture("old.rs", 1, "a");
    let mut after = fixture("new.rs", 1, "b");
    add_stable(&mut before);
    add_stable(&mut after);
    assert_eq!(report(&before, &after)["move_candidates"], json!([]));
}

#[test]
fn run_local_observations_remain_visible_without_cross_run_identity() {
    let mut before = fixture("app.rs", 1, "a");
    before["observations"][0]["occurrence_key"] = Value::Null;
    before["observations"][0]["identity_confidence"] = json!("run_local_only");
    let result = report(&before, &before);
    assert_eq!(result["occurrences"].as_array().unwrap().len(), 2);
    assert!(result["occurrences"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| row["status"] == "unresolved"));
}

#[test]
fn duplicate_occurrence_keys_and_unsafe_paths_are_rejected() {
    let mut before = fixture("app.rs", 1, "a");
    let mut second = before["observations"][0].clone();
    second["id"] = json!(native_id("observation", &["second"]));
    before["observations"].as_array_mut().unwrap().push(second);
    assert!(parse_inventory(&serde_json::to_vec(&before).unwrap()).is_err());
    for path in [
        "../outside.rs",
        "/absolute.rs",
        "C:\\outside.rs",
        "unsafe\npath",
    ] {
        let mut value = fixture("app.rs", 1, "a");
        value["observations"][0]["target"]["path"] = json!(path);
        assert!(parse_inventory(&serde_json::to_vec(&value).unwrap()).is_err());
    }
}

#[test]
fn summary_retains_uncertainty_without_imported_markup() {
    let n = inventory(&fixture("app.rs", 1, "a"));
    let summary = compare(&n, &n, "team/project").unwrap().summary_markdown();
    assert!(summary.contains("never remediation"));
    assert!(summary.contains("Occurrences"));
    assert!(summary.contains("Missing-after records remain unresolved"));
}

fn append_fixture(target: &mut Value, extra: &Value) {
    target["assets"]
        .as_array_mut()
        .unwrap()
        .push(extra["assets"][1].clone());
    target["observations"]
        .as_array_mut()
        .unwrap()
        .push(extra["observations"][0].clone());
    target["unassociated_observation_ids"]
        .as_array_mut()
        .unwrap()
        .push(extra["unassociated_observation_ids"][0].clone());
}

#[test]
fn ambiguous_duplicates_count_before_move_candidate_selection() {
    let mut before = fixture("old.rs", 1, "old");
    let mut duplicate = fixture("duplicate.rs", 1, "duplicate-one");
    duplicate["observations"][0]["identity_confidence"] = json!("ambiguous_duplicate_anchor");
    duplicate["assets"][1]["attributes"]["ambiguous"] = json!(true);
    append_fixture(&mut before, &duplicate);
    let mut second = fixture("duplicate.rs", 2, "duplicate-two");
    let occurrence = native_id("occurrence", &["duplicate-two"]);
    let usage = native_id("asset", &["usage", &occurrence]);
    second["observations"][0]["occurrence_key"] = json!(occurrence);
    second["observations"][0]["asset_ids"][1] = json!(usage);
    second["observations"][0]["identity_confidence"] = json!("ambiguous_duplicate_anchor");
    second["assets"][1]["id"] = json!(usage);
    second["assets"][1]["attributes"]["ambiguous"] = json!(true);
    append_fixture(&mut before, &second);
    assert_eq!(
        report(&before, &fixture("new.rs", 1, "after"))["move_candidates"],
        json!([])
    );
}

#[test]
fn move_candidates_reject_changed_or_ambiguous_referenced_asset_facts() {
    let before = fixture("old.rs", 1, "old");
    for mutation in ["changed_facts", "conflicting_facts", "unknown_identity"] {
        let mut after = fixture("new.rs", 1, "after");
        match mutation {
            "changed_facts" => after["assets"][0]["attributes"]["identifier"] = json!("RS256"),
            "conflicting_facts" => {
                after["assets"][0]["uncertainty"] = json!(["conflicting_descriptors"])
            }
            _ => after["assets"][0]["identity_basis"] = json!("future_identity"),
        }
        assert_eq!(
            report(&before, &after)["move_candidates"],
            json!([]),
            "mutation {mutation}"
        );
    }
}

#[test]
fn after_only_ambiguous_or_unknown_assets_remain_unresolved() {
    let mut before = fixture("old.rs", 1, "old");
    before["assets"] = json!([]);
    before["observations"] = json!([]);
    before["unassociated_observation_ids"] = json!([]);
    for mutation in ["ambiguous_usage", "unknown_identity", "unknown_kind"] {
        let mut after = fixture("new.rs", 1, "after");
        let index = if mutation == "ambiguous_usage" { 1 } else { 0 };
        match mutation {
            "ambiguous_usage" => {
                after["observations"][0]["identity_confidence"] =
                    json!("ambiguous_duplicate_anchor");
                after["assets"][1]["attributes"]["ambiguous"] = json!(true);
            }
            "unknown_identity" => after["assets"][0]["identity_basis"] = json!("future_identity"),
            _ => after["assets"][0]["kind"] = json!("future_kind"),
        }
        let result = report(&before, &after);
        let key = after["assets"][index]["id"].as_str().unwrap();
        let row = result["assets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["key"] == key)
            .unwrap();
        assert_eq!(row["status"], "unresolved", "mutation {mutation}");
    }
}
