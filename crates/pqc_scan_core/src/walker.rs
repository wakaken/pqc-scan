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
    pub custom_excludes: bool,
}

pub(crate) fn walk_repository_detailed(root: &Path, max_size_bytes: usize) -> Result<WalkOutcome> {
    walk_repository_detailed_with_selection(root, max_size_bytes, crate::FileSelection::default())
}

pub(crate) fn walk_repository_detailed_with_selection(
    root: &Path,
    max_size_bytes: usize,
    selection: crate::FileSelection,
) -> Result<WalkOutcome> {
    walk_repository_detailed_with_scope(root, max_size_bytes, selection.into())
}

pub(crate) fn walk_repository_detailed_with_scope(
    root: &Path,
    max_size_bytes: usize,
    selection: crate::ScanSelection,
) -> Result<WalkOutcome> {
    std::fs::metadata(root).with_context(|| "failed to access scan root")?;
    let mut builder = WalkBuilder::new(root);
    builder
        .standard_filters(true)
        .hidden(false)
        .require_git(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true);

    collect_entries_with_scope(root, max_size_bytes, builder.build(), selection)
}

#[cfg(test)]
fn collect_entries(
    root: &Path,
    max_size_bytes: usize,
    entries: impl IntoIterator<Item = std::result::Result<ignore::DirEntry, ignore::Error>>,
) -> Result<WalkOutcome> {
    collect_entries_with_selection(
        root,
        max_size_bytes,
        entries,
        crate::FileSelection::default(),
    )
}
#[cfg(test)]
fn collect_entries_with_selection(
    root: &Path,
    max_size_bytes: usize,
    entries: impl IntoIterator<Item = std::result::Result<ignore::DirEntry, ignore::Error>>,
    selection: crate::FileSelection,
) -> Result<WalkOutcome> {
    collect_entries_with_scope(root, max_size_bytes, entries, selection.into())
}
fn collect_entries_with_scope(
    root: &Path,
    max_size_bytes: usize,
    entries: impl IntoIterator<Item = std::result::Result<ignore::DirEntry, ignore::Error>>,
    selection: crate::ScanSelection,
) -> Result<WalkOutcome> {
    // Read only the selected root's policy through the same bounded, no-follow
    // reader as source files. Do not inherit a policy from outside that root.
    let policy = root.join(".pqcignore");
    let has_policy = if root.is_dir() {
        match std::fs::symlink_metadata(&policy) {
            Ok(_) => true,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
            Err(_) => anyhow::bail!("failed to access .pqcignore"),
        }
    } else {
        false
    };
    let custom = if has_policy {
        let bytes = crate::safe_read::read(root, &policy, max_size_bytes)
            .map_err(|_| anyhow::anyhow!("failed to read .pqcignore"))?;
        let text =
            std::str::from_utf8(&bytes).map_err(|_| anyhow::anyhow!(".pqcignore must be UTF-8"))?;
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let mut builder = ignore::gitignore::GitignoreBuilder::new(root);
        for line in text.lines() {
            builder
                .add_line(Some(policy.clone()), line)
                .map_err(|_| anyhow::anyhow!("invalid .pqcignore pattern"))?;
        }
        Some(
            builder
                .build()
                .map_err(|_| anyhow::anyhow!("invalid .pqcignore policy"))?,
        )
    } else {
        None
    };
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

        if (selection.exclude_tests && crate::file_selection::test_path(root, path))
            || excluded_directory_or_markdown(if path == root {
                path.file_name().map(Path::new).unwrap_or(path)
            } else {
                path.strip_prefix(root).unwrap_or(path)
            })
            || custom.as_ref().is_some_and(|matcher| {
                path == policy || matcher.matched_path_or_any_parents(path, false).is_ignore()
            })
            || (!selection.files.include_non_scannable
                && crate::file_selection::known_extension(path))
        {
            stats.skipped += 1;
            excluded += 1;
            diagnostics.push(ScanDiagnostic::new(
                DiagnosticCode::ExcludedPath,
                Some(relative_path(root, path)),
                None,
            ));
            continue;
        }

        if !entry.file_type().is_some_and(|t| t.is_file()) {
            stats.skipped += 1;
            failed_entries += 1;
            diagnostics.push(ScanDiagnostic::new(
                DiagnosticCode::ReadFailed,
                Some(relative_path(root, path)),
                None,
            ).with_explanation(Some(if entry.file_type().is_some_and(|t| t.is_symlink()) {
                "Symbolic link not followed by scan policy. Its target was not inspected through this path. Select the real target folder explicitly if needed."
            } else {
                "Non-regular filesystem entry not read by scan policy. Select a regular file or inspect this entry separately."
            })));
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
            diagnostics.push(
                ScanDiagnostic::new(
                    DiagnosticCode::FileTooLarge,
                    Some(relative_path(root, path)),
                    None,
                )
                .with_size_limit(max_size_bytes, meta.len()),
            );
            continue;
        }

        // Failure to preflight must not turn a read/metadata error into an exclusion.
        // safe_read retains no-follow, regular-file and size checks here too.
        if !selection.files.include_non_scannable && !crate::file_selection::protects_crypto(path) {
            if let Ok(bytes) = crate::safe_read::read(root, path, max_size_bytes) {
                if crate::file_selection::non_text(path, &bytes) {
                    stats.skipped += 1;
                    excluded += 1;
                    diagnostics.push(ScanDiagnostic::new(
                        DiagnosticCode::ExcludedPath,
                        Some(relative_path(root, path)),
                        None,
                    ));
                    continue;
                }
            }
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
        custom_excludes: custom.is_some(),
    })
}

