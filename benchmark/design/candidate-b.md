# Receipt-based OSS evaluation

## Problem

Evaluate the 90 primary repositories and eight comparison repositories, repair confirmed defects, and compare the frozen baseline with the final integrated scanner. Preserve historical evidence and distinguish a successful process from complete inspection or established accuracy. The proposed design adds one OSS coordinator to the existing quality tools. It keeps the scanner CLI, report schemas, finding identities, authoritative matcher, and existing synthetic corpus contracts unchanged.

This is an explanation and design sketch. It contains no implementation. The grounding checkout is `/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan` at `cf3bfe2071228921f5a33d2a44e0f3b94de4d1de`. Commands below illustrate the proposed interface and were not executed.

## Usage

The operator uses one entry point. The entry point reads the registry and emits a new immutable invocation receipt, including when it reuses earlier successful attempts.

```sh
python3 benchmark/entry.py run --repo bcgit__bc-java --phase baseline \
  --protocol benchmark/protocol.json --profile baseline --resume
python3 benchmark/entry.py run --priority --population primary --phase baseline \
  --protocol benchmark/protocol.json --profile baseline --resume
python3 benchmark/entry.py run --all --phase after \
  --protocol benchmark/protocol.json --profile final --resume
python3 benchmark/entry.py compare --before <baseline-invocation.json> \
  --after <final-invocation.json> --out <new-assessment-directory>
```

Profiles are frozen protocol objects. `baseline` points to `/Users/waka/.cache/pqc-scan-oss98-20261125/baseline-pqc-scan`, the separately frozen baseline rules, and the build/source evidence under `benchmark/evidence/start`. `final` points to a separately built and frozen final binary, rules, and source receipt. A path is never sufficient to identify a profile.

`--repo` is repeatable. `--group`, `--population`, `--priority`, and `--all` cover group, population, priority, and all-project selection. Values within the same selector form a union. Different selector categories intersect. A missing selector, unknown ID, unknown group, or empty intersection is an error. Registry order determines stable output order. The invocation receipt records the resolved ordered IDs and registry digest, not merely the selector spelling. Each project retains population, group, languages, priority, split, and exposure facts from the registry. Comparison projects always have separate aggregates.

A Python caller needs two operations. The sketch derives the types from these callers.

```python
from oss import run_selected, compare_runs

invocation = run_selected(request)
comparison = compare_runs(before_receipt, after_receipt, output_directory)
```

`run_selected` owns resolution, admission, attempt creation, validation, scoring, and evidence publication. A caller cannot omit a validation step. `compare_runs` reopens and verifies receipts and artifacts, then returns an assessment with explicit incompatible or unavailable comparisons.

## Grounding

`tests/quality/run.py` owns process measurements, environment collection, synthetic materialization, and repeat distributions. `run.scan` asserts that every synthetic file was read. It therefore cannot be the OSS scan entry. OSS trees legitimately contain exclusions, unsupported files, read failures, and partial stage outcomes. Reuse `run.measure`, `run.environment`, and `run.distributions` after the independently planned process-lifecycle repair. Do not copy their implementations into the OSS coordinator.

`tests/quality/external.py` owns external tree verification, selection-input bindings, and scoring inside annotated intervals. Its `score_annotated` delegates matching to `evaluate.score` and leaves unattributed findings separate. It does not establish independent labeling. Its `validate_tree` verifies every supplied non-Git entry, including retained symlink identity without traversal. Its `selection_bindings` binds actual home, XDG, ancestor, root, and effective global-ignore inputs without retaining their contents.

`tests/quality/evaluate.py` owns canonical JSON digests, safe relative paths, truth validation, report semantic and order digests, and the authoritative one-to-one matcher. The current `bytes_digest` reads whole files and needs a streaming implementation before it hashes large OSS files. `external.load_bound_report` currently reads a compressed artifact at once and needs bounded streaming verification before adoption for potentially large outputs.

