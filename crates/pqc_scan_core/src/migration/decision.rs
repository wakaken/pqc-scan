//! Adopted local P1-P8 advisory table. Not a numeric model or practitioner truth.
use super::{
    bounded, source, BoundContext, ClaimFreshness, ContextReference, Error, InputFormat,
    InputReference, MigrationContext, ResolvedClaim, Result, SavedEvidence, Target, TargetContext,
    CATALOG_ID, CATALOG_VERSION,
};
use crate::{
    assessment::{AlgorithmAssessment, Purpose, QuantumExposure},
    identity::native_id,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const POLICY_ID: &str = "migration-decision-v1";
pub const POLICY_VERSION: &str = "1.0.0";
const MODEL_ID: &str = "decision_table";
const MODEL_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Urgency {
    Urgent,
    Plan,
    Monitor,
    Undetermined,
    NotApplicable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Information {
    Sufficient,
    Incomplete,
    Conflicting,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Readiness {
    Ready,
    Blocked,
    Unknown,
    NotApplicable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    Investigate,
    Plan,
    Unblock,
    Migrate,
    Monitor,
    None,
}
#[derive(Debug, Clone, Serialize)]
pub struct Action {
    pub kind: ActionKind,
    pub candidate_ids: Vec<String>,
    pub blocking_prerequisites: Vec<String>,
    pub validation_steps: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Decision {
    pub id: String,
    pub target_id: String,
    pub system_id: Option<String>,
    pub urgency: Urgency,
    pub information: Information,
    pub readiness: Readiness,
    pub reason_codes: Vec<String>,
    pub missing_inputs: Vec<String>,
    pub applied_claim_ids: Vec<String>,
    pub action: Action,
    pub source_ids: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Prerequisite {
    pub name: String,
    pub claimed_support: Option<bool>,
    pub applied_claim_ids: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub id: String,
    pub target_id: String,
    pub identifier: String,
    pub purpose: Purpose,
    pub status: String,
    pub parameter_selection: String,
    pub prerequisites: Vec<Prerequisite>,
    pub validation_steps: Vec<String>,
    pub source_ids: Vec<String>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Version {
    pub id: String,
    pub version: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContextDescriptor {
    pub format: String,
    pub version: u32,
    pub sha256: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceRecord {
    pub id: String,
    pub edition: String,
    pub clause: String,
    pub uri: Option<String>,
    pub local_source_id: Option<String>,
    pub scope_limit: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct Coverage {
    pub collection_outcome: String,
    pub input_count: usize,
    pub target_count: usize,
    pub unknown_identity_or_parameters_targets: usize,
    pub unresolved_target_count: usize,
    pub limits: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct MigrationAssessment {
    format: String,
    version: u32,
    input: InputReference,
    context: Option<ContextDescriptor>,
    catalog: Version,
    policy: Version,
    model: Version,
    as_of: String,
    targets: Vec<Target>,
    claims: Vec<ResolvedClaim>,
    candidates: Vec<Candidate>,
    decisions: Vec<Decision>,
    sources: Vec<SourceRecord>,
    coverage: Coverage,
    limitations: Vec<String>,
}
impl MigrationAssessment {
    pub fn catalog(&self) -> &Version {
        &self.catalog
    }
    pub fn policy(&self) -> &Version {
        &self.policy
    }
    pub fn model(&self) -> &Version {
        &self.model
    }
    pub fn input(&self) -> &InputReference {
        &self.input
    }
    pub fn context(&self) -> Option<&ContextDescriptor> {
        self.context.as_ref()
    }
    pub fn as_of(&self) -> &str {
        &self.as_of
    }
    pub fn targets(&self) -> &[Target] {
        &self.targets
    }
    pub fn claims(&self) -> &[ResolvedClaim] {
        &self.claims
    }
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }
    pub fn decisions(&self) -> &[Decision] {
        &self.decisions
    }
    pub fn sources(&self) -> &[SourceRecord] {
        &self.sources
    }
    pub fn coverage(&self) -> &Coverage {
        &self.coverage
    }
    pub fn limitations(&self) -> &[String] {
        &self.limitations
    }

    /// Validate against the actual immutable inputs and scoped claims. Rendering
    /// does not implement policy or accept a report substituted from other facts.
    pub fn validate_for(
        &self,
        evidence: &SavedEvidence,
        context: Option<&MigrationContext>,
    ) -> Result<()> {
        if &self.input != evidence.input()
            || self.context.as_ref().map(|c| c.sha256.as_str())
                != context.map(MigrationContext::sha256)
            || self.targets != evidence.targets()
            || self.format != "pqc-scan.migration-assessment"
            || self.version != 1
            || self.catalog.id != CATALOG_ID
            || self.catalog.version != CATALOG_VERSION
            || self.policy.id != POLICY_ID
            || self.policy.version != POLICY_VERSION
            || self.model.id != MODEL_ID
            || self.model.version != MODEL_VERSION
            || self
                .context
                .as_ref()
                .is_some_and(|c| c.format != "pqc-scan.migration-context" || c.version != 1)
        {
            return Err(Error::InvalidReference);
        }
        let bound = context.map(|c| c.bind(evidence, &self.as_of)).transpose()?;
        if super::parse_as_of(&self.as_of).map_err(|_| Error::InvalidTime)? != self.as_of
            || timestamp(&self.as_of)?
                < bounded::collection_timestamp(&evidence.input().collection_time)?
        {
            return Err(Error::InvalidTime);
        }
        self.validate_references()?;
        let claim_index: BTreeMap<_, _> = self.claims.iter().map(|c| (c.id.as_str(), c)).collect();
        let mut used_claims = BTreeSet::new();
        for decision in &self.decisions {
            if decision.id
                != decision_id(
                    &decision.target_id,
                    context.map(MigrationContext::sha256).unwrap_or("absent"),
                    &self.as_of,
                )
            {
                return Err(Error::InvalidReference);
            }
            let actual = bound.as_ref().and_then(|b| b.target(&decision.target_id));
            if decision.system_id.as_deref() != actual.map(|t| t.system_id.as_str()) {
                return Err(Error::InvalidReference);
            }
            for id in &decision.applied_claim_ids {
                let claim = actual
                    .and_then(|t| t.claims().find(|c| &c.id == id))
                    .ok_or(Error::InvalidReference)?;
                let stored = claim_index
                    .get(id.as_str())
                    .copied()
                    .ok_or(Error::InvalidReference)?;
                if claim != stored {
                    return Err(Error::InvalidReference);
                }
                used_claims.insert(id.as_str());
            }
        }
        if used_claims.len() != claim_index.len() {
            return Err(Error::InvalidReference);
        }
        let target_index: BTreeMap<_, _> = evidence
            .targets()
            .iter()
            .map(|t| (t.id.as_str(), t))
            .collect();
        for candidate in &self.candidates {
            let actual = bound.as_ref().and_then(|b| b.target(&candidate.target_id));
            let target = target_index
                .get(candidate.target_id.as_str())
                .ok_or(Error::InvalidReference)?;
            if candidate.purpose != effective_purpose(target, actual)
                || candidate.id
                    != candidate_id(
                        &candidate.target_id,
                        context.map(MigrationContext::sha256).unwrap_or("absent"),
                        &candidate.identifier,
                        candidate.purpose,
                    )
            {
                return Err(Error::InvalidReference);
            }
            if candidate
                .prerequisites
                .iter()
                .map(|p| p.name.as_str())
                .collect::<Vec<_>>()
                != requirements(target, actual, candidate.purpose)
            {
                return Err(Error::InvalidReference);
            }
            for prerequisite in &candidate.prerequisites {
                let field = format!("support.{}", prerequisite.name);
                let expected_claim = actual.and_then(|t| t.claim(&field));
                let expected_ids = expected_claim
                    .map(|c| vec![c.id.clone()])
                    .unwrap_or_default();
                if prerequisite.applied_claim_ids != expected_ids
                    || prerequisite.claimed_support != boolean(actual, &field)
                {
                    return Err(Error::InvalidReference);
                }
            }
        }
        let mut expected_sources = BTreeMap::new();
        for target in &self.targets {
            add_algorithm_sources(&target.technical.algorithm, &mut expected_sources)?;
            if let Some(id) = &target.technical.protocol_ref {
                add_source(id, &mut expected_sources)?;
            }
        }
        for candidate in &self.candidates {
            for id in &candidate.source_ids {
                add_source(id, &mut expected_sources)?;
            }
        }
        for decision in &self.decisions {
            for id in &decision.source_ids {
                if !id.starts_with("context:") {
                    add_source(id, &mut expected_sources)?;
                }
            }
        }
        if let Some(context) = &bound {
            for claim in &self.claims {
                add_reference_sources(claim, context, &mut expected_sources)?;
            }
        }
        add_source("F14", &mut expected_sources)?;
        if self.sources != expected_sources.into_values().collect::<Vec<_>>() {
            return Err(Error::InvalidReference);
        }
        Ok(())
    }
    fn validate_references(&self) -> Result<()> {
        let targets: BTreeSet<_> = self.targets.iter().map(|t| t.id.as_str()).collect();
        let claims: BTreeSet<_> = self.claims.iter().map(|c| c.id.as_str()).collect();
        let sources: BTreeSet<_> = self.sources.iter().map(|s| s.id.as_str()).collect();
        let candidates: BTreeMap<_, _> =
            self.candidates.iter().map(|c| (c.id.as_str(), c)).collect();
        if targets.len() != self.targets.len()
            || claims.len() != self.claims.len()
            || sources.len() != self.sources.len()
            || candidates.len() != self.candidates.len()
            || self.decisions.len() != targets.len()
        {
            return Err(Error::InvalidReference);
        }
        let mut decided = BTreeSet::new();
        let mut referenced_candidates = BTreeSet::new();
        for t in &self.targets {
            validate_algorithm_sources(&t.technical.algorithm, &sources)?;
            if t.technical
                .protocol_ref
                .as_deref()
                .is_some_and(|s| !sources.contains(s))
            {
                return Err(Error::InvalidReference);
            }
        }
        for c in &self.candidates {
            if !targets.contains(c.target_id.as_str())
                || c.source_ids.iter().any(|s| !sources.contains(s.as_str()))
                || c.prerequisites.iter().any(|p| {
                    p.applied_claim_ids
                        .iter()
                        .any(|id| !claims.contains(id.as_str()))
                })
            {
                return Err(Error::InvalidReference);
            }
        }
        for d in &self.decisions {
            if !targets.contains(d.target_id.as_str())
                || !decided.insert(d.target_id.as_str())
                || d.applied_claim_ids
                    .iter()
                    .any(|id| !claims.contains(id.as_str()))
                || d.source_ids.iter().any(|id| !sources.contains(id.as_str()))
                || d.action.candidate_ids.iter().any(|id| {
                    candidates
                        .get(id.as_str())
                        .is_none_or(|c| c.target_id != d.target_id)
                })
            {
                return Err(Error::InvalidReference);
            }
            referenced_candidates.extend(d.action.candidate_ids.iter().map(String::as_str));
        }
        if referenced_candidates.len() != candidates.len() {
            return Err(Error::InvalidReference);
        }
        Ok(())
    }
}
fn validate_algorithm_sources(a: &AlgorithmAssessment, sources: &BTreeSet<&str>) -> Result<()> {
    if a.source_ids.iter().any(|id| !sources.contains(id.as_str())) {
        return Err(Error::InvalidReference);
    }
    for c in &a.components {
        validate_algorithm_sources(&c.assessment, sources)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct VersionSelection<'a> {
    pub catalog_id: &'a str,
    pub catalog_version: &'a str,
    pub policy_id: &'a str,
    pub policy_version: &'a str,
    pub model_id: &'a str,
    pub model_version: &'a str,
}
impl Default for VersionSelection<'static> {
    fn default() -> Self {
        Self {
            catalog_id: CATALOG_ID,
            catalog_version: CATALOG_VERSION,
            policy_id: POLICY_ID,
            policy_version: POLICY_VERSION,
            model_id: MODEL_ID,
            model_version: MODEL_VERSION,
        }
    }
}
pub fn evaluate(
    evidence: &SavedEvidence,
    context: Option<&MigrationContext>,
    as_of: &str,
) -> Result<MigrationAssessment> {
    evaluate_with_versions(evidence, context, as_of, VersionSelection::default())
}
pub fn evaluate_with_versions(
    evidence: &SavedEvidence,
    context: Option<&MigrationContext>,
    as_of: &str,
    versions: VersionSelection<'_>,
) -> Result<MigrationAssessment> {
    if versions.catalog_id != CATALOG_ID
        || versions.catalog_version != CATALOG_VERSION
        || versions.policy_id != POLICY_ID
        || versions.policy_version != POLICY_VERSION
        || versions.model_id != MODEL_ID
        || versions.model_version != MODEL_VERSION
    {
        return Err(Error::UnsupportedVersion);
    }
    let as_of = super::parse_as_of(as_of).map_err(|_| Error::InvalidTime)?;
    let now = timestamp(&as_of)?;
    if now < bounded::collection_timestamp(&evidence.input().collection_time)? {
        return Err(Error::InvalidTime);
    }
    let bound = context.map(|c| c.bind(evidence, &as_of)).transpose()?;
    let context_digest = context.map(MigrationContext::sha256).unwrap_or("absent");
    let mut decisions = vec![];
    let mut candidates = vec![];
    let mut claims = BTreeMap::new();
    let mut sources = BTreeMap::new();
    for t in evidence.targets() {
        add_algorithm_sources(&t.technical.algorithm, &mut sources)?;
        if let Some(id) = &t.technical.protocol_ref {
            add_source(id, &mut sources)?;
        }
        let target_context = bound.as_ref().and_then(|b| b.target(&t.id));
        let (d, cs, used) = decide(t, target_context, context_digest, &as_of, now)?;
        for id in &d.source_ids {
            add_source(id, &mut sources)?;
        }
        for claim in used {
            if let Some(bound) = &bound {
                add_reference_sources(&claim, bound, &mut sources)?;
            }
            claims.insert(claim.id.clone(), claim);
        }
        for candidate in &cs {
            for id in &candidate.source_ids {
                add_source(id, &mut sources)?;
            }
        }
        decisions.push(d);
        candidates.extend(cs);
    }
    add_source("F14", &mut sources)?;
    // Decision source references include only used context references, plus the
    // fixed technical/local policy sources. Source namespaces cannot collide.
    for d in &mut decisions {
        for id in &d.applied_claim_ids {
            if let Some(claim) = claims.get(id) {
                if let Some(r) = reference_id(claim) {
                    d.source_ids.push(format!("context:{r}"));
                }
            }
        }
        d.source_ids.sort();
        d.source_ids.dedup();
    }
    let coverage = Coverage {
        collection_outcome: evidence.collection_outcome().into(),
        input_count: 1,
        target_count: evidence.targets().len(),
        unknown_identity_or_parameters_targets: evidence
            .targets()
            .iter()
            .filter(|t| {
                t.technical.algorithm.identifier == "unknown"
                    || !t.technical.algorithm.parameters_known
            })
            .count(),
        unresolved_target_count: evidence.targets().iter().filter(|t| !t.bindable).count(),
        limits: if evidence.input().format == InputFormat::InventoryV1 {
            vec!["inventory_not_exhaustive".into()]
        } else {
            vec!["one_saved_attempt_not_endpoint_coverage".into()]
        },
    };
    let report = MigrationAssessment {
        format: "pqc-scan.migration-assessment".into(),
        version: 1,
        input: evidence.input().clone(),
        context: context.map(|c| ContextDescriptor {
            format: "pqc-scan.migration-context".into(),
            version: 1,
            sha256: c.sha256().into(),
        }),
        catalog: Version {
            id: CATALOG_ID.into(),
            version: CATALOG_VERSION.into(),
        },
        policy: Version {
            id: POLICY_ID.into(),
            version: POLICY_VERSION.into(),
        },
        model: Version {
            id: MODEL_ID.into(),
            version: MODEL_VERSION.into(),
        },
        as_of,
        targets: evidence.targets().to_vec(),
        claims: claims.into_values().collect(),
        candidates,
        decisions,
        sources: sources.into_values().collect(),
        coverage,
        limitations: vec![
            "saved_content_not_collector_attestation".into(),
            "no_offline_cryptographic_reverification".into(),
            "local_advisory_policy_not_practitioner_validation".into(),
        ],
    };
    report.validate_references()?;
    Ok(report)
}
fn timestamp(value: &str) -> Result<i64> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|d| d.timestamp())
        .map_err(|_| Error::InvalidTime)
}
fn value<'a>(c: Option<&'a TargetContext>, field: &str) -> Option<&'a Value> {
    c?.claim(field)?.active_value()
}
fn text<'a>(c: Option<&'a TargetContext>, field: &str) -> Option<&'a str> {
    value(c, field)?.as_str()
}
fn boolean(c: Option<&TargetContext>, field: &str) -> Option<bool> {
    value(c, field)?.as_bool()
}
fn purpose(usage: Option<&str>) -> Purpose {
    match usage {
        Some("key_establishment") => Purpose::KeyEstablishment,
        Some("signature" | "jwt" | "pki") => Purpose::Signature,
        Some("encryption") => Purpose::Encryption,
        Some("hashing") => Purpose::Hash,
        _ => Purpose::Unknown,
    }
}
fn effective_purpose(t: &Target, c: Option<&TargetContext>) -> Purpose {
    if c.is_some_and(|c| c.purpose_conflicting) || t.technical.algorithm.identifier == "unknown" {
        return Purpose::Unknown;
    }
    effective_from_usage(t, text(c, "use_case"))
}
fn effective_from_usage(t: &Target, usage: Option<&str>) -> Purpose {
    let actual = t.technical.algorithm.purpose;
    let claimed = purpose(usage);
    if actual != Purpose::Unknown {
        actual
    } else if matches!(t.technical.algorithm.identifier.as_str(), "RSA" | "EC")
        && matches!(claimed, Purpose::Signature | Purpose::KeyEstablishment)
    {
        claimed
    } else {
        Purpose::Unknown
    }
}
/// Input-question hints only. Reuse the evaluator's prerequisite vocabulary;
/// these do not establish active claims, readiness or an assessment decision.
pub fn input_support_fields(t: &Target, usage: Option<&str>) -> Result<Vec<&'static str>> {
    if usage.is_some_and(|s| {
        !matches!(
            s,
            "signature" | "jwt" | "pki" | "key_establishment" | "encryption" | "hashing"
        )
    }) {
        return Err(Error::InvalidInput);
    }
    Ok(support_requirements(
        t,
        usage,
        effective_from_usage(t, usage),
    ))
}
fn requirements(t: &Target, c: Option<&TargetContext>, p: Purpose) -> Vec<&'static str> {
    support_requirements(t, text(c, "use_case"), p)
}
fn support_requirements(t: &Target, usage: Option<&str>, p: Purpose) -> Vec<&'static str> {
    let mut fields = vec!["runtime", "provider", "parameters"];
    if matches!(p, Purpose::KeyEstablishment | Purpose::Signature) {
        fields.extend(["peer", "protocol"]);
    } else if p == Purpose::Encryption {
        fields.push("protocol");
    }
    if p == Purpose::Signature && usage == Some("jwt") {
        fields.extend(["jose", "verifiers"]);
    }
    if p == Purpose::Signature
        && (usage == Some("pki") || t.source_ref.role.starts_with("certificate"))
    {
        fields.push("pki");
    }
    fields
}

