// QA evidence helper, not product code or an independent study observation.
#[allow(dead_code)]
#[path = "/Users/waka/.codex/worktrees/v04-roadmap-20261118/pqc-scan/crates/pqc_scan_core/tests/support/migration/mod.rs"]
mod fixture;
#[path = "/Users/waka/.codex/worktrees/v04-roadmap-20261118/pqc-scan/crates/pqc_scan_core/tests/support/migration/context.rs"]
mod ctx;
use pqc_scan_core::migration::{SavedEvidence,InputFormat};
use serde_json::{json,Value};
use std::{path::PathBuf,process::Command};
fn main() {
    let root=PathBuf::from(std::env::args().nth(1).unwrap());
    let binary=PathBuf::from(std::env::args().nth(2).unwrap());
    let mut ready=ctx::critical_signing();ready["support"]=ctx::support();
    let mut blocked=ready.clone();blocked["support"]["peer"]=ctx::claim(json!(false));
    let mut ordinary=ctx::ordinary_signing();ordinary["support"]=ctx::support();
    let mut stale=ctx::ordinary_signing();stale["deadline"]=ctx::claim(json!({"kind":"at","at":"2026-11-16T00:00:00Z","reference_id":"org-deadline","scope_reason":"organization_policy"}));stale["deadline"]["provenance"]["source_kind"]="external_review_claim".into();stale["deadline"]["provenance"]["asserted_at"]="2026-11-16T00:00:00Z".into();stale["deadline"]["provenance"]["valid_until"]="2026-11-17T00:00:00Z".into();
    let mut unsafe_path=fixture::inventory("RSA");unsafe_path["observations"][0]["target"]["path"]="<script>alert(1)</script>![x](evil.png)`[click](evil.html)".into();unsafe_path["arbitrary_metadata"]=json!({"secret":"SECRET_CANARY_070"});
    let mut partial=fixture::tls();partial["outcome"]="partial".into();partial["diagnostics"]=json!(["certificate_parse_incomplete"]);partial["details"]["certificate_collection"]["status"]="partial".into();partial["details"]["peer_certificates"][0]["attribute_status"]="partial".into();
    let mut failed=fixture::failed_tls();failed["details"]["peer_certificates"][0]["signature_oid"]="1.2.840.113549.1.1.4".into();
    let mut two=fixture::inventory("RSA");
    let mut second=two["observations"][0].clone();
    second["id"]=pqc_scan_core::identity::native_id("observation", &["system-b"]).into();
    second["occurrence_key"]=pqc_scan_core::identity::native_id("occurrence", &["system-b"]).into();
    second["target"]["path"]="system-b/auth.rs".into();
    two["observations"][0]["target"]["path"]="system-a/auth.rs".into();
    two["observations"].as_array_mut().unwrap().push(second);
    let kx_claims=Some(json!({"applicability":ctx::claim(json!("applicable")),"use_case":ctx::claim(json!("key_establishment"))}));
    for (name,mut input,format,claims) in [
        ("unknown",fixture::inventory("RSA"),InputFormat::InventoryV1,None),
        ("urgent",fixture::inventory("RSA"),InputFormat::InventoryV1,Some(ready)),
        ("blocked",fixture::inventory("RSA"),InputFormat::InventoryV1,Some(blocked)),
        ("plan",fixture::inventory("RSA"),InputFormat::InventoryV1,Some(ordinary)),
        ("conflict",fixture::inventory("ML-KEM-768"),InputFormat::InventoryV1,Some(ctx::critical_signing())),
        ("stale",fixture::inventory("RSA"),InputFormat::InventoryV1,Some(stale)),
        ("excluded",fixture::inventory("RSA"),InputFormat::InventoryV1,Some(json!({"applicability":ctx::claim(json!("not_applicable"))}))),
        ("tls",fixture::tls(),InputFormat::TlsV1,kx_claims.clone()),
        ("partial",partial,InputFormat::TlsV1,kx_claims),
        ("failed",failed,InputFormat::TlsV1,None),
        ("escaping",unsafe_path,InputFormat::InventoryV1,None),
        ("two-systems",two,InputFormat::InventoryV1,Some(ctx::critical_signing())),
    ] {
        if format==InputFormat::InventoryV1 && input["assets"][0]["attributes"]["identifier"]=="RSA" {input["assets"][0]["attributes"]["parameters"]=json!({"key_bits":"2048"});}
        let dir=root.join(name);std::fs::create_dir_all(&dir).unwrap();let file=dir.join("input.json");std::fs::write(&file,serde_json::to_vec(&input).unwrap()).unwrap();
        let evidence=SavedEvidence::read(&file,format).unwrap();
        let context=claims.map(|claims| {let path=dir.join("context.json");std::fs::write(&path,serde_json::to_vec(&{
            let mut c=ctx::context(&evidence,claims);
            if name=="two-systems" {
                let a=evidence.targets().iter().find(|t| t.source_ref.path.as_deref()==Some("system-a/auth.rs")).unwrap();
                let b=evidence.targets().iter().find(|t| t.source_ref.path.as_deref()==Some("system-b/auth.rs")).unwrap();
                c["bindings"][0]["target_id"]=a.id.clone().into();
                c["systems"].as_array_mut().unwrap().push(json!({"id":"system-b"}));
                c["bindings"].as_array_mut().unwrap().push(json!({"target_id":b.id,"system_id":"system-b","claims":ctx::ordinary_signing()}));
            }
            c
        }).unwrap()).unwrap();path});
        for lang in ["en","ja"] {
            let mut command=Command::new(&binary);command.arg("assess").arg(&file).args(["--input-format",format.as_str(),"--as-of",ctx::AS_OF,"--out-dir"]).arg(dir.join(lang)).args(["--lang",lang]);if let Some(path)=&context {command.arg("--context").arg(path);}
            let output=command.output().unwrap();assert!(output.status.success(),"{name}: {}",String::from_utf8_lossy(&output.stderr));println!("{name}/{lang}: exit0");
        }
    }
    let _=std::any::type_name::<Value>();
}
