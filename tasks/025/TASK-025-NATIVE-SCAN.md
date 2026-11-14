# Task 025 native scan validation

Implemented locally on `codex/task-022-sarif-location`, starting from clean HEAD
`8ba863ff335023b3e3a33affa7cb250b3f7bec25` (task 024). The user selected the same
branch and additive task 025 scope. No push, PR, merge or remote CI execution was
performed. The implementation commit contains this report; benchmark environment
HEAD is the pre-commit base plus the recorded source bindings, not a remote result.

## Delivered contract

`--native-json` and `scan_repository_native` collect observations before legacy
normalization and add `scan.native.v1.json`. Optional explicit repository scope
supports weak source correlation. Existing Detector implementations and old scan
entrypoints compile unchanged and execute once per inspected file. Default
filenames, CLI behavior, legacy IDs, report structures and detection/rule semantics
remain intact. No dependency/lockfile/rule change or additional crate was required.

The [native contract](../../docs/architecture/NATIVE-SCAN-V1.md) and
[repository-owned Draft 7 schema](../../schemas/native-scan-v1.schema.json)
describe algorithm/public-key/certificate/usage separation, safe evidence,
run-versus-occurrence identity, multi-method links and the structured legacy loss
ledger. READMEs document both flags. The schema is original MIT project material,
not a copied external schema or a CycloneDX claim.

## Correctness evidence

Pinned Rust 1.93.1, workspace default features, offline Cargo resolution. Executed
commands, exit codes and logs are in `target/task025/verification.json`; exact
source/corpus/binary digests and measured results are summarized in
[COMPARISON.json](evidence/task025/COMPARISON.json). Full local logs/artifacts remain under
`target/task025/` and are intentionally not committed.

- `cargo build --offline --workspace` and release build; `cargo fmt --all --check`;
  `cargo test --offline --workspace`;
  `cargo clippy --offline --workspace --all-targets -- -D warnings`.
- All 176 Rust tests, 20 Python quality-harness tests and 22 Node report/self-scan tests passed.
- Actual CLI opt-in/default artifact assertions, local native schema validation,
  semantic golden and tolerant-reader/unknown-vocabulary tests. Reference integrity
  validates unique native IDs, asset/observation links and final legacy indices.
- Length-framed tuple/domain/empty/Unicode identity tests. A deliberate removal of
  per-field lengths failed the tuple test; the production implementation was
  restored before final verification. Initial red tests also reproduced missing
  CLI flags, forged targets, hidden partial containers and opaque-parameter IDs.
- Checkout-root/line insertion/move/scope/duplicates, run-local identity without
  scope, same-target different-rule asset ordering across eight runs, and
  multi-method observation retention. A real legacy hash collision preserves two
  final indices; a legacy delimiter-dedup collision has a counted loss reason.
- Public DER/SPKI sharing, RSA certificate signature versus EC subject key,
  parse-only certificates without certificate rules, and retained good objects
  plus separate trailing DER/malformed neighboring PEM container limitations.
- Synthetic private payloads for RSA/EC/DSA/encrypted/PKCS#8/OpenSSH markers do not
  survive in native JSON, six legacy reports, stdout or stderr. This exercises
  masking/capture paths, not successful parsing of all private key formats.
  Unsafe package paths/credentials and ordinary prerelease versions are tested
  together; rejected identities get a fixed omission status before native IDs.

Initial review requested six material fixes: uninterpreted parameter bytes as
algorithm identity, lost container limitations, unverified source attribution,
missing projection-loss reasons, run-dependent usage ordering and permissive
package path grammar. All were reproduced or tested, corrected and independently
approved. [The review record](TASK-025-REVIEW.md) distinguishes design/source review,
independently executed tests and coordinator validation. No blocking review
finding remains in the accepted scope.

## Compatibility and CI registry

Same current source bytes, root, rules and one thread were scanned with the saved
task 024 release binary, new default binary and new native-enabled binary. Six
legacy artifacts plus diagnostics are equal after removing only generation time.
Native-enabled capture does not alter legacy coverage or findings. The English/
Japanese exit matrix also agrees: default 0, `--fail-on high` 2,
`--require-complete` 3, and their combination 3. Details are in
`legacy-shadow.json` and `exit-matrix.json` under `target/task025/`.

The CI expectations registry was rebound only for 118 existing result entries in
seven modified files, all inside existing `cfg(test)` regions. The source line,
rule, column, multiplicity, path and reason were independently verified unchanged;
only shifted line/source/result hashes changed. No new omission was registered.
All 99 source bindings verify. Raw self-scan has 814 results: 660 expected fixture
results omitted, 154 retained. Task 024 historically had 793/660/133; the additional
21 results are present even using the saved old binary against the current source.
New production/native modules and new tests remain visible. Raw evidence is kept.
The policy digest uses canonical JSON, while the raw SARIF digest binds file bytes.

## Measurements and limitations

The frozen task 023 corpus/truth/labels were not changed. Each old/default/native
run uses three accuracy repeats and first-measured plus three warm samples for
small/medium/large/pathological inputs. Per-family confusion counts and sorted
prediction dispositions agree; raw report order can differ across temporary roots
because legacy finding IDs are root-sensitive. Each run's semantic/diagnostic/
score repeatability passes. Accuracy remains 36 raw predictions on 41 files,
with 7 partial and 34 unknown file dispositions; no new completeness claim is made.

Warm wall-time medians (ms, three samples; full ranges/RSS/CPU in COMPARISON.json):

| Input | Saved task 024 | New default | Native enabled |
|---|---:|---:|---:|
| small | 31.61 | 24.02 | 31.63 |
| medium | 61.84 | 61.67 | 84.42 |
| large | 137.06 | 137.24 | 257.08 |
| pathological | 24.30 | 24.32 | 31.83 |

Native collection has a measurable opt-in cost (large input approximately 1.87x
the default wall-time median here); it does not run on the default path.

Performance distributions and native overhead are descriptive local measurements,
not approved budgets. The native run uses a recorded shell wrapper that adds only
native/scope flags; its own digest and underlying release-binary digest are kept
separately. The timer includes process launch/report writes, has 5 ms child polling,
and does not measure an OS cold cache. Small timing differences are not proof of
an improvement or regression. Task 023's maintainer budget decision remains open.

Native completeness is explicitly not exhaustive. Unknown names are not guessed;
AST evidence remains syntactic, references do not establish active use, and
certificate parsing does not verify trust/signatures. Dependency declared/resolved
version provenance is unavailable pending task 027. Weak occurrence anchors and
duplicate ordinals can shift and are not cross-scan proof. Paths and package labels
are potentially sensitive metadata; the scanner is not a universal secret detector.

No external repositories, network peers, real private keys, platform matrix,
browser acceptance or production deployment were used. No UI was changed. Remote
CI, upload and publication remain unexecuted. During evidence reruns, an existing
benchmark output directory was correctly refused; final measurements use new
`accepted-baseline-*` directories. Earlier failed regression/mutation logs are
retained separately from final successful verification.
