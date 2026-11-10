use std::{
    ffi::OsString,
    io::{self, Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use pqc_scan_core::{
    pipeline, scan_repository_detailed, scan_repository_native, NativeOptions, ScanConfig,
};
use pqc_scan_detectors::default_detectors;
use pqc_scan_report::{
    write_assessment_json, write_cyclonedx_17, write_diagnostics, write_document_reports,
    write_inventory_json, ReportFormat, ReportLanguage,
};
use pqc_scan_rules::{RuleSet, Severity};
mod diff_command;
#[cfg(feature = "probe-tls")]
mod probe_command;

#[cfg(feature = "probe-transport")]
mod probe_resolver;

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
    #[cfg(feature = "probe-tls")]
    Probe {
        #[command(subcommand)]
        command: probe_command::ProbeCommand,
    },
    #[cfg(feature = "probe-transport")]
    #[command(name = "__resolve-host", hide = true)]
    ResolveHost {
        host: String,
    },
    Scan(ScanArgs),
    /// Compare compatible inventories within one explicit repository scope.
    Diff(DiffArgs),
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
}

#[derive(Debug, Parser)]
struct DiffArgs {
    before: PathBuf,
    after: PathBuf,
    #[arg(long)]
    repository_scope: String,
    #[arg(long)]
    out_dir: PathBuf,
}

#[derive(Debug, Parser)]
struct ScanArgs {
    path: PathBuf,

    #[arg(long, value_enum, default_value_t = FormatArg::All)]
    format: FormatArg,

    /// Language of HTML and Markdown reports; machine outputs remain unchanged.
    #[arg(long, value_enum, default_value_t = LanguageArg::En)]
    lang: LanguageArg,

    /// Add separate CycloneDX 1.7 CBOM and dependency SBOM files.
    #[arg(long)]
    cyclonedx_17: bool,

    #[arg(long, default_value = "./pqc-report")]
    out_dir: PathBuf,

    #[arg(long, default_value = "./rules")]
    rules_dir: PathBuf,

    #[arg(long, value_enum)]
    fail_on: Option<FailOnArg>,

    #[arg(long, default_value_t = 0usize)]
    threads: usize,

    /// Write inspection coverage and safe failure reasons to diagnostics.json.
    #[arg(long)]
    diagnostics: bool,

    /// Write the Cryptographic Inventory and Evidence Report to scan.inventory.v1.json.
    #[arg(long)]
    inventory_json: bool,

    /// Explicit offline advisory policy; writes scan.assessment.v1.json.
    #[arg(long, value_parser = ["migration-review-v1"], requires = "inventory_json")]
    policy_profile: Option<String>,

    /// Bounded v1 enum/boolean context JSON; support is operator-declared, not verified.
    #[arg(long, requires = "policy_profile")]
    assessment_context: Option<PathBuf>,

    /// Non-secret project identifier for inventory correspondence keys; never a remote URL.
    #[arg(long, requires = "inventory_json")]
    repository_scope: Option<String>,

    /// Return exit code 3 unless every selected inspection is known to be complete.
    #[arg(long)]
    require_complete: bool,
}

#[derive(Debug, Subcommand)]
enum RulesCommand {
    List(RulesListArgs),
    Show(RulesShowArgs),
}

#[derive(Debug, Parser)]
struct RulesListArgs {
    #[arg(long, default_value = "./rules")]
    rules_dir: PathBuf,
}

#[derive(Debug, Parser)]
struct RulesShowArgs {
    rule_id: String,

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum LanguageArg {
    En,
    Ja,
}

impl From<LanguageArg> for ReportLanguage {
    fn from(value: LanguageArg) -> Self {
        match value {
            LanguageArg::En => Self::En,
            LanguageArg::Ja => Self::Ja,
        }
    }
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
    IncompleteInspection,
}

impl ExitStatus {
    fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::FailThreshold => 2,
            Self::IncompleteInspection => 3,
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
        #[cfg(feature = "probe-tls")]
        Commands::Probe { command } => Ok(
            if probe_command::run(command, stdout)? == pqc_scan_core::probe::ProbeOutcome::Completed
            {
                ExitStatus::Success
            } else {
                ExitStatus::IncompleteInspection
            },
        ),
        #[cfg(feature = "probe-transport")]
        Commands::ResolveHost { host } => {
            probe_resolver::resolve_helper(&host, stdout)?;
            Ok(ExitStatus::Success)
        }
        Commands::Scan(args) => scan_cmd(args, stdout, stderr),
        Commands::Diff(args) => {
            diff_command::run(
                &args.before,
                &args.after,
                &args.repository_scope,
                &args.out_dir,
                stdout,
            )?;
            Ok(ExitStatus::Success)
        }
        Commands::Rules { command } => match command {
            RulesCommand::List(args) => list_rules_cmd(args, stdout),
            RulesCommand::Show(args) => show_rule_cmd(args, stdout),
        },
    }
}

fn scan_cmd<WOut: Write, WErr: Write>(
    args: ScanArgs,
    stdout: &mut WOut,
    stderr: &mut WErr,
) -> Result<ExitStatus> {
    let assessment_context = if let Some(path) = &args.assessment_context {
        let file = std::fs::File::open(path)
            .map_err(|_| anyhow::anyhow!("failed to read assessment context"))?;
        let mut bytes = Vec::new();
        file.take(16385)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("failed to read assessment context"))?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| anyhow::anyhow!("invalid assessment context encoding"))?;
        pqc_scan_core::policy::AssessmentContext::from_json(text)?
    } else {
        pqc_scan_core::policy::AssessmentContext::default()
    };
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
    let document = if args.inventory_json || args.cyclonedx_17 {
        let options = NativeOptions::new(args.repository_scope.as_deref())?;
        scan_repository_native(&config, &rules, &detectors, &options)?
    } else {
        scan_repository_detailed(&config, &rules, &detectors)?
    };
    let assessment = if let Some(profile) = &args.policy_profile {
        Some(pqc_scan_core::policy::evaluate(
            document
                .native()
                .ok_or_else(|| anyhow::anyhow!("policy requires inventory capture"))?,
            &assessment_context,
            profile,
        )?)
    } else {
        None
    };
    let result = document.legacy();

    let mut outputs = write_document_reports(
        &document,
        assessment.as_ref(),
        args.diagnostics,
        &args.out_dir,
        args.format.into(),
        args.lang.into(),
    )?;
    if args.cyclonedx_17 {
        outputs.extend(write_cyclonedx_17(&document, &args.out_dir)?);
    }
    if args.diagnostics {
        outputs.push(write_diagnostics(&document, &args.out_dir)?);
    }
    if args.inventory_json {
        outputs.push(write_inventory_json(&document, &args.out_dir)?);
    }
    if let Some(report) = &assessment {
        outputs.push(write_assessment_json(report, &args.out_dir)?);
    }
    for out in outputs {
        writeln!(stdout, "written {}", out.display())?;
    }

    writeln!(stdout, "total findings: {}", result.summary.total_findings)?;
    writeln!(stdout, "scanned files: {}", result.summary.scanned_files)?;
    writeln!(stdout, "skipped files: {}", result.summary.skipped_files)?;

    if !document.coverage().is_complete() {
        writeln!(stderr,
            "warning: inspection completeness is {}; use --diagnostics for details. No findings does not imply complete inspection.",
            document.coverage().status().as_str()
        )?;
        if args.require_complete {
            writeln!(
                stderr,
                "require-complete: inspection is not known to be complete"
            )?;
            return Ok(ExitStatus::IncompleteInspection);
        }
    }

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

