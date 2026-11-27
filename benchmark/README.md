# Run descriptive OSS comparisons

This developer tool records static scanner observations for the ordered population in
[`registry.json`](registry.json). The study requirements remain in
[`WORKFLOW.md`](WORKFLOW.md) and the frozen protocol. Scanner execution, inspection
coverage, and accuracy are separate results. Every invocation keeps `accuracy: null`
and `evaluationComplete: false`; independent truth, report browser observations,
comparison requirements, and final review require separate evidence.

## Select the intended population

Run the commands from the repository root. A selector is required. Repeated `--repo`
and `--group` values select a union within that field; different fields intersect.
The output retains registry order. Unknown or empty selections fail.

```sh
python3 benchmark/entry.py select --population primary --priority
python3 benchmark/entry.py select --group Rust --group Go
python3 benchmark/entry.py select --repo bcgit__bc-java
python3 benchmark/entry.py select --all
```

`--all` cannot be combined with narrowing selectors. Primary and comparison results
remain separate in comparison summaries.

## Freeze the scanner and input evidence

Provide the scanner binary, its frozen rules directory, and a JSON build receipt.
The build receipt contains these bindings:

```json
{
  "source": {"revision": "saved product revision and source evidence"},
  "binarySha256": "sha256 of the frozen executable",
  "rulesSha256": "evaluate.digest of relative-rule-path to sha256 mapping",
  "features": [],
  "buildProfile": "release"
}
```

`source` contains the saved source evidence, including dirty-source bindings when
applicable. The coordinator does not infer the binary's source from its current
checkout. `coordinatorEnvironment.provenanceRole` states that distinction. A changed
binary or rules directory needs a matching new build receipt.

Each registry row identifies its `root`, `scope`, fixed `commit`, `inputManifest`,
and `inputManifestSha256`. A manifest is an `external.validate_tree` JSON file list.
Regular-file records contain `path`, `mode`, `bytes`, and `sha256`; symlink records
contain `path`, `mode: "120000"`, and `symlink`. Sources stay outside Git. The tool
also accepts the existing historical acquisition records and bounded gzip manifests.
Licensed Sustainsys and Bitwarden subsets require their saved subset decision and
original inventory digest. The tool never falls back to an original full root.

`inputLimitations` retains declared unavailable files and uninitialized submodules.
Unavailable input coverage keeps the inspection partial. Explicit
`unreadablePresent` entries block execution because their contents cannot be verified.
Unexpected extra files also block execution. Missing acquisition is `NotRun`, never
an empty successful scan.

## Record a run

The protocol JSON fixes `threads`, `repeats`, `timeoutSeconds`, `minFreeGiB`, and
`requiredArtifacts`. It can bind a prose protocol using `document` and
`documentSha256`. The required artifact set includes all native report formats,
diagnostics, inventory, and both CycloneDX 1.7 exports. Named profiles can specify
`threads` and `repeats`. CLI timeout and free-disk overrides enter the execution
binding.

```sh
python3 benchmark/entry.py run --population primary --priority \
  --phase baseline \
  --binary /absolute/path/to/frozen/pqc-scan \
  --rules /absolute/path/to/frozen/rules \
  --build-receipt benchmark/evidence/start/build-receipt.json \
  --protocol benchmark/protocol.json \
  --output /absolute/path/to/oss-evidence
```

Output and scanned roots must be disjoint. The tool runs samples serially. Add
`--resume` to reuse successful observations only after it revalidates their input,
selection files, binary, rules, protocol, coordinator, configuration, and artifacts.
A partial inspection can be reused with its partial status retained.

Every attempt has a new directory under `runs/<repository>/<phase>/<attempt>/`.
`started.json` and `terminal.json` sit beside a fresh `output/` directory. New
invocations receive separate directories under `invocations/`. No attempt or
invocation is overwritten. Failed, timed-out, invalid, and abandoned attempts stay
in place and cannot satisfy resume.

The runner enforces the wall timeout and cleans up its process group. Free disk is
checked before each attempt. CPU and child high-water RSS are observations from
`run.measure`; RSS does not describe aggregate process-tree memory. The tool offers
no kernel CPU, memory, filesystem, network, or disk confinement. Hashing and parsing
have finite file, population, byte, and expanded-JSON limits. The supported
`limits.maxArtifactBytes` is 134217728 bytes per file; a different declared limit
is rejected. Expanded JSON has the same limit and total attempt artifacts are
limited to twice that size. Output-size validation
occurs after execution, so it is not a disk quota. A protocol that requires kernel
confinement is rejected.

