# Local scan workflow

## Approved scope and acceptance

The maintainer selected the A mock and its successive revisions on2026-11-19, then
requested product implementation. Use the existing worktree/branch. The main agent
owns implementation/testing; reuse /root/v04_implementation_review and
/root/v04_test_review for the two stages. No extra agents, release or publication.
Historical065–075 evidence remains unchanged and does not cover this follow-up.

Deliver staged input/results/context screens, the approved palette/theme, direct
folder names and minimal paired JA/EN copy. Keep one folder per scan and existing
loopback/session/Host/origin/allowlist/bounds. Ordinary serve keeps automatic report
navigation; opt-in migration serve first shows results and explicit next actions.
Use actual inventory targets with server search, role filters and20-row pages.
Selection is explicit and at most4096 targets. Shared authoring groups have fixed
members; per-target snapshots survive common edits and can be restored explicitly.
Drafts stay in page memory until a canonical download; switching saved inputs resets
bindings. Full imported context stays read-only and lossless. Preserve no-context
assessment and exact-input CLI/web agreement.

Conditional controls and active payload must agree. Not-applicable retains raw draft
values but applies only applicability/provenance. Irrelevant purpose/support values
are dormant. Omission and explicit unknown remain distinct and readable. Offer
named parameter sets and custom input, never an automatically recommended selection.
Known support must be reconfirmed after its purpose/algorithm/configuration changes.
Dates are explicitly Japan time; serialize offset-bearing seconds. Duration is entered
in days and converted exactly to the existing seconds representation.

## Approved unpublished-format amendment

The maintainer approved extending the existing unpublished format rather than adding
v2, after explicitly clarifying that v0.4 is unreleased. Retain old context meaning.
Add optional `migration_target` as a normal provenanced known/unknown claim. A known
value contains `use_case`, `algorithm` and nonempty `parameters`. Families must match
purpose (ML-DSA signature/JWT/PKI, ML-KEM key establishment, AES encryption, SHA-256
hashing). Presets name FIPS20444/65/87, FIPS203512/768/1024, FIPS197 AES128/192/256;
custom strings are declarations whose cryptographic validity is not verified.

The migration-target claim binds support answers supplied in the same scoped claim
set. A known support inherited from another scope cannot silently acquire this basis;
reject that mixed binding. The effective declared use must match the configuration's
use_case. A stale/unknown configuration cannot establish configuration-specific known
support. Contexts without this optional claim retain the existing generic support
semantics. UI edits retain the old basis in memory until explicit reset. A raw file is
still an operator declaration, not evidence that a platform implements that setting.

Include the configuration claim in the human-readable review and assessment evidence.
Keep the existing generic candidate and its explicit-parameter-validation requirement;
recording a requested configuration does not certify it. Extend the closed context and
assessment claim schemas accordingly; do not weaken input/freshness/binding checks.
No new policy branch, numeric score, scanner detection or cryptographic check.

## Implementation and verification plan

1. Reuse approved mock structure and styles in bundled local assets; actual session
   APIs replace synthetic actions/data. Add typed grouped generation transport using
   the existing ContextClaims/generator/binder, not a second policy implementation.
2. Extend the optional context claim, bounds/schema/typed display and binding guard.
   Preserve all old contexts and CLI routes; reject mismatched/ambiguous scopes.
3. Test pure authoring transitions, actual grouped HTTP canonical output and CLI
   agreement, negative input/basis/bounds and existing security/serve contracts.
4. Run Node syntax/state tests, fmt, workspace tests/clippy/build and affected feature
   configurations. Use a dedicated loopback session for actual JA/EN browser checks,
   keyboard/focus/errors, imports/downloads, narrow layout, themes and offline assets.
5. Obtain both reused reviews, resolve material findings and bind final-source
   evidence. Keep actual measurements separate from usability/release claims.

Raw follow-up evidence is under target/ui-implementation-20261119/evidence/; ignored
mock artifacts remain under target/ui-a-mock-20261119/. Final review/source/evidence
records are below. No commit/push is included without a later request.


## Completion record (2026-11-19)

The approved implementation scope is complete. Main implemented the staged local UI,
fixed-membership group editing with independent snapshots, exact conditional payloads,
configuration presets/custom input, and the approved unpublished-format amendment.
New generation feeds the existing typed generator/binder; evaluation still uses the
same Rust policy. The ordinary automatic-report workflow remains available.

Reviewed executable/test/schema/user-document source and binary are bound by
`target/ui-implementation-20261119/evidence/BROWSER-SOURCE-r4.json`, SHA256
`ea335a5f7d302acec0150d22b6b8b61ab1e08b675b16710ea1be400ed565d206`.
The only change after that freeze is this completion metadata. FINAL-SOURCE.json
records the full final worktree including this metadata; runtime/test/schema files
remain identical to the accepted snapshot. Baseline HEAD is3a812a4361cc7400f3811a1b09d7d82779a9012b.
No commit or push was requested for this implementation follow-up.

Passing checks, all exit0:

- `cargo test --workspace`:725 passed;3 existing intentionally ignored worker entrypoints.
- `cargo clippy --workspace --all-targets -- -D warnings` and `cargo build --workspace`.
- `cargo test -p pqc_scan_cli --all-features --test serve --target-dir target/ui-implementation-all-features`:11 passed.
- `cargo clippy -p pqc_scan_cli --all-targets --all-features --target-dir target/ui-implementation-all-features -- -D warnings`.
- Node syntax checks for all3 scripts; `node --test crates/pqc_scan_cli/tests/serve-context.test.cjs`:5 passed.
- `cargo fmt --all -- --check`, `git diff --check`, local document/reference checks.

