use pqc_scan_core::{
    scan_repository_native, Detection, Detector, DetectorOutcome, DetectorStatus, NativeOptions,
    ScanConfig, ScannableFile,
};
use pqc_scan_report::{
    write_document_reports, write_reports_with_language, ReportFormat, ReportLanguage,
};
use pqc_scan_rules::RuleSet;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "pqc036-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir_all(path.join("input")).unwrap();
        fs::create_dir_all(path.join("rules")).unwrap();
        fs::write(path.join("input/<untrusted>&.txt"), "fixture").unwrap();
        Self(path)
    }
    fn document(&self, status: DetectorStatus) -> pqc_scan_core::ScanDocument {
        scan_repository_native(
            &ScanConfig {
                root: self.0.join("input"),
                threads: 1,
                ..Default::default()
            },
            &RuleSet::load_from_dir(&self.0.join("rules")).unwrap(),
            &[std::sync::Arc::new(Example(status))],
            &NativeOptions::new(Some("test/project")).unwrap(),
        )
        .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct Example(DetectorStatus);
impl Detector for Example {
    fn name(&self) -> &'static str {
        "fixture"
    }
    fn detect(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        Ok(vec![])
    }
    fn detect_detailed(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<DetectorOutcome> {
        Ok(DetectorOutcome::new(vec![], self.0))
    }
}

#[test]
fn document_reports_localize_status_and_escape_diagnostics_without_machine_drift() {
    let fixture = Fixture::new();
    for (status, code, ja) in [
        (
            DetectorStatus::Complete,
            "complete_within_scope",
            "宣言した範囲内で検査完了",
        ),
        (
            DetectorStatus::Partial,
            "partial",
            "一部を検査できませんでした",
        ),
        (
            DetectorStatus::NotReported,
            "unknown",
            "検査の完了を確認できません",
        ),
    ] {
        let doc = fixture.document(status);
        let root = fixture.0.join(code);
        for lang in [ReportLanguage::En, ReportLanguage::Ja] {
            let output = root.join(if lang == ReportLanguage::En {
                "en"
            } else {
                "ja"
            });
            write_document_reports(&doc, None, true, &output, ReportFormat::All, lang).unwrap();
            let html = fs::read_to_string(output.join("report.html")).unwrap();
            let md = fs::read_to_string(output.join("report.md")).unwrap();
            assert!(html.contains(code));
            assert!(md.contains(code));
            if lang == ReportLanguage::Ja {
                assert!(html.contains(match status {
                    DetectorStatus::Complete => "対象範囲の検査は完了しました",
                    DetectorStatus::Partial => "一部のファイル・検査を確認できていません",
                    _ => "検査を終えたか確認できません",
                }));
                assert!(md.contains(ja));
            }
            if status == DetectorStatus::Partial {
                assert!(
                    html.contains("&lt;untrusted&gt;&amp;.txt"),
                    "diagnostics: {:?}",
                    doc.diagnostics()
                );
                assert!(!html.contains("<untrusted>"));
                assert!(md.contains("&lt;untrusted&gt;&amp;.txt"));
            }
        }
        write_reports_with_language(
            doc.legacy(),
            &root.join("old"),
            ReportFormat::All,
            ReportLanguage::En,
        )
        .unwrap();
        for name in [
            "report.json",
            "report.sarif",
            "cbom.json",
            "dependency-sbom.json",
        ] {
            assert_eq!(
                fs::read(root.join("en").join(name)).unwrap(),
                fs::read(root.join("ja").join(name)).unwrap()
            );
            assert_eq!(
                fs::read(root.join("en").join(name)).unwrap(),
                fs::read(root.join("old").join(name)).unwrap()
            );
        }
    }
}

#[test]
fn explicit_policy_stays_in_markdown_and_never_changes_ordinary_html() {
    let fixture = Fixture::new();
    let doc = fixture.document(DetectorStatus::Complete);
    let policy = pqc_scan_core::policy::evaluate(
        doc.native().unwrap(),
        &Default::default(),
        pqc_scan_core::policy::PROFILE_ID,
    )
    .unwrap();
    for lang in [ReportLanguage::Ja, ReportLanguage::En] {
        let locale = if lang == ReportLanguage::Ja {
            "ja"
        } else {
            "en"
        };
        let plain = fixture.0.join(format!("plain-{locale}"));
        let assessed = fixture.0.join(format!("assessed-{locale}"));
        write_document_reports(&doc, None, false, &plain, ReportFormat::All, lang).unwrap();
        write_document_reports(
            &doc,
            Some(&policy),
            false,
            &assessed,
            ReportFormat::All,
            lang,
        )
        .unwrap();
        let html = fs::read_to_string(assessed.join("report.html")).unwrap();
        assert_eq!(fs::read_to_string(plain.join("report.html")).unwrap(), html);
        for hidden in [
            "migration-review-v1",
            "ローカルの参考評価",
            "Local advisory assessment",
            "assessment-summary",
        ] {
            assert!(!html.contains(hidden));
        }
        let md = fs::read_to_string(assessed.join("report.md")).unwrap();
        assert!(md.contains("migration-review-v1"));
        assert!(md.contains("indeterminate"));
        for reason in &policy.evaluation().reasons {
            assert!(md.contains(reason));
        }
        for name in [
            "report.json",
            "report.sarif",
            "cbom.json",
            "dependency-sbom.json",
        ] {
            assert_eq!(
                fs::read(plain.join(name)).unwrap(),
                fs::read(assessed.join(name)).unwrap()
            );
        }
    }
    let other = fixture.document(DetectorStatus::Complete);
    assert!(write_document_reports(
        &other,
        Some(&policy),
        false,
        &fixture.0.join("mismatch"),
        ReportFormat::All,
        ReportLanguage::En
    )
    .is_err());
    assert!(!fixture.0.join("mismatch").exists());
}

struct Broken;
impl Detector for Broken {
    fn name(&self) -> &'static str {
        "broken"
    }
    fn detect(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        anyhow::bail!("ARBITRARY_ERROR_CANARY_036")
    }
}
#[test]
fn failed_and_empty_inspection_preserve_scope_and_safe_diagnostics() {
    let fixture = Fixture::new();
    let config = ScanConfig {
        root: fixture.0.join("input"),
        threads: 1,
        ..Default::default()
    };
    let rules = RuleSet::load_from_dir(&fixture.0.join("rules")).unwrap();
    let doc =
        pqc_scan_core::scan_repository_detailed(&config, &rules, &[std::sync::Arc::new(Broken)])
            .unwrap();
    let out = fixture.0.join("failed");
    write_document_reports(
        &doc,
        None,
        true,
        &out,
        ReportFormat::All,
        ReportLanguage::Ja,
    )
    .unwrap();
    let html = fs::read_to_string(out.join("report.html")).unwrap();
    assert!(html.contains("検査に失敗しました"));
    assert!(html.contains("data-inspection-status=\"failed\""));
    assert!(html.contains("検出器の処理に失敗しました"));
    assert!(!html.contains("ARBITRARY_ERROR_CANARY_036"));
    let out = fixture.0.join("without-details");
    write_document_reports(
        &doc,
        None,
        false,
        &out,
        ReportFormat::All,
        ReportLanguage::En,
    )
    .unwrap();
    assert!(!fs::read_to_string(out.join("report.html"))
        .unwrap()
        .contains("detector_failed"));
    fs::remove_file(fixture.0.join("input/<untrusted>&.txt")).unwrap();
    let empty = fixture.document(DetectorStatus::Complete);
    let out = fixture.0.join("empty");
    write_document_reports(
        &empty,
        None,
        false,
        &out,
        ReportFormat::All,
        ReportLanguage::Ja,
    )
    .unwrap();
    let md = fs::read_to_string(out.join("report.md")).unwrap();
    assert!(md.contains("complete_within_scope"));
    assert!(md.contains("|選択したファイル|0|"));
    assert!(md.contains("列挙されなかったファイルは不明"));
}

