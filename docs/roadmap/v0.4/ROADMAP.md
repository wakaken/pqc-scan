> Current maintainer amendment2026-11-19: task071 uses a fresh-context AI-authored
>24-case/12-pair limited technical correctness/conformance/reproducibility study.
> The original human/M2/numeric evaluation is preserved and unexecuted; human utility,
> workload, rater agreement and score validity/adoption remain unmeasured. Revised R5
> requirements and task071/EVALUATION-PROTOCOL-v2.md govern current071/072/075 acceptance.
> Two practitioners/human technical adjudicator are not gates for this limited scope.
> One extra case-author AI only (no redelegation) is authorized; existing2 reviewers
> are reused. No score adoption, release/publication, outreach or fees are authorized.
> Earlier resource-block/completion narrative below remains historical, not current scope.

# v0.4 roadmap: Risk & Migration Intelligence

[日本語](ROADMAP.ja.md)

Status: proposal for maintainer review. Prepared on 2026-11-18.

This roadmap adds organization-specific migration decisions to the existing
cryptographic evidence and assessment pipeline. It does not replace the scanner,
the inventory, or `migration-review-v1`. The five required outcomes and their
acceptance evidence are defined in [REQUIREMENTS.md](REQUIREMENTS.md).

At initial registration, this roadmap and its [task map](TASKS.md) authorized
planning and definitions for tasks 065-072 only. Later execution and completion
are recorded in the task definitions and the map's execution update; the initial
pending states are historical. The present extension authorizes definitions for
073-075 only, not their implementation. Registration does not approve a release
or an unspecified schema/policy change.

## Authorized input-assistance extension (2026-11-18)

The maintainer authorized planning and registration of tasks 073-075 on
`codex/v04-roadmap-20261118`, not implementation. This dated extension takes
precedence over the original exclusion of new UI work only for an opt-in
extension of the existing local `serve` interface. SaaS, a separate web
application, a database and CLI conversational input remain outside this scope.
The five mandatory outcomes and all original acceptance clauses remain intact.

Reuse SavedEvidence, MigrationContext validation/binding, the pure migration
evaluator, safe report writers, the final task-070 human presentation and the existing loopback server. Add a common
configuration-generation layer and an explicit template-generation entrypoint;
then use that same layer from the existing local web interface. No second
scanner, decision table, browser-side evaluator or alternate context schema is
planned. The generated configuration uses the adopted migration-context v1
contract. Public entrypoint names and bounded local HTTP request/response details
must be specified before implementation; this roadmap does not declare new
commands available. Record a concrete D4 supplement before affected UI/API work;
any necessary stored-contract change still requires the separate D1 decision.

The proposed flow is scan (or select an existing session result), inspect targets,
answer only known relevant questions, review and save the generated context,
reassess the same saved bytes, and inspect decisions/reasons/missing information
and next actions. Web scanning must explicitly obtain Inventory v1 for this
selected workflow; the existing serve path must not be assumed to produce it.
The UI uses job-scoped targets and content bindings rather than asking users to
copy hashes/IDs. Reassessment and context import/export never rescan, execute
project code or initiate outbound connections. Existing scan/report defaults stay
compatible when the new workflow is not selected.

Individual users can skip context entirely or provide only known purpose/use
claims. Organization/system aliases can be generated without asking for a company
or department. Personal/organization presentation does not select a different
risk model. Unknown, omitted, false and not-applicable remain distinct. Questions
are purpose-dependent; no business deadline, criticality, protection lifetime,
support status, expiry or factual verification is invented. Existing
business_critical semantics must not be relabelled into a different claim.
Provide a readable source/target selector; ambiguous targets cannot acquire a
binding. Grouped application requires explicit review of the exact target set.

