//! Independent source positions for the audit's legacy TLS counterexample.
use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{regex_detector::RegexDetector, DetectorBudget};
use pqc_scan_rules::RuleSet;
use std::path::{Path, PathBuf};
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .unwrap()
}
fn file(text: &str) -> ScannableFile {
    ScannableFile::from_bytes(PathBuf::from("tls.conf"), text.as_bytes().to_vec())
}
#[test]
fn adjacent_legacy_names_point_to_tokens_not_delimiters() {
    for (text, expected) in [
        ("\nTLSv1.0 TLSv1.0\nTLSv1.0\n", vec![(2, 1), (2, 9), (3, 1)]),
        (
            "\r\nTLSv1.0 TLSv1.0\r\nTLSv1.0\r\n",
            vec![(2, 1), (2, 9), (3, 1)],
        ),
        ("TLSv1", vec![(1, 1)]),
        ("TLSv1_0,TLSv1.0;TLSv1", vec![(1, 1), (1, 9), (1, 17)]),
        ("雪 TLSv1.0 雪 TLSv1.0", vec![(1, 3), (1, 13)]),
    ] {
        let out = RegexDetector
            .detect_observed(&file(text), &rules())
            .unwrap();
        let hits: Vec<_> = out
            .detections()
            .iter()
            .filter(|d| d.rule_id == "TLS_VERSION_1_0")
            .collect();
        assert_eq!(
            hits.iter()
                .map(|d| (d.location.line, d.location.column))
                .collect::<Vec<_>>(),
            expected,
            "{text:?}"
        );
        for h in hits {
            assert!(matches!(
                h.evidence.r#match.as_str(),
                "TLSv1" | "TLSv1.0" | "TLSv1_0"
            ));
        }
    }
}
#[test]
fn later_versions_identifier_fragments_and_comment_only_lines_are_negative() {
    for text in [
        "TLSv1.1 TLSv1.2 TLSv1.3",
        "xTLSv1.0 TLSv1.0x TLSv1_0_",
        "TLSv1.01 TLSv1.0.1",
        "# TLSv1.0\n# TLSv1.0",
    ] {
        assert!(
            RegexDetector
                .detect(&file(text), &rules())
                .unwrap()
                .iter()
                .all(|d| d.rule_id != "TLS_VERSION_1_0"),
            "{text:?}"
        );
    }
}
#[test]
fn real_token_candidates_respect_hit_and_candidate_caps() {
    let text = "TLSv1.0 TLSv1.0 TLSv1.0";
    for (hits, candidates, count, status) in [
        (0, 9, 0, DetectorStatus::Truncated),
        (1, 9, 1, DetectorStatus::Truncated),
        (9, 1, 1, DetectorStatus::Truncated),
        (3, 3, 3, DetectorStatus::Complete),
        (4, 4, 3, DetectorStatus::Complete),
    ] {
        let out = RegexDetector::with_budget(DetectorBudget {
            max_hits_per_rule: hits,
            max_match_candidates: candidates,
            ..Default::default()
        })
        .detect_detailed(&file(text), &rules())
        .unwrap();
        assert_eq!(
            out.detections()
                .iter()
                .filter(|d| d.rule_id == "TLS_VERSION_1_0")
                .count(),
            count
        );
        assert_eq!(out.status(), status);
    }
}
#[test]
fn customized_pattern_scope_and_other_rules_keep_generic_ranges() {
    let dir = std::env::temp_dir().join(format!("pqc-tls-custom-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let original = rules().get("TLS_VERSION_1_0").unwrap().clone();
    for (scope, pattern) in [
        ("custom", original.pattern.as_str()),
        (
            "code",
            r"(?:^|[^A-Za-z0-9_])TLSv1(?:[._]0)?(?:$|[^A-Za-z0-9_.])",
        ),
        ("code", r"TLSv1\.0"),
        ("code", r"a*"),
    ] {
        let mut rule = original.clone();
        rule.scope = scope.into();
        rule.pattern = pattern.into();
        for id in ["TLS_VERSION_1_0", "CUSTOM_TLS"] {
            rule.id = id.into();
            std::fs::write(
                dir.join("rules.yml"),
                serde_yaml::to_string(&vec![rule.clone()]).unwrap(),
            )
            .unwrap();
            let custom = RuleSet::load_from_dir(&dir).unwrap();
            let text = "\nTLSv1.0 TLSv1.0\nTLSv1.0 雪";
            let expected: Vec<_> = regex::Regex::new(pattern)
                .unwrap()
                .find_iter(text)
                .map(|m| {
                    let (l, c) = pqc_scan_core::model::line_col_for_offset(text, m.start());
                    (l, c, pqc_scan_core::model::mask_preview(m.as_str()))
                })
                .collect();
            let actual = RegexDetector.detect(&file(text), &custom).unwrap();
            assert_eq!(
                actual
                    .iter()
                    .map(|d| (
                        d.location.line,
                        d.location.column,
                        d.evidence.r#match.clone()
                    ))
                    .collect::<Vec<_>>(),
                expected,
                "{scope} {id} {pattern}"
            );
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn existing_comment_policy_depends_on_the_source_language() {
    for (path, count) in [("use.js", 0), ("tls.conf", 1)] {
        let input = ScannableFile::from_bytes(PathBuf::from(path), b"// TLSv1.0\n".to_vec());
        assert_eq!(
            RegexDetector
                .detect(&input, &rules())
                .unwrap()
                .iter()
                .filter(|d| d.rule_id == "TLS_VERSION_1_0")
                .count(),
            count
        );
    }
}
