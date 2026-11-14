//! Byte fuzz worker: invokes the ordinary, bounded observed-detector entrypoints.
use pqc_scan_core::{Detector, ScannableFile};
use pqc_scan_detectors::{
    certificate_detector::CertificateDetector, dependency_detector::DependencyDetector,
    key_detector::KeyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use serde_json::{json, Value};
fn inspect(bytes: Vec<u8>, name: String) -> Value {
    let rules = RuleSet::load_from_dir(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let file = ScannableFile::from_bytes(name.into(), bytes);
    let detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(TreeSitterDetector),
        Box::new(RegexDetector),
        Box::new(DependencyDetector),
        Box::new(CertificateDetector::at(1_780_000_000)),
        Box::new(KeyDetector::default()),
    ];
    let mut rows: Vec<_> = detectors.iter().map(|d| match d.detect_observed(&file,&rules) {
        Ok(out) => { for detection in out.detections() {assert_eq!(detection.location.file,file.path.to_string_lossy());} json!({"detector":d.name(),"state":format!("{:?}",out.status()),"hits":out.detections().len(),"observations":out.observations().len()}) },
        Err(_) => json!({"detector":d.name(),"state":"Rejected"})
    }).collect();
    if file.path.extension().is_some_and(|x| x == "der") {
        use pqc_scan_detectors::certificate_detector::{
            parse_certificate_der, parse_public_key_der,
        };
        rows.push(json!({"detector":"public_certificate_der","state":match parse_certificate_der(&file.bytes){Ok(value)=>format!("accepted:{:?}",value.status),Err(error)=>format!("{error:?}")}}));
        rows.push(json!({"detector":"public_spki_der","state":match parse_public_key_der(&file.bytes){Ok(value)=>{if value.algorithm_oid=="1.2.840.113549.1.1.127" {assert!(value.key_bits.is_none());assert_ne!(value.status,pqc_scan_core::DetectorStatus::Complete);}format!("accepted:{:?}",value.status)},Err(error)=>format!("{error:?}")}}));
    }
    json!({"paths":rows})
}
#[test]
fn accepts_public_seed_and_rejects_broken_json_without_panicking() {
    let public = inspect(
        br#"{"dependencies":{"openssl":"1"}}"#.to_vec(),
        "package.json".into(),
    );
    assert!(public["paths"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v["hits"].as_u64().unwrap_or(0) > 0));
    let broken = inspect(b"{".to_vec(), "package.json".into());
    assert!(broken["paths"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v["state"] == "Rejected"));
}
#[test]
#[ignore = "invoked per input by the bounded TASK-047 supervisor"]
fn quality047_worker() {
    let result = std::panic::catch_unwind(|| {
        let bytes = std::fs::read(std::env::var("PQC_FUZZ_INPUT").unwrap()).unwrap();
        assert!(bytes.len() <= 1024 * 1024);
        inspect(bytes, std::env::var("PQC_FUZZ_NAME").unwrap())
    });
    match result {
        Ok(value) => println!("PQC047:{value}"),
        Err(_) => std::process::exit(42),
    }
}
