use super::*;

#[test]
fn cbom_keeps_distinct_locations_when_paths_and_matches_contain_colons() {
    let first = Finding {
        finding_id: "first".into(),
        rule_id: "RULE".into(),
        category: "RSA".into(),
        risk: pqc_scan_rules::Risk::QuantumUncertain,
        severity: pqc_scan_rules::Severity::Info,
        confidence: 0.5,
        description: "Synthetic reference".into(),
        migration_hint: "Check source".into(),
        location: crate::Location {
            file: "source.rs".into(),
            line: 2,
            column: 1,
        },
        evidence: crate::Evidence {
            r#type: "regex_match".into(),
            r#match: "part.rs:3:RSA".into(),
            snippet_preview: String::new(),
            metadata: BTreeMap::new(),
        },
        recommended_actions: Vec::new(),
        source_snippet: None,
    };
    let mut second = first.clone();
    second.finding_id = "second".into();
    second.location.file = "source.rs:2:part.rs".into();
    second.location.line = 3;
    second.evidence.r#match = "RSA".into();
    let mut repeated = first.clone();
    repeated.finding_id = "same-reference-other-column".into();
    repeated.location.column = 8;
    let mut other_rule = first.clone();
    other_rule.finding_id = "other-rule".into();
    other_rule.rule_id = "OTHER".into();
    let mut other_match = second.clone();
    other_match.finding_id = "other-match".into();
    other_match.evidence.r#match = "ECDSA".into();

    let rows = build_cbom(&[first, second, repeated, other_rule, other_match]);
    let projection: Vec<_> = rows
        .iter()
        .map(|row| (row.location.as_str(), row.algorithm.as_str()))
        .collect();
    assert_eq!(
        projection,
        vec![
            ("source.rs:2", "part.rs:3:RSA"),
            ("source.rs:2:part.rs:3", "RSA"),
            ("source.rs:2", "part.rs:3:RSA"),
            ("source.rs:2:part.rs:3", "ECDSA"),
        ]
    );
}

#[test]
fn snippet_boundaries_and_each_private_marker_protect_displayed_source() {
    assert!(build_source_snippet(&[], 1, 2, 2).is_none());
    assert!(build_source_snippet(&["public".into()], 0, 2, 2).is_none());
    for before in [0, 1, usize::MAX] {
        for after in [0, 1, usize::MAX] {
            let snippet =
                build_source_snippet(&["public".into()], usize::MAX, before, after).unwrap();
            assert_eq!(snippet.lines.len(), 1);
            assert_eq!(snippet.lines[0].line, 1);
            assert!(snippet.lines[0].highlighted);
        }
    }
    for marker in ["RSA PRIVATE KEY", "PRIVATE KEY", "OPENSSH PRIVATE KEY"] {
        for direction in ["BEGIN", "END"] {
            let line = format!("-----{direction} {marker}-----");
            assert!(looks_like_private_key_line(&line));
            assert_eq!(sanitize_source_line(&line), "[masked-private-key-material]");
        }
        let text=format!("public\n-----BEGIN {marker}-----\nSYNTHETIC-CANARY\n-----END {marker}-----\npublic-after\n");
        let masked = mask_private_key_blocks(&text);
        assert!(!masked.contains("SYNTHETIC-CANARY"));
        assert!(masked.contains("public-after"));
    }
    for marker in [
        "RSA PRIVATE KEY",
        "PRIVATE KEY",
        "OPENSSH PRIVATE KEY",
        "EC PRIVATE KEY",
        "DSA PRIVATE KEY",
        "ENCRYPTED PRIVATE KEY",
    ] {
        assert!(contains_private_key_marker(&format!(
            "-----BEGIN {marker}-----"
        )));
    }
    assert!(!contains_private_key_marker("-----BEGIN PUBLIC KEY-----"));
}

#[test]
fn nameless_dependency_metadata_is_never_exported_as_a_named_component() {
    let detection = Detection {
        rule_id: "SYNTHETIC".into(),
        location: crate::Location {
            file: "Cargo.toml".into(),
            line: 1,
            column: 1,
        },
        evidence: crate::Evidence {
            r#type: "dependency".into(),
            r#match: "".into(),
            snippet_preview: "".into(),
            metadata: BTreeMap::from([("inventory_type".into(), "dependency".into())]),
        },
    };
    assert!(
        build_dependency_sbom(std::path::Path::new("."), std::slice::from_ref(&detection))
            .is_empty()
    );
    let mut named = detection;
    named
        .evidence
        .metadata
        .insert("dep_name".into(), "unrelated".into());
    assert_eq!(
        build_dependency_sbom(std::path::Path::new("."), &[named]).len(),
        1
    );
}

#[test]
fn undecodable_wide_bom_is_diagnosed_but_pkcs12_is_left_to_its_binary_detector() {
    let rules = RuleSet::default();
    for (name, unsupported) in [
        ("broken.txt", true),
        ("container.p12", false),
        ("container.pfx", false),
        ("container.PFX", false),
        ("container", true),
    ] {
        let file = ScannableFile::from_bytes(name.into(), vec![0xff, 0xfe, 0x00]);
        assert!(file.text.is_none());
        let result = inspect_file(&file, name, &rules, &[], false, None);
        assert_eq!(
            result.disposition,
            if unsupported {
                FileDisposition::Unsupported
            } else {
                FileDisposition::Unknown
            },
            "{name}"
        );
    }
}
