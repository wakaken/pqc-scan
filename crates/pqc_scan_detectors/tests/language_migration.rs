//! Frozen source conformance through the adopted v0.4 reader and evaluator.
use pqc_scan_core::{
    migration::{self, InputFormat, SavedEvidence},
    scan_repository_native, Detector, NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

struct Fixture {
    root: PathBuf,
    rules: RuleSet,
}
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-language-migration082-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(root.join("input")).unwrap();
        Self {
            root,
            rules: RuleSet::load_from_dir(&repo().join("rules")).unwrap(),
        }
    }
    fn replace(&self, name: &str, bytes: &[u8]) {
        for entry in std::fs::read_dir(self.root.join("input")).unwrap() {
            std::fs::remove_file(entry.unwrap().path()).unwrap();
        }
        std::fs::write(self.root.join("input").join(name), bytes).unwrap();
    }
    fn put(&self, name: &str, bytes: &[u8]) {
        std::fs::write(self.root.join("input").join(name), bytes).unwrap();
    }
    fn scan(&self) -> (Value, Vec<u8>, SavedEvidence, String) {
        let ds: Vec<Arc<dyn Detector>> =
            vec![Arc::new(TreeSitterDetector), Arc::new(DependencyDetector)];
        let doc = scan_repository_native(
            &ScanConfig {
                root: self.root.join("input"),
                threads: 1,
                ..Default::default()
            },
            &self.rules,
            &ds,
            &NativeOptions::new(Some("v05/integration")).unwrap(),
        )
        .unwrap();
        let native = serde_json::to_value(doc.native().unwrap()).unwrap();
        let bytes = serde_json::to_vec(&native).unwrap();
        let saved = SavedEvidence::from_bytes(&bytes, InputFormat::InventoryV1).unwrap();
        let as_of = (doc.legacy().generated_at + Duration::from_secs(2))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        (native, bytes, saved, as_of)
    }
}

