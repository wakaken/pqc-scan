# Static 0.3.0 candidate readiness — 2026-11-17

Status: local verification and both independent reviews passed within the scope below;
hosted receipts must be matched to the candidate commit. Maintainer product acceptance
and publication are not granted by this record.
The maintainer selected static macOS arm64 and version **0.3.0**. Published 0.2.0
is not overwritten. Network release holds, real signing and publication are separate.
No original criterion, frozen truth, rule, numerical budget or task state was changed.

Base: origin/develop `168efc6`, then reviewed output-safety checkpoint `fa3e2b7`.
Branch: `codex/static-release-readiness-20261117`. Measurements identify dirty-source
inputs explicitly; a HEAD alone does not identify this candidate. See the source
bindings in `evidence/native-rebuild-final/SOURCE.json` and the review records below.

## Changes and compatibility

The output-safety checkpoint replaces report destination writes with held-descriptor,
no-follow traversal and exclusive staging followed by rename. Existing ordinary
files can be replaced without truncating their inode or following hard/symbolic links.
Unix new outputs are private (0600); old inode permissions/ACLs are not retained.
Non-Unix output is explicitly unsupported. The original P1 evidence and both reviews
remain in the managed remediation collection; this release does not rewrite them.

The maintainer approved the position correction: explicit Python imports start at
the module after `from ` and retain imported symbols/continuations. Configuration
regex evidence excludes leading layout whitespace. Whitespace-only custom matches
retain their previous ranges. Comment selection and original candidate byte-budget
charges are preserved. Detection decisions, counts, rules, confidence and risk are
unchanged. Affected location/evidence/snippet fields and finding/observation IDs can
change. This is an explicit compatibility effect, including custom config rules.

`evidence/LOCATION-COMPATIBILITY.json` compares actual old/new binaries on identical
current inputs: 52/205/216 findings in the 023/042/043 cohorts. It enumerates the
3/1/1 affected findings and every changed field. All other finding fields, including
risk/severity/confidence/actions and metadata, are equal. These cohorts overlap;
these counts are not independent validation populations. Native span tests cover
Unicode, CRLF, multiline and indented input, whitespace-only matches and the exact
original byte-cap boundary. Scanner version and its golden expectation are 0.3.0.

## Original release checks

| Gate | Evidence and result | Limit / remaining action |
|---|---|---|
| Output safety | P1 checkpoint and prior reviews reused; final default and all-feature suites include affected sink tests | Actual publication/signing is not authorized |
| Original 023 accuracy nonregression | `evidence/ACCURACY-NONREGRESSION.json`: Passed, no regressed split/family; truth/corpus/threshold unchanged | Historical Failed remains; this is post-observation tuning, not a fresh independent holdout |
| Test adequacy | M03/M04/M09 now fail meaningful assertions; restored baseline passes. `MUTATION-RESULTS.json` | Three new mutation executions; other 16 caught and M20 equivalent reuse the historical campaign. This is not a new 20-mutant campaign |
| Critical coverage observations | New default-only ordinary export and archived merged profile succeed; raw src-path lines 20,882/22,467, regions 34,151/36,938, branches 4,145/5,490 | Raw src-path rates include inline tests. No exact production-only rate, all-branch coverage or absence-of-defects claim |
| LLVM warnings | New seven-entry mismatch vector independently extracted by a bounded default-only diagnostic run: all the same locked memchr definition outside owned src. `LLVM-MISMATCH.json` | Historical all-feature auxiliary dump SIGSEGV is preserved; not retried or claimed fixed. No general LLVM-health claim |
| Identity and repeatability | Three accuracy repeats stable; two actual native/assessment/CDX runs retain finding/asset/occurrence identity and change run/observation IDs. `OUTPUT-STABILITY.json` | ID-inclusive strict Markdown/native/assessment/CDX equality remains false. No ID removal or relaxed normalizer |
| Dependencies | Fresh advisory database retrieval and pinned cargo-audit: 348 packages, zero findings, exit 0 | Database snapshot/target scope are recorded; no future-vulnerability guarantee. Network notice holds remain |
| Static archive | Actual 0.3.0 static archive, 139 dependency packages, zero missing notices/review flags; clean-prefix offline install, installed rules and synthetic external scan pass | Local unsigned candidate, no publication; caller-supplied binary bound to separate independent rebuild evidence |
| Independent native rebuild | Two clean local environments, no shared compiled artifacts: identical raw binaries SHA-256 `04caf19bffd2839e9dc5ea9b959362f21fb67810fac88ca16748c3c483193b4a` | Same physical host, recorded path normalization/toolchain/SDK; no all-host reproducibility claim |
| Required checks | Rust 1.93.1: fmt/build/default tests 580 passed, 3 ignored; all-feature tests 639 passed, 5 ignored; both Clippy configurations pass. Python quality 160 (3 skipped), distribution 25; Node 53 pass | Skipped checks are not passes. Existing environment-dependent quality hooks remain explicitly skipped; actual candidate runs are separately recorded |
| Current self-scan | All 106 product inputs read/analyzed; 485 raw, 475 reviewed references, 10 requiring review | Four existing source bindings and 94 distinct/98 occurrence positions mapped to unchanged test lines; no new allowance, payload/count/rule/reason unchanged. Raw results remain intact |
| Hosted CI / Code Scanning | Candidate source `d194787`: quality/baseline/scan succeeded; protocols skipped. Server analysis 2183141978 has 10 results, no error/warning; all ten branch alerts match uploaded rule/path/line/full message | `evidence/ci-44017642002/RESULT.json`. Subsequent guide-only changes are outside that run; final-commit verification is recorded in the current handoff. Historical failure and 2,135-newness uncertainty are not erased |

