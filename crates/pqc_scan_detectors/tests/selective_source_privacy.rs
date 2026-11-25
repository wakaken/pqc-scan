use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
use pqc_scan_rules::RuleSet;
use std::{
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

fn scan(source: &str, ext: &str, wide: bool) -> pqc_scan_core::ScanDocument {
    scan_budget(source, ext, wide, None)
}
fn scan_budget(
    source: &str,
    ext: &str,
    wide: bool,
    budget: Option<pqc_scan_detectors::DetectorBudget>,
) -> pqc_scan_core::ScanDocument {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "pqc087-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    let bytes = if wide {
        let mut b = vec![0xff, 0xfe];
        b.extend(source.encode_utf16().flat_map(u16::to_le_bytes));
        b
    } else {
        source.as_bytes().to_vec()
    };
    std::fs::write(root.join(format!("case.{ext}")), bytes).unwrap();
    let rules =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap();
    let config = ScanConfig {
        root: root.clone(),
        threads: 1,
        ..Default::default()
    };
    let mut detectors = pqc_scan_detectors::default_detectors();
    if let Some(budget) = budget {
        detectors[0] = std::sync::Arc::new(
            pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector::with_budget(budget),
        );
    }
    let doc = scan_repository_native(
        &config,
        &rules,
        &detectors,
        &NativeOptions::new(Some("synthetic/selective087")).unwrap(),
    )
    .unwrap();
    let legacy = pqc_scan_core::scan_repository(&config, &rules, &detectors).unwrap();
    assert_eq!(
        serde_json::to_value(&legacy.findings).unwrap(),
        serde_json::to_value(&doc.legacy().findings).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&legacy.cbom).unwrap(),
        serde_json::to_value(&doc.legacy().cbom).unwrap()
    );
    assert!(
        !doc.legacy().findings.is_empty(),
        "fixture has no findings: {source}"
    );
    assert!(!serde_json::to_string(doc.native().unwrap())
        .unwrap()
        .contains("SYNTH087_"));
    std::fs::remove_dir_all(root).unwrap();
    doc
}

#[test]
fn ordinary_arguments_and_neighbouring_code_are_readable() {
    for (ext,source,call) in [
        ("cpp","void f(){\n int before = 7;\n secp256k1_ecdsa_sign(ctx,sig,hash,key.data(),nonce,data);\n int after = 9;\n}","secp256k1_ecdsa_sign(ctx,sig,hash,key.data(),nonce,data)"),
        ("c","void f(){\n int before = 7;\n EVP_PKEY_CTX_new_from_name(NULL,\"RSA\",NULL);\n int after = 9;\n}","EVP_PKEY_CTX_new_from_name(NULL,\"RSA\",NULL)"),
        ("cs","using System.Security.Cryptography; class C {void M(){\n int before = 7;\n RSA.Create(2048);\n int after = 9;\n}}","RSA.Create(2048)"),
        ("php","<?php\n$before = 7;\nopenssl_sign('public message',$sig,$key,OPENSSL_ALGO_SHA256);\n$after = 9;","openssl_sign('public message',$sig,$key,OPENSSL_ALGO_SHA256)"),
    ] {
        for wide in [false,true] {
            let d=scan(source,ext,wide);
            let f=d.legacy().findings.iter().find(|f|f.rule_id.ends_with("CRYPTO_API")).unwrap();
            let snippet=f.source_snippet.as_ref().unwrap();
            let text=snippet.lines.iter().map(|l|l.text.as_str()).collect::<Vec<_>>().join("\n");
            assert!(text.contains(call),"{ext}: ordinary call missing: {text}");
            assert!(text.contains("before") && text.contains("after"),"{ext}: context missing");
            assert!(!text.contains("arguments omitted"));
            assert!(snippet.lines.iter().any(|l|l.highlighted && l.line==f.location.line));
        }
    }
}

