use serde_json::{json, Value};
#[path = "../../pqc_scan_core/tests/support/migration/context.rs"]
mod ctx;
#[path = "../../pqc_scan_core/tests/support/migration/mod.rs"]
#[allow(dead_code)]
mod fixture;
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
struct Session {
    child: Child,
    root: PathBuf,
    reports: PathBuf,
    host: String,
    prefix: String,
}
impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.root);
        let _ = fs::remove_dir_all(&self.reports);
    }
}
impl Session {
    fn start() -> Self {
        Self::start_with(false)
    }
    fn start_with(migration: bool) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "pqc-web-integration-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("project & 日")).unwrap();
        fs::write(
            root.join("project & 日/main.py"),
            "from Crypto.PublicKey import RSA\nkey = RSA.generate(2048)\n",
        )
        .unwrap();
        fs::write(root.join("private-canary.txt"), "DO_NOT_SERVE_CANARY").unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
        if migration {
            command.arg("serve").arg("--migration");
        } else {
            command.arg("serve");
        }
        let mut child = command
            .args(["--port", "0", "--no-open", "--threads", "1", "--root"])
            .arg(&root)
            .arg("--rules-dir")
            .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut out = BufReader::new(child.stdout.take().unwrap());
        let mut lines = Vec::new();
        for _ in 0..4 {
            let mut s = String::new();
            assert!(out.read_line(&mut s).unwrap() > 0);
            lines.push(s);
        }
        let url = lines[0]
            .trim()
            .strip_prefix("pqc-scan local web UI: http://")
            .unwrap();
        let (host, prefix) = url.split_once('/').unwrap();
        let reports = PathBuf::from(lines[2].trim().strip_prefix("Session reports: ").unwrap());
        Self {
            child,
            root,
            reports,
            host: host.into(),
            prefix: format!("/{prefix}"),
        }
    }
    fn request(&self, method: &str, path: &str, host: &str, origin: Option<&str>) -> (u16, String) {
        self.request_headers(method, path, host, origin, "")
    }
    fn request_headers(
        &self,
        method: &str,
        path: &str,
        host: &str,
        origin: Option<&str>,
        extra: &str,
    ) -> (u16, String) {
        let origin = origin
            .map(|o| format!("Origin: {o}\r\n"))
            .unwrap_or_default();
        self.exchange(format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\n{origin}{extra}Content-Length: 0\r\nConnection: close\r\n\r\n").as_bytes())
    }
    fn post(&self, route: &str, body: &str) -> (u16, String) {
        self.exchange(format!("POST {}{route} HTTP/1.1\r\nHost: {}\r\nOrigin: http://{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",self.prefix,self.host,self.host,body.len()).as_bytes())
    }
    fn exchange(&self, request: &[u8]) -> (u16, String) {
        let mut stream = TcpStream::connect(&self.host).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(20)))
            .unwrap();
        stream.write_all(request).unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes).unwrap();
        let split = bytes.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
        let headers = std::str::from_utf8(&bytes[..split]).unwrap();
        let mut body = bytes[split + 4..].to_vec();
        if headers
            .to_ascii_lowercase()
            .contains("transfer-encoding: chunked")
        {
            let mut decoded = Vec::new();
            let mut offset = 0;
            loop {
                let end = body[offset..]
                    .windows(2)
                    .position(|w| w == b"\r\n")
                    .unwrap()
                    + offset;
                let length =
                    usize::from_str_radix(std::str::from_utf8(&body[offset..end]).unwrap(), 16)
                        .unwrap();
                offset = end + 2;
                if length == 0 {
                    break;
                }
                decoded.extend_from_slice(&body[offset..offset + length]);
                offset += length + 2;
            }
            body = decoded;
        }
        (
            headers.split_whitespace().nth(1).unwrap().parse().unwrap(),
            String::from_utf8(body).unwrap(),
        )
    }
    fn get(&self, route: &str) -> (u16, String) {
        self.request("GET", &format!("{}{route}", self.prefix), &self.host, None)
    }
}
#[test]
fn local_ui_selects_folder_scans_and_serves_only_completed_report_artifacts() {
    let s = Session::start();
    let origin = format!("http://{}", s.host);
    assert_eq!(s.request_headers("GET",&s.prefix,&s.host,None,"Sec-Fetch-Site: cross-site\r\nSec-Fetch-Mode: navigate\r\nSec-Fetch-Dest: document\r\n").0,200);
    assert_eq!(
        s.request_headers(
            "GET",
            &format!("{}api/jobs", s.prefix),
            &s.host,
            None,
            "Sec-Fetch-Site: cross-site\r\nSec-Fetch-Mode: no-cors\r\nSec-Fetch-Dest: empty\r\n"
        )
        .0,
        403
    );
    let (code, home) = s.get("");
    assert_eq!(code, 200);
    assert!(home.contains("対象フォルダを選択"));
    let (code, body) = s.get("api/directories?path=");
    assert_eq!(code, 200);
    let listing: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(listing["directories"][0]["name"], "project & 日");
    assert!(!body.contains("DO_NOT_SERVE_CANARY"));
    for route in [
        "api/directories?path=..%2f",
        "api/directories?path=%2f",
        "api/directories?path=%ff",
    ] {
        assert_eq!(s.get(route).0, 400);
    }
    for route in [
        "private-canary.txt",
        "reports/1/../../private-canary.txt",
        "reports/1/report.html",
    ] {
        assert_eq!(s.get(route).0, 404);
    }
    assert_eq!(s.request("GET", "/", &s.host, None).0, 404);
    assert_eq!(s.request("GET", &s.prefix, "attacker.invalid", None).0, 403);
    assert_eq!(
        s.request("GET", &s.prefix, &s.host, Some("https://attacker.invalid"))
            .0,
        403
    );
    let route = format!("{}api/scan?path=project%20%26%20%E6%97%A5", s.prefix);
    assert_eq!(s.request("POST", &route, &s.host, None).0, 403);
    assert_eq!(
        s.request("POST", &route, &s.host, Some("https://attacker.invalid"))
            .0,
        403
    );
    let (code, body) = s.request("POST", &route, &s.host, Some(&origin));
    assert_eq!(code, 202);
    assert_eq!(serde_json::from_str::<Value>(&body).unwrap()["id"], 1);
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let (_, body) = s.get("api/jobs");
        let jobs: Value = serde_json::from_str(&body).unwrap();
        let job = &jobs["jobs"][0];
        if job["status"] != "running" {
            assert_eq!(job["status"], "ready", "{job}");
            assert_eq!(job["report"], "reports/1/report.html");
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    let (code, html) = s.get("reports/1/report.html");
    assert_eq!(code, 200);
    assert_eq!(
        html.as_bytes(),
        fs::read(s.reports.join("1/report.html")).unwrap()
    );
    assert!(html.contains("data-file-tree"));
    assert!(html.contains("main.py"));
    assert!(!html.contains("DO_NOT_SERVE_CANARY"));
    assert_eq!(s.get("reports/1/report.rules.html?lang=en").0, 200);
    let (code, json) = s.get("reports/1/report.json");
    assert_eq!(code, 200);
    assert!(!serde_json::from_str::<Value>(&json).unwrap()["findings"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(s.get("reports/1/main.py").0, 404);
    assert_eq!(
        s.get("api/jobs").0,
        200,
        "server remains available after scan"
    );
}
fn import(s: &Session, v: &Value) -> u64 {
    let (code, body) = s.post("api/migration/import", &serde_json::to_string(v).unwrap());
    assert_eq!(code, 200, "{body}");
    serde_json::from_str::<Value>(&body).unwrap()["id"]
        .as_u64()
        .unwrap()
}
fn targets(s: &Session, id: u64) -> Value {
    let (code, body) = s.get(&format!("api/migration/targets?job={id}&page=0"));
    assert_eq!(code, 200, "{body}");
    serde_json::from_str(&body).unwrap()
}
fn generated_request(sha: &str) -> Value {
    json!({"mode":"generate","input_sha256":sha,"as_of":ctx::AS_OF,"organization_id":"organization","system_id":"system-1","selected":[1],"answers":[{"field":"use_case","state":"known","value":"signature"},{"field":"usage_state","state":"known","value":"active"},{"field":"business_critical","state":"known","value":false},{"field":"confidential","state":"unknown"}],"actor":"owner","asserted_at":"2026-11-17T00:00:00Z"})
}
fn preview(s: &Session, id: u64, request: &Value) -> Value {
    let (code, body) = s.post(
        &format!("api/migration/preview?job={id}"),
        &serde_json::to_string(request).unwrap(),
    );
    assert_eq!(code, 200, "{body}");
    serde_json::from_str(&body).unwrap()
}
fn confirm(s: &Session, id: u64, token: &str) -> Value {
    let (code, body) = s.post(
        &format!("api/migration/assess?job={id}"),
        &json!({"token":token,"language":"ja"}).to_string(),
    );
    assert_eq!(code, 200, "{body}");
    serde_json::from_str(&body).unwrap()
}
#[test]
fn migration_is_opt_in_and_retains_host_origin_session_and_request_bounds() {
    let ordinary = Session::start();
    assert_eq!(ordinary.get("api/config").1, "{\"migration\":false}");
    assert_eq!(ordinary.get("migration.js").0, 404);
    assert_eq!(ordinary.post("api/migration/import", "{}").0, 404);
    let s = Session::start_with(true);
    assert_eq!(s.get("api/config").1, "{\"migration\":true}");
    assert_eq!(s.get("migration.js").0, 200);
    let route = format!("{}api/migration/import", s.prefix);
    for origin in [None, Some("https://attacker.invalid")] {
        assert_eq!(s.request("POST", &route, &s.host, origin).0, 403);
    }
    assert_eq!(
        s.request(
            "GET",
            &format!("{}api/migration/targets?job=1", s.prefix),
            "attacker.invalid",
            None
        )
        .0,
        403
    );
    assert_eq!(s.post("../api/migration/import", "{}").0, 405);
    assert_eq!(
        s.post("api/migration/import", "{\"SECRET_CANARY_074\":true}")
            .0,
        400
    );
    let response=s.exchange(format!("POST {route} HTTP/1.1\r\nHost: {}\r\nOrigin: http://{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",s.host,s.host,32*1024*1024+1).as_bytes());
    assert_eq!(response.0, 413);
    assert!(!response.1.contains("CANARY"));
    assert_eq!(s.get("api/migration/targets?job=999").0, 400);
    assert_eq!(s.get("api/migration/targets?job=1&job=1").0, 400);
}
#[test]
fn actual_web_generation_preview_confirm_and_cli_share_exact_output_without_rescan() {
    let s = Session::start_with(true);
    let id = import(&s, &fixture::inventory("RSA"));
    let list = targets(&s, id);
    let request = generated_request(list["input"]["sha256"].as_str().unwrap());
    let p = preview(&s, id, &request);
    assert_eq!(p["scope"][0]["number"], 1);
    assert!(p["resolved"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c["field"] == "usage_state" && c["freshness"] == "unknown"));
    let native = s.reports.join(format!("{id}/scan.inventory.v1.json"));
    let before = fs::metadata(&native).unwrap().modified().unwrap();
    let original = fs::read(&native).unwrap();
    let ctx_path = s.root.join("downloaded-context.json");
    fs::write(&ctx_path, p["context"].as_str().unwrap()).unwrap();
    let result = confirm(&s, id, p["token"].as_str().unwrap());
    let (code, actual) = s.get(result["json"].as_str().unwrap());
    assert_eq!(code, 200);
    let cli = s.root.join("cli-assess");
    let output = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
        .arg("assess")
        .arg(&native)
        .args(["--input-format", "inventory-v1", "--context"])
        .arg(&ctx_path)
        .args(["--as-of", ctx::AS_OF, "--out-dir"])
        .arg(&cli)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        serde_json::from_str::<Value>(&actual).unwrap(),
        serde_json::from_slice::<Value>(
            &fs::read(cli.join("migration.assessment.v1.json")).unwrap()
        )
        .unwrap()
    );
    assert_eq!(
        s.get(&format!("reports/{id}/migration-context.json")).1,
        p["context"].as_str().unwrap()
    );
    assert!(s
        .get(result["report"].as_str().unwrap())
        .1
        .contains("組織の暗号移行レポート"));
    assert_eq!(fs::read(native.clone()).unwrap(), original);
    assert_eq!(fs::metadata(native).unwrap().modified().unwrap(), before);
    assert_eq!(
        s.get(&format!("reports/{id}/scan.inventory.v1.json")).0,
        404
    );
    assert_eq!(s.get(&format!("reports/{id}/../private-canary.txt")).0, 404);
    let e = pqc_scan_core::migration::SavedEvidence::from_bytes(
        &original,
        pqc_scan_core::migration::InputFormat::InventoryV1,
    )
    .unwrap();
    let provenance =
        pqc_scan_core::migration::Provenance::operator("owner", "2026-11-17T00:00:00Z", None)
            .unwrap();
    let answers = vec![
        pqc_scan_core::migration::ClaimAnswer {
            field: "use_case".into(),
            value: Some(json!("signature")),
        },
        pqc_scan_core::migration::ClaimAnswer {
            field: "usage_state".into(),
            value: Some(json!("active")),
        },
        pqc_scan_core::migration::ClaimAnswer {
            field: "business_critical".into(),
            value: Some(json!(false)),
        },
        pqc_scan_core::migration::ClaimAnswer {
            field: "confidential".into(),
            value: None,
        },
    ];
    let shared = pqc_scan_core::migration::generate_context(
        &e,
        "organization",
        &[pqc_scan_core::migration::ContextSelection {
            target_index: 0,
            system_id: "system-1".into(),
            claims: pqc_scan_core::migration::ContextClaims::from_answers(&answers, provenance)
                .unwrap(),
        }],
        ctx::AS_OF,
    )
    .unwrap();
    assert_eq!(shared, p["context"].as_str().unwrap().as_bytes());
}
#[test]
fn full_import_stale_conflict_no_context_and_cancelled_tokens_preserve_semantics() {
    let s = Session::start_with(true);
    let v = fixture::inventory("ML-KEM-768");
    let id = import(&s, &v);
    let e = fixture::ingest(&v, pqc_scan_core::migration::InputFormat::InventoryV1).unwrap();
    let mut context = ctx::context(&e, ctx::critical_signing());
    context["organization_defaults"] = json!({"historical_ciphertext":ctx::unknown()});
    context["bindings"][0]["claims"]["business_critical"]["provenance"]["valid_until"] =
        "2026-11-17T12:00:00Z".into();
    let p = preview(
        &s,
        id,
        &json!({"mode":"import_context","input_sha256":e.input().sha256,"as_of":ctx::AS_OF,"context":context.to_string()}),
    );
    assert_eq!(
        serde_json::from_str::<Value>(p["context"].as_str().unwrap()).unwrap(),
        context
    );
    assert_eq!(p["purpose_conflicting"], true);
    assert!(p["resolved"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r["freshness"] == "stale"));
    let token = p["token"].as_str().unwrap();
    assert_eq!(
        s.post(
            &format!("api/migration/cancel?job={id}"),
            &json!({"token":token}).to_string()
        )
        .0,
        200
    );
    assert_eq!(
        s.post(
            &format!("api/migration/assess?job={id}"),
            &json!({"token":token}).to_string()
        )
        .0,
        400
    );
    assert_eq!(
        s.get(&format!("reports/{id}/migration.assessment.html")).0,
        404
    );
    let p = preview(
        &s,
        id,
        &json!({"mode":"without_context","input_sha256":e.input().sha256,"as_of":ctx::AS_OF}),
    );
    assert!(p["context"].is_null());
    let r = confirm(&s, id, p["token"].as_str().unwrap());
    let actual: Value = serde_json::from_str(&s.get(r["json"].as_str().unwrap()).1).unwrap();
    assert!(actual["context"].is_null());
    assert_eq!(actual["decisions"][0]["urgency"], "undetermined");
}
#[test]
fn changed_inputs_invalid_scope_duplicate_fields_and_output_links_fail_safely() {
    let s = Session::start_with(true);
    let id = import(&s, &fixture::inventory("RSA"));
    let list = targets(&s, id);
    let request = generated_request(list["input"]["sha256"].as_str().unwrap());
    for selected in [json!([0]), json!([999]), json!([1, 1]), json!([])] {
        let mut bad = request.clone();
        bad["selected"] = selected;
        assert_eq!(
            s.post(&format!("api/migration/preview?job={id}"), &bad.to_string())
                .0,
            400
        );
    }
    assert_eq!(
        s.post(
            &format!("api/migration/preview?job={id}"),
            "{\"mode\":\"without_context\",\"mode\":\"generate\"}"
        )
        .0,
        400
    );
    let p = preview(&s, id, &request);
    let native = s.reports.join(format!("{id}/scan.inventory.v1.json"));
    let original = fs::read(&native).unwrap();
    let mut changed = original.clone();
    changed.push(b' ');
    fs::write(&native, changed).unwrap();
    assert_eq!(
        s.post(
            &format!("api/migration/assess?job={id}"),
            &json!({"token":p["token"]}).to_string()
        )
        .0,
        400
    );
    assert_eq!(
        s.get(&format!("reports/{id}/migration.assessment.html")).0,
        404
    );
    fs::write(&native, &original).unwrap();
    let p = preview(&s, id, &request);
    let dir = s.reports.join(format!("{id}/migration"));
    fs::create_dir(&dir).unwrap();
    fs::hard_link(&native, dir.join("migration-context.json")).unwrap();
    assert_eq!(
        s.post(
            &format!("api/migration/assess?job={id}"),
            &json!({"token":p["token"]}).to_string()
        )
        .0,
        400
    );
    assert_eq!(fs::read(native).unwrap(), original);
    assert!(!dir.join("migration.assessment.html").exists());
}
#[test]
fn selected_web_scan_captures_inventory_while_default_contract_is_retained() {
    let s = Session::start_with(true);
    let origin = format!("http://{}", s.host);
    let route = format!("{}api/scan?path=project%20%26%20%E6%97%A5", s.prefix);
    assert_eq!(s.request("POST", &route, &s.host, Some(&origin)).0, 202);
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let jobs: Value = serde_json::from_str(&s.get("api/jobs").1).unwrap();
        let j = &jobs["jobs"][0];
        if j["status"] != "running" {
            assert_eq!(j["status"], "ready");
            assert_eq!(j["migration_input"], true);
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(s.reports.join("1/scan.inventory.v1.json").is_file());
    let t = targets(&s, 1);
    let scan: pqc_scan_core::model::ScanResult =
        serde_json::from_slice(&fs::read(s.reports.join("1/report.json")).unwrap()).unwrap();
    let as_of = (scan.generated_at + Duration::from_secs(1))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    let p = preview(
        &s,
        1,
        &json!({"mode":"without_context","input_sha256":t["input"]["sha256"],"as_of":as_of}),
    );
    confirm(&s, 1, p["token"].as_str().unwrap());
    assert_eq!(s.get("reports/1/report.html").0, 200);
    assert_eq!(s.get("reports/1/migration.assessment.html").0, 200);
}
#[test]
fn ambiguous_targets_canaries_and_twenty_job_import_limit_are_enforced() {
    let s = Session::start_with(true);
    let mut value = fixture::inventory("RSA");
    value["observations"][0]["target"]["path"] = "<script>alert(1)</script>![x](evil.png)".into();
    value["arbitrary_metadata"] = json!({"secret":"SECRET_CANARY_074"});
    let id = import(&s, &value);
    let list = targets(&s, id);
    assert!(list["targets"][0]["path"]
        .as_str()
        .unwrap()
        .contains("<script"));
    let p = preview(
        &s,
        id,
        &json!({"mode":"without_context","input_sha256":list["input"]["sha256"],"as_of":ctx::AS_OF}),
    );
    let r = confirm(&s, id, p["token"].as_str().unwrap());
    let html = s.get(r["report"].as_str().unwrap()).1;
    assert!(!html.contains("<script>alert(1)"));
    assert!(!html.contains("SECRET_CANARY_074"));
    assert!(html.contains("&lt;script"));
    value["observations"][0]["identity_confidence"] = "ambiguous_duplicate_anchor".into();
    let id = import(&s, &value);
    let list = targets(&s, id);
    assert_eq!(list["targets"][0]["selectable"], false);
    assert_eq!(
        s.post(
            &format!("api/migration/preview?job={id}"),
            &generated_request(list["input"]["sha256"].as_str().unwrap()).to_string()
        )
        .0,
        400
    );
    for _ in 2..20 {
        import(&s, &fixture::inventory("RSA"));
    }
    assert_eq!(
        s.post(
            "api/migration/import",
            &fixture::inventory("RSA").to_string()
        )
        .0,
        400
    );
    assert!(!s.reports.join("21").exists());
}
#[test]
fn nested_duplicate_http_answers_are_rejected_before_preview_and_reports() {
    let s = Session::start_with(true);
    let id = import(&s, &fixture::inventory("RSA"));
    let list = targets(&s, id);
    let raw=generated_request(list["input"]["sha256"].as_str().unwrap()).to_string().replacen("\"answers\":[",r#""answers":[{"field":"deadline","state":"known","value":{"kind":"at","kind":"none"}},"#,1);
    assert_eq!(
        s.post(&format!("api/migration/preview?job={id}"), &raw).0,
        400
    );
    assert!(!s.reports.join(format!("{id}/migration")).exists());
}
#[test]
fn compact_large_context_import_preserves_every_supported_reference() {
    let s = Session::start_with(true);
    let v = fixture::inventory("RSA");
    let id = import(&s, &v);
    let e = fixture::ingest(&v, pqc_scan_core::migration::InputFormat::InventoryV1).unwrap();
    let mut context = ctx::context(&e, json!({}));
    context["references"]=Value::Array((0..7500).map(|n|json!({"id":format!("r{n}"),"issuer_id":"a","document_id":"a","edition":"a","clause":"a","scope_id":"a"})).collect());
    assert!(context.to_string().len() < 1024 * 1024);
    assert!(serde_json::to_vec_pretty(&context).unwrap().len() > 1024 * 1024);
    let p = preview(
        &s,
        id,
        &json!({"mode":"import_context","input_sha256":e.input().sha256,"as_of":ctx::AS_OF,"context":context.to_string()}),
    );
    assert_eq!(
        serde_json::from_str::<Value>(p["context"].as_str().unwrap()).unwrap(),
        context
    );
    assert!(p["context"].as_str().unwrap().len() <= 1024 * 1024);
}
#[test]
fn visible_question_hints_are_server_derived_for_signature_jwt_and_certificates() {
    let s = Session::start_with(true);
    let id = import(&s, &fixture::inventory("RSA"));
    let list = targets(&s, id);
    let hints = &list["targets"][0]["support_fields"];
    assert_eq!(
        hints["signature"],
        json!(["runtime", "provider", "parameters", "peer", "protocol"])
    );
    assert!(!hints["signature"]
        .as_array()
        .unwrap()
        .contains(&json!("verifiers")));
    assert!(hints["jwt"]
        .as_array()
        .unwrap()
        .contains(&json!("verifiers")));
}