struct Referenced;
impl Detector for Referenced {
    fn name(&self) -> &'static str {
        "fixture"
    }
    fn detect(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        Ok(vec![])
    }
    fn detect_observed(
        &self,
        file: &ScannableFile,
        _: &RuleSet,
    ) -> anyhow::Result<DetectorOutcome> {
        let d = Detection {
            rule_id: "FIXTURE".into(),
            location: pqc_scan_core::Location {
                file: file.path.to_string_lossy().into(),
                line: 1,
                column: 1,
            },
            evidence: pqc_scan_core::Evidence {
                r#type: "regex_match".into(),
                r#match: "RSA".into(),
                snippet_preview: "RSA".into(),
                metadata: Default::default(),
            },
        };
        let observation = pqc_scan_core::ObservationDraft::source(
            &d,
            pqc_scan_core::ObservationMethod::LexicalMatch,
            "RSA",
            None,
        );
        Ok(DetectorOutcome::new(vec![], DetectorStatus::Complete)
            .with_observations(vec![observation]))
    }
}
#[test]
fn assurance_usage_and_per_asset_policy_keep_exact_observation_bindings() {
    let fixture = Fixture::new();
    let doc = scan_repository_native(
        &ScanConfig {
            root: fixture.0.join("input"),
            threads: 1,
            ..Default::default()
        },
        &RuleSet::load_from_dir(&fixture.0.join("rules")).unwrap(),
        &[std::sync::Arc::new(Referenced)],
        &NativeOptions::new(Some("test/project")).unwrap(),
    )
    .unwrap();
    let policy = pqc_scan_core::policy::evaluate(
        doc.native().unwrap(),
        &Default::default(),
        pqc_scan_core::policy::PROFILE_ID,
    )
    .unwrap();
    let mut altered = serde_json::to_value(doc.native().unwrap()).unwrap();
    altered["observations"][0]["assurance"] = "parsed".into();
    let altered: pqc_scan_core::NativeScan = serde_json::from_value(altered).unwrap();
    let mismatched = pqc_scan_core::policy::evaluate(
        &altered,
        &Default::default(),
        pqc_scan_core::policy::PROFILE_ID,
    )
    .unwrap();
    assert!(write_document_reports(
        &doc,
        Some(&mismatched),
        false,
        &fixture.0.join("same-id-mismatch"),
        ReportFormat::All,
        ReportLanguage::En
    )
    .is_err());
    for lang in [ReportLanguage::En, ReportLanguage::Ja] {
        let output = fixture.0.join(if lang == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_document_reports(&doc, Some(&policy), true, &output, ReportFormat::All, lang)
            .unwrap();
        for name in ["report.html", "report.md"] {
            let body = fs::read_to_string(output.join(name)).unwrap();
            if name == "report.html" {
                for observation in doc.native().unwrap().observations() {
                    assert!(!body.contains(observation.id()));
                }
                for asset in policy.asset_policies() {
                    assert!(!body.contains(&asset.asset_id));
                }
                assert!(!body.contains("assessment-summary"));
                continue;
            }
            for o in doc.native().unwrap().observations() {
                assert!(body.contains(o.id()));
            }
            assert!(body.contains("lexical"));
            assert!(body.contains("referenced"));
            let decisions: std::collections::BTreeSet<_> = policy
                .asset_policies()
                .iter()
                .map(|p| format!("{:?}", p.decision))
                .collect();
            assert!(
                decisions.len() >= 2,
                "fixture must distinguish asset policies"
            );
            for p in policy.asset_policies() {
                let code = serde_json::to_value(p.decision).unwrap();
                let expected = format!("({}); {}", code.as_str().unwrap(), p.reasons.join(", "));
                let row = body
                    .lines()
                    .find(|line| {
                        if name == "report.html" {
                            line.contains(&format!("<th scope=\"row\">{}</th>", p.asset_id))
                        } else {
                            line.starts_with(&format!("|{}|", p.asset_id))
                        }
                    })
                    .expect("asset policy row");
                assert!(row.contains(&expected), "wrong policy on asset row: {row}");
            }
        }
    }
}

struct MixedEvidence;
impl Detector for MixedEvidence {
    fn name(&self) -> &'static str {
        "mixed_evidence"
    }
    fn detect(&self, _: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
        Ok(vec![])
    }
    fn detect_observed(
        &self,
        file: &ScannableFile,
        _: &RuleSet,
    ) -> anyhow::Result<DetectorOutcome> {
        use pqc_scan_core::ObservationMethod;
        let observations = [
            ObservationMethod::LexicalMatch,
            ObservationMethod::AstPattern,
            ObservationMethod::Configuration,
            ObservationMethod::LegacyAdapter,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, method)| {
            let detection = Detection {
                rule_id: "MIXED_EVIDENCE".into(),
                location: pqc_scan_core::Location {
                    file: file.path.to_string_lossy().into(),
                    line: index + 1,
                    column: 1,
                },
                evidence: pqc_scan_core::Evidence {
                    r#type: "fixture".into(),
                    r#match: "RSA".into(),
                    snippet_preview: "RSA".into(),
                    metadata: Default::default(),
                },
            };
            pqc_scan_core::ObservationDraft::source(&detection, method, "RSA", None)
        })
        .collect();
        Ok(DetectorOutcome::complete(vec![]).with_observations(observations))
    }
}

