# Task 024 AST occurrence fidelity — implementation and evidence

Date: 2026-11-10 (Asia/Tokyo). Base/validation working HEAD:
`757d067bff36f6e0538c1482ae4123611a9aab81`. Branch:
`codex/task-022-sarif-location`, explicitly retained at user request. Initial
checkout clean. One task-specific local commit; no push/PR/merge. The task023
technical baseline is reused under explicit authorization to continue task024;
its pending maintainer protocol/numeric-budget approval is not silently granted,
and its task state/labels/corpus/record remain unchanged.

## Problem and bounded design

The AST detector formerly deduplicated by rule/file/line/normalized match text.
Thus identical tokens or calls at distinct columns on one line collapsed. The
frozen task023 twice.js fixture returned only column 36; its column 1 call was
lost. Five new detector tests failed against the old implementation, as did the
actual fixture regression. The failures cover all seven language scopes, nested
quoted candidates, case/Unicode prefixes, same-line 32/33 unique-hit counting and
a synthetic clipping ellipsis incorrectly becoming a source match.

Private CandidateNode now carries node.start_byte and the real source-prefix
length before a display ellipsis. Each rule deduplicates absolute parsed-source
byte intervals, trimming only edge whitespace and quote/backtick wrappers so
parent/child views of the same lexical occurrence share one key. Different spans
survive even with identical normalized text. No new source-span storage field is
exported. The rule-local set holds at most the existing 32 accepted occurrences;
examined-match 4096, candidate 4096 and scheduled-node 100000 limits/traversal stay
unchanged. Matches crossing the display-only suffix are rejected; clipping still
reports Truncated. Later rules continue after earlier-rule caps as before.

Alternatives rejected: a column-only line key depends on the existing mixed-unit
coordinate calculation; a detector rewrite or new TS grammar would expand scope
and dependencies without measured benefit for this occurrence correction. No
Cargo/grammar/rule/comment-policy/public model/ID algorithm/reporter change occurs.
Existing location/evidence selection order is preserved, including legacy AST
mixed byte-start/code-point-relative columns. Task022 SARIF omission of unproven
columns remains in effect. Source byte provenance is private identity, not an
assertion that exported legacy columns are Unicode-correct. New occurrences get
new existing-algorithm IDs; all previously retained findings must remain intact.

