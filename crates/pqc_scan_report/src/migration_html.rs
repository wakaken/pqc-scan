//! Human migration decisions, using the scan report's styles and preferences.
use crate::{
    migration_copy::{enum_token, location, Labels},
    migration_report::LimitedBuffer,
    ReportLanguage,
};
use pqc_scan_core::migration::{Information, MigrationAssessment, Readiness, Urgency};
use std::{collections::BTreeMap, io::Write};
pub const MIGRATION_HTML: &str = "migration.assessment.html";
fn h(s: &str) -> String {
    crate::japanese::html(
        &s.chars()
            .flat_map(|c| {
                if c.is_control() {
                    format!("\\u{{{:x}}}", c as u32).chars().collect::<Vec<_>>()
                } else {
                    vec![c]
                }
            })
            .collect::<String>(),
    )
}
fn tone(u: Urgency) -> &'static str {
    match u {
        Urgency::Urgent => "urgent",
        Urgency::Plan => "plan",
        Urgency::Monitor => "monitor",
        Urgency::Undetermined => "unknown",
        Urgency::NotApplicable => "outside",
    }
}
fn row(out: &mut LimitedBuffer, label: &str, value: &str) -> anyhow::Result<()> {
    write!(
        out,
        "<tr><th scope=\"row\">{}</th><td>{}</td></tr>",
        h(label),
        h(value)
    )?;
    Ok(())
}
pub fn migration_html_bytes(
    r: &MigrationAssessment,
    language: ReportLanguage,
) -> anyhow::Result<Vec<u8>> {
    let mut out = LimitedBuffer::default();
    let (active, alternate) = if language == ReportLanguage::Ja {
        ("ja", "en")
    } else {
        ("en", "ja")
    };
    write!(out,"<!doctype html><html lang=\"{active}\" data-theme=\"light\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><style>{}\n{}\n{}</style></head><body>",Labels(language).pick("Organization migration review | pqc-scan","組織の暗号移行レポート | pqc-scan"),crate::html_report::css(),include_str!("compact.css"),include_str!("migration.css"))?;
    write!(out,"<div class=\"report-settings\" data-settings hidden><label>Language / 言語<select data-language aria-label=\"Language / 言語\"><option value=\"en\">English</option><option value=\"ja\">日本語</option></select></label><label>Theme / 表示<select data-theme-select aria-label=\"Theme / 表示\"><option value=\"light\">Light / ライト</option><option value=\"dark\">Dark / ダーク</option></select></label></div>")?;
    view(&mut out, r, Labels(language), active)?;
    write!(
        out,
        "<template data-report-translation lang=\"{alternate}\">"
    )?;
    view(
        &mut out,
        r,
        Labels(if language == ReportLanguage::Ja {
            ReportLanguage::En
        } else {
            ReportLanguage::Ja
        }),
        alternate,
    )?;
    write!(
        out,
        "</template><script>{}</script></body></html>",
        include_str!("presentation.js")
    )?;
    Ok(out.bytes)
}
fn view(
    out: &mut LimitedBuffer,
    r: &MigrationAssessment,
    c: Labels,
    lang: &str,
) -> anyhow::Result<()> {
    let targets: BTreeMap<_, _> = r.targets().iter().map(|v| (v.id.as_str(), v)).collect();
    let candidates: BTreeMap<_, _> = r
        .candidates()
        .iter()
        .enumerate()
        .map(|(i, v)| (v.id.as_str(), (i + 1, v)))
        .collect();
    let claims: BTreeMap<_, _> = r
        .claims()
        .iter()
        .enumerate()
        .map(|(i, v)| (v.id.as_str(), (i + 1, v)))
        .collect();
    let sources: BTreeMap<_, _> = r
        .sources()
        .iter()
        .enumerate()
        .map(|(i, v)| (v.id.as_str(), (i + 1, v)))
        .collect();
    write!(out,"<main class=\"container migration\" data-report-view lang=\"{lang}\"><a class=\"skip-link\" href=\"#decisions\">{}</a><header class=\"top\"><div><p class=\"eyebrow\">pqc-scan <span>/</span> {}</p><h1>{}</h1><p class=\"report-description\">{}</p></div><p class=\"meta\">{}<br><time>{}</time></p></header>",c.pick("Skip to decisions","判断結果へ移動"),c.pick("Migration assessment","移行判断"),c.pick("Organization migration review","組織の暗号移行レポート"),c.pick("Saved findings and declared context show what needs attention and what to check next.","検出結果と業務上の申告から、対応が必要な対象と次の確認事項を整理します。"),c.pick("Evaluated at","評価時刻"),h(r.as_of()))?;
    write!(
        out,
        "<section class=\"summary\" aria-label=\"{}\">",
        c.pick("Response overview", "対応の見通し")
    )?;
    for (label, n) in [
        (c.pick("Targets", "評価対象"), r.decisions().len()),
        (
            c.pick("Urgent", "早急な対応"),
            r.decisions()
                .iter()
                .filter(|d| d.urgency == Urgency::Urgent)
                .count(),
        ),
        (
            c.pick("Blocked prerequisites", "阻害条件あり"),
            r.decisions()
                .iter()
                .filter(|d| d.readiness == Readiness::Blocked)
                .count(),
        ),
        (
            c.pick("Incomplete / conflicting information", "情報不足・矛盾"),
            r.decisions()
                .iter()
                .filter(|d| d.information != Information::Sufficient)
                .count(),
        ),
    ] {
        write!(out,"<div class=\"metric\"><div class=\"metric-label\">{label}</div><div class=\"metric-value\">{n}</div></div>")?;
    }
    write!(out,"</section><div class=\"migration-layout\"><nav class=\"migration-index\" aria-label=\"{}\"><h2>{}</h2>",c.pick("Assessment targets","評価対象"),c.pick("Targets","対象を選ぶ"))?;
    for (i, d) in r.decisions().iter().enumerate() {
        let t = targets
            .get(d.target_id.as_str())
            .ok_or_else(|| anyhow::anyhow!("invalid_reference"))?;
        write!(
            out,
            "<a href=\"#target-{}\"><strong>{}</strong><span>{} · {}</span></a>",
            i + 1,
            h(d.system_id
                .as_deref()
                .unwrap_or(c.pick("System not bound", "システム未設定"))),
            h(&t.technical.algorithm.identifier),
            c.urgency(d.urgency)
        )?;
    }
    write!(out,"</nav><section id=\"decisions\" class=\"migration-results\"><div class=\"results-heading\"><h2>{}</h2><span>{} {}</span></div>",c.pick("Response by target","対象ごとの対応"),r.decisions().len(),c.pick("targets","件"))?;
    if r.decisions().is_empty() {
        write!(
            out,
            "<p>{}</p>",
            c.pick(
                "No assessment target was projected. Review the saved collection scope.",
                "評価対象を抽出できませんでした。保存された収集範囲を確認してください。"
            )
        )?;
    }
    for (i, d) in r.decisions().iter().enumerate() {
        let t = targets[d.target_id.as_str()];
        write!(out,"<article class=\"decision-card {}\" id=\"target-{}\"><header><div><p class=\"eyebrow\">{}</p><h3>{} <span class=\"role\">{}</span></h3></div><span class=\"migration-badge {}\">{}</span></header><p class=\"migration-location\">{}</p><div class=\"decision-status\"><span>{}</span><span>{}</span></div><section class=\"response-box\"><p>{}</p><h4>{}</h4><p class=\"decision-reason\">{}</p></section>",tone(d.urgency),i+1,h(d.system_id.as_deref().unwrap_or(c.pick("System not bound","システム未設定"))),h(&t.technical.algorithm.identifier),c.token(&t.source_ref.role),tone(d.urgency),c.urgency(d.urgency),h(&location(t)),c.information(d.information),c.readiness(d.readiness),c.pick("Next response","次に取る対応"),c.action(d.action.kind),c.code(&d.reason_codes[0]))?;
        for (class, title, values) in [
            (
                "blockers",
                c.pick("Declared blockers", "申告された阻害条件"),
                &d.action.blocking_prerequisites,
            ),
            (
                "missing",
                c.pick("Information still needed", "判断に必要な追加情報"),
                &d.missing_inputs,
            ),
        ] {
            if !values.is_empty() {
                write!(
                    out,
                    "<section class=\"decision-note {class}\"><h4>{title}</h4><ul>"
                )?;
                for v in values {
                    write!(out, "<li>{}</li>", h(c.code(v)))?;
                }
                write!(out, "</ul></section>")?;
            }
        }
        write!(
            out,
            "<section class=\"next-checks\"><h4>{}</h4><ol>",
            c.pick("Next checks", "次の確認")
        )?;
        for v in &d.action.validation_steps {
            write!(out, "<li>{}</li>", h(c.code(v)))?;
        }
        if d.action.validation_steps.is_empty() {
            write!(
                out,
                "<li>{}</li>",
                c.pick(
                    "Reassess when facts, scope or declarations change.",
                    "事実・対象範囲・申告が変わった際に再評価してください。"
                )
            )?;
        }
        write!(out, "</ol></section>")?;
        if !d.action.candidate_ids.is_empty() {
            write!(out,"<section class=\"candidate-section\"><h4>{}</h4><p class=\"muted\">{}</p>",c.pick("Migration candidates","移行の検討候補"),c.pick("Integration is unverified. Choose exact parameters and test compatibility before adoption.","組込みは未検証です。採用前に正確なパラメーターを選び、互換性を検証してください。"))?;
            for id in &d.action.candidate_ids {
                let (n, v) = candidates[id.as_str()];
                write!(out,"<details id=\"candidate-{n}\"><summary><strong>{}</strong><span> {} · {}</span></summary><div class=\"detail-body\"><div class=\"table-scroll\"><table><caption>{}</caption><thead><tr><th>{}</th><th>{}</th></tr></thead><tbody>",h(&v.identifier),c.token(&enum_token(&v.purpose)),c.pick("Support to confirm","対応状況を確認"),c.pick("Candidate prerequisites","候補の必要条件"),c.pick("Prerequisite","必要な条件"),c.pick("Declared support","申告上の対応状況"))?;
                for p in &v.prerequisites {
                    row(out, c.code(&p.name), c.support(p.claimed_support))?;
                }
                write!(
                    out,
                    "</tbody></table></div><h5>{}</h5><ul>",
                    c.pick("Validate before adoption", "採用前に検証すること")
                )?;
                for v in &v.validation_steps {
                    write!(out, "<li>{}</li>", h(c.code(v)))?;
                }
                write!(
                    out,
                    "</ul><p class=\"reference-id\">ID: {}</p></div></details>",
                    h(id)
                )?;
            }
            write!(out, "</section>")?;
        }
        write!(out,"<details class=\"evidence-details\" id=\"evidence-{}\"><summary>{}</summary><div class=\"detail-body\"><div class=\"table-scroll\"><table><caption>{}</caption><tbody>",i+1,c.pick("Evidence and assumptions for this response","この対応とした根拠・前提情報"),c.pick("Recorded technical facts","記録された技術上の事実"))?;
        for (label, value) in [
            (c.pick("Role", "暗号の役割"), c.token(&t.source_ref.role)),
            (
                c.pick("Quantum exposure", "量子計算に対する評価"),
                c.token(&enum_token(&t.technical.algorithm.quantum)),
            ),
            (
                c.pick("Evidence assurance", "確認できている範囲"),
                c.token(&t.observation.assurance),
            ),
            (
                c.pick("Collection", "収集状況"),
                c.token(&t.observation.collection_outcome),
            ),
            (
                c.pick("Parameter encoding", "パラメーターの符号化"),
                c.token(&t.technical.encoding_assurance),
            ),
        ] {
            row(out, label, value)?;
        }
        for v in &t.technical.algorithm.classical_concerns {
            row(
                out,
                c.pick("Classical concern", "従来暗号の懸念"),
                c.token(v),
            )?;
        }
        for v in &t.technical.algorithm.components {
            row(
                out,
                c.pick("Hybrid component", "ハイブリッド構成要素"),
                &v.assessment.identifier,
            )?;
        }
        write!(out, "</tbody></table></div><ul>")?;
        for reason in d.reason_codes.iter().skip(1) {
            write!(out, "<li>{}</li>", h(c.code(reason)))?;
        }
        write!(
            out,
            "</ul><h5>{}</h5><ul>",
            c.pick("Claims used", "判断に使った申告")
        )?;
        for id in &d.applied_claim_ids {
            let (n, v) = claims[id.as_str()];
            write!(
                out,
                "<li><a href=\"#claim-{n}\">{}</a> · {} · {}</li>",
                h(c.code(&v.field)),
                h(&c.value(v)),
                c.freshness(v.freshness)
            )?;
        }
        write!(out, "</ul><p>{}: ", c.pick("Sources", "出典"))?;
        for id in &d.source_ids {
            let (n, _) = sources[id.as_str()];
            write!(out, "<a href=\"#source-{n}\">{}</a> ", h(id))?;
        }
        write!(
            out,
            "</p><p class=\"reference-id\">{}: {}<br>{}: {}</p></div></details></article>",
            c.pick("Target ID", "対象ID"),
            h(&t.id),
            c.pick("Decision ID", "判断ID"),
            h(&d.id)
        )?;
    }
    write!(out,"</section></div><section class=\"reference-section\"><h2>{}</h2><p class=\"muted\">{}</p><div class=\"table-scroll\"><table><caption>{}</caption><thead><tr><th>{}</th><th>{}</th><th>{}</th><th>{}</th></tr></thead><tbody>",c.pick("Declared context and provenance","判断に使った申告と由来"),c.pick("All claims are unverified, including external-review claims. Readiness is declared support, not verified deployment.","外部レビュー由来のものも含め、申告は未検証です。移行準備は申告上の対応状況であり、配備の検証結果ではありません。"),c.pick("Applied declarations","使用した申告"),c.pick("Item","項目"),c.pick("Declared value","申告された内容"),c.pick("Validity","有効性"),c.pick("Origin / time","由来・時刻"))?;
    for (i, v) in r.claims().iter().enumerate() {
        write!(out,"<tr id=\"claim-{}\"><th scope=\"row\">{}</th><td>{}</td><td>{}</td><td>{} · {}<br>{}<br>{}: {}</td></tr>",i+1,h(c.code(&v.field)),h(&c.value(v)),c.freshness(v.freshness),c.pick(if v.provenance.source_kind()=="external_review_claim" {"External-review claim (unverified)"}else{"Operator claim (unverified)"},if v.provenance.source_kind()=="external_review_claim" {"外部レビュー由来の申告（未検証）"}else{"利用者の申告（未検証）"}),h(v.provenance.actor_id()),h(v.provenance.asserted_at()),c.pick("Expires","有効期限"),h(v.provenance.valid_until().unwrap_or("unknown")))?;
    }
    write!(out,"</tbody></table></div></section><details class=\"reference-section\" id=\"assessment-sources\"><summary>{}</summary><div class=\"detail-body\">",c.pick("Source editions and assessment scope","出典の版・評価の範囲"))?;
    for (i, v) in r.sources().iter().enumerate() {
        write!(out,"<section id=\"source-{}\"><h3>{} <span class=\"role\">{}</span></h3><p>{}</p><p class=\"reference-id\">{}</p></section>",i+1,h(&v.id),h(&v.edition),h(&v.clause),h(v.uri.as_deref().or(v.local_source_id.as_deref()).unwrap_or("unknown")))?;
    }
    write!(out, "<ul>")?;
    for v in r.coverage().limits.iter().chain(r.limitations()) {
        write!(out, "<li>{}</li>", h(c.code(v)))?;
    }
    write!(out,"</ul><p>{}: {}</p><p class=\"reference-id\">SHA-256: {}</p><p>{} / {} / {}</p></div></details><footer><a href=\"migration.assessment.md\">Markdown</a><a href=\"migration.assessment.v1.json\" download>{}</a></footer></main>",c.pick("Collected at","収集時刻"),h(&r.input().collection_time),h(&r.input().sha256),h(&format!("{} {}",r.catalog().id,r.catalog().version)),h(&format!("{} {}",r.policy().id,r.policy().version)),h(&format!("{} {}",r.model().id,r.model().version)),c.pick("Machine-readable evidence (JSON)","機械処理用の根拠データ（JSON）"))?;
    Ok(())
}
