//! Pure native inventory builder, with no legacy normalization replacement.
use crate::compat::{associate, losses, LegacyAssociation, LegacyLossRecord};
use crate::identity::{native_id, relative_path, valid_scope};
use crate::observation::{AlgorithmSpec, Observation, ObservationDraft};
use crate::{Finding, RuleSet};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Default)]
pub struct NativeOptions {
    repository_scope: Option<String>,
}
impl NativeOptions {
    pub fn new(scope: Option<&str>) -> anyhow::Result<Self> {
        if scope.is_some_and(|s| !valid_scope(s)) {
            anyhow::bail!("repository scope must be a non-secret relative project identifier");
        }
        Ok(Self {
            repository_scope: scope.map(str::to_string),
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoAsset {
    pub(crate) id: String,
    pub(crate) kind: String,
    pub(crate) identity_basis: String,
    pub(crate) attributes: Value,
    pub(crate) relations: Vec<AssetRelation>,
    pub(crate) uncertainty: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRelation {
    pub(crate) relationship: String,
    pub(crate) target: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeRun {
    pub(crate) id: String,
    pub(crate) attempt_id: String,
    pub(crate) generated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeScan {
    pub(crate) schema_version: u32,
    pub(crate) identity_scheme: String,
    pub(crate) scanner_version: String,
    pub(crate) catalog_version: String,
    pub(crate) profile: String,
    pub(crate) rules_digest: String,
    pub(crate) run: NativeRun,
    pub(crate) repository_scope: Option<String>,
    pub(crate) scope: Value,
    pub(crate) coverage: Value,
    pub(crate) diagnostics: Vec<Value>,
    pub(crate) native_completeness: String,
    pub(crate) assets: Vec<CryptoAsset>,
    pub(crate) observations: Vec<Observation>,
    pub(crate) legacy_associations: Vec<LegacyAssociation>,
    pub(crate) unassociated_observation_ids: Vec<String>,
    pub(crate) legacy_loss_records: Vec<LegacyLossRecord>,
    pub(crate) assessment_status: String,
    pub(crate) assessments: Vec<Value>,
    pub(crate) export_loss_notes: Vec<String>,
}
impl NativeScan {
    pub fn run_id(&self) -> &str {
        &self.run.id
    }
    /// Validate v1 reference integrity while allowing future string vocabulary and fields.
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.schema_version == 1,
            "unsupported native schema major version"
        );
        let assets: BTreeSet<_> = self.assets.iter().map(|a| a.id.as_str()).collect();
        let observations: BTreeSet<_> = self.observations.iter().map(|o| o.id.as_str()).collect();
        anyhow::ensure!(
            assets.len() == self.assets.len() && observations.len() == self.observations.len(),
            "duplicate native identity"
        );
        for a in &self.assets {
            for r in &a.relations {
                anyhow::ensure!(
                    assets.contains(r.target.as_str()),
                    "unresolved asset relation"
                );
            }
        }
        for o in &self.observations {
            for a in &o.asset_ids {
                anyhow::ensure!(assets.contains(a.as_str()), "unresolved observation asset");
            }
        }
        for (i, l) in self.legacy_associations.iter().enumerate() {
            anyhow::ensure!(l.legacy_index == i, "invalid legacy index");
            for id in &l.observation_ids {
                anyhow::ensure!(
                    observations.contains(id.as_str()),
                    "unresolved legacy observation"
                );
            }
        }
        for id in &self.unassociated_observation_ids {
            anyhow::ensure!(
                observations.contains(id.as_str()),
                "unresolved unassociated observation"
            );
        }
        for l in &self.legacy_loss_records {
            anyhow::ensure!(
                observations.contains(l.observation_id.as_str()) && l.count > 0,
                "invalid loss record"
            );
        }
        Ok(())
    }
    pub fn observations(&self) -> &[Observation] {
        &self.observations
    }
    pub fn assets(&self) -> &[CryptoAsset] {
        &self.assets
    }
    pub fn legacy_associations(&self) -> &[LegacyAssociation] {
        &self.legacy_associations
    }
    pub(crate) fn build(
        root: &Path,
        rules: &RuleSet,
        findings: &[Finding],
        drafts: Vec<ObservationDraft>,
        options: &NativeOptions,
        diagnostics: Value,
        generated_at: &str,
    ) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let run = native_id(
            "scan",
            &[
                generated_at,
                &std::process::id().to_string(),
                &NEXT.fetch_add(1, Ordering::Relaxed).to_string(),
            ],
        );
        let known_rules = rules.all().iter().map(|r| r.id.clone()).collect();
        let mut value =
            Self::build_internal(root, findings, drafts, options, &run, Some(&known_rules));
        value.run.generated_at = generated_at.into();
        value.rules_digest = native_id(
            "rules",
            &[&serde_json::to_string(rules.all()).expect("rules serialize")],
        );
        value.scope = diagnostics["scope"].clone();
        value.coverage = diagnostics["coverage"].clone();
        value.diagnostics = diagnostics["diagnostics"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|mut d| {
                if d["detector"]
                    .as_str()
                    .is_some_and(|s| !crate::identity::safe_token(s))
                {
                    d["detector"] = json!("unavailable");
                }
                d
            })
            .collect();
        value
    }
    #[cfg(test)]
    pub(crate) fn build_for_run(
        root: &Path,
        findings: &[Finding],
        drafts: Vec<ObservationDraft>,
        options: &NativeOptions,
        run: &str,
    ) -> Self {
        Self::build_internal(root, findings, drafts, options, run, None)
    }
    fn build_internal(
        root: &Path,
        findings: &[Finding],
        mut drafts: Vec<ObservationDraft>,
        options: &NativeOptions,
        run: &str,
        known_rules: Option<&BTreeSet<String>>,
    ) -> Self {
        for draft in &mut drafts {
            draft.target.path = relative_path(root, &draft.raw_path);
            if draft.certificate.as_ref().is_some_and(|c| !c.valid()) {
                draft.certificate = None;
                draft.assurance = "unknown".into();
                draft.evidence.capture_status = "invalid_typed_facts_omitted".into();
            }
        }
        // Physical positions are order/tie-breakers, never structural identity input.
        drafts.sort_by_key(|d| {
            (
                d.target.path.clone(),
                d.method.clone(),
                d.rule_id.clone(),
                d.anchor(),
                d.fact_order(),
                d.target.byte_start,
                d.target.byte_end,
                d.target.line,
                d.target.column,
                d.target.object_index,
            )
        });
        let mut counts = BTreeMap::new();
        for d in &drafts {
            *counts
                .entry((d.target.path.clone(), d.anchor()))
                .or_insert(0usize) += 1;
        }
        let mut ordinals = BTreeMap::new();
        let mut assets = BTreeMap::new();
        let mut observations = Vec::new();
        let mut bindings = Vec::new();
        let mut capture_reasons = BTreeMap::new();
        for draft in drafts {
            let anchor = draft.anchor();
            let group = (draft.target.path.clone(), anchor.clone());
            let ordinal = ordinals.entry(group.clone()).or_insert(0usize);
            let local = *ordinal;
            *ordinal += 1;
            let duplicate = counts[&group] > 1;
            let occurrence = options
                .repository_scope
                .as_deref()
                .zip(draft.target.path.as_deref())
                .map(|(scope, path)| {
                    native_id("occurrence", &[scope, path, &anchor, &local.to_string()])
                });
            let id = native_id(
                "observation",
                &[
                    run,
                    "static-1",
                    &anchor,
                    &draft.target.path.clone().unwrap_or_default(),
                    &local.to_string(),
                ],
            );
            let mut refs = Vec::new();
            let mut uncertainty = Vec::new();
            if draft.target.path.is_none() {
                uncertainty.push("unsafe_or_outside_root_target_omitted".into());
            }
            if duplicate {
                uncertainty.push("duplicate_anchor_ordinals_can_shift".into());
            }
            for algorithm in &draft.algorithms {
                refs.push(insert_algorithm(&mut assets, algorithm));
            }
            if let Some(cert) = &draft.certificate {
                let sig = AlgorithmSpec::oid(
                    &cert.signature_oid,
                    "certificate_signature",
                    BTreeMap::from([("parameter_interpretation".into(), "not_available".into())]),
                );
                let mut params =
                    BTreeMap::from([("parameters_kind".into(), cert.parameters_kind.clone())]);
                if let Some(curve) = &cert.named_curve_oid {
                    params.insert("named_curve_oid".into(), curve.clone());
                }
                if cert.parameters_kind == "sequence" || cert.parameters_kind == "unknown" {
                    params.insert("parameter_interpretation".into(), "not_available".into());
                }
                let key_alg = AlgorithmSpec::oid(&cert.public_key_oid, "public_key", params);
                let sig_id = insert_algorithm(&mut assets, &sig);
                let key_alg_id = insert_algorithm(&mut assets, &key_alg);
                let key_id = native_id("asset", &["public_key", &cert.spki_sha256]);
                insert_asset(
                    &mut assets,
                    CryptoAsset {
                        id: key_id.clone(),
                        kind: "public_key".into(),
                        identity_basis: "public_spki_sha256".into(),
                        attributes: json!({"spki_sha256":cert.spki_sha256,"key_bits":cert.key_bits,"named_curve_oid":cert.named_curve_oid}),
                        relations: vec![relation("algorithm", &key_alg_id)],
                        uncertainty: vec!["public_key_not_cryptographically_validated".into()],
                    },
                );
                let cert_id = native_id("asset", &["certificate", &cert.certificate_sha256]);
                insert_asset(
                    &mut assets,
                    CryptoAsset {
                        id: cert_id.clone(),
                        kind: "certificate".into(),
                        identity_basis: "public_certificate_der_sha256".into(),
                        attributes: json!({"certificate_sha256":cert.certificate_sha256,"not_before_unix":cert.not_before_unix,"not_after_unix":cert.not_after_unix}),
                        relations: vec![
                            relation("subject_key", &key_id),
                            relation("signature_algorithm", &sig_id),
                        ],
                        uncertainty: vec!["trust_and_signature_not_verified".into()],
                    },
                );
                refs.extend([sig_id, key_alg_id, key_id, cert_id]);
            }
            if let Some(dep) = &draft.dependency {
                let dep_id = if draft.typed_dependency {
                    native_id(
                        "asset",
                        &[
                            "dependency_assertion_v1",
                            options.repository_scope.as_deref().unwrap_or(run),
                            draft.target.path.as_deref().unwrap_or("unavailable"),
                            &draft.target.object_index.unwrap_or(0).to_string(),
                            &serde_json::to_string(dep).expect("typed dependency"),
                        ],
                    )
                } else {
                    native_id(
                        "asset",
                        &[
                            "dependency_report",
                            &dep.ecosystem,
                            &dep.name,
                            dep.reported_version.as_deref().unwrap_or(""),
                        ],
                    )
                };
                insert_asset(
                    &mut assets,
                    CryptoAsset {
                        id: dep_id.clone(),
                        kind: "dependency".into(),
                        identity_basis: if draft.typed_dependency {
                            "scoped_source_assertion_not_installed_package"
                        } else {
                            "reported_package_identity_not_runtime_use"
                        }
                        .into(),
                        attributes: serde_json::to_value(dep).expect("typed dependency"),
                        relations: Vec::new(),
                        uncertainty: if draft.typed_dependency {
                            vec!["source_ordinal_can_shift_no_inferred_dependency_graph".into()]
                        } else {
                            vec!["declared_vs_resolved_version_unavailable".into()]
                        },
                    },
                );
                refs.push(dep_id);
            }
            let usage_id = occurrence
                .as_ref()
                .map(|key| native_id("asset", &["usage", key]))
                .unwrap_or_else(|| native_id("asset", &["run_local_usage", run, &id]));
            insert_asset(
                &mut assets,
                CryptoAsset {
                    id: usage_id.clone(),
                    kind: "usage".into(),
                    identity_basis: if occurrence.is_some() {
                        "scoped_weak_source_anchor"
                    } else {
                        "run_local_only"
                    }
                    .into(),
                    attributes: json!({"path":draft.target.path,"method":draft.method,"local_ordinal":local,"ambiguous":duplicate,"anchor":anchor}),
                    relations: refs.iter().map(|a| relation("references", a)).collect(),
                    uncertainty: vec!["static_reference_not_active_use".into()],
                },
            );
            refs.push(usage_id);
            let confidence = if duplicate {
                "ambiguous_duplicate_anchor"
            } else if occurrence.is_some() {
                "weak_structural_anchor"
            } else {
                "run_local_only"
            };
            capture_reasons.insert(
                id.clone(),
                if draft.certificate.is_some() {
                    "certificate_parse_without_rule_finding"
                } else {
                    "container_capture_without_rule_finding"
                }
                .into(),
            );
            bindings.push((id.clone(), draft.bindings));
            observations.push(Observation {
                id,
                occurrence_key: occurrence,
                identity_confidence: confidence.into(),
                asset_ids: refs,
                target: draft.target,
                producer: draft.producer,
                method: draft.method,
                assurance: draft.assurance,
                usage: draft.usage,
                rule_id: draft.rule_id,
                evidence: draft.evidence,
                uncertainty,
            });
        }
        let links = associate(root, findings, &bindings);
        let legacy_loss_records = losses(root, findings, &bindings, &capture_reasons, known_rules);
        let linked: BTreeSet<_> = links
            .iter()
            .flat_map(|l| l.observation_ids.iter().cloned())
            .collect();
        let unassociated = observations
            .iter()
            .filter(|o| !linked.contains(&o.id))
            .map(|o| o.id.clone())
            .collect();
        let mut assets: Vec<_> = assets.into_values().collect();
        assets.sort_by_key(|a| (a.kind.clone(), a.attributes.to_string()));
        Self {
            schema_version: 1,
            identity_scheme: "pqc-scan-length-prefixed-sha256-v1".into(),
            scanner_version: env!("CARGO_PKG_VERSION").into(),
            catalog_version: "native-exact-identifiers-1".into(),
            profile: "static-native-v1".into(),
            rules_digest: String::new(),
            run: NativeRun {
                id: run.into(),
                attempt_id: "static-1".into(),
                generated_at: String::new(),
            },
            repository_scope: options.repository_scope.clone(),
            scope: json!({}),
            coverage: json!({}),
            diagnostics: Vec::new(),
            native_completeness: "not_exhaustive".into(),
            assets,
            observations,
            legacy_associations: links,
            unassociated_observation_ids: unassociated,
            legacy_loss_records,
            assessment_status: "not_evaluated".into(),
            assessments: Vec::new(),
            export_loss_notes: vec![
                "raw_matches_snippets_and_arbitrary_metadata_omitted".into(),
                "unknown_raw_identifiers_not_guessed".into(),
                "legacy_snapshot_not_derived_from_inventory".into(),
                "legacy_ids_are_not_native_identity".into(),
                "legacy_dependency_version_provenance_unavailable".into(),
                "legacy_applicability_coverage_is_not_native_inventory_completeness".into(),
                "weak_anchors_and_duplicate_ordinals_do_not_prove_cross_scan_identity".into(),
                "cryptographic_trust_and_policy_not_evaluated".into(),
            ],
        }
    }
}
fn relation(kind: &str, target: &str) -> AssetRelation {
    AssetRelation {
        relationship: kind.into(),
        target: target.into(),
    }
}
fn insert_algorithm(assets: &mut BTreeMap<String, CryptoAsset>, spec: &AlgorithmSpec) -> String {
    let id = native_id("asset", &["algorithm", &spec.key()]);
    let unknown = spec
        .parameters
        .get("parameter_interpretation")
        .is_some_and(|v| v == "not_available")
        || ["ML-KEM", "ML-DSA", "SLH-DSA", "RSA"].contains(&spec.identifier.as_str());
    insert_asset(
        assets,
        CryptoAsset {
            id: id.clone(),
            kind: "algorithm".into(),
            identity_basis: if unknown {
                "identifier_only_parameters_not_fully_interpreted"
            } else {
                "normalized_identifier_and_reported_parameters"
            }
            .into(),
            attributes: serde_json::to_value(spec).expect("typed algorithm"),
            relations: Vec::new(),
            uncertainty: if unknown {
                vec!["algorithm_parameters_not_fully_interpreted".into()]
            } else {
                Vec::new()
            },
        },
    );
    id
}

fn insert_asset(assets: &mut BTreeMap<String, CryptoAsset>, asset: CryptoAsset) {
    if let Some(existing) = assets.get_mut(&asset.id) {
        let conflict = existing.attributes != asset.attributes
            || serde_json::to_value(&existing.relations).ok()
                != serde_json::to_value(&asset.relations).ok();
        if conflict
            && !existing
                .uncertainty
                .iter()
                .any(|x| x == "conflicting_descriptors")
        {
            existing.uncertainty.push("conflicting_descriptors".into());
        }
    } else {
        assets.insert(asset.id.clone(), asset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observation::{ObservationMethod, PublicCertificateFacts};
    use crate::{Detection, Evidence, Finding, Location};
    use pqc_scan_rules::{Risk, Severity};
    pub(super) fn hit(path: &str, line: usize, raw: &str) -> Detection {
        Detection {
            rule_id: "TEST".into(),
            location: Location {
                file: path.into(),
                line,
                column: 1,
            },
            evidence: Evidence {
                r#type: "test".into(),
                r#match: raw.into(),
                snippet_preview: "NOT_EXPORTED_CANARY".into(),
                metadata: BTreeMap::from([("untrusted".into(), "NOT_EXPORTED_CANARY".into())]),
            },
        }
    }
    fn finding(d: &Detection) -> Finding {
        Finding {
            finding_id: "a".repeat(24),
            rule_id: d.rule_id.clone(),
            category: "test".into(),
            risk: Risk::QuantumUncertain,
            severity: Severity::Medium,
            confidence: 0.5,
            description: String::new(),
            migration_hint: String::new(),
            location: Location {
                file: "app.rs".into(),
                line: d.location.line,
                column: 1,
            },
            evidence: d.evidence.clone(),
            recommended_actions: Vec::new(),
            source_snippet: None,
        }
    }
    pub(super) fn build(
        root: &str,
        path: &str,
        line: usize,
        run: &str,
        scope: Option<&str>,
        copies: usize,
    ) -> NativeScan {
        let d = hit(path, line, "ML-KEM-768");
        let drafts = (0..copies)
            .map(|_| {
                ObservationDraft::source(&d, ObservationMethod::AstPattern, "ML-KEM-768", None)
            })
            .collect();
        NativeScan::build_for_run(
            Path::new(root),
            &[finding(&d)],
            drafts,
            &NativeOptions::new(scope).unwrap(),
            run,
        )
    }
    #[test]
    fn roots_and_line_insertions_preserve_scoped_occurrences_but_moves_change_them() {
        let a = build("/one", "/one/app.rs", 1, "run-a", Some("project"), 1);
        let b = build("/two", "/two/app.rs", 7, "run-b", Some("project"), 1);
        assert_eq!(
            a.observations[0].occurrence_key,
            b.observations[0].occurrence_key
        );
        assert_ne!(a.observations[0].id, b.observations[0].id);
        let moved = build("/two", "/two/moved.rs", 7, "run-b", Some("project"), 1);
        assert_ne!(
            a.observations[0].occurrence_key,
            moved.observations[0].occurrence_key
        );
        let foreign = build("/one", "/one/app.rs", 1, "run-a", Some("other"), 1);
        assert_ne!(
            a.observations[0].occurrence_key,
            foreign.observations[0].occurrence_key
        );
    }
    #[test]
    fn duplicate_members_are_all_ambiguous_and_scope_absence_never_implies_cross_run_usage() {
        let a = build("/one", "/one/app.rs", 1, "run-a", Some("project"), 2);
        assert_eq!(a.observations.len(), 2);
        assert!(a
            .observations
            .iter()
            .all(|o| o.identity_confidence == "ambiguous_duplicate_anchor"));
        assert_ne!(
            a.observations[0].occurrence_key,
            a.observations[1].occurrence_key
        );
        assert_eq!(a.assets.iter().filter(|a| a.kind == "algorithm").count(), 1);
        let b = build("/one", "/one/app.rs", 1, "run-a", None, 1);
        let c = build("/one", "/one/app.rs", 1, "run-b", None, 1);
        assert!(b.observations[0].occurrence_key.is_none());
        assert_ne!(
            b.assets.iter().find(|a| a.kind == "usage").unwrap().id,
            c.assets.iter().find(|a| a.kind == "usage").unwrap().id
        );
    }
    #[test]
    fn distinct_methods_survive_and_colliding_legacy_ids_keep_every_index() {
        let d = hit("/one/app.rs", 1, "ML-KEM-768");
        let drafts = vec![
            ObservationDraft::source(&d, ObservationMethod::AstPattern, "ML-KEM-768", None),
            ObservationDraft::source(&d, ObservationMethod::LexicalMatch, "ML-KEM-768", None),
        ];
        let scan = NativeScan::build_for_run(
            Path::new("/one"),
            &[finding(&d), finding(&d)],
            drafts,
            &NativeOptions::default(),
            "run",
        );
        assert_eq!(scan.observations.len(), 2);
        assert_eq!(scan.legacy_associations.len(), 2);
        for (i, a) in scan.legacy_associations.iter().enumerate() {
            assert_eq!(a.legacy_index, i);
            assert_eq!(a.legacy_id, "a".repeat(24));
            assert_eq!(a.observation_ids.len(), 2);
        }
    }
    #[test]
    fn raw_private_material_metadata_and_unsafe_paths_never_reach_new_evidence_or_identity() {
        let d = hit("/outside/SECRET_CANARY", 1, "PRIVATE_BODY_CANARY");
        let draft = ObservationDraft::legacy(&d);
        let scan = NativeScan::build_for_run(
            Path::new("/one"),
            &[],
            vec![draft],
            &NativeOptions::new(Some("project")).unwrap(),
            "run",
        );
        let json = serde_json::to_string(&scan).unwrap();
        assert!(!json.contains("CANARY"));
        assert_eq!(scan.observations[0].assurance, "unknown");
        assert_eq!(scan.observations[0].usage, "unknown");
        let d2 = hit("/elsewhere/DIFFERENT", 1, "DIFFERENT_SECRET");
        let other = NativeScan::build_for_run(
            Path::new("/one"),
            &[],
            vec![ObservationDraft::legacy(&d2)],
            &NativeOptions::new(Some("project")).unwrap(),
            "run",
        );
        assert_eq!(scan.observations[0].id, other.observations[0].id);
    }
    #[test]
    fn typed_public_certificates_remain_distinct_while_sharing_a_key() {
        let mut metadata = BTreeMap::from([
            ("certificate_sha256".into(), "a".repeat(64)),
            ("spki_sha256".into(), "b".repeat(64)),
            ("signature_oid".into(), "1.2.840.113549.1.1.11".into()),
            ("public_key_oid".into(), "1.2.840.10045.2.1".into()),
            ("not_before_unix".into(), "1".into()),
            ("not_after_unix".into(), "9".into()),
        ]);
        let a = PublicCertificateFacts::from_parser_metadata(&metadata).unwrap();
        metadata.insert("certificate_sha256".into(), "c".repeat(64));
        let b = PublicCertificateFacts::from_parser_metadata(&metadata).unwrap();
        let drafts = vec![
            ObservationDraft::certificate(
                "/one/a.der",
                1,
                1,
                1,
                a,
                crate::DetectorStatus::Complete,
                &[],
            ),
            ObservationDraft::certificate(
                "/one/b.der",
                1,
                1,
                1,
                b,
                crate::DetectorStatus::Complete,
                &[],
            ),
        ];
        let scan = NativeScan::build_for_run(
            Path::new("/one"),
            &[],
            drafts,
            &NativeOptions::default(),
            "run",
        );
        assert_eq!(
            scan.assets
                .iter()
                .filter(|a| a.kind == "certificate")
                .count(),
            2
        );
        assert_eq!(
            scan.assets
                .iter()
                .filter(|a| a.kind == "public_key")
                .count(),
            1
        );
        assert_eq!(
            scan.assets.iter().filter(|a| a.kind == "algorithm").count(),
            2
        );
        let ids: BTreeSet<_> = scan.assets.iter().map(|a| a.id.as_str()).collect();
        assert!(scan
            .assets
            .iter()
            .all(|a| a.relations.iter().all(|r| ids.contains(r.target.as_str()))));
    }
    #[test]
    fn unsafe_dependency_credentials_are_omitted_before_hashing() {
        let mut d = hit("/one/Cargo.toml", 1, "unused");
        d.evidence
            .metadata
            .insert("dep_name".into(), "alice:SECRET_CANARY@host/pkg".into());
        let scan = NativeScan::build_for_run(
            Path::new("/one"),
            &[],
            vec![ObservationDraft::dependency(&d)],
            &NativeOptions::default(),
            "run",
        );
        assert!(!serde_json::to_string(&scan).unwrap().contains("CANARY"));
    }
    #[test]
    fn uninterpreted_parameter_digests_do_not_create_exact_algorithm_variants() {
        let mut m = BTreeMap::from([
            ("certificate_sha256".into(), "a".repeat(64)),
            ("spki_sha256".into(), "b".repeat(64)),
            ("signature_oid".into(), "1.2.840.113549.1.1.10".into()),
            ("public_key_oid".into(), "1.2.840.113549.1.1.10".into()),
            ("parameters_kind".into(), "sequence".into()),
            ("parameters_content_sha256".into(), "d".repeat(64)),
            ("not_before_unix".into(), "1".into()),
            ("not_after_unix".into(), "9".into()),
        ]);
        let a = PublicCertificateFacts::from_parser_metadata(&m).unwrap();
        m.insert("certificate_sha256".into(), "e".repeat(64));
        m.insert("spki_sha256".into(), "f".repeat(64));
        m.insert("parameters_content_sha256".into(), "c".repeat(64));
        let b = PublicCertificateFacts::from_parser_metadata(&m).unwrap();
        let scan = NativeScan::build_for_run(
            Path::new("/one"),
            &[],
            vec![
                ObservationDraft::certificate(
                    "/one/a.der",
                    1,
                    1,
                    1,
                    a,
                    crate::DetectorStatus::Complete,
                    &[],
                ),
                ObservationDraft::certificate(
                    "/one/b.der",
                    1,
                    1,
                    1,
                    b,
                    crate::DetectorStatus::Complete,
                    &[],
                ),
            ],
            &NativeOptions::default(),
            "run",
        );
        assert_eq!(
            scan.assets.iter().filter(|a| a.kind == "algorithm").count(),
            2
        );
        assert!(scan
            .assets
            .iter()
            .filter(|a| a.kind == "algorithm")
            .all(|a| !a.uncertainty.is_empty()));
    }
}

#[cfg(test)]
mod contract_tests {
    use super::*;
    #[test]
    fn usage_order_is_independent_of_run_ids_even_for_same_target_and_method() {
        let a = super::tests::hit("/one/app.rs", 1, "ML-KEM-768");
        let mut b = a.clone();
        b.rule_id = "OTHER".into();
        let make = |run: &str| {
            NativeScan::build_for_run(
                Path::new("/one"),
                &[],
                vec![ObservationDraft::legacy(&a), ObservationDraft::legacy(&b)],
                &NativeOptions::default(),
                run,
            )
        };
        let first = make("a");
        let expected: Vec<_> = first
            .assets
            .iter()
            .map(|a| (&a.kind, &a.attributes))
            .collect();
        for run in ["b", "c", "d", "e", "f", "g", "h"] {
            let next = make(run);
            let actual: Vec<_> = next
                .assets
                .iter()
                .map(|a| (&a.kind, &a.attributes))
                .collect();
            assert_eq!(expected, actual);
        }
    }
    #[test]
    fn tolerant_reader_preserves_unknown_vocabulary_and_rejects_broken_references_or_major_version()
    {
        let scan = super::tests::build("/one", "/one/app.rs", 1, "run", Some("test"), 1);
        let mut value = serde_json::to_value(&scan).unwrap();
        value["future_field"] = json!({"ignored":true});
        value["observations"][0]["method"] = json!("future_method");
        value["assets"][0]["kind"] = json!("future_asset");
        let read: NativeScan = serde_json::from_value(value.clone()).unwrap();
        read.validate().unwrap();
        assert_eq!(
            serde_json::to_value(read).unwrap()["observations"][0]["method"],
            "future_method"
        );
        value["schema_version"] = json!(2);
        assert!(serde_json::from_value::<NativeScan>(value.clone())
            .unwrap()
            .validate()
            .is_err());
        value["schema_version"] = json!(1);
        value["observations"][0]["asset_ids"] = json!(["missing"]);
        assert!(serde_json::from_value::<NativeScan>(value)
            .unwrap()
            .validate()
            .is_err());
    }
}
