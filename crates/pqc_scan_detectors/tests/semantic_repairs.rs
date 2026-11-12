use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use std::path::{Path, PathBuf};
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap()
}
fn ast(name: &str, text: &str) -> pqc_scan_core::DetectorOutcome {
    TreeSitterDetector
        .detect_detailed(
            &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap()
}
fn has(name: &str, text: &str, id: &str) {
    let o = ast(name, text);
    assert_eq!(o.status(), DetectorStatus::Complete, "{name}");
    assert!(
        o.detections().iter().any(|d| d.rule_id == id),
        "missing {id} in {name}"
    );
}
#[test]
fn python_rsa_calls_and_aliases_are_detected() {
    for source in ["from cryptography.hazmat.primitives.asymmetric import rsa\nk = rsa.generate_private_key(public_exponent=65537, key_size=4096)","from cryptography.hazmat.primitives.asymmetric import rsa as r\nk = r.generate_private_key(public_exponent=65537, key_size=4096)","from cryptography.hazmat.primitives.asymmetric.rsa import generate_private_key as make_key\nk = make_key(public_exponent=65537, key_size=4096)"] {has("a.py",source,"TS_PY_RSA_USAGE");}
}
#[test]
fn ruby_rsa_call_without_jwt_literal_is_detected() {
    has(
        "a.rb",
        "key = OpenSSL::PKey::RSA.new(4096)",
        "TS_RUBY_RSA_USAGE",
    );
}
#[test]
fn node_rsa_key_generation_is_detected() {
    has("a.js","const crypto = require('node:crypto');\nconst key = crypto.generateKeyPairSync('rsa', { modulusLength: 4096 });","TS_NODE_RSA_USAGE");
}
#[test]
fn classical_ec_and_eddsa_calls_are_detected() {
    for (name,text,id) in [("a.py","from cryptography.hazmat.primitives.asymmetric import ec\nk = ec.generate_private_key(ec.SECP256R1())","TS_CLASSICAL_EC_API"),("a.py","from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey\nk = Ed25519PrivateKey.generate()","TS_CLASSICAL_EDDSA_API"),("a.rb","k = OpenSSL::PKey::EC.generate('prime256v1')","TS_CLASSICAL_EC_API"),("a.rb","k = OpenSSL::PKey.generate_key('ED25519')","TS_CLASSICAL_EDDSA_API")] {has(name,text,id);}
}
#[test]
fn symmetric_and_unrelated_calls_do_not_become_rsa() {
    for (name, text) in [
        ("a.rb", "token = JWT.encode(payload, secret, 'HS256')"),
        (
            "a.py",
            "def generate_private_key():\n    return 1\nk = generate_private_key()",
        ),
        ("a.rb", "cipher = OpenSSL::Cipher.new('aes-256-gcm')"),
    ] {
        let o = ast(name, text);
        assert!(
            o.detections().iter().all(|d| ![
                "TS_PY_RSA_USAGE",
                "TS_RUBY_RSA_USAGE",
                "TS_CLASSICAL_EC_API",
                "TS_CLASSICAL_EDDSA_API"
            ]
            .contains(&d.rule_id.as_str())),
            "{name}"
        );
    }
}
#[test]
fn go_paths_in_non_go_data_are_not_go_imports() {
    let r = rules();
    for name in ["configdata.pm", "Makefile", "data.txt", "app.js"] {
        let f = ScannableFile::from_bytes(name.into(), b"output/crypto/rsa/example.o".to_vec());
        let o = RegexDetector.detect_detailed(&f, &r).unwrap();
        assert!(o.detections().iter().all(|d| !matches!(
            d.rule_id.as_str(),
            "GO_TLS_RSA_IMPORT" | "API_GO_CRYPTO_RSA"
        )));
    }
    let f = ScannableFile::from_bytes(
        "a.go".into(),
        b"package a\nimport \"crypto/rsa\"\n".to_vec(),
    );
    assert!(RegexDetector
        .detect(&f, &r)
        .unwrap()
        .iter()
        .any(|d| d.rule_id == "GO_TLS_RSA_IMPORT"));
}
#[test]
fn normal_syntax_keeps_crypto_and_original_coordinates() {
    for (name, prefix) in [
        ("a.ts", "type T = typeof import('module');\n"),
        (
            "a.tsx",
            "const x = <a href=\"https://example.invalid/?a=1&b=2\">Runtime & Build</a>;\n",
        ),
        ("a.jsx", "const x = <div class=\"container\" />;\n"),
        ("a.rb", "def kinds = raise NotImplementedError, 'missing'\n"),
        ("a.go", "package a\nvar p = new(int64(60))\n"),
    ] {
        let text = format!(
            "{prefix}{}",
            if name.ends_with(".go") {
                "var alg = \"RS256\""
            } else if name.ends_with(".rb") {
                "alg = 'RS256'"
            } else {
                "const alg = 'RS256';"
            }
        );
        let o = ast(name, &text);
        assert_eq!(o.status(), DetectorStatus::Complete, "{name}");
        assert!(o.detections().iter().any(
            |d| d.rule_id == "TS_JWT_RSA_ALGS" && d.location.line == prefix.lines().count() + 1
        ));
    }
}
#[test]
fn sbom_like_filenames_with_non_sbom_json_are_not_failures() {
    for (name, text) in [
        ("utf8-bom.json", "\u{feff}42"),
        ("spdx-license-ids.json", "[\"MIT\"]"),
        (
            "bom.schema.json",
            "{\"$schema\":\"https://json-schema.org/draft-07/schema#\"}",
        ),
        ("workflow.bom.json", "{\"nodes\":[]}"),
    ] {
        let o = DependencyDetector
            .detect_detailed(
                &ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::NotApplicable, "{name}");
    }
}
#[test]
fn generated_source_with_178k_candidates_is_inspected() {
    let source = "value;\n".repeat(180_000);
    let o = ast("generated.js", &source);
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().is_empty());
}
#[test]
fn malformed_real_sbom_is_still_an_error() {
    assert!(DependencyDetector
        .detect_detailed(
            &ScannableFile::from_bytes("dependency-sbom.json".into(), b"{bad".to_vec()),
            &rules()
        )
        .is_err());
}
fn dependency(name: &str, text: &str) -> pqc_scan_core::DetectorOutcome {
    DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap()
}
#[test]
fn catalog_and_patch_declarations_are_parsed_without_invented_versions() {
    for source in [
        r#"{"dependencies":{"jsonwebtoken":"catalog:","helper":"patch:helper@npm%3A1.0.0#./fix.patch","gitdep":"owner/repo#1234"}}"#,
    ] {
        let o = dependency("package.json", source);
        assert_eq!(o.status(), DetectorStatus::Complete);
        assert_eq!(
            o.detections()
                .iter()
                .filter(|d| d.rule_id == "__DEPENDENCY_INVENTORY__")
                .count(),
            3
        );
        for d in o.detections() {
            assert!(d
                .evidence
                .metadata
                .get("dep_purl")
                .is_none_or(|v| !v.contains('@')));
        }
    }
}
#[test]
fn pnpm_importer_alias_and_catalog_do_not_make_valid_yaml_partial() {
    let o=dependency("pnpm-lock.yaml","lockfileVersion: '9.0'\nimporters:\n  .:\n    dependencies:\n      token:\n        specifier: 'npm:jsonwebtoken@^9.0.0'\n        version: 9.0.2\n      other:\n        specifier: 'catalog:'\n        version: 1.0.0\npackages: {}\n");
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|s| s == "jsonwebtoken")));
}
#[test]
fn go_metadata_and_replacement_directives_are_statically_parsed() {
    let o=dependency("go.mod","module example.com/test\ngo 1.27.0\ngodebug default=go1.27\ntool example.com/tool\nrequire example.com/lib v1.2.3\nreplace example.com/lib => ../lib\n");
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|s| s == "example.com/lib")));
}
#[test]
fn requirements_hashes_are_metadata_not_invalid_dependencies() {
    let o=dependency("requirements.txt","cryptography==48.0.0 \\\n    --hash=sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n");
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|s| s == "cryptography")));
}
#[test]
fn bundler_git_path_sections_preserve_declared_sources() {
    let o=dependency("Gemfile.lock","PATH\n  remote: ../app\n  specs:\n    local-app (0.1.0)\n\nGEM\n  remote: https://rubygems.org/\n  specs:\n    jwt (2.10.1)\n\nPLATFORMS\n  ruby\n\nDEPENDENCIES\n  local-app!\n  jwt\n\nBUNDLED WITH\n   2.6.0\n");
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d.rule_id == "DEP_RUBY_JWT"));
}
#[test]
fn valid_maven_metadata_does_not_invalidate_literal_dependencies() {
    let o = dependency(
        "pom.xml",
        r#"<project><modelVersion>4.0.0</modelVersion><properties><junit.version>5.10.0</junit.version></properties><build><plugins><plugin><artifactId>example</artifactId></plugin></plugins></build><dependencies><dependency><groupId>org.example</groupId><artifactId>library</artifactId><version>1.2.3</version></dependency></dependencies></project>"#,
    );
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|s| s == "org.example:library")));
}
#[test]
fn syntax_compatibility_does_not_hide_malformed_input() {
    for (name, text) in [
        ("a.js", "const x = a &&;"),
        ("a.tsx", "const x = <div> & </span>;"),
        ("a.ts", "type T = import('bad);"),
        ("a.go", "package p\nvar x = new(@)"),
        ("a.rb", "def kinds = raise Error,"),
        ("a.js", "%V8Intrinsic("),
        ("a.js", "if true { }"),
    ] {
        assert_eq!(
            ast(name, text).status(),
            DetectorStatus::Partial,
            "{name}: {text}"
        );
    }
}
fn der(tag: u8, contents: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    if contents.len() < 128 {
        out.push(contents.len() as u8);
    } else {
        let size = contents.len() as u16;
        out.extend_from_slice(&[0x82, (size >> 8) as u8, size as u8]);
    }
    out.extend_from_slice(contents);
    out
}
fn pqc_spki(last: u8, size: usize, parameters: &[u8], unused: u8) -> Vec<u8> {
    let mut oid = vec![0x60, 0x86, 0x48, 1, 0x65, 3, 4, 3];
    oid.push(last);
    let mut alg = der(6, &oid);
    alg.extend_from_slice(parameters);
    let mut bits = vec![unused];
    bits.extend(vec![7; size]);
    let mut fields = der(0x30, &alg);
    fields.extend(der(3, &bits));
    der(0x30, &fields)
}
#[test]
fn pqc_public_key_shapes_are_measured_without_rsa_bit_claims() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    for (oid, size) in [
        (17, 1312),
        (18, 1952),
        (19, 2592),
        (20, 32),
        (22, 48),
        (24, 64),
        (26, 32),
        (28, 48),
        (30, 64),
    ] {
        let a = parse_public_key_der(&pqc_spki(oid, size, &[], 0)).unwrap();
        assert_eq!(a.status, DetectorStatus::Complete, "oid {oid}");
        assert_eq!(a.key_bits, None);
        assert_eq!(a.encoded_bytes, size);
        for invalid in [
            pqc_spki(oid, size - 1, &[], 0),
            pqc_spki(oid, size, &[5, 0], 0),
            pqc_spki(oid, size, &[], 1),
        ] {
            assert!(parse_public_key_der(&invalid)
                .map_or(true, |a| a.status != DetectorStatus::Complete));
        }
    }
}
#[test]
fn non_certificate_pem_does_not_produce_unsupported_certificate_diagnostic() {
    use pqc_scan_detectors::certificate_detector::CertificateDetector;
    let f = ScannableFile::from_bytes(
        "public.pem".into(),
        b"-----BEGIN PUBLIC KEY-----\nAA==\n-----END PUBLIC KEY-----\n".to_vec(),
    );
    let o = CertificateDetector.detect_observed(&f, &rules()).unwrap();
    assert_eq!(o.status(), DetectorStatus::NotApplicable);
    assert!(o.detections().is_empty());
    assert_eq!(o.observations().len(), 1);
}

