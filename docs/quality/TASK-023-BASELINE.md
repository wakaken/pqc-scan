# Task 023 measured quality baseline

Date: 2026-11-10 (Asia/Tokyo). Base/validation HEAD:
`35d988e03629cb15b17a94af81b1e74266a344ff` (task022 local commit).
Branch: `codex/task-022-sarif-location`, retained at the user's explicit request.
This is one task-specific local commit on the existing branch; no push/PR/merge.

Implementation and local validation are complete; numeric/protocol budget approval
remains a maintainer decision. Task status therefore remains **review**, not done.
The [machine-readable measured record](task023/BASELINE.json) binds the exact
uncommitted harness/source/corpus/label/rule/catalog/binary bytes tested, commands,
raw samples and environment. Final commit identity is assigned after review.

## Scope and truth protocol

The [protocol](../../tests/quality/PROTOCOL.md) and [developer instructions](../../tests/quality/README.md)
define nominal source API/import references (not runtime reachability), draft
names, configuration assignments, declared dependencies, public certificate
attributes and synthetic private markers. The corpus has 41 files, 57 units,
30 positive / 20 negative / 2 ambiguous / 5 unsupported labels. It covers seven
languages through reused existing readiness fixtures and derived controls, plus
config, manifest/SBOM, public DER, malformed and unsupported inputs. Code is data,
never executed; no private repositories/endpoints/keys or new runtime dependency.
All source is repository MIT material, reused public certificate provenance is
in its existing README/OpenSSL manifest, and new project files have MIT headers.

Two source-only independent reviewers label exact file/unit bytes; labels/truth
bind full SHA-256. Final reviewers A and fresh C agreed on all 10 newly authored
holdout units. They differed only on pre-existing u027 (incomplete Python call):
positive nominal-reference truth was adjudicated before revised measurement
because the named API member is visible; parser incompleteness is a separate
fact. Both judgments/reasons remain preserved. No labels derive from scanner hits.

### Lineage correction, not hidden tuning

An independent source-only lineage audit found the initial adversarial holdout
contained copied/wrapped readiness calls. The initial 023-v1 proposal was invalid
as an independent holdout. [Exploratory counts](task023/INITIAL-EXPLORATORY.json),
corpus-initial.json and original A/B labels are retained, not rewritten as a
successful holdout. All readiness-derived variants moved together into tuning.
A separate author, with no scanner/rules/metrics/other-label access, then authored
six approval-adapter files from a new project premise: multiline imports/calls,
callback rotation, XML properties and renamed dependency tables. A and fresh C
labeled the current manifest before the new holdout's first run. No rule/detector
was tuned. The old FP/FN observations remain visible below on the tuning split.

This is a public synthetic developer set, not a secret benchmark or proof of
population independence. Common standard identifiers are unavoidable shared
vocabulary; source-only audit and recorded authorship are the evidence for
lineage. Reusing the new holdout to tune a future matcher requires a fresh set.

## Counts and denominators

Scoring is one-to-one by file/rule/line/half-open column span; duplicates are FP,
missing same-line occurrences are FN. Unknown dependency lines match only exact
name metadata on the declared unit, with no invented line. Ambiguous/unsupported
units and their predictions are outside denominators, reported separately.
Other rule families are explicitly outside this study: **10 findings** retained
as out-of-scope, never declared correct. The full scan has 35 findings.

| Split/family | TP | FP | FN | TN | Eligible units | Ambiguous/unsupported | Precision | Recall |
|---|---:|---:|---:|---:|---:|---:|---|---|
| holdout/source_api | 2 | 0 | 1 | 2 | 5 | 1/0 | 2/2 | 2/3 |
| holdout/draft | 0 | 0 | 0 | 0 | 0 | 0/0 | undefined (0 denominator) | undefined (0 denominator) |
| holdout/config | 0 | 0 | 1 | 1 | 2 | 0/0 | undefined (0 denominator) | 0/1 |
| holdout/dependency | 1 | 0 | 1 | 2 | 4 | 0/1 | 1/1 | 1/2 |
| holdout/certificate | 0 | 0 | 0 | 0 | 0 | 0/2 | undefined (0 denominator) | undefined (0 denominator) |
| holdout/key | 0 | 0 | 0 | 0 | 0 | 0/1 | undefined (0 denominator) | undefined (0 denominator) |
| tuning/source_api | 9 | 3 | 5 | 3 | 20 | 1/1 | 9/12 | 9/14 |
| tuning/draft | 1 | 0 | 0 | 2 | 3 | 0/0 | 1/1 | 1/1 |
| tuning/config | 3 | 0 | 0 | 3 | 6 | 0/0 | 3/3 | 3/3 |
| tuning/dependency | 1 | 0 | 0 | 1 | 2 | 0/0 | 1/1 | 1/1 |
| tuning/certificate | 4 | 0 | 0 | 2 | 6 | 0/0 | 4/4 | 4/4 |
| tuning/key | 1 | 0 | 0 | 1 | 2 | 0/0 | 1/1 | 1/1 |

