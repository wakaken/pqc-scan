# OSS quality gates

Design date: 2026-11-09. Status: proposed acceptance policy; passing tests or benchmark results are not asserted by this document. Current capabilities/gaps are in [CURRENT-GAP-ANALYSIS.md](CURRENT-GAP-ANALYSIS.md).

## Evidence and lifecycle

Every implementation task must record exact HEAD/worktree, selected scope, commands and exit statuses, fixture/corpus digest, result artifact paths, independent review, known limitations and unexecuted checks. Keep generated artifacts under ignored `target/`, outside scanned fixture roots. A historical successful CI run does not validate a changed checkout.

Planner selects `pending -> planned`; Executor moves `in-progress -> review`; Reviewer assesses correctness/security/compatibility; Evaluator alone marks `done` after required evidence; Reflection updates durable knowledge without duplication. This design adds pending work only. Preserve the existing 014/016 statuses and the pilot's scope. When an implementation encounters an AGENTS.md stop condition, stop before changing rule semantics, storage contracts or high-risk architecture, and obtain the required human decision.

The default delivery unit is one task, one PR, including implementation, tests and documentation. Internal implementation steps are not separate PRs or task states. Necessary prerequisite feasibility studies retain their own task and decision gate. This changes delivery planning, not acceptance criteria or role ownership.

Source review, local execution, independent review, browser acceptance and remote CI are different evidence classes. Do not report one as another. Failed/unavailable validation stays explicit. A rule count, test declaration count or successful test count is not a quality score.

## Baseline study before numeric targets (task 023)

1. Define truth units by family: source usage instance, configuration assignment, certificate object/attribute, dependency declaration/resolution, session fact or advertised capability. Static truth labels describe reference/configuration, not runtime reachability.
2. Build a redistributable corpus with reviewed positive, negative, ambiguous, malformed and unsupported cases across seven current languages, manifests/SBOMs, certificates/keys and configuration families. Include same-named APIs, aliasing, comments, dead code, old draft names, misleading filenames and duplicate occurrences. Use synthetic non-secret content and public certificate fixtures.
3. Have two reviewers label independently; adjudicate disagreements and record rationale/uncertain labels. Split tuning and held-out sets by project/pattern lineage to avoid template leakage. Record corpus/version/license and scanner/rule/catalog versions.
4. Run existing CLI/core on fixed local inputs with explicit threads, excludes and limits. Collect per-family confusion counts and coverage diagnostics, then repeat to measure semantic determinism. Before diagnostics exist, call completeness unmeasurable rather than filling zeroes.
5. Benchmark fixed small/medium/large and pathological fixtures with input digests, hardware/OS/toolchain/provider/features, cold/warm conditions, repeated runs and peak RSS/wall/CPU/bytes/target counts. Report distributions and noise, not one unlabeled duration. Do not scan an external product without explicit authorization.
6. Set precision/recall and performance budgets only after this baseline and maintainer review. Preserve holdout data to evaluate subsequent changes. Record environment limits and unsupported populations alongside results.

The measured development baseline and maintainer-approved local regression signals
are recorded in [TASK-023-ACCEPTANCE.md](TASK-023-ACCEPTANCE.md). On the same eligible
truth, TP must not decrease and FP/FN must not increase. On the same environment,
wall time or peak RSS above 2x the preceding warm maximum requires investigation.
These are local comparison signals, not universal targets or automatic numeric CI
failure thresholds. Probe limits are safety limits, not quality benchmarks.

## Required measurements

