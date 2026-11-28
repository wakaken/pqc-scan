use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
use pqc_scan_rules::RuleSet;
use std::{
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

fn scan(source: &str, extension: &str) -> pqc_scan_core::ScanDocument {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "pqc086-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join(format!("case.{extension}")), source).unwrap();
    let rules =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap();
    let doc = scan_repository_native(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &pqc_scan_detectors::default_detectors(),
        &NativeOptions::new(Some("synthetic/display086")).unwrap(),
    )
    .unwrap();
    let legacy = pqc_scan_core::scan_repository(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &pqc_scan_detectors::default_detectors(),
    )
    .unwrap();
    let native_matches: Vec<_> = doc
        .legacy()
        .findings
        .iter()
        .map(|f| {
            (
                &f.finding_id,
                &f.evidence.r#match,
                serde_json::to_value(&f.source_snippet).unwrap(),
            )
        })
        .collect();
    let legacy_matches: Vec<_> = legacy
        .findings
        .iter()
        .map(|f| {
            (
                &f.finding_id,
                &f.evidence.r#match,
                serde_json::to_value(&f.source_snippet).unwrap(),
            )
        })
        .collect();
    assert_eq!(
        serde_json::to_value(&doc.legacy().cbom).unwrap(),
        serde_json::to_value(&legacy.cbom).unwrap()
    );
    assert!(!serde_json::to_string(doc.native().unwrap())
        .unwrap()
        .contains("arguments omitted"));
    assert!(
        doc.legacy()
            .cbom
            .iter()
            .all(|row| !row.algorithm.contains("arguments omitted")),
        "Display-only API excerpts must not become CBOM algorithm identities"
    );
    assert_eq!(
        native_matches, legacy_matches,
        "default and inventory scans must show the same excerpts"
    );
    std::fs::remove_dir_all(root).unwrap();
    doc
}

#[test]
fn recognized_calls_show_original_arguments_in_sanitized_source_context() {
    for (ext, source, api) in [
        (
            "c",
            "void f(){secp256k1_ecdsa_sign(ctx,sig,hash,key,nonce,data);}",
            "secp256k1_ecdsa_sign",
        ),
        (
            "cpp",
            "void f(){EVP_PKEY_CTX_new_from_name(nullptr,\"RSA\",nullptr);}",
            "EVP_PKEY_CTX_new_from_name",
        ),
        (
            "cs",
            "using System.Security.Cryptography; class C { void M(){ RSA.Create(); } }",
            "Create",
        ),
        (
            "php",
            "<?php openssl_sign($data,$sig,$key);",
            "openssl_sign",
        ),
    ] {
        let doc = scan(source, ext);
        let found = doc
            .legacy()
            .findings
            .iter()
            .find(|f| {
                f.evidence
                    .r#match
                    .to_ascii_lowercase()
                    .contains(&api.to_ascii_lowercase())
            })
            .unwrap_or_else(|| panic!("{ext}: no readable API source excerpt"));
        assert!(!found.evidence.snippet_preview.contains("arguments omitted"));
        let snippet = found.source_snippet.as_ref().unwrap();
        assert!(snippet
            .lines
            .iter()
            .any(|l| l.highlighted && l.line == found.location.line && l.text.contains(api)));
        assert!(doc.native().unwrap().validate().is_ok());
    }
}

#[test]
fn public_excerpts_do_not_reintroduce_private_or_algorithm_looking_arguments() {
    for ext in ["c", "cpp", "h"] {
        let doc = scan(
            "void f(){OQS_KEM_decaps(k,\"PRIVATE086_OPAQUE_RS256\",data,more);}",
            ext,
        );
        assert!(!serde_json::to_string(doc.legacy())
            .unwrap()
            .contains("PRIVATE086"));
        assert!(!serde_json::to_string(doc.native().unwrap())
            .unwrap()
            .contains("PRIVATE086"));
    }
    for (ext, source) in [
        ("cs", "class C { void M(){System.Security.Cryptography.MLKem.ImportPrivateSeed(System.Security.Cryptography.MLKemAlgorithm.MLKem768,\"PRIVATE086_OPAQUE\");} }"),
        ("php", "<?php openssl_sign($data,$sig,'PRIVATE086_OPAQUE');"),
    ] {
        let doc = scan(source, ext);
        assert!(!serde_json::to_string(doc.legacy()).unwrap().contains("PRIVATE086"));
        assert!(!serde_json::to_string(doc.native().unwrap()).unwrap().contains("PRIVATE086"));
    }
}

#[test]
fn java_and_go_have_readable_normal_source_and_still_protect_real_private_blocks() {
    for (ext, source, token) in [
        ("java", "import java.security.KeyPairGenerator; class C {void f() throws Exception {KeyPairGenerator.getInstance(\"RSA\");}}", "KeyPairGenerator"),
        ("go", "package main\nimport \"crypto/rsa\"\nfunc f(){rsa.GenerateKey(nil,2048)}", "rsa"),
    ] {
        let doc = scan(source, ext);
        assert!(doc.legacy().findings.iter().filter_map(|f| f.source_snippet.as_ref()).flat_map(|s| &s.lines).any(|l| l.text.contains(token)), "{ext}: normal source hidden");
        let private = format!("{source}\n// -----BEGIN PRIVATE KEY-----\nPRIVATE086_BLOCK\n// -----END PRIVATE KEY-----\n");
        let credential = format!("{source}\n// password = \"PRIVATE086_CREDENTIAL\"\n");
        let protected = scan(&credential, ext);
        assert!(!serde_json::to_string(protected.legacy()).unwrap().contains("PRIVATE086_CREDENTIAL"));
        assert!(!serde_json::to_string(protected.native().unwrap()).unwrap().contains("PRIVATE086_CREDENTIAL"));
        let doc = scan(&private, ext);
        assert!(!serde_json::to_string(doc.legacy()).unwrap().contains("PRIVATE086_BLOCK"));
        assert!(!serde_json::to_string(doc.native().unwrap()).unwrap().contains("PRIVATE086_BLOCK"));
    }
}

#[test]
fn all_existing_seven_language_fixtures_retain_normal_readable_source() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tasks/076/evidence/old-seven");
    let mut total = 0;
    for entry in std::fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.file_name().unwrap() == "negative.js" {
            continue;
        }
        let source = std::fs::read_to_string(&path).unwrap();
        let doc = scan(&source, path.extension().unwrap().to_str().unwrap());
        assert!(!doc.legacy().findings.is_empty(), "{}", path.display());
        assert!(
            doc.legacy()
                .findings
                .iter()
                .filter_map(|f| f.source_snippet.as_ref())
                .flat_map(|s| &s.lines)
                .any(|l| !l.text.is_empty() && !l.text.starts_with("[masked-")),
            "{}",
            path.display()
        );
        total += 1;
    }
    assert_eq!(total, 7);
}

