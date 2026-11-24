# Task 066 prerequisite specification and develop integration results

Status: complete for the original prerequisite scope after both independent
reviews and final-source verification.

Date: 2026-11-18. Scope: latest-origin/develop integration followed by the eight
original task066 prerequisite clauses. Product implementation belongs to subsequent
authorized tasks; this record does not claim any R2/R3/R4/R5/X feature implemented.

## Source and preservation

Branch remains `codex/v04-roadmap-20261118` in
`/Users/waka/.codex/worktrees/v04-roadmap-20261118/pqc-scan`.
`git fetch origin develop` followed by `git merge --ff-only origin/develop`
advanced HEAD from `3f2bc6e6209e1f2813caac8077c8d7a7198832cc` to
`48276e48f5c6d4eadafb2a3aa9d4c1a54808f1ab` without conflicts or a new merge commit.
The incoming69 paths include report navigation/language/theme/rule companion,
local `serve` UI, packaging/test evidence and v0.3 release documentation.

All47 pre-existing changed file contents and13 original staged blobs were backed
up externally, temporarily stashed and restored with `--index`, then verified
byte-identical before task066 edits. Only the explicitly selected task066 definition
is subsequently updated. Task065 artifacts and all original index blobs remain
preserved; tasks067-072 remain pending and untouched. The temporary task stash was
removed after successful verification; the external backup is retained. No commit,
push, PR, tag, publication, external message or task067 execution is included.

[Integration receipt](evidence/DEVELOP-INTEGRATION.json),
[initial source manifest](evidence/SOURCE.json),
[final source applicability](evidence/SOURCE-APPLICABILITY.json),
[reuse reconciliation](evidence/REUSE.json)
and [final-source verification](evidence/FINAL-SOURCE.json) identify the actual
uncommitted bytes. The earlier roadmap baseline and task065 results remain
historical records, not rewritten as measurements of the new report source.

## Selected contracts and deliverables

- [Execution plan](TASK-066-PLAN.md).
- [Saved-format feasibility](FEASIBILITY.md): narrow existing-format adapters with
  a small private fact projection; no combined-inventory migration, new crate,
  replacement evaluator, service or generic policy language.
- [Adopted architecture/field/limit/binding/time/output contract](../../docs/architecture/MIGRATION-ASSESSMENT-V1.md):
  separate opt-in assess/context/report dispatch, immutable raw-byte digests and
  target manifest, per-target system claims and explicit provenance/freshness,
  P1-P8 urgency/information/readiness/action semantics, fixed errors and safe output.
- [Technical clause/edition/errata mapping](FACT-SOURCE-MAPPING.md), extended from
  the existing standards register and task056. New exact TLS/SSH component facts
  are a separately selected catalog; old v1 behavior/0.45/rules remain unchanged.
- [Actual D1-D5 decision package](DECISIONS.md) and
  [original maintainer replies](evidence/MAINTAINER-DECISIONS.json).
- [65 frozen semantic expectation cases](ACCEPTANCE-CASES.json), covering all
  A01-A18 and invalid/paired/boundary examples;
  [field/requirement/manual-case reconciliation](COVERAGE.md).
- [Adopted evaluation protocol](EVALUATION-PROTOCOL.md) and
  [freeze receipt](evidence/FREEZE.json): decision table versus ordinal matrix,
  explicit numeric/MCDA screening, roles/denominators/lineage splits/adjudication.

D3 actually selects saved Inventory, TLS, SSH offers and SSH KEX v1. Reading those
records does not enable a probe, provider, discovery process or network package
publication. The default static binary is the intended offline-reader configuration.
D4 selects separate JSON and English/Japanese Markdown support, with no new HTML
or serve integration. The newly imported v0.3 HTML/serve behavior is the baseline.

