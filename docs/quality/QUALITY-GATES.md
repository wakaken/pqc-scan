# Product quality checks

[AGENTS.override.md](../../AGENTS.override.md) defines the active development and
review workflow. Apply the [verification requirements](../agent/VERIFICATION.md)
and the original acceptance criteria for the affected product scope. These checks
supplement those requirements; they do not replace them or introduce another
review process.

## Evidence and comparisons

Bind validation to the actual source revision and dirty changes, inputs, rules,
features, tools, configuration, commands, and exit results. Keep generated outputs
outside scanned fixture roots. Source review, local execution, independent review,
browser observations, and hosted CI establish different facts. Missing or failed
validation remains explicit, and a historical CI success does not validate changed
source. Test counts and rule counts are not quality scores.

The [approved baseline criteria](../../tasks/023/TASK-023-ACCEPTANCE.md) require TP
not to decrease and FP/FN not to increase on the same eligible truth. On the same
environment, wall time or peak RSS above twice the preceding warm maximum requires
investigation. These are local comparison signals, not universal targets or
numeric CI failure thresholds. They do not authorize new performance measurements.

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
| Documentation only | References, links, commands, behavior consistency, and English/Japanese agreement where applicable; follow the documentation category in the verification guide |
| Pipeline/diagnostics | Error injection, failure/empty distinction, counter reconciliation, unknown applicability, limits and partial result retention; existing scan/output regression |
| Crypto rules/certificates | Written standard-based specification and human review; positive/negative/type/filename/time/bundle fixtures; expected-result diff; unchanged unrelated findings |
| New domain/native output | Identity ambiguity tests, root/line/move cases, all model distinctions, schema/golden/tolerant-reader tests, legacy shadow comparison and explicit export-loss ledger |
| Reports/SARIF/CycloneDX | Offline official schema validation, reference integrity and semantic assertions, unknown locations/URI/Unicode units, escaping/canaries, deterministic golden output |
| TLS/SSH | Pinned dependency/features/MSRV/target record; isolated local peers; packet-type/no-auth assertion; timeout/traffic/connection/cancellation and address-binding tests; source/release status review |
| Release/platform | Supported OS/arch build/install matrix, bundled/discoverable rules, license/security/update docs, release artifact verification and declared support policy |

For Rust behavior changes run the pinned toolchain: `cargo build --workspace`, `cargo fmt --all --check` (apply `cargo fmt --all` only in implementation work), `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`; run `node --test shell/compact-report.test.cjs` when presentation/scripts are relevant. Run new probe feature combinations explicitly because default workspace checks may omit optional dependencies. For security experiments use the applicable isolation requirements; unavailable isolation is not a passing test.

## Contract and golden strategy

- Keep existing report/CLI tests and language-readiness fixtures. Do not rebuild test infrastructure merely to rename it.
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

## Release evidence

All supported paths need applicable baselines and agreed regression budgets.
Resolve critical correctness and safety issues in the supported scope, or preserve
accurate exclusions and diagnostics. Check inspection loss, schemas, compatibility,
determinism, privacy, platform/install behavior, rule packs, artifact verification,
and the documented security and release process against the release's original
criteria. Unsupported inputs and residual uncertainty remain visible. Validation
does not itself authorize publication.

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

## Quality reporting

The [quality model](QUALITY-MODEL.md) defines the six evidence axes and their
states. [Metric definitions](QUALITY-METRICS.md) explain scoring and denominators.
The [CI measurement contract](CI-MEASUREMENT-PLAN.md) describes source-bound
aggregation and retention. Aggregation does not execute high-cost studies or turn
unsupported and unknown populations into safe results. Release acceptance depends
on the supported scope and individual evidence, not an average score.
