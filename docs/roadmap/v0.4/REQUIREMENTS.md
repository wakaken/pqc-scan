> Current amendment2026-11-19: R5-01..05 now require only synthetic technical
> correctness, adopted-specification conformance and fixed-condition reproducibility.
> Human usefulness/workload/rater agreement and numeric-score validity/adoption are
> explicitly unmeasured and are not completion gates for this limited scope. Original
> text/approvals/unexecuted96 human judgments are preserved in task071's archive.
> The v2 protocol is authoritative for current R5; conflicting narrative below is
> historical. X05/072 and U08/U09/075 must disclose this amended scope and exclusions.

# v0.4 requirements and evidence map

[日本語](REQUIREMENTS.ja.md)

Status: proposal for maintainer review, 2026-11-18. Source baseline:
`3f2bc6e6209e1f2813caac8077c8d7a7198832cc` (`origin/develop`, product `0.3.0`).
See [ROADMAP.md](ROADMAP.md) for source selection, alternatives, sequencing,
standards, open decisions and scope boundaries.

This document follows v0.3's requirement-to-evidence structure. It defines future
observable outcomes, gaps, verification and acceptance evidence. No row is marked
implemented or passed by writing it. W1-W7 are logical work packages; the
subsequently authorized [tasks and requirement map](TASKS.md) registers 065-072 as
`pending`. Registration does not select or execute them. Evidence names below
describe required content, not existing artifacts or approved new public filenames.

<a id="input-assistance-extension"></a>
## Input-assistance extension (2026-11-18)

The maintainer authorized the following nine additional acceptance clauses and
registration only. They supplement R4/X03 and preserve all five mandatory outcomes
and original 30 clauses below. No feature availability or execution is claimed.
CLI conversational input is deferred; a noninteractive template entrypoint and
existing local-web input/reassessment are in scope. Task 075 owns expanded final
acceptance after original 072; see [the roadmap](ROADMAP.md) and [map](TASKS.md).

| ID | Primary task | Required behavior | Acceptance and required future evidence |
|---|---|---|---|
| U01 | 073 | Common typed configuration generation | Use existing saved-evidence targets and context validation/binding; produce adopted v1 JSON with exact input digest/IDs; valid partial or empty claims are allowed. No second evaluator/schema or invented claims. |
| U02 | 073 | Explicit template generation without manual IDs | A separately selected noninteractive CLI entrypoint writes a validated template through safe output, with readable target selection, configurable known claims and generated aliases. No manual digest/ID transcription, default binding of all targets, or CLI conversation. Unknown/sparse inputs remain valid; invalid/stale bindings fail safely. |
| U03 | 074 | Personal and purpose-dependent optional forms | Users may skip context or answer only known relevant questions. Company/department/business deadlines are not required. Distinguish omission/unknown/false/N/A and preserve existing criticality semantics; no personal-mode low-priority shortcut. |
| U04 | 074 | End-to-end existing local web workflow | Opt-in serve workflow captures Inventory v1, selects job-bound targets, previews/saves context and reassesses the same immutable input without rescanning. Only valid completed session jobs or explicitly selected bounded inputs are eligible; input replacement invalidates bindings and requires explicit reselection. |
| U05 | 074 | Review, provenance and context round trip | Import/export adopted context using the shared layer; review influential claims, actors, dates and exact target scope before applying. No invented expiry/verification or silent scope widening; stale/conflicting/ambiguous input is surfaced or rejected according to existing semantics. Unsupported fields are not silently discarded during edit/export. |
| U06 | 074 | Bilingual accessible result view | Render structured evaluator results in the local UI with urgency/information/readiness/actions, evidence and influential/missing claims; preserve JSON/Markdown semantics and uncertainty. Paired EN/JA labels, keyboard access, focus/error/status handling and real-browser offline interaction evidence are required. |
| U07 | 074 | Bounded local integration and compatibility | Retain loopback/session-token/Host/origin protections; define bounded bodies/jobs/imports, allowed paths/artifacts, cancellation/failure and safe saves before adding APIs. No arbitrary file access, external upload, injected script, project execution or outbound reassessment. Old CLI/report/serve defaults stay compatible; UI and CLI evaluation agree for identical inputs/context/versions/time. |
| U08 | 075 | Measure assisted workflow operation | Freeze a bounded workflow-comparison protocol before observations; compare manual-JSON and assisted personal/organization scenarios using actual CLI/browser operations. Record completion, time, errors, skipped/unknown answers and next-action checks with denominators, operator authorship/exposure, failures and limitations. These controlled observations do not establish independent human usefulness. Reuse task071 practitioner evidence only when applicable; no additional recruitment is required or authorized. The amended task071 limited technical evaluation remains mandatory; human/scoring dimensions remain unmeasured. |
| U09 | 075 | Expanded final-source reconciliation | After tasks 071/072/074, reconcile all original 30 and added 9 clauses against final-source implementation, applicable tests/browser/study evidence and reviews. Original task 072 completion covers its original scope only; preserve historical records and separately bind new evidence. Release/support and numerical adoption remain separate. |


