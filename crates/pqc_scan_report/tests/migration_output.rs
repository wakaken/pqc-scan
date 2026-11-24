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

#[test]
fn exact_inventory_links_are_html_metadata_and_stay_out_of_assessment_json() {
    use pqc_scan_core::identity::native_id;
    use pqc_scan_report::{write_migration_assessment_reports, ReportLanguage};

    let mut input = fixture::inventory("RSA");
    let finding = native_id("finding", &["migration-link-fixture"]);
    input["observations"][0]["rule_id"] = "TEST_RSA_RULE".into();
    input["legacy_associations"] = serde_json::json!([{
        "legacy_index": 0,
        "legacy_id": finding,
        "observation_ids": [input["observations"][0]["id"].clone()]
    }]);
    let evidence = fixture::ingest(&input, InputFormat::InventoryV1).unwrap();
    let assessment = evaluate(&evidence, None, "2026-11-18T00:00:00Z").unwrap();
    let folder = std::env::temp_dir().join(format!("migration-html-links-{}", std::process::id()));
    let files = write_migration_assessment_reports(
        &assessment,
        &evidence,
        None,
        &folder,
        ReportLanguage::Ja,
    )
    .unwrap();
    let json = std::fs::read_to_string(&files[0]).unwrap();
    let html = std::fs::read_to_string(&files[2]).unwrap();
    std::fs::remove_dir_all(folder).unwrap();
    assert!(!json.contains("TEST_RSA_RULE"));
    assert!(!json.contains(&finding));
    assert!(html.contains("\"scanReferences\""));
    assert!(html.contains("TEST_RSA_RULE"));
    assert!(html.contains(&finding));
}