fn classical_concern(t: &Target, p: Purpose) -> bool {
    t.technical
        .algorithm
        .classical_concerns
        .iter()
        .any(|s| match s.as_str() {
            "public_key_below_local_minimum" | "signature_uses_weak_collision_digest" => true,
            "weak_collision_resistance" => p == Purpose::Signature,
            _ => false,
        })
}
fn adequate_use(t: &Target, c: Option<&TargetContext>) -> bool {
    t.observation.usage == "negotiated"
        && t.observation.assurance == "collector_reported_verified_session"
        || matches!(text(c, "usage_state"), Some("active" | "planned"))
}
fn bounded_no_concern(t: &Target) -> bool {
    let a = &t.technical.algorithm;
    if !a.parameters_known || a.identifier == "unknown" || !a.classical_concerns.is_empty() {
        return false;
    }
    if t.source_ref.role == "negotiated_key_establishment"
        && t.observation.assurance == "collector_reported_verified_session"
    {
        return a.quantum != QuantumExposure::Unknown
            || a.construction == "hybrid"
                && matches!(
                    a.standardization.as_str(),
                    "rfc_standard" | "rfc_informational"
                );
    }
    a.quantum != QuantumExposure::Unknown
        && t.observation.assurance != "unauthenticated_advertisement"
        && t.observation.assurance != "unknown"
        && matches!(
            t.observation.collection_outcome.as_str(),
            "complete" | "completed"
        )
        && (!t.source_ref.role.starts_with("certificate")
            || t.technical.encoding_assurance == "known")
}