`tests/quality/report.py` owns metric denominators, ratios, distributions, and the older synthetic assessment. Its `build_assessment` enforces that synthetic corpus's truth, input, and all-file coverage. Reuse its pure metric and distribution functions. Do not send OSS receipts through `build_assessment` or weaken its existing contracts.

`tests/quality/evidence.py` provides bounded, no-follow, duplicate-key-rejecting evidence readers and source applicability checks. Its 64 MiB JSON evidence limit is a real admission boundary. New manifests exceeding the configured limit must fail explicitly or use a specified streamed representation. Do not raise that bound silently or load a 100 MiB source into memory.

The registry currently declares 98 identities, 90 primary, eight comparison, 27 priority, 48 historical primary acquisitions, and 42 not-yet-acquired primary holdouts. It is a target registry, not proof that those inputs or licenses have been revalidated.

## Shape

Use immutable receipt files as authoritative events. A terminal receipt records a fact that happened. A start receipt records an attempt's full intended bindings. A directory index is a derived cache and never decides validity.

The persisted representation is versioned JSON. Runtime records can use frozen dataclasses or validated dictionaries because this is a Python standard-library tool. Parse external JSON once at the coordinator boundary. Keep deserialization and validation private so external callers cannot construct an accepted successful attempt by supplying loose fields.

The following records describe the required shape. `Digest` means a validated lowercase SHA-256, and `ArtifactRef` contains a relative evidence path, byte size, SHA-256, and media type.

```text
RegistryEntry
    id, repository, population, group, languages, priority
    historicalSplit, priorOutputExposure
    historicalEvidence[]

AcquiredInput
    repositoryId, normalizedRepositoryUrl, commit, gitTree
    physicalRoot, scopeKind = FullTree | LicensedSubset
    manifestRef, manifestDigest, fileCount, regularBytes
    licenseDecisionRef, acquisitionEvidenceRefs[]
    originalManifestRef?, subsetDecisionRef?, omittedContext?

ScannerProfile
    name, binaryRef, rulesManifestRef, buildReceiptRef
    productSourceRef, productSourceDigest, features, buildProfile, toolchain

Bindings
    schemaVersion, repositoryId, phase, sampleKind, sampleIndex
    registryEntryDigest, acquisitionDigest, inputManifestDigest
    physicalRoot, selectionBefore, selectionAfter
    scannerProfileDigest, binaryDigest, rulesDigest
    productSourceDigest, productDirtyPatchDigest
    coordinatorSourceDigest, qualityHarnessDigest
    protocolDigest, truthBinding, resolvedConfigDigest
    argvTemplate, effectiveEnvironmentDigest, resourcePolicyDigest

TruthBinding
    state = Unannotated | Exploratory | IndependentFrozen
    truthArtifacts[], corpusDigest?, labelsDigest?
    freezeTime?, exposureEvidenceRefs[], independenceEvidenceRefs[]
    scope = AnnotatedIntervals

AttemptStarted
    schemaVersion, attemptId, createdAt, requestBindings

AttemptFinished
    attemptId, startedRef, completedAt
    execution = Succeeded | Failed | TimedOut | ResourceStopped | InvalidEvidence
    finalBindings, measurements, artifactRefs[]
    inspection, scoring, redactionValidation

Inspection
    status = Complete | Partial | Failed | Unsupported | Unknown
    originalScope, selectedFiles, readFiles, enumerationFailures
    fileStateCounts, stageStateCounts, reasonCounts
    diagnosticsRef, nativeInventoryRef

Scoring
    truthState, scope, accuracy = null | authoritativeScore
    unattributedCount, otherRuleCount, evidenceRefs[]

InvocationReceipt
    invocationId, requestRef, registryDigest, selectedIds[]
    rows[] = AttemptRef | ReusedAttemptRef | NotRunReason
    completenessByPopulation, exposureEvidenceRefs[]

Comparison
    beforeInvocationRef, afterInvocationRef
    perRepository[], primaryAggregate, comparisonAggregate
    unmetPredicates[], artifactRefs[]
```

