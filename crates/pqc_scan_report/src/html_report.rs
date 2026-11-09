use anyhow::Result;
use pqc_scan_core::ScanResult;
use std::path::Path;

pub fn write_html_report(result: &ScanResult, path: &Path) -> Result<()> {
    crate::compact::html_report(result, path, crate::ReportLanguage::En)
}

pub(crate) fn css() -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use pqc_scan_core::{
        CbomEntry, CodeExample, Evidence, Finding, Location, RecommendedAction, ScanResult,
        ScanSummary, SourceSnippet, SourceSnippetLine,
    };
    use pqc_scan_rules::{Risk, Severity};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(prefix: &str, extension: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        path.push(format!(
            "pqc-scan-{prefix}-{}-{nanos}.{extension}",
            std::process::id()
        ));
        path
    }

    fn sample_result() -> ScanResult {
        let action = RecommendedAction {
            action_id: "tls-migrate-1".to_string(),
            title: "Adopt <ML-KEM> & rotate".to_string(),
            priority: "p1".to_string(),
            rationale: "Replace \"RSA\" <soon> & avoid fallback".to_string(),
            steps: vec![
                "Set policy <strict>".to_string(),
                "Roll keys & certs".to_string(),
            ],
            references: vec!["https://example.com/guide?mode=html&v=1".to_string()],
            code_examples: vec![CodeExample {
                language: "rust".to_string(),
                before: "let alg = \"rsa<legacy>\";".to_string(),
                after: "let alg = \"ml-kem\" && ready;".to_string(),
            }],
        };

        let finding = |id: &str, file: &str, line: usize, evidence: &str, snippet: &str| Finding {
            finding_id: id.to_string(),
            rule_id: "TLS_RSA_DEPRECATED".to_string(),
            category: "TLS".to_string(),
            risk: Risk::QuantumVulnerable,
            severity: Severity::High,
            confidence: 0.91,
            description: "TLS <RSA> & fallback".to_string(),
            migration_hint: "Switch to <ML-KEM> & rotate".to_string(),
            location: Location {
                file: file.to_string(),
                line,
                column: 7,
            },
            evidence: Evidence {
                r#type: "regex_match".to_string(),
                r#match: evidence.to_string(),
                snippet_preview: snippet.to_string(),
                metadata: BTreeMap::new(),
            },
            recommended_actions: vec![action.clone()],
            source_snippet: Some(SourceSnippet {
                lines: vec![SourceSnippetLine {
                    line,
                    text: snippet.to_string(),
                    highlighted: true,
                }],
            }),
        };

        ScanResult {
            generated_at: Utc
                .with_ymd_and_hms(2025, 1, 2, 3, 4, 5)
                .single()
                .expect("valid timestamp"),
            findings: vec![
                finding(
                    "f1",
                    "service<prod>.yaml",
                    12,
                    "[masked-private-key-material] <sensitive>",
                    "cipher <rsa> & fallback",
                ),
                finding(
                    "f2",
                    "gateway&edge.yaml",
                    40,
                    "[masked-sensitive-content] & sample",
                    "allow <legacy> & log",
                ),
            ],
            cbom: vec![CbomEntry {
                component: "service<prod>.yaml".to_string(),
                algorithm: "TLS_RSA".to_string(),
                usage_type: "TLS".to_string(),
                location: "service<prod>.yaml:12".to_string(),
                quantum_risk: Risk::QuantumVulnerable,
                migration_hint: "Switch to <ML-KEM> & rotate".to_string(),
            }],
            dependency_sbom: Vec::new(),
            summary: ScanSummary {
                total_findings: 2,
                by_severity: BTreeMap::from([("high".to_string(), 2)]),
                by_risk: BTreeMap::from([("quantum-vulnerable".to_string(), 2)]),
                scanned_files: 2,
                skipped_files: 0,
            },
        }
    }

    #[test]
    fn html_report_escapes_grouped_findings_and_renders_actions() {
        let path = temp_file("html-report-contract", "html");
        let result = sample_result();

        write_html_report(&result, &path).expect("write html report");

        let rendered = fs::read_to_string(&path).expect("read html report");

        assert!(rendered.contains("TLS_RSA_DEPRECATED"));
        assert!(rendered.contains("<span class=\"count\">2 hits</span>"));
        assert!(rendered.contains("TLS &lt;RSA&gt; &amp; fallback"));
        assert!(rendered.contains("service&lt;prod&gt;.yaml"));
        assert!(rendered.contains("gateway&amp;edge.yaml"));
        assert!(rendered.contains("[masked-private-key-material] &lt;sensitive&gt;"));
        assert!(rendered.contains("[masked-sensitive-content] &amp; sample"));
        assert!(rendered.contains("Recommended Actions"));
        assert!(rendered.contains("Adopt &lt;ML-KEM&gt; &amp; rotate"));
        assert!(rendered.contains("Replace &quot;RSA&quot; &lt;soon&gt; &amp; avoid fallback"));
        assert!(rendered.contains("https://example.com/guide?mode=html&amp;v=1"));
        assert!(rendered.contains("cipher &lt;rsa&gt; &amp; fallback"));
        assert!(rendered.contains("allow &lt;legacy&gt; &amp; log"));

        let _ = fs::remove_file(path);
    }
}