The5 configuration tests and11 HTTP tests cover old generic contexts, configuration
binding/freshness/unknown inheritance, groups/individual scope, duplicate/invalid input,
script escaping, query bounds, security and exact Web/CLI output agreement. Final
broad logs use `*-final.log`; presentation-only review fixes have refreshed build and
static evidence and explicitly bounded reuse in REVIEW-FIX-CHECKS.json.

Actual IAB/Chrome observations include Java scan39 targets;24 selected across pages;
23 common configuration declarations plus1 individually excluded target; normal
Python auto-report navigation; canonical context download/reimport; exact Web/CLI
JSON agreement for the actual Java input; BOM rejection; option/basis/draft retention
through language changes; N/A and no-context flows; keyboard/focus/cancellation;
390px layout and themes; and loaded-page behavior after its dedicated server stops.
Actual downloaded canonical files confirm0/30/3652 days become0/2592000/315532800
seconds on each of14 targets. Over-limit, fractional and negative days are rejected
without overwriting saved values. Imported315532801 seconds remain exact and are
shown as seconds, not rounded days. These are controlled functional observations,
not a human usefulness or workload study.

Baseline browser evidence is BROWSER-CHECKS.json, SHA256
0c5d1e7ae1683acfd78884a4da181ef849b5537e1f6f9740b3f98e14d2310e18.
Review-fix evidence is REVIEW-FIX-CHECKS.json, SHA256
2039576a8624e1844fbb0c73bf503f0a47b6aee924f33d8adb61e84150767595.
All artifacts are under the evidence directory above. Downloads were verified from
actual task-named Chrome files because the automation download event did not arrive;
timeouts were not counted as success. An initial final-home capture preceded paint;
it is retained separately, and11-final-home.jpg was recaptured and visually verified.

Both reused independent stages accepted finalr4 with no unresolved finding:

- /root/v04_implementation_review: implementation accepted after strict BOM handling,
  state-preserving dynamic localization and the final status/duration presentation fix.
- /root/v04_test_review: test scope accepted after TUI-01 status localization and
  TUI-02 actual day/second canonical evidence were resolved. Existing broad Rust and
  feature evidence remains applicable to the final presentation-only delta.

Developer and paired user guides reflect the actual behavior. Old task/review/study
history is preserved. Remaining limits are custom configuration validity, independent
human usability/full accessibility/real-device measurements, and the separately
controlled release/publication decision; none is claimed by this implementation.


## Scan feedback and report navigation follow-up (2026-11-19)

Maintainer acceptance: show ongoing work with a spinner and truthful progress,
inherit the shell theme in reports, provide a route back to the same result, and
replace terminal-oriented footer copy with a concise local-processing statement.
No rule, decision policy or persistent JSON schema changes. Existing CLI arguments,
machine-readable contracts and automatic ordinary-serve navigation remain unchanged.

Implementation uses an additive optional pipeline observer, the existing Rayon scan
and existing CLI writers. Enumeration and aggregation/writing are indeterminate;
file inspection reports actual processed/selected counters, including failed reads.
Callbacks may arrive out of order and the session clamps processed counts. A count
is work attempted, never inspection success. Terminal ready/failed states replace
progress. The UI polls once per second and may skip short phases.

Report links carry bounded theme/language preferences. A shared report head script
applies theme before styles/content. The shell uses the same optional preference
key and honors explicit return preferences even without localStorage. The server
adds a fixed same-session result link only to allowlisted HTML responses; report
files on disk remain standalone. Result IDs restore ready session jobs without
rescanning, including ordinary serve. Migration assessment returns to results; input
drafts retain the documented page-memory lifetime. Existing Host/origin/path guards
and explicit optional migration behavior remain in force.

Plan/checks: meaningful observer count/failure/empty tests; HTTP allowlist/standalone
export checks; preference/navigation JS tests; full fmt/workspace tests/clippy/build;
affected optional configurations; actual loopback browser progress, themes, return
navigation, languages, keyboard, narrow/reduced-motion and offline observations.
Reuse the same two reviewer IDs for the combined follow-up. Evidence is under
`target/ui-feedback-20261119/evidence/`; prior completion records remain historical.
Status: completed against this follow-up scope. Both reused review stages accepted
final source; no unresolved findings. The completion evidence below supersedes the
in-progress status, not prior scope or historical measurement limits.

### Follow-up completion evidence

Executable/test/paired-guide source: `SOURCE-r3.json` in the evidence directory,
SHA256 `0c8de0baf6b9e18e8910a9bd57dec6b9517d7a8dd96c1ed21f233b07bc7fd085`.
Baseline HEAD `3a812a4361cc7400f3811a1b09d7d82779a9012b` plus the36 manifest files;
BASELINE.json/baseline/ preserve earlier uncommitted work. Only this explanatory
completion record changes after the reviewed snapshot. FINAL-SOURCE.json binds
that final documentation state and the same final executable.

