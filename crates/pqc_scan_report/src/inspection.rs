//! Human-only views of existing facts; this module never evaluates policy.
use crate::{japanese, ReportLanguage};
use anyhow::{ensure, Result};
use pqc_scan_core::{
    policy::{AssessmentReport, Decision},
    DiagnosticCode, InspectionStatus, ScanDocument,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

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
fn diagnostic_text(code: DiagnosticCode, lang: ReportLanguage) -> &'static str {
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
    text(lang, en, ja)
}
fn diagnostic(code: DiagnosticCode, lang: ReportLanguage) -> Result<String> {
    let encoded = serde_json::to_value(code)?;
    Ok(format!(
        "{} ({})",
        diagnostic_text(code, lang),
        encoded.as_str().unwrap_or("unknown")
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
/// HTML is an investigation summary, not a second inventory dump. The exact
/// per-ID ledger remains in Markdown/inventory/assessment exports.
pub(crate) fn html(context: &HumanContext<'_>, lang: ReportLanguage) -> Result<String> {
    let mut out = String::new();
    let c = context.document.coverage();
    let scope = context.document.scope();
    let h = japanese::html;
    let heading = text(
        lang,
        "Inspection scope and completion",
        "検査範囲と完了状況",
    );
    let (status, advice) = match c.status() {
        InspectionStatus::CompleteWithinScope => (
            text(lang, "Supported inspection completed within the selected scope", "対象範囲の検査は完了しました"),
            text(lang, "Review the findings below. Completion does not mean the project is safe.", "下の所見を確認してください。検査完了は、安全性や移行完了を意味しません。")),
        InspectionStatus::Partial => (
            text(lang, "Some files or inspection steps could not be checked fully", "一部のファイル・検査を確認できていません"),
            text(lang, "Check the reasons below before relying on an absence of findings.", "下の理由を確認してください。未確認の範囲に問題がないとは判断できません。")),
        InspectionStatus::Failed => (
            text(lang, "Inspection failed", "検査に失敗しました"),
            text(lang, "Resolve the read/inspection failures below, then scan again.", "下の読み込み・検査失敗の原因を解消して、再検査してください。")),
        InspectionStatus::Unknown => (
            text(lang, "Inspection completion could not be established", "検査を終えたか確認できません"),
            text(lang, "Some inspection methods do not report completion. Retain their findings, but check the original files.", "完了状況を報告しない検査があります。所見を残したうえで、元のファイルも確認してください。")),
    };
    writeln!(out, "<section class=\"inspection-overview inspection-context\" data-inspection-status=\"{}\"><h2>{heading}</h2><p class=\"inspection-status\">{}</p><p>{}</p><div class=\"inspection-metrics\">", c.status().as_str(), h(status), h(advice))?;
    let needs_check = c.read_failed_files()
        + c.partial_files()
        + c.analysis_failed_files()
        + c.unsupported_files()
        + c.unknown_files();
    for (label, count) in [
        (text(lang, "Files selected", "検査対象"), c.selected_files()),
        (text(lang, "Files analyzed", "解析完了"), c.analyzed_files()),
        (
            text(lang, "Files needing follow-up", "追加確認が必要"),
            needs_check,
        ),
    ] {
        writeln!(
            out,
            "<div><span>{label}</span><strong>{count}</strong></div>"
        )?;
    }
    writeln!(out, "</div><p>{}</p><ul class=\"inspection-scope\">", text(lang, "This is a local inspection of supported files, not a live connection test. File counts are not a cryptographic coverage percentage.", "対応する形式のローカルファイルを調べた結果です。実通信や実際の利用は確認していません。ファイル件数は、暗号利用の網羅率ではありません。"))?;
    writeln!(
        out,
        "<li>{}: <code>{}</code></li>",
        text(lang, "Excluded directories", "除外するディレクトリ"),
        h(&scope.default_excludes().join(", "))
    )?;
    writeln!(
        out,
        "<li>{}: {} bytes{}</li>",
        text(lang, "File size limit", "1ファイルの上限"),
        scope.max_file_size_bytes(),
        if scope.max_file_size_bytes() == 2 * 1024 * 1024 {
            " (2 MiB)"
        } else {
            ""
        }
    )?;
    if scope.git_ignore() && scope.git_global() && scope.git_exclude() {
        writeln!(
            out,
            "<li>{}</li>",
            text(
                lang,
                "Git exclusion settings, including .gitignore, are applied.",
                ".gitignoreなどGitの除外設定を適用しています。"
            )
        )?;
    } else {
        writeln!(
            out,
            "<li>.gitignore: {}; global Git ignore: {}; .git/info/exclude: {}</li>",
            scope.git_ignore(),
            scope.git_global(),
            scope.git_exclude()
        )?;
    }
    if scope.markdown_excluded() {
        writeln!(
            out,
            "<li>{}</li>",
            text(
                lang,
                "Markdown files are excluded.",
                "Markdownファイルは対象外です。"
            )
        )?;
    }
    writeln!(
        out,
        "<li>{}: {}{}</li>",
        text(
            lang,
            "Recorded excluded files",
            "今回記録できた対象外ファイル"
        ),
        c.excluded_enumerated_files(),
        if c.not_enumerated_population() {
            text(
                lang,
                "; files omitted by ignore rules cannot be counted completely.",
                "件。ignore設定などで列挙されなかったファイルは数え切れていません。",
            )
        } else {
            text(
                lang,
                "; unenumerated population is recorded.",
                "件。列挙範囲は記録済みです。",
            )
        }
    )?;
    writeln!(
        out,
        "</ul><div class=\"inspection-reasons\"><h3>{}</h3>",
        text(
            lang,
            "Unfinished checks and reasons",
            "未完了・対象外の内訳"
        )
    )?;
    let mut any = false;
    for (label, count) in [
        (
            text(lang, "Unreadable files", "読み込めないファイル"),
            c.read_failed_files(),
        ),
        (
            text(
                lang,
                "Partially inspected files",
                "一部だけ検査したファイル",
            ),
            c.partial_files(),
        ),
        (
            text(lang, "Inspection failures", "解析に失敗したファイル"),
            c.analysis_failed_files(),
        ),
        (
            text(
                lang,
                "Files with unsupported inspection",
                "未対応の検査があるファイル",
            ),
            c.unsupported_files(),
        ),
        (
            text(
                lang,
                "Files with unknown completion",
                "完了を確認できないファイル",
            ),
            c.unknown_files(),
        ),
        (
            text(
                lang,
                "No applicable inspection",
                "適用する検査がないファイル",
            ),
            c.not_applicable_files(),
        ),
        (
            text(
                lang,
                "Entries that could not be enumerated",
                "列挙できない項目",
            ),
            c.enumeration_failed_entries(),
        ),
    ] {
        if count > 0 {
            any = true;
            writeln!(out, "<p>{label}: {count}</p>")?;
        }
    }
    if !any {
        writeln!(
            out,
            "<p>{}</p>",
            text(
                lang,
                "No unfinished inspection was recorded among selected files.",
                "検査対象に選んだファイルに、未完了の記録はありません。"
            )
        )?;
    }
    if context.diagnostics {
        let mut grouped: BTreeMap<DiagnosticCode, Vec<_>> = BTreeMap::new();
        for d in context.document.diagnostics() {
            grouped.entry(d.code()).or_default().push(d);
        }
        if !grouped.is_empty() {
            writeln!(
                out,
                "<details class=\"inspection-diagnostics\"><summary>{}</summary>",
                text(lang, "Reasons and example files", "理由とファイルの例")
            )?;
            for (code, records) in grouped {
                let files: BTreeSet<_> = records.iter().filter_map(|d| d.file()).collect();
                writeln!(
                    out,
                    "<div class=\"inspection-reason\"><p><strong>{}</strong> — {}{}</p><ul>",
                    h(diagnostic_text(code, lang)),
                    records.len(),
                    text(lang, " records", "件")
                )?;
                for file in files.iter().take(3) {
                    writeln!(out, "<li><code>{}</code></li>", h(file))?;
                }
                if files.len() > 3 {
                    writeln!(
                        out,
                        "<li>{}{}{}</li>",
                        text(lang, "Plus ", "ほか"),
                        files.len() - 3,
                        text(lang, " other files", "件")
                    )?;
                }
                writeln!(out, "</ul></div>")?;
            }
            writeln!(out,"<p>{}</p></details>",text(lang,"Only up to three example paths per reason are shown. Full diagnostic records remain in the optional diagnostics JSON and Markdown report.","理由ごとに最大3件のファイル例を示します。全件の記録は、診断JSONやMarkdownレポートで確認できます。"))?;
        }
    }
    writeln!(out, "</div></section>")?;
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
