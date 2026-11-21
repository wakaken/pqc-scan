//! Separate saved-evidence output; policy remains in core.
use pqc_scan_core::migration::{MigrationAssessment, MigrationContext, SavedEvidence};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

pub const MIGRATION_JSON: &str = "migration.assessment.v1.json";
pub const MAX_MIGRATION_REPORT_BYTES: usize = 64 * 1024 * 1024;

/// Serialize before creating/replacing any output; never truncate a large report.
pub fn migration_json_bytes(report: &MigrationAssessment) -> anyhow::Result<Vec<u8>> {
    let mut output = LimitedBuffer { bytes: Vec::new() };
    serde_json::to_writer_pretty(&mut output, report)
        .map_err(|_| anyhow::anyhow!("resource_limit"))?;
    Ok(output.bytes)
}
pub fn write_migration_assessment_json(
    report: &MigrationAssessment,
    evidence: &SavedEvidence,
    context: Option<&MigrationContext>,
    out_dir: &Path,
) -> anyhow::Result<PathBuf> {
    report.validate_for(evidence, context)?;
    let bytes = migration_json_bytes(report)?;
    let path = out_dir.join(MIGRATION_JSON);
    evidence.validate_output_paths(context, std::slice::from_ref(&path))?;
    pqc_scan_core::safe_output::create_dir_all(out_dir)
        .map_err(|_| anyhow::anyhow!("output_unavailable"))?;
    pqc_scan_core::safe_output::write(&path, bytes)
        .map_err(|_| anyhow::anyhow!("output_unavailable"))?;
    Ok(path)
}
#[derive(Default)]
pub(crate) struct LimitedBuffer {
    pub(crate) bytes: Vec<u8>,
}
impl Write for LimitedBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let size = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .filter(|n| *n <= MAX_MIGRATION_REPORT_BYTES)
            .ok_or_else(|| io::Error::other("resource_limit"))?;
        self.bytes.reserve(size - self.bytes.len());
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Actual serialization/write phases, not an estimate of total elapsed time.
#[derive(Clone, Copy, Debug)]
pub enum MigrationReportProgress {
    Validating,
    Json,
    Markdown,
    Html,
    Writing { completed: usize, total: usize },
}

/// Render and validate the complete selected set before creating any output.
pub fn write_migration_assessment_reports(
    report: &MigrationAssessment,
    evidence: &SavedEvidence,
    context: Option<&MigrationContext>,
    out_dir: &Path,
    language: crate::ReportLanguage,
) -> anyhow::Result<Vec<PathBuf>> {
    write_migration_assessment_reports_with_progress(
        report,
        evidence,
        context,
        out_dir,
        language,
        |_| {},
    )
}

/// Compatible writer with observable real phases; limits and replacement gates are unchanged.
pub fn write_migration_assessment_reports_with_progress(
    report: &MigrationAssessment,
    evidence: &SavedEvidence,
    context: Option<&MigrationContext>,
    out_dir: &Path,
    language: crate::ReportLanguage,
    progress: impl FnMut(MigrationReportProgress),
) -> anyhow::Result<Vec<PathBuf>> {
    write_migration_assessment_reports_with_label_and_progress(
        report, evidence, context, out_dir, language, None, progress,
    )
}

/// Optional display label does not change canonical assessment or context data.
pub fn write_migration_assessment_reports_with_label_and_progress(
    report: &MigrationAssessment,
    evidence: &SavedEvidence,
    context: Option<&MigrationContext>,
    out_dir: &Path,
    language: crate::ReportLanguage,
    label: Option<&str>,
    mut progress: impl FnMut(MigrationReportProgress),
) -> anyhow::Result<Vec<PathBuf>> {
    crate::migration_html::validate_label(label)?;
    progress(MigrationReportProgress::Validating);
    report.validate_for(evidence, context)?;
    progress(MigrationReportProgress::Json);
    let json = migration_json_bytes(report)?;
    progress(MigrationReportProgress::Markdown);
    let markdown = crate::migration_markdown::migration_markdown_bytes(report, language)?;
    progress(MigrationReportProgress::Html);
    let unbindable = evidence
        .targets()
        .iter()
        .filter(|t| !t.is_bindable())
        .map(|t| t.id.as_str())
        .collect::<Vec<_>>();
    let html = crate::migration_html::migration_html_bytes_with_presentation(
        report,
        language,
        label,
        &unbindable,
    )?;
    let paths = vec![
        out_dir.join(MIGRATION_JSON),
        out_dir.join(crate::migration_markdown::MIGRATION_MARKDOWN),
        out_dir.join(crate::migration_html::MIGRATION_HTML),
    ];
    evidence.validate_output_paths(context, &paths)?;
    pqc_scan_core::safe_output::create_dir_all(out_dir)
        .map_err(|_| anyhow::anyhow!("output_unavailable"))?;
    for (completed, (path, bytes)) in paths.iter().zip([json, markdown, html]).enumerate() {
        progress(MigrationReportProgress::Writing {
            completed,
            total: 3,
        });
        pqc_scan_core::safe_output::write(path, bytes)
            .map_err(|_| anyhow::anyhow!("output_unavailable"))?;
    }
    progress(MigrationReportProgress::Writing {
        completed: 3,
        total: 3,
    });
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_byte_limit_is_inclusive_and_does_not_truncate() {
        for size in [
            MAX_MIGRATION_REPORT_BYTES - 1,
            MAX_MIGRATION_REPORT_BYTES,
            MAX_MIGRATION_REPORT_BYTES + 1,
        ] {
            let mut output = LimitedBuffer { bytes: Vec::new() };
            let result = output.write_all(&vec![b'x'; size]);
            assert_eq!(result.is_ok(), size <= MAX_MIGRATION_REPORT_BYTES);
            assert_eq!(
                output.bytes.len(),
                if size <= MAX_MIGRATION_REPORT_BYTES {
                    size
                } else {
                    0
                }
            );
        }
    }
}
