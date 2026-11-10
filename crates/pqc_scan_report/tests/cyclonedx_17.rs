use pqc_scan_core::NativeScan;
use pqc_scan_report::cyclonedx;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

fn native(assets: Value, observations: Value) -> NativeScan {
    serde_json::from_value(json!({
        "schema_version":1,"identity_scheme":"pqc-scan-length-prefixed-sha256-v1",
        "scanner_version":"0.1.0","catalog_version":"native-exact-identifiers-1",
        "profile":"static-native-v1","rules_digest":"test",
        "run":{"id":"scan-test","attempt_id":"static-1","generated_at":"2026-11-10T00:00:00Z"},
        "repository_scope":"tests/project","scope":{},"coverage":{},"diagnostics":[],
        "native_completeness":"not_exhaustive","assets":assets,"observations":observations,
        "legacy_associations":[],"unassociated_observation_ids":[],"legacy_loss_records":[],
        "assessment_status":"not_evaluated","assessments":[],"export_loss_notes":[]
    }))
    .unwrap()
}

fn asset(id: &str, kind: &str, attributes: Value, relations: Value) -> Value {
    json!({"id":id,"kind":kind,"identity_basis":"test","attributes":attributes,
        "relations":relations,"uncertainty":[]})
}

fn validator() -> jsonschema::Validator {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/cyclonedx/1.7");
    let sources: Value =
        serde_json::from_slice(&std::fs::read(root.join("SOURCES.json")).unwrap()).unwrap();
    let mut options = jsonschema::options();
    for name in [
        "spdx.schema.json",
        "jsf-0.82.schema.json",
        "cryptography-defs.schema.json",
    ] {
        let data = std::fs::read(root.join(name)).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&data)),
            sources["files"][name]["sha256"]
        );
        options = options.with_resource(
            format!("http://cyclonedx.org/schema/{name}"),
            jsonschema::Resource::from_contents(serde_json::from_slice(&data).unwrap()).unwrap(),
        );
    }
    let data = std::fs::read(root.join("bom-1.7.schema.json")).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&data)),
        sources["files"]["bom-1.7.schema.json"]["sha256"]
    );
    options
        .build(&serde_json::from_slice::<Value>(&data).unwrap())
        .unwrap()
}

fn validate(value: &Value) {
    let validator = validator();
    let errors: Vec<_> = validator
        .iter_errors(value)
        .map(|e| e.to_string())
        .collect();
    assert!(errors.is_empty(), "{errors:#?}");
}

#[test]
fn empty_documents_are_official_schema_valid_and_truthful() {
    let (crypto, software) = cyclonedx::documents(&native(json!([]), json!([]))).unwrap();
    for value in [crypto, software] {
        validate(&value);
        assert_eq!(value["bomFormat"], "CycloneDX");
        assert_eq!(value["specVersion"], "1.7");
        assert!(value["components"].as_array().unwrap().is_empty());
        assert!(value.to_string().contains("not_exhaustive"));
        assert!(value.get("vulnerabilities").is_none());
        assert!(value.get("dependencies").is_none());
    }
}

