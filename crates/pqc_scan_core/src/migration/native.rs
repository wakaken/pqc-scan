use super::{
    assess_primitive, bounded, target, Error, InputFormat, InputReference, ObservationFacts,
    Result, SavedEvidence, ScanReference, SourceReference,
};
use crate::{
    assessment::AlgorithmAssessment,
    inventory::{CryptoAsset, NativeScan},
    observation::Observation,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn project(value: Value, hash: String) -> Result<SavedEvidence> {
    // The published writer schema is closed; the published Native reader is
    // extension tolerant. Validate known shape without rejecting future fields.
    static VALIDATOR: std::sync::OnceLock<Result<jsonschema::Validator>> =
        std::sync::OnceLock::new();
    let validator = VALIDATOR
        .get_or_init(|| {
            let mut schema: Value = serde_json::from_str(include_str!(
                "../../../../schemas/native-scan-v1.schema.json"
            ))
            .map_err(|_| Error::InvalidInput)?;
            tolerate_extensions(&mut schema);
            jsonschema::validator_for(&schema).map_err(|_| Error::InvalidInput)
        })
        .as_ref()
        .map_err(|e| *e)?;
    if !validator.is_valid(&value) {
        return Err(Error::InvalidInput);
    }
    let native: NativeScan = serde_json::from_value(value).map_err(|_| Error::InvalidInput)?;
    native.validate().map_err(|_| Error::InvalidReference)?;
    for asset in &native.assets {
        if asset.kind == "certificate" {
            let start = asset.attributes["not_before_unix"]
                .as_i64()
                .ok_or(Error::InvalidInput)?;
            let end = asset.attributes["not_after_unix"]
                .as_i64()
                .ok_or(Error::InvalidInput)?;
            if start > end {
                return Err(Error::ContradictorySavedFacts);
            }
        }
        if asset.kind == "public_key"
            && !asset.attributes["key_bits"].is_null()
            && asset.attributes["key_bits"].as_u64().is_none()
        {
            return Err(Error::InvalidInput);
        }
    }
    if !crate::assessment::valid_id(&native.run.id, "scan")
        || native
            .assets
            .iter()
            .any(|a| !crate::assessment::valid_id(&a.id, "asset"))
        || native
            .observations
            .iter()
            .any(|o| !crate::assessment::valid_id(&o.id, "observation"))
    {
        return Err(Error::InvalidReference);
    }
    if native.assets.len() > 16384
        || native.observations.len() > 32768
        || native.assets.iter().any(|a| a.relations.len() > 64)
        || native.observations.iter().any(|o| o.asset_ids.len() > 64)
    {
        return Err(Error::ResourceLimit);
    }
    bounded::collection_timestamp(&native.run.generated_at)?;
    let input = InputReference {
        format: InputFormat::InventoryV1,
        sha256: hash,
        schema_version: 1,
        collection_time: native.run.generated_at.clone(),
        run_id: Some(native.run.id.clone()),
        attempt_id: None,
        protocol: None,
    };
    let index: BTreeMap<_, _> = native.assets.iter().map(|a| (a.id.as_str(), a)).collect();
    let outcome = match native.coverage["status"].as_str() {
        Some("complete_within_scope") => "complete",
        Some("partial") => "partial",
        _ => "unknown",
    }
    .to_string();
    let mut occurrence_counts = BTreeMap::new();
    for o in &native.observations {
        if let Some(key) = &o.occurrence_key {
            *occurrence_counts.entry(key.as_str()).or_insert(0usize) += 1;
        }
    }
    let local_sources = distinguishable_sources(&native, &index, &occurrence_counts);
    let mut targets = vec![];
    for observation in &native.observations {
        let observation_facts = ObservationFacts {
            method: bounded::vocabulary(
                &observation.method,
                &[
                    "lexical_match",
                    "ast_pattern",
                    "configuration_match",
                    "dependency_report",
                    "private_key_marker",
                    "certificate_parse",
                    "legacy_adapter",
                ],
            ),
            assurance: bounded::vocabulary(
                &observation.assurance,
                &["lexical", "syntactic", "parsed", "unknown"],
            ),
            usage: bounded::vocabulary(
                &observation.usage,
                &["referenced", "configured", "unknown"],
            ),
            collection_outcome: outcome.clone(),
        };
        let bindable = observation
            .occurrence_key
            .as_deref()
            .is_none_or(|key| occurrence_counts[key] == 1)
            && (local_sources.contains(observation.id.as_str())
                || matches!(
                    observation.identity_confidence.as_str(),
                    "weak_structural_anchor" | "run_local_only"
                ) && !observation
                    .asset_ids
                    .iter()
                    .any(|id| index[id.as_str()].attributes["ambiguous"] == true));
        let path = observation
            .target
            .path
            .as_deref()
            .and_then(|p| crate::identity::relative_path(std::path::Path::new(""), p));
        if observation.target.path.is_some() && path.is_none() {
            return Err(Error::InvalidReference);
        }
        let base = SourceReference {
            run_id: Some(native.run.id.clone()),
            observation_id: Some(observation.id.clone()),
            path,
            line: observation.target.line,
            ..Default::default()
        };
        let mut emitted = false;
        for root_id in observation.asset_ids.iter().collect::<BTreeSet<_>>() {
            let root = index[root_id.as_str()];
            let mut roles = vec![];
            match root.kind.as_str() {
                "algorithm" => roles.push(("algorithm", Some(root), None)),
                "public_key" => roles.push((
                    "public-key.algorithm",
                    related(root, "algorithm", "algorithm", &index),
                    Some(root),
                )),
                "certificate" => {
                    roles.push((
                        "certificate.signature_algorithm",
                        related(root, "signature_algorithm", "algorithm", &index),
                        None,
                    ));
                    let key = related(root, "subject_key", "public_key", &index);
                    roles.push((
                        "certificate.subject_key.algorithm",
                        key.and_then(|k| related(k, "algorithm", "algorithm", &index)),
                        key,
                    ));
                }
                _ => continue,
            }
            for (role, algorithm, key) in roles {
                let (mut assessment, encoding) = if let Some(a) = algorithm {
                    facts(a, key)?
                } else {
                    (AlgorithmAssessment::unknown(), "unknown")
                };
                if role == "certificate.signature_algorithm" {
                    use crate::assessment::Purpose;
                    if matches!(assessment.identifier.as_str(), "RSA" | "EC") {
                        assessment.purpose = Purpose::Signature;
                    }
                    if !matches!(assessment.purpose, Purpose::Signature | Purpose::Unknown) {
                        assessment
                            .classical_concerns
                            .push("certificate_signature_purpose_mismatch".into());
                    }
                }
                let mut reference = base.clone();
                reference.root_asset_id = Some(root.id.clone());
                reference.role = role.into();
                if root.kind == "certificate" {
                    reference.certificate_sha256 = root.attributes["certificate_sha256"]
                        .as_str()
                        .map(str::to_owned);
                    reference.not_before_unix = root.attributes["not_before_unix"].as_i64();
                    reference.not_after_unix = root.attributes["not_after_unix"].as_i64();
                }
                if let Some(key) = key {
                    reference.spki_sha256 =
                        key.attributes["spki_sha256"].as_str().map(str::to_owned);
                }
                let mut t = target(
                    &input,
                    reference,
                    assessment,
                    observation_facts.clone(),
                    encoding,
                    None,
                    bindable,
                );
                if algorithm.is_none()
                    || local_sources.contains(observation.id.as_str())
                        && t.technical.algorithm.identifier == "unknown"
                {
                    t.limitations.push("unresolved_role".into());
                    t.bindable = false;
                }
                targets.push(t);
                emitted = true;
                if targets.len() > 65536 {
                    return Err(Error::ResourceLimit);
                }
            }
        }
        if !emitted {
            let mut reference = base;
            reference.role = "reference-only".into();
            let roots: Vec<_> = observation
                .asset_ids
                .iter()
                .filter_map(|id| index.get(id.as_str()).copied())
                .filter(|a| a.kind == "dependency")
                .collect();
            if roots.len() == 1 {
                reference.root_asset_id = Some(roots[0].id.clone());
                reference.dependency_provenance = Some(bounded::vocabulary(
                    roots[0].attributes["version_provenance"]
                        .as_str()
                        .unwrap_or("unknown"),
                    &[
                        "manifest_declared",
                        "manifest_dev_declared",
                        "manifest_peer_declared",
                        "manifest_optional_declared",
                        "workspace_declaration",
                        "workspace_inherited",
                        "path_dependency_unresolved",
                        "git_dependency_unresolved",
                        "archive_dependency_unresolved",
                        "lock_resolved",
                        "lock_record_unverified",
                        "sbom_reported_unverified",
                        "heuristic_declared",
                        "heuristic_lock_unverified",
                        "source_unresolved",
                        "legacy_unclassified",
                        "unavailable_or_unsafe_version_omitted",
                        // Finite v0.5 source contexts; informational provenance
                        // only, never installed/runtime support or assurance.
                        "platform_required",
                        "platform_dev_required",
                        "platform_emulated",
                        "platform_emulation_disabled",
                        "sdk_declared",
                        "target_framework_declared",
                        "lock_framework_reported",
                        "registry_baseline_declared",
                        "feature_declared",
                        "override_declared",
                        "conditional_declaration_unresolved",
                        "build_lock_record_unverified",
                        "build_source_unresolved",
                        "build_requirement_unresolved",
                        "lexical_build_reference",
                        "legacy_package_declared",
                        "central_literal_inherited_not_effective",
                        "dev_path_dependency_unresolved",
                        "dev_git_dependency_unresolved",
                        "dev_archive_dependency_unresolved",
                        "dev_lock_record_unverified",
                        "dev_source_unresolved",
                    ],
                ));
            }
            targets.push(target(
                &input,
                reference,
                AlgorithmAssessment::unknown(),
                observation_facts,
                "unknown",
                None,
                bindable,
            ));
        }
    }
    let mut scan_references: BTreeMap<_, _> = native
        .observations
        .iter()
        .filter_map(|observation| {
            let rule = observation.rule_id.as_ref().filter(|rule| {
                rule.len() <= 128
                    && !rule.starts_with("__")
                    && rule
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
            })?;
            Some((
                observation.id.clone(),
                ScanReference {
                    rule: Some(rule.clone()),
                    findings: Vec::new(),
                },
            ))
        })
        .collect();
    for association in &native.legacy_associations {
        if crate::assessment::valid_id(&association.legacy_id, "finding") {
            for observation in &association.observation_ids {
                scan_references
                    .entry(observation.clone())
                    .or_default()
                    .findings
                    .push(association.legacy_id.clone());
            }
        }
    }
    for reference in scan_references.values_mut() {
        reference.findings.sort_unstable();
        reference.findings.dedup();
    }
    Ok(SavedEvidence {
        input,
        targets,
        scan_references,
        collection_outcome: outcome,
        file: None,
        path: None,
    })
}

fn tolerate_extensions(schema: &mut Value) {
    match schema {
        Value::Object(map) => {
            if map.get("additionalProperties") == Some(&Value::Bool(false)) {
                map.remove("additionalProperties");
            }
            for value in map.values_mut() {
                tolerate_extensions(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                tolerate_extensions(value);
            }
        }
        _ => {}
    }
}

#[derive(Default)]
struct SourceGroup<'a> {
    count: usize,
    invalid: bool,
    members: Vec<&'a str>,
    spans: BTreeSet<(usize, usize)>,
    locations: BTreeSet<(usize, usize)>,
}

// Native weak anchors deliberately remain ambiguous across scans. This exception
// establishes only selection of exact records inside this immutable input, never
// cross-scan correspondence, active use or an algorithm inferred from a position.
fn distinguishable_sources<'a>(
    native: &'a NativeScan,
    index: &BTreeMap<&str, &'a CryptoAsset>,
    occurrence_counts: &BTreeMap<&str, usize>,
) -> BTreeSet<&'a str> {
    let mut owners = BTreeMap::new();
    for observation in &native.observations {
        for id in &observation.asset_ids {
            if index[id.as_str()].kind == "usage" {
                *owners.entry(id.as_str()).or_insert(0usize) += 1;
            }
        }
    }
    let mut groups: BTreeMap<(&str, &str), SourceGroup<'_>> = BTreeMap::new();
    let mut ungrouped_paths = BTreeSet::new();
    let mut unlocated_anchors = BTreeSet::new();
    let mut unknown_group = false;
    for observation in &native.observations {
        let usages: Vec<_> = observation
            .asset_ids
            .iter()
            .map(|id| index[id.as_str()])
            .filter(|a| a.kind == "usage")
            .collect();
        if observation.identity_confidence == "ambiguous_duplicate_anchor"
            && observation.target.path.is_none()
            && usages
                .iter()
                .all(|u| u.attributes["path"].as_str().is_none())
        {
            let anchors: Vec<_> = usages
                .iter()
                .filter_map(|u| u.attributes["anchor"].as_str())
                .collect();
            // A pathless member can belong to any file with this anchor. If no
            // group key survives, no source exception can be established safely.
            unknown_group |= anchors.is_empty();
            unlocated_anchors.extend(anchors);
        }
        if observation.identity_confidence == "ambiguous_duplicate_anchor"
            && (usages.len() != 1
                || usages.iter().any(|u| {
                    u.attributes["path"].as_str() != observation.target.path.as_deref()
                        || u.attributes["anchor"]
                            .as_str()
                            .is_none_or(|a| !crate::assessment::valid_id(a, "anchor"))
                }))
        {
            // Do not drop an ungrouped neighbor from the file's uniqueness checks.
            ungrouped_paths.extend(observation.target.path.as_deref());
            ungrouped_paths.extend(usages.iter().filter_map(|u| u.attributes["path"].as_str()));
        }
        for usage in &usages {
            let (Some(path), Some(anchor)) = (
                usage.attributes["path"].as_str(),
                usage.attributes["anchor"].as_str(),
            ) else {
                continue;
            };
            // Count all members before checking eligibility. A malformed or
            // unlocated neighbor must not make a collision appear unique.
            let group = groups.entry((path, anchor)).or_default();
            group.count += 1;
            if usages.len() != 1
                || owners[usage.id.as_str()] != 1
                || observation
                    .occurrence_key
                    .as_deref()
                    .is_some_and(|key| occurrence_counts[key] != 1)
                || !crate::assessment::valid_id(anchor, "anchor")
                || !source_usage_consistent(observation, usage, index)
            {
                group.invalid = true;
                continue;
            }
            let t = &observation.target;
            let (Some(start), Some(end), Some(line), Some(column)) =
                (t.byte_start, t.byte_end, t.line, t.column)
            else {
                group.invalid = true;
                continue;
            };
            if start >= end
                || line == 0
                || column == 0
                || !matches!(
                    t.column_kind.as_str(),
                    "legacy_mixed" | "unicode_code_points"
                )
                || !group.spans.insert((start, end))
                || !group.locations.insert((line, column))
            {
                group.invalid = true;
                continue;
            }
            group.members.push(observation.id.as_str());
        }
    }
    groups
        .into_iter()
        .filter(|((path, anchor), g)| {
            !unknown_group
                && !unlocated_anchors.contains(anchor)
                && !ungrouped_paths.contains(path)
                && !g.invalid
                && g.count > 1
                && g.members.len() == g.count
                && g.spans
                    .iter()
                    .zip(g.spans.iter().skip(1))
                    .all(|(a, b)| a.1 <= b.0)
        })
        .flat_map(|(_, g)| g.members)
        .collect()
}

