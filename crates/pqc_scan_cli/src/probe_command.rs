//! Only explicitly selected, feature-enabled endpoint operations can reach this module.
use anyhow::Result;
use clap::{Args, Subcommand};
#[cfg(feature = "probe-tls")]
use pqc_scan_core::probe::host_name;
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome, ProbeReport, ProbeTarget},
    target_policy::{Cancellation, ProbeBudget, ProbeLimits, TargetPolicy, TargetRequest},
};
use std::{io::Write, path::PathBuf};

#[derive(Debug, Subcommand)]
pub(crate) enum ProbeCommand {
    #[cfg(feature = "probe-tls")]
    Tls(TlsArgs),
}
#[derive(Debug, Args)]
pub(crate) struct CommonArgs {
    host: String,
    #[arg(long,value_parser=clap::value_parser!(u16).range(1..))]
    port: u16,
    #[arg(long)]
    allow_cidr: Vec<String>,
    #[arg(long)]
    out_dir: Option<PathBuf>,
    #[arg(long,value_enum,default_value_t=crate::LanguageArg::En)]
    lang: crate::LanguageArg,
}
#[cfg(feature = "probe-tls")]
#[derive(Debug, Args)]
pub(crate) struct TlsArgs {
    #[command(flatten)]
    common: CommonArgs,
    #[arg(long)]
    server_name: Option<String>,
    #[arg(long)]
    ca_file: Option<PathBuf>,
}
fn cancellation() -> Result<Cancellation> {
    let cancellation = Cancellation::new();
    let signal = cancellation.clone();
    ctrlc::set_handler(move || signal.cancel())
        .map_err(|_| anyhow::anyhow!("failed to install probe cancellation handler"))?;
    Ok(cancellation)
}
fn write<T: serde::Serialize>(
    report: ProbeReport<T>,
    args: &CommonArgs,
    default_dir: &str,
    stdout: &mut impl Write,
) -> Result<ProbeOutcome> {
    report.validate()?;
    let out_dir = args
        .out_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from(default_dir));
    for path in pqc_scan_report::write_probe_reports(&report, &out_dir, args.lang.into())? {
        writeln!(stdout, "written {}", path.display())?;
    }
    let code = serde_json::to_value(report.outcome)?;
    writeln!(stdout, "outcome: {}", code.as_str().unwrap_or("failed"))?;
    Ok(report.outcome)
}
pub(crate) fn run(command: ProbeCommand, stdout: &mut impl Write) -> Result<ProbeOutcome> {
    match command {
        #[cfg(feature = "probe-tls")]
        ProbeCommand::Tls(args) => {
            use pqc_scan_probe_tls::{TlsOptions, TlsProbe};
            let request = TargetRequest::new(&args.common.host, args.common.port)?;
            let policy = TargetPolicy::new(&args.common.allow_cidr)?;
            let options = TlsOptions {
                verification_name: args
                    .server_name
                    .unwrap_or_else(|| host_name(request.host())),
                ca_file: args.ca_file,
            };
            let probe = TlsProbe::new(&options)?;
            let unknown = ProbeTarget::requested(&request);
            let limits = ProbeLimits::default();
            let budget = ProbeBudget::new(limits.clone(), cancellation()?)?;
            let target = policy.resolve(
                request,
                &mut crate::probe_resolver::ProcessResolver::new()?,
                &budget,
            );
            let report = match target {
                Ok(target) => probe.run(target, budget),
                Err(error) => probe.failed(unknown, &limits, ProbeDiagnostic::from(error)),
            };
            write(report, &args.common, "./pqc-probe/tls", stdout)
        }
    }
}
