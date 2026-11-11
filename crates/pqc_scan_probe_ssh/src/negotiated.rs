//! Verified initial hybrid KEX only. No userauth, agent, channel or global-request API.
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport, ProbeTarget},
    probe_transport::BoundedTcpStream,
    target_policy::{AuthorizedTarget, ProbeBudget, ProbeError, ProbeLimits},
};
use russh::{
    client,
    keys::{PublicKey, PublicKeyOrCertificate},
    Names,
};
use serde::Serialize;
use std::{
    borrow::Cow,
    future::Future,
    io::{self, Read},
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
const HYBRID: &str = "mlkem768x25519-sha256";
const PIN_CAP: u64 = 4096;

#[derive(Debug, Clone, Serialize)]
pub struct SshKexObservation {
    pub russh_version: &'static str,
    pub provider_version: &'static str,
    pub offered_kex: &'static str,
    pub offered_host_key: &'static str,
    pub trust_source: &'static str,
    pub host_identity_verified: bool,
    pub handshake_complete: bool,
    pub negotiated_kex: Option<String>,
    pub host_signature: Option<String>,
    pub host_authentication_pqc: bool,
    pub user_authentication_attempted: bool,
}
impl Default for SshKexObservation {
    fn default() -> Self {
        Self {
            russh_version: "0.65.1",
            provider_version: "1.18.0",
            offered_kex: HYBRID,
            offered_host_key: "ssh-ed25519",
            trust_source: "explicit_endpoint_public_key",
            host_identity_verified: false,
            handshake_complete: false,
            negotiated_kex: None,
            host_signature: None,
            host_authentication_pqc: false,
            user_authentication_attempted: false,
        }
    }
}
pub struct SshKexProbe {
    expected: PublicKey,
}
impl SshKexProbe {
    /// Bounded public-only input, parsed before resolution or any connection.
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW);
        }
        let file = options
            .open(path)
            .map_err(|_| anyhow::anyhow!("cannot read trusted public host-key file"))?;
        anyhow::ensure!(
            file.metadata()?.is_file(),
            "host-key input must be a regular public-key file"
        );
        let mut bytes = Vec::new();
        file.take(PIN_CAP + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("cannot read trusted public host-key file"))?;
        anyhow::ensure!(
            bytes.len() <= PIN_CAP as usize,
            "public host-key file exceeds 4096 bytes"
        );
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| anyhow::anyhow!("expected one OpenSSH Ed25519 public host key"))?;
        anyhow::ensure!(
            text.trim().lines().count() == 1 && text.trim().starts_with("ssh-ed25519 "),
            "expected one OpenSSH Ed25519 public host key"
        );
        let expected = PublicKey::from_openssh(text.trim())
            .map_err(|_| anyhow::anyhow!("invalid public host key"))?;
        anyhow::ensure!(
            expected.algorithm() == russh::keys::Algorithm::Ed25519,
            "only Ed25519 public host-key pins are supported"
        );
        Ok(Self { expected })
    }
    pub fn failed(
        &self,
        target: ProbeTarget,
        limits: &ProbeLimits,
        diagnostic: ProbeDiagnostic,
    ) -> ProbeReport<SshKexObservation> {
        let mut report = ProbeReport::new(
            ProbeProtocol::SshKex,
            target,
            limits,
            SshKexObservation::default(),
        );
        report.set_outcome(ProbeOutcome::Failed, vec![diagnostic]);
        report
    }
    pub fn run(
        self,
        target: AuthorizedTarget,
        budget: ProbeBudget,
    ) -> ProbeReport<SshKexObservation> {
        let mut report = self.failed(
            ProbeTarget::authorized(&target),
            budget.limits(),
            ProbeDiagnostic::InspectionNotCompleted,
        );
        // Public synchronous API must fail without networking inside an existing
        // async runtime rather than panic on a nested block_on.
        if tokio::runtime::Handle::try_current().is_ok() {
            report.set_outcome(
                ProbeOutcome::Failed,
                vec![ProbeDiagnostic::UnsupportedLocal],
            );
            return report;
        }
        // Construct the runtime before connecting, so setup errors cause no traffic.
        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
        {
            Ok(r) => r,
            Err(_) => {
                report.set_outcome(
                    ProbeOutcome::Failed,
                    vec![ProbeDiagnostic::UnsupportedLocal],
                );
                return report;
            }
        };
        let stream = match BoundedTcpStream::connect(target, budget) {
            Ok(s) => s,
            Err(e) => {
                report.set_outcome(ProbeOutcome::Failed, vec![e.into()]);
                return report;
            }
        };
        report.target.connected_peer = report.target.pinned_address;
        let shared = Arc::new(Mutex::new(Transport {
            stream,
            error: None,
        }));
        let facts = Arc::new(Mutex::new(Facts::default()));
        let remaining = shared.lock().unwrap().stream.remaining_time();
        let result = match remaining {
            Err(e) => Err(e.into()),
            Ok(remaining) => runtime.block_on(async {
                match tokio::time::timeout(
                    remaining,
                    exchange(
                        shared.clone(),
                        Handler {
                            expected: self.expected,
                            facts: facts.clone(),
                        },
                    ),
                )
                .await
                {
                    Ok(result) => result,
                    Err(_) => Err(ProbeDiagnostic::DeadlineExceeded),
                }
            }),
        };
        // Close first, then drop the runtime: background readers/writers cannot outlive
        // the invocation, retain the socket, send later requests, or race report counts.
        shared.lock().unwrap().stream.close();
        drop(runtime);
        let transport = shared.lock().unwrap();
        report.set_traffic(transport.stream.budget());
        let result = transport.error.map_or(result, |error| Err(error.into()));
        let f = facts.lock().unwrap();
        match result {
            Ok(()) if f.pin_matched && f.kex_count == 1 && f.kex.as_deref() == Some(HYBRID) => {
                report.details.host_identity_verified = true;
                report.details.handshake_complete = true;
                report.details.negotiated_kex = f.kex.clone();
                report.details.host_signature = Some("ssh-ed25519".into());
                report.set_outcome(ProbeOutcome::Completed, vec![]);
            }
            result => {
                let diagnostic = result.err().unwrap_or(ProbeDiagnostic::ProtocolMismatch);
                let partial = matches!(
                    diagnostic,
                    ProbeDiagnostic::DeadlineExceeded
                        | ProbeDiagnostic::Cancelled
                        | ProbeDiagnostic::TrafficLimit
                        | ProbeDiagnostic::LocalResourceLimit
                );
                report.set_outcome(
                    if partial {
                        ProbeOutcome::Partial
                    } else {
                        ProbeOutcome::Failed
                    },
                    vec![diagnostic],
                );
            }
        }
        report
    }
}
#[derive(Default)]
struct Facts {
    pin_matched: bool,
    kex_count: usize,
    kex: Option<String>,
}
impl Facts {
    fn record_kex(&mut self, name: &str) -> Result<(), russh::Error> {
        self.kex_count += 1;
        if self.kex_count != 1 || !self.pin_matched {
            return Err(russh::Error::Disconnect);
        }
        self.kex = Some(name.to_owned());
        Ok(())
    }
}
struct Handler {
    expected: PublicKey,
    facts: Arc<Mutex<Facts>>,
}
impl client::Handler for Handler {
    type Error = russh::Error;
    async fn check_server_key(
        &mut self,
        key: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        let matched = matches!(key, PublicKeyOrCertificate::PublicKey { key, .. } if key.key_data() == self.expected.key_data());
        self.facts.lock().unwrap().pin_matched = matched;
        Ok(matched)
    }
    async fn kex_done(
        &mut self,
        _secret: Option<&[u8]>,
        names: &Names,
        _: &mut client::Session,
    ) -> Result<(), Self::Error> {
        let mut f = self.facts.lock().unwrap();
        f.record_kex(names.kex.as_ref())
    }
}
async fn exchange(shared: Arc<Mutex<Transport>>, handler: Handler) -> Result<(), ProbeDiagnostic> {
    let mut config = client::Config {
        keepalive_interval: None,
        inactivity_timeout: Some(Duration::from_secs(5)),
        ..Default::default()
    };
    config.preferred.kex = Cow::Owned(vec![russh::kex::MLKEM768X25519_SHA256]);
    config.preferred.key = Cow::Owned(vec![russh::keys::Algorithm::Ed25519]);
    config.preferred.compression = Cow::Owned(vec![russh::compression::NONE]);
    let facts = handler.facts.clone();
    let adapter = Adapter {
        shared: shared.clone(),
        read_sleep: None,
        write_sleep: None,
    };
    let session = client::connect_stream(Arc::new(config), adapter, handler)
        .await
        .map_err(|e| match e {
            russh::Error::UnknownKey => ProbeDiagnostic::HostKeyUntrusted,
            russh::Error::WrongServerSig => ProbeDiagnostic::HostSignatureInvalid,
            russh::Error::NoCommonAlgo { .. } => ProbeDiagnostic::NoCommonAlgorithm,
            russh::Error::PacketSize(_) => ProbeDiagnostic::LocalResourceLimit,
            _ => ProbeDiagnostic::ProtocolMismatch,
        })?;
    session
        .disconnect(russh::Disconnect::ByApplication, "inspection complete", "")
        .await
        .map_err(|_| ProbeDiagnostic::TransportFailed)?;
    // Wait for the library task to stop, but do not interpret post-disconnect EOF as
    // a failed initial KEX. Outer transport deadline/cancellation still win.
    let _ = session.await;
    shared
        .lock()
        .unwrap()
        .stream
        .remaining_time()
        .map_err(ProbeDiagnostic::from)?;
    if facts.lock().unwrap().kex_count != 1 {
        return Err(ProbeDiagnostic::RekeyRejected);
    }
    Ok(())
}
struct Transport {
    stream: BoundedTcpStream,
    error: Option<ProbeError>,
}
struct Adapter {
    shared: Arc<Mutex<Transport>>,
    read_sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    write_sleep: Option<Pin<Box<tokio::time::Sleep>>>,
}
fn pending(
    timer: &mut Option<Pin<Box<tokio::time::Sleep>>>,
    cx: &mut Context<'_>,
) -> Poll<io::Result<()>> {
    let sleep = timer.get_or_insert_with(|| Box::pin(tokio::time::sleep(Duration::from_millis(5))));
    if sleep.as_mut().poll(cx).is_ready() {
        *timer = None;
        cx.waker().wake_by_ref();
    }
    Poll::Pending
}
fn io_error(transport: &mut Transport, error: ProbeError) -> io::Error {
    transport.error = Some(error);
    io::Error::other(error)
}
impl AsyncRead for Adapter {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let result = {
            let mut t = self.shared.lock().unwrap();
            match t.stream.try_read_probe(buf.initialize_unfilled()) {
                Ok(v) => Ok(v),
                Err(e) => Err(io_error(&mut t, e)),
            }
        };
        match result {
            Ok(Some(n)) => {
                self.read_sleep = None;
                buf.advance(n);
                Poll::Ready(Ok(()))
            }
            Ok(None) => pending(&mut self.read_sleep, cx),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
impl AsyncWrite for Adapter {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let result = {
            let mut t = self.shared.lock().unwrap();
            match t.stream.try_write_probe(buf) {
                Ok(v) => Ok(v),
                Err(e) => Err(io_error(&mut t, e)),
            }
        };
        match result {
            Ok(Some(n)) => {
                self.write_sleep = None;
                Poll::Ready(Ok(n))
            }
            Ok(None) => pending(&mut self.write_sleep, cx).map_ok(|_| 0),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.shared.lock().unwrap().stream.close();
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_key_exchange_or_missing_prior_trust_cannot_replace_initial_facts() {
        let mut f = Facts {
            pin_matched: true,
            ..Default::default()
        };
        f.record_kex(HYBRID).unwrap();
        assert!(f.record_kex("curve25519-sha256").is_err());
        assert_eq!(f.kex_count, 2);
        assert_eq!(f.kex.as_deref(), Some(HYBRID));
        let mut untrusted = Facts::default();
        assert!(untrusted.record_kex(HYBRID).is_err());
        assert!(untrusted.kex.is_none());
    }
}
