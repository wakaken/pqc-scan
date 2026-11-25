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

#[test]
fn php_xml_reference_and_signature_verification_use_real_case_insensitive_methods() {
    let f = Fixture::new();
    for method in ["validateReference", "VALIDATEREFERENCE", "verify"] {
        let expression = format!("$xml->{method}($key)");
        let source=format!("<?php use RobRichards\\XMLSecLibs\\XMLSecurityDSig as DS; function f(){{$xml=new DS(); {expression};}}");
        let native = f.native(&source);
        let start = source.find(&expression).unwrap();
        assert_eq!(
            native["observations"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|o| o["rule_id"] == "PHP_CRYPTO_API" && o["target"]["byte_start"] == start)
                .count(),
            1,
            "{method}"
        );
    }
    for source in ["<?php class XMLSecurityDSig {} function f(){$xml=new XMLSecurityDSig();$xml->validateReference($key);}","<?php function f($xml){$xml->validateReference($key);}","<?php use RobRichards\\XMLSecLibs\\XMLSecurityDSig; function f(){$xml=new XMLSecurityDSig();$xml->validateSignature($key);}"] {
        let native=f.native(source);let start=source.rfind("$xml->").unwrap();
        assert!(!native["observations"].as_array().unwrap().iter().any(|o|o["target"]["byte_start"]==start));
    }
}

