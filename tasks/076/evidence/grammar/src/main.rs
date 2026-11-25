use std::{fs, path::Path};
use tree_sitter::{Language, Parser};
fn main() {
    if std::env::args().any(|a| a == "--corpus") {
        corpus();
        return;
    }
    let cases = [
        ("php-mixed", "php", "<p>日本語</p><?php use PQCrypto\\MLKem768 as K; [$sk,$pk]=K::generateKeypair(); $pk->encapsulate(); ?>", false),
        ("php-function-alias", "php", "<?php namespace App; use function sodium_crypto_kem_enc as enc; enc($pk);", false),
        ("php-broken", "php", "<?php $k = sodium_crypto_kem_keypair( ;", true),
        ("csharp-file-scoped", "csharp", "global using K = System.Security.Cryptography.MLKem; namespace App; class C { void M() { using var k = K.GenerateKey(System.Security.Cryptography.MLKemAlgorithm.MLKem768); k.Encapsulate(out var ct, out var ss); } }", false),
        ("csharp-ordinary", "csharp", "using System.Security.Cryptography; namespace App { class C { void M() { var k = RSA.Create(); k.SignData(data, hash, pad); } } }", false),
        ("csharp-broken", "csharp", "class C { void M( {", true),
        ("c-evp", "c", "#include <openssl/evp.h>\nvoid f(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_from_name(NULL, \"ML-KEM-768\", NULL); EVP_PKEY_keygen_init(ctx); }", false),
        ("c-conditional", "c", "#if CUSTOM\nOQS_KEM *k;\n#else\nint k;\n#endif\n", false),
        ("c-broken", "c", "void f( {", true),
        ("cpp-oqs", "cpp", "#include <oqs_cpp.hpp>\nnamespace App { void f() { oqs::KeyEncapsulation k(\"ML-KEM-768\"); auto pk = k.generate_keypair(); auto [ct,ss] = k.encap_secret(pk); } }", false),
        ("cpp-broken", "cpp", "namespace { void f( {", true),
        ("header-common-c", "c", "void f(void) { OQS_KEM *k = OQS_KEM_new(OQS_KEM_alg_ml_kem_768); OQS_KEM_keypair(k, pk, sk); }", false),
        ("header-common-cpp", "cpp", "void f(void) { OQS_KEM *k = OQS_KEM_new(OQS_KEM_alg_ml_kem_768); OQS_KEM_keypair(k, pk, sk); }", false),
    ];
    let out = Path::new("tasks/076/evidence/grammar/asts");
    fs::create_dir_all(out).unwrap();
    for (id, language, source, errors) in cases {
        let grammar: Language = match language {
            "php" => tree_sitter_php::LANGUAGE_PHP.into(),
            "csharp" => tree_sitter_c_sharp::LANGUAGE.into(),
            "c" => tree_sitter_c::LANGUAGE.into(),
            "cpp" => tree_sitter_cpp::LANGUAGE.into(),
            _ => unreachable!(),
        };
        assert!(
            (tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION..=tree_sitter::LANGUAGE_VERSION)
                .contains(&grammar.abi_version())
        );
        let mut parser = Parser::new();
        parser.set_language(&grammar).unwrap();
        let tree = parser.parse(source, None).unwrap();
        fs::write(out.join(format!("{id}.txt")), tree.root_node().to_sexp()).unwrap();
        assert_eq!(
            tree.root_node().has_error(),
            errors,
            "{id}: {}",
            tree.root_node().to_sexp()
        );
        println!(
            "{}",
            serde_json::json!({"id":id,"language":language,"grammar_abi":grammar.abi_version(),"bytes":source.len(),"has_error":tree.root_node().has_error(),"range":[tree.root_node().start_byte(),tree.root_node().end_byte()]})
        );
    }
}

// Only syntax is observed here. Subject matching/inventory expectations remain unread by a detector.
fn corpus() {
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read("tasks/076/evidence/CORPUS-CASES.json").unwrap()).unwrap();
    let mut failures = Vec::new();
    let cases = value["cases"].as_array().unwrap();
    for case in cases {
        let language = case["language"].as_str().unwrap();
        let grammar: Language = match language {
            "php" => tree_sitter_php::LANGUAGE_PHP.into(),
            "csharp" => tree_sitter_c_sharp::LANGUAGE.into(),
            "c" => tree_sitter_c::LANGUAGE.into(),
            "cpp" => tree_sitter_cpp::LANGUAGE.into(),
            _ => unreachable!(),
        };
        let source = fs::read(case["path"].as_str().unwrap()).unwrap();
        let mut parser = Parser::new();
        parser.set_language(&grammar).unwrap();
        let tree = parser.parse(&source, None).unwrap();
        if tree.root_node().has_error() {
            failures.push(case["id"].clone());
        }
    }
    println!(
        "{}",
        serde_json::json!({"total":cases.len(),"parse_errors":failures})
    );
    assert!(failures.is_empty());
}
