//! Dependency assertions and a compatible flat projection; no package resolution.
mod decode;
mod parse;
use anyhow::Result;
use pqc_scan_core::model::mask_preview;
use pqc_scan_core::observation::{DependencyOrigin, DependencyVersionFacts};
use pqc_scan_core::{
    Detection, Detector, DetectorOutcome, Evidence, Location, ObservationDraft, ScannableFile,
};
use pqc_scan_rules::{RuleKind, RuleSet};
use std::collections::BTreeMap;
const INVENTORY_RULE_ID: &str = "__DEPENDENCY_INVENTORY__";
#[derive(Debug, Default)]
pub struct DependencyDetector;
#[derive(Debug, Clone, Default)]
struct DependencyItem {
    name: String,
    version: String,
    ecosystem: String,
    purl: String,
    line: usize,
    source_type: String,
    declared: Option<String>,
    resolved: Option<String>,
    reported: Option<String>,
    origin: DependencyOrigin,
    ordinal: usize,
    parsed: bool,
}
impl DependencyDetector {
    fn inspect(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        observe: bool,
    ) -> Result<DetectorOutcome> {
        let parsed = parse::inspect(file)?;
        let mut detections = Vec::new();
        let mut drafts = Vec::new();
        let path = file.path.to_string_lossy();
        for item in parsed.items {
            let mut hits = vec![inventory_detection(&path, &item, self.name())];
            let target = format!(
                "{} {} {} {}",
                item.name, item.version, item.ecosystem, item.purl
            );
            for rule in rules.by_kind(RuleKind::Dependency) {
                if !rule.compiled_pattern().is_some_and(|r| r.is_match(&target)) {
                    continue;
                }
                let mut d = inventory_detection(&path, &item, self.name());
                d.rule_id = rule.id.clone();
                d.evidence.r#type = if item.source_type == "sbom" {
                    "sbom_dependency"
                } else {
                    "dependency"
                }
                .into();
                d.evidence.metadata.remove("inventory_type");
                d.evidence.metadata.insert(
                    "manifest".into(),
                    file.file_name().unwrap_or("unknown").into(),
                );
                d.evidence.snippet_preview = mask_preview(&format!(
                    "{} {} {}",
                    item.ecosystem, item.name, item.version
                ));
                hits.push(d);
            }
            if observe {
                drafts.extend(hits.iter().map(|d| {
                    ObservationDraft::dependency_assertion(
                        d,
                        DependencyVersionFacts {
                            declared: item.declared.as_deref(),
                            resolved: item.resolved.as_deref(),
                            reported: item.reported.as_deref(),
                            origin: item.origin,
                            ordinal: item.ordinal,
                            parsed: item.parsed,
                        },
                    )
                }));
            }
            detections.extend(hits);
        }
        Ok(DetectorOutcome::new(detections, parsed.status).with_observations(drafts))
    }
}
impl Detector for DependencyDetector {
    fn name(&self) -> &'static str {
        "dependency_detector"
    }
    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.inspect(file, rules, false)?.into_detections())
    }
    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.inspect(file, rules, false)
    }
    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.inspect(file, rules, true)
    }
}
fn inventory_detection(path: &str, item: &DependencyItem, producer: &str) -> Detection {
    let mut metadata = BTreeMap::from([
        ("detector".into(), producer.into()),
        ("inventory_type".into(), "dependency".into()),
        ("dep_name".into(), item.name.clone()),
        ("dep_version".into(), item.version.clone()),
        ("dep_ecosystem".into(), item.ecosystem.clone()),
        ("dep_purl".into(), item.purl.clone()),
        ("dep_source_type".into(), item.source_type.clone()),
    ]);
    if matches!(
        item.origin,
        DependencyOrigin::Path
            | DependencyOrigin::Git
            | DependencyOrigin::Archive
            | DependencyOrigin::WorkspaceInherited
            | DependencyOrigin::Unknown
    ) {
        metadata.insert("dep_locator_omitted".into(), "true".into());
    }
    Detection {
        rule_id: INVENTORY_RULE_ID.into(),
        location: Location {
            file: path.into(),
            line: item.line,
            column: usize::from(item.line != 0),
        },
        evidence: Evidence {
            r#type: "dependency_inventory".into(),
            r#match: mask_preview(&format!("{}@{}", item.name, item.version)),
            snippet_preview: mask_preview(&format!("{} {}", item.ecosystem, item.name)),
            metadata,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::Detector;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        dir.push(format!("pqc-scan-{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn dependency_rules(root: &std::path::Path, pattern: &str) -> RuleSet {
        let rules_dir = root.join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(
            rules_dir.join("dependency.yml"),
            format!(
                r#"
- id: DEP_MATCH
  kind: dependency
  category: Dependency
  severity: medium
  risk: quantum-vulnerable
  confidence: 0.8
  migration_hint: migrate
  pattern: "{pattern}"
  scope: code
"#
            ),
        )
        .expect("write rule");
        RuleSet::load_from_dir(&rules_dir).expect("load rules")
    }

    fn detection_for_rule<'a>(
        detections: &'a [Detection],
        rule_id: &str,
        dep_name: &str,
    ) -> &'a Detection {
        detections
            .iter()
            .find(|d| {
                d.rule_id == rule_id
                    && d.evidence.metadata.get("dep_name").map(String::as_str) == Some(dep_name)
            })
            .expect("matching detection exists")
    }

    #[test]
    fn cargo_toml_detects_manifest_dependencies_and_preserves_unknown_workspace_versions() {
        let root = temp_dir("dep-cargo-toml");
        let rules = dependency_rules(&root, "openssl|ring");
        let manifest_path = root.join("Cargo.toml");
        let manifest = r#"[package]
name = "demo"
version = "0.1.0"

[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
serde = { workspace = true }

[build-dependencies]
ring = "0.17"
"#;

        let file = ScannableFile::from_bytes(manifest_path.clone(), manifest.as_bytes().to_vec());
        let detector = DependencyDetector;
        let detections = detector.detect(&file, &rules).expect("run detector");

        let openssl_inventory = detection_for_rule(&detections, INVENTORY_RULE_ID, "openssl");
        assert_eq!(openssl_inventory.location.line, 6);
        assert_eq!(
            openssl_inventory
                .evidence
                .metadata
                .get("dep_purl")
                .map(String::as_str),
            Some("pkg:cargo/openssl")
        );

        let ring_finding = detection_for_rule(&detections, "DEP_MATCH", "ring");
        assert_eq!(ring_finding.location.line, 10);
        assert_eq!(ring_finding.evidence.r#type, "dependency");

        let serde_inventory = detection_for_rule(&detections, INVENTORY_RULE_ID, "serde");
        assert_eq!(
            serde_inventory
                .evidence
                .metadata
                .get("dep_version")
                .map(String::as_str),
            Some("unknown")
        );
    }

    #[test]
    fn cargo_lock_uses_package_block_lines_and_versions() {
        let root = temp_dir("dep-cargo-lock");
        let rules = dependency_rules(&root, "openssl");
        let manifest_path = root.join("Cargo.lock");
        let manifest = r#"[[package]]
name = "openssl"
version = "0.10.64"

[[package]]
name = "serde"
version = "1.0.210"
"#;

        let file = ScannableFile::from_bytes(manifest_path.clone(), manifest.as_bytes().to_vec());
        let detector = DependencyDetector;
        let detections = detector.detect(&file, &rules).expect("run detector");

        let openssl_inventory = detection_for_rule(&detections, INVENTORY_RULE_ID, "openssl");
        assert_eq!(
            openssl_inventory.location.file,
            manifest_path.to_string_lossy()
        );
        assert_eq!(openssl_inventory.location.line, 2);
        assert_eq!(
            openssl_inventory
                .evidence
                .metadata
                .get("dep_version")
                .map(String::as_str),
            Some("0.10.64")
        );

        let openssl_finding = detection_for_rule(&detections, "DEP_MATCH", "openssl");
        assert_eq!(openssl_finding.location.line, 2);
        assert_eq!(openssl_finding.location.column, 1);
    }

    #[test]
    fn requirements_txt_ignores_comments_and_option_lines() {
        let root = temp_dir("dep-requirements");
        let rules = dependency_rules(&root, "cryptography");
        let manifest_path = root.join("requirements.txt");
        let manifest = r#"# generated
--extra-index-url https://example.invalid/simple
cryptography>=41.0.0
pyopenssl==24.1.0
"#;

        let file = ScannableFile::from_bytes(manifest_path, manifest.as_bytes().to_vec());
        let detector = DependencyDetector;
        let detections = detector.detect(&file, &rules).expect("run detector");

        let crypto_inventory = detection_for_rule(&detections, INVENTORY_RULE_ID, "cryptography");
        assert_eq!(crypto_inventory.location.line, 3);
        assert_eq!(
            crypto_inventory
                .evidence
                .metadata
                .get("dep_version")
                .map(String::as_str),
            Some("41.0.0")
        );
        assert!(!detections.iter().any(|d| {
            d.evidence.metadata.get("dep_name").map(String::as_str) == Some("extra-index-url")
        }));
    }

    #[test]
    fn cyclonedx_sbom_dependencies_use_sbom_metadata_and_purl_ecosystem() {
        let root = temp_dir("dep-sbom");
        let rules = dependency_rules(&root, "openssl");
        let manifest_path = root.join("service-bom.cdx.json");
        let manifest = r#"{
  "bomFormat": "CycloneDX",
  "components": [
    {
      "type": "library",
      "name": "openssl",
      "version": "1.1.1w",
      "purl": "pkg:generic/openssl@1.1.1w"
    }
  ]
}"#;

        let file = ScannableFile::from_bytes(manifest_path.clone(), manifest.as_bytes().to_vec());
        let detector = DependencyDetector;
        let detections = detector.detect(&file, &rules).expect("run detector");

        let inventory = detection_for_rule(&detections, INVENTORY_RULE_ID, "openssl");
        assert_eq!(
            inventory
                .evidence
                .metadata
                .get("dep_source_type")
                .map(String::as_str),
            Some("sbom")
        );
        assert_eq!(
            inventory
                .evidence
                .metadata
                .get("dep_ecosystem")
                .map(String::as_str),
            Some("generic")
        );

        let finding = detection_for_rule(&detections, "DEP_MATCH", "openssl");
        assert_eq!(finding.location.file, manifest_path.to_string_lossy());
        assert_eq!(finding.evidence.r#type, "sbom_dependency");
        assert_eq!(
            finding
                .evidence
                .metadata
                .get("dep_purl")
                .map(String::as_str),
            Some("pkg:generic/openssl")
        );
    }

    #[test]
    fn package_json_dependency_detection_reports_actual_line() {
        let root = temp_dir("dep-line");
        let rules = dependency_rules(&root, "openssl");

        let manifest_path = root.join("package.json");
        let manifest = r#"{
  "name": "demo",
  "version": "1.0.0",
  "dependencies": {
    "openssl": "1.0.2"
  }
}"#;
        fs::write(&manifest_path, manifest).expect("write manifest");

        let file = ScannableFile::from_bytes(manifest_path.clone(), manifest.as_bytes().to_vec());
        let detector = DependencyDetector;
        let detections = detector.detect(&file, &rules).expect("run detector");

        let dep = detections
            .iter()
            .find(|d| d.rule_id == "DEP_MATCH")
            .expect("dependency finding exists");

        assert_eq!(dep.location.file, manifest_path.to_string_lossy());
        assert_eq!(dep.location.line, 5);
        assert_eq!(dep.location.column, 1);
    }
}
