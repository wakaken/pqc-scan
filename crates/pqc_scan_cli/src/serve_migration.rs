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

pub(super) struct Generation {
    token: String,
    value: Value,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScopeRequest {
    input_sha256: String,
    keys: Vec<String>,
    module: Option<String>,
    #[serde(default)]
    excluded: Vec<usize>,
}
fn overview(state: &State, id: u64) -> Result<Value> {
    Ok(pqc_scan_report::migration_overview(&evidence(state, id)?))
}
fn scope(state: &State, id: u64, request: ScopeRequest) -> Result<Value> {
    if request.keys.len() > 256
        || request.keys.iter().any(|k| k.len() > 512)
        || request.excluded.len() > 4096
        || request.module.as_ref().is_some_and(|s| s.len() > 4096)
    {
        return Err(migration::Error::ResourceLimit.into());
    }
    let e = evidence(state, id)?;
    if e.input().sha256 != request.input_sha256 {
        return Err(migration::Error::InputBindingMismatch.into());
    }
    let keys: std::collections::BTreeSet<_> = request.keys.iter().collect();
    let valid: std::collections::BTreeSet<_> = e
        .targets()
        .iter()
        .map(pqc_scan_report::migration_scope_key)
        .collect();
    if keys.iter().any(|k| !valid.contains(*k)) {
        bail!("invalid scope");
    }
    let excluded: std::collections::BTreeSet<_> = request.excluded.into_iter().collect();
    if excluded.iter().any(|i| *i == 0 || *i > e.targets().len()) {
        bail!("invalid exclusion");
    }
    let mut rows = vec![];
    let mut cohorts = BTreeMap::<String, Value>::new();
    let mut all = 0;
    let mut ambiguous = 0;
    let mut owned = 0;
    for (i, t) in e.targets().iter().enumerate() {
        if request.module.as_ref().is_some_and(|m| {
            t.source_ref
                .path
                .as_deref()
                .unwrap_or("")
                .split(['/', '\\'])
                .next()
                .unwrap_or("")
                != m
        }) {
            continue;
        }
        let key = pqc_scan_report::migration_scope_key(t);
        let parts: Value = serde_json::from_str(&key)?;
        let entry = cohorts.entry(key.clone()).or_insert_with(|| json!({"key":key,"purpose":parts[0],"role":parts[1],"location":parts[2],"total":0,"eligible":0,"owned":0}));
        entry["total"] = (entry["total"].as_u64().unwrap() + 1).into();
        if t.is_bindable() {
            let field = if excluded.contains(&(i + 1)) {
                "owned"
            } else {
                "eligible"
            };
            entry[field] = (entry[field].as_u64().unwrap() + 1).into();
        }
        if !keys.contains(&key) {
            continue;
        }
        all += 1;
        if !t.is_bindable() {
            ambiguous += 1;
            continue;
        }
        if excluded.contains(&(i + 1)) {
            owned += 1;
            continue;
        }
        if rows.len() == 4096 {
            return Err(migration::Error::ResourceLimit.into());
        }
        rows.push(display(i, t));
    }
    Ok(
        json!({"input":e.input(),"total":all,"ambiguous":ambiguous,"owned":owned,"targets":rows,"cohorts":cohorts.into_values().collect::<Vec<_>>()}),
    )
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
    GenerateGroups {
        input_sha256: String,
        as_of: String,
        organization_id: String,
        groups: Vec<AnswerGroup>,
        #[serde(default)]
        references: Vec<migration::ContextReference>,
    },
    ImportContext {
        input_sha256: String,
        as_of: String,
        context: String,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnswerGroup {
    system_id: String,
    selected: Vec<usize>,
    answers: Vec<Answer>,
    actor: Option<String>,
    asserted_at: Option<String>,
    valid_until: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Confirmation {
    token: String,
    language: Option<String>,
}
struct TargetQuery {
    job: u64,
    page: usize,
    limit: usize,
    search: String,
    role: String,
}
fn query(query: &str) -> Result<TargetQuery> {
    let mut values = BTreeMap::new();
    for pair in query.split('&') {
        let (key, value) = pair.split_once('=').context("invalid query")?;
        if !["job", "page", "limit", "search", "role"].contains(&key)
            || values.insert(key, decode(value)?).is_some()
        {
            bail!("invalid query");
        }
    }
    let job = values.get("job").context("missing job")?.parse()?;
    let page = values
        .get("page")
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(0);
    let limit = values
        .get("limit")
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(100);
    let search = values.get("search").cloned().unwrap_or_default();
    let role = values.get("role").cloned().unwrap_or_default();
    if ![20, 100].contains(&limit) || search.len() > 256 || role.len() > 64 {
        bail!("invalid query");
    }
    Ok(TargetQuery {
        job,
        page,
        limit,
        search,
        role,
    })
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
    json!({"number":index+1,"id":target.id,"algorithm":target.technical.algorithm.identifier,"purpose":target.technical.algorithm.purpose,"role":target.source_ref.role,"role_ja":pqc_scan_report::migration_role_label(&target.source_ref.role,pqc_scan_report::ReportLanguage::Ja),"role_en":pqc_scan_report::migration_role_label(&target.source_ref.role,pqc_scan_report::ReportLanguage::En),"path":target.source_ref.path,"line":target.source_ref.line,"selectable":target.is_bindable(),"support_fields":support_fields})
}

fn targets(state: &State, query: &TargetQuery) -> Result<Value> {
    let e = evidence(state, query.job)?;
    let search = query.search.to_lowercase();
    let rows = e
        .targets()
        .iter()
        .enumerate()
        .filter(|(_, target)| {
            (query.role.is_empty() || target.source_ref.role == query.role)
                && (search.is_empty()
                    || format!(
                        "{} {}",
                        target.technical.algorithm.identifier,
                        target.source_ref.path.as_deref().unwrap_or("")
                    )
                    .to_lowercase()
                    .contains(&search))
        })
        .collect::<Vec<_>>();
    let start = query
        .page
        .checked_mul(query.limit)
        .context("invalid page")?;
    if start > rows.len() {
        bail!("invalid page");
    }
    let result = rows
        .iter()
        .skip(start)
        .take(query.limit)
        .map(|(i, t)| display(*i, t))
        .collect::<Vec<_>>();
    let roles = e
        .targets()
        .iter()
        .map(|t| t.source_ref.role.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let role_labels = roles.iter().map(|role| (role.clone(), json!({"ja":pqc_scan_report::migration_role_label(role,pqc_scan_report::ReportLanguage::Ja),"en":pqc_scan_report::migration_role_label(role,pqc_scan_report::ReportLanguage::En)}))).collect::<BTreeMap<_,_>>();
    Ok(
        json!({"input":e.input(),"total":rows.len(),"all_total":e.targets().len(),"roles":roles,"role_labels":role_labels,"page":query.page,"targets":result}),
    )
}
fn group_selections(
    groups: Vec<AnswerGroup>,
    references: &[migration::ContextReference],
) -> Result<Vec<ContextSelection>> {
    if groups.len() > 4096 {
        return Err(migration::Error::ResourceLimit.into());
    }
    let mut selections = Vec::new();
    for group in groups {
        if group.selected.is_empty()
            || selections.len().saturating_add(group.selected.len()) > 4096
            || group.answers.len() > 20
        {
            return Err(migration::Error::ResourceLimit.into());
        }
        let claims = if group.answers.is_empty() {
            if group.actor.is_some() || group.asserted_at.is_some() || group.valid_until.is_some() {
                bail!("provenance without answers");
            }
            ContextClaims::default()
        } else {
            let answers = group
                .answers
                .into_iter()
                .map(|a| match a {
                    Answer::Known { field, value } => ClaimAnswer {
                        field,
                        value: Some(value.into_value()),
                    },
                    Answer::Unknown { field } => ClaimAnswer { field, value: None },
                })
                .collect::<Vec<_>>();
            ContextClaims::from_answers_with_references(
                &answers,
                Provenance::operator(
                    group.actor.as_deref().context("actor required")?,
                    group.asserted_at.as_deref().context("assertion required")?,
                    group.valid_until.as_deref(),
                )?,
                references,
            )?
        };
        for number in group.selected {
            selections.push(ContextSelection {
                target_index: number.checked_sub(1).context("invalid index")?,
                system_id: group.system_id.clone(),
                claims: claims.clone(),
            });
        }
    }
    Ok(selections)
}
fn import(state: &State, bytes: Vec<u8>, filename: Option<String>) -> Result<Value> {
    if filename.as_ref().is_some_and(|name| {
        name.is_empty()
            || name.len() > 1024
            || name.chars().any(char::is_control)
            || name.contains(['/', '\\'])
    }) {
        bail!("invalid filename");
    }
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
    let mut job = job;
    job["created_at_ms"] = session_time_ms().into();
    if let Some(filename) = filename {
        job["display_name"] = filename.into();
    }
    jobs.entries.insert(id, job.clone());
    Ok(job)
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
            if selected.len() > 4096 || answers.len() > 19 {
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
        Preview::GenerateGroups {
            input_sha256,
            as_of,
            organization_id,
            groups,
            references,
        } => {
            let selections = group_selections(groups, &references)?;
            let bytes = migration::generate_context_with_references(
                &e,
                &organization_id,
                &selections,
                &as_of,
                &references,
            )?;
            let context = MigrationContext::from_bytes(&bytes)?;
            let bound = context.bind(&e, &as_of)?;
            if e.targets().iter().any(|target| {
                bound
                    .target(&target.id)
                    .is_some_and(|t| t.purpose_conflicting)
            }) {
                bail!("purpose mismatch");
            }
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
fn confirm(
    state: &State,
    id: u64,
    confirmation: Confirmation,
    mut progress: impl FnMut(Value),
) -> Result<Value> {
    progress(json!({"phase":"loading"}));
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
    progress(json!({"phase":"evaluating","targets":e.targets().len()}));
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
    let label = state
        .jobs
        .lock()
        .unwrap()
        .entries
        .get(&id)
        .and_then(|j| {
            if j["kind"] == "saved" {
                j["display_name"].as_str()
            } else {
                j["path"]
                    .as_str()
                    .filter(|p| !p.is_empty() && *p != ".")
                    .or_else(|| j["display_name"].as_str())
            }
        })
        .unwrap_or("Saved scan")
        .chars()
        .take(120)
        .collect::<String>();
    let mut label = label
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>();
    if label.len() > 256 {
        while label.len() > 253 {
            label.pop();
        }
        label.push_str("...");
    }
    pqc_scan_report::write_migration_assessment_reports_with_label_and_progress(
        &report,
        &e,
        context.as_ref(),
        &dir,
        language,
        Some(if label.trim().is_empty() {
            "Saved scan"
        } else {
            &label
        }),
        |phase| {
            use pqc_scan_report::MigrationReportProgress as P;
            progress(match phase {
                P::Validating => json!({"phase":"validating"}),
                P::Json => json!({"phase":"json"}),
                P::Markdown => json!({"phase":"markdown"}),
                P::Html => json!({"phase":"html"}),
                P::Writing { completed, total } => {
                    json!({"phase":"writing","completed":completed,"total":total})
                }
            });
        },
    )?;
    progress(json!({"phase":"context"}));
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
fn start_generation(state: &Arc<State>, id: u64, confirmation: Confirmation) -> Result<Value> {
    if !matches!(
        confirmation.language.as_deref().unwrap_or("ja"),
        "ja" | "en"
    ) {
        bail!("invalid language");
    }
    if state
        .drafts
        .lock()
        .unwrap()
        .get(&id)
        .is_none_or(|d| d.token != confirmation.token)
    {
        bail!("draft expired");
    }
    let mut jobs = state.jobs.lock().unwrap();
    if jobs.active {
        bail!("busy");
    }
    jobs.active = true;
    if let Some(job) = jobs.entries.get_mut(&id) {
        job["migration_ready"] = false.into();
    }
    drop(jobs);
    let token = confirmation.token.clone();
    state.generations.lock().unwrap().insert(
        id,
        Generation {
            token: token.clone(),
            value: json!({"status":"running","phase":"loading"}),
        },
    );
    let owner = Arc::clone(state);
    std::thread::spawn(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            confirm(&owner, id, confirmation, |phase| {
                let mut runs = owner.generations.lock().unwrap();
                if let Some(run) = runs.get_mut(&id) {
                    run.value = phase;
                    run.value["status"] = "running".into();
                }
            })
        }));
        let value = match result {
            Ok(Ok(result)) => json!({"status":"ready","phase":"complete","result":result}),
            Ok(Err(error)) => {
                json!({"status":"failed","error":error.downcast_ref::<migration::Error>().map(ToString::to_string).unwrap_or_else(||if error.to_string()=="resource_limit" {"resource_limit".into()}else{"generation_failed".into()})})
            }
            Err(_) => json!({"status":"failed","error":"generation_failed"}),
        };
        if value["status"] == "failed" {
            owner.drafts.lock().unwrap().remove(&id);
        }
        owner.jobs.lock().unwrap().active = false;
        if let Some(run) = owner.generations.lock().unwrap().get_mut(&id) {
            run.value = value;
        }
    });
    Ok(json!({"token":token,"status":"running"}))
}
fn dispatch(
    request: &mut Request,
    state: &Arc<State>,
    route: &str,
    query_text: &str,
) -> Result<Value> {
    if route == "api/migration/import" && request.method() == &Method::Post {
        if !query_text.is_empty() {
            bail!("invalid query");
        }
        let filename = header(request, "X-Pqc-Filename").map(decode).transpose()?;
        return import(state, body(request, INPUT_LIMIT)?, filename);
    }
    let parameters = query(query_text)?;
    let id = parameters.job;
    if route != "api/migration/targets"
        && (parameters.page != 0
            || parameters.limit != 100
            || !parameters.search.is_empty()
            || !parameters.role.is_empty())
    {
        bail!("invalid query");
    }
    if request.method() == &Method::Post
        && state
            .generations
            .lock()
            .unwrap()
            .get(&id)
            .is_some_and(|g| g.value["status"] == "running")
    {
        bail!("generation busy");
    }
    match (request.method(), route) {
        (&Method::Get, "api/migration/overview") => overview(state, id),
        (&Method::Post, "api/migration/scope") => scope(
            state,
            id,
            serde_json::from_slice(&body(request, PREVIEW_LIMIT)?)?,
        ),
        (&Method::Get, "api/migration/progress") => {
            let runs = state.generations.lock().unwrap();
            let run = runs
                .get(&id)
                .filter(|g| Some(g.token.as_str()) == header(request, "X-Pqc-Migration-Token"))
                .context("unavailable generation")?;
            Ok(run.value.clone())
        }
        (&Method::Post, "api/migration/start") => {
            start_generation(state, id, serde_json::from_slice(&body(request, 1024)?)?)
        }
        (&Method::Get, "api/migration/targets") => targets(state, &parameters),
        (&Method::Post, "api/migration/preview") => {
            let mut value = preview(
                state,
                id,
                serde_json::from_slice(&body(request, PREVIEW_LIMIT)?)?,
            )?;
            if header(request, "X-Pqc-Compact-Preview") == Some("1") {
                value.as_object_mut().unwrap().retain(|key, _| {
                    ["token", "input_sha256", "as_of", "context"].contains(&key.as_str())
                });
            }
            Ok(value)
        }
        (&Method::Post, "api/migration/assess") => {
            if state.jobs.lock().unwrap().active {
                bail!("busy");
            }
            confirm(
                state,
                id,
                serde_json::from_slice(&body(request, 1024)?)?,
                |_| {},
            )
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
pub(super) fn handle(mut request: Request, state: &Arc<State>, route: &str, query: &str) {
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
