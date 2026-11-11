#![cfg(feature = "negotiated")]
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome},
    target_policy::*,
};
use pqc_scan_probe_ssh::SshKexProbe;
use std::{
    io::{Read, Write},
    net::TcpListener,
    time::{Duration, Instant},
};
struct NoDns;
impl Resolver for NoDns {
    fn resolve(
        &mut self,
        _: &str,
        _: Instant,
        _: &Cancellation,
    ) -> Result<Vec<std::net::IpAddr>, ProbeError> {
        panic!("numeric target must not use DNS");
    }
}
fn pin() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/ssh-kex-host.pub")
}
fn setup(
    limits: ProbeLimits,
    cancel: Cancellation,
) -> (TcpListener, AuthorizedTarget, ProbeBudget) {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let b = ProbeBudget::new(limits, cancel).unwrap();
    let t = TargetPolicy::new(&["127.0.0.1/32".into()])
        .unwrap()
        .resolve(
            TargetRequest::new("127.0.0.1", l.local_addr().unwrap().port()).unwrap(),
            &mut NoDns,
            &b,
        )
        .unwrap();
    (l, t, b)
}
#[test]
fn cancelled_before_connection_has_no_traffic() {
    let c = Cancellation::new();
    let (_l, t, b) = setup(ProbeLimits::default(), c.clone());
    c.cancel();
    let r = SshKexProbe::new(&pin()).unwrap().run(t, b);
    r.validate().unwrap();
    assert_eq!(r.diagnostics, vec![ProbeDiagnostic::Cancelled]);
    assert_eq!(r.traffic.sent_bytes, 0);
    assert_eq!(r.traffic.received_bytes, 0);
    assert!(!r.details.host_identity_verified);
}
#[test]
fn cancellation_and_timeout_interrupt_async_stalled_reads_and_close_socket() {
    for cancelled in [false, true] {
        let c = Cancellation::new();
        let (l, t, b) = setup(
            ProbeLimits {
                protocol_timeout: Duration::from_millis(200),
                ..Default::default()
            },
            c.clone(),
        );
        let peer = std::thread::spawn(move || {
            let (mut p, _) = l.accept().unwrap();
            p.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            let mut bytes = vec![];
            p.read_to_end(&mut bytes).unwrap();
            bytes
        });
        let trigger = std::thread::spawn(move || {
            if cancelled {
                std::thread::sleep(Duration::from_millis(35));
                c.cancel();
            }
        });
        let start = Instant::now();
        let r = SshKexProbe::new(&pin()).unwrap().run(t, b);
        trigger.join().unwrap();
        assert!(start.elapsed() < Duration::from_secs(1));
        assert_eq!(r.outcome, ProbeOutcome::Partial);
        assert_eq!(
            r.diagnostics,
            vec![if cancelled {
                ProbeDiagnostic::Cancelled
            } else {
                ProbeDiagnostic::DeadlineExceeded
            }]
        );
        let sent = peer.join().unwrap();
        assert_eq!(r.traffic.sent_bytes, sent.len() as u64);
        assert!(!r.details.handshake_complete);
        assert!(r.details.negotiated_kex.is_none());
        r.validate().unwrap();
    }
}
#[test]
fn async_transport_enforces_send_and_receive_budgets() {
    for sending in [true, false] {
        let (l, t, b) = setup(
            ProbeLimits {
                max_sent_bytes: if sending { 4 } else { 65536 },
                max_received_bytes: if sending { 524288 } else { 4 },
                ..Default::default()
            },
            Cancellation::new(),
        );
        let peer = std::thread::spawn(move || {
            let (mut p, _) = l.accept().unwrap();
            p.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            let _ = p.write_all(b"SSH-2.0-fixture\r\n");
            let mut sent = vec![];
            let _ = p.read_to_end(&mut sent);
            sent
        });
        let r = SshKexProbe::new(&pin()).unwrap().run(t, b);
        let bytes = peer.join().unwrap();
        assert_eq!(r.diagnostics, vec![ProbeDiagnostic::TrafficLimit]);
        assert_eq!(r.outcome, ProbeOutcome::Partial);
        assert_eq!(r.traffic.sent_bytes, bytes.len() as u64);
        assert!(r.traffic.sent_bytes <= r.limits.max_sent_bytes);
        assert!(r.traffic.received_bytes <= r.limits.max_received_bytes);
        assert!(!r.details.host_identity_verified);
        r.validate().unwrap();
    }
}
#[cfg(unix)]
#[test]
fn symlink_and_fifo_trust_inputs_are_rejected_without_blocking() {
    use std::{
        ffi::CString,
        os::unix::{ffi::OsStrExt, fs::symlink},
    };
    let p = std::env::temp_dir().join(format!("pqc-kex-file-{}", std::process::id()));
    std::fs::create_dir_all(&p).unwrap();
    symlink(pin(), p.join("link")).unwrap();
    assert!(SshKexProbe::new(&p.join("link")).is_err());
    let fifo = p.join("fifo");
    let c = CString::new(fifo.as_os_str().as_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
    let start = Instant::now();
    assert!(SshKexProbe::new(&fifo).is_err());
    assert!(start.elapsed() < Duration::from_millis(100));
    std::fs::remove_dir_all(p).unwrap();
}

#[test]
fn oversized_ssh_frame_is_rejected_before_large_body_allocation() {
    let (l, t, b) = setup(ProbeLimits::default(), Cancellation::new());
    let peer = std::thread::spawn(move || {
        let (mut p, _) = l.accept().unwrap();
        p.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        p.write_all(b"SSH-2.0-oversized\r\n\xff\xff\xff\xff")
            .unwrap();
        let mut sink = vec![];
        let _ = p.read_to_end(&mut sink);
    });
    let start = Instant::now();
    let r = SshKexProbe::new(&pin()).unwrap().run(t, b);
    peer.join().unwrap();
    assert!(start.elapsed() < Duration::from_secs(1));
    assert_eq!(r.outcome, ProbeOutcome::Partial);
    assert_eq!(r.diagnostics, vec![ProbeDiagnostic::LocalResourceLimit]);
    assert!(r.traffic.received_bytes < 1024);
    assert!(!r.details.host_identity_verified);
    assert!(r.details.negotiated_kex.is_none());
    r.validate().unwrap();
}

#[test]
fn synchronous_api_inside_async_runtime_fails_without_network_instead_of_panicking() {
    let (listener, target, budget) = setup(ProbeLimits::default(), Cancellation::new());
    listener.set_nonblocking(true).unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    let r = runtime.block_on(async { SshKexProbe::new(&pin()).unwrap().run(target, budget) });
    assert_eq!(r.diagnostics, vec![ProbeDiagnostic::UnsupportedLocal]);
    assert_eq!(r.traffic.sent_bytes, 0);
    assert!(listener.accept().is_err());
}
