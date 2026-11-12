use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
struct Rules {
    dir: PathBuf,
    rules: RuleSet,
}
impl Rules {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "pqc-inspection-repairs-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("rules.yml"),r#"[{"id":"TEST","kind":"tree_sitter","category":"test","severity":"high","risk":"quantum-vulnerable","confidence":0.9,"migration_hint":"Review","pattern":"RS256","scope":"javascript,typescript,python"}]"#).unwrap();
        let rules = RuleSet::load_from_dir(&dir).unwrap();
        Self { dir, rules }
    }
    fn inspect(&self, name: &str, text: &str) -> pqc_scan_core::DetectorOutcome {
        TreeSitterDetector
            .detect_detailed(
                &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
                &self.rules,
            )
            .unwrap()
    }
}
impl Drop for Rules {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}
#[test]
fn crypto_after_long_candidate_prefix_is_inspected() {
    let r = Rules::new();
    let text = format!("const value = '{} RS256';", "x".repeat(3000));
    let o = r.inspect("long.js", &text);
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert_eq!(o.detections().len(), 1);
    assert_eq!(
        o.detections()[0].location.column,
        text.find("RS256").unwrap() + 1
    );
}
#[test]
fn repository_sized_occurrence_counts_are_not_silently_capped_at_32() {
    let r = Rules::new();
    let o = r.inspect("many.js", &"use('RS256');\n".repeat(80));
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert_eq!(o.detections().len(), 80);
}
#[test]
fn repository_sized_ast_does_not_stop_at_4096_candidates() {
    let r = Rules::new();
    let o = r.inspect("many.js", &("x;\n".repeat(6000) + "use('RS256');"));
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert_eq!(o.detections().len(), 1);
}
#[test]
fn type_only_star_exports_preserve_later_findings_and_locations() {
    let r = Rules::new();
    for ext in ["ts", "tsx"] {
        let text = "export type * from './types';\nuse('RS256');\n";
        let o = r.inspect(&format!("app.{ext}"), text);
        assert_eq!(o.status(), DetectorStatus::Complete, "{ext}");
        assert_eq!(o.detections().len(), 1);
        assert_eq!(o.detections()[0].location.line, 2);
        assert_eq!(o.detections()[0].location.column, 6);
    }
}
#[test]
fn deferred_namespace_imports_preserve_original_locations() {
    let r = Rules::new();
    for ext in ["js", "mjs", "ts"] {
        let o = r.inspect(
            &format!("app.{ext}"),
            "import defer * as mod from './module';\nuse('RS256');",
        );
        assert_eq!(o.status(), DetectorStatus::Complete, "{ext}");
        assert_eq!(o.detections().len(), 1);
        assert_eq!(o.detections()[0].location.line, 2);
    }
}
#[test]
fn malformed_module_syntax_still_reports_partial_and_retains_hits() {
    let r = Rules::new();
    for text in [
        "import defer { x } from './module';\nuse('RS256');",
        "export type * from ;\nuse('RS256');",
        "use('RS256');\nfunction broken( {",
    ] {
        let o = r.inspect("app.ts", text);
        assert_eq!(o.status(), DetectorStatus::Partial, "{text}");
        assert!(!o.detections().is_empty());
    }
}
#[test]
fn pnpm_plain_scalar_wildcards_are_not_yaml_aliases() {
    let r = Rules::new();
    for value in ["6.* || 8.* || >= 10.*", "'6.* || 8.* || >= 10.*'"] {
        let text=format!("lockfileVersion: '9.0'\npackages:\n  example@1.0.0:\n    resolution: {{integrity: sha512-test}}\n    engines: {{node: {value}}}\n");
        let file = ScannableFile::from_bytes("pnpm-lock.yaml".into(), text.into_bytes());
        let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
        assert_eq!(o.status(), DetectorStatus::Complete, "{value}");
        assert_eq!(o.detections().len(), 1);
    }
}
#[test]
fn yaml_aliases_and_tags_remain_unsupported() {
    let r = Rules::new();
    for extra in ["x: &anchor value\ny: *anchor\n", "x: !custom value\n"] {
        let file = ScannableFile::from_bytes(
            "pnpm-lock.yaml".into(),
            format!("lockfileVersion: '9.0'\npackages: {{}}\n{extra}").into_bytes(),
        );
        let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
        assert_eq!(o.status(), DetectorStatus::Unsupported);
    }
}

