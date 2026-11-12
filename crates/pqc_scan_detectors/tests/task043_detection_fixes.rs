//! Source-derived acceptance cases; specimens are parsed as data, never executed.
use pqc_scan_core::{Detection, Detector, ScannableFile};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use std::path::{Path, PathBuf};

fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
        .unwrap()
}
fn hits(name: &str, text: &str) -> Vec<Detection> {
    let file = ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec());
    let rules = rules();
    let mut hits = TreeSitterDetector.detect(&file, &rules).unwrap();
    hits.extend(RegexDetector.detect(&file, &rules).unwrap());
    hits.extend(DependencyDetector.detect(&file, &rules).unwrap());
    hits
}
fn assert_count(name: &str, text: &str, id: &str, expected: usize) {
    let actual: Vec<_> = hits(name, text)
        .into_iter()
        .filter(|d| d.rule_id == id)
        .collect();
    assert_eq!(actual.len(), expected, "{name}: {id}: {text:?}: {actual:?}");
}

#[test]
fn ssh_disabled_comment_is_not_an_enabled_algorithm() {
    assert_count(
        "sshd.conf",
        "  # HostKeyAlgorithms ssh-dss\nHostKeyAlgorithms ssh-ed25519\n",
        "SSH_DSS_SIGALG",
        0,
    );
    assert_count(
        "sshd.conf",
        "HostKeyAlgorithms ssh-dss\n",
        "SSH_DSS_SIGALG",
        1,
    );
}
#[test]
fn java_static_property_forms_preserve_intent_not_effective_state() {
    for value in ["secp256r1,secp384r1", "x25519"] {
        for text in [
            format!("jdk.tls.namedGroups={value}\n"),
            format!("System.setProperty(\"jdk.tls.namedGroups\", \"{value}\");"),
            format!("<entry key=\"jdk.tls.namedGroups\">{value}</entry>"),
        ] {
            assert_count("settings.conf", &text, "PQC_JAVA_CLASSICAL_GROUPS", 1);
        }
    }
    for text in [
        "# jdk.tls.namedGroups=x25519",
        "// System.setProperty(\"jdk.tls.namedGroups\", \"x25519\");",
        "System.setProperty(\"jdk.tls.namedGroups\", groups);",
        "<entry key=\"jdk.tls.namedGroups\">X25519MLKEM768,x25519</entry>",
        "System.setProperty(\"jdk.tls.namedGroups\", \"x25519-unknown\");",
    ] {
        assert_count("settings.conf", text, "PQC_JAVA_CLASSICAL_GROUPS", 0);
    }
}
#[test]
fn slhdsa_and_prestandard_names_have_exact_variant_boundaries() {
    for value in ["SLH-DSA-SHA2-128s", "SLH-DSA-SHAKE-256f"] {
        assert_count(
            "Use.java",
            &format!("Signature.getInstance(\"{value}\");"),
            "PQC_JAVA_API",
            1,
        );
        assert_count("use.rb", &format!("puts \"{value}\""), "PQC_RUBY_API", 1);
    }
    for value in ["SLH-DSA-SHA2-128z", "SLH-DSA-SHAKE-256f-extra"] {
        assert_count(
            "Use.java",
            &format!("Signature.getInstance(\"{value}\");"),
            "PQC_JAVA_API",
            0,
        );
        assert_count("use.rb", &format!("puts \"{value}\""), "PQC_RUBY_API", 0);
    }
    assert_count(
        "use.js",
        "generateKeyPairSync('kyber768');",
        "PQC_DRAFT_ALGORITHM",
        1,
    );
    assert_count(
        "use.js",
        "const monkeyber768 = 1; const x = 'ML-KEM-768';",
        "PQC_DRAFT_ALGORITHM",
        0,
    );
}
#[test]
fn gradle_static_declarations_are_found_without_executing_dsl() {
    for (name, text) in [
        (
            "build.gradle",
            "dependencies { implementation 'org.bouncycastle:bcprov-jdk18on:1.80' }",
        ),
        (
            "build.gradle.kts",
            "dependencies { implementation(\"org.bouncycastle:bcprov-jdk18on:1.80\") }",
        ),
    ] {
        let detected = hits(name, text);
        let dep = detected
            .iter()
            .find(|d| d.rule_id == "DEP_PQC_CAPABLE_LIBRARY")
            .expect("PQC capable declaration");
        assert_eq!(
            dep.evidence.metadata["dep_name"],
            "org.bouncycastle:bcprov-jdk18on"
        );
        assert_eq!(dep.evidence.metadata["dep_version"], "1.80");
        assert_eq!(dep.location.line, 1);
    }
    for text in [
        "// implementation(\"org.bouncycastle:bcprov-jdk18on:1.80\")",
        "dependencies { implementation(\"org.bouncycastle:bcprov-jdk18on:$version\") }",
        "println(\"implementation 'org.bouncycastle:bcprov-jdk18on:1.80'\")",
    ] {
        assert_count("build.gradle.kts", text, "DEP_PQC_CAPABLE_LIBRARY", 0);
    }
}
#[test]
fn gemspec_binds_the_declared_block_variable() {
    for variable in ["s", "package_spec"] {
        assert_count("package.gemspec", &format!("Gem::Specification.new do |{variable}|\n {variable}.add_dependency 'openssl', '3.3.0'\nend\n"), "DEP_PQC_CAPABLE_LIBRARY", 1);
    }
    for text in [
        "Gem::Specification.new do |s|\n other.add_dependency 'openssl', '3.3.0'\nend",
        "# spec.add_dependency 'openssl', '3.3.0'",
        "Gem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend",
    ] {
        assert_count("package.gemspec", text, "DEP_PQC_CAPABLE_LIBRARY", 0);
    }
}
#[test]
fn capable_library_rule_respects_exact_identity_and_source_ecosystem() {
    assert_count(
        "go.mod",
        "module example.invalid/test\ngo 1.24\nrequire github.com/cloudflare/circl v1.6.1\n",
        "DEP_PQC_CAPABLE_LIBRARY",
        1,
    );
    for text in [
        r#"{"bomFormat":"CycloneDX","specVersion":"1.7","components":[{"type":"library","name":"openssl","version":"3.5.0","purl":"pkg:generic/openssl@3.5.0"}]}"#,
        r#"{"spdxVersion":"SPDX-2.3","packages":[{"name":"openssl","versionInfo":"3.5.0"}]}"#,
    ] {
        assert_count("bom.json", text, "DEP_PQC_CAPABLE_LIBRARY", 1);
    }
    for (name, text) in [
        (
            "go.mod",
            "module example.invalid/test\ngo 1.24\nrequire github.com/example/circl v1.6.1\n",
        ),
        (
            "package.json",
            r#"{"dependencies":{"openssl":"3.5.0","circl":"1.0.0"}}"#,
        ),
        (
            "bom.json",
            r#"{"bomFormat":"CycloneDX","specVersion":"1.7","components":[{"type":"library","name":"my-openssl-wrapper","version":"3.5.0"}]}"#,
        ),
    ] {
        assert_count(name, text, "DEP_PQC_CAPABLE_LIBRARY", 0);
    }
}

