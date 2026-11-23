//! Grouped human review backed by one lossless shared assessment archive.
use crate::{
    migration_copy::{enum_token, Labels},
    migration_report::LimitedBuffer,
    migration_view::{copy_table, work_groups, Archive},
    ReportLanguage,
};
use pqc_scan_core::migration::{MigrationAssessment, ScanReference};
use serde_json::json;
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
fn data_block(
    out: &mut LimitedBuffer,
    id: &str,
    value: &impl serde::Serialize,
) -> anyhow::Result<()> {
    let raw = serde_json::to_string(value)?.replace('<', "\\u003c");
    write!(
        out,
        "<script type=\"application/json\" id=\"{id}\">{raw}</script>"
    )?;
    Ok(())
}
pub fn migration_html_bytes(
    r: &MigrationAssessment,
    language: ReportLanguage,
) -> anyhow::Result<Vec<u8>> {
    migration_html_bytes_with_presentation(r, language, None, &[], &BTreeMap::new())
}

pub(crate) fn validate_label(label: Option<&str>) -> anyhow::Result<()> {
    if label
        .is_some_and(|s| s.trim().is_empty() || s.len() > 256 || s.chars().any(char::is_control))
    {
        anyhow::bail!("invalid_report_label");
    }
    Ok(())
}

pub(crate) fn migration_html_bytes_with_presentation(
    r: &MigrationAssessment,
    language: ReportLanguage,
    label: Option<&str>,
    unbindable: &[&str],
    scan_references: &BTreeMap<String, ScanReference>,
) -> anyhow::Result<Vec<u8>> {
    validate_label(label)?;
    let mut out = LimitedBuffer::default();
    let active = if language == ReportLanguage::Ja {
        "ja"
    } else {
        "en"
    };
    let alternate = if active == "ja" { "en" } else { "ja" };
    let raw = serde_json::to_value(r)?;
    let groups = work_groups(r);
    let words = copy_table(&raw);
    let mut states = BTreeMap::new();
    for l in [ReportLanguage::En, ReportLanguage::Ja] {
        let c = Labels(l);
        let key = if l == ReportLanguage::Ja { "ja" } else { "en" };
        let mut values = BTreeMap::new();
        for d in r.decisions() {
            values.insert(
                format!("urgency:{}", enum_token(&d.urgency)),
                c.urgency(d.urgency).to_owned(),
            );
            values.insert(
                format!("information:{}", enum_token(&d.information)),
                c.information(d.information).to_owned(),
            );
            values.insert(
                format!("readiness:{}", enum_token(&d.readiness)),
                c.readiness(d.readiness).to_owned(),
            );
            values.insert(
                format!("action:{}", enum_token(&d.action.kind)),
                c.action(d.action.kind).to_owned(),
            );
        }
        states.insert(key, values);
    }
    write!(out,"<!doctype html><html lang=\"{active}\" data-theme=\"light\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><script>{}</script><style>{}\n{}\n{}</style></head><body>",Labels(language).pick("Migration review | pqc-scan","暗号移行レポート | pqc-scan"),include_str!("theme-init.js"),crate::html_report::css(),include_str!("compact.css"),include_str!("migration.css"))?;
    write!(out,"<div class=\"report-settings\" data-settings hidden><label>Language / 言語<select data-language aria-label=\"Language / 言語\"><option value=\"en\">English</option><option value=\"ja\">日本語</option></select></label><label>Theme / 表示<select data-theme-select aria-label=\"Theme / 表示\"><option value=\"light\">Light / ライト</option><option value=\"dark\">Dark / ダーク</option></select></label></div>")?;
    // A useful bounded fallback precedes the inert shared data. No per-target templates.
    fallback(&mut out, r, language, active)?;
    write!(
        out,
        "<template data-report-translation lang=\"{alternate}\">"
    )?;
    fallback(
        &mut out,
        r,
        if language == ReportLanguage::Ja {
            ReportLanguage::En
        } else {
            ReportLanguage::Ja
        },
        alternate,
    )?;
    write!(out, "</template>")?;
    data_block(
        &mut out,
        "migration-shared-data",
        &json!({"archive":Archive::new(&raw),"groups":groups,"words":words,"states":states,"label":label,"unbindable":unbindable,"scanReferences":scan_references}),
    )?;
    write!(
        out,
        "<script>{}\n{}</script></body></html>",
        include_str!("migration.js"),
        include_str!("presentation.js")
    )?;
    Ok(out.bytes)
}
fn fallback(
    out: &mut LimitedBuffer,
    r: &MigrationAssessment,
    language: ReportLanguage,
    lang: &str,
) -> anyhow::Result<()> {
    let c = Labels(language);
    let groups = work_groups(r);
    write!(
        out,
        "<main class=\"container migration\" data-report-view lang=\"{lang}\"><header class=\"top\"><h1>{}</h1><time>{}</time></header><noscript>{}</noscript><p>{}: {} · {}: {}</p><section id=\"decisions\">",
        c.pick("Migration response work items", "移行に向けた確認事項"),
        h(r.as_of()),
        c.pick(
            "Enable JavaScript to browse every work item and its evidence.",
            "すべての確認項目と根拠を見るにはJavaScriptを有効にしてください。"
        ),
        c.pick("Work items", "確認項目"),
        groups.len(),
        c.pick("Saved records", "保存された記録"),
        r.decisions().len()
    )?;
    for g in groups.iter().take(20) {
        let d = &r.decisions()[g.members[0]];
        write!(
            out,
            "<article class=\"decision-card\"><h2>{}</h2><p>{} · {} · {}</p><ul>",
            h(c.action(d.action.kind)),
            h(c.urgency(d.urgency)),
            h(c.information(d.information)),
            h(c.readiness(d.readiness))
        )?;
        for code in d.reason_codes.iter().chain(&d.action.validation_steps) {
            write!(out, "<li>{}</li>", h(c.code(code)))?;
        }
        write!(
            out,
            "</ul><p>{}: {}</p></article>",
            c.pick("Records", "記録"),
            g.members.len()
        )?;
    }
    write!(
        out,
        "</section><footer><a href=\"migration.assessment.v1.json\" download>{}</a></footer></main>",
        c.pick("Save assessment data", "評価データを保存")
    )?;
    Ok(())
}