#[test]
fn utf16_and_utf32_keep_findings_and_redact_decoded_source_snippets() {
    let r = Rules::new();
    let input = r.dir.join("input");
    std::fs::create_dir_all(&input).unwrap();
    let source = "const token = 'ENCODING_SYNTHETIC_CANARY'; // 日本😀\nuse('RS256');\n";
    for (name, bytes) in [
        (
            "utf16le.js",
            [
                vec![0xff, 0xfe],
                source.encode_utf16().flat_map(u16::to_le_bytes).collect(),
            ]
            .concat(),
        ),
        (
            "utf16be.js",
            [
                vec![0xfe, 0xff],
                source.encode_utf16().flat_map(u16::to_be_bytes).collect(),
            ]
            .concat(),
        ),
        (
            "utf32le.js",
            [
                vec![0xff, 0xfe, 0, 0],
                source
                    .chars()
                    .flat_map(|c| (c as u32).to_le_bytes())
                    .collect(),
            ]
            .concat(),
        ),
        (
            "utf32be.js",
            [
                vec![0, 0, 0xfe, 0xff],
                source
                    .chars()
                    .flat_map(|c| (c as u32).to_be_bytes())
                    .collect(),
            ]
            .concat(),
        ),
    ] {
        std::fs::write(input.join(name), bytes).unwrap();
    }
    let config = pqc_scan_core::ScanConfig {
        root: input,
        threads: 1,
        ..Default::default()
    };
    let d = pqc_scan_core::scan_repository_native(
        &config,
        &r.rules,
        &pqc_scan_detectors::default_detectors(),
        &pqc_scan_core::NativeOptions::new(Some("test/encoding")).unwrap(),
    )
    .unwrap();
    assert_eq!(d.coverage().analyzed_files(), 4);
    assert_eq!(d.legacy().findings.len(), 4);
    for f in &d.legacy().findings {
        assert_eq!(f.location.line, 2);
        assert_eq!(f.location.column, 6);
        assert!(f.source_snippet.is_some());
    }
    let legacy = serde_json::to_string(d.legacy()).unwrap();
    assert!(!legacy.contains("ENCODING_SYNTHETIC_CANARY"));
    let native = serde_json::to_string(d.native().unwrap()).unwrap();
    assert!(!native.contains("ENCODING_SYNTHETIC_CANARY"));
    let native_json: serde_json::Value = serde_json::from_str(&native).unwrap();
    for o in native_json["observations"].as_array().unwrap() {
        let path = o["target"]["path"].as_str().unwrap();
        let (width, bom) = if path.starts_with("utf16") {
            (2, 2)
        } else {
            (4, 4)
        };
        let prefix = &source[..source.find("RS256").unwrap()];
        let offset = bom
            + if width == 2 {
                prefix.encode_utf16().count() * 2
            } else {
                prefix.chars().count() * 4
            };
        assert_eq!(o["target"]["byte_start"].as_u64(), Some(offset as u64));
        assert_eq!(
            o["target"]["byte_end"].as_u64(),
            Some((offset + 5 * width) as u64)
        );
    }
}
#[test]
fn malformed_wide_source_is_not_silently_excluded_or_counted_complete() {
    let r = Rules::new();
    let input = r.dir.join("input");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::write(input.join("invalid.js"), [0xff, 0xfe, 0x00, 0xd8]).unwrap();
    let d = pqc_scan_core::scan_repository_detailed(
        &pqc_scan_core::ScanConfig {
            root: input,
            threads: 1,
            ..Default::default()
        },
        &r.rules,
        &pqc_scan_detectors::default_detectors(),
    )
    .unwrap();
    assert_eq!(d.coverage().selected_files(), 1);
    assert_eq!(d.coverage().unsupported_files(), 1);
    assert!(!d.coverage().is_complete());
    assert!(d
        .diagnostics()
        .iter()
        .any(|x| x.detector() == Some("text_decoder")));
}
#[test]
fn file_selection_opt_out_retains_custom_inspection_and_crypto_inputs() {
    let r = Rules::new();
    let input = r.dir.join("input");
    std::fs::create_dir_all(&input).unwrap();
    for (name, bytes) in [
        ("image.png", b"not actually an image".as_slice()),
        ("code.js", b"use('RS256');".as_slice()),
        ("module.wasm", b"\0asm\x01\0\0\0".as_slice()),
        ("store.jks", b"\0\x01\0\xff".as_slice()),
    ] {
        std::fs::write(input.join(name), bytes).unwrap();
    }
    let config = pqc_scan_core::ScanConfig {
        root: input,
        threads: 1,
        ..Default::default()
    };
    let detectors = pqc_scan_detectors::default_detectors();
    let normal = pqc_scan_core::scan_repository_detailed(&config, &r.rules, &detectors).unwrap();
    let all = pqc_scan_core::pipeline::scan_repository_with_selection(
        &config,
        &r.rules,
        &detectors,
        pqc_scan_core::FileSelection {
            include_non_scannable: true,
        },
        None,
    )
    .unwrap();
    assert_eq!(normal.coverage().selected_files(), 2);
    assert_eq!(normal.coverage().excluded_enumerated_files(), 2);
    assert_eq!(normal.coverage().not_applicable_files(), 1);
    assert_eq!(all.coverage().selected_files(), 4);
    assert_eq!(all.coverage().excluded_enumerated_files(), 0);
    assert_eq!(all.coverage().not_applicable_files(), 3);
    assert!(normal
        .diagnostics()
        .iter()
        .all(|d| d.file() != Some("store.jks")));
}
#[test]
fn finite_ast_work_budget_keeps_already_obtained_hits() {
    let r = Rules::new();
    let text = format!("use('{}');\nuse('RS256');", "x".repeat(300));
    let file = ScannableFile::from_bytes("code.js".into(), text.into_bytes());
    let o = TreeSitterDetector::with_budget(pqc_scan_detectors::DetectorBudget {
        max_ast_match_bytes: 100,
        ..Default::default()
    })
    .detect_detailed(&file, &r.rules)
    .unwrap();
    assert_eq!(o.status(), DetectorStatus::Truncated);
    assert_eq!(o.detections().len(), 1);
    assert_eq!(o.detections()[0].location.line, 2);
}
#[test]
fn module_compatibility_does_not_rewrite_bindings_strings_or_comments() {
    let r = Rules::new();
    for source in ["import defer from './mod';\nuse('RS256');", "export type /* comment */ * as Types from './types';\nuse('RS256');", "const s = 'import defer * as x from module';\n// export type * from invalid;\nuse('RS256');"] {let o=r.inspect("app.ts",source);assert_eq!(o.status(),DetectorStatus::Complete,"{source}");assert_eq!(o.detections().len(),1);}
}
#[test]
fn flow_yaml_alias_and_tag_after_quoted_key_remain_rejected() {
    let r = Rules::new();
    for doc in ["x: &a value\ny: {\"key\":*a}", "x: {\"key\":!custom value}"] {
        let file = ScannableFile::from_bytes(
            "pnpm-lock.yaml".into(),
            format!("lockfileVersion: '9.0'\npackages: {{}}\n{doc}\n").into_bytes(),
        );
        let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
        assert_eq!(o.status(), DetectorStatus::Unsupported, "{doc}");
    }
}