fn decide(
    t: &Target,
    c: Option<&TargetContext>,
    digest: &str,
    as_of: &str,
    now: i64,
) -> Result<(Decision, Vec<Candidate>, Vec<ResolvedClaim>)> {
    let p = effective_purpose(t, c);
    let conflict = c.is_some_and(|c| c.purpose_conflicting);
    let known_classical = classical_concern(t, p);
    let known_quantum = t.technical.algorithm.quantum == QuantumExposure::Vulnerable;
    let concern = known_classical || known_quantum;
    let applicable = text(c, "applicability") == Some("applicable");
    let use_known = text(c, "use_case").is_some() && !conflict && p != Purpose::Unknown;
    let active = matches!(text(c, "usage_state"), Some("active" | "planned"));
    let deadline = value(c, "deadline");
    let lead = value(c, "migration_lead_seconds").and_then(Value::as_u64);
    let due = deadline
        .filter(|d| d["kind"] == "at")
        .and_then(|d| d["at"].as_str())
        .map(timestamp)
        .transpose()?;
    let horizon = lead
        .map(|l| now.checked_add(l as i64).ok_or(Error::InvalidTime))
        .transpose()?;
    let stale_due = c
        .and_then(|c| c.claim("deadline"))
        .filter(|d| d.freshness == ClaimFreshness::Stale)
        .and_then(|d| d.value.as_ref())
        .filter(|d| d["kind"] == "at")
        .and_then(|d| d["at"].as_str())
        .map(timestamp)
        .transpose()?
        .is_some_and(|d| d <= now);
    let future_protection = value(c, "confidentiality_until")
        .and_then(Value::as_str)
        .map(timestamp)
        .transpose()?
        .is_some_and(|d| d > now);
    let historical = boolean(c, "historical_ciphertext") == Some(true);
    let confidentiality = matches!(p, Purpose::KeyEstablishment | Purpose::Encryption);
    let signing = p == Purpose::Signature;
    let deadline_clear = deadline.is_some_and(|d| d["kind"] == "none")
        || due.zip(horizon).is_some_and(|(d, h)| d > h);
    let protection_clear = boolean(c, "confidential") == Some(false)
        || value(c, "confidentiality_until")
            .and_then(Value::as_str)
            .map(timestamp)
            .transpose()?
            .is_some_and(|d| d <= now)
            && boolean(c, "historical_ciphertext") == Some(false);
    let (row, urgency, reason, mut fields): (u8, Urgency, &str, Vec<&str>) =
        if text(c, "applicability") == Some("not_applicable") {
            (
                1,
                Urgency::NotApplicable,
                "outside_declared_scope",
                vec!["applicability"],
            )
        } else if known_classical {
            (2, Urgency::Urgent, "known_classical_concern", vec![])
        } else if concern && applicable && due.zip(horizon).is_some_and(|(d, h)| d <= h) {
            (
                3,
                Urgency::Urgent,
                "deadline_reached_or_lead_time_boundary",
                vec!["applicability", "deadline", "migration_lead_seconds"],
            )
        } else if known_quantum
            && applicable
            && use_known
            && signing
            && active
            && boolean(c, "business_critical") == Some(true)
        {
            (
                4,
                Urgency::Urgent,
                "critical_use_declared",
                vec![
                    "applicability",
                    "use_case",
                    "usage_state",
                    "business_critical",
                ],
            )
        } else if known_quantum
            && applicable
            && use_known
            && confidentiality
            && boolean(c, "confidential") == Some(true)
            && (historical || active && future_protection)
        {
            (
                4,
                Urgency::Urgent,
                "long_protection_or_historical_ciphertext",
                if historical {
                    vec![
                        "applicability",
                        "use_case",
                        "confidential",
                        "historical_ciphertext",
                    ]
                } else {
                    vec![
                        "applicability",
                        "use_case",
                        "confidential",
                        "usage_state",
                        "confidentiality_until",
                    ]
                },
            )
        } else if concern && stale_due {
            (
                5,
                Urgency::Urgent,
                "stale_urgent_claim_requires_refresh",
                vec!["deadline"],
            )
        } else if concern
            && applicable
            && use_known
            && boolean(c, "business_critical") == Some(false)
            && deadline_clear
            && (!confidentiality || protection_clear)
        {
            (
                6,
                Urgency::Plan,
                "known_concern_plan",
                vec!["applicability", "use_case", "business_critical", "deadline"],
            )
        } else if !concern && applicable && use_known && adequate_use(t, c) && bounded_no_concern(t)
        {
            (
                7,
                Urgency::Monitor,
                "bounded_role_no_known_concern",
                vec!["applicability", "use_case"],
            )
        } else {
            (8, Urgency::Undetermined, "insufficient_information", vec![])
        };
    let mut missing = BTreeSet::new();
    let mut used = BTreeMap::new();
    let mut blockers = vec![];
    if row != 1 {
        if c.is_none() {
            missing.insert("context.binding".into());
        }
        for field in ["applicability", "use_case"] {
            if value(c, field).is_none() {
                missing.insert(field.to_string());
            }
            fields.push(field);
        }
        if !adequate_use(t, c) && row != 5 && text(c, "usage_state").is_none() {
            missing.insert("usage_state".into());
        }
        if value(c, "usage_state").is_some() {
            fields.push("usage_state");
        }
        if t.technical.algorithm.identifier == "unknown" {
            missing.insert("technical.identity".into());
        }
        if !t.technical.algorithm.parameters_known && row != 2 {
            missing.insert("technical.parameters".into());
        }
        if t.source_ref.role.starts_with("certificate")
            && t.technical.encoding_assurance == "unknown"
        {
            missing.insert("technical.encoding".into());
        }
        if row == 8
            && t.technical
                .algorithm
                .classical_concerns
                .iter()
                .any(|s| s == "weak_collision_resistance")
            && p != Purpose::Signature
        {
            missing.insert("technical.collision_sensitive_use".into());
        }
        if row == 8 && t.observation.assurance == "unauthenticated_advertisement" {
            missing.insert("observation.selected_use".into());
        }
        if row == 8 && concern && applicable {
            if deadline.is_none() {
                missing.insert("deadline".into());
                fields.push("deadline");
            } else if due.is_some() && lead.is_none() {
                missing.insert("migration_lead_seconds".into());
                fields.extend(["deadline", "migration_lead_seconds"]);
            }
            if boolean(c, "business_critical").is_none() {
                missing.insert("business_critical".into());
                fields.push("business_critical");
            }
            if confidentiality && boolean(c, "confidential").is_none() {
                missing.insert("confidential".into());
                fields.push("confidential");
            }
            if confidentiality
                && boolean(c, "confidential") == Some(true)
                && !historical
                && !future_protection
            {
                for field in ["confidentiality_until", "historical_ciphertext"] {
                    if value(c, field).is_none() {
                        missing.insert(field.into());
                    }
                    fields.push(field);
                }
            }
        }
        if row == 6 && due.is_some() {
            fields.push("migration_lead_seconds");
        }
        if row == 6 && confidentiality {
            fields.push("confidential");
            if boolean(c, "confidential") != Some(false) {
                fields.extend(["confidentiality_until", "historical_ciphertext"]);
            }
        }
        if value(c, "signature_verification_until").is_some() && signing {
            fields.push("signature_verification_until");
        }
    }
    let required = requirements(t, c, p);
    let readiness = if row == 1 || row == 7 {
        Readiness::NotApplicable
    } else {
        for name in &required {
            let field = format!("support.{name}");
            // Unknown/conflicting purpose still retains the universal runtime,
            // provider and parameter prerequisites selected by requirements().
            // An unrelated purpose-specific claim is never consulted here.
            match boolean(c, &field) {
                Some(false) => blockers.push((*name).to_string()),
                Some(true) => {}
                None => {
                    missing.insert(field.clone());
                }
            }
            if let Some(claim) = c.and_then(|c| c.claim(&field)) {
                used.insert(claim.id.clone(), claim.clone());
            }
        }
        if !blockers.is_empty() {
            Readiness::Blocked
        } else if !use_known
            || t.technical.algorithm.identifier == "unknown"
            || required
                .iter()
                .any(|n| boolean(c, &format!("support.{n}")).is_none())
        {
            Readiness::Unknown
        } else {
            Readiness::Ready
        }
    };
    if let Some(claim) = c.and_then(|c| c.claim("migration_target")) {
        if row != 1 && row != 7 {
            used.insert(claim.id.clone(), claim.clone());
        }
    }
    for field in fields {
        if let Some(claim) = c.and_then(|c| c.claim(field)) {
            used.insert(claim.id.clone(), claim.clone());
        }
    }
    let stale = used.values().any(|c| c.freshness != ClaimFreshness::Active);
    let information = if row == 1 {
        Information::Sufficient
    } else if conflict {
        Information::Conflicting
    } else if !missing.is_empty() || stale || row == 8 {
        Information::Incomplete
    } else {
        Information::Sufficient
    };
    let action = if row == 1 {
        ActionKind::None
    } else if row == 7 {
        ActionKind::Monitor
    } else if row == 5 || conflict || stale || row == 8 || !adequate_use(t, c) || !applicable {
        ActionKind::Investigate
    } else if readiness == Readiness::Blocked {
        ActionKind::Unblock
    } else if readiness == Readiness::Ready && urgency == Urgency::Urgent {
        ActionKind::Migrate
    } else if readiness == Readiness::Ready && urgency == Urgency::Plan {
        ActionKind::Plan
    } else {
        ActionKind::Investigate
    };
    let cs = if row != 1 && row != 7 && concern && p != Purpose::Unknown && !conflict {
        make_candidates(t, c, p, digest, &required)?
    } else {
        vec![]
    };
    let mut reasons = vec![reason.to_string()];
    for code in [
        "reference_only",
        "unauthenticated_advertisement",
        "collection_incomplete",
        "verification_not_completed",
        "unknown_primitive",
        "unknown_parameter_encoding",
        "unknown_binding",
        "combiner_implementation_unverified",
    ] {
        if t.limitations.iter().any(|l| l == code) {
            reasons.push(code.into());
        }
    }
    if c.is_none() && !reasons.iter().any(|s| s == "unknown_binding") {
        reasons.push("unknown_binding".into());
    }
    if stale {
        reasons.push("stale_claim".into());
    }
    if conflict {
        reasons.push("purpose_mismatch".into());
    }
    if reasons.len() > 16 || missing.len() > 16 || cs.len() > 4 {
        return Err(Error::ResourceLimit);
    }
    let mut source_ids = t.technical.algorithm.source_ids.clone();
    source_ids.push("F14".into());
    if fields_time(&used) {
        source_ids.push("F13".into());
    }
    // Planning metadata is retained for review without entering any policy branch.
    if let Some(claim) = c.and_then(|c| c.claim("migration_goal_month")) {
        used.insert(claim.id.clone(), claim.clone());
    }
    let id = decision_id(&t.id, digest, as_of);
    let d = Decision {
        id,
        target_id: t.id.clone(),
        system_id: c.map(|c| c.system_id.clone()),
        urgency,
        information,
        readiness,
        reason_codes: reasons,
        missing_inputs: missing.into_iter().collect(),
        applied_claim_ids: used.keys().cloned().collect(),
        action: Action {
            kind: action,
            candidate_ids: cs.iter().map(|c| c.id.clone()).collect(),
            blocking_prerequisites: blockers,
            validation_steps: match action {
                ActionKind::None | ActionKind::Monitor => vec![],
                ActionKind::Investigate => vec![
                    "confirm_actual_use_and_required_purpose".into(),
                    "refresh_missing_or_stale_scoped_claims".into(),
                ],
                ActionKind::Unblock => vec![
                    "resolve_declared_blocking_prerequisites".into(),
                    "validate_implementation_and_interoperability_separately".into(),
                ],
                _ => vec![
                    "verify_exact_parameters_and_protocol_encoding".into(),
                    "test_runtime_provider_peer_and_rollout".into(),
                ],
            },
        },
        source_ids,
    };
    Ok((d, cs, used.into_values().collect()))
}
fn fields_time(used: &BTreeMap<String, ResolvedClaim>) -> bool {
    used.values().any(|c| {
        matches!(
            c.field.as_str(),
            "deadline"
                | "migration_lead_seconds"
                | "confidentiality_until"
                | "signature_verification_until"
                | "historical_ciphertext"
        )
    })
}

