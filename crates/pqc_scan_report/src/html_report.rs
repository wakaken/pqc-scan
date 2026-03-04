use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};

use crate::grouping::{group_findings, summarize_lines, GroupedFinding};
use pqc_scan_core::Finding;
use pqc_scan_core::ScanResult;

pub fn write_html_report(result: &ScanResult, path: &Path) -> Result<()> {
    let mut out = String::new();
    let groups = group_findings(&result.findings);

    writeln!(&mut out, "<!doctype html>")?;
    writeln!(&mut out, "<html lang=\"en\">")?;
    writeln!(&mut out, "<head>")?;
    writeln!(&mut out, "  <meta charset=\"utf-8\" />")?;
    writeln!(
        &mut out,
        "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />"
    )?;
    writeln!(&mut out, "  <title>PQC Migration Scan Report</title>")?;
    writeln!(&mut out, "  <style>{}</style>", css())?;
    writeln!(&mut out, "</head>")?;
    writeln!(&mut out, "<body>")?;

    writeln!(&mut out, "<main class=\"container\">")?;
    writeln!(&mut out, "  <header class=\"top\">")?;
    writeln!(&mut out, "    <h1>PQC Migration Scan Report</h1>")?;
    writeln!(
        &mut out,
        "    <p class=\"meta\">Generated at {}</p>",
        escape_html(&result.generated_at.to_string())
    )?;
    writeln!(&mut out, "  </header>")?;

    writeln!(&mut out, "  <section class=\"summary\">")?;
    write_metric(
        &mut out,
        "Total Findings",
        &result.summary.total_findings.to_string(),
    )?;
    write_metric(&mut out, "Grouped Findings", &groups.len().to_string())?;
    write_metric(
        &mut out,
        "Scanned Files",
        &result.summary.scanned_files.to_string(),
    )?;
    write_metric(
        &mut out,
        "Skipped Files",
        &result.summary.skipped_files.to_string(),
    )?;
    write_metric(
        &mut out,
        "Dependency SBOM",
        &result.dependency_sbom.len().to_string(),
    )?;
    writeln!(&mut out, "  </section>")?;

    writeln!(&mut out, "  <section>")?;
    writeln!(&mut out, "    <h2>Findings (Grouped by Rule)</h2>")?;

    for group in &groups {
        write_finding_group(&mut out, group)?;
    }

    writeln!(&mut out, "  </section>")?;
    writeln!(&mut out, "</main>")?;
    writeln!(&mut out, "<script>{}</script>", js())?;
    writeln!(&mut out, "</body>")?;
    writeln!(&mut out, "</html>")?;

    std::fs::write(path, out).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn write_metric(out: &mut String, label: &str, value: &str) -> Result<()> {
    writeln!(out, "    <div class=\"metric\">")?;
    writeln!(
        out,
        "      <div class=\"metric-label\">{}</div>",
        escape_html(label)
    )?;
    writeln!(
        out,
        "      <div class=\"metric-value\">{}</div>",
        escape_html(value)
    )?;
    writeln!(out, "    </div>")?;
    Ok(())
}

fn write_finding_group(out: &mut String, group: &GroupedFinding<'_>) -> Result<()> {
    if group.files.is_empty() {
        return Ok(());
    }

    writeln!(out, "    <article class=\"finding\">")?;
    writeln!(
        out,
        "      <h3>{} <span class=\"sev {}\">{}</span> <span class=\"count\">{} hits</span></h3>",
        escape_html(group.rule_id),
        escape_html(&group.severity),
        escape_html(&group.severity),
        group.total_hits
    )?;
    writeln!(
        out,
        "      <p class=\"desc\">{}</p>",
        escape_html(group.description)
    )?;
    writeln!(out, "      <ul class=\"kv\">")?;
    writeln!(
        out,
        "        <li><b>Category:</b> {}</li>",
        escape_html(group.category)
    )?;
    writeln!(
        out,
        "        <li><b>Risk:</b> {}</li>",
        escape_html(&group.risk)
    )?;
    writeln!(
        out,
        "        <li><b>Affected files:</b> {}</li>",
        group.files.len()
    )?;
    writeln!(
        out,
        "        <li><b>Migration hint:</b> {}</li>",
        escape_html(group.migration_hint)
    )?;
    writeln!(out, "      </ul>")?;

    write_sample_evidence(out, &group.sample_evidence)?;
    write_occurrence_table(out, group)?;

    if !group.recommended_actions.is_empty() {
        writeln!(out, "      <div class=\"actions\">")?;
        writeln!(out, "        <h4>Recommended Actions</h4>")?;
        for action in &group.recommended_actions {
            writeln!(out, "        <div class=\"action\">")?;
            writeln!(
                out,
                "          <div class=\"action-title\">{} <span class=\"badge\">{}</span></div>",
                escape_html(&action.title),
                escape_html(&action.priority)
            )?;
            writeln!(out, "          <p>{}</p>", escape_html(&action.rationale))?;

            if !action.steps.is_empty() {
                writeln!(out, "          <ol>")?;
                for step in &action.steps {
                    writeln!(out, "            <li>{}</li>", escape_html(step))?;
                }
                writeln!(out, "          </ol>")?;
            }

            if !action.references.is_empty() {
                writeln!(out, "          <ul class=\"refs\">")?;
                for reference in &action.references {
                    let safe = escape_html(reference);
                    writeln!(
                        out,
                        "            <li><a href=\"{0}\" target=\"_blank\" rel=\"noopener noreferrer\">{0}</a></li>",
                        safe
                    )?;
                }
                writeln!(out, "          </ul>")?;
            }

            if !action.code_examples.is_empty() {
                for example in &action.code_examples {
                    writeln!(
                        out,
                        "          <div class=\"code-example\"><div class=\"code-label\">Before ({})</div><pre><code>{}</code></pre></div>",
                        escape_html(&example.language),
                        escape_html(&example.before)
                    )?;
                    writeln!(
                        out,
                        "          <div class=\"code-example\"><div class=\"code-label\">After ({})</div><pre><code>{}</code></pre></div>",
                        escape_html(&example.language),
                        escape_html(&example.after)
                    )?;
                }
            }

            writeln!(out, "        </div>")?;
        }
        writeln!(out, "      </div>")?;
    }

    writeln!(out, "    </article>")?;
    Ok(())
}

fn write_sample_evidence(out: &mut String, examples: &[&str]) -> Result<()> {
    if examples.is_empty() {
        return Ok(());
    }
    writeln!(out, "      <div class=\"evidence-list\">")?;
    writeln!(
        out,
        "        <div class=\"snippet-title\">Sample evidence</div>"
    )?;
    writeln!(out, "        <ul class=\"kv\">")?;
    for evidence in examples.iter().take(3) {
        writeln!(
            out,
            "          <li><code>{}</code></li>",
            escape_html(evidence)
        )?;
    }
    writeln!(out, "        </ul>")?;
    writeln!(out, "      </div>")?;
    Ok(())
}

fn write_occurrence_table(out: &mut String, group: &GroupedFinding<'_>) -> Result<()> {
    writeln!(out, "      <div class=\"occurrence-wrap\">")?;
    writeln!(
        out,
        "        <div class=\"snippet-title\">Occurrences</div>"
    )?;
    writeln!(out, "        <div class=\"occurrence-scroll\">")?;
    writeln!(out, "        <table class=\"occ-table\">")?;
    writeln!(
        out,
        "          <thead><tr><th>File</th><th>Hits</th><th>Lines</th><th>Sample Evidence</th></tr></thead>"
    )?;
    writeln!(out, "          <tbody>")?;
    for (idx, file) in group.files.iter().enumerate() {
        let panel_id = format!("occ-{}-{}", dom_id(group.rule_id), idx);
        let file_cell = if file.sample_findings.is_empty() {
            format!("<code>{}</code>", escape_html(file.file))
        } else {
            format!(
                "<button class=\"file-toggle\" type=\"button\" data-target=\"{}\"><code>{}</code></button>",
                panel_id,
                escape_html(file.file)
            )
        };
        let sample = file
            .sample_evidence
            .first()
            .map(|v| escape_html(v))
            .unwrap_or_else(|| "-".to_string());
        writeln!(
            out,
            "            <tr><td>{}</td><td>{}</td><td>{}</td><td><code>{}</code></td></tr>",
            file_cell,
            file.hits,
            escape_html(&summarize_lines(&file.lines, 16)),
            sample
        )?;

        if !file.sample_findings.is_empty() {
            writeln!(
                out,
                "            <tr id=\"{}\" class=\"occ-snippet-row hidden\"><td colspan=\"4\"><div class=\"occ-snippet-inline\">",
                escape_html(&panel_id)
            )?;
            writeln!(
                out,
                "              <div class=\"snippet-title\">Source snippets: {}</div>",
                escape_html(file.file)
            )?;
            for finding in file.sample_findings.iter().take(3) {
                write_snippet(out, finding)?;
            }
            writeln!(out, "            </div></td></tr>")?;
        }
    }
    writeln!(out, "          </tbody>")?;
    writeln!(out, "        </table>")?;
    writeln!(out, "        </div>")?;
    writeln!(out, "      </div>")?;
    Ok(())
}

fn write_snippet(out: &mut String, finding: &Finding) -> Result<()> {
    if let Some(snippet) = &finding.source_snippet {
        writeln!(out, "      <div class=\"snippet-wrap\">")?;
        writeln!(
            out,
            "        <div class=\"snippet-title\">Source snippet (line {}:{})</div>",
            finding.location.line, finding.location.column
        )?;
        writeln!(out, "        <div class=\"code-lines\">")?;
        for line in &snippet.lines {
            let class = if line.highlighted { "line hl" } else { "line" };
            writeln!(
                out,
                "          <span class=\"{}\"><span class=\"ln\">{}</span><span class=\"tx\">{}</span></span>",
                class,
                line.line,
                escape_html(&line.text)
            )?;
        }
        writeln!(out, "        </div>")?;
        writeln!(out, "      </div>")?;
    }
    Ok(())
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn dom_id(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('-');
        }
    }
    while out.contains("--") {
        out = out.replace("--", "-");
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "group".to_string()
    } else {
        trimmed.to_string()
    }
}

