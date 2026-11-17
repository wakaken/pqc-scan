use super::*;

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
