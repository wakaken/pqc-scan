use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use pqc_scan_core::{
    scan_repository_detailed, Detector, DetectorStatus, DiagnosticCode, InspectionStatus,
    ScanConfig, ScannableFile,
};
use pqc_scan_detectors::{
    default_detectors, key_detector::KeyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;

struct Fixture {
    root: PathBuf,
    rules: RuleSet,
}

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

impl Fixture {
    fn new(kind: &str, scope: &str, pattern: &str) -> Self {
        Self::with_rules(serde_json::json!([{
            "id": "TEST", "kind": kind, "category": "CryptoAPI", "severity": "high",
            "risk": "quantum-vulnerable", "confidence": 0.9,
            "migration_hint": "Review this synthetic match.", "pattern": pattern,
            "scope": scope
        }]))
    }

    fn with_rules(rules: serde_json::Value) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pqc-task019-{}-{unique}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir_all(root.join("rules")).unwrap();
        std::fs::create_dir_all(root.join("input")).unwrap();
        std::fs::write(root.join("rules/test.yml"), rules.to_string()).unwrap();
        let rules = RuleSet::load_from_dir(&root.join("rules")).unwrap();
        Self { root, rules }
    }

    fn file(&self, name: &str, text: &str) -> ScannableFile {
        ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn regex_accepts_code_after_thirty_two_comment_matches() {
    let f = Fixture::new("regex", "code", "RS256");
    let file = f.file("app.js", &("// RS256\n".repeat(32) + "'RS256';\n"));
    let result = RegexDetector.detect_detailed(&file, &f.rules).unwrap();
    assert_eq!(result.detections().len(), 1);
    assert_eq!(result.status(), DetectorStatus::Complete);
    assert_eq!(result.detections()[0].location.line, 33);
    assert_eq!(RegexDetector.detect(&file, &f.rules).unwrap().len(), 1);
}

#[test]
fn regex_distinguishes_exact_hit_cap_from_proven_truncation() {
    let f = Fixture::new("regex", "code", "RS256");
    for (n, expected) in [
        (0, DetectorStatus::Complete),
        (31, DetectorStatus::Complete),
        (32, DetectorStatus::Complete),
        (33, DetectorStatus::Truncated),
        (40, DetectorStatus::Truncated),
    ] {
        let result = RegexDetector
            .detect_detailed(&f.file("app.txt", &"RS256\n".repeat(n)), &f.rules)
            .unwrap();
        assert_eq!(result.status(), expected, "{n}");
        assert_eq!(result.detections().len(), n.min(32));
        if n > 0 {
            assert_eq!(result.detections().last().unwrap().location.line, n.min(32));
        }
    }
}

#[test]
fn comments_after_exact_regex_cap_do_not_prove_an_omitted_hit() {
    let f = Fixture::new("regex", "code", "RS256");
    let text = "'RS256';\n".repeat(32) + &"// RS256\n".repeat(20);
    let result = RegexDetector
        .detect_detailed(&f.file("app.js", &text), &f.rules)
        .unwrap();
    assert_eq!(result.detections().len(), 32);
    assert_eq!(result.status(), DetectorStatus::Complete);
}

#[test]
fn regex_work_budget_bounds_comment_only_search_without_claiming_missing_hits() {
    let f = Fixture::new("regex", "code", "RS256");
    for (n, expected) in [
        (4095, DetectorStatus::Complete),
        (4096, DetectorStatus::Complete),
        (4097, DetectorStatus::Truncated),
    ] {
        let result = RegexDetector
            .detect_detailed(&f.file("app.js", &"// RS256\n".repeat(n)), &f.rules)
            .unwrap();
        assert!(result.detections().is_empty());
        assert_eq!(result.status(), expected, "{n}");
    }
}

#[test]
fn truncated_regex_rule_does_not_prevent_later_rules_from_running() {
    let rule = |id, pattern| {
        serde_json::json!({
            "id": id, "kind": "regex", "category": "CryptoAPI", "severity": "high",
            "risk": "quantum-vulnerable", "confidence": 0.9, "migration_hint": "Review.",
            "pattern": pattern, "scope": "code"
        })
    };
    let f = Fixture::with_rules(serde_json::json!([
        rule("FIRST", "RS256"),
        rule("SECOND", "RSA")
    ]));
    let result = RegexDetector
        .detect_detailed(
            &f.file("app.txt", &("RS256\n".repeat(40) + "RSA\n")),
            &f.rules,
        )
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Truncated);
    assert_eq!(result.detections().len(), 33);
    assert_eq!(
        result
            .detections()
            .iter()
            .filter(|d| d.rule_id == "SECOND")
            .count(),
        1
    );
}

#[test]
fn key_limit_is_eight_masked_headers_and_ninth_proves_truncation() {
    let f = Fixture::new("key", "key", "-----BEGIN RSA PRIVATE KEY-----");
    let detector = KeyDetector::default();
    for (n, expected) in [
        (0, DetectorStatus::Complete),
        (7, DetectorStatus::Complete),
        (8, DetectorStatus::Complete),
        (9, DetectorStatus::Truncated),
    ] {
        let file = f.file(
            "id_rsa",
            &"-----BEGIN RSA PRIVATE KEY-----\nSYNTHETIC_BODY_DO_NOT_EXPORT\n".repeat(n),
        );
        let result = detector.detect_detailed(&file, &f.rules).unwrap();
        assert_eq!(result.status(), expected, "{n}");
        assert_eq!(result.detections().len(), n.min(8));
        assert_eq!(detector.detect(&file, &f.rules).unwrap().len(), n.min(8));
        for d in result.detections() {
            assert_eq!(d.evidence.r#match, "[masked-private-key]");
            assert_eq!(d.evidence.snippet_preview, "[masked-private-key-material]");
            assert!(!serde_json::to_string(&d.evidence)
                .unwrap()
                .contains("SYNTHETIC_BODY"));
        }
    }
}

#[test]
fn ast_counts_unique_eligible_hits_and_looks_past_duplicates_at_exact_cap() {
    let f = Fixture::new("tree_sitter", "javascript", "RS256");
    for (n, expected) in [
        (0, DetectorStatus::Complete),
        (31, DetectorStatus::Complete),
        (32, DetectorStatus::Complete),
        (33, DetectorStatus::Truncated),
        (40, DetectorStatus::Truncated),
    ] {
        let result = TreeSitterDetector
            .detect_detailed(&f.file("app.js", &"crypto('RS256');\n".repeat(n)), &f.rules)
            .unwrap();
        assert_eq!(result.status(), expected, "{n}");
        assert_eq!(result.detections().len(), n.min(32));
        if n > 0 {
            assert_eq!(result.detections()[0].location.line, n);
        }
    }
}

#[test]
fn ast_collection_exact_limit_can_complete_and_one_more_is_truncated() {
    let f = Fixture::new("tree_sitter", "javascript", "RS256");
    for (n, expected) in [
        (4096, DetectorStatus::Complete),
        (4097, DetectorStatus::Truncated),
    ] {
        let result = TreeSitterDetector
            .detect_detailed(&f.file("app.js", &"x;\n".repeat(n)), &f.rules)
            .unwrap();
        assert!(result.detections().is_empty());
        assert_eq!(result.status(), expected, "{n}");
    }
}

#[test]
fn ast_hit_limit_does_not_prevent_a_later_rule_from_running() {
    let rule = |id, pattern| {
        serde_json::json!({
            "id": id, "kind": "tree_sitter", "category": "CryptoAPI", "severity": "high",
            "risk": "quantum-vulnerable", "confidence": 0.9, "migration_hint": "Review.",
            "pattern": pattern, "scope": "javascript"
        })
    };
    let f = Fixture::with_rules(serde_json::json!([
        rule("FIRST", "RS256"),
        rule("SECOND", "RSA")
    ]));
    let text = "crypto('RS256');\n".repeat(40) + "crypto('RSA');\n";
    let result = TreeSitterDetector
        .detect_detailed(&f.file("app.js", &text), &f.rules)
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Truncated);
    assert_eq!(result.detections().len(), 33);
    let later = result
        .detections()
        .iter()
        .find(|d| d.rule_id == "SECOND")
        .unwrap();
    assert_eq!(later.location.line, 41);
}

#[test]
fn ast_parser_errors_keep_obtainable_hits_and_report_partial_inspection() {
    let f = Fixture::new("tree_sitter", "javascript", "RS256");
    let result = TreeSitterDetector
        .detect_detailed(
            &f.file("app.js", "crypto('RS256');\nfunction broken( {\n"),
            &f.rules,
        )
        .unwrap();
    assert_eq!(result.status(), DetectorStatus::Partial);
    assert!(!result.detections().is_empty());
}

#[test]
fn ast_clipped_text_and_collection_limits_are_not_complete_zero_hits() {
    let f = Fixture::new("tree_sitter", "javascript", "RS256");
    for text in [
        format!("crypto('{}');", "x".repeat(2200)),
        "x;\n".repeat(5000),
        ";".repeat(100001),
    ] {
        let result = TreeSitterDetector
            .detect_detailed(&f.file("app.js", &text), &f.rules)
            .unwrap();
        assert_eq!(result.status(), DetectorStatus::Truncated);
        assert!(result.detections().is_empty());
    }
}

#[test]
fn ast_match_work_budget_applies_across_duplicate_and_rejected_candidates() {
    let f = Fixture::new("tree_sitter", "javascript", "a");
    for (n, expected) in [
        (1300, DetectorStatus::Complete),
        (1500, DetectorStatus::Truncated),
    ] {
        // Each parser node sees the same long identifier fragment. These are
        // rejected embedded matches, not thousands of actual crypto usages.
        let text = format!("crypto('{}');", "a".repeat(n));
        let result = TreeSitterDetector
            .detect_detailed(&f.file("app.js", &text), &f.rules)
            .unwrap();
        assert!(result.detections().is_empty());
        assert_eq!(result.status(), expected, "{n}");
    }
}

#[test]
fn ast_clipping_is_unicode_safe_and_only_over_the_character_limit() {
    let f = Fixture::new("tree_sitter", "javascript", "RS256");
    for (n, expected) in [
        (1990, DetectorStatus::Complete),
        (1991, DetectorStatus::Truncated),
    ] {
        let text = format!("crypto('{}')", "あ".repeat(n));
        let result = TreeSitterDetector
            .detect_detailed(&f.file("app.js", &text), &f.rules)
            .unwrap();
        assert!(result.detections().is_empty());
        assert_eq!(result.status(), expected, "{n}");
    }
}

#[test]
fn inapplicable_inputs_are_explicit_but_plain_supported_text_can_complete() {
    let f = Fixture::new("regex", "code", "RS256");
    let plain = f.file("app.txt", "nothing cryptographic");
    assert_eq!(
        RegexDetector
            .detect_detailed(&plain, &f.rules)
            .unwrap()
            .status(),
        DetectorStatus::Complete
    );
    assert_eq!(
        TreeSitterDetector
            .detect_detailed(&plain, &f.rules)
            .unwrap()
            .status(),
        DetectorStatus::NotApplicable
    );
    assert_eq!(
        KeyDetector::default()
            .detect_detailed(&plain, &f.rules)
            .unwrap()
            .status(),
        DetectorStatus::NotApplicable
    );
    let binary = ScannableFile::from_bytes(PathBuf::from("input.bin"), vec![0, 1, 2]);
    for detector in [
        Box::new(RegexDetector) as Box<dyn Detector>,
        Box::new(TreeSitterDetector),
        Box::new(KeyDetector::default()),
    ] {
        assert_eq!(
            detector
                .detect_detailed(&binary, &f.rules)
                .unwrap()
                .status(),
            DetectorStatus::NotApplicable
        );
    }
    assert_eq!(
        RegexDetector
            .detect_detailed(&plain, &RuleSet::default())
            .unwrap()
            .status(),
        DetectorStatus::NotApplicable
    );
}

#[test]
fn real_detector_outcomes_reach_pipeline_and_legacy_unknowns_remain_visible() {
    let f = Fixture::new("regex", "code", "RS256");
    let root = f.root.join("input");
    std::fs::write(root.join("app.js"), "'RS256';\n").unwrap();
    let config = ScanConfig {
        root: root.clone(),
        threads: 1,
        ..ScanConfig::default()
    };
    let supported: Vec<Arc<dyn Detector>> = vec![
        Arc::new(RegexDetector),
        Arc::new(TreeSitterDetector),
        Arc::new(KeyDetector::default()),
    ];
    let complete = scan_repository_detailed(&config, &f.rules, &supported).unwrap();
    assert_eq!(
        complete.coverage().status(),
        InspectionStatus::CompleteWithinScope
    );
    assert_eq!(complete.legacy().findings.len(), 1);
    let defaults = scan_repository_detailed(&config, &f.rules, &default_detectors()).unwrap();
    assert_eq!(defaults.coverage().status(), InspectionStatus::Unknown);
    assert_eq!(
        defaults
            .diagnostics()
            .iter()
            .filter(|d| d.code() == DiagnosticCode::DetectorNotReported)
            .count(),
        1
    );
    std::fs::write(root.join("app.js"), "'RS256';\n".repeat(40)).unwrap();
    let truncated = scan_repository_detailed(&config, &f.rules, &supported).unwrap();
    assert_eq!(truncated.coverage().status(), InspectionStatus::Partial);
    assert_eq!(truncated.coverage().partial_files(), 1);
    assert_eq!(truncated.legacy().findings.len(), 32);
    assert!(truncated
        .diagnostics()
        .iter()
        .any(|d| d.code() == DiagnosticCode::DetectorTruncated));
}