Checks: workspace tests727 passed/3 ignored; all-feature tests786 passed/5 ignored.
Default ignored tests are worker entrypoints; featured ignored tests include4
worker entrypoints and the existing installed-OpenSSH interoperability prerequisite.
No claim of running that unavailable external prerequisite. Node controller/context/
preference tests10 passed. Workspace build, fmt, default/all-feature clippy and
refreshed default/all-feature HTTP serve11 tests passed with exit0; commands and
results are in CHECKS.json, FEATURE-CHECKS.json, REFRESH-CHECKS.json, CHECKS-r3.json.
Full Rust results precede JS/CSS-only refinements; unchanged Rust semantics/inputs
and refreshed affected asset checks justify reuse as recorded in BROWSER.md and
TUF-01-r3.md. No detection, storage or policy behavior was changed by this follow-up.

Actual browser evidence: real Keycloak10958 selected files, monotonic counters,
sticky spinner and terminal cleanup; JA/EN and390px layout; dark/light inheritance,
explicit same-job return without rescanning, ordinary serve failure/retry, rule
explanation and small migration report navigation, standalone saved HTML. Final r3
product smoke additionally passed actual progress→ready→dark Japanese scan report→
explicit return to result1. Source/binary binding and unchanged job count are in
ACCEPTANCE-r3.json and server-r3.json. The retained tab25/server is the final r3
Keycloak results screen; previous user viewing sessions remain intact.

Reviews: /root/v04_implementation_review accepted r1, sticky/recovery r2 and TUF-01
r3, with no remaining findings. /root/v04_test_review initially found Low TUF-01:
language switching during an outage revived stale progress and cleared its error.
Main added a failing regression, gated progress while disconnected, preserved and
translated the error, reran affected checks and verified503→EN→200 in a controlled
browser HTTP fixture using exact bundled assets. Both reviewers accepted r3 hashes;
test review confirmed reuse of the earlier broad and browser evidence. No reviewers
were replaced and no work beyond these two review identities was delegated.

Limits: reduced-motion CSS was browser-parsed but OS activation was not measured;
blocked preference storage has unit rather than browser-emulated evidence. The
54MB/8750-decision Keycloak migration report rendered dark, but the browser helper
once timed out on its large DOM. Its explicit return click was measured on a small
empty report; large-report interaction performance remains unmeasured. This is not
a human usability/full accessibility/performance or release assessment. No new task,
release/publication, score adoption or commit/push is included in this follow-up.


## Large migration report follow-up (2026-11-19)

The maintainer reported near-freezing when opening the Keycloak migration report.
The prior54MB report materialized8750 full decision cards plus an equally large
alternate-language template. This request authorizes fixing rendering, not changing
policy, evidence, decisions or machine-readable contracts. Main owns implementation
and validation; reuse the same two review identities, no new task or extra agents.

Acceptance: bounded initial/interactive target list(20) and selected detail(1),
search/urgency filter across every target, reachable final pages and evidence links,
shared theme/language/session return behavior, language changes preserving state,
bounded declaration tables(20), complete local data and unchanged JSON/Markdown.
Exercise real8750-target output in the browser, and compare generated canonical
outputs byte-for-byte at the same input/as_of. Retain the64MiB fail-before-write
limit and validate script-context escaping, empty/no-match and invalid-fragment paths.

Approach: reuse the existing Rust card/claim renderers and presentation controller.
Each language shell initially contains one card and20 list/claim rows. HTML strings
for subsequent cards/claims are inert JSON raw-text blocks of20 records with both
languages; only the requested block is decoded, with one cached block per kind.
A small metadata index supports global search/filtering without parsing card HTML.
Delimiter escaping prevents script closing/comment/double-escape contexts while
retaining original values and the existing output-size budget. No hidden-template
fallback that still creates all card DOM nodes. No server-only pagination, so saved
HTML remains standalone. No-JavaScript and printing show the initial/selected slice;
complete Markdown/JSON remain the full-report routes and paired user guides say so.

Checks: renderer cardinality/all-record parity and raw-text security tests; actual
controller tests for lazy page access, last-page search, filters, empty/missing IDs,
claims cross-page fragments and language continuity; fmt/workspace/all-feature tests,
clippy/build and real loopback browser/narrow/keyboard/saved-output observations.
Evidence: target/migration-performance-20261119/evidence. Status: completed against
this maintainer-requested rendering scope.

### Large-report completion evidence

Reviewed executable/test/user-guide snapshot: SOURCE-r2.json SHA256
`ed10a3095879498fdbc4dc66b528a331b86446bd2fcdfdb75629091fd0b33650`, HEAD
`3a812a4361cc7400f3811a1b09d7d82779a9012b` plus40 manifest files. Only this
explanatory completion record changes after review; FINAL-SOURCE.json binds it.
The product viewing binary is bound in PRODUCT-SERVER.json and ACCEPTANCE.json.

Default workspace729 passed/3 pre-existing worker entrypoints ignored; all-feature
788 passed/5 pre-existing ignored (worker entrypoints plus installed-OpenSSH
prerequisite). Final combined Node15 and renderer7 tests passed. Script-context
escaping round-trip passed;64MiB rejection still preserves existing outputs.
Default/all-feature Clippy, workspace build, fmt and diff checks passed. Initial
Clippy failed only for an unnecessary lifetime on the new test helper; this was
fixed and affected checks rerun. Subsequent JS-only same-fragment handling, its
Node regression and that lifetime elision do not invalidate full Rust-suite
results; BROWSER.md and REFRESH-CHECKS.json record the reuse rationale/commands.

