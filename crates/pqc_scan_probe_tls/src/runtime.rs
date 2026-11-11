use crate::*;
use anyhow::{anyhow, ensure, Result};
use pqc_scan_core::{
    probe::{host_name, ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport, ProbeTarget},
    probe_transport::BoundedTcpStream,
    target_policy::{
        AuthorizedTarget, ProbeBudget, ProbeError, ProbeHost, ProbeLimits, TargetRequest,
    },
    DetectorStatus,
};
use pqc_scan_detectors::certificate_detector::parse_certificate_der;
use rustls::{
    client::ResolvesClientCert,
    pki_types::{CertificateDer, ServerName},
    sign::CertifiedKey,
    ClientConfig, ClientConnection, HandshakeKind, RootCertStore, SignatureScheme,
};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
const MAX_CERTIFICATES: usize = 16;
const MAX_CERTIFICATE_BYTES: usize = 256 * 1024;
const ROOT_DIGEST_BASIS: &str = "sorted_length_prefixed_trust_anchor_der_fields_sha256_v1";

pub struct TlsProbe {
    config: Arc<ClientConfig>,
    name: ServerName<'static>,
    initial: TlsObservation,
    client_auth_requested: Arc<AtomicBool>,
}
#[derive(Debug)]
struct NoClientAuthentication(Arc<AtomicBool>);
impl ResolvesClientCert for NoClientAuthentication {
    fn resolve(&self, _: &[&[u8]], _: &[SignatureScheme]) -> Option<Arc<CertifiedKey>> {
        self.0.store(true, Ordering::Release);
        None
    }
    fn has_certs(&self) -> bool {
        false
    }
}
impl TlsProbe {
    pub fn new(options: &TlsOptions) -> Result<Self> {
        let identity = TargetRequest::new(&options.verification_name, 1)
            .map_err(|_| anyhow!("invalid TLS verification identity"))?;
        let normalized = host_name(identity.host());
        let name = ServerName::try_from(normalized.clone())
            .map_err(|_| anyhow!("invalid TLS verification identity"))?;
        let sent_sni = matches!(identity.host(), ProbeHost::Dns(_)).then_some(normalized.clone());
        let (mut roots, trust_source) = (RootCertStore::empty(), "bundled_mozilla".to_string());
        let trust_source = if let Some(path) = &options.ca_file {
            for certificate in read_custom_ca(path)? {
                roots
                    .add(certificate)
                    .map_err(|_| anyhow!("invalid local public CA certificate"))?;
            }
            "local_ca_file".into()
        } else {
            roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            trust_source
        };
        ensure!(!roots.is_empty(), "no public trust anchors available");
        let root_set_sha256 = root_digest(&roots);
        let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
        let profile = TlsClientProfile {
            name: "default_secure_fresh".into(),
            provider: "aws_lc_rs".into(),
            provider_version: "1.18.0".into(),
            rustls_version: "0.23.46".into(),
            protocol_versions: vec![identifier(0x0304, "TLSv1_3"), identifier(0x0303, "TLSv1_2")],
            cipher_suites: provider
                .cipher_suites
                .iter()
                .map(|s| identifier(u16::from(s.suite()), &format!("{:?}", s.suite())))
                .collect(),
            key_exchange_groups: provider
                .kx_groups
                .iter()
                .map(|g| identifier(u16::from(g.name()), &format!("{:?}", g.name())))
                .collect(),
        };
        let mut config = ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
            .map_err(|_| anyhow!("TLS provider configuration unavailable"))?
            .with_root_certificates(roots)
            .with_no_client_auth();
        config.enable_sni = sent_sni.is_some();
        config.resumption = rustls::client::Resumption::disabled();
        config.enable_early_data = false;
        config.enable_secret_extraction = false;
        config.key_log = Arc::new(rustls::NoKeyLog {});
        let client_auth_requested = Arc::new(AtomicBool::new(false));
        config.client_auth_cert_resolver =
            Arc::new(NoClientAuthentication(client_auth_requested.clone()));
        let initial = TlsObservation {
            client_profile: profile,
            verification: TlsVerification {
                verification_name: normalized,
                sent_sni,
                trust_bundle_version: (trust_source == "bundled_mozilla").then(|| "1.0.11".into()),
                trust_source,
                root_set_sha256,
                root_set_digest_basis: ROOT_DIGEST_BASIS.into(),
                certificate_chain_and_name: "not_completed".into(),
                revocation: "not_checked".into(),
            },
            handshake_complete: false,
            handshake_kind: None,
            negotiated: None,
            peer_certificates: vec![],
            certificate_collection: CertificateCollection {
                status: "not_available".into(),
                presented_count: None,
                presented_bytes: None,
                collected_count: 0,
            },
        };
        Ok(Self {
            config: Arc::new(config),
            name,
            initial,
            client_auth_requested,
        })
    }
    pub fn failed(
        &self,
        target: ProbeTarget,
        limits: &ProbeLimits,
        diagnostic: ProbeDiagnostic,
    ) -> ProbeReport<TlsObservation> {
        let mut report = ProbeReport::new(ProbeProtocol::Tls, target, limits, self.initial.clone());
        report.set_outcome(ProbeOutcome::Failed, vec![diagnostic]);
        report
    }
    pub fn run(self, target: AuthorizedTarget, budget: ProbeBudget) -> ProbeReport<TlsObservation> {
        let mut report = ProbeReport::new(
            ProbeProtocol::Tls,
            ProbeTarget::authorized(&target),
            budget.limits(),
            self.initial,
        );
        let mut transport = match BoundedTcpStream::connect(target, budget) {
            Ok(t) => t,
            Err(e) => {
                report.set_outcome(ProbeOutcome::Failed, vec![e.into()]);
                return report;
            }
        };
        report.target.connected_peer = report.target.pinned_address;
        let result = ClientConnection::new(self.config, self.name)
            .map_err(|_| ProbeDiagnostic::UnsupportedLocal)
            .and_then(|mut connection| {
                handshake(&mut connection, &mut transport, &self.client_auth_requested)?;
                if !matches!(
                    connection.handshake_kind(),
                    Some(HandshakeKind::Full | HandshakeKind::FullWithHelloRetryRequest)
                ) {
                    return Err(ProbeDiagnostic::UnsupportedLocal);
                }
                let group = connection
                    .negotiated_key_exchange_group()
                    .map(|g| identifier(u16::from(g.name()), &format!("{:?}", g.name())));
                set_verified_negotiation(
                    &mut report.details,
                    connection.handshake_kind(),
                    connection.protocol_version(),
                    connection.negotiated_cipher_suite().map(|s| s.suite()),
                    group.clone(),
                )?;
                let mut diagnostics = vec![];
                if group.is_none() {
                    diagnostics.push(ProbeDiagnostic::GroupUnavailable);
                }
                if let Some(chain) = connection.peer_certificates() {
                    collect_certificates(chain, &mut report.details, &mut diagnostics);
                } else {
                    diagnostics.push(ProbeDiagnostic::CertificateParseIncomplete);
                }
                if diagnostics.is_empty() {
                    report.set_outcome(ProbeOutcome::Completed, vec![]);
                } else {
                    report.set_outcome(ProbeOutcome::Partial, diagnostics);
                }
                Ok(())
            });
        if let Err(diagnostic) = result {
            if diagnostic == ProbeDiagnostic::CertificateValidationFailed {
                report.details.verification.certificate_chain_and_name = "failed".into();
            }
            report.set_outcome(ProbeOutcome::Failed, vec![diagnostic]);
        }
        report.set_traffic(transport.budget());
        transport.close();
        report
    }
}
// Validate fallible post-handshake accessors before publishing any success facts.
fn set_verified_negotiation(
    details: &mut TlsObservation,
    kind: Option<HandshakeKind>,
    version: Option<rustls::ProtocolVersion>,
    suite: Option<rustls::CipherSuite>,
    group: Option<RegistryIdentifier>,
) -> Result<(), ProbeDiagnostic> {
    let version = version.ok_or(ProbeDiagnostic::ProtocolMismatch)?;
    let suite = suite.ok_or(ProbeDiagnostic::ProtocolMismatch)?;
    let kind = match kind {
        Some(HandshakeKind::Full) => "full",
        Some(HandshakeKind::FullWithHelloRetryRequest) => "full_with_hello_retry_request",
        _ => return Err(ProbeDiagnostic::UnsupportedLocal),
    };
    let classification = match group.as_ref().map(|g| g.id) {
        Some(4587..=4589) => "hybrid",
        Some(23 | 24 | 29) => "classical",
        _ => "unknown",
    };
    details.negotiated = Some(NegotiatedTls {
        version: identifier(u16::from(version), &format!("{version:?}")),
        cipher_suite: identifier(u16::from(suite), &format!("{suite:?}")),
        key_exchange_group: group,
        group_classification: classification.into(),
    });
    details.handshake_complete = true;
    details.handshake_kind = Some(kind.into());
    details.verification.certificate_chain_and_name = "verified".into();
    Ok(())
}