## Mandatory outcomes

| Requirement group | Outcome | Delivery obligation |
|---|---|---|
| V04-R1 | Audit Severity / Risk / Confidence for every built-in rule | Reuse the bound companion and review records; fill missing rationales and current-source gaps |
| V04-R2 | Improve standards-grounded PQC cryptographic assessment | Extend existing technical assessment only where necessary; preserve v1 semantics |
| V04-R3 | Explain organization-specific PQC migration priorities | Add a decision policy using technical facts and scoped context; distinguish urgency, unknowns and readiness |
| V04-R4 | Accept organization/system/asset-specific context | Bind local claims to the correct immutable evidence and usage targets; reassess without rescanning |
| V04-R5 | Evaluate synthetic technical correctness, specification conformance and reproducibility | Reuse existing source-bound PQS-QAM comparison; human/scoring dimensions remain unmeasured under explicit2026-11-19 amendment |

All detailed R-group rows and cross-cutting X rows below are mandatory within the
selected v0.4 scope. Every input format beyond the minimum requires an explicit
scope decision. Scope decisions cannot silently remove one of the five outcomes.

## Existing evidence

References identify existing implementations, specifications, or historical
measurements. They are not fresh test results for v0.4. Before reuse, compare their
source, inputs, configuration and environment with the implementation being
accepted. A `done` evaluation does not certify every product behavior.

