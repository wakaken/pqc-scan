#[path = "../../pqc_scan_core/tests/support/migration/context.rs"]
mod ctx;
#[allow(dead_code)]
#[path = "../../pqc_scan_core/tests/support/migration/mod.rs"]
mod fixture;
use pqc_scan_core::migration::{evaluate, InputFormat, MigrationAssessment};
use pqc_scan_report::{migration_html_bytes, migration_markdown_bytes, ReportLanguage};
use serde_json::{json, Value};
fn report(mut v: Value, claims: Option<Value>) -> MigrationAssessment {
    if v["assets"].is_array() {
        v["assets"][0]["attributes"]["parameters"] = json!({"key_bits":"2048"});
    }
    let e = fixture::ingest(
        &v,
        if v["protocol"] == "tls" {
            InputFormat::TlsV1
        } else {
            InputFormat::InventoryV1
        },
    )
    .unwrap();
    let c = claims.map(|claims| ctx::parse(&ctx::context(&e, claims)));
    evaluate(&e, c.as_ref(), ctx::AS_OF).unwrap()
}
fn text(r: &MigrationAssessment, l: ReportLanguage) -> String {
    String::from_utf8(migration_markdown_bytes(r, l).unwrap()).unwrap()
}
#[test]
fn paired_reports_preserve_decisions_with_human_tables_and_no_json_dump() {
    let mut blocked = ctx::critical_signing();
    blocked["support"] = ctx::support();
    blocked["support"]["peer"] = ctx::claim(json!(false));
    let mut ready = ctx::critical_signing();
    ready["support"] = ctx::support();
    let mut stale = ctx::ordinary_signing();
    stale["deadline"] = ctx::claim(
        json!({"kind":"at","at":"2026-11-16T00:00:00Z","reference_id":"org-deadline","scope_reason":"organization_policy"}),
    );
    stale["deadline"]["provenance"]["source_kind"] = "external_review_claim".into();
    stale["deadline"]["provenance"]["asserted_at"] = "2026-11-16T00:00:00Z".into();
    stale["deadline"]["provenance"]["valid_until"] = "2026-11-17T00:00:00Z".into();
    for (id, claims, en, ja, action_en, action_ja) in [
        (
            "RSA",
            None,
            "Urgency undetermined",
            "緊急度を判断できない",
            "Investigate",
            "調査",
        ),
        (
            "RSA",
            Some(blocked),
            "Urgent",
            "早急に対応",
            "Resolve blocking prerequisites",
            "阻害条件を解消する",
        ),
        (
            "RSA",
            Some(ready),
            "Urgent",
            "早急に対応",
            "Migrate and validate",
            "移行を実施し検証する",
        ),
        (
            "ML-KEM-768",
            Some(ctx::critical_signing()),
            "Urgency undetermined",
            "緊急度を判断できない",
            "Investigate",
            "調査",
        ),
        (
            "RSA",
            Some(stale),
            "Urgent",
            "早急に対応",
            "Investigate",
            "調査",
        ),
        (
            "RSA",
            Some(json!({"applicability":ctx::claim(json!("not_applicable"))})),
            "Outside declared scope",
            "申告された範囲では対象外",
            "No action within the declared scope",
            "申告された範囲では対応なし",
        ),
    ] {
        let r = report(fixture::inventory(id), claims);
        let a = text(&r, ReportLanguage::En);
        let b = text(&r, ReportLanguage::Ja);
        for (md, urgency, action) in [(&a, en, action_en), (&b, ja, action_ja)] {
            assert!(md.contains(urgency), "{md}");
            assert!(md.contains(action));
            assert!(!md.contains("\"prerequisites\":"));
            assert!(!md.contains("\"source_ref\":"));
            assert!(md.contains("migration.assessment.v1.json"));
            let html = String::from_utf8(
                migration_html_bytes(
                    &r,
                    if md == &a {
                        ReportLanguage::En
                    } else {
                        ReportLanguage::Ja
                    },
                )
                .unwrap(),
            )
            .unwrap();
            let active = html.split("<template").next().unwrap();
            assert!(active.contains(urgency));
            assert!(active.contains(action));
            assert!(active.contains("decision-card"));
            assert!(!active.contains("\"claimed_support\":"));
            assert!(active.contains("migration.assessment.v1.json"));
            for source in r.sources() {
                assert!(active.contains(&source.edition));
            }
        }
        assert!(a.find("## Response overview").unwrap() < a.find("## Declared context").unwrap());
        assert!(b.contains("次の確認"));
        assert!(b.contains("外部レビュー由来のものを含め、申告は未検証"));
        assert!(a.contains("not verified deployment"));
    }
}
#[test]
fn internal_links_are_complete_identical_across_languages_and_unknown_is_not_low() {
    let mut claims = ctx::critical_signing();
    claims["support"] = ctx::support();
    let r = report(fixture::inventory("RSA"), Some(claims));
    let mut anchors = None;
    for l in [ReportLanguage::En, ReportLanguage::Ja] {
        let md = text(&r, l);
        let ids = md
            .lines()
            .filter_map(|s| {
                s.strip_prefix("<a id=\"")
                    .and_then(|s| s.strip_suffix("\"></a>"))
            })
            .collect::<Vec<_>>();
        for line in md.lines() {
            for target in line.split("](#").skip(1) {
                let id = target.split(')').next().unwrap();
                assert!(ids.contains(&id), "broken {id}");
            }
        }
        if let Some(prior) = &anchors {
            assert_eq!(prior, &ids);
        } else {
            anchors = Some(ids.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        }
    }
    let unknown = text(&report(fixture::inventory("RSA"), None), ReportLanguage::Ja);
    assert!(!unknown.contains("低優先"));
    assert!(unknown.contains("準備状況は不明"));
}
#[test]
fn markdown_locations_neutralize_html_links_images_controls_and_raw_metadata() {
    let mut v = fixture::inventory("RSA");
    v["observations"][0]["target"]["path"] =
        "<script>alert(1)</script>![x](evil.png)`[click](evil.html)".into();
    v["arbitrary_metadata"] = json!({"secret":"SECRET_CANARY_070"});
    let r = report(v, None);
    for l in [ReportLanguage::En, ReportLanguage::Ja] {
        let md = text(&r, l);
        let first = md.split("## ").nth(1).unwrap().split("## ").next().unwrap();
        assert!(!first.contains("<script>"));
        assert!(!first.contains("![x]"));
        assert!(!first.contains("[click]"));
        assert!(!first.contains("\n# injected"));
        assert!(first.contains("&lt;script&gt;"));
        assert!(first.contains("&#33;&#91;x&#93;"));
        assert!(!md.contains("SECRET_CANARY_070"));
        assert!(!md.contains("\"path\":"));
        let html = String::from_utf8(migration_html_bytes(&r, l).unwrap()).unwrap();
        assert!(!html.contains("<script>alert(1)"));
        assert!(!html.contains("<img"));
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("SECRET_CANARY_070"));
    }
}

