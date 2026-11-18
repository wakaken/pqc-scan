//! Opt-in saved-evidence facts. No collectors, network providers or legacy dispatch.
mod bounded;
mod context;
mod decision;
mod native;
mod runtime;
mod technical;

use crate::{assessment::AlgorithmAssessment, identity::native_id};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, path::Path, str::FromStr};

pub use context::{
    parse_as_of, BoundContext, ClaimFreshness, ContextReference, MigrationContext, ResolvedClaim,
    TargetContext,
};
pub use decision::{
    evaluate, evaluate_with_versions, Action, ActionKind, Candidate, ContextDescriptor, Coverage,
    Decision, Information, MigrationAssessment, Prerequisite, Readiness, SourceRecord, Urgency,
    Version, VersionSelection, POLICY_ID, POLICY_VERSION,
};
pub use technical::{assess_primitive, source, TechnicalSource, CATALOG_ID, CATALOG_VERSION};

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("input_unavailable")]
    InputUnavailable,
    #[error("invalid_input")]
    InvalidInput,
    #[error("resource_limit")]
    ResourceLimit,
    #[error("unsupported_format")]
    UnsupportedFormat,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("invalid_reference")]
    InvalidReference,
    #[error("contradictory_saved_facts")]
    ContradictorySavedFacts,
    #[error("invalid_context_binding")]
    InvalidContextBinding,
    #[error("ambiguous_binding")]
    AmbiguousContextBinding,
    #[error("input_binding_mismatch")]
    InputBindingMismatch,
    #[error("invalid_time")]
    InvalidTime,
    #[error("unsafe_output")]
    UnsafeOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputFormat {
    #[serde(rename = "inventory-v1")]
    InventoryV1,
    #[serde(rename = "tls-v1")]
    TlsV1,
    #[serde(rename = "ssh-offers-v1")]
    SshOffersV1,
    #[serde(rename = "ssh-kex-v1")]
    SshKexV1,
}
impl InputFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InventoryV1 => "inventory-v1",
            Self::TlsV1 => "tls-v1",
            Self::SshOffersV1 => "ssh-offers-v1",
            Self::SshKexV1 => "ssh-kex-v1",
        }
    }
    pub fn byte_limit(self) -> usize {
        if self == Self::InventoryV1 {
            32 * 1024 * 1024
        } else {
            1024 * 1024
        }
    }
}
impl FromStr for InputFormat {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self> {
        match value {
            "inventory-v1" => Ok(Self::InventoryV1),
            "tls-v1" => Ok(Self::TlsV1),
            "ssh-offers-v1" => Ok(Self::SshOffersV1),
            "ssh-kex-v1" => Ok(Self::SshKexV1),
            _ => Err(Error::UnsupportedFormat),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InputReference {
    pub format: InputFormat,
    pub sha256: String,
    pub schema_version: u32,
    pub collection_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct SourceReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_asset_id: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spki_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_provenance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<RegistryFact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_suite: Option<RegistryFact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_exchange_group: Option<RegistryFact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after_unix: Option<i64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RegistryFact {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TechnicalFacts {
    #[serde(flatten)]
    pub algorithm: AlgorithmAssessment,
    pub encoding_assurance: String,
    pub protocol_ref: Option<String>,
    pub role: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObservationFacts {
    pub method: String,
    pub assurance: String,
    pub usage: String,
    pub collection_outcome: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Target {
    pub id: String,
    pub source_ref: SourceReference,
    pub technical: TechnicalFacts,
    pub observation: ObservationFacts,
    pub limitations: Vec<String>,
    /// False for ambiguous native anchors; never inferred from array position.
    #[serde(skip)]
    pub(crate) bindable: bool,
}

/// Validated projection; callers cannot substitute input identity or targets.
/// Hashes bind saved bytes, not collector authenticity or business identity.
#[derive(Debug)]
pub struct SavedEvidence {
    input: InputReference,
    targets: Vec<Target>,
    collection_outcome: String,
    // Keep the opened identity for input/output alias protection in the CLI.
    pub(crate) file: Option<File>,
    path: Option<std::path::PathBuf>,
}
impl SavedEvidence {
    pub fn read(path: &Path, format: InputFormat) -> Result<Self> {
        let (bytes, file) = bounded::read(path, format.byte_limit())?;
        let mut result = Self::from_bytes(&bytes, format)?;
        result.file = Some(file);
        result.path = Some(absolute_path(path)?);
        Ok(result)
    }
    pub fn from_bytes(bytes: &[u8], format: InputFormat) -> Result<Self> {
        Self::from_bytes_with_catalog(bytes, format, CATALOG_ID, CATALOG_VERSION)
    }
    pub fn from_bytes_with_catalog(
        bytes: &[u8],
        format: InputFormat,
        catalog: &str,
        version: &str,
    ) -> Result<Self> {
        if catalog != CATALOG_ID || version != CATALOG_VERSION {
            return Err(Error::UnsupportedVersion);
        }
        if bytes.len() > format.byte_limit() {
            return Err(Error::ResourceLimit);
        }
        let hash = bounded::digest(bytes);
        let value = bounded::parse(bytes, format.byte_limit())?;
        if value["schema_version"] != 1 {
            return Err(Error::UnsupportedVersion);
        }
        let mut result = if format == InputFormat::InventoryV1 {
            native::project(value, hash)?
        } else {
            runtime::project(value, format, hash)?
        };
        if result.targets.len() > 65536 {
            return Err(Error::ResourceLimit);
        }
        // Native references define order; runtime adapters preserve list/index order.
        if format == InputFormat::InventoryV1 {
            result.targets.sort_by_key(|t| {
                (
                    t.source_ref.observation_id.clone(),
                    t.source_ref.root_asset_id.clone(),
                    t.source_ref.detail_path.clone(),
                    t.source_ref.role.clone(),
                )
            });
        }
        Ok(result)
    }
    pub fn input(&self) -> &InputReference {
        &self.input
    }
    pub fn targets(&self) -> &[Target] {
        &self.targets
    }
    pub fn collection_outcome(&self) -> &str {
        &self.collection_outcome
    }

    /// Reject destinations matching an opened input/context name or inode.
    pub fn validate_output_paths(
        &self,
        context: Option<&MigrationContext>,
        paths: &[std::path::PathBuf],
    ) -> Result<()> {
        for destination in paths {
            if aliases(destination, self.path.as_deref(), self.file.as_ref())?
                || context.is_some_and(|c| c.aliases_output(destination))
            {
                return Err(Error::UnsafeOutput);
            }
        }
        Ok(())
    }
}

fn absolute_path(path: &Path) -> Result<std::path::PathBuf> {
    // Existing input paths resolve now; retain their actual path for later alias
    // checks as well as the held inode. No path enters serialized output.
    std::fs::canonicalize(path).map_err(|_| Error::InputUnavailable)
}
fn aliases(
    destination: &Path,
    input_path: Option<&Path>,
    input_file: Option<&File>,
) -> Result<bool> {
    if let Some(input_path) = input_path {
        if std::fs::canonicalize(destination).ok().as_deref() == Some(input_path) {
            return Ok(true);
        }
        if destination
            .parent()
            .and_then(|p| std::fs::canonicalize(p).ok())
            .zip(destination.file_name())
            .is_some_and(|(p, n)| p.join(n) == input_path)
        {
            return Ok(true);
        }
    }
    #[cfg(unix)]
    if let (Some(file), Ok(destination)) = (input_file, std::fs::metadata(destination)) {
        use std::os::unix::fs::MetadataExt;
        let input = file.metadata().map_err(|_| Error::InputUnavailable)?;
        if (input.dev(), input.ino()) == (destination.dev(), destination.ino()) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn target(
    input: &InputReference,
    source_ref: SourceReference,
    algorithm: AlgorithmAssessment,
    observation: ObservationFacts,
    encoding: &str,
    protocol_ref: Option<&str>,
    bindable: bool,
) -> Target {
    let anchor = source_ref.observation_id.as_deref().unwrap_or("attempt1");
    let root = source_ref.root_asset_id.as_deref().unwrap_or("");
    let role_path = source_ref
        .detail_path
        .as_deref()
        .unwrap_or(&source_ref.role);
    let id = native_id(
        "migration_target",
        &[
            input.format.as_str(),
            &input.sha256,
            anchor,
            root,
            role_path,
        ],
    );
    let mut limitations = vec![];
    if algorithm.identifier == "unknown" {
        limitations.push("unknown_primitive".into());
    }
    if encoding == "unknown" {
        limitations.push("unknown_parameter_encoding".into());
    }
    if observation.collection_outcome != "complete" && observation.collection_outcome != "completed"
    {
        limitations.push("collection_incomplete".into());
    }
    if observation.assurance == "unauthenticated_advertisement" {
        limitations.push("unauthenticated_advertisement".into());
    }
    if observation.assurance == "unverified_or_incomplete" {
        limitations.push("verification_not_completed".into());
    }
    if algorithm.combiner_assessment.is_some() {
        limitations.push("combiner_implementation_unverified".into());
    }
    if !bindable {
        limitations.push("unknown_binding".into());
    }
    if source_ref.role == "reference-only" {
        limitations.push("reference_only".into());
    }
    let role = source_ref.role.clone();
    Target {
        id,
        source_ref,
        technical: TechnicalFacts {
            algorithm,
            encoding_assurance: encoding.into(),
            protocol_ref: protocol_ref.map(str::to_owned),
            role,
        },
        observation,
        limitations,
        bindable,
    }
}

fn schema(
    value: &Value,
    body: &str,
    cache: &std::sync::OnceLock<Result<jsonschema::Validator>>,
) -> Result<()> {
    // All schemas are repository constants with local references only. Features
    // enabling HTTP/file retrieval are disabled; no input selects a schema URI.
    let validator = cache
        .get_or_init(|| {
            let schema: Value = serde_json::from_str(body).map_err(|_| Error::InvalidInput)?;
            jsonschema::validator_for(&schema).map_err(|_| Error::InvalidInput)
        })
        .as_ref()
        .map_err(|e| *e)?;
    if !validator.is_valid(value) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}