#[test]
fn separate_public_objects_have_resolving_refs_and_no_secret_claims() {
    let assets = json!([
        asset(
            "alg",
            "algorithm",
            json!({"namespace":"oid","identifier":"1.2.840.113549.1.1.1","purpose":"public_key","parameters":{}}),
            json!([])
        ),
        asset(
            "sig",
            "algorithm",
            json!({"namespace":"named","identifier":"ML-DSA-65","purpose":"reference","parameters":{}}),
            json!([])
        ),
        asset(
            "key",
            "public_key",
            json!({"spki_sha256":"b".repeat(64),"key_bits":2048,"private_bytes":"SECRET_CANARY_034"}),
            json!([{"relationship":"algorithm","target":"alg"}])
        ),
        asset(
            "cert",
            "certificate",
            json!({"certificate_sha256":"a".repeat(64),"not_before_unix":0,"not_after_unix":2000000000}),
            json!([{"relationship":"subject_key","target":"key"},{"relationship":"signature_algorithm","target":"sig"}])
        ),
        asset(
            "usage",
            "usage",
            json!({"path":"public.pem","raw":"SECRET_CANARY_034"}),
            json!([])
        ),
        asset(
            "future",
            "future-kind",
            json!({"value":"SECRET_CANARY_034"}),
            json!([])
        )
    ]);
    let (crypto, software) = cyclonedx::documents(&native(assets, json!([]))).unwrap();
    validate(&crypto);
    validate(&software);
    let components = crypto["components"].as_array().unwrap();
    assert_eq!(components.len(), 4);
    let refs: BTreeSet<_> = components
        .iter()
        .map(|c| c["bom-ref"].as_str().unwrap())
        .collect();
    assert_eq!(refs.len(), components.len());
    let cert = components.iter().find(|c| c["bom-ref"] == "cert").unwrap();
    let cp = &cert["cryptoProperties"]["certificateProperties"];
    assert!(refs.contains(cp["subjectPublicKeyRef"].as_str().unwrap()));
    assert!(refs.contains(cp["signatureAlgorithmRef"].as_str().unwrap()));
    assert!(cp.get("fingerprint").is_none());
    assert_eq!(
        cert["properties"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["name"] == "pqc-scan:certificate-der-sha256")
            .unwrap()["value"],
        "a".repeat(64)
    );
    let loss = crypto["properties"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "pqc-scan:export-loss")
        .unwrap()["value"]
        .as_str()
        .unwrap();
    let loss: Value = serde_json::from_str(loss).unwrap();
    assert_eq!(loss["counts"]["certificate_tbs_fingerprint_unavailable"], 1);
    assert!(cp.get("certificateState").is_none());
    let key = components.iter().find(|c| c["bom-ref"] == "key").unwrap();
    assert_eq!(
        key["cryptoProperties"]["relatedCryptoMaterialProperties"]["type"],
        "public-key"
    );
    assert_eq!(
        key["cryptoProperties"]["relatedCryptoMaterialProperties"]["size"],
        2048
    );
    for value in [crypto, software] {
        let raw = value.to_string();
        assert!(!raw.contains("SECRET_CANARY_034"));
        assert!(!raw.contains("nistQuantumSecurityLevel"));
        assert!(!raw.contains("certificationLevel"));
        assert!(
            raw.contains("unsupported_asset_kind") || raw.contains("software_source_assertions")
        );
    }
}

#[test]
fn source_version_provenance_is_not_upgraded_to_resolution_or_installation() {
    let assets = json!([
        asset(
            "manifest",
            "dependency",
            json!({"name":"openssl","ecosystem":"cargo","declared_constraint":"^1.0","resolved_version":null,"reported_version":"^1.0","version_provenance":"manifest_declared"}),
            json!([])
        ),
        asset(
            "lock",
            "dependency",
            json!({"name":"openssl","ecosystem":"cargo","declared_constraint":null,"resolved_version":"1.0.75","reported_version":"1.0.75","version_provenance":"lock_resolved"}),
            json!([])
        ),
        asset(
            "sbom",
            "dependency",
            json!({"name":"openssl","ecosystem":"cargo","declared_constraint":null,"resolved_version":null,"reported_version":"9.9.9","version_provenance":"sbom_reported_unverified"}),
            json!([])
        )
    ]);
    let (crypto, software) = cyclonedx::documents(&native(assets, json!([]))).unwrap();
    validate(&crypto);
    validate(&software);
    let c = software["components"].as_array().unwrap();
    assert_eq!(c.len(), 3);
    assert!(c
        .iter()
        .find(|v| v["bom-ref"] == "manifest")
        .unwrap()
        .get("version")
        .is_none());
    assert!(c
        .iter()
        .find(|v| v["bom-ref"] == "sbom")
        .unwrap()
        .get("version")
        .is_none());
    assert_eq!(
        c.iter().find(|v| v["bom-ref"] == "lock").unwrap()["version"],
        "1.0.75"
    );
    assert!(software.to_string().contains("^1.0"));
    assert!(software.to_string().contains("sbom_reported_unverified"));
}

#[test]
fn malformed_native_reference_is_rejected_and_order_is_deterministic() {
    let a = asset(
        "key",
        "public_key",
        json!({}),
        json!([{"relationship":"algorithm","target":"missing"}]),
    );
    assert!(cyclonedx::documents(&native(json!([a]), json!([]))).is_err());
    let a = asset(
        "z",
        "algorithm",
        json!({"namespace":"named","identifier":"RSA","purpose":"reference","parameters":{}}),
        json!([]),
    );
    let b = asset(
        "a",
        "algorithm",
        json!({"namespace":"named","identifier":"ML-KEM","purpose":"reference","parameters":{}}),
        json!([]),
    );
    assert_eq!(
        cyclonedx::documents(&native(json!([a.clone(), b.clone()]), json!([]))).unwrap(),
        cyclonedx::documents(&native(json!([b, a]), json!([]))).unwrap()
    );
}

