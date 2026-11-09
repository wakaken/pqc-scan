use std::process::Command;

use serde_json::Value;

#[test]
fn cli_preserves_native_locations_and_projects_unknown_unicode_and_ast_locations() {
    let root = std::env::temp_dir().join(format!("pqc-sarif-cli-{}", std::process::id()));
    let input = root.join("input");
    let rules = root.join("rules");
    let out = root.join("out");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::create_dir_all(&rules).unwrap();
    let source = "fn main() { let _ = \"日😀e\u{301}\tMATCH_TOKEN\"; }\n";
    std::fs::write(input.join("日本 😀 #%.rs"), source).unwrap();
    // JSON-decoded name is known, but its literal source spelling is absent.
    std::fs::write(
        input.join("unknown-sbom.json"),
        r#"{"components":[{"name":"open\u0073sl","version":"1.1.1"}]}"#,
    )
    .unwrap();
    std::fs::write(
        rules.join("locations.yml"),
        r#"
- id: LOCATION_REGEX
  kind: regex
  category: test
  severity: medium
  risk: quantum-vulnerable
  confidence: 1.0
  migration_hint: Review
  pattern: MATCH_TOKEN
  scope: all
- id: LOCATION_AST
  kind: tree_sitter
  category: test
  severity: medium
  risk: quantum-vulnerable
  confidence: 1.0
  migration_hint: Review
  pattern: MATCH_TOKEN
  scope: rust
- id: LOCATION_DEPENDENCY
  kind: dependency
  category: test
  severity: medium
  risk: quantum-vulnerable
  confidence: 1.0
  migration_hint: Review
  pattern: ^openssl\s
  scope: all
"#,
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(&input)
        .args(["--format", "all", "--rules-dir"])
        .arg(&rules)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let native: Value =
        serde_json::from_slice(&std::fs::read(out.join("report.json")).unwrap()).unwrap();
    let sarif: Value =
        serde_json::from_slice(&std::fs::read(out.join("report.sarif")).unwrap()).unwrap();
    assert_eq!(native["summary"]["scanned_files"], 2);
    let findings = native["findings"].as_array().unwrap();
    let results = sarif["runs"][0]["results"].as_array().unwrap();
    assert_eq!(findings.len(), 3);
    assert_eq!(results.len(), findings.len());
    for rule in ["LOCATION_REGEX", "LOCATION_AST", "LOCATION_DEPENDENCY"] {
        let f = findings.iter().find(|f| f["rule_id"] == rule).unwrap();
        let r = results.iter().find(|r| r["ruleId"] == rule).unwrap();
        let physical = &r["locations"][0]["physicalLocation"];
        if rule == "LOCATION_DEPENDENCY" {
            assert_eq!(f["location"]["line"], 0);
            assert_eq!(f["location"]["column"], 0);
            assert_eq!(physical["artifactLocation"]["uri"], "unknown-sbom.json");
            assert!(physical.get("region").is_none());
        } else {
            assert_eq!(
                physical["artifactLocation"]["uri"],
                "%E6%97%A5%E6%9C%AC%20%F0%9F%98%80%20%23%25.rs"
            );
            assert_eq!(physical["region"]["startLine"], 1);
            if rule == "LOCATION_REGEX" {
                let expected = source[..source.find("MATCH_TOKEN").unwrap()]
                    .chars()
                    .count()
                    + 1;
                assert_eq!(f["location"]["column"], expected);
                assert_eq!(physical["region"]["startColumn"], expected);
            } else {
                assert!(physical["region"].get("startColumn").is_none());
            }
        }
    }
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
    std::fs::remove_dir_all(root).unwrap();
}
