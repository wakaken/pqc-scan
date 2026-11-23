# Migration review grouping and interaction mock

Status: design/mock deliverable requested2026-11-19. The maintainer approved the
system/use-case direction and requested concrete grouping rules, three large-input
screens and report-size prevention. This does not approve production storage,
policy, safety-bound or release changes. Existing product code and historical
completion/evaluation records are preserved. Main owns all design/code/verification;
reuse the existing implementation/test reviewer identities, no extra agents.

## Outcome and scope

A user should see useful investigation work before supplying context, confirm a
system/use once, and reach a small action-oriented overview without selecting
thousands of rows. Keep exact evidence, uncertainty and urgency traceable. This
mock demonstrates information architecture and scoped input, not a new evaluator.

The three screens are Overview, Confirm use and Response work items. They share the
approved dark/light palette and minimal Japanese copy. Location/evidence drawers
are secondary, paged20 and read-only; they are not the default input workflow.
UI answers are explicit examples stored in page memory. They never call product
APIs, generate a real context file, re-evaluate policy or claim practitioner utility.

## Layers and deterministic grouping rules

### G1. Evidence is preserved, not merged into a new fact

The immutable source is the complete saved Keycloak migration assessment, with8750
targets/decisions at as_of2026-11-19T04:20:18Z. Each target retains its ID, associated
decision, source/claim/candidate links and original technical/observation/limitations.
Unbound/ambiguous records remain visible evidence. A list record is not a live
business use or independent migration job. No source fact is upgraded by grouping.

### G2. Location partition is an ordered, disjoint navigation aid

Assign each path once, in this priority order (case-sensitive captured path):

1. A slash-separated path component exactly test, tests, testsuite or test-framework
   → test-labelled location. Substrings such as contest or Test.java do not match.
2. Basename exactly pom.xml, package.json, package-lock.json, pnpm-lock.yaml,
   yarn.lock, Cargo.lock or Cargo.toml → dependency declaration.
3. First path component js → frontend.
4. Suffix .java/.rs/.go/.py/.js/.ts/.tsx/.c/.h/.cpp/.kt/.rb → source code.
5. Everything else (including absent path) → other.

Labels describe captured locations only. The partition neither establishes
production/nonproduction, active use, deployment boundary nor applicability. No
bucket is silently excluded from the overview or response work items. Paths cannot
establish a business-system binding. Dependency records are declarations, not proof
of runtime dependency use.

Exact Keycloak counts: source669, frontend235, dependency2211, test-labelled5623,
other12; total8750. This data-specific partition is not a universal classifier.
Any future path taxonomy change must be versioned/reviewed with partition controls.

### G3. Technical facets constrain input candidates

Facet signature only when the saved technical purpose is signature. Facet key/
certificate for captured roles certificate.subject_key.algorithm or
public-key.algorithm when not already signature. All other records are unresolved.
Retain the precise captured role and every technical difference in evidence. Facets
are grouping hints, not declarations of JWT/PKI/business purpose or active use.

The mock's editable form demonstrates one signature-family use. Scope checkboxes
are location buckets containing signature facets: source354, frontend139,
test-labelled2595. Unknown-purpose references, dependency declarations and key-only
records do not acquire a signature answer. Their own investigation items remain.
Future forms for other purposes need the existing purpose/role compatibility gates;
this mock does not invent a KEM/encryption/PKI mapping from a generic key role.

### G4. Confirmation, ambiguity and ownership gate shared answers

A user selects the scope and submits one answer set for the chosen system/use.
A fresh form starts with no selected location. The source-only cohort contains354 candidates:121
eligible,233 with unresolved identity. Only121 can acquire the example answer.
The scope summary and drawer distinguish both sets. The Apply button submits the
currently displayed exact membership. Selecting a location is a user scope choice,
not independent evidence of a correct business mapping.

The mock eligibility hint uses the saved adapter's unknown_binding limitation for
this pinned inventory. Product implementation must use SavedEvidence/BoundContext
eligibility directly; absence of a string is not a general validation substitute.

