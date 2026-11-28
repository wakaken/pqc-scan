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
            "pqc-v05-csharp-{}-{}",
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
    fn native(&self, source: &str) -> Value {
        self.document(source).0
    }
    fn document(&self, source: &str) -> (Value, Value) {
        std::fs::write(self.root.join("input/case.cs"), source).unwrap();
        let ds: Vec<Arc<dyn Detector>> = vec![Arc::new(TreeSitterDetector)];
        let doc = scan_repository_native(
            &ScanConfig {
                root: self.root.join("input"),
                threads: 1,
                ..Default::default()
            },
            &self.rules,
            &ds,
            &NativeOptions::new(Some("v05/csharp")).unwrap(),
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
fn csharp_xml_signature_certificate_chain_and_legacy_rsa_formatter_are_bounded_requests() {
    let f = Fixture::new();
    for (source, expression, expected) in [
        ("using System.Security.Cryptography; class C {void F(){var f=new RSAPKCS1SignatureFormatter(key);}}", "new RSAPKCS1SignatureFormatter(key)", vec!["RSA"]),
        ("using System.Security.Cryptography.Xml; class C {void F(){var x=new SignedXml(doc); x.AddReference(r); x.ComputeSignature();}}", "x.ComputeSignature()", vec![]),
        ("using System.Security.Cryptography.X509Certificates; class C {void F(){using var chain=new X509Chain(); chain.ChainPolicy.RevocationMode = mode; if(chain.Build(cert)){} }}", "chain.Build(cert)", vec![]),
    ] {
        let native=f.native(source);let start=source.find(expression).unwrap();
        let obs:Vec<_>=native["observations"].as_array().unwrap().iter().filter(|o|o["target"]["byte_start"]==start).collect();
        assert_eq!(obs.len(),1,"missing correctly owned request: {expression}");
        assert_eq!(algorithm_names(&native,obs[0]),expected);
        assert_eq!(obs[0]["usage"],"referenced");
    }
    for source in [
        "class X509Chain {} class C {void F(){var chain=new X509Chain();chain.Build(cert);}}",
        "class C {void F(dynamic chain){chain.Build(cert);}}",
    ] {
        assert!(f.native(source)["observations"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}

#[test]
fn csharp_standard_rsa_formatters_do_not_borrow_bouncy_castle_signer_operations() {
    let f = Fixture::new();
    for (ty, method, expected) in [
        (
            "System.Security.Cryptography.RSAPKCS1SignatureFormatter",
            "CreateSignature",
            true,
        ),
        (
            "System.Security.Cryptography.RSAPKCS1SignatureDeformatter",
            "VerifySignature",
            true,
        ),
        (
            "Org.BouncyCastle.Crypto.Signers.RsaDigestSigner",
            "GenerateSignature",
            true,
        ),
        (
            "Org.BouncyCastle.Crypto.Signers.RsaDigestSigner",
            "CreateSignature",
            false,
        ),
        (
            "System.Security.Cryptography.RSAPKCS1SignatureFormatter",
            "GenerateSignature",
            false,
        ),
        (
            "System.Security.Cryptography.RSAPKCS1SignatureFormatter",
            "VerifySignature",
            false,
        ),
        (
            "System.Security.Cryptography.RSAPKCS1SignatureDeformatter",
            "CreateSignature",
            false,
        ),
    ] {
        let expression = format!("signer.{method}(data)");
        let source = format!("class C {{void F(){{var signer=new {ty}(key); {expression};}}}}");
        let native = f.native(&source);
        let start = source.find(&expression).unwrap();
        let obs: Vec<_> = native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|o| o["target"]["byte_start"] == start)
            .collect();
        assert_eq!(!obs.is_empty(), expected, "{ty}.{method}");
        if expected {
            assert_eq!(algorithm_names(&native, obs[0]), vec!["RSA"]);
        }
    }
}

#[test]
fn csharp_composite_implementation_component_does_not_become_whole_construction_safety() {
    let f = Fixture::new();
    let source="namespace Org.BouncyCastle.Crypto.Generators {class CompositeMLDsaKeyPairGenerator {void GenerateKeyPair(){GenerateTraditionalKeyPair(random);} void GenerateTraditionalKeyPair(object random){} }}";
    let native = f.native(source);
    let obs: Vec<_> = native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["rule_id"] == "SOURCE_CONSTRUCTION_REFERENCE")
        .collect();
    assert_eq!(obs.len(), 1);
    assert_eq!(algorithm_names(&native, obs[0]), vec!["CompositeMLDsa"]);
    assert_eq!(obs[0]["usage"], "referenced");
    assert_eq!(native["assessment_status"], "not_evaluated");
    assert!(f.native("namespace Other {class CompositeMLDsaKeyPairGenerator {void F(){GenerateTraditionalKeyPair(random);}}}")["observations"].as_array().unwrap().is_empty());
}

#[test]
fn csharp_identical_value_and_type_names_allow_static_factory_only_for_same_declared_type() {
    let f = Fixture::new();
    for (imports, declaration) in [
        (
            "using System.Security.Cryptography;",
            "RSA RSA { get; set; }",
        ),
        (
            "using RSA = System.Security.Cryptography.RSA;",
            "RSA RSA { get; set; }",
        ),
        (
            "using System.Security.Cryptography;",
            "System.Security.Cryptography.RSA RSA;",
        ),
    ] {
        let source =
            format!("{imports} class C {{ {declaration} void M(){{ var key = RSA.Create(); }} }}");
        let native = f.native(&source);
        let start = source.find("RSA.Create()").unwrap();
        let obs: Vec<_> = native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|o| o["target"]["byte_start"] == start)
            .collect();
        assert_eq!(
            obs.len(),
            1,
            "same declared type must permit static factory"
        );
        assert_eq!(algorithm_names(&native, obs[0]), vec!["RSA"]);
        assert_eq!(obs[0]["usage"], "referenced");
    }
}

#[test]
fn csharp_different_or_unknown_value_types_do_not_bypass_nearest_shadow() {
    let f = Fixture::new();
    for declaration in ["object RSA {get;set;}", "Other RSA;", "class RSA {}"] {
        let source = format!("using System.Security.Cryptography; class C {{ {declaration} void M(){{ RSA.Create(); }} }}");
        let native = f.native(&source);
        let start = source.find("RSA.Create()").unwrap();
        assert!(!native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["target"]["byte_start"] == start));
    }
    let source = "using System.Security.Cryptography; class C { RSA RSA {get;set;} void M(object RSA){ RSA.Create(); } }";
    let native = f.native(source);
    let start = source.find("RSA.Create()").unwrap();
    assert!(!native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|o| o["target"]["byte_start"] == start));
}
#[test]
fn every_frozen_csharp_case_matches_its_original_focal_expectation() {
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
        .filter(|c| c["language"] == "csharp")
    {
        count += 1;
        let id = c["id"].as_str().unwrap();
        let source = std::fs::read_to_string(root.join(c["path"].as_str().unwrap())).unwrap();
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
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
        let native = f.native(&source);
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
    assert_eq!(count, 218);
}

fn focal(native: &Value, start: usize, end: usize) -> Vec<&Value> {
    native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| {
            o["target"]["path"] == "case.cs"
                && o["target"]["byte_start"] == start
                && o["target"]["byte_end"] == end
        })
        .collect()
}

