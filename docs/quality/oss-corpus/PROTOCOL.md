# Pinned OSS reference study

## Purpose and acceptance

Extend the existing OSS benchmark with the 18 user-nominated repositories across
Java, Go, Ruby, Python, JavaScript/TypeScript and Rust. Retain Keycloak, Airflow,
Kubernetes, Node.js, n8n, Discourse, Rails and Vector as existing targets. Record
source URLs, immutable commits, suitability, cryptographic surfaces, scanner
results, elapsed time, per-child peak RSS, failures and representative source
observations. Produce language/repository comparisons and rule priorities.

This is a development research dataset, not a product output format or a scored
accuracy corpus. Product source, rules, tests, scripts, dependencies, CLI output
contracts and previous evidence remain unchanged. No upstream application,
dependency installation, build script or test is executed. Only this repository's
existing scanner is built and run.

## Reuse and sequence

The existing `shell/run-benchmark.sh` defines the product population;
`examples/scripts/scan_oss_products.sh` demonstrates the scan invocation. Both
remain compatible and unchanged. This study freezes commits rather than pulling
mutable branches. The quality runner's existing `measure`, `environment` and
`distributions` helpers in `tests/quality/run.py` provide child resource accounting
and source/binary/toolchain bindings. They are imported from a temporary study
driver; no replacement measurement/scoring framework is introduced.

1. Fetch `origin/develop`, create an isolated branch, and build the unchanged CLI
   with the pinned Rust toolchain and locked dependencies.
2. Acquire one nominated repository per language and scan this six-repository
   pilot sequentially. Validate exit codes, reports, resource receipts and
   diagnostics before expanding.
3. Freeze and acquire the other 12 nominated repositories; materialize the eight
   existing repositories from their recorded local Git commits without including
   user modifications. Scan the remaining targets sequentially with identical
   options. Preserve acquisition failures separately from scanner errors.
4. Inspect representative upstream source and correlate locations with findings.
   Retain verified reference facts, context, file hashes, exact source links,
   observed rule matches and unresolved interpretation. Compare all targets and
   prioritize gaps without changing rules.
5. Validate artifact integrity, source bindings, population preservation,
   diagnostics accounting and report/table consistency against final artifacts.

## Measurement contract

Use release/default features, explicit checked-in rules, one scanner thread,
JSON output, diagnostics and default exclusions/size limits. Output is outside
the input tree. JSON-only output is an intentional difference from the historical
all-format benchmark; old timings/counts are not comparable. One first-observed
sample per full repository characterizes this run; it is neither a cold-cache
measurement nor a performance distribution. Pilot repeats separately check
measurement and semantic repeatability. Do not benchmark scans concurrently.
Acquisition and compilation are excluded; launch through exit and JSON/diagnostic
writing are included. `wait4` supplies per-child RSS (native bytes on macOS) and
CPU time. Do not infer total process-tree memory from child peak RSS.

Enumerate Git trees with NUL-delimited paths so quoted/non-ASCII filenames are
not lost. Git archive honors `export-ignore`; compare its materialized population
to the pinned tree and restore omitted tracked regular files from exact Git blob
bytes before the final scan. Preserve initial bindings/scans when this changes
inputs, and rerun affected scans. Record OS-blocked or subsequently absent files
as unavailable with their Git blob identity; do not silently omit them or change
host protection settings.

Clones can apply tracked line-ending attributes (for example CRLF batch files).
SHA-256 bindings always identify the actual measured bytes. A separate Git blob
parity check records verified CRLF-to-LF checkout transformations; it does not
silently replace measured bytes or rewrite scanner artifacts.
The standard-library archive data filter can strip the trailing slash from a
directory symlink target. Record this separately after verifying the pinned Git
blob differs only by that slash and resolves to the same present directory.
No symlink is followed as a scan input.
Tracked `export-subst` expansion is also recorded separately and verified against
fresh Git archive bytes at the exact commit. This covers archive version metadata
without claiming that expanded working bytes equal the unexpanded Git blob.

Retain original generated JSON/diagnostics as losslessly compressed artifacts,
with raw and compressed SHA-256 bindings, command/exit/resource receipts, stdout
and stderr. Input manifests bind Git commit/tree, tracked files, submodules and
source status. Git submodules are not initialized; missing submodule content,
ignored files, binary/native source, scanner limits and diagnostics remain
explicit limitations. A successful exit does not establish complete inspection.

## Interpretation and representative observations

Separate **implementation**, **API use**, **configuration**, **dependency**, and
**test/example** contexts. An algorithm implementation, declaration, supported
algorithm string, dependency or test does not establish production execution,
enabled backend, negotiated protection or quantum vulnerability. Source references
to ML-KEM/ML-DSA/hybrid algorithms are not mathematical correctness evidence.
X.509 object facts and source API references are distinct observations.

Representative observations are targeted, source-checked, single-analyst research
data. Record selection/output exposure and uncertainty; they are not independently
adjudicated labels or a random/held-out population. Hash-bound path/line/fact
records accumulate reusable evidence without copying upstream private key bodies
or incorporating foreign source into the existing scored corpora. Record license
file paths and upstream attribution. Suspicious unmatched references are coverage
gaps; call them product false negatives only when the documented supported
contract and the specific source fact establish that conclusion. Likewise,
implementation/test hits are not automatically false positives under a nominal
reference contract. Unreviewed findings remain unadjudicated.

Do not calculate Precision, Recall, F1 or an accuracy ranking. Detection counts
and text-search counts are descriptive only. No rule tuning, new numeric gates,
task creation or product changes are authorized by this study.
