use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use pqc_scan_core::{Detection, Detector, DetectorStatus, ScannableFile};
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
            "pqc024-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("test.yml"),
            serde_json::json!([{
                "id": "PQC_OCCURRENCE_TEST", "kind": "tree_sitter", "category": "test",
                "severity": "medium", "risk": "quantum-uncertain", "confidence": 0.5,
                "migration_hint": "Review synthetic fixture.", "pattern": pattern,
                "scope": "java,go,javascript,typescript,python,rust,ruby"
            }])
            .to_string(),
        )
        .unwrap();
        let rules = RuleSet::load_from_dir(&root).unwrap();
        Self { root, rules }
    }

    fn detect(&self, ext: &str, text: &str) -> Vec<Detection> {
        let file = ScannableFile::from_bytes(
            PathBuf::from(format!("app.{ext}")),
            text.as_bytes().to_vec(),
        );
        TreeSitterDetector.detect(&file, &self.rules).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn same_line_occurrences_survive_in_all_seven_language_scopes() {
    let f = Fixture::new("TOKEN");
    for (ext, text) in [
        (
            "java",
            "class T { void f() { wrap(sign(\"TOKEN\"), sign(\"TOKEN\")); } }",
        ),
        (
            "go",
            "package t\nfunc f() { wrap(sign(\"TOKEN\"), sign(\"TOKEN\")) }",
        ),
        ("js", "wrap(sign('TOKEN'), sign('TOKEN'));"),
        ("ts", "wrap(sign('TOKEN'), sign('TOKEN'));"),
        ("py", "wrap(sign('TOKEN'), sign('TOKEN'))"),
        ("rs", "fn f() { wrap(sign(\"TOKEN\"), sign(\"TOKEN\")); }"),
        ("rb", "wrap(sign('TOKEN'), sign('TOKEN'))"),
    ] {
        let hits = f.detect(ext, text);
        assert_eq!(hits.len(), 2, "{ext}: {hits:?}");
        assert_ne!(hits[0].location.column, hits[1].location.column, "{ext}");
    }
}

#[test]
fn nested_parent_child_and_quoted_representations_share_one_source_occurrence() {
    let f = Fixture::new("['\"]TOKEN['\"]|TOKEN");
    for (text, expected) in [
        ("wrap(inner('TOKEN'));", 1),
        ("wrap(inner('TOKEN'), inner('TOKEN'));", 2),
        ("wrap('TOKEN', `TOKEN`);", 2),
    ] {
        assert_eq!(f.detect("js", text).len(), expected, "{text}");
    }
}

#[test]
fn different_case_and_unicode_prefix_do_not_merge_distinct_source_spans() {
    let f = Fixture::new("(?i)token");
    for text in [
        "wrap('TOKEN', 'token');",
        "const 日 = '😀'; wrap('TOKEN', 'TOKEN');",
    ] {
        assert_eq!(f.detect("js", text).len(), 2, "{text}");
    }
    let hits = f.detect("js", "wrap(\n 'TOKEN',\n 'TOKEN'\n);");
    let mut lines: Vec<_> = hits.iter().map(|h| h.location.line).collect();
    lines.sort();
    assert_eq!(lines, vec![2, 3]);
}

#[test]
fn same_line_unique_hits_consume_the_existing_cap() {
    let f = Fixture::new("TOKEN");
    for (n, expected) in [
        (32, DetectorStatus::Complete),
        (33, DetectorStatus::Truncated),
    ] {
        let text = "wrap(".to_string() + &vec!["'TOKEN'"; n].join(",") + ");";
        let file = ScannableFile::from_bytes(PathBuf::from("app.js"), text.into_bytes());
        let outcome = TreeSitterDetector.detect_detailed(&file, &f.rules).unwrap();
        assert_eq!(outcome.detections().len(), 32);
        assert_eq!(outcome.status(), expected);
    }
}

#[test]
fn clipped_display_suffix_is_not_a_source_match() {
    let f = Fixture::new(r"\.{3}");
    let text = format!("wrap('{}');", "x".repeat(3000));
    let file = ScannableFile::from_bytes(PathBuf::from("app.js"), text.into_bytes());
    let outcome = TreeSitterDetector.detect_detailed(&file, &f.rules).unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Truncated);
    assert!(outcome.detections().is_empty());
}

#[test]
fn dedicated_grammars_parse_javascript_typescript_and_tsx() {
    let f = Fixture::new("TOKEN");
    for (ext, text, expected) in [
        ("js", "const key = 'TOKEN';", DetectorStatus::Complete),
        ("ts", "const key = 'TOKEN';", DetectorStatus::Complete),
        (
            "ts",
            "const key: string = 'TOKEN';",
            DetectorStatus::Complete,
        ),
        (
            "tsx",
            "const view = <Widget>{'TOKEN'}</Widget>;",
            DetectorStatus::Complete,
        ),
    ] {
        let file = ScannableFile::from_bytes(
            PathBuf::from(format!("grammar.{ext}")),
            text.as_bytes().to_vec(),
        );
        let outcome = TreeSitterDetector.detect_detailed(&file, &f.rules).unwrap();
        assert_eq!(outcome.status(), expected, "{ext}: {text}");
        assert_eq!(outcome.detections().len(), 1, "{ext}: {text}");
    }
}

#[test]
fn standalone_comments_are_not_duplicate_ast_occurrences() {
    let f = Fixture::new("TOKEN");
    for (ext, text) in [
        (
            "java",
            "// TOKEN\nclass T { void f() { sign(\"TOKEN\"); } }",
        ),
        ("go", "package t\n// TOKEN\nfunc f() { sign(\"TOKEN\") }"),
        ("js", "// TOKEN\nsign('TOKEN');"),
        ("ts", "// TOKEN\nsign('TOKEN');"),
        ("py", "# TOKEN\nsign('TOKEN')"),
        ("rs", "// TOKEN\nfn f() { sign(\"TOKEN\"); }"),
        ("rb", "# TOKEN\nsign('TOKEN')"),
    ] {
        assert_eq!(f.detect(ext, text).len(), 1, "{ext}");
    }
}