#[test]
fn node_alias_calls_are_real_calls_and_local_scopes_are_respected() {
    for text in [
        "import {generateKeyPairSync as create} from 'node:crypto'; create('ml-kem-768');",
        "const {generateKeyPairSync: create} = require('crypto'); create('ml-dsa-44');",
        "import * as crypto from 'node:crypto'; crypto.generateKeyPairSync('ml-kem-768');",
    ] {
        assert_count("use.js", text, "PQC_NODE_API", 1);
    }
    for text in ["const help = 'generateKeyPairSync(\"ml-kem-768\")';", "function generateKeyPairSync(x) { return x; } generateKeyPairSync('ml-kem-768');", "import {generateKeyPairSync} from 'node:crypto'; function render(generateKeyPairSync) { return generateKeyPairSync('ml-kem-768'); }", "const generateKeyPairSync = local; generateKeyPairSync('ml-kem-768');", "const crypto = { generateKeyPairSync: local }; crypto.generateKeyPairSync('ml-kem-768');"] {
        assert_count("use.js", text, "PQC_NODE_API", 0);
    }
    assert_count("use.js", "import {generateKeyPairSync} from 'node:crypto'; { let generateKeyPairSync = local; generateKeyPairSync('ml-kem-768'); } generateKeyPairSync('ml-kem-768');", "PQC_NODE_API", 1);
}
#[test]
fn node_static_loop_variants_are_one_reference_per_call() {
    for text in ["for (const algorithm of ['ml-kem-512', 'ml-kem-1024']) { generateKeyPairSync(algorithm); }", "for (const [algorithm, size] of [['ml-dsa-44', 12], ['ml-dsa-87', 20]]) { generateKeyPairSync(algorithm); }"] {
        assert_count("use.js", text, "PQC_NODE_API", 1);
    }
    for text in ["for (const algorithm of namesFromUser) { generateKeyPairSync(algorithm); }", "for (let algorithm of ['ml-kem-768']) { algorithm = userInput; generateKeyPairSync(algorithm); }", "for (const algorithm of ['ml-kem-768', userInput]) { generateKeyPairSync(algorithm); }", "for (const algorithm of ['ml-kem-768']) { function render(algorithm) { generateKeyPairSync(algorithm); } }", "const algorithm = 'ml-kem-768'; const help = 'generateKeyPairSync(algorithm)';"] {
        assert_count("use.js", text, "PQC_NODE_API", 0);
    }
}
#[test]
fn explicit_python_import_and_rust_typed_operations_are_separate_references() {
    assert_count("use.py", "from cryptography.hazmat.primitives.asymmetric import mlkem\nk = mlkem.MLKEM768PrivateKey.generate()\n", "PQC_PYTHON_API", 2);
    let rust = "use ml_kem::*;\nlet (dk, ek) = MlKem768::generate(&mut rng);\nlet (ct, key) = ek.encapsulate(&mut rng).unwrap();\nlet key2 = dk.decapsulate(&ct).unwrap();\n";
    assert_count("use.rs", rust, "PQC_RUST_API", 4);
    assert_count(
        "use.rs",
        "mod ml_kem { pub struct MlKem768; }\nuse ml_kem::MlKem768;\n",
        "PQC_RUST_API",
        0,
    );
    assert_count("use.rs", "let (dk, ek) = unknown::generate(&mut rng);\nek.encapsulate(&mut rng);\ndk.decapsulate(&ct);", "PQC_RUST_API", 0);
}
#[test]
fn rsa_generation_and_key_bound_signatures_do_not_infer_rsa_from_digest() {
    let text = "const {generateKeyPairSync, createSign, createVerify} = require('node:crypto');\nconst {privateKey, publicKey} = generateKeyPairSync('rsa', {modulusLength: 2048});\nconst s = createSign('SHA256');\ns.sign(privateKey);\nconst v = createVerify('SHA256');\nv.verify(publicKey, signature);\n";
    assert_count("use.js", text, "TS_NODE_RSA_USAGE", 5);
    assert_count("use.js", text, "API_NODE_CRYPTO_SIGN_RSA", 4);
    for text in ["const s = createSign('SHA256'); s.sign(unknownKey);", "const {privateKey} = generateKeyPairSync('ec'); const s = createSign('SHA256'); s.sign(privateKey);", "const {privateKey} = generateKeyPairSync('rsa'); const s = localFactory(); s.sign(privateKey);"] {
        assert_count("use.js", text, "API_NODE_CRYPTO_SIGN_RSA", 0);
    }
}
#[test]
fn malformed_calls_retain_explicit_facts_and_partial_status() {
    let rules = rules();
    for (name, text, id) in [
        (
            "Use.java",
            "Signature.getInstance(\"ML-DSA-65\"\n",
            "PQC_JAVA_API",
        ),
        (
            "use.py",
            "key = mlkem.MLKEM768PrivateKey.generate(\n",
            "PQC_PYTHON_API",
        ),
        ("use.rs", "use ml_kem::\n", "PQC_RUST_API"),
    ] {
        let file = ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec());
        let outcome = TreeSitterDetector.detect_detailed(&file, &rules).unwrap();
        assert_eq!(outcome.status(), pqc_scan_core::DetectorStatus::Partial);
        assert_eq!(
            outcome
                .detections()
                .iter()
                .filter(|d| d.rule_id == id)
                .count(),
            1
        );
    }
    assert_count(
        "use.py",
        "help = 'mlkem.MLKEM768PrivateKey.generate('\n",
        "PQC_PYTHON_API",
        0,
    );
    assert_count(
        "Use.java",
        "String help = \"Signature.getInstance(\\\"ML-DSA-65\\\"\";",
        "PQC_JAVA_API",
        0,
    );
}

