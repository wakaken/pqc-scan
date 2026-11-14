# Independent task 023 review

This is a historical source-bound checkpoint. Its pending/unapproved statements
describe that checkpoint. Current task 023 acceptance and local criteria are in
[TASK-023-ACCEPTANCE.md](TASK-023-ACCEPTANCE.md) and
[QUALITY-GATES.md](../../docs/quality/QUALITY-GATES.md); current task decisions are in
[STATUS-2026-11-11.md](../../docs/records/v0.2-roadmap/STATUS-2026-11-11.md). Saved measurements and reviewer
judgments below are preserved.

Reviewer: separate whole-change review agent, 2026-11-10.
Base: 35d988e03629cb15b17a94af81b1e74266a344ff.
Status: initial review; final CI/documentation/measurement changes pending.

## Scope and evidence

Read repository agent/reviewer instructions, architecture and knowledge documents, task 023, implementation plan, corpus protocol, evaluator/runner, sanity tests, corpus bindings and adjudicated label reasons. Source-only spot review covered all 47 nominated units; source labels were not inferred from scanner outputs. No production runtime/rules/native report changes were present in the initial worktree.

Executed: `python3 -m unittest discover -s tests/quality -v`: 15 tests passed. This review did not execute full Rust checks; the coordinator owns final evaluator evidence.

## Findings requiring correction

1. P2: `normalized_report` drops `assessment_time_unix` and matching snippet text on every finding, not only certificate observations. A noncertificate finding with identical content except metadata assessment time currently hashes equal. Restrict the volatile allowance to certificate-origin evidence and test both permitted and forbidden differences.
2. P2: `run` compares full scoring dictionaries with order-sensitive `digest`, including `details` in native finding order. Reordering otherwise identical findings yields semantic stability but score instability and fails the run. Compare scoring observations without order dependence while continuing to report raw ordering drift separately; add a reversed-finding-order sanity case.

## Reviewed invariants

One-to-one positive matches, duplicate FP, missed same-line occurrence FN, exact dependency-name fallback only for unknown native line, family/split denominators, null zero denominators, ambiguous/unsupported exclusion, unmatched scoped FP and explicit out-of-scope prediction counts are coherent. Negative-unit duplicate predictions intentionally contribute multiple FP observations, so FP is a prediction count rather than a count of negative units.

Source hashes, truth-to-manifest and reviewer-label hashes, lineage split constraints, path traversal rejection, fresh materialization outside ignored target ancestors, output separation, coverage arithmetic and all-file count assertions are present. Corpus is small synthetic/public data; it supports descriptive measurements only. Public tuning and holdout are explicit developer evaluation sets, not confidential validation.

Child resources use wait4 on the exact child rather than process-lifetime maxima. Darwin/Linux RSS conversion and timeout process-group kill are sound for the single native scanner child. Unsupported resource platforms report null values. Success requires non-timeout exit 0. Canary checks inspect generated files and stderr.

Budget approval remains explicitly pending. No accuracy or performance target should be marked approved merely because measurements and harness checks pass.

## Correction review

Both P2 findings were corrected. Volatile normalization now requires CERT_EXPIRED/CERT_NOT_YET_VALID, certificate evidence type, and certificate_detector metadata; validity status remains significant. Scoring canonically orders predictions before assignment. New tests cover certificate versus noncertificate clock changes and reversed observations including duplicate/same-unit positions. Independent rerun: 17 tests passed.

Inspected initial real measurement summary: 35 files, 47 units; three accuracy repetitions had stable semantics, diagnostics, raw order and score. Eight predictions were explicitly out of scope. Holdout source API observations were TP=2, FP=3, FN=5. Holdout config has no positives, and holdout certificate/key have no eligible units; recall for these holdout populations is unmeasurable and must remain explicit. Four workloads each retained first-measured plus three warm samples and per-child resources.

## Final review

Evaluator/runner/corpus changes are acceptable after corrections. Final approval of whole change remains pending coordinator follow-up for final docs/CI/measurement artifacts. Budget approval remains a distinct pending maintainer decision.


## Revised corpus 023-v2 review

