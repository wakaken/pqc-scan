//! Explicit legacy shadow links. Never derive native identities from legacy hashes.
use crate::{Detection, Finding};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone)]
pub(crate) struct LegacyBinding {
    pub(crate) rule: String,
    path: String,
    line: usize,
    column: usize,
    matched: String,
}
type Key = (String, String, usize, usize, String);
impl LegacyBinding {
    pub(crate) fn from_detection(d: &Detection) -> Self {
        Self {
            rule: d.rule_id.clone(),
            path: d.location.file.clone(),
            line: d.location.line,
            column: d.location.column,
            matched: d.evidence.r#match.clone(),
        }
    }
    fn key(&self, root: &Path) -> Option<Key> {
        Some((
            self.rule.clone(),
            Path::new(&self.path)
                .strip_prefix(root)
                .unwrap_or(Path::new(&self.path))
                .to_string_lossy()
                .into_owned(),
            self.line,
            self.column,
            self.matched.clone(),
        ))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyAssociation {
    pub(crate) legacy_index: usize,
    pub(crate) legacy_id: String,
    pub(crate) observation_ids: Vec<String>,
}
pub(crate) fn associate(
    root: &Path,
    findings: &[Finding],
    observations: &[(String, Vec<LegacyBinding>)],
) -> Vec<LegacyAssociation> {
    let mut index: BTreeMap<Key, Vec<usize>> = BTreeMap::new();
    for (i, f) in findings.iter().enumerate() {
        index
            .entry((
                f.rule_id.clone(),
                f.location.file.clone(),
                f.location.line,
                f.location.column,
                f.evidence.r#match.clone(),
            ))
            .or_default()
            .push(i);
    }
    let mut links = vec![Vec::new(); findings.len()];
    for (id, bindings) in observations {
        for binding in bindings {
            if let Some(indices) = binding.key(root).and_then(|k| index.get(&k)) {
                for i in indices {
                    links[*i].push(id.clone());
                }
            }
        }
    }
    findings
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let mut ids = links[i].clone();
            ids.sort();
            ids.dedup();
            LegacyAssociation {
                legacy_index: i,
                legacy_id: f.finding_id.clone(),
                observation_ids: ids,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyLossRecord {
    pub(crate) observation_id: String,
    pub(crate) reason: String,
    pub(crate) count: usize,
    pub(crate) unit: String,
}
pub(crate) fn losses(
    root: &Path,
    findings: &[Finding],
    observations: &[(String, Vec<LegacyBinding>)],
    capture_reasons: &BTreeMap<String, String>,
    known_rules: Option<&BTreeSet<String>>,
) -> Vec<LegacyLossRecord> {
    let keys: BTreeSet<_> = findings
        .iter()
        .map(|f| {
            (
                f.rule_id.clone(),
                f.location.file.clone(),
                f.location.line,
                f.location.column,
                f.evidence.r#match.clone(),
            )
        })
        .collect();
    let mut result = Vec::new();
    for (id, bindings) in observations {
        if bindings.is_empty() {
            result.push(LegacyLossRecord {
                observation_id: id.clone(),
                reason: capture_reasons[id].clone(),
                count: 1,
                unit: "capture_only_observation".into(),
            });
        }
        let mut reasons: BTreeMap<&str, usize> = BTreeMap::new();
        for b in bindings {
            if b.key(root).is_some_and(|k| keys.contains(&k)) {
                continue;
            }
            let reason = if b.rule == "__DEPENDENCY_INVENTORY__" {
                "dependency_inventory_not_a_legacy_finding"
            } else if known_rules.is_some_and(|r| !r.contains(&b.rule)) {
                "unknown_rule_dropped_by_legacy_normalization"
            } else if b.key(root).is_none() {
                "legacy_target_unavailable"
            } else {
                "legacy_normalization_collapsed_distinct_binding"
            };
            *reasons.entry(reason).or_default() += 1;
        }
        result.extend(reasons.into_iter().map(|(reason, count)| LegacyLossRecord {
            observation_id: id.clone(),
            reason: reason.into(),
            count,
            unit: "legacy_binding".into(),
        }));
    }
    result
}
