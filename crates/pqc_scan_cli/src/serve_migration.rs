//! Opt-in, session-bound adapters. All decisions remain in the core evaluator.
use super::*;
use pqc_scan_core::migration::{
    self, ClaimAnswer, ClaimValue, ContextClaims, ContextSelection, InputFormat, MigrationContext,
    Provenance, SavedEvidence,
};
use serde::Deserialize;
use std::io::Read;
const INPUT_LIMIT: usize = 32 * 1024 * 1024;
const PREVIEW_LIMIT: usize = 3 * 1024 * 1024;
const RESPONSE_LIMIT: usize = 16 * 1024 * 1024;
#[derive(Clone)]
pub(super) struct Draft {
    token: String,
    input_sha256: String,
    as_of: String,
    context: Option<Vec<u8>>,
}
#[derive(Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
enum Answer {
    Known { field: String, value: ClaimValue },
    Unknown { field: String },
}
#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
enum Preview {
    WithoutContext {
        input_sha256: String,
        as_of: String,
    },
    Generate {
        input_sha256: String,
        as_of: String,
        organization_id: String,
        system_id: String,
        selected: Vec<usize>,
        answers: Vec<Answer>,
        actor: Option<String>,
        asserted_at: Option<String>,
        valid_until: Option<String>,
    },
    ImportContext {
        input_sha256: String,
        as_of: String,
        context: String,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Confirmation {
    token: String,
    language: Option<String>,
}
fn query(query: &str) -> Result<(u64, usize)> {
    let mut job = None;
    let mut page = None;
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').context("invalid query")?;
        match k {
            "job" if job.is_none() => job = Some(v.parse::<u64>()?),
            "page" if page.is_none() => page = Some(v.parse::<usize>()?),
            _ => bail!("invalid query"),
        }
    }
    Ok((job.context("missing job")?, page.unwrap_or(0)))
}
fn body(request: &mut Request, limit: usize) -> Result<Vec<u8>> {
    if header(request, "Content-Type").and_then(|v| v.split(';').next()) != Some("application/json")
    {
        bail!("invalid content type");
    }
    let length = header(request, "Content-Length")
        .context("length required")?
        .parse::<usize>()?;
    if request.body_length() != Some(length) {
        bail!("invalid length");
    }
    if length > limit {
        return Err(migration::Error::ResourceLimit.into());
    }
    let mut bytes = Vec::with_capacity(length);
    request
        .as_reader()
        .take((limit + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(migration::Error::ResourceLimit.into());
    }
    if bytes.len() != length {
        bail!("invalid body length");
    }
    Ok(bytes)
}
fn evidence(state: &State, id: u64) -> Result<SavedEvidence> {
    if !state
        .jobs
        .lock()
        .unwrap()
        .entries
        .get(&id)
        .is_some_and(|j| j["status"] == "ready" && j["migration_input"] == true)
    {
        bail!("unavailable job");
    }
    Ok(SavedEvidence::read(
        &state
            .output
            .join(id.to_string())
            .join("scan.inventory.v1.json"),
        InputFormat::InventoryV1,
    )?)
}
fn display(index: usize, target: &migration::Target) -> Value {
    let support_fields = [
        ("unspecified", None),
        ("signature", Some("signature")),
        ("jwt", Some("jwt")),
        ("pki", Some("pki")),
        ("key_establishment", Some("key_establishment")),
        ("encryption", Some("encryption")),
        ("hashing", Some("hashing")),
    ]
    .into_iter()
    .map(|(key, usage)| {
        (
            key.to_owned(),
            json!(migration::input_support_fields(target, usage).expect("fixed supported usage")),
        )
    })
    .collect::<serde_json::Map<_, _>>();
    json!({"number":index+1,"id":target.id,"algorithm":target.technical.algorithm.identifier,"role":target.source_ref.role,"role_ja":pqc_scan_report::migration_role_label(&target.source_ref.role,pqc_scan_report::ReportLanguage::Ja),"role_en":pqc_scan_report::migration_role_label(&target.source_ref.role,pqc_scan_report::ReportLanguage::En),"path":target.source_ref.path,"line":target.source_ref.line,"selectable":target.is_bindable(),"support_fields":support_fields})
}

fn targets(state: &State, id: u64, page: usize) -> Result<Value> {
    let e = evidence(state, id)?;
    let start = page.checked_mul(100).context("invalid page")?;
    if start > e.targets().len() {
        bail!("invalid page");
    }
    let rows = e
        .targets()
        .iter()
        .enumerate()
        .skip(start)
        .take(100)
        .map(|(i, t)| display(i, t))
        .collect::<Vec<_>>();
    Ok(json!({"input":e.input(),"total":e.targets().len(),"page":page,"targets":rows}))
}
fn import(state: &State, bytes: Vec<u8>) -> Result<Value> {
    SavedEvidence::from_bytes(&bytes, InputFormat::InventoryV1)?;
    let mut jobs = state.jobs.lock().unwrap();
    if jobs.active || jobs.next >= 20 {
        bail!("session busy or full");
    }
    jobs.next += 1;
    let id = jobs.next;
    let dir = state.output.join(id.to_string());
    pqc_scan_core::safe_output::create_dir_all(&dir)?;
    pqc_scan_core::safe_output::write(&dir.join("scan.inventory.v1.json"), bytes)?;
    let job = json!({"id":id,"path":"Saved inventory","kind":"saved","status":"ready","migration_input":true});
    jobs.entries.insert(id, job);
    Ok(json!({"id":id}))
}
fn preview(state: &State, id: u64, request: Preview) -> Result<Value> {
    let e = evidence(state, id)?;
    let (sha, as_of, bytes) = match request {
        Preview::WithoutContext {
            input_sha256,
            as_of,
        } => {
            migration::generate_context(&e, "organization", &[], &as_of)?;
            (input_sha256, as_of, None)
        }
        Preview::Generate {
            input_sha256,
            as_of,
            organization_id,
            system_id,
            selected,
            answers,
            actor,
            asserted_at,
            valid_until,
        } => {
            if selected.len() > 4096 || answers.len() > 18 {
                return Err(migration::Error::ResourceLimit.into());
            }
            let claims = if answers.is_empty() {
                if actor.is_some() || asserted_at.is_some() || valid_until.is_some() {
                    bail!("provenance without answers");
                }
                ContextClaims::default()
            } else {
                if selected.is_empty() {
                    bail!("unbound answers");
                }
                let answers = answers
                    .into_iter()
                    .map(|a| match a {
                        Answer::Known { field, value } => ClaimAnswer {
                            field,
                            value: Some(value.into_value()),
                        },
                        Answer::Unknown { field } => ClaimAnswer { field, value: None },
                    })
                    .collect::<Vec<_>>();
                ContextClaims::from_answers(
                    &answers,
                    Provenance::operator(
                        actor.as_deref().context("actor required")?,
                        asserted_at.as_deref().context("assertion required")?,
                        valid_until.as_deref(),
                    )?,
                )?
            };
            let selections = selected
                .into_iter()
                .map(|n| {
                    Ok(ContextSelection {
                        target_index: n.checked_sub(1).context("invalid index")?,
                        system_id: system_id.clone(),
                        claims: claims.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let bytes = migration::generate_context(&e, &organization_id, &selections, &as_of)?;
            (input_sha256, as_of, Some(bytes))
        }
        Preview::ImportContext {
            input_sha256,
            as_of,
            context,
        } => {
            let c = MigrationContext::from_bytes(context.as_bytes())?;
            c.bind(&e, &as_of)?;
            (input_sha256, as_of, Some(c.to_bytes()?))
        }
    };
    if sha != e.input().sha256 {
        return Err(migration::Error::InputBindingMismatch.into());
    }
    let mut scope = Vec::new();
    let mut resolved = Vec::new();
    let mut purpose_conflicting = false;
    if let Some(bytes) = &bytes {
        let v: Value = serde_json::from_slice(bytes)?;
        let context = MigrationContext::from_bytes(bytes)?;
        let bound = context.bind(&e, &as_of)?;
        let lookup = e
            .targets()
            .iter()
            .enumerate()
            .map(|(i, t)| (t.id.as_str(), (i, t)))
            .collect::<BTreeMap<_, _>>();
        for binding in v["bindings"].as_array().context("invalid context")? {
            let (i, t) = lookup
                .get(binding["target_id"].as_str().context("invalid target")?)
                .context("invalid target")?;
            scope.push(display(*i, t));
            if let Some(target) = bound.target(&t.id) {
                purpose_conflicting |= target.purpose_conflicting;
                for claim in target.claims() {
                    resolved.push(json!({"target_number":i+1,"field":claim.field,"freshness":claim.freshness}));
                }
            }
        }
    }
    let token = random_token()?;
    let as_of = migration::parse_as_of(&as_of)?;
    let context = bytes
        .as_ref()
        .map(|b| String::from_utf8(b.clone()))
        .transpose()?;
    state.drafts.lock().unwrap().insert(
        id,
        Draft {
            token: token.clone(),
            input_sha256: sha.clone(),
            as_of: as_of.clone(),
            context: bytes,
        },
    );
    Ok(
        json!({"token":token,"input_sha256":sha,"as_of":as_of,"scope":scope,"context":context,"resolved":resolved,"purpose_conflicting":purpose_conflicting}),
    )
}
fn confirm(state: &State, id: u64, confirmation: Confirmation) -> Result<Value> {
    let draft = state
        .drafts
        .lock()
        .unwrap()
        .get(&id)
        .filter(|d| d.token == confirmation.token)
        .cloned()
        .context("draft expired")?;
    let e = evidence(state, id)?;
    if e.input().sha256 != draft.input_sha256 {
        state.drafts.lock().unwrap().remove(&id);
        return Err(migration::Error::InputBindingMismatch.into());
    }
    let context = draft
        .context
        .as_deref()
        .map(MigrationContext::from_bytes)
        .transpose()?;
    let report = migration::evaluate(&e, context.as_ref(), &draft.as_of)?;
    let language = match confirmation.language.as_deref().unwrap_or("ja") {
        "en" => pqc_scan_report::ReportLanguage::En,
        "ja" => pqc_scan_report::ReportLanguage::Ja,
        _ => bail!("invalid language"),
    };
    let dir = state.output.join(id.to_string()).join("migration");
    let context_path = dir.join("migration-context.json");
    e.validate_output_paths(
        context.as_ref(),
        &[
            context_path.clone(),
            dir.join(pqc_scan_report::MIGRATION_JSON),
            dir.join(pqc_scan_report::MIGRATION_HTML),
            dir.join(pqc_scan_report::MIGRATION_MARKDOWN),
        ],
    )?;
    state
        .jobs
        .lock()
        .unwrap()
        .entries
        .get_mut(&id)
        .context("unavailable job")?["migration_ready"] = false.into();
    pqc_scan_report::write_migration_assessment_reports(
        &report,
        &e,
        context.as_ref(),
        &dir,
        language,
    )?;
    if let Some(bytes) = draft.context {
        pqc_scan_core::safe_output::write(&context_path, bytes)?;
    } else if context_path.exists() {
        std::fs::remove_file(&context_path)?;
    }
    state
        .jobs
        .lock()
        .unwrap()
        .entries
        .get_mut(&id)
        .context("unavailable job")?["migration_ready"] = true.into();
    state.drafts.lock().unwrap().remove(&id);
    Ok(
        json!({"report":format!("reports/{id}/migration.assessment.html"),"json":format!("reports/{id}/migration.assessment.v1.json"),"context":context.is_some()}),
    )
}
fn dispatch(request: &mut Request, state: &State, route: &str, query_text: &str) -> Result<Value> {
    if route == "api/migration/import" && request.method() == &Method::Post {
        if !query_text.is_empty() {
            bail!("invalid query");
        }
        return import(state, body(request, INPUT_LIMIT)?);
    }
    let (id, page) = query(query_text)?;
    match (request.method(), route) {
        (&Method::Get, "api/migration/targets") => targets(state, id, page),
        (&Method::Post, "api/migration/preview") => preview(
            state,
            id,
            serde_json::from_slice(&body(request, PREVIEW_LIMIT)?)?,
        ),
        (&Method::Post, "api/migration/assess") => {
            confirm(state, id, serde_json::from_slice(&body(request, 1024)?)?)
        }
        (&Method::Post, "api/migration/cancel") => {
            let c: Confirmation = serde_json::from_slice(&body(request, 1024)?)?;
            let mut drafts = state.drafts.lock().unwrap();
            if drafts.get(&id).is_none_or(|d| d.token != c.token) {
                bail!("draft expired");
            }
            drafts.remove(&id);
            Ok(json!({"cancelled":true}))
        }
        _ => bail!("invalid route"),
    }
}
pub(super) fn handle(mut request: Request, state: &State, route: &str, query: &str) {
    let result = dispatch(&mut request, state, route, query)
        .and_then(|value| Ok(serde_json::to_vec(&value)?));
    match result {
        Ok(bytes) if bytes.len() <= RESPONSE_LIMIT => {
            reply(request, 200, "application/json; charset=utf-8", bytes)
        }
        Ok(_) => json_reply(request, 413, json!({"error":"resource_limit"})),
        Err(error) => {
            let code = error
                .downcast_ref::<migration::Error>()
                .map(ToString::to_string)
                .unwrap_or_else(|| "invalid_request".to_owned());
            let status = if code == "resource_limit" { 413 } else { 400 };
            json_reply(request, status, json!({"error":code}));
        }
    }
}
