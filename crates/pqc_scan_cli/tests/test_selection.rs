use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn scan(root: &Path, out: &Path, exclude: bool) -> Value {
    let mut c = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
    c.arg("scan")
        .arg(root)
        .args([
            "--diagnostics",
            "--inventory-json",
            "--lang",
            "ja",
            "--rules-dir",
        ])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
        .arg("--out-dir")
        .arg(out);
    if exclude {
        c.arg("--exclude-tests");
    }
    let r = c.output().unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    serde_json::from_slice(&fs::read(out.join("report.json")).unwrap()).unwrap()
}

#[test]
fn custom_exclusions_are_recorded_and_change_only_selected_paths() {
    let f =
        Fixture(std::env::temp_dir().join(format!("pqc-custom-selection-{}", std::process::id())));
    let input = f.0.join("target/product");
    fs::create_dir_all(input.join("integration")).unwrap();
    fs::write(input.join("main.py"), "algorithm = 'RS256'").unwrap();
    fs::write(
        input.join("integration/custom_case.py"),
        "algorithm = 'RS512'",
    )
    .unwrap();
    let before = scan(&input, &f.0.join("before"), false);
    fs::write(input.join(".pqcignore"), "integration/\n").unwrap();
    let after = scan(&input, &f.0.join("after"), false);
    assert_eq!(before["summary"]["scanned_files"], 2);
    assert_eq!(after["summary"]["scanned_files"], 1);
    let before_ids: std::collections::BTreeSet<_> = before["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["location"]["file"] == "main.py")
        .map(|f| f["finding_id"].clone().to_string())
        .collect();
    let after_ids: std::collections::BTreeSet<_> = after["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["finding_id"].clone().to_string())
        .collect();
    assert!(!before_ids.is_empty());
    assert_eq!(before_ids, after_ids);
    let diagnostics: Value =
        serde_json::from_slice(&fs::read(f.0.join("after/diagnostics.json")).unwrap()).unwrap();
    assert!(diagnostics["scope"]["default_excludes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v.as_str().unwrap().contains(".pqcignore")));
    assert_eq!(diagnostics["coverage"]["excluded_enumerated_files"], 2);
    assert!(fs::read_to_string(f.0.join("after/report.html"))
        .unwrap()
        .contains(".pqcignore"));
}

#[test]
fn rust_inline_exclusion_preserves_product_and_ambiguous_cfg_in_all_exports() {
    let f =
        Fixture(std::env::temp_dir().join(format!("pqc-inline-selection-{}", std::process::id())));
    let input = f.0.join("product");
    fs::create_dir_all(&input).unwrap();
    let text = "const PROD: &str = \"RS256\";\n#[cfg(test)]\nmod checks { const ONLY_TEST: &str = \"RS512\"; }\n#[test]\nfn one() { let _ = \"ES384\"; }\n#[cfg(any(test, feature = \"runtime\"))]\nfn mixed() { let _ = \"ES512\"; }\nfn main() { let _ = \"日本語\"; let _ = \"RS384\"; }\n";
    fs::write(input.join("lib.rs"), text).unwrap();
    let before = scan(&input, &f.0.join("all"), false);
    let after = scan(&input, &f.0.join("filtered"), true);
    let lines = |v: &Value| {
        v["findings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["location"]["line"].as_u64().unwrap())
            .collect::<std::collections::BTreeSet<_>>()
    };
    assert!(lines(&before).contains(&3));
    assert!(lines(&before).contains(&5));
    assert_eq!(lines(&after), [1, 7, 8].into_iter().collect());
    assert_eq!(after["summary"]["scanned_files"], 1);
    let native: Value =
        serde_json::from_slice(&fs::read(f.0.join("filtered/scan.inventory.v1.json")).unwrap())
            .unwrap();
    let original: Value =
        serde_json::from_slice(&fs::read(f.0.join("all/scan.inventory.v1.json")).unwrap()).unwrap();
    for o in native["observations"].as_array().unwrap() {
        assert!(![3, 5].contains(&o["target"]["line"].as_u64().unwrap()));
        // Observation IDs are intentionally scan-local. Compare source-bound
        // facts instead of imposing cross-scan identity on those IDs.
        assert!(original["observations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|before| {
                before["target"] == o["target"]
                    && before["rule_id"] == o["rule_id"]
                    && before["method"] == o["method"]
                    && before["usage"] == o["usage"]
                    && before["assurance"] == o["assurance"]
            }));
    }
    let sarif: Value =
        serde_json::from_slice(&fs::read(f.0.join("filtered/report.sarif")).unwrap()).unwrap();
    for result in sarif["runs"][0]["results"].as_array().unwrap() {
        let line = result["locations"][0]["physicalLocation"]["region"]["startLine"]
            .as_u64()
            .unwrap();
        assert!(![3, 5].contains(&line));
    }
    let custom = f.0.join("custom-attribute");
    fs::create_dir_all(&custom).unwrap();
    fs::write(custom.join("lib.rs"), "use kept::test;\n#[test]\nfn production() { let _ = \"RS256\"; }\n#[cfg(test)]\nmod only_tests { const ALG: &str = \"RS512\"; }\n").unwrap();
    let guarded = scan(&custom, &f.0.join("custom-filtered"), true);
    assert_eq!(lines(&guarded), [3].into_iter().collect());
    fs::write(custom.join("lib.rs"), "#[kept::prod]\nmod app {\n#[cfg(test)]\npub fn production() { let _ = \"RS256\"; }\n}\n#[cfg(test)] mod confirmed { const ALG: &str = \"RS512\"; }\n").unwrap();
    let outer = scan(&custom, &f.0.join("outer-filtered"), true);
    assert_eq!(lines(&outer), [4].into_iter().collect());
}
#[test]
fn test_exclusion_is_optional_recorded_and_preserves_product_files_and_legacy_defaults() {
    let f =
        Fixture(std::env::temp_dir().join(format!("pqc-test-selection-{}", std::process::id())));
    let input = f.0.join("tests/product");
    fs::create_dir_all(&input).unwrap();
    let kept = [
        "src/main.py",
        "src/contest.py",
        "src/testing/util.py",
        "src/lib.rs",
    ];
    let excluded = [
        "tests/token.py",
        "src/test/java/Key.java",
        "__tests__/key.ts",
        "spec/key_spec.rb",
        "test_key.py",
        "key_test.go",
        "key.test.js",
        "key.spec.ts",
        "KeyTest.java",
    ];
    for name in kept.iter().chain(excluded.iter()) {
        let p = input.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(
            p,
            if name.ends_with(".rs") {
                "const ALG: &str = \"RS256\";\n#[cfg(test)] mod tests { const TEST_ALG: &str = \"RS512\"; }"
            } else {
                "alg = 'RS256'"
            },
        )
        .unwrap();
    }
    let all = scan(&input, &f.0.join("all"), false);
    let filtered = scan(&input, &f.0.join("filtered"), true);
    assert_eq!(all["summary"]["scanned_files"], kept.len() + excluded.len());
    assert_eq!(filtered["summary"]["scanned_files"], kept.len());
    for finding in filtered["findings"].as_array().unwrap() {
        assert!(kept.contains(&finding["location"]["file"].as_str().unwrap()));
    }
    for name in kept {
        assert!(
            filtered["findings"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["location"]["file"] == name),
            "{name}"
        );
    }
    let diagnostics: Value =
        serde_json::from_slice(&fs::read(f.0.join("filtered/diagnostics.json")).unwrap()).unwrap();
    assert!(diagnostics["scope"]["default_excludes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s.as_str().unwrap().contains("test directories")));
    assert_eq!(
        diagnostics["coverage"]["excluded_enumerated_files"],
        excluded.len()
    );
    let html = fs::read_to_string(f.0.join("filtered/report.html")).unwrap();
    assert!(html.contains("テスト用ディレクトリ・ファイル名"));
    assert!(fs::read_to_string(f.0.join("filtered/report.md"))
        .unwrap()
        .contains("テスト用のパスを除外"));
    let baseline: Value =
        serde_json::from_slice(&fs::read(f.0.join("all/diagnostics.json")).unwrap()).unwrap();
    assert!(!baseline["scope"]["default_excludes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s.as_str().unwrap().contains("test directories")));
    let direct = scan(&input.join("test_key.py"), &f.0.join("direct"), true);
    assert_eq!(direct["summary"]["scanned_files"], 0);
}
