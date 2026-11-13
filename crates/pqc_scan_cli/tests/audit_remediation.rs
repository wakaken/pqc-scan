//! Observable controls for the user-approved develop audit repairs.
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Scan {
    root: PathBuf,
    report: Value,
}
impl Drop for Scan {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
impl Scan {
    fn new(files: &[(&str, &str)], lang: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "pqc-audit-regression-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let input = root.join("input");
        std::fs::create_dir_all(&input).unwrap();
        for (name, text) in files {
            let p = input.join(name);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        }
        let r = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(&input)
            .args([
                "--format",
                "all",
                "--diagnostics",
                "--inventory-json",
                "--lang",
                lang,
                "--rules-dir",
            ])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .arg("--out-dir")
            .arg(root.join("report"))
            .output()
            .unwrap();
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        let report =
            serde_json::from_slice(&std::fs::read(root.join("report/report.json")).unwrap())
                .unwrap();
        Self { root, report }
    }
    fn findings(&self) -> &[Value] {
        self.report["findings"].as_array().unwrap()
    }
    fn hits(&self, file: &str, rule: &str) -> Vec<&Value> {
        self.findings()
            .iter()
            .filter(|f| f["location"]["file"] == file && f["rule_id"] == rule)
            .collect()
    }
    fn text(&self, file: &str) -> String {
        std::fs::read_to_string(self.root.join("report").join(file)).unwrap()
    }
}
#[test]
fn rsa_sizes_require_rsa_context_not_pqc_names_or_usage_counts() {
    let scan = Scan::new(&[
        ("Params.java", "class Params { void f() { addKeyFactoryAlgorithm(provider, \"ML-KEM-1024\", PREFIX + \"MLKEMKeyFactorySpi\"); addKeyFactoryAlgorithm(provider, \"FALCON-1024\", PREFIX + \"FalconKeyFactorySpi\"); assertEquals(1024, privKey.getUsagesRemaining()); read(\"keyGen_ML-KEM-1024.txt\"); }}"),
        ("rsa.rb", "OpenSSL::PKey::RSA.new(1024)\nOpenSSL::PKey::RSA.new(2048)\nOpenSSL::PKey::RSA.new(3072)\n"),
    ], "en");
    assert!(scan
        .hits("Params.java", "TS_RSA_KEY_SIZE_LITERAL")
        .is_empty());
    assert_eq!(scan.hits("rsa.rb", "TS_RSA_KEY_SIZE_LITERAL").len(), 3);
}
#[test]
fn tls_references_do_not_claim_effective_configuration_or_static_rsa_exchange() {
    let scan = Scan::new(&[
        ("table.rs", "const A: &str = \"TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256\";\nconst B: &str = \"TLS_DHE_RSA_WITH_AES_128_GCM_SHA256\";\nconst C: &str = \"TLS_RSA_WITH_AES_128_GCM_SHA256\";\n"),
        ("table.c", "const char *x = \"Stopped a TLSv1 heartbeat attack\";\n/* TLSv1 minimum */\n"),
        ("active.conf", "ssl_protocols TLSv1;\n"),
        ("comment.conf", "# ssl_protocols TLSv1;\n# ssl_ciphers RSA;\n"),
    ], "en");
    assert!(scan
        .findings()
        .iter()
        .all(|f| f["location"]["file"] != "comment.conf"));
    assert_eq!(
        scan.hits("active.conf", "NGINX_SSL_PROTOCOLS_LEGACY").len(),
        1
    );
    for f in scan.hits("table.c", "TLS_VERSION_1_0") {
        assert!(!f["description"]
            .as_str()
            .unwrap()
            .contains("configuration detected"));
        assert!(!f["recommended_actions"]
            .to_string()
            .contains("config was detected"));
    }
    let tls = scan.hits("table.rs", "TS_TLS_RSA_CIPHER_LITERAL");
    assert_eq!(tls.len(), 3);
    for f in tls.iter().filter(|f| f["location"]["line"] != 3) {
        assert!(!f["migration_hint"]
            .as_str()
            .unwrap()
            .contains("containing RSA key exchange"));
        assert!(f["recommended_actions"]
            .to_string()
            .contains("authentication"));
    }
    assert_eq!(scan.hits("table.rs", "TLS_RSA_KEY_EXCHANGE").len(), 1);
}
#[test]
fn reference_guidance_does_not_assert_exposure_jwt_issuance_or_missing_hybrid() {
    let scan = Scan::new(&[
        ("Marker.c", "/* -----BEGIN RSA PRIVATE KEY-----\n * [...] */\n"),
        ("Composite.java", "class Composite { void f() { checkLength(\"EdDSA public key\", 32); Cipher.getInstance(\"RSA/NONE/OAEPPadding\"); }}"),
        ("rsa.js", "const crypto = require('crypto'); crypto.generateKeyPairSync('rsa', {modulusLength: 2048});"),
    ], "en");
    let marker = scan.hits("Marker.c", "PRIVATE_KEY_RSA_HEADER");
    assert_eq!(marker.len(), 1);
    assert!(marker[0]["migration_hint"].as_str().unwrap().contains("If"));
    for f in scan.findings() {
        let guidance = format!("{} {}", f["migration_hint"], f["recommended_actions"]);
        assert!(!guidance.contains("Stop issuing new JWTs"));
        assert!(!guidance.contains("Migrate Node.js RSA signing/verification logic"));
        assert!(!guidance.contains("Migrate to hybrid KEM+AEAD cryptographic design"));
        assert!(!guidance.contains("Replace Java RSA API usage"));
    }
    let ed = scan.hits("Composite.java", "TS_JWT_EDDSA_ALG");
    assert_eq!(ed.len(), 1);
    assert!(!ed[0]["recommended_actions"]
        .to_string()
        .contains("jwt.remove-rsa-algorithms"));
}
#[test]
fn standard_pqc_names_and_registrations_are_inventoried_without_execution_claims() {
    let scan = Scan::new(&[
        ("schemes.go", "package schemes\nfunc Name() string { return \"ML-KEM-768\" }\nvar schemes = []interface{}{mlkem512.Scheme(), hybrid.X25519MLKEM768()}\nvar params = \"SLH-DSA-SHA2-128s\"\n"),
        ("algs.ts", "type Alg = 'RS256' | 'ML-DSA-44' | 'ML-DSA-65' | 'ML-DSA-87';\n"),
        ("groups.rs", "enum Group { MLKEM512, MLKEM768, MLKEM1024, X25519MLKEM768 }\n"),
        ("negative.go", "package schemes\nvar x = \"Kyber512\"\nvar y = \"NOTMLKEM768\"\n"),
    ], "en");
    for (file, count) in [("schemes.go", 4), ("algs.ts", 3), ("groups.rs", 4)] {
        let hits = scan.hits(file, "PQC_STANDARD_REFERENCE");
        assert_eq!(hits.len(), count, "{file}");
        for f in hits {
            assert_eq!(f["severity"], "info");
            assert_eq!(f["risk"], "quantum-safe");
            assert!(f["evidence"]["metadata"]["reference_kind"].is_string());
        }
    }
    for f in scan.hits("schemes.go", "PQC_STANDARD_REFERENCE") {
        let expected = match f["location"]["line"].as_u64().unwrap() {
            2 | 4 => "name_literal",
            _ => "call_reference",
        };
        assert_eq!(f["evidence"]["metadata"]["reference_kind"], expected);
    }
    for f in scan.hits("algs.ts", "PQC_STANDARD_REFERENCE") {
        assert_eq!(f["evidence"]["metadata"]["reference_kind"], "name_literal");
    }
    for f in scan.hits("groups.rs", "PQC_STANDARD_REFERENCE") {
        assert_eq!(
            f["evidence"]["metadata"]["reference_kind"],
            "symbol_reference"
        );
    }
    assert!(scan
        .hits("negative.go", "PQC_STANDARD_REFERENCE")
        .is_empty());
    assert_eq!(scan.hits("negative.go", "PQC_DRAFT_ALGORITHM").len(), 1);
}
#[test]
fn gradle_map_declarations_preserve_constraints_without_resolving_variables() {
    let scan = Scan::new(&[("build.gradle", "dependencies {\n implementation group: 'javax.mail', name: 'mail', version: '1.4'\n testImplementation group: 'junit', name: 'junit', version: '4.13.2'\n implementation group: 'io.grpc', name: 'grpc-stub', version: grpcVersion\n // implementation group: 'bad', name: 'comment', version: '1.0'\n implementation \"io.grpc:grpc-api:${grpcVersion}\"\n compileOnly group: 'org.test', name: 'no-version'\n}\n")], "en");
    let sbom: Value = serde_json::from_str(&scan.text("dependency-sbom.json")).unwrap();
    let rows = sbom.as_array().unwrap();
    assert_eq!(rows.len(), 5);
    for (name, version) in [
        ("javax.mail:mail", "1.4"),
        ("junit:junit", "4.13.2"),
        ("io.grpc:grpc-stub", "unknown"),
        ("io.grpc:grpc-api", "unknown"),
        ("org.test:no-version", "unknown"),
    ] {
        let row = rows.iter().find(|r| r["name"] == name).unwrap();
        assert_eq!(row["version"], version);
        assert!(!row["purl"].as_str().unwrap().contains('@'));
    }
    let native: Value = serde_json::from_str(&scan.text("scan.inventory.v1.json")).unwrap();
    let mail = native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["attributes"]["name"] == "javax.mail:mail")
        .unwrap();
    assert_eq!(mail["attributes"]["declared_constraint"], "1.4");
    assert!(mail["attributes"]["resolved_version"].is_null());
}
#[test]
fn human_headers_omit_internal_explanations_but_keep_findings_and_inspection_status() {
    for lang in ["en", "ja"] {
        let scan = Scan::new(
            &[("algorithm.ts", "const alg = 'RS256';\nfunction broken( {\n")],
            lang,
        );
        let html = scan.text("report.html");
        let md = scan.text("report.md");
        for unwanted in [
            "Unique candidate locations",
            "Candidate findings with unknown positions",
            "Uninspected or truncated results are not counted",
            "Inspection is incomplete or unknown",
            "Expand a rule, then a location",
            "not a runtime or migration-completion guarantee",
            "Static evidence does not prove runtime use or completed PQC migration",
            "対応候補の重複を除いた位置数",
            "位置不明の対応候補所見数",
            "検査は未完了または不明です",
            "未検査・打ち切り分は件数に含みません",
            "PQCへの対応完了を保証するものではありません",
            "ルール、検出箇所の順に開くと",
            "実際の利用やPQC対応完了を保証するものではありません",
        ] {
            assert!(!html.contains(unwanted), "HTML: {lang}: {unwanted}");
            assert!(!md.contains(unwanted), "Markdown: {lang}: {unwanted}");
        }
        assert_eq!(
            html.matches("data-finding-id=").count(),
            scan.findings().len()
        );
        assert!(html.contains("data-inspection-status=\"partial\""));
        assert!(md.contains("partial"));
        assert!(html.contains("JWT_RS256"));
        assert!(md.contains("JWT_RS256"));
        let heading = if lang == "en" {
            "Inspection scope and completion"
        } else {
            "検査範囲と完了状況"
        };
        assert!(html.contains(heading));
    }
}