#[test]
fn key_literals_and_credentials_are_masked_but_public_neighbours_survive() {
    for (ext,source,public) in [
        ("c","void f(){\n const char *password=\"SYNTH087_PASSWORD\"; int visible=17;\n EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,\"SYNTH087_KEY\",32);\n int after=9;\n}","EVP_PKEY_ED25519"),
        ("cs","using System.Security.Cryptography; class C{void M(){\n var privateKey = \"SYNTH087_KEY\"; int visible=17;\n MLKem.ImportPrivateSeed(MLKemAlgorithm.MLKem768,\"SYNTH087_SEED\");\n int after=9;\n}}","MLKemAlgorithm.MLKem768"),
        ("php","<?php\n$password=\"SYNTH087_PASSWORD\"; $visible=17;\nopenssl_sign('public message',$sig,'SYNTH087_KEY',OPENSSL_ALGO_SHA256);\n$after=9;","'public message'"),
    ] {
        for wide in [false,true] {
            let d=scan(source,ext,wide); let exported=serde_json::to_string(d.legacy()).unwrap();
            assert!(!exported.contains("SYNTH087_"),"{ext}: secret leaked");
            let text=d.legacy().findings.iter().filter_map(|f|f.source_snippet.as_ref()).flat_map(|s|&s.lines).map(|l|l.text.as_str()).collect::<Vec<_>>().join("\n");
            assert!(text.contains(public) && text.contains("visible=17") && text.contains("after=9"),"{ext}: benign context hidden: {text}");
            assert!(text.contains("[masked-"));
            assert!(!serde_json::to_string(d.native().unwrap()).unwrap().contains("SYNTH087_"));
        }
    }
}

#[test]
fn private_blocks_do_not_hide_unrelated_java_go_or_cpp_code() {
    for (ext,source,public) in [
        ("java","class C {void f(){java.security.KeyPairGenerator.getInstance(\"RSA\");}}\n// -----BEGIN PRIVATE KEY-----\n// SYNTH087_BLOCK\n// -----END PRIVATE KEY-----","getInstance(\"RSA\")"),
        ("go","package main\nimport \"crypto/rsa\"\nfunc f(){rsa.GenerateKey(nil,2048)}\n// -----BEGIN PRIVATE KEY-----\n// SYNTH087_BLOCK\n// -----END PRIVATE KEY-----","rsa.GenerateKey(nil,2048)"),
        ("cpp","void f(){RSA_new();}\n// -----BEGIN PRIVATE KEY-----\n// SYNTH087_BLOCK\n// -----END PRIVATE KEY-----","RSA_new()"),
    ] {
        let d=scan(source,ext,false); let text=serde_json::to_string(d.legacy()).unwrap();
        assert!(!text.contains("SYNTH087_BLOCK"));
        assert!(d.legacy().findings.iter().filter_map(|f|f.source_snippet.as_ref()).flat_map(|s|&s.lines).any(|l|l.text.contains(public)),"{ext}: public code hidden");
    }
}

#[test]
fn reordered_named_key_arguments_and_multiline_credentials_are_selective() {
    let source = "<?php\n$password=\"SYNTH087_FIRST\nSYNTH087_SECOND\"; $visible=17;\nopenssl_sign(private_key:'SYNTH087_KEY',signature:$sig,data:'public message');\n$after=9;";
    let d = scan(source, "php", false);
    let exported = serde_json::to_string(d.legacy()).unwrap();
    assert!(!exported.contains("SYNTH087_"));
    assert!(exported.contains("public message") && exported.contains("visible=17"));
}

#[test]
fn locally_initialized_key_bytes_are_protected_without_hiding_public_key_references() {
    let source="void f(){const char *key=\"SYNTH087_LOCAL_KEY\"; EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,key,32); int visible=17;} void other(){const char *key=\"public label\";RSA_new();}";
    let d = scan(source, "c", false);
    let exported = serde_json::to_string(d.legacy()).unwrap();
    assert!(!exported.contains("SYNTH087_LOCAL_KEY"));
    assert!(
        exported.contains("public label")
            && exported.contains("NULL,key,32")
            && exported.contains("visible=17")
    );
}

#[test]
fn verification_public_keys_and_key_file_paths_are_not_secret_values() {
    let source="<?php openssl_verify('public message',$sig,'PUBLIC087_VERIFICATION_KEY',OPENSSL_ALGO_SHA256); openssl_sign('public message',$sig,'file://keys/signing.pem');";
    let d = scan(source, "php", false);
    let text = d
        .legacy()
        .findings
        .iter()
        .filter_map(|f| f.source_snippet.as_ref())
        .flat_map(|s| &s.lines)
        .map(|l| l.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        text.contains("PUBLIC087_VERIFICATION_KEY") && text.contains("file://keys/signing.pem"),
        "{text}"
    );
}

