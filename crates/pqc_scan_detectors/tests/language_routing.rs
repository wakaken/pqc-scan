use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{tree_sitter_detector::TreeSitterDetector, DetectorBudget};
use pqc_scan_rules::RuleSet;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

struct Rules(PathBuf, RuleSet);
impl Rules {
    fn new(scope: &str) -> Self {
        Self::with_pattern(scope, "CONTROL")
    }
    fn with_pattern(scope: &str, pattern: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "pqc-v05-route-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("rule.yml"),serde_json::json!([{"id":"ROUTE","kind":"tree_sitter","scope":scope,"pattern":pattern,"category":"test","severity":"info","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review"}]).to_string()).unwrap();
        let rules = RuleSet::load_from_dir(&dir).unwrap();
        Self(dir, rules)
    }
}
impl Drop for Rules {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn file(name: &str, source: &str) -> ScannableFile {
    ScannableFile::from_bytes(name.into(), source.as_bytes().to_vec())
}
#[test]
fn all_new_routes_use_real_ast_candidates_and_custom_scope() {
    for (name, lang, source) in [
        ("a.php", "php", "<?php $CONTROL = 1;"),
        ("a.phtml", "php", "<p>html</p><?php $CONTROL = 1; ?>"),
        ("a.inc", "php", "<?php $CONTROL = 1;"),
        ("a.cs", "csharp", "namespace App; class C { int CONTROL; }"),
        ("a.c", "c", "int CONTROL;"),
        ("a.C", "cpp", "namespace App { int CONTROL; }"),
        ("a.cc", "cpp", "namespace App { int CONTROL; }"),
        ("a.cpp", "cpp", "namespace App { int CONTROL; }"),
        ("a.cxx", "cpp", "namespace App { int CONTROL; }"),
        ("a.hpp", "cpp", "class CONTROL {};"),
        ("a.hh", "cpp", "class CONTROL {};"),
        ("a.hxx", "cpp", "class CONTROL {};"),
        ("a.h", "c,cpp", "int CONTROL;"),
    ] {
        let rules = Rules::new(lang);
        let o = TreeSitterDetector
            .detect_detailed(&file(name, source), &rules.1)
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Complete, "{name}: {o:?}");
        assert_eq!(o.detections().len(), 1, "{name}: {o:?}");
        assert_eq!(o.detections()[0].evidence.metadata["language"], lang);
    }
    let r = Rules::new("c");
    assert!(TreeSitterDetector
        .detect(&file("a.C", "int CONTROL;"), &r.1)
        .unwrap()
        .is_empty());
    let r = Rules::new("cpp");
    assert!(TreeSitterDetector
        .detect(&file("a.c", "int CONTROL;"), &r.1)
        .unwrap()
        .is_empty());
}
#[test]
fn php_tags_templates_strings_and_original_unicode_positions() {
    let r = Rules::new("php");
    for (name, text) in [
        ("a.inc", "CONTROL"),
        ("a.inc", "&lt;?php CONTROL"),
        ("a.twig", "<?php $CONTROL=1;"),
        ("a.php", "<p>CONTROL</p>"),
    ] {
        let o = TreeSitterDetector
            .detect_detailed(&file(name, text), &r.1)
            .unwrap();
        assert!(o.detections().is_empty(), "{name}: {o:?}");
    }
    let text = "<p>日本語</p><?php $CONTROL=1; $CONTROL=2; ?>";
    let o = TreeSitterDetector
        .detect_detailed(&file("a.phtml", text), &r.1)
        .unwrap();
    assert_eq!(o.detections().len(), 2, "{o:?}");
    let mut actual: Vec<_> = o
        .detections()
        .iter()
        .map(|d| (d.location.line, d.location.column))
        .collect();
    actual.sort();
    let expected: Vec<_> = text
        .match_indices("CONTROL")
        .map(|(n, _)| pqc_scan_core::model::line_col_for_offset(text, n))
        .collect();
    assert_eq!(actual, expected);
    let o = TreeSitterDetector
        .detect_detailed(
            &file("a.blade.php", "@if($ok) <?php $CONTROL=1; ?> @endif"),
            &r.1,
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert_eq!(o.detections().len(), 1);
    assert!(o.diagnostic_message().unwrap().contains("template"));
}
#[test]
fn malformed_and_ambiguous_headers_remain_incomplete() {
    for (name, lang, text) in [
        ("a.php", "php", "<?php $CONTROL=1; broken( ;"),
        ("a.cs", "csharp", "class C { int CONTROL; void M( {"),
        ("a.c", "c", "int CONTROL; void f( {"),
        ("a.cpp", "cpp", "int CONTROL; namespace { void f( {"),
    ] {
        let r = Rules::new(lang);
        let o = TreeSitterDetector
            .detect_detailed(&file(name, text), &r.1)
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial, "{name}");
        assert_eq!(o.detections().len(), 1, "{name}: {o:?}");
    }
    let r = Rules::new("c,cpp");
    let o = TreeSitterDetector
        .detect_detailed(&file("a.h", "namespace App { int CONTROL; }"), &r.1)
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert!(o.diagnostic_message().is_some());
}
#[test]
fn header_parses_share_limits_and_do_not_duplicate_occurrences() {
    let r = Rules::new("c,cpp");
    let input = file("a.h", "int CONTROL; int CONTROL2;");
    for limit in [0, 1, 2] {
        let budget = DetectorBudget {
            max_hits_per_rule: limit,
            ..Default::default()
        };
        let o = TreeSitterDetector::with_budget(budget)
            .detect_detailed(&input, &r.1)
            .unwrap();
        assert!(o.detections().len() <= limit, "{o:?}");
        if limit == 0 {
            assert_eq!(o.status(), DetectorStatus::Truncated);
        }
    }
    // Two genuinely separate references on one line; CONTROL2 is not a submatch.
    let o = TreeSitterDetector
        .detect_detailed(&file("a.h", "int CONTROL; int other = CONTROL;"), &r.1)
        .unwrap();
    assert_eq!(o.detections().len(), 2);
    for name in ["a.php", "a.cs", "a.c", "a.cpp", "a.h"] {
        let source = if name.ends_with("php") {
            "<?php $CONTROL=1;"
        } else {
            "int CONTROL;"
        };
        let r = Rules::new("php,csharp,c,cpp");
        let o = TreeSitterDetector::with_budget(DetectorBudget {
            max_ast_nodes: 0,
            ..Default::default()
        })
        .detect_detailed(&file(name, source), &r.1)
        .unwrap();
        assert_eq!(o.status(), DetectorStatus::Truncated, "{name}");
    }
}

