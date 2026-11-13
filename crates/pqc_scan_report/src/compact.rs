use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    path::Path,
};

use anyhow::Result;
use pqc_scan_core::{Finding, RecommendedAction, ScanResult};

use crate::{japanese, ReportLanguage};

use crate::grouping::{group_findings as groups, key, rank, RuleGroup};

fn inventory(finding: &Finding) -> bool {
    finding.severity.as_str() == "info" || finding.risk.as_str() == "quantum-safe"
}
fn label<'a>(lang: ReportLanguage, en: &'a str, ja: &'a str) -> &'a str {
    if lang == ReportLanguage::Ja {
        ja
    } else {
        en
    }
}
fn severity(lang: ReportLanguage, s: &str) -> &str {
    if lang == ReportLanguage::Ja {
        japanese::severity(s)
    } else {
        s
    }
}
fn risk(lang: ReportLanguage, s: &str) -> &str {
    if lang == ReportLanguage::Ja {
        japanese::risk(s)
    } else {
        s
    }
}
fn text(finding: &Finding, lang: ReportLanguage) -> Finding {
    let mut display = if lang == ReportLanguage::Ja {
        japanese::localize_finding(finding)
    } else {
        finding.clone()
    };
    if lang == ReportLanguage::En && protocol_rule(&finding.rule_id) {
        display.migration_hint = "Prefer TLS 1.3; restrict to TLS 1.3 when all peers support it. Retain securely configured TLS 1.2 only for a documented interoperability need. Disable TLS 1.0 and 1.1.".into();
    }
    if matches!(
        finding.rule_id.as_str(),
        "TLS_VERSION_1_0" | "TLS_VERSION_1_1" | "TS_TLS_LEGACY_VERSION_LITERAL"
    ) {
        display.migration_hint = format!("{} {}", label(lang, "First distinguish constants, comments and test references from effective configuration.", "まず定数・コメント・テストの参照と有効な設定を区別してください。"), display.migration_hint);
    }
    display
}

fn protocol_rule(id: &str) -> bool {
    matches!(
        id,
        "TLS_VERSION_1_0"
            | "TLS_VERSION_1_1"
            | "TS_TLS_LEGACY_VERSION_LITERAL"
            | "NODE_TLS_MIN_VERSION"
            | "K8S_INGRESS_SSL_PROTOCOLS_LEGACY"
            | "K8S_GATEWAYAPI_TLSV1"
            | "NGINX_SSL_PROTOCOLS_LEGACY"
            | "NGINX_PROXY_SSL_PROTOCOLS_LEGACY"
            | "HTTPD_SSLPROTOCOL_LEGACY"
            | "ENVOY_TLS_MIN_VERSION_LEGACY"
            | "ENVOY_TLS_MAX_VERSION_LEGACY"
            | "ISTIO_DESTINATIONRULE_TLSV1"
            | "ISTIO_GATEWAY_TLSV1"
            | "HAPROXY_BIND_TLSV1"
            | "TRAEFIK_TLS_OPTIONS_LEGACY"
    )
}
fn action(action: &RecommendedAction, file: &str, lang: ReportLanguage) -> RecommendedAction {
    let mut action = action.clone();
    if lang == ReportLanguage::Ja {
        japanese::localize_action(&mut action, file);
    }
    action
}
fn short(value: &str, max: usize) -> String {
    let mut chars = value.chars();
    let mut s: String = chars.by_ref().take(max).collect();
    if chars.next().is_some() {
        s.push_str("...");
    }
    s.replace(['\n', '\r'], " ")
}
fn h(s: &str) -> String {
    japanese::html(s)
}
fn m(s: &str) -> String {
    japanese::md(s)
}
fn location(f: &Finding, lang: ReportLanguage) -> String {
    if f.location.line == 0 {
        format!(
            "{} ({})",
            f.location.file,
            label(lang, "line unavailable", "行番号不明")
        )
    } else {
        format!(
            "{}:{}:{}",
            f.location.file, f.location.line, f.location.column
        )
    }
}
fn next(f: &Finding, group: &RuleGroup<'_>, lang: ReportLanguage) -> String {
    let display = text(f, lang);
    if protocol_rule(&f.rule_id) || f.rule_id.starts_with("PQC_") {
        return display.migration_hint;
    }
    f.recommended_actions
        .iter()
        .filter(|a| {
            !group.shared_keys.contains(&key(a)) && !a.action_id.ends_with("readiness-note")
        })
        .min_by_key(|a| match a.priority.as_str() {
            "immediate" => 0,
            "near-term" => 1,
            "planned" => 2,
            _ => 3,
        })
        .map(|a| action(a, &f.location.file, lang).title)
        .unwrap_or(display.migration_hint)
}

