//! Measurement adapter only: frozen expectations are compared by the independent Python evaluator.
use pqc_scan_core::{
    assessment, identity, policy, recommendation, risk, DependencySbomEntry, Detection, Finding,
    NativeScan,
};
use pqc_scan_rules::Rule;
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn parameters(input: &Value) -> BTreeMap<String, String> {
    serde_json::from_value(input["parameters"].clone()).unwrap()
}
fn native(input: &Value) -> NativeScan {
    let mut value: Value = serde_json::from_str(include_str!(
        "../../pqc_scan_cli/tests/fixtures/native-v1.golden.json"
    ))
    .unwrap();
    value["run"]["id"] = json!(identity::native_id("scan", &["task056-synthetic"]));
    value["run"]["generated_at"] = json!("2026-11-15T00:00:00Z");
    value["assets"] = Value::Array(input["assets"].as_array().unwrap().iter().map(|a| {
        let relations: Vec<_> = a["relations"].as_array().unwrap().iter().map(|r| {
            json!({"relationship":r["relationship"], "target":identity::native_id("asset", &[r["target"].as_str().unwrap()])})
        }).collect();
        json!({"id":identity::native_id("asset", &[a["tag"].as_str().unwrap()]),
            "kind":a["kind"], "identity_basis":"synthetic-task056", "attributes":a["attributes"],
            "relations":relations, "uncertainty":[]})
    }).collect());
    for key in [
        "observations",
        "legacy_associations",
        "unassociated_observation_ids",
        "legacy_loss_records",
    ] {
        value[key] = json!([]);
    }
    if let Some(coverage) = input.get("coverage") {
        value["coverage"] = coverage.clone();
    }
    if let Some(now) = input.get("generated_at") {
        value["run"]["generated_at"] = now.clone();
    }
    if let Some(observations) = input["observations"].as_array() {
        let template: Value = serde_json::from_str(include_str!(
            "../../pqc_scan_cli/tests/fixtures/native-v1.golden.json"
        ))
        .unwrap();
        value["observations"] = Value::Array(
            observations
                .iter()
                .enumerate()
                .map(|(i, o)| {
                    let mut observation = template["observations"][0].clone();
                    observation["id"] =
                        json!(identity::native_id("observation", &[&i.to_string()]));
                    observation["occurrence_key"] =
                        json!(identity::native_id("occurrence", &[&i.to_string()]));
                    observation["asset_ids"] = Value::Array(
                        o["assets"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|a| json!(identity::native_id("asset", &[a.as_str().unwrap()])))
                            .collect(),
                    );
                    for key in ["method", "assurance", "usage"] {
                        observation[key] = o[key].clone();
                    }
                    observation
                })
                .collect(),
        );
        value["unassociated_observation_ids"] = Value::Array(
            value["observations"]
                .as_array()
                .unwrap()
                .iter()
                .map(|o| o["id"].clone())
                .collect(),
        );
    }
    serde_json::from_value(value).unwrap()
}
fn finding(input: &Value) -> Finding {
    let mut metadata = input.get("metadata").cloned().unwrap_or(json!({}));
    if let Some(language) = input["language"].as_str() {
        metadata["language"] = json!(language);
    }
    serde_json::from_value(json!({"finding_id":"task056-synthetic", "rule_id":input["rule_id"].as_str().unwrap_or("TASK056_REFERENCE"),
        "category":input["category"], "risk":input["risk"], "severity":"medium", "confidence":0.8,
        "description":"Synthetic reference, not execution", "migration_hint":"Review prerequisites",
        "location":{"file":input["file"].as_str().unwrap_or("example.txt"), "line":1, "column":1},
        "evidence":{"type":input["evidence_type"], "match":"synthetic reference", "snippet_preview":"synthetic", "metadata":metadata}})).unwrap()
}
fn context(input: &Value) -> policy::AssessmentContext {
    input
        .get("context")
        .map(|v| policy::AssessmentContext::from_json(&v.to_string()).unwrap())
        .unwrap_or_default()
}