Keycloak8750-target assessment was regenerated from the exact saved inventory at
`2026-11-19T04:20:18Z`. Both CLI and actual product-session generation preserve the
prior JSON/Markdown bytes exactly. Initial standalone DOM:1 active decision,20 list
items,702 elements; served final report705 elements including session navigation.
Final-page8750, full-report search(28 matching filename targets), urgent filter480,
no-match0, language state, synthetic45-target/180-claim paging, cross-page references,
keyboard target selection and theme/language-preserving return were observed.
Chrome390x844 showed no horizontal overflow; ineffective IAB viewport override was
recorded as such, not counted as narrow evidence. See BROWSER.md and captures.

Implementation reviewer /root/v04_implementation_review accepted r1 and r2 with no
remaining findings. Main's browser acceptance found repeated same-fragment claim
links could fail after manual paging; a failing Node regression, explicit repeated
fragment jump and final-browser claim47 reproduction resolved it. Test reviewer
/root/v04_test_review accepted r2 and final evidence applicability with no actionable
finding or material gap. Neither reviewer was replaced or delegated further.

The retained user tab30/product loopback server displays the final dark/JA Keycloak
migration report, regenerated from imported saved results; it is not a new source
scan. The prior user scan/report sessions and original artifacts remain intact.
No commit/push, publication/release or new task was included.

Remaining limits: the standalone file still contains all data (~56.5MB) and no
machine-independent latency/memory guarantee is inferred. No-JavaScript and print
show the initial/selected slice, with complete Markdown/JSON documented separately.
No full accessibility, real-device or human usefulness evaluation was performed.


## Proposed product correction: system and use-case decisions (2026-11-19)

Status: proposal requested by maintainer feedback, not an adopted amendment,
implementation, new task or change to historical acceptance. The maintainer found
that selecting thousands of targets makes the workflow unattractive regardless of
paging/render speed. Preserve completed task/review/evaluation records and existing
uncommitted code. No executable behavior or acceptance criterion changes here.

### Purpose and evidence

The roadmap's product outcome is to apply scoped organizational/system context to
saved evidence and identify which investigation/action needs attention, why, and
what is unknown (ROADMAP.md Product outcome; R3-01..05, R4-01..05). An observation
is an evidence unit, not necessarily an operator question or independent migration
work item. The current initial target selector exposes that internal evidence unit
as the user's starting decision unit. Existing shared input helps after selection;
it does not remove the burden of finding and selecting the correct occurrences.

At the previously evaluated Keycloak input/as_of,8750 targets include4412
reference-only records; all8750 actions are investigate and all readiness values
unknown.480 decisions retain urgent investigation and8270 have undetermined urgency.
These are the existing context-free policy results, not evidence that8750 live
business uses exist or that480 deployed systems are insecure. Source-bound counts
are in target/product-direction-20261119/OBSERVATIONS.json. Task071's approved
synthetic study explicitly leaves human usefulness and workload unmeasured;
conformance/reproducibility success does not establish workflow utility.

### Alternatives and recommendation

| Approach | Benefit | Material limitation |
|---|---|---|
| Keep per-occurrence selection with more paging/bulk controls | Small interface change | Still asks users to understand thousands of technical records before seeing value |
| Group every matching algorithm and apply one answer | Fewer interactions | Same RSA label can span production/test, signature/encryption and different systems; no valid basis for shared business facts |
| System/use-case guided review with evidence-group proposals | Aligns questions and output with migration decisions; reuses existing evaluator | Needs explicit grouping/binding/aggregation semantics and large-input acceptance before implementation |

Recommend the third approach. Preserve individual evidence/decision identities
internally, but make systems, uses and response work items the primary user units.
Technical grouping is a proposal, never proof of shared real-world deployment/use.

### Proposed workflow

1. After scanning, show a useful technical overview before context entry: observed
   algorithm/role families, known concerns, captured assurance and missing scope.
   A user can inspect this without selecting every record or inventing context.
2. Confirm the assessment scope and intent. An upstream Keycloak source review
   primarily supplies evidence about product code/dependencies; an assessment of a
   deployed service additionally needs its actual configuration/use. Both can use
   the same technical/policy engine, with unknown business facts retained.
3. Propose groups using captured roles, evidence kinds, modules/dependencies and
   paths. Show group counts, why records are included, representative locations and
   expandable exact membership. Test-directory paths are observable hints, not
   automatic proof of nonproduction use. Do not silently drop their concerns.
4. Let the operator confirm a system/use scope and answer relevant known questions
   once for that scope: purpose, active/planned use and the adopted business-critical
   claim, plus confidentiality/lifetime/deadline only where the branch needs them.
   Shared answers and explicit exceptions reuse the current context inheritance,
   binding and generator. Do not default-bind all8750 or bypass4096 selections,
   ambiguous identity,256-system/size bounds, provenance or content/time binding.
   A scope exceeding current limits needs a concrete reviewed decision; batching
   must not covertly weaken the bound or omit intended members.
5. Defer candidate algorithms/parameters and platform/peer support to the relevant
   later migration planning step, rather than making them the first intake hurdle.
   Changing the target configuration still invalidates configuration-specific
   support as adopted. Unknown/omitted/false/N/A remain distinct.