#[test]
fn secret_alias_limit_keeps_unresolved_key_values_private() {
    let mut source = "void f(){const char *a0=\"SYNTH087_ALIAS_KEY\";".to_string();
    for i in 1..=10 {
        source.push_str(&format!("const char *a{i}=a{};", i - 1));
    }
    source.push_str("EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,a10,32);int visible=17;}");
    let d = scan(&source, "c", false);
    let text = serde_json::to_string(d.legacy()).unwrap();
    assert!(!text.contains("SYNTH087_ALIAS_KEY"));
}

#[test]
fn inline_private_block_preserves_code_after_it_and_long_secret_clipping_is_safe() {
    let source="void f(){const char *privateKey=\"-----BEGIN PRIVATE KEY-----SYNTH087_BLOCK-----END PRIVATE KEY-----\"; RSA_new(); int visible=17;}";
    let d = scan(source, "cpp", false);
    let exported = serde_json::to_string(d.legacy()).unwrap();
    assert!(!exported.contains("SYNTH087_BLOCK"));
    let snippet = d
        .legacy()
        .findings
        .iter()
        .filter_map(|f| f.source_snippet.as_ref())
        .flat_map(|s| &s.lines)
        .map(|l| l.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        snippet.contains("RSA_new()") && snippet.contains("visible=17"),
        "{snippet}"
    );
    let long = format!(
        "void f(){{const char *password=\"{}SYNTH087_TAIL\"; RSA_new();}}",
        "SYNTH087_MIDDLE".repeat(50)
    );
    let d = scan(&long, "c", false);
    let exported = serde_json::to_string(d.legacy()).unwrap();
    assert!(!exported.contains("SYNTH087_"));
}

#[test]
fn secret_roles_and_wrapped_local_values_remain_private() {
    for function in [
        "sodium_crypto_sign_publickey_from_secretkey",
        "sodium_crypto_sign_keypair_from_secretkey_and_publickey",
        "sodium_crypto_box_publickey_from_secretkey",
        "sodium_crypto_box_keypair_from_secretkey_and_publickey",
        "sodium_crypto_scalarmult",
        "sodium_crypto_scalarmult_base",
        "sodium_crypto_sign_publickey",
        "sodium_crypto_sign_secretkey",
        "sodium_crypto_box_secretkey",
        "sodium_crypto_kx_client_session_keys",
        "sodium_crypto_kem_mlkem768_secretkey",
    ] {
        let source = format!("<?php {function}('SYNTH087_ROLE_KEY',$public);");
        let d = scan(&source, "php", false);
        assert!(
            !serde_json::to_string(d.legacy())
                .unwrap()
                .contains("SYNTH087_ROLE_KEY"),
            "{function}"
        );
    }
    for (ext,source) in [
        ("cpp", "void f(){oqs::Signature signer(\"ML-DSA-65\");signer.sign(data,\"SYNTH087_ROLE_KEY\");}"),
        ("cpp", "void f(){oqs::Signature signer(\"ML-DSA-65\",\"SYNTH087_ROLE_KEY\");signer.sign(data);}"),
        ("cpp", "void f(){auto signer=new oqs::Signature(\"ML-DSA-65\",\"SYNTH087_ROLE_KEY\");}"),
        ("php", "<?php \\ParagonIE\\PQCrypto\\Compat::mldsa44_sign('SYNTH087_ROLE_KEY','public message');"),
        ("c", "void f(){const char *material=\"SYNTH087_ROLE_KEY\";const unsigned char *alias=(unsigned char*)material;EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,alias,32);}"),
        ("cs", "using System.Security.Cryptography; class C{void M(){var material=\"SYNTH087_ROLE_KEY\";MLKem.ImportPrivateSeed(MLKemAlgorithm.MLKem768,material);}}"),
        ("c", "void f(){const char *material=\"SYNTH087_ROLE_KEY\";EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,(material),32);}"),
        ("c", "void f(){const char *material=\"SYNTH087_ROLE_KEY\";EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,(unsigned char*)material,32);}"),
        ("cs", "using System.Security.Cryptography; class C{void M(){RSA.CreateFromEncryptedPem(\"SYNTH087_ROLE_KEY\",\"SYNTH087_PASSWORD\");}}"),
        ("php", "<?php $k=\\PQCrypto\\MLKem768\\DecapsulationKey::fromBytes('SYNTH087_ROLE_KEY');$k->decapsulate($ct);"),
    ] {
        let d=scan(source,ext,false);assert!(!serde_json::to_string(d.legacy()).unwrap().contains("SYNTH087_"),"{source}");
    }
}
#[test]
fn node_cap_protects_preceding_literals_used_by_unvisited_calls() {
    let source=format!("void f(){{const char *material=\"SYNTH087_PREFIX_KEY\";{} EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,material,32);RSA_new();}}","int n=0;".repeat(20));
    for max_ast_nodes in [16, 32, 48] {
        let d = scan_budget(
            &source,
            "c",
            false,
            Some(pqc_scan_detectors::DetectorBudget {
                max_ast_nodes,
                ..Default::default()
            }),
        );
        assert!(
            !serde_json::to_string(d.legacy())
                .unwrap()
                .contains("SYNTH087_PREFIX_KEY"),
            "nodecap{max_ast_nodes}"
        );
        assert!(d.coverage().partial_files() > 0);
    }
}