#[test]
fn csharp_bouncy_castle_rsa_digest_signer_tracks_only_resolved_library_types() {
    let f = Fixture::new();
    for (source, constructor) in [
        ("using Org.BouncyCastle.Crypto.Signers; class C { void M(){ var privateKey=\"PRIVATE084_CS_SIGNER_CANARY\"; var signer=new RsaDigestSigner(digest); signer.GenerateSignature(); }}", "new RsaDigestSigner(digest)"),
        ("using S=Org.BouncyCastle.Crypto.Signers.RsaDigestSigner; class C { void M(){ var signer=new S(digest); signer.GenerateSignature(); }}", "new S(digest)"),
    ] {
        let (native, legacy) = f.document(source);
        for expression in [constructor, "signer.GenerateSignature()"] {
            let start = source.find(expression).unwrap();
            let observations = focal(&native, start, start + expression.len());
            assert_eq!(observations.len(), 1, "a resolved RSA signature reference was omitted");
            assert_eq!(algorithm_names(&native, observations[0]), vec!["RSA"]);
            assert_eq!(observations[0]["assurance"], "syntactic");
            assert_eq!(observations[0]["usage"], "referenced");
        }
        assert!(!native.to_string().contains("PRIVATE084_CS_SIGNER_CANARY"));
        assert!(!legacy.to_string().contains("PRIVATE084_CS_SIGNER_CANARY"));
    }
    let source="using Org.BouncyCastle.Crypto.Signers; class RsaDigestSigner {} class C {void M(){var signer=new RsaDigestSigner(digest); signer.GenerateSignature();}}";
    assert!(f.native(source)["observations"]
        .as_array()
        .unwrap()
        .is_empty());
}

#[test]
fn csharp_aliases_local_global_usings_namespaces_and_static_imports_are_local() {
    let f = Fixture::new();
    for (source,expression,name) in [
        ("global using K = global::System.Security.Cryptography.MLKem; namespace App; class C { void M(){ var k=K.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768); k.Encapsulate(data); }}","k.Encapsulate(data)","ML-KEM-768"),
        ("using Crypto = System.Security.Cryptography; namespace App { class C { void M(){ var k=Crypto.RSA.Create(); k.Encrypt(data); }}}","k.Encrypt(data)","RSA"),
        ("using static System.Security.Cryptography.MLKem; using static System.Security.Cryptography.MLKemAlgorithm; class C {void M(){ var k=GenerateKey(MLKem768); k.Decapsulate(data); }}","k.Decapsulate(data)","ML-KEM-768"),
        ("class C {void M(System.Security.Cryptography.MLDsa key){ key.SignData(data); }}","key.SignData(data)","ML-DSA"),
    ] {
        let native=f.native(source);let start=source.find(expression).unwrap();let o=focal(&native,start,start+expression.len());
        assert_eq!(o.len(),1,"{source}: {native}");assert_eq!(algorithm_names(&native,o[0]),vec![name]);
    }
    for source in [
        "using System.Security.Cryptography; class MLKem {public static void GenerateKey(object a){}} class C {void M(){MLKem.GenerateKey(algorithm);}}",
        "namespace System.Security.Cryptography {class MLKem{}} class C{void M(){global::System.Security.Cryptography.MLKem.GenerateKey(algorithm);}}",
        "using System.Security.Cryptography; class C {void M(object MLKem){MLKem.GenerateKey(algorithm);}}",
        "using static System.Security.Cryptography.MLKem; class C {void GenerateKey(object a){} void M(){GenerateKey(algorithm);}}",
        "using static System.Security.Cryptography.MLKem; class C {void M(object GenerateKey){GenerateKey(algorithm);}}",
    ] {
        let native=f.native(source);
        // A real using-static type remains a reference; only the disputed local
        // call must not inherit the imported factory identity.
        let call=if source.contains("global::System.Security.Cryptography.MLKem.GenerateKey") {"global::System.Security.Cryptography.MLKem.GenerateKey(algorithm)"}else if source.contains("MLKem.GenerateKey(algorithm)"){"MLKem.GenerateKey(algorithm)"}else{"GenerateKey(algorithm)"};
        let start=source.rfind(call).unwrap();assert!(focal(&native,start,start+call.len()).is_empty(),"{source}: {native}");
    }
}