6. Present response work items grouped by confirmed system/use and compatible
   action/reasons/dependencies. Examples, not current deployment findings: confirm
   active token-signing algorithms and their verifiers; inspect certificate/auth
   dependencies; identify actual uses of unresolved source references. Each item
   shows reason, affected scope, missing facts, blockers and next check, with the
   underlying observations/decisions accessible on demand. Known urgent facts and
   unknown/conflicting/blocked subsets cannot disappear into an average or a
   single claim of project readiness. Full per-occurrence browsing becomes detail.

No automatic business-use inference from algorithm/path, no blanket claim that all
records in a repository share one context, no numerical project score, and no new
CMDB/SaaS/second policy engine. A grouped presentation cannot change a decision's
meaning or make an unbindable record eligible for declaration. Grouped report
wording/roll-up rules and scope review need specification before executable work.
Shared descriptions should be referenced rather than duplicated per record/language;
file-size improvements need separate measurement, not a promised reduction.

### Acceptance proposal and next design work

- Adding more equivalent observations to one confirmed scope does not require more
  answers. Selecting thousands of individual rows is not a prerequisite to a useful
  first overview or report.
- Two systems or purposes using the same algorithm keep distinct context and
  results. Ambiguous records remain reviewable evidence without forced answers.
- Group membership/reasons and uncertain/test-path classification are visible and
  confirmable in bulk, with exact references retained behind the group.
- Group output preserves every material urgent condition, uncertainty, conflict and
  blocker; each displayed action is traceable to evidence and adopted decisions.
- Exercise the real Keycloak-scale input through overview, scope/group confirmation,
  one shared answer set, exception, reassessment and response overview. Record
  actual questions/clicks/time, not only test pass counts or tiny synthetic demos.
  Controlled operator observations remain distinct from independent human utility.
- Preserve compatible saved-input/context/evaluation paths, byte/reference safety
  and no-network reassessment. Any changed storage/policy/bounds require the explicit
  maintainer review specified by AGENTS.md; this proposal grants no such approval.

Next design deliverable should include the scope/grouping/roll-up rules and mocks
for initial technical overview, scope-level questions and response overview, tested
with8750 records and useful without per-row entry. Reconcile affected v0.4 acceptance
only after explicit agreement; do not rewrite historical completion or create tasks
merely to label this proposal complete.

### Direction approval and requested mock (2026-11-19)

The maintainer approved proceeding with the system/use-case direction and requested
concrete grouping rules plus Overview/Confirm use/Response work-item mocks against
large Keycloak data, including prevention of report-size growth. The implementation
scope at this step is design and interaction mocks, not a production storage/policy
amendment. See [grouping rules and mock contract](MIGRATION-REVIEW-GROUPING.md).
Historical completion/evaluation records remain unchanged; new mock results cannot
establish human usefulness or replace future production acceptance.


## Approved grouped workflow implementation (2026-11-19)