#[test]
fn exact_observation_states_stay_in_markdown_but_do_not_overload_html() {
    let fixture = Fixture::new();
    let doc = scan_repository_native(
        &ScanConfig {
            root: fixture.0.join("input"),
            threads: 1,
            ..Default::default()
        },
        &RuleSet::load_from_dir(&fixture.0.join("rules")).unwrap(),
        &[std::sync::Arc::new(MixedEvidence)],
        &NativeOptions::new(Some("test/project")).unwrap(),
    )
    .unwrap();
    assert_eq!(doc.native().unwrap().observations().len(), 4);
    for lang in [ReportLanguage::En, ReportLanguage::Ja] {
        let output = fixture.0.join(if lang == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_document_reports(&doc, None, false, &output, ReportFormat::All, lang).unwrap();
        let html = fs::read_to_string(output.join("report.html")).unwrap();
        let md = fs::read_to_string(output.join("report.md")).unwrap();
        let mut seen = std::collections::BTreeSet::new();
        for observation in doc.native().unwrap().observations() {
            let states = (observation.assurance(), observation.usage());
            assert!(seen.insert(states));
            let (en, ja) = match states {
                ("lexical", "referenced") => (
                    "Assurance: Lexical match (lexical); Usage: Referenced (referenced)",
                    "確認根拠: 文字列の一致 (lexical); 利用状態: 参照あり (referenced)",
                ),
                ("syntactic", "referenced") => (
                    "Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)",
                    "確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)",
                ),
                ("lexical", "configured") => (
                    "Assurance: Lexical match (lexical); Usage: Configured (configured)",
                    "確認根拠: 文字列の一致 (lexical); 利用状態: 設定あり (configured)",
                ),
                ("unknown", "unknown") => (
                    "Assurance: Unknown (unknown); Usage: Unknown (unknown)",
                    "確認根拠: 不明 (unknown); 利用状態: 不明 (unknown)",
                ),
                other => panic!("unexpected evidence states: {other:?}"),
            };
            let expected = if lang == ReportLanguage::En { en } else { ja };
            assert!(!html.contains(observation.id()));
            assert!(!html.contains("Observation assurance and usage"));
            assert!(!html.contains("観測の確認根拠と利用状態"));
            let markdown_row = md
                .lines()
                .find(|line| line.starts_with(&format!("|{}|", observation.id())))
                .expect("observation Markdown row");
            assert_eq!(markdown_row, format!("|{}|{expected}|", observation.id()));
        }
        assert_eq!(seen.len(), 4);
    }
}

#[test]
fn document_reports_retain_unknown_custom_rule_copy_and_escape_it_in_japanese() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("rules/custom.yml"), r#"[{"id":"CUSTOM036","kind":"regex","category":"custom","severity":"medium","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Check <vendor> & constraints","pattern":"fixture","scope":"code","description":"Custom <vendor> & evidence"}]"#).unwrap();
    struct Custom;
    impl Detector for Custom {
        fn name(&self) -> &'static str {
            "custom_fixture"
        }
        fn detect(&self, file: &ScannableFile, _: &RuleSet) -> anyhow::Result<Vec<Detection>> {
            Ok(vec![Detection {
                rule_id: "CUSTOM036".into(),
                location: pqc_scan_core::Location {
                    file: file.path.to_string_lossy().into(),
                    line: 1,
                    column: 1,
                },
                evidence: pqc_scan_core::Evidence {
                    r#type: "regex_match".into(),
                    r#match: "fixture".into(),
                    snippet_preview: "fixture".into(),
                    metadata: Default::default(),
                },
            }])
        }
    }
    let doc = scan_repository_native(
        &ScanConfig {
            root: fixture.0.join("input"),
            threads: 1,
            ..Default::default()
        },
        &RuleSet::load_from_dir(&fixture.0.join("rules")).unwrap(),
        &[std::sync::Arc::new(Custom)],
        &NativeOptions::new(Some("test/project")).unwrap(),
    )
    .unwrap();
    assert_eq!(doc.legacy().findings.len(), 1);
    let output = fixture.0.join("ja");
    write_document_reports(
        &doc,
        None,
        true,
        &output,
        ReportFormat::All,
        ReportLanguage::Ja,
    )
    .unwrap();
    for name in ["report.html", "report.md"] {
        let body = fs::read_to_string(output.join(name)).unwrap();
        assert!(body.contains("日本語の説明が未登録"));
        assert!(body.contains("Custom &lt;vendor&gt;"));
        assert!(body.contains("Check &lt;vendor&gt;"));
        assert!(!body.contains("<vendor>"));
        assert!(body.contains("CUSTOM036"));
        assert!(body.contains("unknown"));
    }
    let html = fs::read_to_string(output.join("report.html")).unwrap();
    assert!(html.contains("Custom &lt;vendor&gt; &amp; evidence"));
}