#[test]
fn expression_unwrap_limit_protects_source_initializers() {
    let source=format!("void f(){{const char *material=\"SYNTH087_WRAPPED\";EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,{}material{},32);RSA_new();}}","(".repeat(20),")".repeat(20));
    let d = scan(&source, "c", false);
    assert!(!serde_json::to_string(d.legacy())
        .unwrap()
        .contains("SYNTH087_WRAPPED"));
}

#[test]
fn additional_known_secret_slots_preserve_public_messages() {
    for (ext,call) in [
        ("php", "sodium_crypto_box_open('public ciphertext',$nonce,'SYNTH087_SLOT')"),
        ("php", "sodium_crypto_box_seal_open('public ciphertext','SYNTH087_SLOT')"),
        ("php", "sodium_crypto_kem_mlkem768_dec('public ciphertext','SYNTH087_SLOT')"),
        ("c", "OQS_SIG_sign(sig,out,len,msg,size,\"SYNTH087_SLOT\")"),
        ("cs", "System.Security.Cryptography.ECDiffieHellman.Create().DeriveKeyFromHmac(peer,HashAlgorithmName.SHA256,\"SYNTH087_SLOT\")"),
    ] {
        let source=match ext {"php"=>format!("<?php {call};"),"cs"=>format!("class C{{void M(){{{call};}}}}"),_=>format!("void f(){{{call};}}")};
        let d=scan(&source,ext,false);assert!(!serde_json::to_string(d.legacy()).unwrap().contains("SYNTH087_SLOT"),"{call}");
    }
}

#[test]
fn named_secret_roles_and_file_uri_like_secrets_are_not_public_paths() {
    for call in [
        "sodium_crypto_sign_seed_keypair(seed:'SYNTH087_NAMED')",
        "sodium_crypto_sign_publickey(key_pair:'SYNTH087_NAMED')",
        "sodium_crypto_box_seal_open(ciphertext:'public',key_pair:'SYNTH087_NAMED')",
        "sodium_crypto_scalarmult(p:'public',n:'SYNTH087_NAMED')",
        "sodium_crypto_sign_seed_keypair('file://SYNTH087_URI_SEED')",
        "openssl_pkey_get_private('file://keys/key.pem','file://SYNTH087_URI_PASSWORD'); openssl_sign('public message',$sig,$key)",
    ] {
        let d = scan(&format!("<?php {call};"), "php", false);
        assert!(
            !serde_json::to_string(d.legacy())
                .unwrap()
                .contains("SYNTH087_"),
            "{call}"
        );
    }
}
#[test]
fn c_private_key_slots_and_unresolved_initializer_wrappers_are_protected() {
    for call in [
        "RSA_private_encrypt(size,from,to,\"SYNTH087_RSA_KEY\",padding)",
        "RSA_private_decrypt(size,from,to,\"SYNTH087_RSA_KEY\",padding)",
        "wc_RsaSSL_Sign(in,size,out,cap,\"SYNTH087_WC_KEY\",rng)",
    ] {
        let d = scan(&format!("void f(){{{call};}}"), "c", false);
        assert!(
            !serde_json::to_string(d.legacy())
                .unwrap()
                .contains("SYNTH087_"),
            "{call}"
        );
    }
    let source=format!("void f(){{const char *material={}\"SYNTH087_WRAPPED_INIT\"{}; EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,material,32); RSA_new();}}","(".repeat(20),")".repeat(20));
    let d = scan(&source, "c", false);
    assert!(!serde_json::to_string(d.legacy())
        .unwrap()
        .contains("SYNTH087_WRAPPED_INIT"));
}