#[test]
fn csharp_parameters_named_arguments_and_conflicts_do_not_guess_exact_sets() {
    let f = Fixture::new();
    let expression="System.Security.Cryptography.MLKem.ImportPrivateSeed(source: bytes, algorithm: System.Security.Cryptography.MLKemAlgorithm.MLKem768)";
    let source = format!("class C{{void M(){{var key={expression};}}}}");
    let native = f.native(&source);
    let start = source.find(expression).unwrap();
    let o = focal(&native, start, start + expression.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["ML-KEM-768"]);
    let expression="new Org.BouncyCastle.Crypto.Signers.MLDsaSigner(deterministic:false,parameters:Org.BouncyCastle.Crypto.Parameters.MLDsaParameters.ml_dsa_65)";
    let source =
        format!("class C{{void M(){{var key={expression}; key.GenerateSignature(data);}}}}");
    let native = f.native(&source);
    let call = "key.GenerateSignature(data)";
    let start = source.find(call).unwrap();
    let o = focal(&native, start, start + call.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["ML-DSA-65"]);
    for argument in ["dynamicAlgorithm", "System.Security.Cryptography.MLDsaAlgorithm.MLDsa65", "System.Security.Cryptography.MLKemAlgorithm.MLKem512, algorithm:System.Security.Cryptography.MLKemAlgorithm.MLKem768"]{
        let expression=format!("System.Security.Cryptography.MLKem.GenerateKey({argument})");let source=format!("class C{{void M(){{var k={expression};}}}}");let native=f.native(&source);let start=source.find(&expression).unwrap();let o=focal(&native,start,start+expression.len());assert_eq!(o.len(),1);assert_eq!(algorithm_names(&native,o[0]),vec!["ML-KEM"]);
    }
}

#[test]
fn csharp_local_writes_method_boundaries_and_unknown_mutations_stop_old_keys() {
    let f = Fixture::new();
    for write in [
        "k=other;",
        "Opaque(ref k);",
        "Opaque(out k);",
        "(k,x)=other;",
        "k.UnknownMutation();",
        "var alias=k; Opaque(alias);",
        "k.Algorithm=other;",
    ] {
        let source=format!("class C{{void M(){{var k=System.Security.Cryptography.MLKem.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768); {write} k.Encapsulate(data);}}}}");
        let native = f.native(&source);
        let call = "k.Encapsulate(data)";
        let start = source.find(call).unwrap();
        assert!(
            focal(&native, start, start + call.len()).is_empty(),
            "{write}: {native}"
        );
    }
    let source="class C {void A(){var k=System.Security.Cryptography.RSA.Create();} void B(){k.SignData(data);}}";
    let native = f.native(source);
    let call = "k.SignData(data)";
    let start = source.find(call).unwrap();
    assert!(focal(&native, start, start + call.len()).is_empty());
    let source="class C{void M(){var k=System.Security.Cryptography.RSA.Create(); {var inner=System.Security.Cryptography.DSA.Create(); inner.SignData(data);} k.SignData(data);}}";
    let native = f.native(source);
    let call = "k.SignData(data)";
    let start = source.find(call).unwrap();
    let o = focal(&native, start, start + call.len());
    assert_eq!(algorithm_names(&native, o[0]), vec!["RSA"]);
}

#[test]
fn csharp_bc_initialization_resolves_only_the_expected_parameter_container() {
    let f = Fixture::new();
    for (container,expected) in [
        ("new Org.BouncyCastle.Crypto.Parameters.MLKemKeyGenerationParameters(random,Org.BouncyCastle.Crypto.Parameters.MLKemParameters.ml_kem_768)","ML-KEM-768"),
        ("new ForeignOptions(Org.BouncyCastle.Crypto.Parameters.MLKemParameters.ml_kem_768)","ML-KEM"),
        ("Org.BouncyCastle.Crypto.Parameters.MLKemParameters.ml_kem_768","ML-KEM"),
    ]{
        let source=format!("class C{{void M(){{var generator=new Org.BouncyCastle.Crypto.Generators.MLKemKeyPairGenerator(); generator.Init({container}); generator.GenerateKeyPair();}}}}");
        let native=f.native(&source);let call="generator.GenerateKeyPair()";let start=source.find(call).unwrap();let o=focal(&native,start,start+call.len());assert_eq!(o.len(),1);assert_eq!(algorithm_names(&native,o[0]),vec![expected],"{container}");
    }
    let source="class C{void M(){var k=new Org.BouncyCastle.Crypto.Signers.MLDsaSigner(Org.BouncyCastle.Crypto.Parameters.MLDsaParameters.ml_dsa_65,false); k.Init(true, importedKey); k.GenerateSignature(data);}}";
    let native = f.native(source);
    let call = "k.GenerateSignature(data)";
    let start = source.find(call).unwrap();
    let o = focal(&native, start, start + call.len());
    assert_eq!(algorithm_names(&native, o[0]), vec!["ML-DSA-65"]);
}

#[test]
fn csharp_explicit_local_types_keep_only_nominal_family_for_unknown_initializers() {
    let f = Fixture::new();
    let source =
        "class C{void M(){System.Security.Cryptography.RSA k=GetImportedKey(); k.SignData(data);}}";
    let native = f.native(source);
    let call = "k.SignData(data)";
    let start = source.find(call).unwrap();
    let o = focal(&native, start, start + call.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["RSA"]);
}

