//! YAML owns rule definitions; this registry owns only implemented analysis handlers.
use pqc_scan_rules::Rule;
use std::{collections::BTreeMap, sync::LazyLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Handler {
    JavaPqc,
    NodePqc,
    PythonPqc,
    RustPqc,
    NodeRsa,
    NodeRsaSign,
    RubyCipher,
    RustTls,
    NodeJose,
    PhpClassical,
    PhpPqc,
    PhpReference,
    CSharpClassical,
    CSharpPqc,
    CSharpReference,
    CFamilyClassical,
    CFamilyPqc,
    CFamilyTls,
    ImplementationReference,
    ConstructionReference,
}
impl Handler {
    pub const ALL: [Self; 20] = [
        Self::JavaPqc,
        Self::NodePqc,
        Self::PythonPqc,
        Self::RustPqc,
        Self::NodeRsa,
        Self::NodeRsaSign,
        Self::RubyCipher,
        Self::RustTls,
        Self::NodeJose,
        Self::PhpClassical,
        Self::PhpPqc,
        Self::PhpReference,
        Self::CSharpClassical,
        Self::CSharpPqc,
        Self::CSharpReference,
        Self::CFamilyClassical,
        Self::CFamilyPqc,
        Self::CFamilyTls,
        Self::ImplementationReference,
        Self::ConstructionReference,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Self::JavaPqc => "PQC_JAVA_API",
            Self::NodePqc => "PQC_NODE_API",
            Self::PythonPqc => "PQC_PYTHON_API",
            Self::RustPqc => "PQC_RUST_API",
            Self::NodeRsa => "TS_NODE_RSA_USAGE",
            Self::NodeRsaSign => "API_NODE_CRYPTO_SIGN_RSA",
            Self::RubyCipher => "API_RUBY_OPENSSL_CIPHER",
            Self::RustTls => "API_RUST_OPENSSL_TLS",
            Self::NodeJose => "TS_NODE_JOSE_USAGE",
            Self::PhpClassical => "PHP_CRYPTO_API",
            Self::PhpPqc => "PHP_PQC_API",
            Self::PhpReference => "PHP_CRYPTO_REFERENCE",
            Self::CSharpClassical => "CSHARP_CRYPTO_API",
            Self::CSharpPqc => "CSHARP_PQC_API",
            Self::CSharpReference => "CSHARP_CRYPTO_REFERENCE",
            Self::CFamilyClassical => "C_FAMILY_CRYPTO_API",
            Self::CFamilyPqc => "C_FAMILY_PQC_API",
            Self::CFamilyTls => "C_FAMILY_TLS_REFERENCE",
            Self::ImplementationReference => "SOURCE_IMPLEMENTATION_REFERENCE",
            Self::ConstructionReference => "SOURCE_CONSTRUCTION_REFERENCE",
        }
    }
    pub fn handles(self, language: &str) -> bool {
        match self {
            Self::ImplementationReference => matches!(language, "c" | "cpp"),
            Self::ConstructionReference => matches!(language, "c" | "cpp" | "csharp"),
            Self::JavaPqc => language == "java",
            Self::PythonPqc => language == "python",
            Self::RustPqc => language == "rust",
            Self::RustTls => language == "rust",
            Self::RubyCipher => language == "ruby",
            Self::PhpClassical | Self::PhpPqc | Self::PhpReference => language == "php",
            Self::CSharpClassical | Self::CSharpPqc | Self::CSharpReference => language == "csharp",
            Self::CFamilyClassical | Self::CFamilyPqc | Self::CFamilyTls => {
                matches!(language, "c" | "cpp")
            }
            Self::NodePqc | Self::NodeRsa | Self::NodeRsaSign | Self::NodeJose => {
                matches!(language, "javascript" | "typescript")
            }
        }
    }
    pub fn replaces_generic(self) -> bool {
        !matches!(self, Self::NodeRsa | Self::NodeRsaSign | Self::NodeJose)
    }
}

// Build inputs, not a runtime read of mutable repository paths. A YAML edit
// recompiles this crate; no copied pattern/scope/kind literals need updating.
const SOURCES: [&str; 7] = [
    include_str!("../../../../rules/default/pqc.yml"),
    include_str!("../../../../rules/default/tree_sitter.yml"),
    include_str!("../../../../rules/default/crypto_api.yml"),
    include_str!("../../../../rules/default/php.yml"),
    include_str!("../../../../rules/default/csharp.yml"),
    include_str!("../../../../rules/default/c_family.yml"),
    include_str!("../../../../rules/default/source_references.yml"),
];
static DEFAULTS: LazyLock<BTreeMap<String, Rule>> = LazyLock::new(|| {
    let mut rules = BTreeMap::new();
    for yaml in SOURCES {
        let parsed: Vec<Rule> =
            serde_yaml::from_str(yaml).expect("valid compiled-in default rule YAML");
        for rule in parsed {
            if Handler::ALL.iter().any(|h| h.id() == rule.id) {
                assert!(
                    rules.insert(rule.id.clone(), rule).is_none(),
                    "unique compiled-in source rule"
                );
            }
        }
    }
    rules
});
fn same_definition(rule: &Rule, reference: &Rule) -> bool {
    rule.id == reference.id
        && rule.pattern == reference.pattern
        && rule.scope == reference.scope
        && rule.kind == reference.kind
}
pub(super) fn handler(rule: &Rule) -> Option<Handler> {
    let kind = Handler::ALL.into_iter().find(|h| h.id() == rule.id)?;
    same_definition(rule, DEFAULTS.get(&rule.id)?).then_some(kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn installed_rule_copies_bind_but_modified_semantics_do_not() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default");
        let rules = pqc_scan_rules::RuleSet::load_from_dir(&root).unwrap();
        for expected in Handler::ALL {
            let original = rules.get(expected.id()).unwrap();
            assert_eq!(handler(original), Some(expected));
            for field in ["pattern", "scope", "kind", "id"] {
                let mut custom = original.clone();
                match field {
                    "pattern" => custom.pattern = "CONTROL".into(),
                    "scope" => custom.scope = "custom".into(),
                    "kind" => {
                        custom.kind = if original.kind == pqc_scan_rules::RuleKind::Regex {
                            pqc_scan_rules::RuleKind::TreeSitter
                        } else {
                            pqc_scan_rules::RuleKind::Regex
                        }
                    }
                    _ => custom.id = "CUSTOM".into(),
                }
                assert_eq!(handler(&custom), None, "{} {field}", original.id);
            }
            let mut presentation = original.clone();
            presentation.description = Some("Local wording".into());
            assert_eq!(handler(&presentation), Some(expected));
        }
    }
}