#[test]
fn gradle_never_promotes_dynamic_fragments_to_literal_facts() {
    let scan = Scan::new(&[("build.gradle", "dependencies {\n implementation group: 'org.example' + suffix, name: 'bad', version: '1.' + minor\n implementation group: 'org.example', name: 'lib', version: '1.4' + patch\n implementation group: 'org.example', name: 'other' + suffix, version: '2.0'\n}\n")], "en");
    let rows: Value = serde_json::from_str(&scan.text("dependency-sbom.json")).unwrap();
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["name"], "org.example:lib");
    assert_eq!(rows[0]["version"], "unknown");
    assert!(!rows[0]["purl"].as_str().unwrap().contains('@'));
    let native: Value = serde_json::from_str(&scan.text("scan.inventory.v1.json")).unwrap();
    let lib = native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["attributes"]["name"] == "org.example:lib")
        .unwrap();
    assert!(lib["attributes"]["declared_constraint"].is_null());
    assert!(lib["attributes"]["resolved_version"].is_null());
}

#[test]
fn jwt_algorithm_names_do_not_claim_active_signing() {
    let scan = Scan::new(&[("types.ts", "type Algorithms = 'RS256' | 'RS384' | 'RS512' | 'PS256' | 'PS384' | 'PS512' | 'ES256';\n")], "en");
    for f in scan.findings().iter().filter(|f| f["category"] == "JWT") {
        assert!(!f["description"].as_str().unwrap().contains("is in use"));
        assert!(f["migration_hint"].as_str().unwrap().contains("Confirm"));
        assert!(!f["recommended_actions"]
            .to_string()
            .contains("Stop issuing new JWTs"));
    }
}

