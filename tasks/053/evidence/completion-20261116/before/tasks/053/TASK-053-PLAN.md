# Task 053 evidence integration plan

## Purpose, original acceptance and authorization

Implement Q6-01/Q6-02/Q6-16/Q6-18 through the existing quality reporting and CI
paths. Preserve all original task053 criteria and deliverables. User explicitly
authorized starting 053 while leaving 052 unfinished on 2026-11-15 JST. Other
pending dependencies (056, 058-061) remain unstarted; their missing evidence is
an input state, never a fabricated pass. Do not close 052 or modify dependency
criteria to make the integrated report appear complete.

Current HEAD is f07d782cd68cfac95d93585d5ace049f590160d4 on
codex/task051-dependency-supply-chain-review. Baseline preserves all prior dirty
051/052/063/064 changes. No commit, push, PR, merge, release, signature, tag,
remote settings change, badge application or external message is authorized.
Network distribution's existing hold and the paused protocols job remain.

## Approaches and recommendation

1. Prose-only evidence list: low cost, but cannot continuously validate hashes,
   cover every requirement or keep machine/human reports synchronized.
2. Extend report.py with an optional, offline evidence-catalog mode and a focused
   helper: recommended. Reuse current scorer/output behavior, existing quality
   and baseline jobs, immutable task evidence and standard-library Python.
3. Create a separate measurement platform and rerun every upstream study:
   unnecessary parallel infrastructure, greater cost and unauthorized work on
   pending tasks. No such platform or automatic task creation is planned.

## Design and compatibility

- `report.py --evidence-catalog <catalog> --out-dir <new target directory>`
  validates the authoritative requirements table and emits all-axis evidence,
  test-evidence and Markdown assessment outputs. It may accompany the existing
  `--baseline` mode; existing invocations/JSON/metrics are retained.
- A versioned developer-only catalog enrolls dated records with task ownership,
  artifact SHA-256, explicit scope/head/features/standards/method/environment,
  commands and exits, selected raw JSON observations, original source bindings,
  limitations/concerns/comparison and review references. It never executes stored
  commands or rewrites upstream evidence.
- Every authoritative requirement appears even when its owner has no registered
  measurement. Missing work is Not Tested/Unknown with a reason. Hash/path/shape
  failures are reporting-infrastructure errors; measured failures remain findings
  in a successfully generated report. No new numeric gate or overall quality score.
- Preserve observed historical results and independently compare measured source
  bindings with checkout bytes. Stale source remains historical, never current
  Verified. Report source bindings and catalog digest identify dirty sources.
- Separate command success from implementation-review and test-review evidence.
  Record reviewer identities, scopes and bindings; incomplete/stale review stays
  explicit. AI reviews are not relabelled as human GitHub approvals.
- Render Markdown and JSON from the same integrated object. Include Q1-Q6,
  target version/features, methods/environment/standards, observations, concern
  priorities, comparisons, missing work and an Unknown release decision.
- Existing PR quality gets lightweight offline catalog validation/reporting;
  existing baseline gets aggregation beside its more expensive measurements.
  Reuse current manual execution for costly measurements, preserve event/job
  conditions and protocol pause, and retain generated evidence on failure with
  explicit artifact retention. No new schedule or workflow dispatch by this work.
- Developer reports remain English; user-facing README CI descriptions stay
  factually paired in English/Japanese. No HTML product report changes are planned.

## Implementation and verification steps

- [ ] Add independent, hand-derived fixtures/tests for complete requirement
  coverage, pending/blocked evidence, source drift and raw observation retention.
  Observe expected failures before implementing the focused aggregator helper.
- [ ] Test path traversal/symlink/oversize/duplicate/hash/JSON-pointer rejection,
  source/review mismatch, failed measurement vs tool failure, empty denominator
  and Markdown/JSON fact parity. Never execute catalog command text.
- [ ] Add optional report.py dispatch and preserve existing baseline semantics;
  enroll actual historical evidence and 052 partial evidence with explicit scope.
- [ ] Reuse existing quality/baseline CI jobs and update documentation/retention.
  Verify workflow syntax/event selection without changing remote settings.
- [ ] Run meaningful Python suites and real catalog/report generation offline
  with finite time/output resources. Save exact source/input/controller hashes,
  commands, exits, failed attempts and four required target/durable outputs.
- [ ] Inspect combined task053 diff excluding prior work. Obtain one isolated
  implementation review, resolve findings, complete required checks, then one
  isolated test review; reuse each identity for material corrections. Main agent
  performs all implementation/investigation/tests/docs; reviewers cannot delegate.
- [ ] Reconcile original criteria, final-source evidence, pending dependencies
  and any unexecuted hosted checks before the completion decision. Preserve all
  unrelated work and do not start task054 or other pending tasks.

## Review focus

Missing evidence must not disappear; stale/passed evidence must not become a
current pass; log exit 0 must not manufacture adequate testing; malicious paths
and pointers must fail before output; numeric/product findings must not become
reporting-tool failure or an unapproved hard gate. Tests cover these boundaries.

This plan uses the existing authorized task and current main-agent/two-review
policy. General skill suggestions do not create extra agents, tasks, commits or
approval gates for routine implementation choices.

## Resumption on 2026-11-15

The user explicitly resumed053 after056/058-061 became done.052 stays unfinished.
The historical suspension and its baseline remain immutable; the new baseline
is evidence/resume-20261115/BASELINE.json. The saved report matched the active
restored source exactly and all three saved files were restored after checking
their recorded hashes. The 13 suspended tests passed unchanged.

Keep the agreed optional offline catalog design and original acceptance criteria.
Enroll all existing owner studies, including043 context, with frozen actual
artifacts and measured source bindings; missing facts remain explicit. Historical
reviews are retained as artifacts and are not automatically promoted to current
source-bound acceptance. Register the actual result/command receipts and selected
JSON observations without rerunning expensive measurements. An initial focused
red/green cycle adds malformed catalog rejection, explicit Tool Error execution
and preservation of Unknown ownership facts. Resource/path negative controls
exercise real FIFOs and bounded sparse files. CI uses the existing quality and
baseline jobs, offline aggregation, always-run evidence retention, explicit
30-day upload retention and no new triggers or protocol unpausing. Locally replay
the changed commands and validate workflow structure/event policy. Hosted
execution is not authorized and will not be claimed as observed.

The two existing reviewer identities are reused: implementation
/root/task056_implementation_review and test /root/task056_test_review.
No new main-agent work is delegated. The final assessment retains an Unknown
release decision;054 is not started. Task completion measures the aggregation
requirements, not a claim that upstream product quality or052 has passed.

## Completion

All original integration steps were executed and validated against final R3;
see TASK-053-RESULTS.md for the criterion-by-criterion decision, actualcommands,
retained red/green failures, both accepted reviews and unexecuted hosted scope.
The initial unchecked list and pending dependency wording above describe the
preserved pre-interruption plan, superseded by resumption and final evidence.
052 remains unfinished,054 unstarted and release acceptance Unknown.