#[test]
fn php_canonical_jwt_xml_and_type_checked_signers_keep_unknown_primitive_explicit() {
    let f = Fixture::new();
    for (source, expression) in [
        ("<?php use Firebase\\JWT\\JWT; JWT::decode($token,$keys);", "JWT::decode($token,$keys)"),
        ("<?php use RobRichards\\XMLSecLibs\\XMLSecurityDSig; function f(){ $x=new XMLSecurityDSig(); $x->addReference($r); $x->sign($key); }", "$x->sign($key)"),
        ("<?php use phpseclib3\\Crypt\\RSA\\PrivateKey; function f($key){ if (!$key instanceof PrivateKey) {throw new Exception();} $key->sign($data); }", "$key->sign($data)"),
    ] {
        let native=f.native(source);let start=source.find(expression).unwrap();
        let obs:Vec<_>=native["observations"].as_array().unwrap().iter().filter(|o|o["target"]["byte_start"]==start).collect();
        assert_eq!(obs.len(),1,"missing scoped library operation: {expression}");
        assert!(algorithm_names(&native,obs[0]).is_empty());
        assert_eq!(obs[0]["usage"],"referenced");
    }
    for source in [
        "<?php class JWT{static function decode($t,$k){}} JWT::decode($token,$keys);",
        "<?php function f($x){$x->sign($key);}",
        "<?php use phpseclib3\\Crypt\\RSA\\PrivateKey; function f($key,$other){if(!$other instanceof PrivateKey){throw new Exception();} $key->sign($data);}",
        "<?php use phpseclib3\\Crypt\\RSA\\PrivateKey; function f($key){if(!$key instanceof PrivateKey){throw new Exception();} $key=opaque(); $key->sign($data);}",
    ] {
        let native=f.native(source);
        let start=source.rfind("JWT::decode").or_else(||source.rfind("$key->sign")).or_else(||source.rfind("$x->sign")).unwrap();
        assert!(!native["observations"].as_array().unwrap().iter().any(|o|o["target"]["byte_start"]==start));
    }
}
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-v05-php-{}-{}",
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
        std::fs::write(self.root.join("input/case.php"), source).unwrap();
        let ds: Vec<Arc<dyn Detector>> = vec![Arc::new(TreeSitterDetector)];
        let doc = scan_repository_native(
            &ScanConfig {
                root: self.root.join("input"),
                threads: 1,
                ..Default::default()
            },
            &self.rules,
            &ds,
            &NativeOptions::new(Some("v05/php")).unwrap(),
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
fn every_frozen_php_case_matches_its_original_focal_expectation() {
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
        .filter(|c| c["language"] == "php")
    {
        count += 1;
        let id = c["id"].as_str().unwrap();
        let source = std::fs::read_to_string(root.join(c["path"].as_str().unwrap())).unwrap();
        let input = ScannableFile::from_bytes("case.php".into(), source.as_bytes().to_vec());
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
    assert_eq!(count, 180);
}

fn focal(native: &Value, start: usize, end: usize) -> Vec<&Value> {
    native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["target"]["byte_start"] == start && o["target"]["byte_end"] == end)
        .collect()
}
#[test]
fn php_namespaces_aliases_tuple_roles_and_source_order_are_bounded() {
    let f = Fixture::new();
    for (source,expected) in [
        ("<?php namespace App; use PQCrypto\\MLKem768 as K; [$sk,$pk]=K::GENERATEKEYPAIR(); $pk->encapsulate(); $sk->decapsulate($ct);",vec!["ML-KEM-768","ML-KEM-768","ML-KEM-768"]),
        ("<?php namespace App; use function sodium_crypto_kem_mlkem768_enc as ENC; ENC($pk);",vec!["ML-KEM-768"]),
        ("<?php namespace App; use PQCrypto\\{MLKem768 as K, MLDSA44 as S}; [$sk,$pk]=K::generateKeypair(); [$sign,$verify]=S::generateKeypair(); $pk->encapsulate(); $sign->sign($message);",vec!["ML-KEM-768","ML-DSA-44","ML-KEM-768","ML-DSA-44"]),
    ]{
        let native=f.native(source);let observations=native["observations"].as_array().unwrap();assert_eq!(observations.len(),expected.len(),"{native}");let mut actual:Vec<_>=observations.iter().map(|o|(o["target"]["byte_start"].as_u64().unwrap(),algorithm_names(&native,o))).collect();actual.sort_by_key(|r|r.0);assert_eq!(actual.into_iter().map(|r|r.1).collect::<Vec<_>>(),expected.into_iter().map(|n|vec![n.to_owned()]).collect::<Vec<_>>());
    }
    for source in [
        "<?php namespace App; function sodium_crypto_kem_keypair(){} sodium_crypto_kem_keypair();",
        "<?php namespace PQCrypto; class MLKem768 {} MLKem768::generateKeypair();",
        "<?php class MLKem768 {} MLKem768::generateKeypair();",
        "<?php $help='sodium_crypto_kem_keypair()'; $help2='\\PQCrypto\\MLKem768::generateKeypair()'; // sodium_crypto_kem_keypair()\n",
        "<?php sodium_crypto_scalarmult_ristretto255($n,$p); sodium_crypto_pwhash($x); openssl_encrypt($m,'aes-256-gcm',$k);",
    ]{assert!(f.native(source)["observations"].as_array().unwrap().is_empty(),"{source}");}
    let source="<?php [$sk,$pk]=\\PQCrypto\\MLKem768::generateKeypair(); $alias=$pk; $pk=$other; $alias->encapsulate(); $pk->encapsulate();";
    let native = f.native(source);
    let expression = "$alias->encapsulate()";
    let start = source.find(expression).unwrap();
    let o = focal(&native, start, start + expression.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["ML-KEM-768"]);
    let bad = "$pk->encapsulate()";
    let start = source.find(bad).unwrap();
    assert!(focal(&native, start, start + bad.len()).is_empty());
}
#[test]
fn php_unknown_keys_named_arguments_local_shadowing_and_privacy() {
    let f = Fixture::new();
    let source="<?php $options=['private_key_type'=>OPENSSL_KEYTYPE_RSA]; $key=openssl_pkey_new(options:$options); openssl_sign(private_key:$key,data:'LEAKME',signature:$sig);";
    let native = f.native(source);
    let expression = "openssl_sign(private_key:$key,data:'LEAKME',signature:$sig)";
    let start = source.find(expression).unwrap();
    let o = focal(&native, start, start + expression.len());
    assert_eq!(o.len(), 1);
    assert_eq!(algorithm_names(&native, o[0]), vec!["RSA"]);
    let input = ScannableFile::from_bytes("a.php".into(), source.as_bytes().to_vec());
    let outcome = TreeSitterDetector
        .detect_detailed(&input, &f.rules)
        .unwrap();
    for d in outcome.detections() {
        assert!(!d.evidence.r#match.contains("LEAKME"));
        assert!(!d.evidence.snippet_preview.contains("LEAKME"));
        assert!(!serde_json::to_string(&d.evidence.metadata)
            .unwrap()
            .contains("LEAKME"));
    }
    let (_, legacy) = f.document(source);
    assert!(
        !serde_json::to_string(&legacy).unwrap().contains("LEAKME"),
        "{legacy}"
    );
    assert!(!serde_json::to_string(&native).unwrap().contains("LEAKME"));
    for source in [
        "<?php openssl_sign('RSA',$sig,$imported);",
        "<?php $key=openssl_pkey_get_private($pem); openssl_sign($m,$sig,$key);",
        "<?php $key=openssl_pkey_new(['private_key_type'=>OPENSSL_KEYTYPE_RSA]); function test($key){openssl_sign($m,$sig,$key);}",
    ]{
        let native=f.native(source);let o=native["observations"].as_array().unwrap().iter().find(|o|{let a=o["target"]["byte_start"].as_u64().unwrap() as usize;source[a..].starts_with("openssl_sign")}).unwrap();assert!(algorithm_names(&native,o).is_empty(),"{source}: {native}");
    }
}
#[test]
fn php_alias_depth_and_unknown_writes_do_not_reuse_an_older_key() {
    let f = Fixture::new();
    for aliases in [7, 8, 9] {
        let mut source = "<?php [$sk,$pk]=\\PQCrypto\\MLKem768::generateKeypair();".to_owned();
        for i in 1..=aliases {
            source.push_str(&format!(
                "$k{i}={};",
                if i == 1 {
                    "$pk".into()
                } else {
                    format!("$k{}", i - 1)
                }
            ));
        }
        let expression = format!("$k{aliases}->encapsulate()");
        let start = source.len();
        source.push_str(&expression);
        source.push(';');
        let o = TreeSitterDetector
            .detect_observed(
                &ScannableFile::from_bytes("a.php".into(), source.as_bytes().to_vec()),
                &f.rules,
            )
            .unwrap();
        let native = f.native(&source);
        if aliases <= 8 {
            assert_eq!(o.status(), DetectorStatus::Complete, "{aliases}");
            let found = focal(&native, start, start + expression.len());
            assert_eq!(found.len(), 1, "{aliases}: {native}");
            assert_eq!(algorithm_names(&native, found[0]), vec!["ML-KEM-768"]);
        } else {
            assert_eq!(o.status(), DetectorStatus::Truncated);
            assert!(focal(&native, start, start + expression.len()).is_empty());
        }
    }
    for source in [
        "<?php [$sk,$pk]=\\PQCrypto\\MLKem768::generateKeypair(); opaque($pk); $pk->encapsulate();",
        "<?php if($condition){[$sk,$pk]=\\PQCrypto\\MLKem768::generateKeypair();} $pk->encapsulate();",
        "<?php [$sk,$pk]=\\PQCrypto\\MLKem768::generateKeypair(); function test($pk){$pk->encapsulate();}",
    ]{let native=f.native(source);let expression="$pk->encapsulate()";let start=source.find(expression).unwrap();assert!(focal(&native,start,start+expression.len()).is_empty(),"{native}");}
}

#[test]
fn php_key_imports_parameters_nested_calls_and_dynamic_names_are_bounded() {
    let f = Fixture::new();
    for (source, expression, name) in [
        (
            r"<?php function f(\PQCrypto\MLDSA65\SigningKey $key){$key->sign($m);}",
            "$key->sign($m)",
            "ML-DSA-65",
        ),
        (
            r"<?php function f(\PQCrypto\MLDSA65\VerifyingKey $key){$key->verify($m,$s);}",
            "$key->verify($m,$s)",
            "ML-DSA-65",
        ),
        (
            r"<?php function f(\ParagonIE\PQCrypto\MLDSA65\VerificationKey $key){$key->verify($m,$s);}",
            "$key->verify($m,$s)",
            "ML-DSA-65",
        ),
        (
            r"<?php $key=\PQCrypto\MLDSA44\SigningKey::fromBytes('LEAKME'); $key->sign($m);",
            "$key->sign($m)",
            "ML-DSA-44",
        ),
        (
            r"<?php \PQCrypto\MLKem768::generateKeypair()[0]->decapsulate($c);",
            r"\PQCrypto\MLKem768::generateKeypair()[0]->decapsulate($c)",
            "ML-KEM-768",
        ),
    ] {
        let native = f.native(source);
        let start = source.find(expression).unwrap();
        let o = focal(&native, start, start + expression.len());
        assert_eq!(o.len(), 1, "{source}: {native}");
        assert_eq!(algorithm_names(&native, o[0]), vec![name]);
        let (_, legacy) = f.document(source);
        assert!(!serde_json::to_string(&legacy).unwrap().contains("LEAKME"));
    }
    for source in [
        r"<?php class_alias($external,'PQCrypto\MLKem768'); \PQCrypto\MLKem768::generateKeypair();",
        r"<?php use function class_alias as ca; ca($external,'PQCrypto\MLKem768'); \PQCrypto\MLKem768::generateKeypair();",
        r"<?php eval($generated); \PQCrypto\MLKem768::generateKeypair();",
        r"<?php namespace PQCrypto; class MLKem768 {} namespace A; use PQCrypto\MLKem768; MLKem768::generateKeypair();",
        r"<?php namespace A; MLKem768::generateKeypair(); use PQCrypto\MLKem768;",
        r"<?php namespace A; use PQCrypto\MLKem768; namespace A; MLKem768::generateKeypair();",
        r"<?php function f(\PQCrypto\MLDSA65\VerificationKey $key){$key->verify($m,$s);}",
    ] {
        let native = f.native(source);
        assert!(
            native["observations"].as_array().unwrap().is_empty(),
            "{source}: {native}"
        );
    }
}

#[test]
fn php_sensitive_arguments_protect_all_detector_context_and_wide_source() {
    let f = Fixture::new();
    for canary in [
        "s",
        "SECRET078_WITH_LONG_KEY_MATERIAL_abcdefghijklmnopqrstuvwxyz0123456789",
    ] {
        let source = format!(
            r"<?php /* 日本語 */ $key=\PQCrypto\MLDSA44\SigningKey::fromBytes('{canary}'); $key->sign('public');"
        );
        for wide in [false, true] {
            let bytes = if wide {
                let mut b = vec![0xff, 0xfe];
                for c in source.encode_utf16() {
                    b.extend(c.to_le_bytes());
                }
                b
            } else {
                source.as_bytes().to_vec()
            };
            std::fs::write(f.root.join("input/case.php"), bytes).unwrap();
            let doc = scan_repository_native(
                &ScanConfig {
                    root: f.root.join("input"),
                    threads: 1,
                    ..Default::default()
                },
                &f.rules,
                &pqc_scan_detectors::default_detectors(),
                &NativeOptions::new(Some("v05/php")).unwrap(),
            )
            .unwrap();
            let legacy = serde_json::to_value(doc.legacy()).unwrap();
            let native = serde_json::to_value(doc.native().unwrap()).unwrap();
            // A one-character secret cannot be searched globally (it occurs in
            // schema keys); inspect actual source-bearing output fields instead.
            assert!(!serde_json::to_string(&legacy)
                .unwrap()
                .contains(&format!("'{canary}'")));
            assert!(!serde_json::to_string(&native)
                .unwrap()
                .contains(&format!("'{canary}'")));
            if canary.len() > 1 {
                assert!(!serde_json::to_string(&legacy).unwrap().contains(canary));
                assert!(!serde_json::to_string(&native).unwrap().contains(canary));
            }
            for finding in legacy["findings"].as_array().unwrap() {
                for line in finding["source_snippet"]["lines"].as_array().unwrap() {
                    assert_eq!(line["text"], "[masked-sensitive-file-content]");
                }
            }
        }
    }
}

#[test]
fn php_unknown_writes_and_binding_cap_cannot_resurrect_prior_types() {
    let f = Fixture::new();
    for write in [
        "foreach($unknown as $pk){}",
        "foreach($unknown as [$pk,$x]){}",
        "try{}catch(Exception $pk){}",
        "$object->mutate($pk);",
        "$pk->mutate();",
        "unset($pk[0]);",
        "opaque($pk[0]);",
    ] {
        let source = format!(
            r"<?php [$sk,$pk]=\PQCrypto\MLKem768::generateKeypair(); {write} $pk->encapsulate();"
        );
        let native = f.native(&source);
        let expr = "$pk->encapsulate()";
        let start = source.find(expr).unwrap();
        assert!(
            focal(&native, start, start + expr.len()).is_empty(),
            "{write}: {native}"
        );
    }
    let source="<?php $options=['private_key_type'=>OPENSSL_KEYTYPE_RSA]; $options['private_key_type']=OPENSSL_KEYTYPE_EC; $key=openssl_pkey_new($options); openssl_sign($m,$sig,$key);";
    let native = f.native(source);
    let expr = "openssl_sign($m,$sig,$key)";
    let start = source.find(expr).unwrap();
    let o = focal(&native, start, start + expr.len());
    assert_eq!(o.len(), 1);
    assert!(algorithm_names(&native, o[0]).is_empty());
    for count in [63, 64, 65] {
        let source = format!(
            "<?php {} $key=openssl_pkey_new($options);",
            "$options=['private_key_type'=>OPENSSL_KEYTYPE_RSA];".repeat(count)
        );
        let input = ScannableFile::from_bytes("a.php".into(), source.as_bytes().to_vec());
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
        let expr = "openssl_pkey_new($options)";
        let start = source.find(expr).unwrap();
        let o = focal(&native, start, start + expr.len());
        if count <= 64 {
            assert_eq!(algorithm_names(&native, o[0]), vec!["RSA"]);
        } else {
            // Retain the known API request without reviving an overflowed key type.
            assert_eq!(o.len(), 1);
            assert!(algorithm_names(&native, o[0]).is_empty());
        }
    }
}

#[test]
fn php_reference_only_imports_and_class_constants_are_not_runtime_operations() {
    let f = Fixture::new();
    let source = r"<?php use phpseclib3\Crypt\RSA; use phpseclib3\Crypt\EC; use phpseclib3\Crypt\DH; use PQCrypto\SLHDSA; $a=\phpseclib3\Crypt\RSA::class; $b=\PQCrypto\SLHDSA::class;";
    let native = f.native(source);
    let observations = native["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 6);
    for (expression, name) in [
        (r"phpseclib3\Crypt\RSA", "RSA"),
        (r"phpseclib3\Crypt\EC", "EC"),
        (r"phpseclib3\Crypt\DH", "DH"),
        (r"PQCrypto\SLHDSA", "SLH-DSA"),
        (r"\phpseclib3\Crypt\RSA::class", "RSA"),
        (r"\PQCrypto\SLHDSA::class", "SLH-DSA"),
    ] {
        let start = source.find(expression).unwrap();
        let o = focal(&native, start, start + expression.len());
        assert_eq!(o.len(), 1, "{expression}: {native}");
        assert_eq!(o[0]["usage"], "referenced");
        assert_eq!(algorithm_names(&native, o[0]), vec![name]);
    }
}

#[test]
fn php_local_binding_limit_retains_unaffected_api_requests_and_masking() {
    let f = Fixture::new();
    let source = format!(
        "<?php function noisy(){{ {} }} function signing(){{ $key=openssl_pkey_new(['private_key_type'=>OPENSSL_KEYTYPE_RSA]); openssl_sign('PRIVATE084_PHP_BINDING_CANARY',$sig,$key); }}",
        "$noise=1;".repeat(65)
    );
    let input = ScannableFile::from_bytes("case.php".into(), source.as_bytes().to_vec());
    let outcome = TreeSitterDetector
        .detect_observed(&input, &f.rules)
        .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Truncated);
    let (native, legacy) = f.document(&source);
    let expression = "openssl_sign('PRIVATE084_PHP_BINDING_CANARY',$sig,$key)";
    let start = source.find(expression).unwrap();
    let observations = focal(&native, start, start + expression.len());
    assert_eq!(
        observations.len(),
        1,
        "an unrelated local binding cap erased an explicit API request"
    );
    assert_eq!(algorithm_names(&native, observations[0]), vec!["RSA"]);
    assert_eq!(observations[0]["assurance"], "syntactic");
    assert_eq!(observations[0]["usage"], "referenced");
    assert!(!native.to_string().contains("PRIVATE084_PHP_BINDING_CANARY"));
    assert!(!legacy.to_string().contains("PRIVATE084_PHP_BINDING_CANARY"));
}

#[test]
fn php_local_binding_limit_keeps_overflowed_values_unknown_and_shadows_closed() {
    let f = Fixture::new();
    let source = format!(
        "<?php function limited(){{ {} $key=openssl_pkey_new($options); openssl_sign($message,$signature,$key); }}",
        "$options=['private_key_type'=>OPENSSL_KEYTYPE_RSA];".repeat(65)
    );
    let input = ScannableFile::from_bytes("case.php".into(), source.as_bytes().to_vec());
    assert_eq!(
        TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap()
            .status(),
        DetectorStatus::Truncated
    );
    let native = f.native(&source);
    for expression in [
        "openssl_pkey_new($options)",
        "openssl_sign($message,$signature,$key)",
    ] {
        let start = source.find(expression).unwrap();
        let observations = focal(&native, start, start + expression.len());
        assert_eq!(
            observations.len(),
            1,
            "explicit API reference must survive without a guessed key type"
        );
        assert!(algorithm_names(&native, observations[0]).is_empty());
    }
    let shadowed = format!(
        "<?php namespace Project; function openssl_sign($a,$b,$c){{}} function noisy(){{ {} openssl_sign($m,$s,$k); }}",
        "$noise=1;".repeat(65)
    );
    let native = f.native(&shadowed);
    assert!(
        native["observations"].as_array().unwrap().is_empty(),
        "a local namesake must remain suppressed"
    );
}

#[test]
fn php_truncation_still_masks_source_context_for_other_detectors() {
    let f = Fixture::new();
    let source=format!("<?php $key=\\PQCrypto\\MLDSA44\\SigningKey::fromBytes('CANARY078_SECRET_A'); // RSA_new\n{}", "$a=1;\n".repeat(65));
    std::fs::write(f.root.join("input/case.php"), source).unwrap();
    let doc = scan_repository_native(
        &ScanConfig {
            root: f.root.join("input"),
            threads: 1,
            ..Default::default()
        },
        &f.rules,
        &pqc_scan_detectors::default_detectors(),
        &NativeOptions::new(Some("v05/php")).unwrap(),
    )
    .unwrap();
    assert_eq!(doc.coverage().status().as_str(), "partial");
    assert!(!serde_json::to_string(doc.legacy())
        .unwrap()
        .contains("CANARY078_SECRET_A"));
    assert!(doc
        .legacy()
        .findings
        .iter()
        .any(|f| f.rule_id == "API_OPENSSL_RSA_NEW"));
    let input = ScannableFile::from_bytes(
        "a.php".into(),
        b"<?php openssl_sign('CANARY078_SECRET_A',$s,$k);".to_vec(),
    );
    let outcome = TreeSitterDetector::with_budget(pqc_scan_detectors::DetectorBudget {
        max_ast_nodes: 0,
        ..Default::default()
    })
    .detect_observed(&input, &f.rules)
    .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Truncated);
    assert!(outcome.sensitive_source_context());
}

#[test]
fn php_dynamic_option_keys_and_spreads_cannot_preserve_a_known_type() {
    let f = Fixture::new();
    for tail in ["...$dynamic", "$dynamic=>OPENSSL_KEYTYPE_EC"] {
        let source=format!("<?php $key=openssl_pkey_new(['private_key_type'=>OPENSSL_KEYTYPE_RSA,{tail}]); openssl_sign($m,$sig,$key);");
        let native = f.native(&source);
        for expression in [
            format!("openssl_pkey_new(['private_key_type'=>OPENSSL_KEYTYPE_RSA,{tail}])"),
            "openssl_sign($m,$sig,$key)".into(),
        ] {
            let start = source.find(&expression).unwrap();
            let o = focal(&native, start, start + expression.len());
            assert_eq!(o.len(), 1);
            assert!(
                algorithm_names(&native, o[0]).is_empty(),
                "{source}: {native}"
            );
        }
    }
}

#[test]
fn php_secret_that_looks_like_a_legacy_algorithm_is_not_exported_or_promoted() {
    let f = Fixture::new();
    for canary in ["RS256", "TLS_RSA_WITH_CANARY078"] {
        for count in [0, 65] {
            let source = format!(
                "<?php $key=\\PQCrypto\\MLDSA44\\SigningKey::fromBytes('{canary}'); {}",
                "$a=1;".repeat(count)
            );
            std::fs::write(f.root.join("input/case.php"), source).unwrap();
            let doc = scan_repository_native(
                &ScanConfig {
                    root: f.root.join("input"),
                    threads: 1,
                    ..Default::default()
                },
                &f.rules,
                &pqc_scan_detectors::default_detectors(),
                &NativeOptions::new(Some("v05/php")).unwrap(),
            )
            .unwrap();
            for finding in &doc.legacy().findings {
                assert_eq!(finding.evidence.r#match, "[masked-sensitive-content]");
                assert!(finding
                    .evidence
                    .metadata
                    .values()
                    .all(|v| v == "[masked-sensitive-content]"));
            }
            let native = serde_json::to_value(doc.native().unwrap()).unwrap();
            // RS256 is also part of preserved public rule IDs (JWT_RS256).
            // Assert algorithm payloads, not an unrelated identifier substring.
            assert!(native["assets"]
                .as_array()
                .unwrap()
                .iter()
                .all(|a| a["kind"] != "algorithm" || a["attributes"]["identifier"] != canary));
            if canary != "RS256" {
                assert!(!serde_json::to_string(&native).unwrap().contains(canary));
            }
            if count == 0 {
                assert!(native["assets"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|a| a["attributes"]["identifier"] == "ML-DSA-44"));
            }
        }
    }
}

#[test]
fn php_wrapper_projects_remain_distinct_in_final_outputs_without_arguments() {
    let f = Fixture::new();
    let source = r#"<?php $a=new \OQS_KeyEncapsulation('PRIVATE079_OPAQUE_CANARY'); $b=new \OQS\OQSKem('PRIVATE079_OPAQUE_CANARY');"#;
    let (native, legacy) = f.document(source);
    let expected = [
        (
            "OQS_KeyEncapsulation",
            "Muzosh oqsphp wrapper reference",
            "library_ref_php_oqsphp_native",
        ),
        (
            "OQS\\OQSKem",
            "mir-evgenii/liboqs-php FFI wrapper reference",
            "library_ref_php_liboqs_ffi",
        ),
    ];
    let mut markers = std::collections::BTreeSet::new();
    for (name, label, marker) in expected {
        let pos = source.find(name).unwrap();
        let finding = legacy["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["evidence"]["match"] == label)
            .unwrap();
        let association = native["legacy_associations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["legacy_id"] == finding["finding_id"])
            .unwrap();
        assert_eq!(association["observation_ids"].as_array().unwrap().len(), 1);
        let obs = native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["id"] == association["observation_ids"][0])
            .unwrap();
        assert!(obs["target"]["byte_start"].as_u64().unwrap() <= pos as u64);
        assert_eq!(obs["evidence"]["marker_kind"], marker);
        markers.insert(marker);
        assert!(algorithm_names(&native, obs).is_empty());
        assert!(legacy["cbom"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["algorithm"] == label));
    }
    assert_eq!(markers.len(), 2);
    assert!(!native.to_string().contains("PRIVATE079_OPAQUE_CANARY"));
    assert!(!legacy.to_string().contains("PRIVATE079_OPAQUE_CANARY"));
}
#[test]
fn php_non_crypto_receiver_probes_do_not_change_inspection_status() {
    let f = Fixture::new();
    for source in [
        "<?php function show($view) { $view=first(); $view=second(); $view->render(); }",
        "<?php function show($view,$choose) { if($choose) {$view=other();} $view->render(); }",
    ] {
        let input = ScannableFile::from_bytes("case.php".into(), source.as_bytes().to_vec());
        let outcome = TreeSitterDetector
            .detect_observed(&input, &f.rules)
            .unwrap();
        assert_eq!(outcome.status(), DetectorStatus::Complete, "{source}");
        assert!(f.native(source)["observations"]
            .as_array()
            .unwrap()
            .is_empty());
    }
    let source = "<?php function show($key) {$key=first();$key=second();$key->sign($data);}";
    let input = ScannableFile::from_bytes("case.php".into(), source.as_bytes().to_vec());
    let outcome = TreeSitterDetector
        .detect_observed(&input, &f.rules)
        .unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Partial);
    assert!(f.native(source)["observations"]
        .as_array()
        .unwrap()
        .is_empty());
}

#[test]
fn php_xml_receiver_reassignment_restores_identity_after_earlier_unknown_calls() {
    let f = Fixture::new();
    let source = "<?php use RobRichards\\XMLSecLibs\\XMLSecurityDSig as DS; function f($ref,$key) {$x=new DS(); consume($x); consume2($x); consume3($x); $x=new DS(); $x->addReference($ref); $x->sign($key); $x=opaque();}";
    let native = f.native(source);
    let start = source.find("$x->sign").unwrap();
    assert_eq!(
        native["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|o| o["rule_id"] == "PHP_CRYPTO_API" && o["target"]["byte_start"] == start)
            .count(),
        1
    );
    for source in [
        "<?php use RobRichards\\XMLSecLibs\\XMLSecurityDSig as DS; function f($key) {$x=new DS();consume($x);$x->sign($key);}",
        "<?php use RobRichards\\XMLSecLibs\\XMLSecurityDSig as DS; function f($flag,$key) {if($flag){$x=new DS();}$x->sign($key);}",
    ] {
        let start = source.find("$x->sign").unwrap();
        assert!(!f.native(source)["observations"].as_array().unwrap().iter().any(|o|o["target"]["byte_start"]==start));
    }
}
