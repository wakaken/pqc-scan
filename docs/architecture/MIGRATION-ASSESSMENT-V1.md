# Saved-evidence migration assessment proposal

Status: **D1-D5 recommended package adopted by the maintainer on 2026-11-18**.
Owner: task066. This is the selected new
opt-in contract, not a schema file, implemented feature or amendment to Inventory,
Context v1, `migration-review-v1`, scan/serve/probe outputs or support status.
See [feasibility](../../tasks/066/FEASIBILITY.md),
[decisions](../../tasks/066/DECISIONS.md),
[technical sources](../../tasks/066/FACT-SOURCE-MAPPING.md),
[cases](../../tasks/066/ACCEPTANCE-CASES.json) and
[evaluation protocol](../../tasks/066/EVALUATION-PROTOCOL.md).
The actual approval and scope exclusions are recorded in the decision package.
Implementation and independent verification remain downstream obligations.

## Maintainer presentation amendment (2026-11-18)

The maintainer subsequently requested a comprehensible human report with the
existing scan HTML styling and an assess-generated HTML file. This explicitly
selects additive `migration.assessment.html` alongside JSON and Markdown in the
opt-in assess workflow. Human views summarize typed facts and claims without raw
JSON dumps; exact machine data stays in the unchanged JSON contract. Reuse the
existing report styles and local language/theme controls. No scan/serve default,
policy, source schema, numeric score or publication change is authorized. See
[tasks/070 presentation follow-up](../../tasks/070/TASK-070-PRESENTATION-FOLLOWUP.md).
The original066 selection below remains historical where it excludes HTML.

## Dispatch and workflow

Proposed new CLI entry `assess` accepts one local saved input, explicit
`--input-format inventory-v1|tls-v1|ssh-offers-v1|ssh-kex-v1`, optional local context,
required `--as-of` time, `--out-dir`, and `--lang en|ja` (default en). D3 determines
which of the four formats are enabled. No format guessing by filename/content,
automatic format upgrade, glob/directory traversal, compressed input, remote URL,
stdin stream, shell command, probe, dependency execution or rescanning.

Read each regular input/context file once into a bounded immutable buffer,
preflight UTF-8 JSON, hash exact bytes, validate version/structure/references and
project facts. Evaluate pure facts and selected claims at the explicit time.
Write `migration.assessment.v1.json` and `migration.assessment.md` through existing
safe output. English/Japanese Markdown must convey identical facts, IDs,
uncertainty and reasons. No new HTML or serve integration is selected; the new
v0.3 HTML/rules companion and web UI remain unchanged defaults.

Running without context is supported: produce the target manifest, technical
facts and missing-context decisions. A user can bind a later context to that exact
input and its target IDs, then rerun without collection. Never modify the saved
input. Read failures, unknown input format/major version, invalid context/bindings,
contradictory saved facts or resource overflow produce a fixed operational error
and no new report set. Successful advisory evaluation returns 0 even when urgent,
blocked or incomplete. No new fail threshold or policy-dependent exit is added;
existing scan/probe/CLI parse exit semantics remain intact.

## Identity and immutable binding

Compute lower-case SHA-256 over the exact UTF-8 file bytes, before parsing; a
changed byte, including formatting, requires a fresh explicit content binding.
Do not invent a canonical JSON scheme or change native/finding IDs. Reject BOM,
trailing second JSON values and duplicate keys at every nesting level.

A target ID uses the existing length-framed helper with a new `migration_target`
domain and the tuple (input format, input byte digest, existing observation/attempt
identity, source root reference, fixed component role/path). It is a new assessment
reference only. Its identity never depends on context, system claims or as-of time.

For Inventory, preserve run/observation/asset IDs and expose one target per
observation plus directly supported technical component. Resolve only explicit
allowlisted role paths: direct algorithm, public-key.algorithm,
certificate.signature_algorithm, certificate.subject_key.algorithm, or
reference-only. Validate the actual relationship names against the input; each
role path has one unambiguous target or remains unresolved. Never search arbitrary
graph paths or transfer a business binding through a shared algorithm asset.
Ambiguous duplicate anchors cannot receive a business-system binding: reject the
binding with a fixed ambiguity error and retain the technical target in a
context-free assessment. No position/name/glob/algorithm-based guesswork.

