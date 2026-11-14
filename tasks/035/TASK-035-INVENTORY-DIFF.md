# Task 035 implementation evidence

Date: 2026-11-10. Status: **Executor review handoff**, not Evaluator completion.
Baseline/current committed HEAD during these checks:
`3c357cef879f69ae4058fefe7a03b4f074283b3f`. Changes were checked in the shared,
uncommitted continuation worktree; unrelated 030/034/036 implementation was also
in progress. This is not evidence of hosted CI or final committed-tree validation.

## Implemented contract

The maintainer approved the explicit scope/new-format contract in the
[continuation decisions](../../docs/records/v0.2-roadmap/CONTINUATION-DECISIONS.md).
`core::diff::parse_inventory` bounds input to 16 MiB, accepts inventory v1 with
tolerant extra fields, validates reference integrity and comparison identifiers,
and rejects duplicate occurrence keys or unsafe target paths with fixed errors.
The comparison record budget is 100,000 assets plus observations per input.

`compare(&NativeScan, &NativeScan, scope)` requires the same explicit repository
scope and supported identity scheme. It reports exact asset/weak-occurrence
correspondence, changes, additions, unresolved items and metadata drift. Unknown
identity confidence and duplicate anchors are unresolved. Run-local observations
remain individually visible. Algorithm/public-object/source-usage identities
remain distinct. Physical line/column/byte positions and run/time identifiers do
not turn an otherwise unchanged occurrence into a changed semantic record.

Inventory v1 cannot prove target-level exhaustive native inspection. Therefore
every absent-after record remains **unresolved**, including when legacy coverage
says `complete_within_scope`. The `removed` vocabulary is reserved by the new
output schema, but no current v1 input can justify emitting it. `added` means newly
present in the compared record set, not a newly deployed cryptographic asset.
No output means a risk was remediated or PQC migration completed.

Move candidates require a unique semantic anchor/content signature across **all**
observations on each side, a changed path, and no existing exact occurrence match.
Duplicates and a same-anchor observation elsewhere force abstention. Candidates
remain `heuristic_only`; their original added/unresolved records are retained.

The JSON output contains IDs, fixed status/reason codes and the explicit scope;
it does not copy imported arbitrary asset attributes, evidence text or paths.
The Markdown summary uses fixed explanatory copy and counts. Input paths and
scope still have the inventory's operator-sensitive metadata limits; no universal
secret detection is claimed. Unknown extra fields are deliberately not compared,
and this conversion loss is present in every output's limitations.

## Executed checks

Toolchain: Rust 1.93.1, host `aarch64-apple-darwin`, LLVM 21.1.8.
Raw logs are under ignored `target/task-035/`.

| Command/check | Outcome | Log |
|---|---|---|
| Initial `cargo test -p pqc_scan_core --test inventory_diff` | Expected missing-module/API compile failure; concurrent target-policy module was also initially absent | `red.log` |
| Global-anchor uniqueness regression before correction | **13 passed, 1 failed**: a matched same-anchor file incorrectly permitted a move candidate | `move-red.log` |
| Focused comparison/golden suite after correction | **14 passed**, exit 0 | `golden.log` |
| `cargo test -p pqc_scan_core` | **95 passed**, exit 0 at that worktree state | `core-tests.log` |
| `cargo clippy -p pqc_scan_core --lib --test inventory_diff -- -D warnings` | Exit 0 | `clippy.log` |
| `cargo test -p pqc_scan_cli --test inventory_diff` | **1 passed**, exit 0; offline schema validates actual core output and semantic golden, rejects invalid status/missing limitations | `schema.log` |
| `cargo clippy -p pqc_scan_cli --test inventory_diff -- -D warnings` | Exit 0 | `cli-clippy.log` |
| `rustfmt --edition 2021` on owned Rust files | Exit 0 | Applied only to owned files |