No enum collapses execution, inspection, and truth into a single `passed` field. A process may succeed and leave a verified `Partial` inspection. Its output can be reused as an authentic partial observation, but it cannot satisfy a complete-inspection predicate. An unannotated successful process has `accuracy = null`. Exploratory labels can produce separately named exploratory counts, but the assessment's accuracy stays null until independent frozen annotation is established.

### Binding rules

The start receipt binds the requested selection state. The finish receipt records the observed selection state and verifies it matches. Both root inventories and selection bindings are checked immediately before and after execution. Content or selection changes make the attempt `InvalidEvidence`. This is an offline reproducibility check, not an OS filesystem snapshot or an adversarial confinement claim.

The execution key is the digest of all execution-affecting bindings, including input identity, scope, selection, scanner profile, protocol, harness, sample identity, effective configuration, and resource policy. The randomly unique attempt ID is separate. Exclude incidental output paths and wall-clock timestamps from the key, but normalize output placeholders in the command template. Record the exact launched argv separately. New attempts always receive new output directories.

The full resolved protocol includes scanner flags, format set, threads, root, rules, supported features, environment allowlist, locale, timezone, repeat count, sample order, warm-up policy, process limits, output limits, and parsing limits. Record absent selection files as absent. Keep source contents and private configuration contents out of receipts.

`run.environment(binary)` reports bindings for the current checkout. When running the frozen historical binary, it must be labeled the coordinator's current checkout environment. The baseline product source remains the saved `benchmark/evidence/start/source.json` and build bindings. Do not claim that current source built the baseline binary. The binary digest, frozen rules digest, build receipt, features, and saved product source must agree before admission.

No developer can infer final-source applicability from HEAD alone. Bind relevant dirty content and its preserved patch. A later product, rule, configuration, or harness change invalidates affected after-run evidence. Changing only the coordinator does not rewrite baseline observations. It creates a new coordinator binding and a newly assessed applicability decision.

### Acquisitions and licensed subsets

Keep `benchmark/registry.json` as the single target list. New immutable acquisition receipts identify fixed URL, SHA, tree, inventory, license scope, and actual materialized root. Registry facts and historical paths are inputs to resolution, not mutable execution state. Revalidate historical inputs by exact file, mode, size, digest, symlink target, pin, and repository identity. Inventory records are metadata. Raw OSS sources remain outside Git.

The two mixed-license projects require an explicit structured subset binding. The registry currently points to their original acquisition inventories and full checkout roots. Resolve their authorized inputs through `tasks/084/evidence/UPSTREAMS-OSS-SUBSETS.json`, not those original roots.

- Sustainsys uses `tasks/084/evidence/Sustainsys__Saml2-oss-subset-files.json`, 168 recorded files, subset root under `oss-subsets/Sustainsys__Saml2`, and `src/foss plus license notices` scope.
- Bitwarden uses `tasks/084/evidence/bitwarden__server-oss-subset-files.json`, 8,128 recorded files, subset root under `oss-subsets/bitwarden__server`, and the original tree outside `bitwarden_license`.

Bind the historical subset decision, subset inventory hash, original pin and inventory hash, and omissions in a new acquisition receipt. Preserve original relative paths. A missing subset receipt or digest mismatch prevents execution. Never fall back to a full root or recreate a subset by guessing a substring filter. Full-tree coverage and effective missing parent dependency context remain unestablished for these two inputs.

The 42 new acquisitions get new fixed pins and inventories. Do not execute their code, build scripts, hooks, submodules, LFS hydration, or dependency managers. Acquisition failure stays a missing acquisition, rather than a zero-finding scan.

### Truth and exposure

The protocol freezes truth requirements before output inspection. A true holdout's independent annotations and freeze evidence must precede baseline output exposure. The registry's historical `split` alone cannot establish this. If a selected holdout lacks frozen truth, the coordinator either records `NotRunReason` for an accuracy run or performs a separately declared descriptive run and records exposure. A later annotation may be useful, but cannot regain pre-exposure holdout status.