For TLS, target references use attempt1 and fixed JSON detail paths: negotiated
key establishment and certificate-index signature/subject-key components. Preserve
certificate/SPKI fingerprints and original collection/index association. For SSH,
use attempt1, advertised-list role+entry ordinal, or negotiated-KEX/host-signature
role. Extension markers are not cryptographic targets. Do not turn endpoint names
into source paths or expose list order as successful selection.

The context binds exactly one input format and digest. Its binding rows reference
published target IDs from the context-free manifest and one declared system ID.
Each target can have at most one system binding. Missing IDs, duplicates,
content/format mismatch, ambiguous targets and conflicting ownership reject the
context; unknown bindings are not silently ignored. A mechanically valid mapping
still remains an unverified operator statement about the real business system.
Reusing context for another scan requires fresh explicit bindings.

## Proposed context fields and branch obligations

Context has its own new namespace `migration-context-v1`, version1; do not reuse
Context v1 or pass it to the new evaluator implicitly. All fields are closed and
typed. Absent optional fields mean unknown; JSON null and invalid enums are
rejected, rather than merged with false/not-applicable. No file includes or
arbitrary expressions/references are allowed.

| Record/field | Shape and purpose | Consumer/obligation |
|---|---|---|
| format, version, input | Fixed contract/version and exact input format+digest | D1 dispatch and content binding |
| organization_id, systems[].id | Non-secret ASCII aliases, `[a-z][a-z0-9._-]{0,63}`; IDs unique | Scope isolation and human output; no automatic real-world identification |
| organization_defaults, systems[].defaults, bindings[].claims | Same optional typed claim set; fixed priority organization → system → target | Explicit inheritance within the selected input; no cross-purpose or cross-system transfer |
| bindings[].target_id, system_id | Exact manifest target and declared system | R4-02 target ownership; duplicate ownership invalid |
| claim provenance | source_kind (`operator_claim` or `external_review_claim`), actor alias, asserted_at, valid_until, optional reference_id | Preserve source/actor/time/scope; external label is not scanner verification. An active claim requires asserted_at <= as_of < valid_until; no invented freshness window |
| applicability | known applicable / not_applicable, or explicit unknown | Table row P1; N/A retains technical concerns and does not mean safe |
| use_case | known key_establishment / signature / jwt / pki / encryption / hashing, or explicit unknown | Purpose resolution, relevant candidate/support requirements; never overrides a known incompatible primitive purpose |
| usage_state | Optional active / planned / test_only / unused claim | P4 business-use gate and action sufficiency; does not change observed assurance |
| business_critical, confidential | Optional booleans | P4's explicit local critical-use/long-protection policy; false differs from unknown |
| confidentiality_until, signature_verification_until | Optional UTC instants | Protection obligations kept separately; do not use certificate validity or a quantum-arrival prediction as a substitute |
| historical_ciphertext | Optional boolean | Retains HNDL concern for previously collected ciphertext even after new sessions migrate; not remediated by changing future KX alone |
| migration_lead_seconds | Optional integer in [0,315576000] | P3 deadline pressure; inclusive boundary; 10-year engineering bound, not quantum horizon |
| deadline | Optional tagged value: none, or at + reference_id + applicability reason code | Explicit organization/scoped deadline; no built-in universal date. Cannot affect decisions without a valid active applicable claim |
| support | Optional runtime/provider/peer/protocol/parameters/jose/verifiers/pki booleans | Required subset by purpose; all true means declared readiness only, any false blocks, missing remains unknown |

The rows from applicability through support are individually provenance-bearing claims: each chosen value
carries its actor, source kind, asserted interval and optional reference. Defaults
and target overrides operate on complete claim records, not only bare values.
A target-level unknown explicitly replaces a lower-level value; omission inherits.
Provide an explicit typed unknown marker for a claim whose underlying value is
boolean/time/number; this is not null or false. Thus every optional claim is either
absent or `{state: known, value, provenance}` or `{state: unknown, provenance}`.
Applicability/use-case unknown uses only the wrapper state (not a second known
`unknown` enum value); support entries use the same wrapper.
No conflicting duplicate record, equal-precedence merge or last-one-wins behavior.

