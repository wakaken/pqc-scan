pub mod assessment;
pub mod classifier;
pub mod compat;
pub mod diagnostic;
pub mod diff;
pub mod error;
pub mod identity;
pub mod inventory;
pub mod model;
pub mod observation;
pub mod pipeline;
pub mod policy;
pub mod privacy;
pub mod recommendation;
pub mod risk;
pub mod walker;

pub use diagnostic::{
    CoverageSummary, DetectorOutcome, DetectorStatus, DiagnosticCode, DiagnosticDisposition,
    DiagnosticStage, DiagnosticsReport, InspectionScope, InspectionStatus, ScanDiagnostic,
    ScanDocument,
};
pub use inventory::{NativeOptions, NativeScan};
pub use model::{
    CbomEntry, CodeExample, DependencySbomEntry, Detection, Evidence, Finding, Location,
    RecommendedAction, ScanConfig, ScanResult, ScanSummary, ScannableFile, SourceSnippet,
    SourceSnippetLine,
};
pub use observation::{ObservationDraft, ObservationMethod, PublicCertificateFacts};
pub use pipeline::{scan_repository, scan_repository_detailed, scan_repository_native};

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
    /// Opt-in observation capture. Old implementations remain Unknown-assurance adapters.
    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        let outcome = self.detect_detailed(file, rules)?;
        let drafts = outcome
            .detections()
            .iter()
            .map(|d| ObservationDraft::legacy(d).with_producer(self.name()))
            .collect();
        Ok(outcome.with_observations(drafts))
    }
}
