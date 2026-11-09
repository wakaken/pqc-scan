use std::path::{Path, PathBuf};

use pqc_scan_core::{Detector, ScannableFile};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;

fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .unwrap()
}

#[test]
fn checked_in_seven_language_fixtures_match_the_documented_rules() {
    let rules = rules();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/pqc-readiness");
    for (name, id) in [
        ("Example.java", "PQC_JAVA_API"),
        ("example.go", "PQC_GO_API"),
        ("example.js", "PQC_NODE_API"),
        ("example.ts", "PQC_WEBCRYPTO_API"),
        ("example.py", "PQC_PYTHON_API"),
        ("example.rs", "PQC_RUST_API"),
        ("example.rb", "PQC_RUBY_API"),
    ] {
        let path = root.join(name);
        let file = ScannableFile::from_bytes(path.clone(), std::fs::read(path).unwrap());
        assert!(
            TreeSitterDetector
                .detect(&file, &rules)
                .unwrap()
                .iter()
                .any(|hit| hit.rule_id == id),
            "{name}"
        );
    }
    let path = root.join("negative.js");
    let file = ScannableFile::from_bytes(path.clone(), std::fs::read(path).unwrap());
    assert!(TreeSitterDetector
        .detect(&file, &rules)
        .unwrap()
        .iter()
        .all(|hit| !hit.rule_id.starts_with("PQC_")));
}

#[test]
fn inventories_pqc_in_all_seven_languages_but_not_comments_or_similar_names() {
    let cases = [
        (
            "java",
            "PQC_JAVA_API",
            "class Example { void run() { Signature.getInstance(\"ML-DSA\"); } }",
            "// Signature.getInstance(\"ML-DSA\")\nclass Example {}",
        ),
        (
            "go",
            "PQC_GO_API",
            "package example\nimport \"crypto/mlkem\"",
            "package example\n// import \"crypto/mlkem\"",
        ),
        (
            "js",
            "PQC_NODE_API",
            "generateKeyPairSync('ml-kem-768');",
            "// generateKeyPairSync('ml-kem-768');",
        ),
        (
            "ts",
            "PQC_WEBCRYPTO_API",
            "subtle.generateKey({name: 'ML-DSA-65'}, false, ['sign']);",
            "// subtle.generateKey({name: 'ML-DSA-65'}, false, ['sign']);",
        ),
        (
            "py",
            "PQC_PYTHON_API",
            "mlkem.MLKEM768PrivateKey.generate()",
            "# mlkem.MLKEM768PrivateKey.generate()",
        ),
        (
            "rs",
            "PQC_RUST_API",
            "use ml_kem::MlKem768;",
            "// use ml_kem::MlKem768;",
        ),
        (
            "rb",
            "PQC_RUBY_API",
            "OpenSSL::PKey.generate_key('ML-DSA-65')",
            "# OpenSSL::PKey.generate_key('ML-DSA-65')",
        ),
    ];
    let rules = rules();
    for (ext, id, positive, negative) in cases {
        for (text, expected) in [
            (positive, true),
            (negative, false),
            ("const ML_KEM_helper = 1;", false),
        ] {
            let file = ScannableFile::from_bytes(
                PathBuf::from(format!("example.{ext}")),
                text.as_bytes().to_vec(),
            );
            let hits = TreeSitterDetector.detect(&file, &rules).unwrap();
            assert_eq!(
                hits.iter().any(|hit| hit.rule_id == id),
                expected,
                "{ext}: {text}"
            );
        }
    }
}

#[test]
fn detects_only_explicit_classical_tls_settings() {
    let rules = rules();
    for (id, positive, negative) in [
        (
            "PQC_GO_TLS_DISABLED",
            "GODEBUG=tlsmlkem=0",
            "GODEBUG=tlsmlkem=1",
        ),
        (
            "PQC_JAVA_CLASSICAL_GROUPS",
            "jdk.tls.namedGroups=x25519,secp256r1",
            "jdk.tls.namedGroups=X25519MLKEM768,x25519",
        ),
        (
            "PQC_RUBY_CLASSICAL_GROUPS",
            "ctx.groups = 'X25519:P-256'",
            "ctx.groups = 'X25519MLKEM768:X25519'",
        ),
    ] {
        for (text, expected) in [
            (positive, true),
            (negative, false),
            ("unrelated=true", false),
        ] {
            let file =
                ScannableFile::from_bytes(PathBuf::from("settings.conf"), text.as_bytes().to_vec());
            let hits = RegexDetector.detect(&file, &rules).unwrap();
            assert_eq!(
                hits.iter().any(|hit| hit.rule_id == id),
                expected,
                "{id}: {text}"
            );
        }
    }
}

#[test]
fn inventories_dependencies_without_claiming_pqc_is_enabled() {
    let rules = rules();
    for (name, text, expected) in [
        (
            "Cargo.toml",
            "[dependencies]\nrustls = \"0.23.27\"\nml-kem = \"0.2\"\n",
            true,
        ),
        (
            "package.json",
            r#"{"dependencies":{"@noble/post-quantum":"0.7.1"}}"#,
            true,
        ),
        ("requirements.txt", "cryptography==48.0.0", true),
        ("Gemfile", "gem 'openssl', '4.0.0'", true),
        (
            "build.gradle",
            "implementation 'org.bouncycastle:bcprov-jdk18on:1.80'",
            true,
        ),
        (
            "package.json",
            r#"{"dependencies":{"rustls":"0.23.27"}}"#,
            false,
        ),
        (
            "Cargo.toml",
            "[dependencies]\nmy-rustls-wrapper = \"1.0\"",
            false,
        ),
    ] {
        let file = ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec());
        let hits = DependencyDetector.detect(&file, &rules).unwrap();
        assert_eq!(
            hits.iter()
                .any(|hit| hit.rule_id == "DEP_PQC_CAPABLE_LIBRARY"),
            expected
        );
    }
    let rule = rules.get("DEP_PQC_CAPABLE_LIBRARY").unwrap();
    assert_eq!(rule.risk.as_str(), "quantum-uncertain");
}

#[test]
fn identifies_draft_algorithms_separately_from_final_mlkem() {
    let rules = rules();
    for (text, expected) in [
        ("const group = 'X25519Kyber768Draft00';", true),
        ("const group = 'X25519MLKEM768';", false),
        ("// X25519Kyber768Draft00", false),
    ] {
        let file = ScannableFile::from_bytes(PathBuf::from("example.js"), text.as_bytes().to_vec());
        assert_eq!(
            TreeSitterDetector
                .detect(&file, &rules)
                .unwrap()
                .iter()
                .any(|hit| hit.rule_id == "PQC_DRAFT_ALGORITHM"),
            expected
        );
    }
}
