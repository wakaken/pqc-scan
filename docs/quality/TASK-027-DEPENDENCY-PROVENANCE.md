# Task 027 dependency provenance validation

Implemented on `codex/task-022-sarif-location` from clean base
`2baac32cc60adfae5cb2aca957803aae0ae414c5`. User authorized implementation,
tests/docs/independent review/local commit on the same branch, no push/PR.

## Delivered behavior and compatibility

Dependency capture now separates source-declared constraints, optional concrete
lock-record resolutions and fixed provenance, using the existing six inventory
attributes. A manifest `^1.2.3` or SBOM version does not assert installed `1.2.3`.
Git/path/archive/workspace locators are unresolved and omitted. Source-aware
assertion IDs preserve file/section/ordinal differences without inventing a graph
or joining conflicting manifests/locks. Third-party default adapters remain untyped.

Bounded JSON/TOML/YAML subset parsing replaces parse-to-empty fallbacks. Valid empty
inputs, malformed supported inputs, mixed usable records, unsupported versions and
limits have distinct outcomes. Ruby/Gradle/XML/requirements/Go lexical capture stays
NotReported. Other files become NotApplicable. Diagnostics/strict-mode values can
therefore change while their schema, flags and exit-code precedence remain intact.

Existing CLI options/default filenames, flat dependency SBOM field shapes, public
legacy struct literals and strict inventory/schema v1 remain available. No rule,
Risk/severity or recommendation logic was changed. Intentional purl/position/
metadata corrections can change custom purl matches and dependency finding IDs;
the legacy ID algorithm itself is unchanged. Source snippets/previews are masked
for all dependency/SBOM containers, including findings from other detectors there.
No raw locator is emitted by dependency capture. These are semantic corrections,
not a claim that previous erroneous values are byte-identical.

The [contract](../architecture/DEPENDENCY-PROVENANCE.md), both READMEs and inventory
identity documentation describe the supported matrix and projection losses. No
existing storage format/schema was replaced. Pinned TOML parse support adds five
locked MIT/Apache parser packages; no scanner runtime network/download was added.

## Verification and independent review

Pinned Rust 1.93.1, default workspace features, offline locked Cargo resolution:

- Debug/release workspace build, fmt check and clippy with warnings denied passed.
- **201 Rust tests, 20 Python tests and 22 Node tests passed.**
- Eleven new detector tables cover declarations vs resolutions, conflicting sources,
  same-file origins, ambiguous source positions, workspace/path/git/archive/aliases,
  15 ecosystem cases, pnpm peer/root/importer forms, malformed field types/duplicate
  keys, successful empty input, unsupported versions, binary input and byte/node/
  depth/YAML alias limits. Missing Maven namespace remains a safe reported SBOM
  label with unknown ecosystem/empty purl/Partial rather than a guessed identity.
- One actual CLI canary test checks all legacy/inventory/assessment/diagnostic
  outputs and stdout/stderr, including escaped registry URLs outside dependency
  records. Existing schema/golden, third-party API, seven-language, certificate,
  policy and CLI exit tests pass.
- Existing goldens change only corrected coverage/diagnostics and resulting
  assessment inspection reason. Aggregate policy remains indeterminate because
  inventory is not exhaustive. Strict-mode regression still uses genuinely
  unreported dynamic Ruby input to exercise code 3.

[Independent code review](TASK-027-REVIEW.md) approved the final implementation.
Review found dynamic versions falsely resolved, unclassified npm/pnpm locators,
pnpm peer/root record loss, misleading duplicate-name source lines and escaped
locator snippet leakage. Concrete-version validation, source classification,
peer/root parsing, unknown ambiguous lines and filewide masking corrected these.
The reviewer independently ran all eleven dependency tests plus CLI privacy and
diff checks. The [independent Evaluator](TASK-027-EVALUATION.md) verified final
source bindings, command outcomes and CI arithmetic, then marked task027 done.
Final documentation was separately reviewed; unsupported shrinkwrap
claims and overly broad ID-invariance wording were removed.

