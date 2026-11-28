//! Data-only presentation projections; no policy decisions or inferred deployment.
use crate::{
    migration_copy::{enum_token, Labels},
    ReportLanguage,
};
use pqc_scan_core::migration::{MigrationAssessment, SavedEvidence, Target, Urgency};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// A path hint, never proof of production/test applicability.
pub fn migration_location(path: Option<&str>) -> &'static str {
    let lower = path.unwrap_or("").to_ascii_lowercase();
    let parts: Vec<_> = lower.split(['/', '\\']).collect();
    let name = parts.last().copied().unwrap_or("");
    if parts.iter().any(|p| {
        [
            "test",
            "tests",
            "testsuite",
            "test-framework",
            "__tests__",
            "spec",
            "unittests",
            "unit-tests",
            "fixtures",
        ]
        .contains(p)
            || p.ends_with(".tests")
    }) {
        "tests"
    } else if [
        "pom.xml",
        "package.json",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "cargo.toml",
        "cargo.lock",
        "go.mod",
        "go.sum",
        "gemfile",
        "gemfile.lock",
        "requirements.txt",
        "pyproject.toml",
        "composer.json",
        "composer.lock",
        "packages.lock.json",
        "packages.config",
        "project.assets.json",
        "cmakelists.txt",
        "vcpkg.json",
        "conanfile.py",
        "conanfile.txt",
    ]
    .contains(&name)
        || [".csproj", ".fsproj", ".vbproj", ".props", ".targets"]
            .iter()
            .any(|ext| name.ends_with(ext))
    {
        "dependency"
    } else if [
        ".java", ".rs", ".go", ".py", ".js", ".ts", ".tsx", ".jsx", ".c", ".h", ".cpp", ".kt",
        ".rb", ".cs", ".php", ".phtml", ".cc", ".cxx", ".hpp", ".hh", ".hxx", ".cc.inc",
        ".cpp.inc", ".c.inc",
    ]
    .iter()
    .any(|ext| name.ends_with(ext))
    {
        "source"
    } else {
        "other"
    }
}
pub fn migration_location_label(kind: &str, language: ReportLanguage) -> &'static str {
    let c = Labels(language);
    match kind {
        "source" => c.pick("Source files", "ソースファイル"),
        "tests" => c.pick("Inside test folders", "テスト用フォルダ内"),
        "dependency" => c.pick("Dependency declarations", "依存関係の宣言"),
        _ => c.pick("Other records", "その他の記録"),
    }
}
fn cohort_key(target: &Target) -> String {
    serde_json::to_string(&(
        enum_token(&target.technical.algorithm.purpose),
        &target.source_ref.role,
        migration_location(target.source_ref.path.as_deref()),
    ))
    .expect("presentation key")
}
pub fn migration_scope_key(target: &Target) -> String {
    cohort_key(target)
}

/// No target selection is needed to obtain the complete overview.
pub fn migration_overview(evidence: &SavedEvidence) -> Value {
    let mut algorithms = BTreeMap::<String, usize>::new();
    let mut locations = BTreeMap::<String, usize>::new();
    let mut modules = BTreeMap::<String, usize>::new();
    let mut cohorts = BTreeMap::<String, Value>::new();
    for t in evidence.targets() {
        *algorithms
            .entry(t.technical.algorithm.identifier.clone())
            .or_default() += 1;
        let location = migration_location(t.source_ref.path.as_deref());
        *locations.entry(location.into()).or_default() += 1;
        let module = t
            .source_ref
            .path
            .as_deref()
            .unwrap_or("")
            .split(['/', '\\'])
            .next()
            .unwrap_or("");
        *modules.entry(module.into()).or_default() += 1;
        let key = cohort_key(t);
        let entry = cohorts.entry(key.clone()).or_insert_with(|| json!({"key":key,"purpose":t.technical.algorithm.purpose,"role":t.source_ref.role,"location":location,"total":0,"eligible":0,"role_ja":Labels(ReportLanguage::Ja).token(&t.source_ref.role),"role_en":Labels(ReportLanguage::En).token(&t.source_ref.role)}));
        entry["total"] = (entry["total"].as_u64().unwrap() + 1).into();
        if t.is_bindable() {
            entry["eligible"] = (entry["eligible"].as_u64().unwrap() + 1).into();
        }
    }
    json!({"input":evidence.input(),"total":evidence.targets().len(),"algorithms":algorithms,"locations":locations,"modules":modules,"cohorts":cohorts.into_values().collect::<Vec<_>>()})
}

