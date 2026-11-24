//! Shared synthetic context bodies; no expected decision logic.
#![allow(dead_code)]
use pqc_scan_core::migration::{MigrationContext, SavedEvidence};
use serde_json::{json, Value};
pub const AS_OF: &str = "2026-11-18T00:00:00Z";
pub fn provenance() -> Value {
    json!({"source_kind":"operator_claim","actor_id":"owner","asserted_at":"2026-11-17T00:00:00Z","valid_until":"2027-11-17T00:00:00Z"})
}
pub fn claim(value: Value) -> Value {
    json!({"state":"known","value":value,"provenance":provenance()})
}
pub fn unknown() -> Value {
    json!({"state":"unknown","provenance":provenance()})
}
pub fn reference() -> Value {
    json!({"id":"org-deadline","issuer_id":"example-org","document_id":"local-migration-policy","edition":"v1","clause":"3.1","scope_id":"example-org","https_uri":"https://synthetic.invalid/never-fetch"})
}
pub fn context(e: &SavedEvidence, claims: Value) -> Value {
    json!({"format":"pqc-scan.migration-context","version":1,"input":{"format":e.input().format,"sha256":e.input().sha256},"organization_id":"example-org","systems":[{"id":"system-a"}],"bindings":[{"target_id":e.targets()[0].id,"system_id":"system-a","claims":claims}],"references":[reference()]})
}
pub fn parse(v: &Value) -> MigrationContext {
    MigrationContext::from_bytes(&serde_json::to_vec(v).unwrap()).unwrap()
}
pub fn support() -> Value {
    json!({"runtime":claim(json!(true)),"provider":claim(json!(true)),"parameters":claim(json!(true)),"peer":claim(json!(true)),"protocol":claim(json!(true)),"jose":claim(json!(true)),"verifiers":claim(json!(true)),"pki":claim(json!(true))})
}
pub fn critical_signing() -> Value {
    json!({"applicability":claim(json!("applicable")),"use_case":claim(json!("signature")),"usage_state":claim(json!("active")),"business_critical":claim(json!(true))})
}
pub fn ordinary_signing() -> Value {
    json!({"applicability":claim(json!("applicable")),"use_case":claim(json!("signature")),"usage_state":claim(json!("active")),"business_critical":claim(json!(false)),"deadline":claim(json!({"kind":"none"}))})
}
