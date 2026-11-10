use pqc_scan_core::{Detector, ScannableFile};
use pqc_scan_detectors::{
    certificate_detector::CertificateDetector, dependency_detector::DependencyDetector,
    key_detector::KeyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use std::{fs, path::PathBuf};
struct Pack(PathBuf);
impl Pack {
    fn new(kind: &str, scope: &str, pattern: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "pqc028-scope-{}-{kind}-{:x}",
            std::process::id(),
            scope
                .bytes()
                .fold(0u64, |n, b| n.wrapping_mul(31).wrapping_add(b as u64))
        ));
        fs::create_dir_all(&root).unwrap();
        let data = serde_json::json!({"id":"SCOPE_CONTROL","kind":kind,"category":"control","severity":"low","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":pattern,"scope":scope});
        fs::write(root.join("rule.yml"), data.to_string()).unwrap();
        Self(root)
    }
    fn rules(&self) -> RuleSet {
        RuleSet::load_from_dir(&self.0).unwrap()
    }
}
impl Drop for Pack {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn hit(detector: &dyn Detector, pack: &Pack, name: &str, text: &str) -> bool {
    detector
        .detect(
            &ScannableFile::from_bytes(PathBuf::from(name), text.as_bytes().to_vec()),
            &pack.rules(),
        )
        .unwrap()
        .iter()
        .any(|d| d.rule_id == "SCOPE_CONTROL")
}
#[test]
fn regex_scope_is_legacy_metadata_not_a_language_or_file_filter() {
    for scope in ["code", "config", "python", "CUSTOM", "", "*"] {
        let p = Pack::new("regex", scope, "CONTROL");
        assert!(
            hit(&RegexDetector, &p, "app.js", "const value = 'CONTROL';"),
            "{scope}"
        );
    }
}
#[test]
fn ast_scopes_preserve_lists_case_wildcards_javascript_alias_and_unknown_tokens() {
    for (scope, expected) in [
        ("javascript", true),
        ("typescript", true),
        ("RuSt|JaVaScRiPt; python", true),
        ("*", true),
        ("aNy", true),
        (" any ", false),
        ("CUSTOM", false),
        ("", false),
        ("python", false),
    ] {
        let p = Pack::new("tree_sitter", scope, "CONTROL");
        assert_eq!(
            hit(
                &TreeSitterDetector,
                &p,
                "app.ts",
                "const value = 'CONTROL';"
            ),
            expected,
            "{scope}"
        );
    }
}
#[test]
fn certificate_unknown_scopes_keep_path_fallback_but_typed_scopes_do_not() {
    for (scope, expected) in [
        ("certificate", true),
        ("CUSTOM", true),
        ("", true),
        ("signature_oid", false),
        ("public_key_oid", false),
        ("rsa_modulus_bits", false),
        ("certificate_validity", false),
        ("legacy_not_after_2026", false),
        ("SIGNATURE_OID", true),
    ] {
        let p = Pack::new("certificate", scope, r"fixture\.pem");
        assert_eq!(
            hit(&CertificateDetector, &p, "fixture.pem", "not a certificate"),
            expected,
            "{scope}"
        );
    }
}
#[test]
fn key_and_dependency_custom_scopes_remain_nonfiltering() {
    for scope in ["CUSTOM", "", "keymaterial", "dependency"] {
        let p = Pack::new("key", scope, "BEGIN PRIVATE KEY");
        assert!(hit(
            &KeyDetector::default(),
            &p,
            "marker.txt",
            "-----BEGIN PRIVATE KEY-----"
        ));
        let p = Pack::new("dependency", scope, "openssl");
        assert!(hit(
            &DependencyDetector,
            &p,
            "package.json",
            r#"{"dependencies":{"openssl":"^1.2.3"}}"#
        ));
    }
}
