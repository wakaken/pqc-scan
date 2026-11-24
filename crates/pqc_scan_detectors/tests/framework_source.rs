//! Reduced source-flow controls motivated by Rails and Vector. Actual pinned
//! repository source is separately scanned by the OSS acceptance measurement.
use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
use pqc_scan_detectors::default_detectors;
use pqc_scan_rules::RuleSet;
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
fn scan(ext: &str, text: &str) -> Value {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "pqc-framework-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join(format!("sample.{ext}")), text).unwrap();
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let scan = scan_repository_native(
        &ScanConfig {
            root: root.clone(),
            threads: 1,
            ..Default::default()
        },
        &rules,
        &default_detectors(),
        &NativeOptions::new(Some("source-flow")).unwrap(),
    )
    .unwrap();
    let value = serde_json::to_value(scan.native().unwrap()).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    value
}
fn hits<'a>(scan: &'a Value, rule: &str) -> Vec<&'a Value> {
    scan["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["rule_id"] == rule)
        .collect()
}
#[test]
fn ruby_follows_method_branch_and_instance_variable_candidates_without_claiming_runtime_use() {
    let text = r#"class Envelope
      class << self
        def preferred
          if authenticated
            "aes-256-gcm"
          else
            "aes-256-cbc"
          end
        end
      end
      def initialize(options)
        @cipher = options[:cipher] || self.class.preferred
      end
      def new_cipher
        OpenSSL::Cipher.new(@cipher)
      end
    end"#;
    let s = scan("rb", text);
    let h = hits(&s, "API_RUBY_OPENSSL_CIPHER");
    assert_eq!(h.len(), 1);
    assert_eq!(h[0]["usage"], "referenced");
    assert_eq!(h[0]["assurance"], "syntactic");
    let algorithms: Vec<_> = s["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "algorithm")
        .collect();
    assert_eq!(algorithms.len(), 2, "{s}");
    for a in algorithms {
        assert_eq!(a["attributes"]["identifier"], "AES-256");
    }
    assert_eq!(h[0]["target"]["line"], 15);
}

#[test]
fn ruby_literal_cipher_constant_is_a_same_class_candidate_not_runtime_proof() {
    let s = scan("rb", "class Envelope; CIPHER = 'aes-256-gcm'; class << self; def key_length; OpenSSL::Cipher.new(CIPHER).key_len; end; end; def encrypt; OpenSSL::Cipher.new(CIPHER); end; end");
    assert_eq!(hits(&s, "API_RUBY_OPENSSL_CIPHER").len(), 2);
    let algorithms: Vec<_> = s["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["kind"] == "algorithm")
        .collect();
    assert_eq!(algorithms.len(), 1);
    assert_eq!(algorithms[0]["attributes"]["identifier"], "AES-256");
    assert_eq!(algorithms[0]["attributes"]["parameters"]["mode"], "GCM");
    assert!(hits(&s, "API_RUBY_OPENSSL_CIPHER")
        .iter()
        .all(|o| o["usage"] == "referenced"));
    for text in [
        "class A; CIPHER = 'aes-256-gcm'; end; class B; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER = supplied; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = supplied; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; def go; OpenSSL::Cipher.new(CIPHER); end; end; A::CIPHER = supplied",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER ||= supplied; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; const_set(:CIPHER, supplied); def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; send(:const_set, :CIPHER, supplied); def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; class << self; CIPHER = 'aes-256-gcm'; end; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER, OTHER = supplied, other; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER.replace(supplied); def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER.concat(supplied); def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER << supplied; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; CIPHER[0] = supplied; def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; other = CIPHER; other.replace(supplied); def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; external(CIPHER); def go; OpenSSL::Cipher.new(CIPHER); end; end",
        "class A; CIPHER = 'aes-256-gcm'; box = [CIPHER]; box[0].replace(supplied); def go; OpenSSL::Cipher.new(CIPHER); end; end",
    ] {
        let s = scan("rb", text);
        assert_eq!(hits(&s, "API_RUBY_OPENSSL_CIPHER").len(), 1);
        assert!(!s["assets"].as_array().unwrap().iter().any(|a| a["kind"] == "algorithm"), "{text}");
    }
}
#[test]
fn ruby_does_not_transfer_values_between_classes_or_evaluate_dynamic_arguments() {
    for text in [
        "class A; def initialize; @cipher = 'aes-256-gcm'; end; end\nclass B; def go; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; def go; cipher = 'aes-256-gcm'; cipher = supplied; OpenSSL::Cipher.new(cipher); end; end",
        "OpenSSL::Cipher.new(ENV['CIPHER'])",
        "class A; def self.seed; @cipher='aes-256-gcm'; end; def go; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; def initialize; @cipher='aes-256-gcm'; end; def self.go; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; @cipher='aes-256-gcm'; def go; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; def self.preferred; 'aes-256-gcm'; end; def self.go; OpenSSL::Cipher.new(self.class.preferred); end; end",
        "class A; def go(items); cipher='aes-256-gcm'; items.each do |cipher|; OpenSSL::Cipher.new(cipher); end; end; end",
        "class A; def go(items); cipher='aes-256-gcm'; items.each { |cipher| OpenSSL::Cipher.new(cipher) }; end; end",
        "class A; def go(items); cipher='aes-256-gcm'; for cipher in items; OpenSSL::Cipher.new(cipher); end; end; end",
        "class A; def go(obj); obj.instance_eval { @cipher='aes-256-gcm' }; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; def go(items); items.each { |cipher| cipher='aes-256-gcm' }; OpenSSL::Cipher.new(cipher); end; end",
        "class A; def go(obj); obj.instance_eval { def self.preferred; 'aes-256-gcm'; end }; OpenSSL::Cipher.new(self.class.preferred); end; end",
        "class A; def go(obj); obj.instance_eval { def seed; @cipher='aes-256-gcm'; end }; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; class << Object.new; @cipher='aes-256-gcm'; end; def self.go; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; class << self; @cipher='aes-256-gcm'; def go; OpenSSL::Cipher.new(@cipher); end; end; end",
        "class A; def self.seed; @cipher='aes-256-gcm'; end; class << self; OpenSSL::Cipher.new(@cipher); end; end",
        "class A; def cipher; 'aes-256-gcm'; end; def go(cipher); OpenSSL::Cipher.new(cipher); end; end",
        "OpenSSL::Cipher.new(\"aes-#{supplied}-gcm\")",
    ] {
        let s = scan("rb", text);
        assert_eq!(hits(&s, "API_RUBY_OPENSSL_CIPHER").len(), 1, "{text}");
        assert!(!s["assets"].as_array().unwrap().iter().any(|a| a["kind"] == "algorithm"), "{s}");
    }
    for text in [
        "# OpenSSL::Cipher.new('aes-256-gcm')",
        "text = \"OpenSSL::Cipher.new('aes-256-gcm')\"",
        "module OpenSSL; class Cipher; end; end\nOpenSSL::Cipher.new('aes-256-gcm')",
    ] {
        assert!(
            hits(&scan("rb", text), "API_RUBY_OPENSSL_CIPHER").is_empty(),
            "{text}"
        );
    }
}
#[test]
fn rust_openssl_import_alias_and_nested_use_detect_tls_without_inventing_negotiated_crypto() {
    for text in [
        "use openssl::ssl::{SslConnector, SslMethod}; fn wrapper() { SslConnector::builder(SslMethod::tls()); }",
        "use openssl::{ssl::{SslAcceptor as Acceptor, SslMethod as Method}}; fn other() { Acceptor::mozilla_intermediate(Method::tls()); }",
        "fn setup() { openssl::ssl::SslConnector::builder(openssl::ssl::SslMethod::tls_client()); }",
        "use ::openssl::ssl::{SslConnector, SslMethod as Method}; fn setup() { SslConnector::builder(Method::tls()); }",
        "mod openssl {} use ::openssl::ssl::{SslConnector, SslMethod as Method}; fn setup() { SslConnector::builder(Method::tls()); }",
        "use openssl::ssl as tls; fn wrapper() { tls::SslConnector::builder(tls::SslMethod::tls()); }",
        "use openssl::{ssl}; fn wrapper() { ssl::SslConnector::builder(ssl::SslMethod::tls()); }",
        "use openssl as native; fn wrapper() { native::ssl::SslConnector::builder(native::ssl::SslMethod::tls()); }",
    ] {
        let s = scan("rs", text);
        assert_eq!(hits(&s, "API_RUST_OPENSSL_TLS").len(), 2, "{text}: {s}");
        assert!(!s["assets"].as_array().unwrap().iter().any(|a| a["kind"] == "algorithm"));
    }
    for text in [
        "mod openssl {} fn f() { openssl::ssl::SslMethod::tls(); }",
        "struct SslMethod; fn f() { SslMethod::tls(); }",
        "use unrelated::SslMethod; fn f() { SslMethod::tls(); }",
        "use openssl::ssl::SslMethod; fn f() { type SslMethod = Other; SslMethod::tls(); }",
        "use openssl::ssl::SslMethod; fn f<SslMethod>() { SslMethod::tls(); }",
        "// openssl::ssl::SslMethod::tls()",
        "use unrelated::ssl as tls; fn f() { tls::SslMethod::tls(); }",
        "use openssl::ssl as tls; fn f<tls>() { tls::SslMethod::tls(); }",
        "use openssl::ssl as tls; fn f() { mod tls {} tls::SslMethod::tls(); }",
        "use openssl::ssl as tls; fn f() { ::tls::SslMethod::tls(); }",
    ] {
        assert!(
            hits(&scan("rs", text), "API_RUST_OPENSSL_TLS").is_empty(),
            "{text}"
        );
    }
}
#[test]
fn source_flow_caps_and_cycles_remain_visible_and_secret_arguments_are_not_native_facts() {
    use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
    use pqc_scan_detectors::{tree_sitter_detector::TreeSitterDetector, DetectorBudget};
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    let text = "class Envelope; def preferred; preferred; end; def go; OpenSSL::Cipher.new(preferred); end; end";
    let file = ScannableFile::from_bytes(PathBuf::from("cycle.rb"), text.as_bytes().to_vec());
    let outcome = TreeSitterDetector.detect_detailed(&file, &rules).unwrap();
    assert_eq!(outcome.status(), DetectorStatus::Truncated);
    let finite = ScannableFile::from_bytes(
        PathBuf::from("finite.rb"),
        b"OpenSSL::Cipher.new('aes-256-gcm')".to_vec(),
    );
    let complete = TreeSitterDetector.detect_detailed(&finite, &rules).unwrap();
    assert_eq!(complete.status(), DetectorStatus::Complete);
    assert_eq!(
        complete
            .detections()
            .iter()
            .filter(|d| d.rule_id == "API_RUBY_OPENSSL_CIPHER")
            .count(),
        1
    );
    let limited = TreeSitterDetector::with_budget(DetectorBudget {
        max_ast_nodes: 4,
        ..Default::default()
    })
    .detect_detailed(&finite, &rules)
    .unwrap();
    assert_eq!(limited.status(), DetectorStatus::Truncated);
    let s = scan(
        "rb",
        "OpenSSL::Cipher.new('PRIVATE_CIPHER_CANARY_NOT_AN_ALGORITHM')",
    );
    assert!(!serde_json::to_string(&s)
        .unwrap()
        .contains("PRIVATE_CIPHER_CANARY"));
    assert!(!s["assets"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["kind"] == "algorithm"));
    let s = scan("rb", "OpenSSL::Cipher.new('aes-128-cbc')");
    let a = s["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["kind"] == "algorithm")
        .unwrap();
    assert_eq!(a["attributes"]["identifier"], "AES-128");
    assert_eq!(a["attributes"]["parameters"]["key_bits"], "128");
    assert_eq!(a["attributes"]["parameters"]["mode"], "CBC");
}

#[test]
fn ordinary_deep_syntax_is_not_mistaken_for_recursive_value_resolution() {
    use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
    use pqc_scan_detectors::{tree_sitter_detector::TreeSitterDetector, DetectorBudget};
    let rules = RuleSet::load_from_dir(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
    )
    .unwrap();
    // Reduced from Keycloak's fluent configuration builder and event-type union.
    for (name, text) in [
        ("Builder.java", format!("class Builder {{ void configure() {{ Factory.create(){}.finish(); String alg = \"RS256\"; }} }}", ".property().name(\"example\")".repeat(90))),
        ("types.ts", format!("type EventType = {}; const alg = 'RS256';", (0..180).map(|i| format!("'EVENT_{i}'")).collect::<Vec<_>>().join(" | "))),
        ("ops.rs", format!("fn compute() {{ let value = {}; let alg = \"RS256\"; }}", std::iter::repeat_n("1", 180).collect::<Vec<_>>().join(" + "))),
    ] {
        let file = ScannableFile::from_bytes(PathBuf::from(name), text.into_bytes());
        let result = TreeSitterDetector.detect_detailed(&file, &rules).unwrap();
        assert_eq!(result.status(), DetectorStatus::Complete, "{name}");
        assert!(result.detections().iter().any(|d| d.rule_id == "TS_JWT_RSA_ALGS"), "{name}");
        let limited = TreeSitterDetector::with_budget(DetectorBudget { max_ast_nodes: 8, ..Default::default() })
            .detect_detailed(&file, &rules).unwrap();
        assert_eq!(limited.status(), DetectorStatus::Truncated, "{name}");
    }
}