#[test]
fn new_dependency_provenance_survives_saved_projection_without_becoming_runtime_support() {
    let f = Fixture::new();
    let matrix: Value = serde_json::from_slice(
        &std::fs::read(repo().join("tasks/076/evidence/DEPENDENCY-MATRIX.json")).unwrap(),
    )
    .unwrap();
    let mut count = 0;
    let mut nonempty = 0;
    for case in matrix["cases"].as_array().unwrap() {
        count += 1;
        let path = repo().join(case["path"].as_str().unwrap());
        let raw = std::fs::read(&path).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&raw)),
            case["sha256"].as_str().unwrap()
        );
        f.replace(path.file_name().unwrap().to_str().unwrap(), &raw);
        let (native, _, saved, as_of) = f.scan();
        let dependencies: Vec<_> = native["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["kind"] == "dependency")
            .collect();
        nonempty += usize::from(!dependencies.is_empty());
        for asset in dependencies {
            let targets: Vec<_> = saved
                .targets()
                .iter()
                .filter(|t| t.source_ref.root_asset_id.as_deref() == asset["id"].as_str())
                .collect();
            assert!(!targets.is_empty(), "{}", case["id"]);
            for t in targets {
                assert_eq!(
                    t.source_ref.dependency_provenance.as_deref(),
                    asset["attributes"]["version_provenance"].as_str(),
                    "{}",
                    case["id"]
                );
                assert_eq!(t.source_ref.role, "reference-only");
                assert_eq!(t.technical.algorithm.identifier, "unknown");
                assert_eq!(t.observation.usage, "referenced");
            }
        }
        let report =
            serde_json::to_value(migration::evaluate(&saved, None, &as_of).unwrap()).unwrap();
        assert_eq!(
            report["decisions"].as_array().unwrap().len(),
            saved.targets().len()
        );
        assert!(report["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|d| d["readiness"] != "ready"));
        assert!(!report.to_string().contains("CANARY_TOKEN"));
        assert!(!report.to_string().contains("CANARY_NOT_A_KEY"));
    }
    assert_eq!(count, 25);
    assert_eq!(nonempty, 18);
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn cases() -> Vec<Value> {
    serde_json::from_slice::<Value>(
        &std::fs::read(repo().join("tasks/076/evidence/CORPUS-CASES.json")).unwrap(),
    )
    .unwrap()["cases"]
        .as_array()
        .unwrap()
        .clone()
}
fn new_rule(id: &str) -> bool {
    id.starts_with("PHP_") || id.starts_with("CSHARP_") || id.starts_with("C_FAMILY_")
}

fn claims(as_of: &str, answers: Vec<(&str, Option<Value>)>) -> migration::ContextClaims {
    let year = as_of[..4].parse::<u32>().unwrap();
    let expiry = format!("{}-01-01T00:00:00Z", year + 1);
    migration::ContextClaims::from_answers(
        &answers
            .into_iter()
            .map(|(field, value)| migration::ClaimAnswer {
                field: field.into(),
                value,
            })
            .collect::<Vec<_>>(),
        migration::Provenance::operator("owner082", as_of, Some(&expiry)).unwrap(),
    )
    .unwrap()
}
fn active_signature() -> Vec<(&'static str, Option<Value>)> {
    vec![
        ("applicability", Some(json!("applicable"))),
        ("use_case", Some(json!("signature"))),
        ("usage_state", Some(json!("active"))),
        ("business_critical", Some(json!(true))),
    ]
}
fn all_support() -> Vec<(&'static str, Option<Value>)> {
    [
        "runtime",
        "provider",
        "parameters",
        "peer",
        "protocol",
        "jose",
        "verifiers",
        "pki",
    ]
    .into_iter()
    .map(|f| {
        (
            match f {
                "runtime" => "support.runtime",
                "provider" => "support.provider",
                "parameters" => "support.parameters",
                "peer" => "support.peer",
                "protocol" => "support.protocol",
                "jose" => "support.jose",
                "verifiers" => "support.verifiers",
                _ => "support.pki",
            },
            Some(json!(true)),
        )
    })
    .collect()
}

#[test]
fn four_language_targets_keep_scoped_false_unknown_ready_and_exclusion_claims_separate() {
    let f = Fixture::new();
    let source = cases();
    for (lang, name) in [
        ("php", "php.php"),
        ("csharp", "csharp.cs"),
        ("c", "c.c"),
        ("cpp", "cpp.cpp"),
    ] {
        let case = source
            .iter()
            .find(|c| {
                c["language"] == lang
                    && c["expectation"]["canonical_algorithm"] == "RSA"
                    && c["expectation"]["span"].is_array()
            })
            .unwrap();
        f.put(
            name,
            &std::fs::read(repo().join(case["path"].as_str().unwrap())).unwrap(),
        );
    }
    let (_native, bytes, saved, as_of) = f.scan();
    let mut selections = vec![];
    let mut chosen = vec![];
    for (i, (name, system)) in [
        ("php.php", "php-system"),
        ("csharp.cs", "dotnet-system"),
        ("c.c", "c-system"),
        ("cpp.cpp", "cpp-system"),
    ]
    .into_iter()
    .enumerate()
    {
        let index = saved
            .targets()
            .iter()
            .position(|t| {
                t.source_ref.path.as_deref() == Some(name)
                    && t.technical.algorithm.identifier == "RSA"
            })
            .unwrap();
        let t = &saved.targets()[index];
        assert!(t.is_bindable(), "{name}");
        chosen.push(index);
        let mut answers = active_signature();
        match i {
            0 => {
                answers.push(("support.runtime", Some(json!(false))));
            }
            1 => {
                answers.extend(
                    all_support()
                        .into_iter()
                        .filter(|(f, _)| *f != "support.runtime"),
                );
                answers.push(("support.runtime", None));
            }
            2 => answers.extend(all_support()),
            _ => {
                answers = vec![("applicability", Some(json!("not_applicable")))];
            }
        }
        selections.push(migration::ContextSelection {
            target_index: index,
            system_id: system.into(),
            claims: claims(&as_of, answers),
        });
    }
    assert_eq!(chosen.len(), 4);
    let root = saved.targets()[chosen[0]]
        .source_ref
        .root_asset_id
        .as_ref()
        .unwrap();
    assert!(
        chosen
            .iter()
            .all(|i| saved.targets()[*i].source_ref.root_asset_id.as_ref() == Some(root)),
        "same RSA asset must not transfer claims"
    );
    let generated = migration::generate_context(&saved, "org082", &selections, &as_of).unwrap();
    let mut context: Value = serde_json::from_slice(&generated).unwrap();
    let false_runtime = context["bindings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["system_id"] == "php-system")
        .unwrap()["claims"]["support"]["runtime"]
        .clone();
    let true_runtime = context["bindings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["system_id"] == "c-system")
        .unwrap()["claims"]["support"]["runtime"]
        .clone();
    context["organization_defaults"] = json!({"support":{"runtime":true_runtime}});
    for system in context["systems"].as_array_mut().unwrap() {
        if system["id"] == "php-system" {
            system["defaults"] = json!({"support":{"runtime":false_runtime}});
        }
    }
    for binding in context["bindings"].as_array_mut().unwrap() {
        if binding["system_id"] == "php-system" || binding["system_id"] == "c-system" {
            binding["claims"]["support"]
                .as_object_mut()
                .unwrap()
                .remove("runtime");
        }
    }
    let context =
        migration::MigrationContext::from_bytes(&serde_json::to_vec(&context).unwrap()).unwrap();
    // Saved reassessment needs no source files and cannot rescan this removed root.
    std::fs::remove_dir_all(f.root.join("input")).unwrap();
    let report =
        serde_json::to_value(migration::evaluate(&saved, Some(&context), &as_of).unwrap()).unwrap();
    for (index, system, readiness, action) in [
        (chosen[0], "php-system", "blocked", "unblock"),
        (chosen[1], "dotnet-system", "unknown", "investigate"),
        (chosen[2], "c-system", "ready", "migrate"),
        (chosen[3], "cpp-system", "not_applicable", "none"),
    ] {
        let ds: Vec<_> = report["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|d| d["target_id"] == saved.targets()[index].id)
            .collect();
        assert_eq!(ds.len(), 1);
        let d = ds[0];
        assert_eq!(d["system_id"], system);
        assert_eq!(d["readiness"], readiness, "{system}: {d}");
        assert_eq!(d["action"]["kind"], action, "{system}: {d}");
        assert_eq!(saved.targets()[index].observation.usage, "referenced");
        assert_eq!(saved.targets()[index].observation.assurance, "syntactic");
    }
    for (usage, extra) in [
        ("signature", vec!["peer", "protocol"]),
        ("jwt", vec!["peer", "protocol", "jose", "verifiers"]),
        ("pki", vec!["peer", "protocol", "pki"]),
    ] {
        let fields =
            migration::input_support_fields(&saved.targets()[chosen[0]], Some(usage)).unwrap();
        for f in ["runtime", "provider", "parameters"]
            .into_iter()
            .chain(extra)
        {
            assert!(fields.contains(&f));
        }
    }
    let mut replaced = bytes;
    replaced.push(b'\n');
    let changed = SavedEvidence::from_bytes(&replaced, InputFormat::InventoryV1).unwrap();
    assert!(matches!(
        context.bind(&changed, &as_of),
        Err(migration::Error::InputBindingMismatch)
    ));
    assert_ne!(saved.input().sha256, changed.input().sha256);
}

#[test]
fn stale_future_expiry_unknown_and_conflicting_purpose_do_not_make_new_targets_ready() {
    for language in ["php", "csharp", "c", "cpp"] {
        let f = Fixture::new();
        let all = cases();
        let case = all
            .iter()
            .find(|c| {
                c["language"] == language
                    && c["expectation"]["canonical_algorithm"] == "ML-KEM-768"
                    && c["expectation"]["span"].is_array()
            })
            .unwrap();
        let path = repo().join(case["path"].as_str().unwrap());
        f.replace(
            path.file_name().unwrap().to_str().unwrap(),
            &std::fs::read(&path).unwrap(),
        );
        let (_, _, saved, as_of) = f.scan();
        let index = saved
            .targets()
            .iter()
            .position(|t| t.technical.algorithm.identifier == "ML-KEM-768")
            .unwrap();
        assert!(saved.targets()[index].is_bindable());
        let year = as_of[..4].parse::<u32>().unwrap();
        let prior = format!("{}-01-01T00:00:00Z", year - 1);
        let future = format!("{}-01-01T00:00:00Z", year + 1);
        let expiry = format!("{}-01-01T00:00:00Z", year + 2);
        for variant in ["stale", "future", "expiry-unknown", "purpose-conflict"] {
            let mut answers = vec![
                ("applicability", Some(json!("applicable"))),
                ("usage_state", Some(json!("active"))),
                (
                    "use_case",
                    Some(json!(if variant == "purpose-conflict" {
                        "signature"
                    } else {
                        "key_establishment"
                    })),
                ),
            ];
            answers.extend(all_support());
            let provenance = match variant {
                "stale" => migration::Provenance::operator("owner082", &prior, Some(&as_of)),
                "future" => migration::Provenance::operator("owner082", &future, Some(&expiry)),
                "expiry-unknown" => migration::Provenance::operator("owner082", &as_of, None),
                _ => migration::Provenance::operator("owner082", &as_of, Some(&expiry)),
            }
            .unwrap();
            let claims = migration::ContextClaims::from_answers(
                &answers
                    .into_iter()
                    .map(|(field, value)| migration::ClaimAnswer {
                        field: field.into(),
                        value,
                    })
                    .collect::<Vec<_>>(),
                provenance,
            )
            .unwrap();
            let context = migration::generate_context(
                &saved,
                "org082",
                &[migration::ContextSelection {
                    target_index: index,
                    system_id: format!("{language}-system"),
                    claims,
                }],
                &as_of,
            )
            .unwrap();
            let context = migration::MigrationContext::from_bytes(&context).unwrap();
            let report =
                serde_json::to_value(migration::evaluate(&saved, Some(&context), &as_of).unwrap())
                    .unwrap();
            let d: Vec<_> = report["decisions"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|d| d["target_id"] == saved.targets()[index].id)
                .collect();
            assert_eq!(d.len(), 1);
            assert_ne!(d[0]["readiness"], "ready", "{language}/{variant}");
            if variant == "purpose-conflict" {
                assert_eq!(d[0]["information"], "conflicting");
            }
            assert_eq!(
                saved.targets()[index].technical.algorithm.identifier,
                "ML-KEM-768"
            );
            assert_eq!(saved.targets()[index].observation.usage, "referenced");
        }
    }
}

#[test]
fn imports_includes_provider_and_pure_or_hybrid_group_requests_stay_reference_only() {
    let f = Fixture::new();
    for (name,text) in [
        ("refs.php",r#"<?php new \OQS_KEYENCAPSULATION("ML-KEM-768"); new \OQS_SIGNATURE("ML-DSA-65");"#),
        ("refs.cs","using dotnetOQS; using OpenForge.Cryptography.LibOqs; using Org.BouncyCastle.Tls; class C{}"),
        ("refs.c","#include <openssl/evp.h>\n#include <oqs/oqs.h>\nvoid f(){OSSL_PROVIDER_load(NULL,\"oqsprovider\"); EVP_set_default_properties(NULL,\"provider=oqsprovider\"); SSL_CTX_set1_groups_list(ctx,\"MLKEM768\"); SSL_set1_groups_list(ssl,\"X25519MLKEM768:P-256\");}"),
        ("refs.cpp","#include \"oqs_cpp.hpp\"\n"),
    ] {f.put(name,text.as_bytes());}
    let (native, _, saved, as_of) = f.scan();
    assert_eq!(native["observations"].as_array().unwrap().len(), 12);
    assert_eq!(saved.targets().len(), 12);
    assert!(!native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["kind"] == "algorithm"));
    let markers: std::collections::BTreeSet<_> = native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|o| o["evidence"]["marker_kind"].as_str())
        .collect();
    assert!(markers.contains("source_tls_group_ref:MLKEM768"));
    assert!(markers.contains("source_tls_group_ref:X25519MLKEM768:P-256"));
    for target in saved.targets() {
        assert_eq!(target.source_ref.role, "reference-only");
        assert_eq!(target.technical.algorithm.identifier, "unknown");
        assert_eq!(target.observation.method, "ast_pattern");
        assert_eq!(target.observation.assurance, "syntactic");
        assert_eq!(target.observation.usage, "referenced");
    }
    let index = saved
        .targets()
        .iter()
        .position(|t| t.is_bindable())
        .unwrap();
    let mut answers = active_signature();
    answers.extend(all_support());
    let context = migration::generate_context(
        &saved,
        "org082",
        &[migration::ContextSelection {
            target_index: index,
            system_id: "refs-system".into(),
            claims: claims(&as_of, answers),
        }],
        &as_of,
    )
    .unwrap();
    let context = migration::MigrationContext::from_bytes(&context).unwrap();
    let report =
        serde_json::to_value(migration::evaluate(&saved, Some(&context), &as_of).unwrap()).unwrap();
    assert_eq!(report["decisions"].as_array().unwrap().len(), 12);
    assert!(report["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .all(|d| d["readiness"] != "ready"));
}

#[test]
fn unknown_dependency_provenance_and_rejected_metadata_are_not_copied_to_saved_reports() {
    let f = Fixture::new();
    f.replace(
        "composer.json",
        br#"{"config":{"platform":{"ext-oqsphp":false}}}"#,
    );
    let (mut native, _, saved, as_of) = f.scan();
    assert_eq!(saved.targets().len(), 2);
    assert!(saved
        .targets()
        .iter()
        .all(|t| t.source_ref.dependency_provenance.as_deref()
            == Some("platform_emulation_disabled")));
    let private = "PRIVATE082_UNTRUSTED_PROVENANCE_CANARY";
    for asset in native["assets"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .filter(|a| a["kind"] == "dependency")
    {
        asset["attributes"]["version_provenance"] = private.into();
        asset["attributes"]["future_private_metadata"] = private.into();
    }
    let poisoned = SavedEvidence::from_bytes(
        &serde_json::to_vec(&native).unwrap(),
        InputFormat::InventoryV1,
    )
    .unwrap();
    assert_eq!(poisoned.targets().len(), 2);
    assert!(poisoned
        .targets()
        .iter()
        .all(|t| t.source_ref.dependency_provenance.as_deref() == Some("unknown")));
    let report =
        serde_json::to_value(migration::evaluate(&poisoned, None, &as_of).unwrap()).unwrap();
    assert!(!report.to_string().contains(private));
}

#[test]
fn every_frozen_source_case_preserves_focal_identity_assurance_and_existing_technical_meaning() {
    let f = Fixture::new();
    let expectations: Value = serde_json::from_slice(
        &std::fs::read(repo().join("tasks/082/evidence/INTEGRATION-EXPECTATIONS.json")).unwrap(),
    )
    .unwrap();
    let standard: Vec<_> = expectations["standard_identities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let unknown: Vec<_> = expectations["unsupported_constructions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let mut total = 0;
    let mut standard_seen = std::collections::BTreeSet::new();
    let mut unknown_seen = std::collections::BTreeSet::new();
    for case in cases() {
        total += 1;
        let id = case["id"].as_str().unwrap();
        let path = repo().join(case["path"].as_str().unwrap());
        let raw = std::fs::read(&path).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&raw)),
            case["sha256"].as_str().unwrap(),
            "{id}"
        );
        f.replace(path.file_name().unwrap().to_str().unwrap(), &raw);
        let (native, _bytes, saved, as_of) = f.scan();
        let obs = native["observations"].as_array().unwrap();
        let assessment =
            serde_json::to_value(migration::evaluate(&saved, None, &as_of).unwrap()).unwrap();
        assert_eq!(
            assessment["decisions"].as_array().unwrap().len(),
            saved.targets().len(),
            "{id}"
        );
        assert!(
            assessment["decisions"]
                .as_array()
                .unwrap()
                .iter()
                .all(|d| d["readiness"] != "ready"),
            "{id}"
        );
        if case["expectation"]["rule"].is_null() {
            if let Some(span) = case["expectation"]["forbidden_span"].as_array() {
                let focal: Vec<_> = obs
                    .iter()
                    .filter(|o| {
                        o["target"]["byte_start"] == span[0] && o["target"]["byte_end"] == span[1]
                    })
                    .collect();
                assert!(focal.len() <= 1, "{id}");
                for observation in focal {
                    for a in native["assets"].as_array().unwrap().iter().filter(|a| {
                        a["kind"] == "algorithm"
                            && observation["asset_ids"]
                                .as_array()
                                .unwrap()
                                .contains(&a["id"])
                    }) {
                        assert!(
                            !case["expectation"]["forbidden_algorithms"]
                                .as_array()
                                .unwrap()
                                .contains(&a["attributes"]["identifier"]),
                            "{id}"
                        );
                    }
                }
            } else {
                assert!(obs.is_empty(), "{id}");
            }
            continue;
        }
        let expected = &case["expectation"];
        let focal: Vec<_> = obs
            .iter()
            .filter(|o| {
                o["rule_id"] == expected["rule"]
                    && o["target"]["byte_start"] == expected["span"][0]
                    && o["target"]["byte_end"] == expected["span"][1]
            })
            .collect();
        assert_eq!(focal.len(), 1, "{id}");
        let o = focal[0];
        assert_eq!(o["method"], "ast_pattern", "{id}");
        assert_eq!(o["assurance"], "syntactic", "{id}");
        assert_eq!(o["usage"], "referenced", "{id}");
        let targets: Vec<_> = saved
            .targets()
            .iter()
            .filter(|t| t.source_ref.observation_id.as_deref() == o["id"].as_str())
            .collect();
        assert_eq!(targets.len(), 1, "{id}");
        let t = targets[0];
        assert_eq!(t.observation.method, "ast_pattern");
        assert_eq!(t.observation.assurance, "syntactic");
        assert_eq!(t.observation.usage, "referenced");
        assert_eq!(
            t.source_ref.line.map(|n| n as u64),
            o["target"]["line"].as_u64(),
            "{id}"
        );
        let algorithms: Vec<_> = native["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| {
                a["kind"] == "algorithm" && o["asset_ids"].as_array().unwrap().contains(&a["id"])
            })
            .collect();
        if let Some(name) = expected["canonical_algorithm"].as_str() {
            assert_eq!(algorithms.len(), 1, "{id}");
            assert_eq!(algorithms[0]["attributes"]["identifier"], name, "{id}");
            if standard.contains(&name) {
                standard_seen.insert(name.to_string());
                assert_eq!(t.technical.algorithm.identifier, name, "{id}");
                assert_eq!(t.technical.algorithm.standardization, "nist_final", "{id}");
                assert!(t.technical.algorithm.parameters_known, "{id}");
                assert_eq!(
                    serde_json::to_value(t.technical.algorithm.quantum).unwrap(),
                    "designed_resistant",
                    "{id}"
                );
            }
            if unknown.contains(&name) {
                unknown_seen.insert(name.to_string());
                assert_eq!(t.technical.algorithm.identifier, "unknown", "{id}");
                assert_eq!(
                    serde_json::to_value(t.technical.algorithm.quantum).unwrap(),
                    "unknown",
                    "{id}"
                );
            }
            if matches!(name, "Kyber768" | "Dilithium3") {
                assert_ne!(t.technical.algorithm.standardization, "nist_final", "{id}");
            }
        } else {
            assert!(algorithms.is_empty(), "{id}");
            assert_eq!(t.technical.algorithm.identifier, "unknown", "{id}");
        }
        let assessment =
            serde_json::to_value(migration::evaluate(&saved, None, &as_of).unwrap()).unwrap();
        assert_eq!(assessment["input"]["sha256"], saved.input().sha256);
        assert_eq!(
            assessment["decisions"].as_array().unwrap().len(),
            saved.targets().len(),
            "{id}"
        );
        assert!(
            assessment["decisions"]
                .as_array()
                .unwrap()
                .iter()
                .all(|d| d["readiness"] != "ready"),
            "{id}"
        );
    }
    assert_eq!(total, 882);
    assert_eq!(standard_seen.len(), 18);
    assert_eq!(unknown_seen.len(), 4);
}

#[test]
fn derived_quoted_help_inputs_do_not_become_api_or_primitive_facts() {
    let f = Fixture::new();
    let mut count = 0;
    for case in cases()
        .into_iter()
        .filter(|c| c["id"].as_str().unwrap().ends_with("positive"))
    {
        let id = case["id"].as_str().unwrap();
        let expected = &case["expectation"];
        let source = std::fs::read_to_string(repo().join(case["path"].as_str().unwrap())).unwrap();
        // Nine labelled-positive cases intentionally have no positive focal span;
        // quote their entire input without changing the original negative scope.
        let focal = if let Some(span) = expected["span"].as_array() {
            &source[span[0].as_u64().unwrap() as usize..span[1].as_u64().unwrap() as usize]
        } else {
            source.as_str()
        };
        let encoded = serde_json::to_string(focal).unwrap();
        let (name, text) = match case["language"].as_str().unwrap() {
            "php" => (
                "help.php",
                format!(
                    "<?php $help='{}';",
                    focal.replace('\\', "\\\\").replace('\'', "\\'")
                ),
            ),
            "csharp" => ("help.cs", format!("class Help {{string value={encoded};}}")),
            "c" => ("help.c", format!("const char *help={encoded};")),
            "cpp" => ("help.cpp", format!("const char *help={encoded};")),
            _ => unreachable!(),
        };
        let out = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes(name.into(), text.into_bytes()),
                &f.rules,
            )
            .unwrap();
        assert!(
            !out.detections().iter().any(|d| new_rule(&d.rule_id)),
            "{id}: {out:?}"
        );
        count += 1;
    }
    assert_eq!(count, 530);
}