#[test]
fn html_prioritizes_coverage_and_omits_internal_observation_and_asset_tables() {
    let fixture = Fixture::new();
    let doc = scan_repository_native(
        &ScanConfig {
            root: fixture.0.join("input"),
            max_file_size_bytes: 37,
            threads: 1,
        },
        &RuleSet::load_from_dir(&fixture.0.join("rules")).unwrap(),
        &[std::sync::Arc::new(MixedEvidence)],
        &NativeOptions::new(Some("test/project")).unwrap(),
    )
    .unwrap();
    let policy = pqc_scan_core::policy::evaluate(
        doc.native().unwrap(),
        &Default::default(),
        pqc_scan_core::policy::PROFILE_ID,
    )
    .unwrap();
    for lang in [ReportLanguage::Ja, ReportLanguage::En] {
        let out = fixture.0.join(if lang == ReportLanguage::Ja {
            "readable-ja"
        } else {
            "readable-en"
        });
        write_document_reports(&doc, Some(&policy), true, &out, ReportFormat::All, lang).unwrap();
        let html = fs::read_to_string(out.join("report.html")).unwrap();
        let md = fs::read_to_string(out.join("report.md")).unwrap();
        assert!(
            html.find("<section class=\"inspection-overview").unwrap()
                < html.find("<section data-report-controls").unwrap()
        );
        assert!(html.contains("37 bytes"));
        assert!(html.contains("node_modules"));
        assert!(html.contains(".gitignore"));
        assert!(!html.contains("data-observation-row"));
        for observation in doc.native().unwrap().observations() {
            assert!(
                !html.contains(observation.id()),
                "internal observation ID in HTML"
            );
            assert!(
                md.contains(observation.id()),
                "exact record must remain in Markdown"
            );
        }
        for asset in policy.asset_policies() {
            assert!(!html.contains(&asset.asset_id), "internal asset ID in HTML");
            assert!(md.contains(&asset.asset_id));
        }
        assert!(html.contains("inspection-reasons"));
    }
}

#[test]
fn html_groups_diagnostic_reasons_with_bounded_examples_and_preserves_full_markdown() {
    let fixture = Fixture::new();
    fs::remove_file(fixture.0.join("input/<untrusted>&.txt")).unwrap();
    for i in 0..12 {
        fs::write(fixture.0.join(format!("input/file-{i:02}.txt")), "fixture").unwrap();
    }
    let doc = fixture.document(DetectorStatus::Partial);
    let out = fixture.0.join("grouped");
    write_document_reports(
        &doc,
        None,
        true,
        &out,
        ReportFormat::All,
        ReportLanguage::Ja,
    )
    .unwrap();
    let html = fs::read_to_string(out.join("report.html")).unwrap();
    let md = fs::read_to_string(out.join("report.md")).unwrap();
    assert_eq!(html.matches("class=\"inspection-reason\"").count(), 1);
    assert!(html.contains("12件"));
    assert!(html.contains("ほか9件"));
    assert!(!html.contains("file-11.txt"));
    assert!(md.contains("file-11.txt"));
    assert!(html.contains("data-inspection-status=\"partial\""));
}
