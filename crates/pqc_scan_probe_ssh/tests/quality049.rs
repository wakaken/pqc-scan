#![cfg(feature = "runtime")]
//! Independent byte-shaped peers: capture every client byte through EOF/reset.
use pqc_scan_core::target_policy::*;
use pqc_scan_probe_ssh::SshProbe;
use serde_json::{json, Value};
use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    path::PathBuf,
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
    ) -> Result<Vec<IpAddr>, ProbeError> {
        panic!("literal target must not resolve")
    }
}
fn save(case: &str, value: Value) {
    if let Some(path) = std::env::var_os("PQC049_EVIDENCE") {
        let dir = PathBuf::from(path).join("ssh");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join(format!("{case}.json")),
            serde_json::to_vec_pretty(&value).unwrap(),
        )
        .unwrap();
    }
}
fn frame(payload: &[u8]) -> Vec<u8> {
    let pad = 4 + (-(5isize + payload.len() as isize + 4)).rem_euclid(8) as usize;
    let mut out = ((1 + payload.len() + pad) as u32).to_be_bytes().to_vec();
    out.push(pad as u8);
    out.extend_from_slice(payload);
    out.resize(out.len() + pad, 0);
    out
}
fn offers(kex: &str) -> Vec<u8> {
    let mut out = vec![20];
    out.extend([0; 16]);
    for name in [
        kex,
        "ssh-ed25519",
        "aes128-ctr",
        "aes128-ctr",
        "hmac-sha2-256",
        "hmac-sha2-256",
        "none",
        "none",
        "",
        "",
    ] {
        out.extend_from_slice(&(name.len() as u32).to_be_bytes());
        out.extend_from_slice(name.as_bytes());
    }
    out.extend([0; 5]);
    frame(&out)
}
fn drain(s: &mut TcpStream, bytes: &mut Vec<u8>) {
    let mut buf = [0; 4096];
    loop {
        match s.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                bytes.extend_from_slice(&buf[..n]);
                assert!(bytes.len() <= 65536);
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted
                ) =>
            {
                break
            }
            Err(e) => panic!("peer did not observe cleanup: {e}"),
        }
    }
}
fn kinds(bytes: &[u8]) -> Vec<u8> {
    let Some(end) = bytes.iter().position(|b| *b == b'\n') else {
        assert!(bytes.len() <= 4);
        return vec![];
    };
    assert!(bytes[..=end].starts_with(b"SSH-2.0-"));
    let mut rest = &bytes[end + 1..];
    let mut out = vec![];
    while !rest.is_empty() {
        assert!(rest.len() >= 5);
        let n = u32::from_be_bytes(rest[..4].try_into().unwrap()) as usize;
        assert!((5..=262140).contains(&n));
        assert!(rest.len() >= n + 4);
        out.push(rest[5]);
        rest = &rest[n + 4..];
    }
    out
}