#[test]
fn csharp_alias_and_binding_limits_report_truncation_at_actual_boundaries() {
    let f = Fixture::new();
    for aliases in [7, 8, 9] {
        let mut source="class C{void M(){var k=System.Security.Cryptography.MLKem.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768);".to_owned();
        for i in 1..=aliases {
            source.push_str(&format!(
                "var a{i}={};",
                if i == 1 {
                    "k".into()
                } else {
                    format!("a{}", i - 1)
                }
            ));
        }
        let call = format!("a{aliases}.Encapsulate(data)");
        source.push_str(&format!("{call};}}}}"));
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        let native = f.native(&source);
        let start = source.find(&call).unwrap();
        let o = focal(&native, start, start + call.len());
        if aliases <= 8 {
            assert_eq!(outcome.status(), DetectorStatus::Complete);
            assert_eq!(o.len(), 1);
            assert_eq!(algorithm_names(&native, o[0]), vec!["ML-KEM-768"]);
        } else {
            assert_eq!(outcome.status(), DetectorStatus::Truncated);
            assert!(o.is_empty());
        }
    }
    for count in [63, 64, 65] {
        let mut source =
            "class C{void M(){var k=System.Security.Cryptography.RSA.Create();".to_owned();
        source.push_str(&"k=System.Security.Cryptography.RSA.Create();".repeat(count - 1));
        source.push_str("k.SignData(data);}}");
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            outcome.status(),
            if count <= 64 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            }
        );
        let native = f.native(&source);
        let call = "k.SignData(data)";
        let start = source.find(call).unwrap();
        let o = focal(&native, start, start + call.len());
        if count <= 64 {
            assert_eq!(algorithm_names(&native, o[0]), vec!["RSA"]);
        } else {
            assert!(o.is_empty());
        }
    }
}

#[test]
fn csharp_global_aliases_do_not_cross_files_and_conditionals_stay_partial() {
    let f = Fixture::new();
    std::fs::write(
        f.root.join("input/other.cs"),
        "global using K=System.Security.Cryptography.MLKem;",
    )
    .unwrap();
    let source = "class C{void M(){K.GenerateKey(algorithm);}}";
    let native = f.native(source);
    let call = "K.GenerateKey(algorithm)";
    let start = source.find(call).unwrap();
    assert!(focal(&native, start, start + call.len()).is_empty());
    std::fs::remove_file(f.root.join("input/other.cs")).unwrap();
    let source="class C{void M(){\n#if FEATURE\nvar k=System.Security.Cryptography.MLKem.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768);\n#endif\nk.Encapsulate(data);}}";
    let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
    let o = TreeSitterDetector
        .detect_observed(&input, &f.rules)
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    let native = f.native(source);
    let call = "k.Encapsulate(data)";
    let start = source.find(call).unwrap();
    assert!(focal(&native, start, start + call.len()).is_empty());
}

#[test]
fn csharp_wrapper_projects_and_bc_tls_are_distinct_opaque_references() {
    let f = Fixture::new();
    let source =
        "using dotnetOQS; using OpenForge.Cryptography.LibOqs.KEM; using Org.BouncyCastle.Tls;";
    let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
    let o = TreeSitterDetector
        .detect_detailed(&input, &f.rules)
        .unwrap();
    assert_eq!(o.detections().len(), 3);
    let previews: Vec<_> = o
        .detections()
        .iter()
        .map(|d| d.evidence.r#match.as_str())
        .collect();
    assert!(previews.iter().any(|p| p.contains("obsolete official")));
    assert!(previews.iter().any(|p| p.contains("OpenForge community")));
    assert!(previews.iter().any(|p| p.contains("Bouncy Castle TLS")));
    let (native, legacy) = f.document(source);
    let expected = [
        (
            "dotnetOQS",
            "C# obsolete official dotnetOQS wrapper reference",
            "library_ref_csharp_dotnetoqs_legacy",
        ),
        (
            "OpenForge.Cryptography.LibOqs.KEM",
            "C# OpenForge community OQS wrapper reference",
            "library_ref_csharp_openforge_community",
        ),
        (
            "Org.BouncyCastle.Tls",
            "C# Bouncy Castle TLS library reference",
            "library_ref_csharp_bouncycastle_tls",
        ),
    ];
    let mut anchors = std::collections::BTreeSet::new();
    for (name, label, marker) in expected {
        let start = source.find(name).unwrap();
        let obs = focal(&native, start, start + name.len())[0];
        assert!(algorithm_names(&native, obs).is_empty());
        assert_eq!(obs["assurance"], "syntactic");
        assert_eq!(obs["usage"], "referenced");
        assert_eq!(obs["evidence"]["marker_kind"], marker);
        let usage = native["assets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| {
                a["kind"] == "usage" && obs["asset_ids"].as_array().unwrap().contains(&a["id"])
            })
            .unwrap();
        anchors.insert(usage["attributes"]["anchor"].to_string());
        let index = legacy["findings"]
            .as_array()
            .unwrap()
            .iter()
            .position(|f| f["location"]["column"] == start + 1)
            .unwrap();
        assert_eq!(legacy["findings"][index]["evidence"]["match"], label);
        assert!(legacy["findings"][index]["evidence"]["snippet_preview"]
            .as_str()
            .unwrap()
            .contains("using dotnetOQS"));
        assert!(legacy["cbom"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["algorithm"] == label));
        assert!(native["legacy_associations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["legacy_index"] == index
                && a["observation_ids"]
                    .as_array()
                    .unwrap()
                    .contains(&obs["id"])));
    }
    assert_eq!(anchors.len(), 3);
}