#[test]
fn remaining_recognized_factories_operations_and_php_case_variants_are_readable() {
    let mut cases: Vec<(String, &str, &str)> = vec![
        (r"<?php $key=\PQCrypto\MLDSA44\SigningKey::fromBytes('PRIVATE086_KEY'); $key?->sign('PUBLIC087_MESSAGE');".into(), "php", "sign"),
        ("<?php OPENSSL_SIGN($data,$sig,$key);".into(), "php", "openssl_sign"),
        ("<?php sodium_crypto_sign('PUBLIC087_SIGN',$key);".into(), "php", "sodium_crypto_sign"),
        ("<?php sodium_crypto_box('PUBLIC087_BOX',$nonce,$key);".into(), "php", "sodium_crypto_box"),
        ("<?php sodium_crypto_kem_enc($key);".into(), "php", "sodium_crypto_kem_enc"),
        ("<?php sodium_crypto_kem_dec('PUBLIC087_CIPHERTEXT',$key);".into(), "php", "sodium_crypto_kem_dec"),
        ("<?php sodium_crypto_kem_mlkem768_enc($key);".into(), "php", "sodium_crypto_kem_mlkem768_enc"),
        ("using System.Security.Cryptography; class C{void M(){var k=ECDiffieHellman.Create();k.DeriveKeyFromHmac(peer,HashAlgorithmName.SHA256,key);}}".into(), "cs", "DeriveKeyFromHmac"),
        ("using System.Security.Cryptography; class C{void M(){var k=new RSACryptoServiceProvider();}}".into(), "cs", "RSACryptoServiceProvider"),
        ("using Org.BouncyCastle.Crypto.Signers; class C{void M(){var k=new DsaSigner();}}".into(), "cs", "DsaSigner"),
    ];
    for method in [
        "ImportFromPem",
        "ImportFromEncryptedPem",
        "ImportSubjectPublicKeyInfo",
        "ImportPkcs8PrivateKey",
        "ImportEncryptedPkcs8PrivateKey",
        "ImportPrivateSeed",
        "ImportDecapsulationKey",
        "ImportEncapsulationKey",
    ] {
        cases.push((format!("using System.Security.Cryptography; class C{{void M(){{MLKem.{method}(MLKemAlgorithm.MLKem768,\"{}\");}}}}", if method.contains("Public") || method == "ImportEncapsulationKey" {"PUBLIC087_IMPORT"} else {"PRIVATE086_IMPORT"}), "cs", method));
    }
    for (source, ext, name) in cases {
        let doc = scan(&source, ext);
        assert!(
            doc.legacy().findings.iter().any(|f| f
                .evidence
                .r#match
                .to_ascii_lowercase()
                .contains(&name.to_ascii_lowercase())),
            "{name}: missing source excerpt"
        );
        assert!(!serde_json::to_string(doc.legacy())
            .unwrap()
            .contains("PRIVATE086"));
        assert!(!serde_json::to_string(doc.native().unwrap())
            .unwrap()
            .contains("PRIVATE086"));
    }
}