#[test]
fn tls_key_establishment_and_certificate_authentication_and_failed_collection_stay_separate() {
    let applicable = json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment"))});
    let r = report(fixture::tls(), Some(applicable));
    for (l, monitor, unknown) in [
        (
            ReportLanguage::En,
            "Monitor this role",
            "Urgency undetermined",
        ),
        (
            ReportLanguage::Ja,
            "対象の役割を継続確認",
            "緊急度を判断できない",
        ),
    ] {
        let md = text(&r, l);
        assert!(md.contains(monitor));
        assert!(md.contains(unknown));
        for role in if l == ReportLanguage::Ja {
            ["通信の鍵確立", "証明書の署名", "証明書の公開鍵"]
        } else {
            [
                "Connection key establishment",
                "Certificate signature",
                "Certificate public key",
            ]
        } {
            assert!(md.contains(role));
        }
        assert!(md.contains(if l == ReportLanguage::Ja {
            "収集記録がセッションの検証成功を報告"
        } else {
            "Collector reports a verified session"
        }));
        assert!(md.contains(if l == ReportLanguage::Ja {
            "量子計算に対する懸念あり"
        } else {
            "Quantum exposure identified"
        }));
    }
    let r = report(fixture::failed_tls(), None);
    for l in [ReportLanguage::En, ReportLanguage::Ja] {
        let md = text(&r, l);
        assert!(md.contains(if l == ReportLanguage::Ja {
            "検証は完了していません"
        } else {
            "Verification was not completed"
        }));
        assert!(md.contains(if l == ReportLanguage::Ja {
            "緊急度を判断できない"
        } else {
            "Urgency undetermined"
        }));
        assert!(!md.contains("collector_reported_verified_session"));
    }
}

