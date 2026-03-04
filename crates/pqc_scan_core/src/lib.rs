pub mod classifier;
pub mod error;
pub mod model;
pub mod pipeline;
pub mod recommendation;
pub mod risk;
pub mod walker;

pub use model::{
    CbomEntry, CodeExample, DependencySbomEntry, Detection, Evidence, Finding, Location,
    RecommendedAction, ScanConfig, ScanResult, ScanSummary, ScannableFile, SourceSnippet,
    SourceSnippetLine,
};
pub use pipeline::scan_repository;

use anyhow::Result;
use pqc_scan_rules::RuleSet;

pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>>;
}