Only active, purpose-relevant claims influence a positive readiness/policy branch.
Expired/future/unbounded provenance is shown as stale/not-yet-effective/unknown
and cannot establish a favorable claim. A stale deadline whose declared date is already due remains
visible as a historical declared concern and makes refreshing it an urgent
investigation; it cannot silently become low priority or a current legal mandate.
Known technical concerns survive missing, conflicting or stale business context.
A purpose claim incompatible with a known primitive role is a semantic conflict:
retain both facts, mark information conflicting, and do not use it to establish
favorable readiness or a replacement candidate. Structural duplicate/ownership
conflicts remain input errors as specified above.
No raw claim text, credentials, key material or private parser bytes are needed.
Legitimate aliases/reference IDs remain operator-sensitive metadata, not secret
classification or independently verified identity.

## Time and policy semantics

`as_of` is required and explicit, normalized to UTC seconds with a required offset;
reject leap-second/overflow/invalid dates. Input collection time remains unchanged;
reject as_of before collection time. The same input/context/catalog/policy/as_of
produces the same semantic result; creation/presentation timestamps are separate.
No implicit now, quantum-arrival date, automatic deadline lookup or universal
X+Y>Z equation.

Compute deadline pressure only from an active applicable deadline D and known
nonnegative lead L: urgent when D <= as_of + L, including equality and expired D.
Use checked integer arithmetic. Missing D or L cannot be replaced with zero.
An actively declared deadline `none` explicitly rules out deadline pressure; an
absent/unknown deadline does not. Certificate validity retains its captured-time assessment and may additionally
show an explicit as_of validity observation; never conflate it with signature-
verification lifetime or confidentiality lifetime. Future protection obligations
and historical ciphertext are independent of certificate expiry and new rollout.

These are separately encoded axes, not one multiplied score:

- urgency: urgent / plan / monitor / undetermined / not_applicable.
- information: sufficient / incomplete / conflicting.
- readiness: ready / blocked / unknown / not_applicable; `ready` means all relevant
  integration prerequisites are actively **declared** true, not verified deployed.
- next_action: investigate / plan / unblock / migrate / monitor / none.

`undetermined` is unordered, never a low rank or numeric zero. A known urgent
condition remains urgent with incomplete information or blocked readiness. Missing
irrelevant fields do not suppress a branch with all its required inputs. No
aggregate Pass, migration-complete result or automatic total ranking is produced.

## Proposed decision table (precedence P1 through P8)

The policy name `migration-decision-v1` and version1.0.0 are proposed local advisory
policy, not a NIST rule or proven operational utility. Evaluate per target/role.
Validation failures precede this table and return an error, not a safety decision.

| Row | Minimal condition | Urgency | Required reason/action |
|---|---|---|---|
| P1 | Active explicit not_applicable claim | not_applicable | outside_declared_scope; none, with technical facts/known concerns preserved |
| P2 | Known classical weakness in a relevant role (e.g. weak certificate signature/key); P1 exclusion absent; no business fields required | urgent | known_classical_concern; investigate/unblock/migrate by readiness and evidence, not an incident assertion |
| P3 | Applicable known technical concern + active applicable D and known L, D <= as_of+L | urgent | deadline_reached_or_lead_time_boundary; readiness cannot lower urgency |
| P4 | Applicable known quantum concern + active/planned business-use claim + critical-system claim + relevant signing/authentication purpose, OR applicable vulnerable confidentiality use + confidential claim + (active/planned use with a future protection obligation OR historical_ciphertext true) | urgent | critical_use_declared or long_protection_or_historical_ciphertext; local policy choice, no quantum date |
| P5 | Stale deadline claim has an explicit D <= as_of for a known concern, with no stronger active urgent row | urgent | stale_urgent_claim_requires_refresh; investigate; does not assert a current binding mandate |
| P6 | Applicable known concern; active noncritical/use/protection and deadline facts explicitly rule out P3/P4 urgency | plan | known_concern_plan; unknown deadline alone does not fabricate deadline urgency or absence |
| P7 | No known concern in the bounded technical role, exact recognized facts and applicable use, relevant completeness/claim freshness sufficient | monitor | bounded_role_no_known_concern; does not certify the system or unknown roles; readiness remains independent |
| P8 | Otherwise, including unknown applicability/use/parameters, reference-only or offers without selected use | undetermined | needed facts listed; investigate; known concerns remain present |

