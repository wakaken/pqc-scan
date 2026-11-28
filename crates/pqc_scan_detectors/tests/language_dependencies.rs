use pqc_scan_core::{
    scan_repository_native, Detector, DetectorStatus, NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_detectors::dependency_detector::DependencyDetector;
use pqc_scan_rules::RuleSet;
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
struct Fixture {
    root: PathBuf,
    rules: RuleSet,
}
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-v05-deps-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(root.join("input")).unwrap();
        Self {
            root,
            rules: RuleSet::load_from_dir(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"),
            )
            .unwrap(),
        }
    }
    fn native(&self, name: &str, text: &str) -> (Value, Value) {
        self.native_files(&[(name, text)], &[])
    }
    fn native_files(&self, files: &[(&str, &str)], excludes: &[&str]) -> (Value, Value) {
        for entry in std::fs::read_dir(self.root.join("input")).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                std::fs::remove_dir_all(p).unwrap()
            } else {
                std::fs::remove_file(p).unwrap()
            }
        }
        for (name, text) in files {
            let path = self.root.join("input").join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        if !excludes.is_empty() {
            std::fs::write(self.root.join("input/.pqcignore"), excludes.join("\n")).unwrap();
        }
        self.scan_at(self.root.join("input"))
    }
    fn scan_at(&self, root: PathBuf) -> (Value, Value) {
        self.scan_with(root, Arc::new(DependencyDetector))
    }
    fn scan_with(&self, root: PathBuf, detector: Arc<dyn Detector>) -> (Value, Value) {
        let detectors: Vec<Arc<dyn Detector>> = vec![detector];
        let doc = scan_repository_native(
            &ScanConfig {
                root,
                threads: 1,
                ..Default::default()
            },
            &self.rules,
            &detectors,
            &NativeOptions::new(Some("v05/dependencies")).unwrap(),
        )
        .unwrap();
        (
            serde_json::to_value(doc.native().unwrap()).unwrap(),
            serde_json::to_value(doc.legacy()).unwrap(),
        )
    }
}

fn status(f: &Fixture, name: &str, text: &str) -> anyhow::Result<DetectorStatus> {
    Ok(DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
            &f.rules,
        )?
        .status())
}

