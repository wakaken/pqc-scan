use pqc_scan_core::{
    scan_repository_native, Detector, DetectorStatus, NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector;
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
            "pqc-v05-c-family-{}-{}",
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
    fn native(&self, source: &str, extension: &str) -> Value {
        self.document(source, extension).0
    }
    fn document(&self, source: &str, extension: &str) -> (Value, Value) {
        for entry in std::fs::read_dir(self.root.join("input")).unwrap() {
            std::fs::remove_file(entry.unwrap().path()).unwrap();
        }
        std::fs::write(self.root.join(format!("input/case.{extension}")), source).unwrap();
        let ds: Vec<Arc<dyn Detector>> = vec![Arc::new(TreeSitterDetector)];
        let doc = scan_repository_native(
            &ScanConfig {
                root: self.root.join("input"),
                threads: 1,
                ..Default::default()
            },
            &self.rules,
            &ds,
            &NativeOptions::new(Some("v05/c-family")).unwrap(),
        )
        .unwrap();
        (
            serde_json::to_value(doc.native().unwrap()).unwrap(),
            serde_json::to_value(doc.legacy()).unwrap(),
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
fn algorithm_names(native: &Value, observation: &Value) -> Vec<String> {
    observation["asset_ids"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|id| {
            native["assets"]
                .as_array()
                .unwrap()
                .iter()
                .find(|a| a["id"] == *id && a["kind"] == "algorithm")
        })
        .map(|a| a["attributes"]["identifier"].as_str().unwrap().into())
        .collect()
}

#[test]
fn cpp_qualified_implementation_definitions_keep_the_original_name_position() {
    let f = Fixture::new();
    let source="bcm_status bssl::BCM_mldsa65_generate_key(unsigned char out[100], Key *key){return status;}";
    let native = f.native(source, "cc.inc");
    let obs: Vec<_> = native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["rule_id"] == "SOURCE_IMPLEMENTATION_REFERENCE")
        .collect();
    assert_eq!(obs.len(), 1);
    assert_eq!(
        obs[0]["target"]["byte_start"],
        source.find("BCM_mldsa65_generate_key").unwrap()
    );
    assert_eq!(algorithm_names(&native, obs[0]), vec!["ML-DSA-65"]);
}

#[test]
fn additional_canonical_c_and_cpp_apis_keep_unknown_key_and_certificate_algorithms_unknown() {
    let f = Fixture::new();
    for (extension, expression, expected) in [
        (
            "c",
            "wc_RsaSSL_Sign(data,size,out,cap,key,rng)",
            vec!["RSA"],
        ),
        (
            "cpp",
            "secp256k1_ecdsa_sign(ctx,sig,msg,key,nonce,data)",
            vec!["ECDSA"],
        ),
        ("c", "X509_verify_cert(store)", vec![]),
        (
            "cpp",
            "Botan::PK_Signer(key,rng,padding,format,provider)",
            vec![],
        ),
        (
            "cpp",
            "Botan::PK_Verifier(key,padding,format,provider)",
            vec![],
        ),
    ] {
        let source = format!("void f(){{ {expression}; }}");
        let native = f.native(&source, extension);
        let start = source.find(expression).unwrap();
        let obs = focal(&native, start, start + expression.len());
        assert_eq!(
            obs.len(),
            1,
            "missing canonical source request: {expression}"
        );
        assert_eq!(algorithm_names(&native, obs[0]), expected);
        assert_eq!(obs[0]["usage"], "referenced");
    }
    for source in [
        "#define wc_RsaSSL_Sign(a,b,c,d,e,f) local(a)\nvoid f(){wc_RsaSSL_Sign(a,b,c,d,e,f);}",
        "void f(){int secp256k1_ecdsa_sign; secp256k1_ecdsa_sign(a,b,c,d,e,f);}",
        "namespace Botan {class PK_Signer {}; } void f(){Botan::PK_Signer(key,rng,pad,format,provider);}",
    ] {
        assert!(f.native(source, "cpp")["observations"].as_array().unwrap().is_empty());
    }
}

#[test]
fn implementation_and_conditional_construction_references_are_separate_from_api_calls() {
    let f = Fixture::new();
    for (extension, source, rule, algorithm) in [
        (
            "cpp",
            "int ECDSA_sign(int a){return 0;}",
            "SOURCE_IMPLEMENTATION_REFERENCE",
            "ECDSA",
        ),
        (
            "cc.inc",
            "int BCM_mlkem768_generate_key(int a){return 0;}",
            "SOURCE_IMPLEMENTATION_REFERENCE",
            "ML-KEM-768",
        ),
        (
            "cc.inc",
            "int BCM_mldsa65_generate_key(int a){return 0;}",
            "SOURCE_IMPLEMENTATION_REFERENCE",
            "ML-DSA-65",
        ),
        (
            "c",
            "#ifdef USE_SNTRUP761X25519\nint f(){return 0;}\n#endif\n",
            "SOURCE_CONSTRUCTION_REFERENCE",
            "sntrup761x25519-sha512",
        ),
    ] {
        let native = f.native(source, extension);
        let obs: Vec<_> = native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|o| o["rule_id"] == rule)
            .collect();
        assert_eq!(obs.len(), 1, "missing explicitly separate evidence: {rule}");
        assert_eq!(algorithm_names(&native, obs[0]), vec![algorithm]);
        assert_eq!(obs[0]["usage"], "referenced");
        assert!(!native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["rule_id"] == "C_FAMILY_CRYPTO_API"));
    }
    for source in [
        "void f(){const char *s=\"USE_SNTRUP761X25519\";}",
        "int NotECDSA_sign(int a){return 0;}",
        "int ECDSA_sign(int a);",
    ] {
        assert!(f.native(source, "cpp")["observations"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}

#[test]
fn cpp_direct_initializer_retains_nested_request_with_unambiguous_nullptr_argument() {
    let f = Fixture::new();
    for wrapper in ["Holder<EVP_PKEY_CTX>", "other::Owner<EVP_PKEY_CTX>"] {
        let expression = "EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr)";
        let source = format!("void f(){{ {wrapper} value({expression}); }}");
        let (native, legacy) = f.document(&source, "cpp");
        let start = source.find(expression).unwrap();
        let observations = focal(&native, start, start + expression.len());
        assert_eq!(observations.len(), 1, "nested request lost as a declarator");
        assert_eq!(algorithm_names(&native, observations[0]), vec!["RSA"]);
        assert_eq!(observations[0]["assurance"], "syntactic");
        assert_eq!(observations[0]["usage"], "referenced");
        assert!(!legacy.to_string().contains("EVP_PKEY_CTX_new_id("));
    }
}

#[test]
fn cpp_direct_initializer_does_not_promote_possible_declarations_or_shadowed_apis() {
    let f = Fixture::new();
    for source in [
        "void f(){ Holder<EVP_PKEY_CTX> value(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, OtherType)); }",
        "Holder<EVP_PKEY_CTX> value(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr));",
        "#define EVP_PKEY_CTX_new_id(a,b) fake(a)\nvoid f(){ Holder<EVP_PKEY_CTX> value(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr)); }",
        "void f(){ int EVP_PKEY_CTX_new_id; Holder<EVP_PKEY_CTX> value(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr)); }",
    ] {
        let native = f.native(source, "cpp");
        assert!(native["observations"].as_array().unwrap().is_empty(), "declaration or namesake: {source}");
    }
}

#[test]
fn c_family_raw_key_imports_retain_documented_selectors_and_local_key_roles() {
    let f = Fixture::new();
    for extension in ["c", "cpp"] {
        for (constructor, expected) in [
            (
                "EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,bytes,size)",
                "Ed25519",
            ),
            (
                "EVP_PKEY_new_raw_public_key(EVP_PKEY_X25519,NULL,bytes,size)",
                "X25519",
            ),
            (
                "EVP_PKEY_new_raw_private_key_ex(libctx,\"ED25519\",NULL,bytes,size)",
                "Ed25519",
            ),
            (
                "EVP_PKEY_new_raw_public_key_ex(libctx,\"ML-KEM-768\",NULL,bytes,size)",
                "ML-KEM-768",
            ),
        ] {
            let source = format!("void f(){{ EVP_PKEY *key={constructor}; EVP_PKEY_CTX_new_from_pkey(libctx,key,NULL); }}");
            let native = f.native(&source, extension);
            for expression in [constructor, "EVP_PKEY_CTX_new_from_pkey(libctx,key,NULL)"] {
                let start = source.find(expression).unwrap();
                let observations = focal(&native, start, start + expression.len());
                assert_eq!(
                    observations.len(),
                    1,
                    "documented raw key request was omitted: {constructor}"
                );
                assert_eq!(algorithm_names(&native, observations[0]), vec![expected]);
                assert_eq!(observations[0]["assurance"], "syntactic");
                assert_eq!(observations[0]["usage"], "referenced");
            }
        }
    }
}

#[test]
fn c_family_raw_imports_do_not_guess_wrong_selector_dialects_or_expose_key_arguments() {
    let f = Fixture::new();
    for constructor in [
        "EVP_PKEY_new_raw_private_key(\"ED25519\",NULL,\"PRIVATE084_RAW_IMPORT_CANARY\",32)",
        "EVP_PKEY_new_raw_private_key_ex(NULL,EVP_PKEY_ED25519,NULL,\"PRIVATE084_RAW_IMPORT_CANARY\",32)",
        "EVP_PKEY_new_raw_private_key_ex(NULL,\"HMAC\",NULL,\"PRIVATE084_RAW_IMPORT_CANARY\",32)",
        "EVP_PKEY_new_raw_private_key(EVP_PKEY_RSA,NULL,\"PRIVATE084_RAW_IMPORT_CANARY\",32)",
        "EVP_PKEY_new_raw_private_key_ex(NULL,\"RSA\",NULL,\"PRIVATE084_RAW_IMPORT_CANARY\",32)",
    ] {
        let source = format!("void f(){{ EVP_PKEY *key={constructor}; EVP_PKEY_CTX_new_from_pkey(libctx,key,NULL); }}");
        let (native, legacy) = f.document(&source, "c");
        let start = source.find(constructor).unwrap();
        let observations = focal(&native, start, start + constructor.len());
        assert_eq!(observations.len(), 1, "unresolved source request still needs an observation");
        assert!(algorithm_names(&native, observations[0]).is_empty());
        assert!(!native.to_string().contains("PRIVATE084_RAW_IMPORT_CANARY"));
        assert!(!legacy.to_string().contains("PRIVATE084_RAW_IMPORT_CANARY"));
    }
    let shadowed = "#define EVP_PKEY_new_raw_private_key(a,b,c,d) local(a)\nvoid f(){EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,data,size);}";
    let native = f.native(shadowed, "c");
    assert!(native["observations"].as_array().unwrap().is_empty());
    for source in [
        "void f(){int EVP_PKEY_ED25519=7; EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,data,size);}",
        "#define EVP_PKEY_ED25519 7\nvoid f(){EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,data,size);}",
    ] {
        let native = f.native(source, "c");
        let expression = "EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,data,size)";
        let start = source.find(expression).unwrap();
        let observations = focal(&native, start, start + expression.len());
        assert_eq!(observations.len(), 1);
        assert!(algorithm_names(&native, observations[0]).is_empty(), "a local selector namesake cannot assert Ed25519");
    }
    let source="void f(){EVP_PKEY *key=EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,data,size); opaque(&key); EVP_DigestSignInit(md,NULL,NULL,NULL,key);}";
    let native = f.native(source, "c");
    let expression = "EVP_DigestSignInit(md,NULL,NULL,NULL,key)";
    let start = source.find(expression).unwrap();
    let observations = focal(&native, start, start + expression.len());
    assert_eq!(observations.len(), 1);
    assert!(
        algorithm_names(&native, observations[0]).is_empty(),
        "opaque writes must invalidate the imported key fact"
    );
}
#[test]
fn every_frozen_c_family_case_matches_its_original_focal_expectation() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let corpus: Value = serde_json::from_slice(
        &std::fs::read(root.join("tasks/076/evidence/CORPUS-CASES.json")).unwrap(),
    )
    .unwrap();
    let f = Fixture::new();
    let mut count = 0;
    for c in corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| matches!(c["language"].as_str(), Some("c" | "cpp")))
    {
        count += 1;
        let id = c["id"].as_str().unwrap();
        let source = std::fs::read_to_string(root.join(c["path"].as_str().unwrap())).unwrap();
        let extension = c["language"].as_str().unwrap();
        let input = ScannableFile::from_bytes(
            format!("case.{extension}").into(),
            source.as_bytes().to_vec(),
        );
        let o = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        let e = &c["expectation"];
        let expected_status = if e["status"] == "partial" {
            DetectorStatus::Partial
        } else {
            DetectorStatus::Complete
        };
        assert_eq!(o.status(), expected_status, "{id}: {o:?}");
        let native = f.native(&source, extension);
        let observations = native["observations"].as_array().unwrap();
        if let Some(span) = e["span"].as_array() {
            let start = span[0].as_u64().unwrap();
            let end = span[1].as_u64().unwrap();
            let actual: Vec<_> = observations
                .iter()
                .filter(|o| {
                    o["target"]["byte_start"] == start
                        && o["target"]["byte_end"] == end
                        && o["rule_id"] == e["rule"]
                })
                .collect();
            assert_eq!(actual.len(), 1, "{id}: expected {start}..{end}, {native}");
            let actual = actual[0];
            let names = algorithm_names(&native, actual);
            if let Some(name) = e["canonical_algorithm"].as_str() {
                assert_eq!(names, vec![name.to_owned()], "{id}");
            } else {
                assert!(names.is_empty(), "{id}: {names:?}");
            }
            assert_eq!(actual["assurance"], "syntactic");
            assert_eq!(actual["usage"], "referenced");
        } else if let Some(span) = e["forbidden_span"].as_array() {
            for observation in observations.iter().filter(|o| {
                o["target"]["byte_start"] == span[0] && o["target"]["byte_end"] == span[1]
            }) {
                let names = algorithm_names(&native, observation);
                for forbidden in e["forbidden_algorithms"].as_array().unwrap() {
                    assert!(
                        !names.iter().any(|n| n == forbidden.as_str().unwrap()),
                        "{id}: {names:?}"
                    );
                }
            }
        } else {
            assert!(observations.is_empty(), "{id}: {native}");
        }
    }
    assert_eq!(count, 484);
}

fn focal(native: &Value, start: usize, end: usize) -> Vec<&Value> {
    native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| {
            o["rule_id"]
                .as_str()
                .is_some_and(|id| id.starts_with("C_FAMILY_"))
                && o["target"]["byte_start"] == start
                && o["target"]["byte_end"] == end
        })
        .collect()
}
#[test]
fn c_family_configuration_and_include_references_keep_exact_public_identity_without_activation() {
    let f = Fixture::new();
    let source="#include <oqs/oqs.h>\nvoid f(){OSSL_PROVIDER_load(NULL,\"oqsprovider\"); EVP_set_default_properties(NULL,\"provider=legacy\"); SSL_CTX_set1_groups_list(ctx,\"X25519MLKEM768:P-256\");}";
    let (native, legacy) = f.document(source, "c");
    for (expression, marker, label) in [
        (
            "#include <oqs/oqs.h>",
            "library_ref_c_liboqs_include",
            "C/C++ liboqs include reference",
        ),
        (
            "OSSL_PROVIDER_load(NULL,\"oqsprovider\")",
            "source_provider_ref:oqsprovider",
            "C/C++ provider configuration reference: oqsprovider",
        ),
        (
            "EVP_set_default_properties(NULL,\"provider=legacy\")",
            "source_property_ref:provider=legacy",
            "C/C++ property configuration reference: provider=legacy",
        ),
        (
            "SSL_CTX_set1_groups_list(ctx,\"X25519MLKEM768:P-256\")",
            "source_tls_group_ref:X25519MLKEM768:P-256",
            "C/C++ TLS group configuration reference: X25519MLKEM768:P-256",
        ),
    ] {
        let start = source.find(expression).unwrap();
        let obs = native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["target"]["byte_start"] == start)
            .unwrap();
        assert_eq!(obs["evidence"]["marker_kind"], marker);
        assert!(algorithm_names(&native, obs).is_empty());
        assert_eq!(obs["usage"], "referenced");
        assert!(legacy["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["evidence"]["match"] == label));
    }
}
#[test]
fn c_family_local_key_flow_does_not_survive_writes_shadows_or_opaque_calls() {
    let f = Fixture::new();
    for extension in ["c", "cpp"] {
        for source in [
   "void f(){EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL); k=other; EVP_PKEY_encapsulate(k,data);}",
   "void f(){EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL); opaque(k); EVP_PKEY_encapsulate(k,data);}",
   "void f(){EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL); {EVP_PKEY_CTX *k=other; EVP_PKEY_encapsulate(k,data);}}",
  ]{
   let native=f.native(source,extension);let expr="EVP_PKEY_encapsulate(k,data)";let start=source.find(expr).unwrap();
   for obs in focal(&native,start,start+expr.len()){assert!(algorithm_names(&native,obs).is_empty(),"{source}: {native}");}
  }
        for source in [
            "void *RSA_new(){return NULL;} void f(){RSA_new();}",
            "void f(void *(*RSA_new)()){RSA_new();}",
            "#define RSA_new user_call\nvoid f(){RSA_new();}",
            "void f(){int RSA_new=0;RSA_new();}",
        ] {
            let native = f.native(source, extension);
            assert!(
                native["observations"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|o| algorithm_names(&native, o).is_empty()),
                "{source}: {native}"
            );
        }
    }
}
#[test]
fn c_family_private_arguments_are_not_exported_even_through_partial_shared_headers() {
    let f = Fixture::new();
    for extension in ["c", "cpp", "h"] {
        let source="void f(){\n#if FEATURE\nOQS_KEM *k=OQS_KEM_new(\"ML-KEM-768\"); OQS_KEM_decaps(k,\"PRIVATE080_KEY_CANARY_RS256\",data,more);\n#endif\n}";
        let (native, legacy) = f.document(source, extension);
        assert!(!native.to_string().contains("PRIVATE080_KEY_CANARY"));
        assert!(
            !legacy.to_string().contains("PRIVATE080_KEY_CANARY"),
            "{extension}: {legacy}"
        );
    }
}

#[test]
fn c_family_alias_and_write_limits_cannot_resurrect_old_bindings() {
    let f = Fixture::new();
    for aliases in [0, 7, 8, 9] {
        let mut source =
            "void f(){EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);"
                .to_owned();
        for i in 1..=aliases {
            source += &format!(
                "EVP_PKEY_CTX *a{i}={};",
                if i == 1 {
                    "k".into()
                } else {
                    format!("a{}", i - 1)
                }
            );
        }
        let expression = format!(
            "EVP_PKEY_encapsulate({},data)",
            if aliases == 0 {
                "k".into()
            } else {
                format!("a{aliases}")
            }
        );
        source += &format!("{expression};}}");
        let outcome = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        assert_eq!(
            outcome.status(),
            if aliases <= 8 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            },
            "{aliases}: {outcome:?}"
        );
        let native = f.native(&source, "c");
        let start = source.find(&expression).unwrap();
        let obs = focal(&native, start, start + expression.len());
        assert_eq!(obs.len(), 1);
        assert_eq!(
            algorithm_names(&native, obs[0]),
            if aliases <= 8 {
                vec!["ML-KEM-768"]
            } else {
                vec![]
            }
        );
    }
    for writes in [0usize, 63, 64, 65] {
        let mut source = if writes == 0 {
            "void f(){".to_owned()
        } else {
            "void f(){EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);"
                .to_owned()
        };
        source += &"k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);"
            .repeat(writes.saturating_sub(1));
        let expression = "EVP_PKEY_encapsulate(k,data)";
        source += &format!("{expression};}}");
        let outcome = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        assert_eq!(
            outcome.status(),
            if writes <= 64 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            },
            "{writes}: {outcome:?}"
        );
        let native = f.native(&source, "c");
        let start = source.find(expression).unwrap();
        let obs = focal(&native, start, start + expression.len());
        assert_eq!(
            algorithm_names(&native, obs[0]),
            if (1..=64).contains(&writes) {
                vec!["ML-KEM-768"]
            } else {
                vec![]
            }
        );
    }
}
#[test]
fn c_family_help_strings_qualified_names_macros_and_templates_do_not_fabricate_bindings() {
    let f = Fixture::new();
    for (extension,source) in [
  ("c","void f(){const char *help=\"OQS_KEM_new(ML-KEM-768) RSA_new()\";}"),
  ("cpp","void f(){app::RSA_new();}"),
  ("cpp","namespace oqs {class KeyEncapsulation{};} void f(){oqs::KeyEncapsulation k(\"ML-KEM-768\");k.encap_secret(data);}"),
  ("c","#define ALG \"ML-KEM-768\"\nvoid f(){OQS_KEM *k=OQS_KEM_new(ALG);OQS_KEM_encaps(k,data);}"),
  ("cpp","template<class T> void f(){OQS_KEM *k=OQS_KEM_new(T::algorithm);OQS_KEM_encaps(k,data);}"),
 ] {
  let native=f.native(source,extension);
  assert!(native["observations"].as_array().unwrap().iter().all(|o|algorithm_names(&native,o).is_empty()),"{source}: {native}");
 }
}
#[test]
fn c_family_literal_selectors_and_generated_key_roles_preserve_exact_drafts_and_agreement() {
    let f = Fixture::new();
    for (source,expression,expected) in [
  ("void f(){const char *name=\"ML-KEM-768\";EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,name,\"provider=default\");EVP_PKEY_encapsulate(k,data);}","EVP_PKEY_encapsulate(k,data)","ML-KEM-768"),
  ("void f(){EVP_PKEY_CTX *c=EVP_PKEY_CTX_new_id(EVP_PKEY_EC,NULL);EVP_PKEY *key=NULL;EVP_PKEY_keygen(c,&key);EVP_PKEY_CTX *derive=EVP_PKEY_CTX_new_from_pkey(NULL,key,NULL);EVP_PKEY_derive(derive,data);}","EVP_PKEY_derive(derive,data)","ECDH"),
  ("void f(){OQS_KEM *k=OQS_KEM_new(\"Kyber768\");OQS_KEM_encaps(k,data);}","OQS_KEM_encaps(k,data)","Kyber768"),
  ("void f(){OQS_SIG *k=OQS_SIG_new(\"Dilithium3\");OQS_SIG_sign(k,data);}","OQS_SIG_sign(k,data)","Dilithium3"),
 ]{
  let native=f.native(source,"c");let start=source.find(expression).unwrap();let obs=focal(&native,start,start+expression.len());assert_eq!(obs.len(),1,"{source}: {native}");assert_eq!(algorithm_names(&native,obs[0]),vec![expected]);
 }
    for source in [
  "void f(){OQS_KEM *k=OQS_KEM_new(\"ML-KEM-768\");OQS_SIG_sign(k,data);}",
  "void f(){EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);EVP_DigestSignInit(md,NULL,NULL,NULL,k);}",
 ]{
  let native=f.native(source,"c");let expr=if source.contains("OQS_SIG_sign"){"OQS_SIG_sign(k,data)"}else{"EVP_DigestSignInit(md,NULL,NULL,NULL,k)"};let start=source.find(expr).unwrap();for obs in focal(&native,start,start+expr.len()){assert!(algorithm_names(&native,obs).is_empty());}
 }
}

