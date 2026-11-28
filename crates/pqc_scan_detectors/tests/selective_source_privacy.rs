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

#[test]
fn ruby_private_scalar_alias_is_masked_at_its_origin() {
    let source = "def sample\n  material = \"SYNTH087_RUBY_SCALAR\"\n  ec = OpenSSL::PKey::EC.new(\"prime256v1\")\n  ec.private_key = OpenSSL::BN.new(material, 16)\nend\n";
    let document = scan(source, "rb", false);
    let serialized = serde_json::to_string(document.legacy()).unwrap();
    assert!(
        !serialized.contains("SYNTH087_RUBY_SCALAR"),
        "Ruby scalar initializer leaked"
    );
    assert!(serialized.contains("prime256v1"));
}

fn ruby_ranges(source: &str, node_limit: usize) -> Vec<std::ops::Range<usize>> {
    use pqc_scan_core::{Detector, ScannableFile};
    let file = ScannableFile::from_bytes("public.rb".into(), source.as_bytes().to_vec());
    let rules =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap();
    let detector = pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector::with_budget(
        pqc_scan_detectors::DetectorBudget {
            max_ast_nodes: node_limit,
            ..Default::default()
        },
    );
    let detailed = detector.detect_detailed(&file, &rules).unwrap();
    let observed = detector.detect_observed(&file, &rules).unwrap();
    assert_eq!(detailed.sensitive_ranges(), observed.sensitive_ranges());
    detailed.sensitive_ranges().to_vec()
}

fn ruby_protects(source: &str, hidden: &[&str], visible: &[&str]) {
    let ranges = ruby_ranges(source, 100_000);
    for value in hidden {
        let start = source.find(value).unwrap();
        assert!(
            ranges
                .iter()
                .any(|r| r.start <= start && r.end >= start + value.len()),
            "unprotected fake value {value}"
        );
    }
    for value in visible {
        let start = source.find(value).unwrap();
        assert!(
            !ranges
                .iter()
                .any(|r| r.start < start + value.len() && start < r.end),
            "public value was hidden: {value}"
        );
    }
    assert!(ranges.iter().all(|r| source.is_char_boundary(r.start)
        && source.is_char_boundary(r.end)
        && r.end <= source.len()));
}

#[test]
fn ruby_scalar_direct_bn_and_receiver_aliases_keep_public_inputs() {
    ruby_protects(
        r#"def f
  material = "FAKE_SCALAR_ORIGIN"
  copy = material
  scalar = OpenSSL::BN.new(copy, 16)
  ec = OpenSSL::PKey::EC.generate("PUBLIC_CURVE")
  alias_ec = ec
  alias_ec.private_key=(scalar)
  public_number = OpenSSL::BN.new("PUBLIC_BN", 16)
  ec.public_key = OpenSSL::BN.new("PUBLIC_POINT", 16)
end
"#,
        &["FAKE_SCALAR_ORIGIN"],
        &["PUBLIC_CURVE", "PUBLIC_BN", "PUBLIC_POINT", "16"],
    );
    ruby_protects(
        r#"ec = ::OpenSSL::PKey::EC.new("PUBLIC_CURVE")
ec.private_key = ::OpenSSL::BN.new("FAKE_DIRECT", 16)
"#,
        &["FAKE_DIRECT"],
        &["PUBLIC_CURVE", "16"],
    );
}

#[test]
fn ruby_scalar_scope_boundaries_do_not_share_local_names_or_receivers() {
    ruby_protects(
        r#"material = "PUBLIC_TOP"
module First
  material = "PUBLIC_MODULE"
  class Example
    material = "PUBLIC_CLASS"
    def f
      material = "FAKE_METHOD"
      ec = OpenSSL::PKey::EC.new("PUBLIC_CURVE")
      ec.private_key = material
      [1].each { material = "PUBLIC_BLOCK"; ec.private_key = material }
      while false; material = "PUBLIC_LOOP"; ec.private_key = material; end
      l = -> { material = "PUBLIC_LAMBDA"; ec.private_key = material }
      object.instance_eval { material = "PUBLIC_INSTANCE_EVAL"; ec.private_key = material }
    end
    def self.f
      material = "PUBLIC_SINGLETON"
      ec.private_key = material
    end
    def other
      material = "PUBLIC_OTHER"
      ec.private_key = material
    end
  end
end
module Second
  material = "PUBLIC_SECOND"
end
"#,
        &[
            "FAKE_METHOD",
            "PUBLIC_BLOCK",
            "PUBLIC_LOOP",
            "PUBLIC_LAMBDA",
            "PUBLIC_INSTANCE_EVAL",
        ],
        &[
            "PUBLIC_TOP",
            "PUBLIC_MODULE",
            "PUBLIC_CLASS",
            "PUBLIC_SINGLETON",
            "PUBLIC_OTHER",
            "PUBLIC_SECOND",
            "PUBLIC_CURVE",
        ],
    );
}