The CLI returns 0 when all requested scanner processes produce verified output,
including authentic partial inspections. It returns 1 for incomplete execution and
2 for an invalid request. Inspect `inspectionComplete` separately. No exit code
establishes study completion or accuracy.

Set `"collectPerformance": false` in a new protocol to defer performance collection.
The runner preserves execution status, timeout cleanup, logs and artifact checks,
but does not collect elapsed performance, CPU, RSS or hardware observations.
Performance fields remain null, and summaries count these samples as unmeasured.
The effective configuration binds this choice, so measured attempts cannot satisfy
resume for an unmeasured run. Omitting the field or setting it to true preserves
the existing collection behavior.

## Compare preserved observations

Use the `receiptPath` printed by each run:

```sh
python3 benchmark/entry.py compare \
  --before /absolute/path/to/baseline/invocation.json \
  --after /absolute/path/to/final/invocation.json \
  --output /absolute/path/to/new-comparison.json
```

The comparison verifies referenced receipts and output bytes. It pairs repository
and sample identities, then checks input scope, selection, protocol, configuration,
execution environment, coordinator, and build-flag compatibility. Changed scanner product source, binary,
and rules are expected between baseline and final. Different inputs or settings
produce `Incompatible`; failed or absent paired observations produce `Unavailable`.

Report, inventory, diagnostics, and exports use the existing `evaluate` and
`task050.study.fingerprints` normalization rules. Semantic changes and output-order
changes stay separate. Finding identity, uncertainty, coverage, and references are
retained. Per-repository resource distributions use `run.distributions`. They are
descriptive; this command does not establish the alternating sample design required
for a speed claim. Failed samples remain in the comparison matrix.

## Verify the coordinator

The tests execute a first-party fake scanner. They do not acquire, build, or execute
OSS projects.

```sh
python3 -m py_compile benchmark/entry.py tests/quality/oss.py
python3 -m unittest discover -s tests/quality -p test_benchmark.py -v
```

## Audit generated exports

The small CI tests use synthetic fixtures and run through the existing quality-test discovery. Real OSS scans remain explicit local runs. Install the pinned audit-only Python dependencies into an external virtual environment using `benchmark/audit-requirements.txt`. Do not install dependencies from scanned repositories.

Write a JSON list of generated output directories, then run `benchmark/audit_outputs.py --directories <list.json> --output <new-audit.json>` with that environment. The checker uses locally pinned Inventory, SARIF and CycloneDX schemas without remote reference retrieval. It reconciles finding totals, legacy exports, native references, component provenance and selected omission counts. SARIF position abstention and filesystem-to-URI conversion remain intentional projections. HTML and Markdown still require separate rendered observations. The audit is not a detection scorer.

The acquisition code and raw source inventories for this run are preserved externally through `benchmark/evidence/acquisition-index.json`. Run-owned acquisition scripts accept the registry path and retain bounded source fetching, hook/filter suppression and immutable retry evidence. Copy a verified script into a new external acquisition directory before rerunning it. Sources and raw outputs must remain outside this checkout.

## Summarize verified populations

The summary command verifies four immutable invocation receipts before writing an
exclusive JSON/CSV pair. Supply the `receiptPath`, rather than the stdout wrapper.

```sh
python3 benchmark/summarize.py \
  --primary-before /absolute/path/to/primary-baseline/invocation.json \
  --primary-after /absolute/path/to/primary-after/invocation.json \
  --comparison-before /absolute/path/to/comparison-baseline/invocation.json \
  --comparison-after /absolute/path/to/comparison-after/invocation.json \
  --output /absolute/path/to/new-summary.json \
  --csv-output /absolute/path/to/new-summary.csv
```

The JSON separates primary and comparison populations, project groups, actual
finding-location extensions, native categories/rules, coverage counters and
resource distributions. Header files with unresolved C/C++ attribution remain a
separate bucket. Descriptive findings never become an accuracy denominator.
CSV cells reject control characters and guard spreadsheet formula prefixes.
Existing output paths are not overwritten.

