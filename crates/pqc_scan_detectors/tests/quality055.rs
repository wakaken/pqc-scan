//! Fixed public/synthetic inputs; development observations, never a product schema.
use pqc_scan_core::{Detector, DetectorStatus as S, ScannableFile};
use pqc_scan_detectors::{
    certificate_detector::CertificateDetector, dependency_detector::DependencyDetector,
    key_detector::KeyDetector, regex_detector::RegexDetector,
    tree_sitter_detector::TreeSitterDetector, DetectorBudget,
};
use pqc_scan_rules::RuleSet;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
fn save(name: &str, value: &Value) {
    if let Some(dir) = std::env::var_os("PQC_QUALITY_EVIDENCE_DIR") {
        let dir = PathBuf::from(dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), serde_json::to_vec_pretty(value).unwrap()).unwrap();
    }
}
struct Case {
    family: String,
    name: String,
    bytes: Vec<u8>,
    primary: usize,
    expected: S,
    ast: bool,
}
fn add(c: &mut Vec<Case>, family: &str, name: &str, text: impl Into<Vec<u8>>, expected: S) {
    c.push(Case {
        family: if ["manifest", "lexical", "static-gemspec"].contains(&family) {
            format!("{family}:{name}")
        } else {
            family.into()
        },
        name: name.into(),
        bytes: text.into(),
        primary: 2,
        expected,
        ast: name.ends_with(".rb"),
    });
}
fn tlv(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    if body.len() < 128 {
        out.push(body.len() as u8);
    } else {
        let n = body.len().to_be_bytes();
        let n = &n[n.iter().position(|b| *b != 0).unwrap()..];
        out.push(0x80 | n.len() as u8);
        out.extend(n);
    }
    out.extend(body);
    out
}
fn data(body: &[u8]) -> Vec<u8> {
    let mut inner = tlv(6, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 1]);
    inner.extend(tlv(0xa0, &tlv(4, body)));
    tlv(0x30, &inner)
}
fn pfx(mixed: bool) -> Vec<u8> {
    let mut bag = tlv(6, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 12, 10, 1, 3]);
    let mut cert = tlv(6, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 9, 22, 1]);
    cert.extend(tlv(
        0xa0,
        &tlv(4, include_bytes!("fixtures/certificates/rsa2048.der")),
    ));
    bag.extend(tlv(0xa0, &tlv(0x30, &cert)));
    let safe = tlv(0x30, &tlv(0x30, &bag));
    let mut contents = data(&safe);
    if mixed {
        let mut enc = tlv(6, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 6]);
        enc.extend(tlv(0xa0, &tlv(0x30, &[])));
        contents.extend(tlv(0x30, &enc));
    }
    let auth = tlv(0x30, &contents);
    let mut root = tlv(2, &[3]);
    root.extend(data(&auth));
    tlv(0x30, &root)
}

