//! Readable human Markdown; exact machine records remain in the JSON companion.
use crate::{
    migration_copy::{enum_token, location, Labels},
    migration_report::LimitedBuffer,
    ReportLanguage,
};
use pqc_scan_core::migration::MigrationAssessment;
use std::{collections::BTreeMap, io::Write};
pub const MIGRATION_MARKDOWN: &str = "migration.assessment.md";
fn code(s: &str) -> String {
    let mut result = String::from("<code>");
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            c if c.is_control() => result.push_str(&format!("\\u{{{:x}}}", c as u32)),
            // HTML entities also neutralize Markdown/image/link delimiters
            // inside an inline <code> element in CommonMark renderers.
            c if c.is_ascii_punctuation() => result.push_str(&format!("&#{};", c as u32)),
            _ => result.push(c),
        }
    }
    result.push_str("</code>");
    result
}
fn index<T>(items: &[T], id: impl Fn(&T) -> &str) -> BTreeMap<&str, usize> {
    items
        .iter()
        .enumerate()
        .map(|(i, v)| (id(v), i + 1))
        .collect()
}
pub fn migration_markdown_bytes(
    r: &MigrationAssessment,
    language: ReportLanguage,
) -> anyhow::Result<Vec<u8>> {
    let c = Labels(language);
    let mut out = LimitedBuffer::default();
    let targets: BTreeMap<_, _> = r.targets().iter().map(|t| (t.id.as_str(), t)).collect();
    let candidates: BTreeMap<_, _> = r.candidates().iter().map(|v| (v.id.as_str(), v)).collect();
    let candidate_index = index(r.candidates(), |v| &v.id);
    let claim_index = index(r.claims(), |v| &v.id);
    let source_index = index(r.sources(), |v| &v.id);
    writeln!(
        out,
        "# {}\n",
        c.pick("Organization migration review", "組織の暗号移行レポート")
    )?;
    writeln!(out,"{}\n",c.pick("Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.","保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。"))?;
    writeln!(
        out,
        "{}: {} · {}: {}\n",
        c.pick("Targets", "対象数"),
        r.decisions().len(),
        c.pick("Evaluated at", "評価時刻"),
        code(r.as_of())
    )?;
    writeln!(out, "## {}\n", c.pick("Response overview", "対応の一覧"))?;
    writeln!(
        out,
        "| {} | {} | {} | {} |\n|---|---|---|---|",
        c.pick("System / location", "システム・場所"),
        c.pick("Primitive / role", "方式・役割"),
        c.pick("Urgency", "緊急度"),
        c.pick("Next response", "次の対応")
    )?;
    for (i, d) in r.decisions().iter().enumerate() {
        let t = targets
            .get(d.target_id.as_str())
            .ok_or_else(|| anyhow::anyhow!("invalid_reference"))?;
        writeln!(
            out,
            "| [{}](#target-{}) · {} | {} · {} | {} | {} |",
            code(
                d.system_id
                    .as_deref()
                    .unwrap_or(c.pick("Not bound", "未設定"))
            ),
            i + 1,
            code(&location(t)),
            code(&t.technical.algorithm.identifier),
            c.token(&t.source_ref.role),
            c.urgency(d.urgency),
            c.action(d.action.kind)
        )?;
    }
    writeln!(out)?;
    if r.decisions().is_empty() {
        writeln!(
            out,
            "{}\n",
            c.pick(
                "No assessment targets were projected. Review the saved collection scope.",
                "評価対象を抽出できませんでした。保存された収集範囲を確認してください。"
            )
        )?;
    }
    for (i, d) in r.decisions().iter().enumerate() {
        let t = targets[d.target_id.as_str()];
        writeln!(
            out,
            "<a id=\"target-{}\"></a>\n\n## {} · {}\n",
            i + 1,
            code(
                d.system_id
                    .as_deref()
                    .unwrap_or(c.pick("System not bound", "システム未設定"))
            ),
            code(&t.technical.algorithm.identifier)
        )?;
        writeln!(
            out,
            "**{}** · {} · {}\n",
            c.urgency(d.urgency),
            c.information(d.information),
            c.readiness(d.readiness)
        )?;
        writeln!(
            out,
            "{}: {} ({})\n",
            c.pick("Where", "場所"),
            code(&location(t)),
            c.token(&t.source_ref.role)
        )?;
        if let Some(provenance) = &t.source_ref.dependency_provenance {
            writeln!(
                out,
                "{}: {}\n",
                c.pick("Dependency declaration", "依存関係の宣言"),
                code(c.code(provenance))
            )?;
        }
        writeln!(
            out,
            "### {}\n\n{}\n\n{}\n",
            c.pick("Next response", "次の対応"),
            c.action(d.action.kind),
            c.code(&d.reason_codes[0])
        )?;
        for (name, items) in [
            (
                c.pick("Declared blockers", "申告された阻害条件"),
                &d.action.blocking_prerequisites,
            ),
            (
                c.pick("Obtain this information", "判断に必要な追加情報"),
                &d.missing_inputs,
            ),
        ] {
            if !items.is_empty() {
                writeln!(out, "**{name}**\n")?;
                for s in items {
                    writeln!(out, "- {}", c.code(s))?;
                }
                writeln!(out)?;
            }
        }
        writeln!(out, "**{}**\n", c.pick("Next checks", "次の確認"))?;
        for s in &d.action.validation_steps {
            writeln!(out, "- {}", c.code(s))?;
        }
        if d.action.validation_steps.is_empty() {
            writeln!(
                out,
                "- {}",
                c.pick(
                    "Reassess when facts, scope or claims change.",
                    "事実・対象範囲・申告が変わった際に再評価してください。"
                )
            )?;
        }
        writeln!(
            out,
            "\n### {}\n",
            c.pick("Migration candidates", "移行の検討候補")
        )?;
        if d.action.candidate_ids.is_empty() {
            writeln!(
                out,
                "{}\n",
                c.pick(
                    "No candidate selected for this decision.",
                    "この判断で選んだ候補はありません。"
                )
            )?;
        }
        for id in &d.action.candidate_ids {
            let v = candidates[id.as_str()];
            writeln!(
                out,
                "<a id=\"candidate-{}\"></a>\n\n**{} · {}**\n",
                candidate_index[id.as_str()],
                code(&v.identifier),
                c.token(&enum_token(&v.purpose))
            )?;
            writeln!(out,"{}\n",c.pick("Integration is unverified. Select exact parameters and validate compatibility before adoption.","組込みは未検証です。採用前に正確なパラメーターを選び、互換性を検証してください。"))?;
            writeln!(
                out,
                "| {} | {} |\n|---|---|",
                c.pick("Prerequisite", "必要な条件"),
                c.pick("Declared support", "申告上の対応状況")
            )?;
            for p in &v.prerequisites {
                writeln!(
                    out,
                    "| {} | {} |",
                    c.code(&p.name),
                    c.support(p.claimed_support)
                )?;
            }
            writeln!(out)?;
        }
        writeln!(
            out,
            "<a id=\"evidence-{}\"></a>\n\n### {}\n",
            i + 1,
            c.pick("Evidence for this response", "この対応とした根拠")
        )?;
        if let Some(explanation) = crate::migration_copy::observation_explanation(t, language) {
            writeln!(out, "{explanation}\n")?;
        }
        writeln!(
            out,
            "| {} | {} |\n|---|---|",
            c.pick("Fact", "確認項目"),
            c.pick("Recorded value", "記録された内容")
        )?;
        for (label, value) in [
            (
                c.pick("Primitive", "暗号方式"),
                t.technical.algorithm.identifier.clone(),
            ),
            (
                c.pick("Quantum exposure", "量子計算に対する評価"),
                c.token(&enum_token(&t.technical.algorithm.quantum)).into(),
            ),
            (
                c.pick("Evidence assurance", "確認できている範囲"),
                c.token(&t.observation.assurance).into(),
            ),
            (
                c.pick("Observed use state", "検出時の利用状態"),
                c.token(&t.observation.usage).into(),
            ),
            (
                c.pick("Collection", "収集状況"),
                c.token(&t.observation.collection_outcome).into(),
            ),
        ] {
            writeln!(out, "| {label} | {} |", code(&value))?;
        }
        writeln!(out)?;
        for reason in d.reason_codes.iter().skip(1) {
            writeln!(out, "- {}", c.code(reason))?;
        }
        writeln!(out)?;
        for id in &d.applied_claim_ids {
            let claim = &r.claims()[claim_index[id.as_str()] - 1];
            writeln!(
                out,
                "- [{}](#claim-{})",
                c.code(&claim.field),
                claim_index[id.as_str()]
            )?;
        }
        for id in &d.source_ids {
            writeln!(
                out,
                "- [{}](#source-{})",
                code(id),
                source_index[id.as_str()]
            )?;
        }
        writeln!(
            out,
            "\n<details><summary>{}</summary>\n\n{}: {}\n\n{}: {}\n\n</details>\n",
            c.pick("Reference IDs", "照合用ID"),
            c.pick("Target", "対象"),
            code(&t.id),
            c.pick("Decision", "判断"),
            code(&d.id)
        )?;
    }
    writeln!(out,"## {}\n\n{}\n",c.pick("Declared context and provenance","判断に使った申告と由来"),c.pick("These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.","外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。"))?;
    for (i, v) in r.claims().iter().enumerate() {
        writeln!(
            out,
            "<a id=\"claim-{}\"></a>\n\n### {}\n\n{} · {}\n\n{}: {} · {}\n\n{}: {} / {}: {}\n",
            i + 1,
            c.code(&v.field),
            code(&c.value(v)),
            c.freshness(v.freshness),
            c.pick("Declared by", "申告者"),
            code(v.provenance.actor_id()),
            c.pick(
                if v.provenance.source_kind() == "external_review_claim" {
                    "External-review claim (unverified)"
                } else {
                    "Operator claim (unverified)"
                },
                if v.provenance.source_kind() == "external_review_claim" {
                    "外部レビュー由来の申告（未検証）"
                } else {
                    "利用者の申告（未検証）"
                }
            ),
            c.pick("Asserted", "申告時刻"),
            code(v.provenance.asserted_at()),
            c.pick("Expires", "有効期限"),
            code(v.provenance.valid_until().unwrap_or("unknown"))
        )?;
    }
    writeln!(out, "## {}\n", c.pick("Sources", "判断の出典"))?;
    for (i, v) in r.sources().iter().enumerate() {
        writeln!(
            out,
            "<a id=\"source-{}\"></a>\n\n**{} · {}**\n\n{} · {}\n",
            i + 1,
            code(&v.id),
            code(&v.edition),
            code(&v.clause),
            code(
                v.uri
                    .as_deref()
                    .or(v.local_source_id.as_deref())
                    .unwrap_or("unknown")
            )
        )?;
    }
    writeln!(out, "## {}\n", c.pick("Assessment scope", "評価の範囲"))?;
    for s in r.coverage().limits.iter().chain(r.limitations()) {
        writeln!(out, "- {}", c.code(s))?;
    }
    writeln!(
        out,
        "\n{}: {}\n\n{}: {}\n\n[{}](migration.assessment.v1.json)\n",
        c.pick("Input SHA-256", "入力SHA-256"),
        code(&r.input().sha256),
        c.pick("Collected at", "収集時刻"),
        code(&r.input().collection_time),
        c.pick(
            "Machine-readable evidence and complete reference data (JSON)",
            "機械処理用の根拠・照合データ（JSON）"
        )
    )?;
    Ok(out.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn literal_controls_cannot_create_headings_links_or_html() {
        let s = code("x\n# injected ![x](//evil) `<script>");
        assert!(!s.contains('\n'));
        assert!(!s.contains("!["));
        assert!(!s.contains("<script>"));
        assert!(s.contains("\\u{a}"));
        assert!(s.contains("&lt;script&gt;"));
    }
}