#[test]
fn matrix049_offer_and_failure_paths_close_without_service_auth_or_channel_packets() {
    run_mode(false);
}
#[cfg(feature = "negotiated")]
#[test]
fn matrix049_kex_failure_paths_close_without_service_auth_or_channel_packets() {
    run_mode(true);
}
fn run_mode(kex: bool) {
    for case in [
        "classical",
        "hybrid",
        "unknown",
        "fragmented",
        "oversized",
        "truncated",
        "disconnect",
        "unexpected-auth",
        "timeout",
        "cancelled",
        "receive-limit",
        "send-limit",
        "refused",
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let cancel = Cancellation::new();
        let trigger = cancel.clone();
        let peer = if case == "refused" {
            drop(listener);
            None
        } else {
            Some(thread::spawn(move || {
                let (mut s, _) = listener.accept().unwrap();
                listener.set_nonblocking(true).unwrap();
                s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                s.set_write_timeout(Some(Duration::from_secs(2))).unwrap();
                let mut received = vec![];
                let mut b = [0];
                loop {
                    match s.read(&mut b) {
                        Ok(0) => return received,
                        Ok(_) => {
                            received.push(b[0]);
                            if b[0] == b'\n' {
                                break;
                            }
                        }
                        Err(e) => panic!("banner: {e}"),
                    };
                    assert!(received.len() <= 256);
                }
                let _ = s.write_all(b"SSH-2.0-fixture049\r\n");
                let wire = match case {
                    "classical" => offers("curve25519-sha256"),
                    "hybrid" | "fragmented" => offers("mlkem768x25519-sha256"),
                    "unknown" => offers("future@fixture.invalid"),
                    "oversized" => vec![255; 4],
                    "truncated" => vec![0, 0, 0, 20, 4, 20],
                    "disconnect" => frame(&[1]),
                    "unexpected-auth" => frame(&[50]),
                    _ => vec![],
                };
                if case == "fragmented" {
                    for chunk in wire.chunks(3) {
                        let _ = s.write_all(chunk);
                    }
                } else {
                    let _ = s.write_all(&wire);
                }
                if case == "truncated" {
                    let _ = s.shutdown(std::net::Shutdown::Write);
                }
                if case == "cancelled" {
                    thread::sleep(Duration::from_millis(30));
                    trigger.cancel();
                }
                drain(&mut s, &mut received);
                assert!(
                    matches!(listener.accept(),Err(e) if e.kind()==std::io::ErrorKind::WouldBlock),
                    "unexpected retry connection"
                );
                received
            }))
        };
        let limits = ProbeLimits {
            protocol_timeout: Duration::from_millis(180),
            max_received_bytes: if case == "receive-limit" { 4 } else { 524288 },
            max_sent_bytes: if case == "send-limit" { 4 } else { 65536 },
            ..Default::default()
        };
        let budget = ProbeBudget::new(limits, Cancellation::clone(&cancel)).unwrap();
        let target = TargetPolicy::new(&["127.0.0.1/32".into()])
            .unwrap()
            .resolve(
                TargetRequest::new("127.0.0.1", port).unwrap(),
                &mut NoDns,
                &budget,
            )
            .unwrap();
        let start = Instant::now();
        let report = if kex {
            #[cfg(feature = "negotiated")]
            {
                let pin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../tests/fixtures/ssh-kex-host.pub");
                let r = pqc_scan_probe_ssh::SshKexProbe::new(&pin)
                    .unwrap()
                    .run(target, budget);
                r.validate().unwrap();
                serde_json::to_value(r).unwrap()
            }
            #[cfg(not(feature = "negotiated"))]
            {
                panic!("negotiated feature required")
            }
        } else {
            let r = SshProbe::new().run(target, budget);
            r.validate().unwrap();
            serde_json::to_value(r).unwrap()
        };
        let bytes = peer.map(|p| p.join().unwrap()).unwrap_or_default();
        let packet_types = kinds(&bytes);
        assert!(
            packet_types.iter().all(|k| if kex {
                matches!(*k, 1 | 20 | 30)
            } else {
                *k == 20
            }),
            "{case}: {packet_types:?}"
        );
        assert_eq!(report["traffic"]["sent_bytes"], bytes.len() as u64);
        assert!(start.elapsed() < Duration::from_secs(2));
        assert!(
            report["traffic"]["sent_bytes"].as_u64().unwrap()
                <= report["limits"]["max_sent_bytes"].as_u64().unwrap()
                && report["traffic"]["received_bytes"].as_u64().unwrap()
                    <= report["limits"]["max_received_bytes"].as_u64().unwrap()
        );
        let good = !kex && matches!(case, "classical" | "hybrid" | "unknown" | "fragmented");
        if good {
            assert_eq!(report["outcome"], "completed");
            assert_eq!(
                report["details"]["assurance"],
                "unauthenticated_advertisement"
            );
            assert_eq!(packet_types, [20]);
            assert!(!report.to_string().contains("negotiated"));
        } else {
            assert_ne!(report["outcome"], "completed");
        }
        if kex {
            assert_eq!(report["details"]["host_identity_verified"], false);
            assert_eq!(report["details"]["handshake_complete"], false);
            assert!(report["details"]["negotiated_kex"].is_null());
            assert_eq!(report["details"]["user_authentication_attempted"], false);
        }
        let expected = match case {
            "oversized" => Some("local_resource_limit"),
            "timeout" => Some("deadline_exceeded"),
            "cancelled" => Some("cancelled"),
            "receive-limit" | "send-limit" => Some("traffic_limit"),
            "refused" => Some("transport_failed"),
            _ => None,
        };
        if let Some(expected) = expected {
            assert!(
                report["diagnostics"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|v| v == expected),
                "{case}: {}",
                report["diagnostics"]
            );
        }
        let mode = if kex { "kex" } else { "offers" };
        save(
            &format!("{mode}-{case}"),
            json!({"case":case,"mode":mode,"report":report,"peer":{"receivedMessageTypes":packet_types,"receivedBytes":bytes.len(),"joined":true,"connections":if case=="refused"{0}else{1},"elapsedMillis":start.elapsed().as_millis()}}),
        );
    }
}