#[derive(Serialize)]
pub(crate) struct WorkGroup {
    pub members: Vec<usize>,
}
/// Every decision appears exactly once; material states never become averages.
pub(crate) fn work_groups(report: &MigrationAssessment) -> Vec<WorkGroup> {
    let targets: BTreeMap<_, _> = report.targets().iter().map(|t| (&t.id, t)).collect();
    let candidates: BTreeMap<_, _> = report.candidates().iter().map(|c| (&c.id, c)).collect();
    let mut groups = BTreeMap::<String, (u8, Vec<usize>)>::new();
    for (i, d) in report.decisions().iter().enumerate() {
        let t = targets[&d.target_id];
        let choices: Vec<_> = d
            .action
            .candidate_ids
            .iter()
            .map(|id| {
                let c = candidates[id];
                let prerequisites: Vec<_> = c
                    .prerequisites
                    .iter()
                    .map(|p| (&p.name, p.claimed_support))
                    .collect();
                json!([
                    c.identifier,
                    c.purpose,
                    c.status,
                    c.parameter_selection,
                    prerequisites,
                    c.validation_steps
                ])
            })
            .collect();
        let key = serde_json::to_string(&json!([
            d.system_id,
            t.technical.algorithm.purpose,
            t.source_ref.role,
            d.urgency,
            d.information,
            d.readiness,
            d.reason_codes,
            d.missing_inputs,
            d.action.kind,
            d.action.blocking_prerequisites,
            d.action.validation_steps,
            choices,
            t.technical.algorithm.classical_concerns
        ]))
        .expect("presentation key");
        let rank = match d.urgency {
            Urgency::Urgent => 0,
            Urgency::Plan => 1,
            Urgency::Undetermined => 2,
            Urgency::Monitor => 3,
            Urgency::NotApplicable => 4,
        };
        groups
            .entry(key)
            .or_insert_with(|| (rank, vec![]))
            .1
            .push(i);
    }
    let mut entries: Vec<_> = groups.into_iter().collect();
    entries.sort_by(|a, b| a.1 .0.cmp(&b.1 .0).then(a.0.cmp(&b.0)));
    entries
        .into_iter()
        .map(|(_, (_, members))| WorkGroup { members })
        .collect()
}

/// Typed structural sharing. JSON numbers remain lexemes, never JS float coercions.
#[derive(Default, Serialize)]
pub(crate) struct Archive {
    pub root: usize,
    pub nodes: Vec<Value>,
    #[serde(skip)]
    memo: BTreeMap<Vec<u8>, usize>,
}
impl Archive {
    pub fn new(value: &Value) -> Self {
        let mut a = Self::default();
        a.root = a.intern(value);
        a
    }
    fn intern(&mut self, value: &Value) -> usize {
        let node = match value {
            Value::Object(object) => {
                let mut edges = vec![];
                for (k, v) in object {
                    edges.push(self.intern(&Value::String(k.clone())));
                    edges.push(self.intern(v));
                }
                json!(["o", edges])
            }
            Value::Array(array) => {
                let edges: Vec<_> = array.iter().map(|v| self.intern(v)).collect();
                json!(["a", edges])
            }
            Value::Number(number) => json!(["n", number.to_string()]),
            Value::String(s) => json!(["s", s]),
            Value::Bool(b) => json!(["b", b]),
            Value::Null => json!(["z"]),
        };
        let key = serde_json::to_vec(&node).expect("typed node");
        if let Some(&i) = self.memo.get(&key) {
            return i;
        }
        let i = self.nodes.len();
        self.memo.insert(key, i);
        self.nodes.push(node);
        i
    }
}

pub(crate) fn copy_table(value: &Value) -> Value {
    fn visit(v: &Value, words: &mut BTreeMap<String, Value>) {
        match v {
            Value::Object(o) => {
                for (k, v) in o {
                    visit(&Value::String(k.clone()), words);
                    visit(v, words);
                }
            }
            Value::Array(a) => {
                for v in a {
                    visit(v, words);
                }
            }
            Value::String(s) => {
                let en = Labels(ReportLanguage::En).token(s);
                let ja = Labels(ReportLanguage::Ja).token(s);
                if en != s || ja != s {
                    words.insert(s.clone(), json!({"en":en,"ja":ja}));
                }
            }
            _ => {}
        }
    }
    let mut words = BTreeMap::new();
    visit(value, &mut words);
    json!(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn restore(a: &Archive, i: usize) -> Value {
        let n = &a.nodes[i];
        match n[0].as_str().unwrap() {
            "o" => Value::Object(
                n[1].as_array()
                    .unwrap()
                    .chunks(2)
                    .map(|p| {
                        (
                            restore(a, p[0].as_u64().unwrap() as usize)
                                .as_str()
                                .unwrap()
                                .to_owned(),
                            restore(a, p[1].as_u64().unwrap() as usize),
                        )
                    })
                    .collect(),
            ),
            "a" => Value::Array(
                n[1].as_array()
                    .unwrap()
                    .iter()
                    .map(|p| restore(a, p.as_u64().unwrap() as usize))
                    .collect(),
            ),
            "n" => serde_json::from_str(n[1].as_str().unwrap()).unwrap(),
            "z" => Value::Null,
            _ => n[1].clone(),
        }
    }
    #[test]
    fn sharing_preserves_complete_types_and_large_numbers() {
        let v = json!({"a":[true,false,null,0,"0",u64::MAX,"</script>"],"b":[true,false,null,0,"0",u64::MAX,"</script>"]});
        let a = Archive::new(&v);
        assert_eq!(restore(&a, a.root), v);
        assert!(a.nodes.len() < 20);
    }
    #[test]
    fn paths_are_explicit_hints_without_product_names() {
        assert_eq!(migration_location(Some("backend/spec/keys.pem")), "tests");
        assert_eq!(migration_location(Some("contest/Test.java")), "source");
        assert_eq!(migration_location(Some("server/go.mod")), "dependency");
        assert_eq!(migration_location(None), "other");
    }
}