The current assessor includes `enumeration_failed_entries` when deriving Partial.
Only the exact historical `oss.py` hash named by `LEGACY_ENUMERATION_ASSESSOR` can
verify the earlier enumeration aggregation. All receipt and artifact bindings
still have to match. Original terminal and invocation states remain historical.
Current compare and summary commands derive inspection again from native counters
and input limitations. Do not quote an old invocation's `inspectionComplete` as
the current assessment. See `evidence/inspection-reassessment.json`.

The existing CI quality job discovers the small synthetic tests in
`tests/quality/test_*.py`. Run representative and full source scans explicitly;
they require licensed, pinned local inputs and do not belong in small CI.

## Admit source annotation contracts

Before interval scoring, check the supplied source-reviewed input-format and
finding-anchor contracts separately from cryptographic labels. A true source
fact can still be unsuitable for a start-column scorer. A Python call may be
anchored at its complete callee, and a dependency finding at column1. A declared
dependency in an unsupported manifest is not an eligible detection expectation.

```sh
python3 benchmark/annotation_contracts.py \
  --corpus /absolute/path/to/frozen/corpus.json \
  --truth /absolute/path/to/frozen/truth.json \
  --contracts /absolute/path/to/source-contracts.json \
  --out /absolute/path/to/new-admission-result.json
```

The mandatory sidecar binds the original corpus, truth, checker, source audit
and routing/anchor documents. It supplies one supported-input decision and
expected anchor for every originally positive or negative unit, including
controls. Missing, duplicate, extra, malformed or changed evidence is rejected.
Source reads are bounded and do not execute the target. The checker neither
discovers anchors nor establishes their semantic correctness or independence;
the source review and exposure record remain necessary.

Exit0 means the supplied contracts fit the frozen intervals. Exit1 writes a
valid receipt listing not-evaluable units. Exit2 rejects invalid evidence or an
existing output. None of these outcomes modifies original labels, intervals or
scores. Do not shift finding coordinates or silently remove failed units to
improve a score. Keep additive validity corrections and null affected metrics;
a post-prediction sensitivity scope is not a new independent holdout.

## Reassess accuracy while performance is deferred

The `temporal_reassessment/` package performs bounded static rescans without
collecting CPU, peak RSS, elapsed-time comparisons or load observations. Its
process timestamps provide execution provenance and enforce the timeout. They
are not performance results. Keep this path separate from the controlled
performance study below.

Use the pinned audit dependencies in `audit-requirements.txt` and an absolute,
canonical, private `0700` directory outside the product checkout. Set
`PQC_SCAN_REASSESSMENT_STORAGE` to that directory. The source remains in the
checkout; source trees and raw/generated artifacts remain outside Git.

The execution stages have separate evidence requirements:

| Stage | Required evidence | Entry point |
| --- | --- | --- |
| Compile the private clock overlay and certificate witness | Actual compiler/source/dependency closure, unchanged shipping inputs, two exact clock expression edits, synthetic boundary tests | `adapter.py` plans and verifies builds |
| Prove certificate time conditions where the original clock is absent | Independent source-only run gate binding all ten subjects; all 98 recovery pins checked before any required raw job | `proof_coordinator.py` |
| Admit historical clock values | Independent review of authenticated observed seconds and every required raw proof, source relation, witness and overlay builds | `main.py derive-clocks` |
| Execute repaired 90+8 scans | Exact final admission, source/harness bindings, input/selection equality, minimum 4 GiB free space, 600-second timeout | `main.py run` |
| Compare and assess results | Independently approved exact compatibility changes; separately admitted source/claim/Native truth | `main.py compare` and the existing quality scorers |

For the certificate proof, pass the authenticated JSON file binding produced by
an independent reviewer. A source-only gate authorizes that proof only. It does
not approve unexecuted coverage, accuracy results or the main repaired scan.

```sh
python benchmark/temporal_reassessment/proof_coordinator.py \
  --harness benchmark/temporal_reassessment \
  --gate "$PQC_WITNESS_GATE_BINDING" \
  --output "$PQC_SCAN_REASSESSMENT_STORAGE/producer-proof"
```

Each successful project preserves its configuration, public certificate
endpoints, selected file hashes, actual source/input/selection checks, process
receipt and terminal record. Completed records are verified on resume. An
incomplete existing project is rejected instead of overwritten. Preserve the
failed attempt, resolve its cause, and use a new output directory. Final
98-project clock admission remains a separate check.