Existing saved-use membership owns records. Adding another use cannot silently
transfer/overwrite them; overlapping records are counted separately and withheld.
Edit the original scope to split it, or explicitly remove its answer. A module
exclusion dialog operates on exact captured first path components; affected members
are visible in the scope drawer. The page binds the displayed scope to the exact eligible member sequence and the
pinned report digest. Scope/ownership changes refresh the visible count before an
explicit Apply click; a stale displayed snapshot cannot be committed. Removing an
existing use cannot silently expand an already saved use. Unsaved drafts never
replace the confirmed use shown in the report.
The stable production identity must bind input digest and exact member target IDs;
mock use-N labels and record offsets are local UI references, not new public IDs.

Preserve the4096-binding total across all confirmed uses; reject overflow atomically
without dropping members or splitting behind the user's back. Preserve256-system,
string/byte/freshness/provenance and ambiguity gates in future product work. A larger
intended scope needs explicit boundary review, not a covert batching workaround.

### G5. Questions follow the use and relevance

Purpose and usage are optional; an empty answer remains unentered, distinct from
explicit unknown. The form asks (only for known signature purpose with active or
planned use) the existing business-critical signature/authentication meaning.
Omitted, unknown, false and inactive/N/A remain distinct. Hidden irrelevant draft
values are dormant. No expiry, technical support, successful deployment, urgency or
migration completion is inferred. Candidate algorithms/parameters and compatibility
support are deferred to a later planning step, with the adopted configuration-basis
invalidation preserved when implemented.

The mock accepts unknown purpose; it does not force a guessed answer. It labels
all submitted use answers as examples and states that saved technical decisions
were not recalculated. A known example criticality answer is not applied as policy.

### G6. Response work items preserve material differences

For this source, assign every decision exactly once:

1. Existing urgent decision → inspect locations with known classical concerns480.
2. Other signature/key-certificate facet → clarify signature/certificate use3400.
3. Remaining records → identify algorithm/use4870.

All8750 original actions are investigate. The three items are presentation topics,
not three independently assessed migration projects. Each card retains counts,
location subsets, reason and original evidence.480 urgency is the existing saved
technical result, not480 vulnerable deployed systems. Unknown readiness/information
is never averaged into readiness or safety. Test-labelled urgent evidence remains.

For future datasets, work items must be keyed by confirmed system/use plus compatible
original action/reason/dependency sets. New reason classes cannot silently join the
classical-concern item or be discarded to keep a three-card design. Distinct urgent,
unknown, conflicting and blocked subsets must be exposed; no overall numeric score
or invented group verdict is authorized. Shared input needs the real evaluator
before any production decision changes.

## Size design: shared lossless data, bounded rendering

The current ~56.5MB product HTML repeats complete human card markup in two languages
per target. The prototype has one layout and shared explanation per work item.
A typed structural dictionary stores repeated strings/objects/arrays once and uses
integer references. All original assessment fields, links and numeric values are
retained (numbers are stored as JSON numeric strings, not JS floating-point values). Metadata rows reference this dictionary rather than duplicate IDs/paths.
The dictionary has68929 nodes; source recovery is checked against the complete
assessment. Objects preserve property order; null/boolean/string/numeric types stay
distinct, including unsafe JS integer lexemes. Lazy detail decoding avoids eager
materialization of the whole original object. No full card HTML per record, hidden
all-record templates, per-record language copies or base64-expanded payload.

The generated self-contained Japanese mock is about5.9MB including8750 complete
saved targets/decisions and UI code. Exact sizes/hashes are in evidence/BUILD.json.
This is not a like-for-like bilingual production benchmark: presentation is a
Japanese interaction mock and answers are examples. Complete source facts are
lossless; complete old per-target human layout is intentionally not duplicated.
Future bilingual completion and language semantics still need their own evidence.

The Save report button generates a read-only self-contained mock report with all
saved use examples and source data. No network/assets are needed after loading.
A prototype8MiB export budget rejects with explanation if exceeded; it never
truncates data.8MiB is a design target for this8750-record fixture, not a newly
approved product format limit. Retain the existing64MiB fail-before-replacement
contract pending a separately reviewed product size design. For growth beyond
budget, compare shared-code/data improvements and explicit companion/archive
options; no automatic loss or unapproved format switch. Do not embed raw report
JSON in a second copy or carry rendered drawer HTML into exports. Source JSON can
be saved through exact numeric-lexeme serialization, not lossy JS Number parsing.