fn make_candidates(
    t: &Target,
    c: Option<&TargetContext>,
    p: Purpose,
    digest: &str,
    required: &[&str],
) -> Result<Vec<Candidate>> {
    let usage = text(c, "use_case").unwrap_or(match p {
        Purpose::KeyEstablishment => "key_establishment",
        Purpose::Signature => "signature",
        Purpose::Encryption => "encryption",
        Purpose::Hash => "hashing",
        Purpose::Unknown => return Ok(vec![]),
    });
    let context = crate::policy::AssessmentContext::from_json(
        &json!({"version":1,"use_case":usage}).to_string(),
    )
    .map_err(|_| Error::InvalidInput)?;
    let mut algorithm = t.technical.algorithm.clone();
    algorithm.purpose = p;
    Ok(crate::policy::candidates(&algorithm, &context)
        .into_iter()
        .map(|old| {
            let id = candidate_id(&t.id, digest, &old.identifier, old.purpose);
            Candidate {
                id,
                target_id: t.id.clone(),
                identifier: old.identifier,
                purpose: old.purpose,
                status: old.status,
                parameter_selection: old.parameter_selection,
                prerequisites: required
                    .iter()
                    .map(|name| {
                        let field = format!("support.{name}");
                        Prerequisite {
                            name: (*name).into(),
                            claimed_support: boolean(c, &field),
                            applied_claim_ids: c
                                .and_then(|c| c.claim(&field))
                                .map(|c| vec![c.id.clone()])
                                .unwrap_or_default(),
                        }
                    })
                    .collect(),
                validation_steps: old.validation_steps,
                source_ids: old.source_ids,
            }
        })
        .collect())
}
fn add_algorithm_sources(
    a: &AlgorithmAssessment,
    sources: &mut BTreeMap<String, SourceRecord>,
) -> Result<()> {
    for id in &a.source_ids {
        add_source(id, sources)?;
    }
    for c in &a.components {
        add_algorithm_sources(&c.assessment, sources)?;
    }
    Ok(())
}
fn add_source(id: &str, sources: &mut BTreeMap<String, SourceRecord>) -> Result<()> {
    let s = source(id).ok_or(Error::InvalidReference)?;
    let remote = s.uri.starts_with("https://");
    sources.entry(id.into()).or_insert_with(|| SourceRecord {
        id: id.into(),
        edition: s.edition.into(),
        clause: s.clause.into(),
        uri: remote.then(|| s.uri.into()),
        local_source_id: (!remote).then(|| s.uri.into()),
        scope_limit: s.scope_limit.into(),
    });
    Ok(())
}
fn reference_id(claim: &ResolvedClaim) -> Option<&str> {
    claim
        .value
        .as_ref()
        .filter(|_| claim.field == "deadline")
        .and_then(|v| v["reference_id"].as_str())
}
fn add_reference_sources(
    claim: &ResolvedClaim,
    context: &BoundContext,
    sources: &mut BTreeMap<String, SourceRecord>,
) -> Result<()> {
    let ids = [claim.provenance.reference_id(), reference_id(claim)];
    for id in ids.into_iter().flatten() {
        let r: &ContextReference = context
            .references()
            .iter()
            .find(|r| r.id == id)
            .ok_or(Error::InvalidReference)?;
        let key = format!("context:{id}");
        sources.entry(key.clone()).or_insert_with(|| SourceRecord {
            id: key,
            edition: r.edition.clone(),
            clause: r.clause.clone(),
            uri: r.https_uri.clone(),
            local_source_id: Some(r.id.clone()),
            scope_limit: "operator_declared_reference_not_independently_verified".into(),
        });
    }
    Ok(())
}

