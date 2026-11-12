//! New built-in source-fact paths through real exports; no fixture execution.
use serde_json::Value;
use std::{fs, path::Path, process::Command};
#[test]
fn local_rsa_source_positions_spans_and_secret_masking_survive_exports() {
    let root = std::env::temp_dir().join(format!("pqc043-cli-{}", std::process::id()));
    fs::create_dir_all(root.join("input")).unwrap();
    let text="const {generateKeyPairSync,createSign,createVerify}=require('node:crypto');\nconst 日='😀'; const {privateKey, publicKey}=generateKeyPairSync('rsa',{modulusLength:2048,password:'LEAK043_KEY_OPTIONS_END043'}); const signer=createSign('SHA256',{password:'LEAK043_FACTORY_END043'}); signer.sign(privateKey); const verifier=createVerify('SHA256'); verifier.verify(publicKey,signature); const password='LEAK043_NEIGHBOR_END043';\n";
    fs::write(root.join("input/app.js"), text).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("scan")
        .arg(root.join("input"))
        .args([
            "--format",
            "all",
            "--inventory-json",
            "--diagnostics",
            "--repository-scope",
            "synthetic/task043",
            "--rules-dir",
        ])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
        .arg("--out-dir")
        .arg(root.join("reports"))
        .output()
        .unwrap();
    assert!(output.status.success());
    let mut names: Vec<_> = fs::read_dir(root.join("reports"))
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    let mut expected = vec![
        "report.html",
        "report.md",
        "report.json",
        "report.sarif",
        "cbom.json",
        "dependency-sbom.json",
        "diagnostics.json",
        "scan.inventory.v1.json",
    ];
    expected.sort();
    assert_eq!(names, expected);
    for bytes in [&output.stdout, &output.stderr] {
        let s = String::from_utf8_lossy(bytes);
        assert!(!s.contains("LEAK043"));
        assert!(!s.contains("END043"));
        for middle in ["KEY_OPTIONS", "FACTORY", "NEIGHBOR"] {
            assert!(!s.contains(middle));
        }
    }
    for entry in fs::read_dir(root.join("reports")).unwrap() {
        let p = entry.unwrap().path();
        let s = fs::read_to_string(&p).unwrap();
        assert!(!s.contains("LEAK043"), "{}", p.display());
        assert!(!s.contains("END043"), "{}", p.display());
        for middle in ["KEY_OPTIONS", "FACTORY", "NEIGHBOR"] {
            assert!(!s.contains(middle), "{}", p.display());
        }
    }
    let report: Value =
        serde_json::from_slice(&fs::read(root.join("reports/report.json")).unwrap()).unwrap();
    let hits: Vec<_> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["rule_id"] == "API_NODE_CRYPTO_SIGN_RSA")
        .collect();
    assert_eq!(hits.len(), 4);
    let starts: Vec<_> = [
        "createSign('SHA256'",
        "signer.sign(privateKey)",
        "createVerify('SHA256'",
        "verifier.verify(publicKey",
    ]
    .iter()
    .map(|s| text.find(s).unwrap())
    .collect();
    for (hit, start) in hits.iter().zip(&starts) {
        assert_eq!(hit["location"]["line"], 2);
        let column = text[..*start].rsplit('\n').next().unwrap().chars().count() + 1;
        assert_eq!(hit["location"]["column"], column);
    }
    let native: Value =
        serde_json::from_slice(&fs::read(root.join("reports/scan.inventory.v1.json")).unwrap())
            .unwrap();
    let mut observed: Vec<_> = native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["rule_id"] == "API_NODE_CRYPTO_SIGN_RSA")
        .collect();
    observed.sort_by_key(|o| o["target"]["byte_start"].as_u64());
    assert_eq!(observed.len(), 4);
    let expected_ends: Vec<_> = [
        "createSign('SHA256'",
        "signer.sign(privateKey",
        "createVerify('SHA256'",
        "verifier.verify(publicKey",
    ]
    .iter()
    .map(|span| text.find(span).unwrap() + span.len())
    .collect();
    for ((o, start), expected_end) in observed.iter().zip(starts).zip(expected_ends) {
        assert_eq!(o["target"]["byte_start"], start);
        let end = o["target"]["byte_end"].as_u64().unwrap() as usize;
        assert_eq!(end, expected_end);
        assert!(text.is_char_boundary(end));
        assert!(end > start);
        assert!(!text[start..end].contains("LEAK043"));
    }
}

#[test]
fn compiled_default_bindings_work_with_copied_rules_outside_the_checkout() {
    let root = std::env::temp_dir().join(format!("pqc043-portable-{}", std::process::id()));
    for name in ["input", "rules", "cwd"] {
        fs::create_dir_all(root.join(name)).unwrap();
    }
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for name in ["pqc.yml", "tree_sitter.yml", "crypto_api.yml"] {
        fs::copy(
            repository.join("rules/default").join(name),
            root.join("rules").join(name),
        )
        .unwrap();
    }
    fs::write(
        root.join("input/app.js"),
        "import {generateKeyPairSync as build} from 'node:crypto'; build('ml-kem-768');\n",
    )
    .unwrap();
    fs::write(root.join("input/app.py"),"from cryptography.hazmat.primitives.asymmetric.mlkem import MLKEM768PrivateKey as Key\nkey = Key.generate()\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .current_dir(root.join("cwd"))
        .arg("scan")
        .arg(root.join("input"))
        .args(["--format", "json", "--rules-dir"])
        .arg(root.join("rules"))
        .arg("--out-dir")
        .arg(root.join("output"))
        .output()
        .unwrap();
    assert!(output.status.success());
    let result: Value =
        serde_json::from_slice(&fs::read(root.join("output/report.json")).unwrap()).unwrap();
    let findings = result["findings"].as_array().unwrap();
    assert_eq!(
        findings
            .iter()
            .filter(|f| f["rule_id"] == "PQC_NODE_API")
            .count(),
        1
    );
    assert_eq!(
        findings
            .iter()
            .filter(|f| f["rule_id"] == "PQC_PYTHON_API")
            .count(),
        2
    );
}
