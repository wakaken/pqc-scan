//! Additive inspection outcomes. Legacy scan/report structures remain unchanged.

use serde::Serialize;

use crate::{Detection, ScanConfig, ScanResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectorStatus {
    Complete,
    NotApplicable,
    Unsupported,
    Partial,
    Truncated,
    NotReported,
}

/// A detector's explicit statement about its own inspection, not finding severity.
#[derive(Debug, Clone)]
pub struct DetectorOutcome {
    detections: Vec<Detection>,
    status: DetectorStatus,
    observations: Vec<crate::observation::ObservationDraft>,
    explanation: Option<&'static str>,
}

impl DetectorOutcome {
    pub fn new(detections: Vec<Detection>, status: DetectorStatus) -> Self {
        // Contradictory applicability must never turn returned hits into a clean scan.
        let status = if status == DetectorStatus::NotApplicable && !detections.is_empty() {
            DetectorStatus::Partial
        } else {
            status
        };
        Self {
            detections,
            status,
            observations: Vec::new(),
            explanation: None,
        }
    }

    pub fn with_explanation(mut self, explanation: &'static str) -> Self {
        self.explanation = Some(explanation);
        self
    }
    pub fn explanation(&self) -> Option<&'static str> {
        self.explanation
    }
    pub fn complete(detections: Vec<Detection>) -> Self {
        Self::new(detections, DetectorStatus::Complete)
    }

    pub fn with_observations(
        mut self,
        observations: Vec<crate::observation::ObservationDraft>,
    ) -> Self {
        self.observations = observations;
        self
    }

    pub fn observations(&self) -> &[crate::observation::ObservationDraft] {
        &self.observations
    }

    pub fn status(&self) -> DetectorStatus {
        self.status
    }

    pub fn detections(&self) -> &[Detection] {
        &self.detections
    }

    /// Consume an outcome for a legacy detector entrypoint without cloning hits.
    pub fn into_detections(self) -> Vec<Detection> {
        self.detections
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    ReadFailed,
    DetectorFailed,
    DetectorNotReported,
    DetectorUnsupported,
    DetectorPartial,
    DetectorTruncated,
    NoDetectors,
    ExcludedPath,
    FileTooLarge,
    MetadataFailed,
    EnumerationFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticStage {
    Enumeration,
    Read,
    Detection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDisposition {
    ExcludedByPolicy,
    Failed,
    NotReported,
    Unsupported,
    Partial,
    Truncated,
}

/// Fixed messages intentionally omit arbitrary I/O and detector error contents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScanDiagnostic {
    code: DiagnosticCode,
    stage: DiagnosticStage,
    disposition: DiagnosticDisposition,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detector: Option<String>,
    message: &'static str,
}

impl ScanDiagnostic {
    pub(crate) fn with_explanation(mut self, explanation: Option<&'static str>) -> Self {
        if let Some(message) = explanation {
            self.message = message;
        }
        self
    }
    pub(crate) fn new(
        code: DiagnosticCode,
        file: Option<String>,
        detector: Option<String>,
    ) -> Self {
        use DiagnosticCode as C;
        use DiagnosticDisposition as D;
        use DiagnosticStage as S;
        let (stage, disposition, message) = match code {
            C::ReadFailed => (S::Read, D::Failed, "File contents could not be read."),
            C::DetectorFailed => (S::Detection, D::Failed, "Detector execution failed."),
            C::DetectorNotReported => (
                S::Detection,
                D::NotReported,
                "Detector does not report internal inspection completeness.",
            ),
            C::DetectorUnsupported => (
                S::Detection,
                D::Unsupported,
                "A required inspection is unsupported by the detector.",
            ),
            C::DetectorPartial => (
                S::Detection,
                D::Partial,
                "Detector completed only part of the inspection.",
            ),
            C::DetectorTruncated => (
                S::Detection,
                D::Truncated,
                "Detector reached an inspection limit; omitted results are not counted.",
            ),
            C::NoDetectors => (
                S::Detection,
                D::NotReported,
                "No detectors were supplied for selected files.",
            ),
            C::ExcludedPath => (
                S::Enumeration,
                D::ExcludedByPolicy,
                "Path is excluded by the default scan policy.",
            ),
            C::FileTooLarge => (
                S::Enumeration,
                D::ExcludedByPolicy,
                "File exceeds the configured size limit.",
            ),
            C::MetadataFailed => (
                S::Enumeration,
                D::Failed,
                "File metadata could not be read; the entry was not selected.",
            ),
            C::EnumerationFailed => (
                S::Enumeration,
                D::Failed,
                "A repository entry could not be enumerated.",
            ),
        };
        Self {
            code,
            stage,
            disposition,
            file,
            detector,
            message,
        }
    }

    pub fn code(&self) -> DiagnosticCode {
        self.code
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    pub fn detector(&self) -> Option<&str> {
        self.detector.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectionStatus {
    CompleteWithinScope,
    Partial,
    Failed,
    Unknown,
}

impl InspectionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompleteWithinScope => "complete_within_scope",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CoverageSummary {
    pub(crate) selected_files: u64,
    pub(crate) read_ok_files: u64,
    pub(crate) read_failed_files: u64,
    pub(crate) analyzed_files: u64,
    pub(crate) partial_files: u64,
    pub(crate) analysis_failed_files: u64,
    pub(crate) unsupported_files: u64,
    pub(crate) not_applicable_files: u64,
    pub(crate) unknown_files: u64,
    pub(crate) excluded_enumerated_files: u64,
    pub(crate) enumeration_failed_entries: u64,
    not_enumerated_population: bool,
    status: InspectionStatus,
}

macro_rules! count_accessors {
    ($($name:ident),+ $(,)?) => {
        $(pub fn $name(&self) -> u64 { self.$name })+
    };
}

impl CoverageSummary {
    pub(crate) fn new() -> Self {
        Self {
            selected_files: 0,
            read_ok_files: 0,
            read_failed_files: 0,
            analyzed_files: 0,
            partial_files: 0,
            analysis_failed_files: 0,
            unsupported_files: 0,
            not_applicable_files: 0,
            unknown_files: 0,
            excluded_enumerated_files: 0,
            enumeration_failed_entries: 0,
            not_enumerated_population: true,
            status: InspectionStatus::Unknown,
        }
    }

    count_accessors!(
        selected_files,
        read_ok_files,
        read_failed_files,
        analyzed_files,
        partial_files,
        analysis_failed_files,
        unsupported_files,
        not_applicable_files,
        unknown_files,
        excluded_enumerated_files,
        enumeration_failed_entries
    );

    pub fn not_enumerated_population(&self) -> bool {
        self.not_enumerated_population
    }

    pub fn status(&self) -> InspectionStatus {
        self.status
    }

    pub fn is_complete(&self) -> bool {
        self.status == InspectionStatus::CompleteWithinScope
    }

    pub(crate) fn finish(&mut self) {
        let failed =
            self.read_failed_files + self.analysis_failed_files + self.enumeration_failed_entries;
        let incomplete = self.partial_files + self.unsupported_files;
        self.status = if failed > 0 || incomplete > 0 {
            if failed > 0 && self.read_ok_files == self.analysis_failed_files {
                InspectionStatus::Failed
            } else {
                InspectionStatus::Partial
            }
        } else if self.unknown_files > 0 {
            InspectionStatus::Unknown
        } else {
            InspectionStatus::CompleteWithinScope
        };
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InspectionScope {
    max_file_size_bytes: usize,
    git_ignore: bool,
    git_global: bool,
    git_exclude: bool,
    default_excludes: Vec<&'static str>,
    markdown_excluded: bool,
}

impl InspectionScope {
    pub fn max_file_size_bytes(&self) -> usize {
        self.max_file_size_bytes
    }
    pub fn default_excludes(&self) -> &[&'static str] {
        &self.default_excludes
    }
    pub fn markdown_excluded(&self) -> bool {
        self.markdown_excluded
    }
    pub fn git_ignore(&self) -> bool {
        self.git_ignore
    }
    pub fn git_global(&self) -> bool {
        self.git_global
    }
    pub fn git_exclude(&self) -> bool {
        self.git_exclude
    }
}

/// No Serialize implementation: this is not a replacement native scan schema.
#[derive(Debug, Clone)]
pub struct ScanDocument {
    legacy: ScanResult,
    diagnostics: Vec<ScanDiagnostic>,
    coverage: CoverageSummary,
    scope: InspectionScope,
    native: Option<crate::inventory::NativeScan>,
}

impl ScanDocument {
    pub(crate) fn record_binary_exclusion_policy(&mut self) {
        self.scope
            .default_excludes
            .push(crate::file_selection::POLICY_LABEL);
    }

    pub(crate) fn new(
        legacy: ScanResult,
        mut diagnostics: Vec<ScanDiagnostic>,
        mut coverage: CoverageSummary,
        config: &ScanConfig,
    ) -> Self {
        diagnostics
            .sort_by(|a, b| (&a.file, &a.detector, a.code).cmp(&(&b.file, &b.detector, b.code)));
        diagnostics.dedup();
        coverage.finish();
        Self {
            legacy,
            diagnostics,
            coverage,
            native: None,
            scope: InspectionScope {
                max_file_size_bytes: config.max_file_size_bytes,
                git_ignore: true,
                git_global: true,
                git_exclude: true,
                default_excludes: crate::walker::DEFAULT_EXCLUDES.to_vec(),
                markdown_excluded: true,
            },
        }
    }

    pub fn native(&self) -> Option<&crate::inventory::NativeScan> {
        self.native.as_ref()
    }

    pub(crate) fn with_native(mut self, native: crate::inventory::NativeScan) -> Self {
        self.native = Some(native);
        self
    }

    pub fn legacy(&self) -> &ScanResult {
        &self.legacy
    }

    pub(crate) fn legacy_mut(&mut self) -> &mut ScanResult {
        &mut self.legacy
    }

    pub fn into_legacy(self) -> ScanResult {
        self.legacy
    }

    pub fn diagnostics(&self) -> &[ScanDiagnostic] {
        &self.diagnostics
    }

    pub fn coverage(&self) -> &CoverageSummary {
        &self.coverage
    }

    pub fn scope(&self) -> &InspectionScope {
        &self.scope
    }

    pub fn diagnostics_report(&self) -> DiagnosticsReport<'_> {
        DiagnosticsReport {
            schema_version: "1",
            generated_at: &self.legacy.generated_at,
            scope: &self.scope,
            coverage: &self.coverage,
            diagnostics: &self.diagnostics,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DiagnosticsReport<'a> {
    schema_version: &'static str,
    generated_at: &'a chrono::DateTime<chrono::Utc>,
    scope: &'a InspectionScope,
    coverage: &'a CoverageSummary,
    diagnostics: &'a [ScanDiagnostic],
}
