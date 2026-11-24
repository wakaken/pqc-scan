//! Source-backed normalization and call-binding controls for remaining OSS unknowns.
use pqc_scan_core::{
    migration::{assess_primitive, InputFormat, SavedEvidence},
    scan_repository_native, NativeOptions, ScanConfig,
};
use pqc_scan_detectors::default_detectors;
use pqc_scan_rules::RuleSet;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

fn scan(ext: &str, text: &str) -> Value {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "pqc-unknowns-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join(format!("sample.{ext}")), text).unwrap();
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let result = scan_repository_native(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
        &NativeOptions::new(Some("unknowns-test")).unwrap(),
    )
    .unwrap();
    let value = serde_json::to_value(result.native().unwrap()).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    value
}
fn algorithms(scan: &Value, rule: &str) -> Vec<String> {
    let assets: BTreeMap<_, _> = scan["assets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| (a["id"].as_str().unwrap(), a))
        .collect();
    let mut names = vec![];
    for o in scan["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["rule_id"] == rule)
    {
        for id in o["asset_ids"].as_array().unwrap() {
            let a = assets[id.as_str().unwrap()];
            if a["kind"] == "algorithm" {
                names.push(a["attributes"]["identifier"].as_str().unwrap().to_owned());
            }
        }
    }
    names.sort();
    names.dedup();
    names
}
#[test]
fn explicit_jose_families_and_module_tokens_survive_native_projection() {
    let py = scan("py", "from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey\nfrom cryptography.hazmat.primitives.asymmetric import rsa, ed25519\na = rsa.generate_private_key(public_exponent=65537, key_size=2048)\nb = ed25519.Ed25519PrivateKey.generate()\nalgorithm = 'EdDSA'\n");
    assert_eq!(algorithms(&py, "API_PY_RSA_MODULE"), ["RSA"]);
    assert_eq!(algorithms(&py, "TS_PY_RSA_USAGE"), ["RSA"]);
    assert_eq!(algorithms(&py, "TS_CLASSICAL_EDDSA_API"), ["Ed25519"]);
    assert_eq!(algorithms(&py, "TS_JWT_EDDSA_ALG"), ["EC"]);
    let go = scan(
        "go",
        "package example\nimport \"crypto/rsa\"\nvar algorithms = []string{\"ES384\", \"ES512\"}\n",
    );
    for rule in ["TS_GO_RSA_USAGE", "API_GO_CRYPTO_RSA", "GO_TLS_RSA_IMPORT"] {
        assert_eq!(algorithms(&go, rule), ["RSA"], "{rule}");
    }
    assert_eq!(algorithms(&go, "TS_JWT_ES_ALG"), ["ECDSA"]);
    let saved =
        SavedEvidence::from_bytes(&serde_json::to_vec(&go).unwrap(), InputFormat::InventoryV1)
            .unwrap();
    assert!(saved
        .targets()
        .iter()
        .all(|t| t.technical.algorithm.identifier != "unknown"));
}
#[test]
fn jose_normalization_uses_existing_catalog_without_inventing_an_eddsa_curve() {
    use pqc_scan_core::observation::AlgorithmSpec;
    for (name, bits, oid) in [
        ("ES384", "384", "1.3.132.0.34"),
        ("ES512", "521", "1.3.132.0.35"),
    ] {
        let spec = serde_json::to_value(AlgorithmSpec::recognized(name).unwrap()).unwrap();
        assert_eq!(spec["identifier"], "ECDSA");
        let parameters: BTreeMap<String, String> =
            serde_json::from_value(spec["parameters"].clone()).unwrap();
        assert_eq!(parameters["key_bits"], bits);
        assert_eq!(parameters["named_curve_oid"], oid);
        let a = assess_primitive("ECDSA", &parameters);
        assert!(a.parameters_known);
        assert_eq!(a.construction, "classical_public_key");
        assert_eq!(serde_json::to_value(a.purpose).unwrap(), "signature");
        let mut wrong = parameters;
        wrong.insert("key_bits".into(), "256".into());
        let a = assess_primitive("ECDSA", &wrong);
        assert!(!a.parameters_known);
        assert!(a
            .classical_concerns
            .iter()
            .any(|s| s == "contradictory_parameters"));
        assert_eq!(
            assess_primitive(name, &BTreeMap::new()).identifier,
            "unknown",
            "held-input catalog meaning remains unchanged"
        );
    }
    let spec = serde_json::to_value(AlgorithmSpec::recognized("EdDSA").unwrap()).unwrap();
    assert_eq!(spec["identifier"], "EC");
    assert_eq!(spec["purpose"], "reference");
    assert_eq!(spec["parameters"], serde_json::json!({}));
    let family = assess_primitive("EC", &BTreeMap::new());
    assert!(!family.parameters_known);
    assert_eq!(serde_json::to_value(family.quantum).unwrap(), "vulnerable");
    assert_eq!(serde_json::to_value(family.purpose).unwrap(), "unknown");
    assert_eq!(
        assess_primitive("EdDSA", &BTreeMap::new()).identifier,
        "unknown"
    );
}
#[test]
fn jsonwebtoken_sign_is_bound_to_its_own_explicit_options_not_neighboring_names() {
    for (prefix, callee) in [
        ("import * as jwt from 'jsonwebtoken';", "jwt.sign"),
        ("import jwt from 'jsonwebtoken';", "jwt.sign"),
        ("const jwt = require('jsonwebtoken');", "jwt.sign"),
        ("import {sign as issue} from 'jsonwebtoken';", "issue"),
        (
            "const jwt = require('jsonwebtoken'); const other = jwt;",
            "other.sign",
        ),
        (
            "const jwt = require('jsonwebtoken'); const issue = jwt.sign;",
            "issue",
        ),
    ] {
        let text = format!("{prefix}\nconst signed = {callee}({{payload: 'CANARY_DO_NOT_EXPORT'}}, secret, {{algorithm:'RS256', header:{{alg:'RS256'}}}});");
        let s = scan("ts", &text);
        assert_eq!(algorithms(&s, "TS_NODE_JOSE_USAGE"), ["RS256"], "{prefix}");
        assert!(!s.to_string().contains("CANARY_DO_NOT_EXPORT"));
        for o in s["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|o| o["rule_id"] == "TS_NODE_JOSE_USAGE")
        {
            assert_eq!(o["usage"], "referenced");
            assert_eq!(o["assurance"], "syntactic");
        }
    }
    for text in [
        "const jwt=require('jsonwebtoken'); const other=jwt; other['sign']=replacement; jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); jwt['sign']=replacement; jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); const other=jwt; other.sign=replacement; jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); const other=jwt; Object.assign(other, replacement); jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); const box={value:jwt}; box.value.sign=replacement; jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); const box=[jwt]; box[0].sign=replacement; jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); let other; other=jwt; other.sign=replacement; jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); mutate(jwt); jwt.sign({},key,{algorithm:'RS256'});",
        "const jwt=require('jsonwebtoken'); delete jwt.sign; jwt.sign({},key,{algorithm:'RS256'});",
        "import * as jwt from 'mock'; jwt.sign({}, key, {algorithm:'RS256'});",
        "import * as jwt from 'jsonwebtoken'; function f(jwt) { jwt.sign({}, key, {algorithm:'RS256'}); }",
        "import * as jwt from 'jsonwebtoken'; jwt.sign = replacement; jwt.sign({}, key, {algorithm:'RS256'});",
        "import * as jwt from 'jsonwebtoken'; jwt.sign({algorithm:'RS256'}, key);",
        "import * as jwt from 'jsonwebtoken'; const nearby='RS256'; jwt.sign({}, key, dynamic);",
        "import * as jwt from 'jsonwebtoken'; jwt.sign({}, key, {algorithm:'RS256', ...dynamic});",
        "import * as jwt from 'jsonwebtoken'; jwt.sign({}, key, {algorithm:'RS256', algorithm:'ES384'});",
        "import * as jwt from 'jsonwebtoken'; jwt.sign({}, key, {algorithm:'RS256', header:{alg:'ES384'}});",
        "import * as jwt from 'jsonwebtoken'; jwt.sign({}, key, {algorithm:'RS256', header:{...dynamic}});",
        "import * as jwt from 'jsonwebtoken'; jwt.sign({}, key, {algorithm:'RS256', [dynamic]:'ES384'});",
        "import * as jwt from 'jsonwebtoken'; jwt.verify(token, key, {algorithms:['RS256']});",
    ] { assert!(algorithms(&scan("ts", text), "TS_NODE_JOSE_USAGE").is_empty(), "{text}"); }
}
#[test]
fn ruby_optional_defaults_are_candidates_but_required_or_reassigned_parameters_are_not() {
    let text = "class Envelope\nclass << self\ndef default_cipher\nif enabled\n'aes-256-gcm'\nelse\n'aes-256-cbc'\nend\nend\nend\ndef self.key_len(cipher = default_cipher)\nOpenSSL::Cipher.new(cipher).key_len\nend\nend";
    let s = scan("rb", text);
    let saved =
        SavedEvidence::from_bytes(&serde_json::to_vec(&s).unwrap(), InputFormat::InventoryV1)
            .unwrap();
    assert_eq!(saved.targets().len(), 2);
    assert!(saved
        .targets()
        .iter()
        .all(|t| t.technical.algorithm.identifier == "AES-256"
            && t.observation.usage == "referenced"));
    for text in [
        "class A; def self.cipher; 'aes-256-gcm'; end; def self.f(cipher); OpenSSL::Cipher.new(cipher); end; end",
        "class A; def self.f(cipher = 'aes-256-gcm'); cipher = supplied; OpenSSL::Cipher.new(cipher); end; end",
        "class A; def self.f(cipher = supplied); OpenSSL::Cipher.new(cipher); end; end",
        "OpenSSL::Cipher.new('EdDSA')",
        "OpenSSL::Cipher.new('rsa')",
        "OpenSSL::Cipher.new('RSA')",
        "OpenSSL::Cipher.new('ML-KEM-768')",
        "OpenSSL::Cipher.new('ES512')",
        "class A; def default_cipher; 'aes-256-gcm'; end; def self.f(cipher = default_cipher); OpenSSL::Cipher.new(cipher); end; end",
    ] { assert!(algorithms(&scan("rb", text), "API_RUBY_OPENSSL_CIPHER").is_empty(), "{text}"); }
}
