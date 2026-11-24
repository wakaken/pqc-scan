//! A loopback-only UI. Source contents are never served; scans use the CLI pipeline.
use anyhow::{bail, Context, Result};
use clap::Parser;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};
#[path = "serve_migration.rs"]
mod migration_ui;

#[derive(Debug, Parser)]
pub(super) struct ServeArgs {
    /// Folder-picker boundary; defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Loopback port; use 0 to request an available port.
    #[arg(long, default_value_t = 8787)]
    port: u16,
    #[arg(long, default_value = "./rules")]
    rules_dir: PathBuf,
    #[arg(long, default_value_t = 4usize)]
    threads: usize,
    /// Initial per-file size limit in MiB (1..64), editable in the folder picker.
    #[arg(long, default_value_t = super::file_limits::DEFAULT_MIB, value_parser = super::file_limits::parse_mib)]
    max_file_size_mib: usize,
    /// Do not open the default browser automatically.
    #[arg(long)]
    no_open: bool,
    /// Enable optional migration context forms and saved-evidence reassessment.
    #[arg(long)]
    migration: bool,
}
#[derive(Default)]
struct Jobs {
    active: bool,
    next: u64,
    entries: BTreeMap<u64, Value>,
}
struct State {
    root: PathBuf,
    rules: PathBuf,
    output: PathBuf,
    origin: String,
    prefix: String,
    threads: usize,
    max_file_size_mib: usize,
    jobs: Mutex<Jobs>,
    migration: bool,
    drafts: Mutex<BTreeMap<u64, migration_ui::Draft>>,
    generations: Mutex<BTreeMap<u64, migration_ui::Generation>>,
}
fn random_token() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|_| anyhow::anyhow!("could not obtain session randomness"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}
fn session_directory() -> Result<PathBuf> {
    let path = std::env::temp_dir().join(format!("pqc-scan-web-{}", random_token()?));
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&path)?;
    Ok(path.canonicalize()?)
}
pub(super) fn run(args: ServeArgs, stdout: &mut impl Write) -> Result<()> {
    let root = args
        .root
        .canonicalize()
        .context("folder-picker root is unavailable")?;
    if !root.is_dir() {
        bail!("folder-picker root must be a directory");
    }
    let rules = args
        .rules_dir
        .canonicalize()
        .context("rules directory is unavailable")?;
    let set = pqc_scan_rules::RuleSet::load_from_dir(&rules)?;
    if set.is_empty() {
        bail!("no rules found");
    }
    let server = Server::http((std::net::Ipv4Addr::LOCALHOST, args.port))
        .map_err(|e| anyhow::anyhow!("could not start local server: {e}"))?;
    let origin = format!("http://{}", server.server_addr());
    let state = Arc::new(State {
        root,
        rules,
        output: session_directory()?,
        origin,
        prefix: format!("/{}/", random_token()?),
        threads: args.threads,
        max_file_size_mib: args.max_file_size_mib,
        jobs: Mutex::new(Jobs::default()),
        migration: args.migration,
        drafts: Mutex::new(BTreeMap::new()),
        generations: Mutex::new(BTreeMap::new()),
    });
    let url = format!("{}{}", state.origin, state.prefix);
    writeln!(stdout, "pqc-scan local web UI: {url}")?;
    writeln!(stdout, "Folder boundary: {}", state.root.display())?;
    writeln!(stdout, "Session reports: {}", state.output.display())?;
    writeln!(stdout, "Stop with Ctrl+C. Keep the session URL private.")?;
    stdout.flush()?;
    if !args.no_open {
        open_browser(&url);
    }
    for request in server.incoming_requests() {
        handle(request, &state);
    }
    Ok(())
}
fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let command = Some(("open", vec![url]));
    #[cfg(target_os = "linux")]
    let command = Some(("xdg-open", vec![url]));
    #[cfg(target_os = "windows")]
    let command = Some(("rundll32", vec!["url.dll,FileProtocolHandler", url]));
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let command: Option<(&str, Vec<&str>)> = None;
    if let Some((program, args)) = command {
        let _ = std::process::Command::new(program).args(args).spawn();
    }
}
fn header<'a>(request: &'a Request, name: &'static str) -> Option<&'a str> {
    request
        .headers()
        .iter()
        .find(|h| h.field.equiv(name))
        .map(|h| h.value.as_str())
}
fn reply(request: Request, code: u16, mime: &str, bytes: Vec<u8>) {
    let mut response = Response::from_data(bytes).with_status_code(StatusCode(code));
    for (name,value) in [("Content-Type",mime),("Cache-Control","no-store"),("Referrer-Policy","no-referrer"),("X-Content-Type-Options","nosniff"),("Content-Security-Policy","default-src 'none'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; frame-ancestors 'none'; base-uri 'none'; form-action 'self'")] {
        if let Ok(h)=Header::from_bytes(name,value) {response.add_header(h);}
    }
    if code == 200
        && request
            .url()
            .split('?')
            .next()
            .is_some_and(|path| path.ends_with("/migration-context.json"))
    {
        response.add_header(
            Header::from_bytes(
                "Content-Disposition",
                "attachment; filename=\"migration-context.json\"",
            )
            .expect("fixed download header"),
        );
    }
    let _ = request.respond(response);
}
fn json_reply(request: Request, code: u16, value: Value) {
    reply(
        request,
        code,
        "application/json; charset=utf-8",
        serde_json::to_vec(&value).expect("JSON value"),
    );
}
fn handle(request: Request, state: &Arc<State>) {
    let expected_host = state.origin.trim_start_matches("http://");
    if header(&request, "Host") != Some(expected_host) {
        json_reply(request, 403, json!({"error":"Invalid host"}));
        return;
    }
    let document_navigation = request.method() == &Method::Get
        && header(&request, "Sec-Fetch-Mode") == Some("navigate")
        && header(&request, "Sec-Fetch-Dest") == Some("document");
    if header(&request, "Origin").is_some_and(|o| o != state.origin)
        || (header(&request, "Sec-Fetch-Site") == Some("cross-site") && !document_navigation)
    {
        json_reply(request, 403, json!({"error":"Use the local session page"}));
        return;
    }
    if request.method() == &Method::Post
        && header(&request, "Origin") != Some(state.origin.as_str())
    {
        json_reply(
            request,
            403,
            json!({"error":"A same-origin scan request is required"}),
        );
        return;
    }
    let Some(url) = request.url().strip_prefix(&state.prefix).map(str::to_owned) else {
        json_reply(
            request,
            404,
            json!({"error":"Open the session URL printed by pqc-scan"}),
        );
        return;
    };
    if url.len() > 8192 {
        json_reply(request, 400, json!({"error":"Request is too long"}));
        return;
    }
    let (route, query) = url.split_once('?').unwrap_or((&url, ""));
    if route.starts_with("api/migration/") {
        if state.migration {
            migration_ui::handle(request, state, route, query);
        } else {
            json_reply(request, 404, json!({"error":"Not found"}));
        }
        return;
    }
    if request.method() == &Method::Get {
        match route {
            "api/config" => json_reply(
                request,
                200,
                json!({"migration":state.migration,"max_file_size_mib":state.max_file_size_mib}),
            ),
            "style.css" => reply(
                request,
                200,
                "text/css; charset=utf-8",
                include_bytes!("serve.css").to_vec(),
            ),
            "context-model.js" if state.migration => reply(
                request,
                200,
                "text/javascript; charset=utf-8",
                include_bytes!("serve-context.js").to_vec(),
            ),
            "migration-form.html" if state.migration => reply(
                request,
                200,
                "text/html; charset=utf-8",
                include_bytes!("serve-migration.html").to_vec(),
            ),
            "report-view.js" if state.migration => reply(
                request,
                200,
                "text/javascript; charset=utf-8",
                pqc_scan_report::migration_view_script().as_bytes().to_vec(),
            ),
            "report-view.css" if state.migration => reply(
                request,
                200,
                "text/css; charset=utf-8",
                pqc_scan_report::migration_view_styles().as_bytes().to_vec(),
            ),
            "migration.js" if state.migration => reply(
                request,
                200,
                "text/javascript; charset=utf-8",
                include_bytes!("serve-migration.js").to_vec(),
            ),
            "" => reply(
                request,
                200,
                "text/html; charset=utf-8",
                include_bytes!("serve.html").to_vec(),
            ),
            "report-session.js" => reply(
                request,
                200,
                "text/javascript; charset=utf-8",
                include_bytes!("report-session.js").to_vec(),
            ),
            "app.js" => reply(
                request,
                200,
                "text/javascript; charset=utf-8",
                include_bytes!("serve.js").to_vec(),
            ),
            "api/directories" => match decode(query.strip_prefix("path=").unwrap_or(""))
                .and_then(|p| directories(state, &p))
            {
                Ok(value) => json_reply(request, 200, value),
                Err(_) => json_reply(
                    request,
                    400,
                    json!({"error":"This folder is unavailable or outside the selected root."}),
                ),
            },
            "api/jobs" => json_reply(
                request,
                200,
                json!({"jobs":state.jobs.lock().unwrap().entries.values().collect::<Vec<_>>()}),
            ),
            _ => report_file(request, state, route),
        }
    } else if request.method() == &Method::Post && route == "api/scan" {
        match scan_query(query)
            .and_then(|(path, exclude_tests, limit)| start_scan(state, &path, exclude_tests, limit))
        {
            Ok(id) => json_reply(request, 202, json!({"id":id})),
            Err(_) => json_reply(
                request,
                400,
                json!({"error":"Cannot start scan. Choose an available folder inside the root, wait for the active scan, or restart after 20 scans."}),
            ),
        }
    } else {
        json_reply(request, 405, json!({"error":"Method not allowed"}));
    }
}
fn decode(value: &str) -> Result<String> {
    let mut bytes = Vec::new();
    let mut i = 0;
    let raw = value.as_bytes();
    while i < raw.len() {
        match raw[i] {
            b'%' => {
                if i + 2 >= raw.len() {
                    bail!("invalid escape");
                }
                let s = std::str::from_utf8(&raw[i + 1..i + 3])?;
                bytes.push(u8::from_str_radix(s, 16)?);
                i += 3;
            }
            b'+' => {
                bytes.push(b' ');
                i += 1;
            }
            b => {
                bytes.push(b);
                i += 1;
            }
        }
    }
    Ok(String::from_utf8(bytes)?)
}
fn target(root: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    if path
        .components()
        .any(|c| !matches!(c, Component::Normal(_)))
        && !relative.is_empty()
    {
        bail!("invalid relative path");
    }
    let mut current = root.to_path_buf();
    for part in path.components() {
        current.push(part);
        if fs::symlink_metadata(&current)?.file_type().is_symlink() {
            bail!("linked folder");
        }
    }
    let canonical = current.canonicalize()?;
    if !canonical.starts_with(root) || !canonical.is_dir() {
        bail!("outside root or not directory");
    }
    Ok(canonical)
}
fn directories(state: &State, relative: &str) -> Result<Value> {
    let folder = target(&state.root, relative)?;
    let mut children = Vec::new();
    let mut omitted = 0;
    for entry in fs::read_dir(folder)? {
        let Ok(entry) = entry else {
            omitted += 1;
            continue;
        };
        let Ok(kind) = entry.file_type() else {
            omitted += 1;
            continue;
        };
        if !kind.is_dir() || kind.is_symlink() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            omitted += 1;
            continue;
        };
        if name == ".git" {
            continue;
        }
        if children.len() >= 1000 {
            omitted += 1;
            continue;
        }
        let path = if relative.is_empty() {
            name.clone()
        } else {
            format!("{relative}/{name}")
        };
        children.push(json!({"name":name,"path":path}));
    }
    children.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    Ok(
        json!({"path":relative,"root":state.root.display().to_string(),"directories":children,"omitted":omitted}),
    )
}
fn session_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
fn scan_query(query: &str) -> Result<(String, bool, Option<usize>)> {
    let mut path = None;
    let mut tests = None;
    let mut limit = None;
    for pair in query.split('&').filter(|p| !p.is_empty()) {
        let (key, value) = pair
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("invalid scan query"))?;
        match key {
            "path" if path.is_none() => path = Some(decode(value)?),
            "max_file_size_mib" if limit.is_none() => {
                limit = Some(super::file_limits::parse_mib(value).map_err(anyhow::Error::msg)?)
            }
            "exclude_tests" if tests.is_none() => {
                tests = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => bail!("invalid test selection"),
                })
            }
            _ => bail!("unknown or duplicate scan option"),
        }
    }
    Ok((path.unwrap_or_default(), tests.unwrap_or(false), limit))
}