#[test]
fn public_labels_survive_private_key_member_and_algorithm_references() {
    for (ext,source) in [
        ("cpp", "void f(){const char *label=\"PUBLIC087_LABEL\"; secp256k1_ecdsa_sign(ctx,sig,hash,key.data(),nonce,data);}"),
        ("cs", "using System.Security.Cryptography; class C{void M(){var label=\"PUBLIC087_LABEL\";var material=\"SYNTH087_MATERIAL\";MLKem.ImportPrivateSeed(MLKemAlgorithm.MLKem768,material);}}"),
    ] {
        let d=scan(source,ext,false);let text=serde_json::to_string(d.legacy()).unwrap();
        assert!(text.contains("PUBLIC087_LABEL"),"{ext}: public label hidden");
        assert!(!text.contains("SYNTH087_MATERIAL"));
    }
}

#[test]
fn key_data_nested_in_conversion_calls_is_masked_without_hiding_the_call() {
    for (ext,source,visible) in [
        ("cpp", "void f(){const char *label=\"PUBLIC087_LABEL\";EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,std::string(\"SYNTH087_NESTED\"),32);}", "std::string("),
        ("cs", "using System.Security.Cryptography; class C{void M(){var label=\"PUBLIC087_LABEL\";var material=Decode(\"SYNTH087_NESTED\");MLKem.ImportPrivateSeed(MLKemAlgorithm.MLKem768,material);}}", "Decode("),
    ] {
        let d=scan(source,ext,false);let text=serde_json::to_string(d.legacy()).unwrap();
        assert!(!text.contains("SYNTH087_NESTED"),"{ext}");assert!(text.contains(visible));
        assert!(text.contains("PUBLIC087_LABEL"));
    }
}

#[test]
fn key_conversion_with_variable_expression_does_not_lose_initializer_protection() {
    let source="void f(){const char *material=\"SYNTH087_COMPOSED\";EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,std::string(material+\"suffix\"),32);}";
    let d = scan(source, "cpp", false);
    assert!(!serde_json::to_string(d.legacy())
        .unwrap()
        .contains("SYNTH087_COMPOSED"));
}

#[test]
fn php_private_aliases_are_protected_and_public_key_aliases_stay_readable() {
    for source in [
        "<?php use PQCrypto\\MLDSA44\\SigningKey as SK; $k=SK::fromBytes('SYNTH087_ALIAS'); $k->sign('public message');",
        "<?php use ParagonIE\\PQCrypto\\MLKem768\\DecapsulationKey as DK; $k=DK::fromBytes('SYNTH087_ALIAS'); $k->decapsulate($ct);",
        "<?php use function openssl_sign as signIt; signIt('public message',$sig,'SYNTH087_ALIAS');",
        "<?php use function sodium_crypto_sign_detached as sig; sig('public message','SYNTH087_ALIAS');",
    ] {
        let d=scan(source,"php",false);assert!(!serde_json::to_string(d.legacy()).unwrap().contains("SYNTH087_ALIAS"),"{source}");
    }
    for source in [
        "<?php use PQCrypto\\MLDSA44\\VerifyingKey as VK; $k=VK::fromBytes('PUBLIC087_ALIAS_KEY'); $k->verify($m,$s);",
        "<?php use ParagonIE\\PQCrypto\\MLKem768\\EncapsulationKey as EK; $k=EK::fromBytes('PUBLIC087_ALIAS_KEY'); $k->encapsulate();",
    ] {
        let d=scan(source,"php",false);assert!(serde_json::to_string(d.legacy()).unwrap().contains("PUBLIC087_ALIAS_KEY"),"{source}");
    }
}

