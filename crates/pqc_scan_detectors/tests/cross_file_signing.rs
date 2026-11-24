//! Public pipeline controls: source selection and immutable import provenance.
use pqc_scan_core::pipeline::scan_repository_with_scope;
use pqc_scan_core::{NativeOptions, ScanConfig, ScanDocument, ScanSelection};
use pqc_scan_detectors::default_detectors;
use pqc_scan_rules::RuleSet;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
struct Repo(PathBuf);
impl Repo {
    fn new(files: &[(&str, &str)]) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-relative-signing-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        let repo = Self(root);
        for (path, text) in files {
            repo.write(path, text);
        }
        repo
    }
    fn write(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    fn scan(&self, exclude_tests: bool) -> ScanDocument {
        let rules = RuleSet::load_from_dir(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
        )
        .unwrap();
        scan_repository_with_scope(
            &ScanConfig {
                root: self.0.clone(),
                threads: 1,
                ..Default::default()
            },
            &rules,
            &default_detectors(),
            ScanSelection {
                exclude_tests,
                ..Default::default()
            },
            Some(&NativeOptions::new(Some("cross-file")).unwrap()),
            None,
        )
        .unwrap()
    }
}
impl Drop for Repo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
const CONFIG: &str = "export const JWT_ALGORITHM = 'RS256';";
const WRAPPER: &str = "import jwt from 'jsonwebtoken'; import { JWT_ALGORITHM } from './config'; export function issueToken(payload: object, privateKey: string) { return jwt.sign(payload, privateKey, { algorithm: JWT_ALGORITHM }); }";
const DIRECT: &str = "import jwt from 'jsonwebtoken'; import {JWT_ALGORITHM as ALG} from './config'; jwt.sign(payload, privateKey, {algorithm: ALG});";
fn resolved(document: &ScanDocument, path: &str) -> bool {
    document.legacy().findings.iter().any(|f| {
        f.location.file == path
            && f.rule_id == "TS_NODE_JOSE_USAGE"
            && f.evidence
                .metadata
                .get("source_resolution")
                .is_some_and(|s| s == "bounded_relative_esm_signing_reference")
    })
}
#[test]
fn imported_constant_and_pure_forwarding_wrapper_resolve_at_the_call_site() {
    let repo = Repo::new(&[
        ("config.ts", CONFIG),
        ("token.ts", WRAPPER),
        ("direct.ts", DIRECT),
        (
            "caller.ts",
            "import {issueToken as issue} from './token'; issue(payload, key);",
        ),
    ]);
    let scan = repo.scan(false);
    for path in ["token.ts", "direct.ts", "caller.ts"] {
        assert!(resolved(&scan, path), "{path}");
    }
    let native = serde_json::to_value(scan.native().unwrap()).unwrap();
    assert!(native["assets"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["attributes"]["identifier"] == "RS256"));
    for observation in native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["rule_id"] == "TS_NODE_JOSE_USAGE")
    {
        assert_eq!(observation["usage"], "referenced");
    }
}
#[test]
fn unknown_mutating_dynamic_and_impure_modules_remain_unresolved() {
    for config in [
        "export let JWT_ALGORITHM = 'RS256';",
        "export const JWT_ALGORITHM = process.env.ALG;",
        "export const JWT_ALGORITHM = 'RS256'; mutate();",
        "export {JWT_ALGORITHM} from './other';",
        "export const JWT_ALGORITHM = 'RS256'; class C { static { mutate(); } }",
    ] {
        let repo = Repo::new(&[("config.ts", config), ("direct.ts", DIRECT)]);
        assert!(!resolved(&repo.scan(false), "direct.ts"), "{config}");
    }
    for wrapper in [
        WRAPPER.replace("return jwt.sign", "mutate(); return jwt.sign"),
        WRAPPER.replace("jwt.sign(payload, privateKey", "jwt.sign(other, privateKey"),
        WRAPPER.replace(
            "algorithm: JWT_ALGORITHM",
            "algorithm: JWT_ALGORITHM, header: {alg:'ES256'}",
        ),
        WRAPPER.replace(
            "algorithm: JWT_ALGORITHM",
            "...options, algorithm: JWT_ALGORITHM",
        ),
        WRAPPER.replace(
            "export function issueToken",
            "jwt.sign = other; export function issueToken",
        ),
    ] {
        let repo = Repo::new(&[
            ("config.ts", CONFIG),
            ("token.ts", &wrapper),
            (
                "caller.ts",
                "import {issueToken} from './token'; issueToken(p,k);",
            ),
        ]);
        assert!(!resolved(&repo.scan(false), "caller.ts"), "{wrapper}");
    }
}
#[test]
fn excluded_ambiguous_cyclic_and_shadowed_imports_do_not_resolve() {
    for extra in [
        vec![(".pqcignore", "config.ts\n")],
        vec![("config.js", CONFIG)],
        vec![
            (
                "config.ts",
                "import {ALG} from './other'; export const JWT_ALGORITHM = ALG;",
            ),
            (
                "other.ts",
                "import {JWT_ALGORITHM} from './config'; export const ALG = JWT_ALGORITHM;",
            ),
        ],
    ] {
        let repo = Repo::new(&[("config.ts", CONFIG), ("direct.ts", DIRECT)]);
        for (path, text) in extra {
            repo.write(path, text);
        }
        assert!(!resolved(&repo.scan(false), "direct.ts"));
    }
    for source in [
        DIRECT.replace("jwt.sign(payload", "function f(ALG) { jwt.sign(payload") + "}",
        DIRECT.replace("jwt.sign(payload", "ALG = 'ES256'; jwt.sign(payload"),
    ] {
        let repo = Repo::new(&[("config.ts", CONFIG), ("direct.ts", &source)]);
        assert!(!resolved(&repo.scan(false), "direct.ts"), "{source}");
    }
    let repo = Repo::new(&[
        ("tests/config.ts", CONFIG),
        (
            "direct.ts",
            &DIRECT.replace("'./config'", "'./tests/config'"),
        ),
    ]);
    assert!(resolved(&repo.scan(false), "direct.ts"));
    assert!(!resolved(&repo.scan(true), "direct.ts"));
}
#[cfg(unix)]
#[test]
fn symbolic_link_import_is_not_followed() {
    let repo = Repo::new(&[("real.ts", CONFIG), ("direct.ts", DIRECT)]);
    std::os::unix::fs::symlink(repo.0.join("real.ts"), repo.0.join("config.ts")).unwrap();
    let scan = repo.scan(false);
    assert!(!resolved(&scan, "direct.ts"));
    assert!(scan
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("config.ts")
            && d.message().starts_with("Symbolic link not followed")));
}
#[test]
fn depth_limit_retains_known_findings_and_explains_unfinished_work() {
    let repo = Repo::new(&[("direct.ts", DIRECT)]);
    for i in 0..6 {
        repo.write(
            &if i == 0 {
                "config.ts".into()
            } else {
                format!("c{i}.ts")
            },
            &format!(
                "import {{JWT_ALGORITHM as A}} from './c{}'; export const JWT_ALGORITHM = A;",
                i + 1
            ),
        );
    }
    repo.write("c6.ts", CONFIG);
    let scan = repo.scan(false);
    assert!(!resolved(&scan, "direct.ts"));
    assert!(!scan.legacy().findings.is_empty());
    assert!(scan
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("direct.ts")
            && d.code() == pqc_scan_core::DiagnosticCode::DetectorTruncated));
}

