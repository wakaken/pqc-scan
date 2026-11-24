#![cfg(all(
    unix,
    any(
        feature = "probe-tls",
        feature = "probe-ssh",
        feature = "probe-ssh-kex"
    )
))]

use std::{
    fs,
    io::Read,
    net::TcpListener,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct RunningProbe {
    child: Child,
    output: PathBuf,
}
impl Drop for RunningProbe {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.output);
    }
}

fn interrupts_blocked_peer(protocol: &str, filename: &str) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let output = std::env::temp_dir().join(format!(
        "pqc-sigint-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut command = Command::new(env!("CARGO_BIN_EXE_pqc-scan"));
    command.args([
        "probe",
        protocol,
        "127.0.0.1",
        "--port",
        &listener.local_addr().unwrap().port().to_string(),
        "--allow-cidr",
        "127.0.0.1/32",
        "--out-dir",
    ]);
    command
        .arg(&output)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if protocol == "ssh-kex" {
        command.arg("--host-key-file").arg(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/ssh-kex-host.pub"),
        );
    }
    let mut probe = RunningProbe {
        child: command.spawn().unwrap(),
        output,
    };
    let ready_deadline = Instant::now() + Duration::from_secs(5);
    let mut peer = loop {
        match listener.accept() {
            Ok((peer, _)) => break peer,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(Instant::now() < ready_deadline, "probe did not connect");
                assert!(
                    probe.child.try_wait().unwrap().is_none(),
                    "probe exited early"
                );
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(error) => panic!("fixture accept failed: {error}"),
        }
    };
    // The peer deliberately withholds its handshake. SIGINT must interrupt the
    // blocked transport, rather than waiting for the five-second protocol timeout.
    let interrupted = Instant::now();
    assert_eq!(
        unsafe { libc::kill(probe.child.id() as i32, libc::SIGINT) },
        0
    );
    let _ = unsafe { libc::kill(probe.child.id() as i32, libc::SIGINT) };
    let status = loop {
        if let Some(status) = probe.child.try_wait().unwrap() {
            break status;
        }
        assert!(
            interrupted.elapsed() < Duration::from_secs(2),
            "SIGINT did not interrupt probe"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(status.code(), Some(3));
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(probe.output.join(filename)).unwrap()).unwrap();
    assert_eq!(report["diagnostics"], serde_json::json!(["cancelled"]));
    assert_ne!(report["outcome"], "completed");
    peer.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let mut bytes = [0; 4096];
    while peer.read(&mut bytes).unwrap() != 0 {}
}

#[cfg(feature = "probe-tls")]
#[test]
fn sigint_stops_blocked_tls_and_writes_cancelled_result() {
    interrupts_blocked_peer("tls", "probe.tls.v1.json");
}

#[cfg(feature = "probe-ssh")]
#[test]
fn sigint_stops_blocked_ssh_and_writes_cancelled_result() {
    interrupts_blocked_peer("ssh", "probe.ssh-offers.v1.json");
}

#[cfg(feature = "probe-ssh-kex")]
#[test]
fn sigint_stops_blocked_ssh_kex_and_writes_cancelled_result() {
    interrupts_blocked_peer("ssh-kex", "probe.ssh-kex.v1.json");
}
