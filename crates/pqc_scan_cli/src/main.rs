use std::{
    ffi::OsString,
    io::{self, Write},
    path::PathBuf,
};

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
    let exit_code = match run() {
        Ok(status) => status.code(),
        Err(err) => {
            eprintln!("error: {err:#}");
            1
        }
    };

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExitStatus {
    Success,
    FailThreshold,
}

impl ExitStatus {
    fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::FailThreshold => 2,
        }
    }
}

fn run() -> Result<ExitStatus> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    run_with_args(std::env::args_os(), &mut stdout, &mut stderr)
}

fn run_with_args<I, T, WOut, WErr>(
    args: I,
    stdout: &mut WOut,
    stderr: &mut WErr,
) -> Result<ExitStatus>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    WOut: Write,
    WErr: Write,
{
    let cli = Cli::try_parse_from(args)?;
    run_cli(cli, stdout, stderr)
}

fn run_cli<WOut: Write, WErr: Write>(
    cli: Cli,
    stdout: &mut WOut,
    stderr: &mut WErr,
) -> Result<ExitStatus> {
    match cli.command {
        Commands::Scan(args) => scan_cmd(args, stdout, stderr),
        Commands::Rules { command } => match command {
            RulesCommand::List(args) => list_rules_cmd(args, stdout),
        },
    }
}

fn scan_cmd<WOut: Write, WErr: Write>(
    args: ScanArgs,
    stdout: &mut WOut,
    stderr: &mut WErr,
) -> Result<ExitStatus> {
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
        writeln!(stdout, "written {}", out.display())?;
    }

    writeln!(stdout, "total findings: {}", result.summary.total_findings)?;
    writeln!(stdout, "scanned files: {}", result.summary.scanned_files)?;
    writeln!(stdout, "skipped files: {}", result.summary.skipped_files)?;

    if let Some(level) = args.fail_on {
        let threshold: Severity = level.into();
        if pipeline::exceeds_fail_threshold(&result.findings, threshold) {
            writeln!(stderr, "fail-on threshold reached: {threshold}")?;
            return Ok(ExitStatus::FailThreshold);
        }
    }

    Ok(ExitStatus::Success)
}

fn list_rules_cmd<WOut: Write>(args: RulesListArgs, stdout: &mut WOut) -> Result<ExitStatus> {
    let rules = RuleSet::load_from_dir(&args.rules_dir)
        .with_context(|| format!("failed to load rules from {}", args.rules_dir.display()))?;

    writeln!(stdout, "rules loaded: {}", rules.len())?;
    let counts = rules.counts_by_kind();
    for (kind, count) in counts {
        writeln!(stdout, "- {:?}: {}", kind, count)?;
    }

    for rule in rules.all() {
        writeln!(
            stdout,
            "{} | kind={:?} | severity={} | risk={}",
            rule.id, rule.kind, rule.severity, rule.risk
        )?;
    }

    Ok(ExitStatus::Success)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("workspace root")
    }

    fn rules_dir() -> PathBuf {
        workspace_root().join("rules/default")
    }

    fn temp_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("pqc-scan-{prefix}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn write_file(root: &Path, relative: &str, contents: &str) {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, contents).expect("write test file");
    }

    fn run_cli_test(args: Vec<String>) -> (Result<ExitStatus>, String, String) {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let result = run_with_args(args, &mut stdout, &mut stderr);
        let stdout = String::from_utf8(stdout).expect("utf8 stdout");
        let stderr = String::from_utf8(stderr).expect("utf8 stderr");
        (result, stdout, stderr)
    }

    #[test]
    fn scan_command_writes_requested_report() {
        let repo_dir = temp_dir("scan");
        let out_dir = temp_dir("scan-out");
        write_file(&repo_dir, "src/app.js", "const alg = 'RS256';\n");

        let args = vec![
            "pqc-scan".to_string(),
            "scan".to_string(),
            repo_dir.display().to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--out-dir".to_string(),
            out_dir.display().to_string(),
            "--rules-dir".to_string(),
            rules_dir().display().to_string(),
            "--threads".to_string(),
            "1".to_string(),
        ];

        let (result, stdout, stderr) = run_cli_test(args);

        assert_eq!(result.expect("scan succeeds"), ExitStatus::Success);
        assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
        assert!(
            out_dir.join("report.json").exists(),
            "report.json should exist"
        );
        assert!(stdout.contains("written "));
        assert!(stdout.contains("total findings:"));

        fs::remove_dir_all(repo_dir).expect("cleanup repo");
        fs::remove_dir_all(out_dir).expect("cleanup out");
    }

    #[test]
    fn scan_command_returns_fail_on_threshold_exit_code() {
        let repo_dir = temp_dir("fail-on");
        let out_dir = temp_dir("fail-on-out");
        write_file(
            &repo_dir,
            "secrets/id_rsa",
            "-----BEGIN RSA PRIVATE KEY-----\nTEST\n-----END RSA PRIVATE KEY-----\n",
        );

        let args = vec![
            "pqc-scan".to_string(),
            "scan".to_string(),
            repo_dir.display().to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--out-dir".to_string(),
            out_dir.display().to_string(),
            "--rules-dir".to_string(),
            rules_dir().display().to_string(),
            "--fail-on".to_string(),
            "critical".to_string(),
            "--threads".to_string(),
            "1".to_string(),
        ];

        let (result, _stdout, stderr) = run_cli_test(args);

        assert_eq!(
            result.expect("scan succeeds with fail-on exit"),
            ExitStatus::FailThreshold
        );
        assert!(
            out_dir.join("report.json").exists(),
            "report.json should exist"
        );
        assert!(stderr.contains("fail-on threshold reached: critical"));

        fs::remove_dir_all(repo_dir).expect("cleanup repo");
        fs::remove_dir_all(out_dir).expect("cleanup out");
    }

    #[test]
    fn rules_list_command_prints_summary_and_rules() {
        let args = vec![
            "pqc-scan".to_string(),
            "rules".to_string(),
            "list".to_string(),
            "--rules-dir".to_string(),
            rules_dir().display().to_string(),
        ];

        let (result, stdout, stderr) = run_cli_test(args);

        assert_eq!(result.expect("rules list succeeds"), ExitStatus::Success);
        assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
        assert!(stdout.contains("rules loaded:"));
        assert!(stdout.contains("API_BC_RSA_ENGINE | kind=Regex"));
    }

    #[test]
    fn scan_command_errors_for_missing_rules_dir() {
        let repo_dir = temp_dir("missing-rules-repo");
        write_file(&repo_dir, "src/app.js", "const alg = 'RS256';\n");
        let missing_rules_dir = repo_dir.join("missing-rules");

        let args = vec![
            "pqc-scan".to_string(),
            "scan".to_string(),
            repo_dir.display().to_string(),
            "--rules-dir".to_string(),
            missing_rules_dir.display().to_string(),
        ];

        let (result, _stdout, _stderr) = run_cli_test(args);
        let err = result.expect_err("missing rules dir should fail");
        assert!(err.to_string().contains("no rules found under"));

        fs::remove_dir_all(repo_dir).expect("cleanup repo");
    }
}