#[test]
fn encoded_and_unreadable_new_source_never_gets_false_complete() {
    let r = Rules::new("php,csharp,c,cpp");
    for (name, text) in [
        ("a.php", "<?php /* 日本語 */ $CONTROL=1;"),
        ("a.cs", "class C { /* 日本語 */ int CONTROL; }"),
        ("a.c", "/* 日本語 */ int CONTROL;"),
        ("a.cpp", "/* 日本語 */ int CONTROL;"),
        ("a.h", "/* 日本語 */ int CONTROL;"),
    ] {
        let mut bytes = vec![0xff, 0xfe];
        for u in text.encode_utf16() {
            bytes.extend(u.to_le_bytes());
        }
        let input = ScannableFile::from_bytes(name.into(), bytes);
        let o = TreeSitterDetector.detect_observed(&input, &r.1).unwrap();
        assert_eq!(o.status(), DetectorStatus::Complete, "{name}: {o:?}");
        assert_eq!(o.detections().len(), 1);
        let offset = text.find("CONTROL").unwrap();
        assert_eq!(
            o.detections()[0].location.column,
            pqc_scan_core::model::line_col_for_offset(text, offset).1
        );
        assert_eq!(
            o.observations()[0].source_span(),
            Some((offset, offset + 7))
        );
    }
    for name in ["a.php", "a.cs", "a.c", "a.cpp", "a.h"] {
        let input = ScannableFile::from_bytes(name.into(), vec![0xff, 0x00, 0xfd]);
        assert!(input.text.is_none());
        let o = TreeSitterDetector.detect_detailed(&input, &r.1).unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial, "{name}");
        assert!(o.diagnostic_message().unwrap().contains("decoded"));
    }
}
#[test]
fn all_shared_header_work_caps_are_visible_on_exhaustion() {
    let r = Rules::new("c,cpp");
    let input = file("a.h", "void f(void) { CONTROL(); CONTROL(); }");
    for field in 0..5 {
        for n in [0, 1, 2] {
            let mut b = DetectorBudget::default();
            match field {
                0 => b.max_ast_nodes = n,
                1 => b.max_ast_candidates = n,
                2 => b.max_match_candidates = n,
                3 => b.max_ast_match_bytes = n,
                _ => b.max_hits_per_rule = n,
            };
            let o = TreeSitterDetector::with_budget(b)
                .detect_detailed(&input, &r.1)
                .unwrap();
            assert_eq!(
                o.status(),
                DetectorStatus::Truncated,
                "field {field} cap {n}: {o:?}"
            );
            assert!(o.diagnostic_message().is_some());
        }
    }
    let hostile = format!("void f(void) {{ {} }}", "CONTROL();".repeat(3000));
    let o = TreeSitterDetector::with_budget(DetectorBudget {
        max_ast_nodes: 64,
        ..Default::default()
    })
    .detect_detailed(&file("a.h", &hostile), &r.1)
    .unwrap();
    assert_eq!(o.status(), DetectorStatus::Truncated);
}

