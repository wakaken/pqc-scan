//! Exact package identity controls for the approved F054-01 correction.
use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::dependency_detector::DependencyDetector;
use pqc_scan_rules::RuleSet;
use serde_json::json;
use std::path::{Path, PathBuf};

const RULE: &str = "DEP_RUST_RSA_CRATE";
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .unwrap()
}
fn check(name: &str, text: &str, expected: usize) {
    let outcome = DependencyDetector
        .detect_detailed(
            &ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Complete, "{name}: {text}");
    assert_eq!(
        outcome
            .detections()
            .iter()
            .filter(|d| d.rule_id == RULE)
            .count(),
        expected,
        "{name}: {text}"
    );
    // Rejecting a crypto classification must not discard the dependency inventory.
    assert!(outcome
        .detections()
        .iter()
        .any(|d| d.rule_id == "__DEPENDENCY_INVENTORY__"));
}

#[test]
fn exact_names_keep_manifest_aliases_optional_declarations_and_lock_versions() {
    for name in ["rsa", "jsonwebtoken", "RSA", "JsonWebToken"] {
        check(
            "Cargo.toml",
            &format!("[dependencies]\n{name} = '^0.9'\n"),
            1,
        );
        check("Cargo.toml", &format!("[dependencies]\nalias = {{ package = '{name}', version = '^0.9', optional = true }}\n"), 1);
        for version in ["0.9.8", "1.0.0-rc.1", "1.0.0+build.3"] {
            check(
                "Cargo.lock",
                &format!("version = 4\n[[package]]\nname = '{name}'\nversion = '{version}'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n"),
                1,
            );
        }
    }
    // Local/git lock records retain inventory but do not assert registry identity.
    for source in ["", "source = 'git+https://example.invalid/rsa#deadbeef'\n"] {
        check(
            "Cargo.lock",
            &format!("version = 4\n[[package]]\nname = 'rsa'\nversion = '0.9.8'\n{source}"),
            0,
        );
    }
    for name in [
        "universal-hash",
        "rsa-helper",
        "my-rsa",
        "librsa",
        "jsonwebtoken-extra",
        "myjsonwebtoken",
        "rsa_jsonwebtoken",
    ] {
        check(
            "Cargo.toml",
            &format!("[dependencies]\n'{name}' = '0.6.1'\n"),
            0,
        );
        check(
            "Cargo.lock",
            &format!("version = 4\n[[package]]\nname = '{name}'\nversion = '0.6.1'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n"),
            0,
        );
    }
}

#[test]
fn supported_sbom_formats_classify_exact_cargo_packages_only() {
    for (name, expected) in [
        ("rsa", 1),
        ("jsonwebtoken", 1),
        ("universal-hash", 0),
        ("rsa-helper", 0),
        ("myjsonwebtoken", 0),
    ] {
        for ecosystem in ["cargo", "npm"] {
            let count = if ecosystem == "cargo" { expected } else { 0 };
            let purl = format!("pkg:{ecosystem}/{name}@0.6.1");
            check("bom.cdx.json", &json!({"bomFormat":"CycloneDX","specVersion":"1.7","components":[{"type":"library","name":name,"version":"0.6.1","purl":purl}]}).to_string(), count);
            check("sbom.spdx.json", &json!({"spdxVersion":"SPDX-2.3","packages":[{"name":name,"versionInfo":"0.6.1","externalRefs":[{"referenceType":"purl","referenceLocator":purl}]}]}).to_string(), count);
        }
    }
}

#[test]
fn malformed_or_unsupported_purls_never_become_rsa_facts() {
    // Query/subpath parsing was already unsupported: the fix must not add it.
    for purl in [
        "pkg:cargo/rsa@",
        "pkg:cargo/rsa?arch=x",
        "pkg:cargo/rsa#src",
        "pkg:cargo/rsa@0.9.8?arch=x",
        "pkg:cargo/rsa@0.9.8#src",
        "pkg:cargo/namespace/rsa@0.9.8",
        "pkg:cargo/rsa/@0.9.8",
        "https://example.invalid/pkg:cargo/rsa@0.9.8",
        "pkg:cargo/rsa@0.9.8 trailing",
        "pkg:cargo/rsa%2Fextra@0.9.8",
    ] {
        let source = json!({"bomFormat":"CycloneDX","specVersion":"1.7","components":[{"type":"library","name":"rsa","version":"0.9.8","purl":purl}]}).to_string();
        let outcome = DependencyDetector
            .detect_detailed(
                &ScannableFile::from_bytes(PathBuf::from("bom.cdx.json"), source.into_bytes()),
                &rules(),
            )
            .unwrap();
        assert_eq!(outcome.status(), DetectorStatus::Partial, "{purl}");
        assert!(
            !outcome.detections().iter().any(|d| d.rule_id == RULE),
            "{purl}"
        );
    }
}

#[test]
fn compiled_predicate_respects_whole_purl_boundaries_and_keeps_public_rule_fields() {
    let set = rules();
    let rule = set.get(RULE).unwrap();
    let regex = rule.compiled_pattern().unwrap();
    for target in [
        "rsa  cargo pkg:cargo/rsa",
        "jsonwebtoken 0.9.8 cargo pkg:cargo/jsonwebtoken@0.9.8",
        "RSA 1.0.0 cargo pkg:cargo/RSA@1.0.0",
        "rsa 1.0.0+build cargo pkg:cargo/rsa@1.0.0%2Bbuild",
    ] {
        assert!(regex.is_match(target), "{target}");
    }
    for target in [
        "universal-hash 0.6.1 cargo pkg:cargo/universal-hash@0.6.1",
        "rsa-helper 1 cargo pkg:cargo/rsa-helper@1.0.0",
        "rsa 1 cargo pkg:cargo/rsa/extra",
        "rsa 1 cargo pkg:cargo/rsa@",
        "rsa 1 cargo pkg:cargo/rsa@1.0.0?x=y",
        "rsa 1 cargo pkg:cargo/rsa#src",
        "rsa 1 cargo xpkg:cargo/rsa@1.0.0",
        "rsa 1 cargo pkg:cargo/rsa@1.0.0 trailing",
        "rsa 1 cargo pkg:npm/rsa@1.0.0",
    ] {
        assert!(!regex.is_match(target), "{target}");
    }
    let value = serde_json::to_value(rule).unwrap();
    assert_eq!(value["id"], RULE);
    assert_eq!(value["kind"], "dependency");
    assert_eq!(value["scope"], "dependency");
    assert_eq!(value["risk"], "quantum-vulnerable");
    assert_eq!(value["severity"], "medium");
    assert_eq!(value["confidence"], 0.5);
}