#[cfg(test)]
mod paging_tests {
    use super::*;
    #[test]
    fn inert_json_neutralizes_raw_text_delimiters_without_changing_values() {
        let value = serde_json::json!([
            "</sCrIpT><ScRiPt>alert(1)</SCRIPT><!--",
            "日本語 🗝️",
            "<div>safe</div>"
        ]);
        let mut out = LimitedBuffer::default();
        data_block(&mut out, "fixture", &value).unwrap();
        let html = String::from_utf8(out.bytes).unwrap();
        let body = html
            .split_once('>')
            .unwrap()
            .1
            .strip_suffix("</script>")
            .unwrap();
        assert!(!body.to_lowercase().contains("</script"));
        assert!(!body.to_lowercase().contains("<script"));
        assert!(!body.contains("<!--"));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(body).unwrap(),
            value
        );
    }
    #[test]
    fn display_label_is_bounded_without_becoming_a_data_or_markup_contract() {
        assert!(validate_label(Some("Keycloak / JWT <script>")).is_ok());
        assert!(validate_label(Some(" ")).is_err());
        assert!(validate_label(Some("name\nother")).is_err());
        assert!(validate_label(Some(&"x".repeat(257))).is_err());
        assert!(validate_label(Some(&"日".repeat(85))).is_ok());
    }
    #[test]
    fn shared_html_data_respects_the_existing_byte_limit() {
        let value = "x".repeat(crate::MAX_MIGRATION_REPORT_BYTES);
        let mut out = LimitedBuffer::default();
        assert!(data_block(&mut out, "large", &value).is_err());
        assert!(out.bytes.len() <= crate::MAX_MIGRATION_REPORT_BYTES);
    }
}
