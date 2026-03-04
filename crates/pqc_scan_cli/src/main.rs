use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use pqc_scan_core::{pipeline, scan_repository, ScanConfig};
use pqc_scan_detectors::default_detectors;
use pqc_scan_report::{write_reports, ReportFormat};
use pqc_scan_rules::{RuleSet, Severity};

#[derive(Debug, Parser)]
#[command(
    name = "pqc-scan",
    version,
    about = "PQC migration scanner for source repositories"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Scan(ScanArgs),
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
}

#[derive(Debug, Parser)]
struct ScanArgs {
    path: PathBuf,

    #[arg(long, value_enum, default_value_t = FormatArg::All)]
    format: FormatArg,

    #[arg(long, default_value = "./pqc-report")]
    out_dir: PathBuf,

    #[arg(long, default_value = "./rules")]
    rules_dir: PathBuf,

    #[arg(long, value_enum)]
    fail_on: Option<FailOnArg>,

    #[arg(long, default_value_t = 0usize)]
    threads: usize,
}

#[derive(Debug, Subcommand)]
enum RulesCommand {
    List(RulesListArgs),
}

#[derive(Debug, Parser)]
struct RulesListArgs {
    #[arg(long, default_value = "./rules")]
    rules_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum FormatArg {
    Json,
    Html,
    Md,
    Sarif,
    All,
}

impl From<FormatArg> for ReportFormat {
    fn from(value: FormatArg) -> Self {
        match value {
            FormatArg::Json => Self::Json,
            FormatArg::Html => Self::Html,
            FormatArg::Md => Self::Markdown,
            FormatArg::Sarif => Self::Sarif,
            FormatArg::All => Self::All,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum FailOnArg {
    High,
    Critical,
}

impl From<FailOnArg> for Severity {
    fn from(value: FailOnArg) -> Self {
        match value {
            FailOnArg::High => Self::High,
            FailOnArg::Critical => Self::Critical,
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan(args) => scan_cmd(args),
        Commands::Rules { command } => match command {
            RulesCommand::List(args) => list_rules_cmd(args),
        },
    }
}

fn scan_cmd(args: ScanArgs) -> Result<()> {
    let rules = RuleSet::load_from_dir(&args.rules_dir)
        .with_context(|| format!("failed to load rules from {}", args.rules_dir.display()))?;

    if rules.is_empty() {
        anyhow::bail!("no rules found under {}", args.rules_dir.display());
    }

    let config = ScanConfig {
        root: args.path,
        max_file_size_bytes: 2 * 1024 * 1024,
        threads: args.threads,
    };

    let detectors = default_detectors();
    let result = scan_repository(&config, &rules, &detectors)?;

    let outputs = write_reports(&result, &args.out_dir, args.format.into())?;
    for out in outputs {
        println!("written {}", out.display());
    }

    println!("total findings: {}", result.summary.total_findings);
    println!("scanned files: {}", result.summary.scanned_files);
    println!("skipped files: {}", result.summary.skipped_files);

    if let Some(level) = args.fail_on {
        let threshold: Severity = level.into();
        if pipeline::exceeds_fail_threshold(&result.findings, threshold) {
            eprintln!("fail-on threshold reached: {threshold}");
            std::process::exit(2);
        }
    }

    Ok(())
}

fn list_rules_cmd(args: RulesListArgs) -> Result<()> {
    let rules = RuleSet::load_from_dir(&args.rules_dir)
        .with_context(|| format!("failed to load rules from {}", args.rules_dir.display()))?;

    println!("rules loaded: {}", rules.len());
    let counts = rules.counts_by_kind();
    for (kind, count) in counts {
        println!("- {:?}: {}", kind, count);
    }

    for rule in rules.all() {
        println!(
            "{} | kind={:?} | severity={} | risk={}",
            rule.id, rule.kind, rule.severity, rule.risk
        );
    }

    Ok(())
}