## Implementation artifacts and acceptance

Owned mock sources: target/migration-workflow-mock-20261119/{model.py,build.py,
core.js,app.js,style.css,shell.html,test_model.py,test_core.cjs}. Generated site and
raw evidence stay under that directory; no new task, production entrypoint,
dependency or build rule is created. Browser server must serve only site/ on127.0.0.1.

Required checks: disjoint8750 coverage and exact counts; complete semantic archive
roundtrip; script-context escaping/type/large-number behavior; eligibility121/233,
purpose relevance, ownership isolation, confirmation and4096 rejection; three real
screens, input example and manual flow, scope exclusion, multiple-use editing,
unknown fields, drawer paging/search, keyboard/modal/narrow/theme behavior; actual
saved-report download and reopen on a dedicated loopback static server without the
product backend; verify self-contained assets, source restoration, content, budget
and state. Physical network-disconnect and file:// behavior are not covered. Record browser,
viewport, commands/exits, source and artifact hashes. Do not claim product migration
judgments or independent human workload/usefulness from this mock.

Status: design and three-screen interaction mock completed on2026-11-19 within the
scope above. Python4 and Node7 tests, JavaScript syntax checks, deterministic rebuild,
and diff whitespace check passed (all exit0); see evidence/CHECKS-FINAL.json.
Actual browser behavior and limitations are recorded in evidence/BROWSER.md;
DOWNLOAD.json binds the actual5880806-byte read-only browser export and complete
semantic restoration of the8750-record source. Three final screenshots and live
loopback views are available. Production implementation and human usefulness remain
unverified; historical v0.4 completion/evaluation/release decisions are unchanged.

Implementation review /root/v04_implementation_review accepted r3 with no unresolved
findings after the ownership-expansion fix. Test review /root/v04_test_review
accepted the mock with no unresolved findings after all required checks. Both
reviewed manifest SHA256
a1df255743b811037a5f326d2521ce49529c7d1ed25750c89b8b89ea87c902d6.
Later changes only clarify the actual saved-report reopening conditions and add
this completion/evidence record; executable sources and generated views remain r3.
The final artifact manifest separately binds these documentation/evidence additions.
No physical network-disconnect/file:// or production release coverage is claimed.
No Rust/Cargo/product execution source is changed; broad Rust reruns do not validate
this separate prototype. Node/Python checks and actual browser evidence are required.


## Maintainer UI feedback revision (2026-11-19, 16:10)

The maintainer requested clearer terminology, shorter headers, X/backdrop dialog
closing, and moving Apply into the scope summary without a separate confirmation
checkbox. Selection with eligible members enables Apply even if optional answers
are empty; no selection/zero eligible/aggregate4096 overflow remains disabled.
The click is the explicit submission of the displayed exact scope. Identity,
unknown-binding exclusion, no automatic overwrite and aggregate bounds remain.
A fresh form has no selection; the explicit sample-fill control selects source if
nothing is selected. Applying a draft does not rerun the product evaluator.

Display unknown algorithm as 未特定, distinct from an unidentifiable input target.
The input-status column distinguishes eligible signature input, unidentifiable
targets, and records outside this signature-use form. Do not label a reference-only
unknown-purpose record eligible for signature input merely because it lacks an
unknown_binding limitation. テスト用フォルダ内 is a path-based label, not verified
runtime/test usage. Omit redundant workflow explanations and input confirmation text.

The current prototype is pinned to Keycloak and three fixture presentation topics;
production must derive topics from each input's actions/reasons/dependencies and
confirmed systems/uses. Neither fixed three-item output nor general product
classification has been implemented or validated by this mock. The feedback revision
is complete within the interaction-mock scope. Earlier r3 acceptance remains historical;
the maintainer-authorized selection/Apply flow supersedes its checkbox requirement.

Python4 and Node10 tests, syntax checks and diff checks passed (exit0). The last two
report-copy changes required only rebuild, syntax and final rendered/export checks;
core/model/test results remain applicable. See evidence/ui-feedback-1610/CHECKS-r2.json,
BROWSER.md and DOWNLOAD.json. Final actual export5881066 bytes is read-only and restores
the complete8750-record source semantically. Product source preservation is checked
against the existing baseline in PRESERVATION.json. No Rust reruns were needed.