#[test]
fn quality055_formats_versions_and_incomplete_inspection_are_distinct() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rules = RuleSet::load_from_dir(&root.join("rules/default")).unwrap();
    let detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(TreeSitterDetector),
        Box::new(RegexDetector),
        Box::new(DependencyDetector),
        Box::new(CertificateDetector::at(1_780_000_000)),
        Box::new(KeyDetector::default()),
    ];
    let mut cases = vec![];
    for (name,text) in [("Cargo.toml","[dependencies]\nopenssl = \"0.10\"\n"),("package.json","{\"dependencies\":{\"openssl\":\"1.2.3\"}}"),("requirements.txt","openssl==1.2.3\n"),("go.mod","module example.org/app\nrequire example.org/openssl v1.2.3\n"),("Gemfile","gem 'openssl', '1.2.3'\n"),("gems.rb","gem 'openssl', '1.2.3'\n"),("Gemfile.lock","GEM\n  specs:\n    openssl (1.2.3)\n"),("gems.locked","GEM\n  specs:\n    openssl (1.2.3)\n"),("pom.xml","<project><dependencies><dependency><groupId>org.example</groupId><artifactId>openssl</artifactId><version>1</version></dependency></dependencies></project>")]{add(&mut cases,"manifest",name,text,S::Complete);}
    for name in [
        "build.gradle",
        "build.gradle.kts",
        "sample.gemspec",
        "gradle.lockfile",
    ] {
        let text = if name == "gradle.lockfile" {
            "org.example:openssl:1.2.3=runtimeClasspath\n"
        } else if name.ends_with("gemspec") {
            "Gem::Specification.new do |s|\n s.add_dependency 'openssl', '1.2.3'\nend\n"
        } else {
            "implementation 'org.example:openssl:1.2.3'\n"
        };
        add(
            &mut cases,
            if name.ends_with("gemspec") {
                "static-gemspec"
            } else {
                "lexical"
            },
            name,
            text,
            if name.ends_with("gemspec") {
                S::Complete
            } else {
                S::NotReported
            },
        );
    }
    for v in [1, 2, 3, 4] {
        add(
            &mut cases,
            &format!("npm-lock-{v}"),
            "package-lock.json",
            format!("{{\"lockfileVersion\":{v},\"packages\":{{}},\"dependencies\":{{}}}}"),
            if v < 4 { S::Complete } else { S::Unsupported },
        );
    }
    for v in [1, 2, 3, 4, 5] {
        add(
            &mut cases,
            &format!("cargo-lock-{v}"),
            "Cargo.lock",
            format!("version = {v}\npackage = []\n"),
            if v < 5 { S::Complete } else { S::Unsupported },
        );
    }
    for v in [5, 6, 7] {
        add(
            &mut cases,
            &format!("pipfile-{v}"),
            "Pipfile.lock",
            format!("{{\"_meta\":{{\"pipfile-spec\":{v}}},\"default\":{{}}}}"),
            if v == 6 { S::Complete } else { S::Unsupported },
        );
    }
    for v in ["1.1", "2.0", "2.1", "3.0"] {
        add(
            &mut cases,
            &format!("poetry-{v}"),
            "poetry.lock",
            format!("package=[]\n[metadata]\nlock-version=\"{v}\"\n"),
            if v == "3.0" {
                S::Unsupported
            } else {
                S::Complete
            },
        );
    }
    for v in ["5.3", "5.4", "6", "6.0", "9", "9.0", "10"] {
        add(
            &mut cases,
            &format!("pnpm-{v}"),
            "pnpm-lock.yaml",
            format!("lockfileVersion: '{v}'\npackages: {{}}\n"),
            if v == "10" {
                S::Unsupported
            } else {
                S::Complete
            },
        );
    }
    for v in ["1.3", "1.4", "1.5", "1.6", "1.7", "2.0"] {
        add(
            &mut cases,
            &format!("cdx-{v}"),
            "bom.cdx.json",
            format!("{{\"bomFormat\":\"CycloneDX\",\"specVersion\":\"{v}\",\"components\":[]}}"),
            if ["1.3", "2.0"].contains(&v) {
                S::Unsupported
            } else {
                S::Complete
            },
        );
    }
    for v in ["2.1", "2.2", "2.3", "3.0"] {
        add(
            &mut cases,
            &format!("spdx-{v}"),
            "sbom.spdx.json",
            format!("{{\"spdxVersion\":\"SPDX-{v}\",\"packages\":[]}}"),
            if ["2.1", "3.0"].contains(&v) {
                S::Unsupported
            } else {
                S::Complete
            },
        );
    }
    add(
        &mut cases,
        "malformed-json",
        "package.json",
        "{",
        S::NotReported,
    ); // Fixed error is asserted below, not guessed as success.
    for (name, text) in [
        ("app.java", "class App {}"),
        ("app.go", "package app\n"),
        ("app.js", "const x=1;"),
        ("app.ts", "const x:number=1;"),
        ("app.tsx", "const x=<div/>;"),
        ("app.py", "x=1\n"),
        ("app.rs", "fn main() {}"),
        ("app.rb", "x=1\n"),
    ] {
        cases.push(Case {
            family: format!("source-{}", name.rsplit('.').next().unwrap()),
            name: name.into(),
            bytes: text.into(),
            primary: 0,
            expected: S::Complete,
            ast: true,
        });
    }
    for name in [
        "nginx.conf",
        "apache.conf",
        "envoy.yaml",
        "ingress.yaml",
        "istio.yaml",
        "haproxy.cfg",
        "traefik.toml",
    ] {
        cases.push(Case {
            family: format!("configuration-{name}"),
            name: name.into(),
            bytes: b"# static TLS reference\n".to_vec(),
            primary: 1,
            expected: S::Complete,
            ast: false,
        });
    }
    for (name, bytes, expected) in [
        (
            "public.der",
            include_bytes!("fixtures/certificates/rsa2048.der").to_vec(),
            S::Complete,
        ),
        (
            "public.pem",
            pem::encode(&pem::Pem::new(
                "CERTIFICATE",
                include_bytes!("fixtures/certificates/rsa2048.der"),
            ))
            .into_bytes(),
            S::Complete,
        ),
        (
            "public.crt",
            include_bytes!("fixtures/certificates/rsa2048.der").to_vec(),
            S::Complete,
        ),
        (
            "public.cer",
            include_bytes!("fixtures/certificates/rsa2048.der").to_vec(),
            S::Complete,
        ),
        ("public.p12", pfx(false), S::Complete),
        ("public.pfx", pfx(false), S::Complete),
        ("broken.der", vec![0x30, 0x82, 0xff], S::Partial),
        ("mixed.p12", pfx(true), S::Partial),
    ] {
        cases.push(Case {
            family: format!("certificate-{}", name.rsplit('.').next().unwrap()),
            name: name.into(),
            bytes,
            primary: 3,
            expected,
            ast: false,
        });
    }
    let mut enc = tlv(6, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 6]);
    enc.extend(tlv(0xa0, &tlv(0x30, &[])));
    let mut opaque = tlv(2, &[3]);
    opaque.extend(data(&tlv(0x30, &tlv(0x30, &enc))));
    cases.push(Case {
        family: "encrypted-pkcs12".into(),
        name: "opaque.p12".into(),
        bytes: tlv(0x30, &opaque),
        primary: 3,
        expected: S::Unsupported,
        ast: false,
    });
    for label in ["RSA PRIVATE KEY", "PRIVATE KEY", "OPENSSH PRIVATE KEY"] {
        cases.push(Case {
            family: "private-marker".into(),
            name: "marker.pem".into(),
            bytes: format!(
                "-----BEGIN {label}-----\nSYNTHETIC_INVALID_BODY\n-----END {label}-----\n"
            )
            .into_bytes(),
            primary: 4,
            expected: S::Complete,
            ast: false,
        });
    }
    let mut output = vec![];
    for (i, c) in cases.iter().enumerate() {
        let file = ScannableFile::from_bytes(c.name.clone().into(), c.bytes.clone());
        let text = file.text.is_some();
        let mut rows = vec![];
        for (d, detector) in detectors.iter().enumerate() {
            let out = detector.detect_detailed(&file, &rules);
            let expected = if d == c.primary {
                c.expected
            } else if d == 0 {
                if c.ast {
                    S::Complete
                } else {
                    S::NotApplicable
                }
            } else if d == 1 || d == 4 {
                if text {
                    S::Complete
                } else {
                    S::NotApplicable
                }
            } else {
                S::NotApplicable
            };
            if (c.name == "mixed.p12" && d == 3) || (c.family == "private-marker" && d == 4) {
                assert!(!out.as_ref().unwrap().detections().is_empty());
            }
            let status = match &out {
                Ok(o) => format!("{:?}", o.status()),
                Err(_) => "Failed".into(),
            };
            let expected_text = if c.family == "malformed-json" && d == 2 {
                "Failed".to_string()
            } else {
                format!("{expected:?}")
            };
            assert_eq!(
                status,
                expected_text,
                "{}:{} {}",
                c.family,
                c.name,
                detector.name()
            );
            rows.push(json!({"case":i,"format":c.family,"inputName":c.name,"detector":detector.name(),"status":status,"expected":expected_text,"detections":out.as_ref().map(|o|o.detections().len()).unwrap_or(0)}));
        }
        output.push(json!({"id":i,"format":c.family,"inputName":c.name,"inputSha256":format!("{:x}",Sha256::digest(&c.bytes)),"inputHex":hex::encode(&c.bytes),"detectors":rows}));
    }
    save(
        "formats055.json",
        &json!({"cases":output,"assessmentClock":1_780_000_000,"scope":"documented bounded subsets; zero detections is not safety"}),
    );
}

