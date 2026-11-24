# Task 065 results

Date: 2026-11-18. Scope: original R1-01 through R1-04 only. The all-rule audit is
complete; this does not assert calibrated priorities, measured detection accuracy,
implemented v0.4 decisions or completed migration. Product version remains 0.3.0.

## Selected source and deliverables

Worktree: `/Users/waka/.codex/worktrees/v04-roadmap-20261118/pqc-scan`.
Branch: `codex/v04-roadmap-20261118`.
HEAD: `3f2bc6e6209e1f2813caac8077c8d7a7198832cc`, plus the thirteen pre-existing
staged planning/definition additions and this task's unstaged audit documents.
[Initial source/index bindings](evidence/SOURCE.json) and
[final verification](evidence/FINAL-SOURCE.json) identify the actual bytes;
HEAD alone does not identify this work. Original index bytes remain preserved.
No commit, push, PR, release, external message or task 066 execution is included.

- [Execution plan](TASK-065-PLAN.md).
- [123-row audit supplement](RULE-AUDIT.json), composed with the unchanged
  [maintained companion](../../docs/quality/rules/RULE-QUALITY-METADATA.json).
- [Population, exact snapshots and source bindings](evidence/POPULATION.json).
- [Correction impacts and D5 inputs](CORRECTION-IMPACTS.md).
- [Historical evidence applicability](evidence/REUSE.json),
  [reference checks](evidence/REFERENCE-CHECKS.json),
  [commands and actual exits](evidence/COMMANDS.json).
- [960-case severity comparison](evidence/SEVERITY-COMPARISON.json),
  [selected frozen cases](evidence/SEVERITY-CASES.json),
  [current observations](evidence/SEVERITY-OBSERVATIONS.json), and
  [real validator negative controls](evidence/NEGATIVE-CONTROLS.json).

These task-local records extend the existing audit/evaluation assets, not the
product schema, runtime evaluator or a parallel audit platform. New developer
prose is English. Existing Japanese roadmap/requirements and historical records
remain unchanged; no user-facing behavior changed, so paired user guides need no
revision. The maintained rule guide now reports current counts/dispositions and
links to this supplement.

## Original acceptance reconciliation

| Criterion | Final-source evidence and conclusion |
|---|---|
| R1-01 | Direct YAML enumeration finds eight files and 123 unique IDs, exactly equal to companion and supplement IDs. Each row has a source path/line/hash and exact companion pointer/snapshot. The actual Rust companion validator passes on original data and rejects seven deliberate malformed variants, including missing/duplicate/extra rows, substituted ID, changed snapshot/hash and unresolved reference ID. Loader and detector custom-scope tables pass; CLI invalid metadata returns operational exit 1 before reports. Effective local references are resolved in the supplement. |
| R1-02 | Every composed row records purpose, family, parameters/explicit unknown, quantum/classical axes, method, reference editions/application limits, known gaps and current FP/FN disposition. Java/Ruby SLH-DSA family/FIPS205 omissions are completed in the supplement, and two relocated local references resolve through its explicit map. Current controls retain four false-positive targets; four historical false positives have corrected negative expectations. Unobserved end-to-end FP/FN rates remain unknown, never inferred from passing pattern controls. |
| R1-03 | Every row gives actual base Severity/confidence plus an explicit rationale-gap disposition and its promotion gates/results. The seven shared rationale records distinguish category, Risk, f32 confidence, exact private_key evidence and saturating bump. Fresh 960-case frozen expectation comparison has zero mismatches/unassessed cases; five risk tests include 0.449/0.45 and saturation, and normalization preserves confidence bits. Unknown empirical numeric justification is expressly permitted by this criterion; no required audit was left unperformed or relabeled as an optimality study. |
| R1-04 | Five correction dispositions identify exact affected sets, output/exit/custom-pack impacts, review/decision requirements and positive/negative/boundary obligations. Only audit annotations/guide prose are corrected. All executable rule bytes and runtime contracts are unchanged. D5 proposals remain unapproved and unimplemented. |

## Findings and meaning boundaries

Current population is 123, with 268 synthetic compiled-pattern targets; these are
not 123 independent accuracy samples. Four retained negative targets match in
Apache protocol/cipher exclusions, nginx cipher exclusions and Ingress cipher
exclusions. Their exact rule IDs and fixtures are preserved. Four old examples
(TLS1.0 modern-version boundaries, Node minVersion, AST legacy version and Ruby
generic JWT) have corrected current expectations. Later Cargo package-identity
and TLS reference span/adjacency repairs remain in place and pass targeted tests.

Two current family/reference annotations omit SLH-DSA despite the predicates
including it; the supplement supplies the missing facts. `LOCAL021` and
`LOCAL028` point to moved documents in the inherited metadata; the supplement
retains old values and resolves current paths. Reference-ID checks alone did not
prove path existence. No inherited snapshot or historical result was rewritten.

The reviewed bounded records do not establish independent justification for any
exact base Severity level or empirical derivation for any of the 123 numeric
confidence values. Signal descriptions and primitive facts explain what a rule
seeks; they do not validate its priority magnitude. These unknowns are explicit
for every row, rather than silently filled with invented calibration.