fn show_rule_cmd<WOut: Write>(args: RulesShowArgs, stdout: &mut WOut) -> Result<ExitStatus> {
    let rules = RuleSet::load_from_dir(&args.rules_dir)
        .with_context(|| format!("failed to load rules from {}", args.rules_dir.display()))?;

    let rule = rules.get(&args.rule_id).with_context(|| {
        format!(
            "rule '{}' not found under {}",
            args.rule_id,
            args.rules_dir.display()
        )
    })?;

    writeln!(stdout, "id: {}", rule.id)?;
    writeln!(stdout, "kind: {:?}", rule.kind)?;
    writeln!(stdout, "category: {}", rule.category)?;
    writeln!(stdout, "severity: {}", rule.severity)?;
    writeln!(stdout, "risk: {}", rule.risk)?;
    writeln!(stdout, "confidence: {:.2}", rule.confidence)?;
    writeln!(stdout, "scope: {}", rule.scope)?;
    writeln!(stdout, "migration_hint: {}", rule.migration_hint)?;
    writeln!(stdout, "pattern: {}", rule.pattern)?;
    if let Some(description) = &rule.description {
        writeln!(stdout, "description: {}", description)?;
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
    fn diagnostics_opt_in_exports_complete_coverage_without_changing_legacy_json() {
        let repo = temp_dir("diagnostics-input");
        let plain = temp_dir("diagnostics-plain");
        let detailed = temp_dir("diagnostics-extra");
        write_file(&repo, "app.js", "const algorithm = 'RS256';\n");
        let arguments = |out: &Path| {
            vec![
                "pqc-scan".into(),
                "scan".into(),
                repo.display().to_string(),
                "--rules-dir".into(),
                rules_dir().display().to_string(),
                "--out-dir".into(),
                out.display().to_string(),
                "--threads".into(),
                "1".into(),
            ]
        };
        let (result, stdout, stderr) = run_cli_test(arguments(&plain));
        assert_eq!(result.unwrap(), ExitStatus::Success);
        assert_eq!(stdout.matches("written ").count(), 6);
        assert!(!plain.join("diagnostics.json").exists());
        assert!(stderr.is_empty(), "{stderr}");
        let mut args = arguments(&detailed);
        args.push("--diagnostics".into());
        let (result, stdout, _) = run_cli_test(args);
        assert_eq!(result.unwrap(), ExitStatus::Success);
        assert_eq!(stdout.matches("written ").count(), 7);
        let diagnostics: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(detailed.join("diagnostics.json")).unwrap())
                .unwrap();
        assert_eq!(diagnostics["schema_version"], "1");
        assert_eq!(diagnostics["coverage"]["status"], "complete_within_scope");
        assert_eq!(diagnostics["coverage"]["selected_files"], 1);
        assert_eq!(diagnostics["coverage"]["unknown_files"], 0);
        let mut unreported: Vec<_> = diagnostics["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| {
                assert_eq!(d["code"], "detector_not_reported");
                d["detector"].as_str().unwrap()
            })
            .collect();
        unreported.sort_unstable();
        assert!(unreported.is_empty());
        assert!(diagnostics.get("findings").is_none());
        let read = |out: &Path, file: &str| {
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(out.join(file)).unwrap())
                .unwrap()
        };
        let mut before = read(&plain, "report.json");
        let mut after = read(&detailed, "report.json");
        before.as_object_mut().unwrap().remove("generated_at");
        after.as_object_mut().unwrap().remove("generated_at");
        assert_eq!(before, after);
        for file in ["cbom.json", "dependency-sbom.json", "report.sarif"] {
            assert_eq!(read(&plain, file), read(&detailed, file), "{file}");
        }
        for path in [repo, plain, detailed] {
            fs::remove_dir_all(path).unwrap();
        }
    }

    #[test]
    fn require_complete_returns_three_before_finding_threshold_and_keeps_reports() {
        let repo = temp_dir("strict-input");
        let out = temp_dir("strict-output");
        write_file(&repo, "app.js", "const algorithm = 'RS256';\n");
        write_file(&repo, "Gemfile", "gem ENV['DYNAMIC_DEPENDENCY']\n");
        let (result, _, stderr) = run_cli_test(vec![
            "pqc-scan".into(),
            "scan".into(),
            repo.display().to_string(),
            "--rules-dir".into(),
            rules_dir().display().to_string(),
            "--out-dir".into(),
            out.display().to_string(),
            "--require-complete".into(),
            "--diagnostics".into(),
            "--fail-on".into(),
            "high".into(),
        ]);
        assert_eq!(result.unwrap().code(), 3);
        assert!(stderr.contains("require-complete"));
        assert!(out.join("report.json").exists());
        assert!(out.join("diagnostics.json").exists());
        fs::remove_dir_all(repo).unwrap();
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn require_complete_accepts_empty_scope_and_operational_errors_stay_errors() {
        let repo = temp_dir("strict-empty");
        let out = temp_dir("strict-empty-out");
        let args = |root: &Path, output: &Path| {
            vec![
                "pqc-scan".into(),
                "scan".into(),
                root.display().to_string(),
                "--rules-dir".into(),
                rules_dir().display().to_string(),
                "--out-dir".into(),
                output.display().to_string(),
                "--require-complete".into(),
                "--fail-on".into(),
                "high".into(),
            ]
        };
        let (result, _, stderr) = run_cli_test(args(&repo, &out));
        assert_eq!(result.unwrap().code(), 0);
        assert!(stderr.is_empty());
        let (result, _, _) = run_cli_test(args(&repo.join("missing"), &out));
        assert!(result.is_err());
        let blocked = out.join("ordinary-file");
        fs::write(&blocked, "not a directory").unwrap();
        write_file(&repo, "hit.js", "RS256");
        let (result, _, _) = run_cli_test(args(&repo, &blocked));
        assert!(
            result.is_err(),
            "output errors must precede completeness/threshold gates"
        );
        fs::remove_dir_all(repo).unwrap();
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn require_complete_rejects_broken_ignore_policy_even_with_no_selected_files() {
        let repo = temp_dir("strict-ignore");
        let out = temp_dir("strict-ignore-out");
        write_file(&repo, ".gitignore", "[z-a]SYNTHETIC_IGNORE_CONTENT\n*\n");
        write_file(&repo, "ignored.txt", "ignored");
        let (result, _, stderr) = run_cli_test(vec![
            "pqc-scan".into(),
            "scan".into(),
            repo.display().to_string(),
            "--rules-dir".into(),
            rules_dir().display().to_string(),
            "--out-dir".into(),
            out.display().to_string(),
            "--diagnostics".into(),
            "--require-complete".into(),
        ]);
        assert_eq!(result.unwrap().code(), 3);
        assert!(!stderr.contains("SYNTHETIC_IGNORE_CONTENT"));
        let text = fs::read_to_string(out.join("diagnostics.json")).unwrap();
        assert!(!text.contains("SYNTHETIC_IGNORE_CONTENT"));
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(json["coverage"]["selected_files"], 0);
        assert_eq!(json["coverage"]["enumeration_failed_entries"], 1);
        assert_eq!(json["coverage"]["status"], "failed");
        fs::remove_dir_all(repo).unwrap();
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn scan_japanese_language_localizes_human_reports() {
        let out = temp_dir("japanese-cli");
        let (result, _, stderr) = run_cli_test(vec![
            "pqc-scan".into(),
            "scan".into(),
            workspace_root()
                .join("examples/pqc-readiness")
                .display()
                .to_string(),
            "--out-dir".into(),
            out.display().to_string(),
            "--rules-dir".into(),
            rules_dir().display().to_string(),
            "--lang".into(),
            "ja".into(),
        ]);
        assert_eq!(result.unwrap(), ExitStatus::Success);
        assert!(stderr.is_empty(), "{stderr}");
        let html = fs::read_to_string(out.join("report.html")).unwrap();
        assert!(html.contains("耐量子暗号への移行調査レポート"));
        assert!(html.contains("対応完了の保証ではありません"));
        assert!(html.contains("Goのハイブリッド鍵共有を無効にする設定"));
        assert!(html.contains("example.py"));
        let md = fs::read_to_string(out.join("report.md")).unwrap();
        assert!(md.contains("検出箇所"));
        assert!(md.contains("tlsmlkem=0"));
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
        assert_eq!(json["summary"]["total_findings"], 10);
        assert!(json["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["description"].as_str().unwrap().starts_with("Go hybrid")));
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn report_language_defaults_to_english_and_rejects_unknown_values() {
        assert!(Cli::try_parse_from(["pqc-scan", "scan", ".", "--lang", "fr"]).is_err());
        let cli = Cli::try_parse_from(["pqc-scan", "scan", "."]).unwrap();
        let Commands::Scan(args) = cli.command else {
            panic!("expected scan")
        };
        assert_eq!(args.lang, LanguageArg::En);
    }

    #[test]
    fn pqc_fixtures_preserve_report_contract_and_do_not_trigger_high_gate() {
        let out = temp_dir("pqc-reports");
        let (result, stdout, stderr) = run_cli_test(vec![
            "pqc-scan".into(),
            "scan".into(),
            workspace_root()
                .join("examples/pqc-readiness")
                .display()
                .to_string(),
            "--format".into(),
            "all".into(),
            "--out-dir".into(),
            out.display().to_string(),
            "--rules-dir".into(),
            rules_dir().display().to_string(),
            "--fail-on".into(),
            "high".into(),
        ]);
        assert_eq!(result.unwrap(), ExitStatus::Success);
        assert!(stderr.is_empty(), "{stderr}");
        assert_eq!(stdout.matches("written ").count(), 6);
        let report: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
        let hits = report["findings"].as_array().unwrap();
        assert_eq!(hits.len(), 10);
        assert_eq!(
            hits.iter()
                .filter(|hit| hit["risk"] == "quantum-safe" && hit["severity"] == "info")
                .count(),
            7
        );
        assert!(hits
            .iter()
            .all(|hit| hit["location"]["file"] != "negative.js"));
        assert!(hits
            .iter()
            .filter(|hit| hit["risk"] == "quantum-safe")
            .all(|hit| hit.get("recommended_actions").is_none()));
        let cbom: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(out.join("cbom.json")).unwrap()).unwrap();
        assert_eq!(cbom, report["cbom"]);
        assert_eq!(cbom.as_array().unwrap().len(), 10);
        for file in ["report.html", "report.md", "report.sarif"] {
            let text = fs::read_to_string(out.join(file)).unwrap();
            assert!(text.contains("PQC_PYTHON_API"));
            assert!(text.contains("example.py"));
            if file != "report.sarif" {
                assert!(text.contains("quantum-safe"));
            }
        }
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn synthetic_scan_acceptance_checks_all_reports() {
        let repo = temp_dir("acceptance");
        let out = temp_dir("acceptance-out");
        write_file(&repo, "positive.js", "const alg = 'RS256';\n");
        write_file(&repo, "negative.js", "// RS256\nconst RS256_helper = 1;\n");
        write_file(&repo, ".gitignore", "ignored.js\n");
        write_file(&repo, "ignored.js", "const alg = 'RS256';\n");
        write_file(
            &repo,
            "synthetic.key",
            "-----BEGIN RSA PRIVATE KEY-----\nSYNTHETIC_ACCEPTANCE_PAYLOAD\n-----END RSA PRIVATE KEY-----\n",
        );
        let (result, stdout, stderr) = run_cli_test(vec![
            "pqc-scan".into(),
            "scan".into(),
            repo.display().to_string(),
            "--format".into(),
            "all".into(),
            "--out-dir".into(),
            out.display().to_string(),
            "--rules-dir".into(),
            rules_dir().display().to_string(),
        ]);
        assert_eq!(result.expect("acceptance scan"), ExitStatus::Success);
        assert!(stderr.is_empty(), "{stderr}");
        assert_eq!(stdout.matches("written ").count(), 6);
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
        let findings = json["findings"].as_array().unwrap();
        let hit = findings
            .iter()
            .find(|f| f["rule_id"] == "JWT_RS256")
            .unwrap();
        assert_eq!(hit["location"]["file"], "positive.js");
        assert_eq!(hit["location"]["line"], 1);
        assert!(!hit["recommended_actions"].as_array().unwrap().is_empty());
        assert!(hit["source_snippet"]["lines"]
            .as_array()
            .unwrap()
            .iter()
            .any(|line| line["highlighted"] == true && line["line"] == 1));
        assert!(findings.iter().all(
            |f| f["location"]["file"] != "negative.js" && f["location"]["file"] != "ignored.js"
        ));
        assert!(findings
            .iter()
            .any(|f| f["evidence"]["type"] == "private_key"));
        for name in [
            "report.json",
            "report.html",
            "report.md",
            "report.sarif",
            "cbom.json",
            "dependency-sbom.json",
        ] {
            let text = fs::read_to_string(out.join(name)).unwrap();
            assert!(
                !text.contains("SYNTHETIC_ACCEPTANCE_PAYLOAD"),
                "unmasked payload in {name}"
            );
            if name != "dependency-sbom.json" {
                assert!(text.contains("positive.js"), "missing location in {name}");
            }
        }
        for (file, field) in [
            ("cbom.json", "cbom"),
            ("dependency-sbom.json", "dependency_sbom"),
        ] {
            let inventory: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(out.join(file)).unwrap()).unwrap();
            assert_eq!(inventory, json[field]);
        }
        let sarif: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(out.join("report.sarif")).unwrap()).unwrap();
        assert_eq!(sarif["version"], "2.1.0");
        assert_eq!(
            sarif["runs"][0]["results"].as_array().unwrap().len(),
            findings.len()
        );
        fs::remove_dir_all(repo).unwrap();
        fs::remove_dir_all(out).unwrap();
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
        assert!(stderr.is_empty(), "{stderr}");
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
    fn rules_show_command_prints_single_rule_details() {
        let args = vec![
            "pqc-scan".to_string(),
            "rules".to_string(),
            "show".to_string(),
            "JWT_RS256".to_string(),
            "--rules-dir".to_string(),
            rules_dir().display().to_string(),
        ];

        let (result, stdout, stderr) = run_cli_test(args);

        assert_eq!(result.expect("rules show succeeds"), ExitStatus::Success);
        assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
        assert!(stdout.contains("id: JWT_RS256"));
        assert!(stdout.contains("kind: Regex"));
        assert!(stdout.contains("category: JWT"));
        assert!(stdout.contains("severity: high"));
        assert!(stdout.contains("risk: quantum-vulnerable"));
        assert!(stdout.contains("scope: code"));
        assert!(stdout.contains("pattern: \\bRS256\\b"));
    }

    #[test]
    fn rules_show_command_errors_for_missing_rule_id() {
        let args = vec![
            "pqc-scan".to_string(),
            "rules".to_string(),
            "show".to_string(),
            "MISSING_RULE".to_string(),
            "--rules-dir".to_string(),
            rules_dir().display().to_string(),
        ];

        let (result, _stdout, _stderr) = run_cli_test(args);
        let err = result.expect_err("missing rule should fail");
        assert!(err.to_string().contains("rule 'MISSING_RULE' not found"));
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
        assert!(err.to_string().contains("failed to load rules"));

        fs::remove_dir_all(repo_dir).expect("cleanup repo");
    }
}