D6 is an approved resource **plan**, not secured people/data:24 vignettes/12 pairs,
two practitioners rating both methods, independent technical adjudication and
policy authority,8 tuning/16 held-out cases and bounded sessions. Actual resources
must be secured under task071 before its observations; no recruitment/contact or
spending is authorized here. D7 score adoption remains a separate later decision;
no numeric product score or weights were adopted or validated.

## Original acceptance reconciliation

| Original clause | Evidence and disposition |
|---|---|
| Compare adapters/internal projection/integrated migration and select minimum sufficient approach | FEASIBILITY compares actual types, schemas, validation and dependency direction. Serialize-only runtime types need saved-reader DTO/semantic validation; no provider/core dependency cycle or replacement architecture is required. D1 adopts narrow adapters |
| Specify facts/capture gaps/clause-edition-errata mapping and v1 dispatch before coding | FACT-SOURCE-MAPPING F01-F14, current schema/type/source hashes and acquired official bodies/errata. TLS parameter-encoding/CertificateVerify gaps, static references and unknown components remain explicit. New dispatch cannot mutate old helper outcomes |
| Resolve blocking D1-D5 with actual maintainer decisions | The maintainer explicitly adopted the recommended D1/D2/D4/D5 package, selected all four D3 formats and adopted the study plan. Exact replies and exclusions are recorded; task selection alone was not treated as approval |
| Define context/bindings/precedence/clock/minimum inputs/separate axes/actions/offline workflow | Adopted contract provides exact root/claim/output shapes, target identity, one-system ownership, defaults→system→target precedence, validity intervals, explicit as_of and checked deadline/lead boundary, P1-P8 and total readiness/action precedence |
| Specify Inventory/TLS plus explicit SSH decision and unsupported behavior | D3/all-four formats recorded; adapters preserve offers versus selection, reported collector verification versus new verification, component roles and valid partial TLS facts. Unknown/unselected format/version rejects instead of negative evidence |
| Freeze A01-A18 technical/policy expectations, invalid/boundary cases and exposure disclosure before new implementation observations |65 cases,18 groups, structural/base/source checks and hand-derived critical boundaries in PRE-FREEZE-CHECK/COVERAGE. Expectations frozen before any new v0.4 code/results; source-exposed local-policy truth is not blind or practitioner truth |
| Freeze R5 methods/measures/roles/recruitment/resources/adjudication/splits; retain D6 needs | EVALUATION-PROTOCOL and FREEZE define M1/M2, numeric/MCDA screening without invented weights, distinct technical/conformance/usefulness claims, denominators, effort/disagreement, proposed24/2 resource bounds and downstream independent-data freeze; no actual participants claimed |
| Retain later D7 and distinguish prerequisite from product acceptance | No score adoption, formula, new product schema file or decision implementation. Product acceptance remains with primary owners in TASKS.md; only this prerequisite can close |

## Actual validation

Pinned Rust1.93.1, offline locked Cargo. [Commands/results/binary hashes](evidence/COMMANDS.json)
retain current source/feature configuration and raw logs.

| Check | Actual result |
|---|---|
| `cargo test --offline --locked --workspace` | exit0;620 passed,0 failed,3 existing ignored supervisor workers |
| `cargo build --offline --locked --workspace` | exit0 |
| `cargo clippy --offline --locked --workspace --all-targets -- -D warnings` | exit0 |
| `cargo fmt --all -- --check` | exit0; no formatting mutation |
| Separate `CARGO_TARGET_DIR=target/task066-all-features` all-target/all-feature strict workspace clippy | exit0; no shared default-binary replacement |
| Node report-ui/compact/self-scan-notification tests | exit0;27 passed |
| Distribution Python unittest discovery | exit0;25 passed; expected invalid-image negative and post-summary archive JSON retained |
| Real CLI examples report generation | exit0;55 analyzed files,322 findings; generated Inventory/assessment v1 remain existing formats |
| Browser integration checks | Japanese/English, light/dark,1280×720 and390×844, no horizontal overflow at measured widths, search/summary totals, Tab focus, rule explanation/back links, HTTP-server-stopped loaded UI operation; actual serve folder selection→scan→report workflow observed |
| Contract/case/source/reference/link/task/index/dirty-source checks | exit0 on final bytes; recorded in FINAL-SOURCE.json and final-validation.log |

