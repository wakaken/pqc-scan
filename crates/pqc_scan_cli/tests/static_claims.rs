use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

struct Fixture(PathBuf);

impl Fixture {
    fn new(tag: &str, files: &[(&str, &str)]) -> Self {
        let root =
            std::env::temp_dir().join(format!("pqc-static-claims-{}-{tag}", std::process::id()));
        std::fs::create_dir_all(root.join("input")).unwrap();
        for (name, source) in files {
            std::fs::write(root.join("input").join(name), source).unwrap();
        }
        Self(root)
    }

    fn scan(&self, language: &str) -> Value {
        let out = self.0.join(language);
        let run = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .arg("scan")
            .arg(self.0.join("input"))
            .args([
                "--format",
                "all",
                "--threads",
                "1",
                "--diagnostics",
                "--inventory-json",
                "--lang",
                language,
                "--rules-dir",
            ])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"))
            .arg("--out-dir")
            .arg(&out)
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        serde_json::from_slice(&std::fs::read(out.join("report.json")).unwrap()).unwrap()
    }

    fn human_reports(&self, language: &str) -> [String; 2] {
        ["report.html", "report.md"]
            .map(|name| std::fs::read_to_string(self.0.join(language).join(name)).unwrap())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn hits<'a>(report: &'a Value, rule: &str, paths: &[&str]) -> Vec<&'a Value> {
    let found: Vec<_> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["rule_id"] == rule)
        .collect();
    assert_eq!(found.len(), paths.len(), "{rule}");
    for path in paths {
        assert_eq!(
            found
                .iter()
                .filter(|finding| finding["location"]["file"] == *path)
                .count(),
            1,
            "{rule} in {path}"
        );
    }
    found
}