#[test]
fn changed_export_and_unused_initializing_import_prevent_wrapper_resolution() {
    for (module,helper) in [
        ("import jwt from 'jsonwebtoken'; export function issue(p,k){return jwt.sign(p,k,{algorithm:'RS256'});} export function reset(){issue=replacement;}", "export const unused='RS256';"),
        ("import jwt from 'jsonwebtoken'; import {unused} from './helper'; export function issue(p,k){return jwt.sign(p,k,{algorithm:'RS256'});}", "import jwt from 'jsonwebtoken'; jwt.sign=replacement; export const unused='RS256';"),
        ("import jwt from 'jsonwebtoken'; import {unused} from './helper'; export function issue(p,k){return jwt.sign(p,k,{algorithm:'RS256'});}", "import {x} from './missing'; export const unused='RS256';"),
    ] {
        let repo=Repo::new(&[("token.ts",module),("helper.ts",helper),("caller.ts","import {issue,reset} from './token'; reset(); issue(p,k);")]);
        assert!(!resolved(&repo.scan(false),"caller.ts"),"{module}");
    }
}
#[test]
fn imports_cannot_widen_the_selected_root() {
    let outside = Repo::new(&[("config.ts", CONFIG)]);
    let relative = format!(
        "../{}/config.ts",
        outside.0.file_name().unwrap().to_str().unwrap()
    );
    let repo = Repo::new(&[("direct.ts", &DIRECT.replace("./config", &relative))]);
    assert!(!resolved(&repo.scan(false), "direct.ts"));
}