All families combined are 22 TP, 3 FP, 8 FN and 17 TN over 50 eligible units;
these are descriptive finite-corpus counts, not a general accuracy estimate.
Tuning source errors expose local namesake functions/modules, Ruby display-only
strings, aliases, lost duplicate occurrence, typed TypeScript and malformed-source
handling. New holdout misses are u051 (nominal ML-DSA import), u054 (XML Java TLS
policy), u056 (renamed Cargo package identity). They are measured, not repaired by
this baseline task. Raw details are in BASELINE.json and report.json artifacts.

No held-out certificate/key eligible population exists (only unsupported controls),
so those held-out precision/recall values are undefined. Held-out draft population
is empty. Source API holdout has only three positives; config has one; dependency
has two. Perfect precision in a tiny supported cell does not justify a quality
claim, and a null denominator must never become 100 percent. The full seven-
language set is not seven independent projects or a seven-language holdout.

## Reproducibility and coverage

Three actual CLI runs use the same temporary root, explicit checked-in rules,
`--threads 1 --format all --diagnostics`; each reads all 41 files. Semantic,
diagnostic and score digests match on every repeat; native array order also
happened to match and is reported separately. Canonicalization sorts native
finding/inventory arrays while retaining duplicates/IDs/all facts; only top-level
generated_at and the built-in certificate validity assessment clock (plus its
identical preview token) are volatile. Expiry/status/notAfter remain meaningful.
Two independent-review bugs in timestamp allowlist breadth and score ordering
were reproduced by failing tests and corrected before accepted measurements.

Diagnostics reconcile selected 41 = read-ok 41 + read-failed 0, and read-ok 41 =
partial 7 + unknown 34 (all other terminal categories 0). Overall status is partial,
not complete. Detector-unsupported observations are distinct from file terminal
unsupported counts. Dependency internals still do not report completeness, and
not-enumerated population is explicitly unknown. No invented zero-loss verdict.
All report files and stdout/stderr were checked for both synthetic private-body
canaries; none survived. This is not a universal privacy proof.

## Performance measurements

Release Rust 1.93.1/aarch64-apple-darwin scanner, Apple M4 Pro, 12 logical CPUs,
48 GiB RAM; Python 3.12.7 and Darwin version/build are recorded in BASELINE.json.
Default workspace features, default 2 MiB cap/exclusions/gitignore policy and one
thread. Source/rules/catalog, binary SHA, every root/command and each input digest
are recorded. The medium/large recipes are 8/32 complete corpus copies, not new
independent accuracy samples. The largest ordinary workload is only 184 KB;
these names describe relative synthetic sizes, not enterprise-scale projects.

Each workload has one first-measured sample plus three warm samples. First means
after materialization, not flushed OS cold cache. Wall includes process launch,
scanning and report writing; build time is excluded. CPU and peak RSS come from
wait4 for that child only; macOS byte/Linux KiB conversion is explicit. Unsupported
platform metrics are null. Timeout/nonzero exits cannot become fast successes.
Wall polling has roughly 5 ms resolution, material for these short workloads.

| Workload | Files / input bytes | Warm wall seconds min / median / max | Warm CPU median seconds | Warm peak RSS MiB min / median / max |
|---|---:|---|---:|---|
| small | 41 / 5746 | 0.020012 / 0.022659 / 0.024152 | 0.016906 | 16.19 / 16.20 / 16.30 |
| medium | 328 / 45968 | 0.053316 / 0.058883 / 0.062421 | 0.043495 | 18.83 / 18.84 / 19.00 |
| large | 1312 / 183872 | 0.150452 / 0.170867 / 0.175764 | 0.141617 | 31.67 / 32.52 / 32.92 |
| pathological | 1 / 60017 | 0.020698 / 0.021065 / 0.021262 | 0.016629 | 13.84 / 13.86 / 14.20 |