fn identifier(id: u16, name: &str) -> RegistryIdentifier {
    RegistryIdentifier {
        id,
        name: name.into(),
    }
}
fn handshake(
    c: &mut ClientConnection,
    transport: &mut BoundedTcpStream,
    client_auth_requested: &AtomicBool,
) -> Result<(), ProbeDiagnostic> {
    while c.is_handshaking() {
        while c.wants_write() {
            c.write_tls(transport).map_err(io_diagnostic)?;
        }
        if c.read_tls(transport).map_err(io_diagnostic)? == 0 {
            return Err(ProbeDiagnostic::PeerDisconnected);
        }
        c.process_new_packets().map_err(tls_diagnostic)?;
        if client_auth_requested.load(Ordering::Acquire) {
            return Err(ProbeDiagnostic::UnsupportedLocal);
        }
    }
    while c.wants_write() {
        c.write_tls(transport).map_err(io_diagnostic)?;
    }
    if c.is_early_data_accepted() {
        return Err(ProbeDiagnostic::UnsupportedLocal);
    }
    Ok(())
}
fn io_diagnostic(e: io::Error) -> ProbeDiagnostic {
    if let Some(error) = e.get_ref().and_then(|e| e.downcast_ref::<ProbeError>()) {
        return (*error).into();
    }
    if let Some(error) = e.get_ref().and_then(|e| e.downcast_ref::<rustls::Error>()) {
        return tls_diagnostic(error.clone());
    }
    ProbeDiagnostic::TransportFailed
}
#[allow(deprecated)] // Pinned rustls retains its older unsupported-signature variant.
fn tls_diagnostic(e: rustls::Error) -> ProbeDiagnostic {
    match e {
        rustls::Error::InvalidCertificate(
            rustls::CertificateError::UnsupportedSignatureAlgorithm
            | rustls::CertificateError::UnsupportedSignatureAlgorithmContext { .. },
        ) => ProbeDiagnostic::UnsupportedLocal,
        rustls::Error::InvalidCertificate(_) => ProbeDiagnostic::CertificateValidationFailed,
        rustls::Error::PeerSentOversizedRecord => ProbeDiagnostic::LocalResourceLimit,
        rustls::Error::InvalidMessage(
            rustls::InvalidMessage::HandshakePayloadTooLarge
            | rustls::InvalidMessage::CertificatePayloadTooLarge
            | rustls::InvalidMessage::MessageTooLarge,
        ) => ProbeDiagnostic::LocalResourceLimit,
        rustls::Error::InvalidMessage(_) => ProbeDiagnostic::ProtocolMismatch,
        _ => ProbeDiagnostic::TlsAlert,
    }
}
fn collect_certificates(
    chain: &[CertificateDer<'_>],
    out: &mut TlsObservation,
    diagnostics: &mut Vec<ProbeDiagnostic>,
) {
    out.certificate_collection.presented_count = Some(chain.len());
    let total = chain.iter().try_fold(0usize, |n, c| n.checked_add(c.len()));
    out.certificate_collection.presented_bytes = total;
    let mut bytes = 0usize;
    let mut limited = chain.len() > MAX_CERTIFICATES;
    for certificate in chain.iter().take(MAX_CERTIFICATES) {
        let Some(next) = bytes
            .checked_add(certificate.len())
            .filter(|n| *n <= MAX_CERTIFICATE_BYTES)
        else {
            limited = true;
            break;
        };
        bytes = next;
        let fallback = || PeerCertificate {
            certificate_der_sha256: hex::encode(Sha256::digest(certificate.as_ref())),
            spki_sha256: None,
            signature_oid: None,
            public_key_oid: None,
            key_bits: None,
            named_curve_oid: None,
            not_before_unix: None,
            not_after_unix: None,
            attribute_status: "unsupported_or_malformed".into(),
        };
        match parse_certificate_der(certificate.as_ref()) {
            Ok(f) => {
                let partial = f.status != DetectorStatus::Complete;
                if partial && !diagnostics.contains(&ProbeDiagnostic::CertificateParseIncomplete) {
                    diagnostics.push(ProbeDiagnostic::CertificateParseIncomplete);
                }
                out.peer_certificates.push(PeerCertificate {
                    certificate_der_sha256: f.certificate_sha256,
                    spki_sha256: Some(f.public_key.spki_sha256),
                    signature_oid: Some(f.signature_oid),
                    public_key_oid: Some(f.public_key.algorithm_oid),
                    key_bits: f.public_key.key_bits.map(|n| n as u64),
                    named_curve_oid: f.public_key.named_curve_oid,
                    not_before_unix: Some(f.not_before_unix),
                    not_after_unix: Some(f.not_after_unix),
                    attribute_status: if partial { "partial" } else { "complete" }.into(),
                });
            }
            Err(_) => {
                out.peer_certificates.push(fallback());
                if !diagnostics.contains(&ProbeDiagnostic::CertificateParseIncomplete) {
                    diagnostics.push(ProbeDiagnostic::CertificateParseIncomplete);
                }
            }
        }
    }
    if limited {
        diagnostics.push(ProbeDiagnostic::CertificateCollectionLimit);
    }
    out.certificate_collection.collected_count = out.peer_certificates.len();
    out.certificate_collection.status =
        if limited || diagnostics.contains(&ProbeDiagnostic::CertificateParseIncomplete) {
            "partial"
        } else {
            "complete"
        }
        .into();
}
fn read_custom_ca(path: &Path) -> Result<Vec<CertificateDer<'static>>> {
    const ERROR: &str = "invalid local public CA input (bounded certificate-only PEM required)";
    let metadata = std::fs::metadata(path).map_err(|_| anyhow!(ERROR))?;
    ensure!(
        metadata.is_file() && metadata.len() <= MAX_CERTIFICATE_BYTES as u64,
        ERROR
    );
    let file = File::open(path).map_err(|_| anyhow!(ERROR))?;
    ensure!(
        file.metadata().map_err(|_| anyhow!(ERROR))?.is_file(),
        ERROR
    );
    let mut bytes = vec![];
    file.take(MAX_CERTIFICATE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| anyhow!(ERROR))?;
    ensure!(bytes.len() <= MAX_CERTIFICATE_BYTES, ERROR);
    let mut rest = std::str::from_utf8(&bytes)
        .map_err(|_| anyhow!(ERROR))?
        .trim();
    let mut result = vec![];
    const START: &str = "-----BEGIN CERTIFICATE-----";
    const END: &str = "-----END CERTIFICATE-----";
    while !rest.is_empty() {
        ensure!(
            result.len() < MAX_CERTIFICATES && rest.starts_with(START),
            ERROR
        );
        let end = rest.find(END).ok_or_else(|| anyhow!(ERROR))? + END.len();
        let block = pem::parse(&rest[..end]).map_err(|_| anyhow!(ERROR))?;
        ensure!(block.tag() == "CERTIFICATE", ERROR);
        result.push(CertificateDer::from(block.into_contents()));
        rest = rest[end..].trim();
    }
    ensure!(!result.is_empty(), ERROR);
    Ok(result)
}
fn root_digest(roots: &RootCertStore) -> String {
    let mut anchors = vec![];
    for root in &roots.roots {
        let mut encoded = vec![];
        for part in [
            Some(root.subject.as_ref()),
            Some(root.subject_public_key_info.as_ref()),
            root.name_constraints.as_ref().map(|n| n.as_ref()),
        ] {
            if let Some(part) = part {
                encoded.extend_from_slice(&(part.len() as u64).to_be_bytes());
                encoded.extend_from_slice(part);
            } else {
                encoded.extend_from_slice(&u64::MAX.to_be_bytes());
            }
        }
        anchors.push(encoded);
    }
    anchors.sort();
    anchors.dedup();
    let mut hash = Sha256::new();
    hash.update(ROOT_DIGEST_BASIS.as_bytes());
    for anchor in anchors {
        hash.update((anchor.len() as u64).to_be_bytes());
        hash.update(anchor);
    }
    hex::encode(hash.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_post_handshake_parameters_do_not_publish_verified_details() {
        for (version, suite) in [
            (None, Some(rustls::CipherSuite::TLS13_AES_128_GCM_SHA256)),
            (Some(rustls::ProtocolVersion::TLSv1_3), None),
        ] {
            let mut details = TlsProbe::new(&TlsOptions {
                verification_name: "localhost".into(),
                ca_file: None,
            })
            .unwrap()
            .initial;
            assert_eq!(
                set_verified_negotiation(
                    &mut details,
                    Some(HandshakeKind::Full),
                    version,
                    suite,
                    None
                ),
                Err(ProbeDiagnostic::ProtocolMismatch)
            );
            assert!(!details.handshake_complete);
            assert_eq!(
                details.verification.certificate_chain_and_name,
                "not_completed"
            );
            assert!(details.negotiated.is_none());
            assert!(details.handshake_kind.is_none());
        }
    }

    #[test]
    fn unsupported_certificate_signature_is_a_local_limitation() {
        let error = rustls::Error::InvalidCertificate(
            rustls::CertificateError::UnsupportedSignatureAlgorithmContext {
                signature_algorithm_id: b"UNTRUSTED_OID_CANARY".to_vec(),
                supported_algorithms: vec![],
            },
        );
        assert_eq!(tls_diagnostic(error), ProbeDiagnostic::UnsupportedLocal);
    }
}