fn source_usage_consistent(
    observation: &Observation,
    usage: &CryptoAsset,
    index: &BTreeMap<&str, &CryptoAsset>,
) -> bool {
    let expected_assurance = match observation.method.as_str() {
        "ast_pattern" => "syntactic",
        "lexical_match" | "configuration_match" => "lexical",
        _ => return false,
    };
    let refs: BTreeSet<_> = observation
        .asset_ids
        .iter()
        .filter(|id| id.as_str() != usage.id)
        .map(String::as_str)
        .collect();
    observation.identity_confidence == "ambiguous_duplicate_anchor"
        && observation.assurance == expected_assurance
        && matches!(observation.usage.as_str(), "referenced" | "configured")
        && observation
            .target
            .path
            .as_deref()
            .is_some_and(|p| !p.is_empty() && usage.attributes["path"] == p)
        && observation.target.object_index.is_none()
        && usage.attributes["method"] == observation.method
        && usage.attributes["ambiguous"] == true
        && usage.identity_basis
            == if observation.occurrence_key.is_some() {
                "scoped_weak_source_anchor"
            } else {
                "run_local_only"
            }
        && refs
            .iter()
            .all(|id| index[id].attributes["ambiguous"] != true)
        && usage.relations.len() == refs.len()
        && usage
            .relations
            .iter()
            .all(|r| r.relationship == "references")
        && usage
            .relations
            .iter()
            .map(|r| r.target.as_str())
            .collect::<BTreeSet<_>>()
            == refs
}