| Dimension | Definition | Report and failure interpretation |
|---|---|---|
| Precision | TP/(TP+FP), per truth unit and family | Publish TP/FP and denominator; no positives means undefined, not 100% |
| Recall | TP/(TP+FN) on independently labeled eligible truth | Record unsupported/skipped positive cases; do not improve recall by silently removing them |
| Ambiguity/abstention | Unresolved predictions and labels | Separate count and adjudication policy; never silently count as TP or true negative |
| False positives | Token/string, wrong API, comment, wrong algorithm/type/parameter, duplicate, provenance mismatch | Attribute regressions to detector/rule and evidence method |
| False negatives | Missed alias/import, parser failure, unsupported grammar/container, limit, scope exclusion, normalization loss | Keep detection miss and failed inspection distinct |
| Inspection completeness | File and applicable-stage terminal dispositions within declared scope | Selected/read/analyzed/partial/failed/unsupported/excluded counters reconcile; unknown ignored population explicit |
| Reproducibility | Same semantic facts for same input/config/versions | Exclude timestamps/run IDs; specify total ordering and duplicate policy. Report ordering drift separately |
| Performance | Wall/CPU/peak RSS and throughput over fixed corpus | Compare baseline distributions under same setup; thresholds approved after measuring noise |
| Compatibility | Public CLI/API and output contract corpus | Record intended semantic fixes separately from accidental schema/ID drift |
| Privacy | Synthetic sensitive canary survival by data path/format | No real keys; any secret canary in output/logs blocks release of that path |
| Protocol reproducibility | Known peer profile vs observed offers/selections/outcomes | No network Internet dependencies; distinguish client limitation, server rejection and incomplete attempt |
| Rule quality | Purpose, exact identifiers, standard/source edition, positives/negatives, known limits | New detection requires more than a larger rule count |

## Gate by change class

| Change | Required checks before done |
|---|---|
| Documentation/task-only (this change) | Whitespace/diff, links and required sections, task YAML fields/IDs/dependency DAG, code/rule/old-task immutability, source citation spot checks, independent design review; no Rust product execution required |
| Pipeline/diagnostics | Error injection, failure/empty distinction, counter reconciliation, unknown applicability, limits and partial result retention; existing scan/output regression |
| Crypto rules/certificates | Written standard-based specification and human review; positive/negative/type/filename/time/bundle fixtures; expected-result diff; unchanged unrelated findings |
| New domain/native output | Identity ambiguity tests, root/line/move cases, all model distinctions, schema/golden/tolerant-reader tests, legacy shadow comparison and explicit export-loss ledger |
| Reports/SARIF/CycloneDX | Offline official schema validation, reference integrity and semantic assertions, unknown locations/URI/Unicode units, escaping/canaries, deterministic golden output |
| TLS/SSH | Pinned dependency/features/MSRV/target record; isolated local peers; packet-type/no-auth assertion; timeout/traffic/connection/cancellation and address-binding tests; source/release status review |
| Release/platform | Supported OS/arch build/install matrix, bundled/discoverable rules, license/security/update docs, release artifact verification and declared support policy |

For Rust behavior changes run the pinned toolchain: `cargo build --workspace`, `cargo fmt --all --check` (apply `cargo fmt --all` only in implementation work), `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`; run `node --test shell/compact-report.test.cjs` when presentation/scripts are relevant. Run new probe feature combinations explicitly because default workspace checks may omit optional dependencies. For security experiments use the isolation requirements of the review workflow; unavailable isolation is not a passing test.

## Contract and golden strategy

- Keep existing report/CLI tests and all seven-language readiness fixtures. Do not rebuild test infrastructure merely to rename it.
- Check golden semantic JSON/SARIF/inventory with stable ordering and a documented volatile-field allowlist. Preserve fields, enums, CLI defaults, old Finding IDs and default filenames except a reviewed intentional fix. A schema additive field is not automatically safe for strict consumers.
- Version and separately opt in to richer native and CycloneDX output; never replace arrays with objects in place. Test consumers of old entrypoints and third-party Detector default adaptation.
- Validate official CycloneDX 1.7 and SARIF 2.1.0 schemas offline with pinned local dependencies/digests; prohibit network schema fetching in tests. Schema success alone does not prove accurate key size, trust, version provenance, unique references or PQC readiness.
- Add semantic checks for unknown values, draft identifiers, algorithm-vs-key-vs-certificate identity, safe omitted properties, declared-vs-resolved versions and unresolved reference handling.