Implementation reviewer /root/v04_implementation_review accepted after the displayed
Japanese search-label fix. Test reviewer /root/v04_test_review accepted with no
unresolved findings. Both reviewed source manifest SHA256
5ab68c9e500bad92588dc9577c4c3ea5eb81e1ccc0f474f110518daa6c7c2b88
(evidence/ui-feedback-1610/MANIFEST-r3.json). Subsequent changes are this completion
record only; the final manifest binds the document and browser artifacts separately.
No production execution, general classification or human usefulness is established.


## Navigation and screen separation follow-up (2026-11-19)

The maintainer requested concise Next labels, moving the overview navigation out
of the header, and removing duplication between Overview and Response work items.
The mock now separates three responsibilities:

- Overview shows saved-record totals, identified/unidentified algorithm-name counts,
  algorithm distribution and location distribution. Known algorithm names group
  exact saved identifiers; these counts do not establish parameters, live use,
  safety or production deployability. The initial distribution shows five known
  names plus unknown; a disclosure retains every other name/count. Location drawers
  retain exact evidence. Next is in the footer after the overview content.
- Use input retains the existing selection/optional answers/exact displayed-scope
  submission gates, with the scope-summary button labelled Next.
- Response work items show the three pinned Keycloak investigation topics, visible
  next checks, evidence drawers and saved-use examples. They do not repeat the
  overview's algorithm/location distributions. Shared-source evidence remains fully
  retained and traceable in both live and read-only saved reports.

No production evaluator, storage, arbitrary-product taxonomy or binding limit is
changed. Empty-context access to the response report remains available directly.
This follow-up supersedes the previous Overview's repeated work cards, preserving
all prior evidence. This navigation/separation mock follow-up is complete.

Rebuild (python3 build.py), app syntax (node --check app.js) and git diff --check
passed with exit0; unchanged Python4/Node10 results are reused with source hashes
in evidence/navigation-separation/CHECKS.json. BROWSER.md records actual IAB1280x720
and Chrome390x844 flows, complete distribution counts, footer geometry, immediate
work checks and no-context access. DOWNLOAD.json binds the actual5881694-byte,
read-only121-member export, its complete8750-record semantic source restoration
and exact static-server reopening. Source preservation is in PRESERVATION.json.

Implementation reviewer /root/v04_implementation_review and test reviewer
/root/v04_test_review accepted with no unresolved findings. Both checked manifest
SHA256a17c1044806430d5115877a763a896bd519d7eb40fc2e119a0b54dae1bdfa67b
(evidence/navigation-separation/MANIFEST.json). This completion record is the only
subsequent source change. The final manifest binds documentation and screenshots.
Production implementation, evaluator effects, arbitrary-product grouping and human
usefulness remain outside the verified mock scope.


## Approved production implementation (2026-11-19)

The maintainer approved implementing the three-screen mock, adapting overview and
work items to each product's actual scan data, preventing report-size growth, and
showing generation progress. This authorizes the shared HTML representation and
presentation replacement demonstrated by the mock; machine assessment/context JSON
formats, evaluator decisions, cryptographic logic and existing bounds stay intact.
No release, numeric score, external contact or new task creation is authorized.

### Implementation plan and acceptance

1. Preserve the dirty baseline and prior mock evidence. Reuse the current typed
   SavedEvidence/context generator/evaluator and report writers; no second policy.
2. Project scan overview and proposed input scopes from observed purpose, role,
   evidence kind and path hints rather than product names. Display exact membership,
   excluded ambiguity and module filters; submit one common answer per chosen use.
   Explicit selection/Next is the scope submission, with original identity/digest,
   provenance, relevance and4096 bindings/256 systems enforced. Separate use groups
   must remain distinct; per-target overrides and saved canonical imports remain.
3. Group report decisions only when system, observed role/purpose, urgency,
   information, readiness, reasons, missing facts, action, blocking dependencies and
   candidate support/validation semantics agree. Preserve all exact target/decision/
   claim/candidate/source identities behind bounded evidence views. Count work items
   from data (including0); never force3, average verdicts, hide urgent/test concerns,
   infer deployment or silently propagate answers to unrelated groups.