#[test]
fn csharp_private_arguments_and_legacy_looking_payloads_are_protected_when_partial() {
    let f = Fixture::new();
    for canary in [
        "PRIVATE079_KEY_CANARY_abcdefghijklmnopqrstuvwxyz0123456789",
        "RS256",
        "TLS_RSA_WITH_CANARY079",
    ] {
        for count in [0, 65] {
            let source=format!("class C{{void M(){{var k=System.Security.Cryptography.MLKem.ImportPrivateSeed(System.Security.Cryptography.MLKemAlgorithm.MLKem768,\"{canary}\"); {} }}}}", "x=1;".repeat(count));
            let mut wide = vec![0xff, 0xfe];
            for c in source.encode_utf16() {
                wide.extend(c.to_le_bytes());
            }
            for bytes in [source.as_bytes().to_vec(), wide] {
                std::fs::write(f.root.join("input/case.cs"), bytes).unwrap();
                let doc = scan_repository_native(
                    &ScanConfig {
                        root: f.root.join("input"),
                        threads: 1,
                        ..Default::default()
                    },
                    &f.rules,
                    &pqc_scan_detectors::default_detectors(),
                    &NativeOptions::new(Some("v05/csharp")).unwrap(),
                )
                .unwrap();
                for finding in &doc.legacy().findings {
                    assert!(!serde_json::to_string(&finding.evidence)
                        .unwrap()
                        .contains(&format!("\"{canary}\"")));
                    assert!(!serde_json::to_string(&finding.source_snippet)
                        .unwrap()
                        .contains(&format!("\"{canary}\"")));
                }
                let native = serde_json::to_value(doc.native().unwrap()).unwrap();
                assert!(native["assets"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|a| a["kind"] != "algorithm" || a["attributes"]["identifier"] != canary));
                if canary != "RS256" {
                    assert!(!serde_json::to_string(doc.legacy())
                        .unwrap()
                        .contains(canary));
                    assert!(!serde_json::to_string(&native).unwrap().contains(canary));
                }
            }
        }
    }
}

#[test]
fn csharp_deferred_captures_and_implicit_parameter_names_do_not_reuse_outer_values() {
    let f = Fixture::new();
    let source="class C{void M(){var k=System.Security.Cryptography.MLKem.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768); Action later=()=>k.Encapsulate(data); k=other;}}";
    let native = f.native(source);
    let call = "k.Encapsulate(data)";
    let start = source.find(call).unwrap();
    assert!(focal(&native, start, start + call.len()).is_empty());
    for source in [
        "using System.Security.Cryptography; class C{void M(){Action<object> later=MLKem=>MLKem.GenerateKey(algorithm);}}",
        "using System.Security.Cryptography; class C{void M(){if(input is object MLKem){MLKem.GenerateKey(algorithm);}}}",
    ]{
        let native=f.native(source);let call="MLKem.GenerateKey(algorithm)";let start=source.find(call).unwrap();assert!(focal(&native,start,start+call.len()).is_empty(),"{source}: {native}");
    }
}

#[test]
fn csharp_prehash_parameter_reference_stays_unresolved_instead_of_becoming_pure() {
    let f = Fixture::new();
    let expression = "Org.BouncyCastle.Crypto.Parameters.MLDsaParameters.ml_dsa_65_with_sha512";
    let source = format!("class C{{void M(){{var p={expression};}}}}");
    let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
    let o = TreeSitterDetector
        .detect_observed(&input, &f.rules)
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    let native = f.native(&source);
    let start = source.find(expression).unwrap();
    let o = focal(&native, start, start + expression.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["ML-DSA"]);
}

#[test]
fn csharp_import_name_and_variable_caps_bound_real_sources() {
    let f = Fixture::new();
    for count in [0, 63, 64, 65] {
        let mut source = String::new();
        for i in 0..count {
            source.push_str(&format!("using A{i}=System.Security.Cryptography.RSA;"));
        }
        source.push_str("class C{void M(){System.Security.Cryptography.RSA.Create();}}");
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
        let o = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            o.status(),
            if count <= 64 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            }
        );
        let native = f.native(&source);
        let expr = "System.Security.Cryptography.RSA.Create()";
        let start = source.find(expr).unwrap();
        let hits = focal(&native, start, start + expr.len());
        assert_eq!(hits.len(), usize::from(count <= 64));
    }
    for bytes in [511, 512, 513] {
        let source = format!(
            "class {}{{}} class C{{void M(){{System.Security.Cryptography.RSA.Create();}}}}",
            "A".repeat(bytes)
        );
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
        let o = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            o.status(),
            if bytes <= 512 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            }
        );
        let native = f.native(&source);
        let expression = "System.Security.Cryptography.RSA.Create()";
        let start = source.find(expression).unwrap();
        assert_eq!(
            focal(&native, start, start + expression.len()).len(),
            usize::from(bytes <= 512)
        );
    }
    for bytes in [255, 256, 257] {
        let name = "a".repeat(bytes);
        let source=format!("class C{{void M(){{var {name}=System.Security.Cryptography.RSA.Create(); {name}.SignData(data);}}}}");
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
        let o = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(
            o.status(),
            if bytes <= 256 {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Truncated
            }
        );
    }
}

#[test]
fn csharp_unicode_same_line_calls_have_exact_original_wide_spans_and_distinct_ids() {
    let f = Fixture::new();
    let expression = "鍵.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768)";
    let source=format!("using 鍵=System.Security.Cryptography.MLKem; class C{{void M(){{var a={expression};var b={expression};}}}}");
    let mut bytes = vec![0xff, 0xfe];
    for c in source.encode_utf16() {
        bytes.extend(c.to_le_bytes());
    }
    std::fs::write(f.root.join("input/case.cs"), &bytes).unwrap();
    let ds: Vec<Arc<dyn Detector>> = vec![Arc::new(TreeSitterDetector)];
    let doc = scan_repository_native(
        &ScanConfig {
            root: f.root.join("input"),
            threads: 1,
            ..Default::default()
        },
        &f.rules,
        &ds,
        &NativeOptions::new(Some("v05/csharp")).unwrap(),
    )
    .unwrap();
    let native = serde_json::to_value(doc.native().unwrap()).unwrap();
    let mut ids = vec![];
    for (utf8_start, _) in source.match_indices(expression) {
        let start = 2 + 2 * source[..utf8_start].encode_utf16().count();
        let end = start + 2 * expression.encode_utf16().count();
        let hits = focal(&native, start, end);
        assert_eq!(hits.len(), 1);
        let obs = hits[0];
        assert_eq!(algorithm_names(&native, obs), vec!["ML-KEM-768"]);
        assert_eq!(
            obs["target"]["column"],
            source[..utf8_start].chars().count() + 1
        );
        assert_eq!(obs["target"]["column_kind"], "unicode_code_points");
        assert_eq!(
            &bytes[start..end],
            expression
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect::<Vec<_>>()
        );
        ids.push(obs["id"].clone());
    }
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
}

