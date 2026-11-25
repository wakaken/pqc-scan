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
            assert!(!active.contains("migration.assessment.md"));
            assert!(!active.contains("<summary>Sources</summary>"));
            assert!(!active.contains("<summary>出典</summary>"));
            let footer = active.split_once("<footer").unwrap().1;
            assert_eq!(footer.matches("<a ").count(), 1);
            assert!(footer.contains(if md == &a {
                "Save assessment data"
            } else {
                "評価データを保存"
            }));
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
        assert!(html.contains("\\u003cscript>"));
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
fn report_byte_limit_rejects_without_replacing_existing_json_markdown_or_html() {
    let mut v = fixture::inventory("RSA");
    let observation = v["observations"][0].clone();
    v["observations"] = Value::Array(
        (0..28000)
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
    // Structural sharing now makes the old16k HTML-only overflow fixture fit.
    // A genuinely oversized report still fails before replacing any artifact.
    assert!(pqc_scan_report::migration_json_bytes(&r).is_err());
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
    assert!(result.is_err(), "oversized report accepted");
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
            let data = embedded(&html, "migration-shared-data");
            let label = data["words"]["generic_search"][if language == ReportLanguage::Ja {
                "ja"
            } else {
                "en"
            }]
            .as_str()
            .unwrap();
            assert!(label.contains(if language == ReportLanguage::Ja {
                "汎用の量子探索による影響"
            } else {
                "Generic quantum search effects"
            }));
            assert!(!active.contains("generic_search"));
            {
                let view = md.as_str();
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
        let data = embedded(&html, "migration-shared-data");
        let label = data["words"]["unverified_or_incomplete"][if language == ReportLanguage::Ja {
            "ja"
        } else {
            "en"
        }]
        .as_str()
        .unwrap();
        assert!(label.contains(if language == ReportLanguage::Ja {
            "検証未完了、または収集情報が不完全"
        } else {
            "Verification not completed or collection incomplete"
        }));
        assert!(!active.contains("unverified_or_incomplete"));
        {
            let view = md.as_str();
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

fn embedded(html: &str, id: &str) -> Value {
    let start = format!("<script type=\"application/json\" id=\"{id}\">");
    let body = html
        .split_once(&start)
        .unwrap()
        .1
        .split_once("</script>")
        .unwrap()
        .0;
    serde_json::from_str(body).unwrap()
}
#[test]
fn large_html_defers_every_target_and_claim_but_bounds_initial_elements() {
    let mut input = fixture::inventory("RSA");
    let observation = input["observations"][0].clone();
    input["observations"] = (0..45)
        .map(|i| {
            let mut o = observation.clone();
            o["id"] =
                pqc_scan_core::identity::native_id("observation", &[&format!("paged-{i}")]).into();
            o["occurrence_key"] =
                pqc_scan_core::identity::native_id("occurrence", &[&format!("paged-{i}")]).into();
            o["target"]["path"] = format!("src/target-{i}.java").into();
            o
        })
        .collect();
    let e = fixture::ingest(&input, InputFormat::InventoryV1).unwrap();
    let mut context = ctx::context(&e, ctx::critical_signing());
    context["bindings"] = e
        .targets()
        .iter()
        .map(|t| json!({"target_id":t.id,"system_id":"system-a","claims":ctx::critical_signing()}))
        .collect();
    let context = ctx::parse(&context);
    let r = evaluate(&e, Some(&context), ctx::AS_OF).unwrap();
    assert_eq!(r.decisions().len(), 45);
    assert!(r.claims().len() > 20);
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let html = String::from_utf8(migration_html_bytes(&r, language).unwrap()).unwrap();
        let initial = html.split("<template").next().unwrap();
        assert_eq!(initial.matches("<article class=\"decision-card").count(), 1);
        assert_eq!(initial.matches("<a href=\"#target-").count(), 0);
        assert_eq!(initial.matches("<tr id=\"claim-").count(), 0);
        // Member-specific quantum/assurance facts must not masquerade as group facts.
        assert!(!initial.contains("Quantum exposure"));
        assert!(!initial.contains("解析した属性"));
        let payload = embedded(&html, "migration-shared-data");
        assert_eq!(
            restore_archive(
                &payload["archive"],
                payload["archive"]["root"].as_u64().unwrap() as usize
            ),
            serde_json::to_value(&r).unwrap()
        );
        let groups = payload["groups"].as_array().unwrap();
        assert_eq!(groups.len(), 1);
        let mut members = groups
            .iter()
            .flat_map(|g| {
                g["members"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap())
            })
            .collect::<Vec<_>>();
        members.sort();
        assert_eq!(members, (0..45).collect::<Vec<_>>());
        assert!(!html.contains("migration-cards-"));
        assert!(!html.contains("migration-claims-"));
    }
}

fn restore_archive(archive: &Value, reference: usize) -> Value {
    let node = &archive["nodes"][reference];
    match node[0].as_str().unwrap() {
        "o" => Value::Object(
            node[1]
                .as_array()
                .unwrap()
                .chunks(2)
                .map(|pair| {
                    (
                        restore_archive(archive, pair[0].as_u64().unwrap() as usize)
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        restore_archive(archive, pair[1].as_u64().unwrap() as usize),
                    )
                })
                .collect(),
        ),
        "a" => Value::Array(
            node[1]
                .as_array()
                .unwrap()
                .iter()
                .map(|reference| restore_archive(archive, reference.as_u64().unwrap() as usize))
                .collect(),
        ),
        "n" => serde_json::from_str(node[1].as_str().unwrap()).unwrap(),
        "z" => Value::Null,
        _ => node[1].clone(),
    }
}
#[test]
fn repeated_observations_share_one_work_item_but_different_systems_do_not() {
    let mut input = fixture::inventory("RSA");
    let original = input["observations"][0].clone();
    input["observations"] = (0..2)
        .map(|i| {
            let mut o = original.clone();
            o["id"] =
                pqc_scan_core::identity::native_id("observation", &[&format!("distinct-{i}")])
                    .into();
            o["occurrence_key"] =
                pqc_scan_core::identity::native_id("occurrence", &[&format!("distinct-{i}")])
                    .into();
            o["target"]["path"] = format!("different-product/src/use-{i}.java").into();
            o
        })
        .collect();
    let evidence = fixture::ingest(&input, InputFormat::InventoryV1).unwrap();
    let mut context = ctx::context(&evidence, ctx::critical_signing());
    context["systems"] = json!([{"id":"system-a"},{"id":"system-b"}]);
    context["bindings"]=evidence.targets().iter().enumerate().map(|(i,t)|json!({"target_id":t.id,"system_id":if i==0 {"system-a"} else {"system-b"},"claims":ctx::critical_signing()})).collect();
    let context = ctx::parse(&context);
    let r = evaluate(&evidence, Some(&context), ctx::AS_OF).unwrap();
    let html = String::from_utf8(migration_html_bytes(&r, ReportLanguage::Ja).unwrap()).unwrap();
    let data = embedded(&html, "migration-shared-data");
    assert_eq!(data["groups"].as_array().unwrap().len(), 2);
    assert_eq!(
        restore_archive(
            &data["archive"],
            data["archive"]["root"].as_u64().unwrap() as usize
        ),
        serde_json::to_value(r).unwrap()
    );
}
#[test]
fn empty_and_blocked_scopes_keep_their_actual_work_item_counts() {
    let mut empty = fixture::inventory("RSA");
    empty["assets"] = json!([]);
    empty["observations"] = json!([]);
    let e = fixture::ingest(&empty, InputFormat::InventoryV1).unwrap();
    let r = evaluate(&e, None, ctx::AS_OF).unwrap();
    let html = String::from_utf8(migration_html_bytes(&r, ReportLanguage::En).unwrap()).unwrap();
    assert_eq!(
        embedded(&html, "migration-shared-data")["groups"],
        json!([])
    );
    let mut blocked = ctx::critical_signing();
    blocked["support"] = ctx::support();
    blocked["support"]["peer"] = ctx::claim(json!(false));
    let r = report(fixture::inventory("RSA"), Some(blocked));
    let html = String::from_utf8(migration_html_bytes(&r, ReportLanguage::Ja).unwrap()).unwrap();
    let data = embedded(&html, "migration-shared-data");
    assert_eq!(data["groups"].as_array().unwrap().len(), 1);
    let restored = restore_archive(
        &data["archive"],
        data["archive"]["root"].as_u64().unwrap() as usize,
    );
    assert_eq!(restored["decisions"][0]["readiness"], "blocked");
    assert!(
        !restored["decisions"][0]["action"]["blocking_prerequisites"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn writer_progress_tracks_real_phases_and_never_finishes_on_output_failure() {
    use pqc_scan_report::MigrationReportProgress as P;
    let e = fixture::ingest(&fixture::inventory("RSA"), InputFormat::InventoryV1).unwrap();
    let r = evaluate(&e, None, ctx::AS_OF).unwrap();
    let dir = std::env::temp_dir().join(format!("pqc-grouped-progress-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut phases = vec![];
    let paths = pqc_scan_report::write_migration_assessment_reports_with_progress(
        &r,
        &e,
        None,
        &dir,
        ReportLanguage::Ja,
        |p| phases.push(p),
    )
    .unwrap();
    assert_eq!(paths.len(), 3);
    assert!(paths.iter().all(|p| p.is_file()));
    assert!(matches!(phases[0], P::Validating));
    assert!(matches!(
        phases.last(),
        Some(P::Writing {
            completed: 3,
            total: 3
        })
    ));
    assert!(phases.iter().any(|p| matches!(p, P::Json)));
    assert!(phases.iter().any(|p| matches!(p, P::Markdown)));
    assert!(phases.iter().any(|p| matches!(p, P::Html)));
    let blocked = dir.join("file-not-directory");
    std::fs::write(&blocked, b"KEEP").unwrap();
    phases.clear();
    assert!(
        pqc_scan_report::write_migration_assessment_reports_with_progress(
            &r,
            &e,
            None,
            &blocked,
            ReportLanguage::Ja,
            |p| phases.push(p)
        )
        .is_err()
    );
    assert!(!phases
        .iter()
        .any(|p| matches!(p, P::Writing { completed: 3, .. })));
    assert_eq!(std::fs::read(blocked).unwrap(), b"KEEP");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn shared_candidates_have_one_definition_and_retain_all_target_links() {
    let mut v = fixture::inventory("RSA");
    let mut second = v["observations"][0].clone();
    second["id"] = pqc_scan_core::identity::native_id("observation", &["second"]).into();
    second["target"]["path"] = "other.rs".into();
    second["occurrence_key"] = pqc_scan_core::identity::native_id("occurrence", &["second"]).into();
    v["observations"].as_array_mut().unwrap().push(second);
    let evidence = fixture::ingest(&v, InputFormat::InventoryV1).unwrap();
    let mut claims = ctx::critical_signing();
    claims["support"] = ctx::support();
    let mut context = ctx::context(&evidence, claims);
    let mut second_binding = context["bindings"][0].clone();
    second_binding["target_id"] = evidence.targets()[1].id.clone().into();
    second_binding["claims"]["support"]["peer"] = ctx::claim(json!(false));
    context["bindings"]
        .as_array_mut()
        .unwrap()
        .push(second_binding);
    let r = evaluate(&evidence, Some(&ctx::parse(&context)), ctx::AS_OF).unwrap();
    assert_eq!(r.decisions().len(), 2);
    assert!(!r.decisions()[0].action.candidate_ids.is_empty());
    let before = serde_json::to_value(&r).unwrap();
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let md = text(&r, language);
        for i in 1..=r.decisions().len() {
            assert_eq!(md.matches(&format!("<a id=\"target-{i}\"></a>")).count(), 1);
        }
        for (i, candidate) in r.candidates().iter().enumerate() {
            let anchor = format!("candidate-{}", i + 1);
            assert_eq!(md.matches(&format!("<a id=\"{anchor}\"></a>")).count(), 1);
            let section = md
                .split(&format!("<a id=\"{anchor}\"></a>"))
                .nth(1)
                .unwrap();
            let section = section.split("<a id=").next().unwrap();
            let owner = r
                .decisions()
                .iter()
                .position(|d| d.target_id == candidate.target_id)
                .unwrap()
                + 1;
            assert!(
                section.contains(&format!("](#target-{owner})")),
                "candidate owner missing"
            );
            assert!(section.contains(
                if candidate
                    .prerequisites
                    .iter()
                    .any(|p| p.claimed_support == Some(false))
                {
                    if language == ReportLanguage::Ja {
                        "申告上は未対応"
                    } else {
                        "Declared unsupported"
                    }
                } else if language == ReportLanguage::Ja {
                    "申告上は対応"
                } else {
                    "Declared supported"
                }
            ));
            let references = r
                .decisions()
                .iter()
                .filter(|d| d.action.candidate_ids.contains(&candidate.id))
                .count();
            assert_eq!(md.matches(&format!("](#{anchor})")).count(), references);
        }
        assert_eq!(serde_json::to_value(&r).unwrap(), before);
    }
}
