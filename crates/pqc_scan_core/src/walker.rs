use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ignore::WalkBuilder;

use crate::{DiagnosticCode, ScanDiagnostic};

pub(crate) const DEFAULT_EXCLUDES: &[&str] = &[".git", "node_modules", "target", "dist", "docs"];

#[derive(Debug, Clone, Default)]
pub struct WalkStats {
    pub scanned: usize,
    pub skipped: usize,
}

pub fn walk_repository(root: &Path, max_size_bytes: usize) -> Result<(Vec<PathBuf>, WalkStats)> {
    let outcome = walk_repository_detailed(root, max_size_bytes)?;
    Ok((outcome.files, outcome.stats))
}

pub(crate) struct WalkOutcome {
    pub files: Vec<PathBuf>,
    pub stats: WalkStats,
    pub diagnostics: Vec<ScanDiagnostic>,
    pub excluded: u64,
    pub failed_entries: u64,
}

pub(crate) fn walk_repository_detailed(root: &Path, max_size_bytes: usize) -> Result<WalkOutcome> {
    std::fs::metadata(root).with_context(|| "failed to access scan root")?;
    let mut builder = WalkBuilder::new(root);
    builder
        .standard_filters(true)
        .hidden(false)
        .require_git(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true);

    collect_entries(root, max_size_bytes, builder.build())
}

fn collect_entries(
    root: &Path,
    max_size_bytes: usize,
    entries: impl IntoIterator<Item = std::result::Result<ignore::DirEntry, ignore::Error>>,
) -> Result<WalkOutcome> {
    let mut files = Vec::new();
    let mut stats = WalkStats::default();
    let mut diagnostics = Vec::new();
    let mut excluded = 0;
    let mut failed_entries = 0;

    for entry in entries {
        // Iterator errors were fatal in the legacy walker, including root read_dir
        // failures after successful metadata. Preserve that operational contract.
        let entry = entry.map_err(|_| anyhow::anyhow!("failed to enumerate repository"))?;
        let path = entry.path();

        // ignore attaches soft ignore-file errors to otherwise valid directories.
        // Account for these before the directory early return, without exporting
        // raw patterns/error contents or discarding the remaining usable entries.
        if entry.error().is_some() {
            failed_entries += 1;
            diagnostics.push(ScanDiagnostic::new(
                DiagnosticCode::EnumerationFailed,
                Some(relative_path(root, path)),
                None,
            ));
        }

        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }

        if should_exclude(path) {
            stats.skipped += 1;
            excluded += 1;
            diagnostics.push(ScanDiagnostic::new(
                DiagnosticCode::ExcludedPath,
                Some(relative_path(root, path)),
                None,
            ));
            continue;
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => {
                stats.skipped += 1;
                failed_entries += 1;
                diagnostics.push(ScanDiagnostic::new(
                    DiagnosticCode::MetadataFailed,
                    Some(relative_path(root, path)),
                    None,
                ));
                continue;
            }
        };

        if meta.len() as usize > max_size_bytes {
            stats.skipped += 1;
            excluded += 1;
            diagnostics.push(ScanDiagnostic::new(
                DiagnosticCode::FileTooLarge,
                Some(relative_path(root, path)),
                None,
            ));
            continue;
        }

        stats.scanned += 1;
        files.push(path.to_path_buf());
    }

    Ok(WalkOutcome {
        files,
        stats,
        diagnostics,
        excluded,
        failed_entries,
    })
}