fn decision_id(target: &str, digest: &str, as_of: &str) -> String {
    native_id(
        "migration_decision",
        &[
            target,
            digest,
            CATALOG_ID,
            CATALOG_VERSION,
            POLICY_ID,
            POLICY_VERSION,
            MODEL_ID,
            MODEL_VERSION,
            as_of,
        ],
    )
}
fn candidate_id(target: &str, digest: &str, identifier: &str, purpose: Purpose) -> String {
    let purpose = match purpose {
        Purpose::KeyEstablishment => "key_establishment",
        Purpose::Signature => "signature",
        Purpose::Encryption => "encryption",
        Purpose::Hash => "hash",
        Purpose::Unknown => "unknown",
    };
    native_id(
        "migration_candidate",
        &[
            target,
            digest,
            CATALOG_ID,
            CATALOG_VERSION,
            POLICY_ID,
            POLICY_VERSION,
            identifier,
            purpose,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (SavedEvidence, MigrationContext, MigrationAssessment) {
        let mut v: Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../pqc_scan_cli/tests/fixtures/native-v1.golden.json"
        )))
        .unwrap();
        v["run"]["id"] = native_id("scan", &["guard"]).into();
        v["run"]["generated_at"] = "2026-11-17T00:00:00Z".into();
        let asset = native_id("asset", &["guard-rsa"]);
        v["assets"] = json!([{"id":asset,"kind":"algorithm","identity_basis":"normalized_identifier_and_reported_parameters","attributes":{"identifier":"RSA","namespace":"named","purpose":"reference","parameters":{"key_bits":"2048"}},"relations":[],"uncertainty":[]}]);
        let mut observation = v["observations"][0].clone();
        observation["id"] = native_id("observation", &["guard"]).into();
        observation["asset_ids"] = json!([asset]);
        observation["identity_confidence"] = "weak_structural_anchor".into();
        v["observations"] = json!([observation]);
        for name in [
            "legacy_associations",
            "legacy_loss_records",
            "unassociated_observation_ids",
        ] {
            v[name] = json!([]);
        }
        let evidence =
            SavedEvidence::from_bytes(&serde_json::to_vec(&v).unwrap(), InputFormat::InventoryV1)
                .unwrap();
        let provenance = json!({"source_kind":"operator_claim","actor_id":"owner","asserted_at":"2026-11-17T00:00:00Z","valid_until":"2027-11-17T00:00:00Z"});
        let claim = |value: Value| json!({"state":"known","value":value,"provenance":provenance});
        let c = json!({"format":"pqc-scan.migration-context","version":1,"input":{"format":"inventory-v1","sha256":evidence.input().sha256},"organization_id":"org","systems":[{"id":"system"}],"bindings":[{"target_id":evidence.targets()[0].id,"system_id":"system","claims":{"applicability":claim(json!("applicable")),"use_case":claim(json!("signature")),"usage_state":claim(json!("active")),"business_critical":claim(json!(true)),"support":{"runtime":claim(json!(true)),"provider":claim(json!(true)),"parameters":claim(json!(true)),"peer":claim(json!(false)),"protocol":claim(json!(true))}}}]});
        let context = MigrationContext::from_bytes(&serde_json::to_vec(&c).unwrap()).unwrap();
        let report = evaluate(&evidence, Some(&context), "2026-11-18T00:00:00Z").unwrap();
        (evidence, context, report)
    }
    #[test]
    fn output_validation_rejects_missing_duplicate_extra_and_substituted_references() {
        let (e, c, valid) = fixture();
        assert!(valid.validate_for(&e, Some(&c)).is_ok());
        let mut bad = valid.clone();
        bad.decisions.clear();
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.decisions.push(bad.decisions[0].clone());
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.targets[0].id = native_id("migration_target", &["missing"]);
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.sources.clear();
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.sources[0].scope_limit = "scanner_verified".into();
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.claims.clear();
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        let mut extra = bad.claims[0].clone();
        extra.id = native_id("migration_claim", &["extra"]);
        bad.claims.push(extra);
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.claims
            .iter_mut()
            .find(|c| c.field == "business_critical")
            .unwrap()
            .value = Some(json!(false));
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        bad.candidates[0].id = native_id("migration_candidate", &["missing"]);
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid.clone();
        let peer = bad.candidates[0]
            .prerequisites
            .iter_mut()
            .find(|p| p.name == "peer")
            .unwrap();
        peer.claimed_support = Some(true);
        assert!(bad.validate_for(&e, Some(&c)).is_err());
        let mut bad = valid;
        bad.policy.version = "2.0.0".into();
        assert!(bad.validate_for(&e, Some(&c)).is_err());
    }
}
