use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn final_scan_distinguishes_tls_versions_and_rejects_malformed_go_retract() {
    let f = Fixture(std::env::temp_dir().join(format!("pqc-semantic-cli-{}", std::process::id())));
    let input = f.0.join("input");
    std::fs::create_dir_all(&input).unwrap();
    for version in [
        "TLSv1_0", "TLSv1_1", "TLSv1.0", "TLSv1.1", "TLSv1_2", "TLSv1_3", "TLSv1.2", "TLSv1.3",
    ] {
        std::fs::write(
            input.join(format!("{version}.js")),
            format!("const version = \"{version}\";\n"),
        )
        .unwrap();
    }
    std::fs::write(
        input.join("go.mod"),
        "module example.org/app\nrequire example.org/lib v1.2.3\nretract banana\n",
    )
    .unwrap();
    let output = f.0.join("output");
    let r = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(&input)
        .args([
            "--format",
            "all",
            "--diagnostics",
            "--inventory-json",
            "--rules-dir",
        ])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
        .arg("--out-dir")
        .arg(&output)
        .output()
        .unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let report: Value =
        serde_json::from_slice(&std::fs::read(output.join("report.json")).unwrap()).unwrap();
    let hits = report["findings"].as_array().unwrap();
    for version in ["TLSv1_0", "TLSv1_1", "TLSv1.0", "TLSv1.1"] {
        assert!(
            hits.iter()
                .any(|h| h["rule_id"] == "TS_TLS_LEGACY_VERSION_LITERAL"
                    && h["location"]["file"] == format!("{version}.js")),
            "{version}"
        );
    }
    assert!(hits.iter().all(|h| !matches!(
        h["location"]["file"].as_str(),
        Some("TLSv1_2.js" | "TLSv1_3.js" | "TLSv1.2.js" | "TLSv1.3.js")
    )));
    let diagnostics: Value =
        serde_json::from_slice(&std::fs::read(output.join("diagnostics.json")).unwrap()).unwrap();
    assert_eq!(diagnostics["coverage"]["partial_files"], 1);
    assert!(diagnostics["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["file"] == "go.mod"
            && d["code"] == "detector_partial"
            && d["detector"] == "dependency_detector"));
}

#[test]
fn a123_repairs_reach_all_reports_and_keep_stable_findings_and_invalid_controls() {
    let f = Fixture(std::env::temp_dir().join(format!("pqc-a123-cli-{}", std::process::id())));
    let input = f.0.join("input");
    std::fs::create_dir_all(&input).unwrap();
    for (name, text) in [
        ("types.ts", "type Shape<> = { v: true };\ntype View = () => import('view').Node[];\nconst alg = 'RS256';"),
        ("view.tsx", "const view = <A href='/start?first-key=1&second-key=2' other={x && y} />;\nconst alg = 'RS256';"),
        ("Dockerfile.java", "FROM base\nENV TOKEN_ALG RS256\n"),
        ("Gemfile.lock", "GEM\n  remote: https://rubygems.org/\n  specs:\n    openssl (3.3.0-arm64-darwin)\n\nPLATFORMS\n  arm64-darwin\n\nBUNDLED WITH\n  4.0.12\n"),
    ] { std::fs::write(input.join(name), text).unwrap(); }
    let mut expected_ids = None;
    for pass in 0..3 {
        if pass == 2 {
            std::fs::write(input.join("invalid.ts"), "type Shape<> = ;").unwrap();
        }
        let output = f.0.join(format!("output-{pass}"));
        let r = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(&input)
            .args([
                "--format",
                "all",
                "--diagnostics",
                "--inventory-json",
                "--cyclonedx-17",
                "--repository-scope",
                "a123",
                "--rules-dir",
            ])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .arg("--out-dir")
            .arg(&output)
            .output()
            .unwrap();
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(std::fs::read_dir(&output).unwrap().count(), 10);
        let read = |name| {
            serde_json::from_slice::<Value>(&std::fs::read(output.join(name)).unwrap()).unwrap()
        };
        let diagnostics = read("diagnostics.json");
        assert_eq!(
            diagnostics["coverage"]["partial_files"],
            if pass == 2 { 1 } else { 0 }
        );
        assert_eq!(diagnostics["coverage"]["analysis_failed_files"], 0);
        let report = read("report.json");
        let hits = report["findings"].as_array().unwrap();
        for name in ["types.ts", "view.tsx", "Dockerfile.java"] {
            assert!(
                hits.iter()
                    .any(|h| h["location"]["file"] == name && h["rule_id"] == "JWT_RS256"),
                "{name}"
            );
        }
        assert_eq!(
            read("report.sarif")["runs"][0]["results"]
                .as_array()
                .unwrap()
                .len(),
            hits.len()
        );
        let native = read("scan.inventory.v1.json");
        let gem = native["assets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["attributes"]["name"] == "openssl")
            .unwrap();
        assert_eq!(gem["attributes"]["reported_version"], "3.3.0-arm64-darwin");
        assert_eq!(gem["attributes"]["resolved_version"], "3.3.0");
        let mut ids = hits
            .iter()
            .map(|h| h["finding_id"].as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        ids.sort();
        if let Some(expected) = &expected_ids {
            assert_eq!(&ids, expected);
        } else {
            expected_ids = Some(ids);
        }
    }
}

#[test]
fn malformed_source_retains_findings_without_claiming_completed_analysis() {
    let f =
        Fixture(std::env::temp_dir().join(format!("pqc-partial-example-{}", std::process::id())));
    let input = f.0.join("input");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::write(
        input.join("example.js"),
        "const algorithm = 'RS256';\nfunction broken( {\n",
    )
    .unwrap();
    let mut saved = None;
    for valid in [false, true] {
        if valid {
            std::fs::write(
                input.join("example.js"),
                "const algorithm = 'RS256';\nfunction valid() {}\n",
            )
            .unwrap();
        }
        let output = f.0.join(if valid { "valid" } else { "broken" });
        let r = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(&input)
            .args([
                "--format",
                "all",
                "--diagnostics",
                "--inventory-json",
                "--cyclonedx-17",
                "--rules-dir",
            ])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .arg("--out-dir")
            .arg(&output)
            .output()
            .unwrap();
        assert!(r.status.success());
        assert_eq!(std::fs::read_dir(&output).unwrap().count(), 10);
        let d: Value =
            serde_json::from_slice(&std::fs::read(output.join("diagnostics.json")).unwrap())
                .unwrap();
        assert_eq!(d["coverage"]["read_ok_files"], 1);
        assert_eq!(d["coverage"]["partial_files"], if valid { 0 } else { 1 });
        let n: Value =
            serde_json::from_slice(&std::fs::read(output.join("report.json")).unwrap()).unwrap();
        let hit = n["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["rule_id"] == "JWT_RS256")
            .unwrap();
        assert_eq!(hit["location"]["line"], 1);
        if let Some(id) = &saved {
            assert_eq!(&hit["finding_id"], id);
        } else {
            saved = Some(hit["finding_id"].clone());
        }
    }
}