#[test]
fn ruby_scalar_namesakes_public_bn_and_shadowed_namespaces_do_not_seed_ranges() {
    for source in [
        r#"ec = Something::EC.new("PUBLIC_CURVE"); material = "PUBLIC_NAMESAKE"; ec.private_key = OpenSSL::BN.new(material,16)"#,
        r#"OpenSSL = Something; ec = OpenSSL::PKey::EC.new("PUBLIC_CURVE"); ec.private_key = "PUBLIC_NAMESAKE""#,
        r#"module M; OpenSSL = Something; ec = OpenSSL::PKey::EC.new("PUBLIC_CURVE"); ec.private_key = "PUBLIC_NAMESAKE"; end"#,
        r#"OpenSSL::PKey::EC = Something; ec = OpenSSL::PKey::EC.new("PUBLIC_CURVE"); ec.private_key = "PUBLIC_NAMESAKE""#,
        r#"ec = OpenSSL::PKey::EC.new("PUBLIC_CURVE"); number = OpenSSL::BN.new("PUBLIC_NAMESAKE",16); ec.public_key = number"#,
        r#"ec = a; a = ec; ec.private_key = "PUBLIC_NAMESAKE"; curve = "PUBLIC_CURVE""#,
    ] {
        ruby_protects(source, &[], &["PUBLIC_NAMESAKE", "PUBLIC_CURVE"]);
    }
}

#[test]
fn ruby_scalar_ambiguous_writes_cycles_and_limits_protect_affected_scope() {
    for body in [
        "a = b; b = a; other = \"FAKE_FALLBACK\"; ec.private_key = a".to_owned(),
        "a = \"FAKE_FALLBACK\"; a = \"FAKE_SECOND\"; ec.private_key = a".to_owned(),
        "a = \"FAKE_FALLBACK\"; b = opaque(a); ec.private_key = b".to_owned(),
        "a = \"FAKE_FALLBACK\"; b = \"computed#{a}\"; ec.private_key = b".to_owned(),
        format!(
            "a0 = \"FAKE_FALLBACK\"; {} ec.private_key = a12",
            (1..=12)
                .map(|n| format!("a{n} = a{};", n - 1))
                .collect::<String>()
        ),
        "a = \"FAKE_FALLBACK\"; other = ec; other = unknown; other.private_key = a".to_owned(),
        "a = \"FAKE_FALLBACK\"; a += unknown; ec.private_key = a".to_owned(),
    ] {
        let source = format!("def f\nec = OpenSSL::PKey::EC.new(\"curve\")\n{body}\nend\ndef other; a = \"PUBLIC_OTHER\"; end");
        ruby_protects(&source, &["FAKE_FALLBACK"], &["PUBLIC_OTHER"]);
    }
}

#[test]
fn ruby_scalar_unicode_multiline_and_node_exhaustion_use_original_ranges() {
    let source = "def f\n material = \"合成FAKE_FIRST\nFAKE_SECOND\"\n ec = OpenSSL::PKey::EC.new(\"curve\")\n ec.private_key = material\nend\n";
    ruby_protects(source, &["合成FAKE_FIRST\nFAKE_SECOND"], &["curve"]);
    for limit in [0, 4, 12, 24] {
        let ranges = ruby_ranges(source, limit);
        for marker in ["FAKE_FIRST", "FAKE_SECOND"] {
            let start = source.find(marker).unwrap();
            assert!(
                ranges
                    .iter()
                    .any(|r| r.start <= start && r.end >= start + marker.len()),
                "node cap exposed prior scalar initializer"
            );
        }
        assert!(ranges
            .iter()
            .all(|r| source.is_char_boundary(r.start) && source.is_char_boundary(r.end)));
    }
}

