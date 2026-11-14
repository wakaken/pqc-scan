//! serde_yaml uses YAML 1.2; Python sanity covers the YAML 1.1 interpretation.
use std::{collections::BTreeMap, fs, path::Path};

#[test]
fn task_ids_are_exact_unique_strings_and_dependencies_resolve() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tasks");
    let mut tasks = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        assert_ne!(
            path.extension().and_then(|s| s.to_str()),
            Some("yaml"),
            "task definitions belong in tasks/<id>/task.yaml: {}",
            path.display()
        );
        if !path.is_dir() {
            continue;
        }
        let directory_id = path.file_name().unwrap().to_str().unwrap();
        assert!(
            directory_id.len() == 3 && directory_id.bytes().all(|b| b.is_ascii_digit()),
            "invalid task directory: {}",
            path.display()
        );
        let definition = path.join("task.yaml");
        let task: serde_yaml::Value =
            serde_yaml::from_str(&fs::read_to_string(&definition).unwrap()).unwrap();
        let id = task["id"]
            .as_str()
            .expect("task id must remain a string")
            .to_string();
        assert_eq!(id, directory_id, "task ID must match its directory");
        assert!(tasks.insert(id, task).is_none(), "duplicate task id");
    }
    for id in 1..=62 {
        assert!(tasks.contains_key(&format!("{id:03}")), "missing task");
    }
    for (id, task) in &tasks {
        if let Some(dependencies) = task.get("depends_on") {
            for dep in dependencies.as_sequence().unwrap() {
                let dep = dep.as_str().unwrap();
                assert_ne!(id, dep);
                assert!(tasks.contains_key(dep), "{id}: unknown dependency {dep}");
            }
        }
    }
}
