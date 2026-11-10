use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::{fs, process::Command};
#[test]
fn private_bodies_and_inline_credentials_never_survive_cli_report_paths() {
    let root = std::env::temp_dir().join(format!("pqc029-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    let fields = json!([{"id":"BODY_CONTROL","kind":"regex","category":"Control","severity":"low","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"LEAK029[A-Z0-9_]+","scope":"code"},{"id":"TECH_CONTROL","kind":"regex","category":"JWT","severity":"high","risk":"quantum-vulnerable","confidence":0.75,"migration_hint":"Preserve facts","pattern":"RS256","scope":"code"}]);
    fs::write(root.join("rules/control.yml"), fields.to_string()).unwrap();
    for (i, label) in [
        "RSA PRIVATE KEY",
        "PRIVATE KEY",
        "OPENSSH PRIVATE KEY",
        "EC PRIVATE KEY",
        "DSA PRIVATE KEY",
        "ENCRYPTED PRIVATE KEY",
    ]
    .iter()
    .enumerate()
    {
        fs::write(root.join(format!("input/key{i}.txt")),format!("RS256\n-----BEGIN {label}-----\nLEAK029A_SYNTHETIC_BODY_NOT_KEY_MATERIAL_Z029ZZ\n-----END {label}-----\n")).unwrap();
    }
    fs::write(
        root.join("input/unterminated.txt"),
        "RS256\n-----BEGIN EC PRIVATE KEY-----\nLEAK029B_SYNTHETIC_UNTERMINATED_BODY_Z029ZZ\n",
    )
    .unwrap();
    fs::write(root.join("input/設定.conf"),"RS256\npassword = \"LEAK029C_SYNTHETIC_CREDENTIAL_Z029ZZ\"\nurl = \"https://user:LEAK029D_SYNTHETIC_USERINFO_Z029ZZ@example.invalid/path?access_token=LEAK029E_SYNTHETIC_QUERY_Z029ZZ&mode=read\"\n").unwrap();
    // The old surrounding preview retained a middle fragment after truncation.
    let long_value = "MID029".repeat(30);
    fs::write(root.join("input/preview.txt"), format!("RS256 password=\"{long_value}\"\nhttps://example.invalid/?token=LEAK029F.jwt%21&mode=read\n")).unwrap();
    let mut expected_machine: Option<Vec<Value>> = None;
    for lang in ["en", "ja"] {
        let out = root.join(lang);
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(root.join("input"))
            .args([
                "--format",
                "all",
                "--diagnostics",
                "--inventory-json",
                "--repository-scope",
                "synthetic/privacy",
                "--policy-profile",
                "migration-review-v1",
                "--lang",
                lang,
                "--rules-dir",
            ])
            .arg(root.join("rules"))
            .arg("--out-dir")
            .arg(&out)
            .output()
            .unwrap();
        assert!(result.status.success());
        for stream in [&result.stdout, &result.stderr] {
            assert!(!String::from_utf8_lossy(stream).contains("LEAK029"));
        }
        let mut count = 0;
        for entry in fs::read_dir(&out).unwrap() {
            let path = entry.unwrap().path();
            let text = fs::read_to_string(&path).unwrap();
            assert!(
                !text.contains("LEAK029"),
                "canary survived in {}",
                path.file_name().unwrap().to_string_lossy()
            );
            assert!(!text.contains("Z029ZZ"), "partial canary survived");
            assert!(!text.contains("MID029"), "middle canary fragment survived");
            count += 1;
        }
        assert_eq!(count, 9);
        let mut native: Value =
            serde_json::from_slice(&fs::read(out.join("scan.inventory.v1.json")).unwrap()).unwrap();
        let mut map: BTreeMap<String, String> = std::iter::once((
            native["run"]["id"].as_str().unwrap().to_string(),
            "RUN".into(),
        ))
        .chain(
            native["observations"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(i, o)| (o["id"].as_str().unwrap().to_string(), format!("OBS-{i}"))),
        )
        .collect();
        assert!(native["legacy_associations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| !a["observation_ids"].as_array().unwrap().is_empty()));
        let mut assessment: Value =
            serde_json::from_slice(&fs::read(out.join("scan.assessment.v1.json")).unwrap())
                .unwrap();
        for (i, a) in assessment["assessments"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            map.insert(
                a["id"].as_str().unwrap().to_string(),
                format!("ASSESSMENT-{i}"),
            );
        }
        for (payload, schema) in [
            (
                &native,
                include_str!("../../../schemas/native-scan-v1.schema.json"),
            ),
            (
                &assessment,
                include_str!("../../../schemas/assessment-v1.schema.json"),
            ),
        ] {
            let schema: Value = serde_json::from_str(schema).unwrap();
            let validator = jsonschema::options()
                .with_draft(jsonschema::Draft::Draft7)
                .build(&schema)
                .unwrap();
            assert!(validator.is_valid(payload));
        }
        fn normalize(v: &mut Value, ids: &BTreeMap<String, String>) {
            match v {
                Value::String(s) => {
                    if let Some(id) = ids.get(s) {
                        *s = id.clone();
                    }
                }
                Value::Array(a) => {
                    for x in a {
                        normalize(x, ids);
                    }
                }
                Value::Object(o) => {
                    for x in o.values_mut() {
                        normalize(x, ids);
                    }
                }
                _ => {}
            }
        }
        native["run"]["generated_at"] = json!("TIME");
        normalize(&mut native, &map);
        normalize(&mut assessment, &map);
        let mut machine = vec![native, assessment];
        for file in [
            "report.json",
            "report.sarif",
            "cbom.json",
            "dependency-sbom.json",
            "diagnostics.json",
        ] {
            let mut v: Value = serde_json::from_slice(&fs::read(out.join(file)).unwrap()).unwrap();
            if let Some(object) = v.as_object_mut() {
                object.remove("generated_at");
            }
            machine.push(v);
        }
        if let Some(expected) = &expected_machine {
            for (i, (actual, expected)) in machine.iter().zip(expected.iter()).enumerate() {
                assert!(
                    actual == expected,
                    "technical machine facts differ in artifact index {i}"
                );
            }
        } else {
            expected_machine = Some(machine);
        }

        let report: serde_json::Value =
            serde_json::from_slice(&fs::read(out.join("report.json")).unwrap()).unwrap();
        assert_eq!(
            report["findings"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|f| f["rule_id"] == "TECH_CONTROL")
                .count(),
            9
        );
    }
    fs::remove_dir_all(root).unwrap();
}
