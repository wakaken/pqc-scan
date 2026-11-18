use serde_json::Value;
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
        let root = std::env::temp_dir().join(format!("pqc-web-integration-{}", std::process::id()));
        fs::create_dir_all(root.join("project & 日")).unwrap();
        fs::write(
            root.join("project & 日/main.py"),
            "from Crypto.PublicKey import RSA\nkey = RSA.generate(2048)\n",
        )
        .unwrap();
        fs::write(root.join("private-canary.txt"), "DO_NOT_SERVE_CANARY").unwrap();
        let mut child = Command::new(env!("CARGO_BIN_EXE_pqc-scan"))
            .args([
                "serve",
                "--port",
                "0",
                "--no-open",
                "--threads",
                "1",
                "--root",
            ])
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
        let mut stream = TcpStream::connect(&self.host).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(20)))
            .unwrap();
        let origin = origin
            .map(|o| format!("Origin: {o}\r\n"))
            .unwrap_or_default();
        write!(stream,"{method} {path} HTTP/1.1\r\nHost: {host}\r\n{origin}{extra}Content-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
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