#[test]
fn source_resolution_does_not_turn_unknown_bindings_into_crypto() {
    for text in ["const f = function generateKeyPairSync() { generateKeyPairSync('ml-kem-768'); };", "import {generateKeyPairSync as make} from 'node:crypto'; make = userInput; make('ml-kem-768');", "for (const algorithm in ['ml-kem-768']) { generateKeyPairSync(algorithm); }", "for (const [, algorithm] of [['ml-kem-768','rsa']]) { generateKeyPairSync(algorithm); }"] {
        assert_count("use.js",text,"PQC_NODE_API",0);
    }
    assert_count(
        "use.js",
        "generateKeyPairSync(/* public selection */ 'ml-kem-768');",
        "PQC_NODE_API",
        1,
    );
    assert_count(
        "use.js",
        "for (const [, algorithm] of [[0, 'ml-kem-768']]) { generateKeyPairSync(algorithm); }",
        "PQC_NODE_API",
        1,
    );
    assert_count("use.js", "const {privateKey} = generateKeyPair('rsa'); const s = createSign('SHA256'); s.sign(privateKey);", "API_NODE_CRYPTO_SIGN_RSA", 0);
}
#[test]
fn inline_configuration_comments_do_not_create_enabled_ssh_findings() {
    assert_count(
        "sshd.conf",
        "HostKeyAlgorithms ssh-ed25519 # old ssh-dss disabled\n",
        "SSH_DSS_SIGALG",
        0,
    );
    assert_count(
        "sshd.conf",
        "HostKeyAlgorithms ssh-dss # migration required\n",
        "SSH_DSS_SIGALG",
        1,
    );
}
#[test]
fn rust_key_roles_and_shadowing_are_not_guessed_from_variable_names() {
    assert_count(
        "use.rs",
        "use ml_kem::*;\nlet (_, ek) = MlKem768::generate(&mut rng);\nek.encapsulate(&mut rng);",
        "PQC_RUST_API",
        3,
    );
    let text="use ml_dsa::MlDsa44;\nlet (dk, ek) = MlDsa44::generate(&mut rng);\nek.encapsulate(&mut rng);";
    assert!(!hits("use.rs", text)
        .iter()
        .any(|d| d.rule_id == "PQC_RUST_API" && d.location.line == 3));
    let text="use ml_kem::*; fn f(MlKem768: Unknown) { let (dk, ek) = MlKem768::generate(&mut rng); ek.encapsulate(&mut rng); }";
    assert!(!hits("use.rs", text)
        .iter()
        .any(|d| d.rule_id == "PQC_RUST_API" && d.evidence.r#match.contains("generate")));
}

#[test]
fn malformed_display_text_and_identifier_prefixes_are_not_recovered_as_calls() {
    for text in [
        "text = 'mlkem.MLKEM768PrivateKey.generate(\n",
        "# mlkem.MLKEM768PrivateKey.generate(\n",
    ] {
        assert_count("use.py", text, "PQC_PYTHON_API", 0);
    }
    assert_count(
        "Use.java",
        "OtherSignature.getInstance(\"ML-DSA-65\");",
        "PQC_JAVA_API",
        0,
    );
}
#[test]
fn source_fact_limits_are_explicit_and_custom_patterns_keep_their_meaning() {
    let rules = rules();
    for (count, expected_status) in [
        (32, pqc_scan_core::DetectorStatus::Complete),
        (33, pqc_scan_core::DetectorStatus::Truncated),
    ] {
        let text = "generateKeyPairSync('ml-kem-768');\n".repeat(count);
        let file = ScannableFile::from_bytes(PathBuf::from("use.js"), text.into_bytes());
        let out = TreeSitterDetector.detect_detailed(&file, &rules).unwrap();
        assert_eq!(out.status(), expected_status);
        assert_eq!(
            out.detections()
                .iter()
                .filter(|d| d.rule_id == "PQC_NODE_API")
                .count(),
            32
        );
    }
    let values = (0..33)
        .map(|_| "'ml-kem-768'")
        .collect::<Vec<_>>()
        .join(",");
    let file = ScannableFile::from_bytes(
        PathBuf::from("use.js"),
        format!("for(const algorithm of [{values}]) {{ generateKeyPairSync(algorithm); }}")
            .into_bytes(),
    );
    assert_eq!(
        TreeSitterDetector
            .detect_detailed(&file, &rules)
            .unwrap()
            .status(),
        pqc_scan_core::DetectorStatus::Truncated
    );
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "../../target/task043-custom-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("rules.yml"), "- id: PQC_NODE_API\n  kind: tree_sitter\n  category: Test\n  severity: info\n  risk: quantum-uncertain\n  confidence: 0.5\n  migration_hint: Test only\n  pattern: 'literal-control'\n  scope: javascript\n").unwrap();
    let custom = RuleSet::load_from_dir(&dir).unwrap();
    let file = ScannableFile::from_bytes(
        PathBuf::from("use.js"),
        b"const text = 'literal-control';".to_vec(),
    );
    assert_eq!(TreeSitterDetector.detect(&file, &custom).unwrap().len(), 1);
}

#[test]
fn unchanged_pattern_with_custom_scope_does_not_enable_builtin_semantics() {
    let mut rule = rules().get("PQC_NODE_API").unwrap().clone();
    rule.scope = "java".into();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "../../target/task043-custom-scope-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("rules.yml"),
        serde_yaml::to_string(&vec![rule]).unwrap(),
    )
    .unwrap();
    let custom = RuleSet::load_from_dir(&dir).unwrap();
    let file = ScannableFile::from_bytes(
        PathBuf::from("Use.java"),
        b"class Use { String help = \"generateKeyPairSync('ml-kem-768')\"; }".to_vec(),
    );
    assert_eq!(TreeSitterDetector.detect(&file, &custom).unwrap().len(), 1);
}

#[test]
fn java_parameter_sets_are_not_interchanged_between_pqc_families() {
    for value in ["ML-KEM-44", "ML-DSA-768", "SLH-DSA-SHA2-512s"] {
        assert_count(
            "Use.java",
            &format!("Signature.getInstance(\"{value}\");"),
            "PQC_JAVA_API",
            0,
        );
    }
}

#[test]
fn method_and_closure_bindings_do_not_leak_across_scopes() {
    assert_count("use.js", "import {generateKeyPairSync} from 'crypto'; class View { render(generateKeyPairSync) { generateKeyPairSync('ml-kem-768'); } } generateKeyPairSync('ml-kem-768');", "PQC_NODE_API", 1);
    assert_count("use.rs", "use ml_kem::*; let (dk,ek) = MlKem768::generate(&mut rng); let closure = |ek| ek.encapsulate(&mut rng); ek.encapsulate(&mut rng);", "PQC_RUST_API", 3);
    assert_count(
        "use.rs",
        "use ml_kem::MlKem768; fn f<MlKem768>() { MlKem768::generate(&mut rng); }",
        "PQC_RUST_API",
        1,
    );
    assert_count("use.py", "from cryptography.hazmat.primitives.asymmetric import mlkem\nclass View:\n mlkem = object()\n def render(self):\n  mlkem.MLKEM768PrivateKey.generate()\n", "PQC_PYTHON_API", 2);
}

#[test]
fn binding_limit_does_not_reuse_an_older_crypto_binding() {
    let mut text = "var make = generateKeyPairSync;\n".repeat(64);
    text.push_str("var make = localFunction;\nmake('ml-kem-768');\n");
    let file = ScannableFile::from_bytes(PathBuf::from("use.js"), text.into_bytes());
    let out = TreeSitterDetector.detect_detailed(&file, &rules()).unwrap();
    assert_eq!(out.status(), pqc_scan_core::DetectorStatus::Truncated);
    assert!(!out.detections().iter().any(|d| d.rule_id == "PQC_NODE_API"));
}

#[test]
fn default_rest_and_loop_bindings_do_not_fall_back_to_crypto_names() {
    for text in [
        "function f(generateKeyPairSync = local) { generateKeyPairSync('ml-kem-768'); }",
        "function f(...generateKeyPairSync) { generateKeyPairSync('ml-kem-768'); }",
        "const f = (generateKeyPairSync = local) => generateKeyPairSync('ml-kem-768');",
    ] {
        assert_count("use.js", text, "PQC_NODE_API", 0);
    }
    for text in [
        "def f(mlkem=local):\n mlkem.MLKEM768PrivateKey.generate()\n",
        "def f(mlkem: object):\n mlkem.MLKEM768PrivateKey.generate()\n",
        "def f(mlkem: object=local):\n mlkem.MLKEM768PrivateKey.generate()\n",
        "def f(*mlkem):\n mlkem.MLKEM768PrivateKey.generate()\n",
        "for mlkem in values:\n mlkem.MLKEM768PrivateKey.generate()\n",
        "mlkem += local\nmlkem.MLKEM768PrivateKey.generate()\n",
    ] {
        assert_count("use.py", text, "PQC_PYTHON_API", 0);
    }
    assert_count("use.py", "from cryptography.hazmat.primitives.asymmetric import mlkem\nmlkem += local\nmlkem.MLKEM768PrivateKey.generate()\n", "PQC_PYTHON_API", 1);
    assert_count("use.js", "import {generateKeyPairSync} from 'crypto'; function f(local = null) { generateKeyPairSync('ml-kem-768'); }", "PQC_NODE_API", 1);
}

#[test]
fn rust_items_shadow_for_the_whole_scope_but_let_is_sequential() {
    assert_count(
        "use.rs",
        "use ml_kem::MlKem768; mod ml_kem { pub struct MlKem768; }",
        "PQC_RUST_API",
        0,
    );
    assert_count(
        "use.rs",
        "use ml_kem::*; fn f() { MlKem768::generate(&mut rng); } struct MlKem768;",
        "PQC_RUST_API",
        1,
    );
    assert_count(
        "use.rs",
        "fn f() { MlKem768::generate(&mut rng); } use ml_kem::MlKem768;",
        "PQC_RUST_API",
        2,
    );
    assert_count("use.rs", "use ml_kem::*; fn f() { MlKem768::generate(&mut rng); let MlKem768 = local; MlKem768::generate(&mut rng); }", "PQC_RUST_API", 2);
}

#[test]
fn gemspec_receivers_end_with_their_block_and_rebindings() {
    for text in [
        "Gem::Specification.new do |s|\nend\ns = unrelated\ns.add_dependency 'openssl', '3.3.0'\n",
        "Gem::Specification.new do |s|\n s = unrelated\n s.add_dependency 'openssl', '3.3.0'\nend\n",
        "Gem::Specification.new do |s|\n values.each do |s|\n s.add_dependency 'openssl', '3.3.0'\n end\nend\n",
        "display = \"Gem::Specification.new do |s|\"\ns.add_dependency 'openssl', '3.3.0'\n",
    ] { assert_count("package.gemspec", text, "DEP_PQC_CAPABLE_LIBRARY", 0); }
    assert_count(
        "package.gemspec",
        "Gem::Specification.new do |s|\n s.add_dependency 'openssl', '3.3.0'\nend\nother = local\n",
        "DEP_PQC_CAPABLE_LIBRARY",
        1,
    );
}

#[test]
fn built_in_source_facts_enforce_alias_depth_ast_depth_and_candidate_budgets() {
    for (count, expected) in [
        (8, pqc_scan_core::DetectorStatus::Complete),
        (9, pqc_scan_core::DetectorStatus::Truncated),
    ] {
        let mut text = String::from("import {generateKeyPairSync as base} from 'crypto';\n");
        for i in 0..count {
            let previous = if i == 0 {
                "base".to_string()
            } else {
                format!("a{}", i - 1)
            };
            text.push_str(&format!("const a{i}={previous};\n"));
        }
        text.push_str(&format!("a{}('ml-kem-768');", count - 1));
        let file = ScannableFile::from_bytes(PathBuf::from("use.js"), text.into_bytes());
        let out = TreeSitterDetector.detect_detailed(&file, &rules()).unwrap();
        assert_eq!(out.status(), expected);
        assert_eq!(
            out.detections()
                .iter()
                .filter(|d| d.rule_id == "PQC_NODE_API")
                .count(),
            usize::from(count == 8)
        );
    }
    for text in [
        format!(
            "{}generateKeyPairSync('ml-kem-768');{}",
            "{".repeat(70),
            "}".repeat(70)
        ),
        format!(
            "{}generateKeyPairSync('ml-kem-768');",
            "ordinary();\n".repeat(4097)
        ),
    ] {
        let file = ScannableFile::from_bytes(PathBuf::from("use.js"), text.into_bytes());
        let out = TreeSitterDetector.detect_detailed(&file, &rules()).unwrap();
        assert_eq!(out.status(), pqc_scan_core::DetectorStatus::Truncated);
        assert!(!out.detections().iter().any(|d| d.rule_id == "PQC_NODE_API"));
    }
}

#[test]
fn enhanced_regex_keeps_distinct_source_starts_and_its_32_hit_budget() {
    for (count, status) in [
        (32, pqc_scan_core::DetectorStatus::Complete),
        (33, pqc_scan_core::DetectorStatus::Truncated),
    ] {
        let text = format!(
            "const 日='😀'; {}",
            "createSign('RSA-SHA256'); ".repeat(count)
        );
        let file = ScannableFile::from_bytes(PathBuf::from("use.js"), text.as_bytes().to_vec());
        let out = RegexDetector.detect_observed(&file, &rules()).unwrap();
        assert_eq!(out.status(), status);
        let hits: Vec<_> = out
            .detections()
            .iter()
            .filter(|d| d.rule_id == "API_NODE_CRYPTO_SIGN_RSA")
            .collect();
        assert_eq!(hits.len(), 32);
        let expected: Vec<_> = text
            .match_indices("createSign(")
            .take(32)
            .map(|(i, _)| text[..i].chars().count() + 1)
            .collect();
        assert_eq!(
            hits.iter().map(|d| d.location.column).collect::<Vec<_>>(),
            expected
        );
        assert!(hits.iter().all(|d| d.location.line == 1));
    }
}

#[test]
fn python_context_exception_and_unrelated_imports_block_name_fallback() {
    for text in [
        "with local_context as mlkem:\n mlkem.MLKEM768PrivateKey.generate()\n",
        "try:\n ordinary()\nexcept Error as mlkem:\n mlkem.MLKEM768PrivateKey.generate()\n",
        "import unrelated as mlkem\nmlkem.MLKEM768PrivateKey.generate()\n",
    ] {
        assert_count("use.py", text, "PQC_PYTHON_API", 0);
    }
}

#[test]
fn gemspec_ast_budget_is_truncated_rather_than_an_empty_unreported_success() {
    let text = format!(
        "Gem::Specification.new do |s|\n{}\n s.add_dependency 'openssl', '3.3.0'\nend\n",
        "ordinary()\n".repeat(4097)
    );
    let file = ScannableFile::from_bytes(PathBuf::from("package.gemspec"), text.into_bytes());
    assert_eq!(
        DependencyDetector
            .detect_detailed(&file, &rules())
            .unwrap()
            .status(),
        pqc_scan_core::DetectorStatus::Truncated
    );
}

#[test]
fn python_loop_body_assignments_also_shadow_following_calls() {
    assert_count("use.py", "from cryptography.hazmat.primitives.asymmetric import mlkem\nfor other in values:\n mlkem = local\nmlkem.MLKEM768PrivateKey.generate()\n", "PQC_PYTHON_API", 1);
}