#[test]
fn nested_typescript_import_types_keep_module_literals_and_crypto() {
    for prefix in [
        "const x = vi.importOriginal<typeof import('module')>();\n",
        "function f(headers: import('module').Headers) {}\n",
    ] {
        let text = format!("{prefix}const alg = 'RS256';");
        let o = ast("a.ts", &text);
        assert_eq!(o.status(), DetectorStatus::Complete);
        assert!(o
            .detections()
            .iter()
            .any(|d| d.rule_id == "TS_JWT_RSA_ALGS" && d.location.line == 2));
    }
}
#[test]
fn valid_rust_attributes_and_macro_token_trees_keep_algorithm_locations() {
    for source in ["struct Config { api: bool }\nfn f(config: Config) { let Config { #[cfg(feature = \"api\")] api } = config; let alg = \"RS256\"; }", "macro_rules! make { ($ name : ty) => { impl $name {} }; }\nconst ALG: &str = \"RS256\";", "macro_rules! punctuation { (~) => { 1 }; }\nconst ALG: &str = \"RS256\";", "unsafe extern \"C\" { fn f(); }\nconst ALG: &str = \"RS256\";"] {
  let o=ast("a.rs",source);assert_eq!(o.status(),DetectorStatus::Complete,"{source}");assert!(o.detections().iter().any(|d|d.rule_id=="TS_JWT_RSA_ALGS"&&d.location.line==2));
 }
}
#[test]
fn tls_versions_and_enum_symbols_are_not_misclassified_as_legacy_tls_or_suites() {
    for text in [
        "curl TLSv1_2",
        "const options = { minVersion: 'TLSv1.3' };",
        "const mode = TLS_PK_RSA_ALT;",
        "const mode = TLS_RSA_PSS_RSAE_SHA256;",
    ] {
        let o = RegexDetector
            .detect(
                &ScannableFile::from_bytes("a.js".into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert!(
            o.iter().all(|d| ![
                "TLS_VERSION_1_0",
                "TLS_VERSION_1_1",
                "NODE_TLS_MIN_VERSION",
                "TLS_RSA_CIPHER_SUITE",
                "TLS_RSA_KEY_EXCHANGE"
            ]
            .contains(&d.rule_id.as_str())),
            "{text}"
        );
    }
    for (text, id) in [
        ("const version = 'TLSv1.0';", "TLS_VERSION_1_0"),
        (
            "const suite = 'TLS_RSA_WITH_AES_128_GCM_SHA256';",
            "TLS_RSA_KEY_EXCHANGE",
        ),
    ] {
        assert!(RegexDetector
            .detect(
                &ScannableFile::from_bytes("a.js".into(), text.as_bytes().to_vec()),
                &rules()
            )
            .unwrap()
            .iter()
            .any(|d| d.rule_id == id));
    }
}
#[test]
fn python_rebound_or_unimported_receivers_are_not_resolved_as_crypto() {
    for source in ["class Fake:\n    def generate_private_key(self): pass\nrsa = Fake()\nk = rsa.generate_private_key()", "from cryptography.hazmat.primitives.asymmetric import rsa as r\nr = other\nk = r.generate_private_key()"] {
  assert!(ast("a.py",source).detections().iter().all(|d|d.rule_id!="TS_PY_RSA_USAGE"));
 }
}
#[test]
fn supported_dialects_and_module_attributes_keep_source_locations() {
    for (name,prefix) in [("a.js","// @flow\nimport React, { type Node } from 'react';\ntype Props = { children: Node };\n"),("a.mjs","import defer * as m from './data.json' with { type: 'json' };\n"),("a.js","import { import as other } from './module';\n"),("a.js","class C { accessor value = 1; }\n"),("a.js","for (await using x = resource; x.value < 2; ) { run(x); }\n"),("a.js","const result = do {\n  if (ok) { 1; } else { 2; }\n};\n")] {
  let text=format!("{prefix}const alg = 'RS256';");let o=ast(name,&text);assert_eq!(o.status(),DetectorStatus::Complete,"{name}: {prefix}");assert!(o.detections().iter().any(|d|d.rule_id=="TS_JWT_RSA_ALGS"&&d.location.line==prefix.lines().count()+1));
 }
}
#[test]
fn valid_private_fields_asi_and_tagged_template_types_are_supported() {
    for (name, prefix) in [
        ("a.js", "class C {\n  #abort\n  constructor() {}\n}\n"),
        (
            "a.js",
            "const f = () => { return 1 }\n\n    [...values].forEach(f);\n",
        ),
        (
            "a.tsx",
            "const tag = styled.div<{ animation: Keyframes }>`color: red`;\n",
        ),
        ("a.ts", "const x = list as (typeof type)[];\n"),
        ("a.js", "% OptimizeOsr();\n"),
    ] {
        let text = format!("{prefix}const alg='RS256';");
        let o = ast(name, &text);
        assert_eq!(o.status(), DetectorStatus::Complete, "{prefix}");
        assert!(o
            .detections()
            .iter()
            .any(|d| d.rule_id == "TS_JWT_RSA_ALGS"));
    }
}
#[test]
fn malformed_module_attributes_and_go_directives_do_not_complete() {
    for text in [
        "import * as m from './m' with { type: 'json' other: 'text' };",
        "import * as m from './m' with { type: 'json', type: 'text' };",
        "const x = <X prop={class = value} />;",
    ] {
        assert_eq!(
            ast("a.js", text).status(),
            DetectorStatus::Partial,
            "{text}"
        );
    }
    for text in [
        "module example.com/a\nrequire example.com/b v1.2.3\nreplace example.com/b => bad value\n",
        "module example.com/a\nrequire example.com/b v1.2.3\nexclude example.com/b vgarbage\n",
    ] {
        assert_ne!(
            dependency("go.mod", text).status(),
            DetectorStatus::Complete
        );
    }
}
#[test]
fn pqc_signature_oid_does_not_make_wrong_signature_shapes_complete() {
    use pqc_scan_detectors::certificate_detector::parse_certificate_der;
    let mut bytes = include_bytes!("fixtures/certificates/rsa2048.der").to_vec();
    let old = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 11];
    let new = [0x60, 0x86, 0x48, 1, 0x65, 3, 4, 3, 17];
    let offsets: Vec<_> = bytes
        .windows(old.len())
        .enumerate()
        .filter_map(|(i, b)| (b == old).then_some(i))
        .collect();
    assert_eq!(offsets.len(), 2);
    for i in offsets {
        bytes[i..i + 9].copy_from_slice(&new);
    }
    assert_ne!(
        parse_certificate_der(&bytes).unwrap().status,
        DetectorStatus::Complete
    );
}
#[test]
fn gemfile_static_loading_options_do_not_change_dependency_versions() {
    let o=dependency("Gemfile","source 'https://rubygems.org'\ngem 'jwt', '~> 2.0', require: false\ngem 'openssl', :require => false\n");
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|s| s == "jwt")));
}
#[test]
fn private_field_before_computed_field_uses_class_field_asi() {
    let o = ast(
        "a.js",
        "class C {\n  #aborted = false\n  #abort\n\n  [key] = null\n}\nconst alg='RS256';",
    );
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o
        .detections()
        .iter()
        .any(|d| d.rule_id == "TS_JWT_RSA_ALGS"));
}
#[test]
fn compatibility_work_limit_is_explicit_and_keeps_available_hits() {
    let source = "const x = vi.importOriginal<typeof import('module')>();\nconst alg = 'RS256';";
    let f = ScannableFile::from_bytes("a.ts".into(), source.as_bytes().to_vec());
    let o = TreeSitterDetector::with_budget(pqc_scan_detectors::DetectorBudget {
        max_ast_match_bytes: 1000,
        ..Default::default()
    })
    .detect_detailed(&f, &rules())
    .unwrap();
    assert_eq!(o.status(), DetectorStatus::Truncated);
    assert!(o
        .detections()
        .iter()
        .any(|d| d.rule_id == "TS_JWT_RSA_ALGS"));
}
#[test]
fn rust_compatibility_does_not_accept_bad_macro_or_missing_syntax() {
    for source in [
        "macro_rules! f { ($arg:ident) => { $123 }; }\nconst A:&str=\"RS256\";",
        "fn f( {\nlet alg=\"RS256\";",
    ] {
        assert_eq!(ast("a.rs", source).status(), DetectorStatus::Partial);
    }
}
#[test]
fn python_import_forms_and_unicode_aliases_preserve_real_call_spans() {
    for source in ["import cryptography.hazmat.primitives.asymmetric.rsa as r\nkey = r.generate_private_key(public_exponent=65537, key_size=4096)", "from cryptography.hazmat.primitives.asymmetric import rsa as 暗号\nkey = 暗号 . generate_private_key(public_exponent=65537, key_size=4096)"] {let o=ast("a.py",source);let d=o.detections().iter().find(|d|d.rule_id=="TS_PY_RSA_USAGE").expect("RSA call");assert_eq!(d.location.line,2);assert_eq!(d.location.column,7);assert!(d.evidence.r#match.starts_with("r.genera") || d.evidence.r#match.starts_with("暗号 . gen"));
        assert!(d.evidence.r#match.ends_with("te_key"));
        assert_eq!(d.evidence.metadata["resolution_method"],"static_import_binding");}
}
#[test]
fn python_function_local_imports_do_not_bind_other_functions() {
    let source="def one():\n    from cryptography.hazmat.primitives.asymmetric import rsa as r\n\ndef two():\n    return r.generate_private_key(public_exponent=65537,key_size=4096)";
    assert!(ast("a.py", source)
        .detections()
        .iter()
        .all(|d| d.rule_id != "TS_PY_RSA_USAGE"));
    let source="def one():\n    from cryptography.hazmat.primitives.asymmetric import rsa as r\n    return r.generate_private_key(public_exponent=65537,key_size=4096)";
    has("a.py", source, "TS_PY_RSA_USAGE");
}
#[test]
fn python_import_shadowing_and_parameter_annotations_are_distinct() {
    let shadow="from cryptography.hazmat.primitives.asymmetric import rsa as r\nfrom unrelated import value as r\nk = r.generate_private_key()";
    assert!(ast("a.py", shadow)
        .detections()
        .iter()
        .all(|d| d.rule_id != "TS_PY_RSA_USAGE"));
    has("a.py","from cryptography.hazmat.primitives.asymmetric import rsa as r\ndef make(key: r.RSAPrivateKey):\n    return r.generate_private_key(public_exponent=65537,key_size=4096)","TS_PY_RSA_USAGE");
}
#[test]
fn irrelevant_go_strings_do_not_exhaust_the_import_candidate_budget() {
    let text = format!(
        "package p\nimport \"crypto/rsa\"\nvar paths = []string{{ {} }}",
        "\"crypto/rsa\",".repeat(10_000)
    );
    let o = RegexDetector::with_budget(pqc_scan_detectors::DetectorBudget {
        max_match_candidates: 2,
        ..Default::default()
    })
    .detect_detailed(
        &ScannableFile::from_bytes("a.go".into(), text.into_bytes()),
        &rules(),
    )
    .unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert_eq!(
        o.detections()
            .iter()
            .filter(|d| matches!(
                d.rule_id.as_str(),
                "GO_TLS_RSA_IMPORT" | "API_GO_CRYPTO_RSA"
            ))
            .count(),
        2
    );
}
#[test]
fn python_default_argument_calls_use_definition_scope_instead_of_parameter_bindings() {
    for source in ["from cryptography.hazmat.primitives.asymmetric import rsa as r\ndef make(r=r.generate_private_key(public_exponent=65537,key_size=4096)):\n    return r", "from cryptography.hazmat.primitives.asymmetric import rsa as r\nmake = lambda r=r.generate_private_key(public_exponent=65537,key_size=4096): r"] {has("a.py",source,"TS_PY_RSA_USAGE");}
    let source="from cryptography.hazmat.primitives.asymmetric import rsa as r\ndef make(r):\n    return r.generate_private_key(public_exponent=65537,key_size=4096)";
    assert!(ast("a.py", source)
        .detections()
        .iter()
        .all(|d| d.rule_id != "TS_PY_RSA_USAGE"));
}

#[test]
fn go_expression_new_uses_lexical_strings_when_error_tree_mispairs_quotes() {
    let text = "package p\nvar x = []Entry{\n{PtrName:new(\"a\"),PtrPort:new(int32(8080)),Name:\"n\"},\n{PtrName:new(\"a\"),PtrPort:new(int32(8080)),Name:\"n\"},\n}\nvar alg = \"RS256\"\n";
    has("a.go", text, "TS_JWT_RSA_ALGS");
    assert_eq!(
        ast("a.go", "package p\nvar x = new(\"unterminated)\n").status(),
        DetectorStatus::Partial
    );
    let literal = ast("a.go", "package p\nvar x = `new(int32(8080))`\n");
    assert_eq!(literal.status(), DetectorStatus::Complete);
}

#[test]
fn ruby_endless_raise_string_is_valid_but_unterminated_string_is_partial() {
    has(
        "a.rb",
        "class A\n  def index = raise \"missing\"\nend\nalg = 'RS256'\n",
        "TS_JWT_RSA_ALGS",
    );
    assert_eq!(
        ast("a.rb", "class A\n def index = raise \"missing\nend\n").status(),
        DetectorStatus::Partial
    );
}

#[test]
fn go_retract_requires_versions_or_a_closed_two_version_interval() {
    for body in [
        "banana",
        "[v1.0.0,]",
        "[,v1.2.3]",
        "[v1.0.0,v2.0.0,v3.0.0]",
        "v1.2.3.4",
        "[v1.0.0,v2.0.0",
        "v01.2.3",
        "v1.2.3-01",
    ] {
        let source =
            format!("module example.org/app\nrequire example.org/lib v1.2.3\nretract {body}\n");
        assert_eq!(
            dependency("go.mod", &source).status(),
            DetectorStatus::Partial,
            "{body}"
        );
    }
    for body in [
        "v1.2.3",
        "v1.2.3-rc.1",
        "[v1.0.0, v1.2.3]",
        "[v1.2.3-0.20260101010101-abcdef012345,v1.2.3]",
    ] {
        assert_eq!(
            dependency(
                "go.mod",
                &format!("module example.org/app\nretract {body}\n")
            )
            .status(),
            DetectorStatus::Complete,
            "{body}"
        );
    }
}

#[test]
fn ast_tls_versions_have_the_same_boundaries_as_regex_detection() {
    for version in [
        "TLSv1_2", "TLSv1_3", "TLSv1.2", "TLSv1.3", "TLSv1_10", "TLSv1.10",
    ] {
        let source = format!("const version = \"{version}\";");
        let o = ast("a.js", &source);
        assert_eq!(o.status(), DetectorStatus::Complete);
        assert!(
            o.detections()
                .iter()
                .all(|d| d.rule_id != "TS_TLS_LEGACY_VERSION_LITERAL"),
            "{version}"
        );
    }
    for version in ["TLSv1_0", "TLSv1_1", "TLSv1.0", "TLSv1.1", "TLSv1"] {
        has(
            "a.js",
            &format!("const version = \"{version}\";"),
            "TS_TLS_LEGACY_VERSION_LITERAL",
        );
    }
}

fn der_fields(encoded: &[u8]) -> Vec<Vec<u8>> {
    fn header(bytes: &[u8]) -> (usize, usize) {
        if bytes[1] < 128 {
            (2, usize::from(bytes[1]))
        } else {
            let n = usize::from(bytes[1] & 127);
            let size = bytes[2..2 + n]
                .iter()
                .fold(0usize, |v, b| (v << 8) | usize::from(*b));
            (2 + n, size)
        }
    }
    let (h, n) = header(encoded);
    let mut rest = &encoded[h..h + n];
    let mut result = Vec::new();
    while !rest.is_empty() {
        let (h, n) = header(rest);
        result.push(rest[..h + n].to_vec());
        rest = &rest[h + n..];
    }
    result
}
fn pqc_signed_certificate(oid: u8, signature_size: usize, parameters: &[u8]) -> Vec<u8> {
    let mut fields = der_fields(include_bytes!("fixtures/certificates/rsa2048.der"));
    let mut tbs = der_fields(&fields[0]);
    let mut oid_bytes = vec![0x60, 0x86, 0x48, 1, 0x65, 3, 4, 3];
    oid_bytes.push(oid);
    let mut alg = der(6, &oid_bytes);
    alg.extend(parameters);
    let alg = der(0x30, &alg);
    assert_eq!(tbs[0][0], 0xa0);
    tbs[2] = alg.clone();
    fields[0] = der(0x30, &tbs.concat());
    fields[1] = alg;
    let mut bits = vec![0];
    bits.resize(signature_size + 1, 0x5a);
    fields[2] = der(3, &bits);
    der(0x30, &fields.concat())
}
#[test]
fn pqc_certificate_signature_shapes_have_positive_and_adjacent_negative_controls() {
    use pqc_scan_detectors::certificate_detector::parse_certificate_der;
    // RFC9881 ML-DSA and RFC9909 SLH/HashSLH signature encodings; no trust validation claim.
    for (oid, size) in [(17, 2420), (18, 3309), (19, 4627)]
        .into_iter()
        .chain((20..=43).map(|oid| {
            (
                oid,
                [7856, 17088, 16224, 35664, 29792, 49856][usize::from((oid - 20) % 6)],
            )
        }))
    {
        assert_eq!(
            parse_certificate_der(&pqc_signed_certificate(oid, size, &[]))
                .unwrap()
                .status,
            DetectorStatus::Complete,
            "{oid}"
        );
        assert_eq!(
            parse_certificate_der(&pqc_signed_certificate(oid, size - 1, &[]))
                .unwrap()
                .status,
            DetectorStatus::Partial,
            "short {oid}"
        );
        assert_eq!(
            parse_certificate_der(&pqc_signed_certificate(oid, size, &[5, 0]))
                .unwrap()
                .status,
            DetectorStatus::Partial,
            "params {oid}"
        );
    }
}
#[test]
fn all_supported_pqc_public_key_oids_have_shape_controls() {
    use pqc_scan_detectors::certificate_detector::parse_public_key_der;
    for oid in 17..=31 {
        let size = if oid < 20 {
            [1312, 1952, 2592][usize::from(oid - 17)]
        } else {
            [32, 32, 48, 48, 64, 64][usize::from((oid - 20) % 6)]
        };
        let a = parse_public_key_der(&pqc_spki(oid, size, &[], 0)).unwrap();
        assert_eq!(a.status, DetectorStatus::Complete, "{oid}");
        assert_eq!(a.key_bits, None);
        assert_ne!(
            parse_public_key_der(&pqc_spki(oid, size - 1, &[], 0))
                .unwrap()
                .status,
            DetectorStatus::Complete,
            "{oid}"
        );
    }
}
#[test]
fn non_certificate_der_formats_remain_inapplicable_without_exposing_private_content() {
    use pqc_scan_detectors::certificate_detector::CertificateDetector;
    let rsa_public = der(0x30, &[der(2, &[17]), der(2, &[3])].concat());
    let encrypted = der(
        0x30,
        &[
            der(0x30, &der(6, &[0x2a, 3, 4])),
            der(4, b"SECRET_DER_CANARY"),
        ]
        .concat(),
    );
    for bytes in [rsa_public, encrypted, pqc_spki(17, 1312, &[], 0)] {
        let o = CertificateDetector
            .detect_observed(
                &ScannableFile::from_bytes("object.der".into(), bytes),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::NotApplicable);
        assert!(o.detections().is_empty());
        assert!(!format!("{o:?}").contains("SECRET_DER_CANARY"));
    }
    for bytes in [
        der(0x30, &[der(2, &[17]), der(2, &[3]), der(2, &[1])].concat()),
        der(0x30, &der(4, b"not a certificate")),
    ] {
        let o = CertificateDetector
            .detect_detailed(
                &ScannableFile::from_bytes("bad.der".into(), bytes),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial);
    }
}