fn observe(case: &Value) -> Value {
    let input = &case["input"];
    match case["kind"].as_str().unwrap() {
        "algorithm" => serde_json::to_value(assessment::assess_algorithm(
            input["identifier"].as_str().unwrap(),
            &parameters(input),
        ))
        .unwrap(),
        "policy" => {
            let a = assessment::assess_algorithm(
                input["identifier"].as_str().unwrap(),
                &parameters(input),
            );
            let context =
                policy::AssessmentContext::from_json(&input["context"].to_string()).unwrap();
            json!({"algorithm":a, "policy":policy::assess_primitive_policy(&a, &context),
                "candidates":policy::candidates(&a, &context)})
        }
        "severity" => {
            let rule: Rule = serde_json::from_value(json!({"id":"TASK056_REFERENCE", "kind":"regex",
                "category":input["category"], "risk":input["risk"], "severity":input["severity"],
                "confidence":input["confidence"], "migration_hint":"Review", "pattern":"synthetic", "scope":"code"})).unwrap();
            let f = finding(&json!({"category":input["category"], "risk":input["risk"],
                "evidence_type":input["evidence_type"], "language":"unknown"}));
            let detection = Detection {
                rule_id: f.rule_id,
                location: f.location,
                evidence: f.evidence,
            };
            json!({"severity":risk::adjusted_severity(&rule, &detection), "confidence_unchanged":rule.confidence})
        }
        "aggregate" => {
            let decisions: Vec<_> = input["decisions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|d| match d.as_str().unwrap() {
                    "pass" => policy::Decision::Pass,
                    "fail" => policy::Decision::Fail,
                    "not_applicable" => policy::Decision::NotApplicable,
                    "indeterminate" => policy::Decision::Indeterminate,
                    _ => panic!("Invalid synthetic decision"),
                })
                .collect();
            serde_json::to_value(policy::summarize(
                &decisions,
                input["inspection_complete"].as_bool().unwrap(),
                input["native_complete"].as_bool().unwrap(),
                &context(input),
            ))
            .unwrap()
        }
        "inventory" => {
            match policy::evaluate(&native(input), &context(input), "migration-review-v1") {
                Ok(result) => serde_json::to_value(result).unwrap(),
                Err(_) => json!({"error":"synthetic inventory evaluation rejected"}),
            }
        }
        "recommendation" => {
            let mut findings = vec![finding(input)];
            let dependencies: Vec<DependencySbomEntry> = input
                .get("dependencies")
                .map(|v| serde_json::from_value(v.clone()).unwrap())
                .unwrap_or_default();
            let mut profile = recommendation::RuntimeProfile::from_repository(
                &std::env::temp_dir(),
                &[],
                &dependencies,
            );
            if let Some(versions) = input.get("versions") {
                profile.java_version = versions["java"].as_str().map(str::to_string);
                profile.node_version = versions["node"].as_str().map(str::to_string);
                profile.go_version = versions["go"].as_str().map(str::to_string);
                profile.python_version = versions["python"].as_str().map(str::to_string);
                profile.rust_version = versions["rust"].as_str().map(str::to_string);
                profile.ruby_version = versions["ruby"].as_str().map(str::to_string);
            }
            recommendation::annotate_findings(&mut findings, &profile);
            let mut value = serde_json::to_value(&findings[0]).unwrap();
            value["action_ids"] = json!(findings[0]
                .recommended_actions
                .iter()
                .map(|a| &a.action_id)
                .collect::<Vec<_>>());
            value
        }
        _ => panic!("Unsupported frozen probe kind"),
    }
}
#[test]
fn frozen_judgment_probe_emits_observations_without_using_expected_answers() {
    let oracle: Value =
        serde_json::from_str(include_str!("../../../tests/quality/judgment/oracle.json")).unwrap();
    let supplement: Value = serde_json::from_str(include_str!(
        "../../../tests/quality/judgment/supplement.json"
    ))
    .unwrap();
    let all: Vec<&Value> = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .chain(supplement["cases"].as_array().unwrap())
        .collect();
    let rows: Vec<Value> = all
        .iter()
        .map(|case| json!({"id":case["id"], "actual":observe(case)}))
        .collect();
    assert_eq!(rows.len(), all.len());
    // This proves adapter completion only, never correctness of all judgments.
    println!(
        "QUALITY056_OBSERVATIONS:{}",
        serde_json::to_string(&rows).unwrap()
    );
}