#[test]
fn encoded_pem_public_certificate_offsets_refer_to_original_bytes() {
    let r = Rules::new();
    let der = include_bytes!("fixtures/certificates/rsa2048.der");
    let source = format!(
        "\n{}",
        pem::encode(&pem::Pem::new("CERTIFICATE", der.to_vec()))
    );
    // Use the repository's existing certificate rules; the fixture is public test data.
    let rules =
        RuleSet::load_from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .unwrap();
    let mut utf8 = vec![0xef, 0xbb, 0xbf];
    utf8.extend(source.as_bytes());
    let cases = [
        (utf8, 4),
        (
            [
                vec![0xff, 0xfe],
                source.encode_utf16().flat_map(u16::to_le_bytes).collect(),
            ]
            .concat(),
            4,
        ),
        (
            [
                vec![0xfe, 0xff],
                source.encode_utf16().flat_map(u16::to_be_bytes).collect(),
            ]
            .concat(),
            4,
        ),
        (
            [
                vec![0xff, 0xfe, 0, 0],
                source
                    .chars()
                    .flat_map(|c| (c as u32).to_le_bytes())
                    .collect(),
            ]
            .concat(),
            8,
        ),
        (
            [
                vec![0, 0, 0xfe, 0xff],
                source
                    .chars()
                    .flat_map(|c| (c as u32).to_be_bytes())
                    .collect(),
            ]
            .concat(),
            8,
        ),
    ];
    for (bytes, expected) in cases {
        let file = ScannableFile::from_bytes(r.dir.join("certificate.pem"), bytes);
        let o = pqc_scan_detectors::certificate_detector::CertificateDetector
            .detect_detailed(&file, &rules)
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Complete);
        let certs: Vec<_> = o
            .detections()
            .iter()
            .filter(|d| d.evidence.metadata.contains_key("certificate_sha256"))
            .collect();
        assert!(!certs.is_empty());
        for d in certs {
            assert_eq!(d.location.line, 2);
            assert_eq!(
                d.evidence.metadata.get("byte_offset").map(String::as_str),
                Some(expected.to_string().as_str())
            );
        }
    }
}

