use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn scan(input: &Path, rules: &Path, out: &Path) -> Value {
    let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(input)
        .args(["--format", "all", "--threads", "1", "--rules-dir"])
        .arg(rules)
        .arg("--out-dir")
        .arg(out)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&std::fs::read(out.join("report.json")).unwrap()).unwrap()
}

#[test]
fn cli_keeps_distinct_occurrence_ids_and_legacy_report_contracts() {
    let fixture = Fixture(std::env::temp_dir().join(format!("pqc024-cli-{}", std::process::id())));
    let input = fixture.0.join("input");
    let rules = fixture.0.join("rules");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(input.join("app.js"), "wrap('TOKEN', 'TOKEN');\n").unwrap();
    std::fs::write(
        rules.join("test.yml"),
        serde_json::json!([{
            "id": "PQC_TEST", "kind": "tree_sitter", "category": "test", "severity": "medium",
            "risk": "quantum-uncertain", "confidence": 0.5, "migration_hint": "Review fixture.",
            "pattern": "'TOKEN'|TOKEN", "scope": "javascript"
        }])
        .to_string(),
    )
    .unwrap();
    let out = fixture.0.join("out");
    let first = scan(&input, &rules, &out);
    let second = scan(&input, &rules, &fixture.0.join("repeat"));
    assert_eq!(first["summary"]["scanned_files"], 1);
    assert_eq!(first["summary"]["total_findings"], 2);
    let hits = first["findings"].as_array().unwrap();
    let mut columns: Vec<_> = hits
        .iter()
        .map(|h| h["location"]["column"].as_u64().unwrap())
        .collect();
    columns.sort();
    assert_eq!(columns, vec![6, 15]);
    assert_ne!(hits[0]["finding_id"], hits[1]["finding_id"]);
    assert_eq!(first["findings"], second["findings"]);
    for name in [
        "report.json",
        "report.html",
        "report.md",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
    ] {
        assert!(out.join(name).is_file(), "{name}");
    }
    let sarif: Value =
        serde_json::from_slice(&std::fs::read(out.join("report.sarif")).unwrap()).unwrap();
    assert_eq!(sarif["runs"][0]["results"].as_array().unwrap().len(), 2);
    assert_eq!(
        sarif["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let schema: Value = serde_json::from_str(include_str!(
        "../../pqc_scan_report/tests/schema/sarif-schema-2.1.0.json"
    ))
    .unwrap();
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft4)
        .should_validate_formats(true)
        .build(&schema)
        .unwrap();
    assert!(validator.is_valid(&sarif));
}
