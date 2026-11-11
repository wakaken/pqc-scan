# Task023 acceptance — 2026-11-11

Task023 is accepted as a **development-only labeled baseline and measurement
capability**. This does not accept the current product as OSS1.0 or replace
Cursor's ongoing independent audit. Product source was not changed.

## Resolved decision gate

The maintainer explicitly approved the concrete criteria on2026-11-11. The
approval was previously recorded in commit4291ae7, task037 and
`REMAINING-ROADMAP-VALIDATION.md`; task023's pending wording had not been reconciled.
The criteria are:

- On unchanged eligible truth, per split/family TP must not decrease and FP/FN
  must not increase; cells without denominators are not percentage targets.
- On the same host/profile/config/input, wall time or peak RSS above twice the
  previous observed warm maximum requires investigation. This is a local review
  signal, not statistical confidence or a universal performance guarantee.
- Static scanning is the formal1.0 target, probes remain experimental and local
  distribution validation covers macOS arm64. Actual1.0 acceptance remains037.

No new numeric threshold, label, truth unit or matcher tuning was introduced.
The machine baseline `task023/BASELINE.json` remains an immutable historical
record: its then-unapproved proposal is not rewritten as an approval at run time.
Current approval is this acceptance record and task023's closure_decision.
The approved later comparison remains documented in REMAINING-ROADMAP-VALIDATION;
this closure does not substitute different numeric limits for that decision.
CI still does not automatically enforce these numeric review signals.

## Evidence checked for closure

- Existing independent technical review: `TASK-023-REVIEW.md`, final approval after
  source-binding correction; no unresolved material finding in that scope.
- Corpus023-v2:41 files,57 truth units,30 positive/20 negative/2 ambiguous/
  5 unsupported. Corpus, both independent labels and adjudication hashes match.
  `validate_corpus` confirms input SHA/provenance and lineage split integrity.
- All original harnessBindings, including evaluator, runner, tests, protocol,
  developer README, fixtures and label files, still match the recorded hashes.
- The original local raw summary equals committed BASELINE.json after removing
  only the disclosed regenerable per-copy benchmark path bindings and the
  committed metadata additions. Original three-run repeatability flags are true.
- The saved later baseline at `target/continuation-20261111/baseline-final/` uses
  the same corpus/truth/input and release environment. Compared with the original
  task023 baseline, no split/family TP/FP/FN metric worsens. Two groups improve:
  holdout/dependency TP1→2 and FN1→0; tuning/source_api TP9→11 and FN5→3.
- Saved later warm time/RSS maxima are within the original task023 proposed
  resource signals as an additional consistency check, and within the separately
  approved later comparison. This is not a fresh benchmark of current HEAD.
- Fresh evaluator sanity execution, with all network denied:
  `sandbox-exec -p '(version 1)(allow default)(deny network*)' python3 -m unittest
  discover -s tests/quality -p 'test_*.py' -v` —20 passed, exit0.

Closure artifacts: `target/task-023-closure/evaluator-tests.log` and
`INTEGRITY-CHECKS.json`. Exact source/evidence hashes and the check scope are in
`TASK-023-ACCEPTANCE.json`. No corpus code was executed, and no new performance
measurement competed with the ongoing Cursor audit.

## Remaining work belongs elsewhere

Target-commit hosted CI/Code Scanning, final-source release regression measurements,
current product/platform acceptance and audit finding remediation remain001/037
or subsequent authorized fixes. Linux/Windows generalization, true OS cold-cache
measurement, multi-host/real-product accuracy and universal coverage are not
claimed by023. Publication/signing and license clearance are separate decisions.
These limitations remain explicit; they are not silently converted into passes.

Evaluator closes023 after the existing independent technical acceptance, fresh
harness/data integrity/sanity checks and explicit maintainer criterion approval.
No additional implementation or test-review agents were started: this request
reconciles acceptance documentation and adds no implementation behavior.
