//! An index of finding-bearing paths only. Never reads source files.
use crate::{japanese::html as h, ReportLanguage};
use pqc_scan_core::ScanResult;
use std::{collections::BTreeMap, fmt::Write};

// Small inline icons keep standalone reports independent of fonts and CDNs.
const CHEVRON: &str = r#"<svg class="tree-chevron" width="16" height="16" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="m6 4 4 4-4 4"/></svg>"#;
const FOLDER: &str = r#"<svg class="tree-icon tree-folder" width="16" height="16" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M1 3.25C1 2.56 1.56 2 2.25 2h3.3c.4 0 .78.19 1.02.51L7.7 4h6.05c.69 0 1.25.56 1.25 1.25v7.5c0 .69-.56 1.25-1.25 1.25H2.25C1.56 14 1 13.44 1 12.75Z"/></svg>"#;
const FILE: &str = r#"<svg class="tree-icon tree-file" width="16" height="16" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.5 1.5h5.75l3.25 3.25V14a.5.5 0 0 1-.5.5H4a.5.5 0 0 1-.5-.5Z"/><path d="M9 1.5V5h3.5"/></svg>"#;

#[derive(Default)]
struct Directory {
    directories: BTreeMap<String, Directory>,
    files: BTreeMap<String, (String, usize)>,
    count: usize,
}
impl Directory {
    fn insert(&mut self, parts: &[&str], path: &str) {
        self.count += 1;
        if parts.len() == 1 {
            let entry = self
                .files
                .entry(parts[0].to_string())
                .or_insert((path.to_string(), 0));
            entry.1 += 1;
        } else if let Some((name, rest)) = parts.split_first() {
            self.directories
                .entry((*name).to_string())
                .or_default()
                .insert(rest, path);
        }
    }
    fn render(&self, out: &mut String, id: &mut usize, lang: ReportLanguage) {
        out.push_str("<ul>");
        for (name, directory) in &self.directories {
            let current = *id;
            *id += 1;
            write!(out, "<li><details id=\"tree-dir-{current}\"><summary>{CHEVRON}{FOLDER}<span class=\"tree-name\">{}</span><span class=\"tree-count\">{}</span></summary>", h(name), directory.count).unwrap();
            directory.render(out, id, lang);
            out.push_str("</details></li>");
        }
        for (name, (path, count)) in &self.files {
            let label = if lang == ReportLanguage::Ja {
                format!("{path}、全検出{count}件")
            } else {
                format!("{path}, {count} total findings")
            };
            write!(out, "<li><button type=\"button\" data-tree-file=\"{}\" title=\"{}\" aria-label=\"{}\" aria-pressed=\"false\"><span class=\"tree-chevron-space\" aria-hidden=\"true\"></span>{FILE}<span class=\"tree-name\">{}</span><span class=\"tree-count\">{count}</span></button></li>", h(path),h(path),h(&label),h(name)).unwrap();
        }
        out.push_str("</ul>");
    }
}
pub(crate) fn render(result: &ScanResult, lang: ReportLanguage) -> String {
    let mut root = Directory::default();
    for finding in &result.findings {
        let parts: Vec<_> = finding
            .location
            .file
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        if !parts.is_empty() {
            if parts.len() > 64 {
                root.insert(&[finding.location.file.as_str()], &finding.location.file);
            } else {
                root.insert(&parts, &finding.location.file);
            }
        }
    }
    let mut out = String::new();
    let (title, note, clear, empty) = if lang == ReportLanguage::Ja {
        (
            "ファイル",
            "検出のあるファイル · 件数は全検出",
            "すべてのファイル",
            "検出のあるファイルはありません。",
        )
    } else {
        (
            "Files",
            "Files with findings · counts include all findings",
            "All files",
            "No files with findings.",
        )
    };
    write!(out, "<section id=\"report-file-tree\" class=\"file-tree\" data-file-tree hidden aria-label=\"{title}\"><h2>{title}</h2><p>{note}</p><button type=\"button\" data-tree-clear>{clear}</button>").unwrap();
    if root.count == 0 {
        write!(out, "<p>{empty}</p>").unwrap();
    }
    root.render(&mut out, &mut 0, lang);
    out.push_str("</section>");
    out
}