#[test]
fn pnpm_empty_metadata_and_plain_apostrophes_do_not_hide_quoted_wildcards() {
    let r = Rules::new();
    for extra in ["optionalMetadata:\n", "deprecated: Use your platform's native implementation\npeerDependencies: {typescript: '*'}\n", "description: |\n  Literal &anchor and *alias are text here\n"] {
  let file=ScannableFile::from_bytes("pnpm-lock.yaml".into(),format!("lockfileVersion: '9.0'\npackages: {{}}\n{extra}").into_bytes());
  let o=DependencyDetector.detect_detailed(&file,&r.rules).unwrap();assert_eq!(o.status(),DetectorStatus::Complete,"{extra}");
 }
}
#[test]
fn repository_sized_dependency_values_are_supported_and_real_exhaustion_is_not_unsupported() {
    let r = Rules::new();
    let values = vec!["0"; 80_000].join(",");
    let file = ScannableFile::from_bytes(
        "pnpm-lock.yaml".into(),
        format!("lockfileVersion: '9.0'\npackages: {{}}\nmetadata: [{values}]\n").into_bytes(),
    );
    let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    let values = vec!["0"; 500_001].join(",");
    let file = ScannableFile::from_bytes(
        "pnpm-lock.yaml".into(),
        format!("lockfileVersion: '9.0'\npackages: {{}}\nmetadata: [{values}]\n").into_bytes(),
    );
    let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
    assert_eq!(o.status(), DetectorStatus::Truncated);
}

#[test]
fn pnpm_boot_and_project_documents_are_both_inventoried_without_overwriting() {
    let r = Rules::new();
    let text="---\nlockfileVersion: '9.0'\npackages:\n  tool-only@1.0.0:\n    resolution: {integrity: sha512-test}\n---\nlockfileVersion: '9.0'\npackages:\n  app-only@2.0.0:\n    resolution: {integrity: sha512-test}\n";
    let file = ScannableFile::from_bytes("pnpm-lock.yaml".into(), text.as_bytes().to_vec());
    let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert_eq!(o.detections().len(), 2);
    let names: std::collections::BTreeSet<_> = o
        .detections()
        .iter()
        .map(|d| d.evidence.metadata["dep_name"].as_str())
        .collect();
    assert_eq!(names, ["app-only", "tool-only"].into_iter().collect());
    let too_many = format!("{text}---\nlockfileVersion: '9.0'\npackages: {{}}\n");
    let file = ScannableFile::from_bytes("pnpm-lock.yaml".into(), too_many.into_bytes());
    let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
    assert_eq!(o.status(), DetectorStatus::Unsupported);
    let bad = format!("{text}broken: [\n");
    let file = ScannableFile::from_bytes("pnpm-lock.yaml".into(), bad.into_bytes());
    assert!(DependencyDetector.detect_detailed(&file, &r.rules).is_err());
}

