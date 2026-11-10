use pqc_scan_core::probe::{
    ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport, ProbeTarget,
};
use pqc_scan_core::target_policy::ProbeLimits;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SshOffers {
    pub kex_algorithms: Vec<String>,
    pub server_host_key_algorithms: Vec<String>,
    pub encryption_algorithms_client_to_server: Vec<String>,
    pub encryption_algorithms_server_to_client: Vec<String>,
    pub mac_algorithms_client_to_server: Vec<String>,
    pub mac_algorithms_server_to_client: Vec<String>,
    pub compression_algorithms_client_to_server: Vec<String>,
    pub compression_algorithms_server_to_client: Vec<String>,
    pub languages_client_to_server: Vec<String>,
    pub languages_server_to_client: Vec<String>,
}
impl SshOffers {
    pub(crate) fn lists(&self) -> [&[String]; 10] {
        [
            &self.kex_algorithms,
            &self.server_host_key_algorithms,
            &self.encryption_algorithms_client_to_server,
            &self.encryption_algorithms_server_to_client,
            &self.mac_algorithms_client_to_server,
            &self.mac_algorithms_server_to_client,
            &self.compression_algorithms_client_to_server,
            &self.compression_algorithms_server_to_client,
            &self.languages_client_to_server,
            &self.languages_server_to_client,
        ]
    }
    pub fn extension_markers(&self) -> Vec<String> {
        self.kex_algorithms
            .iter()
            .filter(|name| is_marker(name))
            .cloned()
            .collect()
    }
    pub fn kex_candidates(&self) -> Vec<AlgorithmCandidate> {
        self.kex_algorithms
            .iter()
            .filter(|name| !is_marker(name))
            .map(|name| classify(name, false))
            .collect()
    }
    pub fn host_key_candidates(&self) -> Vec<AlgorithmCandidate> {
        self.server_host_key_algorithms
            .iter()
            .map(|name| classify(name, true))
            .collect()
    }
}
fn is_marker(name: &str) -> bool {
    matches!(
        name,
        "ext-info-c"
            | "ext-info-s"
            | "kex-strict-c-v00@openssh.com"
            | "kex-strict-s-v00@openssh.com"
    )
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlgorithmCandidate {
    pub name: String,
    pub construction: &'static str,
    pub namespace: &'static str,
    pub provenance: &'static str,
    pub reference: Option<&'static str>,
}
fn classify(name: &str, host_key: bool) -> AlgorithmCandidate {
    let (construction, provenance, reference) = if host_key {
        match name {
            "ssh-ed25519" => (
                "classical",
                "rfc_defined",
                Some("https://www.rfc-editor.org/rfc/rfc8709.html"),
            ),
            "ecdsa-sha2-nistp256" | "ecdsa-sha2-nistp384" | "ecdsa-sha2-nistp521" => (
                "classical",
                "rfc_defined",
                Some("https://www.rfc-editor.org/rfc/rfc5656.html"),
            ),
            "rsa-sha2-256" | "rsa-sha2-512" => (
                "classical",
                "rfc_defined",
                Some("https://www.rfc-editor.org/rfc/rfc8332.html"),
            ),
            "ssh-rsa" | "ssh-dss" => (
                "classical",
                "rfc_defined",
                Some("https://www.rfc-editor.org/rfc/rfc4253.html"),
            ),
            _ => ("unknown", "unknown", None),
        }
    } else {
        match name {
            "mlkem768x25519-sha256" | "mlkem768nistp256-sha256" | "mlkem1024nistp384-sha384" => (
                "hybrid",
                "rfc_informational",
                Some("https://www.rfc-editor.org/rfc/rfc10042.html"),
            ),
            "sntrup761x25519-sha512" | "sntrup761x25519-sha512@openssh.com" => (
                "hybrid",
                "openssh_defined",
                Some("https://www.openssh.org/releasenotes.html"),
            ),
            "curve25519-sha256" | "curve448-sha512" => (
                "classical",
                "rfc_defined",
                Some("https://www.rfc-editor.org/rfc/rfc8731.html"),
            ),
            "curve25519-sha256@libssh.org" => (
                "classical",
                "vendor_defined",
                Some("https://www.openssh.org/releasenotes.html"),
            ),
            _ => ("unknown", "unknown", None),
        }
    };
    AlgorithmCandidate {
        name: name.into(),
        construction,
        namespace: if name.contains('@') {
            "vendor_namespaced"
        } else {
            "unqualified"
        },
        provenance,
        reference,
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct SshClientProfile {
    pub identification: String,
    pub purpose: &'static str,
    pub offers: SshOffers,
    pub first_kex_packet_follows: bool,
    pub reserved: u32,
}
#[derive(Debug, Clone, Serialize)]
pub struct SshObservation {
    pub client_profile: SshClientProfile,
    pub assurance: &'static str,
    pub server_identification: Option<String>,
    pub offers: Option<SshOffers>,
    pub first_kex_packet_follows: Option<bool>,
    pub kex_candidates: Vec<AlgorithmCandidate>,
    pub host_key_candidates: Vec<AlgorithmCandidate>,
    pub extension_markers: Vec<String>,
    pub pre_identification_lines: usize,
    pub early_messages: usize,
}
#[derive(Debug, Clone)]
pub struct SshProbe {
    pub(crate) profile: SshClientProfile,
}
impl Default for SshProbe {
    fn default() -> Self {
        Self::new()
    }
}
impl SshProbe {
    pub fn new() -> Self {
        let names = |values: &[&str]| values.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        let ciphers = names(&[
            "chacha20-poly1305@openssh.com",
            "aes256-gcm@openssh.com",
            "aes128-gcm@openssh.com",
            "aes256-ctr",
            "aes128-ctr",
        ]);
        let macs = names(&[
            "hmac-sha2-256-etm@openssh.com",
            "hmac-sha2-512-etm@openssh.com",
            "hmac-sha2-256",
        ]);
        Self {
            profile: SshClientProfile {
                identification: format!("SSH-2.0-pqc_scan_{}", env!("CARGO_PKG_VERSION")),
                purpose: "diagnostic_proposal_only",
                offers: SshOffers {
                    kex_algorithms: names(&[
                        "mlkem768x25519-sha256",
                        "sntrup761x25519-sha512",
                        "sntrup761x25519-sha512@openssh.com",
                        "curve25519-sha256",
                        "curve25519-sha256@libssh.org",
                    ]),
                    server_host_key_algorithms: names(&[
                        "ssh-ed25519",
                        "ecdsa-sha2-nistp256",
                        "rsa-sha2-512",
                        "rsa-sha2-256",
                    ]),
                    encryption_algorithms_client_to_server: ciphers.clone(),
                    encryption_algorithms_server_to_client: ciphers,
                    mac_algorithms_client_to_server: macs.clone(),
                    mac_algorithms_server_to_client: macs,
                    compression_algorithms_client_to_server: names(&["none"]),
                    compression_algorithms_server_to_client: names(&["none"]),
                    languages_client_to_server: vec![],
                    languages_server_to_client: vec![],
                },
                first_kex_packet_follows: false,
                reserved: 0,
            },
        }
    }
    pub fn profile(&self) -> &SshClientProfile {
        &self.profile
    }
    pub fn failed(
        &self,
        target: ProbeTarget,
        limits: &ProbeLimits,
        diagnostic: ProbeDiagnostic,
    ) -> ProbeReport<SshObservation> {
        let mut report = ProbeReport::new(
            ProbeProtocol::SshOffers,
            target,
            limits,
            SshObservation {
                client_profile: self.profile.clone(),
                assurance: "not_observed",
                server_identification: None,
                offers: None,
                first_kex_packet_follows: None,
                kex_candidates: vec![],
                host_key_candidates: vec![],
                extension_markers: vec![],
                pre_identification_lines: 0,
                early_messages: 0,
            },
        );
        report.set_outcome(ProbeOutcome::Failed, vec![diagnostic]);
        report
    }
}