#[test]
fn c_family_local_enums_and_cpp_namespace_namesakes_do_not_select_real_apis() {
    let f = Fixture::new();
    for (extension,source) in [
  ("c","void f(){enum {EVP_PKEY_RSA=123};EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL);}"),
  ("cpp","struct oqs {class Signature {};};void f(){oqs::Signature k(\"ML-DSA-65\");k.sign(data);}"),
  ("cpp","namespace oqs=app;void f(){oqs::Signature k(\"ML-DSA-65\");k.sign(data);}"),
  ("cpp","void f(){oqs::KeyEncapsulation k(\"ML-KEM-768\"); k.reset(other); k.encap_secret(data);}"),
 ]{
  let native=f.native(source,extension);
  let observations=native["observations"].as_array().unwrap();
  if let Some(start)=source.find("k.encap_secret") {for o in observations.iter().filter(|o|o["target"]["byte_start"].as_u64().unwrap()>=start as u64){assert!(algorithm_names(&native,o).is_empty(),"{source}");}}
  else {assert!(observations.iter().all(|o|algorithm_names(&native,o).is_empty()),"{source}: {native}");}
 }
}

#[test]
fn c_family_wrong_constructor_suites_and_non_evp_keys_do_not_cross_roles() {
    let f = Fixture::new();
    for (extension,source,expression) in [
  ("cpp","void f(){oqs::KeyEncapsulation k(\"ML-DSA-65\");k.encap_secret(data);}","k.encap_secret(data)"),
  ("c","void f(){OQS_KEM *k=OQS_KEM_new(\"ML-KEM-768\");EVP_PKEY_CTX *c=EVP_PKEY_CTX_new_from_pkey(NULL,k,NULL);EVP_PKEY_encapsulate(c,data);}","EVP_PKEY_encapsulate(c,data)"),
  ("c","void f(){RSA *key=RSA_new();EVP_DigestSignInit(md,NULL,NULL,NULL,key);}","EVP_DigestSignInit(md,NULL,NULL,NULL,key)"),
 ] {
  let native=f.native(source,extension);let start=source.find(expression).unwrap();
  for obs in focal(&native,start,start+expression.len()){assert!(algorithm_names(&native,obs).is_empty(),"{source}: {native}");}
 }
}

