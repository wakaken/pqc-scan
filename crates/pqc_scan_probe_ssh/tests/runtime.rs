#![cfg(feature = "runtime")]
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome},
    target_policy::{
        AuthorizedTarget, Cancellation, ProbeBudget, ProbeError, ProbeLimits, Resolver,
        TargetPolicy, TargetRequest,
    },
};
use pqc_scan_probe_ssh::{encode_kexinit, encode_packet, parse_kexinit, parse_packet, SshProbe};
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
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
        panic!("literal target must not resolve")
    }
}
fn setup(
    limits: ProbeLimits,
    cancel: Cancellation,
) -> (TcpListener, AuthorizedTarget, ProbeBudget) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let budget = ProbeBudget::new(limits, cancel).unwrap();
    let policy = TargetPolicy::new(&["127.0.0.1/32".into()]).unwrap();
    let target = policy
        .resolve(
            TargetRequest::new("127.0.0.1", address.port()).unwrap(),
            &mut NoDns,
            &budget,
        )
        .unwrap();
    (listener, target, budget)
}
fn accept(listener: TcpListener) -> TcpStream {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        match listener.accept() {
            Ok((s, _)) => {
                s.set_nonblocking(false).unwrap();
                s.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
                s.set_write_timeout(Some(Duration::from_secs(1))).unwrap();
                return s;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(Instant::now() < deadline);
                thread::sleep(Duration::from_millis(1));
            }
            Err(e) => panic!("accept {e}"),
        }
    }
}
fn line(s: &mut TcpStream) -> Vec<u8> {
    let mut out = vec![];
    for _ in 0..255 {
        let mut b = [0];
        if s.read(&mut b).unwrap() == 0 {
            break;
        }
        out.push(b[0]);
        if b[0] == b'\n' {
            break;
        }
    }
    out
}
fn packet(s: &mut TcpStream) -> Vec<u8> {
    let mut h = [0; 4];
    s.read_exact(&mut h).unwrap();
    let len = u32::from_be_bytes(h) as usize;
    assert!(len <= 256 * 1024 - 4);
    let mut result = h.to_vec();
    result.resize(len + 4, 0);
    s.read_exact(&mut result[4..]).unwrap();
    result
}
fn frame(payload: &[u8]) -> Vec<u8> {
    let mut n = 8 - ((5 + payload.len()) % 8);
    if n < 4 {
        n += 8;
    }
    encode_packet(payload, &vec![0; n]).unwrap()
}
fn offers(kex: &str, follows: u8) -> Vec<u8> {
    let p = encode_kexinit([0; 16]);
    let n = u32::from_be_bytes(p[17..21].try_into().unwrap()) as usize;
    let mut result = p[..17].to_vec();
    result.extend_from_slice(&(kex.len() as u32).to_be_bytes());
    result.extend_from_slice(kex.as_bytes());
    result.extend_from_slice(&p[21 + n..]);
    let end = result.len();
    result[end - 5] = follows;
    result
}
fn no_more_packets(s: &mut TcpStream) {
    let mut b = [0; 1];
    match s.read(&mut b) {
        Ok(0) => {}
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted
            ) => {}
        other => panic!("unexpected post-KEXINIT client bytes/result: {other:?}"),
    }
}
fn limits() -> ProbeLimits {
    ProbeLimits {
        protocol_timeout: Duration::from_millis(400),
        overall_timeout: Duration::from_secs(2),
        ..Default::default()
    }
}

#[test]
fn waiting_peer_fragmentation_and_guessed_packet_never_trigger_kex_or_authentication() {
    let (listener, target, budget) = setup(limits(), Cancellation::new());
    let peer = thread::spawn(move || {
        let mut s = accept(listener);
        assert!(line(&mut s).starts_with(b"SSH-2.0-pqc_scan_"));
        s.write_all(b"notice\r\nSSH-2.0-fixture\r\n").unwrap();
        let p = packet(&mut s);
        let payload = parse_packet(&p).unwrap();
        assert_eq!(payload[0], 20);
        let (client, follows) = parse_kexinit(payload).unwrap();
        assert!(!follows);
        assert_eq!(client, SshProbe::new().profile().offers);
        let p = frame(&offers(
            "ext-info-s,mlkem768x25519-sha256,future@vendor.test",
            2,
        ));
        for b in &p {
            s.write_all(&[*b]).unwrap();
        }
        let _ = s.write_all(&frame(&[30, 0, 0, 0, 0]));
        no_more_packets(&mut s);
    });
    let report = SshProbe::new().run(target, budget);
    peer.join().unwrap();
    report.validate().unwrap();
    assert_eq!(report.outcome, ProbeOutcome::Completed);
    assert_eq!(report.details.assurance, "unauthenticated_advertisement");
    assert_eq!(report.details.first_kex_packet_follows, Some(true));
    assert_eq!(report.details.pre_identification_lines, 1);
    assert_eq!(report.details.kex_candidates.len(), 2);
    assert_eq!(report.details.extension_markers, ["ext-info-s"]);
    assert!(report.target.connected_peer.is_some());
    let data = serde_json::to_string(&report).unwrap();
    assert!(!data.contains("negotiated"));
    assert!(!data.contains("cookie"));
}

