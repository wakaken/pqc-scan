use pqc_scan_core::{
    scan_repository_native, Detector, DetectorStatus, NativeOptions, ScanConfig, ScannableFile,
};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap()
}
fn ast(text: &str) -> pqc_scan_core::DetectorOutcome {
    TreeSitterDetector
        .detect_observed(
            &ScannableFile::from_bytes("input.js".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap()
}
#[test]
fn intrinsic_prefixes_are_expressions_not_remainders_or_literal_text() {
    for prefix in [
        "if (%Check(x)) {}",
        "while (%Check(x)) break;",
        "for (; %Check(x);) break;",
        "function g(){try { f(); } catch(e) {} %Check(x);}",
        "const x = typeof %Check(y);",
        "function f(){return %Check(x);}",
    ] {
        let source = format!("{prefix}\nconst alg = 'RS256';");
        let o = ast(&source);
        assert_eq!(o.status(), DetectorStatus::Complete, "{prefix}");
        assert!(o
            .detections()
            .iter()
            .any(|d| d.rule_id == "TS_JWT_RSA_ALGS" && d.location.line == 2));
    }
    for source in [
        "const x = 1 % Check(y);",
        "const s = '%Check(x)';",
        "// %Check(x)\nconst x=1;",
    ] {
        assert_eq!(ast(source).status(), DetectorStatus::Complete, "{source}");
    }
    assert_eq!(
        ast("const x = value %Check(y) unexpected;").status(),
        DetectorStatus::Partial
    );
}
#[test]
fn sloppy_yield_and_runtime_negative_syntax_are_not_parse_errors() {
    for source in [
        "function f(yield) { yield() } f();",
        "var yield; ({p: yield} = class {q=()=>42;});",
        "try { parseInt() = 0; } catch(e) {}",
        "for (f(x) in []) {}",
        "assertThrows(()=>Array() **= 10, ReferenceError);",
        "var x; [(x)] = [2]; [(x) = 3] = [];",
        r"tag`\u``\01`();",
    ] {
        assert_eq!(ast(source).status(), DetectorStatus::Complete, "{source}");
    }
    for source in [
        "'use strict'; function f(yield) {}",
        "function yield(){'use strict';}",
        "(function yield(){'use strict';})",
        "function* gen(){'use strict'; function yield(){}}",
        "function* f(){ yield = 1; }",
        r"const bad = `\u`;",
        "func_tion f(){}",
        r"function* f(){yield`\u`;} bye();",
        "'other directive'; 'use strict'; function f(yield){}",
        r"async function f(){await `\u`;}",
        r#"tag`x${ {a: 1} && "\u" }z`;"#,
        r#"tag`x${ /}/.test(x) && "\u" }z`;"#,
    ] {
        assert_eq!(ast(source).status(), DetectorStatus::Partial, "{source}");
    }
}
struct Fixture(PathBuf);
impl Fixture {
    fn new(tag: &str) -> Self {
        let p = std::env::temp_dir().join(format!("pqc-context-{}-{tag}", std::process::id()));
        std::fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn put(&self, n: &str, s: &str) {
        let p = self.0.join(n);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, s).unwrap();
    }
    fn scan(&self) -> serde_json::Value {
        let c = ScanConfig {
            root: self.0.clone(),
            threads: 1,
            ..Default::default()
        };
        let d: Vec<Arc<dyn Detector>> = vec![Arc::new(DependencyDetector)];
        serde_json::to_value(
            scan_repository_native(
                &c,
                &rules(),
                &d,
                &NativeOptions::new(Some("fixture")).unwrap(),
            )
            .unwrap()
            .native()
            .unwrap(),
        )
        .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn local_parent_properties_are_expanded_without_registry_resolution() {
    let f = Fixture::new("parent");
    f.put("pom.xml","<project><groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version><properties><crypto.version>1.2.3</crypto.version></properties></project>");
    f.put("child/pom.xml","<project><parent><groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version></parent><artifactId>child</artifactId><dependencies><dependency><groupId>org.example</groupId><artifactId>crypto</artifactId><version>${crypto.version}</version></dependency></dependencies></project>");
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 0);
    let a = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["attributes"]["name"] == "org.example:crypto")
        .unwrap();
    assert_eq!(a["attributes"]["declared_constraint"], "1.2.3");
    assert!(a["attributes"]["resolved_version"].is_null());
}
#[test]
fn gemspec_static_dependency_declarations_have_parsed_provenance() {
    let text="Gem::Specification.new do |s|\n s.name = 'example'\n s.add_dependency 'openssl', '~> 3.0'\nend\n";
    let o = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("example.gemspec".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|n| n == "openssl")
        && d.location.line == 3));
}

