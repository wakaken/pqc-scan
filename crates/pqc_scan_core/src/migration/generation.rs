//! Generate the adopted context from explicit selections; never infer claims.
use super::*;

/// Duplicate-aware, bounded JSON at an authoring boundary, before Value can erase keys.
#[derive(Debug)]
pub struct ClaimValue(Value);
impl ClaimValue {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let value = bounded::parse(bytes, LIMIT)?;
        reject_null(&value)?;
        Ok(Self(value))
    }
    pub fn into_value(self) -> Value {
        self.0
    }
}
impl<'de> Deserialize<'de> for ClaimValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = bounded::deserialize_value(d)?;
        reject_null(&value).map_err(serde::de::Error::custom)?;
        Ok(Self(value))
    }
}

/// One explicitly supplied answer. `None` means explicit unknown, not omission.
#[derive(Debug, Clone)]
pub struct ClaimAnswer {
    pub field: String,
    pub value: Option<Value>,
}

/// Validated existing claim set, shared by headless and local UI callers.
#[derive(Debug, Clone, Default)]
pub struct ContextClaims(std::sync::Arc<ClaimSet>);
impl ContextClaims {
    pub fn from_answers(answers: &[ClaimAnswer], provenance: Provenance) -> Result<Self> {
        Self::from_answers_with_references(answers, provenance, &[])
    }

    /// Author existing sourced claims without bypassing reference validation.
    pub fn from_answers_with_references(
        answers: &[ClaimAnswer],
        provenance: Provenance,
        references: &[ContextReference],
    ) -> Result<Self> {
        if answers.len() > 20 {
            return Err(Error::ResourceLimit);
        }
        let mut set = serde_json::Map::new();
        let mut seen = BTreeSet::new();
        for answer in answers {
            if !seen.insert(&answer.field) {
                return Err(Error::InvalidInput);
            }
            let claim = match &answer.value {
                Some(value) => {
                    serde_json::json!({"state":"known","value":value,"provenance":provenance})
                }
                None => serde_json::json!({"state":"unknown","provenance":provenance}),
            };
            if let Some(name) = answer.field.strip_prefix("support.") {
                if !SUPPORT_NAMES.contains(&name) {
                    return Err(Error::InvalidInput);
                }
                set.entry("support")
                    .or_insert_with(|| serde_json::json!({}))
                    .as_object_mut()
                    .ok_or(Error::InvalidInput)?
                    .insert(name.to_owned(), claim);
            } else {
                if answer.field == "support" {
                    return Err(Error::InvalidInput);
                }
                set.insert(answer.field.clone(), claim);
            }
        }
        Self::parse_with_references(
            &serde_json::to_vec(&set).map_err(|_| Error::InvalidInput)?,
            references,
        )
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::parse_with_references(bytes, &[])
    }
    fn parse_with_references(bytes: &[u8], references: &[ContextReference]) -> Result<Self> {
        let value = bounded::parse(bytes, LIMIT)?;
        reject_null(&value)?;
        let mut claims: ClaimSet =
            serde_json::from_value(value).map_err(|_| Error::InvalidInput)?;
        validate_set(&mut claims, &reference_ids(references)?)?;
        Ok(Self(std::sync::Arc::new(claims)))
    }
}

impl Provenance {
    /// The caller supplies the assertion instant; absence of expiry stays unknown.
    pub fn operator(actor: &str, asserted_at: &str, valid_until: Option<&str>) -> Result<Self> {
        let mut result = Self {
            source_kind: SourceKind::OperatorClaim,
            actor_id: actor.to_owned(),
            asserted_at: asserted_at.to_owned(),
            valid_until: valid_until.map(str::to_owned),
            reference_id: None,
        };
        provenance(&mut result, &BTreeSet::new())?;
        Ok(result)
    }
}

/// Zero-based index into the exact SavedEvidence target list, never an asset-wide match.
#[derive(Debug, Clone)]
pub struct ContextSelection {
    pub target_index: usize,
    pub system_id: String,
    pub claims: ContextClaims,
}