pub(crate) fn html_report(result: &ScanResult, path: &Path, lang: ReportLanguage) -> Result<()> {
    html_report_with_context(result, path, lang, None)
}

pub(crate) fn html_report_with_context(
    result: &ScanResult,
    path: &Path,
    lang: ReportLanguage,
    context: Option<&crate::inspection::HumanContext<'_>>,
) -> Result<()> {
    let groups = groups(result);
    let title = label(
        lang,
        "PQC Migration Scan Report",
        "耐量子暗号への移行調査レポート",
    );
    let mut out = String::new();
    writeln!(out,"<!doctype html><html lang=\"{}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><style>{}\n{}</style></head><body><main class=\"container\" data-compact-report>",label(lang,"en","ja"),crate::html_report::css(),include_str!("compact.css"))?;
    writeln!(
        out,
        "<header class=\"top\"><h1>{title}</h1><p class=\"meta\">{}: {}</p><p>{}</p></header>",
        label(lang, "Generated", "作成日時"),
        result.generated_at,
        label(
            lang,
            "Cryptographic references found in source code and configuration.",
            "コードや設定の手掛かりをまとめた調査結果です。"
        )
    )?;
    let inv = result.findings.iter().filter(|f| inventory(f)).count();
    writeln!(out, "<section class=\"summary\">")?;
    let mut metrics = vec![
        (
            label(lang, "Risk candidates", "対応を確認する候補"),
            result.findings.len() - inv,
        ),
        (label(lang, "Inventory findings", "参考情報"), inv),
    ];
    if context.is_none() {
        metrics.push((
            label(lang, "Scanned Files", "調査したファイル"),
            result.summary.scanned_files,
        ));
        metrics.push((
            label(lang, "Skipped Files", "対象外のファイル"),
            result.summary.skipped_files,
        ));
    }
    metrics.push((
        label(lang, "Affected files", "該当ファイル"),
        result
            .findings
            .iter()
            .map(|f| &f.location.file)
            .collect::<BTreeSet<_>>()
            .len(),
    ));
    for (name, count) in metrics {
        writeln!(out,"<div class=\"metric\"><div class=\"metric-label\">{name}</div><div class=\"metric-value\">{count}</div></div>")?;
    }
    writeln!(out, "</section>")?;
    writeln!(
        out,
        "<p>{}</p><p>",
        label(
            lang,
            "Severity counts (all findings):",
            "重大度の内訳（全検出）："
        )
    )?;
    for (s, count) in &result.summary.by_severity {
        write!(
            out,
            "<span class=\"sev {}\">{}: {count}</span> ",
            h(s),
            h(severity(lang, s))
        )?;
    }
    writeln!(
        out,
        "</p>{}",
        context
            .map(|c| crate::inspection::html(c, lang))
            .transpose()?
            .unwrap_or_default()
    )?;
    writeln!(out,"<section data-report-controls hidden aria-label=\"{}\"><label>{}<select data-mode><option value=\"candidate\">{}</option><option value=\"inventory\">{}</option><option value=\"all\">{}</option></select></label><label>{}<input data-search type=\"search\" placeholder=\"{}\"></label><label>{}<select data-severity><option value=\"\">{}</option>",label(lang,"Report filters","表示の絞り込み"),label(lang,"View","表示対象"),label(lang,"Risk candidates","対応候補"),label(lang,"Inventory","参考情報"),label(lang,"All","すべて"),label(lang,"Search","検索"),label(lang,"Rule, title or file","ルール・説明・ファイル名"),label(lang,"Severity","重大度"),label(lang,"All severities","すべて"))?;
    for s in ["critical", "high", "medium", "low", "info"] {
        writeln!(out, "<option value=\"{s}\">{}</option>", severity(lang, s))?;
    }
    writeln!(
        out,
        "</select></label><label>{}<select data-risk><option value=\"\">{}</option>",
        label(lang, "Risk", "リスク"),
        label(lang, "All risks", "すべて")
    )?;
    for r in [
        "quantum-vulnerable",
        "quantum-uncertain",
        "quantum-safe",
        "non-quantum-risk",
    ] {
        writeln!(out, "<option value=\"{r}\">{}</option>", risk(lang, r))?;
    }
    writeln!(out,"</select></label><button type=\"button\" data-reset>{}</button></section><noscript><p>{}</p></noscript><p data-report-status role=\"status\" aria-live=\"polite\"></p><p data-empty hidden>{}</p><nav data-group-pager hidden aria-label=\"{}\"><button type=\"button\" data-prev>{}</button><span data-page></span><button type=\"button\" data-next>{}</button></nav>",label(lang,"Reset","条件を解除"),label(lang,"JavaScript is unavailable. All findings are shown below; native details still work. Interactive filters and pagination are unavailable.","JavaScriptを使わない場合は、全件を表示します。折りたたみは開けますが、検索・絞り込み・ページ切替は使えません。"),label(lang,"No matches. Change the view or filters; this does not mean the entire scan found nothing.","表示条件に一致する検出はありません。表示対象や絞り込み条件を変更してください。調査全体で検出がなかったという意味ではありません。"),label(lang,"Rule pages","ルール一覧のページ"),label(lang,"Previous","前へ"),label(lang,"Next","次へ"))?;
    let mut finding_index = 0;
    let mut related: BTreeMap<(&str, usize, usize), BTreeSet<usize>> = BTreeMap::new();
    for (g, group) in groups.iter().enumerate() {
        for f in &group.occurrences {
            related
                .entry((&f.location.file, f.location.line, f.location.column))
                .or_default()
                .insert(g);
        }
    }
    for (g, group) in groups.iter().enumerate() {
        let representative = text(group.occurrences[0], lang);
        let max_s = group
            .occurrences
            .iter()
            .max_by_key(|f| rank(f.severity.as_str()))
            .unwrap()
            .severity
            .as_str();
        writeln!(out,"<details class=\"rule-card\" id=\"rule-{g}\" data-rule data-page-size=\"20\"><summary><span><strong>{}</strong> <code>{}</code></span><span class=\"sev {max_s}\">{}</span><span class=\"count\">{} {}</span><span>{} {} / {} <span data-match-count></span></span></summary><div class=\"rule-body\"><p>{}</p>",h(&short(&representative.description,110)),h(group.id),severity(lang,max_s),group.occurrences.len(),label(lang,"hits","件"),label(lang,"Files","ファイル"),group.occurrences.iter().map(|f|&f.location.file).collect::<BTreeSet<_>>().len(),label(lang,"matches","表示対象"),h(&representative.migration_hint))?;
        writeln!(out,"<h4>{}</h4><nav data-occ-pager hidden><button type=\"button\" data-occ-prev>{}</button><span data-occ-status aria-live=\"polite\"></span><button type=\"button\" data-occ-next>{}</button></nav><div data-occurrence-list>",label(lang,"Locations and next steps","検出箇所と対応"),label(lang,"Previous locations","前の箇所"),label(lang,"Next locations","次の箇所"))?;
        for f in &group.occurrences {
            let index = finding_index;
            finding_index += 1;
            let display = text(f, lang);
            let search = format!(
                "{} {} {}",
                group.id, representative.description, f.location.file
            );
            writeln!(out,"<details class=\"occurrence\" id=\"finding-{index}\" data-finding-id=\"{}\" data-inventory=\"{}\" data-severity=\"{}\" data-risk=\"{}\" data-search=\"{}\"><summary><code>{}</code><span class=\"occ-match\">{}</span><span class=\"next-step\">{}: {}</span></summary><p>{}: {}</p><p>{}: {}</p><p>{}: <code>{}</code></p>",h(&f.finding_id),inventory(f),f.severity.as_str(),f.risk.as_str(),h(&search),h(&location(f,lang)),h(&short(&f.evidence.r#match,90)),label(lang,"Next","対応"),h(&short(&next(f,group,lang),150)),label(lang,"Risk","判断の目安"),risk(lang,f.risk.as_str()),label(lang,"Check","確認すること"),h(&display.migration_hint),label(lang,"Evidence","検出した記述"),h(&f.evidence.r#match))?;
            if let Some(snippet) = &f.source_snippet {
                writeln!(
                    out,
                    "<div class=\"snippet-wrap\"><div class=\"code-lines\">"
                )?;
                for line in &snippet.lines {
                    writeln!(out,"<span class=\"{}\"><span class=\"ln\">{}</span><span class=\"tx\">{}</span></span>",if line.highlighted{"line hl"}else{"line"},line.line,h(&line.text))?;
                }
                writeln!(out, "</div></div>")?;
            } else {
                writeln!(
                    out,
                    "<p>{}</p>",
                    label(
                        lang,
                        "Source context unavailable; inspect the original file.",
                        "周辺のコードは取得できませんでした。元のファイルを確認してください。"
                    )
                )?;
            }
            writeln!(
                out,
                "<h4>{}</h4>",
                label(lang, "Recommended Actions", "この箇所の対応")
            )?;
            let mut seen = BTreeSet::new();
            for a in &f.recommended_actions {
                let k = key(a);
                if group.shared_keys.contains(&k) || !seen.insert(k) {
                    continue;
                }
                render_html_action(&mut out, &action(a, &f.location.file, lang), lang)?;
            }
            if f.recommended_actions.is_empty() {
                writeln!(out, "<p>{}</p>", h(&display.migration_hint))?;
            }
            if !group.shared.is_empty() {
                writeln!(
                    out,
                    "<p>{}</p><ul>",
                    label(
                        lang,
                        "Shared checks and planning (not verified facts about this line):",
                        "共通の確認・計画（この行で確認済みの事実ではありません）："
                    )
                )?;
            }
            for (i, a) in group.shared.iter().enumerate() {
                writeln!(
                    out,
                    "<li><a href=\"#shared-{g}-{i}\">{}</a></li>",
                    h(&action(a, &f.location.file, lang).title)
                )?;
            }
            if !group.shared.is_empty() {
                writeln!(out, "</ul>")?;
            }
            let other = &related[&(f.location.file.as_str(), f.location.line, f.location.column)];
            if other.len() > 1 {
                writeln!(
                    out,
                    "<p>{}: ",
                    label(
                        lang,
                        "Related rules at this location",
                        "同じ箇所の関連ルール"
                    )
                )?;
                for r in other.iter().filter(|r| **r != g) {
                    write!(out, "<a href=\"#rule-{r}\">{}</a> ", h(groups[*r].id))?;
                }
                writeln!(out, "</p>")?;
            }
            writeln!(out, "</details>")?;
        }
        writeln!(out, "</div>")?;
        if !group.shared.is_empty() {
            writeln!(
                out,
                "<details class=\"shared\"><summary>{} ({})</summary>",
                label(lang, "Shared checks and planning", "共通の確認・計画"),
                group.shared.len()
            )?;
            for (i, a) in group.shared.iter().enumerate() {
                writeln!(out, "<section id=\"shared-{g}-{i}\" tabindex=\"-1\">")?;
                render_html_action(
                    &mut out,
                    &action(a, &group.occurrences[0].location.file, lang),
                    lang,
                )?;
                writeln!(out, "</section>")?;
            }
            writeln!(out, "</details>")?;
        }
        writeln!(out, "</div></details>")?;
    }
    if result.findings.is_empty() {
        writeln!(
            out,
            "<p>{}</p>",
            label(
                lang,
                "No findings in the scanned files.",
                "該当する検出はありませんでした。"
            )
        )?;
    }
    writeln!(
        out,
        "</main><script>{}</script></body></html>",
        include_str!("compact.js")
    )?;
    std::fs::write(path, out)?;
    Ok(())
}

fn render_html_action(out: &mut String, a: &RecommendedAction, lang: ReportLanguage) -> Result<()> {
    writeln!(
        out,
        "<section class=\"action\"><h4>{} <span class=\"badge\">{}</span></h4><p>{}</p><ol>",
        h(&a.title),
        h(&a.priority),
        h(&a.rationale)
    )?;
    for s in &a.steps {
        writeln!(out, "<li>{}</li>", h(s))?;
    }
    writeln!(out, "</ol>")?;
    if !a.references.is_empty() || !a.code_examples.is_empty() {
        writeln!(
            out,
            "<details><summary>{}</summary>",
            label(lang, "References and examples", "参考資料・変更例")
        )?;
        for url in &a.references {
            if url.starts_with("https://") || url.starts_with("http://") {
                writeln!(
                    out,
                    "<p><a href=\"{}\" rel=\"noopener noreferrer\">{}</a></p>",
                    h(url),
                    h(url)
                )?;
            } else {
                writeln!(out, "<p>{}</p>", h(url))?;
            }
        }
        for example in &a.code_examples {
            writeln!(
                out,
                "<p>{} ({})</p><pre><code>{}</code></pre><p>{}</p><pre><code>{}</code></pre>",
                label(lang, "Before", "変更前"),
                h(&example.language),
                h(&example.before),
                label(lang, "After", "変更後"),
                h(&example.after)
            )?;
        }
        writeln!(out, "</details>")?;
    }
    writeln!(out, "</section>")?;
    Ok(())
}

pub(crate) fn markdown_report(
    result: &ScanResult,
    path: &Path,
    lang: ReportLanguage,
) -> Result<()> {
    markdown_report_with_context(result, path, lang, None)
}

pub(crate) fn markdown_report_with_context(
    result: &ScanResult,
    path: &Path,
    lang: ReportLanguage,
    context: Option<&crate::inspection::HumanContext<'_>>,
) -> Result<()> {
    let groups = groups(result);
    let mut out = String::new();
    let inv = result.findings.iter().filter(|f| inventory(f)).count();
    writeln!(
        out,
        "# {}\n\n{}: {}\n\n{}: {} / {}: {} / {}: {}\n\n{}\n",
        label(
            lang,
            "PQC Migration Scan Report",
            "耐量子暗号への移行調査レポート"
        ),
        label(lang, "Generated", "作成日時"),
        result.generated_at,
        label(lang, "Risk candidates", "対応を確認する候補"),
        result.findings.len() - inv,
        label(lang, "Inventory findings", "参考情報"),
        inv,
        label(lang, "Scanned files", "調査したファイル"),
        result.summary.scanned_files,
        label(
            lang,
            "Cryptographic references found in source code and configuration.",
            "コードや設定の手掛かりをまとめた調査結果です。"
        )
    )?;
    writeln!(
        out,
        "|{}|{}|\n|---|---:|",
        label(lang, "Severity", "重大度"),
        label(lang, "Count (all findings)", "件数（全検出）")
    )?;
    for (s, count) in &result.summary.by_severity {
        writeln!(out, "|{}|{}|", severity(lang, s), count)?;
    }
    for (is_inventory, title) in [
        (false, label(lang, "Risk candidate index", "対応候補の索引")),
        (
            true,
            label(lang, "Informational inventory index", "参考情報の索引"),
        ),
    ] {
        writeln!(
            out,
            "\n## {title}\n\n|{}|{}|{}|{}|{}|\n|---|---|---|---:|---:|",
            label(lang, "Rule / description", "ルール・説明"),
            label(lang, "Severity", "重大度"),
            label(lang, "Risk", "判断の目安"),
            label(lang, "Hits", "件数"),
            label(lang, "Files", "ファイル数")
        )?;
        for (g, group) in groups.iter().enumerate() {
            let count = group
                .occurrences
                .iter()
                .filter(|f| inventory(f) == is_inventory)
                .count();
            if count == 0 {
                continue;
            }
            let f = text(group.occurrences[0], lang);
            let s = group
                .occurrences
                .iter()
                .filter(|f| inventory(f) == is_inventory)
                .max_by_key(|f| rank(f.severity.as_str()))
                .unwrap()
                .severity
                .as_str();
            writeln!(
                out,
                "|[{}](#rule-{g}) {}|{}|{}|{}|{}|",
                m(group.id),
                m(&short(&f.description, 100)),
                severity(lang, s),
                group
                    .occurrences
                    .iter()
                    .filter(|f| inventory(f) == is_inventory)
                    .map(|f| risk(lang, f.risk.as_str()))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", "),
                count,
                group
                    .occurrences
                    .iter()
                    .filter(|f| inventory(f) == is_inventory)
                    .map(|f| &f.location.file)
                    .collect::<BTreeSet<_>>()
                    .len()
            )?;
        }
    }
    // The catalog deduplicates payloads across the document, not just rule groups.
    let mut catalog: Vec<(&RecommendedAction, &str)> = Vec::new();
    let mut action_ids = BTreeMap::new();
    for group in &groups {
        for f in &group.occurrences {
            for a in &f.recommended_actions {
                let k = key(a);
                if let std::collections::btree_map::Entry::Vacant(entry) = action_ids.entry(k) {
                    entry.insert(catalog.len());
                    catalog.push((a, &f.location.file));
                }
            }
        }
    }
    let mut index = 0;
    for (g, group) in groups.iter().enumerate() {
        let representative = text(group.occurrences[0], lang);
        writeln!(
            out,
            "\n<a id=\"rule-{g}\"></a>\n\n## {}\n\n{}\n\n{}: {}\n\n|{}|{}|{}|\n|---|---|---|",
            m(group.id),
            m(&representative.description),
            label(lang, "Check", "確認すること"),
            m(&representative.migration_hint),
            label(lang, "Location", "検出箇所"),
            label(lang, "Evidence", "検出した記述"),
            label(lang, "Next action / details", "対応・詳細")
        )?;
        let start = index;
        for f in &group.occurrences {
            write!(
                out,
                "|<a id=\"finding-{index}\"></a>[{}](#finding-{index})|{}|{}",
                m(&location(f, lang)),
                m(&short(&f.evidence.r#match, 90)),
                m(&next(f, group, lang))
            )?;
            let mut seen = BTreeSet::new();
            for a in &f.recommended_actions {
                let k = key(a);
                if group.shared_keys.contains(&k) || !seen.insert(k.clone()) {
                    continue;
                }
                write!(
                    out,
                    " / [{}](#action-{})",
                    m(&action(a, &f.location.file, lang).title),
                    action_ids[&k]
                )?;
            }
            if f.source_snippet.is_some() {
                write!(
                    out,
                    " / [{}](#source-{index})",
                    label(lang, "Source", "周辺コード")
                )?;
            } else {
                write!(
                    out,
                    " / {}",
                    label(lang, "Source unavailable", "周辺コードなし")
                )?;
            }
            writeln!(out, "|")?;
            index += 1;
        }
        if !group.shared.is_empty() {
            writeln!(
                out,
                "\n{}:\n",
                label(
                    lang,
                    "Shared checks for these locations (not verified facts about each line)",
                    "このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）"
                )
            )?;
            for a in &group.shared {
                writeln!(
                    out,
                    "- [{}](#action-{})",
                    m(&action(a, &group.occurrences[0].location.file, lang).title),
                    action_ids[&key(a)]
                )?;
            }
        }
        for (offset, f) in group.occurrences.iter().enumerate() {
            if let Some(snippet) = &f.source_snippet {
                writeln!(
                    out,
                    "\n<a id=\"source-{}\"></a>\n\n### {}\n",
                    start + offset,
                    m(&location(f, lang))
                )?;
                for line in &snippet.lines {
                    writeln!(
                        out,
                        "    {} {} {}",
                        if line.highlighted { ">" } else { " " },
                        line.line,
                        line.text
                    )?;
                }
            }
            let display = text(f, lang);
            if display.migration_hint != representative.migration_hint {
                writeln!(
                    out,
                    "\n{}: {}\n",
                    m(&location(f, lang)),
                    m(&display.migration_hint)
                )?;
            }
        }
    }
    writeln!(out,"\n## {}\n\n{}\n",label(lang,"Action details","対応案の詳細"),label(lang,"Each location links only to its applicable action. Identical payloads are printed once.","各検出箇所から、その箇所に該当する対応案だけを参照します。同じ内容は一度だけ掲載します。"))?;
    for (i, (a, file)) in catalog.iter().enumerate() {
        writeln!(out, "<a id=\"action-{i}\"></a>\n")?;
        render_md_action(&mut out, &action(a, file, lang), lang)?;
    }
    writeln!(
        out,
        "\n## {}\n\n- [CBOM](cbom.json): {}\n- [Dependency SBOM](dependency-sbom.json): {}\n",
        label(lang, "Inventory exports", "棚卸しデータ"),
        result.cbom.len(),
        result.dependency_sbom.len()
    )?;
    if let Some(context) = context {
        out.push_str(&crate::inspection::markdown(context, lang)?);
    }
    std::fs::write(path, out)?;
    Ok(())
}

fn render_md_action(out: &mut String, a: &RecommendedAction, lang: ReportLanguage) -> Result<()> {
    writeln!(
        out,
        "\n#### {} ({})\n\n{}\n",
        m(&a.title),
        m(&a.priority),
        m(&a.rationale)
    )?;
    for (i, step) in a.steps.iter().enumerate() {
        writeln!(out, "{}. {}", i + 1, m(step))?;
    }
    if !a.references.is_empty() {
        writeln!(out, "\n{}:\n", label(lang, "References", "参考資料"))?;
        for url in &a.references {
            writeln!(out, "- {}", m(url))?;
        }
    }
    for e in &a.code_examples {
        writeln!(
            out,
            "\n{} ({})\n\n    {}\n\n{}\n\n    {}\n",
            label(lang, "Before", "変更前"),
            m(&e.language),
            e.before.replace('\n', "\n    "),
            label(lang, "After", "変更後"),
            e.after.replace('\n', "\n    ")
        )?;
    }
    Ok(())
}