#[test]
fn c_family_group_lists_retain_standard_draft_and_alias_spelling_and_reject_opaque_data() {
    let f = Fixture::new();
    for group in [
        "MLKEM768",
        "X25519MLKEM768",
        "X25519Kyber768Draft00",
        "SecP256r1MLKEM768",
        "P-256",
        "secp256r1",
    ] {
        let source = format!("void f(){{SSL_set1_groups_list(ssl,\"{group}\");}}");
        let (native, legacy) = f.document(&source, "c");
        let obs = &native["observations"][0];
        assert_eq!(
            obs["evidence"]["marker_kind"],
            format!("source_tls_group_ref:{group}")
        );
        assert!(algorithm_names(&native, obs).is_empty());
        assert!(legacy["findings"][0]["evidence"]["match"]
            .as_str()
            .unwrap()
            .ends_with(group));
    }
    for group in [
        "",
        "PRIVATE080_OPAQUE_SELECTOR",
        "?X25519MLKEM768",
        "X25519MLKEM768/P-256",
        "X-Wing",
        "MLKEM768:unrecognized",
    ] {
        let source = format!("void f(){{SSL_set1_groups_list(ssl,\"{group}\");}}");
        let input = ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            outcome.status(),
            DetectorStatus::Partial,
            "{group}: {outcome:?}"
        );
        let (native, legacy) = f.document(&source, "c");
        assert!(native["observations"][0]["evidence"]["marker_kind"].is_null());
        assert_eq!(
            legacy["findings"][0]["evidence"]["match"],
            "[masked-sensitive-content]"
        );
    }
    for count in [0usize, 15, 16, 17] {
        let group = vec!["X25519"; count].join(":");
        let source = format!("void f(){{SSL_set1_groups_list(ssl,\"{group}\");}}");
        let outcome = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        assert_eq!(
            outcome.status(),
            if (1..=16).contains(&count) {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Partial
            }
        );
    }
}
#[test]
fn c_family_source_positions_identity_and_utf16_bytes_are_original() {
    let f = Fixture::new();
    let source =
        "/* 日本語💻 */ void f(){OQS_KEM_new(\"ML-KEM-768\"); OQS_KEM_new(\"ML-KEM-768\");}";
    for ext in ["c", "cpp", "h"] {
        for wide in [false, true] {
            for entry in std::fs::read_dir(f.root.join("input")).unwrap() {
                std::fs::remove_file(entry.unwrap().path()).unwrap();
            }
            let bytes = if wide {
                let mut b = vec![0xff, 0xfe];
                b.extend(source.encode_utf16().flat_map(u16::to_le_bytes));
                b
            } else {
                source.as_bytes().to_vec()
            };
            std::fs::write(f.root.join(format!("input/case.{ext}")), &bytes).unwrap();
            let doc = scan_repository_native(
                &ScanConfig {
                    root: f.root.join("input"),
                    threads: 1,
                    ..Default::default()
                },
                &f.rules,
                &[Arc::new(TreeSitterDetector)],
                &NativeOptions::new(Some("v05/c-family")).unwrap(),
            )
            .unwrap();
            let native = serde_json::to_value(doc.native().unwrap()).unwrap();
            let observations = native["observations"].as_array().unwrap();
            assert_eq!(observations.len(), 2, "{ext}: {native}");
            assert_ne!(observations[0]["id"], observations[1]["id"]);
            for o in observations {
                let a = o["target"]["byte_start"].as_u64().unwrap() as usize;
                let z = o["target"]["byte_end"].as_u64().unwrap() as usize;
                let expression = "OQS_KEM_new(\"ML-KEM-768\")";
                assert_eq!(
                    &bytes[a..z],
                    if wide {
                        expression
                            .encode_utf16()
                            .flat_map(u16::to_le_bytes)
                            .collect::<Vec<_>>()
                    } else {
                        expression.as_bytes().to_vec()
                    }
                );
                let decoded = if wide {
                    String::from_utf16(
                        &bytes[2..a]
                            .chunks_exact(2)
                            .map(|b| u16::from_le_bytes([b[0], b[1]]))
                            .collect::<Vec<_>>(),
                    )
                    .unwrap()
                } else {
                    String::from_utf8(bytes[..a].to_vec()).unwrap()
                };
                assert_eq!(o["target"]["column"], decoded.chars().count() + 1);
                assert_eq!(o["target"]["column_kind"], "unicode_code_points");
            }
        }
    }
}
#[test]
fn c_family_all_detectors_keep_canaries_and_algorithm_looking_payloads_private_after_limits() {
    let f = Fixture::new();
    for extension in ["c", "cpp", "h"] {
        for canary in [
            "PRIVATE080_OPAQUE_CANARY_abcdefghijklmnopqrstuvwxyz0123456789",
            "RS256",
            "TLS_RSA_WITH_PRIVATE080",
        ] {
            for count in [0, 65] {
                let source=format!("void f(){{EVP_PKEY_CTX *k=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);{}OQS_KEM_decaps(imported,\"{canary}\",data,more);}}","k=other;".repeat(count));
                let mut bytes = vec![0xff, 0xfe];
                bytes.extend(source.encode_utf16().flat_map(u16::to_le_bytes));
                for data in [source.as_bytes().to_vec(), bytes] {
                    for entry in std::fs::read_dir(f.root.join("input")).unwrap() {
                        std::fs::remove_file(entry.unwrap().path()).unwrap();
                    }
                    std::fs::write(f.root.join(format!("input/case.{extension}")), data).unwrap();
                    let doc = scan_repository_native(
                        &ScanConfig {
                            root: f.root.join("input"),
                            threads: 1,
                            ..Default::default()
                        },
                        &f.rules,
                        &pqc_scan_detectors::default_detectors(),
                        &NativeOptions::new(Some("v05/c-family")).unwrap(),
                    )
                    .unwrap();
                    let native = serde_json::to_value(doc.native().unwrap()).unwrap();
                    let legacy = serde_json::to_value(doc.legacy()).unwrap();
                    // RS256 also occurs in fixed public JWT rule IDs/descriptions.
                    // Inspect payload-bearing fields and assets separately; retain
                    // those original public IDs rather than deleting findings.
                    if canary != "RS256" {
                        assert!(
                            !native.to_string().contains(canary),
                            "native {extension}/{count}: {native}"
                        );
                        assert!(
                            !legacy.to_string().contains(canary),
                            "legacy {extension}/{count}: {legacy}"
                        );
                    }
                    for finding in legacy["findings"].as_array().unwrap() {
                        assert!(!finding["evidence"].to_string().contains(canary));
                        assert!(!finding["source_snippet"].to_string().contains(canary));
                    }
                    assert!(native["assets"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter(|a| a["kind"] == "algorithm")
                        .all(|a| a["attributes"]["identifier"] == "ML-KEM-768"));
                    assert!(legacy["findings"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|f| f["evidence"]["snippet_preview"] == "[masked-sensitive-content]"));
                }
            }
        }
    }
}
#[test]
fn c_family_overlap_preserves_original_lexical_rule_occurrences_and_ids() {
    let f = Fixture::new();
    let source="void f(){RSA_new(); RSA_generate_key_ex(key,2048,e,NULL); EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL);}";
    std::fs::write(f.root.join("input/case.c"), source).unwrap();
    let scan = |detectors: &[Arc<dyn Detector>]| {
        scan_repository_native(
            &ScanConfig {
                root: f.root.join("input"),
                threads: 1,
                ..Default::default()
            },
            &f.rules,
            detectors,
            &NativeOptions::new(Some("v05/c-family")).unwrap(),
        )
        .unwrap()
    };
    let old = scan(&[Arc::new(pqc_scan_detectors::regex_detector::RegexDetector)]);
    let new = scan(&pqc_scan_detectors::default_detectors());
    assert!(!old.legacy().findings.is_empty());
    for old in &old.legacy().findings {
        let actual = new
            .legacy()
            .findings
            .iter()
            .find(|f| f.finding_id == old.finding_id)
            .unwrap();
        assert_eq!(actual.rule_id, old.rule_id);
        assert_eq!(actual.location.file, old.location.file);
        assert_eq!(actual.location.line, old.location.line);
        assert_eq!(actual.location.column, old.location.column);
        assert_eq!(actual.risk, old.risk);
    }
    assert!(new
        .legacy()
        .findings
        .iter()
        .any(|f| f.rule_id.starts_with("C_FAMILY_")));
}

#[test]
fn c_family_indirect_writes_and_unproven_named_methods_invalidate_local_selectors() {
    let f = Fixture::new();
    for (extension, mutation) in [
        ("c", "name[0]='x';"),
        ("c", "*name='x';"),
        ("cpp", "obj.sign(name);"),
    ] {
        let source=format!("void f(){{char *name=\"ML-KEM-768\";{mutation}EVP_PKEY_CTX_new_from_name(NULL,name,NULL);}}");
        let native = f.native(&source, extension);
        let expression = "EVP_PKEY_CTX_new_from_name(NULL,name,NULL)";
        let start = source.find(expression).unwrap();
        let obs = focal(&native, start, start + expression.len());
        assert_eq!(obs.len(), 1);
        assert!(
            algorithm_names(&native, obs[0]).is_empty(),
            "{source}: {native}"
        );
    }
}
#[test]
fn c_family_shared_builtin_budgets_have_zero_below_at_and_above_boundaries() {
    use pqc_scan_detectors::DetectorBudget;
    let f = Fixture::new();
    let expression = "OQS_KEM_new(\"ML-KEM-768\")";
    let source = format!("void f(){{{expression}; {expression};}}");
    let file = ScannableFile::from_bytes("case.h".into(), source.as_bytes().to_vec());
    for cap in ["hits", "matches", "candidates", "bytes"] {
        // Each grammar has nine generic candidates (f, two call nodes,
        // two callee identifiers, two string literals and two string contents)
        // alongside the two bounded facts. Generic collection retains its cap.
        let boundary = match cap {
            "bytes" => expression.len() * 4,
            "candidates" => 18,
            _ => 4,
        };
        for limit in [0, boundary - 1, boundary, boundary + 1] {
            let mut budget = DetectorBudget::default();
            match cap {
                "hits" => budget.max_hits_per_rule = limit,
                "matches" => budget.max_match_candidates = limit,
                "candidates" => budget.max_ast_candidates = limit,
                _ => budget.max_ast_match_bytes = limit,
            }
            let outcome = TreeSitterDetector::with_budget(budget)
                .detect_observed(&file, &f.rules)
                .unwrap();
            assert_eq!(
                outcome.status(),
                if limit < boundary {
                    DetectorStatus::Truncated
                } else {
                    DetectorStatus::Complete
                },
                "{cap}/{limit}: {outcome:?}"
            );
            if limit >= boundary {
                assert_eq!(outcome.detections().len(), 2, "{cap}");
            }
            assert!(outcome.sensitive_source_context());
        }
    }
    let c: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&c).unwrap();
    let tree = parser.parse(&source, None).unwrap();
    let mut nodes = vec![tree.root_node()];
    let mut total = 0;
    while let Some(n) = nodes.pop() {
        total += 1;
        let mut cursor = n.walk();
        nodes.extend(n.children(&mut cursor));
    }
    for limit in [0, total * 2 - 1, total * 2, total * 2 + 1] {
        let outcome = TreeSitterDetector::with_budget(DetectorBudget {
            max_ast_nodes: limit,
            ..Default::default()
        })
        .detect_observed(&file, &f.rules)
        .unwrap();
        assert_eq!(
            outcome.status(),
            if limit < total * 2 {
                DetectorStatus::Truncated
            } else {
                DetectorStatus::Complete
            },
            "nodes/{limit}: {outcome:?}"
        );
    }
}