#[test]
fn selected_writer_adds_exactly_two_files_and_preserves_legacy_bytes() {
    use pqc_scan_core::{
        scan_repository_detailed, scan_repository_native, NativeOptions, ScanConfig,
    };
    use pqc_scan_report::{write_reports, ReportFormat};
    use pqc_scan_rules::RuleSet;
    let root = std::env::temp_dir().join(format!("pqc-cdx-writer-{}", std::process::id()));
    std::fs::create_dir_all(root.join("input")).unwrap();
    let rules = RuleSet::load_from_dir(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"),
    )
    .unwrap();
    let config = ScanConfig {
        root: root.join("input"),
        threads: 1,
        ..Default::default()
    };
    let document = scan_repository_native(&config, &rules, &[], &NativeOptions::default()).unwrap();
    let out = root.join("out");
    let paths = write_reports(document.legacy(), &out, ReportFormat::All).unwrap();
    let before: Vec<_> = paths.iter().map(|p| std::fs::read(p).unwrap()).collect();
    let added = cyclonedx::write_cyclonedx_17(&document, &out).unwrap();
    assert_eq!(
        added
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect::<Vec<_>>(),
        ["cbom.cdx-1.7.json", "dependency-sbom.cdx-1.7.json"]
    );
    assert_eq!(std::fs::read_dir(&out).unwrap().count(), 8);
    assert_eq!(
        before,
        paths
            .iter()
            .map(|p| std::fs::read(p).unwrap())
            .collect::<Vec<_>>()
    );
    for path in added {
        validate(&serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap());
    }
    let detailed = scan_repository_detailed(&config, &rules, &[]).unwrap();
    assert!(cyclonedx::write_cyclonedx_17(&detailed, &root.join("missing-native")).is_err());
    assert!(!root.join("missing-native").exists());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn pinned_upstream_example_and_all_source_digests_validate_offline() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/cyclonedx/1.7");
    let sources: Value =
        serde_json::from_slice(&std::fs::read(root.join("SOURCES.json")).unwrap()).unwrap();
    for (name, record) in sources["files"].as_object().unwrap() {
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(std::fs::read(root.join(name)).unwrap())
            ),
            record["sha256"]
        );
    }
    validate(
        &serde_json::from_slice(&std::fs::read(root.join("example-certificate.json")).unwrap())
            .unwrap(),
    );
}

#[test]
fn relationships_with_wrong_asset_kind_do_not_become_crypto_references() {
    let assets = json!([
        asset(
            "alg",
            "algorithm",
            json!({"identifier":"RSA","namespace":"named"}),
            json!([])
        ),
        asset(
            "cert",
            "certificate",
            json!({"certificate_sha256":"c".repeat(64)}),
            json!([{"relationship":"subject_key","target":"alg"}])
        ),
        asset(
            "key",
            "public_key",
            json!({"spki_sha256":"b".repeat(64)}),
            json!([{"relationship":"algorithm","target":"cert"}])
        )
    ]);
    let (crypto, _) = cyclonedx::documents(&native(assets, json!([]))).unwrap();
    validate(&crypto);
    let c = crypto["components"].as_array().unwrap();
    assert!(
        c.iter().find(|v| v["bom-ref"] == "cert").unwrap()["cryptoProperties"]
            ["certificateProperties"]
            .get("subjectPublicKeyRef")
            .is_none()
    );
    assert!(
        c.iter().find(|v| v["bom-ref"] == "key").unwrap()["cryptoProperties"]
            ["relatedCryptoMaterialProperties"]
            .get("algorithmRef")
            .is_none()
    );
    let loss = crypto["properties"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == "pqc-scan:export-loss")
        .unwrap()["value"]
        .as_str()
        .unwrap();
    let loss: Value = serde_json::from_str(loss).unwrap();
    assert_eq!(loss["counts"]["unprojected_asset_relations"], 2);
}

#[test]
fn lexical_reference_provenance_matches_semantic_golden_without_runtime_claims() {
    let assets = json!([asset(
        "algorithm-test",
        "algorithm",
        json!({"identifier":"ML-KEM-768","namespace":"named","purpose":"reference","parameters":{}}),
        json!([])
    )]);
    let observations = json!([{
        "id":"observation-test","occurrence_key":null,"identity_confidence":"run_local_only",
        "asset_ids":["algorithm-test"],"target":{"path":"src/例.rs","line":7,"column":3,
            "column_kind":"unicode_scalar","byte_start":null,"byte_end":null,"object_index":null},
        "method":"lexical_match","producer":"fixture","assurance":"lexical","usage":"referenced",
        "rule_id":null,"evidence":{"marker_kind":null,"capture_status":"bounded"},"uncertainty":[]
    }]);
    let (crypto, _) = cyclonedx::documents(&native(assets, observations)).unwrap();
    validate(&crypto);
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/cyclonedx-17-algorithm.json")).unwrap();
    assert_eq!(crypto["components"], expected);
    assert!(!crypto.to_string().contains("protocolProperties"));
}