Store new independent annotations once, as references to the existing truth format. Invoke `evaluate.load_truth` and applicable freeze validation where the format supports them, then `external.score_annotated`. Do not duplicate truth, matcher logic, or interpretation of repeated predictions. `score_annotated` is the only OSS scoring path, and its `accuracy` comes from `evaluate.score`. Aggregate metric denominators with `report.metrics` only after preserving the scored scope and exposure groups.

### Attempts and resumption

Each attempt gets `<cache>/runs/<repositoryId>/<phase>/<attemptId>/`. Create it exclusively. Write `started.json` exclusively before launch. Open process logs exclusively in that new directory. Write final artifact hashes and a terminal receipt to a temporary file, then atomically publish the terminal file without replacing an existing one. The output directory stays outside every scanned root.

A crash can leave a start receipt, partial outputs, or a temporary terminal receipt. Those are visible abandoned attempts and never reusable. A fresh invocation creates a new attempt. Do not complete or repair an abandoned directory in place. A separate later abandonment event may describe it without editing its files.

For `--resume`, derive the desired execution key, enumerate candidate successful terminal receipts, rehash the full bound artifacts, revalidate current input and selection bindings, and check all requested sample identities. Reuse only a successful, fully bound, unmodified observation with complete required output artifacts and successful validation. Failed, timed out, resource-stopped, abandoned, unknown, incompatible, or corrupted attempts never satisfy reuse. A fresh invocation receipt points to reused attempts and records why reuse remains applicable. Never pick a run by latest timestamp alone.

A successful process with a partial inspection can be reused only with that `Partial` status preserved. It cannot be substituted when the requested predicate requires complete inspection. Accuracy availability and rendered-report acceptance remain separate evidence requirements.

A unique invocation freezes its expected project and sample matrix before launching. Expected IDs and sample slots determine completion. Directory counts or discovered receipts cannot make a missing project disappear. Determinism requires matching semantic and diagnostic digests across required repeats. Preserve raw order digest changes separately.

### Resource limits

The independently repaired `run.measure` remains the one process runner. It must guarantee unique non-overwriting logs and clean up the launched process group on timeout and on leader exit when descendants remain. The OSS coordinator consumes its measurements and cleanup result. It does not add a second polling runner.

The protocol gives finite wall, CPU, RSS, per-process output, per-attempt disk, aggregate study disk, hashing, manifest, and JSON parsing limits. Admission checks remaining disk before every sample. Repeated performance measurements run serially, without builds or concurrent scans. Wall measurements exclude acquisition and validation hashes unless an explicitly separate end-to-end measurement says otherwise. Keep first-run and warm-repeat policy visible.

Record each limit's capability as `Enforced`, `Observed`, or `Unavailable`, with the measured scope. `wait4` CPU covers the child and waited descendants. Its RSS is the child's high-water value, not aggregate process-tree RSS. macOS observation and process-group cleanup do not provide strict process-tree memory, CPU, filesystem, network, or disk confinement. A strict required capability that is unavailable yields `NotRunReason` or `Unsupported`, rather than an invented guarantee. Polling overshoot remains recorded for observed limits.

Hash through a bounded buffer and a no-follow regular-file descriptor. Bind size and mode before reading, reject unexpected growth, and enforce per-file and total-byte budgets. Preserve valid symlink metadata without following targets. Extend the shared digest helper while preserving the existing call signature; do not add a separate OSS whole-file hasher. Compressed artifact verification must stream compressed hashes and enforce declared uncompressed size and parser limits.

## Public signatures and ownership

