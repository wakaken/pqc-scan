use super::*;

#[test]
fn go_metadata_accepts_only_complete_declarations_without_evaluation() {
    let valid = [
        ("tool", "example.org/tool"),
        ("ignore", "./generated"),
        ("toolchain", "default"),
        ("toolchain", "go1.24.1"),
        ("godebug", "tlsmlkem=0"),
        ("godebug", "key_1=word.2-3"),
        ("replace", "example.org/old => ./local"),
        ("replace", "example.org/old => /local"),
        (
            "replace",
            "example.org/old v1.0.0 => example.org/new v1.2.3",
        ),
        ("replace", "example.org/old => example.org/new v1.2.3"),
        ("exclude", "example.org/pkg v1.2.3"),
        ("retract", "v1.2.3"),
        ("retract", "[v1.0.0, v1.2.3]"),
    ];
    for (kind, value) in valid {
        assert!(valid_go_metadata(kind, value), "{kind} {value}");
    }
    let invalid = [
        ("tool", ""),
        ("tool", "bad?name"),
        ("tool", "example.org/tool extra"),
        ("ignore", ""),
        ("ignore", "bad\0name"),
        ("ignore", "a b"),
        ("ignore", "bad\"name"),
        ("toolchain", ""),
        ("toolchain", "go1.x"),
        ("toolchain", "default extra"),
        ("godebug", ""),
        ("godebug", "=1"),
        ("godebug", "key="),
        ("godebug", "a-=1"),
        ("godebug", "key=bad/value"),
        ("godebug", "a=1 b=2"),
        ("replace", ""),
        ("replace", "=> ./local"),
        ("replace", "a b c => ./local"),
        ("replace", "a =>"),
        ("replace", "a => b v1.0.0 extra"),
        ("replace", "bad?name => ./local"),
        ("replace", "a bad-version => ./local"),
        ("replace", "a => ./local extra"),
        ("replace", "a => ./bad\"path"),
        ("replace", "a => other"),
        ("replace", "a => bad?name v1.0.0"),
        ("replace", "a => other bad-version"),
        ("exclude", "a"),
        ("exclude", "bad?name v1.0.0"),
        ("exclude", "a bad-version"),
        ("retract", "[v1.0.0]"),
        ("retract", "[bad, v1.0.0]"),
        ("unknown", "a"),
    ];
    for (kind, value) in invalid {
        assert!(!valid_go_metadata(kind, value), "{kind} {value}");
    }
}

#[test]
fn go_blocks_and_incomplete_grammar_never_become_complete_inventory() {
    for source in [
        "module example.org/app\ngo 1.24\nrequire (\nexample.org/a v1.0.0\n)\n",
        "godebug (\ntlsmlkem=0\n)\nrequire example.org/a v1.0.0\n",
    ] {
        let p = go(source).unwrap();
        assert_eq!(p.status, DetectorStatus::Complete);
        assert_eq!(p.items.len(), 1);
    }
    for source in [
        "godebug (\nbad-value\n)\n",
        "require (\n",
        ")\n",
        "require (\nrequire (\n)\n",
        "module bad?name\n",
        "go 1.x\n",
        "go\n",
        "go 1.24\ngo 1.24\n",
        "require example.org/a bad-version\n",
        "require example.org/a\n",
        "unrecognized a\n",
    ] {
        assert_ne!(
            go(source).unwrap().status,
            DetectorStatus::Complete,
            "{source}"
        );
    }
    for source in ["v1.2.3", "v1.2.3-rc.1", "v1.2.3+build.1"] {
        assert!(valid_go_module_version(source));
    }
    for source in ["v01.2.3", "v1.2.3-01", "1.2.3"] {
        assert!(!valid_go_module_version(source));
    }
}

#[test]
fn quoted_comments_continuations_and_limits_preserve_explicit_incompleteness() {
    assert_eq!(comment("'a#b' # tail", '#', true), "'a#b' ");
    assert_eq!(comment("\"a#b\"#tail", '#', false), "\"a#b\"");
    assert_eq!(comment("name#fragment", '#', true), "name#fragment");
    assert_eq!(
        logical_lines("openssl\\\n", true),
        vec![(1, "openssl \\".into())]
    );
    assert_eq!(
        inspect("go.mod", &"\n".repeat(50001))
            .unwrap()
            .unwrap()
            .status,
        DetectorStatus::Truncated
    );
    let b = Build::new(DetectorStatus::Truncated);
    assert_eq!(finish(b, true).unwrap().status, DetectorStatus::Truncated);
    let mut b = Build::new(DetectorStatus::Complete);
    for _ in 0..10000 {
        b.add(
            "pypi",
            "openssl",
            Some("1.0"),
            None,
            None,
            Origin::Manifest,
            1,
            false,
        );
    }
    preserve(&mut b, "requirements.txt", "openssl==1.0", 7);
    assert_eq!(b.items.len(), 10000);
    assert_eq!(b.status, DetectorStatus::Truncated);
}