pub(crate) fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn should_exclude(path: &Path) -> bool {
    if path.components().any(|c| {
        let value = c.as_os_str().to_string_lossy();
        DEFAULT_EXCLUDES
            .iter()
            .any(|exclude| value.eq_ignore_ascii_case(exclude))
    }) {
        return true;
    }

    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some(ext) if ext.eq_ignore_ascii_case("md")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        dir.push(format!("pqc-scan-{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn iterator_failures_remain_fatal_instead_of_becoming_successful_empty_scans() {
        for depth in [0, 1] {
            let error = ignore::Error::WithDepth {
                depth,
                err: Box::new(ignore::Error::Io(std::io::Error::other(
                    "SYNTHETIC_ENUMERATION_ERROR_CONTENT",
                ))),
            };
            let result = collect_entries(Path::new("."), 1024, [Err(error)]);
            let Err(error) = result else {
                panic!("iterator failure must remain fatal at depth {depth}")
            };
            assert!(!format!("{error:#}").contains("SYNTHETIC_ENUMERATION_ERROR_CONTENT"));
        }
    }

    #[test]
    fn ignore_parse_errors_are_diagnostic_even_if_all_files_are_excluded() {
        let root = temp_dir("walker-ignore-error");
        fs::write(
            root.join(".gitignore"),
            "[z-a]SYNTHETIC_IGNORE_CONTENT\n*\n",
        )
        .unwrap();
        fs::write(root.join("ignored.txt"), "not scanned").unwrap();
        let outcome = walk_repository_detailed(&root, 1024).unwrap();
        assert!(outcome.files.is_empty());
        assert_eq!(outcome.failed_entries, 1);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.code() == DiagnosticCode::EnumerationFailed));
        let json = serde_json::to_string(&outcome.diagnostics).unwrap();
        assert!(!json.contains("SYNTHETIC_IGNORE_CONTENT"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn respects_gitignore_patterns() {
        let root = temp_dir("walker-gitignore");
        fs::write(root.join(".gitignore"), "ignored.conf\n").expect("write .gitignore");
        fs::write(root.join("ignored.conf"), "ssl_protocols TLSv1;").expect("write ignored");
        fs::write(root.join("scanned.conf"), "ssl_protocols TLSv1.2;").expect("write scanned");

        let (files, stats) = walk_repository(&root, 1024 * 1024).expect("walk");
        let file_names: Vec<String> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&root)
                    .expect("relative path")
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        assert!(file_names.iter().any(|n| n == "scanned.conf"));
        assert!(!file_names.iter().any(|n| n == "ignored.conf"));
        assert!(stats.scanned >= 1);
    }

    #[test]
    fn excludes_default_directories() {
        let root = temp_dir("walker-excludes");
        fs::create_dir_all(root.join("node_modules/pkg")).expect("mkdir node_modules");
        fs::create_dir_all(root.join("src")).expect("mkdir src");
        fs::write(root.join("node_modules/pkg/secret.txt"), "TLSv1").expect("write node_modules");
        fs::write(root.join("src/app.conf"), "TLSv1.2").expect("write src file");

        let (files, _) = walk_repository(&root, 1024 * 1024).expect("walk");
        let names: Vec<String> = files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        assert!(names.iter().any(|p| p.ends_with("src/app.conf")));
        assert!(!names
            .iter()
            .any(|p| p.contains("node_modules/pkg/secret.txt")));
    }

    #[test]
    fn skips_files_over_size_limit() {
        let root = temp_dir("walker-size");
        fs::write(root.join("small.txt"), "ok").expect("write small");
        fs::write(root.join("big.txt"), "0123456789ABCDEF").expect("write big");

        let (files, stats) = walk_repository(&root, 8).expect("walk with strict size");
        let names: Vec<String> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&root)
                    .expect("relative path")
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        assert!(names.iter().any(|n| n == "small.txt"));
        assert!(!names.iter().any(|n| n == "big.txt"));
        assert!(stats.skipped >= 1);
    }

    #[test]
    fn excludes_docs_directory_and_markdown_files() {
        let root = temp_dir("walker-docs-md");
        fs::create_dir_all(root.join("docs")).expect("mkdir docs");
        fs::create_dir_all(root.join("src")).expect("mkdir src");
        fs::write(root.join("docs/guide.yaml"), "ssl_protocols TLSv1;").expect("write docs");
        fs::write(root.join("README.md"), "RS256").expect("write readme");
        fs::write(root.join("src/main.rs"), "fn main() {}").expect("write rust");

        let (files, _) = walk_repository(&root, 1024 * 1024).expect("walk");
        let names: Vec<String> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&root)
                    .expect("relative path")
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        assert!(names.iter().any(|n| n == "src/main.rs"));
        assert!(!names.iter().any(|n| n == "README.md"));
        assert!(!names.iter().any(|n| n == "docs/guide.yaml"));
    }
}