Tasks 065-070 and their completion/review records are preserved. Task 071 remains
responsible for the original mandatory study and its unresolved D6 resources;
new input assistance cannot substitute for it. Task 072 retains its original
30-row reconciliation scope. Task 075, depending on071/072/074, supplies the
additional workflow observations and final reconciliation of all 39 rows (30
original plus U01-U09). Completion of072 alone cannot close the expanded v0.4.
No existing protocol or historical result is silently amended: freeze the added
workflow-comparison protocol before its observations and retain separate versions/evidence.

Acceptance requires common-helper/CLI/web configuration equivalence, headless
operation, bounded session-local inputs, explicit ownership and freshness,
existing security and output protections, bilingual accessible forms/results,
real-browser interaction evidence, and measured personal/organization scenarios.
Controlled operator observations are not independent practitioner validation; no
additional recruitment is a new prerequisite. Task071 remains mandatory.
An unavailable required observation keeps the extension unfinished. See the
[additional requirements](REQUIREMENTS.md#input-assistance-extension) and
[task map](TASKS.md#input-assistance-extension).


## Baseline and relationship to v0.3

The source baseline is `origin/develop` at
`3f2bc6e6209e1f2813caac8077c8d7a7198832cc`, fetched on 2026-11-18. The planning
branch is `codex/v04-roadmap-20261118`, created in a clean, separate worktree.
The original local checkout at `929caed4caf36461ee6d7fc9a81491807b2fdcb4` has
uncommitted work; none of it was copied or used as the implementation baseline.
Before future implementation, pin the then-selected source and reassess evidence
applicability rather than assuming this baseline is still current.

The baseline product version is `0.3.0`. Its [release notes](../../releases/v0.3.0.md)
and [publication decision](https://github.com/wakaken/pqc-scan/blob/a915b3d4f4381aa8308f18a50bb864b54493c72c/docs/releases/static-candidate-20261117/PUBLICATION-APPROVAL.md)
record bounded acceptance of static macOS Apple Silicon operation and conditional
publication authorization. Preparation is not proof that release assets have
been published. TLS/SSH source-build features remain experimental; network
publication is still held. v0.4 does not expand those support or release claims.

The current [v0.3 roadmap](../v0.3/ROADMAP.md) and
[task 054 reconciliation](../../../tasks/054/evidence/f054-fix-20261117/ROADMAP-RECONCILIATION.json)
record completion of the original finite quality-evaluation scope. Task 052's
[reassessment](../../../tasks/052/TASK-052-REASSESSMENT-20261116.md) supersedes its
earlier blocked assessment. The original task 056 evaluation and its
[five guidance repairs](../../../tasks/056/TASK-056-FOLLOWUP.md) are separate,
preserved records. Earlier incomplete decisions and measured failures are not
current blockers merely because they still appear in historical prose.

Acceptance within a declared scope does not establish universal detection
accuracy or deployment safety. The release notes retain limits including nominal
source-aware controls, nine unobserved invariant directions, auxiliary LLVM
failure, a partially completed additional fuzz campaign, and same-host rebuild
scope. These limits remain visible; this roadmap neither repairs unrelated
issues nor relabels old measurements. Fresh v0.4 behavior needs its own evidence.

### Existing assets and the actual increment

| Required outcome | Existing asset | Proposed increment | Treatment |
|---|---|---|---|
| V04-R1: all-rule audit | Source-bound metadata for all 123 built-in rules; task 028 and 056 reviews | Fill missing severity/confidence rationale, promotion analysis and current applicability | Reuse and extend the audit; no bulk rule rewrite |
| V04-R2: technical classification | Pure `assessment` functions and a finite, versioned catalog | Close explicit classification and evidence-ingestion gaps; retain old behavior | Extend shared assessment logic; keep legacy contracts |
| V04-R3: migration decisions | Versioned primitive-suitability policy, reasons, candidates and simple priority | Explain organization-specific urgency, missing information, readiness and actions | Add a distinct decision policy using shared technical facts |
| V04-R4: organization and asset context | Strict, bounded, scan-wide operator context v1 | Bind scoped claims to systems and usage occurrences; reassess saved evidence | Extend input handling through a separately selected contract |
| V04-R5: limited technical evaluation | PQS-QAM, existing judgment comparison and source-bound evidence |24 AI-authored synthetic cases/12 pairs: technical correctness, adopted specification and reproducibility | Human utility/workload/rater agreement/numeric validity remain unmeasured; no parallel quality platform |

Read-only enumeration found 123 unique rule IDs and 123 matching metadata rows.
All eight rule-file SHA-256 bindings in the companion matched this baseline.
This is a population/binding check, not a new semantic audit or detection test.
The companion's older prose counts must not override the actual rule set. The
evidence index in [the requirements](REQUIREMENTS.md#existing-evidence) identifies
source files and the limits of each existing asset.

## Product outcome and boundaries

A user should be able to apply explicit local context to captured evidence and
understand which migration investigation or action deserves attention, why, and
what remains unknown. A source reference, configured use, parsed certificate,
declared dependency, observed negotiation and operator claim remain different
kinds of evidence. A standardized PQC primitive is not proof of deployment,
interoperability, trusted authentication or completed migration.

All five outcomes are mandatory. A new numerical product score is optional; the
evaluation of numerical scoring is mandatory. The following are outside the
proposed v0.4 scope: SaaS, a new web application, database services, a generic GRC
or CMDB, cross-scan automatic identity tracking, an arbitrary policy language,
external evidence-certification services, automatic endpoint discovery, and
replacement of TLS/SSH transports. Removing a mandatory outcome or reducing its
acceptance scope requires explicit agreement, not a change of work-package label.

### Minimum evidence scope

The proposed delivery scope includes saved Inventory v1 and saved TLS v1
observations. TLS input is evaluated offline and does not make the experimental
network package a supported public distribution. Static findings retain their
original capture limits; a lexical hit without an algorithm asset is not silently
promoted to a fully specified primitive. The audit must identify these capture
gaps; any required bounded repair needs a specification and compatibility review.

The treatment of SSH offers v1 and SSH KEX v1 is a maintainer decision at D3 below.
The design must compare their adapter cost and retain their original assurance
limits. Until a format is selected and verified, it must be explicitly rejected
or reported unsupported, never treated as negative evidence. Excluding SSH does
not remove the requirement to model advertisement versus negotiated evidence;
the decision specification and boundary cases must preserve that distinction.

## Recommended architecture and alternatives

Use the existing collection paths and pure technical assessment functions.
Introduce a bounded connection from supported saved records to a migration
decision policy. Combine those facts with explicitly scoped organization and
system claims, then produce a separately selected decision report. Keep report
writers responsible for rendering and validation, not policy evaluation.

This is a responsibility map, not a requirement to create crates, a second
catalog, or a general intermediate framework. Preserve `migration-review-v1`
and Context v1, including their scan-wide claim semantics and existing priority.
The current evaluator's `high` / `medium` / `review` values must not acquire the
new decision model's meaning. New catalog behavior must not silently change the
frozen v1 policy through a shared helper.

| Approach | Change and benefit | Cost or limitation | Recommendation |
|---|---|---|---|
| Existing formats plus narrow adapters | Keep Inventory/probe schemas and collection; reuse technical evaluation | Explicit mapping and source-reference validation are needed | First choice |
| Small common assessment input | Share only facts required by selected decision branches | May drift into a second inventory if not bounded | Use internally only when adapters demonstrate a real need |
| New integrated Inventory major version | Unifies storage, identities and coverage | Broad migration of readers, references and compatibility evidence | Do not adopt without a demonstrated unmet requirement and human review |
| Replace existing policy/model | One apparent path | Changes published v1 meaning and duplicates prior work | Reject as the default approach |

### Context binding and identity

Cryptographic algorithm definitions, keys, certificates, source usage occurrences,
connection attempts, and business systems are different entities. A normalized
RSA asset ID cannot identify which business system uses RSA. The first proposal
is an external, explicit binding between a system and selected usage/observation
references in an immutable input. Keep existing IDs unchanged.

Bind context to input content as well as its IDs. A run ID or filename alone is
insufficient. W3 must choose a deterministic content-binding method and define
the treatment of provenance, schema version and evaluation time. Repeated or
ambiguous occurrences cannot inherit a system's claims by guesswork. Reuse across
scans requires a fresh explicit binding; automatic tracking is not required.

Define organization defaults, explicit per-target overrides, absence, explicit
false, conflicts, expiration and stale references. Do not use file globbing,
algorithm names or a successful structural check as proof that an operator has
identified the correct real-world system. Mechanically invalid references must
be rejected; well-formed false claims remain a disclosed assurance limit.

### Decision semantics

The recommended model separates response urgency, information sufficiency and
migration readiness. These may be separate fields or an equivalently explicit
representation; D2 settles names and encoding. `Prioritize`, `Plan` and `Monitor`
are proposed action classes, not adopted public enums. `Needs Investigation` is
not a low-priority rank. `Not Applicable` means outside the selected policy, not
quantum-safe. Existing policy `Pass` / `Fail` remain separate.

Each branch needs only the inputs that affect that decision. Missing unrelated
fields must not block a supported urgent action. A known concern survives
incomplete context. An unavailable replacement does not reduce urgency; it
changes the action to investigation, planning or dependency resolution. Do not
multiply urgency by heuristic confidence or impose a total ranking on unresolved
cases. The versioned decision table must define precedence and reasons.

Keep confidentiality lifetime, signature-verification lifetime, certificate
validity, migration lead time and an applicable deadline distinct. Preserve
historically harvestable ciphertext risk. A quantum-computer arrival estimate
must not be silently supplied by the product; `X + Y > Z` is not a complete or
universal decision rule. Every deadline needs a source, scope and applicability.

### Saved-evidence reassessment and reports

Compare a new explicit offline assessment entry point with extending the current
scan-time path. The proposed user workflow must support applying different
contexts to the same saved evidence without rescanning or opening a connection.
Command names, new context/report versions and schema shapes remain proposals
until D1/D4 review. Reuse the existing reference validators and safe output path.

The output must identify input content, observation references, selected context,
model/catalog/policy versions, evaluation time, technical concerns, decision
reasons, missing information, readiness and next actions. Common records may be
referenced instead of copied into every row. Unsupported formats and partial
inspection remain visible.

The proposed minimum human output is an explicitly selected bilingual Markdown
decision view alongside machine-readable results. Compare an opt-in HTML section
with that minimum at D4; a new UI is unnecessary. Readers must see what was found,
where, the recommended response, its reason and the next useful check. Preserve
all default report behavior. Future presentation changes require the browser
checks in [REPORTS.md](../../agent/REPORTS.md), not source inspection alone.

## Work packages and sequence

W1-W7 are logical planning labels. Their registered task IDs and primary
requirement ownership are listed in [TASKS.md](TASKS.md). Registration does not
start a package. All are required for the selected v0.4 scope; lower sequencing
priority does not waive an outcome.

| Package | Requirements | Output and completion condition | Depends on | Relative change |
|---|---|---|---|---|
| W1: baseline and audit | R1; baseline of all R groups | Source-bound population, reused audit, missing rationales and affected-rule impact analysis; D1/D3/D5 scope inputs | Selected source | Low to medium: metadata and source review; no automatic rule edits |
| W2: technical facts | R2 | Approved classification/evidence mapping and finite gap corrections with independent cases; old catalog behavior remains reproducible | W1, D5 | Medium: shared catalog and limited adapters; unknowns stay explicit |
| W3: joint decision/context specification | R3, R4, R5-01/02 | Decision table, minimal fields, binding/precedence/clock contract, evaluator study protocol and frozen expectations; D1-D6 resolved where blocking | W1; W2 fact contract | Medium to high: business-to-usage mapping is the main new design |
| W4: context and decision implementation | R3, R4, R2-04 | Explicit offline reassessment and selected-input adapters implement W3; targeted negative/boundary checks and implementation review | W2, W3; applicable human decisions | Medium to high: new policy and bounded inputs, not scanner replacement |
| W5: decision reporting | R3-04, R4-05, X01-X03 | Selected machine-readable and bilingual human views; exact reference/meaning/privacy validation and applicable browser checks | W3 output contract, W4 | Medium: reuse writers and safe output, no new UI |
| W6: amended technical evaluation | R5; bounded R3 conformance | Execute frozen24/12 source-separated AI synthetic cases; main confirms facts/expectations before outputs, retains failures/unknowns and repeats | W3 adopted policy; W4/W5; explicit2026-11-19 authority | No human expert/practitioner or score-validity conclusion |
| W7: integrated acceptance | All rows | Final-source requirement/evidence reconciliation, applicable checks and independent review stages; separate product/release disposition | W1-W6 | Medium: extend QAM aggregation and existing compatibility suite |

Prepare the evaluation protocol and expected cases in W3 before observing W4/W6
results. Classification facts may evolve in W2, but implementation must not
precede the joint context/decision contract. W5 may specify presentation against
the W3 contract while W4 proceeds. Keep one coherent implementation per requirement
even when several packages consume it. The task/PR boundaries are in TASKS.md;
further registration or execution requires the applicable separate instruction
under the repository's lifecycle rules.

For each future package, record its purpose, exact scope and requirement IDs,
reused sources/evidence, affected interfaces, dependencies, acceptance cases,
verification, required artifacts, risks and approval boundaries. Do not derive
calendar estimates from these relative change labels.

## Evaluation strategy and scoring disposition

Reuse [PQS-QAM](../../quality/QUALITY-MODEL.md), the
[judgment protocol](../../../tests/quality/judgment/PROTOCOL.md) and existing
[evidence aggregation](../../quality/EVIDENCE-CATALOG.json). Do not create another
top-level quality system. Distinguish technical fact correctness, conformance to
the approved decision table, and practitioner usefulness. A table implemented
correctly does not independently validate the table's business value.

The baseline method is an explicit categorical decision table/tree. Screen an
ordinal matrix and a justified numerical or multi-criteria method as comparison
candidates. Select at least two comparable methods under the same cases,
information and policy; do not implement five product engines merely to compare
labels. Categories and numbers are output representations, not alternatives to
decision trees. Reject an unjustified numerical candidate with documented scale,
weight or missing-data failures rather than inventing parameters.

The study must specify recruitment/roles, case selection, expected technical
facts, policy authority, adjudication, implementation exposure, tuning/holdout
separation and resource bounds before execution. No real human expert or
independent-author dataset has been secured by this planning work. AI review
does not fill that role. Correlated synthetic cases support regression, not
population estimates. Existing task 056 cases are not rebranded as blind data.

Measure explanation traceability, agreed-action consistency, harmful
under-prioritization, unknown handling, input sensitivity, defensible ordering,
input effort and inter-rater disagreement. Use paired cases for a known urgent
asset with an unavailable replacement, missing context, different systems using
the same algorithm, and partial inspection. Measure same-version reproducibility
separately from explainable changes between policy versions. Proposed thresholds
and sample-size rationale require review before results are seen; no unapproved
new hard quality gate is introduced here.

The eventual disposition is supported adoption, supported non-adoption, or
insufficient evidence. Insufficient evidence is not proof of numerical inferiority.
Unexecuted mandatory evaluation remains unfinished. A justified product decision
to withhold a numerical score is compatible with completing a valid bounded
comparison; omitting the comparison is not. Adoption requires a separate approved
formula, scale, model name/version, uncertainty statement and compatibility plan.

## Standards and their limits

Publication/status checks below were performed on 2026-11-18. They confirm
planning references, not full standard conformance or a new deadline table.
Reuse the [existing register](../../product/STANDARDS-REGISTER.md) and task 056's
source mapping. W1-W3 must pin relevant sections, errata dispositions and exact
applicability for every adopted behavior before implementation.

| Issuer and reference | Edition/status checked; relevant material | Planning use and limit |
|---|---|---|
| NIST [FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) | Final, 2024-08-13; abstract and update notice | ML-KEM purpose/parameters; no protocol deployment claim |
| NIST [FIPS 204](https://csrc.nist.gov/pubs/fips/204/final), [FIPS 205](https://csrc.nist.gov/pubs/fips/205/final) | Finals, 2024-08-13; abstracts and publication notices | ML-DSA/SLH-DSA signing; do not equate draft names or infer safe implementation |
| NIST [IR 8547](https://www.nist.gov/publications/transition-post-quantum-cryptography-standards) | Final publication dated 2026-10-27; publication record/abstract; existing register cites sections 2.2, 3.1 and 4.1 | Migration rationale; federal scope is not a universal deadline. The CSRC fetch timed out; the alternate official publication record was available |
| NIST [SP 800-131A Rev. 2](https://csrc.nist.gov/pubs/sp/800/131/a/r2/final), [Rev. 3 draft](https://csrc.nist.gov/pubs/sp/800/131/a/r3/ipd) | Rev. 2 Final, March 2019; Rev. 3 IPD, 2024-10-21; publication/planning notes | Preserve the v1 policy's Rev. 2 thresholds; draft presence does not revise them |
| NIST [SP 800-57 Part 1 Rev. 5](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final), [Rev. 6 draft](https://csrc.nist.gov/pubs/sp/800/57/pt1/r6/ipd) | Rev. 5 Final, May 2020; Rev. 6 IPD, 2025-12-05; publication records | Key lifetime/protection context; exact adopted provisions still need mapping |
| NIST [SP 800-227](https://csrc.nist.gov/pubs/sp/800/227/final) | Final, September 2025; abstract/publication record | KEM usage and limits; no KEM-as-signature substitution |
| NIST [CSWP 39upd1](https://csrc.nist.gov/pubs/cswp/39/upd1/considerations-for-achieving-crypto-agility/final) | 2025-12-19 publication, updated 2026-06-29; abstract/status | Migration constraints and crypto agility; not a readiness certification |
| NIST [SP 800-30 Rev. 1](https://csrc.nist.gov/pubs/sp/800/30/r1/final) | Final, September 2012; purpose/abstract | Risk-assessment context; no mandated pqc-scan score |
| FIRST [CVSS v4.0 User Guide](https://www.first.org/cvss/v4.0/user-guide#CVSS-Base-Score-CVSS-B-Measures-Severity-not-Risk) | Section on Base severity versus risk | Severity is an input, not the organization's migration priority |
| CMU SEI [SSVC 2.0](https://www.sei.cmu.edu/library/prioritizing-vulnerability-response-a-stakeholder-specific-vulnerability-categorization-version-20/) and CERT/CC [customization guidance](https://certcc.github.io/SSVC/howto/tree_customization/) | 2021-04-30 report; current guidance on features, parsimony and versioning | Decision-table design; no claim of an official PQC SSVC model |
| CISA [SSVC guide](https://www.cisa.gov/sites/default/files/publications/cisa-ssvc-guide%20508c.pdf) | Retrieval returned HTTP 403; edition-specific claims not newly verified | Supplemental reference only until available; no CISA-specific branch copied on this evidence |

Protocol mappings must reuse the selected RFC/OID references in the existing
register, keeping TLS group, authentication, SSH offer and KEX semantics separate.
A selected adapter needs its own clause-to-field review and fixtures. SP 800-37,
SP 800-53 and OWASP risk methods are supplementary only if a concrete requirement
needs them; this roadmap does not initiate a general compliance or GRC program.

## Maintainer decisions before affected implementation

| Decision | Recommended proposal | Boundary and dependency |
|---|---|---|
| D1: context/assessment contract | Separate opt-in version; retain v1 semantics; bind exact saved input content | Approve input/report storage contracts, scope and dispatch before W4/W5 |
| D2: decision vocabulary and precedence | Separate urgency, sufficiency and readiness with explicit actions/reasons | Approve the decision table, branch-minimum inputs and unresolved-case ordering in W3 |
| D3: runtime input scope | Saved TLS v1 required in the proposed minimum; compare SSH offers/KEX adapters without changing probe support | Select the exact runtime formats and fixture obligations before W2/W3 contract closure; no implicit SSH support |
| D4: human output and CLI entry | Explicit saved-evidence reassessment, structured report and bilingual Markdown; assess opt-in HTML cost | Approve command/schema names and presentation scope; defaults unchanged |
| D5: audit/classification repairs | Reuse correct current judgments; propose only evidence-backed gaps | Rule meaning, confidence/severity, high-risk architecture and unspecified crypto logic retain AGENTS human-review gates |
| D6: limited evaluation authority/resources | Explicit2026-11-19 amendment; maximum1 fresh-context nondelegating case-author AI, main source/specification verification,24/12 family-grouped freeze | Original two practitioners/human adjudicator removed only as limited-scope gates; original96 human judgments remain unexecuted |
| D7: numerical product score | Validity/superiority/adoption not evaluated by limited071 | No adoption/non-adoption conclusion; future formula/model/support/publication needs separate authority |

These are open product/design decisions, not requests for permission to perform
the already authorized documentation work. Recommendations are not adopted public
contracts. No arbitrary limits, policy dates, score weights or support expansions
are approved by inclusion here. Decisions may refine the design but cannot remove
a mandatory outcome without explicit scope agreement.

## Verification, evidence and completion

For future executable changes, follow [VERIFICATION.md](../../agent/VERIFICATION.md)
and [REVIEWS.md](../../agent/REVIEWS.md): targeted checks, all applicable final
checks/configurations, implementation review followed by test review, scoped
follow-up and final-source evidence. Review budgets remain per authorized request,
not per package. Use [REVIEWER.md](../../agent/REVIEWER.md). Do not create extra
reviewers or independent study participants by renaming roles.

Extend existing judgment/schema/privacy/compatibility tests. Preserve the old
0.45 behavior, legacy outputs, identity/reference contracts, explicit probe
authorization and safe output handling. Compare supported feature configurations.
Saved-evidence evaluation must not initiate network requests. Real protocol tests,
when separately authorized, use the existing isolated loopback policy. The
[acceptance examples](REQUIREMENTS.md#acceptance-examples) cover positive,
negative, unknown, conflict, readiness and version boundaries. Passing adapter
execution or table agreement alone does not establish practitioner usefulness.

Future evidence must bind source including uncommitted changes, inputs/truth,
rules, model/policy/catalog/context versions, evaluation time, tool/environment,
commands, feature configuration, exit status and outputs. Keep failures, skipped
checks, disagreement and reused-evidence applicability. Reuse existing artifact
and aggregation conventions; do not add a second generic evidence database.

v0.4 is complete only after every mandatory row in REQUIREMENTS.md has final-source
implementation and applicable verified evidence for the selected scope. Neither
an artifact list, partial implementation, `Unknown` display, a task count, nor
withholding a numerical score substitutes for required work. Product acceptance,
support and release/publication remain separate decisions. This planning change
does not assert v0.4 implementation, measured quality, approval or release.

For the planning and registration requests, completion requires the paired
planning documents, task definitions and ownership map, source/link/dependency
and cross-document consistency checks, and the requested independent document
review with findings resolved. No product build, test, scan, probe or measurement
is claimed. The reviewer identity, source hashes, decision and final verification
are retained in the current chat as allowed for work without a task for this
documentation activity.