One transient compile failure exposed a duplicate `observations()` getter added
by another continuation edit. The owning parent corrected it; 035 did not remove
or replace unrelated work. The first schema run reported a transient unused
variable warning in the concurrently edited target-policy module; subsequent
CLI clippy with `-D warnings` passed. Python `jsonschema` is unavailable, so the
existing pinned Rust development validator (`jsonschema=0.29.1`, default features
disabled) performs schema validation without downloads or external references.

## Source and fixture bindings

SHA-256 values at the successful focused/schema checks:

| Path | SHA-256 |
|---|---|
| `crates/pqc_scan_core/src/diff.rs` | `345abbaff02fbb1fa2be65dbe0cde9d1542a5be7c33c7df415b1db8e9169eac4` |
| `crates/pqc_scan_core/tests/inventory_diff.rs` | `e5b767310c50071f1c05419ad002866c3b1fd0139d3498142ea3ce437c8e4165` |
| `crates/pqc_scan_cli/tests/inventory_diff.rs` | `0e869bb703a912cb62dbf2551d1c6f3ac00d620cc6e1414587124826395e4922` |
| `crates/pqc_scan_core/tests/fixtures/diff/unchanged.v1.json` | `e800113b4519108d78f5abebd37d057213eeadc6d3ce59e08b1c617c34b79e48` |
| `schemas/scan-diff-v1.schema.json` | `2af1455168c0611220edc29afb9d9ee5948e1181d98c434b549a8af5574ceb6d` |

Fixtures and the diff schema are synthetic repository-authored MIT material.
No external repository or live endpoint was scanned by these tests.

## Parent integration and unexecuted acceptance

Parent subsequently delegated `src/diff_command.rs` and the dedicated CLI test
to the 035 Executor. The adapter now reads at most `MAX_INPUT_BYTES + 1` bytes,
validates both inputs before creating output, and writes only separate
`scan.diff.v1.json`/`scan.diff.v1.md`. Parent retains the main-command hook,
Cargo dependency integration, both README updates and user guides. Preserve all
legacy outputs. The core module's export was added with parent authorization.

Adapter verification: `adapter-red.log` records the valid-input test failing on
the unimplemented adapter; `adapter-green.log` records 4 adapter cases plus the
offline schema test passing. `adapter-final.log` rechecks the four adapter cases
after strengthening the missing-input assertion so output-directory failure cannot
mask a missed input-read error. The binary scan → inventory → diff acceptance
test first failed on the missing `diff` subcommand (`binary-red.log`), as expected
before parent main-command integration.

An additional completeness-drift regression failed with a matching record set
but an undisclosed changed coverage state (`coverage-red.log`). The comparison
and schema now include `inspection_coverage_changed` and
`native_completeness_changed`; `coverage-green.log` records all **15** core
comparison tests passing. These outcomes supersede the earlier 14-test count.

After parent main-command integration, the first binary suite had **5 pass /
1 fail** (`target/roadmap-continuation/035-cli-integration.log`, parent-run exit
101). The test incorrectly assumed clap's conventional exit 2 for missing scope;
this CLI already maps argument/setup errors to operational exit 1. The assertion
was corrected to preserve the existing CLI contract; no product exit behavior
was changed. `binary-green.log` records **6 passed, exit 0**, including actual
binary scan → inventory → diff, line-insertion correspondence, output schema and
scope requirements. `adapter-clippy.log` records the integrated dedicated CLI
target passing `cargo clippy -p pqc_scan_cli --test inventory_diff -- -D warnings`.

The latest owned source bindings after adapter integration and the coverage-drift
fix supersede the corresponding earlier bindings above. Committed HEAD was still
`3c357cef879f69ae4058fefe7a03b4f074283b3f`; integration was checked in the shared
uncommitted worktree.

