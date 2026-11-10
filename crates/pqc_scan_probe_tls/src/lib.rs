//! Optional, fresh and ordinarily verified TLS observations.
//! The default crate contains data types only and performs no networking.
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TlsOptions {
    pub verification_name: String,
    pub ca_file: Option<PathBuf>,
}
#[derive(Debug, Clone, Serialize)]
pub struct RegistryIdentifier {
    pub id: u16,
    pub name: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct TlsClientProfile {
    pub name: String,
    pub provider: String,
    pub provider_version: String,
    pub rustls_version: String,
    pub protocol_versions: Vec<RegistryIdentifier>,
    pub cipher_suites: Vec<RegistryIdentifier>,
    pub key_exchange_groups: Vec<RegistryIdentifier>,
}
#[derive(Debug, Clone, Serialize)]
pub struct TlsVerification {
    pub verification_name: String,
    pub sent_sni: Option<String>,
    pub trust_source: String,
    pub trust_bundle_version: Option<String>,
    pub root_set_sha256: String,
    pub root_set_digest_basis: String,
    pub certificate_chain_and_name: String,
    pub revocation: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct NegotiatedTls {
    pub version: RegistryIdentifier,
    pub cipher_suite: RegistryIdentifier,
    pub key_exchange_group: Option<RegistryIdentifier>,
    pub group_classification: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct PeerCertificate {
    pub certificate_der_sha256: String,
    pub spki_sha256: Option<String>,
    pub signature_oid: Option<String>,
    pub public_key_oid: Option<String>,
    pub key_bits: Option<u64>,
    pub named_curve_oid: Option<String>,
    pub not_before_unix: Option<i64>,
    pub not_after_unix: Option<i64>,
    pub attribute_status: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct CertificateCollection {
    pub status: String,
    pub presented_count: Option<usize>,
    pub presented_bytes: Option<usize>,
    pub collected_count: usize,
}
#[derive(Debug, Clone, Serialize)]
pub struct TlsObservation {
    pub client_profile: TlsClientProfile,
    pub verification: TlsVerification,
    pub handshake_complete: bool,
    pub handshake_kind: Option<String>,
    pub negotiated: Option<NegotiatedTls>,
    pub peer_certificates: Vec<PeerCertificate>,
    pub certificate_collection: CertificateCollection,
}
#[cfg(feature = "runtime")]
mod runtime;
#[cfg(feature = "runtime")]
pub use runtime::TlsProbe;