#[test]
fn coalesced_banner_and_early_ignored_payloads_preserve_only_safe_observations() {
    let (listener, target, budget) = setup(limits(), Cancellation::new());
    let peer = thread::spawn(move || {
        let mut s = accept(listener);
        line(&mut s);
        let data = b"IGNORED_SECRET_CANARY_033";
        let mut ignored = vec![2];
        ignored.extend_from_slice(&(data.len() as u32).to_be_bytes());
        ignored.extend_from_slice(data);
        let mut all = b"SSH-2.0-fixture\r\n".to_vec();
        all.extend(frame(&ignored));
        all.extend(frame(&offers("curve25519-sha256", 0)));
        s.write_all(&all).unwrap();
        assert_eq!(parse_packet(&packet(&mut s)).unwrap()[0], 20);
        no_more_packets(&mut s);
    });
    let report = SshProbe::new().run(target, budget);
    peer.join().unwrap();
    assert_eq!(report.outcome, ProbeOutcome::Completed);
    assert_eq!(report.details.early_messages, 1);
    assert!(!serde_json::to_string(&report)
        .unwrap()
        .contains("IGNORED_SECRET_CANARY_033"));
}

#[test]
fn oversized_frame_stops_after_header_and_does_not_allocate_or_silently_complete() {
    let (listener, target, budget) = setup(limits(), Cancellation::new());
    let peer = thread::spawn(move || {
        let mut s = accept(listener);
        line(&mut s);
        s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
        packet(&mut s);
        s.write_all(&u32::MAX.to_be_bytes()).unwrap();
        no_more_packets(&mut s);
    });
    let report = SshProbe::new().run(target, budget);
    peer.join().unwrap();
    assert_eq!(report.outcome, ProbeOutcome::Partial);
    assert_eq!(report.diagnostics, [ProbeDiagnostic::LocalResourceLimit]);
    assert!(report.details.offers.is_none());
    assert_eq!(
        report.traffic.received_bytes,
        b"SSH-2.0-fixture\r\n".len() as u64 + 4
    );
}

#[test]
fn cancellation_and_deadline_interrupt_offer_wait_and_close_socket() {
    for cancelled in [false, true] {
        let cancellation = Cancellation::new();
        let peer_cancel = cancellation.clone();
        let (listener, target, budget) = setup(
            ProbeLimits {
                protocol_timeout: Duration::from_millis(50),
                ..limits()
            },
            cancellation,
        );
        let peer = thread::spawn(move || {
            let mut s = accept(listener);
            line(&mut s);
            s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
            packet(&mut s);
            if cancelled {
                peer_cancel.cancel();
            }
            no_more_packets(&mut s);
        });
        let start = Instant::now();
        let report = SshProbe::new().run(target, budget);
        peer.join().unwrap();
        assert!(start.elapsed() < Duration::from_secs(1));
        assert_eq!(report.outcome, ProbeOutcome::Partial);
        assert_eq!(
            report.diagnostics,
            [if cancelled {
                ProbeDiagnostic::Cancelled
            } else {
                ProbeDiagnostic::DeadlineExceeded
            }]
        );
    }
}

#[test]
fn disconnect_and_unexpected_auth_messages_are_terminal_with_no_client_followup() {
    for msg in [1u8, 52u8] {
        let (listener, target, budget) = setup(limits(), Cancellation::new());
        let peer = thread::spawn(move || {
            let mut s = accept(listener);
            line(&mut s);
            s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
            packet(&mut s);
            let p = if msg == 1 {
                vec![1, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0]
            } else {
                vec![msg]
            };
            s.write_all(&frame(&p)).unwrap();
            no_more_packets(&mut s);
        });
        let report = SshProbe::new().run(target, budget);
        peer.join().unwrap();
        assert_eq!(report.outcome, ProbeOutcome::Failed);
        assert_eq!(
            report.diagnostics,
            [if msg == 1 {
                ProbeDiagnostic::PeerDisconnected
            } else {
                ProbeDiagnostic::ProtocolMismatch
            }]
        );
    }
}