| Path | SHA-256 |
|---|---|
| `crates/pqc_scan_core/src/diff.rs` | `6e3eb2cc1c09976b5808b4a870ba17b9c171f8a5a1cd147f7c1c452f748a3200` |
| `crates/pqc_scan_core/tests/inventory_diff.rs` | `924834312fdaa0f2d62c3b7dd509a71f4cc43230995c9b75b9a63e726b3136e4` |
| `crates/pqc_scan_cli/src/diff_command.rs` | `74770abe36de4ba86b150d213163be4788aa23b744ead25a7ae16d1faebcf380` |
| `crates/pqc_scan_cli/tests/inventory_diff.rs` | `459bf7eb5de1a1144771d9ddc1d866fb782f230a94c41b6b47c1ec7daf347f01` |
| `schemas/scan-diff-v1.schema.json` | `c96fffc3535477ed81f09bd738482f5c3d0acfe14031710497477fe09380b16a` |

Independent review, final-worktree build/fmt/workspace
tests/clippy, self-scan source rebinding and final local task commit remain parent
responsibilities. No independent review or final completion is asserted here.

## Independent review correction round 1

The independent Reviewer found three correctness gaps in the initial comparison:

1. Filtering ambiguous observations before move multiplicity counting could make
   one remaining anchor appear unique despite two ambiguous duplicates elsewhere.
2. Equal referenced asset IDs alone could suggest a move while the referenced
   descriptor changed, or contained conflicting/unknown identity evidence.
3. After-only assets bypassed correspondence checks and were always `added`, even
   when the corresponding usage observation was ambiguous or the asset identity
   was unknown.

`review-red.log` reproduces all three: **15 passed / 3 failed**, exit 101.
The corrections count every observation carrying a usage anchor before applying
confidence filters. A move now requires exactly one anchor on each side and
unchanged, correspondence-eligible referenced asset descriptors. Descriptor
comparison is performed once per shared asset rather than copying large shared
attributes into every move signature. Ambiguous/unknown referenced assets cannot
qualify. After-only assets use the same supported kind/identity-basis and
ambiguity checks as existing assets; uncertainty stays unresolved.

Verified after the correction, still at committed HEAD
`3c357cef879f69ae4058fefe7a03b4f074283b3f` plus the shared continuation worktree:

| Check | Outcome | Log under `target/task-035/` |
|---|---|---|
| Core comparison suite | **18 passed**, exit 0 | `review-green.log` |
| `cargo test -p pqc_scan_core` | **101 passed**, exit 0 at that shared worktree state | `review-core-suite.log` |
| Dedicated CLI/adapter/schema/binary suite | **6 passed**, exit 0 | `review-cli-green.log` |
| Core dedicated `clippy -- -D warnings` | Exit 0 | `review-clippy-final.log` |
| Owned-source rustfmt and whitespace check | Exit 0 | Executed after corrections |

Latest changed-source SHA-256 bindings, superseding the corresponding earlier rows:

- `crates/pqc_scan_core/src/diff.rs`:
  `9bccb6185e5d185239336f50b1a4eba141f8b633a3c12206741139f9b2f6b07d`
- `crates/pqc_scan_core/tests/inventory_diff.rs`:
  `ae0438a28f6d1f4720f7ea7d2a2a7d2e424c871854a09405c5efe0cf903c739c`

The new format/schema and other source bindings are unchanged by this correction.
Independent re-review and final workspace acceptance remain pending; this record
does not claim that the initial review approved the implementation.
# Parent integration acceptance

Independent spec and quality re-review accepted all three correspondence fixes.
The same external reproduction now yields zero move candidates for both ambiguous
duplicate anchors and changed referenced-algorithm facts; ambiguous after-only
assets remain unresolved. Eighteen comparison tests and six CLI/schema/adapter
tests passed. English/Japanese user guides and both READMEs explain explicit scope,
unresolved absence and heuristic-only moves. Evaluator accepted task035 with the
[integration checks](../../docs/records/roadmap-continuation/ROADMAP-CONTINUATION-VALIDATION.md). No completeness or
remediation claim is inferred from disappearance in inventory v1.