The three ignored tests are worker entrypoints intended for separately bounded
supervisor campaigns, not silent failures or newly skipped tests. No protocol
interoperability matrix, remote CI, new v0.4 decision implementation, numerical
calibration or practitioner study was run/claimed. All-feature clippy is compilation/
lint evidence, not network-protocol execution. Imported report tests do not validate
future assessment usefulness or future report rendering.

The browser's [observation record](evidence/BROWSER-OBSERVATIONS.json) and captures
bind actual viewports and interactions. Its version metadata is not exposed by the
supported browser API; an attempted navigator read failed in the inspection scope,
not in the product. The dedicated public-example report viewer remains available
at `http://127.0.0.1:52875/report.html?lang=ja` with the visible report tab preserved.
The additional serve E2E test session was stopped and its session capability was
redacted from retained logs. No real repository/private source was served.

Primary sources: exact FIPS203/204/205 PDF parameter pages were text-extracted and
visually checked; RFC9846/9954/10024/10042 texts and current errata searches were
acquired with hashes. Old endpoint/redirect failures are retained before the new
errata-host approach succeeded. Eight new errata records were dispositioned;
RFC10024 search returned none in the acquired snapshot. Existing14 FIPS potential
updates and43 RFC records remain separately scoped historical evidence, not a
claim that reported/potential corrections were enacted.

A temporary evidence-log collector twice failed because it assumed unittest OK
was the final log line; an archive test emits JSON afterward. The corrected
collector recognizes an anchored OK line, preserves the raw log and all actual
exit0 outcomes. See [recording correction](evidence/RECORDING-CORRECTION.json).
No product source or test was changed or rerun to hide that recording error.

## Independent review and freeze

The two reviewer identities, source scopes, findings and decisions are in
[TASK-066-REVIEWS.md](TASK-066-REVIEWS.md). Implementation review accepted after
same-identity correction of three P2 specification findings: valid TLS partial
versus failed verification, a valid stale claim interval for A15, and known-false
readiness/action precedence. Four new counterexamples raised the draft61 cases
to65. Those are specification corrections within the adopted decision scope;
no executable repair was enacted.

The freeze receipt binds the approved contract, case expectations and protocol
before any new v0.4 implementation observations. The only post-implementation-review
case change is freeze-status metadata; the65 expectations are byte-equivalent as
structured content, with pre/post hashes and applicability recorded. Test review identified one P2 evidence-labeling ambiguity: the initial source
snapshot retained the old standards-register hash after its authorized appendix.
The correction preserves that historical hash, explicitly labels the initial
capture and binds all current source entries through SOURCE-APPLICABILITY.json.
Current file/FREEZE/FINAL-SOURCE hashes already agree. No product/expectation
changed, and only static checks need rerunning. The same test reviewer accepted the correction, independently checking250 source
entries and8 frozen artifacts. Both stages are accepted; final source/artifact
reconciliation includes the report-server bookkeeping and final task state.

## Remaining work and next eligible task

The prerequisite specifies future work; it does not remove task065's four retained
configuration false positives or supply empirical numeric rationale. These remain
explicit limitations, with no unapproved rule repair. Actual reader/validator/
projection/decision/schema/rendering implementation and its executable boundary/
privacy/no-network/v1 regressions belong to tasks067-070. The practitioner study
and independent data/resources remain task071 requirements; numerical product
adoption remains later D7.

Task066's required reviews and final verification passed; task067 is the next
eligible task. Tasks067-072 remain pending and were not selected or executed.
No product/release/support acceptance is inferred from completing this prerequisite.