Applicability absent means unknown for business decisions (unlike Context v1's
local-profile default). P2 can preserve urgent technical investigation even when
business applicability is unknown, except active P1 exclusion; uncertainty remains.
For P3/P4/P6/P7, applicability must be actively applicable. P4's OR grouping is explicit: its confidentiality branch does not require
business_critical, and historical ciphertext does not require current active use.
All P4 claims used to establish a branch must be active/fresh.
P6 minimum facts are active applicability, known concern/purpose, explicit
business_critical=false, an active deadline none or a known D > as_of+L,
and (for confidentiality) confidential=false or no
remaining protection obligation explicitly established. When those facts are
unknown, use P8 rather than assuming ordinary priority. A future active deadline
with L unknown is not proof it is outside P3. No hidden urgency override field is accepted.

Information is conflicting when a relevant semantic claim contradicts known facts;
otherwise it is incomplete when relevant collection/encoding/use facts or applicable
branch/action prerequisite claims are missing, and sufficient otherwise. Unused
fields do not enter missing_inputs. A known concern requires readiness evaluation;
P1/P7 with no selected migration action use readiness not_applicable.

Readiness precedence is total for a validated target: (1) P1/P7 without a
selected migration action use not_applicable; (2) any active, relevant explicit
false support claim makes readiness blocked, even when other prerequisites are
missing/stale/unknown; (3) otherwise a missing/stale/unknown relevant prerequisite
or unresolved/incompatible purpose makes readiness unknown; (4) otherwise all
relevant active claims true make readiness ready (declared, not verified).
Known false blockers remain listed even with incomplete information.

Action precedence is likewise explicit:

| Order | Condition after urgency | Single action |
|---|---|---|
| 1 | P1 | none |
| 2 | P7 bounded no-known-concern role | monitor |
| 3 | P5, semantic conflict, stale claim requiring refresh, P8, or unresolved reference-only/offer use | investigate |
| 4 | Known applicable urgent/plan concern, adequate use facts, readiness blocked | unblock |
| 5 | Same, urgent and declared ready | migrate |
| 6 | Same, plan and declared ready | plan |
| 7 | Remaining urgent/plan concern, including unknown readiness | investigate |

Adequate business use can be a collector-reported session or a fresh explicit
active/planned-use claim; that claim never upgrades observed assurance. An urgent
blocked result retains urgency and known blocking dependencies. Missing unrelated
fields never erase a supported urgent row or a known false blocker.

Relevant support names follow the existing prerequisite concepts: runtime,
provider and parameters for every candidate; peer and protocol for key establishment
or signatures; protocol also for encryption; no extra protocol/peer requirement
for a hash candidate. JWT signatures additionally require jose and verifiers;
PKI signatures require pki (covering certificate binding and issuer/trust-store
support). Apply the false-before-unknown readiness precedence above; explicit
unknown never cancels a known blocker. Irrelevant purpose claims
cannot satisfy any of these requirements.

Reasons are fixed codes with source refs, applied claim refs and missing-input
codes, never arbitrary generated advice. Required codes cover every P row,
unknown/unsupported primitive, partial collection, reported verification failure,
unknown binding/parameter, stale claim, purpose mismatch and unverified combiner.
Next actions reuse existing candidate-purpose/prerequisite concepts; no KEM for
signatures, guessed replacement, unverified hybrid PKI deployment or automatic
code edit. Generic RSA intended purpose cannot be resolved by a hashing claim.

## Technical dispatch and ingestion invariants

Keep `local-assessment-1` and `migration-review-v1`/1.0.0 unchanged. The new path
selects proposed `migration-facts-v1`/1.0.0 and `migration-decision-v1`/1.0.0;
unknown catalog/policy/major versions reject rather than silently fallback.
Reuse pure primitive evaluation for unchanged facts and supplement exact
protocol/component/encoding results in the new dispatch only. See source mapping.
Old reports never consume a changed shared helper without old golden regressions.

