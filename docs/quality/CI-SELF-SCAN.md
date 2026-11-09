# Repository self-scan expectations (task 040)

## Goal and authorized scope

The repository intentionally contains positive detection fixtures, public certificate fixtures and inline unit-test strings. Uploading all of these as actionable production alerts caused the PR 11 Code Scanning results check to fail on a synthetic private-marker test. Keep scanner detection, native outputs, rule severity and GitHub failure thresholds unchanged. User authorized this CI correction, tests, documentation, independent review and a local commit; no push, PR or merge.

Baseline: `dc7b63c361263dd3ef44e03855b983c485c9fde6` (`origin/develop`, task 020 merged). Branch: `codex/ci-self-scan-expectations`. Initial checkout clean; work in this dedicated branch in place. One task/PR unit. Task 021/022 and Rust changes are excluded.

## Design and alternatives

Chosen: preserve raw self-scan SARIF and prepare a separate upload-only copy using a checked-in exact expectation manifest. Keep raw/filtered SARIF and an omission audit as workflow artifacts. This is repository CI tooling, not a scanner suppression feature or a new native report format.

Rejected: disabling Code Scanning failure thresholds, downgrading severity, excluding all test/source directories from detection, `continue-on-error`, changing the scanner's rules, or escaping test strings merely to evade detection. These would either weaken checking or hide evidence. SARIF suppression markers would depend on consumer-specific handling; an explicit upload projection with preserved raw evidence is directly testable.

Policy `docs/quality/SELF-SCAN-EXPECTATIONS.json` uses version 1 and explicit files. Each file has a repository-relative path, complete source-file SHA-256, a reviewed reason and exact expected results (rule ID, start line/column, SHA-256 of canonical full result, allowed count). No globs, severity-wide exceptions or automatic learning in CI. Only reviewed fixture files, examples and actual inline unit-test regions are registered initially. Product/rule implementation findings remain visible.

The Node helper `shell/prepare-code-scanning.cjs` exports `prepareCodeScanning(sarif, policy, root)` and `resultDigest(result)`, and has a CLI accepting `--input`, `--expectations`, `--root`, `--output`, `--audit`. Use built-in Node modules only. Canonical JSON sorts object keys recursively, preserves arrays and includes every SARIF result field; the current writer has no volatile result ID/timestamp to discard.

An expected result is omitted only if the tool is exactly `pqc-scan`, there is one physical primary location, no secondary/code-flow/related locations, the path is literal/normalized, source bytes and the full result digest match, and the observed count equals the registered count. Counts are checked across the whole SARIF before omission: an unexpected extra duplicate or copied run keeps the whole group. URI base/index references and run-level artifact/base/invocation indirection are ineligible. Different messages, levels, properties, rules or locations remain unchanged. A changed/missing/symlinked/outside-root source cannot be omitted. Unexpected/other-tool results and run metadata are preserved. Bad input/policy causes a safe nonzero exit, never an empty successful upload.

Audit JSON contains counts, policy/source digests and safe result identities/reasons, not scanned source text or messages. It reconciles raw = expected omissions + retained. The workflow also archives the exact policy. Input SARIF is never overwritten. Errors do not echo malformed JSON/source payloads. CLI refuses input/output/policy/audit aliasing. Both native scanner outputs and full scan coverage remain available independently of this projection; absence of an uploaded result is not proof of safety.

## Files and implementation plan

- `.github/workflows/pqc-scan.yml`: retain full scan, add preparation step and raw/filtered/audit artifact with `if: always()`; upload only the prepared SARIF. Run all `shell/*.test.cjs` in quality checks; preserve Rust and existing SBOM checks.
- New Node helper and `shell/prepare-code-scanning.test.cjs`: strict policy/data validation, exact matching, unchanged unknown results, count reconciliation and CLI behavior.
- New explicit manifest: reviewed known test/fixture observations only; never derive a general product-code baseline from all current results.
- Both READMEs, this document, task 040: operational instructions, limitations and executed evidence.

- [x] Capture current full self-scan and baseline checks under ignored `target/task040/`.
- [x] Write failing unit/CLI tests before implementation: exact match only; source mutation; new same-file result; duplicate excess; other tool/multiple locations; unsafe paths/symlinks; invalid policy/input; input immutability; safe audit/errors.
- [x] Implement minimal dependency-free helper and run Node tests, including existing report navigation tests.
- [x] Register reviewed fixture-only results from actual self-scan, with exact source/result hashes and counts. Test full repository preparation and actual scanner output with an identical unregistered product input that must remain actionable.
- [x] Update workflow/docs. Validate YAML, execute workflow commands locally and compare native outputs with baseline; prove Rust/rules/CLI/report code unchanged.
- [x] Independent whole-diff review; resolve material issues. Run pinned workspace build/fmt/test/clippy and all Node checks; record local versus remote evidence separately.
- [x] Evaluator records digests/results and completion; commit locally and verify clean tree. No publication.

## Review focus

No directory/rule-wide bypass; a real payload replacing a synthetic body invalidates source hash; excess duplicate/new same-rule finding cannot consume an allowance; path traversal/URI aliases/symlinks cannot select another file; zero/malformed/other-tool runs cannot turn failures into a clean upload. Remote GitHub acceptance cannot be asserted before a separately authorized push; local tests demonstrate exactly which error-level results reach the upload file.

## Local use and maintenance

From the repository root, run the normal scanner first, then prepare only its CI upload copy:

```sh
cargo run --bin pqc-scan -- scan . --format sarif --out-dir ./target/self-scan --rules-dir ./rules
node shell/prepare-code-scanning.cjs --input target/self-scan/report.sarif --expectations docs/quality/SELF-SCAN-EXPECTATIONS.json --root . --output target/self-scan/code-scanning.sarif --audit target/self-scan/code-scanning-audit.json
node --test shell/*.test.cjs
```