fn js() -> &'static str {
    r#"
document.addEventListener('click', (event) => {
  const button = event.target.closest('.file-toggle');
  if (!button) return;
  const id = button.getAttribute('data-target');
  if (!id) return;
  const panel = document.getElementById(id);
  if (!panel) return;
  panel.classList.toggle('hidden');
});
"#
}

fn css() -> &'static str {
    r#"
:root {
  --bg: #f5f7fb;
  --card: #ffffff;
  --text: #1a2233;
  --muted: #5e6b85;
  --border: #dfe5f2;
  --hl: #fff2c7;
  --line: #f8fafc;
  --critical: #9d0208;
  --high: #d00000;
  --medium: #ef8a17;
  --low: #1f7a8c;
  --info: #3d5a80;
}
* { box-sizing: border-box; }
body {
  margin: 0;
  font-family: "IBM Plex Sans", "Noto Sans", sans-serif;
  background: linear-gradient(180deg, #eef3fb 0%, var(--bg) 100%);
  color: var(--text);
}
.container {
  width: min(1200px, 96vw);
  margin: 0 auto;
  padding: 20px 12px 40px;
}
.top h1 { margin: 0; font-size: 28px; }
.meta { color: var(--muted); margin-top: 8px; }
.summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 12px;
  margin: 18px 0 22px;
}
.metric {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 12px;
}
.metric-label { color: var(--muted); font-size: 13px; }
.metric-value { font-size: 24px; font-weight: 700; }
.finding {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 14px;
  margin-bottom: 16px;
}
.finding h3 {
  margin: 0;
  font-size: 18px;
  display: flex;
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
}
.desc {
  margin: 10px 0 10px;
  color: #26324a;
}
.kv {
  margin: 0 0 10px;
  padding-left: 18px;
}
.kv li {
  margin: 4px 0;
}
.sev {
  font-size: 12px;
  border-radius: 999px;
  padding: 3px 9px;
  border: 1px solid transparent;
}
.sev.critical { background: #ffe5e8; color: var(--critical); border-color: #f3b8bf; }
.sev.high { background: #ffe9e9; color: var(--high); border-color: #f7c0c0; }
.sev.medium { background: #fff4df; color: var(--medium); border-color: #f7d7a2; }
.sev.low { background: #e5f5f8; color: var(--low); border-color: #b8dde4; }
.sev.info { background: #e7eef7; color: var(--info); border-color: #c7d6ea; }
.count {
  font-size: 12px;
  color: var(--muted);
  border: 1px solid #d6dfef;
  border-radius: 999px;
  padding: 2px 8px;
  background: #f7f9fe;
}
.snippet-wrap {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 10px;
}
.evidence-list {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 10px;
  background: #fbfcff;
}
.occurrence-wrap {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 10px;
  background: #fbfcff;
}
.occurrence-scroll {
  width: 100%;
  overflow-x: auto;
}
.snippet-title {
  background: #f3f6fc;
  border-bottom: 1px solid var(--border);
  padding: 6px 10px;
  font-size: 12px;
  color: var(--muted);
}
.code-lines {
  background: #fbfcff;
  font-family: "JetBrains Mono", "SFMono-Regular", Menlo, Consolas, monospace;
  font-size: 13px;
  line-height: 1.2;
}
pre {
  margin: 0;
  overflow-x: auto;
  background: #fbfcff;
}
code {
  display: inline;
  font-family: "JetBrains Mono", "SFMono-Regular", Menlo, Consolas, monospace;
  font-size: 12.5px;
  line-height: 1.35;
}
pre code {
  display: block;
  font-size: 13px;
  line-height: 1.28;
}
.occ-table {
  width: 100%;
  table-layout: fixed;
  border-collapse: collapse;
  font-size: 13px;
}
.occ-table th, .occ-table td {
  border-top: 1px solid #e4ebf7;
  padding: 6px 8px;
  text-align: left;
  vertical-align: top;
}
.occ-table th {
  background: #f8faff;
  color: #3b4964;
  font-weight: 600;
}
.occ-table th:nth-child(1),
.occ-table td:nth-child(1) {
  width: 54%;
}
.occ-table td:nth-child(2) {
  width: 72px;
  text-align: right;
}
.occ-table th:nth-child(3),
.occ-table td:nth-child(3) {
  width: 20%;
}
.occ-table th:nth-child(4),
.occ-table td:nth-child(4) {
  width: 20%;
}
.file-toggle {
  border: 0;
  background: transparent;
  color: #1b5fbf;
  text-decoration: underline;
  cursor: pointer;
  padding: 0;
  width: 100%;
  text-align: left;
  white-space: normal;
  overflow-wrap: anywhere;
  word-break: break-word;
}
.file-toggle code {
  color: inherit;
  white-space: normal;
  overflow-wrap: anywhere;
  word-break: break-word;
}
.occ-table td code {
  white-space: normal;
  overflow-wrap: anywhere;
  word-break: break-word;
}
.line {
  display: grid;
  grid-template-columns: 56px 1fr;
  column-gap: 8px;
  background: var(--line);
  min-height: 1.5em;
}
.line.hl {
  background: var(--hl);
}
.ln {
  user-select: none;
  text-align: right;
  color: #73819c;
  border-right: 1px solid #dfe5f2;
  padding: 0 8px 0 0;
  line-height: 1.5;
}
.tx {
  white-space: pre;
  line-height: 1.5;
}
.occ-snippet-row.hidden {
  display: none;
}
.occ-snippet-inline {
  background: #f9fbff;
  border: 1px solid #e2e9f7;
  border-radius: 8px;
  padding: 8px;
}
.occ-snippet-inline .snippet-wrap {
  margin-top: 8px;
}
.actions {
  border-top: 1px solid var(--border);
  padding-top: 8px;
}
.actions h4 { margin: 0 0 8px; }
.action {
  border: 1px dashed #d7deed;
  border-radius: 8px;
  padding: 8px 10px;
  margin-bottom: 8px;
  background: #fafcff;
}
.action-title {
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}
.badge {
  font-size: 11px;
  border: 1px solid #bfcbe3;
  background: #edf2fb;
  border-radius: 999px;
  padding: 2px 8px;
}
.refs {
  margin: 6px 0;
  padding-left: 18px;
}
.code-example {
  margin-top: 6px;
}
.code-label {
  color: var(--muted);
  font-size: 12px;
  margin-bottom: 3px;
}
@media (max-width: 640px) {
  .line { grid-template-columns: 46px 1fr; }
  code { font-size: 12px; }
  .occ-table th:nth-child(1),
  .occ-table td:nth-child(1) {
    width: 48%;
  }
}
"#
}