`OriginalClockCohort` verifies all four historical invocations before reading
their report or Native bodies. It derives the observed and absent clock cases
from those authenticated originals on every admission. A valid observed second
is replayed directly. An absent second requires the full raw certificate proof.
Malformed, conflicting, present-null or provenance-invalid clock metadata is
rejected even when raw evidence is available.

The producer writes `temporal-required-raw-index-v2` with exactly the freshly
derived absent-clock keys. No repository list or expected subset size selects
this branch. An empty index is valid only when every original clock is observed.
The reader preserves the older `temporal-coverage-index-v1` contract, including
its full 98 raw rows and validation of raw evidence for observed originals.
Existing failed helpers remain failed and do not become successful coverage.
Final clock derivation closes 196 historical rows, with 98 after clocks and no
fabricated baseline clocks. The main scan still validates every regular input
hash, input population, mode and selection before and after execution.

Private witness v2 retains no-follow symlink boundaries as enumeration failures.
Its eleven-subject source gate and eight-subject temporal admission bind a
separate source-policy relation and an actual v2 helper build. Every boundary
receipt must match an authenticated original diagnostic and input-manifest entry.
Regular-file counts, hashes, certificate objects and clock emissions retain
all existing checks. Boundary failures remain Partial and are not exclusions.
V1 remains strict and rejects enumeration failures.

Historical declared inspection and coverage-derived inspection are separate.
A legacy Complete declaration with enumeration failures is accepted only under
the exact historical assessor binding and only when enumeration accounts for
the difference. The original terminal, diagnostic artifact and coverage remain
unchanged. The new proof retains the coverage-derived Partial state. This
qualification does not excuse other failures or establish complete inspection.

The normal product binary and the instrumented overlay have different hashes.
Report the overlay as instrumented execution evidence. Do not describe its
results as an unmodified shipping-binary run. Observed historical detector
seconds and interval-equivalent seconds are different states. An interval proof
must account for every time-dependent certificate object and numeric clock
emission, not just objects retained in Native inventory.

Use `python benchmark/temporal_reassessment/ci.py` for the small offline
first-party witness tests, and the quality test suite for admission regressions.
These commands do not acquire or execute external OSS. Full corpus execution
requires the independently bound admission artifacts above.

## Resume the controlled performance study

The additive [completion index](evidence/completion-index.json) binds the frozen
driver, its independent review, raw quiet-window observations and the latest
product build receipt. The first attempt waited900 seconds without admission;
it launched no scanner measurement and accepted zero of five paired rounds.
Historical110 samples and the excluded observer preflight remain separate.

After other owned scans, builds, audits and browser operations finish, run the
source-bound controller in a naturally quiet environment. Its recorded30-second
admission and per-measurement CPU, memory and thermal checks remain mandatory.
The external CPU residual is an estimate; disk isolation and causal speedup are
not established by that estimate. Rejected windows remain rejected evidence.

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 \
  /Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/controlled-perf-fresh-f37888d15aca4599a78e84a3f7014ff1/controlled_performance.py \
  --owned-work-drained \
  --after-binary /Users/waka/.cache/pqc-scan-oss98-20261125/final-accessible-pqc-scan \
  --after-build-receipt /Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan/benchmark/evidence/final-accessible-build-receipt.json \
  --after-rules /Users/waka/.cache/pqc-scan-oss98-20261125/final-integrated-rules