The source registry lives beside this quality policy. An initial `.github/` location made the scanner detect 37 self-references in the registry itself. Keeping quality metadata under the existing `docs/` scope avoids that feedback loop without moving any inspected source/test input or changing walker exclusions. This registry is not a deployment input or an additional scan capability.

Review a new expectation as a code change: establish that the exact source is an intentional test/example, inspect the raw rule/location/message, verify secret material is synthetic/public, and preserve any product findings. Compute complete source SHA-256 and `resultDigest(result)` with the exported helper. Copy explicit rule/line/column/digest/count records into the relevant file entry with a reason. Do not blanket-register a raw scan or automatically approve every current finding. The initial registration is limited to examples, integration-test fixtures and actual `cfg(test)` unit-test regions; a source file may contain both registered test results and retained product results.

A changed file requires re-review of its *whole* source binding, even if the edited line is unrelated. New result fields, locations or occurrence counts also require re-review. A stale/unmatched entry never suppresses a different observation; its source status appears in the audit. Keep policy updates with the test/behavior they cover in one PR. Existing rule/catalog or product-code observations that have no reviewed test expectation remain eligible for the usual Code Scanning failure threshold.

The audit exposes `rawResults`, `expectedResults`, `retainedResults`, source verification statuses, exact omission identities/reasons, a canonical policy digest and the original SARIF byte digest. Raw and upload copies must satisfy the count partition. Workflow evidence preserves both copies, the audit and policy even when preparation/upload fails. The separate existing dependency SBOM artifact is unchanged. Previously uploaded live alerts are not dismissed by this local change; remote check behavior must be verified after a separately authorized push.

## Primary references

Consulted 2026-11-09:

- [GitHub: triage alerts in pull requests](https://docs.github.com/en/code-security/how-tos/manage-security-alerts/manage-code-scanning-alerts/triage-alerts-in-pull-requests): new qualifying alerts can fail the Code Scanning results check, separately from successful scan execution.
- [GitHub: SARIF support](https://docs.github.com/en/code-security/reference/code-scanning/sarif-support-for-code-scanning): results/rules/location handling for uploaded SARIF. This task preserves the existing writer and remaining result records.
- [GitHub: workflow artifacts](https://docs.github.com/en/actions/how-tos/writing-workflows/choosing-what-your-workflow-does/storing-and-sharing-data-from-a-workflow): retain raw evidence separately from the upload projection.


## Review and local validation record

Independent reviewer `ci_self_scan_review` approved design and full diff with no material findings. They verified every source digest and registration context (81 files, 618 exact groups), confirmed all source-file registrations lie in actual `cfg(test)` modules, replayed the raw projection and compared remaining results/run metadata, and independently ran all 21 Node tests successfully. No Rust/rules/fixture change was found.

Local Rust 1.93.1 offline build/fmt/clippy succeeded; 125 workspace tests and 21 Node tests passed. Initial 16 helper/CLI cases failed before implementation; an additional digest-type case failed before its explicit string validation. Actual scanner acceptance detects identical fixture/product private-marker inputs; only the reviewed fixture is absent from the upload and the product result remains error-level, with raw bytes preserved.

The actual workflow YAML was parsed and its scan/preparation commands executed locally, redirecting only output paths under ignored `target/task040/` and adding Cargo offline. Final partition: **746 raw = 618 reviewed omissions + 128 retained**. All 745 pre-change raw results still exist; the additional result is the new intentional shell acceptance input. The task020 synthetic private-marker result is omitted from the upload copy. Full raw evidence remains, and retained product/rule observations and metadata are unchanged.

No push, PR, live alert dismissal or remote Actions/Code Scanning check was executed. Remote acceptance is pending a separately authorized publication. `actionlint` is unavailable; YAML and local command execution are recorded without claiming full hosted-workflow acceptance. Exact tested source/registry digests and logs are recorded in task 040. No performance/accuracy benchmark or renderer change is part of this task.

## Task 022 location correction

Expectation version 1 also accepts `column: null` to bind an explicitly reviewed
line-only SARIF result. This matches only an absent `startColumn`, not zero,
explicit null, or a present positive column. The positive start line, literal
safe path, complete source/result SHA-256 and exact occurrence count remain
mandatory. Unknown-line/artifact-only and encoded/indirect paths remain retained;
the helper does not decode URIs. Existing positive-column registrations retain
their meaning.

Task 022 re-reviewed the 109 existing test groups affected solely by omitted
unproven AST columns and rebound their full-result digests to explicit null
columns. It refreshed the source binding for the appended helper test and
registered six observations confined to the new writer-contract Unicode input
and its lookup. No production finding was newly registered. Local partition:
752 raw = 624 reviewed test observations + 128 retained. The retained payloads
match the previous 128 observations, allowing only the intended omission of
unproven columns. Raw bytes and run metadata (including columnKind) are preserved.
See [task 022 evidence](SARIF-LOCATION-CONTRACT-022.md) for limitations and checks.

## Task 023 baseline fixtures

Task023 adds exactly 21 reviewed known-location observations from 15 explicit
baseline fixture/generator sources. These are positive, negative and unsupported
controls, not product configuration. No existing source/result allowance changes.
The partition is 776 raw = 645 reviewed omissions + 131 retained (99 verified
source files). All prior 128 retained results remain unchanged. Three new
unknown-line SBOM dependency observations remain in the upload because the
existing strict projection does not register artifact-only locations. No URI
handling/line validation/threshold was weakened to suppress these results.
Hosted alert-check success is unverified; these three observations remain eligible
for it. See [the measured baseline record](TASK-023-BASELINE.md).
