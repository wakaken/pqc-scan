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
    if lang == ReportLanguage::En && finding.rule_id == "CERT_EXPIRY_2026_OR_BEFORE" {
        display.description = "Parsed certificate expiry is at or before 2026.".into();
        display.migration_hint = "This finding identifies certificates expiring before January 1, 2027 UTC. To determine whether a certificate was expired at scan time, check its expiry date and the CERT_EXPIRED findings.".into();
    }
    if lang == ReportLanguage::En && protocol_rule(&finding.rule_id) {
        display.migration_hint = "Prefer TLS 1.3; restrict to TLS 1.3 when all peers support it. Retain securely configured TLS 1.2 only for a documented interoperability need. Disable TLS 1.0 and 1.1.".into();
    }
    if matches!(
        finding.rule_id.as_str(),
        "TLS_VERSION_1_0" | "TLS_VERSION_1_1" | "TS_TLS_LEGACY_VERSION_LITERAL"
    ) {
        display.migration_hint = format!("{} {}", label(lang, "First distinguish constants, comments and test references from effective configuration.", "まず定数・コメント・テストの参照と有効な設定を区別してください。"), display.migration_hint);
    }
    if finding
        .evidence
        .metadata
        .get("reference_kind")
        .is_some_and(|s| s == "display_text")
    {
        display.description = label(
            lang,
            "An algorithm name appears in commentary, a declaration or recorded text.",
            "注釈・宣言・記録に暗号方式名が含まれています。",
        )
        .into();
        display.migration_hint = label(lang, "This text alone does not establish cryptographic use. Inspect the implementation and effective configuration separately.", "この文言だけでは暗号処理での利用は確認できません。実装と実際の設定を別途確認してください。").into();
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
// Normalize every Markdown line ending before indenting untrusted code. A
// continuation must not escape an indented block and become HTML or a link.
fn md_code(s: &str) -> String {
    s.replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "\n    ")
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
    let guide_path = path.with_extension("rules.html");
    let guide_url = crate::presentation::filename_url(&guide_path);
    let report_url = crate::presentation::filename_url(path);
    let en = html_view(result, ReportLanguage::En, context, &guide_url)?;
    let ja = html_view(result, ReportLanguage::Ja, context, &guide_url)?;
    let page = crate::presentation::page(&en, &ja, lang);
    pqc_scan_core::safe_output::write(path, page)?;
    let guide = crate::presentation::page(
        &rule_guide(result, ReportLanguage::En, &report_url)?,
        &rule_guide(result, ReportLanguage::Ja, &report_url)?,
        lang,
    );
    pqc_scan_core::safe_output::write(&guide_path, guide)?;
    Ok(())
}

const FILTER_ICON: &str = r#"<svg class="panel-icon" width="18" height="18" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="M3 5h18M6 12h12M9 19h6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#;

