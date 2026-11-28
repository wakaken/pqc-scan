# Registry-driven OSS evaluation adapter

This candidate supports evaluation of 90 primary OSS repositories and eight separate comparison repositories, bounded repairs, and source-bound before/after evidence. It adds a small adapter over the existing quality helpers. It does not change scanner schemas, detection semantics, historical records, or the truth matcher.

## Design checklist

- [x] Ground. Read the affected quality helpers, registry, project constraints, quality requirements, and historical subset records.
- [x] Sketch. Describe caller usage, types, signatures, ownership, failure behavior, and an alternative.
- [ ] Agree. Parent arena synthesis owns selection.
- [ ] Implement. Outside this candidate's scope.
- [ ] Scrap. Apply if pilot evidence invalidates the chosen shape.

## Caller usage first

The operator sees one entry point. Selection is independent from execution. A frozen plan is the unit that can be reviewed and rerun.

```text
python3 benchmark/run.py plan --registry benchmark/registry.json \
    --population primary --priority --protocol benchmark/protocol.json \
    --baseline-build benchmark/evidence/start/source.json --out <new-plan-directory>
python3 benchmark/run.py run --plan <plan.json> --phase before --resume
python3 benchmark/run.py run --plan <plan.json> --phase after \
    --build <new-build-receipt.json> --resume
python3 benchmark/run.py compare --plan <plan.json> --before <index.json> \
    --after <index.json> --out <new-comparison-directory>
```

`plan` accepts repeated `--oss ID`, `--group GROUP`, `--population primary|comparison`, `--priority`, or `--all`. Unspecified selection is an error. Different selector dimensions intersect. Multiple values within one dimension form a union. `--all` is exclusive with filters. Unknown IDs or groups fail rather than producing an empty selection. Registry order is canonical output order. The selected IDs and every selector are frozen in the plan. Each index lists every selected ID, including unavailable inputs.

A priority plan has 27 primary entries only when generated against the inspected registry. Counts are derived and validated, not copied into the CLI. Comparison inputs remain a separate aggregation even in an all-input plan. Per-OSS selection uses the same path as population runs.

The plan step does not acquire sources or execute scanners. Acquisition is a separate authorized operation that produces immutable acquisition receipts. The adapter consumes those receipts. No package manager, build script, target project hook, submodule checkout, or dependency resolver runs on corpus sources.

## Grounding and ownership

At inspected HEAD `cf3bfe2071228921f5a33d2a44e0f3b94de4d1de`, the worktree has untracked `benchmark/` files. `benchmark/registry.json` uses `repositories`, not `entries`. It holds 90 primary and eight comparison records. The primary split is 48 exposed historical tuning repositories and 42 not-yet-acquired holdouts. There are 27 priority entries.

`tests/quality/run.py` owns process measurement, environment observations, distributions, and synthetic fixture execution. Its `scan` function requires every fixture file to be selected and read. That contract is useful for synthetic tests and unsuitable for OSS roots with intentional exclusions, unsupported inputs, and incomplete inspection.

`tests/quality/external.py` owns external-source population verification, external selection configuration bindings, bound report checks, and annotated-interval adaptation. Its `score_annotated` calls `evaluate.score`. Predictions outside nominated intervals remain unattributed, not false positives.

`tests/quality/evaluate.py` owns canonical digests, truth loading, matching, and normalized output comparisons. Its one-to-one scorer remains authoritative. `load_truth` requires two distinct source label records and can validate freeze metadata. These records need independent provenance checks in addition to distinct names.

`tests/quality/evidence.py` owns offline evidence aggregation and applicability. Its checked reader rejects symlinks and unbounded evidence. It has a 64 MiB per-file contract. `report.py` owns existing assessment composition and count-to-metric arithmetic. Its synthetic coverage assumptions must not be imposed on OSS receipts.

## Module map

| Module | Responsibility and reuse |
|---|---|
| `benchmark/registry.json` | Human-reviewed identity, population, group, pin, license, scope, exposure, and historical references. No scan status or mutable latest-run pointer. |
| `benchmark/run.py` | CLI parsing and exit status only. Calls the OSS domain module. Does not implement scoring, acquisition, hashing, or measurement. |
| `tests/quality/oss.py` | New domain boundary. Validates registry and acquisition references, freezes plans, binds inputs and attempts, executes scans through `run.measure`, verifies reuse, and builds comparison indexes. Owns new OSS evidence schema. |
| `tests/quality/run.py` | Reuse `measure`, `environment`, `distributions`, and `resource_scope`. Independent measurement repair supplies unique output handling, descendant cleanup, and declared guards. Existing synthetic callers retain their contracts. |
| `tests/quality/external.py` | Reuse `validate_tree`, `selection_bindings`, and `score_annotated`. Reuse bound report verification only after the bounded-read repair. |
| `tests/quality/evaluate.py` | Reuse canonical encoding, digests, truth loading, semantic/order normalization, and sole scorer. Central bounded streaming hashing repair replaces whole-file `bytes_digest` reads without changing digest values. |
| `tests/quality/evidence.py` | Reuse checked bounded metadata parsing/applicability where schema fits. Do not pretend external large reports satisfy the existing 64 MiB evidence contract. |
| `tests/quality/report.py` | Reuse `metrics` and existing comparison arithmetic only with matching input/truth contracts. Do not fork matcher logic or rewrite the existing synthetic assessment schema. |

