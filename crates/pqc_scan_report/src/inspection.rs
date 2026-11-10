//! Human-only views of existing facts; this module never evaluates policy.
use crate::{japanese, ReportLanguage};
use anyhow::{ensure, Result};
use pqc_scan_core::{
    policy::{AssessmentReport, Decision},
    DiagnosticCode, InspectionStatus, ScanDocument,
};
use std::fmt::Write;

pub(crate) struct HumanContext<'a> {
    pub document: &'a ScanDocument,
    pub assessment: Option<&'a AssessmentReport>,
    pub diagnostics: bool,
}
impl HumanContext<'_> {
    pub fn validate(&self) -> Result<()> {
        if let Some(report) = self.assessment {
            ensure!(
                self.document
                    .native()
                    .is_some_and(|n| n.run_id() == report.inventory_run_id()),
                "assessment belongs to another inventory"
            );
            report.validate_for_inventory(
                self.document.native().expect("checked native inventory"),
            )?;
        }
        Ok(())
    }
}
fn text<'a>(lang: ReportLanguage, en: &'a str, ja: &'a str) -> &'a str {
    if lang == ReportLanguage::Ja {
        ja
    } else {
        en
    }
}
fn decision(value: Decision, lang: ReportLanguage) -> String {
    let (code, en, ja) = match value {
        Decision::Pass => ("pass", "Within local policy", "ローカル方針の条件を満たす"),
        Decision::Fail => ("fail", "Local policy concern", "ローカル方針で要確認"),
        Decision::Indeterminate => ("indeterminate", "Indeterminate", "判定保留"),
        Decision::NotApplicable => ("not_applicable", "Not applicable", "評価対象外"),
    };
    format!("{} ({code})", text(lang, en, ja))
}
fn state(value: &str, lang: ReportLanguage) -> String {
    let translated = match value {
        "lexical" => text(lang, "Lexical match", "文字列の一致"),
        "syntactic" => text(lang, "Syntactic pattern", "構文パターンの一致"),
        "parsed" => text(lang, "Parsed public data", "公開情報を解析済み"),
        "referenced" => text(lang, "Referenced", "参照あり"),
        "configured" => text(lang, "Configured", "設定あり"),
        "unknown" => text(lang, "Unknown", "不明"),
        _ => return value.to_owned(),
    };
    format!("{translated} ({value})")
}
fn diagnostic(code: DiagnosticCode, lang: ReportLanguage) -> Result<String> {
    use DiagnosticCode::*;
    let (en, ja) = match code {
        ReadFailed => ("File could not be read", "ファイルを読み込めませんでした"),
        DetectorFailed => ("Detector failed", "検出器の処理に失敗しました"),
        DetectorNotReported => (
            "Detector does not report completeness",
            "検出器が検査の完全性を報告しません",
        ),
        DetectorUnsupported => ("Inspection unsupported", "対応していない検査です"),
        DetectorPartial => (
            "Only part of inspection completed",
            "検査の一部だけが完了しました",
        ),
        DetectorTruncated => ("Inspection limit reached", "検査の上限に達しました"),
        NoDetectors => ("No detectors supplied", "検出器が指定されていません"),
        ExcludedPath => ("Excluded by default policy", "既定の設定で対象外です"),
        FileTooLarge => (
            "File exceeds size limit",
            "ファイルサイズが上限を超えています",
        ),
        MetadataFailed => (
            "Metadata could not be read",
            "ファイル情報を読み取れませんでした",
        ),
        EnumerationFailed => (
            "Repository entry could not be enumerated",
            "リポジトリ内の項目を列挙できませんでした",
        ),
    };
    let code = serde_json::to_value(code)?;
    Ok(format!(
        "{} ({})",
        text(lang, en, ja),
        code.as_str().unwrap_or("unknown")
    ))
}
struct Section {
    heading: String,
    intro: String,
    rows: Vec<(String, String)>,
}
fn sections(context: &HumanContext<'_>, lang: ReportLanguage) -> Result<Vec<Section>> {
    let c = context.document.coverage();
    let status = match c.status() {
        InspectionStatus::CompleteWithinScope => text(
            lang,
            "Inspection complete within declared scope",
            "宣言した範囲内で検査完了",
        ),
        InspectionStatus::Partial => text(
            lang,
            "Some inspection could not complete",
            "一部を検査できませんでした",
        ),
        InspectionStatus::Failed => text(lang, "Inspection failed", "検査に失敗しました"),
        InspectionStatus::Unknown => text(
            lang,
            "Inspection completeness is unknown",
            "検査の完了を確認できません",
        ),
    };
    let mut rows = vec![(
        text(lang, "Status", "検査状態").into(),
        format!("{status} ({})", c.status().as_str()),
    )];
    for (en, ja, count) in [
        ("Selected files", "選択したファイル", c.selected_files()),
        ("Read successfully", "読み込み成功", c.read_ok_files()),
        ("Read failed", "読み込み失敗", c.read_failed_files()),
        ("Analyzed", "解析完了", c.analyzed_files()),
        ("Partial", "部分成功", c.partial_files()),
        ("Analysis failed", "解析失敗", c.analysis_failed_files()),
        ("Unsupported", "未対応", c.unsupported_files()),
        ("Not applicable", "検出器の対象外", c.not_applicable_files()),
        ("Unknown", "不明", c.unknown_files()),
        (
            "Enumerated exclusions",
            "列挙できた除外",
            c.excluded_enumerated_files(),
        ),
        (
            "Enumeration failures",
            "列挙失敗",
            c.enumeration_failed_entries(),
        ),
    ] {
        rows.push((text(lang, en, ja).into(), count.to_string()));
    }
    rows.push((
        text(lang, "Unenumerated population", "列挙されなかった範囲").into(),
        if c.not_enumerated_population() {
            text(lang, "Unknown", "不明")
        } else {
            text(lang, "Recorded", "記録済み")
        }
        .into(),
    ));
    let mut result = vec![Section {
        heading: text(lang, "Inspection coverage", "検査範囲と完了状況").into(),
        intro: text(lang, "Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.", "完了は、ignore設定・除外・サイズ制限を適用した範囲内で、対応する検査を終えたという意味です。実際の稼働、証明書の信頼性、安全性を保証しません。列挙されなかったファイルは不明です。").into(), rows,
    }];
    if context.diagnostics {
        let rows = context
            .document
            .diagnostics()
            .iter()
            .map(|d| {
                Ok((
                    diagnostic(d.code(), lang)?,
                    format!(
                        "{} / {}",
                        d.file().unwrap_or("-"),
                        d.detector().unwrap_or("-")
                    ),
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        result.push(Section { heading: text(lang, "Inspection diagnostics", "検査の診断情報").into(), intro: text(lang, "Safe reason codes, paths and detector names; file contents and arbitrary error messages are omitted.", "理由コード、パス、検出器名を示します。ファイル内容や任意のエラー本文は含めません。").into(), rows });
    }
    if let Some(native) = context.document.native() {
        let rows = native
            .observations()
            .iter()
            .map(|o| {
                (
                    o.id().to_string(),
                    format!(
                        "{}: {}; {}: {}",
                        text(lang, "Assurance", "確認根拠"),
                        state(o.assurance(), lang),
                        text(lang, "Usage", "利用状態"),
                        state(o.usage(), lang)
                    ),
                )
            })
            .collect();
        result.push(Section { heading: text(lang, "Observation assurance and usage", "観測の確認根拠と利用状態").into(), intro: text(lang, "Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.", "確認根拠は情報を集めた方法、利用状態は観測した内容を示します。静的な参照や証明書の解析だけでは、稼働や信頼性を確認したことにはなりません。").into(), rows });
    }
    if let Some(report) = context.assessment {
        let mut rows = vec![
            (
                text(lang, "Profile", "評価方針").into(),
                report.profile_id().into(),
            ),
            (
                text(lang, "Overall decision", "全体の判定").into(),
                decision(report.decision(), lang),
            ),
            (
                text(lang, "Reason codes", "理由コード").into(),
                report.evaluation().reasons.join(", "),
            ),
        ];
        for p in report.asset_policies() {
            rows.push((
                p.asset_id.clone(),
                format!("{}; {}", decision(p.decision, lang), p.reasons.join(", ")),
            ));
        }
        result.push(Section { heading: text(lang, "Local advisory assessment", "ローカルの参考評価").into(), intro: text(lang, "This versioned local policy uses collected facts and unverified operator context. It does not guarantee runtime operation or migration completion. A pass applies only to the stated policy conditions.", "収集した事実と、利用者が申告した未検証の条件に基づくローカル方針の評価です。稼働確認や移行完了を保証しません。条件を満たす判定も、その評価方針の範囲に限ります。").into(), rows });
    }
    Ok(result)
}
pub(crate) fn html(context: &HumanContext<'_>, lang: ReportLanguage) -> Result<String> {
    let mut out = String::new();
    for section in sections(context, lang)? {
        writeln!(
            out,
            "<section class=\"inspection-context\"><h2>{}</h2><p>{}</p><table><tbody>",
            japanese::html(&section.heading),
            japanese::html(&section.intro)
        )?;
        for (label, value) in section.rows {
            writeln!(
                out,
                "<tr><th scope=\"row\">{}</th><td>{}</td></tr>",
                japanese::html(&label),
                japanese::html(&value)
            )?;
        }
        writeln!(out, "</tbody></table></section>")?;
    }
    Ok(out)
}
pub(crate) fn markdown(context: &HumanContext<'_>, lang: ReportLanguage) -> Result<String> {
    let mut out = String::new();
    let escape = |value: &str| japanese::md(&value.replace('&', "&amp;")).replace('\r', " ");
    for section in sections(context, lang)? {
        writeln!(
            out,
            "\n## {}\n\n{}\n\n|{}|{}|\n|---|---|",
            escape(&section.heading),
            escape(&section.intro),
            text(lang, "Item", "項目"),
            text(lang, "Value", "内容")
        )?;
        for (label, value) in section.rows {
            writeln!(out, "|{}|{}|", escape(&label), escape(&value))?;
        }
    }
    Ok(out)
}