#[test]
fn overlong_function_alias_still_protects_preceding_key_literal() {
    let alias = "a".repeat(300);
    let source=format!("<?php use function openssl_sign as {alias};\n$material='SYNTH087_LONG_ALIAS';\nRSA_new();\n{alias}('public message',$sig,$material);");
    let d = scan(&source, "php", false);
    assert!(!serde_json::to_string(d.legacy())
        .unwrap()
        .contains("SYNTH087_LONG_ALIAS"));
}

#[test]
fn c_and_cpp_array_initializers_follow_private_key_argument_roles() {
    let source="void f(){\n const unsigned char material[32]=\"SYNTH087_ARRAY_KEY\";\n EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,material,32);\n int visible=17;\n}";
    for ext in ["c", "cpp", "h"] {
        let d = scan(source, ext, false);
        let text = serde_json::to_string(d.legacy()).unwrap();
        assert!(!text.contains("SYNTH087_ARRAY_KEY"), "{ext}");
        assert!(text.contains("visible=17"));
    }
}

#[test]
fn named_credential_arrays_are_protected_without_a_crypto_key_argument() {
    for (ext,source) in [
        ("c", "void f(){const char password[32]=\"SYNTH087_PASSWORD_ARRAY\";RSA_new();int visible=17;}"),
        ("cpp", "void f(){unsigned char private_key[32]={0x13,0x37,0x42};RSA_new();int visible=17;}"),
        ("cs", "using System.Security.Cryptography;class C{void M(){byte[] privateKey={0x13,0x37,0x42};RSA.Create();int visible=17;}}"),
    ] {
        let d=scan(source,ext,false);let text=serde_json::to_string(d.legacy()).unwrap();
        assert!(!text.contains("SYNTH087_PASSWORD_ARRAY") && !text.contains("0x13,0x37,0x42"),"{ext}");
        assert!(text.contains("visible=17"));
    }
}

#[test]
fn computed_credential_expressions_keep_public_arguments_readable() {
    let source="using System.Security.Cryptography;class C{void M(){var password=Lookup(\"PUBLIC087_ENV_NAME\");RSA.Create();}}";
    let d = scan(source, "cs", false);
    assert!(serde_json::to_string(d.legacy())
        .unwrap()
        .contains("PUBLIC087_ENV_NAME"));
}

#[test]
fn php_credential_array_values_are_masked_and_public_entries_are_readable() {
    let source="<?php $options=['password'=>'SYNTH087_ARRAY_PASSWORD','public_label'=>'PUBLIC087_OPTION'];openssl_sign('public message',$sig,$key);";
    let d = scan(source, "php", false);
    let text = serde_json::to_string(d.legacy()).unwrap();
    assert!(!text.contains("SYNTH087_ARRAY_PASSWORD"));
    assert!(text.contains("PUBLIC087_OPTION"));
    let source="<?php $options=['password'=>Lookup('PUBLIC087_ENV_NAME')];openssl_sign('public message',$sig,$key);";
    let d = scan(source, "php", false);
    assert!(serde_json::to_string(d.legacy())
        .unwrap()
        .contains("PUBLIC087_ENV_NAME"));
}

#[test]
fn deeply_wrapped_named_credentials_keep_public_computed_expressions_visible() {
    for (ext, prefix, suffix, call) in [
        ("c", "void f(){const char *password=", ";}", "RSA_new()"),
        ("cpp", "void f(){const char *password=", ";}", "RSA_new()"),
        (
            "cs",
            "using System.Security.Cryptography;class C{void M(){var password=",
            ";}}",
            "RSA.Create()",
        ),
        (
            "php",
            "<?php $password=",
            ";",
            "openssl_sign('public',$sig,$key)",
        ),
    ] {
        let source = format!(
            "{prefix}{}\"SYNTH087_NAMED_WRAPPED\"{};{call}{suffix}",
            "(".repeat(20),
            ")".repeat(20)
        );
        let d = scan(&source, ext, false);
        assert!(
            !serde_json::to_string(d.legacy())
                .unwrap()
                .contains("SYNTH087_NAMED_WRAPPED"),
            "{ext}"
        );
        let public = format!(
            "{prefix}{}Lookup(\"PUBLIC087_ENV_NAME\"){};{call}{suffix}",
            "(".repeat(20),
            ")".repeat(20)
        );
        let d = scan(&public, ext, false);
        assert!(
            serde_json::to_string(d.legacy())
                .unwrap()
                .contains("PUBLIC087_ENV_NAME"),
            "{ext}"
        );
    }
}