Inventory: validate v1 structure/IDs/references using current NativeScan rules,
then bounded fact/role validation; unknown attributes/vocabulary retain the
published reader tolerance and become unknown, not invented technical facts.
Do not globally forbid future string vocabulary or arbitrary graph cycles;
resolve only fixed role paths with bounded depth and ambiguity checks.

TLS: validate existing envelope/schema semantics, keeping verification and
collection outcome separate. Publish collector-reported negotiated facts only
when connected peer is bound to pinned address/answers, handshake_complete is true,
kind is full or full_with_hello_retry_request, chain_and_name is verified, and
negotiated version/suite are present and consistent. This valid conjunction can
occur in completed **or partial** records: the current collector sets verified
facts before missing-group/certificate-attribute diagnostics cause partial.
Completed requires empty diagnostics; partial requires nonempty diagnostics under
the existing envelope invariant. Preserve known version/suite/group in such valid
partial records while missing group/attributes and affected roles remain unknown.
A complete KX role may coexist with partial certificate collection; no global
safety/completeness claim follows. Failed verification/handshake cannot carry a
verified negotiation; reject that contradiction. Valid unverified failure/partial
records retain safe facts without a reported verified-session claim.
Known numeric group/name/classification contradictions reject. Unknown IDs remain
unknown. Exact final hybrid groups do not make certificate authentication PQC;
missing group is unknown. Parsed certificate attributes do not add ASN.1 parameter
encoding, signature verification or revocation facts absent from the saved input.

SSH offers: preserve unauthenticated-advertisement assurance and ordered roles;
strip extension markers from cryptographic candidates; do not infer selection.
SSH KEX: publish collector-reported selected KEX only when completed, handshake
complete, host identity verified under supported recorded pin, user authentication
not attempted, and selected method consistent with the recorded profile. Otherwise
retain safe failure facts or reject contradictory success claims. Host signature,
KEX and user authentication remain distinct. D3 must enable these readers explicitly.

No importer loads TLS/SSH providers, initializes sockets/DNS, reads trust stores,
fetches revocation/catalog/schema links or uses system SSH. Pure metadata adapters
must compile/test in the default static configuration. Reading a saved verified
flag retains provenance but does not perform new trust verification.

## Proposed limits and error/privacy/output contract

These are proposed resource bounds for human approval, not measured performance
or new hard quality-score gates. Use checked allocation/counting and reject whole
input on overflow; never truncate to an apparently complete assessment.

| Boundary | Proposed inclusive maximum | Rationale / future boundary proof |
|---|---|---|
| Inventory bytes | 32 MiB | Bounded local batch; independent of file-scan size limits; current small goldens fit |
| Any probe bytes | 1 MiB | Compact metadata/fingerprints, not wire packets or DER; existing collector per-list/chain bounds still apply |
| Context bytes | 1 MiB | Allows explicit per-target mappings; Context v1's 16KiB is unchanged |
| JSON depth / object fields | 16 / 64 per object | Closed shallow envelopes/claim records; bounded preflight, duplicate-key rejection |
| Assets / observations / projected targets | 16384 / 32768 / 65536 | Explicit bounded indices; reject oversized valid-v1 batches on new reader only |
| Relations or asset refs per record | 64 | Fixed role resolution only; prevents combinatorial expansion |
| Systems / bindings | 256 / 4096 | Explicit local scope; no discovery or cross-input bulk reuse |
| Ordinary string / aliases / reference IDs | 1024 UTF-8 bytes / 64 ASCII bytes / 128 ASCII bytes | Fixed known vocabularies and non-secret identifiers; key names also bounded |
| Candidate/reason/missing/action refs per target | 4 / 16 / 16 / 16 | Reuse fixed catalogs; no unbounded advice expansion |
| Produced report bytes | 64 MiB per file | Bound rendered output independently; error before replacing an oversized file |
| UTC fields / lead | UTC second precision, years1970..9999 / 0..315576000 seconds | Validatable clocks; checked difference/addition; not a quantum forecast |