Source-only lineage audit correctly invalidated the original adversarial holdout as independent evidence. The original manifest/labels and exploratory measurements are retained rather than overwritten or promoted. All readiness-derived source/config transformations now share readiness-017 tuning lineage. The new offline approval-adapter project has six separately authored source/config/dependency fixtures and ten units. Source review of these bytes and final A/C reasons found no additional truth-label concern. Its provenance explicitly excludes scanner/rule/metric access, and the final manifest binds all 41 files/57 units to current independent labels before v2 measurement. This is a small public project-level holdout, not representative prevalence evidence.

Independently executed current Python sanity suite: 19 tests passed, including duplicate-label rejection and high-memory then small child RSS separation. The exact current corpus digest is d6fa706b8f2c2c1267e9e1f0383d6318f4c11b2761f8785b6d7865797ee2afb6.

Inspected baseline-v2 summary: all three accuracy repetitions were stable in semantic/diagnostic/order/score dimensions. Holdout source API TP=2, FP=0, FN=1; config TP=0, FN=1; dependency TP=1, FN=1. Out-of-scope predictions=10. Empty eligible holdout draft/certificate/key populations remain null. The initial v1 holdout observations above are exploratory only and are superseded by v2 for current holdout claims.

Reviewed CI baseline job, dual README additions and quality-gate changes: CI runs fixed-source measurement, archives reports and checks harness sanity/repeatability/coverage; it does not enforce unapproved accuracy or resource thresholds. No new blocking finding. Final measured-results document and individually reviewed self-scan ledger were not yet present at this review checkpoint.


## Final documentation and projection review

Reviewed TASK-023-BASELINE.md, committed BASELINE/INITIAL-EXPLORATORY machine records, CI-SELF-SCAN and QUALITY-GATES additions, both READMEs, task023 review lifecycle record and explicit fixture allowance updater. The measured record agrees with accepted local raw summary except the explicitly disclosed regenerable per-copy benchmark binding omission and added proposal/artifact fields. Recomputed every performance proposal as twice the measured warm maximum and checked positive=TP+FN and eligible=positive+negative for every group.

The prior policy file entries are byte-equivalent as parsed objects; additions are 21 exact observations in 15 named sources. Independently reran the existing unmodified strict projection against self-scan-final/report.sarif: 776 raw, 645 reviewed, 131 retained. Unknown-line SBOM observations remain retained and hosted alert-check success is explicitly unexecuted. No broad suppression or policy weakening was found.

Final independent checks: 19 Python sanity tests passed; git diff --check passed. Inspected coordinator command-results and raw Rust/Node logs: build, formatting, 145 Rust tests, clippy, 22 Node tests and runtime immutability checks are recorded successful. Hosted Actions/Code Scanning and cross-platform/performance generalization remain unexecuted.

One final evidence improvement requested: runtime source bindings must include the two compiled include_str renderer assets compact.js and compact.css, in addition to Rust and Cargo/rule files. Binary SHA already binds the executable, but complete source provenance should not omit these assets. Final implementation approval is pending that narrow correction. No other blocking finding remains. Maintainer protocol/budget approval remains a separate pending task decision, and review status is correct.


## Final approval after source-binding correction

The final source-provenance finding is resolved. The runner now hashes compiled JavaScript/CSS renderer assets as well as Rust/Cargo/rule sources. A regression test specifically verifies compact.js and compact.css binding. Independent final execution: 20 Python sanity tests passed; git diff --check passed.

Independently recomputed every recorded runtime/harness file SHA-256 against the current worktree: no mismatches. The accepted final evidence is target/task023/baseline-complete/. Its complete local machine summary agrees exactly with tasks/023/evidence/task023/BASELINE.json after the disclosed regenerable benchmark binding omission and added proposal/artifact fields. All three semantic, diagnostic, raw-order and score repeats remain stable. All provisional resource signals again equal twice the final observed warm maxima. The exact run.py whole-source policy binding was refreshed and matches; its intentional benchmark literal location and finding payload remain unchanged.

Final decision: APPROVED for this development-only baseline implementation, documentation, CI integrity checks and narrowly scoped fixture-policy additions. No unresolved material correctness, compatibility or scope finding remains in the reviewed changes. Production scanner/rules/native schemas and projection behavior remain unchanged. This approval is not maintainer approval of numeric budgets, population-level accuracy, hosted Actions/Code Scanning or cross-platform behavior; those limitations and the pending protocol/budget decision are correctly disclosed, and task023 should remain review until that decision. Local commit is within the authorized scope; no push/PR/merge is approved or performed by this review.
