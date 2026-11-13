#![cfg(feature = "test-fixtures")]
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome},
    target_policy::{
        AuthorizedTarget, Cancellation, ProbeBudget, ProbeError, ProbeLimits, Resolver,
        TargetPolicy, TargetRequest,
    },
};
use pqc_scan_probe_tls::{TlsOptions, TlsProbe};
use rcgen::{BasicConstraints, CertificateParams, CertifiedIssuer, IsCa, KeyPair, KeyUsagePurpose};
use rustls::{
    crypto::SupportedKxGroup,
    pki_types::{CertificateDer, PrivatePkcs8KeyDer},
    ServerConfig, ServerConnection,
};
use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpListener},
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
struct NoDns;
impl Resolver for NoDns {
    fn resolve(
        &mut self,
        _: &str,
        _: Instant,
        _: &Cancellation,
    ) -> Result<Vec<IpAddr>, ProbeError> {
        panic!("literal must not resolve")
    }
}
static N: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    ca: PathBuf,
    chain: Vec<CertificateDer<'static>>,
    key: PrivatePkcs8KeyDer<'static>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.ca);
    }
}
fn fixture(expired: bool, extra: usize) -> Fixture {
    fixture_time(expired, false, extra)
}
fn fixture_time(expired: bool, future: bool, extra: usize) -> Fixture {
    let mut ca = CertificateParams::new(vec![]).unwrap();
    ca.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    let issuer = CertifiedIssuer::self_signed(ca, KeyPair::generate().unwrap()).unwrap();
    let mut leaf = CertificateParams::new(vec!["localhost".into(), "127.0.0.1".into()]).unwrap();
    if expired {
        leaf.not_before = rcgen::date_time_ymd(2000, 1, 1);
        leaf.not_after = rcgen::date_time_ymd(2001, 1, 1);
    }
    if future {
        leaf.not_before = rcgen::date_time_ymd(2090, 1, 1);
        leaf.not_after = rcgen::date_time_ymd(2091, 1, 1);
    }
    let key = KeyPair::generate().unwrap();
    let leaf = leaf.signed_by(&key, &issuer).unwrap();
    let mut chain = vec![leaf.der().clone()];
    chain.extend(std::iter::repeat_n(issuer.der().clone(), extra));
    let ca = std::env::temp_dir().join(format!(
        "pqc032-public-ca-{}-{}-{}.pem",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(
        &ca,
        pem::encode(&pem::Pem::new("CERTIFICATE", issuer.der().to_vec())),
    )
    .unwrap();
    Fixture {
        ca,
        chain,
        key: PrivatePkcs8KeyDer::from(key.serialize_der()),
    }
}

#[path = "support/quality049.rs"]
mod quality049;
fn server(
    listener: TcpListener,
    f: &Fixture,
    group: &'static dyn SupportedKxGroup,
    tls12: bool,
    client_auth: bool,
    record_types: Option<Arc<Mutex<Vec<u8>>>>,
    expected_name: Option<String>,
) -> thread::JoinHandle<bool> {
    let mut p = rustls::crypto::aws_lc_rs::default_provider();
    p.kx_groups = vec![group];
    let builder = ServerConfig::builder_with_provider(Arc::new(p))
        .with_protocol_versions(&[if tls12 {
            &rustls::version::TLS12
        } else {
            &rustls::version::TLS13
        }])
        .unwrap();
    let builder = if client_auth {
        let mut roots = rustls::RootCertStore::empty();
        roots
            .add(CertificateDer::from(
                pem::parse(std::fs::read(&f.ca).unwrap())
                    .unwrap()
                    .into_contents(),
            ))
            .unwrap();
        builder.with_client_cert_verifier(
            rustls::server::WebPkiClientVerifier::builder(Arc::new(roots))
                .build()
                .unwrap(),
        )
    } else {
        builder.with_no_client_auth()
    };
    let mut config = builder
        .with_single_cert(f.chain.clone(), f.key.clone_key().into())
        .unwrap();
    config.send_tls13_tickets = 0;
    config.session_storage = Arc::new(rustls::server::NoServerSessionStorage {});
    config.max_early_data_size = 0;
    listener.set_nonblocking(true).unwrap();
    thread::spawn(move || {
        let end = Instant::now() + Duration::from_secs(3);
        let socket = loop {
            match listener.accept() {
                Ok((s, _)) => break s,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < end);
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => panic!("accept failed"),
            }
        };
        socket.set_nonblocking(false).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        socket
            .set_write_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut socket = TrackedStream {
            inner: socket,
            record_types,
            header: [0; 5],
            used: 0,
            remaining: 0,
        };
        let mut c = ServerConnection::new(Arc::new(config)).unwrap();
        while c.is_handshaking() || c.wants_write() {
            if c.complete_io(&mut socket).is_err() {
                return false;
            }
        }
        assert_eq!(c.server_name(), expected_name.as_deref());
        assert!(c.peer_certificates().is_none());
        assert!(c.received_resumption_data().is_none());
        match c.reader().read(&mut [0u8; 1]) {
            Ok(0) => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            _ => panic!("application data was sent"),
        };
        true
    })
}
fn target(port: u16, limits: ProbeLimits, cancel: Cancellation) -> (AuthorizedTarget, ProbeBudget) {
    let budget = ProbeBudget::new(limits, cancel).unwrap();
    let target = TargetPolicy::new(&["127.0.0.1/32".into()])
        .unwrap()
        .resolve(
            TargetRequest::new("127.0.0.1", port).unwrap(),
            &mut NoDns,
            &budget,
        )
        .unwrap();
    (target, budget)
}
fn probe(
    f: &Fixture,
    name: &str,
    trust: bool,
    group: &'static dyn SupportedKxGroup,
    tls12: bool,
) -> pqc_scan_core::probe::ProbeReport<pqc_scan_probe_tls::TlsObservation> {
    let probe = TlsProbe::new(&TlsOptions {
        verification_name: name.into(),
        ca_file: trust.then(|| f.ca.clone()),
    })
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = server(
        listener,
        f,
        group,
        tls12,
        false,
        None,
        name.parse::<IpAddr>().is_err().then(|| name.to_owned()),
    );
    let (target, budget) = target(port, ProbeLimits::default(), Cancellation::new());
    let report = probe.run(target, budget);
    let completed = server.join().unwrap();
    if report.outcome == ProbeOutcome::Completed {
        assert!(completed);
    }
    report.validate().unwrap();
    validate_schema(&report);
    report
}
#[test]
fn verified_classical_versions_and_hybrid_hrr_use_actual_group_not_suite() {
    for (tls12, group, expected, kind) in [
        (
            true,
            rustls::crypto::aws_lc_rs::kx_group::X25519,
            29,
            "classical",
        ),
        (
            false,
            rustls::crypto::aws_lc_rs::kx_group::X25519,
            29,
            "classical",
        ),
        (
            false,
            rustls::crypto::aws_lc_rs::kx_group::X25519MLKEM768,
            4588,
            "hybrid",
        ),
    ] {
        let f = fixture(false, 1);
        let r = probe(&f, "localhost", true, group, tls12);
        assert_eq!(r.outcome, ProbeOutcome::Completed);
        let v = serde_json::to_value(&r).unwrap();
        assert_eq!(
            v["details"]["verification"]["certificate_chain_and_name"],
            "verified"
        );
        assert_eq!(v["details"]["verification"]["sent_sni"], "localhost");
        assert_eq!(
            v["details"]["negotiated"]["key_exchange_group"]["id"],
            expected
        );
        assert_eq!(v["details"]["negotiated"]["group_classification"], kind);
        assert_eq!(
            v["details"]["peer_certificates"].as_array().unwrap().len(),
            2
        );
        assert!(r.traffic.received_bytes > 0 && r.traffic.sent_bytes > 0);
        if expected == 4588 {
            assert_eq!(
                v["details"]["handshake_kind"],
                "full_with_hello_retry_request"
            );
        }
    }
}
#[test]
fn failed_verification_never_publishes_negotiated_facts() {
    for (trust, expired, name) in [
        (false, false, "localhost"),
        (true, false, "wrong.example"),
        (true, true, "localhost"),
    ] {
        let f = fixture(expired, 1);
        let r = probe(
            &f,
            name,
            trust,
            rustls::crypto::aws_lc_rs::kx_group::X25519,
            false,
        );
        assert_eq!(r.outcome, ProbeOutcome::Failed);
        assert!(r
            .diagnostics
            .contains(&ProbeDiagnostic::CertificateValidationFailed));
        let v = serde_json::to_value(r).unwrap();
        assert!(v["details"]["negotiated"].is_null());
        assert_eq!(v["details"]["handshake_complete"], false);
        assert_eq!(
            v["details"]["verification"]["certificate_chain_and_name"],
            "failed"
        );
    }
}
#[test]
fn chain_collection_and_upstream_limits_remain_incomplete() {
    for extra in [16, 200] {
        let f = fixture(false, extra);
        let r = probe(
            &f,
            "localhost",
            true,
            rustls::crypto::aws_lc_rs::kx_group::X25519,
            false,
        );
        assert_ne!(r.outcome, ProbeOutcome::Completed);
        assert!(r.diagnostics.contains(&if extra == 16 {
            ProbeDiagnostic::CertificateCollectionLimit
        } else {
            ProbeDiagnostic::LocalResourceLimit
        }));
        let v = serde_json::to_value(&r).unwrap();
        assert!(v["details"]["peer_certificates"].as_array().unwrap().len() <= 16);
    }
}
#[test]
fn configuration_rejects_private_pem_and_oversized_ca_without_exposing_input() {
    let f = fixture(false, 1);
    for bytes in [
        b"-----BEGIN PRIVATE KEY-----\nSYNTHETIC_PRIVATE_CANARY\n-----END PRIVATE KEY-----\n"
            .to_vec(),
        vec![b'a'; 256 * 1024 + 1],
    ] {
        std::fs::write(&f.ca, bytes).unwrap();
        let e = TlsProbe::new(&TlsOptions {
            verification_name: "localhost".into(),
            ca_file: Some(f.ca.clone()),
        })
        .err()
        .unwrap();
        assert!(!e.to_string().contains("SYNTHETIC_PRIVATE_CANARY"));
    }
}
#[test]
fn slow_peer_and_cancellation_close_with_typed_outcomes() {
    for cancelled in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            thread::sleep(Duration::from_millis(100));
            let mut bytes = Vec::new();
            s.read_to_end(&mut bytes).ok();
        });
        let token = Cancellation::new();
        let other = token.clone();
        let limits = ProbeLimits {
            protocol_timeout: Duration::from_millis(30),
            ..Default::default()
        };
        let (t, b) = target(port, limits, token);
        let cancel = cancelled.then(|| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(10));
                other.cancel();
            })
        });
        let r = TlsProbe::new(&TlsOptions {
            verification_name: "localhost".into(),
            ca_file: None,
        })
        .unwrap()
        .run(t, b);
        assert!(r.diagnostics.contains(&if cancelled {
            ProbeDiagnostic::Cancelled
        } else {
            ProbeDiagnostic::DeadlineExceeded
        }));
        if let Some(c) = cancel {
            c.join().unwrap();
        }
        server.join().unwrap();
    }
}