#[test]
fn conditional_requirements_keep_both_declarations_and_do_not_resolve_either() {
    let text =
        "base==2.0\n{% if environment %}\nopenssl==3.0\n{% else %}\nopenssl==2.0\n{% endif %}\n";
    let o = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("requirements.txt".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    let hits: Vec<_> = o
        .detections()
        .iter()
        .filter(|d| {
            d.rule_id == "__DEPENDENCY_INVENTORY__" && d.evidence.metadata["dep_name"] == "openssl"
        })
        .collect();
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].location.line, 3);
    assert_eq!(hits[1].location.line, 5);
    assert!(hits.iter().all(|d| d
        .evidence
        .metadata
        .get("dep_context")
        .is_some_and(|c| c == "conditional_declaration_not_effective")));
}
#[test]
fn unrendered_source_templates_keep_original_findings_and_state_the_missing_context() {
    let f = Fixture::new("template");
    f.put(
        "view.js",
        "{% if enabled %}\nconst alg='RS256';\n{% endif %}\nconst injected={{ value }};\n",
    );
    let c = ScanConfig {
        root: f.0.clone(),
        threads: 1,
        ..Default::default()
    };
    let d: Vec<Arc<dyn Detector>> = vec![Arc::new(TreeSitterDetector)];
    let n = scan_repository_native(
        &c,
        &rules(),
        &d,
        &NativeOptions::new(Some("fixture")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(n.native().unwrap()).unwrap()["coverage"]["partial_files"],
        1
    );
    assert!(n
        .legacy()
        .findings
        .iter()
        .any(|f| f.rule_id == "TS_JWT_RSA_ALGS" && f.location.line == 2));
    assert!(serde_json::to_string(n.native().unwrap())
        .unwrap()
        .contains("Unrendered template"));
}

#[test]
fn parent_scope_identity_conflicts_and_cycles_do_not_fabricate_facts() {
    for (tag,parent,expected) in [
  ("mismatch","<groupId>wrong</groupId><artifactId>parent</artifactId><version>1</version><properties><v>9.9</v></properties>",1),
  ("conflict","<groupId>g</groupId><artifactId>parent</artifactId><version>1</version><properties><v>1</v><v>2</v></properties>",2),
  ("idempotent","<groupId>g</groupId><artifactId>parent</artifactId><version>1</version><properties><v>1</v><v>1</v></properties>",0),
 ] {
  let f=Fixture::new(tag);f.put("pom.xml",&format!("<project>{parent}<dependencies><dependency><groupId>g</groupId><artifactId>root</artifactId><version>${{v}}</version></dependency></dependencies></project>"));
  f.put("child/pom.xml","<project><parent><groupId>g</groupId><artifactId>parent</artifactId><version>1</version></parent><artifactId>child</artifactId><dependencies><dependency><groupId>g</groupId><artifactId>crypto</artifactId><version>${v}</version></dependency></dependencies></project>");
  let n=f.scan();assert_eq!(n["coverage"]["partial_files"],expected,"{tag}");
  if tag!="idempotent" {let a=n["assets"].as_array().unwrap().iter().find(|a|a["attributes"]["name"]=="g:crypto").unwrap();assert!(a["attributes"]["reported_version"].is_null(),"{tag}");}
 }
    let f = Fixture::new("parent-cycle");
    for (name, other) in [("a", "b"), ("b", "a")] {
        f.put(&format!("{name}/pom.xml"),&format!("<project><groupId>g</groupId><artifactId>{name}</artifactId><version>1</version><parent><groupId>g</groupId><artifactId>{other}</artifactId><version>1</version><relativePath>../{other}/pom.xml</relativePath></parent></project>"));
    }
    assert!(f.scan()["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["code"] == "detector_truncated"));
}
#[test]
fn profile_versions_are_conditional_candidates_not_a_selected_effective_version() {
    let f = Fixture::new("profiles");
    f.put("pom.xml","<project><dependencies><dependency><groupId>g</groupId><artifactId>crypto</artifactId><version>${v}</version></dependency></dependencies><profiles><profile><id>a</id><properties><v>1.0</v></properties></profile><profile><id>b</id><properties><v>2.0</v></properties></profile></profiles></project>");
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 0);
    let a: Vec<_> = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["attributes"]["name"] == "g:crypto")
        .collect();
    assert_eq!(a.len(), 2);
    assert!(a.iter().all(|a| a["attributes"]["version_provenance"]
        == "manifest_optional_declared"
        && a["attributes"]["resolved_version"].is_null()));
}
#[cfg(unix)]
#[test]
fn excluded_and_symlink_parents_are_not_read_for_context() {
    for symlink in [false, true] {
        let f = Fixture::new(if symlink {
            "parent-link"
        } else {
            "parent-exclude"
        });
        f.put("real/pom.xml","<project><groupId>g</groupId><artifactId>parent</artifactId><version>1</version><properties><v>9.9.9</v></properties></project>");
        if symlink {
            std::fs::create_dir_all(f.0.join("parent")).unwrap();
            std::os::unix::fs::symlink(f.0.join("real/pom.xml"), f.0.join("parent/pom.xml"))
                .unwrap();
        } else {
            f.put(
                "parent/pom.xml",
                &std::fs::read_to_string(f.0.join("real/pom.xml")).unwrap(),
            );
            f.put(".gitignore", "/parent/\n");
        }
        f.put("child/pom.xml","<project><parent><groupId>g</groupId><artifactId>parent</artifactId><version>1</version><relativePath>../parent/pom.xml</relativePath></parent><artifactId>child</artifactId><dependencies><dependency><groupId>g</groupId><artifactId>crypto</artifactId><version>${v}</version></dependency></dependencies></project>");
        let n = f.scan();
        let a = n["assets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["attributes"]["name"] == "g:crypto")
            .unwrap();
        assert!(a["attributes"]["reported_version"].is_null());
        assert!(n["coverage"]["partial_files"].as_u64().unwrap() >= 1);
    }
}
#[test]
fn ruby_dynamic_or_invalid_declarations_stay_partial_and_strings_are_not_calls() {
    for text in [
        "gem ENV['NAME']",
        "gem 'openssl', 'banana'",
        "gem 'openssl', ENV['VERSION']",
        "send(:gem, 'openssl')",
    ] {
        let o = DependencyDetector
            .detect_observed(
                &ScannableFile::from_bytes("Gemfile".into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial, "{text}");
    }
    let o = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes(
                "Gemfile".into(),
                b"# gem 'fake'\ntext=\"gem 'fake'\"\nif ENV['USE']\n gem 'openssl', '~> 3'\nend\n"
                    .to_vec(),
            ),
            &rules(),
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o
        .detections()
        .iter()
        .all(|d| d.evidence.metadata["dep_name"] == "openssl"));
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_context")
        .is_some_and(|s| s == "conditional_declaration_not_effective")));
}