#[test]
fn repeated_wrapper_calls_reuse_facts_and_distinct_exports_have_a_parse_budget() {
    let module = "import jwt from 'jsonwebtoken'; ".to_owned()
        + &(0..40)
            .map(|i| {
                format!(
                    "export function issue{i}(p,k){{return jwt.sign(p,k,{{algorithm:'RS256'}});}} "
                )
            })
            .collect::<String>();
    let repo = Repo::new(&[
        ("token.ts", &module),
        (
            "caller.ts",
            &("import {issue0} from './token'; ".to_owned() + &"issue0(p,k);".repeat(100)),
        ),
    ]);
    let repeated = repo.scan(false);
    assert_eq!(
        repeated
            .legacy()
            .findings
            .iter()
            .filter(|f| f.location.file == "caller.ts" && f.rule_id == "TS_NODE_JOSE_USAGE")
            .count(),
        100
    );
    assert!(!repeated
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("caller.ts")
            && d.code() == pqc_scan_core::DiagnosticCode::DetectorTruncated));
    repo.write(
        "caller.ts",
        &format!(
            "import {{{}}} from './token'; {}",
            (0..40)
                .map(|i| format!("issue{i}"))
                .collect::<Vec<_>>()
                .join(","),
            (0..40)
                .map(|i| format!("issue{i}(p,k);"))
                .collect::<String>()
        ),
    );
    let distinct = repo.scan(false);
    assert!(distinct
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("caller.ts")
            && d.message()
                .contains("relative_module_parses; limit=32; processed=32")));
    assert!(resolved(&distinct, "caller.ts"));
}

#[test]
fn warmed_export_cache_does_not_bypass_the_active_import_chain_limit() {
    let repo=Repo::new(&[
        ("config.ts","export const A='RS256';"),
        ("token.ts","import jwt from 'jsonwebtoken'; import {A} from './config.ts'; export const SHARED=A; export function issue(p,k){return jwt.sign(p,k,{algorithm:A});}"),
        ("d2.ts","import {SHARED} from './token.ts'; export const MID=SHARED;"),
        ("d1.ts","import {MID} from './d2.ts'; export const DEEP=MID;"),
        ("caller.ts","import jwt from 'jsonwebtoken'; import {issue} from './token.ts'; import {DEEP} from './d1.ts'; issue(p,k); jwt.sign(p,k,{algorithm:DEEP});"),
    ]);
    let scan = repo.scan(false);
    let calls: Vec<_> = scan
        .legacy()
        .findings
        .iter()
        .filter(|f| {
            f.location.file == "caller.ts"
                && f.evidence
                    .metadata
                    .get("source_resolution")
                    .is_some_and(|v| v == "bounded_relative_esm_signing_reference")
        })
        .collect();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].evidence.r#match, "issue");
    assert!(scan
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("caller.ts")
            && d.message().contains("relative_module_depth; limit=4")));
}