## Residual product-quality disposition

These results support a bounded static inspection candidate, not comprehensive
cryptographic safety certification or universal detection accuracy. Original 123-rule
coverage classification remains 93 nominated Not Tested, 5 Partial and 25 positive/
negative strata. Do not turn these labels into Passed or infer complete support from
absence of a finding. Existing reviewed rule descriptions and documented limits apply.

`evidence/CRITICAL-OUTCOMES.json` maps every historical 154 exact source span to the
new default export and actual source line. Eighteen belong to held network features;
136 static outcome observations remain incomplete (89 bounded dependency grammar,
26 certificate attribute interpretation, 17 pipeline/snippet/privacy handling, and
4 redaction outcomes). These are production-source locations, not automatically
test-only, unreachable or confirmed vulnerabilities. Existing boundary/adversarial
and privacy tests remain evidence for their assertions; a missing LLVM outcome is
not evidence that an arbitrary untested input is safe. Further coverage work needs
specific behavior or invariant oracles, not counter-only tests or a lower threshold.
No original requirement demands a newly invented 100% branch threshold.

Product acceptance therefore requires an explicit maintainer disposition of these
bounded quality limitations after the concrete candidate and hosted evidence are
available. Public claims must remain consistent with the measured scope. Acceptance,
roadmap completion, task 054 evaluation completion and authority to publish are four
different decisions. Historical tasks/measurements are preserved; no task is reopened
or closed by this preparation record. Static acceptance does not release network holds.

## Evidence reuse and reviews

The original P1 default/all-feature and Linux safety evidence is retained. New final
native suites cover the combined source. The three mutation trials preceded the
version bump; the tested severity/normalization function and assertion bodies are
unchanged. The final golden correction changes only scanner version. A Clippy finding
in the new position test was corrected by replacing a single-element loop with an
equivalent block; targeted tests and both Clippy configurations pass. Coverage was
collected before that test-only syntactic correction; product src and test inputs/
assertions are unchanged, and final-source runtime tests pass. New metadata-only
notification bindings do not enter the compiled scanner or package payload.

Original test failures and preliminary measurements remain in the preparation scratch;
they are not relabelled as successful runs. The old 0.2.1 experiment and earlier rebuilt
0.3.0 intermediate are not the final binary. Archive provenance records the dirty-source
snapshot; any later clean-commit preparation must retain the binary/source binding.
No renderer/template/browser interaction changed, so existing presentation acceptance
is reused; actual generated reports and machine/native outputs were exercised.

Implementation reviewer `/root/output_implementation_review` accepted the combined
candidate, then the material notification/test follow-up (`REVIEW-SOURCE-R2.json` in
preparation evidence). Independently verified all 94 mappings, original count/reason/
payload constraints and the current-source tests. No unresolved implementation finding.
Test reviewer `/root/output_test_review` accepted the expanded final local candidate
against `evidence/TEST-REVIEW-SOURCE.json` (manifest SHA-256
`9fa98a29255d56c5852e65a6942ce688bf9f9754ea3a746ef77b2067cc50bb45`), with no actionable
finding; the earlier P1 acceptance is preserved. Both reviewers explicitly excluded
maintainer product acceptance/publication and pending exact-commit hosted CI from
their approval. No new reviewer identity is introduced. Reviews follow
`docs/agent/REVIEWER.md`. Later edits to this status record are prose-only and do not
change the reviewed product/test inputs or measured outcomes.

## Distribution-guide follow-up

The actual archive revealed a stale workspace-version statement and local archive
verification filename in the paired installation guides. Both now describe the
unpublished 0.3.0 candidate and its output/position compatibility effects; published
0.2.0 history and maintenance policy remain unchanged. Both existing reviewers
accepted this limited follow-up. Actual revised archive preparation and offline
installation pass; the dirty-source receipt is kept separately from clean-commit
preparation. Product/test bytes remain unchanged. Final package/source and hosted
receipts must identify the final commit, rather than relabelling the d194787 run.

A same-host, interleaved actual-binary performance check uses the same small corpus,
rules, one thread and all reports/native/assessment/CDX outputs: one warm-up and
five measured samples per binary. Candidate/old maximum wall-time ratio is 1.096,
below the original greater-than-2x investigation signal. Output time is included;
this is a small-input signal, not a new scaling/resource study or numeric CI gate.
See `evidence/PERFORMANCE-SIGNAL.json`; test review accepted this measurement scope.

## Individual quality follow-up

The maintainer subsequently authorized individual assessment of the 98 empty/
one-sided rule strata and 136 static missing-outcome spans. Current work and
source-specific measurements are in [QUALITY-COMPLETION.md](QUALITY-COMPLETION.md).
Earlier candidate binaries, hosted receipts, labels and limitations above remain
historical and are not relabelled as executions of the new product fixes.

## Independent audit fix follow-up

The current candidate follow-up is [AUDIT-FIXES.md](AUDIT-FIXES.md), with all73
original clauses preserved in [the candidate matrix](audit-fixes/REQUIREMENTS.md).
Its local checks and both reviews are accepted; exact hosted and clean-source
package gates are recorded there separately. This preserves the older candidate
measurements above rather than applying them to changed product bytes.
