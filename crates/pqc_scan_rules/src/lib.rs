use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use anyhow::{Context, Result};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum RuleKind {
    Regex,
    #[serde(rename = "tree_sitter", alias = "treesitter")]
    TreeSitter,
    Dependency,
    Certificate,
    Key,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Risk {
    QuantumVulnerable,
    QuantumUncertain,
    QuantumSafe,
    NonQuantumRisk,
}

impl Risk {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QuantumVulnerable => "quantum-vulnerable",
            Self::QuantumUncertain => "quantum-uncertain",
            Self::QuantumSafe => "quantum-safe",
            Self::NonQuantumRisk => "non-quantum-risk",
        }
    }
}

impl fmt::Display for Risk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub kind: RuleKind,
    pub category: String,
    pub severity: Severity,
    pub risk: Risk,
    pub confidence: f32,
    pub migration_hint: String,
    pub pattern: String,
    pub scope: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip)]
    compiled_pattern: Option<Regex>,
}

impl Rule {
    pub fn compiled_pattern(&self) -> Option<&Regex> {
        self.compiled_pattern.as_ref()
    }

    fn compile(&mut self) -> Result<()> {
        match self.kind {
            RuleKind::Regex
            | RuleKind::TreeSitter
            | RuleKind::Dependency
            | RuleKind::Key
            | RuleKind::Certificate => {
                let regex = RegexBuilder::new(&self.pattern)
                    .size_limit(1 << 20)
                    .build()
                    .with_context(|| format!("failed to compile regex for rule {}", self.id))?;
                self.compiled_pattern = Some(regex);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
    index_by_id: BTreeMap<String, usize>,
}

impl RuleSet {
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let mut rules = Vec::new();

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let ext = entry
                .path()
                .extension()
                .and_then(|x| x.to_str())
                .unwrap_or_default();
            if ext != "yml" && ext != "yaml" {
                continue;
            }

            let raw = std::fs::read_to_string(entry.path())
                .with_context(|| format!("failed to read {}", entry.path().display()))?;
            let mut loaded = load_rules_from_yaml(&raw)
                .with_context(|| format!("failed to parse {}", entry.path().display()))?;
            for rule in &mut loaded {
                rule.compile()?;
            }
            rules.extend(loaded);
        }

        rules.sort_by(|a, b| a.id.cmp(&b.id));

        let mut index_by_id = BTreeMap::new();
        for (idx, rule) in rules.iter().enumerate() {
            if index_by_id.insert(rule.id.clone(), idx).is_some() {
                anyhow::bail!("duplicate rule id detected: {}", rule.id);
            }
        }

        Ok(Self { rules, index_by_id })
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn all(&self) -> &[Rule] {
        &self.rules
    }

    pub fn by_kind(&self, kind: RuleKind) -> impl Iterator<Item = &Rule> {
        self.rules.iter().filter(move |rule| rule.kind == kind)
    }

    pub fn get(&self, id: &str) -> Option<&Rule> {
        self.index_by_id
            .get(id)
            .and_then(|idx| self.rules.get(*idx))
    }

    pub fn counts_by_kind(&self) -> BTreeMap<RuleKind, usize> {
        let mut out = BTreeMap::new();
        for rule in &self.rules {
            *out.entry(rule.kind).or_insert(0) += 1;
        }
        out
    }
}

fn load_rules_from_yaml(raw: &str) -> Result<Vec<Rule>> {
    let value: serde_yaml::Value = serde_yaml::from_str(raw)?;
    match value {
        serde_yaml::Value::Sequence(_) => Ok(serde_yaml::from_value(value)?),
        serde_yaml::Value::Mapping(_) => Ok(vec![serde_yaml::from_value(value)?]),
        _ => anyhow::bail!("unsupported YAML structure, expected map or sequence"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        dir.push(format!("pqc-scan-{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn loads_yaml_sequence_and_map() {
        let sequence = r#"
- id: TEST_SEQ
  kind: regex
  category: TLS
  severity: high
  risk: non-quantum-risk
  confidence: 0.7
  migration_hint: Upgrade.
  pattern: "TLSv1\\.0"
  scope: code
"#;
        let map = r#"
id: TEST_MAP
kind: regex
category: TLS
severity: high
risk: non-quantum-risk
confidence: 0.7
migration_hint: Upgrade.
pattern: "TLSv1\\.1"
scope: code
"#;

        let seq_rules = load_rules_from_yaml(sequence).expect("parse sequence");
        let map_rules = load_rules_from_yaml(map).expect("parse map");

        assert_eq!(seq_rules.len(), 1);
        assert_eq!(map_rules.len(), 1);
        assert_eq!(seq_rules[0].id, "TEST_SEQ");
        assert_eq!(map_rules[0].id, "TEST_MAP");
    }

    #[test]
    fn load_from_dir_compiles_regex() {
        let dir = temp_dir("rules-compile");
        fs::write(
            dir.join("a.yml"),
            r#"
- id: TEST_COMPILE
  kind: regex
  category: TLS
  severity: high
  risk: non-quantum-risk
  confidence: 0.7
  migration_hint: Upgrade.
  pattern: "TLSv1(?:\\.0|\\.1)?(?:$|[^0-9.])"
  scope: code
"#,
        )
        .expect("write rules file");

        let rules = RuleSet::load_from_dir(&dir).expect("load rules");
        let rule = rules.get("TEST_COMPILE").expect("compiled rule exists");
        assert!(rule.compiled_pattern().is_some());
    }

    #[test]
    fn rejects_duplicate_rule_ids() {
        let dir = temp_dir("rules-dup");
        let rule = r#"
- id: DUP_ID
  kind: regex
  category: TLS
  severity: low
  risk: non-quantum-risk
  confidence: 0.5
  migration_hint: Hint.
  pattern: "TLS"
  scope: code
"#;
        fs::write(dir.join("one.yml"), rule).expect("write first rule file");
        fs::write(dir.join("two.yml"), rule).expect("write second rule file");

        let err = RuleSet::load_from_dir(&dir).expect_err("duplicate id should fail");
        assert!(err.to_string().contains("duplicate rule id"));
    }

    #[test]
    fn tls_v1_legacy_pattern_does_not_match_tls_1_2() {
        let pattern = r#"(?m)^\s*ssl_protocols\s+[^;]*(TLSv1(?:\.0|\.1)?(?:$|[^0-9.]))"#;
        let regex = Regex::new(pattern).expect("compile test regex");

        assert!(regex.is_match("ssl_protocols TLSv1;"));
        assert!(regex.is_match("ssl_protocols TLSv1.1;"));
        assert!(!regex.is_match("ssl_protocols TLSv1.2;"));
        assert!(!regex.is_match("ssl_protocols TLSv1.3;"));
    }
}
