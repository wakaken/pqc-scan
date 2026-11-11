//! Explicit authorization and finite budgets for optional endpoint probes.
//! No resolver, proxy, configuration file or protocol is invoked automatically.
use std::{
    cell::Cell,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ProbeError {
    #[error("invalid explicit probe target")]
    InvalidTarget,
    #[error("invalid explicit CIDR")]
    InvalidCidr,
    #[error("address is not authorized")]
    AddressDenied,
    #[error("DNS answer limit exceeded")]
    TooManyAnswers,
    #[error("DNS resolution failed")]
    DnsFailed,
    #[error("probe deadline exceeded")]
    DeadlineExceeded,
    #[error("probe cancelled")]
    Cancelled,
    #[error("probe attempt limit exceeded")]
    AttemptLimit,
    #[error("probe traffic limit reached")]
    TrafficLimit,
    #[error("connected peer differs from pinned target")]
    PeerMismatch,
    #[error("probe transport failed")]
    Io,
    #[error("probe capability unsupported")]
    Unsupported,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeHost {
    Ip(IpAddr),
    Dns(String),
}
#[derive(Debug)]
pub struct TargetRequest {
    host: ProbeHost,
    port: u16,
}
impl TargetRequest {
    pub fn new(host: &str, port: u16) -> Result<Self, ProbeError> {
        if port == 0
            || host.is_empty()
            || host.len() > 1024
            || host.chars().any(|c| {
                c.is_control()
                    || c.is_whitespace()
                    || matches!(c, '/' | '\\' | '@' | '%' | '*' | '?' | '#')
            })
        {
            return Err(ProbeError::InvalidTarget);
        }
        let unbracketed = if host.starts_with('[') && host.ends_with(']') {
            &host[1..host.len() - 1]
        } else {
            host
        };
        let host = if let Ok(ip) = unbracketed.parse::<IpAddr>() {
            if host.starts_with('[') && !matches!(ip, IpAddr::V6(_)) {
                return Err(ProbeError::InvalidTarget);
            }
            ProbeHost::Ip(normalize(ip))
        } else {
            if host.contains(['[', ']', ':']) {
                return Err(ProbeError::InvalidTarget);
            }
            let ascii = idna::domain_to_ascii_strict(host)
                .map_err(|_| ProbeError::InvalidTarget)?
                .to_ascii_lowercase();
            let ascii = ascii.strip_suffix('.').unwrap_or(&ascii);
            if ascii.is_empty()
                || ascii.len() > 253
                || ascii.split('.').any(|s| s.is_empty() || s.len() > 63)
                || ascii
                    .rsplit('.')
                    .next()
                    .is_some_and(|s| s.bytes().all(|b| b.is_ascii_digit()))
                || (!ascii.contains('.') && ascii.starts_with("0x"))
            {
                return Err(ProbeError::InvalidTarget);
            }
            ProbeHost::Dns(ascii.to_string())
        };
        Ok(Self { host, port })
    }
    pub fn host(&self) -> &ProbeHost {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
}
fn normalize(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map(IpAddr::V4).unwrap_or(ip),
        _ => ip,
    }
}
#[derive(Debug)]
struct Cidr {
    network: IpAddr,
    prefix: u8,
}
impl FromStr for Cidr {
    type Err = ProbeError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() > 64 {
            return Err(ProbeError::InvalidCidr);
        }
        let (ip, prefix) = input.split_once('/').ok_or(ProbeError::InvalidCidr)?;
        if prefix.is_empty() || !prefix.bytes().all(|b| b.is_ascii_digit()) {
            return Err(ProbeError::InvalidCidr);
        }
        let mut network = ip.parse::<IpAddr>().map_err(|_| ProbeError::InvalidCidr)?;
        let mut prefix = prefix.parse::<u8>().map_err(|_| ProbeError::InvalidCidr)?;
        if let IpAddr::V6(v6) = network {
            if let Some(v4) = v6.to_ipv4_mapped() {
                if !(96..=128).contains(&prefix) {
                    return Err(ProbeError::InvalidCidr);
                }
                network = IpAddr::V4(v4);
                prefix -= 96;
            }
        }
        let bits = if network.is_ipv4() { 32 } else { 128 };
        if prefix > bits {
            return Err(ProbeError::InvalidCidr);
        }
        Ok(Self { network, prefix })
    }
}
impl Cidr {
    fn contains(&self, ip: IpAddr) -> bool {
        match (self.network, normalize(ip)) {
            (IpAddr::V4(a), IpAddr::V4(b)) => {
                self.prefix == 0
                    || (u32::from(a) >> (32 - self.prefix)) == (u32::from(b) >> (32 - self.prefix))
            }
            (IpAddr::V6(a), IpAddr::V6(b)) => {
                self.prefix == 0
                    || (u128::from(a) >> (128 - self.prefix))
                        == (u128::from(b) >> (128 - self.prefix))
            }
            _ => false,
        }
    }
}
/// Public-unicast default; special-purpose/tunnel ranges require explicit CIDRs.
/// Classification is deliberately conservative, not an assertion of routability.
#[derive(Debug)]
pub struct TargetPolicy {
    allowed: Vec<Cidr>,
}
impl TargetPolicy {
    pub fn new(allow_cidrs: &[String]) -> Result<Self, ProbeError> {
        if allow_cidrs.len() > 64 {
            return Err(ProbeError::InvalidCidr);
        }
        Ok(Self {
            allowed: allow_cidrs
                .iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
        })
    }
    fn authorized(&self, ip: IpAddr) -> bool {
        let ip = normalize(ip);
        if ip.is_unspecified() || ip.is_multicast() || ip == IpAddr::V4(Ipv4Addr::BROADCAST) {
            return false;
        }
        public_unicast(ip) || self.allowed.iter().any(|cidr| cidr.contains(ip))
    }
    pub fn resolve(
        &self,
        request: TargetRequest,
        resolver: &mut impl Resolver,
        budget: &ProbeBudget,
    ) -> Result<AuthorizedTarget, ProbeError> {
        budget.begin_resolution()?;
        let deadline = budget.phase_deadline(budget.limits.dns_timeout);
        budget.check(deadline)?;
        let answers = match &request.host {
            ProbeHost::Ip(ip) => Ok(vec![*ip]),
            ProbeHost::Dns(host) => resolver.resolve(host, deadline, budget.cancellation()),
        };
        budget.check(deadline)?;
        let answers = answers?;
        if answers.len() > budget.limits.max_dns_answers {
            return Err(ProbeError::TooManyAnswers);
        }
        if answers.is_empty() {
            return Err(ProbeError::DnsFailed);
        }
        if answers.iter().any(|ip| !self.authorized(*ip)) {
            return Err(ProbeError::AddressDenied);
        }
        let mut checked = Vec::new();
        for ip in answers.into_iter().map(normalize) {
            if !checked.contains(&ip) {
                checked.push(ip);
            }
        }
        Ok(AuthorizedTarget {
            socket: SocketAddr::new(checked[0], request.port),
            host: request.host,
            answers: checked,
            budget_id: budget.id,
        })
    }
}
fn public_unicast(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let [a, b, c, _] = v4.octets();
            !(a == 0
                || a == 10
                || a == 127
                || a >= 224
                || (a == 100 && (64..=127).contains(&b))
                || (a == 169 && b == 254)
                || (a == 172 && (16..=31).contains(&b))
                || (a == 192 && b == 168)
                || (a == 192 && b == 0 && c == 0 && ![9, 10].contains(&v4.octets()[3]))
                || (a == 192 && b == 0 && c == 2)
                || (a == 192 && b == 88 && c == 99)
                || (a == 198 && [18, 19].contains(&b))
                || (a == 198 && b == 51 && c == 100)
                || (a == 203 && b == 0 && c == 113))
        }
        IpAddr::V6(v6) => {
            let segments = v6.segments();
            // Permit native global unicast only. Special-purpose 2001::/23,
            // documentation and 6to4 require explicit local authorization.
            segments[0] & 0xe000 == 0x2000
                && !(segments[0] == 0x2001 && segments[1] < 0x0200)
                && !(segments[0] == 0x2001 && segments[1] == 0x0db8)
                && segments[0] != 0x2002
                && !(segments[0] == 0x3fff && segments[1] & 0xf000 == 0)
        }
    }
}
/// A resolver adapter must stop and clean up on deadline/cancellation and retain
/// at most max+1 answers. No system getaddrinfo adapter is hidden in core.
pub trait Resolver {
    fn resolve(
        &mut self,
        host: &str,
        deadline: Instant,
        cancellation: &Cancellation,
    ) -> Result<Vec<IpAddr>, ProbeError>;
}
#[derive(Debug)]
#[cfg_attr(not(feature = "probe-transport"), allow(dead_code))]
pub struct AuthorizedTarget {
    socket: SocketAddr,
    host: ProbeHost,
    answers: Vec<IpAddr>,
    budget_id: u64,
}
impl AuthorizedTarget {
    #[cfg(feature = "probe-transport")]
    pub(crate) fn check_budget(&self, budget: &ProbeBudget) -> Result<(), ProbeError> {
        if self.budget_id == budget.id {
            Ok(())
        } else {
            Err(ProbeError::InvalidTarget)
        }
    }
    pub fn socket_addr(&self) -> SocketAddr {
        self.socket
    }
    pub fn host(&self) -> &ProbeHost {
        &self.host
    }
    pub fn answers(&self) -> &[IpAddr] {
        &self.answers
    }
    pub fn check_peer(&self, peer: SocketAddr) -> Result<(), ProbeError> {
        if SocketAddr::new(normalize(peer.ip()), peer.port()) == self.socket
            && peer.scope_id_or_zero() == 0
        {
            Ok(())
        } else {
            Err(ProbeError::PeerMismatch)
        }
    }
}
trait PeerScope {
    fn scope_id_or_zero(&self) -> u32;
}
impl PeerScope for SocketAddr {
    fn scope_id_or_zero(&self) -> u32 {
        match self {
            SocketAddr::V4(_) => 0,
            SocketAddr::V6(a) => a.scope_id(),
        }
    }
}
#[derive(Debug, Default)]
struct CancellationState {
    cancelled: AtomicBool,
    #[cfg(feature = "probe-transport")]
    socket: std::sync::Mutex<Option<std::sync::Weak<std::net::TcpStream>>>,
}
#[derive(Debug, Clone, Default)]
pub struct Cancellation(Arc<CancellationState>);
impl Cancellation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::Release);
        #[cfg(feature = "probe-transport")]
        if let Ok(guard) = self.0.socket.lock() {
            if let Some(socket) = guard.as_ref().and_then(std::sync::Weak::upgrade) {
                let _ = socket.shutdown(std::net::Shutdown::Both);
            }
        }
    }
    #[cfg(feature = "probe-transport")]
    pub(crate) fn register(&self, socket: &Arc<std::net::TcpStream>) -> Result<(), ProbeError> {
        let mut guard = self.0.socket.lock().map_err(|_| ProbeError::Io)?;
        if guard.as_ref().and_then(std::sync::Weak::upgrade).is_some() {
            return Err(ProbeError::AttemptLimit);
        }
        *guard = Some(Arc::downgrade(socket));
        if self.is_cancelled() {
            let _ = socket.shutdown(std::net::Shutdown::Both);
            return Err(ProbeError::Cancelled);
        }
        Ok(())
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.cancelled.load(Ordering::Acquire)
    }
}
#[derive(Debug, Clone)]
pub struct ProbeLimits {
    pub max_dns_answers: usize,
    pub dns_timeout: Duration,
    pub tcp_timeout: Duration,
    pub protocol_timeout: Duration,
    pub overall_timeout: Duration,
    pub max_received_bytes: u64,
    pub max_sent_bytes: u64,
}
impl Default for ProbeLimits {
    fn default() -> Self {
        Self {
            max_dns_answers: 8,
            dns_timeout: Duration::from_secs(5),
            tcp_timeout: Duration::from_secs(3),
            protocol_timeout: Duration::from_secs(5),
            overall_timeout: Duration::from_secs(15),
            max_received_bytes: 512 * 1024,
            max_sent_bytes: 64 * 1024,
        }
    }
}
#[derive(Debug)]
pub struct ProbeBudget {
    id: u64,
    limits: ProbeLimits,
    cancellation: Cancellation,
    overall_deadline: Instant,
    resolved: Cell<bool>,
    #[cfg(feature = "probe-transport")]
    attempted: Cell<bool>,
    received: u64,
    sent: u64,
}
impl ProbeBudget {
    pub fn new(limits: ProbeLimits, cancellation: Cancellation) -> Result<Self, ProbeError> {
        let caps = ProbeLimits::default();
        for (value, max) in [
            (limits.dns_timeout, caps.dns_timeout),
            (limits.tcp_timeout, caps.tcp_timeout),
            (limits.protocol_timeout, caps.protocol_timeout),
            (limits.overall_timeout, caps.overall_timeout),
        ] {
            if value.is_zero() || value > max {
                return Err(ProbeError::InvalidTarget);
            }
        }
        if limits.max_dns_answers == 0
            || limits.max_dns_answers > caps.max_dns_answers
            || limits.max_received_bytes == 0
            || limits.max_received_bytes > caps.max_received_bytes
            || limits.max_sent_bytes == 0
            || limits.max_sent_bytes > caps.max_sent_bytes
        {
            return Err(ProbeError::InvalidTarget);
        }
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let id = NEXT_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| ProbeError::AttemptLimit)?;
        Ok(Self {
            id,
            overall_deadline: Instant::now() + limits.overall_timeout,
            limits,
            cancellation,
            resolved: Cell::new(false),
            #[cfg(feature = "probe-transport")]
            attempted: Cell::new(false),
            received: 0,
            sent: 0,
        })
    }
    pub fn limits(&self) -> &ProbeLimits {
        &self.limits
    }
    pub fn cancellation(&self) -> &Cancellation {
        &self.cancellation
    }
    pub fn phase_deadline(&self, duration: Duration) -> Instant {
        std::cmp::min(
            Instant::now()
                .checked_add(duration)
                .unwrap_or(self.overall_deadline),
            self.overall_deadline,
        )
    }
    pub fn check(&self, deadline: Instant) -> Result<(), ProbeError> {
        if self.cancellation.is_cancelled() {
            Err(ProbeError::Cancelled)
        } else if Instant::now() >= std::cmp::min(deadline, self.overall_deadline) {
            Err(ProbeError::DeadlineExceeded)
        } else {
            Ok(())
        }
    }
    pub fn remaining(&self, deadline: Instant) -> Result<Duration, ProbeError> {
        self.check(deadline)?;
        Ok(
            std::cmp::min(deadline, self.overall_deadline)
                .saturating_duration_since(Instant::now()),
        )
    }
    fn begin_resolution(&self) -> Result<(), ProbeError> {
        if self.resolved.replace(true) {
            Err(ProbeError::AttemptLimit)
        } else {
            Ok(())
        }
    }
    #[cfg(feature = "probe-transport")]
    pub(crate) fn begin_attempt(&self) -> Result<(), ProbeError> {
        if self.attempted.replace(true) {
            Err(ProbeError::AttemptLimit)
        } else {
            Ok(())
        }
    }
    pub fn received_bytes(&self) -> u64 {
        self.received
    }
    pub fn sent_bytes(&self) -> u64 {
        self.sent
    }
    #[cfg(feature = "probe-transport")]
    pub(crate) fn read_remaining(&self) -> u64 {
        self.limits.max_received_bytes - self.received
    }
    #[cfg(feature = "probe-transport")]
    pub(crate) fn write_remaining(&self) -> u64 {
        self.limits.max_sent_bytes - self.sent
    }
    #[cfg(feature = "probe-transport")]
    pub(crate) fn debit_read(&mut self, n: usize) {
        self.received += n as u64;
    }
    #[cfg(feature = "probe-transport")]
    pub(crate) fn debit_write(&mut self, n: usize) {
        self.sent += n as u64;
    }
}