#[test]
fn truncated_peer_frame_is_not_complete() {
    let (listener, target, budget) = setup(limits(), Cancellation::new());
    let peer = thread::spawn(move || {
        let mut s = accept(listener);
        line(&mut s);
        s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
        packet(&mut s);
        s.write_all(&[0, 0, 0]).unwrap();
        s.shutdown(Shutdown::Write).unwrap();
        no_more_packets(&mut s);
    });
    let report = SshProbe::new().run(target, budget);
    peer.join().unwrap();
    assert_eq!(report.outcome, ProbeOutcome::Failed);
    assert_eq!(report.diagnostics, [ProbeDiagnostic::MalformedPeerData]);
}

#[test]
fn declared_frame_without_any_body_is_malformed_instead_of_a_fresh_disconnect() {
    let (listener, target, budget) = setup(limits(), Cancellation::new());
    let peer = thread::spawn(move || {
        let mut s = accept(listener);
        line(&mut s);
        s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
        packet(&mut s);
        s.write_all(&12u32.to_be_bytes()).unwrap();
        s.shutdown(Shutdown::Write).unwrap();
        no_more_packets(&mut s);
    });
    let report = SshProbe::new().run(target, budget);
    peer.join().unwrap();
    assert_eq!(report.diagnostics, [ProbeDiagnostic::MalformedPeerData]);
}

#[test]
fn banner_and_early_message_limits_accept_exactly64_and_reject_the_next() {
    for (banner, count) in [(true, 64), (true, 65), (false, 64), (false, 65)] {
        let (listener, target, budget) = setup(limits(), Cancellation::new());
        let peer = thread::spawn(move || {
            let mut s = accept(listener);
            line(&mut s);
            if banner {
                let mut bytes = b"note\r\n".repeat(count);
                bytes.extend_from_slice(b"SSH-2.0-fixture\r\n");
                s.write_all(&bytes).unwrap();
                if count <= 64 {
                    packet(&mut s);
                    s.write_all(&frame(&offers("curve25519-sha256", 0)))
                        .unwrap();
                }
            } else {
                s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
                packet(&mut s);
                let mut bytes = frame(&[2, 0, 0, 0, 0]).repeat(count);
                bytes.extend(frame(&offers("curve25519-sha256", 0)));
                s.write_all(&bytes).unwrap();
            }
            no_more_packets(&mut s);
        });
        let report = SshProbe::new().run(target, budget);
        peer.join().unwrap();
        assert_eq!(
            report.outcome,
            if count == 64 {
                ProbeOutcome::Completed
            } else {
                ProbeOutcome::Partial
            }
        );
        assert_eq!(
            if banner {
                report.details.pre_identification_lines
            } else {
                report.details.early_messages
            },
            64
        );
        if count == 65 {
            assert_eq!(report.diagnostics, [ProbeDiagnostic::LocalResourceLimit]);
        }
    }
}

#[test]
fn banner_aggregate_and_transport_budgets_are_enforced_at_actual_io() {
    for kind in [0, 1, 2] {
        let mut limits = limits();
        if kind == 1 {
            limits.max_sent_bytes = 10;
        }
        if kind == 2 {
            limits.max_received_bytes = 64;
        }
        let (listener, target, budget) = setup(limits, Cancellation::new());
        let peer = thread::spawn(move || {
            let mut s = accept(listener);
            let client = line(&mut s);
            if kind == 1 {
                assert_eq!(client.len(), 10);
                return;
            }
            if kind == 0 {
                s.write_all(&vec![b'x'; 8192]).unwrap();
            } else {
                s.write_all(b"SSH-2.0-fixture\r\n").unwrap();
                packet(&mut s);
                s.write_all(&frame(&encode_kexinit([0; 16]))).unwrap();
            }
            no_more_packets(&mut s);
        });
        let report = SshProbe::new().run(target, budget);
        peer.join().unwrap();
        assert_eq!(report.outcome, ProbeOutcome::Partial);
        assert_eq!(
            report.diagnostics,
            [if kind == 0 {
                ProbeDiagnostic::LocalResourceLimit
            } else {
                ProbeDiagnostic::TrafficLimit
            }]
        );
        if kind == 0 {
            assert_eq!(report.traffic.received_bytes, 8192);
        }
        if kind == 1 {
            assert_eq!(report.traffic.sent_bytes, 10);
        }
        if kind == 2 {
            assert_eq!(report.traffic.received_bytes, 64);
        }
    }
}