pub fn generate_context(
    evidence: &SavedEvidence,
    organization_id: &str,
    selections: &[ContextSelection],
    as_of: &str,
) -> Result<Vec<u8>> {
    generate_context_with_references(evidence, organization_id, selections, as_of, &[])
}

/// Generate the existing context schema with explicitly supplied scoped references.
pub fn generate_context_with_references(
    evidence: &SavedEvidence,
    organization_id: &str,
    selections: &[ContextSelection],
    as_of: &str,
    references: &[ContextReference],
) -> Result<Vec<u8>> {
    reference_ids(references)?;
    if selections.len() > 4096 {
        return Err(Error::ResourceLimit);
    }
    let mut systems = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut bindings = Vec::new();
    let mut minimum_size = 0usize;
    for selection in selections {
        let target = evidence
            .targets()
            .get(selection.target_index)
            .ok_or(Error::InvalidReference)?;
        if !seen.insert(&target.id) {
            return Err(Error::AmbiguousContextBinding);
        }
        if !target.bindable {
            return Err(Error::AmbiguousContextBinding);
        }
        if selection.system_id.len() > 64 {
            return Err(Error::ResourceLimit);
        }
        if !alias(&selection.system_id) {
            return Err(Error::InvalidInput);
        }
        systems.insert(selection.system_id.clone());
        if systems.len() > 256 {
            return Err(Error::ResourceLimit);
        }
        let mut row = serde_json::to_value(Binding {
            target_id: target.id.clone(),
            system_id: selection.system_id.clone(),
            claims: None,
        })
        .map_err(|_| Error::InvalidInput)?;
        row["claims"] =
            serde_json::to_value(selection.claims.0.as_ref()).map_err(|_| Error::InvalidInput)?;
        omit_absent(&mut row);
        // A lower bound cannot reject any valid final pretty representation.
        // Check before retaining another row; shared claims never multiply upfront.
        minimum_size = minimum_size
            .checked_add(
                serde_json::to_vec(&row)
                    .map_err(|_| Error::InvalidInput)?
                    .len(),
            )
            .filter(|n| *n <= LIMIT)
            .ok_or(Error::ResourceLimit)?;
        bindings.push(row);
    }

    let input = Input {
        format: FORMAT.to_owned(),
        version: 1,
        input: InputBinding {
            format: evidence.input().format,
            sha256: evidence.input().sha256.clone(),
        },
        organization_id: organization_id.to_owned(),
        systems: systems
            .into_iter()
            .map(|id| System { id, defaults: None })
            .collect(),
        bindings: Vec::new(),
        organization_defaults: None,
        references: references.to_vec(),
    };
    let mut value = serde_json::to_value(&input).map_err(|_| Error::InvalidInput)?;
    value["bindings"] = Value::Array(bindings);
    let bytes = encode_value(value)?;
    let context = MigrationContext::from_bytes(&bytes)?;
    context.bind(evidence, as_of)?;
    Ok(bytes)
}

fn omit_absent(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            map.values_mut().for_each(omit_absent);
        }
        Value::Array(values) => values.iter_mut().for_each(omit_absent),
        _ => {}
    }
}
fn encode_value(mut value: Value) -> Result<Vec<u8>> {
    struct Limited(Vec<u8>);
    impl std::io::Write for Limited {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self
                .0
                .len()
                .checked_add(bytes.len())
                .is_none_or(|n| n > LIMIT)
            {
                return Err(std::io::Error::other("resource_limit"));
            }
            self.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    omit_absent(&mut value);
    let mut output = Limited(Vec::new());
    if serde_json::to_writer_pretty(&mut output, &value).is_err() {
        // Formatting must not reject a supported compact context or drop any fields.
        output.0.clear();
        serde_json::to_writer(&mut output, &value).map_err(|_| Error::ResourceLimit)?;
    }
    Ok(output.0)
}
fn encode(input: &Input) -> Result<Vec<u8>> {
    encode_value(serde_json::to_value(input).map_err(|_| Error::InvalidInput)?)
}

impl MigrationContext {
    /// Export every supported field; typed validation has normalized timestamps.
    /// Call bind against the selected evidence before applying imported context.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        encode(&self.input)
    }
}
