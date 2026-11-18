#[allow(dead_code)]
#[path = "../../pqc_scan_core/tests/support/migration/mod.rs"]
mod fixture;
use pqc_scan_core::migration::{evaluate, InputFormat, SavedEvidence};
use pqc_scan_report::write_migration_assessment_json;
#[test]
fn substituted_evidence_cannot_write_a_new_report_directory() {
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let r = evaluate(&e, None, "2026-11-18T00:00:00Z").unwrap();
    let path = std::env::temp_dir().join(format!("migration-output-069-{}", std::process::id()));
    assert!(!path.exists());
    let mut other = serde_json::to_vec(&fixture::inventory("RSA")).unwrap();
    other.push(b' ');
    let other = SavedEvidence::from_bytes(&other, InputFormat::InventoryV1).unwrap();
    assert!(write_migration_assessment_json(&r, &other, None, &path).is_err());
    assert!(!path.exists());
}
