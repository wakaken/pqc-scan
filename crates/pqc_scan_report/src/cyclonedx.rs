//! Loss-aware CycloneDX 1.7 projection of the existing static inventory.
//!
//! This conversion does not perform analysis, resolve packages or validate trust.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pqc_scan_core::{NativeScan, ScanDocument};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct Inventory {
    assets: Vec<Asset>,
    observations: Vec<Observation>,
    native_completeness: String,
    rules_digest: String,
}

#[derive(Deserialize)]
struct Asset {
    id: String,
    kind: String,
    identity_basis: String,
    attributes: Value,
    relations: Vec<Relation>,
    uncertainty: Vec<String>,
}

#[derive(Deserialize)]
struct Relation {
    relationship: String,
    target: String,
}

#[derive(Deserialize)]
struct Observation {
    id: String,
    asset_ids: Vec<String>,
    method: String,
    assurance: String,
    usage: String,
    target: Value,
}

fn property(name: &str, value: impl Into<String>) -> Value {
    json!({"name":format!("pqc-scan:{name}"),
        "value":pqc_scan_core::privacy::redact_text(&value.into())})
}

fn text<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty())
}

fn fingerprint(value: &Value, key: &str) -> Option<Value> {
    let value = text(value, key)?;
    (value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit()))
        .then(|| json!({"alg":"SHA-256","content":value}))
}

fn relation<'a>(
    asset: &'a Asset,
    kind: &str,
    target_kind: &str,
    ids: &BTreeMap<&str, &str>,
) -> Option<&'a str> {
    let mut values = asset
        .relations
        .iter()
        .filter(|r| r.relationship == kind && ids.get(r.target.as_str()) == Some(&target_kind));
    let first = values.next()?;
    values.next().is_none().then_some(first.target.as_str())
}