#[test]
fn html_limit_rejects_without_replacing_existing_json_markdown_or_html() {
    let mut v = fixture::inventory("RSA");
    let observation = v["observations"][0].clone();
    v["observations"] = Value::Array(
        (0..16000)
            .map(|i| {
                let mut o = observation.clone();
                o["id"] = pqc_scan_core::identity::native_id(
                    "observation",
                    &[&format!("html-limit-{i}")],
                )
                .into();
                o["occurrence_key"] =
                    pqc_scan_core::identity::native_id("occurrence", &[&format!("html-limit-{i}")])
                        .into();
                o
            })
            .collect(),
    );
    let evidence = fixture::ingest(&v, InputFormat::InventoryV1).unwrap();
    let r = evaluate(&evidence, None, ctx::AS_OF).unwrap();
    // Both existing representations fit; the new bilingual HTML must enforce
    // its own byte boundary before any member of the report set is replaced.
    assert!(pqc_scan_report::migration_json_bytes(&r).is_ok());
    assert!(migration_markdown_bytes(&r, ReportLanguage::En).is_ok());
    let path = std::env::temp_dir().join(format!("pqc-html-limit-{}", std::process::id()));
    std::fs::create_dir_all(&path).unwrap();
    for name in [
        "migration.assessment.v1.json",
        "migration.assessment.md",
        "migration.assessment.html",
    ] {
        std::fs::write(path.join(name), b"KEEP").unwrap();
    }
    let result = pqc_scan_report::write_migration_assessment_reports(
        &r,
        &evidence,
        None,
        &path,
        ReportLanguage::En,
    );
    assert!(result.is_err(), "oversized HTML accepted");
    assert!(result.unwrap_err().to_string().contains("resource_limit"));
    for name in [
        "migration.assessment.v1.json",
        "migration.assessment.md",
        "migration.assessment.html",
    ] {
        assert_eq!(std::fs::read(path.join(name)).unwrap(), b"KEEP");
    }
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn symmetric_hash_and_failed_runtime_states_use_plain_bilingual_terms() {
    for id in ["AES-256", "SHA-256"] {
        let mut input = fixture::inventory(id);
        input["assets"][0]["attributes"]["parameters"] = json!({"key_bits":"256"});
        let e = fixture::ingest(&input, InputFormat::InventoryV1).unwrap();
        let r = evaluate(&e, None, ctx::AS_OF).unwrap();
        for language in [ReportLanguage::En, ReportLanguage::Ja] {
            let md = text(&r, language);
            let html = String::from_utf8(migration_html_bytes(&r, language).unwrap()).unwrap();
            let active = html.split("<template").next().unwrap();
            for view in [md.as_str(), active] {
                assert!(view.contains(if language == ReportLanguage::Ja {
                    "汎用の量子探索による影響"
                } else {
                    "Generic quantum search effects"
                }));
                assert!(!view.contains("generic_search"));
                assert!(!view.contains("Quantum exposure identified"));
            }
        }
    }
    let r = report(fixture::failed_tls(), None);
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let md = text(&r, language);
        let html = String::from_utf8(migration_html_bytes(&r, language).unwrap()).unwrap();
        let active = html.split("<template").next().unwrap();
        for view in [md.as_str(), active] {
            assert!(view.contains(if language == ReportLanguage::Ja {
                "検証未完了、または収集情報が不完全"
            } else {
                "Verification not completed or collection incomplete"
            }));
            assert!(!view.contains("unverified_or_incomplete"));
            assert!(!view.contains("通信で選ばれた鍵確立"));
            assert!(!view.contains("Negotiated key establishment"));
        }
    }
}
