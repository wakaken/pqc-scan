use pqc_scan_rules::RuleSet;
use std::{fs, path::PathBuf};
struct Pack(PathBuf);
impl Pack {
    fn new(tag: &str) -> Self {
        let p = std::env::temp_dir().join(format!("pqc028-{}-{tag}", std::process::id()));
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn write(&self, id: &str, confidence: &str, scope: &str) {
        fs::write(self.0.join("rule.yml"), format!("id: {id}\nkind: regex\ncategory: custom\nseverity: low\nrisk: quantum-uncertain\nconfidence: {confidence}\nmigration_hint: Review\npattern: CONTROL\nscope: {scope}\ncustom_extension: accepted\n")).unwrap();
    }
}
impl Drop for Pack {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn blank_ids_are_rejected_without_trimming_valid_custom_ids() {
    let p = Pack::new("ids");
    for id in ["''", "'   '", "\"\\t\\n\"", "'　'"] {
        p.write(id, "0.5", "code");
        assert!(RuleSet::load_from_dir(&p.0).is_err(), "accepted blank {id}");
    }
    p.write("' custom:日本語 '", "0.5", "code");
    assert_eq!(
        RuleSet::load_from_dir(&p.0).unwrap().all()[0].id,
        " custom:日本語 "
    );
}
#[test]
fn confidence_rejects_nonfinite_out_of_bounds_and_source_rounding_loopholes() {
    let p = Pack::new("confidence");
    for value in [
        "-0.1",
        "1.1",
        "1.00000001",
        "-1e-50",
        ".nan",
        ".inf",
        "-.inf",
        "1e100",
    ] {
        p.write("CUSTOM", value, "code");
        assert!(RuleSet::load_from_dir(&p.0).is_err(), "accepted {value}");
    }
}
#[test]
fn valid_endpoints_and_custom_scopes_keep_existing_acceptance_and_values() {
    let p = Pack::new("custom");
    for scope in [
        "code",
        "config",
        "' CUSTOM, odd|tokens; go '",
        "''",
        "'*'",
        "any",
    ] {
        for confidence in ["0", "-0.0", "1", "0.12345678"] {
            p.write("CUSTOM", confidence, scope);
            let pack = RuleSet::load_from_dir(&p.0).unwrap();
            let r = &pack.all()[0];
            assert_eq!(r.confidence, confidence.parse::<f32>().unwrap());
            assert!(r.compiled_pattern().unwrap().is_match("CONTROL"));
        }
    }
}
#[test]
fn missing_root_is_not_a_successful_empty_pack() {
    let p = Pack::new("missing");
    assert!(RuleSet::load_from_dir(&p.0.join("missing")).is_err());
}
#[test]
fn empty_directory_readable_yaml_root_and_unrelated_extensions_stay_compatible() {
    let p = Pack::new("root");
    fs::write(p.0.join("not-rules.txt"), "not YAML [").unwrap();
    assert!(RuleSet::load_from_dir(&p.0).unwrap().is_empty());
    p.write("CUSTOM", "0.5", "code");
    assert_eq!(
        RuleSet::load_from_dir(&p.0.join("rule.yml")).unwrap().len(),
        1
    );
}
#[test]
fn nested_invalid_rules_prevent_partial_success_and_regex_validation_remains() {
    let p = Pack::new("nested");
    p.write("GOOD", "0.5", "code");
    fs::create_dir_all(p.0.join("nested")).unwrap();
    fs::write(p.0.join("nested/bad.yaml"), "[invalid YAML").unwrap();
    assert!(RuleSet::load_from_dir(&p.0).is_err());
    fs::remove_file(p.0.join("nested/bad.yaml")).unwrap();
    let text = fs::read_to_string(p.0.join("rule.yml"))
        .unwrap()
        .replace("pattern: CONTROL", "pattern: '['");
    fs::write(p.0.join("rule.yml"), text).unwrap();
    assert!(RuleSet::load_from_dir(&p.0).is_err());
}