#[test]
fn cpp_qualified_constructor_forms_and_multiple_declarators_keep_original_occurrences() {
    let f = Fixture::new();
    let source="void f(){oqs::KeyEncapsulation a{\"ML-KEM-768\"},b(\"ML-KEM-512\"); a.encap_secret(data); b.decap_secret(data); auto q=oqs::Signature(\"ML-DSA-44\"); q.sign(data); auto p=new oqs::Signature(\"ML-DSA-65\");}";
    let native = f.native(source, "cpp");
    for (expression, name) in [
        ("oqs::KeyEncapsulation a{\"ML-KEM-768\"}", "ML-KEM-768"),
        ("b(\"ML-KEM-512\")", "ML-KEM-512"),
        ("a.encap_secret(data)", "ML-KEM-768"),
        ("b.decap_secret(data)", "ML-KEM-512"),
        ("oqs::Signature(\"ML-DSA-44\")", "ML-DSA-44"),
        ("q.sign(data)", "ML-DSA-44"),
        ("new oqs::Signature(\"ML-DSA-65\")", "ML-DSA-65"),
    ] {
        let start = source.find(expression).unwrap();
        let obs = focal(&native, start, start + expression.len());
        assert_eq!(obs.len(), 1, "{expression}: {native}");
        assert_eq!(algorithm_names(&native, obs[0]), vec![name]);
    }
}