fn start_scan(
    state: &Arc<State>,
    relative: &str,
    exclude_tests: bool,
    limit: Option<usize>,
) -> Result<u64> {
    let max_file_size_mib = limit.unwrap_or(state.max_file_size_mib);
    super::file_limits::bytes(max_file_size_mib)?;
    let path = target(&state.root, relative)?;
    if state.output.starts_with(&path) {
        bail!("session reports are inside selected source");
    }
    let mut jobs = state.jobs.lock().unwrap();
    if jobs.active || jobs.next >= 20 {
        bail!("busy or session limit");
    }
    jobs.next += 1;
    let id = jobs.next;
    jobs.active = true;
    let created_at_ms = session_time_ms();
    let display_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("/")
        .to_owned();
    jobs.entries.insert(id, json!({"id":id,"path":relative,"display_name":display_name,"created_at_ms":created_at_ms,"exclude_tests":exclude_tests,"max_file_size_mib":max_file_size_mib,"status":"running"}));
    let owner = Arc::clone(state);
    let relative = relative.to_string();
    drop(jobs);
    std::thread::spawn(move || {
        let output = owner.output.join(id.to_string());
        let args = super::ScanArgs {
            path,
            format: super::FormatArg::All,
            lang: super::LanguageArg::Ja,
            cyclonedx_17: false,
            out_dir: output.clone(),
            rules_dir: owner.rules.clone(),
            fail_on: None,
            threads: owner.threads,
            max_file_size_mib,
            include_non_scannable: false,
            exclude_tests,
            diagnostics: true,
            inventory_json: owner.migration,
            policy_profile: None,
            assessment_context: None,
            repository_scope: None,
            require_complete: false,
        };
        let observer = |event| {
            use pqc_scan_core::pipeline::ScanProgress as P;
            let mut jobs = owner.jobs.lock().unwrap();
            if let Some(job) = jobs.entries.get_mut(&id) {
                job["progress"] = match event {
                    P::Enumerating => json!({"phase":"enumerating"}),
                    P::Inspecting { processed, total } => {
                        let previous = job["progress"]["processed"].as_u64().unwrap_or(0);
                        json!({"phase":"inspecting","processed":processed.max(previous),"total":total})
                    }
                    P::Aggregating => json!({"phase":"aggregating"}),
                    P::WritingReports => json!({"phase":"writing"}),
                };
            }
        };
        let result = std::panic::catch_unwind(|| {
            super::scan_cmd_with_progress(
                args,
                &mut std::io::sink(),
                &mut std::io::sink(),
                Some(&observer),
            )
        });
        let mut value = match result {
            Ok(Ok(_)) => match fs::read(output.join("diagnostics.json"))
                .ok()
                .and_then(|v| serde_json::from_slice::<Value>(&v).ok())
            {
                Some(d) => {
                    json!({"id":id,"path":relative,"status":"ready","coverage":d["coverage"]["status"],"report":format!("reports/{id}/report.html")})
                }
                None => json!({"id":id,"path":relative,"status":"failed"}),
            },
            _ => json!({"id":id,"path":relative,"status":"failed"}),
        };
        value["display_name"] = display_name.into();
        value["created_at_ms"] = created_at_ms.into();
        value["exclude_tests"] = exclude_tests.into();
        value["max_file_size_mib"] = max_file_size_mib.into();
        if owner.migration && value["status"] == "ready" {
            value["migration_input"] = true.into();
        }
        let mut jobs = owner.jobs.lock().unwrap();
        jobs.active = false;
        jobs.entries.insert(id, value);
    });
    Ok(id)
}
fn report_file(request: Request, state: &State, route: &str) {
    let parts: Vec<_> = route.split('/').collect();
    if parts.len() != 3 || parts[0] != "reports" {
        json_reply(request, 404, json!({"error":"Not found"}));
        return;
    }
    let Ok(id) = parts[1].parse::<u64>() else {
        json_reply(request, 404, json!({"error":"Not found"}));
        return;
    };
    if !state
        .jobs
        .lock()
        .unwrap()
        .entries
        .get(&id)
        .is_some_and(|j| j["status"] == "ready")
    {
        json_reply(request, 404, json!({"error":"Report is not ready"}));
        return;
    }
    let migration_artifact = matches!(
        parts[2],
        "migration.assessment.html"
            | "migration.assessment.md"
            | "migration.assessment.v1.json"
            | "migration-context.json"
    );
    if migration_artifact
        && (!state.migration
            || !state
                .jobs
                .lock()
                .unwrap()
                .entries
                .get(&id)
                .is_some_and(|j| j["migration_ready"] == true))
    {
        json_reply(request, 404, json!({"error":"Report is not ready"}));
        return;
    }
    let mime = match parts[2] {
        "migration.assessment.html" if migration_artifact => "text/html; charset=utf-8",
        "migration.assessment.md" if migration_artifact => "text/markdown; charset=utf-8",
        "migration.assessment.v1.json" | "migration-context.json" if migration_artifact => {
            "application/json; charset=utf-8"
        }
        "report.html" | "report.rules.html" => "text/html; charset=utf-8",
        "report.json"
        | "report.sarif"
        | "cbom.json"
        | "dependency-sbom.json"
        | "diagnostics.json" => "application/json; charset=utf-8",
        "report.md" => "text/markdown; charset=utf-8",
        _ => {
            json_reply(request, 404, json!({"error":"Not found"}));
            return;
        }
    };
    let directory = state.output.join(id.to_string());
    let file = if migration_artifact {
        directory.join("migration").join(parts[2])
    } else {
        directory.join(parts[2])
    };
    match fs::read(file) {
        Ok(bytes) if mime.starts_with("text/html") => {
            let scan_reference = if migration_artifact && directory.join("report.html").is_file() {
                " data-scan-report"
            } else {
                ""
            };
            let html = String::from_utf8_lossy(&bytes)
                .replacen("</head>", "<script defer src=\"../../report-session.js\"></script></head>", 1)
                .replacen("<body>", &format!("<body><nav class=\"report-settings\" aria-label=\"Session / セッション\"><a id=\"pqc-session-back\" href=\"../../?result={id}\"{scan_reference}>結果に戻る</a></nav>"), 1);
            reply(request, 200, mime, html.into_bytes());
        }
        Ok(bytes) => reply(request, 200, mime, bytes),
        Err(_) => json_reply(request, 404, json!({"error":"Report is unavailable"})),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture(PathBuf);
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn scan_options_decode_path_once_and_reject_ambiguous_flags() {
        assert_eq!(
            scan_query("path=project%20%26%20name&exclude_tests=true").unwrap(),
            ("project & name".into(), true, None)
        );
        assert_eq!(
            scan_query("path=tests").unwrap(),
            ("tests".into(), false, None)
        );
        assert_eq!(scan_query("").unwrap(), ("".into(), false, None));
        assert_eq!(
            scan_query("path=a&max_file_size_mib=16").unwrap(),
            ("a".into(), false, Some(16))
        );
        for query in [
            "path=a&path=b",
            "path=a&exclude_tests=1",
            "path=a&exclude_tests=true&exclude_tests=false",
            "unknown=x",
            "path=a&max_file_size_mib=0",
            "path=a&max_file_size_mib=65",
            "path=a&max_file_size_mib=2.5",
            "path=a&max_file_size_mib=99999999999999999999",
            "path=a&max_file_size_mib=16&max_file_size_mib=4",
        ] {
            assert!(scan_query(query).is_err(), "{query}");
        }
    }
    #[test]
    fn picker_rejects_traversal_absolute_files_and_links_and_lists_only_directories() {
        let f = Fixture(session_directory().unwrap());
        fs::create_dir(f.0.join("project & 日本語")).unwrap();
        fs::write(f.0.join("source.txt"), "not served").unwrap();
        let canonical_root = f.0.canonicalize().unwrap();
        for invalid in ["../", "/", "project/../../", "source.txt", "missing"] {
            assert!(target(&canonical_root, invalid).is_err(), "{invalid}");
        }
        assert_eq!(
            target(&canonical_root, "").unwrap(),
            f.0.canonicalize().unwrap()
        );
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(std::env::temp_dir(), f.0.join("outside")).unwrap();
            assert!(target(&canonical_root, "outside").is_err());
        }
        let state = Arc::new(State {
            root: f.0.canonicalize().unwrap(),
            rules: f.0.clone(),
            output: f.0.join("outputs"),
            origin: "http://127.0.0.1:1".into(),
            prefix: "/token/".into(),
            threads: 1,
            max_file_size_mib: 2,
            jobs: Mutex::new(Jobs::default()),
            migration: false,
            drafts: Mutex::new(BTreeMap::new()),
            generations: Mutex::new(BTreeMap::new()),
        });
        let listing = directories(&state, "").unwrap();
        assert_eq!(listing["directories"].as_array().unwrap().len(), 1);
        assert_eq!(listing["directories"][0]["name"], "project & 日本語");
        assert!(
            start_scan(&state, "", false, None).is_err(),
            "do not scan own session artifacts"
        );
        state.jobs.lock().unwrap().active = true;
        assert!(
            start_scan(&state, "project & 日本語", false, None).is_err(),
            "single active job"
        );
        state.jobs.lock().unwrap().active = false;
        state.jobs.lock().unwrap().next = 20;
        assert!(
            start_scan(&state, "project & 日本語", false, None).is_err(),
            "bounded session"
        );
    }
    #[test]
    fn directory_listing_reports_the_exact_thousand_entry_boundary() {
        let fixture = Fixture(session_directory().unwrap());
        for i in 0..1000 {
            fs::create_dir(fixture.0.join(format!("project-{i:04}"))).unwrap();
        }
        let state = State {
            root: fixture.0.clone(),
            rules: fixture.0.clone(),
            output: fixture.0.join("outputs"),
            origin: "http://127.0.0.1:1".into(),
            prefix: "/token/".into(),
            threads: 1,
            max_file_size_mib: 2,
            jobs: Mutex::new(Jobs::default()),
            migration: false,
            drafts: Mutex::new(BTreeMap::new()),
            generations: Mutex::new(BTreeMap::new()),
        };
        let exact = directories(&state, "").unwrap();
        assert_eq!(exact["directories"].as_array().unwrap().len(), 1000);
        assert_eq!(exact["omitted"], 0);
        fs::create_dir(fixture.0.join("project-1000")).unwrap();
        let over = directories(&state, "").unwrap();
        assert_eq!(over["directories"].as_array().unwrap().len(), 1000);
        assert_eq!(over["omitted"], 1);
    }
    #[test]
    fn request_paths_decode_once_and_reject_invalid_utf8_and_escapes() {
        assert_eq!(decode("folder%20%26%20%E6%97%A5").unwrap(), "folder & 日");
        for bad in ["%", "%2", "%zz", "%ff"] {
            assert!(decode(bad).is_err());
        }
        assert_eq!(decode("%252e%252e").unwrap(), "%2e%2e");
    }
}