fn html_view(
    result: &ScanResult,
    lang: ReportLanguage,
    context: Option<&crate::inspection::HumanContext<'_>>,
    guide_url: &str,
) -> Result<String> {
    let raw = result;
    let view = crate::deduplicated::project(raw);
    let result = &view.result;
    let guide_groups = groups(raw);
    let groups = groups(result);
    let title = label(
        lang,
        "PQC Migration Scan Report",
        "耐量子暗号への移行調査レポート",
    );
    let mut out = String::new();
    writeln!(out, "<main class=\"container\" data-report-view lang=\"{}\" data-compact-report><a class=\"skip-link\" href=\"#findings\">{}</a>", label(lang,"en","ja"),label(lang,"Skip to findings","検出結果へ移動"))?;
    writeln!(out, "<header class=\"top\"><div><p class=\"eyebrow\">pqc-scan <span> / </span> {}</p><h1>{title}</h1></div><p class=\"meta\">{}<br><time data-local-time datetime=\"{}\" title=\"{}\">{}</time></p></header>", label(lang,"Static analysis","静的解析"),label(lang,"Generated","作成日時"), result.generated_at.to_rfc3339(),result.generated_at.to_rfc3339(),result.generated_at.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S %Z (UTC%:z)"))?;
    let inv = result.findings.iter().filter(|f| inventory(f)).count();
    writeln!(out, "<section class=\"summary\">")?;
    let mut metrics = vec![
        (
            label(lang, "Findings to review", "確認が必要な指摘"),
            result.findings.len() - inv,
        ),
        (label(lang, "Informational findings", "参考情報"), inv),
    ];
    if context.is_none() {
        metrics.push((
            label(lang, "Files inspected", "検査対象ファイル数"),
            result.summary.scanned_files,
        ));
        metrics.push((
            label(lang, "Files skipped", "対象外ファイル数"),
            result.summary.skipped_files,
        ));
    }
    metrics.push((
        label(lang, "Files with findings", "検出のあったファイル数"),
        result
            .findings
            .iter()
            .map(|f| &f.location.file)
            .collect::<BTreeSet<_>>()
            .len(),
    ));
    for (index, (name, count)) in metrics.into_iter().enumerate() {
        let unit = if index < 2 {
            format!(
                " <small class=\"metric-unit\">{}</small>",
                label(lang, "items", "件")
            )
        } else {
            String::new()
        };
        writeln!(out,"<div class=\"metric\"><div class=\"metric-label\">{name}</div><div class=\"metric-value\">{count}{unit}</div></div>")?;
    }
    writeln!(out, "</section>")?;
    render_severity_distribution(&mut out, result, lang)?;
    if let Some(context) = context {
        writeln!(out, "{}", crate::inspection::html(context, lang)?)?;
    }
    writeln!(
        out,
        "<div class=\"report-workspace\"><aside data-report-sidebar hidden>"
    )?;
    writeln!(out,"<details id=\"report-filters\" data-report-controls hidden open aria-label=\"{}\"><summary>{FILTER_ICON}{}</summary><div class=\"panel-body\"><label>{}<select data-mode><option value=\"candidate\">{}</option><option value=\"inventory\">{}</option><option value=\"all\">{}</option></select></label><label>{}<input data-search type=\"search\" placeholder=\"{}\"></label><label>{}<select data-severity><option value=\"\">{}</option>",label(lang,"Search conditions","検索条件"),label(lang,"Search conditions","検索条件"),label(lang,"View","表示対象"),label(lang,"Risk candidates","対応候補"),label(lang,"Inventory","参考情報"),label(lang,"All","すべて"),label(lang,"Search","検索"),label(lang,"Rule, title or file","ルール・説明・ファイル名"),label(lang,"Severity","重大度"),label(lang,"All severities","すべて"))?;
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
    writeln!(
        out,
        "</select></label><label>{}<select data-rule-filter><option value=\"\">{}</option>",
        label(lang, "Rule", "ルール"),
        label(lang, "All rules", "すべてのルール")
    )?;
    for group in &guide_groups {
        writeln!(
            out,
            "<option value=\"{}\">{}</option>",
            h(group.id),
            h(&format!(
                "{} — {}",
                group.id,
                short(&text(group.occurrences[0], lang).description, 72)
            ))
        )?;
    }
    writeln!(
        out,
        "</select></label><a class=\"rule-guide-link\" data-rule-help=\"all\" href=\"{guide_url}?lang={}\">{}</a><label>{}<select data-file-filter><option value=\"\">{}</option>",
        label(lang, "en", "ja"),
        label(lang, "Rule explanations", "ルールの説明"),
        label(lang, "File", "ファイル"),
        label(lang, "All files", "すべてのファイル")
    )?;
    for file in result
        .findings
        .iter()
        .map(|f| &f.location.file)
        .collect::<BTreeSet<_>>()
    {
        writeln!(out, "<option value=\"{}\">{}</option>", h(file), h(file))?;
    }
    writeln!(out, "</select></label><label class=\"test-results-option\"><input type=\"checkbox\" data-show-tests checked>{}</label><button type=\"button\" data-reset>{}</button></div></details>{}</aside><section class=\"results\" id=\"findings\" tabindex=\"-1\" aria-label=\"{}\"><div class=\"results-heading\"><h2>{}</h2><p data-report-status role=\"status\" aria-live=\"polite\" aria-atomic=\"true\"></p></div>", label(lang,"Show test code results","テストコードの結果を表示"),label(lang,"Clear all filters","条件をすべて解除"), crate::file_tree::render(result, lang), label(lang,"Findings","検出結果"), label(lang,"Findings","検出結果"))?;
    writeln!(out,"<noscript><p>{}</p></noscript><p class=\"empty-state\" data-empty hidden>{}</p><p class=\"print-note\">{}</p><nav data-group-pager hidden aria-label=\"{}\"><button type=\"button\" data-prev>{}</button><span data-page></span><button type=\"button\" data-next>{}</button></nav>",label(lang,"JavaScript is unavailable. All findings are shown below; native details still work. Interactive filters and pagination are unavailable.","JavaScriptを使わない場合は、全件を表示します。折りたたみは開けますが、検索・絞り込み・ページ切替は使えません。"),label(lang,"No findings match these filters. Clear the filters to see the full report.","条件に一致する検出はありません。条件を解除すると全体の結果を確認できます。"),label(lang,"Print view includes all findings, regardless of screen filters or pagination.","印刷には、画面の絞り込みやページに関係なく、すべての検出結果を含みます。"),label(lang,"Rule pages","ルール一覧のページ"),label(lang,"Previous","前へ"),label(lang,"Next","次へ"))?;
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
        let guide_index = guide_groups.iter().position(|r| r.id == group.id).unwrap();
        let total = group.occurrences.len();
        writeln!(out,"<details class=\"rule-card\" id=\"rule-{g}\" data-rule data-rule-id=\"{}\" data-page-size=\"20\"><summary><span class=\"rule-title\"><strong>{}</strong></span><span class=\"sev {max_s}\">{}</span><span class=\"count\">{} {}</span><span class=\"rule-meta\">{} {} · {} <span data-match-count>{total} / {total}</span></span></summary><div class=\"rule-body\"><p>{}</p>",h(group.id),h(&short(&representative.description,110)),severity(lang,max_s),group.occurrences.len(),label(lang,"hits","件"),label(lang,"Files","ファイル"),group.occurrences.iter().map(|f|&f.location.file).collect::<BTreeSet<_>>().len(),label(lang,"matches","表示対象"),h(&representative.migration_hint))?;
        writeln!(
            out,
            "<p><a class=\"rule-guide-link\" data-rule-help=\"{guide_index}\" href=\"{guide_url}?lang={}#rule-{guide_index}\">{}</a></p>",
            label(lang, "en", "ja"),
            label(lang, "About this rule", "このルールの説明")
        )?;
        writeln!(out,"<h4>{}</h4><nav data-occ-pager hidden><button type=\"button\" data-occ-prev>{}</button><span data-occ-status aria-live=\"polite\"></span><button type=\"button\" data-occ-next>{}</button></nav><div data-occurrence-list>",label(lang,"Locations and next steps","検出箇所と対応"),label(lang,"Previous locations","前の箇所"),label(lang,"Next locations","次の箇所"))?;
        for f in &group.occurrences {
            let index = finding_index;
            finding_index += 1;
            let display = text(f, lang);
            let originals = &view.originals[&f.finding_id];
            let rule_ids: Vec<_> = originals.iter().map(|o| o.rule_id.as_str()).collect();
            let search = format!(
                "{} {} {}",
                rule_ids.join(" "),
                originals
                    .iter()
                    .map(|o| text(o, lang).description)
                    .collect::<Vec<_>>()
                    .join(" "),
                f.location.file
            );
            writeln!(out,"<details class=\"occurrence\" id=\"finding-{index}\" data-finding-id=\"{}\" data-rules=\"{}\" data-inventory=\"{}\" data-severity=\"{}\" data-risk=\"{}\" data-search=\"{}\" data-file=\"{}\" data-test=\"{}\"><summary><code>{}</code><span class=\"occ-match\">{}</span><span class=\"next-step\">{}: {}</span></summary><p class=\"finding-description\">{}</p><p>{}: {}</p><p>{}: {}</p><p>{}: <code>{}</code></p>",h(&f.finding_id),h(&serde_json::to_string(&rule_ids)?),inventory(f),f.severity.as_str(),f.risk.as_str(),h(&search),h(&f.location.file),pqc_scan_core::file_selection::is_test_path(Path::new(&f.location.file)),h(&location(f,lang)),h(&short(&f.evidence.r#match,90)),label(lang,"Next","対応"),h(&short(&next(f,group,lang),150)),h(&display.description),label(lang,"Risk","判断の目安"),risk(lang,f.risk.as_str()),label(lang,"Check","確認すること"),h(&display.migration_hint),label(lang,"Evidence","検出した記述"),h(&f.evidence.r#match))?;
            writeln!(
                out,
                "<details class=\"matched-rules\" id=\"matched-rules-{index}\"><summary>{} ({})</summary>",
                label(lang, "Matched rules", "一致したルール"),
                originals.len()
            )?;
            for original in originals {
                let copy = text(original, lang);
                writeln!(
                    out,
                    "<section id=\"scan-match-{}\" tabindex=\"-1\"><h4>{}</h4><p><code>{}</code> · {}</p><p>{}</p></section>",
                    h(&original.finding_id),
                    h(&copy.description),
                    h(&original.rule_id),
                    severity(lang, original.severity.as_str()),
                    h(&copy.migration_hint)
                )?;
            }
            writeln!(out, "</details>")?;
            let overlaps = view.coincident_records(f);
            if overlaps > 1 {
                writeln!(out,"<p class=\"coincident-records\">{}: {overlaps} · {}</p>",label(lang,"Detection records at this location","この位置の検出記録"),label(lang,"Review the distinct rules and evidence. These records are not necessarily independent cryptographic uses.","異なるルールと根拠を確認してください。記録の件数は、独立した暗号利用の数とは限りません。"))?;
            }
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
            for (action_index, a) in f.recommended_actions.iter().enumerate() {
                let k = key(a);
                if group.shared_keys.contains(&k) || !seen.insert(k) {
                    continue;
                }
                render_html_action(
                    &mut out,
                    &action(a, &f.location.file, lang),
                    lang,
                    &format!("finding-{index}-action-{action_index}"),
                )?;
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
                "<details class=\"shared\" id=\"shared-group-{g}\"><summary>{} ({})</summary>",
                label(lang, "Shared checks and planning", "共通の確認・計画"),
                group.shared.len()
            )?;
            for (i, a) in group.shared.iter().enumerate() {
                writeln!(out, "<section id=\"shared-{g}-{i}\" tabindex=\"-1\">")?;
                render_html_action(
                    &mut out,
                    &action(a, &group.occurrences[0].location.file, lang),
                    lang,
                    &format!("shared-{g}-{i}-references"),
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
    writeln!(out, "</section></div><dialog class=\"rule-dialog\" data-rule-dialog aria-label=\"{}\"><header><h2>{}</h2><button type=\"button\" data-close-rule-dialog>{}</button></header><div data-rule-dialog-body></div></dialog><template data-rule-explanations>{}</template></main>", label(lang,"Rule explanations","ルールの説明"),label(lang,"Rule explanations","ルールの説明"),label(lang,"Close","閉じる"),rule_guide(raw,lang,"")?)?;
    Ok(out)
}

fn rule_guide(result: &ScanResult, lang: ReportLanguage, report_url: &str) -> Result<String> {
    let mut out = String::new();
    writeln!(out, "<main class=\"container rule-guide\" data-report-view lang=\"{}\"><header class=\"top\"><div><p class=\"eyebrow\">pqc-scan / {}</p><h1>{}</h1></div><a href=\"{report_url}?lang={}\">{}</a></header><p>{}</p>",label(lang,"en","ja"),label(lang,"Reference","解説"),label(lang,"Rule explanations","ルールの説明"),label(lang,"en","ja"),label(lang,"Back to report","レポートに戻る"),label(lang,"Rules found in this report. Findings identify references to investigate, not confirmed vulnerabilities or runtime use.","このレポートで検出されたルールを説明します。所見は調査の手掛かりであり、脆弱性や稼働時の利用を確定するものではありません。"))?;
    writeln!(out,"<section class=\"rule-guide-intro\"><h2>{}</h2><p>{}</p><p>{}</p></section>",label(lang,"How to read the labels","表示の読み方"),label(lang,"Severity prioritizes investigation; it is not a count of confirmed vulnerabilities. Risk describes the matched cryptographic signal. Informational PQC references do not establish migration completion.","重大度は調査の優先順位です。確認済みの脆弱性の数ではありません。リスクは一致した暗号の手掛かりを分類します。PQC方式の参照も移行完了を保証しません。"),label(lang,"Review the file role (product, test, fixture or documentation), source context and actual deployment before applying the suggested action.","推奨対応を適用する前に、製品・テスト・fixture・文書のどこにある記述か、周辺コードと実際の稼働環境を確認してください。"))?;
    let projection = crate::deduplicated::project(result);
    let projected_groups = groups(&projection.result);
    let groups = groups(result);
    if groups.is_empty() {
        writeln!(out,"<p>{}</p>",label(lang,"No rules produced findings in this scan. This does not establish absence of cryptography. Scope and inspection limits remain in the report.","このスキャンでは、所見を出したルールはありません。暗号利用の不存在を示すものではありません。検査範囲と制約はレポートで確認してください。"))?;
    }
    for (g, group) in groups.iter().enumerate() {
        let report_group = projected_groups
            .iter()
            .position(|p| {
                p.occurrences.iter().any(|f| {
                    projection.originals[&f.finding_id]
                        .iter()
                        .any(|o| o.rule_id == group.id)
                })
            })
            .unwrap();
        let f = text(group.occurrences[0], lang);
        let severities: BTreeSet<_> = group
            .occurrences
            .iter()
            .map(|f| severity(lang, f.severity.as_str()))
            .collect();
        let risks: BTreeSet<_> = group
            .occurrences
            .iter()
            .map(|f| risk(lang, f.risk.as_str()))
            .collect();
        writeln!(out,"<section class=\"rule-explanation\" id=\"rule-{g}\" tabindex=\"-1\"><h2>{}</h2><code>{}</code><p>{}</p><dl><dt>{}</dt><dd>{}</dd><dt>{}</dt><dd>{}</dd><dt>{}</dt><dd>{}</dd><dt>{}</dt><dd>{}</dd></dl><h3>{}</h3><p>{}</p><a href=\"{report_url}?lang={}#rule-{report_group}\">{}</a></section>",h(&f.description),h(group.id),h(&f.migration_hint),label(lang,"Category","分類"),h(&f.category),label(lang,"Observed severities","今回の重大度"),h(&severities.into_iter().collect::<Vec<_>>().join(", ")),label(lang,"Observed risks","今回のリスク"),h(&risks.into_iter().collect::<Vec<_>>().join(", ")),label(lang,"Occurrences","検出箇所"),group.occurrences.len(),label(lang,"Example follow-up; confirm the advice at each location","確認事項の例（各検出箇所の案内も確認してください）"),h(&next(group.occurrences[0], group,lang)),label(lang,"en","ja"),label(lang,"View locations and evidence","検出箇所と根拠を見る"))?;
    }
    writeln!(out, "</main>")?;
    Ok(out)
}

fn render_severity_distribution(
    out: &mut String,
    result: &ScanResult,
    lang: ReportLanguage,
) -> Result<()> {
    let mut counts = BTreeMap::<&str, usize>::new();
    for finding in &result.findings {
        *counts.entry(finding.severity.as_str()).or_default() += 1;
    }
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by_key(|(s, _)| std::cmp::Reverse(rank(s)));
    writeln!(out, "<figure class=\"severity-distribution\"><figcaption>{}</figcaption><div class=\"severity-bar\" aria-hidden=\"true\">", label(lang,"Severity distribution · all findings","重大度の分布 · 全検出"))?;
    for (s, count) in &counts {
        let width = *count as f64 / result.findings.len() as f64 * 100.0;
        writeln!(
            out,
            "<span class=\"severity-segment {}\" style=\"width:{width:.4}%\"></span>",
            h(s)
        )?;
    }
    writeln!(out, "</div><ul class=\"severity-legend\">")?;
    for (s, count) in counts {
        writeln!(out, "<li><span class=\"severity-dot {}\" aria-hidden=\"true\"></span>{} <strong>{count}</strong></li>", h(s), h(severity(lang,s)))?;
    }
    if result.findings.is_empty() {
        writeln!(out, "<li>{}</li>", label(lang, "No findings", "検出なし"))?;
    }
    writeln!(out, "</ul></figure>")?;
    Ok(())
}

fn render_html_action(
    out: &mut String,
    a: &RecommendedAction,
    lang: ReportLanguage,
    disclosure_id: &str,
) -> Result<()> {
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
            "<details id=\"{}\"><summary>{}</summary>",
            h(disclosure_id),
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
    let view = crate::deduplicated::project(result);
    let result = &view.result;
    let groups = groups(result);
    let mut out = String::new();
    let inv = result.findings.iter().filter(|f| inventory(f)).count();
    writeln!(
        out,
        "# {}\n\n{}: {}\n\n{}: {} / {}: {} / {}: {}\n",
        label(
            lang,
            "PQC Migration Scan Report",
            "耐量子暗号への移行調査レポート"
        ),
        label(lang, "Generated", "作成日時"),
        result
            .generated_at
            .with_timezone(&chrono::Local)
            .format("%Y-%m-%d %H:%M:%S %Z (UTC%:z)"),
        label(lang, "Findings to review (items)", "確認が必要な指摘（件）"),
        result.findings.len() - inv,
        label(lang, "Informational findings (items)", "参考情報（件）"),
        inv,
        label(lang, "Files inspected", "検査対象ファイル数"),
        result.summary.scanned_files
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
            let overlaps = view.coincident_records(f);
            if overlaps > 1 {
                writeln!(out,"\n{}: {overlaps} · {}\n",label(lang,"Detection records at this location","この位置の検出記録"),label(lang,"Review the distinct rules and evidence. These records are not necessarily independent cryptographic uses.","異なるルールと根拠を確認してください。記録の件数は、独立した暗号利用の数とは限りません。"))?;
            }
            writeln!(
                out,
                "\n### {} — {}\n",
                m(&location(f, lang)),
                label(lang, "Matched rules", "一致したルール")
            )?;
            for original in &view.originals[&f.finding_id] {
                let copy = text(original, lang);
                writeln!(
                    out,
                    "- `{}` ({}): {} — {}",
                    m(&original.rule_id),
                    severity(lang, original.severity.as_str()),
                    m(&copy.description),
                    m(&copy.migration_hint)
                )?;
            }
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
                        md_code(&line.text)
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
    pqc_scan_core::safe_output::write(path, out)?;
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
            md_code(&e.before),
            label(lang, "After", "変更後"),
            md_code(&e.after)
        )?;
    }
    Ok(())
}