## Saved-binary comparison and CI expectations

Old default/inventory and current default/inventory/policy scan the exact same
current repository/rules, one thread: **856 findings each**. Of old/new findings,
808 are unchanged, 47 retained dependency findings have reviewed evidence/snippet/
position differences, one is added and one removed. Added: an existing Cargo
renamed dependency table now captures `ml-kem`; removed: SPDX's unrelated display
label no longer gains Maven identity from a raw external purl. Six Cargo.lock
locations move from block headers to package-name lines. Nondependency findings
are unchanged on this source scope. Current default/inventory/policy legacy
reports plus diagnostics agree after generation-time normalization; current
inventory/policy inventory payloads agree after run/time/observation reference
normalization. No old/new inventory equality is claimed for corrected provenance.

CI expectations rebind existing source positions/hashes for 164 entries across
three modified test-bearing files (only a subset changes result hashes). Six
existing Cargo fixture entries move to package-name lines; one extinct Maven
purl-only expectation is removed. No new omission or broad suppression is added.
Independent recomputation confirms **856 raw = 659 expected omissions + 197
retained**, with all 99 source hashes verified. New parser production/test evidence
and newly captured Cargo table findings remain visible.

## Frozen corpus and cost measurements

Task023 corpus/truth/labels remain unchanged: 41 files / 5746 bytes. Predictions
increase 36 to 37: the pre-existing held-out Cargo alias-table unit changes FN to
TP. Holdout dependency TP/FN changes 1/1 to 2/0; other family confusion counts are
unchanged. No relabeling or budget approval occurred. Coverage changes from
7 partial/34 unknown to 32 analyzed/8 partial/1 unknown; the malformed SBOM now
produces explicit detector failure while usable findings survive. Partial-file
aggregation can include one failed applicable detector alongside other stages.

Each old/current/policy profile ran three accuracy repetitions and first-measured
plus three warm resource samples at four fixed sizes. Semantic/diagnostic/score
repeatability passed. Warm wall-time medians (milliseconds):

| Input | Old default | Current default | Inventory + policy |
|---|---:|---:|---:|
| Small | 26.69 | 26.88 | 33.05 |
| Medium | 64.45 | 64.37 | 95.64 |
| Large | 133.32 | 146.04 | 276.81 |
| Pathological | 20.60 | 21.04 | 33.29 |

These local launch/report-write measurements include a 5 ms polling interval,
not OS cold-cache measurements or policy-only costs. The large default sample is
slower in this run; unapproved numeric budgets and this finite noisy sample do not
establish a general performance claim. Raw distributions include CPU/peak RSS.

[COMPARISON.json](task027/COMPARISON.json) records exact base/branch, source and
binary/schema digests, commands/exits, comparison deltas, source-bound CI audit and
corpus/resource evidence. Detailed local artifacts are under `target/task027/`:
`verification.json`, `self-*`, `final-{before,after,policy}/`, review and red/green
logs, registry rebind records, `code-scanning-audit.json`. Initial failing tests and
superseded measurement outputs remain explicitly separate from final runs.

## Remaining limits and unexecuted checks

Resolution is recorded lock evidence, not installation verification. No package
manager execution, cross-file resolution, graph, signatures/trust, deployment or
PQC-availability inference. Parser coverage is a documented subset, not universal
schema equivalence; private registries and namespace-less SBOM labels may remain
unresolved/partial. Best-effort line 0 and ordinal changes are documented. Existing
legacy flat projection still loses origin distinctions; native remains not exhaustive.
Filewide masking sacrifices source context; it is not a universal secret detector.

No remote CI, push, PR, merge, external-product acceptance, other platform build,
real secrets or browser/UI validation was performed. Human-report layout did not
change. Numeric quality/performance budgets still need maintainer review under
task023's existing decision gate.
