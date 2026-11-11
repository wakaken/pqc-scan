//! Development-only pinned TLS feasibility harness. No product entrypoint.
//! Private fixture keys exist in memory only; logs contain sanitized session facts.
#[cfg(test)]
mod tests {
    use pqc_scan_core::{
        probe_transport::BoundedTcpStream,
        target_policy::{
            Cancellation, ProbeBudget, ProbeError, ProbeLimits, Resolver, TargetPolicy,
            TargetRequest,
        },
    };
    use rcgen::{
        BasicConstraints, CertificateParams, CertifiedIssuer, IsCa, KeyPair, KeyUsagePurpose,
    };
    use rustls::{
        crypto::{CryptoProvider, SupportedKxGroup},
        pki_types::{CertificateDer, PrivatePkcs8KeyDer, ServerName},
        ClientConfig, ClientConnection, HandshakeKind, NamedGroup, ProtocolVersion, RootCertStore,
        ServerConfig, ServerConnection, SupportedProtocolVersion,
    };
    use serde_json::json;
    use std::{
        io::{self, Read},
        net::{IpAddr, TcpListener},
        sync::Arc,
        thread,
        time::{Duration, Instant},
    };

    struct NoResolver;
    impl Resolver for NoResolver {
        fn resolve(
            &mut self,
            _: &str,
            _: Instant,
            _: &Cancellation,
        ) -> Result<Vec<IpAddr>, ProbeError> {
            panic!("literal loopback must not resolve")
        }
    }
    struct Fixture {
        root: CertificateDer<'static>,
        chain: Vec<CertificateDer<'static>>,
        key: PrivatePkcs8KeyDer<'static>,
    }
    fn fixture(expired: bool, extra_chain: usize) -> Fixture {
        let mut ca = CertificateParams::new(vec![]).unwrap();
        ca.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        ca.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
        let issuer = CertifiedIssuer::self_signed(ca, KeyPair::generate().unwrap()).unwrap();
        let mut leaf = CertificateParams::new(vec!["localhost".into()]).unwrap();
        if expired {
            leaf.not_before = rcgen::date_time_ymd(2000, 1, 1);
            leaf.not_after = rcgen::date_time_ymd(2001, 1, 1);
        }
        let key = KeyPair::generate().unwrap();
        let cert = leaf.signed_by(&key, &issuer).unwrap();
        let root = issuer.der().clone();
        let mut chain = vec![cert.der().clone()];
        chain.extend(std::iter::repeat_n(root.clone(), extra_chain));
        Fixture {
            root,
            chain,
            key: PrivatePkcs8KeyDer::from(key.serialize_der()),
        }
    }
    fn provider(groups: Vec<&'static dyn SupportedKxGroup>) -> Arc<CryptoProvider> {
        let mut p = rustls::crypto::aws_lc_rs::default_provider();
        p.kx_groups = groups;
        Arc::new(p)
    }
    fn client_config(
        p: Arc<CryptoProvider>,
        versions: &[&'static SupportedProtocolVersion],
        root: Option<CertificateDer<'static>>,
    ) -> Arc<ClientConfig> {
        let mut roots = RootCertStore::empty();
        if let Some(root) = root {
            roots.add(root).unwrap();
        }
        let mut config = ClientConfig::builder_with_provider(p)
            .with_protocol_versions(versions)
            .unwrap()
            .with_root_certificates(roots)
            .with_no_client_auth();
        config.resumption = rustls::client::Resumption::disabled();
        config.enable_early_data = false;
        config.enable_secret_extraction = false;
        config.key_log = Arc::new(rustls::NoKeyLog {});
        assert!(!config.client_auth_cert_resolver.has_certs());
        assert!(!config.key_log.will_log("CLIENT_RANDOM"));
        assert!(!config.key_log.will_log("CLIENT_HANDSHAKE_TRAFFIC_SECRET"));
        assert!(!config.key_log.will_log("SERVER_TRAFFIC_SECRET_0"));
        Arc::new(config)
    }
    fn server_config(
        p: Arc<CryptoProvider>,
        versions: &[&'static SupportedProtocolVersion],
        f: Fixture,
    ) -> Arc<ServerConfig> {
        let mut c = ServerConfig::builder_with_provider(p)
            .with_protocol_versions(versions)
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(f.chain, f.key.into())
            .unwrap();
        c.send_tls13_tickets = 0;
        c.session_storage = Arc::new(rustls::server::NoServerSessionStorage {});
        c.max_early_data_size = 0;
        c.enable_secret_extraction = false;
        c.key_log = Arc::new(rustls::NoKeyLog {});
        Arc::new(c)
    }
    #[derive(Debug)]
    struct SessionFacts {
        version: ProtocolVersion,
        group: NamedGroup,
        suite: u16,
        kind: HandshakeKind,
        certificate_count: usize,
        certificate_bytes: usize,
        received: u64,
        sent: u64,
    }
    fn no_plaintext(reader: &mut dyn Read) -> usize {
        match reader.read(&mut [0; 1]) {
            Ok(0) => 0,
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => 0,
            _ => panic!("unexpected application plaintext"),
        }
    }
    fn classify(error: &rustls::Error) -> &'static str {
        use rustls::{CertificateError, Error};
        match error {
            Error::InvalidCertificate(CertificateError::UnknownIssuer) => "unknown_issuer",
            Error::InvalidCertificate(
                CertificateError::NotValidForName | CertificateError::NotValidForNameContext { .. },
            ) => "name_mismatch",
            Error::InvalidCertificate(
                CertificateError::Expired | CertificateError::ExpiredContext { .. },
            ) => "expired",
            Error::InvalidMessage(
                rustls::InvalidMessage::HandshakePayloadTooLarge
                | rustls::InvalidMessage::CertificatePayloadTooLarge,
            ) => "local_tls_message_limit",
            _ => "other_tls_failure",
        }
    }
    fn collect_chain(
        chain: &[CertificateDer<'_>],
        max_count: usize,
        max_bytes: usize,
    ) -> Result<(usize, usize), &'static str> {
        if chain.len() > max_count {
            return Err("certificate_count_limit");
        }
        let bytes = chain.iter().try_fold(0usize, |n, c| {
            n.checked_add(c.len())
                .filter(|n| *n <= max_bytes)
                .ok_or("certificate_bytes_limit")
        })?;
        Ok((chain.len(), bytes))
    }
    fn session(
        client_config: Arc<ClientConfig>,
        server_config: Arc<ServerConfig>,
        name: &str,
    ) -> Result<SessionFacts, &'static str> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut socket = loop {
                match listener.accept() {
                    Ok((socket, _)) => break socket,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline, "local accept deadline");
                        thread::sleep(Duration::from_millis(5))
                    }
                    Err(_) => panic!("local accept failure"),
                }
            };
            socket.set_nonblocking(false).unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            socket
                .set_write_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut c = ServerConnection::new(server_config).unwrap();
            while c.is_handshaking() {
                if c.complete_io(&mut socket).is_err() {
                    return false;
                }
            }
            while c.wants_write() {
                if c.complete_io(&mut socket).is_err() {
                    return false;
                }
            }
            assert!(c.peer_certificates().is_none());
            assert!(c.received_resumption_data().is_none());
            assert_eq!(no_plaintext(&mut c.reader()), 0);
            true
        });
        let budget = ProbeBudget::new(ProbeLimits::default(), Cancellation::new()).unwrap();
        let target = TargetPolicy::new(&["127.0.0.1/32".into()])
            .unwrap()
            .resolve(
                TargetRequest::new("127.0.0.1", port).unwrap(),
                &mut NoResolver,
                &budget,
            )
            .unwrap();
        let mut transport = BoundedTcpStream::connect(target, budget).unwrap();
        let mut c = ClientConnection::new(
            client_config,
            ServerName::try_from(name.to_owned()).unwrap(),
        )
        .unwrap();
        assert!(c.protocol_version().is_none());
        assert!(c.negotiated_cipher_suite().is_none());
        assert!(c.negotiated_key_exchange_group().is_none());
        assert!(c.peer_certificates().is_none());
        assert!(c.early_data().is_none());
        let result = (|| {
            while c.is_handshaking() {
                while c.wants_write() {
                    c.write_tls(&mut transport).map_err(|_| "transport_write")?;
                }
                if c.read_tls(&mut transport).map_err(|_| "transport_read")? == 0 {
                    return Err("peer_eof");
                }
                c.process_new_packets().map_err(|e| classify(&e))?;
            }
            while c.wants_write() {
                c.write_tls(&mut transport).map_err(|_| "transport_write")?;
            }
            assert!(!c.is_early_data_accepted());
            assert!(c.early_data().is_none());
            assert_eq!(no_plaintext(&mut c.reader()), 0);
            let (certificate_count, certificate_bytes) =
                collect_chain(c.peer_certificates().ok_or("no_chain")?, 16, 256 * 1024)?;
            Ok(SessionFacts {
                version: c.protocol_version().ok_or("no_version")?,
                group: c
                    .negotiated_key_exchange_group()
                    .ok_or("unknown_group")?
                    .name(),
                suite: u16::from(c.negotiated_cipher_suite().ok_or("no_suite")?.suite()),
                kind: c.handshake_kind().ok_or("unknown_handshake")?,
                certificate_count,
                certificate_bytes,
                received: transport.budget().received_bytes(),
                sent: transport.budget().sent_bytes(),
            })
        })();
        if matches!(result, Err("unknown_issuer" | "name_mismatch" | "expired")) {
            assert!(c.is_handshaking());
            println!(
                "{}",
                json!({"scenario":"failed_handshake_accessor_state", "verified":false, "handshake_complete":false, "unverified_accessor_group_id": c.negotiated_key_exchange_group().map(|group|u16::from(group.name())), "presented_chain_available":c.peer_certificates().is_some()})
            );
        }
        transport.close();
        let server_completed = server.join().unwrap();
        if result.is_ok() {
            assert!(server_completed, "server must complete the full handshake");
        }
        result
    }
    fn success(
        version: &'static SupportedProtocolVersion,
        group: &'static dyn SupportedKxGroup,
        hrr: bool,
    ) {
        let fixture = fixture(false, 1);
        let root = fixture.root.clone();
        let client_groups = if hrr {
            vec![rustls::crypto::aws_lc_rs::kx_group::X25519, group]
        } else {
            vec![group]
        };
        let client = client_config(provider(client_groups), &[version], Some(root));
        let server = server_config(provider(vec![group]), &[version], fixture);
        let facts = session(client, server, "localhost").unwrap();
        assert_eq!(facts.version, version.version);
        assert_eq!(facts.group, group.name());
        assert_ne!(facts.kind, HandshakeKind::Resumed);
        assert_eq!(
            facts.kind,
            if hrr {
                HandshakeKind::FullWithHelloRetryRequest
            } else {
                HandshakeKind::Full
            }
        );
        assert_eq!(facts.certificate_count, 2);
        assert!(facts.certificate_bytes < 256 * 1024);
        println!(
            "{}",
            json!({"scenario":"verified_fresh","version":format!("{:?}",facts.version),"version_id":u16::from(facts.version),"suite_id":facts.suite,"group":format!("{:?}",facts.group),"group_id":u16::from(facts.group),"handshake":format!("{:?}",facts.kind),"certificate_count":facts.certificate_count,"certificate_bytes":facts.certificate_bytes,"received":facts.received,"sent":facts.sent,"application_plaintext_bytes":0,"client_certificate":false})
        );
    }
    #[cfg(feature = "tls12")]
    #[test]
    fn classical_tls12() {
        success(
            &rustls::version::TLS12,
            rustls::crypto::aws_lc_rs::kx_group::X25519,
            false,
        );
    }
    #[test]
    fn classical_tls13() {
        success(
            &rustls::version::TLS13,
            rustls::crypto::aws_lc_rs::kx_group::X25519,
            false,
        );
    }
    #[test]
    fn hybrid_x25519_mlkem768() {
        success(
            &rustls::version::TLS13,
            rustls::crypto::aws_lc_rs::kx_group::X25519MLKEM768,
            false,
        );
    }
    #[test]
    fn hybrid_p256_mlkem768() {
        success(
            &rustls::version::TLS13,
            rustls::crypto::aws_lc_rs::kx_group::SECP256R1MLKEM768,
            false,
        );
    }
    #[test]
    fn hybrid_hello_retry_request() {
        success(
            &rustls::version::TLS13,
            rustls::crypto::aws_lc_rs::kx_group::X25519MLKEM768,
            true,
        );
    }
    #[test]
    fn ordinary_verifier_rejects_untrusted_name_and_expiry() {
        for (trust, expired, name, expected) in [
            (false, false, "localhost", "unknown_issuer"),
            (true, false, "wrong.example", "name_mismatch"),
            (true, true, "localhost", "expired"),
        ] {
            let fixture = fixture(expired, 1);
            let root = trust.then(|| fixture.root.clone());
            let group = rustls::crypto::aws_lc_rs::kx_group::X25519;
            let client = client_config(provider(vec![group]), &[&rustls::version::TLS13], root);
            let server = server_config(provider(vec![group]), &[&rustls::version::TLS13], fixture);
            assert_eq!(session(client, server, name).unwrap_err(), expected);
            println!(
                "{}",
                json!({"scenario":"verifier_rejection","outcome":expected,"verified":false})
            );
        }
    }
    #[test]
    fn bounded_certificate_access_reports_limits_without_copying() {
        let chain = vec![CertificateDer::from(vec![0; 4]); 2];
        assert_eq!(collect_chain(&chain, 2, 8), Ok((2, 8)));
        assert_eq!(collect_chain(&chain, 1, 8), Err("certificate_count_limit"));
        assert_eq!(collect_chain(&chain, 2, 7), Err("certificate_bytes_limit"));
    }
    #[test]
    fn real_chain_collection_limit_and_upstream_message_limit_are_distinct() {
        for (extra, expected) in [
            (16, "certificate_count_limit"),
            (200, "local_tls_message_limit"),
        ] {
            let f = fixture(false, extra);
            let root = f.root.clone();
            let group = rustls::crypto::aws_lc_rs::kx_group::X25519;
            let client = client_config(
                provider(vec![group]),
                &[&rustls::version::TLS13],
                Some(root),
            );
            let server = server_config(provider(vec![group]), &[&rustls::version::TLS13], f);
            assert_eq!(session(client, server, "localhost").unwrap_err(), expected);
            println!(
                "{}",
                json!({"scenario":"certificate_limit","chain_elements_sent":extra+1,"outcome":expected,"complete_inspection":false})
            );
        }
    }
    #[test]
    fn reused_client_configuration_still_performs_fresh_handshake() {
        let f = fixture(false, 1);
        let root = f.root.clone();
        let group = rustls::crypto::aws_lc_rs::kx_group::X25519;
        let client = client_config(
            provider(vec![group]),
            &[&rustls::version::TLS13],
            Some(root),
        );
        let server = server_config(provider(vec![group]), &[&rustls::version::TLS13], f);
        for _ in 0..2 {
            assert_eq!(
                session(client.clone(), server.clone(), "localhost")
                    .unwrap()
                    .kind,
                HandshakeKind::Full
            );
        }
    }
    #[test]
    fn provider_capabilities_are_explicit_not_inferred_from_suite() {
        let names: Vec<_> = rustls::crypto::aws_lc_rs::ALL_KX_GROUPS
            .iter()
            .map(|g| g.name())
            .collect();
        assert!(names.contains(&NamedGroup::X25519));
        assert!(names.contains(&NamedGroup::X25519MLKEM768));
        assert!(names.contains(&NamedGroup::secp256r1MLKEM768));
        assert!(!names.iter().any(|g| u16::from(*g) == 4589));
        println!(
            "{}",
            json!({"scenario":"provider_capabilities","group_ids":names.iter().map(|g|u16::from(*g)).collect::<Vec<_>>(),"p384_mlkem1024":"not_in_provider_group_list","certificate_verify_scheme":"no_general_accessor_tested"})
        );
    }
}