#[test]
fn requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values() {
    let p = requirements("cryptography>=1.0,!=2.0\nparamiko\n").unwrap();
    assert_eq!(p.status, DetectorStatus::Complete);
    assert_eq!(p.items.len(), 2);
    assert_eq!(p.items[0].version, "1.0");
    for s in [
        "cryptography>=1.*\n",
        "cryptography===1.*\n",
        "cryptography~=1.*\n",
        "cryptography==1.*\n",
        "cryptography!=1.*\n",
    ] {
        let p = requirements(s).unwrap();
        if s.contains(">=") || s.contains("~=") {
            assert_ne!(p.status, DetectorStatus::Complete);
        } else {
            assert_eq!(p.status, DetectorStatus::Complete);
        }
    }
    for s in [
        "source 'https://rubygems.org'\n",
        "source \"https://rubygems.org\"\n",
    ] {
        assert_eq!(gemfile(s).unwrap().status, DetectorStatus::Complete);
    }
    let p = gemfile("gem 'openssl', '~> 3.0', require: false\ngem \"jwt\"\n").unwrap();
    assert_eq!(p.status, DetectorStatus::Complete);
    assert_eq!(p.items.len(), 2);
    for s in [
        "gem 'openssl\"\n",
        "gem 'openssl', 'bad'\n",
        "gem 'openssl', `execute`\n",
        "gem \"openssl', '1.0'\n",
    ] {
        assert_ne!(gemfile(s).unwrap().status, DetectorStatus::Complete, "{s}");
    }
}

#[test]
fn lock_origin_and_platform_correspondence_do_not_invent_registry_resolution() {
    let platforms = std::collections::BTreeSet::from(["x86_64-linux", "arm64-darwin"]);
    assert_eq!(
        gemlock_registry_version("1.2.3-x86_64-linux-gnu", &platforms),
        Some("1.2.3")
    );
    assert_eq!(gemlock_registry_version("1.2.3-ruby", &platforms), None);
    assert_eq!(gemlock_registry_version(&"x".repeat(257), &platforms), None);
    for source in [
        "GIT\n  remote: git@example.org/repo\n  revision: abcdef1\n  specs:\n    openssl (1.0.0)\nDEPENDENCIES\n  openssl!\n",
        "PATH\n  remote: ../local\n  specs:\n    openssl (1.0.0)\n",
        "GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl (1.0.0)\n      dependency (>= 1)\nPLATFORMS\n  ruby\nBUNDLED WITH\n  2.5.0\nCHECKSUMS\n  openssl (1.0.0) sha256=synthetic\n",
    ] { let p=gemlock(source).unwrap();assert_eq!(p.status,DetectorStatus::Complete,"{source}");assert_eq!(p.items.len(),1); }
    for source in [
        "GIT\n  specs:\n    openssl (1.0.0)\n",
        "GIT\n  remote: git@example.org/repo\n  specs:\n    openssl (1.0.0)\n",
        "GIT\n  revision: abcdef1\nPLATFORMS\n  ruby\n",
        "PATH\n  specs:\n    openssl (1.0.0)\nPLATFORMS\n  ruby\n",
        "GIT\n  remote: x\n  revision: zzzzzzz\n",
        "GIT\n  remote: x\n  revision: abc\n",
        "GEM\n  specs:\n  remote: https://rubygems.org/\n",
        "UNKNOWN\n  value\n",
        "BUNDLED WITH\nnot-indented\n",
        "CHECKSUMS\n  invalid\0checksum\n",
        "PLATFORMS\n  bad?platform\n",
    ] {
        assert_ne!(
            gemlock(source).unwrap().status,
            DetectorStatus::Complete,
            "{source}"
        );
    }
}

