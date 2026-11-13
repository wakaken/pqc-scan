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
}
impl Handler {
    pub const ALL: [Self; 6] = [
        Self::JavaPqc,
        Self::NodePqc,
        Self::PythonPqc,
        Self::RustPqc,
        Self::NodeRsa,
        Self::NodeRsaSign,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Self::JavaPqc => "PQC_JAVA_API",
            Self::NodePqc => "PQC_NODE_API",
            Self::PythonPqc => "PQC_PYTHON_API",
            Self::RustPqc => "PQC_RUST_API",
            Self::NodeRsa => "TS_NODE_RSA_USAGE",
            Self::NodeRsaSign => "API_NODE_CRYPTO_SIGN_RSA",
        }
    }
    pub fn handles(self, language: &str) -> bool {
        match self {
            Self::JavaPqc => language == "java",
            Self::PythonPqc => language == "python",
            Self::RustPqc => language == "rust",
            Self::NodePqc | Self::NodeRsa | Self::NodeRsaSign => {
                matches!(language, "javascript" | "typescript")
            }
        }
    }
    pub fn replaces_generic(self) -> bool {
        !matches!(self, Self::NodeRsa | Self::NodeRsaSign)
    }
}

// Build inputs, not a runtime read of mutable repository paths. A YAML edit
// recompiles this crate; no copied pattern/scope/kind literals need updating.
const SOURCES: [&str; 3] = [
    include_str!("../../../../rules/default/pqc.yml"),
    include_str!("../../../../rules/default/tree_sitter.yml"),
    include_str!("../../../../rules/default/crypto_api.yml"),
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