| Reference | Existing asset | Reuse and limitation |
|---|---|---|
| E-RULE | [Rule companion](../../quality/rules/RULE-QUALITY-METADATA.json), [rule review guide](../../quality/rules/RULE-QUALITY.md), [binding test](../../../crates/pqc_scan_rules/tests/quality_metadata.rs) | 123 matching IDs/rows and eight matching source hashes checked read-only at baseline; not a new semantic audit. Old narrative counts are historical |
| E-TECH | [Technical evaluator](../../../crates/pqc_scan_core/src/assessment.rs), [assessment/policy tests](../../../crates/pqc_scan_core/tests/assessment_policy.rs), [local model contract](../../architecture/LOCAL-ASSESSMENT-V1.md) | Finite catalog, purpose/quantum/classical/components/assurance separation. Catalog capability does not guarantee normalized assets for every source hit |
| E-POLICY | [Policy/context implementation](../../../crates/pqc_scan_core/src/policy.rs), [assessment v1 schema](../../../schemas/assessment-v1.schema.json) | Versioned advisory primitive suitability, reasons, candidates and scan-wide claims; no full business-priority model |
| E-RISK | [Severity adjustment](../../../crates/pqc_scan_core/src/risk.rs), [threshold analysis](../../../tasks/056/THRESHOLD-ANALYSIS.md) | 0.45 is preserved legacy promotion behavior, not a calibrated probability or a justified new decision threshold |
| E-ID | [Inventory contract](../../architecture/INVENTORY-REPORT-V1.md), [inventory v1 schema](../../../schemas/native-scan-v1.schema.json) | Content objects and scoped/run-local occurrence references; neither universal business-system identity nor complete cross-scan correlation |
| E-RUNTIME | [Runtime contract](../../architecture/RUNTIME-PROBE-IMPLEMENTATION.md), [probe guide](../../user/PROBING.md), [protocol tests](../../../tests/protocol/README.md) | Separate TLS, SSH offers and SSH KEX records; retain exact verification/collection limits and experimental support status |
| E-JUDGMENT | [Task 056 results](../../../tasks/056/TASK-056-RESULTS.md), [guidance repairs](../../../tasks/056/TASK-056-FOLLOWUP.md), [judgment protocol](../../../tests/quality/judgment/PROTOCOL.md) | Source-based property/routing cases and disclosed manual review; not blind independent-author population truth. Five guidance findings have a completed repair record |
| E-QAM | [Quality model](../../quality/QUALITY-MODEL.md), [metrics](../../quality/QUALITY-METRICS.md), [gates](../../quality/QUALITY-GATES.md), [evidence catalog](../../quality/EVIDENCE-CATALOG.json), [CI measurement plan](../../quality/CI-MEASUREMENT-PLAN.md) | Reuse measurement, comparison, source binding and aggregation; do not infer v0.4 results from historical scores or initial model prose |
| E-COMPAT | [Task 058 results](../../../tasks/058/TASK-058-RESULTS.md), [compatibility tests](../../../crates/pqc_scan_report/tests/compatibility058.rs), [report guide](../../user/REPORTS.md) | Existing public/report checks; extend for new opt-in behavior and supported configurations |
| E-OUTPUT | [Safe output implementation](../../../crates/pqc_scan_core/src/safe_output.rs), [architecture boundary](../../architecture.md#report-destination-boundary), [report rules](../../agent/REPORTS.md) | Reuse output safety, sanitization and presentation checks; no new output bypass |
| E-V03 | [v0.3 requirements](../v0.3/REQUIREMENTS.md), [current reconciliation](../../../tasks/054/evidence/f054-fix-20261117/ROADMAP-RECONCILIATION.json), [v0.3.0 release notes](../../releases/v0.3.0.md) | Bounded prior acceptance and retained limitations; not an approval of new policies, network publication or unmeasured behavior |

## V04-R1: all-rule audit

Owner proposal: W1. Existing correct metadata and judgments are reused; rule and
legacy classification changes require separate impact analysis and applicable
human review. Unknown empirical rationale is recorded, not fabricated.

| ID | Required behavior | Existing evidence and gap | Acceptance and verification | Required future evidence |
|---|---|---|---|---|
| R1-01 | Enumerate every shipped rule in the selected source | E-RULE; bind the exact population at implementation time | IDs are unique; every loaded built-in has one source-bound audit entry; no missing/extra row; custom-pack acceptance is characterized | Population, snapshots, source hashes, missing/duplicate rejection results |
| R1-02 | Explain each rule's classification and evidence limits | E-RULE/E-TECH; complete or explicitly mark unknown the purpose, family, parameters, quantum/classical concerns, detection method, references and known FP/FN | Per-rule facts and limits are reviewable; current fixes are distinguished from historical findings; source/reference presence is not active use | Reused/extended audit with editions, limits, current finding dispositions and source links |
| R1-03 | Explain Severity, Confidence and promotion separately | E-RISK/E-JUDGMENT; existing value snapshots do not supply empirical rationale | Each base value and promotion condition has a rationale or explicit unknown; category/risk/evidence/threshold boundaries are covered; confidence is not probability or impact | Meaning definitions, rationale gaps, promotion decision table and boundary observations |
| R1-04 | Identify corrections without silently changing behavior | E-RULE/E-COMPAT; audit findings may require later approved repairs | Each proposed correction has affected rules, output/exit/custom-pack impact, review decision and positive/negative cases; no bulk rewrite to make the audit pass | Prioritized discrepancy list, impact analysis and approvals for any enacted repair |

## V04-R2: technical cryptographic assessment

Owner proposals: W2 and the W3 fact contract. Reuse the pure catalog and evaluator;
do not replace public Risk or assume all evidence already enters that evaluator.

| ID | Required behavior | Existing evidence and gap | Acceptance and verification | Required future evidence |
|---|---|---|---|---|
| R2-01 | Separate identity, purpose, standardization, quantum exposure and classical concerns | E-TECH; audit actual catalog/capture gaps | RSA signing/transport, ECDSA/ECDH, KEM/signatures, symmetric/hash models, exact parameters and unknowns are distinguished; no source-presence safety claim | Clause-to-judgment map and independent positive/negative/boundary cases |
| R2-02 | Preserve component and protocol meaning | E-TECH/E-RUNTIME; bridge only supported facts | Draft/final identifiers, hybrid components/combiner, certificate subject/signature roles, key establishment/authentication and protocol/provider readiness remain distinct | Exact identifier/role mapping, mismatch and unsupported-encoding cases |
| R2-03 | Keep observation and claim assurance explicit | E-ID/E-JUDGMENT; selected runtime adapters are new | Lexical/AST/config references, declared/resolved dependencies, parsed attributes, offers, negotiated facts and claims cannot be promoted beyond their verified scope; missing facts remain unknown | Evidence mapping with assurance gates and failure-path cases |
| R2-04 | Consume selected saved formats without changing their contracts | E-ID/E-RUNTIME; current static policy does not ingest probe envelopes | Inventory v1 and TLS v1 enter the new decision path offline; format/version/required facts/references are validated; partial TLS does not imply verified negotiation; unselected SSH formats are explicit unsupported inputs | Adapter contract, content/reference validation and success/partial/failure fixtures; D3 disposition |
| R2-05 | Trace new judgments while preserving old behavior | E-TECH/E-POLICY/E-COMPAT; shared helpers can affect v1 | Every adopted new judgment identifies catalog/standard edition and applicable clause; v1 policy/catalog results and legacy Risk stay unchanged absent explicit approved changes | Version dispatch/mapping, retained v1 goldens and source-bound regression results |

## V04-R3: migration decisions

Owner proposals: W3 specification, W4 implementation, W5 presentation and W6
validity evaluation. Policy conformance and operational usefulness are different
acceptance claims. D2 must settle output vocabulary before implementation.

| ID | Required behavior | Existing evidence and gap | Acceptance and verification | Required future evidence |
|---|---|---|---|---|
| R3-01 | Determine an explainable response using technical facts and applicable context | E-POLICY has simple priority; business/time inputs are new | An approved, versioned decision table defines outputs, branch-minimum inputs, precedence and reasons; representative system contexts produce justified different responses to the same algorithm | Decision table/tree, definitions, cited inputs and agreed expected cases |
| R3-02 | Separate urgency, information sufficiency and readiness | E-POLICY prerequisites are reusable; new output semantics needed | Unknown is not low priority; urgent-but-blocked retains urgency and states the dependency; known concerns survive incomplete context; investigation can itself be urgent; N/A is not a safety verdict | Positive/negative/unknown/conflict/readiness cases and output semantics review |
| R3-03 | Handle time without an invented quantum deadline | E-POLICY uses captured scan time; new time-dependent policy needs an explicit clock | Confidentiality and signing-verification lifetimes, certificate validity, historical ciphertext, lead time and scoped deadlines remain separate; boundary/expired/unknown dates are explained; no hidden Z or universal X+Y>Z decision | Time model, deadline provenance, boundary cases and evaluation-time binding |
| R3-04 | Show reasons, missing information and useful next actions | E-POLICY/E-COMPAT; new organization-specific guidance | Each decision identifies evidence, applied claims, missing inputs, constraints and next actions; unsupported claims or replacements are not invented; actions distinguish investigation/planning/dependency resolution/migration | Reason/action catalog, per-case expected payloads and human review |
| R3-05 | Reproduce decisions and explain changes | E-JUDGMENT/E-QAM; extend to new policy inputs | Same saved content/context/model/catalog/policy/time yields semantically equal results; changing an influential input explains a decision change; version changes are visible and do not alter old v1 results | Repeat evaluations, paired input cases, version-change and legacy comparisons |

## V04-R4: organization and asset context

Owner proposals: W3 joint contract, W4 bounded parsing/binding/reassessment and W5
provenance display. Context v1 remains a scan-wide operator declaration. A new
version, dispatch or file format is a proposed contract requiring D1 review.

| ID | Required behavior | Existing evidence and gap | Acceptance and verification | Required future evidence |
|---|---|---|---|---|
| R4-01 | Accept the minimum typed local context needed by the decision table | E-POLICY; system/asset scope and new policy/time fields are absent | Organization policy/applicability and per-target purpose, criticality, sensitivity, relevant lifetimes, deadline/lead time and support/constraints are represented where used; omitted/false/N/A are distinct; irrelevant fields are not mandatory | Field-to-branch map, approved schema/input limits and missing/false/type/duplicate-field cases |
| R4-02 | Bind a business system to the intended usage/observation | E-ID; algorithm and object IDs are not business identities | Explicit bindings identify the immutable input and selected targets; two systems using RSA can receive different contexts; IDs are retained; ambiguity, missing IDs and content mismatch are rejected or left unresolved per contract | Binding contract, content-integrity controls and same-algorithm/multiple-system fixtures |
| R4-03 | Apply defaults and overrides without leaking claims | E-POLICY scan-wide semantics cannot be repurposed silently | Precedence, applicability, duplicates, conflicts and scope are explicit; a claim cannot cross organizations/systems/purposes or arbitrary new scans; unresolved bindings are never guessed | Precedence table, cross-scope negative cases and v1 compatibility comparison |
| R4-04 | Preserve provenance, uncertainty and freshness | E-POLICY has fixed unverified provenance; per-claim scope/time are new | Observed/inferred/operator/external-review sources remain distinguishable; verification actor/scope/date/reference are recorded where applicable; a self-labelled verified value is not scanner verification; stale/conflicting claims do not pass silently | Provenance/freshness contract, expired/unknown/conflict cases and assurance review |
| R4-05 | Reassess the same saved evidence with different context | Current scan path evaluates in-process; saved offline workflow is new | Users can evaluate fixed captured content with contexts A/B without rescanning, executing scanned code or contacting an endpoint; results bind both input and selected context | CLI/library entry contract, paired-context results, no-network and immutable-input checks |
| R4-06 | Reject unsafe inputs and protect operator information | E-POLICY/E-OUTPUT; new fields broaden the input boundary | Finite size/depth/count/string/reference limits are approved; malformed/unknown/duplicate fields and invalid references have safe errors; no key/password/token input is required; synthetic secrets do not reach logs/reports | Limits and threat cases, negative parser/path/reference tests and secret-canary results |

## V04-R5: limited synthetic technical evaluation

Current scope is the explicit2026-11-19 amendment: technical correctness against
saved evidence/primary sources, adopted-specification conformance and fixed-condition
semantic reproducibility only. The original practitioner/M2/numerical study remains
archived and unexecuted; usefulness, actual human workload, rater agreement and
numeric-score validity/adoption remain Not Tested. Completion applies to revised scope.

| ID | Required behavior | Existing evidence and gap | Acceptance and verification | Required future evidence |
|---|---|---|---|---|
| R5-01 | Limited technical evaluation (maintainer amendment2026-11-19) | Existing PQS-QAM and archived original study | Freeze a24-case/12-pair source-separated AI-authored synthetic protocol, family-grouped8/16 tuning/final allocation, exact inputs/expectations/conditions before subject outputs; preserve old method/practitioner plan unexecuted. | Task071 v2 protocol, frozen cases, source checks, raw results and review records |
| R5-02 | Limited technical evaluation (maintainer amendment2026-11-19) | Existing PQS-QAM and archived original study | Main checks proposed technical facts against saved evidence and public primary clauses, and policy expectations against adopted P1-P8; record AI prompt/model visibility/sources/results/corrections/exposure. Unconfirmed facts are not successes; no human independent-verification claim. | Task071 v2 protocol, frozen cases, source checks, raw results and review records |
| R5-03 | Limited technical evaluation (maintainer amendment2026-11-19) | Existing PQS-QAM and archived original study | Measure technical property/case agreement, exact policy tuple/reasons/actions and semantic fixed-input reproducibility with explicit denominators, splits, failures and limits. Human practical usefulness, actual workload and inter-rater agreement remain Not Tested, observed0 and rates null. | Task071 v2 protocol, frozen cases, source checks, raw results and review records |
| R5-04 | Limited technical evaluation (maintainer amendment2026-11-19) | Existing PQS-QAM and archived original study | Draw no conclusion on numeric scoring validity, superiority/inferiority or adoption/non-adoption from this limited study; label that dimension unassessed. Preserve original planned comparison and no unapproved formula/model/threshold/adoption. | Task071 v2 protocol, frozen cases, source checks, raw results and review records |
| R5-05 | Limited technical evaluation (maintainer amendment2026-11-19) | Existing PQS-QAM and archived original study | Reuse existing PQS-QAM judgment comparison and evidence/report aggregation with source/input/context/model/catalog/policy/time binding, negative consistency controls and existing independent reviewers; preserve historical truth/failures and separate amended task completion from product quality/release. | Task071 v2 protocol, frozen cases, source checks, raw results and review records |

## Cross-cutting contracts and acceptance

Owner proposals: W3 contracts, W5 reporting and W7 integrated checks. These rows
apply across all five groups, not as optional polish after functional work.

| ID | Required behavior | Existing evidence and gap | Acceptance and verification | Required future evidence |
|---|---|---|---|---|
| X01 | Keep every existing public path compatible | E-COMPAT/E-ID/E-POLICY/E-RISK | New options absent: existing CLI/exit precedence, six reports, JSON/SARIF/CBOM/CycloneDX, Inventory/assessment v1, Risk/Severity/confidence, IDs and references remain unchanged for identical facts/configuration; approved corrections are separately explained | Legacy and custom-pack goldens, CLI errors/exits, schema/reference and applicable feature checks |
| X02 | Provide a traceable, separately selected decision document | E-POLICY/E-OUTPUT; new contract required | D1/D4-approved output includes input content/reference, target links, versions, context provenance, clock, technical assessment, decisions/reasons/missing information/readiness/actions/limits; no false aggregate safety claim; shared facts need not be duplicated | Schema positive/negative tests, broken-link/content-substitution controls and deterministic output checks |
| X03 | Make decisions usable in English and Japanese | E-COMPAT/report writers; existing default HTML has a different purpose | At least an opt-in bilingual Markdown view explains result/location/response/reason/next check; machine/human meaning and uncertainty agree; default views remain compatible; any changed presentation completes applicable browser checks | Paired rendered outputs, terminology/meaning review, escaping/privacy tests and required browser evidence |
| X04 | Preserve operational and privacy boundaries | E-OUTPUT/E-RUNTIME | Reassessment is offline, bounded and safe on invalid input; existing report destination protection is reused; ingestion never triggers probing; optional runtime support/publication status is unchanged | No-network/security/failure controls, safe-writer integration and support-scope comparison |
| X05 | Demonstrate every original requirement against final source | E-QAM/E-V03 | Every row maps to implementation and executed applicable evidence; independent implementation/test review gates apply to future executable changes; gaps/failures and reused-evidence applicability are explicit; no new hard gate or scope waiver is smuggled in | Final row-level reconciliation, commands/configurations/exits, source/input/tool hashes, review records and separate acceptance/release decision |

## Acceptance examples

The following are specification/evaluation cases, not measured outputs or adopted
new enum values. W3 must freeze branch-specific expected results and references
before using them to evaluate implementation. Human priority preferences require
an agreed policy; unknown business context must not be assigned invented truth.

| Case | Input and condition | Required observable distinction | Requirements |
|---|---|---|---|
| A01 | Source reference to RSA signing only | Technical concern retained; active use and business impact unresolved; identify the next verification | R2-01/03, R3-02/04 |
| A02 | Important authentication system with operator-declared RSA signing | Claim remains unverified; response reflects the declared system policy; signing/peer rollout prerequisites shown | R3-01/04, R4-01/04 |
| A03 | Long-lived confidential data with vulnerable establishment | HNDL and relevant lifetime/lead time considered; no invented quantum-arrival date | R3-01/03 |
| A04 | PQC API reference without deployment evidence | Standardized primitive is not system readiness or verified use | R2-01/03, R3-02 |
| A05 | Successful verified TLS with an identified ML-KEM hybrid group | Negotiated KEX scope preserved; combiner/protocol mapping cited; no automatic PQC-authentication claim | R2-02/04, R3-04 |
| A06 | PQC-capable library in a manifest or lock | Declaration/resolution retained; enabled provider and deployed PQC remain unproved | R2-03, R3-02 |
| A07 | Unknown purpose; separately, unknown system criticality | Missing inputs affect only dependent branches; known concern survives; no low-priority fallback | R3-01/02, R4-01 |
| A08 | Applicable deadline at a lead-time boundary | Explicit clock, source and time boundary explain the response; no universal NIST deadline | R3-03/05 |
| A09 | Urgent migration with no supported replacement or unready peer | Urgency retained while action records the blocking prerequisite | R3-02/04, R5-03 |
| A10 | Hybrid key exchange with classical certificate authentication | Establishment and signing exposure are separate and visible | R2-02, R3-04 |
| A11 | Partial inspection or failed TLS verification | Preserve safe facts/known concerns; no verified negotiation or clean global verdict | R2-03/04, X02 |
| A12 | Two systems using the same normalized RSA algorithm | Separate usage bindings and claims; no propagation through the shared algorithm ID | R4-02/03 |
| A13 | Same immutable evidence, contexts A and B | No new scan/connection; exact content binding and influential input differences explain outputs | R3-05, R4-05 |
| A14 | Missing IDs, replaced content, ambiguous duplicate occurrences or conflicting bindings | Reject or retain unresolved status as specified; no guessed system identity or cross-input reuse | R4-02/03, X02 |
| A15 | Expired claim or operator-supplied verified label | Freshness/actor/scope visible; no automatic independent-verification promotion | R4-04 |
| A16 | SSH offers and SSH KEX input records | Preserve advertisement/negotiation/authentication boundaries if selected; otherwise reject explicitly as unsupported | R2-02/04, D3 |
| A17 | No new options; old context/profile including 0.45 boundaries | Existing outputs, priorities and exits retain their meaning | R1-03, R2-05, X01 |
| A18 | Malformed/oversized context, path/reference abuse, synthetic secrets | Safe bounded rejection; no network, panic, hang or secret disclosure | R4-06, X04 |

## Decisions, verification and completion rules

Open D1-D7 in [ROADMAP.md](ROADMAP.md#maintainer-decisions-before-affected-implementation)
identify contract, policy, runtime scope, output, rule/classification, study and
score-adoption decisions. Record their resolution before dependent implementation.
Until then, names/fields/limits/thresholds are proposals, not public contracts.
No chosen score, policy date, storage change or support expansion is approved here.

For future changes, use the pinned toolchain and applicable checks in
[VERIFICATION.md](../../agent/VERIFICATION.md), the two stages in
[REVIEWS.md](../../agent/REVIEWS.md), and applicable report browser checks.
Targeted passing results do not replace required workspace/configuration checks.
Independent review budgets remain per authorized request; work packages do not
grant extra agent identities. Practitioner-study participation is a separate
resource/authority requirement, not an excuse to fabricate independent evidence.

The final v0.4 acceptance record must identify each row's implementation, selected
scope, actual verification, exact source/input/context/model/time bindings,
reviewed evidence and remaining limits. Required work that is Partial, Unknown,
Not Tested, failed or deferred prevents completion; a justified bounded assessment
result must not be confused with an unexecuted assessment. Preserve earlier
truth and failures. Do not average dimension scores to bypass an unmet row.

No v0.4 product behavior, comparison experiment, expert study, compatibility test,
browser check or release has been executed by this document's authorship. This
planning and registration requests end after the paired planning documents,
task definitions/map, documentation/dependency verification and the requested
independent document review. Implementation requires a subsequent instruction;
the authorization to register definitions is not authorization to execute them.