4. Replace repeated per-target/per-language human HTML with one inert typed shared
   archive plus common bilingual rendering/copy. Preserve complete source assessment
   semantically and exact large numeric lexemes. Render bounded pages/details;
   retain source JSON/Markdown,64MiB fail-before-replacement and self-contained HTML.
5. Keep old synchronous local API compatible; add a session/job/token-bound background
   generation path and poll actual evaluation/serialization/writing stages. A spinner
   and stage text precede requests; show counts only when actually measured, never
   invented overall percentages or success before all artifacts/context are saved.
   Concurrent runs, stale tokens, errors and retry must not expose stale success.
6. Verify report grouping/coverage on several materially different fixtures (signature,
   key establishment, symmetric/hash, incomplete/ambiguous, contradictory/blocked,
   empty and distinct systems), real Keycloak8750 and a non-Keycloak scan. Check exact
   restoration, byte size, bounded DOM, scope common-input and exceptions, empty-context
   path, theme/language/navigation, real background progress and failure/retry.
7. Run meaningful Rust/JS tests, workspace fmt/test/build/clippy and applicable optional
   configurations; inspect combined changes. Reuse /root/v04_implementation_review then
   /root/v04_test_review for required stages. Update paired user guides and architecture.

Alternative: retain per-target cards with paging only, or compress the old duplicated
markup. These leave the entry workflow and repeated human output intact. The approved
shared-data/presentation approach addresses both; no new compression dependency or
browser decompression requirement is needed. Rust owns presentation grouping, not
policy changes. Durable evidence is under target/grouped-migration-implementation-20261119/.
Status: completed against the approved production implementation scope.


### Production results and measured limits

The production UI now separates overview distributions, shared-use context and
response work items. Scope proposals are generated from observed purpose/role/path
hints for any product. Exact binding excludes ambiguous identities, tracks other
owners, supports module narrowing and keeps individual full-snapshot exceptions.
The old detailed selector remains optional rather than the initial8750-row hurdle.
Input and response views share a session; Next is in the scope summary, with no
extra confirmation checkbox. Existing context imports and exact typed validation
remain available. Successful context artifact delivery is explicitly an attachment.

Final Keycloak same-input/time comparison:8750 target decisions become16 response
work items without context, not a fixed three. Two common-use declarations produce20
items. Actual Python scan:40 records,13 with algorithm names and27 unidentified,
producing3 items without context. Empty input produces0 in Rust and shipped-renderer
tests. This is deterministic presentation grouping, not a new quality score or
claim that every product yields a small fixed number of groups.

For Keycloak the bilingual HTML is5739622 bytes versus56478304 previously, about90%
smaller. JSON27889807 bytes and Markdown21021137 bytes remain complete. The entire
new assessment equals the previous same-input/context-absence/as_of assessment,
and every8750 decision is preserved exactly once. The typed shared archive restores
the complete assessment losslessly; it does not replace original facts with averages.
Human rendering bounds work/evidence/claim/source pages to20 and selected details to1.
The64MiB output guard remains enforced, including rejection before existing outputs
are replaced; growth with genuinely distinct facts remains finite but not eliminated.

The actual saved5776894-byte context-bearing HTML reopened with EN/dark on a dedicated
loopback server and equals actual CLI evaluation at the same input/context/time.
Current-language/theme export strips session-only navigation and external session
scripts. Browser evidence covers exact classical concerns, hybrid component and
combiner nonverification details, dialog X/Escape/backdrop,390px layout, navigation,
common-edit exception preservation and explicit96947-byte canonical-context download.
The141-member context has2 systems:134 planned-use bindings,1 test-only exception and
6 bindings with omitted use state. These are test declarations, not observed deployment.

Actual stage polling reports loading, evaluation, serialization and writing; file
counts come from real write callbacks. Failure/retry, stale tokens, exact job binding,
Host/Origin/session guards, ambiguous scopes and output aliases are tested. Ready
requires every report and supplied context to be saved. Generation is non-cancellable
once started; no estimated global percentage or premature success is displayed.