#[test]
fn native_capture_maps_encoded_source_spans_to_original_file_bytes() {
    use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
    use std::sync::Arc;
    let r = Rules::new("php,csharp,c,cpp");
    let root = r.0.join("input");
    std::fs::create_dir_all(&root).unwrap();
    for (name, text) in [
        ("a.php", "<?php /* 日本語 */ $CONTROL=1;"),
        ("a.cs", "class C { /* 日本語 */ int CONTROL; }"),
        ("a.c", "/* 日本語 */ int CONTROL;"),
        ("a.cpp", "/* 日本語 */ int CONTROL;"),
        ("a.h", "/* 日本語 */ int CONTROL;"),
    ] {
        let mut bytes = vec![0xff, 0xfe];
        for u in text.encode_utf16() {
            bytes.extend(u.to_le_bytes());
        }
        std::fs::write(root.join(name), bytes).unwrap();
        let ds: Vec<Arc<dyn Detector>> = vec![Arc::new(TreeSitterDetector)];
        let doc = scan_repository_native(
            &ScanConfig {
                root: root.clone(),
                threads: 1,
                ..Default::default()
            },
            &r.1,
            &ds,
            &NativeOptions::new(Some("v05/coordinates")).unwrap(),
        )
        .unwrap();
        let native = serde_json::to_value(doc.native().unwrap()).unwrap();
        let observations = native["observations"].as_array().unwrap();
        assert_eq!(observations.len(), 1, "{name}");
        let target = &observations[0]["target"];
        let start = 2 + text[..text.find("CONTROL").unwrap()].encode_utf16().count() * 2;
        assert_eq!(target["byte_start"], start, "{name}");
        assert_eq!(target["byte_end"], start + 14, "{name}");
        assert_eq!(target["column_kind"], "unicode_code_points");
        std::fs::remove_file(root.join(name)).unwrap();
    }
}

#[test]
fn header_exact_shared_hit_boundary_retains_both_source_occurrences() {
    let r = Rules::new("c,cpp");
    let input = file("a.h", "void f(void) { CONTROL(); CONTROL(); }");
    // Each candidate view needs two hits. The total shared allocation is four;
    // facts at identical source spans are then retained only once.
    for (limit, status, count) in [
        (3, DetectorStatus::Truncated, 1),
        (4, DetectorStatus::Complete, 2),
        (5, DetectorStatus::Complete, 2),
    ] {
        let o = TreeSitterDetector::with_budget(DetectorBudget {
            max_hits_per_rule: limit,
            ..Default::default()
        })
        .detect_detailed(&input, &r.1)
        .unwrap();
        assert_eq!(o.status(), status, "limit {limit}");
        assert_eq!(o.detections().len(), count, "limit {limit}");
    }
}

