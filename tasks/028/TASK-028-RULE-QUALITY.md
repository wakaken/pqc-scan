# Task028 compatible rule validation and quality review

Base **b21330034d33ef30ec002c8e0d8222effd4b9d01**, clean latest origin/develop.
Branch **codex/scanner-quality-improvements** intentionally omits a task number so
later task commits can continue here. User authorized implementation, compatibility,
tests/docs/independent review and local commit; **no push/PR/merge**.

## Delivered scope and decisions

The existing loader now rejects blank IDs and non-finite/out-of-range confidence,
validating source f64 before conversion to the public f32 field. Inclusive0/1,
signed zero, valid custom ID bytes and scope behavior remain accepted. Every
actual WalkDir error propagates; even a failure after a valid YAML file prevents
a partial RuleSet. The existing regex and duplicate-ID checks remain.

Existing CLI flags, model/report fields, rule sorting, kind alias, unknown fields,
map/sequence forms, root-YAML input and empty-directory behavior remain intact.
Unknown/custom scopes are not rejected or rewritten, and no default warning stream
is added. Missing rule roots now report failed-to-load rather than no-rules-found;
both are operational exit1. Invalid metadata intentionally becomes an early exit1
without report writes. These corrections are the explicit task scope, not a new
restriction on valid custom matching semantics.

No detection pattern, scope, rule ID, Risk or severity was changed. The task's old
117 count was reconciled with **119 actual current built-ins**. The separate
[quality companion](../../docs/quality/rules/RULE-QUALITY.md) binds all8 rule files and119 rule snapshots,
then reviews purpose/algorithm/quantum/classical axes, source editions, method limits,
known gaps and244 synthetic pattern-target controls. It is development-only and
never loaded by the scanner. Existing public Rule fields and output schemas stay
unchanged; serde_json/sha2 are reused dev dependencies, with no new package version
or runtime dependency. The two lockfile changes are dev dependency-list additions.

[The loading/scope contract](../../docs/architecture/RULE-VALIDATION.md) documents the actual
asymmetric legacy semantics. Both READMEs, architecture and rules reference link to
it and the companion. Task023's corpus/labels/status and unapproved numeric budgets
are unchanged; its implemented baseline is used without claiming its budget gate
has been approved.

## Correctness and tests

Pinned Rust1.93.1, default workspace features, offline locked Cargo resolution:

- Workspace debug/release build, fmt and clippy warnings-denied passed.
- **214 Rust / 28 Node / 20 Python tests passed.**
- Thirteen new Rust tests: six loader tables, late traversal injection, source-bound
  quality companion, four real-detector scope tables and one actual CLI failure
  test. Existing regex/duplicate/map+sequence/seven-language/certificate/schema/
  privacy/inventory/policy tests still pass.
- Loader controls cover Unicode/ASCII blank IDs, preservation of valid custom IDs,
  negative/above-one/NaN/infinite/overflow confidence, `1.00000001` and `-1e-50`
  narrowing traps, valid endpoints/signed zero, missing root, unrelated extensions,
  readable root YAML and mixed-valid/malformed nested files. Private iterator
  injection proves a late walker error cannot return earlier successful rules.
- Actual detector controls retain regex scope nonfiltering, AST case/list/wildcard/
  JS-to-TS behavior, typed certificate no-filename-fallback versus unknown-scope
  fallback, and key/dependency nonfiltering scopes.
- Actual CLI list/scan invalid packs use exit1 before report output; no Clap-error
  skip is part of the final test. Five valid custom packs agree old/new across
  rules list/show/scan. Five invalid packs deliberately change to exit1, including
  previously rounded boundary inputs. Failed-after report directories are absent.

Initial tests reproduced missing-root success, blank IDs and invalid confidence.
The first full-suite attempt found an old missing-root message assertion, updated
to the corrected failed-to-load error with the same exit1 contract. Initial Node
policy tests also exposed the intentionally stale source registry and passed after
reviewed rebinding. Final logs exclude those failing attempts. No fixture outcome
or matcher was changed merely to make quality scores pass.

## Independent review and classification limits