#[test]
fn csharp_type_parameters_and_properties_shadow_only_their_owning_type_or_method() {
    let f = Fixture::new();
    let expression = "MLKem.GenerateKey(MLKemAlgorithm.MLKem768)";
    for declaration in [
        "class C<MLKem> where MLKem:IFactory {void M(){var k=MLKem.GenerateKey(MLKemAlgorithm.MLKem768);}}",
        "class C {dynamic MLKem{get;set;} void M(){var k=MLKem.GenerateKey(MLKemAlgorithm.MLKem768);}}",
        "class C {void M<MLKem>() where MLKem:IFactory {var k=MLKem.GenerateKey(MLKemAlgorithm.MLKem768);}}",
    ]{
        let source=format!("using System.Security.Cryptography; interface IFactory{{static abstract object GenerateKey(MLKemAlgorithm a);}} {declaration} class D{{void M(){{var k={expression};}}}}");
        let native=f.native(&source);let positions:Vec<_>=source.match_indices(expression).map(|(i,_)|i).collect();assert_eq!(positions.len(),2);assert!(focal(&native,positions[0],positions[0]+expression.len()).is_empty(),"{declaration}: {native}");let valid=focal(&native,positions[1],positions[1]+expression.len());assert_eq!(valid.len(),1);assert_eq!(algorithm_names(&native,valid[0]),vec!["ML-KEM-768"]);
    }
}

#[test]
fn csharp_parameter_values_do_not_survive_a_possible_deferred_mutation() {
    let f = Fixture::new();
    let source="using System.Security.Cryptography; class C{void M(){var p=MLKemAlgorithm.MLKem768; void Change(){p=MLKemAlgorithm.MLKem512;} Change();var k=MLKem.GenerateKey(p);k.Encapsulate(data);}}";
    let native = f.native(source);
    for expression in ["MLKem.GenerateKey(p)", "k.Encapsulate(data)"] {
        let start = source.find(expression).unwrap();
        let o = focal(&native, start, start + expression.len());
        assert_eq!(o.len(), 1);
        assert_eq!(algorithm_names(&native, o[0]), vec!["ML-KEM"]);
    }
}

#[test]
fn csharp_namespace_alias_qualifiers_resolve_without_value_name_confusion() {
    let f = Fixture::new();
    for qualifier in ["Crypto::RSA", "global :: System.Security.Cryptography.RSA"] {
        let expression = format!("{qualifier}.Create()");
        let source=format!("using Crypto=System.Security.Cryptography;class C{{void M(object Crypto){{var k={expression};k.SignData(data);}}}}");
        let native = f.native(&source);
        let start = source.find(&expression).unwrap();
        let o = focal(&native, start, start + expression.len());
        assert_eq!(o.len(), 1, "{native}");
        assert_eq!(algorithm_names(&native, o[0]), vec!["RSA"]);
    }
    let source="using K=System.Security.Cryptography.MLKem;class C{void M(){K::GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768);}}";
    let expression = "K::GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768)";
    let native = f.native(source);
    let start = source.find(expression).unwrap();
    assert!(focal(&native, start, start + expression.len()).is_empty());
}

#[test]
fn csharp_parameter_container_does_not_survive_a_possible_deferred_mutation() {
    let f = Fixture::new();
    let source="class C{void M(){var options=new Org.BouncyCastle.Crypto.Parameters.MLKemKeyGenerationParameters(random,Org.BouncyCastle.Crypto.Parameters.MLKemParameters.ml_kem_768); void Change(){options=new Org.BouncyCastle.Crypto.Parameters.MLKemKeyGenerationParameters(random,Org.BouncyCastle.Crypto.Parameters.MLKemParameters.ml_kem_512);} Change();var g=new Org.BouncyCastle.Crypto.Generators.MLKemKeyPairGenerator();g.Init(options);g.GenerateKeyPair();}}";
    let native = f.native(source);
    let expression = "g.GenerateKeyPair()";
    let start = source.find(expression).unwrap();
    let o = focal(&native, start, start + expression.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["ML-KEM"]);
}

#[test]
fn csharp_opaque_wrapper_arguments_cannot_become_public_project_labels() {
    let f = Fixture::new();
    let source = r#"using dotnetOQS; class C { void M(){ var k=new dotnetOQS.KEM("PUBLIC087_SELECTOR"); } }"#;
    let (native, legacy) = f.document(source);
    assert!(!native.to_string().contains("PUBLIC087_SELECTOR"));
    assert!(legacy.to_string().contains("PUBLIC087_SELECTOR"));
    for finding in legacy["findings"].as_array().unwrap() {
        assert_eq!(
            finding["evidence"]["match"],
            "C# obsolete official dotnetOQS wrapper reference"
        );
        assert!(finding["source_snippet"]
            .to_string()
            .contains("PUBLIC087_SELECTOR"));
    }
}

