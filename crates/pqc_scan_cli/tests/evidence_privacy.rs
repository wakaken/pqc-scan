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
        assert_eq!(count, 10);
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

#[test]
fn structured_private_sources_are_masked_in_every_cli_export() {
    let root = std::env::temp_dir().join(format!("pqc098-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("rules/control.yml"), r#"[{"id":"TECH_CONTROL","kind":"regex","category":"JWT","severity":"high","risk":"quantum-vulnerable","confidence":0.75,"migration_hint":"Preserve facts","pattern":"RS256","scope":"code"}]"#).unwrap();
    fs::write(root.join("input/jwk.json"), "{\n\"alg\":\"RS256\",\n\"d\":\"SYNTH098_PRIVATE_D_VALUE\",\n\"kty\":\"RSA\",\n\"n\":\"PUBLIC_MODULUS\",\n\"e\":\"AQAB\"\n}\n").unwrap();
    fs::write(root.join("input/values.py"), "algorithm = 'RS256'\nprivate = {'nested': ['SYNTH098_PRIVATE_NESTED_VALUE']}\nshared = 'SYNTH098_SHARED_VALUE'\nseed = [98217364198217364, 12836712836712836]\nscalar = b'SYNTH098_SCALAR_VALUE'\n").unwrap();
    fs::write(root.join("input/comments.js"), "// don't confuse quotes\nconst key = {alg: 'RS256', kty: 'RSA', d: { /* } */ value: 'SYNTH098_COMMENT_D' }, n: 'PUBLIC_MODULUS'}; privateValue = 0; scalar = 98217364198217364n; private = { /* } */ value: 'SYNTH098_COMMENT_PRIVATE' }; private = (17 + Lookup('PUBLIC_EXPRESSION'));\n").unwrap();
    fs::write(root.join("input/Synthetic.java"), "// unpaired \"\nclass Synthetic { String algorithm = \"RS256\"; long scalar = 98217364198217364L; }\n").unwrap();
    fs::write(root.join("input/comments-v5.rs"), "const ALGORITHM: &str = \"RS256\"; /* { examples aren't code. { kty: 'RSA', d: 'SYNTH098_COMMENT_PROSE', n: 'PUBLIC_MODULUS', e: 'AQAB' } */\n// unpaired \" scalar = 'SYNTH098_COMMENT_SCALAR';\nfn synthetic() { let private = [ /* outer /* inner */ ] */ \"SYNTH098_NESTED_COMMENT\" ]; let scalar = 98217364198217364u64; let scalar = 98217364198217364f32; }\n").unwrap();
    fs::write(root.join("input/comments-v6.rs"), "const ALGORITHM: &str = \"RS256\"; /* Note: \" scalar = 'SYNTH098_NOTE_SCALAR'; */\n// NOTE: 'do not publish {kty:\"RSA\",d:\"SYNTH098_NOTE_D\",n:\"PUBLIC_MODULUS\"}\n/* description: \"scalar = 'SYNTH098_DESCRIPTION_SCALAR';\" */\n").unwrap();
    fs::write(
        root.join("input/Synthetic.cs"),
        "class Synthetic { string algorithm = \"RS256\"; decimal scalar = 98217364198217364M; }\n",
    )
    .unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(root.join("input"))
        .args([
            "--format",
            "all",
            "--diagnostics",
            "--inventory-json",
            "--repository-scope",
            "synthetic/structured-privacy",
            "--rules-dir",
        ])
        .arg(root.join("rules"))
        .arg("--out-dir")
        .arg(root.join("out"))
        .output()
        .unwrap();
    assert!(status.status.success());
    for stream in [&status.stdout, &status.stderr] {
        assert!(!String::from_utf8_lossy(stream).contains("SYNTH098"));
    }
    let mut count = 0;
    for entry in fs::read_dir(root.join("out")).unwrap() {
        let text = fs::read_to_string(entry.unwrap().path()).unwrap();
        assert!(
            !text.contains("SYNTH098"),
            "synthetic private value escaped"
        );
        assert!(!text.contains("98217364198217364"));
        assert!(!text.contains("12836712836712836"));
        count += 1;
    }
    assert_eq!(count, 9);
    let report: Value =
        serde_json::from_slice(&fs::read(root.join("out/report.json")).unwrap()).unwrap();
    assert_eq!(report["findings"].as_array().unwrap().len(), 7);
    assert!(report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .all(|f| f["evidence"]["match"] == "RS256"));
    let inventory: Value =
        serde_json::from_slice(&fs::read(root.join("out/scan.inventory.v1.json")).unwrap())
            .unwrap();
    assert_eq!(
        inventory["legacy_associations"].as_array().unwrap().len(),
        7
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn serialized_jwk_and_other_prime_values_are_masked_in_both_languages() {
    let root = std::env::temp_dir().join(format!("pqc-v7-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("rules/control.yml"), r#"[{"id":"TECH_CONTROL","kind":"regex","category":"JWT","severity":"high","risk":"quantum-vulnerable","confidence":0.75,"migration_hint":"Preserve facts","pattern":"RS256","scope":"code"}]"#).unwrap();
    for (file, source) in [
        (
            "single.js",
            r#"const algorithm = 'RS256'; const jwk = '{"kty":"RSA","d":"SYNTH_V7_CLI_PRIVATE_D","n":"PUBLIC_N","e":"AQAB"}';"#,
        ),
        (
            "template.js",
            r#"const algorithm = 'RS256'; const jwk = `{"kty":"RSA","oth":[{"r":"SYNTH_V7_CLI_PRIVATE_R","d":"SYNTH_V7_CLI_PRIVATE_OTHER_D","t":"SYNTH_V7_CLI_PRIVATE_T"}],"n":"PUBLIC_N","e":"AQAB"}`;"#,
        ),
        (
            "raw.py",
            r#"algorithm = 'RS256'; jwk = r'{"kty":"RSA","d":"SYNTH_V7_CLI_PRIVATE_RAW","n":"PUBLIC_N","e":"AQAB"}'"#,
        ),
        (
            "public.js",
            r#"const algorithm = 'RS256'; const jwk = '{"kty":"EC","x":"PUBLIC_X","y":"PUBLIC_Y","r":"PUBLIC_R","t":"PUBLIC_T"}'; const math = {d:'PUBLIC_D'};"#,
        ),
    ] {
        fs::write(root.join("input").join(file), source).unwrap();
    }
    let mut expected_findings = None;
    for language in ["en", "ja"] {
        let out = root.join(language);
        let status = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(root.join("input"))
            .args([
                "--format",
                "all",
                "--diagnostics",
                "--inventory-json",
                "--lang",
                language,
                "--rules-dir",
            ])
            .arg(root.join("rules"))
            .arg("--out-dir")
            .arg(&out)
            .output()
            .unwrap();
        assert!(status.status.success());
        for stream in [&status.stdout, &status.stderr] {
            assert!(!String::from_utf8_lossy(stream).contains("SYNTH_V7_CLI_PRIVATE"));
        }
        let mut count = 0;
        for entry in fs::read_dir(&out).unwrap() {
            let text = fs::read_to_string(entry.unwrap().path()).unwrap();
            assert!(!text.contains("SYNTH_V7_CLI_PRIVATE"));
            count += 1;
        }
        assert_eq!(count, 9);
        let report: Value =
            serde_json::from_slice(&fs::read(out.join("report.json")).unwrap()).unwrap();
        let findings = report["findings"].as_array().unwrap();
        assert_eq!(findings.len(), 4);
        assert!(findings
            .iter()
            .all(|finding| finding["evidence"]["match"] == "RS256"));
        let text = report.to_string();
        for public in [
            "PUBLIC_N", "AQAB", "PUBLIC_X", "PUBLIC_Y", "PUBLIC_R", "PUBLIC_T", "PUBLIC_D",
        ] {
            assert!(text.contains(public));
        }
        if let Some(expected) = &expected_findings {
            assert_eq!(findings, expected);
        } else {
            expected_findings = Some(findings.clone());
        }
        let inventory: Value =
            serde_json::from_slice(&fs::read(out.join("scan.inventory.v1.json")).unwrap()).unwrap();
        assert_eq!(
            inventory["legacy_associations"].as_array().unwrap().len(),
            4
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn printed_private_key_dumps_are_hidden_in_cli_reports_with_public_hex_controls() {
    let root = std::env::temp_dir().join(format!("pqc-printed-dump-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("rules/control.yml"), "id: PRINTED_DUMP_CONTROL\nkind: regex\ncategory: Control\nseverity: low\nrisk: quantum-uncertain\nconfidence: 0.5\nmigration_hint: Review\npattern: '(?:11:22:33:44|ab:cd:ef:09)'\nscope: code\n").unwrap();
    fs::write(root.join("input/private.rs"), "const expected: &str = r#\"Private-Key: (256 bit)\r\npriv:\r\n  11:22:33:44:55:66\r\npub:\r\n  04:ca:fe:00\r\n\"#;\r\nconst tail: &str = \"PUBLIC_TAIL\";\r\n").unwrap();
    fs::write(
        root.join("input/public.rs"),
        "const expected: &str = \"ED25519 Public-Key:\\npub:\\n  ab:cd:ef:09\";\n",
    )
    .unwrap();
    let mut stable_findings = None;
    for language in ["en", "ja"] {
        let out = root.join(language);
        let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(root.join("input"))
            .args([
                "--format",
                "all",
                "--diagnostics",
                "--inventory-json",
                "--repository-scope",
                "synthetic/printed-dump",
                "--lang",
                language,
                "--rules-dir",
            ])
            .arg(root.join("rules"))
            .arg("--out-dir")
            .arg(&out)
            .output()
            .unwrap();
        assert!(result.status.success());
        for stream in [&result.stdout, &result.stderr] {
            assert!(!String::from_utf8_lossy(stream).contains("11:22:33:44"));
        }
        for entry in fs::read_dir(&out).unwrap() {
            let text = fs::read_to_string(entry.unwrap().path()).unwrap();
            assert!(!text.contains("11:22:33:44"));
            assert!(!text.contains("55:66"));
        }
        let report: Value =
            serde_json::from_slice(&fs::read(out.join("report.json")).unwrap()).unwrap();
        let findings = report["findings"].as_array().unwrap();
        assert_eq!(findings.len(), 2);
        let public = findings
            .iter()
            .find(|f| f["location"]["file"] == "public.rs")
            .unwrap();
        assert_eq!(public["evidence"]["match"], "ab:cd:ef:09");
        let private = findings
            .iter()
            .find(|f| f["location"]["file"] == "private.rs")
            .unwrap();
        assert_eq!(private["location"]["line"], 3);
        assert_eq!(private["location"]["column"], 3);
        let ids: Vec<_> = findings
            .iter()
            .map(|f| (f["finding_id"].clone(), f["location"].clone()))
            .collect();
        if let Some(expected) = &stable_findings {
            assert_eq!(&ids, expected);
        } else {
            stable_findings = Some(ids);
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ruby_private_scalar_aliases_are_masked_before_clipping_in_all_exports() {
    let root = std::env::temp_dir().join(format!("pqc-ruby-scalar-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("rules/control.yml"), r#"[{"id":"RUBY_PUBLIC_CONTROL","kind":"regex","category":"CryptoAPI","severity":"medium","risk":"quantum-vulnerable","confidence":0.65,"migration_hint":"Review","pattern":"OpenSSL::PKey::EC\\.new","scope":"code"}]"#).unwrap();
    let value = "FAKE_RUBY_MIDDLE".repeat(100);
    let source = format!("def sample\n  material = \"合成FAKE_RUBY_FIRST\n{value}\"\n  key = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\")\n  key.private_key = OpenSSL::BN.new(material, 16)\n  key.public_key = OpenSSL::BN.new(\"PUBLIC_POINT\", 16)\nend\n");
    fs::write(root.join("input/public.rb"), source).unwrap();
    for (rule_name, rules) in [
        (
            "default",
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
        ),
        ("custom", root.join("rules")),
    ] {
        for lang in ["en", "ja"] {
            let out = root.join(format!("{rule_name}-{lang}"));
            let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
                .arg("scan")
                .arg(root.join("input"))
                .args([
                    "--format",
                    "all",
                    "--diagnostics",
                    "--inventory-json",
                    "--cyclonedx-17",
                    "--threads",
                    "1",
                    "--repository-scope",
                    "synthetic/ruby-scalar",
                    "--lang",
                    lang,
                    "--rules-dir",
                ])
                .arg(&rules)
                .arg("--out-dir")
                .arg(&out)
                .output()
                .unwrap();
            assert!(result.status.success());
            for stream in [&result.stdout, &result.stderr] {
                assert!(!String::from_utf8_lossy(stream).contains("FAKE_RUBY_"));
            }
            let mut count = 0;
            for entry in fs::read_dir(&out).unwrap() {
                let path = entry.unwrap().path();
                let text = fs::read_to_string(&path).unwrap();
                assert!(
                    !text.contains("FAKE_RUBY_"),
                    "Ruby scalar escaped in {}",
                    path.file_name().unwrap().to_string_lossy()
                );
                if path
                    .extension()
                    .is_some_and(|extension| extension == "json" || extension == "sarif")
                {
                    let _: Value = serde_json::from_str(&text).unwrap();
                }
                count += 1;
            }
            assert_eq!(count, 11);
            let report: Value =
                serde_json::from_slice(&fs::read(out.join("report.json")).unwrap()).unwrap();
            assert_eq!(report["findings"].as_array().unwrap().len(), 1);
            let finding = &report["findings"][0];
            assert_eq!(finding["location"]["line"], 4);
            assert!(serde_json::to_string(finding)
                .unwrap()
                .contains("PUBLIC_CURVE"));
            let native: Value =
                serde_json::from_slice(&fs::read(out.join("scan.inventory.v1.json")).unwrap())
                    .unwrap();
            let schema: Value =
                serde_json::from_str(include_str!("../../../schemas/native-scan-v1.schema.json"))
                    .unwrap();
            assert!(jsonschema::options()
                .with_draft(jsonschema::Draft::Draft7)
                .build(&schema)
                .unwrap()
                .is_valid(&native));
            assert_eq!(native["legacy_associations"].as_array().unwrap().len(), 1);
            assert!(!native["legacy_associations"][0]["observation_ids"]
                .as_array()
                .unwrap()
                .is_empty());
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ruby_private_scalar_comments_captures_and_wrappers_are_masked_in_all_exports() {
    let root = std::env::temp_dir().join(format!("pqc-ruby-review-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("rules/control.yml"), r#"[{"id":"RUBY_PUBLIC_CONTROL","kind":"regex","category":"CryptoAPI","severity":"medium","risk":"quantum-vulnerable","confidence":0.65,"migration_hint":"Review","pattern":"OpenSSL::PKey::EC\\.new","scope":"code"}]"#).unwrap();
    let wrapped = format!(
        "{}OpenSSL::PKey::EC.new(\"PUBLIC_WRAPPED_CURVE\"){}",
        "(".repeat(17),
        ")".repeat(17)
    );
    let source = format!(
        r#"def direct
  ec = OpenSSL::PKey::EC.new("PUBLIC_CURVE"); n = OpenSSL::BN.new("PUBLIC_BN", 16); ec.public_key = "PUBLIC_POINT"
  ec.private_key = OpenSSL::BN.new(
# public comment
"FAKE_REVIEW_DIRECT", 16)
end
def alias_input
  material = "FAKE_REVIEW_ALIAS"
  ec = OpenSSL::PKey::EC.new("PUBLIC_ALIAS_CURVE")
  ec.private_key = OpenSSL::BN.new(
# public comment
material, 16)
end
def capture
  material = "FAKE_REVIEW_CAPTURE"
  ec = OpenSSL::PKey::EC.new("PUBLIC_CAPTURE_CURVE")
  while flag; items.each {{ ec.private_key = material }}; end
end
def wrapped
  material = "FAKE_REVIEW_WRAPPED"
  ec = {wrapped}
  ec.private_key = material
end
"#
    );
    let mut source = source;
    for depth in [127, 128, 129, 130] {
        source.push_str(&format!(
            "def capture_{depth}\nmaterial = \"FAKE_REVIEW_CAPTURE_{depth}\"; ec = OpenSSL::PKey::EC.new(\"PUBLIC_DEEP_CURVE\")\n{}ec.private_key = OpenSSL::BN.new(material,16)\n{}end\n",
            "items.each {\n".repeat(depth),
            "}\n".repeat(depth),
        ));
    }
    for depth in [128, 129, 140] {
        source.push_str(&format!(
            "def ancestor_{depth}\nmaterial = \"FAKE_REVIEW_ANCESTOR_{depth}\"\n{}ec = OpenSSL::PKey::EC.new(\"PUBLIC_DEEP_CURVE\"); ec.private_key = OpenSSL::BN.new(material,16){}\nend\n",
            "if true; ".repeat(depth),
            "; end".repeat(depth),
        ));
    }
    fs::write(root.join("input/public.rb"), source).unwrap();
    for (rule_name, rules) in [
        (
            "default",
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
        ),
        ("custom", root.join("rules")),
    ] {
        for lang in ["en", "ja"] {
            let out = root.join(format!("{rule_name}-{lang}"));
            let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
                .arg("scan")
                .arg(root.join("input"))
                .args([
                    "--format",
                    "all",
                    "--diagnostics",
                    "--inventory-json",
                    "--cyclonedx-17",
                    "--threads",
                    "1",
                    "--repository-scope",
                    "synthetic/ruby-review",
                    "--lang",
                    lang,
                    "--rules-dir",
                ])
                .arg(&rules)
                .arg("--out-dir")
                .arg(&out)
                .output()
                .unwrap();
            assert!(result.status.success());
            for stream in [&result.stdout, &result.stderr] {
                assert!(!String::from_utf8_lossy(stream).contains("FAKE_REVIEW_"));
            }
            let mut count = 0;
            for entry in fs::read_dir(&out).unwrap() {
                let path = entry.unwrap().path();
                let text = fs::read_to_string(&path).unwrap();
                assert!(
                    !text.contains("FAKE_REVIEW_"),
                    "fake scalar escaped in {}",
                    path.display()
                );
                if path
                    .extension()
                    .is_some_and(|ext| ext == "json" || ext == "sarif")
                {
                    let _: Value = serde_json::from_str(&text).unwrap();
                }
                count += 1;
            }
            assert_eq!(count, 11);
            let report = fs::read_to_string(out.join("report.json")).unwrap();
            for control in ["PUBLIC_CURVE", "PUBLIC_BN", "PUBLIC_POINT", "16"] {
                assert!(report.contains(control), "missing public control {control}");
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ruby_long_receiver_and_multiline_material_names_are_masked_in_all_exports() {
    let root = std::env::temp_dir().join(format!("pqc-ruby-long-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    fs::create_dir_all(root.join("rules")).unwrap();
    fs::write(root.join("rules/control.yml"), r#"[{"id":"LONG_NAME_PUBLIC_CONTROL","kind":"regex","category":"CryptoAPI","severity":"medium","risk":"quantum-vulnerable","confidence":0.65,"migration_hint":"Review","pattern":"OpenSSL::PKey::EC\\.new","scope":"code"}]"#).unwrap();
    for mode in ["receiver", "material"] {
        let mut source = String::from("def public_control\nec = OpenSSL::PKey::EC.new(\"PUBLIC_CURVE\"); ec.public_key = \"PUBLIC_POINT\"; n = OpenSSL::BN.new(\"PUBLIC_BN\",16)\nend\n");
        for bytes in [511, 512, 513, 4096] {
            for unicode in [false, true] {
                let name = if unicode {
                    format!(
                        "r{}{}",
                        "名".repeat((bytes - 1) / 3),
                        "x".repeat((bytes - 1) % 3)
                    )
                } else {
                    "r".repeat(bytes)
                };
                let marker =
                    format!("FAKE_LONG_{mode}_{bytes}_{unicode}_ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                source.push_str(&format!("def boundary_{bytes}_{unicode}\n"));
                if mode == "receiver" {
                    source.push_str(&format!("material = \"{marker}\"\n{name} = OpenSSL::PKey::EC.new(\"PUBLIC_BOUNDARY_CURVE\")\n{name}.private_key = material\n"));
                } else {
                    source.push_str(&format!("{name} =\n \"{marker}\"\nec = OpenSSL::PKey::EC.new(\"PUBLIC_BOUNDARY_CURVE\")\nec.private_key = OpenSSL::BN.new({name},16)\n"));
                }
                source.push_str("end\n");
            }
        }
        fs::write(root.join("input/public.rb"), source).unwrap();
        for (rule_name, rules) in [
            (
                "default",
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
            ),
            ("custom", root.join("rules")),
        ] {
            for lang in ["en", "ja"] {
                let out = root.join(format!("{mode}-{rule_name}-{lang}"));
                let result = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
                    .arg("scan")
                    .arg(root.join("input"))
                    .args([
                        "--format",
                        "all",
                        "--diagnostics",
                        "--inventory-json",
                        "--cyclonedx-17",
                        "--threads",
                        "1",
                        "--repository-scope",
                        "synthetic/ruby-long-name",
                        "--lang",
                        lang,
                        "--rules-dir",
                    ])
                    .arg(&rules)
                    .arg("--out-dir")
                    .arg(&out)
                    .output()
                    .unwrap();
                assert!(result.status.success());
                for stream in [&result.stdout, &result.stderr] {
                    assert!(!String::from_utf8_lossy(stream).contains("FAKE_LONG_"));
                }
                let mut count = 0;
                for entry in fs::read_dir(&out).unwrap() {
                    let path = entry.unwrap().path();
                    let text = fs::read_to_string(&path).unwrap();
                    assert!(
                        !text.contains("FAKE_LONG_"),
                        "long-name scalar escaped in {}",
                        path.display()
                    );
                    if path
                        .extension()
                        .is_some_and(|ext| ext == "json" || ext == "sarif")
                    {
                        let _: Value = serde_json::from_str(&text).unwrap();
                    }
                    count += 1;
                }
                assert_eq!(count, 11);
                let report = fs::read_to_string(out.join("report.json")).unwrap();
                for control in ["PUBLIC_CURVE", "PUBLIC_POINT", "PUBLIC_BN", "16"] {
                    assert!(report.contains(control), "missing public control {control}");
                }
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}