```

This creates a new run without resuming rejected samples. The first five accepted
before/after rounds cover the fixed eleven representatives. Changed product,
rules, inputs or settings require applicable fresh evidence. The study remains
unfinished until valid controlled measurements and their final assessment exist.
Repeated performance samples use the frozen `evidence/performance-design.json`.
Drain owned build, scan, audit, test and browser work first. Record other host
load and keep causal performance conclusions inconclusive when isolation is
unavailable. The preserved run driver and its hash are in the result index.

The current study result is [REPORT-98.ja.md](../docs/quality/oss-corpus/REPORT-98.ja.md).
The machine evidence index is `evidence/results-index.json`. Missing acquisition,
unqualified accuracy scope or required isolation remains unfinished work.

### Fresh-source bounded cohorts

`fresh_cohort.py` adds a separate execution path for a new source epoch. Existing
v1 commands and records keep their original meaning. The new execution admission
must include `sourceEpoch` and `freshCohortEligible: true`. Its usual product,
harness, compiler, temporal, input, and environment gates remain mandatory.

Publish a `fresh-cohort-policy-v1` document with the exact admission binding,
`sourceEpoch`, nonempty `authorIds`, and registry-ordered `allowedFallbackIds`.
The latter is an explicit subset, including an empty subset. The fixed policy
fields are `initialTimeoutSeconds: 600`, `fallbackTimeoutSeconds: 3600`,
`maxAttemptsPerProject: 2`, `threads: 1`, `repeats: 1`, `minFreeGiB: 4`, and
`maxFileSizeMiB: 2`. Set `maxArtifactBytes` and `maxAttemptArtifactBytes` to
`boundary.MAX` and `boundary.TOTAL`, and `requiredArtifacts` to the ordered
`boundary.FORMATS` list. Set `performance` to null and
`performanceObservations` to false.

```sh
python -B benchmark/temporal_reassessment/main.py plan-cohort \
  --admission "$PQC_FRESH_ADMISSION_BINDING" \
  --policy "$PQC_FRESH_POLICY_BINDING" \
  --output "$PQC_SCAN_REASSESSMENT_STORAGE/fresh-plan.json" \
  --run-output "$PQC_SCAN_REASSESSMENT_STORAGE/run-fresh"
python -B benchmark/temporal_reassessment/main.py run-cohort \
  --plan "$PQC_FRESH_PLAN_BINDING" \
  --review "$PQC_FRESH_PLAN_REVIEW_BINDING" \
  --output "$PQC_SCAN_REASSESSMENT_STORAGE/run-fresh"
```

The plan seals all 98 ordered identities, sample zero, expected input and
selection bindings, historical clocks, binaries, source, harness, schemas,
environment, and the private output owner. Without `--run-output`, planning
chooses a unique `run-fresh-*` owner. An independent
`fresh-cohort-plan-review-v1` document binds `plan`, `sourceBindings`,
`harnessBindings`, and `validationEnvironment`. It requires `verdict: PASS`,
`reviewerModel: codex:gpt-6.1-sol@high`, a nonauthor `reviewerId`, and `PASS` for
`implementationCorrectness`, `testAdequacy`, `evidenceAuthenticity`, and
`resourceDifferences`.

Each project starts with one 600-second attempt. Only a verified `TimedOut`
terminal for an allowed identity permits one 3600-second attempt. Scanner exits
and rejected artifacts do not permit a retry. A successful fallback retains the
original failed attempt and sets `originalBudgetSatisfied` to false. The plan's
`maxLaunches` and `scannerBudgetSeconds` are finite upper bounds, not measured
performance. No attempt collects CPU, RSS, or timing comparisons.

At the start of each run or resume invocation, the runner revalidates all 98
input trees, manifests, selections, and sealed attempt expectations before any
scanner launch. Before and after each attempt, it checks the current project's
input and selection, shared source, harness, binaries, environment, bound
records, disk guard, and all previously protected attempt stores. Integrity
failures stop execution before the next launch. Local scanner failures remain
visible while other projects proceed.

Resume verifies complete attempts, requires existing project directories to
follow registry order, skips verified successes and terminal failures, and can
start only an allowed unstarted fallback. Protected project directories reject
extra entries. An attempt directory without a completion marker requires
explicit inspection. It is never removed or treated as unstarted.

The `fresh-cohort-index-v1` index retains exactly 98 rows in registry order.
`executionComplete` means the finite plan reached terminal states.
`artifactCoverageComplete` requires 98 selected valid artifact sets.
`evaluationComplete` remains false. Failed rows have null artifacts and no
selected attempt. `fresh_cohort.load_progress(binding)` authenticates such a
terminal partial index. `fresh_cohort.load_index(binding)` requires full artifact
coverage and returns the index and a repository-ID map. Complete evaluation
consumers must use the latter. Old-source rows cannot be added to this index.
`run-cohort` exits with status 1 if any required scan remains unresolved.

The existing quality discovery runs `tests/test_fresh_cohort.py` through
`tests/quality/test_temporal_reassessment.py`. Its public synthetic fixtures use
the canonical input, clock, schema, reference, and gzip checks. They do not run
external OSS or establish actual-data acceptance.
