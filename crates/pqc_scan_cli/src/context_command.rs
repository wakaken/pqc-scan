use clap::Args;
use pqc_scan_core::migration::{
    generate_context, ClaimAnswer, ClaimValue, ContextClaims, ContextSelection, Error, InputFormat,
    Provenance, SavedEvidence,
};
use std::{io::Write, path::PathBuf};

#[derive(Debug, Args)]
pub(crate) struct ContextArgs {
    input: PathBuf,
    #[arg(long)]
    input_format: String,
    /// Show one-based target numbers, roles and locations without writing context.
    #[arg(long)]
    list_targets: bool,
    /// Select a displayed target number. Repeat for an explicitly reviewed group.
    #[arg(long)]
    target: Vec<usize>,
    #[arg(long)]
    out: Option<PathBuf>,
    #[arg(long)]
    as_of: Option<String>,
    #[arg(long, default_value = "organization")]
    organization: String,
    #[arg(long, default_value = "system-1")]
    system: String,
    /// Known claim as FIELD=JSON_VALUE; strings require JSON quotes.
    #[arg(long = "set")]
    values: Vec<String>,
    /// Explicit unknown claim field; omitted fields are not answered.
    #[arg(long)]
    unknown: Vec<String>,
    #[arg(long)]
    actor: Option<String>,
    #[arg(long)]
    asserted_at: Option<String>,
    #[arg(long)]
    valid_until: Option<String>,
}

pub(crate) fn run(args: ContextArgs, stdout: &mut impl Write) -> anyhow::Result<()> {
    let evidence = SavedEvidence::read(&args.input, args.input_format.parse::<InputFormat>()?)?;
    if args.list_targets {
        if args.out.is_some()
            || !args.target.is_empty()
            || !args.values.is_empty()
            || !args.unknown.is_empty()
        {
            return Err(Error::InvalidInput.into());
        }
        for (i, target) in evidence.targets().iter().enumerate() {
            let location = target
                .source_ref
                .path
                .as_deref()
                .or(target.source_ref.detail_path.as_deref())
                .unwrap_or("(saved record)");
            writeln!(
                stdout,
                "{}\t{}\t{}\t{}{}\t{}",
                i + 1,
                target.technical.algorithm.identifier,
                target.source_ref.role,
                serde_json::to_string(location)?,
                target
                    .source_ref
                    .line
                    .map(|n| format!(":{n}"))
                    .unwrap_or_default(),
                if target.is_bindable() {
                    "selectable"
                } else {
                    "ambiguous; cannot bind"
                }
            )?;
        }
        return Ok(());
    }
    let out = args.out.ok_or(Error::InvalidInput)?;
    let as_of = args.as_of.ok_or(Error::InvalidTime)?;
    let mut answers = Vec::new();
    for value in &args.values {
        if value.len() > 1024 * 1024 {
            return Err(Error::ResourceLimit.into());
        }
        let (field, value) = value.split_once('=').ok_or(Error::InvalidInput)?;
        answers.push(ClaimAnswer {
            field: field.to_owned(),
            value: Some(ClaimValue::from_bytes(value.as_bytes())?.into_value()),
        });
    }
    answers.extend(
        args.unknown
            .into_iter()
            .map(|field| ClaimAnswer { field, value: None }),
    );
    let claims = if answers.is_empty() {
        if args.actor.is_some() || args.asserted_at.is_some() || args.valid_until.is_some() {
            return Err(Error::InvalidInput.into());
        }
        ContextClaims::default()
    } else {
        if args.target.is_empty() {
            return Err(Error::InvalidContextBinding.into());
        }
        ContextClaims::from_answers(
            &answers,
            Provenance::operator(
                args.actor.as_deref().ok_or(Error::InvalidInput)?,
                args.asserted_at.as_deref().ok_or(Error::InvalidTime)?,
                args.valid_until.as_deref(),
            )?,
        )?
    };
    let selections = args
        .target
        .into_iter()
        .map(|n| {
            Ok(ContextSelection {
                target_index: n.checked_sub(1).ok_or(Error::InvalidReference)?,
                system_id: args.system.clone(),
                claims: claims.clone(),
            })
        })
        .collect::<pqc_scan_core::migration::Result<Vec<_>>>()?;
    let bytes = generate_context(&evidence, &args.organization, &selections, &as_of)?;
    evidence.validate_output_paths(None, std::slice::from_ref(&out))?;
    pqc_scan_core::safe_output::create_dir_all(
        out.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(std::path::Path::new(".")),
    )
    .map_err(|_| Error::UnsafeOutput)?;
    pqc_scan_core::safe_output::write(&out, bytes).map_err(|_| Error::UnsafeOutput)?;
    writeln!(
        stdout,
        "written migration-context v1; {} explicitly selected targets",
        selections.len()
    )?;
    Ok(())
}
