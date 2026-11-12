use pqc_scan_core::{Detector, DetectorStatus, ScannableFile};
use pqc_scan_detectors::{
    dependency_detector::DependencyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector,
};
use pqc_scan_rules::RuleSet;
use std::path::Path;
fn rules() -> RuleSet {
    RuleSet::load_from_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules")).unwrap()
}
fn ast(name: &str, text: &str) -> pqc_scan_core::DetectorOutcome {
    TreeSitterDetector
        .detect_observed(
            &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap()
}
fn complete_with_original_hit(name: &str, text: &str) {
    let o = ast(name, text);
    assert_eq!(o.status(), DetectorStatus::Complete, "{text}");
    let hit = o
        .detections()
        .iter()
        .find(|d| d.rule_id == "TS_JWT_RSA_ALGS")
        .expect("original algorithm reference retained");
    let pos = text.find("RS256").unwrap();
    let prefix = &text[..pos];
    assert_eq!(
        hit.location.line,
        prefix.bytes().filter(|b| *b == b'\n').count() + 1
    );
    assert_eq!(
        hit.location.column,
        prefix.rsplit('\n').next().unwrap().chars().count() + 1
    );
    assert!(hit.evidence.snippet_preview.contains("RS256"));
}
#[test]
fn empty_type_alias_parameters_are_accepted_without_accepting_empty_function_parameters() {
    for prefix in [
        "export type Shape<> = { ok: true };",
        "type Shape< /* intentionally empty */ > = number;",
        "type Shape<\n> = string;",
    ] {
        complete_with_original_hit("library.ts", &format!("{prefix}\nconst alg = 'RS256';"));
    }
    for source in [
        "function f<>() {}",
        "class C<> {}",
        "type Shape<T = > = number;",
        "type Shape<> = ;",
    ] {
        assert_eq!(
            ast("library.ts", source).status(),
            DetectorStatus::Partial,
            "{source}"
        );
    }
}
#[test]
fn quoted_jsx_query_parameters_are_recovered_when_the_original_tree_loses_the_tag() {
    for name in ["component.tsx", "component.jsx"] {
        complete_with_original_hit(name,"export const view = <>\n<Link href=\"https://deploy.invalid/new?repository-url=https%3A%2F%2Fexample.org%2Fapp&project-name=demo&repository-name=demo&env=APP_URL,APP_KEY\" target=\"_blank\"><Button>Deploy</Button></Link>\n</>;\nconst alg = 'RS256';");
        complete_with_original_hit(
            name,
            "const v = <Widget url='/start?first-key=1&second-key=2' other={x && y} />;\nconst alg = 'RS256';",
        );
    }
    for source in [
        "const x = <A href=\"/path?first-key=1&second-key=2\" other={ />;",
        "const x = <A href=\"/path?first-key=1&second-key=2 />;",
        "const x = <A href=\"/path?first-key=1&second-key=2\"></B>;",
    ] {
        assert_eq!(
            ast("component.tsx", source).status(),
            DetectorStatus::Partial,
            "{source}"
        );
    }
}
#[test]
fn import_types_in_assertions_and_function_return_types_keep_original_locations() {
    for prefix in [
        "const result = values as import('types').Entry[];",
        "type Slots = Record<string, (() => import('view').Node[]) | undefined>;",
        "type Slots = { render: () => import('view').Node[] };",
        "type Factory = new () => import('view').Node;",
        "const result = values satisfies import('types').Entry[];",
    ] {
        complete_with_original_hit("library.ts", &format!("{prefix}\nconst alg = 'RS256';"));
    }
}
#[test]
fn dynamic_imports_and_nearby_invalid_expressions_are_not_retyped() {
    for source in [
        "const load = () => import('view').then(value => value);\nconst alg = 'RS256';",
        "async function load() { const v = await import('view'); return v; }\nconst alg = 'RS256';",
        "const v = (import('view')) as unknown;\nconst alg = 'RS256';",
    ] {
        complete_with_original_hit("library.ts", source);
    }
    for source in [
        "const load = () => import('view').Node[];",
        "const v = value as import('view').;",
        "type Slots = Record<string, (() => import('view').Node[) | undefined>;",
        "const load = import(variable).Node[];",
        "type T = import('x').123;\nconst alg='RS256';",
    ] {
        assert_eq!(
            ast("library.ts", source).status(),
            DetectorStatus::Partial,
            "{source}"
        );
    }
}
#[test]
fn dockerfile_names_override_source_suffixes_and_keep_lexical_inspection() {
    let text = "FROM image:latest\nRUN command --option\nENV TOKEN_ALG RS256\n";
    for name in [
        "Dockerfile",
        "Dockerfile.java",
        "Dockerfile.go",
        "Dockerfile.ts",
        "Dockerfile.py",
        "Dockerfile.production.rb",
        "nested/Dockerfile.rs",
    ] {
        assert_eq!(
            ast(name, text).status(),
            DetectorStatus::NotApplicable,
            "{name}"
        );
        let o = RegexDetector
            .detect_detailed(
                &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert!(
            o.detections().iter().any(|d| d.rule_id == "JWT_RS256"),
            "{name}"
        );
    }
    for name in [
        "DockerfileFactory.java",
        "Dockerfiles.java",
        "Dockerfile.java/Actual.java",
        "service.java",
    ] {
        assert_eq!(ast(name, text).status(), DetectorStatus::Partial, "{name}");
    }
    complete_with_original_hit("DockerfileFactory.ts", "const alg = 'RS256';");
}
fn dependency(text: &str) -> pqc_scan_core::DetectorOutcome {
    DependencyDetector
        .detect_observed(
            &ScannableFile::from_bytes("Gemfile.lock".into(), text.as_bytes().to_vec()),
            &rules(),
        )
        .unwrap()
}
#[test]
fn bundler_platform_records_and_standard_metadata_complete_without_guessing_versions() {
    for (platform, record, version) in [
        ("arm64-darwin", "1.17.4-arm64-darwin", "1.17.4"),
        ("aarch64-linux", "1.17.4-aarch64-linux-gnu", "1.17.4"),
        ("x86_64-linux-musl", "1.17.4-x86_64-linux-musl", "1.17.4"),
        ("java", "3.0.0-java", "3.0.0"),
        ("custom_os-v1", "1.2.3-custom_os-v1", "1.2.3"),
    ] {
        let source=format!("GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl ({record})\n\nPLATFORMS\n  {platform}\n\nDEPENDENCIES\n  openssl\n\nBUNDLED WITH\n  4.0.12\n");
        let o = dependency(&source);
        assert_eq!(o.status(), DetectorStatus::Complete, "{platform}");
        let hit = o
            .detections()
            .iter()
            .find(|d| d.rule_id == "__DEPENDENCY_INVENTORY__")
            .unwrap();
        assert_eq!(hit.evidence.metadata["dep_version"], record);
        assert_eq!(
            hit.evidence.metadata["dep_purl"],
            format!("pkg:gem/openssl@{version}")
        );
    }
}
#[test]
fn undeclared_platforms_bad_versions_and_bad_bundler_metadata_remain_incomplete() {
    for spec in [
        "1.2.3-unknown",
        "1.2.3-alpha",
        "1.2.3+garbage",
        "1..2-arm64-darwin",
    ] {
        let text=format!("GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl ({spec})\n\nPLATFORMS\n  arm64-darwin\n\nBUNDLED WITH\n  4.0.12\n");
        assert_eq!(
            dependency(&text).status(),
            DetectorStatus::Partial,
            "{spec}"
        );
    }
    for metadata in ["  nonsense", "  4.0.12 extra"] {
        let text=format!("GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl (3.0.0)\n\nBUNDLED WITH\n{metadata}\n");
        assert_eq!(dependency(&text).status(), DetectorStatus::Partial);
    }
}

#[test]
fn dockerfile_named_real_sources_are_still_parsed_as_their_languages() {
    complete_with_original_hit(
        "Dockerfile.java",
        "public class Dockerfile { String alg = \"RS256\"; }",
    );
    complete_with_original_hit("Dockerfile.ts", "const alg = 'RS256';");
    complete_with_original_hit("Dockerfile.py", "alg = 'RS256'");
    let f = ScannableFile::from_bytes(
        "Dockerfile.go".into(),
        b"package image\nimport \"crypto/rsa\"\n".to_vec(),
    );
    assert_eq!(
        TreeSitterDetector
            .detect_detailed(&f, &rules())
            .unwrap()
            .status(),
        DetectorStatus::Complete
    );
    assert!(RegexDetector
        .detect(&f, &rules())
        .unwrap()
        .iter()
        .any(|d| d.rule_id == "GO_TLS_RSA_IMPORT"));
    for name in ["Dockerfile.go", "dockerfile.ts", "DOCKERFILE.JAVA"] {
        let text="\u{feff}# syntax=docker/dockerfile:1\n\narg IMAGE=base\nfrom ${IMAGE}\nRUN echo 'import \"crypto/rsa\"'\nENV TOKEN_ALG RS256\n";
        assert_eq!(ast(name, text).status(), DetectorStatus::NotApplicable);
        let o = RegexDetector
            .detect(
                &ScannableFile::from_bytes(name.into(), text.as_bytes().to_vec()),
                &rules(),
            )
            .unwrap();
        assert!(o.iter().all(|d| !matches!(
            d.rule_id.as_str(),
            "GO_TLS_RSA_IMPORT" | "API_GO_CRYPTO_RSA"
        )));
        assert!(o.iter().any(|d| d.rule_id == "JWT_RS256"));
    }
}