#[test]
fn public_jwk_and_jose_header_keep_kid_signals_without_claiming_header_use() {
    let fixture = Fixture::new(
        "kid",
        &[
            (
                "public-jwk.json",
                r#"{"keys":[{"kty":"EC","crv":"P-256","kid":"public-id"}]}"#,
            ),
            ("jose-header.json", r#"{"alg":"ES256","kid":"header-id"}"#),
            ("unrelated.json", r#"{"name":"unrelated"}"#),
        ],
    );
    let report = fixture.scan("en");
    for hit in hits(
        &report,
        "JWT_KID_HEADER",
        &["public-jwk.json", "jose-header.json"],
    ) {
        let description = hit["description"].as_str().unwrap();
        assert!(description.contains("reference"), "{description}");
        assert!(!description.contains("header usage"), "{description}");
        assert_eq!(hit["category"], "JWT");
        assert_eq!(hit["risk"], "quantum-uncertain");
        assert_eq!(hit["location"]["line"], 1);
    }
    let japanese = fixture.scan("ja");
    assert_eq!(
        japanese["findings"], report["findings"],
        "machine findings keep their language-independent contract"
    );
    for text in fixture.human_reports("ja") {
        assert!(text.contains("メンバー名"));
        assert!(text.contains("鍵オブジェクトかJWTヘッダーか"));
    }
}

#[test]
fn macro_names_and_calls_keep_rsa_signals_without_claiming_a_call() {
    let fixture = Fixture::new(
        "rsa",
        &[
            ("names.h", "#define RSA_generate_key_flags 7\n"),
            (
                "operation.c",
                "void example(void) { RSA_generate_key(2048, 65537, 0, 0); }\n",
            ),
            ("unrelated.h", "#define EC_generate_key_flags 7\n"),
        ],
    );
    let report = fixture.scan("en");
    for hit in hits(&report, "OPENSSL_RSA_API", &["names.h", "operation.c"]) {
        let description = hit["description"].as_str().unwrap();
        assert!(description.contains("match"), "{description}");
        assert!(!description.contains("API call detected"), "{description}");
        assert!(hit["migration_hint"].as_str().unwrap().contains("Confirm"));
        assert_eq!(hit["category"], "TLS");
        assert_eq!(hit["risk"], "quantum-vulnerable");
    }
    fixture.scan("ja");
    for text in fixture.human_reports("ja") {
        assert!(text.contains("呼び出しの有無"));
    }
}

#[test]
fn manual_and_build_identifiers_do_not_claim_an_rsa_configured_key() {
    let fixture = Fixture::new(
        "evp",
        &[
            ("manual.pod", "=head1 NAME\n\nEVP_PKEY_set1_RSA - example API\n"),
            ("build.map", "manual/EVP_PKEY_RSA.pod: manual-output\n"),
            ("dispatch.c", "void example(EVP_PKEY *key) { switch (EVP_PKEY_base_id(key)) { case EVP_PKEY_RSA_PSS: break; } }\n"),
            ("configuration.c", "void example(EVP_PKEY *p, RSA *r) { EVP_PKEY_set1_RSA(p, r); }\n"),
            ("unrelated.pod", "=head1 NAME\n\nEVP_PKEY_EC - unrelated API\n"),
        ],
    );
    let report = fixture.scan("en");
    for hit in hits(
        &report,
        "API_EVP_PKEY_RSA",
        &["manual.pod", "build.map", "dispatch.c", "configuration.c"],
    ) {
        let description = hit["description"].as_str().unwrap();
        assert!(description.contains("identifier"), "{description}");
        assert!(
            !description.contains("configured for RSA detected"),
            "{description}"
        );
        assert!(hit["migration_hint"].as_str().unwrap().contains("Confirm"));
        assert_eq!(hit["category"], "CryptoAPI");
    }
    fixture.scan("ja");
    for text in fixture.human_reports("ja") {
        assert!(text.contains("識別子に一致する文字列"));
        assert!(text.contains("鍵の設定内容"));
    }
}

#[test]
fn empty_and_arbitrary_certificate_paths_keep_only_filename_claims() {
    let fixture = Fixture::new(
        "filename",
        &[
            ("empty.pem", ""),
            ("newline.pem", "\n"),
            ("arbitrary.crt", "This is ordinary text.\n"),
            ("malformed.der", "Malformed binary placeholder.\n"),
            ("arbitrary.p12", "This is ordinary text.\n"),
            ("arbitrary.pfx", "This is ordinary text.\n"),
            ("unrelated.txt", "This is ordinary text.\n"),
        ],
    );
    let report = fixture.scan("en");
    for (rule, paths) in [
        (
            "CERT_PEM_CONTAINER",
            vec!["empty.pem", "newline.pem", "arbitrary.crt", "malformed.der"],
        ),
        ("CERT_P12_CONTAINER", vec!["arbitrary.p12", "arbitrary.pfx"]),
    ] {
        for hit in hits(&report, rule, &paths) {
            let description = hit["description"].as_str().unwrap();
            assert!(description.contains("Filename"), "{description}");
            assert!(!description.contains("container detected"), "{description}");
            let preview = hit["evidence"]["snippet_preview"].as_str().unwrap();
            assert!(!preview.contains("container detected"), "{preview}");
            for key in [
                "signature_oid",
                "public_key_oid",
                "key_bits",
                "certificate_sha256",
            ] {
                assert!(hit["evidence"]["metadata"].get(key).is_none(), "{key}");
            }
            assert_eq!(hit["location"]["line"], 1);
            assert_eq!(hit["location"]["column"], 1);
        }
    }
    let diagnostics: Value =
        serde_json::from_slice(&std::fs::read(fixture.0.join("en/diagnostics.json")).unwrap())
            .unwrap();
    assert!(diagnostics["coverage"]["partial_files"].as_u64().unwrap() >= 5);
    let inventory: Value = serde_json::from_slice(
        &std::fs::read(fixture.0.join("en/scan.inventory.v1.json")).unwrap(),
    )
    .unwrap();
    assert!(inventory["assets"]
        .as_array()
        .unwrap()
        .iter()
        .all(|asset| asset["kind"] != "certificate" && asset["kind"] != "public_key"));
    fixture.scan("ja");
    for text in fixture.human_reports("ja") {
        assert!(text.contains("拡張子"));
        assert!(text.contains("格納内容"));
    }
}