The adapter owns the new receipt structure. Existing quality modules retain their current schemas and callers. A small standalone adapter is preferable to distributing registry conditionals among all five modules.

## Public signatures and data shape

These signatures are a sketch, not implemented code. Internal helpers remain private.

```python
@dataclass(frozen=True)
class Selection:
    ids: tuple[str, ...] = ()
    groups: tuple[str, ...] = ()
    population: str | None = None
    priority_only: bool = False
    all_entries: bool = False

@dataclass(frozen=True)
class ArtifactRef:
    path: Path
    sha256: str
    bytes: int

@dataclass(frozen=True)
class ScannerBuild:
    binary: ArtifactRef
    rules: ArtifactRef  # Manifest of every actual rule file, not only default rules.
    source: ArtifactRef # Build-time source/dirty-patch manifest and build receipt.

@dataclass(frozen=True)
class BoundInput:
    id: str
    root: Path
    commit: str
    inventory: ArtifactRef
    selection: ArtifactRef
    license_scope: ArtifactRef
    truth: ArtifactRef | None

@dataclass(frozen=True)
class Plan:
    path: Path
    digest: str
    selected_ids: tuple[str, ...]
    baseline: ScannerBuild
    protocol: ArtifactRef
    input_records: tuple[ArtifactRef, ...]
    harness: ArtifactRef

# Each function includes validation, so callers do not sequence private stages.
def freeze_plan(registry: Path, selection: Selection, protocol: Path,
                baseline_build: Path, output: Path) -> Plan:
    raise NotImplementedError

def run_plan(plan: Path, phase: str, output_root: Path,
             *, build: Path | None = None, resume: bool = False) -> ArtifactRef:
    raise NotImplementedError

def compare_runs(plan: Path, before: Path, after: Path,
                 output: Path) -> ArtifactRef:
    raise NotImplementedError
```

A frozen plan references validated bound-input records and explicit blocked-input records. An unavailable acquisition is represented as `inputState=not_acquired` with a reason. It is never represented as a zero-file successful input.

`ScannerBuild.source` is captured when the binary is built. Calling `run.environment(baseline_binary)` after product edits observes the current checkout, not the binary's source. Store its useful runtime and host observations, but do not relabel that current source as the frozen baseline source. The given baseline executable and baseline rules under `/Users/waka/.cache/pqc-scan-oss98-20261125/` are verified against `benchmark/evidence/start/` before use.

The plan pins the registry bytes, selected entry bytes, protocol, input manifest, license scope, truth or explicit absence, selection environment, baseline binary/rules/build, harness source, feature configuration, command arguments, and output-format contract. Paths alone do not bind anything.

The attempt receipt separates independent facts.

```text
version
attemptId, planDigest, entryId, phase, sampleIndex, startedUtc, endedUtc
bindings {registry, entry, buildSource, binary, rules, input, licenseScope,
          selection, protocol, truthOrAbsent, harness, configuration}
execution {state, reason, exitCode, timedOut, resourceLimit, measurementRef}
inspection {state, scope, coverage, diagnosticCodes, diagnosticsRef}
truth {state, independence, exposure, scope, bindings}
accuracy = null | {authoritative annotated-interval scorer result}
artifacts {report.json, diagnostics.json, cbom.json, dependency-sbom.json,
           report.sarif, report.html, report.md, stdout.log, stderr.log, ...}
completion {validated, artifactBindings, receiptDigest}
```

`execution.state` is one of `succeeded`, `failed`, `timed_out`, `resource_limited`, `interrupted`, or `not_run`. Inspection separately preserves `complete`, `partial`, `failed`, `unsupported`, `unknown`, and `not_applicable`. A successful scanner process can have partial inspection. The adapter does not collapse detector states into repository safety.

The truth record distinguishes `absent`, `exploratory`, `independent_frozen`, `unsupported`, and `invalid`. Accuracy is null unless the input/truth binding and independent freeze prerequisites hold. Exploratory source judgments can be retained separately, but cannot populate independent accuracy. No findings does not create negative truth units.