#[test]
fn private_marker_observation_does_not_invent_a_key_asset() {
    let assets = json!([asset(
        "usage",
        "usage",
        json!({"path":"key.pem"}),
        json!([])
    )]);
    let observations = json!([{
        "id":"marker","occurrence_key":null,"identity_confidence":"run_local_only",
        "asset_ids":["usage"],"target":{"path":"key.pem","line":1,"column":1,
            "column_kind":"unicode_scalar","byte_start":null,"byte_end":null,"object_index":null},
        "method":"private_key_marker","producer":"fixture","assurance":"lexical","usage":"unknown",
        "rule_id":null,"evidence":{"marker_kind":"private_key","capture_status":"bounded"},"uncertainty":[]
    }]);
    let (crypto, software) = cyclonedx::documents(&native(assets, observations)).unwrap();
    for value in [crypto, software] {
        validate(&value);
        assert!(value["components"].as_array().unwrap().is_empty());
        let loss = value["properties"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == "pqc-scan:export-loss")
            .unwrap()["value"]
            .as_str()
            .unwrap();
        let loss: Value = serde_json::from_str(loss).unwrap();
        assert_eq!(loss["counts"]["unprojected_observations"], 1);
    }
}

#[test]
fn absent_public_fingerprints_do_not_fabricate_parsed_objects_or_dangling_refs() {
    let assets = json!([
        asset("missing-key", "public_key", json!({}), json!([])),
        asset("missing-cert", "certificate", json!({}), json!([])),
        asset(
            "cert",
            "certificate",
            json!({"certificate_sha256":"c".repeat(64)}),
            json!([{"relationship":"subject_key","target":"missing-key"}])
        )
    ]);
    let (crypto, _) = cyclonedx::documents(&native(assets, json!([]))).unwrap();
    validate(&crypto);
    assert_eq!(crypto["components"].as_array().unwrap().len(), 1);
    assert!(
        crypto["components"][0]["cryptoProperties"]["certificateProperties"]
            .get("subjectPublicKeyRef")
            .is_none()
    );
    let loss = crypto["properties"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == "pqc-scan:export-loss")
        .unwrap()["value"]
        .as_str()
        .unwrap();
    let loss: Value = serde_json::from_str(loss).unwrap();
    assert_eq!(loss["counts"]["unavailable_public_object_facts"], 2);
}

#[test]
fn observation_columns_preserve_byte_and_scalar_units_with_their_values() {
    let assets = json!([asset(
        "alg",
        "algorithm",
        json!({"identifier":"RSA","namespace":"named"}),
        json!([])
    )]);
    let observations:Vec<_>=[("bytes",5,"utf8_byte"),("scalars",3,"unicode_scalar")].into_iter().map(|(id,column,unit)|json!({
        "id":id,"occurrence_key":null,"identity_confidence":"run_local_only",
        "asset_ids":["alg"],"target":{"path":"src/例.rs","line":7,"column":column,
            "column_kind":unit,"byte_start":null,"byte_end":null,"object_index":null},
        "method":"ast_pattern","producer":"fixture","assurance":"syntactic","usage":"referenced",
        "rule_id":null,"evidence":{"marker_kind":null,"capture_status":"bounded"},"uncertainty":[]
    })).collect();
    let (crypto, _) = cyclonedx::documents(&native(assets, json!(observations))).unwrap();
    validate(&crypto);
    let properties = crypto["components"][0]["properties"].as_array().unwrap();
    let observations: Value = serde_json::from_str(
        properties
            .iter()
            .find(|p| p["name"] == "pqc-scan:observations")
            .unwrap()["value"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    for (id, column, unit) in [("bytes", 5, "utf8_byte"), ("scalars", 3, "unicode_scalar")] {
        let observation = observations
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == id)
            .unwrap();
        assert_eq!(observation["path"], "src/例.rs");
        assert_eq!(observation["line"], 7);
        assert_eq!(observation["column"], column);
        assert_eq!(observation["column_kind"], unit);
    }
}