The maintainer subsequently approved implementing the three-screen grouping mock,
data-driven arbitrary-product scopes/work items, bounded shared HTML and visible
report-generation progress. Earlier mock-only and per-target layout records above
remain historical. The production plan, acceptance and current evidence are in
[MIGRATION-REVIEW-GROUPING.md](MIGRATION-REVIEW-GROUPING.md#approved-production-implementation-2026-11-19).
It retains the current typed context/evaluator, exact membership, individual
exceptions, canonical imports, safety limits and existing synchronous API. This is
not authorization to release, adopt numerical scoring or claim human usefulness.


## USWDS correction follow-up (2026-11-20, complete)

The maintainer authorized all thirteen corrections in the handoff, without a
commit, push or release. Baseline HEAD remains 3a812a4361cc7400f3811a1b09d7d82779a9012b;
45 pre-existing dirty files are preserved in
`target/uswds-ui-20261120/evidence/BASELINE.json` and its adjacent snapshot.
Previous completion and review records above remain historical, not evidence for
this follow-up. The original reviewer paths cannot be resumed in this session.
The maintainer explicitly authorized two replacement identities, one read-only
implementation reviewer and one read-only test reviewer; main owns all other work.

Acceptance and implementation sequence:

1. Explain inventory JSON (32 MiB) and context JSON (1 MiB) at their inputs.
2. Separate the startup access boundary from an explicit folder choice; preserve
   confinement and symlink rules. No native filesystem picker is claimed.
3. Open scan reports in a new tab, carrying language/theme; preserve authoring state.
4. Name session jobs, imported filenames, actual request times/status and newest order.
5. Separate workflow progress from navigation within migration review; unify UI.
6. Offer visible new-input/saved-input choices and concrete scope/use guidance.
7. Move exclusion to explicit scope controls, explain its declaration-only meaning
   and hide irrelevant inputs while preserving technical evidence.
8. Explain omission/inheritance/explicit unknown and avoid equating unknown with urgency.
9. Show purpose/candidate/configuration/support prerequisites and local empty states.
10. Add calendar year/month duration authoring with a visible Japan-time anchor,
    end-of-month clamping, exact seconds and the existing 315576000-second bound.
    Existing imported durations remain untouched. Add a separately provenanced
    `migration_goal_month` claim (`YYYY-MM`, years 0001–9999): approved explicitly
    by the maintainer in this chat. Month precision has no timezone/day conversion;
    it is planning information, never a contractual or organizational deadline,
    and cannot alter policy urgency/readiness. Keep absent/unknown states distinct.
11. Return location-filtered cohort totals/eligible counts from the same server
    membership logic; show exact owned/ambiguous exclusions and reject stale replies.
12. Delete an authored use with its exception count, offer undo, preserve unrelated
    drafts/groups/overrides and invalidate generated previews, never artifact files.
13. Apply USWDS form, link, navigation, alert and spacing guidance throughout;
    preserve bounded 20-row rendering, shared lossless archives and grouped reports.

Verification: meaningful Node state/controller tests, Rust HTTP/core/schema tests,
workspace fmt/test/Clippy/build and isolated optional CLI tests/Clippy; inspect the
combined diff, implementation review, required tests then test review. Browser
acceptance uses a new dedicated loopback server/tab and covers first input through
scan/results/new-tab report, filtered/edit/delete/undo authoring, generation,
HTML/context download and offline reopening, Keycloak and a different product,
JA/EN, both themes, keyboard, narrow layout, empty/error cases. Bind evidence to
source and binary hashes. Completion remains pending until every criterion passes.

Implementation review is assigned to `/root/uswds_implementation_review`, the
maintainer-authorized replacement identity. Its initial source is
`target/uswds-ui-20261120/evidence/REVIEW-SOURCE-r1.json` (SHA256
`d14608eefa8addc1d79ae47f8be2089124ccc6db740bcd585d1bf4e8d657d3bf`).
The only immediate post-freeze change is this review-assignment metadata.
Test review remains unassigned until required checks finish. Original review IDs
and their historical decisions remain preserved above/in the grouping record.


### Follow-up completion and review record

All thirteen original follow-up criteria are implemented and verified. This record
supersedes the pending/unassigned statements in the earlier execution-stage notes,
without replacing historical acceptance/failure records. No commit, push or
publication was performed. The startup --root boundary is retained; choosing a
folder is explicit and no OS-native picker is claimed.

| Criterion | Final outcome and evidence |
|---|---|
| 1: import formats | Inventory JSON32 MiB/context JSON1 MiB hints rendered beside inputs; valid imports, wrong JSON and size/binding rejection paths verified. |
| 2: initial folder | Boundary and selected folder separated; no initial selection; Django/FastAPI scans required explicit folder choice; confinement/symlink tests retained. |
| 3: report tabs | Scan/migration report links open separate tabs with language/theme; original inputs retained; migration-entry optional badges removed. |
| 4: result list | Actual root/project/import filename, request time in Japan time, status, newest/latest verified; unknown historical metadata is labelled. |
| 5: navigation | Overall progress stays separate from labelled migration-view navigation, with current-page state and keyboard focus. |
| 6: input guidance | Visible new/saved radio choices; scope→purpose/use→configuration/support guidance verified. |
| 7: exclusion | Explicit advanced declaration, dormant inputs, actor/time requirements and retained technical concerns; no evidence deletion or safety inference. |
| 8: unknown | Omission/inheritance/explicit unknown/false remain distinct; hint and core inheritance regressions verified. |
| 9: prerequisites | No empty support disclosure; prerequisites and inapplicable-question states visible; configuration change/reset verified. |
| 10: calendar inputs | Calendar years/months, explicit anchor/end-of-month conversion and exact seconds/bounds; goal month precision/provenance retained, every freshness state excluded from policy effects. |
| 11: filter | Server-derived filtered cohorts/counts and exact membership; core126→42 eligible/84 ambiguous, ownership/empty and delayed-response tests verified. |
| 12: delete/undo | A six-record group with one exception restored; separate2-record group delete/undo preserved another4-record draft/selection; stale view invalidated and artifact files retained. Pending membership blocks deletion/undo. |
| 13: complete UI | Actual scan→results→new-tab→authoring/edit/delete/undo→generation→HTML/JSON save→saved HTML reopen; JA/EN, themes,390px, keyboard, empty/error and large/different products verified. |

Implementation review: `/root/uswds_implementation_review` (authorized replacement).
Initial r1 review requested changes: planning-goal freshness influenced policy,
stale Next continuation navigated after cancellation, and standalone HTML retained
unavailable sidecar links. All were fixed and r2 accepted. A subsequent browser
finding added pending-selection deletion/undo guards; the same reviewer accepted
r3 without further findings. Reviewed source:
`target/uswds-ui-20261120/evidence/REVIEW-SOURCE-r3.json`, SHA256
`2901150064321dd05504747e2936417d36f76bf011a2768c1244cf949d995e6f`.

Test review: `/root/uswds_test_review` (authorized replacement), accepted r3 after
all required checks/browser evidence. No blocking coverage gap or important
untested requirement was identified. Its P3 documentation finding (“schemas
remain unchanged”) was corrected to preserve existing input meaning and schema
version1 while adding the approved optional field. JA/EN navigation/save labels
were also aligned. These final changes are prose only; runtime, schema, test and
binary hashes remain equal to the accepted r3 source. Main checked relative links,
paired factual consistency and the final combined diff; no Rust rerun is needed
for completion metadata or prose-only corrections.

Final successful checks (Rust1.93.1; all exit0): workspace741 passed with3 existing
intentionally ignored worker entrypoints; isolated all-feature CLI109 passed;
Node31 passed; workspace build/fmt/Clippy and all-feature/all-target CLI Clippy.
Exact commands, configurations and logs are in CHECKS-default-r3.json and
CHECKS-optional-r3.json under `target/uswds-ui-20261120/evidence/`.

BROWSER.md and BROWSER-OBSERVATIONS-final.json record actual browser actions,
viewports, outcomes, artifact paths and absence of final-page console errors.
DRAFT-UNDO-final.json proves retained unsaved membership/text. EXPORT-CHECKS-final
and CLI-AGREEMENT-final prove complete archive/context retention and complete
web/CLI assessment equality, beyond counts. Keycloak no-context8750 decisions
still form16 work groups; HTML5740852 bytes versus the historical56478304, with
no lost/duplicated decision. The final42-bound-context run forms20 groups and its
standalone HTML is5939283 bytes. These are scoped measurements, not a general
performance, usability or security certification.

Final delivered saved HTML files are under `target/uswds-ui-20261120/offline/`;
canonical contexts are `evidence/django-context-final.json` and
`evidence/keycloak-context-final.json`. The final live/output-only servers and
marked deliverable tabs remain available; superseded owned servers were stopped.
The original user tab/server was preserved. No required follow-up remains.


## Folder-browser follow-up (2026-11-20)

The maintainer requested direct list selection and Finder-like navigation instead
of the separate “Select the displayed folder” action and parent-folder button.
Single click selects a child without opening it; double click, Enter or its
explicit open chevron enters it. Back/Forward preserve each location's selection;
clickable path segments navigate ancestors and the current segment selects the
current directory, including the startup root. Opening a directory selects it;
the initial page remains unselected. Arrow/Home/End keys select rows, Space selects
the focused row, and Alt+Up navigates to the parent. This is a bounded browser
folder UI, with the existing root/symlink/Host/origin protections unchanged.

Acceptance: submit the selected path even when a different directory is displayed;
remove the extra selection action; preserve double-click targets during selection;
support history, ancestors, keyboard and touch opening; prevent scans during
pending navigation and retain selection/history on failed navigation. Verify actual
browser selection/navigation/scan, keyboard, narrow layout and paired JA/EN copy.
No Rust, API, persisted format, scanning engine or report behavior is changed.
Evidence will be recorded under `target/folder-picker-20261120/evidence/`.


Folder-browser completion: the implementation reviewer
`/root/uswds_implementation_review` accepted SOURCE-r2 after the paired README
correction; `/root/uswds_test_review` accepted its test coverage and evidence with
no unresolved findings. Both reviewed SOURCE-r2.json SHA256
`e8e18a9cf611e37a682a45c68fce887c1920e52657ac54c83fa7bd3cff823509`
on HEAD `3a812a4361cc7400f3811a1b09d7d82779a9012b` plus recorded dirty source.
Node28, HTTP15, confinement3 and workspace build passed (exit0); BROWSER.md records
1280/390px pointer/keyboard navigation and FastAPI-only scan from its parent list.
CHECKS-r2.json records commands, reuse and the corrected unavailable library-test
attempt. Existing Rust/core/report/optional checks remain applicable as described
there. Only this completion prose changed after review. SOURCE-final.json binds
final files and executable. User initial page tab15 remains open at the loopback
server identified by server-r1.log/PID53016. Prior user tabs/servers were preserved;
no commit, push or publication was performed. All follow-up criteria are verified.


## Sequential multi-folder scans and blocking progress (2026-11-20)

The maintainer explicitly approved multiple folder selection, sequential scans,
separate per-folder results, and a progress modal that prevents other page actions.
The prior one-folder UI constraint is superseded for selection only. Each existing
scan request/job/report still belongs to one folder, with the unchanged session
limit20 and one-active-job coordinator. Use an in-memory browser queue rather
than extending API/storage formats or adding parallel server work. Preserve
confinement, symlink, Host/origin protections, CLI output and existing user tabs.

Selection is visible via row toggles/checkboxes and an exact-path removable list;
it survives browsing. The operator can clear selection and scan in selection
order. A double-click opens/selects that folder, whereas the open chevron only
navigates. A native modal dialog blocks pointer/keyboard background interaction
and Escape during submission, execution, inter-folder transitions and temporary
poll failures. Progress identifies the current folder and item index, plus actual
phase/file counts. Before-unload warns while queue/work is active; pending items
are memory-only, not a durable background scheduler.

Completed/failed jobs remain individually in session history. A failed scan does
not prevent the next folder. A failed/uncertain start stops pending requests (no
automatic POST retry); preserve completed jobs and explain the stopped remainder.
All selected folders must fit remaining session capacity before starting. For
one default-mode scan retain automatic report navigation; multi-folder batches
finish on results/history so every folder can be reviewed, including failures.

Checks: meaningful Node controller cases for order/no overlap, modal lifecycle,
Escape/background guards, network failure/recovery, failed scan/start, capacity,
selection across directories/clear/remove and old single-scan restoration. Build
new embedded assets and exercise real modal focus/inert background, multiple
actual scans/results, single-scan default behavior, JA/EN and narrow layout.
Main owns all changes/tests; reuse the two authorized reviewers for implementation
then test review. Capture baseline/source/binary/logs under
target/multi-scan-20261120/evidence. No commit/push/publication.


Sequential-scan completion: `/root/uswds_implementation_review` accepted r3 and
confirmed the test-only r4 delta; `/root/uswds_test_review` accepted r4 coverage
and evidence with no unresolved findings. Reviewed source is SOURCE-r4.json
SHA256 `3c354e7dadb0676f71f340429a3999dc6a29b107bb984e3789ffffedbdff5435`
on HEAD `3a812a4361cc7400f3811a1b09d7d82779a9012b` plus recorded dirty source.
The initial implementation review found old-GET/uncertain-POST early unlock and
folder controls remaining disabled after completion. Both were fixed and have
regressions. Actual browser Tab focus was also corrected and verified on r3.

All final checks pass: Node40, HTTP15, confinement3 and workspace build exit0.
CHECKS.json/BROWSER.md under target/multi-scan-20261120/evidence record commands,
source/binary bindings, prior-check applicability and actual browser observations.
Final product remains r3; r4 adds only tests. Native modal, Escape/Tab/ShiftTab,
background actions, actual Django/FastAPI individual results and post-completion
folder navigation passed in the browser;390px fits without horizontal scrolling.
A default-mode fixture scan retained automatic report navigation. Ready/report
completion is distinct from coverage; Django's incomplete coverage stays visible.

Only this completion prose changed after reviews. SOURCE-final.json and
FINAL-CHECKS.json bind final source/evidence. User initial screen tab19/live server
PID59321 remains available. Temporary owned servers58990/59564 and browser test
tabs were closed; original user tabs and servers remain intact. No API/storage
format, Rust/core/rule/report logic, commit, push or publication changed. All
criteria of this approved follow-up are verified; no required work remains.


## User-requested usability follow-up (2026-11-20)

Implement the maintainer's screenshot feedback: queue cancellation stops all
unsubmitted folders while retaining the active result; explicit starting-folder
copy, remove self-evident hints/footer, add clear-selection spacing; styled scan
history without duplicate paths; report new-tab icon and clear folder-selection
return action. Migration overview loading and Next generation use blocking native
dialogs with focus management. Response tab navigation alone never starts work;
an explicit preparation button does. Shorten duration conversion copy, order
anchor/years/months, make explicit duration/goal changes mutually exclusive without
changing imported context contracts. Move evaluation time next to input method,
put radio choices on one desktop row and fold optional declaration expiry with its
assessment meaning explained. Preserve root confinement, formats and prior data.
At ordinary desktop widths the introductory sentence should fit one line; retain
responsive wrapping on narrow screens rather than forcing horizontal overflow.

Main owns implementation/verification. Reuse the implementation/test reviewers.
The user separately authorized one additional persona reviewer after fixes: an
application-security engineer planning PQC work, using concrete known purpose and
usage answers (never Unknown/Unanswered for purpose), actually navigating from
folder scan through migration work items and reporting comprehensive usability
and wording findings. Do not mistake simulated declarations for verified product
support. Evidence: target/ux-followup-20261120/evidence.


Screenshot-follow-up completion: `/root/uswds_implementation_review` accepted r7;
`/root/uswds_test_review` accepted final tests/browser/reuse with no unresolved
required findings. SOURCE-r7.json SHA256
`a0d6aa8238251d4c073ce4dc2b7562d8b112c5e7f8bde3720e54886ea5cd226e`
binds HEAD3a812a4361cc7400f3811a1b09d7d82779a9012b and dirty source/binary.
The separately authorized `/root/ux_persona_review` completed the real Django
scan-to-concrete-declarations-to-work-items/export/return journey on r5; its
unchanged report and exact simulated input are in evidence/persona. Main verified
later r6/r7 changes using the same inventory and saved context, then actual known
new input, native Next modal and report completion. Persona's expiry-guidance
finding was fixed with visible consequence text, without changing evaluation.
Observed-versus-declared purpose and unlabelled UTC display remain explicit
out-of-scope report UX proposals; no new task or evaluator change was created.

Node44, HTTP15, confinement3 and workspace build pass (exit0). CHECKS.json and
BROWSER.md record commands, configurations, source applicability, actual1280/390px
layout/modal observations, screenshots and exports. Desktop methods share a row
and intro is one line; narrow controls and native modal fit390px without horizontal
overflow. Declared expiry remains optional but its unknown-validity consequence
is now visible before expansion. Queue cancel stops unsubmitted requests and
waits for the active folder's result. Required reviews confirmed imported-context
preservation, visible validation failures, localization and immediate Next modal.

Only completion prose changes after review. SOURCE-final.json/FINAL-CHECKS.json
bind the final source. User initial tab22/PID65650 and persona result tab1/PID63967
remain available. User's original tabs/servers were preserved; main scratch tab
and owned r3 server were closed. No commit/push/publication. All specified UI
changes and the requested additional user-perspective review are complete; the
two additional report UX proposals are distinct from the verified deliverables.

## Current per-folder result navigation (2026-11-23)

The implementation at `87e27cd` gives every completed folder in a multi-scan
batch and session history a named result card. Open scan report opens that
folder's original report in a separate tab with current language/theme. Review
migration, when enabled/available, restores that folder's input or generated
review. Imported inventory can be reviewed but has no original scan HTML.
History is session-local, newest first, and retains the existing job limits.

Opening the work-items tab does not generate a report. Prepare review items
starts generation explicitly, with blocking progress and actual phase labels.
Cancel remaining scans affects unsubmitted folders; the active scan finishes.
The generated review uses the shared decision presentation documented in
[MIGRATION-REVIEW-GROUPING](MIGRATION-REVIEW-GROUPING.md#current-decision-presentation-2026-11-23).
Save report (HTML) and Save input (JSON) belong to the local input workflow;
Save assessment data inside the report exports evaluation JSON instead of
context. Context remains bound to the exact saved inventory bytes.