#[test]
fn yaml_document_start_properties_are_rejected_but_scalar_markers_remain_text() {
    let r = Rules::new();
    for property in ["&anchor", "!<tag:yaml.org,2002:map>"] {
        let file = ScannableFile::from_bytes(
            "pnpm-lock.yaml".into(),
            format!("--- {property}\nlockfileVersion: '9.0'\npackages: {{}}\n").into_bytes(),
        );
        let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
        assert_eq!(o.status(), DetectorStatus::Unsupported, "{property}");
    }
    let file = ScannableFile::from_bytes(
        "pnpm-lock.yaml".into(),
        b"lockfileVersion: '9.0'\npackages: {}\nmetadata: |\n  --- &anchor *alias !tag\n".to_vec(),
    );
    let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
}

#[test]
fn pnpm_document_versions_and_shared_value_budget_are_enforced() {
    let r = Rules::new();
    for (second, expected) in [
        ("lockfileVersion: 9", DetectorStatus::Complete),
        ("lockfileVersion: '6.0'", DetectorStatus::Unsupported),
        ("other: value", DetectorStatus::Unsupported),
    ] {
        let file = ScannableFile::from_bytes(
            "pnpm-lock.yaml".into(),
            format!("---\nlockfileVersion: '9.0'\npackages: {{}}\n---\n{second}\npackages: {{}}\n")
                .into_bytes(),
        );
        let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
        assert_eq!(o.status(), expected);
    }
    let values = vec!["0"; 300_000].join(",");
    let doc = format!("---\nlockfileVersion: '9.0'\npackages: {{}}\nmetadata: [{values}]\n");
    let file =
        ScannableFile::from_bytes("pnpm-lock.yaml".into(), format!("{doc}{doc}").into_bytes());
    let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
    assert_eq!(o.status(), DetectorStatus::Truncated);
}
#[test]
fn duplicate_packages_in_two_documents_keep_distinct_native_record_ordinals() {
    let r = Rules::new();
    let input = r.dir.join("input");
    std::fs::create_dir_all(&input).unwrap();
    let doc="---\nlockfileVersion: '9.0'\npackages:\n  same@1.0.0:\n    resolution: {integrity: sha512-test}\n";
    std::fs::write(input.join("pnpm-lock.yaml"), format!("{doc}{doc}")).unwrap();
    let d = pqc_scan_core::scan_repository_native(
        &pqc_scan_core::ScanConfig {
            root: input,
            threads: 1,
            ..Default::default()
        },
        &r.rules,
        &pqc_scan_detectors::default_detectors(),
        &pqc_scan_core::NativeOptions::new(Some("test/pnpm-documents")).unwrap(),
    )
    .unwrap();
    let native = serde_json::to_value(d.native().unwrap()).unwrap();
    let observations = native["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 2);
    let ordinals: std::collections::BTreeSet<_> = observations
        .iter()
        .map(|o| o["target"]["object_index"].as_u64().unwrap())
        .collect();
    assert_eq!(ordinals, [1, 2].into_iter().collect());
    assert_ne!(observations[0]["id"], observations[1]["id"]);
}
#[test]
fn pnpm_record_limit_is_shared_by_both_documents_and_retains_partial_inventory() {
    let r = Rules::new();
    let doc = |start: usize, count: usize| {
        let entries = (start..start + count)
            .map(|i| format!("p{i}@1.0.0: {{resolution: {{integrity: sha512-test}}}} "))
            .collect::<Vec<_>>()
            .join(",");
        format!("---\nlockfileVersion: '9.0'\npackages: {{{entries}}}\n")
    };
    for (extra, expected) in [
        (0, DetectorStatus::Complete),
        (1, DetectorStatus::Truncated),
    ] {
        let text = doc(0, 5000) + &doc(5000, 5000 + extra);
        let file = ScannableFile::from_bytes("pnpm-lock.yaml".into(), text.into_bytes());
        let o = DependencyDetector.detect_detailed(&file, &r.rules).unwrap();
        assert_eq!(o.status(), expected);
        assert_eq!(o.detections().len(), 10_000);
    }
}
