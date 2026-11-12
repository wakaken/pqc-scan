//! Read-only references to files already selected by this scan's walker.
use crate::ScannableFile;
use std::{
    collections::BTreeSet,
    path::{Component, Path, PathBuf},
};

pub struct RepositoryContext {
    root: PathBuf,
    selected: BTreeSet<PathBuf>,
    max_bytes: usize,
}
fn normalized(path: &Path) -> Option<PathBuf> {
    let mut out = PathBuf::new();
    for part in std::path::absolute(path).ok()?.components() {
        match part {
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            Component::CurDir => {}
            _ => out.push(part.as_os_str()),
        }
    }
    Some(out)
}
impl RepositoryContext {
    pub(crate) fn new(root: &Path, selected: &[PathBuf], max_bytes: usize) -> Option<Self> {
        Some(Self {
            root: normalized(root)?,
            selected: selected.iter().filter_map(|p| normalized(p)).collect(),
            max_bytes: max_bytes.min(2 * 1024 * 1024),
        })
    }
    pub fn read_relative(&self, from: &Path, relative: &Path) -> Option<ScannableFile> {
        if relative.is_absolute() || relative.components().count() > 128 {
            return None;
        }
        let target = normalized(&normalized(from)?.parent()?.join(relative))?;
        if !target.starts_with(&self.root) || !self.selected.contains(&target) {
            return None;
        }
        let bytes = crate::safe_read::read(&self.root, &target, self.max_bytes).ok()?;
        Some(ScannableFile::from_bytes(target, bytes))
    }
    pub fn sibling_files(&self, from: &Path, extension: &str) -> Vec<PathBuf> {
        let Some(from) = normalized(from) else {
            return vec![];
        };
        let Some(parent) = from.parent() else {
            return vec![];
        };
        self.selected
            .range(parent.to_path_buf()..)
            .take_while(|p| p.starts_with(parent))
            .filter(|p| p.parent() == Some(parent) && p.extension().is_some_and(|e| e == extension))
            .take(129)
            .cloned()
            .collect()
    }
}