Tree-sitter pinned 0.26.6 source/API defines node byte offsets; see
[the pinned API](https://docs.rs/tree-sitter/0.26.6/tree_sitter/struct.Node.html)
and [basic parsing](https://tree-sitter.github.io/tree-sitter/using-parsers/2-basic-parsing.html).
No runtime network request or new dependency is introduced.

## Tests and grammar/API characterization

- Seven-language same-line repeated-token cases retain exactly two occurrences.
- Parent/child matches, quoted/bare alternatives, template strings and nested calls
  do not duplicate one span. Distinct case/spans and Unicode prefixes remain two;
  multiline cases retain their correct native lines without changing column units.
- 32 unique same-line hits Complete; the 33rd proves Truncated. Existing later-rule,
  work/node/candidate-budget and malformed-parser cases remain green.
- A long string with no real ellipsis yields zero ellipsis hits and Truncated.
- Plain JS and untyped TS Complete, typed TS is Partial with recoverable token
  output, and the JSX-only TSX subset can Complete using the existing JS grammar.
  This is not a claim of full TypeScript/TSX support.
- Standalone comment controls across seven language scopes yield one actual code
  occurrence. comment_filter.rs is unchanged, including legacy Java regex policy.
  Inline/block comments inside ancestor text are not full semantic comment analysis.
- Actual CLI E2E retains two quote-inclusive native columns (6,15), distinct IDs,
  repeated-run identical findings, the six legacy report filenames, one SARIF rule
  and two schema-valid SARIF results against the pinned official offline schema.

Frozen task023 alias/local-function/plain-string/typed-TS units provide measured
limits without rewriting truth or matcher semantics: imported aliases can still
be missed; local JS/Rust namesakes and Ruby display strings remain false positives;
typed TS and malformed Python remain missed API references. No whole-program
resolver/dataflow, import attribution or TS grammar is added. The new holdout is
used for unchanged-regression evaluation only, not output-driven tuning.

## Same-corpus before/after measurements

[Machine-readable comparison](evidence/task024/COMPARISON.json) records binary SHA-256,
complete source/harness/rule/catalog bindings, corpus/truth digests, input bytes,
exact roots/commands, environment, repeats and raw resource samples. Both sides
use identical 41 files / 57 truth units / 5746 input bytes (corpus023-v2); materialized
input digest, split/labels/config/rules are identical. Each has three accuracy
runs on one root, one thread, --format all --diagnostics, default 2 MiB cap and
default exclusion/gitignore policy. All 41 files are actually read. Each side's
semantic/diagnostic/score digests and raw order are stable over repeats.

Counts below are TP/FP/FN/TN; null denominators in COMPARISON.json stay null.
They are descriptive finite synthetic-corpus observations, not global accuracy.

| Split/family | Before TP/FP/FN/TN | After TP/FP/FN/TN | Eligible units | Ambiguous/unsupported |
|---|---|---|---:|---:|
| holdout/source_api | 2/0/1/2 | 2/0/1/2 | 5 | 1/0 |
| holdout/draft | 0/0/0/0 | 0/0/0/0 | 0 | 0/0 |
| holdout/config | 0/0/1/1 | 0/0/1/1 | 2 | 0/0 |
| holdout/dependency | 1/0/1/2 | 1/0/1/2 | 4 | 0/1 |
| holdout/certificate | 0/0/0/0 | 0/0/0/0 | 0 | 0/2 |
| holdout/key | 0/0/0/0 | 0/0/0/0 | 0 | 0/1 |
| tuning/source_api | 9/3/5/3 | 10/3/4/3 | 20 | 1/1 |
| tuning/draft | 1/0/0/2 | 1/0/0/2 | 3 | 0/0 |
| tuning/config | 3/0/0/3 | 3/0/0/3 | 6 | 0/0 |
| tuning/dependency | 1/0/0/1 | 1/0/0/1 | 2 | 0/0 |
| tuning/certificate | 4/0/0/2 | 4/0/0/2 | 6 | 0/0 |
| tuning/key | 1/0/0/1 | 1/0/0/1 | 2 | 0/0 |

Only tuning/source_api changes: TP9→10, FN5→4, FP3 stays 3. Its precision is
10/13 and recall 10/14; the single repaired truth unit is u016, not a newly relabeled
success. All holdout cells are unchanged, including misses, unsupported controls
and undefined certificate/key/draft denominators. Full corpus raw findings are
35→36; out-of-scope findings remain 10. The prior shortcomings stay visible.
Coverage remains selected/read-ok41, partial7 + unknown34, overall Partial;
dependency NotReported and unenumerated populations prevent a completeness claim.
Synthetic private-body canaries are absent from all reports/stdout/stderr on
both sides. No real application, key, endpoint or crypto operation is exercised.

## Repeated performance comparison

Same Apple M4 Pro / 12 logical CPUs / 48 GiB RAM / aarch64-apple-darwin, Rust1.93.1
release/default features, Python3.12.7. Each workload has one first-measured plus
three warm samples. First measured is not flushed OS cold cache. Wall includes
launch/report writing, CPU/peak RSS are wait4 per child (macOS bytes, Linux KiB
conversion); build time is excluded. Five-ms polling affects these short times.
Small/medium/large are 1/8/32 copies of the same small synthetic corpus; duplicates
are performance load, never independent accuracy truth. The largest ordinary
workload is only 184 KB, not an enterprise-project performance acceptance.

| Workload | Files / bytes | Warm wall median seconds before / after | Warm CPU median seconds before / after | Warm RSS median MiB before / after | First-run findings before / after |
|---|---:|---|---|---|---|
| small | 41 / 5746 | 0.026784 / 0.023496 | 0.017651 / 0.017624 | 16.27 / 16.89 | 35 / 36 |
| medium | 328 / 45968 | 0.066659 / 0.059936 | 0.044255 / 0.042282 | 19.38 / 19.08 | 280 / 288 |
| large | 1312 / 183872 | 0.139754 / 0.155821 | 0.116802 / 0.132806 | 31.52 / 32.28 | 1120 / 1152 |
| pathological | 1 / 60017 | 0.020811 / 0.022966 | 0.015646 / 0.015936 | 13.89 / 15.00 | 66 / 128 |

Full min/median/max distributions and samples are in COMPARISON.json. Output
counts increase under repeated inputs as intended. The 10000-token pathological
input remains explicitly detector_truncated, with more distinct retained AST
occurrences under the same cap. Differences in this small, three-warm-sample run
are not a statistical speedup/regression verdict. Task023's numeric proposals
remain unapproved; no threshold is adopted to declare success. No true cold-cache,
Linux/Windows, multi-host, optional-feature or real-product benchmark was executed.

## Compatibility and CI projection

Actual old/new CLIs scanned identical roots/bytes/config. An unaffected 12-file
input (existing examples, public DER, synthetic marker, Unicode single use) has
18 findings on both sides. Native JSON matches excluding generated_at; SARIF,
CBOM and dependency-SBOM match bytes; HTML/Markdown match after removing only the
exact generation timestamp. A 42-file occurrence input (frozen corpus + Unicode
repeated-token case) has 41→44 findings. Every existing full native finding/ID,
SARIF result, CBOM record and dependency inventory remains; three added observations
are AST occurrences in twice.js and unicode.js. CLI flags/report filenames/schema
shapes/severity/rules/recommendation semantics remain unchanged. Generated report
counts and human occurrence details intentionally reflect the new observations.
Detailed digest/root/delta evidence: target/task024/compatibility/summary.json.

Repository self-scan compares old/new binaries on the SAME current repository
bytes: 776→793 raw, no old result removed, 17 additional AST observations. Fifteen
are intentional existing fixture occurrences: six added exact-position groups in
comment_filter.rs, one new twice.js position, and six existing groups with reviewed
counts 1→2/3 in regex_detector.rs/bounded_outcomes.rs. The appended readiness test
changes only its four prior source-bound approvals' whole-file hash; original
result digests/positions/counts stay unchanged. No blanket registration occurs.

Two extra observations in production recommendation.rs:720 and japanese.rs:89
remain retained, even though they are instruction/catalog strings. The projection
helper, rule severity and unknown-location policy are not relaxed. Final partition:
**793 raw = 660 reviewed omissions + 133 retained**, 99 verified source files.
All previous 131 retained records remain; the three unknown-line SBOM observations
from task023 still remain. Raw bytes, result objects and run metadata are preserved
by projection. Hosted Code Scanning acceptance is unexecuted; these observations
remain eligible for hosted checks.

## Validation and limits

Pinned-toolchain commands all exited 0: cargo fmt --all; cargo build --workspace
--locked --offline; cargo fmt --all -- --check; cargo test --workspace --locked
--offline (154 Rust tests); cargo clippy --workspace --all-targets --locked
--offline -- -D warnings; Python20 sanity tests; Node22 tests; git diff --check.
Official offline SARIF schema and canary checks are exercised by actual CLI tests
and existing report contracts. Unrelated runtime/core/model/report/rule/CLI,
Cargo dependencies, task023 corpus/harness and workflow/helper paths are byte-
unchanged, checked explicitly. Both READMEs and architecture document changed
behavior/remaining limits. Ignored artifacts live under target/task024/.

Independent review inspected the source span implementation, focused 29 tests,
measurement/compatibility/registry evidence and final documentation; its record
is [TASK-024-REVIEW.md](TASK-024-REVIEW.md). Full coordinator verification is distinct
from independently executed focused tests. No hosted Actions, platform matrix,
browser UI acceptance (renderer unchanged) or real-product accuracy claim.

Residual limits: LIFO traversal/budgets do not guarantee source-order completeness;
legacy native mixed-unit Unicode coordinates and nominal API/comment attribution
remain; clipping still yields partial inspection; TypeScript types use recovery.
The correction does not claim to solve aliasing, local-shadow false positives,
configuration formats, dependency identity or grammar support beyond tested subsets.