#[test]
fn csharp_binding_saturation_retains_independent_references_and_truncated_status() {
    let f = Fixture::new();
    let source = format!(
        "class C{{void Busy(){{var value=0; {} }} void Independent(){{ System.Security.Cryptography.RSA.Create(); new Org.BouncyCastle.Crypto.Signers.SlhDsaSigner(parameters,false); }} }}",
        "value=1;".repeat(65)
    );
    let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
    let outcome = TreeSitterDetector
        .detect_observed(&input, &f.rules)
        .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Truncated);
    let native = f.native(&source);
    for (expression, expected) in [
        ("System.Security.Cryptography.RSA.Create()", "RSA"),
        (
            "new Org.BouncyCastle.Crypto.Signers.SlhDsaSigner(parameters,false)",
            "SLH-DSA",
        ),
    ] {
        let start = source.find(expression).unwrap();
        let observations = focal(&native, start, start + expression.len());
        assert_eq!(
            observations.len(),
            1,
            "independent reference lost after local saturation"
        );
        assert_eq!(algorithm_names(&native, observations[0]), vec![expected]);
        assert_eq!(observations[0]["assurance"], "syntactic");
        assert_eq!(observations[0]["usage"], "referenced");
    }
}

#[test]
fn csharp_saturated_values_do_not_restore_stale_variants_or_namesakes_or_secrets() {
    let f = Fixture::new();
    let source=format!(
        "class C{{void Busy(){{var choice=System.Security.Cryptography.MLKemAlgorithm.MLKem768; {} choice=unresolved; System.Security.Cryptography.MLKem.GenerateKey(choice);}} void Independent(){{var privateKey=\"PRIVATE084_CSHARP_CAP_CANARY\"; System.Security.Cryptography.RSA.Create();}} }} namespace Local{{class RSA{{public static void Create(){{}}}} class Other{{void M(){{RSA.Create();}}}}}}",
        "choice=System.Security.Cryptography.MLKemAlgorithm.MLKem768;".repeat(64)
    );
    let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
    assert_eq!(
        TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap()
            .status(),
        DetectorStatus::Truncated
    );
    let (native, legacy) = f.document(&source);
    let expression = "System.Security.Cryptography.MLKem.GenerateKey(choice)";
    let start = source.find(expression).unwrap();
    let observations = focal(&native, start, start + expression.len());
    assert_eq!(
        observations.len(),
        1,
        "unresolved algorithm retains only its documented family"
    );
    assert_eq!(algorithm_names(&native, observations[0]), vec!["ML-KEM"]);
    let start = source.rfind("RSA.Create()").unwrap();
    assert!(focal(&native, start, start + 12).is_empty());
    assert!(!native.to_string().contains("PRIVATE084_CSHARP_CAP_CANARY"));
    assert!(!legacy.to_string().contains("PRIVATE084_CSHARP_CAP_CANARY"));
}

#[test]
fn csharp_predefined_types_never_inherit_tls_namespace_or_import_qualification() {
    let f = Fixture::new();
    for prefix in [
        "namespace Org.BouncyCastle.Tls;",
        "using Org.BouncyCastle.Tls; namespace Application;",
    ] {
        for typ in [
            "bool", "byte", "sbyte", "char", "decimal", "double", "float", "int", "uint", "nint",
            "nuint", "long", "ulong", "short", "ushort", "object", "string", "int?", "int[]",
        ] {
            let source =
                format!("{prefix} class C {{ {typ} field; void F({typ} arg) {{ {typ} local; }} }}");
            let (native, legacy) = f.document(&source);
            let type_findings: Vec<_> = legacy["findings"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| {
                    v["evidence"]["metadata"]["source_resolution"] == "qualified_type_reference"
                })
                .collect();
            assert!(
                type_findings.is_empty(),
                "built-in type acquired nominal crypto meaning: {typ}"
            );
            for finding in legacy["findings"].as_array().unwrap() {
                assert_eq!(finding["location"]["line"], 1);
                assert!(prefix.starts_with("using"));
                assert_eq!(finding["location"]["column"], 7);
            }
            assert!(native["observations"].as_array().unwrap().len() <= 1);
        }
    }
    let (native, legacy) =
        f.document("namespace Org.BouncyCastle.Tls; class C { void F() { var local = 1; } }");
    assert!(native["observations"].as_array().unwrap().is_empty());
    assert!(legacy["findings"].as_array().unwrap().is_empty());
}

#[test]
fn csharp_tls_alias_qualified_and_escaped_identifier_type_controls_remain_references() {
    let f = Fixture::new();
    let source = "using TLS = Org.BouncyCastle.Tls.TlsClientProtocol; namespace Org.BouncyCastle.Tls; class C { TLS alias; TlsClientProtocol actual; global::Org.BouncyCastle.Tls.TlsClientProtocol full; @int escaped; }";
    let (native, legacy) = f.document(source);
    for token in [
        "TLS alias",
        "TlsClientProtocol actual",
        "global::Org.BouncyCastle.Tls.TlsClientProtocol full",
        "@int escaped",
    ] {
        let start = source.find(token).unwrap();
        let observations: Vec<_> = native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["target"]["byte_start"] == start)
            .collect();
        assert_eq!(
            observations.len(),
            1,
            "missing nominal type reference: {token}"
        );
        let findings: Vec<_> = legacy["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["location"]["column"] == start + 1)
            .collect();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0]["rule_id"], "CSHARP_CRYPTO_REFERENCE");
        assert_eq!(
            findings[0]["evidence"]["metadata"]["source_resolution"],
            "qualified_type_reference"
        );
    }
}

fn csharp_status(source: &str, fixture: &Fixture) -> DetectorStatus {
    let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
    TreeSitterDetector
        .detect_observed(&input, &fixture.rules)
        .unwrap()
        .status()
}

