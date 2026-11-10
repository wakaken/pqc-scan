use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::Utc;
use pqc_scan_core::{Evidence, Finding, Location, ScanResult, SourceSnippet, SourceSnippetLine};
use pqc_scan_report::{write_reports, ReportFormat};
use pqc_scan_rules::{Risk, Severity};
use serde_json::{json, Value};

fn finding(file: &str, line: usize, column: usize) -> Finding {
    Finding {
        finding_id: "stable-id".into(),
        rule_id: "TEST_LOCATION".into(),
        category: "test".into(),
        risk: Risk::QuantumVulnerable,
        severity: Severity::High,
        confidence: 1.0,
        description: "Synthetic location".into(),
        migration_hint: "Review".into(),
        location: Location {
            file: file.into(),
            line,
            column,
        },
        evidence: Evidence {
            r#type: "regex_match".into(),
            r#match: "test".into(),
            snippet_preview: String::new(),
            metadata: BTreeMap::new(),
        },
        recommended_actions: vec![],
        source_snippet: None,
    }
}

fn report(findings: Vec<Finding>) -> Value {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "pqc-sarif-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let result = ScanResult {
        generated_at: Utc::now(),
        findings,
        cbom: vec![],
        dependency_sbom: vec![],
        summary: Default::default(),
    };
    write_reports(&result, &dir, ReportFormat::Sarif).unwrap();
    let value = serde_json::from_slice(&std::fs::read(dir.join("report.sarif")).unwrap()).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
    value
}

fn physical(value: &Value) -> &Value {
    &value["runs"][0]["results"][0]["locations"][0]["physicalLocation"]
}

#[test]
fn unknown_positions_are_not_invented() {
    for (line, column, expected) in [
        (0, 0, None),
        (0, 9, None),
        (4, 0, Some(json!({"startLine": 4}))),
        (4, 9, Some(json!({"startLine": 4, "startColumn": 9}))),
    ] {
        let value = report(vec![finding("Cargo.lock", line, column)]);
        assert_eq!(physical(&value)["artifactLocation"]["uri"], "Cargo.lock");
        assert_eq!(physical(&value).get("region"), expected.as_ref());
    }
    let value = report(vec![finding("", 3, 2)]);
    assert!(value["runs"][0]["results"][0].get("locations").is_none());
}

#[test]
fn paths_are_uri_references_not_queries_or_fragments() {
    for (file, expected) in [
        ("src/a.rs", "src/a.rs"),
        (
            "日本/😀 e\u{301} #%?.rs",
            "%E6%97%A5%E6%9C%AC/%F0%9F%98%80%20e%CC%81%20%23%25%3F.rs",
        ),
        ("a%20b.rs", "a%2520b.rs"),
        ("a:b.rs", "a%3Ab.rs"),
        ("./src/a.rs", "./src/a.rs"),
        ("/tmp/a b.rs", "file:///tmp/a%20b.rs"),
        ("a\t\n.rs", "a%09%0A.rs"),
    ] {
        assert_eq!(
            physical(&report(vec![finding(file, 1, 1)]))["artifactLocation"]["uri"],
            expected
        );
    }
    #[cfg(unix)]
    assert_eq!(
        physical(&report(vec![finding("a\\b.rs", 1, 1)]))["artifactLocation"]["uri"],
        "a%5Cb.rs"
    );
    #[cfg(windows)]
    for (file, expected) in [
        (r"C:\src\a b.rs", "file:///C:/src/a%20b.rs"),
        (r"\\server\share\a.rs", "file://server/share/a.rs"),
    ] {
        assert_eq!(
            physical(&report(vec![finding(file, 1, 1)]))["artifactLocation"]["uri"],
            expected
        );
    }
}

#[test]
fn unicode_columns_and_legacy_ast_abstention() {
    let text = "日😀e\u{301}\tRS256";
    let (line, column) =
        pqc_scan_core::model::line_col_for_offset(text, text.find("RS256").unwrap());
    assert_eq!((line, column), (1, 6));
    let value = report(vec![finding("unicode.rs", line, column)]);
    assert_eq!(value["runs"][0]["columnKind"], "unicodeCodePoints");
    assert_eq!(physical(&value)["region"]["startColumn"], 6);

    for (snippet, column, expected) in [
        (None, 5, None),
        (Some("let x = RSA;"), 9, Some(9)),
        (Some("日😀e\u{301}\tRSA;"), 12, None),
        (Some("[masked-sensitive-file-content]"), 5, None),
        (Some("abc..."), 7, None),
    ] {
        let mut f = finding("ast.rs", 1, column);
        f.evidence.r#type = "tree_sitter_match".into();
        f.source_snippet = snippet.map(|text| SourceSnippet {
            lines: vec![SourceSnippetLine {
                line: 1,
                text: text.into(),
                highlighted: true,
            }],
        });
        let value = report(vec![f]);
        assert_eq!(physical(&value)["region"]["startLine"], 1);
        assert_eq!(
            physical(&value)["region"]
                .get("startColumn")
                .and_then(Value::as_u64),
            expected
        );
    }
}

#[test]
fn official_schema_validates_output_offline_and_rejects_zero_positions() {
    use sha2::{Digest, Sha256};
    let bytes = include_bytes!("schema/sarif-schema-2.1.0.json");
    assert_eq!(
        format!("{:x}", Sha256::digest(bytes)),
        "c3b4bb2d6093897483348925aaa73af03b3e3f4bd4ca38cef26dcb4212a2682e"
    );
    let schema: Value = serde_json::from_slice(bytes).unwrap();
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft4)
        .should_validate_formats(true)
        .build(&schema)
        .unwrap();
    let mut value = report(vec![
        finding("unknown.json", 0, 0),
        finding("日本 #.rs", 2, 5),
    ]);
    assert!(
        validator.is_valid(&value),
        "{:?}",
        validator.iter_errors(&value).collect::<Vec<_>>()
    );
    value["runs"][0]["results"][0]["locations"][0]["physicalLocation"]["region"] =
        json!({"startLine":0,"startColumn":0});
    assert!(!validator.is_valid(&value));
    assert!(validator.is_valid(&report(vec![])));
}

#[test]
fn projection_preserves_results_rules_and_is_deterministic() {
    let f = finding("src/a.rs", 4, 2);
    let value = report(vec![f.clone(), f.clone()]);
    assert_eq!(value, report(vec![f.clone(), f]));
    assert_eq!(value["version"], "2.1.0");
    assert_eq!(
        value["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let results = value["runs"][0]["results"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0],
        json!({
            "ruleId":"TEST_LOCATION", "level":"error", "message":{"text":"Synthetic location | migration_hint=Review"},
            "locations":[{"physicalLocation":{"artifactLocation":{"uri":"src/a.rs"},"region":{"startLine":4,"startColumn":2}}}],
            "properties":{"migration_hint":"Review"}
        })
    );
}
