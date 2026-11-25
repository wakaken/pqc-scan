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
    limit_detail: Option<String>,
    test_ranges: Vec<std::ops::Range<usize>>,
    display_ranges: Vec<std::ops::Range<usize>>,
    sensitive_ranges: Vec<std::ops::Range<usize>>,
    sensitive_source_context: bool,
    typed_source_references_only: bool,
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
            limit_detail: None,
            test_ranges: Vec::new(),
            display_ranges: Vec::new(),
            sensitive_ranges: Vec::new(),
            sensitive_source_context: false,
            typed_source_references_only: false,
        }
    }

    pub fn with_explanation(mut self, explanation: &'static str) -> Self {
        self.explanation = Some(explanation);
        self
    }
    /// Optional syntax-proven test-only regions in decoded UTF-8 source.
    /// This ephemeral scope annotation is never part of stored detector output.
    pub fn with_test_ranges(mut self, ranges: Vec<std::ops::Range<usize>>) -> Self {
        self.test_ranges = ranges;
        self
    }
    pub fn test_ranges(&self) -> &[std::ops::Range<usize>] {
        &self.test_ranges
    }
    /// Ephemeral syntax-proven diagnostic/display text, never stored as source.
    pub fn with_display_ranges(mut self, ranges: Vec<std::ops::Range<usize>>) -> Self {
        self.display_ranges = ranges;
        self
    }
    pub fn display_ranges(&self) -> &[std::ops::Range<usize>] {
        &self.display_ranges
    }
    /// Ephemeral decoded UTF-8 argument ranges that may contain key material.
    /// The pipeline protects overlapping evidence and source context, including
    /// context requested by other detectors. No payload or range is serialized.
    pub fn with_sensitive_ranges(mut self, ranges: Vec<std::ops::Range<usize>>) -> Self {
        self.sensitive_ranges = ranges;
        self
    }
    pub fn sensitive_ranges(&self) -> &[std::ops::Range<usize>] {
        &self.sensitive_ranges
    }
    /// Suppress source context independently of successful parsing/binding.
    /// Ephemeral only; useful when partial analysis cannot enumerate key arguments.
    pub fn with_sensitive_source_context(mut self, sensitive: bool) -> Self {
        self.sensitive_source_context = sensitive;
        self
    }
    pub fn sensitive_source_context(&self) -> bool {
        self.sensitive_source_context
    }
    /// Restrict algorithm inventory to the bounded source handler's typed facts,
    /// independently of source presentation privacy. Ephemeral, not serialized.
    pub fn with_typed_source_references_only(mut self, typed_only: bool) -> Self {
        self.typed_source_references_only = typed_only;
        self
    }
    pub fn typed_source_references_only(&self) -> bool {
        self.typed_source_references_only
    }
    /// Fixed reason code and bounded numeric work counters only; never source text.
    pub fn with_limit(mut self, reason: &'static str, limit: usize, processed: usize) -> Self {
        self.limit_detail = Some(format!("Inspection limit: {reason}; limit={limit}; processed={processed}. Retained findings remain available; omitted findings are unknown. Inspect the remaining source separately."));
        self
    }
    pub fn explanation(&self) -> Option<&'static str> {
        self.explanation
    }
    pub fn diagnostic_message(&self) -> Option<&str> {
        self.limit_detail.as_deref().or(self.explanation)
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
    message: String,
}

impl ScanDiagnostic {
    pub(crate) fn with_explanation(mut self, explanation: Option<&str>) -> Self {
        if let Some(message) = explanation {
            self.message = message.into();
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
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
    pub(crate) fn with_size_limit(mut self, limit: usize, bytes: u64) -> Self {
        self.message = format!("File size limit: limit={limit}; file_bytes={bytes}. File was not inspected. Increase the size limit deliberately or inspect the file separately.");
        self
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
    pub(crate) fn record_custom_exclusion_policy(&mut self) {
        self.scope
            .default_excludes
            .push("root .pqcignore patterns (explicit opt-out)");
    }
    pub(crate) fn record_test_exclusion_policy(&mut self) {
        self.scope
            .default_excludes
            .push(crate::file_selection::TEST_POLICY_LABEL);
        self.scope
            .default_excludes
            .push("syntax-proven Rust inline test findings (explicit opt-out)");
    }
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
