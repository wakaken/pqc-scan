//! Offline, explicitly selected local advisory policy. Operator claims are not telemetry.
use crate::assessment::{AlgorithmAssessment, Purpose, QuantumExposure};
use serde::{Deserialize, Serialize};

pub const PROFILE_ID: &str = "migration-review-v1";
pub const PROFILE_VERSION: &str = "1.0.0";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Pass,
    Fail,
    Indeterminate,
    NotApplicable,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DecisionRecord {
    pub decision: Decision,
    pub reasons: Vec<String>,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Applicability {
    #[default]
    Applicable,
    Unknown,
    NotApplicable,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UseCase {
    #[default]
    Unknown,
    KeyEstablishment,
    Jwt,
    Pki,
    Signature,
    Encryption,
    Hashing,
}
impl UseCase {
    fn purpose(self) -> Purpose {
        match self {
            Self::Unknown => Purpose::Unknown,
            Self::KeyEstablishment => Purpose::KeyEstablishment,
            Self::Jwt | Self::Pki | Self::Signature => Purpose::Signature,
            Self::Encryption => Purpose::Encryption,
            Self::Hashing => Purpose::Hash,
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Support {
    runtime: Option<bool>,
    provider: Option<bool>,
    peer: Option<bool>,
    protocol: Option<bool>,
    parameters: Option<bool>,
    jose: Option<bool>,
    verifiers: Option<bool>,
    pki: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContextInput {
    version: u32,
    #[serde(default)]
    applicability: Applicability,
    #[serde(default)]
    use_case: UseCase,
    #[serde(default)]
    support: Support,
    long_lived_confidentiality: Option<bool>,
    hash_collision_required: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
pub struct AssessmentContext {
    input: ContextInput,
    provenance: String,
}
impl Default for AssessmentContext {
    fn default() -> Self {
        Self {
            input: ContextInput {
                version: 1,
                applicability: Applicability::Applicable,
                use_case: UseCase::Unknown,
                support: Support::default(),
                long_lived_confidentiality: None,
                hash_collision_required: None,
            },
            provenance: "local_profile_default_unknown_context".into(),
        }
    }
}
impl AssessmentContext {
    pub fn from_json(input: &str) -> anyhow::Result<Self> {
        const ERROR: &str = "invalid assessment context: expected bounded v1 enum/boolean object";
        anyhow::ensure!(input.len() <= 16384, ERROR);
        let value: ContextInput =
            serde_json::from_str(input).map_err(|_| anyhow::anyhow!(ERROR))?;
        anyhow::ensure!(value.version == 1, ERROR);
        Ok(Self {
            input: value,
            provenance: "operator_supplied_unverified".into(),
        })
    }
    pub(crate) fn high_confidentiality_priority(&self) -> bool {
        self.input.long_lived_confidentiality == Some(true)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Prerequisite {
    pub name: String,
    pub claimed_support: Option<bool>,
    pub provenance: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct MigrationCandidate {
    pub identifier: String,
    pub purpose: Purpose,
    pub status: String,
    pub parameter_selection: String,
    pub prerequisites: Vec<Prerequisite>,
    pub validation_steps: Vec<String>,
    pub source_ids: Vec<String>,
}
fn prerequisites(purpose: Purpose, c: &AssessmentContext) -> Vec<Prerequisite> {
    let relevant = c.input.use_case.purpose() == purpose && purpose != Purpose::Unknown;
    let s = &c.input.support;
    let mut values = vec![
        ("runtime_api", s.runtime),
        ("provider_implementation", s.provider),
        ("parameter_agreement", s.parameters),
    ];
    if matches!(purpose, Purpose::KeyEstablishment | Purpose::Signature) {
        values.push(("peer_capability", s.peer));
    }
    if purpose != Purpose::Hash {
        values.push(("protocol_binding", s.protocol));
    }
    if c.input.use_case == UseCase::Jwt && purpose == Purpose::Signature {
        values.extend([
            ("jose_identifier_and_encoding", s.jose),
            ("verifier_rollout", s.verifiers),
        ]);
    }
    if c.input.use_case == UseCase::Pki && purpose == Purpose::Signature {
        values.extend([
            ("certificate_algorithm_binding", s.pki),
            ("issuer_and_trust_store_support", s.pki),
        ]);
    }
    values
        .into_iter()
        .map(|(name, value)| Prerequisite {
            name: name.into(),
            claimed_support: if relevant { value } else { None },
            provenance: if relevant {
                c.provenance.clone()
            } else {
                "not_applicable_to_declared_use_case".into()
            },
        })
        .collect()
}
pub fn candidates(a: &AlgorithmAssessment, c: &AssessmentContext) -> Vec<MigrationCandidate> {
    if a.identifier == "unknown" {
        return vec![];
    }
    let desired = c.input.use_case.purpose();
    if desired != Purpose::Unknown && a.purpose != Purpose::Unknown && desired != a.purpose {
        return vec![];
    }
    let purposes = if a.purpose == Purpose::Unknown {
        if matches!(desired, Purpose::Signature | Purpose::KeyEstablishment) {
            vec![desired]
        } else if desired == Purpose::Unknown && a.quantum == QuantumExposure::Vulnerable {
            vec![Purpose::KeyEstablishment, Purpose::Signature]
        } else {
            vec![]
        }
    } else {
        vec![a.purpose]
    };
    purposes
        .into_iter()
        .filter_map(|purpose| {
            let (id, source) = match purpose {
                Purpose::KeyEstablishment => ("ML-KEM", "S203"),
                Purpose::Signature => ("ML-DSA", "S204"),
                Purpose::Hash => ("SHA-256", "S131A2"),
                Purpose::Encryption => ("AES", "NIST-PQ-FAQ"),
                Purpose::Unknown => return None,
            };
            Some(MigrationCandidate {
                identifier: id.into(),
                purpose,
                status: "candidate_primitive_integration_unverified".into(),
                parameter_selection: "explicit_parameter_selection_required".into(),
                prerequisites: prerequisites(purpose, c),
                validation_steps: vec![
                    "confirm_actual_use_and_required_purpose".into(),
                    "verify_exact_parameters_and_protocol_encoding".into(),
                    "test_runtime_provider_peer_and_rollout".into(),
                    "validate_implementation_and_interoperability_separately".into(),
                ],
                source_ids: if c.input.use_case == UseCase::Jwt && purpose == Purpose::Signature {
                    vec![source.into(), "R8725".into(), "LOCAL026".into()]
                } else {
                    vec![source.into(), "LOCAL026".into()]
                },
            })
        })
        .collect()
}
fn record(decision: Decision, reasons: &[&str]) -> DecisionRecord {
    DecisionRecord {
        decision,
        reasons: reasons.iter().map(|s| (*s).into()).collect(),
    }
}
pub fn assess_primitive_policy(a: &AlgorithmAssessment, c: &AssessmentContext) -> DecisionRecord {
    match c.input.applicability {
        Applicability::NotApplicable => {
            return record(
                Decision::NotApplicable,
                &["operator_excluded_local_profile"],
            )
        }
        Applicability::Unknown => {
            return record(Decision::Indeterminate, &["profile_applicability_unknown"])
        }
        _ => {}
    }
    let known_classical = a
        .classical_concerns
        .iter()
        .any(|s| s != "weak_collision_resistance" && s != "certificate_validity_unknown")
        || (a
            .classical_concerns
            .iter()
            .any(|s| s == "weak_collision_resistance")
            && (c.input.hash_collision_required == Some(true)
                || matches!(
                    c.input.use_case,
                    UseCase::Jwt | UseCase::Pki | UseCase::Signature
                )));
    if a.quantum == QuantumExposure::Vulnerable || known_classical {
        let mut reasons = vec![];
        if a.quantum == QuantumExposure::Vulnerable {
            reasons.push("observed_primitive_quantum_vulnerable");
        }
        if known_classical {
            reasons.push("known_classical_concern_in_profile_scope");
        }
        return record(Decision::Fail, &reasons);
    }
    if a.quantum == QuantumExposure::Unknown || !a.parameters_known {
        return record(
            Decision::Indeterminate,
            &["algorithm_or_parameters_unresolved"],
        );
    }
    if !a.classical_concerns.is_empty() {
        return record(
            Decision::Indeterminate,
            &["classical_concern_context_unresolved"],
        );
    }
    if a.purpose == Purpose::Unknown || c.input.use_case.purpose() != a.purpose {
        return record(
            Decision::Indeterminate,
            &["use_case_unknown_or_purpose_mismatch"],
        );
    }
    if prerequisites(a.purpose, c)
        .iter()
        .any(|p| p.claimed_support != Some(true))
    {
        return record(
            Decision::Indeterminate,
            &["required_support_not_declared_true"],
        );
    }
    record(
        Decision::Pass,
        &["bounded_primitive_review_only_operator_claims_unverified"],
    )
}
/// Aggregate Pass requires positive relevant results and both completeness conditions.
/// Current inventory v1 is never exhaustive; its report evaluator always passes false.
pub fn summarize(
    decisions: &[Decision],
    inspection_complete: bool,
    inventory_exhaustive: bool,
    c: &AssessmentContext,
) -> DecisionRecord {
    match c.input.applicability {
        Applicability::NotApplicable => {
            return record(
                Decision::NotApplicable,
                &["operator_excluded_local_profile"],
            )
        }
        Applicability::Unknown => {
            return record(Decision::Indeterminate, &["profile_applicability_unknown"])
        }
        _ => {}
    }
    let mut reasons = vec![];
    if !inspection_complete {
        reasons.push("inspection_incomplete_or_unknown");
    }
    if !inventory_exhaustive {
        reasons.push("inventory_not_exhaustive");
    }
    let decision = if decisions.contains(&Decision::Fail) {
        reasons.push("applicable_asset_violation");
        Decision::Fail
    } else if !decisions.contains(&Decision::Pass)
        || decisions.contains(&Decision::Indeterminate)
        || !reasons.is_empty()
    {
        reasons.push("no_complete_positive_policy_evidence");
        Decision::Indeterminate
    } else {
        reasons.push("bounded_scope_checks_passed_not_deployment_assurance");
        Decision::Pass
    };
    record(decision, &reasons)
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetPolicy {
    pub asset_id: String,
    pub decision: Decision,
    pub reasons: Vec<String>,
    pub priority: String,
    pub candidates: Vec<MigrationCandidate>,
}
#[derive(Debug, Clone, Serialize)]
pub struct ProfileDescription {
    id: String,
    version: String,
    scope: String,
    source_editions: Vec<String>,
    effective_dates: Option<String>,
    deadline_policy: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct InventoryReference {
    file: String,
    run_id: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct AssessmentReport {
    schema_version: u32,
    catalog_version: String,
    profile: ProfileDescription,
    inventory: InventoryReference,
    context: AssessmentContext,
    assessments: Vec<crate::assessment::AssetAssessment>,
    asset_policies: Vec<AssetPolicy>,
    evaluation: DecisionRecord,
    inspection_status: String,
    inventory_completeness: String,
}
impl AssessmentReport {
    /// Bind advisory decisions to the actual inventory facts, not only its run ID.
    pub fn validate_for_inventory(
        &self,
        native: &crate::inventory::NativeScan,
    ) -> anyhow::Result<()> {
        self.validate()?;
        let expected = evaluate(native, &self.context, &self.profile.id)?;
        anyhow::ensure!(
            serde_json::to_value(self)? == serde_json::to_value(expected)?,
            "assessment does not match inventory facts"
        );
        Ok(())
    }
    pub fn profile_id(&self) -> &str {
        &self.profile.id
    }
    pub fn inventory_run_id(&self) -> &str {
        &self.inventory.run_id
    }
    pub fn evaluation(&self) -> &DecisionRecord {
        &self.evaluation
    }
    pub fn assessments(&self) -> &[crate::assessment::AssetAssessment] {
        &self.assessments
    }
    pub fn asset_policies(&self) -> &[AssetPolicy] {
        &self.asset_policies
    }
    pub fn decision(&self) -> Decision {
        self.evaluation.decision
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        use std::collections::BTreeSet;
        anyhow::ensure!(
            self.schema_version == 1 && crate::assessment::valid_id(&self.inventory.run_id, "scan"),
            "invalid assessment report version or scan"
        );
        let mut assets = BTreeSet::new();
        let mut ids = BTreeSet::new();
        for a in &self.assessments {
            anyhow::ensure!(
                crate::assessment::valid_id(&a.id, "assessment")
                    && crate::assessment::valid_id(&a.asset_id, "asset")
                    && assets.insert(&a.asset_id)
                    && ids.insert(&a.id),
                "invalid assessment references"
            );
        }
        anyhow::ensure!(
            self.asset_policies.len() == assets.len(),
            "incomplete policy references"
        );
        let mut seen = BTreeSet::new();
        for p in &self.asset_policies {
            anyhow::ensure!(
                assets.contains(&p.asset_id) && seen.insert(&p.asset_id),
                "unresolved policy reference"
            );
        }
        anyhow::ensure!(
            self.evaluation.decision != Decision::Pass,
            "inventory v1 cannot yield aggregate pass"
        );
        Ok(())
    }
}
/// Evaluate one immutable inventory. No report writer, rule mutation or hidden clock.
pub fn evaluate(
    native: &crate::inventory::NativeScan,
    context: &AssessmentContext,
    profile: &str,
) -> anyhow::Result<AssessmentReport> {
    anyhow::ensure!(profile == PROFILE_ID, "unknown local policy profile");
    let assessments = crate::assessment::evaluate_inventory(native)?;
    let asset_policies: Vec<_> = assessments
        .iter()
        .map(|a| {
            let mut p = if ["usage", "dependency"].contains(&a.asset_kind.as_str()) {
                record(Decision::NotApplicable, &["not_a_primitive_policy_target"])
            } else {
                assess_primitive_policy(&a.algorithm, context)
            };
            if p.decision == Decision::Pass && a.certainty == "unknown" {
                p = record(
                    Decision::Indeterminate,
                    &["observation_assurance_unavailable"],
                );
            }
            let high = !a.algorithm.classical_concerns.is_empty()
                || (a.algorithm.quantum == QuantumExposure::Vulnerable
                    && context.high_confidentiality_priority());
            AssetPolicy {
                asset_id: a.asset_id.clone(),
                decision: p.decision,
                reasons: p.reasons,
                priority: if high {
                    "high"
                } else if a.algorithm.quantum == QuantumExposure::Vulnerable {
                    "medium"
                } else {
                    "review"
                }
                .into(),
                candidates: if ["usage", "dependency"].contains(&a.asset_kind.as_str()) {
                    vec![]
                } else {
                    candidates(&a.algorithm, context)
                },
            }
        })
        .collect();
    let complete = native.coverage["status"] == "complete_within_scope";
    let evaluation = summarize(
        &asset_policies
            .iter()
            .map(|p| p.decision)
            .collect::<Vec<_>>(),
        complete,
        false,
        context,
    );
    let report = AssessmentReport {
        schema_version: 1,
        catalog_version: crate::assessment::CATALOG_VERSION.into(),
        profile: ProfileDescription {
            id: PROFILE_ID.into(),
            version: PROFILE_VERSION.into(),
            scope: "local_advisory_candidate_review_not_compliance_or_deployment".into(),
            source_editions: vec![
                "LOCAL026:1.0.0".into(),
                "FIPS203:2024-08-13".into(),
                "FIPS204:2024-08-13".into(),
                "FIPS205:2024-08-13".into(),
                "SP800-131A:Rev2:2019".into(),
                "RFC8725:2020".into(),
                "RFC6151:2011".into(),
                "RFC4055:2005".into(),
                "NIST-PQ-FAQ:reviewed-2026-11-10".into(),
                "BHT1997:v1".into(),
                "CSOR:reviewed-2026-11-10".into(),
            ],
            effective_dates: None,
            deadline_policy: "no_universal_deadlines".into(),
        },
        inventory: InventoryReference {
            file: "scan.inventory.v1.json".into(),
            run_id: native.run.id.clone(),
        },
        context: context.clone(),
        assessments,
        asset_policies,
        evaluation,
        inspection_status: if complete {
            "complete_within_scope"
        } else {
            "incomplete_or_unknown"
        }
        .into(),
        inventory_completeness: "not_exhaustive".into(),
    };
    report.validate()?;
    Ok(report)
}
