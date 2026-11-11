//! Bounded unauthenticated SSH offer collection. No key exchange execution.
mod model;
mod parser;
#[cfg(feature = "runtime")]
mod runtime;

pub use model::{AlgorithmCandidate, SshClientProfile, SshObservation, SshOffers, SshProbe};
pub use parser::{
    encode_kexinit, encode_packet, parse_identification, parse_kexinit, parse_packet,
};

pub const MAX_FRAME_BYTES: usize = 256 * 1024;
pub const MAX_BANNER_BYTES: usize = 8 * 1024;
pub const MAX_PRE_IDENTIFICATION_LINES: usize = 64;
pub const MAX_EARLY_MESSAGES: usize = 64;
pub const MAX_NAMES_PER_LIST: usize = 256;
pub const MAX_NAME_BYTES: usize = 64;

#[cfg(feature = "negotiated")]
mod negotiated;
#[cfg(feature = "negotiated")]
pub use negotiated::{SshKexObservation, SshKexProbe};