#[test]
fn csharp_partial_fluent_comment_padding_preserves_independent_rsa_references() {
    let f = Fixture::new();
    let expression = "System.Security.Cryptography.RSA.Create()";
    let mut controls = vec![];
    for padding in [1, 32] {
        let chain = format!(
            "new Builder(){}",
            format!(".Step(/*{}*/)", "x".repeat(padding)).repeat(20)
        );
        assert_eq!(chain.len(), if padding == 1 { 253 } else { 873 });
        let source = format!("class Builder {{ public Builder Step() {{ return this; }} }}\nclass Example {{ void Check() {{\n{expression};\n{chain};\n{expression};\n}} }}\n");
        controls.push(source.replace(&format!("/*{}*/", "x".repeat(padding)), ""));
        let native = f.native(&source);
        for (start, _) in source.match_indices(expression) {
            let observations = focal(&native, start, start + expression.len());
            assert_eq!(
                observations.len(),
                1,
                "padding {padding} lost an independent RSA reference"
            );
            assert_eq!(algorithm_names(&native, observations[0]), vec!["RSA"]);
            assert_eq!(observations[0]["usage"], "referenced");
        }
        assert_eq!(csharp_status(&source, &f), DetectorStatus::Complete);
    }
    assert_eq!(controls[0], controls[1]);
}

#[test]
fn csharp_partial_ordinary_receiver_and_generic_calls_do_not_create_crypto_uncertainty() {
    let f = Fixture::new();
    for body in [
        "var ordinary=new Builder(); Opaque(); ordinary.ToString();",
        "var ordinary=new Builder(); if(flag){ordinary=new Builder();} ordinary.ToString();",
        "var ordinary=new Builder(); ordinary.Route<int>();",
        "Route<int>();",
        "builder?.Route<int>();",
        "builder?.Invoke();",
        "(GetCallback())();",
    ] {
        let source = format!("class C{{void M(dynamic builder,bool flag){{{body}}}}}");
        assert_eq!(
            csharp_status(&source, &f),
            DetectorStatus::Complete,
            "{body}"
        );
        assert!(f.native(&source)["observations"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}

#[test]
fn csharp_partial_unresolved_supported_consumers_retain_uncertainty_without_guessed_keys() {
    let f = Fixture::new();
    for setup in [
        "",
        "var k=System.Security.Cryptography.RSA.Create(); k=other;",
        "var k=System.Security.Cryptography.RSA.Create(); Opaque(ref k);",
        "var k=System.Security.Cryptography.RSA.Create(); Opaque(out k);",
        "var k=System.Security.Cryptography.RSA.Create(); k.UnknownMutation();",
        "var k=System.Security.Cryptography.RSA.Create(); k.UnknownMutation<int>();",
        "var k=System.Security.Cryptography.RSA.Create(); (GetCallback())();",
        "var k=System.Security.Cryptography.RSA.Create(); Opaque(k);",
        "var k=System.Security.Cryptography.RSA.Create(); if(flag){k=other;}",
        "var k=System.Security.Cryptography.RSA.Create(); {dynamic k=other; k.Encrypt(data);}",
    ] {
        let call = "k.Encrypt(data)";
        let source = format!("class C{{dynamic k;void M(bool flag){{{setup}{call};}}}}");
        assert_eq!(
            csharp_status(&source, &f),
            DetectorStatus::Partial,
            "{setup}"
        );
        let native = f.native(&source);
        let start = source.rfind(call).unwrap();
        assert!(
            focal(&native, start, start + call.len()).is_empty(),
            "{setup}"
        );
    }
    for source in [
        "class C{System.Security.Cryptography.RSA k=System.Security.Cryptography.RSA.Create();void M(){k.Encrypt(data);}}",
        "class C{void M(){System.Security.Cryptography.MLKem.GenerateKey(unknownAlgorithm);}}",
    ] {
        assert_eq!(csharp_status(source, &f), DetectorStatus::Partial);
        let native = f.native(source);
        let call = "k.Encrypt(data)";
        if let Some(start) = source.find(call) {
            assert!(focal(&native, start, start + call.len()).is_empty());
        } else {
            let call = "System.Security.Cryptography.MLKem.GenerateKey(unknownAlgorithm)";
            let start = source.find(call).unwrap();
            let observations = focal(&native, start, start + call.len());
            assert_eq!(observations.len(), 1);
            assert_eq!(algorithm_names(&native, observations[0]), vec!["ML-KEM"]);
        }
    }
    for (declaration, call) in [
        ("dynamic k", "k.Encrypt<int>(data)"),
        ("dynamic k", "k?.Encrypt<int>(data)"),
        ("System.Security.Cryptography.RSA k", "k.Encrypt<int>(data)"),
    ] {
        let source = format!("class C{{void M({declaration}){{{call};}}}}");
        assert_eq!(
            csharp_status(&source, &f),
            DetectorStatus::Partial,
            "{call}"
        );
        let native = f.native(&source);
        let start = source.find(call).unwrap();
        assert!(focal(&native, start, start + call.len()).is_empty());
    }
}

#[test]
fn csharp_partial_conditional_relevance_stays_bounded_and_fail_closed() {
    let f = Fixture::new();
    for method in ["Encrypt<int>", "Route<int>"] {
        let receiver = format!("k{}", "?.Next".repeat(80));
        let source = format!("class C{{void M(dynamic k){{{receiver}?.{method}(data);}}}}");
        assert_eq!(csharp_status(&source, &f), DetectorStatus::Truncated);
        assert!(f.native(&source)["observations"]
            .as_array()
            .unwrap()
            .is_empty());
    }
    for source in [
        "class C{void M(dynamic k){k?.Encrypt<int>(data);}}",
        "class C{void M(dynamic k){k?.Encrypt<int>(",
    ] {
        let input = ScannableFile::from_bytes("case.cs".into(), source.as_bytes().to_vec());
        let limited = TreeSitterDetector::with_budget(pqc_scan_detectors::DetectorBudget {
            max_ast_candidates: 1,
            ..Default::default()
        })
        .detect_observed(&input, &f.rules)
        .unwrap();
        assert_eq!(limited.status(), DetectorStatus::Truncated);
        assert_eq!(csharp_status(source, &f), DetectorStatus::Partial);
        assert!(f.native(source)["observations"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}