#[test]
fn gradle_quoted_commas_do_not_drop_dependency_identity() {
    let scan = Scan::new(&[("build.gradle", "dependencies {\n implementation group: 'org.example', name: 'map-lib', version: '[1.0,2.0)'\n implementation 'org.example:coordinate-lib:[1.0,2.0)'\n}\n")], "en");
    let rows: Value = serde_json::from_str(&scan.text("dependency-sbom.json")).unwrap();
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 2);
    for name in ["org.example:map-lib", "org.example:coordinate-lib"] {
        let row = rows.iter().find(|r| r["name"] == name).unwrap();
        assert_eq!(row["version"], "unknown");
        assert!(!row["purl"].as_str().unwrap().contains('@'));
    }
}

#[test]
fn gradle_dynamic_versions_keep_literal_identity_without_resolved_facts() {
    for name in ["build.gradle", "build.gradle.kts"] {
        let scan = Scan::new(
            &[(
                name,
                r#"dependencies {
 implementation("org.bouncycastle:bcprov-jdk18on:$version")
 implementation("${group}:not-a-literal:1.0")
}
"#,
            )],
            "en",
        );
        let rows: Value = serde_json::from_str(&scan.text("dependency-sbom.json")).unwrap();
        assert_eq!(rows.as_array().unwrap().len(), 1);
        assert_eq!(rows[0]["name"], "org.bouncycastle:bcprov-jdk18on");
        assert_eq!(rows[0]["version"], "unknown");
        assert_eq!(rows[0]["purl"], "");
        let native: Value = serde_json::from_str(&scan.text("scan.inventory.v1.json")).unwrap();
        let dependency = native["assets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["attributes"]["name"] == "org.bouncycastle:bcprov-jdk18on")
            .unwrap();
        assert!(dependency["attributes"]["declared_constraint"].is_null());
        assert!(dependency["attributes"]["resolved_version"].is_null());
    }
}

#[test]
fn gradle_inline_maps_stop_at_statement_boundaries_outside_quotes() {
    let scan = Scan::new(
        &[(
            "build.gradle",
            r#"dependencies { implementation group: 'org.example', name: 'known', version: '1.80'; compileOnly group: 'org.example', name: 'no-version' }
dependencies { implementation version: '1;}2', group: 'org.example', name: 'quoted-delimiters' }
"#,
        )],
        "en",
    );
    let rows: Value = serde_json::from_str(&scan.text("dependency-sbom.json")).unwrap();
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 3);
    for (name, version) in [
        ("org.example:known", "1.80"),
        ("org.example:no-version", "unknown"),
        ("org.example:quoted-delimiters", "unknown"),
    ] {
        let row = rows.iter().find(|r| r["name"] == name).unwrap();
        assert_eq!(row["version"], version);
    }
}