fn public_crypto(asset: &Asset, ids: &BTreeMap<&str, &str>) -> Option<(String, Value)> {
    let a = &asset.attributes;
    match asset.kind.as_str() {
        "algorithm" => {
            let identifier = text(a, "identifier")?;
            let mut crypto = json!({"assetType":"algorithm"});
            if text(a, "namespace") == Some("oid")
                && identifier.split('.').count() >= 2
                && identifier
                    .split('.')
                    .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
            {
                crypto["oid"] = json!(identifier);
            }
            // Do not guess primitive, strength, parameters or certification from a name.
            Some((pqc_scan_core::privacy::redact_text(identifier), crypto))
        }
        "public_key" => {
            let mut key = json!({"type":"public-key","fingerprint":fingerprint(a,"spki_sha256")?});
            if let Some(bits) = a.get("key_bits").and_then(Value::as_u64).filter(|n| *n > 0) {
                key["size"] = json!(bits);
            }
            if let Some(id) = relation(asset, "algorithm", "algorithm", ids) {
                key["algorithmRef"] = json!(id);
            }
            Some((
                "Parsed public key".into(),
                json!({"assetType":"related-crypto-material","relatedCryptoMaterialProperties":key}),
            ))
        }
        "certificate" => {
            fingerprint(a, "certificate_sha256")?;
            // Inventory stores the whole DER hash. CycloneDX's certificate
            // fingerprint excludes the signature; that value was not collected.
            let mut certificate = json!({"certificateFormat":"X.509"});
            for (source, target) in [
                ("not_before_unix", "notValidBefore"),
                ("not_after_unix", "notValidAfter"),
            ] {
                if let Some(time) = a
                    .get(source)
                    .and_then(Value::as_i64)
                    .and_then(|seconds| chrono::DateTime::from_timestamp(seconds, 0))
                {
                    certificate[target] =
                        json!(time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
                }
            }
            for (source, target_kind, target) in [
                ("subject_key", "public_key", "subjectPublicKeyRef"),
                ("signature_algorithm", "algorithm", "signatureAlgorithmRef"),
            ] {
                if let Some(id) = relation(asset, source, target_kind, ids) {
                    certificate[target] = json!(id);
                }
            }
            Some((
                "Parsed public certificate".into(),
                json!({"assetType":"certificate","certificateProperties":certificate}),
            ))
        }
        _ => None,
    }
}

fn provenance(asset: &Asset, observations: &[Observation]) -> Vec<Value> {
    let mut properties = vec![property("identity-basis", &asset.identity_basis)];
    if !asset.uncertainty.is_empty() {
        properties.push(property("uncertainty", asset.uncertainty.join(",")));
    }
    // Select fields explicitly; no raw evidence, snippets or arbitrary attributes.
    let mut observations: Vec<_> = observations
        .iter()
        .filter(|o| o.asset_ids.contains(&asset.id))
        .map(|o| {
            json!({"id":o.id,"method":o.method,"assurance":o.assurance,"usage":o.usage,
            "path":o.target.get("path"),"line":o.target.get("line"),"column":o.target.get("column"),
            "column_kind":o.target.get("column_kind")})
        })
        .collect();
    observations.sort_by_key(Value::to_string);
    if !observations.is_empty() {
        properties.push(property(
            "observations",
            Value::Array(observations).to_string(),
        ));
    }
    properties
}

fn software(asset: &Asset, observations: &[Observation]) -> Option<Value> {
    let a = &asset.attributes;
    let name = text(a, "name")?;
    let mut properties = provenance(asset, observations);
    for key in [
        "ecosystem",
        "declared_constraint",
        "reported_version",
        "version_provenance",
    ] {
        if let Some(value) = text(a, key) {
            properties.push(property(key, value));
        }
    }
    properties.push(property(
        "evidence-scope",
        "software_source_assertions_not_installed_packages",
    ));
    let mut result = json!({"type":"library","name":pqc_scan_core::privacy::redact_text(name),
        "bom-ref":asset.id,"properties":properties});
    if text(a, "version_provenance") == Some("lock_resolved") {
        if let (Some(ecosystem), Some(version)) =
            (text(a, "ecosystem"), text(a, "resolved_version"))
        {
            if pqc_scan_core::observation::resolved_dependency_version(ecosystem, version) {
                result["version"] = json!(version);
            }
        }
    }
    Some(result)
}

fn document(
    components: Vec<Value>,
    native: &Inventory,
    kind: &str,
    losses: BTreeMap<&str, u64>,
) -> Value {
    json!({
        "$schema":"http://cyclonedx.org/schema/bom-1.7.schema.json",
        "bomFormat":"CycloneDX","specVersion":"1.7","version":1,
        "metadata":{"tools":{"components":[{"type":"application","name":"pqc-scan","version":env!("CARGO_PKG_VERSION")}] }},
        "components":components,
        "properties":[
            property("export-kind",kind),
            property("native-completeness",&native.native_completeness),
            property("rules-digest",&native.rules_digest),
            property("evidence-scope","static_references_and_parsed_public_facts_not_runtime_or_trust_verification"),
            property("export-loss",json!({"counts":losses,"notes":[
                "no_inferred_dependency_graph_or_runtime_protocols",
                "observations_are_properties_not_complete_native_records",
                "no_security_strength_certification_or_lifecycle_state_inference",
                "legacy_associations_assessments_and_diagnostics_not_projected",
                "unknown_or_unrepresentable_fields_omitted"
            ]}).to_string())
        ]
    })
}

/// Produce distinct crypto/software BOMs from validated inventory v1.
/// IDs remain native bom-refs; unknown asset vocabulary is loss-accounted.
pub fn documents(native: &NativeScan) -> Result<(Value, Value)> {
    native.validate()?;
    // A local v1 adapter avoids exposing mutable core model fields to exporters.
    let mut native: Inventory = serde_json::from_value(serde_json::to_value(native)?)?;
    native.assets.sort_by(|a, b| a.id.cmp(&b.id));
    let ids: BTreeMap<_, _> = native
        .assets
        .iter()
        .filter(|a| match a.kind.as_str() {
            "algorithm" => text(&a.attributes, "identifier").is_some(),
            "certificate" => fingerprint(&a.attributes, "certificate_sha256").is_some(),
            "public_key" => fingerprint(&a.attributes, "spki_sha256").is_some(),
            _ => false,
        })
        .map(|a| (a.id.as_str(), a.kind.as_str()))
        .collect();
    let mut crypto = Vec::new();
    let mut packages = Vec::new();
    let mut crypto_loss = BTreeMap::new();
    let mut software_loss = BTreeMap::new();
    for asset in &native.assets {
        if let Some((name, properties)) = public_crypto(asset, &ids) {
            let projected = [
                properties.pointer("/relatedCryptoMaterialProperties/algorithmRef"),
                properties.pointer("/certificateProperties/subjectPublicKeyRef"),
                properties.pointer("/certificateProperties/signatureAlgorithmRef"),
            ]
            .into_iter()
            .flatten()
            .count();
            let omitted = asset.relations.len().saturating_sub(projected) as u64;
            if omitted > 0 {
                *crypto_loss
                    .entry("unprojected_asset_relations")
                    .or_insert(0) += omitted;
            }
            let mut provenance = provenance(asset, &native.observations);
            if asset.kind == "certificate" {
                if let Some(der_hash) = text(&asset.attributes, "certificate_sha256") {
                    provenance.push(property("certificate-der-sha256", der_hash));
                }
                *crypto_loss
                    .entry("certificate_tbs_fingerprint_unavailable")
                    .or_insert(0) += 1;
            }
            if asset.kind == "algorithm" {
                for key in ["namespace", "purpose"] {
                    if let Some(value) = text(&asset.attributes, key) {
                        provenance.push(property(key, value));
                    }
                }
                if let Some(parameters) =
                    asset.attributes.get("parameters").filter(|v| v.is_object())
                {
                    // Only the native parser's named safe parameter fields are projected.
                    for key in [
                        "parameters_kind",
                        "named_curve_oid",
                        "parameter_interpretation",
                    ] {
                        if let Some(value) = text(parameters, key) {
                            provenance.push(property(key, value));
                        }
                    }
                }
            }
            crypto.push(
                json!({"type":"cryptographic-asset","name":name,"bom-ref":asset.id,
                "cryptoProperties":properties,"properties":provenance}),
            );
            *software_loss
                .entry("crypto_assets_not_software")
                .or_insert(0) += 1;
        } else if asset.kind == "dependency" {
            if let Some(package) = software(asset, &native.observations) {
                packages.push(package);
            } else {
                *software_loss
                    .entry("dependency_identity_unavailable")
                    .or_insert(0) += 1;
            }
            *crypto_loss
                .entry("software_source_assertions_separate_document")
                .or_insert(0) += 1;
        } else {
            let reason = if asset.kind == "usage" {
                "usage_assets_in_observation_properties"
            } else if matches!(asset.kind.as_str(), "certificate" | "public_key") {
                "unavailable_public_object_facts"
            } else {
                "unsupported_asset_kind"
            };
            *crypto_loss.entry(reason).or_insert(0) += 1;
            *software_loss.entry(reason).or_insert(0) += 1;
        }
    }
    for (components, losses) in [(&crypto, &mut crypto_loss), (&packages, &mut software_loss)] {
        let projected: std::collections::BTreeSet<_> = components
            .iter()
            .filter_map(|c| c["bom-ref"].as_str())
            .collect();
        let omitted = native
            .observations
            .iter()
            .filter(|o| !o.asset_ids.iter().any(|id| projected.contains(id.as_str())))
            .count() as u64;
        if omitted > 0 {
            losses.insert("unprojected_observations", omitted);
        }
    }
    Ok((
        document(crypto, &native, "cryptographic_assets", crypto_loss),
        document(
            packages,
            &native,
            "software_source_assertions",
            software_loss,
        ),
    ))
}

/// Write only the separately selected standard outputs; never changes legacy files.
pub fn write_cyclonedx_17(document: &ScanDocument, out_dir: &Path) -> Result<Vec<PathBuf>> {
    let native = document
        .native()
        .context("CycloneDX export requires inventory capture")?;
    let (crypto, software) = documents(native)?;
    std::fs::create_dir_all(out_dir)?;
    let mut paths = Vec::new();
    for (name, value) in [
        ("cbom.cdx-1.7.json", crypto),
        ("dependency-sbom.cdx-1.7.json", software),
    ] {
        let path = out_dir.join(name);
        std::fs::write(&path, serde_json::to_string_pretty(&value)?)
            .with_context(|| format!("failed to write {}", path.display()))?;
        paths.push(path);
    }
    Ok(paths)
}