#[test]
fn quoted_custom_matches_keep_original_span_and_code_point_column() {
    let r = Rules::with_pattern("php,csharp,c,cpp", r#"["']CONTROL["']"#);
    for (name, source) in [
        ("a.php", r#"<p>日本語</p><?php $a="CONTROL";"#),
        ("a.cs", r#"class C { string a="CONTROL"; }"#),
        ("a.c", r#"char *a="CONTROL";"#),
        ("a.cpp", r#"const char *a="CONTROL";"#),
        ("a.h", r#"const char *a="CONTROL";"#),
    ] {
        let o = TreeSitterDetector
            .detect_observed(&file(name, source), &r.1)
            .unwrap();
        let offset = source.find("CONTROL").unwrap();
        assert_eq!(o.detections().len(), 1, "{name}: {o:?}");
        assert_eq!(
            o.observations()[0].source_span(),
            Some((offset, offset + 7)),
            "{name}"
        );
        assert_eq!(
            o.detections()[0].location.column,
            pqc_scan_core::model::line_col_for_offset(source, offset).1,
            "{name}"
        );
    }
}

#[test]
fn large_unicode_header_keeps_locations_without_per_hit_prefix_scans() {
    let r = Rules::new("c,cpp");
    let prefix = "/* 💻 */".repeat(50_000);
    let source = format!("{prefix}\nvoid f(void) {{ {} }}", "CONTROL();".repeat(2000));
    let o = TreeSitterDetector
        .detect_observed(&file("a.h", &source), &r.1)
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert_eq!(o.detections().len(), 2000);
    let mut actual: Vec<_> = o
        .detections()
        .iter()
        .zip(o.observations())
        .map(|(d, o)| (o.source_span().unwrap(), d.location.line, d.location.column))
        .collect();
    actual.sort_by_key(|r| r.0);
    let base = prefix.len() + 1 + "void f(void) { ".len();
    for (i, (span, line, column)) in actual.into_iter().enumerate() {
        assert_eq!(span, (base + i * 10, base + i * 10 + 7));
        assert_eq!(line, 2);
        assert_eq!(column, "void f(void) { ".len() + 1 + i * 10);
    }
    let o = TreeSitterDetector::with_budget(DetectorBudget {
        max_ast_nodes: 8,
        ..Default::default()
    })
    .detect_detailed(
        &file("a.php", "<?php /* 💻💻💻💻💻💻💻💻💻💻 */ $CONTROL=1;"),
        &Rules::new("php").1,
    )
    .unwrap();
    assert_eq!(o.status(), DetectorStatus::Truncated);
    assert!(o
        .diagnostic_message()
        .unwrap()
        .contains("source_position_entries"));
}

#[test]
fn every_shared_work_budget_has_an_explicit_exact_boundary() {
    // Pinned C/C++ grammars both parse this as five nodes (including root and
    // semicolon), with one semantic identifier and seven matched source bytes.
    // Two candidate views therefore need totals of 10 nodes, 2 candidates,
    // 2 match candidates and 14 retained-match bytes; output is one occurrence.
    let text = "int CONTROL;";
    for grammar in [
        tree_sitter::Language::from(tree_sitter_c::LANGUAGE),
        tree_sitter::Language::from(tree_sitter_cpp::LANGUAGE),
    ] {
        let mut p = tree_sitter::Parser::new();
        p.set_language(&grammar).unwrap();
        let tree = p.parse(text, None).unwrap();
        let mut stack = vec![tree.root_node()];
        let mut count = 0;
        while let Some(n) = stack.pop() {
            count += 1;
            let mut c = n.walk();
            stack.extend(n.children(&mut c));
        }
        assert_eq!(count, 5);
        assert!(!tree.root_node().has_error());
    }
    let r = Rules::new("c,cpp");
    let input = file("boundary.h", text);
    for (field, exact) in [
        ("nodes", 10),
        ("ast_candidates", 2),
        ("match_candidates", 2),
        ("bytes", 14),
    ] {
        for limit in [exact - 1, exact, exact + 1] {
            let mut b = DetectorBudget::default();
            match field {
                "nodes" => b.max_ast_nodes = limit,
                "ast_candidates" => b.max_ast_candidates = limit,
                "match_candidates" => b.max_match_candidates = limit,
                _ => b.max_ast_match_bytes = limit,
            };
            let o = TreeSitterDetector::with_budget(b)
                .detect_observed(&input, &r.1)
                .unwrap();
            let full = limit >= exact;
            assert_eq!(
                o.status(),
                if full {
                    DetectorStatus::Complete
                } else {
                    DetectorStatus::Truncated
                },
                "{field}={limit}: {o:?}"
            );
            assert_eq!(o.detections().len(), usize::from(full), "{field}={limit}");
            assert_eq!(o.observations().len(), usize::from(full), "{field}={limit}");
            if full {
                assert_eq!(o.observations()[0].source_span(), Some((4, 11)));
                assert_eq!(
                    (
                        o.detections()[0].location.line,
                        o.detections()[0].location.column
                    ),
                    (1, 5)
                );
            } else {
                assert!(o.diagnostic_message().is_some());
            }
        }
    }
}