#[test]
fn c_family_incomplete_name_indices_never_turn_deep_shadowing_into_builtin_apis() {
    let f = Fixture::new();
    for pointers in [0usize, 6, 7, 8] {
        let source = format!("void f(int {}RSA_new){{RSA_new();}}", "*".repeat(pointers));
        let input = ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            outcome.status(),
            if pointers < 8 {
                DetectorStatus::Partial
            } else {
                DetectorStatus::Truncated
            },
            "{pointers}: {outcome:?}"
        );
        let native = f.native(&source, "c");
        assert!(
            native["observations"]
                .as_array()
                .unwrap()
                .iter()
                .all(|o| algorithm_names(&native, o).is_empty()),
            "{pointers}: {native}"
        );
    }
    for nested in [0usize, 61, 62, 63] {
        let source = format!(
            "void f(int RSA_new){{{}RSA_new();{}}}",
            "{".repeat(nested),
            "}".repeat(nested)
        );
        let input = ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            outcome.status(),
            if nested <= 62 {
                DetectorStatus::Partial
            } else {
                DetectorStatus::Truncated
            },
            "{nested}: {outcome:?}"
        );
        let native = f.native(&source, "c");
        assert!(native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|o| algorithm_names(&native, o).is_empty()));
    }
}

#[test]
fn c_family_selector_vocabulary_does_not_coerce_jose_generic_or_tls_names_into_key_types() {
    let f = Fixture::new();
    for name in [
        "ML-KEM",
        "ML-DSA",
        "SLH-DSA",
        "RS256",
        "X25519Kyber768Draft00",
        "X25519MLKEM768",
        "ML-KEM-513",
        "PRIVATE080_SELECTOR",
    ] {
        let source = format!("void f(){{EVP_PKEY_CTX_new_from_name(NULL,\"{name}\",NULL);}}");
        let input = ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            outcome.status(),
            DetectorStatus::Partial,
            "{name}: {outcome:?}"
        );
        let native = f.native(&source, "c");
        assert!(
            native["observations"]
                .as_array()
                .unwrap()
                .iter()
                .all(|o| algorithm_names(&native, o).is_empty()),
            "{name}: {native}"
        );
    }
}

