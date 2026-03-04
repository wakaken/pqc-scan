use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("failed to read file: {0}")]
    ReadFile(String),
}
