//! Controlled peers use the existing fixture; no private key is retained.
use super::*;
use serde_json::{json, Value};

#[test]
fn matrix049_fresh_connections_do_not_reuse_public_key_shares_or_offer_resumption() {
    use sha2::{Digest, Sha256};
    let mut hashes = vec![];
    for _ in 0..2 {
        let tls = TlsProbe::new(&TlsOptions {
            verification_name: "localhost".into(),
            ca_file: None,
        })
        .unwrap();
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = l.local_addr().unwrap().port();
        let peer = thread::spawn(move || {
            let (mut s, _) = l.accept().unwrap();
            s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            let mut h = [0; 5];
            s.read_exact(&mut h).unwrap();
            assert_eq!(h[0], 22);
            let n = u16::from_be_bytes([h[3], h[4]]) as usize;
            assert!(n <= 16384);
            let mut hello = vec![0; n];
            s.read_exact(&mut hello).unwrap();
            assert_eq!(hello[0], 1);
            let mut i = 4 + 2 + 32;
            let sessions = hello[i] as usize;
            i += 1 + sessions;
            let ciphers = u16::from_be_bytes([hello[i], hello[i + 1]]) as usize;
            i += 2 + ciphers;
            let compression = hello[i] as usize;
            i += 1 + compression;
            let length = u16::from_be_bytes([hello[i], hello[i + 1]]) as usize;
            i += 2;
            assert_eq!(i + length, hello.len());
            let mut shares = None;
            let mut versions = None;
            while i < hello.len() {
                let kind = u16::from_be_bytes([hello[i], hello[i + 1]]);
                let len = u16::from_be_bytes([hello[i + 2], hello[i + 3]]) as usize;
                i += 4;
                assert!(i + len <= hello.len());
                assert!(!matches!(kind, 41 | 42), "PSK/early data offered");
                if kind == 51 {
                    shares = Some(format!("{:x}", Sha256::digest(&hello[i..i + len])));
                }
                if kind == 43 {
                    versions = Some(hello[i..i + len].to_vec());
                }
                i += len;
            }
            assert_eq!(versions.unwrap(), vec![4, 3, 4, 3, 3]);
            shares.unwrap()
        });
        let (t, b) = target(port, ProbeLimits::default(), Cancellation::new());
        let r = tls.run(t, b);
        assert_ne!(r.outcome, ProbeOutcome::Completed);
        assert!(r.details.negotiated.is_none());
        hashes.push(peer.join().unwrap());
    }
    assert_ne!(hashes[0], hashes[1]);
    if let Some(path) = std::env::var_os("PQC049_EVIDENCE") {
        let dir = PathBuf::from(path).join("tls");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("freshness.json"),serde_json::to_vec_pretty(&json!({"case":"freshness","publicKeyShareSha256":hashes,"offeredVersions":[772,771],"pskOffered":false,"earlyDataOffered":false,"joined":true})).unwrap()).unwrap();
    }
}