#[test]
fn c_family_identifier_boundaries_and_hostile_resolution_work_are_visible() {
    let f = Fixture::new();
    for len in [255usize, 256, 257] {
        let name = "k".repeat(len);
        let call = format!("EVP_PKEY_encapsulate({name},data)");
        let source=format!("void f(){{EVP_PKEY_CTX *{name}=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);{call};}}");
        let outcome = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        assert_eq!(
            outcome.status(),
            if len <= 256 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            }
        );
        let native = f.native(&source, "c");
        let start = source.find(&call).unwrap();
        let obs = focal(&native, start, start + call.len());
        assert_eq!(obs.len(), 1);
        assert_eq!(
            algorithm_names(&native, obs[0]),
            if len <= 256 {
                vec!["ML-KEM-768"]
            } else {
                vec![]
            }
        );
    }
    for len in [511usize, 512, 513] {
        let source = format!("void f(){{{}();}}", "A".repeat(len));
        let outcome = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        assert_eq!(
            outcome.status(),
            if len <= 512 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            }
        );
        assert!(outcome.detections().is_empty());
    }
    let source = format!(
        "void f(int RSA_new){{{}{}{}}}",
        "{".repeat(60),
        "RSA_new();".repeat(200),
        "}".repeat(60)
    );
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(&source, None).unwrap();
    let mut stack = vec![tree.root_node()];
    let mut total = 0;
    while let Some(n) = stack.pop() {
        total += 1;
        let mut cursor = n.walk();
        stack.extend(n.children(&mut cursor));
    }
    let outcome = TreeSitterDetector::with_budget(pqc_scan_detectors::DetectorBudget {
        max_ast_nodes: total,
        ..Default::default()
    })
    .detect_observed(
        &ScannableFile::from_bytes("case.c".into(), source.as_bytes().to_vec()),
        &f.rules,
    )
    .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Truncated);
    assert!(outcome
        .diagnostic_message()
        .unwrap()
        .contains("c_family_resolution_work"));
    assert!(outcome.detections().is_empty());
}
#[test]
fn c_family_early_limits_and_malformed_inputs_still_protect_other_detector_context() {
    let f = Fixture::new();
    for ext in ["c", "cpp", "h"] {
        for source in [
            "void f(){OQS_KEM_decaps(imported,\"PRIVATE080_EARLY_KEY\",data); RSA_new();}",
            "void f(){OQS_KEM_decaps(imported,\"PRIVATE080_EARLY_KEY\",data); RSA_new();",
        ] {
            for entry in std::fs::read_dir(f.root.join("input")).unwrap() {
                std::fs::remove_file(entry.unwrap().path()).unwrap();
            }
            std::fs::write(f.root.join(format!("input/case.{ext}")), source).unwrap();
            let detectors: Vec<Arc<dyn Detector>> = vec![
                Arc::new(TreeSitterDetector::with_budget(
                    pqc_scan_detectors::DetectorBudget {
                        max_ast_nodes: 0,
                        ..Default::default()
                    },
                )),
                Arc::new(pqc_scan_detectors::regex_detector::RegexDetector),
            ];
            let doc = scan_repository_native(
                &ScanConfig {
                    root: f.root.join("input"),
                    threads: 1,
                    ..Default::default()
                },
                &f.rules,
                &detectors,
                &NativeOptions::new(Some("v05/c-family")).unwrap(),
            )
            .unwrap();
            assert!(!serde_json::to_string(doc.legacy())
                .unwrap()
                .contains("PRIVATE080_EARLY_KEY"));
            assert!(!doc.legacy().findings.is_empty());
            assert!(doc
                .legacy()
                .findings
                .iter()
                .all(|f| f.evidence.snippet_preview == "[masked-sensitive-content]"));
        }
    }
}

