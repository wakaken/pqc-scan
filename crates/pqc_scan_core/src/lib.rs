pub mod classifier;
pub mod diagnostic;
pub mod error;
pub mod model;
pub mod pipeline;
pub mod recommendation;
pub mod risk;
pub mod walker;

pub use diagnostic::{
    CoverageSummary, DetectorOutcome, DetectorStatus, DiagnosticCode, DiagnosticDisposition,
    DiagnosticStage, DiagnosticsReport, InspectionScope, InspectionStatus, ScanDiagnostic,
    ScanDocument,
};
pub use model::{
    CbomEntry, CodeExample, DependencySbomEntry, Detection, Evidence, Finding, Location,
    RecommendedAction, ScanConfig, ScanResult, ScanSummary, ScannableFile, SourceSnippet,
    SourceSnippetLine,
};
pub use pipeline::{scan_repository, scan_repository_detailed};

use anyhow::Result;
use pqc_scan_rules::RuleSet;

pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>>;

    /// Compatibility adapter: successful legacy output does not attest to completeness.
    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        Ok(DetectorOutcome::new(
            self.detect(file, rules)?,
            DetectorStatus::NotReported,
        ))
    }
}
