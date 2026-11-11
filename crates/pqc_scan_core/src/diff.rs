//! Conservative, explicit-scope comparison of existing inventory-v1 records.
//!
//! A missing observation is never evidence that a cryptographic risk is resolved.
//! Inventory v1 does not contain per-target exhaustive native inspection proof.
use crate::assessment::valid_id;
use crate::identity::valid_scope;
use crate::inventory::{CryptoAsset, NativeScan};
use crate::observation::Observation;
use anyhow::{ensure, Result};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_RECORDS: usize = 100_000;
const IDENTITY_SCHEME: &str = "pqc-scan-length-prefixed-sha256-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeStatus {
    Added,
    Removed,
    Changed,
    Unchanged,
    Unresolved,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordChange {
    pub key: String,
    pub status: ChangeStatus,
    pub before_ids: Vec<String>,
    pub after_ids: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MoveCandidate {
    pub before_occurrence_key: String,
    pub after_occurrence_key: String,
    pub confidence: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffReport {
    pub schema_version: u32,
    pub repository_scope: String,
    pub identity_scheme: String,
    pub drift: Vec<String>,
    pub assets: Vec<RecordChange>,
    pub occurrences: Vec<RecordChange>,
    pub move_candidates: Vec<MoveCandidate>,
    pub limitations: Vec<String>,
}

impl DiffReport {
    /// Fixed copy and counts only: imported strings cannot inject Markdown.
    pub fn summary_markdown(&self) -> String {
        let mut out = String::from("# Inventory comparison\n\nThis compares reported static records, not runtime security or completed PQC migration.\n\n| Record set | Added | Removed | Changed | Unchanged | Unresolved |\n|---|---:|---:|---:|---:|---:|\n");
        for (name, rows) in [("Assets", &self.assets), ("Occurrences", &self.occurrences)] {
            let count = |status| rows.iter().filter(|row| row.status == status).count();
            out.push_str(&format!(
                "| {name} | {} | {} | {} | {} | {} |\n",
                count(ChangeStatus::Added),
                count(ChangeStatus::Removed),
                count(ChangeStatus::Changed),
                count(ChangeStatus::Unchanged),
                count(ChangeStatus::Unresolved)
            ));
        }
        out.push_str(&format!("\nMetadata drift categories: {}. Heuristic move candidates: {}.\n\nMissing-after records remain unresolved because inventory v1 cannot prove exhaustive inspection of that target. Added means newly present in the compared records. Move candidates do not merge identities. Removal would describe an observation set, never remediation of cryptographic risk. See scan.diff.v1.json for exact keys and reason codes.\n", self.drift.len(), self.move_candidates.len()));
        out
    }
}

/// Deserialize bounded untrusted input. Diagnostics never echo imported content.
pub fn parse_inventory(input: &[u8]) -> Result<NativeScan> {
    ensure!(
        input.len() <= MAX_INPUT_BYTES,
        "inventory input limit exceeded"
    );
    let value: NativeScan =
        serde_json::from_slice(input).map_err(|_| anyhow::anyhow!("invalid inventory JSON"))?;
    validate(&value)?;
    Ok(value)
}

fn validate(n: &NativeScan) -> Result<()> {
    n.validate()
        .map_err(|_| anyhow::anyhow!("invalid inventory references or schema"))?;
    ensure!(
        n.identity_scheme == IDENTITY_SCHEME,
        "unsupported inventory identity scheme"
    );
    ensure!(
        n.assets.len().saturating_add(n.observations.len()) <= MAX_RECORDS,
        "inventory record limit exceeded"
    );
    ensure!(
        serde_json::to_vec(n)?.len() <= MAX_INPUT_BYTES,
        "inventory input limit exceeded"
    );
    ensure!(
        valid_id(&n.run.id, "scan"),
        "invalid inventory run identity"
    );
    for a in &n.assets {
        ensure!(valid_id(&a.id, "asset"), "invalid inventory asset identity");
    }
    let mut keys = BTreeSet::new();
    for o in &n.observations {
        ensure!(
            valid_id(&o.id, "observation"),
            "invalid inventory observation identity"
        );
        if let Some(key) = &o.occurrence_key {
            ensure!(
                valid_id(key, "occurrence") && keys.insert(key),
                "invalid or duplicate occurrence key"
            );
        }
        if let Some(path) = &o.target.path {
            ensure!(safe_path(path), "invalid inventory target path");
        }
    }
    Ok(())
}

fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= 4096
        && !path.starts_with('/')
        && !path.contains([':', '\\'])
        && !path.chars().any(char::is_control)
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

/// Compare scoped record correspondence conservatively; performs no I/O.
pub fn compare(before: &NativeScan, after: &NativeScan, scope: &str) -> Result<DiffReport> {
    ensure!(valid_scope(scope), "invalid explicit repository scope");
    validate(before)?;
    validate(after)?;
    ensure!(
        before.repository_scope.as_deref() == Some(scope)
            && after.repository_scope.as_deref() == Some(scope),
        "repository scopes must match explicit scope"
    );
    let mut drift = Vec::new();
    for (changed, reason) in [
        (
            before.rules_digest != after.rules_digest,
            "rules_digest_changed",
        ),
        (before.profile != after.profile, "profile_changed"),
        (before.scope != after.scope, "inspection_scope_changed"),
        (
            before.scanner_version != after.scanner_version,
            "scanner_version_changed",
        ),
        (
            before.catalog_version != after.catalog_version,
            "catalog_version_changed",
        ),
        (
            before.coverage != after.coverage,
            "inspection_coverage_changed",
        ),
        (
            before.native_completeness != after.native_completeness,
            "native_completeness_changed",
        ),
    ] {
        if changed {
            drift.push(reason.into());
        }
    }
    let old_assets: BTreeMap<_, _> = before.assets.iter().map(|a| (a.id.as_str(), a)).collect();
    let new_assets: BTreeMap<_, _> = after.assets.iter().map(|a| (a.id.as_str(), a)).collect();
    let mut assets = Vec::new();
    for key in old_assets
        .keys()
        .chain(new_assets.keys())
        .copied()
        .collect::<BTreeSet<_>>()
    {
        let old = old_assets.get(key).copied();
        let new = new_assets.get(key).copied();
        let (status, reason) = match (old, new) {
            (Some(a), Some(b)) if !asset_correspondence(a) || !asset_correspondence(b) => (
                ChangeStatus::Unresolved,
                "ambiguous_or_unknown_asset_identity",
            ),
            (Some(a), Some(b)) if asset_facts(a) == asset_facts(b) => {
                (ChangeStatus::Unchanged, "same_exact_asset_key_and_facts")
            }
            (Some(_), Some(_)) => (ChangeStatus::Changed, "same_exact_asset_key_changed_facts"),
            (None, Some(b)) if !asset_correspondence(b) => (
                ChangeStatus::Unresolved,
                "ambiguous_or_unknown_asset_identity",
            ),
            (None, Some(_)) => (ChangeStatus::Added, "newly_present_in_observation_set"),
            _ => (ChangeStatus::Unresolved, "target_inspection_not_proven"),
        };
        assets.push(change(
            key,
            status,
            old.map(|a| a.id.as_str()),
            new.map(|a| a.id.as_str()),
            reason,
        ));
    }
    let old_occ = occurrences(before);
    let new_occ = occurrences(after);
    let mut occurrences = Vec::new();
    for key in old_occ
        .keys()
        .chain(new_occ.keys())
        .copied()
        .collect::<BTreeSet<_>>()
    {
        let old = old_occ.get(key).copied();
        let new = new_occ.get(key).copied();
        let (status, reason) = match (old, new) {
            (Some(a), Some(b))
                if !observation_correspondence(a) || !observation_correspondence(b) =>
            {
                (
                    ChangeStatus::Unresolved,
                    "ambiguous_or_unknown_occurrence_identity",
                )
            }
            (Some(a), Some(b)) if observation_facts(a) == observation_facts(b) => (
                ChangeStatus::Unchanged,
                "same_weak_occurrence_key_and_facts",
            ),
            (Some(_), Some(_)) => (
                ChangeStatus::Changed,
                "same_weak_occurrence_key_changed_facts",
            ),
            (None, Some(b)) if observation_correspondence(b) => {
                (ChangeStatus::Added, "newly_present_in_observation_set")
            }
            (None, Some(_)) => (
                ChangeStatus::Unresolved,
                "ambiguous_or_unknown_occurrence_identity",
            ),
            _ => (ChangeStatus::Unresolved, "target_inspection_not_proven"),
        };
        occurrences.push(change(
            key,
            status,
            old.map(|o| o.id.as_str()),
            new.map(|o| o.id.as_str()),
            reason,
        ));
    }
    // Run-local observations have no cross-scan key. Keep every record visible,
    // including when neither side can be correlated.
    for (source, side) in [(before, "before"), (after, "after")] {
        for o in source
            .observations
            .iter()
            .filter(|o| o.occurrence_key.is_none())
        {
            occurrences.push(change(
                &format!("{side}:{}", o.id),
                ChangeStatus::Unresolved,
                (side == "before").then_some(o.id.as_str()),
                (side == "after").then_some(o.id.as_str()),
                "run_local_observation_not_correlatable",
            ));
        }
    }
    occurrences.sort_by(|a, b| a.key.cmp(&b.key));
    let move_candidates = moves(before, after, &old_occ, &new_occ);
    Ok(DiffReport {
        schema_version: 1,
        repository_scope: scope.into(),
        identity_scheme: IDENTITY_SCHEME.into(),
        drift,
        assets,
        occurrences,
        move_candidates,
        limitations: vec![
            "inventory_v1_has_no_target_level_exhaustive_native_inspection_proof".into(),
            "absence_is_not_risk_resolution".into(),
            "weak_occurrence_keys_are_correlation_hints_only".into(),
            "unknown_extra_input_fields_are_not_compared".into(),
        ],
    })
}

fn change(
    key: &str,
    status: ChangeStatus,
    old: Option<&str>,
    new: Option<&str>,
    reason: &str,
) -> RecordChange {
    RecordChange {
        key: key.into(),
        status,
        before_ids: old.into_iter().map(str::to_owned).collect(),
        after_ids: new.into_iter().map(str::to_owned).collect(),
        reasons: vec![reason.into()],
    }
}

fn asset_correspondence(a: &CryptoAsset) -> bool {
    matches!(
        (a.kind.as_str(), a.identity_basis.as_str()),
        (
            "algorithm",
            "normalized_identifier_and_reported_parameters"
                | "identifier_only_parameters_not_fully_interpreted"
        ) | ("public_key", "public_spki_sha256")
            | ("certificate", "public_certificate_der_sha256")
            | (
                "dependency",
                "reported_package_identity_not_runtime_use"
                    | "scoped_source_assertion_not_installed_package"
            )
            | ("usage", "scoped_weak_source_anchor")
    ) && !a.uncertainty.iter().any(|u| u == "conflicting_descriptors")
        && !(a.kind == "usage" && a.attributes.get("ambiguous") != Some(&Value::Bool(false)))
}

fn asset_facts(a: &CryptoAsset) -> Value {
    let mut relations: Vec<_> = a
        .relations
        .iter()
        .map(|r| (&r.relationship, &r.target))
        .collect();
    relations.sort();
    let mut uncertainty = a.uncertainty.clone();
    uncertainty.sort();
    json!([
        a.kind,
        a.identity_basis,
        a.attributes,
        relations,
        uncertainty
    ])
}
fn observation_correspondence(o: &Observation) -> bool {
    o.occurrence_key.is_some()
        && o.target.path.is_some()
        && o.identity_confidence == "weak_structural_anchor"
}
fn observation_facts(o: &Observation) -> Value {
    let mut assets = o.asset_ids.clone();
    assets.sort();
    let mut uncertainty = o.uncertainty.clone();
    uncertainty.sort();
    json!([
        assets,
        o.target.path,
        o.target.object_index,
        o.method,
        o.producer,
        o.assurance,
        o.usage,
        o.rule_id,
        o.evidence,
        uncertainty
    ])
}
fn occurrences(n: &NativeScan) -> BTreeMap<&str, &Observation> {
    n.observations
        .iter()
        .filter_map(|o| o.occurrence_key.as_deref().map(|k| (k, o)))
        .collect()
}

fn moves(
    before: &NativeScan,
    after: &NativeScan,
    old: &BTreeMap<&str, &Observation>,
    new: &BTreeMap<&str, &Observation>,
) -> Vec<MoveCandidate> {
    // Check shared descriptors once per asset, rather than copying potentially
    // large shared attributes into every observation's move signature.
    let new_assets: BTreeMap<_, _> = after.assets.iter().map(|a| (a.id.as_str(), a)).collect();
    let unchanged_content: BTreeSet<_> = before
        .assets
        .iter()
        .filter(|a| {
            a.kind != "usage"
                && asset_correspondence(a)
                && new_assets
                    .get(a.id.as_str())
                    .is_some_and(|b| asset_correspondence(b) && asset_facts(a) == asset_facts(b))
        })
        .map(|a| a.id.as_str())
        .collect();
    // Count every observation carrying a usage anchor before applying any
    // confidence filter. Otherwise discarded ambiguous duplicates can make a
    // remaining anchor appear unique. Missing/conflicting facts cannot qualify.
    fn candidates<'a>(
        n: &'a NativeScan,
        unchanged_content: &BTreeSet<&str>,
    ) -> BTreeMap<String, Vec<(&'a Observation, Option<Value>)>> {
        let assets: BTreeMap<_, _> = n.assets.iter().map(|a| (a.id.as_str(), a)).collect();
        let mut result: BTreeMap<String, Vec<(&Observation, Option<Value>)>> = BTreeMap::new();
        for o in &n.observations {
            let referenced: Vec<_> = o
                .asset_ids
                .iter()
                .filter_map(|id| assets.get(id.as_str()).copied())
                .collect();
            let usage_assets: Vec<_> = referenced
                .iter()
                .copied()
                .filter(|a| a.kind == "usage")
                .collect();
            let mut content: Vec<_> = referenced
                .iter()
                .filter(|a| a.kind != "usage")
                .map(|a| a.id.as_str())
                .collect();
            content.sort();
            let mut uncertainty = o.uncertainty.clone();
            uncertainty.sort();
            let signature = (usage_assets.len() == 1
                && observation_correspondence(o)
                && referenced.len() == o.asset_ids.len()
                && referenced.iter().all(|a| asset_correspondence(a))
                && content.iter().all(|id| unchanged_content.contains(id)))
            .then(|| {
                json!([
                    content,
                    o.method,
                    o.producer,
                    o.assurance,
                    o.usage,
                    o.rule_id,
                    o.evidence,
                    uncertainty
                ])
            });
            for usage in usage_assets {
                if let Some(anchor) = usage.attributes.get("anchor").and_then(Value::as_str) {
                    result
                        .entry(anchor.to_owned())
                        .or_default()
                        .push((o, signature.clone()));
                }
            }
        }
        result
    }
    let a = candidates(before, &unchanged_content);
    let b = candidates(after, &unchanged_content);
    let mut result = Vec::new();
    for (anchor, left) in a {
        let Some(right) = b.get(&anchor) else {
            continue;
        };
        if left.len() != 1 || right.len() != 1 {
            continue;
        }
        let ((before_observation, before_signature), (after_observation, after_signature)) =
            (&left[0], &right[0]);
        if before_signature.is_some()
            && before_signature == after_signature
            && before_observation.target.path != after_observation.target.path
            && !new.contains_key(before_observation.occurrence_key.as_deref().unwrap_or(""))
            && !old.contains_key(after_observation.occurrence_key.as_deref().unwrap_or(""))
        {
            result.push(MoveCandidate {
                before_occurrence_key: before_observation
                    .occurrence_key
                    .clone()
                    .unwrap_or_default(),
                after_occurrence_key: after_observation.occurrence_key.clone().unwrap_or_default(),
                confidence: "heuristic_only".into(),
                reason: "unique_semantic_anchor_with_changed_path_no_identity_merge".into(),
            });
        }
    }
    result.sort_by(|a, b| {
        (&a.before_occurrence_key, &a.after_occurrence_key)
            .cmp(&(&b.before_occurrence_key, &b.after_occurrence_key))
    });
    result
}