#[test]
fn ruby_scalar_privacy_runs_without_ruby_rules_through_all_outer_entrypoints() {
    use pqc_scan_core::{Detector, ScannableFile};
    let root = std::env::temp_dir().join(format!("pqc-ruby-no-rules-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("control.yml"), "[]").unwrap();
    let rules = RuleSet::load_from_dir(&root).unwrap();
    let source = "a = \"FAKE_NO_RULES\"; ec = OpenSSL::PKey::EC.new(\"curve\"); ec.private_key = a";
    let file = ScannableFile::from_bytes("public.rb".into(), source.as_bytes().to_vec());
    let detector = pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector;
    for outcome in [
        detector.detect_detailed(&file, &rules),
        detector.detect_observed(&file, &rules),
        detector.detect_in_repository(&file, &rules, None, false),
        detector.detect_in_repository(&file, &rules, None, true),
    ] {
        let outcome = outcome.unwrap();
        assert!(outcome.detections().is_empty());
        let start = source.find("FAKE_NO_RULES").unwrap();
        assert!(outcome
            .sensitive_ranges()
            .iter()
            .any(|r| r.contains(&start)));
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn ruby_scalar_eight_round_aliases_stay_selective_and_recovery_is_conservative() {
    let chain = (1..=7)
        .map(|n| format!("a{n} = a{};", n - 1))
        .collect::<String>();
    let source = format!("a0 = \"FAKE_EIGHT\"; {chain} ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.private_key = a7");
    ruby_protects(&source, &["FAKE_EIGHT"], &["PUBLIC_CURVE"]);
    ruby_protects(
        "a = \"FAKE_RECOVERED\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.private_key = (a",
        &["FAKE_RECOVERED"],
        &[],
    );
}

#[test]
fn ruby_scalar_bn_comments_keep_argument_position_and_public_radix() {
    for value in ["\"FAKE_COMMENT_DIRECT\"", "material"] {
        let source = format!("material = \"FAKE_COMMENT_ALIAS\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.private_key = OpenSSL::BN.new(\n# public comment\n{value}, 16); public_number = OpenSSL::BN.new(\"PUBLIC_BN\", 16)");
        let hidden = if value == "material" {
            "FAKE_COMMENT_ALIAS"
        } else {
            "FAKE_COMMENT_DIRECT"
        };
        ruby_protects(&source, &[hidden], &["PUBLIC_CURVE", "PUBLIC_BN", "16"]);
    }
}

#[test]
fn ruby_scalar_loops_and_blocks_capture_both_receiver_and_material() {
    for body in [
        "while flag; ec.private_key = material; end",
        "until flag; ec.private_key = material; end",
        "for item in items; ec.private_key = material; end",
        "ec.private_key = material while flag",
        "items.each { ec.private_key = material }",
        "items.each do; ec.private_key = material; end",
        "l = -> { ec.private_key = material }",
        "items.each { local_ec = OpenSSL::PKey::EC.new(\"PUBLIC_LOCAL_CURVE\"); local_ec.private_key = material }",
        "items.each { local_material = \"FAKE_LOCAL\"; ec.private_key = local_material }",
        "items.each { items.each { ec.private_key = material } }",
    ] {
        let source = format!("def f; material = \"FAKE_CAPTURE\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); {body}; end; def other; material = \"PUBLIC_OTHER\"; end");
        let hidden = if body.contains("FAKE_LOCAL") { "FAKE_LOCAL" } else { "FAKE_CAPTURE" };
        ruby_protects(&source, &[hidden], &["PUBLIC_CURVE", "PUBLIC_OTHER"]);
    }
}

#[test]
fn ruby_scalar_explicit_block_shadows_and_new_block_locals_stay_isolated() {
    for body in [
        "items.each { |ec| ec.private_key = \"PUBLIC_SHADOW\" }",
        "items.each { |; ec| ec = Something::EC.new; ec.private_key = \"PUBLIC_SHADOW\" }",
        "l = ->(ec) { ec.private_key = \"PUBLIC_SHADOW\" }",
        "items.each { |(ec, other)| ec.private_key = \"PUBLIC_SHADOW\" }",
    ] {
        let source = format!("ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); {body}");
        ruby_protects(&source, &[], &["PUBLIC_SHADOW", "PUBLIC_CURVE"]);
    }
    for body in [
        "items.each { |material| material = \"FAKE_SHADOW\"; ec.private_key = material }",
        "items.each { |; material| material = \"FAKE_SHADOW\"; ec.private_key = material }",
        "l = ->(material) { material = \"FAKE_SHADOW\"; ec.private_key = material }",
        "items.each { material = \"FAKE_SHADOW\"; ec.private_key = material }",
    ] {
        let source = format!(
            "ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); {body}; material = \"PUBLIC_LATER\""
        );
        ruby_protects(&source, &["FAKE_SHADOW"], &["PUBLIC_LATER", "PUBLIC_CURVE"]);
    }
}

#[test]
fn ruby_scalar_receiver_wrapper_exhaustion_preserves_uncertainty() {
    for depth in [16, 17, 40] {
        for expression in [false, true] {
            let wrapped =
                |value: &str| format!("{}{value}{}", "(".repeat(depth), ")".repeat(depth));
            let source = if expression {
                format!("material = \"FAKE_WRAPPED\"; ec = OpenSSL::PKey::EC.new(\"curve\"); {}.private_key = material; def other; control = \"PUBLIC_OTHER\"; end", wrapped("ec"))
            } else {
                format!("material = \"FAKE_WRAPPED\"; ec = {}; ec.private_key = material; def other; control = \"PUBLIC_OTHER\"; end", wrapped("OpenSSL::PKey::EC.new(\"curve\")"))
            };
            ruby_protects(&source, &["FAKE_WRAPPED"], &["PUBLIC_OTHER"]);
        }
    }
    ruby_protects("ec = (((Something::EC.new))); ec.private_key = \"PUBLIC_NAMESAKE\"; other.private_key = \"PUBLIC_OTHER\"", &[], &["PUBLIC_NAMESAKE", "PUBLIC_OTHER"]);
}

#[test]
fn ruby_scalar_many_distinct_aliases_keep_public_controls() {
    let mut source = String::from("ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\");");
    for n in 0..192 {
        source.push_str(&format!(
            "a{n} = \"FAKE_MANY_{n}_END\"; ec.private_key = a{n};"
        ));
    }
    source.push_str(
        "public_bn = OpenSSL::BN.new(\"PUBLIC_BN\", 16); ec.public_key = \"PUBLIC_POINT\"",
    );
    let hidden: Vec<_> = (0..192).map(|n| format!("FAKE_MANY_{n}_END")).collect();
    let hidden: Vec<_> = hidden.iter().map(String::as_str).collect();
    ruby_protects(
        &source,
        &hidden,
        &["PUBLIC_CURVE", "PUBLIC_BN", "PUBLIC_POINT", "16"],
    );
}

#[test]
fn ruby_scalar_uncertain_capture_protects_outer_candidates_within_binding_barriers() {
    for body in [
        "items.each { ec.private_key = opaque(material) }".to_owned(),
        "items.each { items.each { ec.private_key = missing } }".to_owned(),
        format!(
            "{} items.each {{ a10.private_key = material }}",
            (1..=10)
                .map(|n| format!(
                    "a{n} = {};",
                    if n == 1 {
                        "ec".to_owned()
                    } else {
                        format!("a{}", n - 1)
                    }
                ))
                .collect::<String>()
        ),
    ] {
        let source = format!("def f; material = \"FAKE_UNCERTAIN_CAPTURE\"; ec = OpenSSL::PKey::EC.new(\"curve\"); {body}; end; def other; material = \"PUBLIC_OTHER\"; end");
        ruby_protects(&source, &["FAKE_UNCERTAIN_CAPTURE"], &["PUBLIC_OTHER"]);
    }
}

#[test]
fn ruby_scalar_alias_exhaustion_in_block_protects_captured_outer_material() {
    let chain = (1..=12)
        .map(|n| {
            format!(
                "a{n} = {};",
                if n == 1 {
                    "material".to_owned()
                } else {
                    format!("a{}", n - 1)
                }
            )
        })
        .collect::<String>();
    let source = format!("def f; material = \"FAKE_DEEP_CAPTURE\"; ec = OpenSSL::PKey::EC.new(\"curve\"); items.each {{ {chain} ec.private_key = a12 }}; end; def other; control = \"PUBLIC_OTHER\"; end");
    ruby_protects(&source, &["FAKE_DEEP_CAPTURE"], &["PUBLIC_OTHER"]);
}

#[test]
fn ruby_scalar_captured_bindings_do_not_depend_on_ancestor_depth() {
    for depth in [127, 128, 129, 130, 256] {
        let source = format!(
            "def private_method\nmaterial = \"FAKE_DEEP_BINDING\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{}ec.private_key = OpenSSL::BN.new(material, 16)\n{}end\ndef public_method\nmaterial = \"PUBLIC_OTHER_METHOD\"; ec = Something::EC.new; ec.private_key = material\nend\n",
            "items.each {\n".repeat(depth),
            "}\n".repeat(depth),
        );
        ruby_protects(
            &source,
            &["FAKE_DEEP_BINDING"],
            &["PUBLIC_CURVE", "PUBLIC_OTHER_METHOD"],
        );
    }
}

#[test]
fn ruby_scalar_syntax_depth_does_not_change_namespace_trust_or_scope() {
    for depth in [127, 128, 129, 130, 140] {
        let source = format!(
            "material = \"FAKE_DEEP_SYNTAX\"\n{}ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.private_key = OpenSSL::BN.new(material, 16){}\ndef other; material = \"PUBLIC_OTHER_METHOD\"; end\n",
            "if true; ".repeat(depth),
            "; end".repeat(depth),
        );
        ruby_protects(
            &source,
            &["FAKE_DEEP_SYNTAX"],
            &["PUBLIC_CURVE", "PUBLIC_OTHER_METHOD"],
        );
    }
}

#[test]
fn ruby_scalar_deep_captures_keep_parameter_and_block_local_shadows() {
    for parameter in ["|ec|", "|; ec|", "|(ec, other)|", "|ec = fallback|"] {
        let source = format!(
            "material = \"PUBLIC_SHADOW_MATERIAL\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{}items.each {{ {parameter} ec.private_key = material }}\n{}",
            "items.each {\n".repeat(130),
            "}\n".repeat(130),
        );
        ruby_protects(&source, &[], &["PUBLIC_SHADOW_MATERIAL", "PUBLIC_CURVE"]);
    }
    for parameter in ["|material|", "|; material|"] {
        let source = format!(
            "material = \"PUBLIC_OUTER_SHADOW\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{}items.each {{ {parameter} material = \"FAKE_SHADOW_INPUT\"; ec.private_key = material }}\n{}",
            "items.each {\n".repeat(130),
            "}\n".repeat(130),
        );
        ruby_protects(
            &source,
            &["FAKE_SHADOW_INPUT"],
            &["PUBLIC_OUTER_SHADOW", "PUBLIC_CURVE"],
        );
    }
}

#[test]
fn ruby_scalar_deep_namesakes_and_namespace_shadows_remain_public() {
    for prefix in ["", "OpenSSL = Something\n", "module OpenSSL; end\n"] {
        let constructor = if prefix.is_empty() {
            "Something::EC.new"
        } else {
            "OpenSSL::PKey::EC.new"
        };
        let source = format!(
            "{prefix}material = \"PUBLIC_NAMESAKE_MATERIAL\"\n{}ec = {constructor}(\"PUBLIC_CURVE\"); ec.private_key = material{}",
            "if true; ".repeat(140),
            "; end".repeat(140),
        );
        ruby_protects(&source, &[], &["PUBLIC_NAMESAKE_MATERIAL", "PUBLIC_CURVE"]);
    }
    let source = format!(
        "module Outer\nOpenSSL = Something\n{}material = \"PUBLIC_NAMESPACE_SHADOW\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.private_key = material\n{}end\n",
        "module Inner\n".repeat(130),
        "end\n".repeat(130),
    );
    ruby_protects(&source, &[], &["PUBLIC_NAMESPACE_SHADOW", "PUBLIC_CURVE"]);
    let source = format!(
        "{}material = \"FAKE_NAMESPACE_INPUT\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.private_key = material\n{}",
        "module Inner\n".repeat(130),
        "end\n".repeat(130),
    );
    ruby_protects(&source, &["FAKE_NAMESPACE_INPUT"], &["PUBLIC_CURVE"]);
}

#[test]
fn ruby_scalar_deep_binding_events_preserve_declaration_order_and_siblings() {
    let source = format!(
        "def container\n{}items.each {{ material = \"FAKE_LOCAL_BEFORE_CAPTURE\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); items.each {{ ec.private_key = material }} }}\nitems.each {{ material = \"PUBLIC_SIBLING\"; ec = Something::EC.new; ec.private_key = material }}\n{}material = \"PUBLIC_LATER_OUTER\"\nend\n",
        "items.each {\n".repeat(130),
        "}\n".repeat(130),
    );
    ruby_protects(
        &source,
        &["FAKE_LOCAL_BEFORE_CAPTURE"],
        &["PUBLIC_CURVE", "PUBLIC_SIBLING", "PUBLIC_LATER_OUTER"],
    );
}

#[test]
fn ruby_scalar_deep_metadata_truncation_keeps_unobserved_origins_private() {
    let source = format!(
        "material = \"FAKE_TRUNCATED_METADATA\"; ec = OpenSSL::PKey::EC.new(\"curve\")\n{}ec.private_key = OpenSSL::BN.new(material,16)\n{}",
        "items.each {\n".repeat(130),
        "}\n".repeat(130),
    );
    let start = source.find("FAKE_TRUNCATED_METADATA").unwrap();
    for limit in [0, 1, 2, 3, 5, 10, 20, 32, 64, 128, 512, 1024, 100_000] {
        let ranges = ruby_ranges(&source, limit);
        assert!(
            ranges.iter().any(|range| range.start <= start
                && range.end >= start + "FAKE_TRUNCATED_METADATA".len()),
            "unprotected metadata limit {limit}"
        );
    }
}

#[test]
fn ruby_scalar_deep_captures_run_through_both_entrypoints_without_rules() {
    use pqc_scan_core::{Detector, ScannableFile};
    let source = format!(
        "material = \"FAKE_DEEP_NO_RULES\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{}ec.private_key = material\n{}",
        "items.each {\n".repeat(130), "}\n".repeat(130),
    );
    let file = ScannableFile::from_bytes("public.rb".into(), source.as_bytes().to_vec());
    let detector = pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector;
    let rules = RuleSet::default();
    let direct = detector.detect_detailed(&file, &rules).unwrap();
    let observed = detector.detect_observed(&file, &rules).unwrap();
    assert_eq!(direct.sensitive_ranges(), observed.sensitive_ranges());
    let start = source.find("FAKE_DEEP_NO_RULES").unwrap();
    assert!(direct
        .sensitive_ranges()
        .iter()
        .any(|range| range.start <= start && range.end >= start + "FAKE_DEEP_NO_RULES".len()));
}

#[test]
fn ruby_scalar_identifier_byte_boundaries_keep_exact_bindings_across_rules() {
    use pqc_scan_core::{Detector, ScannableFile};
    let root = std::env::temp_dir().join(format!("pqc-ruby-long-name-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("control.yml"), r#"[{"id":"LONG_NAME_PUBLIC_CONTROL","kind":"regex","category":"CryptoAPI","severity":"medium","risk":"quantum-vulnerable","confidence":0.65,"migration_hint":"Review","pattern":"OpenSSL::PKey::EC\\.new","scope":"code"}]"#).unwrap();
    let custom = RuleSet::load_from_dir(&root).unwrap();
    let defaults =
        RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
            .unwrap();
    let empty = RuleSet::default();
    let detector = pqc_scan_detectors::tree_sitter_detector::TreeSitterDetector;
    for bytes in [511, 512, 513, 4096] {
        for unicode in [false, true] {
            let name = if unicode {
                format!(
                    "r{}{}",
                    "名".repeat((bytes - 1) / 3),
                    "x".repeat((bytes - 1) % 3)
                )
            } else {
                "r".repeat(bytes)
            };
            assert_eq!(name.len(), bytes);
            for source in [
                format!("material = \"FAKE_LONG_RECEIVER\"\n{name} = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{name}.private_key = material\nother.private_key = \"PUBLIC_OTHER\"\n"),
                format!("{name} =\n \"FAKE_LONG_MATERIAL\"\nec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\nec.private_key = OpenSSL::BN.new({name},16)\nother.private_key = \"PUBLIC_OTHER\"\n"),
            ] {
                let file = ScannableFile::from_bytes("public.rb".into(), source.as_bytes().to_vec());
                for rules in [&defaults, &custom, &empty] {
                    let direct = detector.detect_detailed(&file, rules).unwrap();
                    let observed = detector.detect_observed(&file, rules).unwrap();
                    assert_eq!(direct.sensitive_ranges(), observed.sensitive_ranges());
                    let marker = if source.contains("FAKE_LONG_RECEIVER") { "FAKE_LONG_RECEIVER" } else { "FAKE_LONG_MATERIAL" };
                    let start = source.find(marker).unwrap();
                    assert!(direct.sensitive_ranges().iter().any(|range| range.start <= start && range.end >= start + marker.len()), "unprotected identifier bytes {bytes}, unicode {unicode}");
                    for public in ["PUBLIC_CURVE", "PUBLIC_OTHER"] {
                        let start = source.find(public).unwrap();
                        assert!(!direct.sensitive_ranges().iter().any(|range| range.start < start + public.len() && range.end > start));
                    }
                }
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn ruby_scalar_long_aliases_and_deep_captures_keep_parameter_shadows() {
    let receiver = "r".repeat(1025);
    let material = "m".repeat(1027);
    let alias = "a".repeat(1029);
    let source = format!(
        "{material} = \"FAKE_LONG_CAPTURE\"\n{receiver} = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{alias} = {receiver}\n{}{alias}.private_key = {material}\n{}\n",
        "items.each {\n".repeat(130),
        "}\n".repeat(130),
    );
    ruby_protects(&source, &["FAKE_LONG_CAPTURE"], &["PUBLIC_CURVE"]);
    for parameter in [receiver.clone(), format!("; {receiver}")] {
        let source = format!("{material} = \"PUBLIC_LONG_SHADOW\"\n{receiver} = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{}items.each {{ |{parameter}| {receiver}.private_key = {material} }}\n{}", "items.each {\n".repeat(130), "}\n".repeat(130));
        ruby_protects(&source, &[], &["PUBLIC_LONG_SHADOW", "PUBLIC_CURVE"]);
    }
    let source = format!("def private_input\n{material} = \"FAKE_LONG_METHOD\"\n{receiver} = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n{receiver}.private_key = {material}\nend\ndef unrelated\n{material} = \"PUBLIC_LONG_METHOD\"\n{receiver} = Something::EC.new\n{receiver}.private_key = {material}\nend\n");
    ruby_protects(
        &source,
        &["FAKE_LONG_METHOD"],
        &["PUBLIC_LONG_METHOD", "PUBLIC_CURVE"],
    );
}

#[test]
fn ruby_scalar_long_material_survives_node_and_alias_exhaustion() {
    let material = "m".repeat(2049);
    let receiver = "r".repeat(2051);
    let mut source = format!("{material} =\n \"FAKE_LONG_LIMIT\"\n{receiver} = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n");
    for i in 0..12 {
        let value = if i == 0 {
            material.clone()
        } else {
            format!("a{}_{}", "x".repeat(513), i - 1)
        };
        source.push_str(&format!("a{}_{} = {value}\n", "x".repeat(513), i));
    }
    source.push_str(&format!(
        "{receiver}.private_key = a{}_11\n",
        "x".repeat(513)
    ));
    let start = source.find("FAKE_LONG_LIMIT").unwrap();
    for limit in [0, 1, 2, 3, 5, 10, 20, 32, 64, 128, 512, 1024, 100_000] {
        let ranges = ruby_ranges(&source, limit);
        assert!(
            ranges
                .iter()
                .any(|range| range.start <= start && range.end >= start + "FAKE_LONG_LIMIT".len()),
            "unprotected long initializer at node limit {limit}"
        );
    }
}
