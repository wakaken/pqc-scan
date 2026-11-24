use clap::Args;
use pqc_scan_core::migration::{evaluate, Error, InputFormat, MigrationContext, SavedEvidence};
use std::{io::Write, path::PathBuf};

#[derive(Debug, Args)]
pub(crate) struct AssessArgs {
    /// One saved JSON file; never scanned or executed.
    input: PathBuf,
    /// Explicit inventory-v1, tls-v1, ssh-offers-v1 or ssh-kex-v1.
    #[arg(long)]
    input_format: String,
    /// Explicit evaluation instant with UTC offset and whole seconds.
    #[arg(long)]
    as_of: String,
    /// Scoped migration-context-v1 file; old assessment context is separate.
    #[arg(long)]
    context: Option<PathBuf>,
    #[arg(long)]
    out_dir: PathBuf,
    /// Initial HTML and Markdown language: en or ja.
    #[arg(long, default_value = "en")]
    lang: String,
    /// Optional non-secret display label for the HTML report (up to 256 bytes).
    #[arg(long)]
    report_label: Option<String>,
}
pub(crate) fn run(args: AssessArgs, stdout: &mut impl Write) -> anyhow::Result<()> {
    let language = match args.lang.as_str() {
        "en" => pqc_scan_report::ReportLanguage::En,
        "ja" => pqc_scan_report::ReportLanguage::Ja,
        _ => return Err(anyhow::anyhow!("unsupported_language")),
    };
    let format: InputFormat = args.input_format.parse()?;
    let evidence = SavedEvidence::read(&args.input, format)?;
    let context = args
        .context
        .as_deref()
        .map(MigrationContext::read)
        .transpose()
        .map_err(|e| match e {
            Error::InvalidInput | Error::InvalidContextBinding => {
                anyhow::anyhow!("invalid_context")
            }
            other => anyhow::Error::from(other),
        })?;
    let report = evaluate(&evidence, context.as_ref(), &args.as_of)?;
    pqc_scan_report::write_migration_assessment_reports_with_label_and_progress(
        &report,
        &evidence,
        context.as_ref(),
        &args.out_dir,
        language,
        args.report_label.as_deref(),
        |_| {},
    )?;
    writeln!(
        stdout,
        "written {} {} {}",
        pqc_scan_report::MIGRATION_JSON,
        pqc_scan_report::MIGRATION_MARKDOWN,
        pqc_scan_report::MIGRATION_HTML
    )?;
    Ok(())
}