#[test]
fn c_family_global_variables_typedefs_and_aliases_cannot_impersonate_api_or_selector_names() {
    let f = Fixture::new();
    for (ext, source) in [
        ("c", "int RSA_new;void f(){RSA_new();}"),
        (
            "c",
            "int EVP_PKEY_RSA=123;void f(){EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL);}",
        ),
        (
            "c",
            "enum {EVP_PKEY_RSA=123};void f(){EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL);}",
        ),
        ("c", "typedef int RSA_new;void f(){RSA_new();}"),
        ("cpp", "using RSA_new=Custom;void f(){RSA_new();}"),
        ("cpp", "void f(){using RSA_new=Custom;RSA_new();}"),
        ("cpp", "struct RSA_new{};void f(){RSA_new();}"),
    ] {
        let native = f.native(source, ext);
        assert!(
            native["observations"]
                .as_array()
                .unwrap()
                .iter()
                .all(|o| algorithm_names(&native, o).is_empty()),
            "{source}: {native}"
        );
    }
}

#[test]
fn c_family_review1_output_context_and_opaque_out_pointer_updates_drop_old_bindings() {
    let f = Fixture::new();
    for ext in ["c", "cpp"] {
        for (file, expression) in [
            ("digest-out.c", "EVP_PKEY_keygen_init(ctx)"),
            ("pointer-out.c", "EVP_DigestSignInit(md,NULL,NULL,NULL,key)"),
        ] {
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../tasks/080/evidence/review1-reproduction/inputs")
                .join(file);
            let source = std::fs::read_to_string(path).unwrap();
            let native = f.native(&source, ext);
            let start = source.find(expression).unwrap();
            let obs = focal(&native, start, start + expression.len());
            assert_eq!(obs.len(), 1);
            assert!(
                algorithm_names(&native, obs[0]).is_empty(),
                "{ext}/{file}: {native}"
            );
        }
        let source="void f(){EVP_PKEY_CTX *a=EVP_PKEY_CTX_new_from_name(NULL,\"RSA\",NULL);EVP_PKEY *key=NULL;EVP_PKEY_keygen(a,&key);EVP_PKEY_CTX *ctx=EVP_PKEY_CTX_new_from_name(NULL,\"ML-KEM-768\",NULL);EVP_DigestSignInit(md,&ctx,NULL,NULL,key);EVP_PKEY_keygen_init(ctx);}";
        let native = f.native(source, ext);
        let expr = "EVP_PKEY_keygen_init(ctx)";
        let start = source.find(expr).unwrap();
        let obs = focal(&native, start, start + expr.len());
        assert_eq!(algorithm_names(&native, obs[0]), vec!["RSA"]);
    }
}
#[test]
fn c_family_review1_library_specific_slh_spelling_is_exact() {
    let f = Fixture::new();
    for ext in ["c", "cpp"] {
        for (call, valid) in [
            ("OQS_SIG_new(\"SLH_DSA_PURE_SHA2_128S\")", true),
            ("OQS_SIG_new(OQS_SIG_alg_slh_dsa_pure_sha2_128s)", true),
            ("OQS_SIG_new(\"SLH_DSA_PURE_SHA2_128s\")", false),
            ("OQS_SIG_new(\"SLH-DSA-SHA2-128s\")", false),
            (
                "EVP_PKEY_CTX_new_from_name(NULL,\"SLH_DSA_PURE_SHA2_128S\",NULL)",
                false,
            ),
            (
                "EVP_PKEY_CTX_new_from_name(NULL,\"SLH-DSA-SHA2-128s\",NULL)",
                true,
            ),
            ("EVP_PKEY_CTX_new_id(\"RSA\",NULL)", false),
            ("EVP_PKEY_CTX_new_from_name(NULL,EVP_PKEY_RSA,NULL)", false),
        ] {
            let source = format!("void f(){{{call};}}");
            let native = f.native(&source, ext);
            let start = source.find(call).unwrap();
            let obs = focal(&native, start, start + call.len());
            assert_eq!(obs.len(), 1, "{call}: {native}");
            assert_eq!(
                algorithm_names(&native, obs[0]),
                if valid {
                    vec!["SLH-DSA-SHA2-128s"]
                } else {
                    vec![]
                },
                "{call}: {native}"
            );
        }
    }
}
#[test]
fn cpp_review1_template_parameters_and_using_declarations_shadow_only_their_owners() {
    let f = Fixture::new();
    for source in [
  "template<class OQS_KEM_new> void f(){OQS_KEM_new(\"ML-KEM-768\");} void g(){OQS_KEM_new(\"ML-KEM-768\");}",
  "template<class RSA_new> void f(){RSA_new();} void g(){RSA_new();}",
  "namespace App {using Vendor::RSA_new;void f(){RSA_new();}} void g(){RSA_new();}",
 ] {
  let native=f.native(source,"cpp");let expression=if source.contains("OQS_KEM_new"){"OQS_KEM_new(\"ML-KEM-768\")"}else{"RSA_new()"};let first=source.find(expression).unwrap();let last=source.rfind(expression).unwrap();assert!(focal(&native,first,first+expression.len()).iter().all(|o|algorithm_names(&native,o).is_empty()),"{source}: {native}");let actual=focal(&native,last,last+expression.len());assert_eq!(actual.len(),1,"{source}: {native}");assert_eq!(algorithm_names(&native,actual[0]),if source.contains("OQS_KEM_new"){vec!["ML-KEM-768"]}else{vec!["RSA"]});
 }
    let source = "using Vendor::RSA_new;void f(){RSA_new();}";
    let native = f.native(source, "cpp");
    assert!(native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .all(|o| algorithm_names(&native, o).is_empty()));
    let source = "template<class RSA_new> void f(){::RSA_new();}";
    let native = f.native(source, "cpp");
    let expr = "::RSA_new()";
    let start = source.find(expr).unwrap();
    assert_eq!(
        algorithm_names(&native, focal(&native, start, start + expr.len())[0]),
        vec!["RSA"]
    );
}