```python
# tests/quality/oss.py

def run_selected(request: RunRequest) -> InvocationReceipt:
    # not implemented

def compare_runs(before: Path, after: Path, out: Path) -> Comparison:
    # not implemented

# tests/quality/run.py, existing helpers remain authoritative

def measure(command, directory, timeout):
    # existing public shape, independently repaired lifecycle and unique logs

def environment(binary):
    # existing shape, provenance role is explicit in the OSS receipt

def distributions(samples):
    # existing implementation

# tests/quality/external.py, existing helpers remain authoritative

def validate_tree(root, files):
    # reuse; bounded descriptor hashing comes from the shared digest helper

def selection_bindings(root, *, home=None, xdg=None, cwd=None):
    # reuse with the exact effective environment

def score_annotated(data, labels, findings):
    # reuse without a new matcher
```

`RunRequest` has registry path, selector, frozen protocol path, profile name, phase, evidence root, resume policy, and sample purpose. It does not expose internal stages or let the caller supply a trusted success flag. Execution config comes from the bound profile and protocol rather than arbitrary independent Python keyword arguments.

| File | Ownership |
| --- | --- |
| `benchmark/entry.py` | Thin argparse adapter and exit behavior. It imports only the coordinator's public operations. |
| `benchmark/registry.json` | Single ordered project list and historical metadata. |
| `benchmark/protocol.json` | Versioned study contract, profiles, sample matrix, truth policy, limits, and comparison criteria. Freeze by digest. |
| `tests/quality/oss.py` | Registry and receipt boundary validation, acquisition resolution, admitted attempt state, reuse decisions, OSS assessment projection. One owner of new receipts. |
| `tests/quality/run.py` | Process lifecycle, measurements, current execution environment, distributions. |
| `tests/quality/external.py` | External trees, selection bindings, source-bound annotated scoring, bounded report loading. |
| `tests/quality/evaluate.py` | Streaming shared file digest, canonical digests, authoritative matcher and truth contracts. |
| `tests/quality/evidence.py` | Existing bounded evidence readers and applicability logic. |
| `tests/quality/report.py` | Existing metrics and synthetic assessment. Its synthetic contract remains intact. |
| New focused quality tests | Receipt reuse, subset scope, incomplete states, compatibility, and process behavior. |

The new coordinator deliberately stays one cohesive module until its size or testing needs justify a split. A generic event bus, job scheduler, plugin registry, or database adds no capability needed for 98 sequential targets. Private helpers can separate the coordinator's parsing, key calculation, and projection without creating public temporal stages.

## Before and after comparison

`compare_runs` first verifies both invocation matrices and every referenced receipt and artifact. Match by repository identity, input inventory, explicit scope, selection, resolved configuration, sample purpose, protocol compatibility, and truth scope. Different scanner product, binary, and rules bindings are expected for before and after. A different input or licensed subset is an incompatible comparison, not a regression.

Compare native report, diagnostics, inventory, and all required machine exports using their applicable semantic contracts. Use `evaluate.semantic_digest` and `order_digest` where they apply. Remove only specified volatile timestamps. Do not drop uncertainty, partial inspection, finding identity, coordinates, unsupported inventory, or references to force equality. Separate explained intended changes from unexpected differences, each with source-backed evidence.

Report independent annotation metrics only on the same bound truth and scope. A zero-finding unannotated project contributes findings and inspection counts, with null accuracy. Keep tuning, true holdout, post-exposure annotation, and comparison populations separate. Performance comparisons use comparable successful sample sets and `run.distributions`. Failed samples and unavailable CPU or RSS do not silently disappear from denominators.

A report check receipt binds the exact generated HTML, scanner profile, viewport, browser observations, and captures for representative normal, empty, partial, and repaired cases. Browser acceptance follows `docs/agent/REPORTS.md`, including a dedicated loopback server and loaded-offline checks. The coordinator links that evidence. It cannot infer browser acceptance from DOM tests or report hashes.

A repair creates new source, binary, rules if changed, and after attempts. Preserve baseline and failed after attempts. Independent correctness and test-adequacy review target the final integrated source and final evidence. Reassess review applicability after subsequent changes.

## Alternatives considered