#[test]
fn all_formats_distinguish_valid_empty_malformed_duplicate_and_future_editions() {
    let f = Fixture::new();
    for (name, empty) in [
        ("composer.json", "{}"),
        ("composer.lock", r#"{"packages":[],"packages-dev":[]}"#),
        ("a.csproj", "<Project/>"),
        ("Directory.Packages.props", "<Project/>"),
        ("Directory.Build.props", "<Project/>"),
        ("packages.config", "<packages/>"),
        ("global.json", "{}"),
        ("packages.lock.json", r#"{"version":2,"dependencies":{}}"#),
        ("vcpkg.json", "{}"),
        ("vcpkg-configuration.json", "{}"),
        ("conanfile.txt", "[requires]\n"),
        ("conan.lock", r#"{"version":"0.5","requires":[]}"#),
    ] {
        assert_eq!(
            status(&f, name, empty).unwrap(),
            DetectorStatus::Complete,
            "{name}"
        );
        assert!(dependencies(&f.native(name, empty).0).is_empty(), "{name}");
    }
    for name in ["CMakeLists.txt", "test.cmake"] {
        assert_eq!(
            status(&f, name, "# empty\n").unwrap(),
            DetectorStatus::Partial
        );
    }
    for name in [
        "composer.json",
        "composer.lock",
        "global.json",
        "packages.lock.json",
        "vcpkg.json",
        "vcpkg-configuration.json",
        "conan.lock",
    ] {
        assert!(status(&f, name, "{").is_err(), "{name}");
        assert!(
            status(&f, name, r#"{"require":{},"require":{}}"#).is_err(),
            "{name}"
        );
        assert!(status(&f, name, "[]").is_err(), "{name}");
    }
    for name in [
        "a.csproj",
        "Directory.Packages.props",
        "Directory.Build.props",
        "packages.config",
    ] {
        for text in [
            "<Project>",
            r#"<!DOCTYPE Project [<!ENTITY x SYSTEM "file:///CANARY_TOKEN">]><Project>&x;</Project>"#,
            "<wrong/>",
        ] {
            assert!(status(&f, name, text).is_err(), "{name}");
        }
    }
    assert!(status(&f, "conanfile.txt", "openssl/3.5").is_err());
    assert!(status(&f, "CMakeLists.txt", "find_package(OpenSSL").is_err());
    for (name, text) in [
        ("packages.lock.json", r#"{"version":3,"dependencies":{}}"#),
        ("conan.lock", r#"{"version":"0.6","requires":[]}"#),
    ] {
        assert_eq!(status(&f, name, text).unwrap(), DetectorStatus::Unsupported);
    }
}

#[test]
fn source_assertions_do_not_merge_case_variants_sections_files_or_framework_buckets() {
    let f = Fixture::new();
    let (doc,_) = f.native_files(&[
        ("a.csproj",r#"<Project><ItemGroup><PackageReference Include="BouncyCastle.Cryptography" Version="2.7.0"/><PackageReference Include="bouncycastle.cryptography" Version="2.7.0"/></ItemGroup></Project>"#),
        ("b.csproj",r#"<Project><ItemGroup><PackageReference Include="BOUNCYCASTLE.CRYPTOGRAPHY" Version="2.7.0"/></ItemGroup></Project>"#),
        ("packages.lock.json",r#"{"version":2,"dependencies":{"net10.0":{"BouncyCastle.Cryptography":{"type":"Direct","requested":"[2.7.0, )","resolved":"2.7.0"}},"net11.0":{"bouncycastle.cryptography":{"type":"Transitive","resolved":"2.7.0"}}}}"#),
    ],&[]);
    assert_eq!(named(&doc, "bouncycastle.cryptography").len(), 5);
    let ids: std::collections::BTreeSet<_> = doc["assets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids.len(), doc["assets"].as_array().unwrap().len());
    assert_eq!(
        named(&doc, "bouncycastle.cryptography")
            .iter()
            .filter(|a| a["resolved_version"] == "2.7.0")
            .count(),
        2
    );
}

#[test]
fn central_context_is_nearest_selected_literal_and_source_local() {
    let f = Fixture::new();
    let project = r#"<Project><ItemGroup><PackageReference Include="BouncyCastle.Cryptography"/></ItemGroup></Project>"#;
    let outer = r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="2.6.0"/></ItemGroup></Project>"#;
    let inner = r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="2.7.0"/></ItemGroup></Project>"#;
    let (doc, _) = f.native_files(
        &[
            ("Directory.Packages.props", outer),
            ("src/Directory.Packages.props", inner),
            ("src/app/a.csproj", project),
        ],
        &[],
    );
    let inherited: Vec<_> = named(&doc, "bouncycastle.cryptography")
        .into_iter()
        .filter(|a| a["version_provenance"] == "central_literal_inherited_not_effective")
        .collect();
    assert_eq!(inherited.len(), 1);
    assert_eq!(inherited[0]["declared_constraint"], "2.7.0");
    assert!(inherited[0]["resolved_version"].is_null());
    for props in [
        r#"<Project><ItemGroup Condition="'$(Flag)'=='true'"><PackageVersion Include="BouncyCastle.Cryptography" Version="2.7.0"/></ItemGroup></Project>"#,
        r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="$(Version)"/></ItemGroup></Project>"#,
        r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography"><Version Condition="'$(Flag)'=='true'">2.7.0</Version></PackageVersion></ItemGroup></Project>"#,
        r#"<Project><ItemGroup><PackageVersion Update="BouncyCastle.Cryptography" Version="2.7.0"/></ItemGroup></Project>"#,
        r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" VersionOverride="2.7.0"/></ItemGroup></Project>"#,
        r#"<Project><Import Project="https://CANARY_TOKEN"/><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="2.7.0"/></ItemGroup></Project>"#,
        r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="2.7.0"/><PackageVersion Include="bouncycastle.cryptography" Version="2.6.0"/></ItemGroup></Project>"#,
    ] {
        let (doc, _) = f.native_files(
            &[
                ("Directory.Packages.props", outer),
                ("src/Directory.Packages.props", props),
                ("src/app/a.csproj", project),
            ],
            &[],
        );
        let inherited = named(&doc, "bouncycastle.cryptography")
            .into_iter()
            .find(|a| a["version_provenance"] == "central_literal_inherited_not_effective")
            .unwrap();
        assert!(inherited["declared_constraint"].is_null(), "{props}");
    }
    let (doc, _) = f.native_files(
        &[
            ("Directory.Packages.props", inner),
            ("src/a.csproj", project),
        ],
        &["**/Directory.Packages.props"],
    );
    assert_eq!(named(&doc, "bouncycastle.cryptography").len(), 1);
    assert!(named(&doc, "bouncycastle.cryptography")[0]["declared_constraint"].is_null());
}

#[test]
fn central_files_outside_the_root_unselected_or_linked_are_never_read() {
    let f = Fixture::new();
    let project = r#"<Project><ItemGroup><PackageReference Include="BouncyCastle.Cryptography"/></ItemGroup></Project>"#;
    let props = r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="9.9.9"/></ItemGroup></Project>"#;
    std::fs::write(f.root.join("Directory.Packages.props"), props).unwrap();
    std::fs::write(f.root.join("input/a.csproj"), project).unwrap();
    let (doc, _) = f.scan_at(f.root.join("input"));
    assert!(named(&doc, "bouncycastle.cryptography")[0]["declared_constraint"].is_null());
    std::fs::write(f.root.join("input/Directory.Packages.props"), props).unwrap();
    let (doc, _) = f.scan_at(f.root.join("input/a.csproj"));
    assert!(named(&doc, "bouncycastle.cryptography")[0]["declared_constraint"].is_null());
    #[cfg(unix)]
    {
        std::fs::remove_file(f.root.join("input/Directory.Packages.props")).unwrap();
        std::os::unix::fs::symlink(
            f.root.join("Directory.Packages.props"),
            f.root.join("input/Directory.Packages.props"),
        )
        .unwrap();
        let (doc, _) = f.scan_at(f.root.join("input"));
        assert!(named(&doc, "bouncycastle.cryptography")[0]["declared_constraint"].is_null());
    }
}

#[test]
fn central_malformed_explicit_versions_never_supply_inheritance() {
    let f = Fixture::new();
    let props = r#"<Project><ItemGroup><PackageVersion Include="BouncyCastle.Cryptography" Version="2.8.0"/></ItemGroup></Project>"#;
    for project in [
        r#"<Project><ItemGroup><PackageReference Include="BouncyCastle.Cryptography" Version="2.6.0" VersionOverride="2.7.0"/></ItemGroup></Project>"#,
        r#"<Project><ItemGroup><PackageReference Include="BouncyCastle.Cryptography" Version="2.6.0"><Version>2.7.0</Version></PackageReference></ItemGroup></Project>"#,
    ] {
        let (doc, _) = f.native_files(
            &[("Directory.Packages.props", props), ("a.csproj", project)],
            &[],
        );
        let facts = named(&doc, "bouncycastle.cryptography");
        assert_eq!(facts.len(), 2);
        assert_eq!(
            facts
                .iter()
                .filter(|d| d["declared_constraint"].is_null())
                .count(),
            1,
            "{project}"
        );
        assert!(!facts
            .iter()
            .any(|d| d["version_provenance"] == "central_literal_inherited_not_effective"));
    }
}

#[test]
fn central_truncated_sources_never_supply_inheritance() {
    let f = Fixture::new();
    let props = format!(
        "<Project><ItemGroup>{}{}</ItemGroup></Project>",
        (0..10000)
            .map(|i| format!("<PackageVersion Include=\"Package{i}\" Version=\"1.0\"/>"))
            .collect::<String>(),
        r#"<PackageVersion Include="BouncyCastle.Cryptography" Version="2.8.0"/>"#
    );
    assert_eq!(
        status(&f, "Directory.Packages.props", &props).unwrap(),
        DetectorStatus::Truncated
    );
    let project = r#"<Project><ItemGroup><PackageReference Include="BouncyCastle.Cryptography"/></ItemGroup></Project>"#;
    // Select the props as context while exporting only the project assertion.
    // Its own detector status is checked above; exporting10k unrelated assets
    // would test legacy inventory assembly rather than this context boundary.
    struct ProjectOnly;
    impl Detector for ProjectOnly {
        fn name(&self) -> &'static str {
            "dependency_detector"
        }
        fn detect(
            &self,
            file: &ScannableFile,
            rules: &RuleSet,
        ) -> anyhow::Result<Vec<pqc_scan_core::Detection>> {
            DependencyDetector.detect(file, rules)
        }
        fn detect_in_repository(
            &self,
            file: &ScannableFile,
            rules: &RuleSet,
            repo: Option<&pqc_scan_core::RepositoryContext>,
            observed: bool,
        ) -> anyhow::Result<pqc_scan_core::DetectorOutcome> {
            if file.file_name() == Some("Directory.Packages.props") {
                Ok(pqc_scan_core::DetectorOutcome::new(
                    vec![],
                    DetectorStatus::NotApplicable,
                ))
            } else {
                DependencyDetector.detect_in_repository(file, rules, repo, observed)
            }
        }
    }
    std::fs::write(f.root.join("input/Directory.Packages.props"), props).unwrap();
    std::fs::write(f.root.join("input/a.csproj"), project).unwrap();
    let (doc, _) = f.scan_with(f.root.join("input"), Arc::new(ProjectOnly));
    let facts = named(&doc, "bouncycastle.cryptography");
    assert_eq!(facts.len(), 1);
    assert!(facts[0]["declared_constraint"].is_null());
}

#[test]
fn development_platform_features_baselines_and_revisions_remain_non_installed_claims() {
    let f = Fixture::new();
    let (doc,legacy)=f.native("composer.json",r#"{"require-dev":{"phpseclib/phpseclib":"^3"},"config":{"platform":{"php":"8.6.0","ext-oqsphp":false}}}"#);
    assert_eq!(
        named(&doc, "ext-oqsphp")[0]["version_provenance"],
        "platform_emulation_disabled"
    );
    assert!(named(&doc, "ext-oqsphp")[0]["reported_version"].is_null());
    assert!(legacy["dependency_sbom"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["purl"] == "pkg:composer/phpseclib/phpseclib"));
    let (doc,_)=f.native("vcpkg.json",r#"{"name":"app","dependencies":[{"name":"liboqs","version>=":"0.16.0#2","features":["openssl"]}],"builtin-baseline":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","overrides":[{"name":"openssl","version":"3.5.0","port-version":2}]}"#);
    assert_eq!(
        named(&doc, "liboqs/openssl")[0]["version_provenance"],
        "feature_declared"
    );
    assert_eq!(
        named(&doc, "liboqs")[0]["declared_constraint"],
        ">=0.16.0#2"
    );
    assert_eq!(
        named(&doc, "builtin")[0]["reported_version"],
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(named(&doc, "openssl")[0]["reported_version"], "3.5.0#2");
    let (doc, _) = f.native(
        "conan.lock",
        r#"{"version":"0.5","requires":["openssl/3.5.0#abcdef%1234567890.1"]}"#,
    );
    assert_eq!(
        named(&doc, "openssl")[0]["reported_version"],
        "3.5.0#abcdef%1234567890.1"
    );
    assert!(named(&doc, "openssl")[0]["resolved_version"].is_null());
    let (doc, _) = f.native(
        "conan.lock",
        r#"{"version":"0.5","requires":["openssl/3.5.0@custom/channel#abcdef%1234567890.1"]}"#,
    );
    assert_eq!(
        named(&doc, "openssl")[0]["reported_version"],
        "3.5.0#abcdef%1234567890.1"
    );
    assert_eq!(
        named(&doc, "openssl")[0]["version_provenance"],
        "source_unresolved"
    );
}

#[test]
fn nuget_resolution_requires_a_concrete_version_in_its_supported_numeric_grammar() {
    let f = Fixture::new();
    for (version, valid) in [
        ("2.7", true),
        ("2.7.0.1", true),
        ("2.7.0-preview.1+build", true),
        ("2.7.0spoof", false),
        ("2..7", false),
        ("v2.7.0", false),
    ] {
        let text=serde_json::json!({"version":2,"dependencies":{"net11.0/win-x64":{"BouncyCastle.Cryptography":{"type":"Direct","requested":"[2.7.0, )","resolved":version}}}}).to_string();
        let (doc, _) = f.native("packages.lock.json", &text);
        assert_eq!(
            named(&doc, "bouncycastle.cryptography")[0]["resolved_version"].is_string(),
            valid,
            "{version}"
        );
        assert_eq!(
            named(&doc, "runtime-identifier")[0]["reported_version"],
            "win-x64"
        );
        assert_eq!(
            named(&doc, "target-framework")[0]["version_provenance"],
            "lock_framework_reported"
        );
    }
}

#[test]
fn cmake_comments_strings_nested_and_dynamic_expressions_cannot_fabricate_packages() {
    let f = Fixture::new();
    let (doc, _) = f.native(
        "CMakeLists.txt",
        r##"
# find_package(OpenSSL 9.9)
#[=[ find_package(liboqs 9.9) ]=]
message("find_package(OpenSSL 9.9)")
set(fake [[find_package(liboqs 9.9)]])
find_package(${LIB} ${VERSION})
target_link_libraries(app PRIVATE $<IF:flag,OpenSSL::Crypto,oqs>)
"##,
    );
    assert!(dependencies(&doc).is_empty());
    assert!(status(&f, "CMakeLists.txt", "[[unterminated").is_err());
}

#[test]
fn malformed_siblings_survive_without_unsafe_identity_or_version_promotion() {
    let f = Fixture::new();
    for (name, text, wanted) in [
        (
            "composer.json",
            r#"{"require":{"paragonie/pqcrypto_compat":"^1","https://CANARY_TOKEN":"v1"}}"#,
            "paragonie/pqcrypto_compat",
        ),
        (
            "vcpkg.json",
            r#"{"dependencies":["openssl",{"name":"liboqs","version>=":null}]}"#,
            "openssl",
        ),
        (
            "packages.lock.json",
            r#"{"version":1,"dependencies":{"net11.0":{"BouncyCastle.Cryptography":{"type":"Direct","resolved":"2.7.0"},"Bad":{"type":"Direct","resolved":"https://CANARY_TOKEN"}}}}"#,
            "bouncycastle.cryptography",
        ),
        (
            "conan.lock",
            r#"{"version":"0.5","requires":["openssl/3.5.0",7]}"#,
            "openssl",
        ),
    ] {
        assert_eq!(
            status(&f, name, text).unwrap(),
            DetectorStatus::Partial,
            "{name}"
        );
        let (doc, legacy) = f.native(name, text);
        assert!(!named(&doc, wanted).is_empty());
        assert!(!doc.to_string().contains("CANARY_TOKEN"));
        assert!(!legacy.to_string().contains("CANARY_TOKEN"));
    }
}

#[test]
fn custom_composer_repository_declarations_never_fabricate_registry_identity() {
    let f = Fixture::new();
    let (doc,legacy)=f.native("composer.json",r#"{"repositories":[{"type":"path","url":"../CANARY_TOKEN"}],"require":{"paragonie/pqcrypto_compat":"^1"},"require-dev":{"phpseclib/phpseclib":"^3"}}"#);
    assert_eq!(
        named(&doc, "paragonie/pqcrypto_compat")[0]["version_provenance"],
        "source_unresolved"
    );
    assert_eq!(
        named(&doc, "phpseclib/phpseclib")[0]["version_provenance"],
        "dev_source_unresolved"
    );
    assert!(legacy["dependency_sbom"]
        .as_array()
        .unwrap()
        .iter()
        .all(|d| d["purl"] == ""));
    assert!(!legacy.to_string().contains("CANARY_TOKEN"));
}

#[test]
fn original_xml_json_and_utf16_source_lines_are_useful_and_duplicates_stay_unknown() {
    let f = Fixture::new();
    let xml="<Project>\n<!-- 日本語💻 -->\n<ItemGroup>\n<PackageReference Include=\"BouncyCastle.Cryptography\" Version=\"2.7.0\"/>\n</ItemGroup>\n</Project>";
    for bytes in [
        xml.as_bytes().to_vec(),
        std::iter::once(0xfeffu16)
            .chain(xml.encode_utf16())
            .flat_map(u16::to_le_bytes)
            .collect(),
    ] {
        let out = DependencyDetector
            .detect_observed(
                &ScannableFile::from_bytes("a.csproj".into(), bytes),
                &f.rules,
            )
            .unwrap();
        assert_eq!(out.status(), DetectorStatus::Complete);
        let hits: Vec<_> = out
            .detections()
            .iter()
            .filter(|d| {
                d.rule_id == "DEP_NUGET_CRYPTO"
                    && d.evidence
                        .metadata
                        .get("dep_name")
                        .is_some_and(|n| n == "bouncycastle.cryptography")
            })
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!((hits[0].location.line, hits[0].location.column), (4, 1));
    }
    let text="{\n\"version\":2,\n\"dependencies\":{\"net11.0\":{\n\"BouncyCastle.Cryptography\":{\"type\":\"Direct\",\"resolved\":\"2.7.0\"}}}}";
    let out = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("packages.lock.json".into(), text.as_bytes().to_vec()),
            &f.rules,
        )
        .unwrap();
    assert_eq!(out.status(), DetectorStatus::Complete);
    let hits: Vec<_> = out
        .detections()
        .iter()
        .filter(|d| {
            d.rule_id == "DEP_NUGET_CRYPTO"
                && d.evidence
                    .metadata
                    .get("dep_name")
                    .is_some_and(|n| n == "bouncycastle.cryptography")
        })
        .collect();
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].location.line, hits[0].location.column), (4, 1));
    let text = r#"{"require":{"paragonie\/pqcrypto_compat":"^1"},"require-dev":{"paragonie\/pqcrypto_compat":"^2"}}"#;
    let out = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("composer.json".into(), text.as_bytes().to_vec()),
            &f.rules,
        )
        .unwrap();
    assert_eq!(out.status(), DetectorStatus::Complete);
    let hits: Vec<_> = out
        .detections()
        .iter()
        .filter(|d| d.rule_id == "DEP_PHP_CRYPTO")
        .collect();
    assert_eq!(hits.len(), 2);
    assert!(hits
        .iter()
        .all(|d| d.location.line == 0 && d.location.column == 0));
}

#[test]
fn byte_depth_value_record_and_xml_node_limits_are_explicit() {
    let f = Fixture::new();
    assert_eq!(
        status(
            &f,
            "composer.json",
            &format!("{{}}{}", " ".repeat(2 * 1024 * 1024 - 2))
        )
        .unwrap(),
        DetectorStatus::Complete
    );
    assert_eq!(
        status(
            &f,
            "composer.json",
            &format!("{{}}{}", " ".repeat(2 * 1024 * 1024 - 1))
        )
        .unwrap(),
        DetectorStatus::Truncated
    );
    let deep = format!("{{\"ignored\":{}0{}}}", "[".repeat(65), "]".repeat(65));
    assert_eq!(
        status(&f, "composer.json", &deep).unwrap(),
        DetectorStatus::Truncated
    );
    let many = format!("{{\"ignored\":[{}]}}", vec!["0"; 50000].join(","));
    assert_eq!(
        status(&f, "composer.json", &many).unwrap(),
        DetectorStatus::Truncated
    );
    for count in [9999, 10000, 10001] {
        let data=serde_json::json!({"require":(0..count).map(|i|(format!("vendor/p{i}"),"^1")).collect::<std::collections::BTreeMap<_,_>>()}).to_string();
        assert_eq!(
            status(&f, "composer.json", &data).unwrap(),
            if count > 10000 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            }
        );
    }
    let xml = format!("<Project>{}</Project>", "<None/>".repeat(50000));
    assert_eq!(
        status(&f, "a.csproj", &xml).unwrap(),
        DetectorStatus::Truncated
    );
    let xml = format!("{}{}", "<Project>".repeat(66), "</Project>".repeat(66));
    assert_eq!(
        status(&f, "a.csproj", &xml).unwrap(),
        DetectorStatus::Truncated
    );
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
fn dependencies(native: &Value) -> Vec<&Value> {
    native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "dependency")
        .map(|a| &a["attributes"])
        .collect()
}
fn named<'a>(native: &'a Value, name: &str) -> Vec<&'a Value> {
    dependencies(native)
        .into_iter()
        .filter(|a| a["name"] == name)
        .collect()
}
#[test]
fn all_frozen_dependency_inputs_keep_their_original_status_and_scoped_facts() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let matrix: Value = serde_json::from_slice(
        &std::fs::read(root.join("tasks/076/evidence/DEPENDENCY-MATRIX.json")).unwrap(),
    )
    .unwrap();
    let f = Fixture::new();
    let mut count = 0;
    for case in matrix["cases"].as_array().unwrap() {
        count += 1;
        let id = case["id"].as_str().unwrap();
        let path = root.join(case["path"].as_str().unwrap());
        let name = path.file_name().unwrap().to_str().unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        let outcome = DependencyDetector
            .detect_observed(
                &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        let expected = match case["expected_status"].as_str().unwrap() {
            "complete" => DetectorStatus::Complete,
            "partial" => DetectorStatus::Partial,
            "unsupported" => DetectorStatus::Unsupported,
            s => panic!("unknown frozen status {s}"),
        };
        assert_eq!(outcome.status(), expected, "{id}: {outcome:?}");
        let (native, legacy) = f.native(name, &text);
        let records = dependencies(&native);
        assert!(
            !native.to_string().contains("CANARY_TOKEN"),
            "{id}: {native}"
        );
        assert!(
            !legacy.to_string().contains("CANARY_TOKEN"),
            "{id}: {legacy}"
        );
        assert!(!native.to_string().contains("CANARY_NOT_A_KEY"));
        for record in &records {
            assert_eq!(record.as_object().unwrap().len(), 6, "{id}");
        }
        match id {
            "composer-manifest" => {
                assert_eq!(
                    named(&native, "paragonie/pqcrypto_compat")[0]["declared_constraint"],
                    "^1.0"
                );
                assert_eq!(
                    named(&native, "phpseclib/phpseclib")[0]["version_provenance"],
                    "manifest_dev_declared"
                );
                let php = named(&native, "php");
                assert_eq!(php.len(), 2);
                assert!(php
                    .iter()
                    .any(|a| a["version_provenance"] == "platform_emulated"));
                assert!(php.iter().all(|a| a["resolved_version"].is_null()));
            }
            "composer-lock-zip" | "composer-lock-path" | "composer-lock-git" => {
                let package = named(&native, "paragonie/pqcrypto_compat")[0];
                assert_eq!(package["reported_version"], "v1.0.0");
                assert!(package["resolved_version"].is_null());
                assert!(legacy["dependency_sbom"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|d| d["name"] == "paragonie/pqcrypto_compat")
                    .all(|d| d["purl"] == ""));
            }
            "composer-wrappers" => {
                assert_eq!(named(&native, "mir-evgenii/liboqs-php").len(), 1);
                assert_eq!(named(&native, "ext-oqsphp").len(), 1);
                assert_eq!(named(&native, "ext-pqcrypto").len(), 1);
                assert!(named(&native, "Muzosh/OQS-PHP").is_empty());
            }
            "nuget-project" => {
                let bc = named(&native, "bouncycastle.cryptography")[0];
                assert_eq!(bc["declared_constraint"], "2.7.0");
                assert!(bc["resolved_version"].is_null());
                assert_eq!(named(&native, "target-framework").len(), 2);
            }
            "nuget-lock-1" | "nuget-lock-2" => {
                let bc = named(&native, "bouncycastle.cryptography")[0];
                assert_eq!(bc["declared_constraint"], "[2.7.0, )");
                assert_eq!(bc["resolved_version"], "2.7.0");
            }
            "nuget-conditional" => {
                let bc = named(&native, "bouncycastle.cryptography")[0];
                assert!(bc["resolved_version"].is_null());
                assert!(bc["declared_constraint"].is_null());
            }
            "nuget-central" => {
                assert_eq!(
                    named(&native, "bouncycastle.cryptography")[0]["declared_constraint"],
                    "2.7.0"
                );
            }
            "nuget-build" => {
                assert_eq!(named(&native, "openforge.cryptography.liboqs").len(), 1);
                assert_eq!(named(&native, "target-framework").len(), 1);
            }
            "nuget-legacy" => {
                assert_eq!(
                    named(&native, "bouncycastle.cryptography")[0]["reported_version"],
                    "2.7.0"
                );
                assert!(
                    named(&native, "bouncycastle.cryptography")[0]["resolved_version"].is_null()
                );
            }
            "nuget-sdk" => {
                assert!(records
                    .iter()
                    .any(|r| r["version_provenance"] == "sdk_declared"
                        && r["reported_version"] == "11.0.100"
                        && r["resolved_version"].is_null()));
            }
            "vcpkg-manifest" => {
                assert!(!named(&native, "liboqs").is_empty());
                assert!(!named(&native, "openssl").is_empty());
                assert!(records.iter().all(|a| a["resolved_version"].is_null()));
            }
            "vcpkg-config" => {
                assert!(!named(&native, "openssl").is_empty());
                assert!(records.iter().all(|a| a["resolved_version"].is_null()));
            }
            "conan-text" | "conan-lock" => {
                assert_eq!(named(&native, "openssl").len(), 1);
                assert!(records.iter().all(|a| a["resolved_version"].is_null()));
            }
            "cmake-literals" => {
                assert!(!named(&native, "openssl").is_empty());
                assert!(!named(&native, "liboqs").is_empty());
            }
            "composer-negative" | "nuget-negative" | "vcpkg-negative" | "conan-negative"
            | "cmake-dynamic" | "nuget-lock-future" | "conan-lock-future" => {
                assert!(records.is_empty(), "{id}: {records:?}")
            }
            _ => panic!("unmapped frozen case {id}"),
        }
        if matches!(
            id,
            "vcpkg-manifest" | "vcpkg-config" | "conan-text" | "conan-lock" | "cmake-literals"
        ) {
            assert!(
                legacy["dependency_sbom"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|d| d["purl"] == ""),
                "{id}"
            );
        }
    }
    assert_eq!(count, 25);
}

#[test]
fn final_dependency_fixed_budgets_have_empty_below_at_and_above_controls() {
    let f = Fixture::new();
    // Actual format boundaries, including construction before allocation.
    for bytes in [2 * 1024 * 1024 - 1, 2 * 1024 * 1024, 2 * 1024 * 1024 + 1] {
        let data = format!("{{}}{}", " ".repeat(bytes - 2));
        assert_eq!(
            status(&f, "composer.json", &data).unwrap(),
            if bytes > 2 * 1024 * 1024 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            }
        );
    }
    for count in [0, 49_997, 49_998, 49_999] {
        // Object + array + count scalars: 50,000 total values at count=49,998.
        let data = format!("{{\"ignored\":[{}]}}", vec!["0"; count].join(","));
        assert_eq!(
            status(&f, "composer.json", &data).unwrap(),
            if count > 49_998 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            },
            "JSON values {count}"
        );
    }
    for levels in [0, 62, 63, 64] {
        // Root object adds one bracket level; the scalar also consumes depth.
        let data = format!(
            "{{\"ignored\":{}0{}}}",
            "[".repeat(levels),
            "]".repeat(levels)
        );
        assert_eq!(
            status(&f, "composer.json", &data).unwrap(),
            if levels > 63 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            },
            "JSON depth {levels}"
        );
    }
    for children in [0, 49_997, 49_998, 49_999] {
        // roxmltree includes the document and Project nodes in its 50k budget.
        let data = format!("<Project>{}</Project>", "<None/>".repeat(children));
        assert_eq!(
            status(&f, "a.csproj", &data).unwrap(),
            if children > 49_998 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            },
            "XML nodes {children}"
        );
    }
    for levels in [1, 63, 64, 65] {
        let data = format!(
            "{}{}",
            "<Project>".repeat(levels),
            "</Project>".repeat(levels)
        );
        assert_eq!(
            status(&f, "a.csproj", &data).unwrap(),
            if levels > 64 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            },
            "XML depth {levels}"
        );
    }
    for lines in [0, 49_999, 50_000, 50_001] {
        assert_eq!(
            status(&f, "conanfile.txt", &"#\n".repeat(lines)).unwrap(),
            if lines > 50_000 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            },
            "Conan lines {lines}"
        );
    }
    for tokens in [0, 49_999, 50_000, 50_001] {
        let data = vec!["noop"; tokens].join(" ");
        assert_eq!(
            status(&f, "CMakeLists.txt", &data).unwrap(),
            if tokens > 50_000 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Partial
            },
            "CMake tokens {tokens}"
        );
    }
    for levels in [1, 63, 64, 65] {
        let data = format!("noop{}{}", "(".repeat(levels), ")".repeat(levels));
        assert_eq!(
            status(&f, "CMakeLists.txt", &data).unwrap(),
            if levels > 64 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Partial
            },
            "CMake depth {levels}"
        );
    }
}
