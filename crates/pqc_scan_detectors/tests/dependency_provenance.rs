use pqc_scan_core::{
    scan_repository_native, Detector, DetectorStatus, NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_detectors::dependency_detector::DependencyDetector;
use pqc_scan_rules::RuleSet;
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
struct Fixture(PathBuf);
impl Fixture {
    fn new(tag: &str) -> Self {
        let p = std::env::temp_dir().join(format!("pqc027-{}-{tag}", std::process::id()));
        std::fs::create_dir_all(p.join("input")).unwrap();
        std::fs::create_dir_all(p.join("rules")).unwrap();
        std::fs::write(p.join("rules/test.yml"),r#"[{"id":"DEP_TEST","kind":"dependency","category":"dependency","severity":"medium","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"openssl|ring","scope":"code"}]"#).unwrap();
        Self(p)
    }
    fn rules(&self) -> RuleSet {
        RuleSet::load_from_dir(&self.0.join("rules")).unwrap()
    }
    fn inspect(&self, name: &str, text: &str) -> pqc_scan_core::DetectorOutcome {
        DependencyDetector
            .detect_detailed(
                &ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec()),
                &self.rules(),
            )
            .unwrap()
    }
    fn native(&self) -> Value {
        let c = ScanConfig {
            root: self.0.join("input"),
            threads: 1,
            ..Default::default()
        };
        let d: Vec<Arc<dyn Detector>> = vec![Arc::new(DependencyDetector)];
        let scan = scan_repository_native(
            &c,
            &self.rules(),
            &d,
            &NativeOptions::new(Some("test")).unwrap(),
        )
        .unwrap();
        serde_json::to_value(scan.native().unwrap()).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn structured_outcomes_distinguish_empty_invalid_partial_and_unknown_versions() {
    let f = Fixture::new("outcomes");
    for (name, text, status) in [
        (
            "package.json",
            r#"{"dependencies":{}}"#,
            DetectorStatus::Complete,
        ),
        (
            "package.json",
            r#"{"dependencies":{"openssl":"^1.2.3","ring":12}}"#,
            DetectorStatus::Partial,
        ),
        (
            "package-lock.json",
            r#"{"lockfileVersion":99,"packages":{}}"#,
            DetectorStatus::Unsupported,
        ),
        ("Cargo.toml", "[dependencies]\n", DetectorStatus::Complete),
        (
            "README.txt",
            "no dependencies",
            DetectorStatus::NotApplicable,
        ),
        ("Gemfile", "gem ENV['NAME']", DetectorStatus::NotReported),
    ] {
        assert_eq!(f.inspect(name, text).status(), status, "{name}: {text}");
    }
    for (name, text) in [("package.json", "{"), ("Cargo.toml", "[dependencies\n")] {
        assert!(DependencyDetector
            .detect_detailed(
                &ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec()),
                &f.rules()
            )
            .is_err());
    }
    let binary = ScannableFile::from_bytes(PathBuf::from("package.json"), vec![0, 255]);
    assert_eq!(
        DependencyDetector
            .detect_detailed(&binary, &f.rules())
            .unwrap()
            .status(),
        DetectorStatus::Unsupported
    );
}
#[test]
fn declarations_and_lock_evidence_remain_distinct_without_inferred_resolution() {
    let f = Fixture::new("versions");
    std::fs::write(
        f.0.join("input/package.json"),
        r#"{"dependencies":{"openssl":"^1.2.3"}}"#,
    )
    .unwrap();
    std::fs::write(
        f.0.join("input/package-lock.json"),
        r#"{"lockfileVersion":3,"packages":{"node_modules/openssl":{"version":"2.0.0"}}}"#,
    )
    .unwrap();
    let n = f.native();
    let deps: Vec<_> = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "dependency")
        .collect();
    assert_eq!(deps.len(), 2);
    let manifest = deps
        .iter()
        .find(|a| a["attributes"]["declared_constraint"] == "^1.2.3")
        .unwrap();
    assert!(manifest["attributes"]["resolved_version"].is_null());
    assert_eq!(manifest["attributes"]["reported_version"], "^1.2.3");
    assert!(deps
        .iter()
        .any(|a| a["attributes"]["resolved_version"] == "2.0.0"));
    let out = f.inspect("package.json", r#"{"dependencies":{"openssl":"^1.2.3"}}"#);
    assert!(out
        .detections()
        .iter()
        .all(|d| d.evidence.metadata["dep_purl"] == "pkg:npm/openssl"));
    let lock = f.inspect(
        "package-lock.json",
        r#"{"lockfileVersion":3,"packages":{"node_modules/openssl":{"version":"2.0.0"}}}"#,
    );
    assert!(lock
        .detections()
        .iter()
        .all(|d| d.evidence.metadata["dep_purl"] == "pkg:npm/openssl@2.0.0"));
}
#[test]
fn workspace_path_git_and_unsafe_locators_do_not_fabricate_registry_versions() {
    let f = Fixture::new("sources");
    std::fs::write(
        f.0.join("input/Cargo.toml"),
        r#"[dependencies]
openssl = { workspace = true }
ring = { path = "../private", version = "^0.17" }
safe = { git = "https://TOKEN_CANARY@host/private", version = "1.2" }
"#,
    )
    .unwrap();
    let n = f.native();
    let deps: Vec<_> = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "dependency")
        .collect();
    assert_eq!(deps.len(), 3);
    assert!(deps
        .iter()
        .all(|a| a["attributes"]["resolved_version"].is_null()));
    assert!(!n.to_string().contains("CANARY"));
    assert!(deps
        .iter()
        .any(|a| a["attributes"]["version_provenance"] == "workspace_inherited"));
    let out = f.inspect(
        "package.json",
        r#"{"dependencies":{"openssl":"git+https://TOKEN_CANARY@host/repo"}}"#,
    );
    let encoded = serde_json::to_string(
        &out.detections()
            .iter()
            .map(|d| &d.evidence)
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert!(!encoded.contains("CANARY"));
    assert!(out
        .detections()
        .iter()
        .all(|d| !d.evidence.metadata["dep_purl"].contains('@')));
}
#[test]
fn duplicate_input_and_partial_records_are_never_clean_empty() {
    let f = Fixture::new("partial");
    assert!(DependencyDetector
        .detect_detailed(
            &ScannableFile::from_bytes(
                PathBuf::from("package.json"),
                br#"{"dependencies":{},"dependencies":{"openssl":"1"}}"#.to_vec()
            ),
            &f.rules()
        )
        .is_err());
    let o=f.inspect("package-lock.json",r#"{"lockfileVersion":3,"packages":{"node_modules/openssl":{"version":"1.2.3"},"node_modules/ring":7}}"#);
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert!(o
        .detections()
        .iter()
        .any(|d| d.evidence.metadata["dep_name"] == "openssl"));
}

#[test]
fn range_versions_and_nonregistry_lock_sources_are_not_resolved() {
    let f = Fixture::new("lock-negative");
    for version in ["1.x", "1.2.x", "1.+"] {
        let text=serde_json::json!({"lockfileVersion":3,"packages":{"node_modules/openssl":{"version":version}}}).to_string();
        let out = f.inspect("package-lock.json", &text);
        assert_ne!(out.status(), DetectorStatus::Complete);
        assert!(out
            .detections()
            .iter()
            .all(|d| !d.evidence.metadata["dep_purl"].contains('@')));
    }
    for text in [
        r#"{"lockfileVersion":1,"dependencies":{"openssl":{"version":"1.2.3","resolved":"git+https://TOKEN_CANARY@host/repo"}}}"#,
        r#"{"lockfileVersion":3,"packages":{"node_modules/openssl":{"version":"1.2.3","resolved":"https://private.invalid/archive.tgz"}}}"#,
    ] {
        let out = f.inspect("package-lock.json", text);
        assert!(out
            .detections()
            .iter()
            .all(|d| !d.evidence.metadata["dep_purl"].contains('@')));
    }
}
#[test]
fn pnpm_peer_variants_and_root_declarations_survive() {
    let f = Fixture::new("pnpm");
    for (version, key) in [
        ("5.4", "/openssl/1.2.3_peer@2.0.0"),
        ("6.0", "/openssl@1.2.3(peer@2.0.0)"),
        ("9.0", "openssl@1.2.3(peer@2.0.0)"),
    ] {
        let text = format!("lockfileVersion: '{version}'\npackages:\n  '{key}': {{}}\n");
        let out = f.inspect("pnpm-lock.yaml", &text);
        assert_eq!(out.status(), DetectorStatus::Complete);
        assert!(out
            .detections()
            .iter()
            .any(|d| d.evidence.metadata["dep_name"] == "openssl"
                && d.evidence.metadata["dep_purl"] == "pkg:npm/openssl@1.2.3"));
        assert!(out.detections().iter().all(|d| d.location.line == 3));
    }
    let out=f.inspect("pnpm-lock.yaml","lockfileVersion: '5.4'\nspecifiers:\n  openssl: ^1.2.3\ndependencies:\n  openssl: 1.2.4\npackages: {}\n");
    assert!(out
        .detections()
        .iter()
        .any(|d| d.evidence.metadata["dep_name"] == "openssl"));
}
#[test]
fn same_file_origins_are_distinct_and_ambiguous_line_is_not_guessed() {
    let f = Fixture::new("origins");
    let text="{\n\"dependencies\":{\"openssl\":\"^1.2.3\"},\n\"devDependencies\":{\"openssl\":\"^1.2.3\"}\n}";
    std::fs::write(f.0.join("input/package.json"), text).unwrap();
    let n = f.native();
    let deps: Vec<_> = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "dependency")
        .collect();
    assert_eq!(deps.len(), 2);
    assert_ne!(deps[0]["id"], deps[1]["id"]);
    let out = f.inspect("package.json", text);
    assert!(out.detections().iter().all(|d| d.location.line == 0));
}

#[test]
fn ecosystem_matrix_preserves_declarations_and_only_lock_records_resolve() {
    let f = Fixture::new("matrix");
    let cases=[
 ("Cargo.toml","[dependencies]\nrename = { package = 'openssl', version = '^1.2.3' }\n",false,DetectorStatus::Complete),
 ("Cargo.lock","version = 4\n[[package]]\nname = 'openssl'\nversion = '1.2.3'\nsource = 'registry+https://github.com/rust-lang/crates.io-index'\n",true,DetectorStatus::Complete),
 ("Cargo.lock","version = 4\n[[package]]\nname = 'openssl'\nversion = '1.2.3'\n",false,DetectorStatus::Complete),
 ("package.json",r#"{"dependencies":{"alias":"npm:openssl@^1.2.3"}}"#,false,DetectorStatus::Complete),
 ("Pipfile.lock",r#"{"_meta":{"pipfile-spec":6},"default":{"openssl":{"version":"==1.2.3"}}}"#,true,DetectorStatus::Complete),
 ("poetry.lock","[[package]]\nname = 'openssl'\nversion = '1.2.3'\n[metadata]\nlock-version = '2.1'\n",true,DetectorStatus::Complete),
 ("requirements.txt","openssl>=1.2.3\n",false,DetectorStatus::Complete),
 ("go.mod","module example.invalid/app\nrequire example.invalid/openssl v1.2.3\n",false,DetectorStatus::Complete),
 ("Gemfile","gem 'openssl', '~> 1.2'\n",false,DetectorStatus::Complete),
 ("Gemfile.lock","GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl (1.2.3)\n",true,DetectorStatus::Complete),
 ("Gemfile.lock","GIT\n  remote: https://private.invalid/repo\n  specs:\n    openssl (1.2.3)\n",false,DetectorStatus::Partial),
 ("gradle.lockfile","org.example:openssl:1.2.3=compileClasspath\n",true,DetectorStatus::NotReported),
 ("pom.xml","<project><dependencies><dependency><groupId>org.example</groupId><artifactId>openssl</artifactId><version>1.2.3</version></dependency></dependencies></project>",false,DetectorStatus::Complete),
 ("bom.json",r#"{"bomFormat":"CycloneDX","specVersion":"1.7","components":[{"name":"openssl","version":"1.2.3","purl":"pkg:npm/openssl@1.2.3"}]}"#,false,DetectorStatus::Complete),
 ("sbom.json",r#"{"spdxVersion":"SPDX-2.3","packages":[{"name":"openssl","versionInfo":"1.2.3","externalRefs":[{"referenceType":"purl","referenceLocator":"pkg:npm/openssl@1.2.3"}]}]}"#,false,DetectorStatus::Complete)
 ];
    for (i, (name, text, resolved, status)) in cases.into_iter().enumerate() {
        let case = Fixture::new(&format!("eco-{i}"));
        std::fs::write(case.0.join("input").join(name), text).unwrap();
        let outcome = f.inspect(name, text);
        assert_eq!(outcome.status(), status, "{name}: {text}");
        let n = case.native();
        let deps: Vec<_> = n["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["kind"] == "dependency")
            .collect();
        assert!(!deps.is_empty(), "{name}: {text}");
        assert_eq!(
            deps.iter()
                .any(|a| a["attributes"]["resolved_version"].is_string()),
            resolved,
            "{name}: {text}"
        );
        assert!(
            outcome
                .detections()
                .iter()
                .all(|d| d.evidence.metadata["dep_purl"].contains("@1.2.3") == resolved),
            "{name}: {text}"
        );
    }
}
#[test]
fn parser_resource_limits_are_explicit_and_never_expand_aliases() {
    let f = Fixture::new("limits");
    let deep = format!(
        "{{\"dependencies\":{},\"x\":{}{}}}",
        "{}",
        "[".repeat(70),
        "]".repeat(70)
    );
    assert_eq!(
        f.inspect("package.json", &deep).status(),
        DetectorStatus::Truncated
    );
    assert_eq!(
        f.inspect("package.json", &" ".repeat(2 * 1024 * 1024 + 1))
            .status(),
        DetectorStatus::Truncated
    );
    assert_eq!(
        f.inspect(
            "pnpm-lock.yaml",
            "lockfileVersion: '9.0'\na: &x [one]\nb: *x\n"
        )
        .status(),
        DetectorStatus::Unsupported
    );
}

#[test]
fn malformed_known_record_fields_and_node_limits_cannot_be_complete() {
    let f = Fixture::new("typed-invalid");
    for (name,text) in [("package.json",r#"{"dependencies":{"openssl":"${VERSION}"}}"#),("Cargo.toml","[dependencies]\nopenssl = { git = 7 }\n"),("Cargo.lock","version = 4\n[[package]]\nname = 'openssl'\nversion = '1.2.3'\nsource = 7\n"),("Pipfile.lock",r#"{"_meta":{"pipfile-spec":6},"default":{"openssl":{"git":7}}}"#),("poetry.lock","[[package]]\nname = 'openssl'\nversion = '1.2.3'\nsource = {}\n[metadata]\nlock-version = '2.1'\n"),("pnpm-lock.yaml","lockfileVersion: '9.0'\npackages:\n  openssl@1.2.3:\n    resolution: 7\n")] {let result=DependencyDetector.detect_detailed(&ScannableFile::from_bytes(PathBuf::from(name),text.as_bytes().to_vec()),&f.rules());assert!(result.as_ref().map_or(true,|o|o.status()!=DetectorStatus::Complete),"{name}: {text}");}
    let text = format!(
        "{{\"x\":[{}]}}",
        std::iter::repeat_n("0", 50001)
            .collect::<Vec<_>>()
            .join(",")
    );
    assert_eq!(
        f.inspect("package.json", &text).status(),
        DetectorStatus::Truncated
    );
}

#[test]
fn sbom_display_label_uses_validated_maven_identity_without_resolution_claim() {
    let f = Fixture::new("sbom-label");
    let text = r#"{"spdxVersion":"SPDX-2.3","packages":[{"name":"bouncycastle","versionInfo":"1.78","externalRefs":[{"referenceType":"purl","referenceLocator":"pkg:maven/org.bouncycastle/bcprov@1.78"}]}]}"#;
    let outcome = f.inspect("sbom.spdx.json", text);
    assert_eq!(outcome.status(), DetectorStatus::Complete);
    let d = &outcome.detections()[0];
    assert_eq!(d.evidence.metadata["dep_name"], "org.bouncycastle:bcprov");
    assert_eq!(d.evidence.metadata["dep_ecosystem"], "maven");
    assert_eq!(
        d.evidence.metadata["dep_purl"],
        "pkg:maven/org.bouncycastle/bcprov"
    );
}
