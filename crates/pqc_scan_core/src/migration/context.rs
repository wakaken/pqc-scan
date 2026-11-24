//! Closed local claims and immutable target bindings; no policy decisions.
use super::{bounded, Error, InputFormat, Result, SavedEvidence, Target};
use crate::{assessment::Purpose, identity::native_id};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    path::Path,
};

const LIMIT: usize = 1024 * 1024;
const FORMAT: &str = "pqc-scan.migration-context";
#[path = "generation.rs"]
pub mod generation;
pub(crate) const SUPPORT_NAMES: [&str; 8] = [
    "runtime",
    "provider",
    "peer",
    "protocol",
    "parameters",
    "jose",
    "verifiers",
    "pki",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct InputBinding {
    format: InputFormat,
    sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum SourceKind {
    OperatorClaim,
    ExternalReviewClaim,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    source_kind: SourceKind,
    actor_id: String,
    asserted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
}
impl Provenance {
    pub fn source_kind(&self) -> &str {
        match self.source_kind {
            SourceKind::OperatorClaim => "operator_claim",
            SourceKind::ExternalReviewClaim => "external_review_claim",
        }
    }
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }
    pub fn asserted_at(&self) -> &str {
        &self.asserted_at
    }
    pub fn valid_until(&self) -> Option<&str> {
        self.valid_until.as_deref()
    }
    pub fn reference_id(&self) -> Option<&str> {
        self.reference_id.as_deref()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
enum Claim<T> {
    Known { value: T, provenance: Provenance },
    Unknown { provenance: Provenance },
}
impl<T> Claim<T> {
    fn provenance_mut(&mut self) -> &mut Provenance {
        match self {
            Self::Known { provenance, .. } | Self::Unknown { provenance } => provenance,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Applicability {
    Applicable,
    NotApplicable,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum UseCase {
    KeyEstablishment,
    Signature,
    Jwt,
    Pki,
    Encryption,
    Hashing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Usage {
    Active,
    Planned,
    TestOnly,
    Unused,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DeadlineScope {
    OrganizationPolicy,
    ContractualRequirement,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum Deadline {
    None {},
    At {
        at: String,
        reference_id: String,
        scope_reason: DeadlineScope,
    },
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Support {
    runtime: Option<Claim<bool>>,
    provider: Option<Claim<bool>>,
    peer: Option<Claim<bool>>,
    protocol: Option<Claim<bool>>,
    parameters: Option<Claim<bool>>,
    jose: Option<Claim<bool>>,
    verifiers: Option<Claim<bool>>,
    pki: Option<Claim<bool>>,
}
/// Explicit operator-selected integration configuration, not verified compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MigrationConfiguration {
    use_case: String,
    algorithm: String,
    parameters: String,
}
impl MigrationConfiguration {
    fn validate(&self) -> Result<()> {
        let expected = match self.use_case.as_str() {
            "signature" | "jwt" | "pki" => "ML-DSA",
            "key_establishment" => "ML-KEM",
            "encryption" => "AES",
            "hashing" => "SHA-256",
            _ => return Err(Error::InvalidInput),
        };
        if self.algorithm != expected
            || self.parameters.trim().is_empty()
            || self.parameters.len() > 320
            || self.parameters.chars().count() > 80
            || self.parameters.chars().any(char::is_control)
        {
            return Err(Error::InvalidInput);
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimSet {
    migration_target: Option<Claim<MigrationConfiguration>>,
    migration_goal_month: Option<Claim<String>>,
    applicability: Option<Claim<Applicability>>,
    use_case: Option<Claim<UseCase>>,
    usage_state: Option<Claim<Usage>>,
    business_critical: Option<Claim<bool>>,
    confidential: Option<Claim<bool>>,
    confidentiality_until: Option<Claim<String>>,
    signature_verification_until: Option<Claim<String>>,
    historical_ciphertext: Option<Claim<bool>>,
    migration_lead_seconds: Option<Claim<u64>>,
    deadline: Option<Claim<Deadline>>,
    support: Option<Support>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct System {
    id: String,
    defaults: Option<ClaimSet>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    target_id: String,
    system_id: String,
    claims: Option<ClaimSet>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextReference {
    pub id: String,
    pub issuer_id: String,
    pub document_id: String,
    pub edition: String,
    pub clause: String,
    pub scope_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_uri: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    format: String,
    version: u32,
    input: InputBinding,
    organization_id: String,
    systems: Vec<System>,
    bindings: Vec<Binding>,
    organization_defaults: Option<ClaimSet>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    references: Vec<ContextReference>,
}

/// Validated context. No raw context or input path is serialized into results.
#[derive(Debug)]
pub struct MigrationContext {
    input: Input,
    sha256: String,
    pub(crate) file: Option<File>,
    path: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimFreshness {
    Active,
    Stale,
    NotYetEffective,
    Unknown,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResolvedClaim {
    pub id: String,
    pub context_pointer: String,
    pub field: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    pub provenance: Provenance,
    pub freshness: ClaimFreshness,
}
impl ResolvedClaim {
    pub fn active_value(&self) -> Option<&Value> {
        (self.freshness == ClaimFreshness::Active)
            .then_some(self.value.as_ref())
            .flatten()
    }
}
#[derive(Debug, Clone)]
pub struct TargetContext {
    pub system_id: String,
    claims: BTreeMap<String, ResolvedClaim>,
    pub purpose_conflicting: bool,
}
impl TargetContext {
    pub fn claim(&self, field: &str) -> Option<&ResolvedClaim> {
        let claim = self.claims.get(field)?;
        if field.starts_with("support.") {
            if let Some(configuration) = self.claims.get("migration_target") {
                let value = configuration.active_value()?;
                if self
                    .claims
                    .get("use_case")
                    .and_then(ResolvedClaim::active_value)
                    .and_then(Value::as_str)
                    != value["use_case"].as_str()
                {
                    return None;
                }
            }
        }
        Some(claim)
    }
    pub fn claims(&self) -> impl Iterator<Item = &ResolvedClaim> {
        self.claims.values()
    }
}
#[derive(Debug)]
pub struct BoundContext {
    pub organization_id: String,
    pub sha256: String,
    targets: BTreeMap<String, TargetContext>,
    references: Vec<ContextReference>,
}
impl BoundContext {
    pub fn target(&self, id: &str) -> Option<&TargetContext> {
        self.targets.get(id)
    }
    pub fn references(&self) -> &[ContextReference] {
        &self.references
    }
}

impl MigrationContext {
    pub fn read(path: &Path) -> Result<Self> {
        let (bytes, file) = bounded::read(path, LIMIT)?;
        let mut result = Self::from_bytes(&bytes)?;
        result.file = Some(file);
        result.path = Some(super::absolute_path(path)?);
        Ok(result)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > LIMIT {
            return Err(Error::ResourceLimit);
        }
        let sha256 = bounded::digest(bytes);
        let value = bounded::parse(bytes, LIMIT)?;
        if value["format"] != FORMAT || value["version"] != 1 {
            return Err(Error::UnsupportedVersion);
        }
        // Serde Option alone treats null as absent; the contract forbids null at
        // every context position, including optional claims and nested wrappers.
        reject_null(&value)?;
        let mut input: Input = serde_json::from_value(value).map_err(|_| Error::InvalidInput)?;
        validate(&mut input)?;
        Ok(Self {
            input,
            sha256,
            file: None,
            path: None,
        })
    }
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
    pub(crate) fn aliases_output(&self, destination: &Path) -> bool {
        super::aliases(destination, self.path.as_deref(), self.file.as_ref()).unwrap_or(true)
    }
    pub fn bind(&self, evidence: &SavedEvidence, as_of: &str) -> Result<BoundContext> {
        let now = instant(as_of).map_err(|_| Error::InvalidTime)?;
        if now < bounded::collection_timestamp(&evidence.input().collection_time)? {
            return Err(Error::InvalidTime);
        }
        if self.input.input.format != evidence.input().format
            || self.input.input.sha256 != evidence.input().sha256
        {
            return Err(Error::InputBindingMismatch);
        }
        let index: BTreeMap<_, _> = evidence
            .targets()
            .iter()
            .map(|t| (t.id.as_str(), t))
            .collect();
        let systems: BTreeMap<_, _> = self
            .input
            .systems
            .iter()
            .enumerate()
            .map(|(i, s)| (s.id.as_str(), (i, s)))
            .collect();
        let mut targets = BTreeMap::new();
        for (i, binding) in self.input.bindings.iter().enumerate() {
            let target = index
                .get(binding.target_id.as_str())
                .ok_or(Error::InvalidReference)?;
            if !target.bindable {
                return Err(Error::AmbiguousContextBinding);
            }
            let (system_index, system) = systems
                .get(binding.system_id.as_str())
                .ok_or(Error::InvalidContextBinding)?;
            let mut claims = BTreeMap::new();
            for (set, pointer) in [
                (
                    self.input.organization_defaults.as_ref(),
                    "/organization_defaults".to_string(),
                ),
                (
                    system.defaults.as_ref(),
                    format!("/systems/{system_index}/defaults"),
                ),
                (binding.claims.as_ref(), format!("/bindings/{i}/claims")),
            ] {
                if let Some(set) = set {
                    merge_claims(&mut claims, set, &pointer, &self.sha256, now)?;
                }
            }
            if let Some(configuration) = claims.get("migration_target") {
                if let Some(value) = &configuration.value {
                    let declared = claims
                        .get("use_case")
                        .and_then(|c| c.value.as_ref())
                        .and_then(Value::as_str);
                    if declared != value["use_case"].as_str() {
                        return Err(Error::InvalidInput);
                    }
                }
                if configuration.state == "known" {
                    let scope = configuration
                        .context_pointer
                        .strip_suffix("/migration_target")
                        .ok_or(Error::InvalidInput)?;
                    for claim in claims
                        .values()
                        .filter(|c| c.field.starts_with("support.") && c.state == "known")
                    {
                        if !claim
                            .context_pointer
                            .starts_with(&format!("{scope}/support/"))
                        {
                            return Err(Error::InvalidContextBinding);
                        }
                    }
                }
            }
            let purpose_conflicting = claims
                .get("use_case")
                .and_then(ResolvedClaim::active_value)
                .and_then(Value::as_str)
                .is_some_and(|usage| purpose_conflict(target, usage));
            if targets
                .insert(
                    binding.target_id.clone(),
                    TargetContext {
                        system_id: binding.system_id.clone(),
                        claims,
                        purpose_conflicting,
                    },
                )
                .is_some()
            {
                return Err(Error::InvalidContextBinding);
            }
        }
        Ok(BoundContext {
            organization_id: self.input.organization_id.clone(),
            sha256: self.sha256.clone(),
            targets,
            references: self.input.references.clone(),
        })
    }
}

pub fn parse_as_of(value: &str) -> Result<String> {
    normalized(value)
}
fn instant(value: &str) -> Result<i64> {
    use chrono::{Datelike, Timelike};
    let date = chrono::DateTime::parse_from_rfc3339(value).map_err(|_| Error::InvalidInput)?;
    let utc = date.with_timezone(&chrono::Utc);
    if !(1970..=9999).contains(&date.year())
        || !(1970..=9999).contains(&utc.year())
        || date.nanosecond() != 0
    {
        return Err(Error::InvalidInput);
    }
    Ok(date.timestamp())
}
fn normalized(value: &str) -> Result<String> {
    chrono::DateTime::from_timestamp(instant(value)?, 0)
        .map(|d| d.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        .ok_or(Error::InvalidInput)
}
fn reject_null(value: &Value) -> Result<()> {
    match value {
        Value::Null => return Err(Error::InvalidInput),
        Value::Object(m) => {
            for v in m.values() {
                reject_null(v)?;
            }
        }
        Value::Array(a) => {
            for v in a {
                reject_null(v)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn alias(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(&b))
}
fn identifier(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128 && value.bytes().all(|b| (0x21..=0x7e).contains(&b))
}
fn sha(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn validate(input: &mut Input) -> Result<()> {
    if input.format != FORMAT || input.version != 1 {
        return Err(Error::UnsupportedVersion);
    }
    if input.organization_id.len() > 64 {
        return Err(Error::ResourceLimit);
    }
    if !alias(&input.organization_id) || !sha(&input.input.sha256) {
        return Err(Error::InvalidInput);
    }
    if input.systems.len() > 256 || input.bindings.len() > 4096 {
        return Err(Error::ResourceLimit);
    }
    let refs = reference_ids(&input.references)?;
    let mut systems = BTreeSet::new();
    for s in &input.systems {
        if s.id.len() > 64 {
            return Err(Error::ResourceLimit);
        }
        if !alias(&s.id) || !systems.insert(s.id.as_str()) {
            return Err(Error::InvalidContextBinding);
        }
    }
    let mut bindings = BTreeSet::new();
    for b in &input.bindings {
        if b.system_id.len() > 64 {
            return Err(Error::ResourceLimit);
        }
        if !crate::assessment::valid_id(&b.target_id, "migration_target")
            || !systems.contains(b.system_id.as_str())
        {
            return Err(Error::InvalidReference);
        }
        if !bindings.insert(b.target_id.as_str()) {
            return Err(Error::AmbiguousContextBinding);
        }
    }
    if let Some(set) = &mut input.organization_defaults {
        validate_set(set, &refs)?;
    }
    for s in &mut input.systems {
        if let Some(set) = &mut s.defaults {
            validate_set(set, &refs)?;
        }
    }
    for b in &mut input.bindings {
        if let Some(set) = &mut b.claims {
            validate_set(set, &refs)?;
        }
    }
    Ok(())
}
fn validate_uri(uri: &str) -> Result<()> {
    if uri.len() > 1024 {
        return Err(Error::ResourceLimit);
    }
    let rest = uri
        .strip_prefix("https://")
        .ok_or(Error::InvalidReference)?;
    if uri.chars().any(char::is_control) || uri.contains(['@', '?', '\\', ' ']) {
        return Err(Error::InvalidReference);
    }
    let authority = rest
        .split(['/', '#'])
        .next()
        .ok_or(Error::InvalidReference)?;
    let (host, port) = if let Some(rest) = authority.strip_prefix('[') {
        let (host, suffix) = rest.split_once(']').ok_or(Error::InvalidReference)?;
        (
            host,
            if suffix.is_empty() {
                443
            } else {
                suffix
                    .strip_prefix(':')
                    .and_then(|s| s.parse::<u16>().ok())
                    .ok_or(Error::InvalidReference)?
            },
        )
    } else if let Some((host, port)) = authority.rsplit_once(':') {
        (
            host,
            port.parse::<u16>().map_err(|_| Error::InvalidReference)?,
        )
    } else {
        (authority, 443)
    };
    crate::target_policy::TargetRequest::new(host, port).map_err(|_| Error::InvalidReference)?;
    Ok(())
}
fn provenance(p: &mut Provenance, refs: &BTreeSet<&str>) -> Result<()> {
    if p.actor_id.len() > 64 {
        return Err(Error::ResourceLimit);
    }
    if p.reference_id.as_ref().is_some_and(|id| id.len() > 128) {
        return Err(Error::ResourceLimit);
    }
    if !alias(&p.actor_id) {
        return Err(Error::InvalidInput);
    }
    p.asserted_at = normalized(&p.asserted_at)?;
    if let Some(until) = &mut p.valid_until {
        *until = normalized(until)?;
        if instant(&p.asserted_at)? >= instant(until)? {
            return Err(Error::InvalidInput);
        }
    }
    if p.reference_id.as_deref().is_some_and(|r| !refs.contains(r)) {
        return Err(Error::InvalidReference);
    }
    Ok(())
}
fn validate_claim<T>(claim: &mut Option<Claim<T>>, refs: &BTreeSet<&str>) -> Result<()> {
    if let Some(claim) = claim {
        provenance(claim.provenance_mut(), refs)?;
    }
    Ok(())
}
fn validate_set(s: &mut ClaimSet, refs: &BTreeSet<&str>) -> Result<()> {
    validate_claim(&mut s.migration_target, refs)?;
    if let Some(Claim::Known { value, .. }) = &s.migration_target {
        value.validate()?;
    }
    validate_claim(&mut s.migration_goal_month, refs)?;
    if let Some(Claim::Known { value, .. }) = &s.migration_goal_month {
        let bytes = value.as_bytes();
        if bytes.len() != 7
            || bytes[4] != b'-'
            || !bytes[..4].iter().all(u8::is_ascii_digit)
            || !bytes[5..].iter().all(u8::is_ascii_digit)
            || &value[..4] == "0000"
            || !("01"..="12").contains(&&value[5..])
        {
            return Err(Error::InvalidInput);
        }
    }
    validate_claim(&mut s.applicability, refs)?;
    validate_claim(&mut s.use_case, refs)?;
    validate_claim(&mut s.usage_state, refs)?;
    validate_claim(&mut s.business_critical, refs)?;
    validate_claim(&mut s.confidential, refs)?;
    validate_claim(&mut s.historical_ciphertext, refs)?;
    for c in [
        &mut s.confidentiality_until,
        &mut s.signature_verification_until,
    ] {
        validate_claim(c, refs)?;
        if let Some(Claim::Known { value, .. }) = c {
            *value = normalized(value)?;
        }
    }
    validate_claim(&mut s.migration_lead_seconds, refs)?;
    if matches!(&s.migration_lead_seconds,Some(Claim::Known{value,..}) if *value>315576000) {
        return Err(Error::InvalidInput);
    }
    validate_claim(&mut s.deadline, refs)?;
    if let Some(Claim::Known {
        value: Deadline::At {
            at, reference_id, ..
        },
        ..
    }) = &mut s.deadline
    {
        *at = normalized(at)?;
        if reference_id.len() > 128 {
            return Err(Error::ResourceLimit);
        }
        if !refs.contains(reference_id.as_str()) {
            return Err(Error::InvalidReference);
        }
    }
    if let Some(s) = &mut s.support {
        for c in [
            &mut s.runtime,
            &mut s.provider,
            &mut s.peer,
            &mut s.protocol,
            &mut s.parameters,
            &mut s.jose,
            &mut s.verifiers,
            &mut s.pki,
        ] {
            validate_claim(c, refs)?;
        }
    }
    Ok(())
}

fn add<T: Serialize>(
    claims: &mut BTreeMap<String, ResolvedClaim>,
    claim: &Option<Claim<T>>,
    field: &str,
    pointer: &str,
    digest: &str,
    now: i64,
) -> Result<()> {
    let Some(claim) = claim else {
        return Ok(());
    };
    let (state, value, p) = match claim {
        Claim::Known { value, provenance } => (
            "known",
            Some(serde_json::to_value(value).map_err(|_| Error::InvalidInput)?),
            provenance,
        ),
        Claim::Unknown { provenance } => ("unknown", None, provenance),
    };
    let freshness = if now < instant(&p.asserted_at)? {
        ClaimFreshness::NotYetEffective
    } else if let Some(until) = &p.valid_until {
        if now >= instant(until)? {
            ClaimFreshness::Stale
        } else {
            ClaimFreshness::Active
        }
    } else {
        ClaimFreshness::Unknown
    };
    let context_pointer = format!("{pointer}/{}", field.replace('.', "/"));
    let id = native_id("migration_claim", &[digest, &context_pointer, field]);
    claims.insert(
        field.into(),
        ResolvedClaim {
            id,
            context_pointer,
            field: field.into(),
            state: state.into(),
            value,
            provenance: p.clone(),
            freshness,
        },
    );
    Ok(())
}
fn merge_claims(
    c: &mut BTreeMap<String, ResolvedClaim>,
    s: &ClaimSet,
    pointer: &str,
    digest: &str,
    now: i64,
) -> Result<()> {
    macro_rules! field {
        ($name:ident) => {
            add(c, &s.$name, stringify!($name), pointer, digest, now)?;
        };
    }
    field!(migration_target);
    field!(migration_goal_month);
    field!(applicability);
    field!(use_case);
    field!(usage_state);
    field!(business_critical);
    field!(confidential);
    field!(confidentiality_until);
    field!(signature_verification_until);
    field!(historical_ciphertext);
    field!(migration_lead_seconds);
    field!(deadline);
    if let Some(s) = &s.support {
        for (name, value) in SUPPORT_NAMES.into_iter().zip([
            &s.runtime,
            &s.provider,
            &s.peer,
            &s.protocol,
            &s.parameters,
            &s.jose,
            &s.verifiers,
            &s.pki,
        ]) {
            add(c, value, &format!("support.{name}"), pointer, digest, now)?;
        }
    }
    Ok(())
}
fn purpose_conflict(target: &Target, usage: &str) -> bool {
    let selected = match usage {
        "key_establishment" => Purpose::KeyEstablishment,
        "signature" | "jwt" | "pki" => Purpose::Signature,
        "encryption" => Purpose::Encryption,
        "hashing" => Purpose::Hash,
        _ => return false,
    };
    let a = &target.technical.algorithm;
    if target.source_ref.role.contains("signature") && selected != Purpose::Signature {
        return true;
    }
    if target.source_ref.role.contains("key_establishment") && selected != Purpose::KeyEstablishment
    {
        return true;
    }
    if a.purpose != Purpose::Unknown {
        return a.purpose != selected;
    }
    matches!(a.identifier.as_str(), "RSA" | "EC")
        && !matches!(selected, Purpose::Signature | Purpose::KeyEstablishment)
}

fn reference_ids(references: &[ContextReference]) -> Result<BTreeSet<&str>> {
    if references.len() > 65536 {
        return Err(Error::ResourceLimit);
    }
    let mut refs = BTreeSet::new();
    for r in references {
        if [
            &r.id,
            &r.issuer_id,
            &r.document_id,
            &r.edition,
            &r.clause,
            &r.scope_id,
        ]
        .iter()
        .any(|s| s.len() > 128)
        {
            return Err(Error::ResourceLimit);
        }
        if ![
            &r.id,
            &r.issuer_id,
            &r.document_id,
            &r.edition,
            &r.clause,
            &r.scope_id,
        ]
        .iter()
        .all(|s| identifier(s))
            || !refs.insert(r.id.as_str())
        {
            return Err(Error::InvalidReference);
        }
        if let Some(uri) = &r.https_uri {
            validate_uri(uri)?;
        }
    }
    Ok(refs)
}
