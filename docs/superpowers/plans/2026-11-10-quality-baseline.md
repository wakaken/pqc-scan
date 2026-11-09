# Task 023 quality baseline implementation plan

> Execute inline with superpowers:executing-plans; two independent source-only
> labelers and a fresh whole-change reviewer provide separate checks.

**Goal:** Establish a reviewed, reproducible local baseline without changing scanner behavior.
**Architecture:** Python standard-library development harness invokes the existing CLI, scores a pinned source-only corpus and records raw reports/diagnostics/resource samples outside inputs. Existing examples and public certificates remain immutable.
**Tech Stack:** Rust 1.93.1 scanner, Python 3.12 development tools, existing Node CI helper.
**Spec:** tasks/023-quality-baseline.yaml; tests/quality/PROTOCOL.md; docs/quality/QUALITY-GATES.md.

## Global constraints and design

Stay on codex/task-022-sarif-location at user request; start HEAD 35d988e03629cb15b17a94af81b1e74266a344ff. One task-specific commit; do not amend task022 or publish. No runtime/Rust model/rule/CLI changes. All fixture code is data, never executed. Synthetic/public MIT bytes only, no keys/endpoints. Preserve full raw self-scan; register only explicitly reviewed new test-input observations if needed.

Use a separate developer harness rather than extend public report schemas or turn all existing hits into truth. Family-specific nominated rule IDs and occurrence spans define evaluation; independent labels bind exact corpus bytes before measurement. Lineage splits are structural, not random file splits. All unmatched scoped predictions count as FP; unresolved/unsupported labels are separate. Holdout is frozen before measuring; no rule tuning here. Resource measurements are per child (wait4), including launch/report writing, and cold-cache is explicitly unmeasured. Approved numeric budgets remain a maintainer decision on concrete measurements; commit a proposal without claiming approval or enforcing it.

## Task 023 steps

- [x] Review existing architecture/tests/quality policy and write protocol with truth/split/scoring/coverage/measurement semantics.
- [x] Reuse seven-language examples and public certificate manifest; add source/config/dependency/malformed/unsupported/key-marker controls, immutable source digests and lineage split.
- [x] Two independent reviewers label all units from source only; adjudicate differences before running scanner.
- [x] Add evaluator sanity tests first; observe missing implementation failure. Implement one-to-one scoring (duplicate FP, missed occurrence FN, ambiguity/exclusion accounting), denominator nulls, canonical/order digests, safe manifest materialization, timeout/resources and coverage equalities.
- [x] Run real CLI accuracy/repeatability and small/medium/large/pathological repeated benchmarks, recording commands/binary/source/rules/catalog/fixtures/versions/environment/config, errors, canary survival, diagnostics and distributions.
- [x] Add CI evaluator and semantic smoke/repeatability artifact steps; do not enforce unapproved accuracy/performance budgets. Update both READMEs and quality policy.
- [x] Compare old/new runtime outputs and self-scan projection, review precise new intentional fixtures, preserve production findings and output contracts.
- [x] Independent final review; full Rust/Python/Node/build/fmt/clippy checks; record baseline, limitations and measured budget proposal; commit locally.

## Review focus

One-to-one interval matching and duplicates; uncertainty cannot inflate denominators;
lineage and digest drift cannot silently relabel truth; ignored ancestor directories
must not hide inputs; child RSS must not be cumulative/lifetime maximum. Tests cover
these boundaries. All precision/recall are finite-corpus descriptive counts, never
an estimate of global product accuracy. Maintainer measurement/budget review is
reported pending when not actually performed.

## Ledger

Initial checkout clean. User explicitly requested same-branch execution and local
commit after implementation/tests/docs/review. The chosen implementation leaves
production code untouched; protocol and corpus are concrete and reviewable.

Execution outcome: development implementation, source-only labeling/adjudication,
measurement, CI/documentation, regression checks and independent technical review
are complete. Source-only lineage audit invalidated the initial held-out group;
all derived templates were moved together into tuning, exploratory evidence was
retained, and fresh independent project data was frozen/labeled before measuring.
Accepted final artifact: target/task023/baseline-complete and committed BASELINE.json.
Review fixes: narrow clock normalization, order-independent scoring, complete JS/CSS
source bindings; all had regression coverage. Final checks: Rust145/Python20/Node22,
offline build/fmt/clippy and production path immutability all passed. The remaining
maintainer protocol/budget decision keeps task023 in review; no approved numeric
budgets are invented. User-authorized endpoint is local commit, no publication.
