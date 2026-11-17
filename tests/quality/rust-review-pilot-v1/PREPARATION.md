# Preparation record and execution handoff

Date: 2026-11-17. Source baseline: HEAD
`f07d782cd68cfac95d93585d5ace049f590160d4` plus preserved existing uncommitted
work. The six fixtures instead bind immutable historical commits in
`private/provenance.json`; they do not depend on the current dirty product code.

Authorized scope: prepare six fixtures/labels from existing Rust source, specify
execution and cost. No experiment, installation, Clippy/CI/product change, task
creation or superpowers use. Only this dataset and the existing experiment
proposal are changed. No source repair, new function, script or executable
benchmark harness is introduced. Copied excerpts are inert review data.

The previous synthetic-case design is superseded by the user's selected
historical-source approach. It has three before/after pairs covering identity,
Markdown rendering integrity and unknown SARIF coordinates. It does not claim
to preserve synthetic privacy/API case coverage; those were proposed examples,
not measured results. Selection is targeted and known to the preparer.

Verification will bind source excerpts, packets, labels and schedule using the
dataset manifest and `private/validation.json`. Rust workspace, browser and
model runs are outside this preparation; none is claimed. The command example
is checked against local CLI help, not executed. Existing unrelated user edits
are protected by baseline diff/config hashes.

## Preparation reviews

This request includes benchmark fixtures and an execution command example;
apply both independent review stages to the combined preparation change.
Preparation reviewers may inspect hidden labels to verify validity. Their reviews
are not any of the 24 A/B trials and do not estimate the supplement's benefit.
Reviewer IDs, source bindings and decisions are recorded here. Reviewers
must not delegate, edit files or issue model trials. Reuse these identities for
material follow-up within this request.

- Implementation review: `/root/prep_implementation_review`, accepted with no
  actionable findings. See `private/reviews/implementation.json`.
- Test review: `/root/prep_test_review`, accepted with no actionable findings.
  See `private/reviews/test.json`.

## Before any experiment

Human truth approval/freeze, execution authorization, agent/context budget,
actual model/tier availability, strict environment/tool isolation, enforceable
usage/time/spend bounds and telemetry remain execution prerequisites. No human
label approval, experiment success or product release acceptance is implied by
completion of preparation. Follow `RUNBOOK.md` after those conditions are met.

## Final preparation outcome

The six fixtures, source-based labels, twelve A/B input packets, 24-slot schedule,
runbook and estimate are prepared. Static integrity, local links, JSON and
whitespace checks passed; both preparation review stages accepted their scope.
No model experiment, Rust/browser run or fixture execution occurred.

The reviewed initial manifest SHA-256 is
`ccff6b4fb6525aba639e8f1b40c52f17ed48eee21930330bac408800ebc35746`.
Only preparation validation/review/source-receipt bookkeeping was added after
those reviews; case source, labels, requests, common policies, schedule, cost,
runbook and experiment proposal remain the reviewed bytes. The final manifest
binds those receipts too. `private/preparation-source.json` records protected
config hashes and observed unrelated checkout drift; whole-checkout immutability
is not claimed. No unrelated change was reverted.