Probe-specific smaller limits (e.g. 16 certificates, bounded algorithm lists) remain
as published; new caps do not enlarge them. Validate unknown/duplicate context/TLS/
SSH fields, wrong types, null, malformed UTF-8/JSON and invalid references with
fixed errors that do not echo rejected values. Native extension tolerance is
explicit above; known bounded fields still validate. Unselected formats reject.

No keys/passwords/tokens or raw private content are required or copied. Output
retains only safe typed facts, declared aliases/reference IDs, applied provenance,
known public fingerprints and necessary saved target/location links. Do not copy
raw evidence/snippets/arbitrary input metadata or absolute local context paths.
External-review labels stay claims. Tests place secret canaries in rejected fields
and arbitrary metadata, checking logs and every emitted format. Do not promise a
general secret detector for legitimate operator-selected identifiers.

Reject output destinations aliasing either opened input/context path or file
identity before writes; existing safe output guards still apply. No automatic
input update. Existing unrelated output files are not deleted. Existing regular
report files may be replaced atomically per file, not as an all-files transaction.
Use explicit new filenames to preserve scan/serve/report.rules defaults.

## Record shapes (proposed storage contracts)

All named records below reject unknown and duplicate fields, except the existing
Inventory extension-tolerant reader already described. Context and new output
contracts are new namespaces; they do not amend any v1 input schema in place.

| Context record | Exact fields and types |
|---|---|
| Root (required) | format=`pqc-scan.migration-context`, version=1, input, organization_id, systems, bindings; organization_defaults and references optional |
| input | format (selected fixed input-format enum), sha256 (64 lower-case hex characters) |
| systems[] | id (unique alias), defaults (optional claim-set object) |
| bindings[] | target_id (exact generated target ID), system_id (existing alias), claims (optional claim-set object) |
| claim set | Only applicability, use_case, usage_state, business_critical, confidential, confidentiality_until, signature_verification_until, historical_ciphertext, migration_lead_seconds, deadline, support |
| ordinary claim | state=`known` with value of the field's declared type, or state=`unknown` with no value; provenance required in both forms |
| support | Optional map of the eight fixed prerequisite names to individual boolean/unknown claim records; missing names inherit individually |
| provenance | source_kind, actor_id, asserted_at required; valid_until and reference_id optional. Missing valid_until means freshness unknown, not indefinitely active. A provided interval must have asserted_at < valid_until |
| deadline value | kind=`none` and no other fields, or kind=`at` with at, reference_id and scope_reason (fixed `organization_policy` or `contractual_requirement`). Its source reference must resolve; no law/compliance adjudication |
| references[] | id, issuer_id, document_id, edition, clause, scope_id required (bounded ASCII identifiers); optional https_uri (<=1024 bytes, no credentials/query/control characters; never fetched) |

An unknown wrapper has no value field; a known wrapper cannot contain null.
References must be unique and resolve whenever used by a claim/deadline. Identifier
aliases and document references are operator declarations, not attested issuers.
All date fields use the stated UTC parsing/normalization rules. Defaults may have
only the same claim-set fields; target IDs cannot appear in defaults.