| Shape | Complexity hidden | Complexity exposed | Decision |
| --- | --- | --- | --- |
| Immutable receipts with derived indexes | Crash recovery, source applicability, sample identity, and reuse validation belong to one coordinator. | Operators retain the evidence directory and bound acquisition roots. | Recommended. Per-attempt facts permit simple offline inspection and avoid shared mutable status. |
| Mutable registry status plus one summary per repository | A central table makes current progress easy to query. | Every writer must coordinate status transitions, retries, changed profiles, and partially written outputs. A caller can read stale `complete` state. | Rejected. It makes identity and execution history compete for one record, and overwrites the evidence needed for repair comparison. |
| SQLite event store with worker leases | Transactional claims, indexing, and concurrent dispatch are centralized. | Operators must preserve a database, WAL lifecycle, schema migration, and artifact reconciliation. Callers still need artifact hashing. | Rejected for this run. Sequential performance execution and 98 targets do not require distributed ownership. |

A full global append-only event journal also loses. An interrupted final JSON line or multiple writers introduces shared-file recovery. Immutable per-attempt receipts provide the useful event semantics without that shared state. A disposable index can accelerate lookup and can always be rebuilt from verified terminal receipts.

## Tradeoffs accepted

We accept hashing before reuse in exchange for proof that a path still names the recorded input and output. An mtime shortcut cannot establish this.

We accept another attempt after a crash in exchange for immutable failed evidence and simple recovery. Reusing partial output is not worth a second recovery protocol.

We accept descriptive results with null accuracy when independent truth is unavailable. A receipt proves a bound observation, not truth independence or product safety.

We accept observed resource limits on macOS only when the protocol explicitly allows those capability levels. Required unavailable confinement keeps the corresponding execution unfinished.

The interface has two substantive public operations. They hide the reuse and evidence rules, while existing specialist modules retain their authoritative contracts. The caller chooses the study and scope, rather than coordinating internal stages.

## Design screen and verification plan

`Model the Domain` changes the design from boolean completion flags to separate execution, inspection, and truth states. `Make Operations Idempotent` changes retries into fresh attempts and verified reuse, rather than continuation inside a half-written directory. Both leaf skills were read for this candidate.

The design has one writer for new receipts, one project list, one scorer, and one process runner. No public pass-through wrapper adds a second execution or scoring path. The main remaining implementation risk is an oversized coordinator. Extract a cohesive receipt store only if the implementation demonstrates repeated state rules, not merely because operations happen in sequence.

The initial throughput gate is a tiny synthetic external tree. Verify one fresh run, verified resume, changed source, changed rule, changed selection file including absent-to-present, changed truth, output corruption, and a crash after every publication boundary. No build or execution of OSS input is needed for these tool tests.

Verify explicit unsupported, partial, unknown, read-failed, and timed-out cases. Verify that missing annotation keeps accuracy null, independent labels use the existing scorer, and duplicate predictions retain its existing behavior. Verify both licensed subsets cannot fall back to their original full roots. Test gzip expansion limits and a file that grows during streaming hash. Test the repaired process runner with a child that leaves a descendant.

After the pilot, execute the priority 27 under the frozen protocol. Check disk and measured throughput before the remaining population. Final verification must cover the integrated source, meaningful positive and negative product regressions, required optional features, all mandatory checks in `docs/agent/VERIFICATION.md`, and independent correctness and test-adequacy assessment. Keep Cargo outputs isolated by feature matrix.

The exact completion predicate still comes from the original task and `benchmark/WORKFLOW.md`. Missing acquisitions, truth, required comparisons, report observations, validation, or review cannot be converted into a completed task by the coordinator's summary.

## Synthesis decision

This is candidate B for parent synthesis. It recommends immutable per-attempt receipts with no global mutable job state. The parent must compare it against the other structurally distinct candidate before selecting the final design.

## Next implementation step

Implement the coordinator's receipt admission and verified-reuse tests over a tiny external fixture before wiring real acquisitions or launching the frozen baseline.