fn save(
    case: &str,
    peer: Value,
    report: &pqc_scan_core::probe::ProbeReport<pqc_scan_probe_tls::TlsObservation>,
) {
    validate_schema(report);
    if let Some(path) = std::env::var_os("PQC049_EVIDENCE") {
        let dir = PathBuf::from(path).join("tls");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join(format!("{case}.json")),
            serde_json::to_vec_pretty(&json!({"case":case,"peer":peer,"report":report})).unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn matrix049_actual_groups_verification_and_collection_are_not_inferred_from_suites() {
    use rustls::crypto::aws_lc_rs::kx_group::*;
    for (case, tls12, group, id, kind) in [
        ("tls12-x25519", true, X25519, 29, "classical"),
        ("tls13-x25519", false, X25519, 29, "classical"),
        ("tls13-p256", false, SECP256R1, 23, "classical"),
        ("tls13-p384", false, SECP384R1, 24, "classical"),
        ("tls13-hybrid-hrr", false, X25519MLKEM768, 4588, "hybrid"),
    ] {
        let f = fixture(false, 1);
        let r = probe(&f, "localhost", true, group, tls12);
        assert_eq!(r.outcome, ProbeOutcome::Completed);
        let n = r.details.negotiated.as_ref().unwrap();
        assert_eq!(n.key_exchange_group.as_ref().unwrap().id, id);
        assert_eq!(n.group_classification, kind);
        assert_eq!(n.version.id, if tls12 { 771 } else { 772 });
        assert_eq!(
            r.details.verification.certificate_chain_and_name,
            "verified"
        );
        assert_eq!(r.details.client_profile.provider, "aws_lc_rs");
        if id == 4588 {
            assert_eq!(
                r.details.handshake_kind.as_deref(),
                Some("full_with_hello_retry_request")
            );
        }
        save(
            case,
            json!({"group":id,"tlsVersion":if tls12 {771} else {772},"trust":"fixture-public-ca","expectedClassification":kind,"joined":true}),
            &r,
        );
    }
    for (case, trust, expired, future, name) in [
        ("untrusted", false, false, false, "localhost"),
        ("wrong-name", true, false, false, "wrong.example"),
        ("expired", true, true, false, "localhost"),
        ("not-yet-valid", true, false, true, "localhost"),
    ] {
        let f = fixture_time(expired, future, 1);
        let r = probe(&f, name, trust, X25519, false);
        assert_eq!(r.outcome, ProbeOutcome::Failed);
        assert!(r
            .diagnostics
            .contains(&ProbeDiagnostic::CertificateValidationFailed));
        assert!(r.details.negotiated.is_none());
        assert!(!r.details.handshake_complete);
        save(
            case,
            json!({"trusted":trust,"expired":expired,"future":future,"verificationName":name,"joined":true}),
            &r,
        );
    }
    for (case, extra, diagnostic) in [
        (
            "chain-limit",
            16,
            ProbeDiagnostic::CertificateCollectionLimit,
        ),
        ("message-limit", 200, ProbeDiagnostic::LocalResourceLimit),
    ] {
        let f = fixture(false, extra);
        let r = probe(&f, "localhost", true, X25519, false);
        assert_ne!(r.outcome, ProbeOutcome::Completed);
        assert!(r.diagnostics.contains(&diagnostic));
        assert!(r.details.peer_certificates.len() <= 16);
        save(
            case,
            json!({"presentedCertificates":extra+1,"joined":true}),
            &r,
        );
    }
    let f = fixture(false, 1);
    let r = probe(&f, "localhost", true, SECP256R1MLKEM768, false);
    assert_ne!(r.outcome, ProbeOutcome::Completed);
    assert!(r.details.negotiated.is_none());
    assert!(!r
        .details
        .client_profile
        .key_exchange_groups
        .iter()
        .any(|g| g.id == 4587));
    save(
        "profile-no-overlap",
        json!({"group":4587,"joined":true}),
        &r,
    );
}

#[test]
fn matrix049_mtls_refused_deadline_and_cancellation_keep_failure_facts() {
    let f = fixture(false, 1);
    let types = Arc::new(Mutex::new(Vec::new()));
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = l.local_addr().unwrap().port();
    let peer = server(
        l,
        &f,
        rustls::crypto::aws_lc_rs::kx_group::X25519,
        false,
        true,
        Some(types.clone()),
        Some("localhost".into()),
    );
    let (t, b) = target(port, ProbeLimits::default(), Cancellation::new());
    let r = TlsProbe::new(&TlsOptions {
        verification_name: "localhost".into(),
        ca_file: Some(f.ca.clone()),
    })
    .unwrap()
    .run(t, b);
    assert!(!peer.join().unwrap());
    assert_eq!(r.outcome, ProbeOutcome::Failed);
    assert!(r.diagnostics.contains(&ProbeDiagnostic::UnsupportedLocal));
    assert!(r.details.negotiated.is_none());
    let types = types.lock().unwrap();
    assert!(types.contains(&22));
    assert!(types.iter().all(|t| matches!(*t, 20 | 22)));
    save(
        "mtls",
        json!({"receivedRecordTypes":*types,"joined":true}),
        &r,
    );
    for case in [
        "refused",
        "timeout",
        "cancelled",
        "oversized-record",
        "truncated-record",
    ] {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = l.local_addr().unwrap().port();
        let token = Cancellation::new();
        let trigger = token.clone();
        let peer = if case == "refused" {
            drop(l);
            None
        } else {
            Some(thread::spawn(move || {
                let (mut s, _) = l.accept().unwrap();
                s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                if case == "oversized-record" || case == "truncated-record" {
                    let mut header = [0; 5];
                    s.read_exact(&mut header).unwrap();
                    let len = u16::from_be_bytes([header[3], header[4]]) as usize;
                    assert!(len <= 16384);
                    s.read_exact(&mut vec![0; len]).unwrap();
                    let bytes: &[u8] = if case == "oversized-record" {
                        &[22, 3, 3, 255, 255]
                    } else {
                        &[22, 3, 3, 0, 8, 2]
                    };
                    s.write_all(bytes).unwrap();
                    if case == "truncated-record" {
                        s.shutdown(std::net::Shutdown::Write).unwrap();
                    }
                }
                if case == "cancelled" {
                    thread::sleep(Duration::from_millis(30));
                    trigger.cancel();
                }
                let mut bytes = Vec::new();
                let _ = s.read_to_end(&mut bytes);
                bytes.len()
            }))
        };
        let limits = ProbeLimits {
            protocol_timeout: Duration::from_millis(150),
            ..Default::default()
        };
        let (t, b) = target(port, limits, token);
        let start = Instant::now();
        let r = TlsProbe::new(&TlsOptions {
            verification_name: "localhost".into(),
            ca_file: None,
        })
        .unwrap()
        .run(t, b);
        let bytes = peer.map(|p| p.join().unwrap());
        assert!(start.elapsed() < Duration::from_secs(2));
        assert_ne!(r.outcome, ProbeOutcome::Completed);
        assert!(!r.details.handshake_complete);
        assert!(r.details.negotiated.is_none());
        let expected = match case {
            "refused" => ProbeDiagnostic::TransportFailed,
            "timeout" => ProbeDiagnostic::DeadlineExceeded,
            "cancelled" => ProbeDiagnostic::Cancelled,
            "oversized-record" => ProbeDiagnostic::LocalResourceLimit,
            _ => ProbeDiagnostic::PeerDisconnected,
        };
        assert!(
            r.diagnostics.contains(&expected),
            "{case}: {:?}",
            r.diagnostics
        );
        save(
            case,
            json!({"joined":true,"bytesAfterClientHello":bytes,"elapsedMillis":start.elapsed().as_millis()}),
            &r,
        );
    }
}