#[test]
fn quality055_real_limits_retain_hits_and_do_not_complete() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rules = RuleSet::load_from_dir(&root.join("rules/default")).unwrap();
    let budget = DetectorBudget {
        max_hits_per_rule: 2,
        ..Default::default()
    };
    let mut rows = vec![];
    for n in [1, 2, 3] {
        let text = "RS256\n".repeat(n);
        let file = ScannableFile::from_bytes("settings.conf".into(), text.clone().into_bytes());
        let out = RegexDetector::with_budget(budget)
            .detect_detailed(&file, &rules)
            .unwrap();
        let hits = out
            .detections()
            .iter()
            .filter(|d| d.rule_id == "JWT_RS256")
            .count();
        assert_eq!(hits, n.min(2));
        assert_eq!(out.status(), if n > 2 { S::Truncated } else { S::Complete });
        rows.push(json!({"input":text,"n":n,"maxHits":2,"retained":hits,"status":format!("{:?}",out.status())}));
    }
    let text = "{\"dependencies\":{\"openssl\":\"1\",\"bad\":42}}";
    let out = DependencyDetector
        .detect_detailed(
            &ScannableFile::from_bytes("package.json".into(), text.into()),
            &rules,
        )
        .unwrap();
    assert_eq!(out.status(), S::Partial);
    assert!(out.detections().iter().any(|d| d
        .evidence
        .metadata
        .get("dep_name")
        .is_some_and(|name| name == "openssl")));
    save(
        "limits055.json",
        &json!({"regex":rows,"malformedSibling":{"input":text,"status":"Partial","retained":out.detections().len()}}),
    );
}