## Privacy and human reports

Use synthetic canaries for every collection-to-output path: evidence match/metadata/preview, full file snippets, certificate metadata, dependency names/versions, actions, inventory, diagnostics, all report formats and stderr. Exercise EC/DSA/encrypted/PKCS#8/OpenSSH markers and inline secret-like text. Markers and parser format coverage are separate from general secret scanning; do not claim universal secret detection. Review public fingerprint/endpoint/subject metadata sensitivity and collect only what is needed.

Keep the existing English/Japanese catalog and full-payload shared action logic. New diagnostic/assessment copy must preserve exact technical facts, risk, numeric values, IDs, paths, source code and references. Tests compare machine outputs across languages and localized fact bindings. `humanizer-jp` is an optional development editing aid; `textlint` with a pinned, small Japanese configuration is an optional authoring check. Neither is a runtime LLM dependency, and neither may rewrite structured facts. Separate semantic correctness review from prose editing.

HTML changes require offline desktop/mobile/keyboard acceptance plus automated behavior checks. Node's simulated DOM tests do not prove real-browser accessibility/layout. Record browser/version/viewport and user-reported acceptance when applicable; do not expand the claim beyond the observation. Markdown must retain every occurrence and associated action without script support.

## Isolated protocol fixtures

Use local versioned test servers and CAs inside a network-isolated environment. Pin server configs/images/digests, run on loopback with explicit allowed CIDR, use ephemeral test keys, bound resources and clean up listeners/processes. Tests can observe message types/counts but must not retain secret keys or TLS session secrets. No public TLS/SSH host is a test dependency.

TLS matrix: TLS 1.2/1.3, supported hybrid/classical KX, HRR, unknown/unavailable group, certificate/name/trust/expiry errors, mTLS-required failure without credentials, alerts, timeout and oversized input. SSH matrix: classical/hybrid offers, unknown names/extension markers, KEXINIT-wait behavior, fragmentation, malformed framing and limits; verify zero userauth/service/channel requests. Completed SSH KEX uses explicit public-key pinning and a separate no-auth packet-type assertion.

## 1.0 release gate

All supported paths have measured baselines and agreed regression budgets; critical correctness/safety issues in the supported scope are resolved or explicitly excluded with accurate diagnostics; no silent inspection loss; schemas and compatibility policy published; deterministic and privacy contracts tested; platform/install/rule-pack evidence recorded; security reporting and release process documented. External publication/release verification is a future explicit action, not performed by this design task.

Do not claim complete coverage of all cryptography or universal migration compliance at 1.0. Unsupported languages/protocol profiles and residual uncertainty remain visible. See [OSS-ROADMAP.md](../roadmap/OSS-ROADMAP.md).

## Review record for this design

On 2026-11-09, an independent read-only builder/Reviewer examined the new design and task dependencies. Four material issues were corrected: explicit file-counter partitions/equalities; separation of hybrid construction from quantum assessment; ownership of internal dependency-parser parse-to-empty errors in task 027; and packaging/install evidence in 038 preceding final 1.0 evaluation in 037. A second review confirmed these corrections and reported no remaining material contradiction, planning blocker or unnecessary architectural expansion in the reviewed scope.

Documentation validation parsed all 22 new YAML records, checked pending status/unique IDs, existing target paths, acyclic/resolving dependencies, document links/fences and untouched tracked files. This is design validation, not implementation verification. Rust build/test/clippy/fmt, protocol interoperability, measured precision/recall/performance and current-HEAD remote CI were not executed. No implementation task was advanced or closed; no push, PR or merge was performed.

## Task 023 implementation evidence