## Input and licensed subset binding

Historical manifests and acquisition inventories are imports, not proof that a current checkout still matches. Verify URL, exact commit, inventory digest, every regular file's bytes/mode/hash, symlink target, and absence of unexpected files. Use `external.validate_tree` after the shared streaming hash repair. Selection binds both presence and absence of ignore/configuration files before and after each scan. Record the actual root and environment used for selection.

The two C# inputs must import `tasks/084/evidence/UPSTREAMS-OSS-SUBSETS.json`, not their full-root acquisition records.

| ID | Authorized historical subset | Frozen inventory |
|---|---|---|
| `Sustainsys__Saml2` | `src/foss` plus license notices, 168 files | `tasks/084/evidence/Sustainsys__Saml2-oss-subset-files.json`, SHA-256 `5e561d06a825161bf1c0552a33fe607be03526d1dd9dc49069b345da08df6752` |
| `bitwarden__server` | Exclude `bitwarden_license`, 8,128 files | `tasks/084/evidence/bitwarden__server-oss-subset-files.json`, SHA-256 `ff5a8688ecb0c153d9fb5fc2e611831f87b6b421be5156c56ba0eed4e3ed615f` |

The historical subset roots are under `/Users/waka/.cache/pqc-scan-oss-validation-084/oss-subsets/`. Preserve the original pins and parent-context omissions. The current registry roots point to full checkouts. Resolve to the subset record explicitly and reject use of the full root for these entries. Do not silently broaden the scope or reconstruct excluded parent build context.

Holdout acquisition freezes source and truth before scanner-output exposure. Until independent truth exists, scanning can supply descriptive inspection evidence with null accuracy, but cannot satisfy an independent-accuracy acceptance criterion. Historical study outputs are descriptive context and never the new before measurements.

## Immutable attempts and resumption

Create `runs/<plan-digest>/<entry-id>/<phase>/<uuid>/` with exclusive creation. Every process sample gets a new directory. A preexisting directory is an error. Write a start record before launch. Write a final receipt only after process exit, artifact validation, and post-run binding checks. A crash leaves an incomplete attempt that remains preserved.

An index is a new immutable snapshot, not a mutable latest pointer. It references attempt receipts by path, bytes, and digest. Resume searches indexed receipts for an exact binding match and verifies every artifact. It can reuse only successful processes with finalized valid receipts. A partial inspection can be reused as its truthful partial observation; it does not become completed evaluation. Missing outputs, changed inputs, truncated JSON, changed config, failed guards, and incomplete receipts force a new attempt. Earlier attempts remain visible.

Performance samples also bind a measurement-session ID, sample order, warmup policy, concurrency policy, and host environment. An interrupted session does not silently combine old warm samples with later cold samples. Rerun the required distribution in a new session unless the frozen protocol explicitly defines cross-session aggregation. Reuse never turns missing required repeats into a distribution.

## Resource and privacy boundary

The runner enforces a wall timeout, bounded file/output/log sizes, input/output byte budgets, and a minimum free-disk threshold. It records requested and available CPU/RSS guards and kills the launched process group on violation or timeout. It also cleans up descendants after a successful leader exit. A measurement receipt must describe the actual implemented enforcement scope.

On macOS, process-group cleanup and polling are not a kernel sandbox. `wait4` reports child CPU and high-water RSS, not aggregate live process-tree memory. RLIMIT settings are per-process where available and do not promise a total descendant RSS cap. If the protocol requires a guard the host cannot provide, mark that run configuration unsupported before launch. Do not claim unavailable confinement.

Use finite streaming reads for source, binary, rules, and artifact digests. Fix `evaluate.bytes_digest`, which currently calls `read_bytes`, so reusing `validate_tree` does not materialize a large file. A streaming hash still needs a declared maximum file size, total bytes, file count, and deadline. Check mode/size before reading, enforce the byte cap during reading, and verify metadata after reading. Reject symlink races with no-follow descriptors where supported.

Hash compressed output and decompressed output incrementally. Bound decompressed bytes before JSON parsing. An output over the parse limit is a preserved resource-limited attempt, not a missing report treated as zero findings. The existing `external.load_bound_report` currently loads compressed bytes eagerly and needs the shared bounded-reader repair before large-output reuse.

Avoid writing raw source or sensitive snippets into Git evidence. Keep complete machine outputs in the dedicated cache with bound artifact references. Commit only safe metadata and reviewed summaries. Preserve existing scanner redaction. Do not publish selection configuration contents. Hashing a private ignore file does not authorize copying its content.

## Before/after comparison