#[cfg(test)]
mod failure_test {
    use super::*;
    #[test]
    fn completed_jobs_retain_scope_for_success_failure_and_both_choices() {
        for exclude_tests in [false, true] {
            for fail in [false, true] {
                let root = session_directory().unwrap();
                fs::create_dir_all(root.join("project/tests")).unwrap();
                fs::write(root.join("project/main.py"), "alg = 'RS256'").unwrap();
                fs::write(root.join("project/tests/test_main.py"), "alg = 'RS512'").unwrap();
                let state = Arc::new(State {
                    root: root.canonicalize().unwrap(),
                    rules: if fail {
                        root.join("missing-rules")
                    } else {
                        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules/default")
                    },
                    output: root.join("reports"),
                    origin: "http://127.0.0.1:1".into(),
                    prefix: "/token/".into(),
                    threads: 1,
                    max_file_size_mib: 2,
                    jobs: Mutex::new(Jobs::default()),
                    migration: false,
                    drafts: Mutex::new(BTreeMap::new()),
                    generations: Mutex::new(BTreeMap::new()),
                });
                let id = start_scan(&state, "project", exclude_tests, None).unwrap();
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
                loop {
                    let jobs = state.jobs.lock().unwrap();
                    if !jobs.active {
                        assert_eq!(jobs.entries[&id]["exclude_tests"], exclude_tests);
                        assert_eq!(
                            jobs.entries[&id]["status"],
                            if fail { "failed" } else { "ready" }
                        );
                        break;
                    }
                    drop(jobs);
                    assert!(std::time::Instant::now() < deadline);
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                if !fail {
                    let d: Value = serde_json::from_slice(
                        &fs::read(root.join(format!("reports/{id}/diagnostics.json"))).unwrap(),
                    )
                    .unwrap();
                    assert_eq!(
                        d["coverage"]["selected_files"],
                        if exclude_tests { 1 } else { 2 }
                    );
                }
                fs::remove_dir_all(root).unwrap();
            }
        }
    }
    #[test]
    fn scan_failure_is_recorded_and_releases_the_busy_state() {
        let root = session_directory().unwrap();
        fs::create_dir(root.join("project")).unwrap();
        let state = Arc::new(State {
            root: root.canonicalize().unwrap(),
            rules: root.join("missing-rules"),
            output: root.join("reports"),
            origin: "http://127.0.0.1:1".into(),
            prefix: "/token/".into(),
            threads: 1,
            max_file_size_mib: 2,
            jobs: Mutex::new(Jobs::default()),
            migration: false,
            drafts: Mutex::new(BTreeMap::new()),
            generations: Mutex::new(BTreeMap::new()),
        });
        let id = start_scan(&state, "project", false, None).unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            let jobs = state.jobs.lock().unwrap();
            if !jobs.active {
                assert_eq!(jobs.entries[&id]["status"], "failed");
                assert!(jobs.entries[&id].get("report").is_none());
                break;
            }
            drop(jobs);
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        fs::remove_dir_all(root).unwrap();
    }
}
