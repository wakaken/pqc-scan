//! Versioned endpoint observations, independent of static inventory v1.
use crate::target_policy::{
    AuthorizedTarget, ProbeBudget, ProbeError, ProbeHost, ProbeLimits, TargetRequest,
};
use serde::Serialize;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeProtocol {
    Tls,
    SshOffers,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeOutcome {
    Completed,
    Partial,
    Failed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeDiagnostic {
    InspectionNotCompleted,
    InvalidTarget,
    AddressDenied,
    DnsFailed,
    DeadlineExceeded,
    Cancelled,
    TrafficLimit,
    AttemptLimit,
    PeerMismatch,
    TransportFailed,
    UnsupportedLocal,
    ProtocolMismatch,
    MalformedPeerData,
    PeerDisconnected,
    CertificateValidationFailed,
    TlsAlert,
    LocalResourceLimit,
    CertificateCollectionLimit,
    CertificateParseIncomplete,
    GroupUnavailable,
}
impl From<ProbeError> for ProbeDiagnostic {
    fn from(value: ProbeError) -> Self {
        match value {
            ProbeError::InvalidTarget | ProbeError::InvalidCidr => Self::InvalidTarget,
            ProbeError::AddressDenied => Self::AddressDenied,
            ProbeError::TooManyAnswers => Self::LocalResourceLimit,
            ProbeError::DnsFailed => Self::DnsFailed,
            ProbeError::DeadlineExceeded => Self::DeadlineExceeded,
            ProbeError::Cancelled => Self::Cancelled,
            ProbeError::TrafficLimit => Self::TrafficLimit,
            ProbeError::AttemptLimit => Self::AttemptLimit,
            ProbeError::PeerMismatch => Self::PeerMismatch,
            ProbeError::Io => Self::TransportFailed,
            ProbeError::Unsupported => Self::UnsupportedLocal,
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ProbeTarget {
    pub host: String,
    pub port: u16,
    pub resolved_addresses: Option<Vec<IpAddr>>,
    pub pinned_address: Option<SocketAddr>,
    pub connected_peer: Option<SocketAddr>,
}
impl ProbeTarget {
    pub fn requested(request: &TargetRequest) -> Self {
        Self {
            host: host_name(request.host()),
            port: request.port(),
            resolved_addresses: None,
            pinned_address: None,
            connected_peer: None,
        }
    }
    pub fn authorized(target: &AuthorizedTarget) -> Self {
        Self {
            host: host_name(target.host()),
            port: target.socket_addr().port(),
            resolved_addresses: Some(target.answers().to_vec()),
            pinned_address: Some(target.socket_addr()),
            connected_peer: None,
        }
    }
}
pub fn host_name(host: &ProbeHost) -> String {
    match host {
        ProbeHost::Ip(ip) => ip.to_string(),
        ProbeHost::Dns(name) => name.clone(),
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ProbeLimitSnapshot {
    pub max_dns_answers: usize,
    pub dns_timeout_ns: u64,
    pub tcp_timeout_ns: u64,
    pub protocol_timeout_ns: u64,
    pub overall_timeout_ns: u64,
    pub max_received_bytes: u64,
    pub max_sent_bytes: u64,
}
impl From<&ProbeLimits> for ProbeLimitSnapshot {
    fn from(v: &ProbeLimits) -> Self {
        Self {
            max_dns_answers: v.max_dns_answers,
            dns_timeout_ns: v.dns_timeout.as_nanos().min(u64::MAX as u128) as u64,
            tcp_timeout_ns: v.tcp_timeout.as_nanos().min(u64::MAX as u128) as u64,
            protocol_timeout_ns: v.protocol_timeout.as_nanos().min(u64::MAX as u128) as u64,
            overall_timeout_ns: v.overall_timeout.as_nanos().min(u64::MAX as u128) as u64,
            max_received_bytes: v.max_received_bytes,
            max_sent_bytes: v.max_sent_bytes,
        }
    }
}
#[derive(Debug, Clone, Default, Serialize)]
pub struct ProbeTraffic {
    pub received_bytes: u64,
    pub sent_bytes: u64,
}
#[derive(Debug, Clone, Serialize)]
pub struct ProbeReport<T> {
    pub schema_version: u32,
    pub scanner_version: String,
    pub generated_at: String,
    pub redaction_applied: bool,
    pub protocol: ProbeProtocol,
    pub attempt_id: u32,
    pub target: ProbeTarget,
    pub limits: ProbeLimitSnapshot,
    pub traffic: ProbeTraffic,
    pub outcome: ProbeOutcome,
    pub diagnostics: Vec<ProbeDiagnostic>,
    pub details: T,
}
impl<T> ProbeReport<T> {
    pub fn new(
        protocol: ProbeProtocol,
        target: ProbeTarget,
        limits: &ProbeLimits,
        details: T,
    ) -> Self {
        Self {
            schema_version: 1,
            scanner_version: env!("CARGO_PKG_VERSION").into(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            redaction_applied: false,
            protocol,
            attempt_id: 1,
            target,
            limits: limits.into(),
            traffic: Default::default(),
            outcome: ProbeOutcome::Failed,
            diagnostics: vec![ProbeDiagnostic::InspectionNotCompleted],
            details,
        }
    }
    pub fn set_outcome(&mut self, outcome: ProbeOutcome, diagnostics: Vec<ProbeDiagnostic>) {
        self.outcome = outcome;
        self.diagnostics = diagnostics;
    }
    pub fn set_traffic(&mut self, budget: &ProbeBudget) {
        self.traffic.received_bytes = budget.received_bytes();
        self.traffic.sent_bytes = budget.sent_bytes();
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.schema_version == 1 && self.attempt_id == 1,
            "unsupported probe envelope"
        );
        TargetRequest::new(&self.target.host, self.target.port)?;
        let caps = ProbeLimits::default();
        anyhow::ensure!(
            self.limits.max_dns_answers > 0 && self.limits.max_dns_answers <= caps.max_dns_answers,
            "invalid probe answer cap"
        );
        for (actual, cap) in [
            (
                self.limits.dns_timeout_ns,
                caps.dns_timeout.as_nanos() as u64,
            ),
            (
                self.limits.tcp_timeout_ns,
                caps.tcp_timeout.as_nanos() as u64,
            ),
            (
                self.limits.protocol_timeout_ns,
                caps.protocol_timeout.as_nanos() as u64,
            ),
            (
                self.limits.overall_timeout_ns,
                caps.overall_timeout.as_nanos() as u64,
            ),
            (self.limits.max_received_bytes, caps.max_received_bytes),
            (self.limits.max_sent_bytes, caps.max_sent_bytes),
        ] {
            anyhow::ensure!(actual > 0 && actual <= cap, "invalid probe hard cap");
        }
        if let Some(answers) = &self.target.resolved_addresses {
            anyhow::ensure!(
                !answers.is_empty() && answers.len() <= self.limits.max_dns_answers,
                "invalid probe answer set"
            );
        }
        anyhow::ensure!(
            self.traffic.received_bytes <= self.limits.max_received_bytes
                && self.traffic.sent_bytes <= self.limits.max_sent_bytes,
            "probe traffic exceeds limits"
        );
        anyhow::ensure!(
            (self.outcome == ProbeOutcome::Completed) == self.diagnostics.is_empty(),
            "probe outcome and diagnostics disagree"
        );
        anyhow::ensure!(
            self.outcome != ProbeOutcome::Completed || self.target.connected_peer.is_some(),
            "completed probe has no connected peer"
        );
        anyhow::ensure!(
            self.target.connected_peer.is_some()
                || (self.traffic.received_bytes == 0 && self.traffic.sent_bytes == 0),
            "unbound probe traffic"
        );
        if let Some(pinned) = self.target.pinned_address {
            anyhow::ensure!(
                pinned.port() == self.target.port
                    && self
                        .target
                        .resolved_addresses
                        .as_ref()
                        .is_some_and(|answers| answers.contains(&pinned.ip())),
                "unbound probe endpoint"
            );
        }
        if let Some(peer) = self.target.connected_peer {
            anyhow::ensure!(
                Some(peer) == self.target.pinned_address,
                "probe peer mismatch"
            );
        }
        Ok(())
    }
}