Evidence: target/grouped-migration-implementation-20261119/evidence/. FINAL-CHECKS.json
binds source, binary and commands; KEYCLOAK-FINAL.json binds the like-for-like size and
semantic measurement; BROWSER.md distinguishes final checks from provisional runs.
The accepted implementation revision is IMPLEMENTATION-r7.json, SHA256
32a27d79d225230c974cf90a2d1c542f55bbfea2602103203ebf80332562fe68. All31 file hashes matched.
Full workspace tests738, Node tests21, workspace Clippy/build/fmt and all-feature CLI
tests passed with exit0; optional builds used a separate target directory. Both user
languages and architecture are synchronized. Existing41 dirty files were preserved;
no commit, push, release or task-number creation was requested for this follow-up.

Independent implementation review reused /root/v04_implementation_review and accepted
r7 after correcting standalone export contamination, missing exact technical facts,
representative-only no-JS group facts, saved-job folder restoration and explicit
context download delivery. Test review reused /root/v04_test_review and accepted the final executable/test
source with no actionable findings or required coverage gaps. The reviewer independently
confirmed full archive restoration, complete decision membership, old/new semantic
identity and constrained context/exception behavior. All-feature CLI tests108 passed.
These technical checks do not measure practitioner usefulness.


### Production completion record

The main agent completed the approved implementation, verification and both independent
review stages. Implementation and test review accepted the same r7 executable/test
source, manifest32a27d79d225230c974cf90a2d1c542f55bbfea2602103203ebf80332562fe68,
using /root/v04_implementation_review and /root/v04_test_review respectively. No other
agent implemented or evaluated this request. All reported findings are resolved.
Subsequent changes only correct paired guide button names and append this English
results/completion record; FINAL-MANIFEST.json separately binds that final prose.
No executable/test/input changes followed the successful r7 checks. FINAL-CHECKS.json
records the evidence reuse and final-source correspondence.