#[test]
fn c_family_known_buffer_outputs_invalidate_selectors_but_preserve_immutable_object_roles() {
    let f = Fixture::new();
    let source="void f(){char name[]=\"ML-KEM-768\";OQS_KEM_decaps(imported,name,ct,sk);EVP_PKEY_CTX_new_from_name(NULL,name,NULL);}";
    let native = f.native(source, "c");
    let expr = "EVP_PKEY_CTX_new_from_name(NULL,name,NULL)";
    let start = source.find(expr).unwrap();
    assert!(algorithm_names(&native, focal(&native, start, start + expr.len())[0]).is_empty());
    let source="void f(){OQS_KEM *k=OQS_KEM_new(\"ML-KEM-768\");OQS_KEM_keypair(k,pk,sk);OQS_KEM_encaps(k,ct,ss,pk);}";
    let native = f.native(source, "c");
    let expr = "OQS_KEM_encaps(k,ct,ss,pk)";
    let start = source.find(expr).unwrap();
    assert_eq!(
        algorithm_names(&native, focal(&native, start, start + expr.len())[0]),
        vec!["ML-KEM-768"]
    );
}

#[test]
fn cpp_review1_qualified_namespace_roots_and_non_type_parameters_are_owner_scoped() {
    let f = Fixture::new();
    for source in [
  "template<class oqs> void f(){oqs::Signature s(\"ML-DSA-65\");s.sign(data);} void g(){oqs::Signature s(\"ML-DSA-65\");s.sign(data);}",
  "template<int(*RSA_new)()> void f(){RSA_new();} void g(){RSA_new();}",
  "template<class RSA_new=Custom> void f(){RSA_new();} void g(){RSA_new();}",
  "template<int RSA_new=0> void f(){RSA_new();} void g(){RSA_new();}",
  "template<template<class> class RSA_new> void f(){RSA_new();} void g(){RSA_new();}",
 ]{
  let native=f.native(source,"cpp");let expression=if source.contains("Signature"){"s.sign(data)"}else{"RSA_new()"};let first=source.find(expression).unwrap();let last=source.rfind(expression).unwrap();assert!(focal(&native,first,first+expression.len()).iter().all(|o|algorithm_names(&native,o).is_empty()),"{source}: {native}");let o=focal(&native,last,last+expression.len());assert_eq!(o.len(),1);assert_eq!(algorithm_names(&native,o[0]),if source.contains("Signature"){vec!["ML-DSA-65"]}else{vec!["RSA"]});
 }
}