struct TrackedStream {
    inner: std::net::TcpStream,
    record_types: Option<Arc<Mutex<Vec<u8>>>>,
    header: [u8; 5],
    used: usize,
    remaining: usize,
}
impl Read for TrackedStream {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(out)?;
        if let Some(types) = &self.record_types {
            for byte in &out[..n] {
                if self.remaining > 0 {
                    self.remaining -= 1;
                    continue;
                }
                self.header[self.used] = *byte;
                self.used += 1;
                if self.used == 5 {
                    let mut types = types.lock().unwrap();
                    assert!(types.len() < 32);
                    types.push(self.header[0]);
                    self.remaining = u16::from_be_bytes([self.header[3], self.header[4]]) as usize;
                    self.used = 0;
                }
            }
        }
        Ok(n)
    }
}
impl Write for TrackedStream {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.inner.write(b)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
#[test]
fn client_auth_request_stops_before_any_encrypted_client_auth_or_finished_record() {
    let f = fixture(false, 1);
    let tls = TlsProbe::new(&TlsOptions {
        verification_name: "localhost".into(),
        ca_file: Some(f.ca.clone()),
    })
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let types = Arc::new(Mutex::new(Vec::new()));
    let peer = server(
        listener,
        &f,
        rustls::crypto::aws_lc_rs::kx_group::X25519,
        false,
        true,
        Some(types.clone()),
        Some("localhost".into()),
    );
    let (t, b) = target(port, ProbeLimits::default(), Cancellation::new());
    let report = tls.run(t, b);
    assert!(!peer.join().unwrap());
    assert_eq!(report.outcome, ProbeOutcome::Failed);
    assert!(report
        .diagnostics
        .contains(&ProbeDiagnostic::UnsupportedLocal));
    assert!(!report.details.handshake_complete);
    assert!(report.details.negotiated.is_none());
    let types = types.lock().unwrap();
    assert!(types.contains(&22));
    assert!(
        types.iter().all(|t| matches!(*t, 20 | 22)),
        "only plaintext handshake/compatibility records may be sent"
    );
}

fn validate_schema(report: &pqc_scan_core::probe::ProbeReport<pqc_scan_probe_tls::TlsObservation>) {
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../../../schemas/probe-tls-v1.schema.json")).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let value = serde_json::to_value(report).unwrap();
    let errors: Vec<_> = validator
        .iter_errors(&value)
        .map(|e| e.to_string())
        .collect();
    assert!(errors.is_empty(), "{errors:?}");
}
#[test]
fn ip_identity_sends_no_sni_and_root_provenance_is_public_and_deterministic() {
    let f = fixture(false, 1);
    let r = probe(
        &f,
        "127.0.0.1",
        true,
        rustls::crypto::aws_lc_rs::kx_group::X25519,
        false,
    );
    assert_eq!(r.outcome, ProbeOutcome::Completed);
    assert_eq!(r.details.verification.verification_name, "127.0.0.1");
    assert!(r.details.verification.sent_sni.is_none());
    assert_eq!(r.details.verification.trust_source, "local_ca_file");
    assert_eq!(r.details.verification.root_set_sha256.len(), 64);
    let options = TlsOptions {
        verification_name: "127.0.0.1".into(),
        ca_file: Some(f.ca.clone()),
    };
    let a = TlsProbe::new(&options).unwrap();
    let b = TlsProbe::new(&options).unwrap();
    let req = TargetRequest::new("127.0.0.1", 443).unwrap();
    let a = a.failed(
        pqc_scan_core::probe::ProbeTarget::requested(&req),
        &ProbeLimits::default(),
        ProbeDiagnostic::AddressDenied,
    );
    let b = b.failed(
        pqc_scan_core::probe::ProbeTarget::requested(&req),
        &ProbeLimits::default(),
        ProbeDiagnostic::AddressDenied,
    );
    assert_eq!(
        a.details.verification.root_set_sha256,
        b.details.verification.root_set_sha256
    );
    assert!(!serde_json::to_string(&a)
        .unwrap()
        .contains(f.ca.to_str().unwrap()));
    validate_schema(&a);
}
#[test]
fn unsupported_client_profile_never_becomes_remote_absence_or_verified_negotiation() {
    let f = fixture(false, 1);
    let r = probe(
        &f,
        "localhost",
        true,
        rustls::crypto::aws_lc_rs::kx_group::SECP256R1MLKEM768,
        false,
    );
    assert_ne!(r.outcome, ProbeOutcome::Completed);
    assert!(r.details.negotiated.is_none());
    assert!(!r
        .details
        .client_profile
        .key_exchange_groups
        .iter()
        .any(|g| g.id == 4587));
}

#[test]
fn custom_roots_are_bounded_certificate_only_and_have_set_digest_semantics() {
    let f = fixture(false, 1);
    let pem = std::fs::read_to_string(&f.ca).unwrap();
    let options = TlsOptions {
        verification_name: "localhost".into(),
        ca_file: Some(f.ca.clone()),
    };
    let request = TargetRequest::new("localhost", 443).unwrap();
    let read_digest = || {
        TlsProbe::new(&options)
            .unwrap()
            .failed(
                pqc_scan_core::probe::ProbeTarget::requested(&request),
                &ProbeLimits::default(),
                ProbeDiagnostic::AddressDenied,
            )
            .details
            .verification
            .root_set_sha256
    };
    let original = read_digest();
    std::fs::write(&f.ca, pem.repeat(16)).unwrap();
    assert_eq!(read_digest(), original);
    for input in [
        pem.repeat(17),
        format!("{pem}UNSUPPORTED_TRAILING_CA_CANARY"),
        String::new(),
    ] {
        std::fs::write(&f.ca, input).unwrap();
        let error = TlsProbe::new(&options).err().unwrap();
        assert!(!error.to_string().contains("CANARY"));
    }
    for name in [
        "https://localhost/path",
        "localhost\ninvalid",
        "user@localhost",
    ] {
        assert!(TlsProbe::new(&TlsOptions {
            verification_name: name.into(),
            ca_file: None
        })
        .is_err());
    }
}
#[test]
fn schema_rejects_a_completed_label_on_unverified_details() {
    let request = TargetRequest::new("localhost", 443).unwrap();
    let probe = TlsProbe::new(&TlsOptions {
        verification_name: "localhost".into(),
        ca_file: None,
    })
    .unwrap();
    let report = probe.failed(
        pqc_scan_core::probe::ProbeTarget::requested(&request),
        &ProbeLimits::default(),
        ProbeDiagnostic::AddressDenied,
    );
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../../../schemas/probe-tls-v1.schema.json")).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let mut value = serde_json::to_value(&report).unwrap();
    assert!(validator.is_valid(&value));
    value["outcome"] = "completed".into();
    value["diagnostics"] = serde_json::json!([]);
    value["target"]["connected_peer"] = "127.0.0.1:443".into();
    assert!(!validator.is_valid(&value));
}

#[test]
fn oversized_tls_record_is_a_local_resource_limit_not_an_algorithm_verdict() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let peer = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut header = [0; 5];
        socket.read_exact(&mut header).unwrap();
        let size = u16::from_be_bytes([header[3], header[4]]) as usize;
        assert!(size <= 16384);
        socket.read_exact(&mut vec![0; size]).unwrap();
        socket.write_all(&[22, 3, 3, 255, 255]).unwrap();
        while let Ok(n) = socket.read(&mut [0; 1024]) {
            if n == 0 {
                break;
            }
        }
    });
    let (target, budget) = target(port, ProbeLimits::default(), Cancellation::new());
    let report = TlsProbe::new(&TlsOptions {
        verification_name: "localhost".into(),
        ca_file: None,
    })
    .unwrap()
    .run(target, budget);
    peer.join().unwrap();
    assert!(
        report
            .diagnostics
            .contains(&ProbeDiagnostic::LocalResourceLimit),
        "safe diagnostics: {:?}",
        report.diagnostics
    );
    assert!(report.details.negotiated.is_none());
    validate_schema(&report);
}