#[test]
fn quality055_explicit_work_limits_have_boundary_outcomes() {
    let root = std::env::temp_dir().join(format!("pqc055-budgets-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("rules.yml"),r#"[{"id":"AST_CONTROL","kind":"tree_sitter","category":"Control","severity":"low","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"RS256","scope":"javascript"},{"id":"REGEX_CONTROL","kind":"regex","category":"Control","severity":"low","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review","pattern":"RS256","scope":"code"}]"#).unwrap();
    let rules = RuleSet::load_from_dir(&root).unwrap();
    let mut rows = vec![];
    for (kind, values) in [
        ("ast_nodes", vec![2, 3, 4]),
        ("ast_candidates", vec![0, 1, 2]),
        ("ast_match_bytes", vec![4, 5, 6]),
    ] {
        for n in values {
            let mut budget = DetectorBudget::default();
            match kind {
                "ast_nodes" => budget.max_ast_nodes = n,
                "ast_candidates" => budget.max_ast_candidates = n,
                _ => budget.max_ast_match_bytes = n,
            }
            let out = TreeSitterDetector::with_budget(budget)
                .detect_detailed(
                    &ScannableFile::from_bytes("one.js".into(), b"RS256".to_vec()),
                    &rules,
                )
                .unwrap();
            let minimum = match kind {
                "ast_nodes" => 3,
                "ast_candidates" => 1,
                _ => 5,
            };
            assert_eq!(
                out.status(),
                if n < minimum {
                    S::Truncated
                } else {
                    S::Complete
                },
                "{kind}:{n}"
            );
            if n >= minimum {
                assert_eq!(out.detections().len(), 1);
            }
            rows.push(json!({"limit":kind,"configured":n,"input":"RS256","status":format!("{:?}",out.status()),"retained":out.detections().len()}));
        }
    }
    for kind in ["match_candidates", "accepted_hits"] {
        for n in [1, 2, 3] {
            let mut budget = DetectorBudget::default();
            if kind == "match_candidates" {
                budget.max_match_candidates = 2;
            } else {
                budget.max_hits_per_rule = 2;
            }
            let input = "RS256\n".repeat(n);
            let out = RegexDetector::with_budget(budget)
                .detect_detailed(
                    &ScannableFile::from_bytes("many.conf".into(), input.clone().into_bytes()),
                    &rules,
                )
                .unwrap();
            assert_eq!(out.status(), if n > 2 { S::Truncated } else { S::Complete });
            assert_eq!(out.detections().len(), n.min(2));
            rows.push(json!({"limit":kind,"configured":2,"inputCount":n,"input":input,"status":format!("{:?}",out.status()),"retained":out.detections().len()}));
        }
    }
    save("work-limits055.json", &json!({"cases":rows}));
    fs::remove_dir_all(root).unwrap();
}