#[test]
fn delegated_gemspec_records_keep_their_own_file_and_rebound_receivers_are_not_trusted() {
    let f = Fixture::new("delegated-gemspec");
    f.put("Gemfile", "source 'https://rubygems.org'\ngemspec\n");
    f.put(
        "x.gemspec",
        "Gem::Specification.new do |s|\n s.name='x'\n s.add_dependency 'openssl', '~> 3.0'\nend\n",
    );
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 0);
    let obs = n["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["method"] == "dependency_report")
        .unwrap();
    assert_eq!(obs["target"]["path"], "x.gemspec");
    assert_eq!(obs["target"]["line"], 3);
    let text = "Gem::Specification.new do |s|\ns = other\ns.add_dependency 'openssl', '1.0'\nend\n";
    let o = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("x.gemspec".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert!(o.detections().is_empty());
}

#[test]
fn mjs_and_explicit_module_sources_do_not_gain_sloppy_yield_bindings() {
    for (name, text) in [
        ("script.mjs", "var yield;"),
        ("script.js", "export {}; var yield;"),
        ("script.js", "// Flags: --use-strict\nvar yield;"),
    ] {
        let o = TreeSitterDetector
            .detect_detailed(
                &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial, "{name}: {text}");
    }
}

#[test]
fn jsx_objects_and_quoted_template_examples_are_not_unrendered_source() {
    for text in [
        "const alg='RS256'; const x=<A cfg={{alg}}/>;",
        "// {% example %}\nconst alg='RS256'; const x=<A cfg={{alg}}/>;",
        "const example='{% if x %}'; const alg='RS256'; const x=<A cfg={{alg}}/>;",
    ] {
        assert_eq!(ast(text).status(), DetectorStatus::Complete, "{text}");
    }
}

#[test]
fn profile_overrides_keep_default_and_override_candidates_conditional() {
    let f = Fixture::new("profile-overrides");
    f.put("pom.xml","<project><properties><v>1.0</v></properties><dependencies><dependency><groupId>g</groupId><artifactId>crypto</artifactId><version>${v}</version></dependency></dependencies><profiles><profile><id>override</id><properties><v>2.0</v></properties></profile></profiles></project>");
    let n = f.scan();
    let a = n["assets"].as_array().unwrap();
    let versions: std::collections::BTreeSet<_> = a
        .iter()
        .filter(|a| a["attributes"]["name"] == "g:crypto")
        .map(|a| a["attributes"]["reported_version"].as_str().unwrap())
        .collect();
    assert_eq!(versions, std::collections::BTreeSet::from(["1.0", "2.0"]));
    assert!(a
        .iter()
        .filter(|a| a["attributes"]["name"] == "g:crypto")
        .all(
            |a| a["attributes"]["version_provenance"] == "manifest_optional_declared"
                && a["uncertainty"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|s| s == "conditional_declaration_not_effective_dependency")
        ));
}
#[test]
fn review_javascript_invalid_modes_and_untagged_templates_remain_partial() {
    for source in [
        r"if (x) {} `\u`;",
        r"if (x) `\u`;",
        "const obj={*g(){yield = 1;}};",
        "const obj={async *g(){yield = 1;}};",
    ] {
        assert_eq!(ast(source).status(), DetectorStatus::Partial, "{source}");
    }
}
#[test]
fn review_ruby_parallel_assignment_and_deferred_method_context_are_not_unconditional() {
    let text="Gem::Specification.new do |s|\ns, other = Object.new, 1\ns.add_dependency 'openssl', '3.0.0'\nend";
    let o = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("x.gemspec".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert!(o.detections().is_empty());
    let text = "def unused\n gem 'openssl', '3.0.0'\nend\n";
    let o = DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("Gemfile".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert!(o.detections().iter().all(|d| d
        .evidence
        .metadata
        .get("dep_context")
        .is_some_and(|s| s == "deferred_declaration_not_evaluated")));
}
#[test]
fn whitespace_control_and_multiline_jinja_branches_keep_conditional_provenance() {
    for text in [
        "{%- if target %}\nopenssl==1.0\n{%- else %}\nopenssl==2.0\n{%- endif %}\n",
        "{%\nif target\n%}\nopenssl==1.0\n{% else %}\nopenssl==2.0\n{% endif %}\n",
    ] {
        let o = DependencyDetector
            .detect_observed(
                &ScannableFile::from_bytes("requirements.txt".into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial);
        let hits: Vec<_> = o
            .detections()
            .iter()
            .filter(|d| d.rule_id == "__DEPENDENCY_INVENTORY__")
            .collect();
        assert_eq!(hits.len(), 2);
        assert!(hits.iter().all(|d| d
            .evidence
            .metadata
            .get("dep_context")
            .is_some_and(|s| s == "conditional_declaration_not_effective")));
    }
}

#[test]
fn unknown_receiver_blocks_and_postfix_loops_are_deferred() {
    for text in [
        "Object.new do\n gem 'openssl', '3.0.0'\nend",
        "object.group do\n gem 'openssl', '3.0.0'\nend",
        "gem 'openssl', '3.0.0' while false",
        "gem 'openssl', '3.0.0' until true",
    ] {
        let o = DependencyDetector
            .detect_observed(
                &ScannableFile::from_bytes("Gemfile".into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert_eq!(o.status(), DetectorStatus::Partial, "{text}");
        assert!(!o.detections().is_empty());
        assert!(o.detections().iter().all(|d| d
            .evidence
            .metadata
            .get("dep_context")
            .is_some_and(|s| s == "deferred_declaration_not_evaluated")));
    }
}

#[test]
fn regular_expression_template_examples_are_not_generated_source() {
    for text in [
        r"const pattern=/{% if example %}/; const alg='RS256';",
        r"const pattern=/{{ $example := 1 }}/; const alg='RS256';",
    ] {
        assert_eq!(ast(text).status(), DetectorStatus::Complete, "{text}");
    }
}

#[test]
fn string_in_an_ordinary_block_does_not_enable_function_strict_mode() {
    for text in [
        "while(false) { 'use strict'; var yield; }",
        "for(;;) { 'use strict'; var yield; break; }",
    ] {
        assert_eq!(ast(text).status(), DetectorStatus::Complete, "{text}");
    }
}

#[test]
fn intrinsic_after_recovered_control_block_keeps_statement_context() {
    let source = "if (%Check(x)) { assertFalse(%Other(x)); }\n%Check(x);\nconst alg='RS256';";
    let outcome = ast(source);
    assert_eq!(outcome.status(), DetectorStatus::Complete);
    assert!(outcome
        .detections()
        .iter()
        .any(|d| d.rule_id == "TS_JWT_RSA_ALGS" && d.location.line == 3));
}
#[test]
fn computed_fields_do_not_rewrite_nested_call_or_catch_parentheses() {
    let source="function* maker(){ class C { #m(){} [(exported = function access(receiver,resume){try {receiver.#m();} catch(e) {} advance(resume);},yield 1,'x')] = 1; } return C; }\n%Check(maker);\nconst alg='RS256';";
    assert_eq!(ast(source).status(), DetectorStatus::Complete);
    assert_eq!(
        ast("class C { [(function f(){advance value;},'x')]=1; }").status(),
        DetectorStatus::Partial
    );
}
#[test]
fn tagged_invalid_escapes_are_identified_despite_original_error_recovery() {
    for source in [
        r"tag`left${0}\u{}${1}right`;",
        r"tag`\u{\u{0}${0}right`;",
        r"tag`\u{-0}`; tag`\u{110000}`;",
        r"tag`\xg${0}right`;",
    ] {
        assert_eq!(ast(source).status(), DetectorStatus::Complete, "{source}");
    }
    for source in [
        r"if(x){} `\u{}`;",
        r"if(x) `\u{}`;",
        r"const x=`\u{}`;",
        r"tag`ok${ '\u' }`;",
    ] {
        assert_eq!(ast(source).status(), DetectorStatus::Partial, "{source}");
    }
}

#[test]
fn javascript_unicode_whitespace_does_not_hide_intrinsic_expression_boundaries() {
    for gap in ["\u{00a0}", "\u{feff}", "\u{2003}", "\u{2028}"] {
        assert_eq!(
            ast(&format!(
                "const o={{valueOf:()=>{{{gap}%Check(x); return 20;}}}};"
            ))
            .status(),
            DetectorStatus::Complete
        );
    }
    assert_eq!(
        ast("const x = value\u{00a0}%Check(y) unexpected;").status(),
        DetectorStatus::Partial
    );
}

#[test]
fn separate_typescript_signatures_do_not_share_parameter_budgets() {
    let rules = rules();
    for shape in [
        "declare function f{n}(key: string): void;",
        "type F{n} = (key: string) => void;",
        "type F{n} = new (key: string) => object;",
    ] {
        let mut text = (0..80)
            .map(|i| shape.replace("{n}", &i.to_string()))
            .collect::<Vec<_>>()
            .join("\n");
        text.push_str("\nconst algorithm = 'RS256';");
        let file = ScannableFile::from_bytes("sample.d.ts".into(), text.into_bytes());
        for detector in [
            Box::new(TreeSitterDetector) as Box<dyn Detector>,
            Box::new(pqc_scan_detectors::regex_detector::RegexDetector),
        ] {
            let o = detector.detect_detailed(&file, &rules).unwrap();
            assert_eq!(o.status(), DetectorStatus::Complete, "{shape}");
            assert!(o
                .detections()
                .iter()
                .any(|d| d.evidence.r#match.contains("RS256")));
        }
    }
}

#[test]
fn ruby_selected_plain_version_files_are_declared_not_resolved() {
    let f = Fixture::new("ruby-version-file");
    f.put("VERSION", "3.4.0\n");
    f.put("pkg/Gemfile", "source 'https://rubygems.org'\ngemspec\n");
    f.put("pkg/sample.gemspec", "version = File.read(File.expand_path('../VERSION', __dir__)).strip\nGem::Specification.new do |s|\n s.version = version\n s.requirements << 'none'\n s.rdoc_options << '--exclude' << '.'\n s.rdoc_options.concat ['--main', 'README']\n s.extra_rdoc_files = ['README']\n s.add_dependency 'openssl', version\nend\n");
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 0);
    let a = n["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["attributes"]["name"] == "openssl")
        .unwrap();
    assert_eq!(a["attributes"]["declared_constraint"], "3.4.0");
    assert!(a["attributes"]["resolved_version"].is_null());
}

#[test]
fn ruby_version_inputs_reject_mutation_escape_dynamic_code_and_bad_values() {
    let expression = "File.read(File.expand_path('../VERSION', __dir__)).strip";
    for (extra, value) in [
        ("version.replace('9.9.9')", "3.4.0\n"),
        ("File.write('../VERSION', '9.9.9')", "3.4.0\n"),
        ("helper.rewrite_version_file", "3.4.0\n"),
        ("alias_version = version", "3.4.0\n"),
        ("mutate(version)", "3.4.0\n"),
        ("arbitrary.add_dependency('x', version)", "3.4.0\n"),
        ("version = '9.9.9'", "3.4.0\n"),
        ("version += '0'", "3.4.0\n"),
        ("require 'something'", "3.4.0\n"),
        ("File = CustomFile", "3.4.0\n"),
        ("Gem = CustomGem", "3.4.0\n"),
        ("def f(version); end", "3.4.0\n"),
        ("", "not-a-version"),
        ("", "\u{2003}3.4.0"),
        ("", "3.4.0\nFile.write('marker', 'executed')"),
    ] {
        let f = Fixture::new("ruby-version-negative");
        f.put("VERSION", value);
        f.put("pkg/sample.gemspec", &format!("version = {expression}\n{extra}\nGem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend\n"));
        let n = f.scan();
        assert_eq!(n["coverage"]["partial_files"], 1, "{extra}: {value}");
        assert!(
            n["assets"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|a| a["attributes"]["name"] == "openssl")
                .all(|a| a["attributes"]["declared_constraint"].is_null()),
            "{extra}"
        );
        assert!(!f.0.join("marker").exists());
    }
}

#[test]
fn ruby_version_files_respect_selection_size_and_no_follow() {
    for mode in ["ignored", "missing", "outside", "large", "wrong_expression"] {
        let f = Fixture::new("ruby-version-selection");
        f.put(
            "VERSION",
            &if mode == "large" {
                "3".repeat(129)
            } else {
                "3.4.0\n".into()
            },
        );
        if mode == "ignored" {
            f.put(".gitignore", "VERSION\n");
        }
        let path = match mode {
            "outside" => "../../VERSION",
            "missing" => "../ABSENT",
            _ => "../VERSION",
        };
        let expr = if mode == "wrong_expression" {
            "File.read('../VERSION').strip".into()
        } else {
            format!("File.read(File.expand_path('{path}', __dir__)).strip")
        };
        f.put("pkg/sample.gemspec", &format!("version = {expr}\nGem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend\n"));
        let n = f.scan();
        assert_eq!(n["coverage"]["partial_files"], 1, "{mode}");
    }
    #[cfg(unix)]
    {
        let f = Fixture::new("ruby-version-link");
        f.put("REAL_VERSION", "3.4.0\n");
        std::os::unix::fs::symlink("REAL_VERSION", f.0.join("VERSION")).unwrap();
        f.put("sample.gemspec", "version = File.read(File.expand_path('VERSION', __dir__)).strip\nGem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend\n");
        assert_eq!(f.scan()["coverage"]["partial_files"], 1);
    }
}

#[test]
fn ruby_version_input_budget_is_explicit_and_literal_aliases_are_not_installation_facts() {
    let f = Fixture::new("ruby-version-budget");
    let mut text = String::new();
    for i in 0..9 {
        f.put(&format!("V{i}"), "3.4.0\n");
        text.push_str(&format!(
            "v{i} = File.read(File.expand_path('V{i}', __dir__)).strip\n"
        ));
    }
    text.push_str("Gem::Specification.new do |s|\n");
    for i in 0..9 {
        text.push_str(&format!(" s.add_dependency 'package{i}', v{i}\n"));
    }
    text.push_str("end\n");
    f.put("sample.gemspec", &text);
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 1);
    assert!(n["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["code"] == "detector_truncated"));
    assert_eq!(
        n["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["attributes"]["declared_constraint"] == "3.4.0")
            .count(),
        8
    );
    let o = DependencyDetector.detect_observed(&ScannableFile::from_bytes("sample.gemspec".into(), b"version = '3.4.0'\nGem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend\n".to_vec()), &rules()).unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_version")
        .is_some_and(|v| v == "3.4.0")));
}

#[test]
fn independent_functions_keep_complete_inspection_across_all_supported_languages() {
    let rules = rules();
    let cases = [
        (
            "java",
            "class C {",
            "void f{n}(String key){String algorithm = \"RS256\";}",
            "}",
        ),
        (
            "go",
            "package p\n",
            "func f{n}(key string){var algorithm = \"RS256\"; _ = algorithm}",
            "",
        ),
        ("py", "", "def f{n}(key):\n return 'RS256'", ""),
        (
            "rs",
            "",
            "fn f{n}(key: &str){let algorithm = \"RS256\";}",
            "",
        ),
        ("rb", "", "def f{n}(key); 'RS256'; end", ""),
        ("js", "", "function f{n}(key){return 'RS256';}", ""),
        ("ts", "", "function f{n}(key: string){return 'RS256';}", ""),
    ];
    for (ext, prefix, body, suffix) in cases {
        let source = format!(
            "{prefix}\n{}{suffix}",
            (0..80)
                .map(|i| body.replace("{n}", &i.to_string()))
                .collect::<Vec<_>>()
                .join("\n")
        );
        let file = ScannableFile::from_bytes(format!("input.{ext}").into(), source.into_bytes());
        for detector in [
            Box::new(TreeSitterDetector) as Box<dyn Detector>,
            Box::new(pqc_scan_detectors::regex_detector::RegexDetector),
        ] {
            let o = detector.detect_detailed(&file, &rules).unwrap();
            assert_eq!(o.status(), DetectorStatus::Complete, "{ext}");
            assert!(!o.detections().is_empty(), "{ext}");
        }
    }
}

#[test]
fn real_same_scope_binding_exhaustion_still_reports_truncation() {
    let text = format!(
        "{}\nconst algorithm = 'RS256';\n",
        "var repeated = unknown;\n".repeat(80)
    );
    let file = ScannableFile::from_bytes("sample.js".into(), text.into_bytes());
    for detector in [
        Box::new(TreeSitterDetector) as Box<dyn Detector>,
        Box::new(pqc_scan_detectors::regex_detector::RegexDetector),
    ] {
        let o = detector.detect_detailed(&file, &rules()).unwrap();
        assert_eq!(o.status(), DetectorStatus::Truncated);
        assert!(o
            .detections()
            .iter()
            .any(|d| d.evidence.r#match.contains("RS256")));
    }
}

#[test]
fn signature_parameters_do_not_shadow_outer_runtime_signer_imports() {
    let file = ScannableFile::from_bytes("sample.ts".into(), b"import { sign } from 'jsonwebtoken';\ndeclare function f(sign: string): void;\ntype F = (sign: string) => void;\nsign({}, key, { algorithm: 'RS256' });\n".to_vec());
    let o = TreeSitterDetector.detect_detailed(&file, &rules()).unwrap();
    assert_eq!(o.status(), DetectorStatus::Complete);
    assert!(o
        .detections()
        .iter()
        .any(|d| d.rule_id == "TS_NODE_JOSE_USAGE" && d.location.line == 4));
}

#[test]
fn delegated_ruby_version_context_does_not_recurse_through_gemspec_cycles() {
    let f = Fixture::new("ruby-version-cycle");
    f.put("Gemfile", "gemspec\n");
    f.put("sample.gemspec", "gemspec\n");
    assert_eq!(f.scan()["coverage"]["partial_files"], 2);
}

#[test]
fn bare_gemspec_needs_one_selected_sibling_and_bound_locals_are_not_directives() {
    for text in [
        "gemspec\n",
        "gemspec = false\ngemspec\n",
        "group :development do\n gemspec\nend\n",
        "group :development do\n gemspec()\nend\n",
        "if ENV['LOCAL']\n gemspec\nend\n",
    ] {
        let f = Fixture::new("ruby-gemspec-missing");
        f.put("Gemfile", text);
        assert_eq!(f.scan()["coverage"]["partial_files"], 1);
    }
    let f = Fixture::new("ruby-gemspec-ignored");
    f.put("Gemfile", "gemspec\n");
    f.put(
        "hidden.gemspec",
        "Gem::Specification.new do |s|\n s.add_dependency 'openssl', '3.4.0'\nend\n",
    );
    f.put(".gitignore", "hidden.gemspec\n");
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 1);
    assert!(n["assets"].as_array().unwrap().is_empty());
}

#[test]
fn ruby_version_declarations_do_not_borrow_a_later_assignment() {
    let o = DependencyDetector.detect_observed(&ScannableFile::from_bytes("sample.gemspec".into(), b"Gem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend\nversion = '3.4.0'\n".to_vec()), &rules()).unwrap();
    assert_eq!(o.status(), DetectorStatus::Partial);
    assert!(o.detections().iter().all(|d| d
        .evidence
        .metadata
        .get("dep_version")
        .is_none_or(|v| v == "unknown")));
}

#[test]
fn ruby_method_and_directory_shadowing_do_not_certify_version_paths() {
    for prefix in [
        "alias __dir__ to_s",
        "undef __dir__",
        "__dir__ = '/not-the-manifest-directory'",
        "alias require_relative to_s",
    ] {
        let f = Fixture::new("ruby-version-methods");
        f.put("VERSION", "3.4.0\n");
        f.put("sample.gemspec", &format!("{prefix}\nversion = File.read(File.expand_path('VERSION', __dir__)).strip\nGem::Specification.new do |s|\n s.add_dependency 'openssl', version\nend\n"));
        let n = f.scan();
        assert_eq!(n["coverage"]["partial_files"], 1, "{prefix}");
        assert!(n["assets"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| a["attributes"]["declared_constraint"].is_null()));
    }
}

#[test]
fn ruby_constant_only_relative_loads_do_not_execute_or_duplicate_dependencies() {
    let f = Fixture::new("ruby-constant-module");
    f.put(
        "lib/version.rb",
        "module Example\n VERSION = '0.0.2'\nend\n",
    );
    let mut text = "require_relative 'lib/version'\n".repeat(200);
    text.push_str("Gem::Specification.new do |s|\n s.version = Example::VERSION\n s.add_dependency 'openssl', '~> 3.4'\nend\n");
    f.put("sample.gemspec", &text);
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 0);
    assert_eq!(
        n["assets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["attributes"]["name"] == "openssl")
            .count(),
        1
    );
}

#[test]
fn ruby_relative_modules_with_code_or_ambiguous_loader_context_remain_partial() {
    for (prefix, module) in [
        (
            "",
            "module Example\n File.write('marker', 'executed')\n VERSION = '0.0.2'\nend\n",
        ),
        (
            "",
            "module Example\n def self.modify; end\n VERSION = '0.0.2'\nend\n",
        ),
        (
            "",
            "module Example\n require 'another'\n VERSION = '0.0.2'\nend\n",
        ),
        ("", "module Example\n VERSION = dynamic\nend\n"),
        ("", "module Gem\n VERSION = '0.0.2'\nend\n"),
        (
            "alias require_relative to_s\n",
            "module Example\n VERSION = '0.0.2'\nend\n",
        ),
        (
            "setup_environment\n",
            "module Example\n VERSION = '0.0.2'\nend\n",
        ),
    ] {
        let f = Fixture::new("ruby-relative-code");
        f.put("lib/version.rb", module);
        f.put("sample.gemspec", &format!("{prefix}require_relative 'lib/version'\nGem::Specification.new do |s|\n s.add_dependency 'openssl', '~> 3.4'\nend\n"));
        assert_eq!(
            f.scan()["coverage"]["partial_files"],
            1,
            "{prefix}: {module}"
        );
        assert!(!f.0.join("marker").exists());
    }
}

#[test]
fn ruby_relative_module_readers_keep_selection_and_real_resource_limits() {
    for mode in ["ignored", "outside", "large", "nodes", "depth"] {
        let f = Fixture::new("ruby-relative-boundary");
        let module = match mode {
            "large" => format!("#{}\n", "x".repeat(16384)),
            "nodes" => format!("module Example\n{}end\n", "VERSION='v'\n".repeat(1100)),
            "depth" => format!(
                "{}VERSION='v'\n{}",
                "module Example\n".repeat(9),
                "end\n".repeat(9)
            ),
            _ => "module Example\n VERSION = '0.0.2'\nend\n".into(),
        };
        f.put("version.rb", &module);
        if mode == "ignored" {
            f.put(".gitignore", "version.rb\n");
        }
        let target = if mode == "outside" {
            "../../version"
        } else {
            "version"
        };
        f.put("sample.gemspec", &format!("require_relative '{target}'\nGem::Specification.new do |s|\n s.add_dependency 'openssl', '~> 3.4'\nend\n"));
        let n = f.scan();
        assert_eq!(n["coverage"]["partial_files"], 1, "{mode}");
        if ["large", "nodes", "depth"].contains(&mode) {
            assert!(
                n["diagnostics"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|d| d["code"] == "detector_truncated"),
                "{mode}"
            );
        }
    }
    let f = Fixture::new("ruby-relative-count");
    let mut text = String::new();
    for i in 0..9 {
        f.put(&format!("v{i}.rb"), "module Example\n VERSION='v'\nend\n");
        text.push_str(&format!("require_relative 'v{i}'\n"));
    }
    text.push_str("Gem::Specification.new do |s|\n s.add_dependency 'openssl', '~> 3.4'\nend\n");
    f.put("sample.gemspec", &text);
    let n = f.scan();
    assert_eq!(n["coverage"]["partial_files"], 1);
    assert!(n["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["code"] == "detector_truncated"));
}

#[test]
fn ruby_qualified_loads_and_linked_relative_modules_stay_unresolved() {
    for directive in [
        "Kernel.require_relative 'version'",
        "Kernel.require 'version'",
        "require_relative('version') { arbitrary_code }",
    ] {
        let f = Fixture::new("ruby-qualified-load");
        f.put("version.rb", "module Example\n VERSION='v'\nend\n");
        f.put("sample.gemspec", &format!("{directive}\nGem::Specification.new do |s|\n s.add_dependency 'openssl', '~> 3.4'\nend\n"));
        assert_eq!(f.scan()["coverage"]["partial_files"], 1, "{directive}");
    }
    #[cfg(unix)]
    {
        let f = Fixture::new("ruby-relative-link");
        f.put("real.rb", "module Example\n VERSION='v'\nend\n");
        std::os::unix::fs::symlink("real.rb", f.0.join("version.rb")).unwrap();
        f.put("sample.gemspec", "require_relative 'version'\nGem::Specification.new do |s|\n s.add_dependency 'openssl', '~> 3.4'\nend\n");
        assert_eq!(f.scan()["coverage"]["partial_files"], 1);
    }
}