The development-only [measurement protocol](../../tests/quality/PROTOCOL.md),
source-bound corpus and two independent label records now support baseline
measurements through the existing CLI. [Measured results and limitations](TASK-023-BASELINE.md)
record finite-corpus counts, actual diagnostics and repeated resource samples.
The baseline implementation and the maintainer-approved local criteria satisfy
task 023's development-only decision gate; task 023 is done. CI validates evaluator
sanity, corpus bindings, read/terminal-state arithmetic and semantic repeatability.
It publishes metrics without enforcing numeric score or latency/RSS thresholds.
Task 037, final-source hosted CI and release acceptance remain separate gates.

## Task023 criterion approval and local acceptance — 2026-11-11

The maintainer explicitly approved frozen-truth TP/FP/FN nonregression and the
same-environment2x prior warm maximum time/RSS investigation signals. The earlier
2026-11-09 design checkpoint is historical; current criteria are stated above. Existing independent technical
approval, fresh corpus/harness integrity checks and20 evaluator sanity tests allow
023 to close for its development-only baseline scope; see
[TASK-023-ACCEPTANCE.md](TASK-023-ACCEPTANCE.md). No numeric CI enforcement is
claimed. This does not close037, imply current-HEAD hosted CI, replace the ongoing
Cursor audit, change truth/labels or promise multi-platform/global accuracy.

## Protocol execution boundary

`tests/protocol/prepare-image.py` prepares an immutable tool/cache image. This
network-capable phase receives Cargo manifests and empty stubs, and performs only
locked dependency fetching and trusted tool preparation; it does not build or run
repository target code. The image ID and input lock/tool digests are recorded.

`tests/protocol/run-isolated.py` then builds and executes target code with no
external network, a nonroot UID, read-only root/source/cache, no host HOME,
credentials, SSH agent or Docker socket, and an explicit environment. Docker
cgroup v2 bounds the entire process tree: 6 GiB memory with no additional swap,
2 CPU quota and 128 PIDs. A 4 GiB tmpfs contains all build outputs, HOME, temporary
files and evidence; the only additional data-writable mount is 1 MiB shared
memory. The host enforces a 1200-second deadline and removes the container on
success, failure and timeout. Incremental caches and debug symbols are disabled
for this bounded test build; test/debug assertions remain enabled. Evidence extraction rejects links, traversal,
special files and aggregate output above 64 MiB.

The same boundary covers synthetic report previews and the product OpenSSH
fixture. Resource tests independently trigger cgroup memory OOM, aggregate
multi-file ENOSPC and descendant PID exhaustion, as well as read-only source/root,
loopback-only communication, empty environment and deadline cleanup. Run them
with `PQC_RUN_CONTAINER_TESTS=1` and `PQC_PROTOCOL_IMAGE_FILE` set to the prepared
image record. The legacy native macOS smoke runner is explicitly weaker and is
not the CI confinement gate. No hosted-CI success is inferred from local runs.

## PQS-QAM evidence reporting (TASK-041)

The independent [quality model](QUALITY-MODEL.md) and [metric definitions](QUALITY-METRICS.md)
classify evidence as Verified, Passed, Failed, Partial, Unsupported, Unknown,
Not Tested or Not Applicable. [Current measurements](QUALITY-BASELINE.md) and
[assessment](QUALITY-ASSESSMENT.md) are new snapshots; old baselines remain intact.
The existing baseline CI job also runs `tests/quality/report.py` over its raw
measurements and saves JSON/Markdown. Inconsistent truth/source/raw evidence is
an evaluation-infrastructure failure, not a measured product-accuracy regression.
Numeric accuracy comparisons are descriptive; no new numeric CI failure policy
is introduced. Performance beyond the existing same-environment 2x signal still
requires investigation, not an automatic claim of regression or acceptance.
Unsupported and unknown populations never become safe or zero-loss results.
Release acceptance uses the supported scope and individual evidence, not an
average score. [Follow-up work](QUALITY-ROADMAP.md) records unexecuted dimensions.