| Output record | Required shape and optionality |
|---|---|
| Root | format=`pqc-scan.migration-assessment`, version=1, input, context, catalog, policy, model, as_of, targets, claims, candidates, decisions, sources, coverage, limitations |
| input | format, sha256, schema_version, collection_time; original run_id for Inventory or attempt_id/protocol for runtime; no absolute input path |
| context | null when absent; otherwise format, version and sha256; no raw context embedding |
| catalog / policy / model | id and version. Selected catalog migration-facts-v1/1.0.0; policy migration-decision-v1/1.0.0; model decision_table/1.0.0 |
| targets[] | id, source_ref, technical, observation, limitations. source_ref preserves native run/observation/root-asset/role references or runtime attempt/detail-path/public-fingerprint references; never invented source paths |
| technical | Pure primitive AlgorithmAssessment fields where applicable (identifier, construction, standardization, quantum, purpose, parameters_known, classical_concerns, attack_models, components, combiner_assessment, source_ids), plus encoding_assurance (known/unknown), protocol_ref and role. Unknown/reference-only targets retain fixed unknown values; protocol components keep their separate roles |
| observation | method, assurance, usage, collection_outcome. Fixed current Inventory vocabularies retained where known; runtime assurance distinguishes unauthenticated_advertisement, collector_reported_verified_session and unverified_or_incomplete. Unknown values do not upgrade assurance |
| claims[] | id, context_pointer, field, state, safe typed value only when known, provenance, freshness (active/stale/not_yet_effective/unknown); only applied or decision-relevant stale/conflicting claims |
| decisions[] | target_id, system_id (null if unbound), urgency, information, readiness, reason_codes, missing_inputs, applied_claim_ids, action, source_ids. Each target has exactly one decision. Unbound targets still have a result |
| action | kind from the fixed next_action enum, candidate_ids, blocking_prerequisites, validation_steps. Empty arrays allowed; no guessed replacement. Candidate records reuse existing purpose/prerequisite concepts, with fixed IDs and sources |
| candidates[] | id, target_id, identifier, purpose, status, parameter_selection, prerequisites, validation_steps, source_ids. Prerequisites carry the target-specific applied claim refs; candidate IDs use a new migration_candidate domain over target_id, context digest/absence, catalog/policy version, identifier and purpose. They cannot be shared to transfer one system's support claims to another |
| sources[] | id, edition, clause, URI or local source identifier, scope_limit; fixed technical catalog sources and declared policy references kept distinct |
| coverage | source collection status and its limits, selected input/target count, unresolved/unknown facts; not a new whole-repository inspection count or aggregate Pass |

Technical primitive enums retain the existing serialized vocabulary (quantum:
vulnerable/designed_resistant/generic_search/unknown; purpose: the existing Purpose
variants). New protocol compositions name their construction/components and a
`standard_specified_implementation_unverified` combiner scope; no combined
resistance conclusion is fabricated from the mere presence of one PQC component.
A saved TLS record lacking parameter encoding has encoding_assurance unknown even
when its OID identifies a parameter-set name. No incompatible primitive purpose
is overwritten to satisfy a context claim.

Fixed policy reason codes are outside_declared_scope, known_classical_concern,
deadline_reached_or_lead_time_boundary, critical_use_declared,
long_protection_or_historical_ciphertext, stale_urgent_claim_requires_refresh,
known_concern_plan, bounded_role_no_known_concern and insufficient_information.
Additive fixed limitation/conflict codes identify reference_only,
unauthenticated_advertisement, collection_incomplete, verification_not_completed,
unknown_primitive, unknown_parameter_encoding, unknown_binding, stale_claim,
purpose_mismatch and combiner_implementation_unverified. Missing-input names
are fixed context/technical field names, never raw rejected values. Renderers
translate labels/explanations only; code values and source facts remain exact.

## Proposed output records and final implementation obligations

The new report has format/version, input format+digest+original version/run/time,
selected context digest or absent, target manifest and source links, catalog/model/
policy IDs+versions, explicit as_of, technical facts/concerns, observation method/
reported assurance, applied claim provenance/freshness, urgency/information/
readiness, reason refs, missing fields, next actions/candidates and coverage/limits.
A result references the exact input and target; source run ID alone is insufficient.
Shared provenance/source records may be referenced instead of copied per target.
Context values are included only as the safe applied claims needed to explain a
branch; raw context is not embedded. No aggregate Pass/safety or numerical score.

Semantic IDs/order are deterministic across contexts except decision IDs, which
also bind selected context/policy/as_of. Sort targets by source reference then
role; sort reasons by fixed policy precedence; preserve ordered advertised lists.
Repeated evaluation uses the same explicit versions/time. Human rendering never
re-evaluates policy or translates identifiers/numeric facts.

Tasks067-070 must implement schema/readers/limits, source/target/claim checks,
fixed decision table, old v1 goldens, no-network/default-feature paths, privacy,
malformed/oversize/duplicate/unknown/false/stale/conflict cases and paired rendered
Markdown checks. Adopted HTML would additionally require a new D4 decision and
browser verification. A passing future parser or table agreement is not an
independent assessment of practitioner usefulness; task071 executes the protocol.