fn related<'a>(
    asset: &CryptoAsset,
    role: &str,
    kind: &str,
    index: &BTreeMap<&str, &'a CryptoAsset>,
) -> Option<&'a CryptoAsset> {
    let mut matches = asset.relations.iter().filter(|r| r.relationship == role);
    let first = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    index
        .get(first.target.as_str())
        .copied()
        .filter(|a| a.kind == kind)
}

fn facts(
    algorithm: &CryptoAsset,
    key: Option<&CryptoAsset>,
) -> Result<(AlgorithmAssessment, &'static str)> {
    let id = algorithm.attributes["identifier"].as_str().unwrap_or("");
    let namespace = algorithm.attributes["namespace"].as_str().unwrap_or("");
    let numeric = id.contains('.') && id.bytes().all(|b| b.is_ascii_digit() || b == b'.');
    let draft =
        id.starts_with("Kyber") || id.starts_with("Dilithium") || id == "X25519Kyber768Draft00";
    if !((namespace == "oid" && numeric)
        || (namespace == "draft" && draft)
        || (namespace == "named" && !numeric && !draft))
    {
        return Ok((AlgorithmAssessment::unknown(), "unknown"));
    }
    let mut parameters: BTreeMap<String, String> = algorithm.attributes["parameters"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.iter())
        .filter(|(k, _)| {
            [
                "key_bits",
                "named_curve_oid",
                "parameters_kind",
                "parameter_interpretation",
            ]
            .contains(&k.as_str())
        })
        .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.into())))
        .collect();
    if let Some(key) = key {
        if let Some(bits) = key.attributes["key_bits"].as_u64() {
            if parameters
                .get("key_bits")
                .and_then(|s| s.parse::<u64>().ok())
                .is_some_and(|original| original != bits)
            {
                return Err(Error::ContradictorySavedFacts);
            }
            if parameters
                .get("key_bits")
                .is_some_and(|s| s.parse::<u64>().is_err())
            {
                parameters.insert("parameter_interpretation".into(), "not_available".into());
            }
            parameters.insert("key_bits".into(), bits.to_string());
        }
        if let Some(curve) = key.attributes["named_curve_oid"].as_str() {
            if let Some(original) = parameters.get("named_curve_oid") {
                let known = ["1.2.840.10045.3.1.7", "1.3.132.0.34", "1.3.132.0.35"];
                if known.contains(&original.as_str()) && known.contains(&curve) && original != curve
                {
                    return Err(Error::ContradictorySavedFacts);
                }
                if !known.contains(&original.as_str()) {
                    parameters.insert("parameter_interpretation".into(), "not_available".into());
                }
            }
            parameters.insert("named_curve_oid".into(), curve.into());
        }
    }
    let encoding = if namespace == "oid"
        && parameters
            .get("parameters_kind")
            .is_some_and(|s| s == "absent" || s == "null" || s == "oid")
        && parameters
            .get("parameter_interpretation")
            .is_none_or(|s| s != "not_available")
    {
        "known"
    } else {
        "unknown"
    };
    if !super::technical::parameters_consistent(id, &parameters) {
        return Err(Error::ContradictorySavedFacts);
    }
    Ok((assess_primitive(id, &parameters), encoding))
}