Pair entries only when input inventory, license scope, external selection, truth, command/protocol, relevant host configuration, and compatible output schemas match. The source, binary, and rules are expected to differ between before and after and remain independently bound. Any harness change requires both phases to run with the final harness or an independently assessed equivalence statement. Changed truth invalidates prior accuracy comparisons rather than improving the old score retroactively.

Compare normalized semantic and ordering digests with existing helpers. Retain all changed machine-output artifacts and field-level differences for every emitted format. Report accuracy counts through `external.score_annotated` and `report.metrics` only when truth qualifies. Descriptive finding counts never become precision. Repeated serial process samples feed `run.distributions`. Reports keep the primary population, comparison population, group, priority, and truth scope distinct.

A before/after index can be complete as an accounting artifact while many evaluation predicates remain unmet. The summary lists required, attempted, successful, partial, unsupported, failed, and unmeasured IDs. It must not declare the full study complete from 98 rows alone.

## Alternative considered

| Shape | Benefit | Cost and rejection |
|---|---|---|
| Recommended registry-driven OSS adapter | Three public operations hide binding, selection, attempts, and comparison policy. Existing quality scorer, measurement, and external validation stay authoritative. | Adds one domain module and one CLI. Keep metadata schemas small and versioned. |
| Generate a synthetic corpus per registry entry and call existing `run.run` | Most runner plumbing is reused. Existing tests appear immediately reusable. | `validate_corpus` requires MIT fixtures and excludes scanner-policy paths. `run.scan` requires every fixture file to be selected/read. OSS intentional exclusions and partial inspection would require disabling those checks or rewriting input scope. Either choice changes the existing synthetic contract or misstates OSS coverage. Reject. |
| Extend each legacy task084 script with registry selection | Lowest initial edit count for one historical dataset. | Repeats phase-specific paths, scorers/adjudication assumptions, and subset policy. New contributors would have several ways to execute one study. Historical scripts are evidence, not the permanent entry point. Reject. |

The recommendation passes the architectural red-flag screen because registry identity, attempt mutation, and truth matching each have one owner. It avoids per-population executor classes and thin stage wrappers. Registry counts and selected lists are derived once. Existing historical schemas remain intact.

## Verification required before the first 98-input run

Use standard-library temporary fixtures and a local fake scanner for harness behavior. Never run corpus code. Verify selector algebra, unknown selections, explicit missing acquisitions, unique attempt directories, interruption recovery, changed input/selection/rules/truth/harness rejection, bounded hashing, symlink rejection, growing output rejection, descendant cleanup, and finalized-output reuse. Exercise a successful partial-inspection receipt to prove it remains partial.

Test licensed subset resolution against the preserved subset inventories. A full-root override for either C# subset must fail. Test null accuracy without independent labels and equality with the authoritative scorer on a bounded annotated fixture. Test duplicate and overlapping truth intervals through existing validation. Test comparison incompatibility when inputs or truth differ.

Then run one small OSS input, inspect actual outputs, and verify every declared artifact and coverage reconciliation. Run the priority cohort after the pilot. Run all primary and separate comparison inputs only after measurement guards and receipt validation pass. Keep Cargo feature/build outputs isolated as required by `docs/MISTAKES.md`. Required integrated source validation and independent implementation/test-adequacy review remain the parent workflow's gates.

## Principles that changed this design

`Model the Domain` led to a frozen plan, distinct execution/inspection/truth states, and an immutable attempt receipt instead of mutable registry status flags. `technical-writing` and `unslop` kept the proposal focused on one explanation of ownership and tradeoffs. Their leaf skills were read for this candidate. The architecture red-flag checklist led to one OSS domain owner rather than separate phase executors.

## Evidence inspected

- `AGENTS.override.md`, `docs/agent/VERIFICATION.md`, `docs/agent/REPORTS.md`, `docs/architecture.md`, and affected sections of `docs/BEST-PRACTICES.md`, `docs/PATTERNS.md`, and `docs/MISTAKES.md`.
- `tests/quality/run.py`, `external.py`, `evaluate.py`, `evidence.py`, and `report.py`.
- `benchmark/registry.json`, `benchmark/WORKFLOW.md`, and the `benchmark/evidence/start/` file list.
- `tasks/084/evidence/acquisition/{bitwarden__server,Sustainsys__Saml2}.json`, full acquisition inventories, `UPSTREAMS-OSS-SUBSETS.json`, `OSS-SUBSET-SCOPE-DECISION.json`, and `TASK-084-RESULTS.md` subset descriptions.

This candidate performs no scanner runs, source acquisition, dependency resolution, or implementation changes. The inspected source revision and dirty status are recorded above. The only written artifact is this design under `/tmp/pqc-oss98-design-a/`.