#[test]
fn unrelated_import_discovery_does_not_spend_crypto_budgets_or_hide_a_later_signer() {
    let repo = Repo::new(&[("token.ts", "import jwt from 'jsonwebtoken'; export function issue(p,k){return jwt.sign(p,k,{algorithm:'RS256'});}")]);
    let mut caller = String::new();
    for i in 0..24 {
        let padding = if i == 0 {
            "x".repeat(130_000)
        } else {
            String::new()
        };
        repo.write(
            &format!("helper{i}.ts"),
            &format!("/*{padding}*/ export function helper{i}(a,b){{return combine(a,b);}}"),
        );
        caller += &format!("import {{helper{i}}} from './helper{i}'; helper{i}(a,b);\n");
    }
    caller += "import {issue} from './token'; issue(payload,key);";
    repo.write("caller.ts", &caller);
    let scan = repo.scan(false);
    assert!(resolved(&scan, "caller.ts"));
    assert!(!scan
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("caller.ts")
            && d.code() == pqc_scan_core::DiagnosticCode::DetectorTruncated));
}

#[test]
fn actual_crypto_modules_and_imported_constants_keep_the_existing_size_limit() {
    for module in [
        format!("/*{}*/ import jwt from 'jsonwebtoken'; export function issue(p,k){{return jwt.sign(p,k,{{algorithm:'RS256'}});}}", "x".repeat(66_000)),
        format!("/*{}*/ export const JWT_ALGORITHM='RS256';", "x".repeat(66_000)),
    ] {
        let direct = if module.contains("jsonwebtoken") { "import {issue} from './config'; issue(p,k);" } else { DIRECT };
        let repo=Repo::new(&[("config.ts",&module),("caller.ts",direct)]);
        let scan=repo.scan(false);
        assert!(!resolved(&scan,"caller.ts"));
        assert!(scan.diagnostics().iter().any(|d| d.file()==Some("caller.ts") && d.message().contains("relative_module_bytes; limit=65536")));
        assert!(!scan.legacy().findings.is_empty());
    }
}

#[test]
fn discovery_exhaustion_is_reported_and_retains_existing_direct_evidence() {
    let repo = Repo::new(&[]);
    let mut caller =
        "import jwt from 'jsonwebtoken'; jwt.sign(p,k,{algorithm:'RS256'});\n".to_string();
    for i in 0..70 {
        caller += &format!("import {{f{i}}} from './missing{i}'; f{i}(p,k);\n");
    }
    repo.write("caller.ts", &caller);
    let scan = repo.scan(false);
    assert!(scan
        .legacy()
        .findings
        .iter()
        .any(|f| f.location.file == "caller.ts" && f.rule_id == "TS_NODE_JOSE_USAGE"));
    assert!(scan
        .diagnostics()
        .iter()
        .any(|d| d.file() == Some("caller.ts")
            && d.message()
                .contains("relative_discovery_probes; limit=256; processed=256")));
}

#[test]
fn discovery_file_and_byte_budgets_remain_finite_for_unrelated_modules() {
    for (count, padding, reason) in [
        (65, 0, "relative_discovery_files; limit=64; processed=64"),
        (3, 1_500_000, "relative_discovery_bytes; limit=4194304"),
    ] {
        let repo = Repo::new(&[]);
        let mut caller = String::new();
        for i in 0..count {
            repo.write(
                &format!("m{i}.ts"),
                &format!(
                    "/*{}*/ export function f{i}(a,b){{return a;}}",
                    "x".repeat(padding)
                ),
            );
            caller += &format!("import {{f{i}}} from './m{i}.ts'; f{i}(p,k);\n");
        }
        repo.write("caller.ts", &caller);
        let scan = repo.scan(false);
        assert!(
            scan.diagnostics()
                .iter()
                .any(|d| d.file() == Some("caller.ts") && d.message().contains(reason)),
            "{reason}"
        );
    }
}