Raw samples and first-run values are in BASELINE.json. Small runs are startup/
report dominated; these three warm samples do not characterize host noise or
production throughput. The 10,000-token pathological JS case emits explicit
detector_truncated diagnostics rather than claiming complete inspection.
No true cold-cache, multi-host, Windows, optional feature, CPU architecture or
real-product performance acceptance was executed.

## Measured budget proposal — UNAPPROVED

Propose a corpus-specific review signal: for the unchanged eligible truth set,
per split/family TP may not decrease and FP/FN may not increase. Improvements are
allowed; cells without denominators have no percent target. This preserves known
shortcomings rather than inventing a desired accuracy percentage. It must not
turn the small public holdout into a tuned acceptance claim.

Provisional performance review signals are twice the observed warm maximum wall
and peak RSS, with exact computed numbers in BASELINE.json. This is heuristic
headroom chosen after measurement, not a statistically justified confidence
bound. Compare only the same host/profile/config/bytes and corroborate with
repeat runs. Maintainers should review whether to use these signals or require a
larger independent corpus and multi-session noise study first. No new global
accuracy/latency/RSS guarantee is proposed. **No numeric budget is approved or
enforced by CI.** Maintainer protocol/budget review is the remaining task023 gate.

## CI, compatibility and self-scan policy

The new baseline job builds the existing release CLI, runs the same offline
harness with Python 3.12 and preserves measurements/full reports even on failure.
Quality also runs evaluator sanity tests. CI enforces manifest/label bindings,
lineage split integrity, actual file reads, coverage arithmetic, canary absence,
timeouts and semantic/diagnostic/score stability. It never auto-updates truth or
enforces unapproved numeric scores/timing. Existing Code Scanning failure severity
and raw-SARIF upload projection policy remain unchanged.

Production crates, rules, Cargo dependencies, CLI, report schemas/names and shell
projection code/tests are byte-unchanged from task022 (verified with git diff on
those exact paths). Existing seven-language Rust tests remain and run unchanged.
Both READMEs and example README document the new developer workflow; no default
scanner behavior changes. Existing legacy output regression tests all pass.

Self-scan: **776 raw = 645 reviewed test omissions + 131 retained**, 99 verified
source bindings. Exactly 21 known-location observations in 15 specifically reviewed
new fixture/generator files were registered; no broad path/rule suppression or
product registration. All prior 128 retained results remain byte-equivalent.
Three new artifact-only dependency results from the escaped-name SBOM intentionally
remain retained: the existing helper cannot safely bind unknown-line expectations.
Its positive-line/literal-path/full-source/full-result/count checks were not
relaxed. These three fixture observations remain eligible for hosted Code Scanning
checks; local scan/preparation success does not assert hosted alert-check success.
Raw bytes, remaining objects and run metadata remain preserved.

## Validation and review evidence

- `cargo build --workspace --release --locked --offline`: exit 0; release-build.log.
- `cargo fmt --all`, build/fmt check, workspace tests and clippy with `--locked --offline`:
  exit 0; target/task023/command-results.json and respective logs; 145 Rust tests.
- `python3 -m unittest discover -s tests/quality -p 'test_*.py'`: exit 0, 20 tests.
  Hand-derived TP/FP/FN/duplicate/ambiguity/boundary/denominator controls, source/
  label/path/lineage/digest guards, clock/order controls, actual timeout and per-
  child RSS high-water regression, deterministic materialization are covered.
- `node --test shell/*.test.cjs`: exit 0, 22 tests, including actual fixture/product
  separation with product error retained.
- Workflow YAML parsed locally; exact baseline build/run commands executed locally.
  Hosted Actions/Code Scanning, actionlint and Linux execution are unexecuted.
- Actual baseline command: `python3 tests/quality/run.py --binary target/release/pqc-scan --build-profile release --out-dir target/task023/baseline-complete`;
  exit 0; baseline-complete.log plus full raw artifacts in that directory.
- Actual self-scan and strict Node projection: exit 0; self-scan-final/, self-scan-summary.json.
- `git diff --check` and runtime-path immutability comparison: exit 0.

Independent reviewer review023 inspected code, all truth-source units, v2 lineage,
CI and measured results in a fresh context, independently ran the sanity suite,
and required fixes to the two meaningful comparison bugs and inclusion of
embedded JS/CSS in source bindings. Final review outcome is
in [the committed independent review record](TASK-023-REVIEW.md). Original labels
and independent lineage audit history are preserved; a role change by the
coordinator is not presented as independent review.