All 25 category/Risk-eligible built-ins retain their original values: 24 pass the
0.45 gate and `TLS_CERT_PIN_RSA` at 0.42 does not. No eligible built-in has exactly
0.45; custom rules can, so absence in built-ins does not justify changing the
boundary. The current category set is exact case-insensitive TLS/PKI/JWT/auth/
authentication, with no trimming. Middleware, SSH and CryptoAPI do not enter it.
Private-key evidence becomes Critical only after category/Risk/confidence gates;
ordinary evidence bumps one level with Critical saturation. Confidence is an
uncalibrated f32 rule-match heuristic, preserved in findings, not a vulnerability
probability, impact, deployment assurance or organization-specific urgency.

Custom-pack acceptance remains asymmetric: unknown fields/scopes and valid custom
IDs are preserved; f64 confidence validation occurs before f32 narrowing; AST
scopes, typed certificate scopes and generic regex/key/dependency scopes retain
their existing contracts. A custom private_key rule can remain unpromoted when
an earlier gate fails. No new custom-pack restriction or warning stream is added.

## Actual verification and reuse

All maintained executable sources, rules, tests, dependencies and toolchain inputs
match the initial source manifest. Default-feature Rust 1.93.1 tests use offline,
locked resolution. Current executions passed:

| Command/check | Actual result |
|---|---|
| `cargo test --offline --locked -p pqc_scan_rules` | exit 0; 14 tests, including one companion validator with all 268 pattern targets |
| Same compiled companion validator with seven deliberate metadata mutations | exit 101 for each expected rejection; original bytes restored in finally, original validator exit 0 |
| `cargo test --offline --locked -p pqc_scan_detectors --test rule_scope_compatibility --test semantic_repairs --test f054_dependency_identity --test tls_reference_boundaries` | exit 0; 56 tests |
| `cargo test --offline --locked -p pqc_scan_core --test quality056 -- --nocapture` | exit 0; one adapter test emits raw observations without expected-answer access |
| Existing independent `judgment.canonical_cases` / `judgment.compare`, selecting severity rows only | exit 0; 960 correct, 0 incorrect, 0 unassessed; frozen case/actual row sets match |
| `cargo test --offline --locked -p pqc_scan_core --lib risk::tests` | exit 0; five tests, all levels, saturation and gate boundaries |
| `cargo test --offline --locked -p pqc_scan_core --lib normalization_preserves_declared_confidence_and_metadata_exactly` | exit 0; one test, exact confidence bits |
| `PYTHONPATH=tests/quality python3 -m unittest discover -s tests/quality -p test_judgment.py -v` | exit 0; eight comparator controls |
| `cargo test --offline --locked -p pqc_scan_cli --test rule_validation` | exit 0; one actual CLI operational-error/output-prevention test |
| Final audit/population/snapshots/references/links/index/source/task checks and `git diff --check`, `git diff --cached --check` | exit 0; details in FINAL-SOURCE.json and final-validation.log |

The 960 cases cover eight category spellings, four Risk values, all five base
levels, confidence 0.4/0.45/0.8 and ordinary/private_key evidence. The current risk
tests additionally check 0.449. This is conformance to preserved legacy policy,
not new intended-severity truth. The source-exposed task056 author, correlated
synthetic cases, non-blind dataset and frozen compatibility disposition remain
disclosed. Other adapter judgments were emitted but not claimed compared here.

Historical task028/056 reviews and measurements are reused only within the
applicability stated in REUSE.json. Six of eight rule files match the older
threshold study; the later crypto_api/tls changes are covered by current bindings
and rerun tests. Runtime risk functions match the prior source; later tests alter
the whole risk.rs hash, so whole-file equality is not claimed. Task056's five
original guidance findings have a separate completed repair record; they are not
reported as newly outstanding.

Verification is scoped to the original audit requirements. No workspace-wide
build/test/clippy/fmt, feature-matrix probe, browser check, population-accuracy
trial, calibration study or release test was run or claimed. No executable or
presentation change requires those broader change-category checks here.

## Review and completion disposition

Under [REVIEWS.md](../../docs/agent/REVIEWS.md#required-review-stages), this is
research/prose/development audit data without changes to maintained executable
behavior, tests, scripts, product storage or runnable examples. Both independent
review stages are exempt. Temporary local measurement/negative-check commands
are retained as raw evidence transcripts, not shipped implementation scripts.
No reviewer identities were allocated; historical reviewers are not claimed to
have reviewed these new audit conclusions. The main agent performed the complete
inspection, analysis, tests, combined-diff assessment and acceptance reconciliation.

The lifecycle records pending → planned → in-progress → review → done after final
verification. Every original audit deliverable and criterion is covered; unknown
empirical quality and pending behavioral repairs are findings within this
completed audit, not waived or deferred mandatory audit work.

## Remaining decisions and next eligible work

The four retained false-positive controls, legacy technical-tag/assurance gaps and
unknown numeric derivations remain product limitations. C065-01/03/04/05 prepare
concrete D5 inputs; preserve behavior until an applicable decision is recorded.
C065-02 is resolved for the composed audit only. None of these proposals creates
or starts another task.

Task 066 is the next eligible registered task because its sole dependency, 065,
is now satisfied. It still requires a separate execution instruction. It prepares
the joint feasibility/contracts and necessary D1-D6 decisions, including D5
classification scope and explicit SSH input selection. No decision enum, schema,
policy date, study participant, optional score or support expansion is approved by
this audit. Tasks 066-072 remain pending and untouched.