The same worktree/branch and pre-existing changes remain available for maintainer
review. The real Japanese/dark Keycloak session and saved report tab are kept open;
loopback URLs and owned server PIDs are in SESSION-r7.json and evidence/*.pid.
No required implementation work remains for this request. Practitioner usefulness,
numeric-score adoption and public release remain outside this technical UI scope.


## Report clarity follow-up (2026-11-20)

User approved fixing persona report UX findings and questioned repetitive unknowns,
search, single-category urgency filters and why the six dependency references were
undetermined. Clarify HTML presentation only: distinguish observed evidence from
known active applied declarations (including proposed configuration, never proof
of deployment), show concrete missing facts and next checks before full reasoning,
and keep every canonical reason/claim/decision available in evidence/JSON. Do not
modify urgency policy, detector output, grouping membership, archive, schema or
input format. Raw group names are not in saved assessment; do not invent names or
expand storage to reconstruct them. Use meaningful declared purpose/location in
the visible card instead of leading with a generated system ID; keep IDs in evidence.

Search matches observed algorithms, locations, action/reasons and declared purpose,
usage/configuration. Show search only for multiple work items and explain its
fields/example; show urgency dropdown only for multiple distinct urgency states.
Never manufacture categories absent from the assessment. Label evaluation time
and render Japan time consistently with input/history, retaining exact UTC in the
time element/JSON. Surface undetermined record counts beside urgent counts.

Mixed declarations in one existing work group must be summarized from all members,
not inferred from its first decision. Claims are shown as current declarations
only when known/active; stale/missing/conflicting values must not gain that status.
Action presentation may name concrete saved missing facts; detailed original
validation steps/reasons remain traceable, and no claim can override unknown
observed implementation or imply PQC readiness. Empty, single, mixed, escaped and
large reports and lossless export need verification. Main owns implementation,
tests/browser; reuse the two existing independent reviewers. Preserve the original
persona saved report byte-for-byte; show the revised renderer on an unchanged
assessment as a new artifact. Evidence: target/report-clarity-20261120/evidence.


Report-clarity completion: `/root/uswds_implementation_review` and
`/root/uswds_test_review` accepted SOURCE-r5.json SHA256
`cab74572a50569f7fca2dea0165e1f9b735ab33c18a477f584688ae06b892b46`,
on HEAD3a812a4361cc7400f3811a1b09d7d82779a9012b plus recorded dirty source.
No required findings remain. One review finding was resolved by distinguishing
technical evidence from operator declarations when advising how to fill missing
facts; no saved technical fact can be upgraded by declarations alone.

Node62, report Rust80 (one intentional worker entrypoint ignored), HTTP15 and
workspace build passed, exit0. CHECKS.json/BROWSER.md record source/configuration,
actual1280/390px JA/EN dark/light observations, filter/empty-state/evidence/modal
operations and applicability of unchanged Rust/core/optional checks. The same
persona assessment was regenerated with identical input/context/as-of; the entire
assessment equals both the original saved report's archive and the new JSON.
Original persona HTML remains unchanged. Keycloak8750 decisions/252claims/20groups
retain every decision exactly once; the filtered browser JSON download retains
all8750 decisions. EQUALITY-final.json binds source/binary/inputs and artifacts.

The user's Django example consists of six development-declared dependency
references with unidentified observed primitive/parameters. It supports locating
further investigation, not determining actual migration priority or completion.
Known active encryption/usage/proposed AES256 declarations remain visible and are
not recast as observations. Undetermined urgency is not low risk; display now
makes this distinction and the concrete evidence gaps explicit. No missing facts
were invented to make a report look complete. Front summaries are bounded; all
canonical reasons, steps and declarations remain available in details/evidence.

Only this completion prose changes after reviews; SOURCE-final.json and
FINAL-CHECKS.json bind final source. Static report server62629/PID75652 and current
full UI server63647/PID77322 remain available. Revised persona report tab31 is the
user deliverable; original user report60722 remains unchanged/open. Temporary test
tabs closed and viewport reset. No commit/push/publication, policy/schema/group
membership change or new task. All approved report fixes and explanations are
verified; no required work remains for this follow-up.

## Authorized OSS-persona follow-up (2026-11-20)

The maintainer requested fixes for all five findings in `target/oss-persona-20261120/evidence/persona/REVIEW.md` and further real-OSS language comparisons. The follow-up preserves decision policy, grouping membership and canonical JSON schemas. Known unbindable target IDs and an optional bounded scope label are presentation metadata outside the lossless assessment archive. Existing report writer entrypoints delegate to the optional-label writer. Deadline authoring supplies explicit issuer/document/edition/clause references through the existing schema and validator. Applied active deadline/lead-time pairs are aggregated per member with counts and a bounded variant display; mixed groups never inherit the first member's dates. Distinct recorded locations are not distinct runtime uses. New detector gaps measured by the language review do not authorize rule changes.

## Current decision presentation (2026-11-23)

This section describes the implementation at `87e27cd`; earlier dated mock,
filter, ledger and review records above remain historical. No evaluator, schema
or supported distribution scope is changed by this documentation update.

The HTML derives use-specific scopes from active applied claims. A system,
non-conflicting information, supported use and active/planned usage are required
for plan/unblock/migrate/monitor scopes. Explicit applicable exclusions have their
own scope. Only plan/unblock/migrate render three phases: current-use discovery,
compatibility testing and staged rollout. Monitor/exclusion render one check;
other decisions render investigation actions and reassessment conditions.

Cards with scopes render first by urgency. Urgent unscoped investigations also
remain visible. When any scope exists, remaining unscoped nonurgent cards start
inside Other checks; without scopes, every card renders by urgency. There is no
work-item search or urgency filter. Location-dialog search remains.

The top schedule uses up to three active per-record deadline/lead-time pairs,
with separate start-by/deadline status. Goal month remains a declaration, never a
policy deadline. Card details contain the applied assumptions and blockers.
The location dialog groups a known path/line once; semantically equivalent
records share details while their rule/method sets remain visible. Unknown
locations and differing facts/decisions stay distinguishable. Long paths wrap.

The bottom exposes Save assessment data. Separate checklist, owner list,
request-copy and trailing claim/source ledgers are removed from the human page.
Canonical JSON, CLI Markdown and lossless embedded assessment remain available.
Original-scan navigation requires the matching completed local job; standalone
HTML does not invent that dependency. See the paired [user guide](../user/REPORTS.md#saved-assessment).
