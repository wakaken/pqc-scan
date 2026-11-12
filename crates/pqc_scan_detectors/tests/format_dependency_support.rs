use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::dependency_detector::DependencyDetector;
use pqc_scan_rules::RuleSet;
use std::path::PathBuf;
fn inspect(name: &str, text: &str) -> anyhow::Result<pqc_scan_core::DetectorOutcome> {
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )?;
    DependencyDetector.detect_detailed(
        &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
        &rules,
    )
}
#[test]
fn actual_examples_are_fully_consumed() {
    for name in [
        "go/go.mod",
        "java/pom.xml",
        "python/requirements.txt",
        "ruby/Gemfile",
        "ruby/Gemfile.lock",
    ] {
        let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples")
            .join(name);
        let text = std::fs::read_to_string(&p).unwrap();
        let result = inspect(p.file_name().unwrap().to_str().unwrap(), &text).unwrap();
        assert_eq!(result.status(), DetectorStatus::Complete, "{name}");
        assert!(!result.detections().is_empty());
    }
}
#[test]
fn unsupported_syntax_never_becomes_complete_and_retains_safe_records() {
    for (name, text) in [
        ("requirements.txt", "openssl==1.2.3\n-r secret.txt\n"),
        ("requirements.txt", "openssl==1.2.3\nother; python_version > '3'\n"),
        ("Gemfile", "gem 'openssl'\neval_gemfile 'other'\n"),
        ("Gemfile", "gem 'openssl'\ncustom_dependencies()\n"),
        ("Gemfile.lock", "GEM\n  specs:\n    openssl (1.2.3)\nUNKNOWN\n  ignored\n"),
        ("pom.xml", "<project><dependencies><dependency><groupId>org.example</groupId><artifactId>openssl</artifactId><version>${v}</version></dependency></dependencies><profiles/></project>"),
    ] {
        let result = inspect(name, text).unwrap();
        assert_ne!(result.status(), DetectorStatus::Complete, "{name}: {text}");
        assert!(!result.detections().is_empty(), "{name}");
    }
}
#[test]
fn bounded_declarations_support_bom_comments_and_continuations() {
    for (name, text) in [
        ("go.mod", "\u{feff}// comment\nmodule example.org/app\ngo 1.24\nrequire (\n example.org/openssl v1.2.3 // indirect\n)\n"),
        ("requirements.txt", "\u{feff}# comment\nopenssl>=1.2,\\\n <2 # comment\n"),
        ("Gemfile", "\u{feff}# comment\nsource 'https://rubygems.org'\ngem 'openssl',\\\n '~> 1.2' # comment\n"),
        ("pom.xml", "\u{feff}<project><!--comment--><dependencies><dependency><artifactId>openssl</artifactId><groupId>org.example</groupId><version>1.2.3</version></dependency></dependencies></project>"),
    ] { assert_eq!(inspect(name, text).unwrap().status(), DetectorStatus::Complete, "{name}"); }
}
#[test]
fn sbom_validated_purl_separates_display_label_from_package_identity() {
    let result = inspect("sbom.json", r#"{"spdxVersion":"SPDX-2.3","packages":[{"name":"bouncycastle","versionInfo":"1.78","externalRefs":[{"referenceType":"purl","referenceLocator":"pkg:maven/org.bouncycastle/bcprov-jdk18on@1.78"}]}]}"#).unwrap();
    assert_eq!(result.status(), DetectorStatus::Complete);
    let dep = &result.detections()[0].evidence.metadata;
    assert_eq!(dep["dep_name"], "org.bouncycastle:bcprov-jdk18on");
    assert_eq!(dep["dep_purl"], "pkg:maven/org.bouncycastle/bcprov-jdk18on");
}
#[test]
fn malicious_or_conflicting_purls_are_not_identity_evidence() {
    for purl in [
        "https://user:secret@example.org/pkg:maven/a/b",
        "pkg:maven/a/b@9",
        "pkg:maven/user:secret/b@1.78",
        "pkg:invalid/a/b@1.78",
        "pkg:maven/a/b@1.78?download_url=https://user:secret@example.org",
    ] {
        let text = serde_json::json!({"bomFormat":"CycloneDX","specVersion":"1.6","components":[{"name":"openssl","version":"1.78","purl":purl}]}).to_string();
        let result = inspect("bom.json", &text).unwrap();
        assert_ne!(result.status(), DetectorStatus::Complete, "{purl}");
        assert!(result
            .detections()
            .iter()
            .all(|d| !d.evidence.metadata["dep_purl"].contains("secret")));
    }
}
#[test]
fn malformed_and_unknown_constructs_are_never_complete() {
    for (name, text) in [
        ("Gemfile", "gem 'openssl\"\n"),
        ("Gemfile", "gem 'openssl', '1.2.3' garbage\n"),
        ("requirements.txt", "openssl[crypto]==1.2.3\n"),
        ("requirements.txt", "openssl==1.2.3 --hash=sha256:abc\n"),
        ("go.mod", "require (\nexample.org/openssl v1.2.3\n"),
        ("pom.xml", "<project><dependencies><dependency><groupId>org.example</groupId><artifactId>openssl</artifactId><version>1</version><version>2</version></dependency></dependencies></project>"),
        ("pom.xml", "<project><dependencies magic='ignored'/></project>"),
        ("pom.xml", "<!DOCTYPE project [<!ENTITY x SYSTEM 'file:///secret'>]><project/>"),
        ("Gemfile.lock", "GEM\n  specs:\n    openssl (1.2.3)\n    truncated\n"),
    ] {
        if let Ok(result) = inspect(name, text) { assert_ne!(result.status(), DetectorStatus::Complete, "{name}: {text}"); }
    }
}
#[test]
fn resource_limits_are_honest() {
    assert_eq!(
        inspect("go.mod", &" ".repeat(2 * 1024 * 1024 + 1))
            .unwrap()
            .status(),
        DetectorStatus::Truncated
    );
    let deep = format!(
        "<project>{}{}</project>",
        "<dependencies>".repeat(70),
        "</dependencies>".repeat(70)
    );
    assert_eq!(
        inspect("pom.xml", &deep).unwrap().status(),
        DetectorStatus::Truncated
    );
    let many = format!(
        "module example.org/app\n{}",
        "require example.org/openssl v1.2.3\n".repeat(10001)
    );
    assert_eq!(
        inspect("go.mod", &many).unwrap().status(),
        DetectorStatus::Truncated
    );
}
#[test]
fn explicit_identity_conflicts_and_encoded_credentials_do_not_survive() {
    for (name, group, purl) in [
        (
            "openssl",
            "org.other",
            "pkg:maven/org.example/openssl@1.2.3",
        ),
        ("openssl", "", "pkg:npm/ring@1.2.3"),
        ("openssl", "", "pkg:maven/user%3Asecret/openssl@1.2.3"),
        ("openssl", "", "pkg:npm/%40user%3Asecret/openssl@1.2.3"),
    ] {
        let mut c = serde_json::json!({"name":name,"version":"1.2.3","purl":purl});
        if !group.is_empty() {
            c["group"] = group.into();
        }
        let text =
            serde_json::json!({"bomFormat":"CycloneDX","specVersion":"1.6","components":[c]})
                .to_string();
        let result = inspect("bom.json", &text).unwrap();
        assert_eq!(result.status(), DetectorStatus::Partial, "{purl}");
        assert!(result
            .detections()
            .iter()
            .all(|d| d.evidence.metadata["dep_purl"].is_empty()));
    }
}
#[test]
fn unsupported_static_dependency_locators_keep_safe_package_records() {
    for (name, text) in [
        (
            "Gemfile",
            "gem 'openssl', git: 'https://user:secret@example.org/repo'\n",
        ),
        (
            "requirements.txt",
            "openssl[crypto]>=1.2.3; python_version > '3'\n",
        ),
    ] {
        let result = inspect(name, text).unwrap();
        if name == "Gemfile" {
            assert_eq!(result.status(), DetectorStatus::Complete);
            assert!(result
                .detections()
                .iter()
                .all(|d| d.evidence.metadata["dep_purl"].is_empty()));
        } else {
            assert_ne!(result.status(), DetectorStatus::Complete);
        }
        assert!(!result.detections().is_empty());
        for d in result.detections() {
            assert_eq!(d.evidence.metadata["dep_name"], "openssl");
            assert!(!format!("{:?}", d).contains("secret"));
        }
    }
}
#[test]
fn malformed_supported_directives_and_conflicting_sbom_references_are_incomplete() {
    for (name, text) in [
        ("go.mod", "module example.org/a\nmodule example.org/b\n"),
        ("requirements.txt", "openssl>=!\n"),
        (
            "sbom.json",
            r#"{"spdxVersion":"SPDX-2.3","packages":[{"name":"openssl","versionInfo":"1.2.3","externalRefs":[{"referenceType":"purl","referenceLocator":"pkg:npm/openssl@1.2.3"},{"referenceType":"purl","referenceLocator":"pkg:npm/ring@1.2.3"}]}]}"#,
        ),
    ] {
        assert_ne!(
            inspect(name, text).unwrap().status(),
            DetectorStatus::Complete,
            "{name}"
        );
    }
}
#[test]
fn malformed_sbom_identity_field_types_and_xml_node_limits_are_explicit() {
    for text in [
        r#"{"bomFormat":"CycloneDX","specVersion":"1.6","components":[{"name":"openssl","purl":12}]}"#,
        r#"{"bomFormat":"CycloneDX","specVersion":"1.6","components":[{"name":"openssl","group":12,"purl":"pkg:npm/openssl"}]}"#,
        r#"{"spdxVersion":"SPDX-2.3","packages":[{"name":"openssl","externalRefs":"invalid"}]}"#,
    ] {
        assert_ne!(
            inspect("sbom.json", text).unwrap().status(),
            DetectorStatus::Complete
        );
    }
    let xml = format!("<project>{}</project>", "<name/>".repeat(50001));
    assert_eq!(
        inspect("pom.xml", &xml).unwrap().status(),
        DetectorStatus::Truncated
    );
}
#[test]
fn invalid_version_requirements_are_never_complete_and_keep_package_hits() {
    for (name, text) in [
        ("Gemfile", "gem 'openssl', '>='\n"),
        ("Gemfile", "gem 'openssl', 'garbage'\n"),
        ("Gemfile", "gem 'openssl', '~> 1..2'\n"),
        ("requirements.txt", "openssl>=1!\n"),
        ("requirements.txt", "openssl>=1..2\n"),
        ("requirements.txt", "openssl>=1.*\n"),
    ] {
        let result = inspect(name, text).unwrap();
        assert_ne!(result.status(), DetectorStatus::Complete, "{name}: {text}");
        assert!(!result.detections().is_empty());
    }
}
#[test]
fn purl_only_version_is_reported_without_lock_resolution() {
    let result = inspect("bom.json", r#"{"bomFormat":"CycloneDX","specVersion":"1.6","components":[{"name":"openssl","purl":"pkg:npm/openssl@1.2.3"}]}"#).unwrap();
    assert_eq!(result.status(), DetectorStatus::Complete);
    let m = &result.detections()[0].evidence.metadata;
    assert_eq!(m["dep_version"], "1.2.3");
    assert_eq!(m["dep_purl"], "pkg:npm/openssl");
}
#[test]
fn pom_unconsumed_mixed_text_or_namespaces_never_complete() {
    for text in [
        "<project>ignored<dependencies/></project>",
        "<project><dependencies>ignored</dependencies></project>",
        "<project><dependencies><dependency>ignored<groupId>org.example</groupId><artifactId>openssl</artifactId></dependency></dependencies></project>",
        "<project xmlns='https://unknown.invalid'><dependencies/></project>",
    ] { assert_ne!(inspect("pom.xml", text).unwrap().status(), DetectorStatus::Complete, "{text}"); }
}
#[test]
fn go_and_scoped_npm_purls_keep_valid_source_claims() {
    for (label, purl, expected, version) in [
        (
            "golang.org/x/crypto",
            "pkg:golang/golang.org/x/crypto@v0.29.0",
            "pkg:golang/golang.org/x/crypto",
            "v0.29.0",
        ),
        (
            "@scope/openssl",
            "pkg:npm/@scope/openssl",
            "pkg:npm/%40scope/openssl",
            "unknown",
        ),
        (
            "@scope/openssl",
            "pkg:npm/%40scope/openssl@1.2.3",
            "pkg:npm/%40scope/openssl",
            "1.2.3",
        ),
    ] {
        let text = serde_json::json!({"bomFormat":"CycloneDX","specVersion":"1.6","components":[{"name":label,"purl":purl}]}).to_string();
        let result = inspect("bom.json", &text).unwrap();
        assert_eq!(result.status(), DetectorStatus::Complete, "{purl}");
        let m = &result.detections()[0].evidence.metadata;
        assert_eq!(m["dep_name"], label);
        assert_eq!(m["dep_purl"], expected);
        assert_eq!(m["dep_version"], version);
    }
}
#[test]
fn invalid_purl_versions_retain_independent_safe_namespaced_labels() {
    let text = r#"{"bomFormat":"CycloneDX","specVersion":"1.6","components":[{"name":"golang.org/x/crypto","version":"v0.29.0","purl":"pkg:golang/golang.org/x/crypto@invalid"}]}"#;
    let result = inspect("bom.json", text).unwrap();
    assert_eq!(result.status(), DetectorStatus::Partial);
    assert_eq!(
        result.detections()[0].evidence.metadata["dep_name"],
        "golang.org/x/crypto"
    );
    assert!(result.detections()[0].evidence.metadata["dep_purl"].is_empty());
}
#[test]
fn requirements_comments_require_a_whitespace_boundary() {
    let invalid = inspect("requirements.txt", "openssl==1.2#junk\n").unwrap();
    assert_eq!(invalid.status(), DetectorStatus::Partial);
    assert!(!invalid.detections().is_empty());
    assert!(invalid
        .detections()
        .iter()
        .all(|d| d.evidence.metadata["dep_name"] == "openssl"));
    for text in [
        "openssl==1.2 #comment\n",
        "#comment\nopenssl==1.2\n",
        "  #comment\nopenssl==1.2\n",
    ] {
        assert_eq!(
            inspect("requirements.txt", text).unwrap().status(),
            DetectorStatus::Complete
        );
    }
    assert_eq!(
        inspect("Gemfile", "gem 'openssl'#comment\n")
            .unwrap()
            .status(),
        DetectorStatus::Complete
    );
}

#[test]
fn supported_go_replace_and_exclude_blocks_preserve_unresolved_declarations() {
    for text in ["module example.org/app\nrequire example.org/openssl v1.2.3\nreplace example.org/openssl => ../local\n", "module example.org/app\nexclude (\n example.org/openssl v1.2.3\n)\n"] {
        let result=inspect("go.mod",text).unwrap();assert_eq!(result.status(),DetectorStatus::Complete);
        for d in result.detections(){assert!(!d.evidence.metadata["dep_purl"].contains('@'));}
    }
}