#[test]
fn maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete() {
    for xml in [
        "<!ENTITY sample 'value'><project/>",
        "<!DOCTYPE project><project/>",
    ] {
        assert_eq!(pom(xml).unwrap().status, DetectorStatus::Unsupported);
    }
    for xml in [
        "<project extra='x'/>","<project><dependencies extra='x'/></project>",
        "<project><modelVersion>4</modelVersion><modelVersion>4</modelVersion></project>",
        "<project><unknown/></project>","<project><dependencies><dependency><artifactId>a</artifactId></dependency></dependencies></project>",
        "<project><dependencies><dependency><groupId>${g}</groupId><artifactId>a</artifactId></dependency></dependencies></project>",
        "<project><dependencies><dependency><groupId>g</groupId><artifactId>${a}</artifactId></dependency></dependencies></project>",
    ] { assert_ne!(pom(xml).unwrap().status,DetectorStatus::Complete,"{xml}"); }
    let p=pom("<project xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' xsi:schemaLocation='urn:a urn:b'/>").unwrap();
    assert_eq!(p.status, DetectorStatus::Complete);
    let p=pom("<project><profiles><profile><id>x</id><dependencies><dependency><groupId>org.example</groupId><artifactId>lib</artifactId><version>1.0</version></dependency></dependencies></profile></profiles></project>").unwrap();
    assert_eq!(p.items.len(), 1);
    assert!(matches!(p.items[0].origin, Origin::ManifestOptional));
    assert_eq!(
        p.items[0].context,
        Some("conditional_profile_declaration_not_effective")
    );
    let p=pom("<project><dependencies><dependency><groupId>g</groupId><artifactId>a</artifactId><version>${missing}</version></dependency></dependencies></project>").unwrap();
    assert_ne!(p.status, DetectorStatus::Complete);
    assert_eq!(p.items.len(), 1);
    assert!(matches!(p.items[0].origin, Origin::Unknown));
    assert_eq!(
        p.items[0].context,
        Some("unresolved_property_not_effective")
    );
    let context = super::super::maven_context::Context {
        properties: Default::default(),
        profiles: vec![std::collections::BTreeMap::from([
            ("g".into(), "org.example".into()),
            ("a".into(), "lib".into()),
        ])],
        limited: false,
    };
    let p=pom_with_context("<project><dependencies><dependency><groupId>${g}</groupId><artifactId>${a}</artifactId></dependency></dependencies></project>",&context).unwrap();
    assert_eq!(p.items.len(), 1);
    assert!(matches!(p.items[0].origin, Origin::ManifestOptional));
}

#[test]
fn maven_expansion_keeps_unresolved_cycles_and_bounds_each_intermediate() {
    let props = std::collections::BTreeMap::from([
        ("big".into(), "x".repeat(4097)),
        ("cycle".into(), "${cycle}".into()),
    ]);
    assert_eq!(expand_maven("${big}", &props), "${big}");
    let tail = "x".repeat(4097) + "${missing}";
    assert_eq!(expand_maven(&tail, &props), tail);
    assert_eq!(expand_maven("${cycle}", &props), "${cycle}");
    assert_eq!(expand_maven("${unknown}", &props), "${unknown}");
}

#[test]
fn rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component() {
    for invalid in [
        "<profiles><profile><dependencies><dependency><groupId>bad?</groupId><artifactId>lib</artifactId><version>1.0</version></dependency></dependencies></profile></profiles>",
        "<dependencies><dependency><groupId>bad?</groupId><artifactId>lib</artifactId><version>${missing}</version></dependency></dependencies>",
    ] {
        let xml=format!("<project><dependencies><dependency><groupId>org.example</groupId><artifactId>public-lib</artifactId><version>1.0</version></dependency></dependencies>{invalid}</project>");
        let p=pom(&xml).unwrap();assert_ne!(p.status,DetectorStatus::Complete);
        assert_eq!(p.items.len(),1);assert_eq!(p.items[0].name,"org.example:public-lib");
        assert!(matches!(p.items[0].origin,Origin::Manifest));
        assert_eq!(p.items[0].context,None,"Rejected coordinate must not mutate preceding inventory");
    }
}

#[test]
fn malformed_fallback_records_and_metadata_edges_never_corrobate_resolution() {
    assert!(heuristic("requirements.txt", "bad..name==1.0").is_err());
    let mut b = Build::new(DetectorStatus::NotReported);
    preserve(&mut b, "requirements.txt", "bad..name==1.0", 1);
    assert!(b.items.is_empty());
    assert!(requirements("bad..name==1.0\n").is_err());
    assert_eq!(
        gemfile("\nsource 'https://rubygems.org'\n").unwrap().status,
        DetectorStatus::Complete
    );
    let label = "a".repeat(129);
    let lock=format!("GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl (1.2.3-{label})\nPLATFORMS\n  {label}\n");
    let parsed = gemlock(&lock).unwrap();
    assert_ne!(parsed.status, DetectorStatus::Complete);
    assert!(parsed.items[0].resolved.is_none());
    for source in [
        "GIT\n  remote: x\nPLATFORMS\n  ruby\n",
        "GEM\n  unsupported: x\n",
        "GIT\n  branch:\n",
        "PLATFORMS\n ruby\n",
        "BUNDLED WITH\n 2.5.0\n",
        "CHECKSUMS\n checksum\n",
    ] {
        assert_ne!(
            gemlock(source).unwrap().status,
            DetectorStatus::Complete,
            "{source}"
        );
    }
    let source="GIT\n  remote: https://rubygems.org/\n  revision: abcdef1\n  specs:\n    openssl (1.0.0)\n";
    let parsed = gemlock(source).unwrap();
    assert_eq!(parsed.status, DetectorStatus::Complete);
    assert!(matches!(parsed.items[0].origin, Origin::Git));
    assert!(parsed.items[0].resolved.is_none());
    let large = "x".repeat(4097);
    assert_eq!(expand_maven(&large, &Default::default()), large);
}