[Independent source/classification review](TASK-028-REVIEW.md) approved final loader
and all119 rows. Three P2 companion issues were fixed: generic Java RSA/Go imports
were incorrectly limited to TLS; Ruby generic JWT API branches could not establish
RSA; configuration disable/exclusion operands needed explicit gap controls.
Classification now stays conditional where appropriate. The
[independent Evaluator](TASK-028-EVALUATION.md) also reran build/tests/fmt/clippy,
recomputed source/CI bindings and accepted task028 as done. ECDSA-only cipher examples
with excluded RSA/weak operands do not claim ECDSA is quantum resistant.

Eight known false-positive controls remain intentionally visible: three legacy
TLS prefix/boundary cases, generic Ruby JWT HMAC, disabled Apache TLS operands and
Apache/NGINX/Kubernetes cipher exclusions. The244 controls compare compiled pattern
targets only, not119 independently labeled end-to-end source populations. Method
limits explicitly include aliases, shadowing, configuration precedence, runtime,
provider/peer support, dependency selection, certificate trust and parameter gaps.
The companion does not replace legacy tags or certify complete detection accuracy.

## Compatibility, measurements and CI projection

Saved old default/inventory and current default/inventory/policy scan the same
current source/rules/one-thread input: **857 findings each**. Six legacy reports
and diagnostics agree after generation-time normalization. Old/current inventory
payloads agree after run/time/observation-ID reference normalization; stable scoped
asset IDs and facts remain exact. No CLI flag/default filename or report format
changed. The baseline saved binary hash matches task027's validated after binary.

Frozen task023 corpus/truth/labels remain byte-bound:41 files/5746 bytes,37
predictions,32 analyzed/8 partial/1 unknown. Per-family confusion counts and sorted
prediction dispositions agree across old/default/policy profiles. Each used three
accuracy repetitions plus first-measured and three warm samples at four fixed
sizes; semantic/diagnostic/score repeatability passed.

Warm wall-time medians, milliseconds:

| Input | Old default | Current default | Inventory + policy |
|---|---:|---:|---:|
| Small | 31.52 | 24.12 | 39.17 |
| Medium | 62.03 | 59.62 | 91.82 |
| Large | 137.07 | 129.64 | 264.12 |
| Pathological | 24.93 | 24.36 | 32.08 |

These descriptive local launch/report-write samples include5ms child polling,
not controlled cold caches or a general speed claim. Numeric budgets remain
unapproved. All raw CPU/RSS/wall distributions remain bound in measurement outputs.

CI expectations rebind **46 existing entries in two changed source files** with
unchanged source-line content/rule/column/count. One new exact observation is
registered in the new integration test: a synthetic private-header-only control,
with no private-key body. No production finding or broad omission is added.
Final partition **857 raw = 853 reviewed omissions + 4 retained**, with **113**
source hashes verified. Four artifact-only test observations remain retained;
positive-line requirements and failure thresholds are unchanged.

[COMPARISON.json](evidence/task028/COMPARISON.json) records72 current Rust/test/Cargo/rule
source hashes, metadata/binary bindings and environment evidence, ten successful check
commands, custom pack comparisons, source-bound CI audit and frozen measurements.
Detailed artifacts are local under `target/task028/`: final logs, `self-*`,
`final-{before,after,policy}/`, `custom-compatibility.json`, registry rebind/audit,
independent reviews and separate historical red/initial logs.

## Remaining constraints and unexecuted checks

Source confidence validation uses f64 rather than arbitrary-precision decimal
arithmetic. Nested symlinks/extensions remain outside historical loader selection;
no new filesystem sandbox or total rule-pack schema validation is claimed. Unknown
scope semantics are retained, not standardized. Fixed reference editions provide
classification context, not current-law compliance or universal migration dates.

No known matcher false positive was repaired in this task. No rule DSL, runtime
quality-metadata service, dependency download, external product scan, platform
matrix, browser/UI acceptance, remote CI, push, PR or merge was performed. Existing
human report layout did not change. Historical alert dismissals from earlier work
are not part of this task and were not modified.
