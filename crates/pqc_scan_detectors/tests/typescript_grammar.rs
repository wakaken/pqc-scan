use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use pqc_scan_core::{Detector, DetectorOutcome, DetectorStatus, ScannableFile};
use pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector;
use pqc_scan_rules::RuleSet;

struct Fixture {
    root: PathBuf,
    rules: RuleSet,
}

impl Fixture {
    fn new(pattern: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-typescript-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("rule.yml"),
            serde_json::json!({
                "id": "TYPESCRIPT_GRAMMAR_TEST", "kind": "tree_sitter",
                "category": "test", "severity": "low", "risk": "quantum-uncertain",
                "confidence": 0.5, "migration_hint": "Review synthetic fixture.",
                "pattern": pattern, "scope": "javascript"
            })
            .to_string(),
        )
        .unwrap();
        let rules = RuleSet::load_from_dir(&root).unwrap();
        Self { root, rules }
    }

    fn detect(&self, name: &str, text: &str) -> DetectorOutcome {
        TreeSitterDetector
            .detect_detailed(
                &ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec()),
                &self.rules,
            )
            .unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn typed_example_completes_and_retains_algorithm_literals() {
    let fixture = Fixture::new("RS256");
    let outcome = fixture.detect(
        "tokenService.ts",
        include_str!("../../../examples/typescript/src/services/tokenService.ts"),
    );
    assert_eq!(outcome.status(), DetectorStatus::Complete);
    assert!(outcome
        .detections()
        .iter()
        .any(|d| d.evidence.r#match == "RS256"));
}

#[test]
fn typescript_types_generics_and_angle_assertions_complete() {
    let fixture = Fixture::new("RS256");
    for text in [
        "interface Options { algorithm: string }; const options: Options = { algorithm: 'RS256' };",
        "type Algorithm = 'RS256'; function choose<T extends Algorithm>(value: T): T { return value; }",
        "const choose = <T>(value: T): T => value; const algorithm = <string>'RS256';",
        "const options = { algorithm: 'RS256' } satisfies { algorithm: string };",
    ] {
        let outcome = fixture.detect("app.ts", text);
        assert_eq!(outcome.status(), DetectorStatus::Complete, "{text}");
        assert_eq!(outcome.detections().len(), 1, "{text}");
    }
}

#[test]
fn tsx_combines_typed_props_and_jsx_without_losing_expression_hits() {
    let fixture = Fixture::new("RS256");
    let outcome = fixture.detect(
        "view.tsx",
        "type Props = { algorithm: string }; const View = (props: Props) => <Widget algorithm={'RS256'}>{sign('RS256')}</Widget>;",
    );
    assert_eq!(outcome.status(), DetectorStatus::Complete);
    assert_eq!(outcome.detections().len(), 2);
    assert_ne!(
        outcome.detections()[0].location.column,
        outcome.detections()[1].location.column
    );
    assert!(outcome
        .detections()
        .iter()
        .all(|d| d.evidence.metadata["language"] == "typescript"));
}

#[test]
fn malformed_typed_source_is_partial_and_retains_obtainable_hits() {
    let fixture = Fixture::new("RS256");
    for name in ["app.ts", "view.tsx"] {
        let outcome = fixture.detect(
            name,
            "const algorithm: string = 'RS256'; function broken( {",
        );
        assert_eq!(outcome.status(), DetectorStatus::Partial, "{name}");
        assert_eq!(outcome.detections().len(), 1, "{name}");
    }
}

#[test]
fn typed_calls_keep_distinct_source_occurrences_and_ignore_comments_and_namesakes() {
    let fixture = Fixture::new("createSign|RS256|RSA");
    for name in ["app.ts", "view.tsx"] {
        let outcome = fixture.detect(
            name,
            "// RS256 RSA createSign\nconst 日: string = '😀'; const registerRSAMapper = 1; wrap(crypto.createSign('RS256'), sign('RS256'));",
        );
        assert_eq!(outcome.status(), DetectorStatus::Complete, "{name}");
        assert_eq!(
            outcome.detections().len(),
            3,
            "{name}: {:?}",
            outcome.detections()
        );
        assert!(outcome.detections().iter().all(|d| d.location.line == 2));
        let mut columns: Vec<_> = outcome
            .detections()
            .iter()
            .map(|d| d.location.column)
            .collect();
        columns.sort();
        columns.dedup();
        assert_eq!(columns.len(), 3);
    }
}

#[test]
fn typed_source_inspects_occurrences_beyond_the_old_hit_limit() {
    let fixture = Fixture::new("RS256");
    for name in ["app.ts", "view.tsx"] {
        for (count, expected) in [
            (32, DetectorStatus::Complete),
            (33, DetectorStatus::Complete),
            (80, DetectorStatus::Complete),
        ] {
            let text = "const algorithms: string[] = [".to_string()
                + &vec!["'RS256'"; count].join(",")
                + "];";
            let outcome = fixture.detect(name, &text);
            assert_eq!(outcome.status(), expected, "{name}: {count}");
            assert_eq!(outcome.detections().len(), count);
        }
    }
}