pub(crate) fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

#[cfg(test)]
fn should_exclude(path: &Path) -> bool {
    excluded_directory_or_markdown(path) || crate::file_selection::known_extension(path)
}
fn excluded_directory_or_markdown(path: &Path) -> bool {
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

    #[test]
    fn selected_root_is_not_excluded_by_ancestor_names() {
        let parent = temp_dir("relative-excludes");
        let root = parent.join("target/docs/product");
        fs::create_dir_all(root.join("target")).unwrap();
        fs::write(root.join("main.rs"), "fn main() {}").unwrap();
        fs::write(root.join("target/generated.rs"), "fn ignored() {}").unwrap();
        let outcome = walk_repository_detailed(&root, 4096).unwrap();
        assert_eq!(outcome.files, vec![root.join("main.rs")]);
        assert_eq!(outcome.excluded, 1);
        // Explicit file roots still honor the markdown exclusion.
        fs::write(root.join("notes.md"), "notes").unwrap();
        assert!(walk_repository_detailed(&root.join("notes.md"), 4096)
            .unwrap()
            .files
            .is_empty());
        fs::remove_dir_all(parent).unwrap();
    }

    #[test]
    fn custom_policy_is_root_local_supports_negation_and_rejects_invalid_input() {
        let parent = temp_dir("custom-excludes");
        let root = parent.join("product");
        fs::create_dir_all(root.join("fixtures")).unwrap();
        fs::write(parent.join(".pqcignore"), "*\n").unwrap();
        fs::write(
            root.join(".pqcignore"),
            "\u{feff}fixtures/*.rs\r\n!fixtures/keep.rs\r\n",
        )
        .unwrap();
        fs::write(root.join("main.rs"), "fn main() {}").unwrap();
        fs::write(root.join("fixtures/drop.rs"), "fn test() {}").unwrap();
        fs::write(root.join("fixtures/keep.rs"), "fn keep() {}").unwrap();
        let outcome = walk_repository_detailed(&root, 4096).unwrap();
        assert!(outcome.custom_excludes);
        assert_eq!(outcome.files.len(), 2);
        assert!(outcome.files.contains(&root.join("main.rs")));
        assert!(outcome.files.contains(&root.join("fixtures/keep.rs")));
        assert_eq!(outcome.excluded, 2);
        fs::write(root.join(".pqcignore"), "[z-a]PRIVATE_PATTERN\n").unwrap();
        let error = walk_repository_detailed(&root, 4096).err().unwrap();
        assert!(!error.to_string().contains("PRIVATE_PATTERN"));
        fs::write(root.join(".pqcignore"), [255]).unwrap();
        assert!(walk_repository_detailed(&root, 4096).is_err());
        fs::write(root.join(".pqcignore"), "fixtures/*.rs\n").unwrap();
        assert!(walk_repository_detailed(&root, 4).is_err());
        fs::remove_dir_all(parent).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn custom_policy_does_not_follow_symlinks() {
        let root = temp_dir("custom-policy-link");
        let outside = temp_dir("custom-policy-outside");
        fs::write(outside.join("policy"), "*\n").unwrap();
        std::os::unix::fs::symlink(outside.join("policy"), root.join(".pqcignore")).unwrap();
        assert!(walk_repository_detailed(&root, 4096).is_err());
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }

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

    #[cfg(unix)]
    #[test]
    fn special_entries_are_failed_without_selecting_or_following_them() {
        use std::os::unix::fs::symlink;
        let root = temp_dir("walker-special");
        let outside = temp_dir("walker-outside");
        fs::write(outside.join("large.txt"), vec![b'x'; 4096]).unwrap();
        symlink(outside.join("large.txt"), root.join("link.txt")).unwrap();
        symlink(outside.join("missing"), root.join("broken.txt")).unwrap();
        let fifo = root.join("pipe");
        assert!(std::process::Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success());
        fs::write(root.join("ok.txt"), "ok").unwrap();
        let result = walk_repository_detailed(&root, 128).unwrap();
        assert_eq!(result.files, vec![root.join("ok.txt")]);
        assert_eq!(result.failed_entries, 3);
        assert_eq!(result.stats.skipped, 3);
        assert_eq!(
            result
                .diagnostics
                .iter()
                .filter(|d| d.code() == DiagnosticCode::ReadFailed)
                .count(),
            3
        );
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(outside).unwrap();
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

#[cfg(test)]
mod non_scannable_repairs {
    #[test]
    fn known_binary_artifacts_are_excluded_but_crypto_files_are_retained() {
        for name in [
            "image.png",
            "archive.zip",
            "module.wasm",
            "binary.exe",
            "messages.mo",
            "fixture.pb",
        ] {
            assert!(super::should_exclude(std::path::Path::new(name)), "{name}");
        }
        for name in [
            "code.js",
            "key.pem",
            "trust.jks",
            "store.pkcs12",
            "unknown.data",
        ] {
            assert!(!super::should_exclude(std::path::Path::new(name)), "{name}");
        }
    }
}
