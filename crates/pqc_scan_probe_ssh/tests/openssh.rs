#![cfg(all(feature = "runtime", unix))]
use pqc_scan_core::{
    probe::ProbeOutcome,
    target_policy::{
        Cancellation, ProbeBudget, ProbeError, ProbeLimits, Resolver, TargetPolicy, TargetRequest,
    },
};
use pqc_scan_probe_ssh::SshProbe;
use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
struct NoDns;
impl Resolver for NoDns {
    fn resolve(
        &mut self,
        _: &str,
        _: Instant,
        _: &Cancellation,
    ) -> Result<Vec<std::net::IpAddr>, ProbeError> {
        panic!("literal target must not resolve")
    }
}
struct Fixture {
    child: Child,
    reader: Option<JoinHandle<String>>,
    _dir: TestDirectory,
    port: u16,
    stopped: bool,
}
struct TestDirectory(PathBuf);
impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
impl Fixture {
    fn new(kex: &str) -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/task-033");
        std::fs::create_dir_all(&root).unwrap();
        let dir = root.join(format!(
            "openssh-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&dir).unwrap();
        let dir = TestDirectory(dir);
        let key = dir.0.join("host_ed25519");
        let result = Command::new("/usr/bin/ssh-keygen")
            .args(["-q", "-t", "ed25519", "-N", ""])
            .arg("-f")
            .arg(&key)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        assert!(result.success());
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        let config = dir.0.join("sshd_config");
        let quote = |path: PathBuf| {
            format!(
                "\"{}\"",
                path.to_string_lossy()
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
            )
        };
        std::fs::write(&config,format!("ListenAddress 127.0.0.1\nPort {port}\nHostKey {}\nPidFile {}\nAuthorizedKeysFile none\nPasswordAuthentication no\nKbdInteractiveAuthentication no\nPubkeyAuthentication no\nUsePAM no\nUseDNS no\nPermitRootLogin no\nAllowAgentForwarding no\nAllowTcpForwarding no\nX11Forwarding no\nLogLevel DEBUG3\nKexAlgorithms {kex}\nCiphers aes128-ctr\nMACs hmac-sha2-256\nCompression no\n",quote(key),quote(dir.0.join("pid")))).unwrap();
        let child = Command::new("/usr/sbin/sshd")
            .args(["-ddd", "-e", "-f"])
            .arg(config)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .process_group(0)
            .spawn()
            .unwrap();
        let mut fixture = Self {
            child,
            reader: None,
            _dir: dir,
            port,
            stopped: false,
        };
        let stderr = fixture.child.stderr.take().unwrap();
        let (tx, rx) = mpsc::channel();
        let reader = thread::spawn(move || {
            let mut result = String::new();
            for line in BufReader::new(stderr).lines() {
                let Ok(line) = line else { break };
                if line.contains("Server listening on 127.0.0.1 port") {
                    let _ = tx.send(());
                }
                if result.len() < 65536 {
                    result.push_str(&line);
                    result.push('\n');
                }
            }
            result
        });
        fixture.reader = Some(reader);
        assert!(
            rx.recv_timeout(Duration::from_secs(3)).is_ok(),
            "isolated OpenSSH fixture did not start"
        );
        fixture
    }
    fn stop(&mut self) -> String {
        if self.stopped {
            return String::new();
        }
        let pid = libc::pid_t::try_from(self.child.id()).unwrap();
        // SAFETY: the positive PID identifies our unreaped child, spawned in its own
        // process group. Its negative value targets only this fixture's descendants.
        unsafe {
            libc::kill(-pid, libc::SIGTERM);
        }
        let _ = self.child.wait();
        self.stopped = true;
        self.reader
            .take()
            .map(|reader| reader.join().unwrap())
            .unwrap_or_default()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.stop();
    }
}

#[test]
#[ignore = "requires explicit isolated execution with installed OpenSSH daemon/keygen"]
fn installed_openssh_classical_and_hybrid_offers_have_zero_authentication_messages() {
    // Test-peer selection only; the product never reads this environment value.
    let hybrid = std::env::var("PQC_SCAN_TEST_SSH_HYBRID")
        .unwrap_or_else(|_| "mlkem768x25519-sha256".into());
    assert!(
        matches!(
            hybrid.as_str(),
            "mlkem768x25519-sha256"
                | "sntrup761x25519-sha512"
                | "sntrup761x25519-sha512@openssh.com"
        ),
        "unsupported test-only SSH hybrid profile"
    );
    for (label, kex) in [
        ("classical", "curve25519-sha256"),
        ("hybrid", hybrid.as_str()),
    ] {
        let mut fixture = Fixture::new(kex);
        let budget = ProbeBudget::new(ProbeLimits::default(), Cancellation::new()).unwrap();
        let target = TargetPolicy::new(&["127.0.0.1/32".into()])
            .unwrap()
            .resolve(
                TargetRequest::new("127.0.0.1", fixture.port).unwrap(),
                &mut NoDns,
                &budget,
            )
            .unwrap();
        let report = SshProbe::new().run(target, budget);
        let log = fixture.stop();
        assert_eq!(report.outcome,ProbeOutcome::Completed,"local OpenSSH offer collection failed: diagnostics={:?}, identification={:?}, fixture_errors={:?}",report.diagnostics,report.details.server_identification,log.lines().filter(|line|line.contains("fatal:")||line.contains("error:")||line.contains("sandbox")||line.contains("Bad packet")).collect::<Vec<_>>());
        assert!(report
            .details
            .kex_candidates
            .iter()
            .any(|candidate| candidate.name == kex));
        let received: Vec<u8> = log
            .lines()
            .filter_map(|line| {
                line.split("receive packet: type ")
                    .nth(1)?
                    .split_whitespace()
                    .next()?
                    .parse()
                    .ok()
            })
            .collect();
        assert_eq!(
            received,
            [20],
            "only the diagnostic KEXINIT may reach the server"
        );
        assert!(!log.contains("userauth-request"));
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/task-033");
        std::fs::write(
            root.join(format!("openssh-{label}-report.json")),
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        println!(
            "{}",
            serde_json::json!({"profile":label,"offered_kex":kex,"received_message_types":received,"service_requests":0,"userauth_requests":0,"channel_requests":0})
        );
    }
}
