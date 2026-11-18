# Task 065 execution plan

Selected by the maintainer on 2026-11-18, in the existing worktree and branch
`codex/v04-roadmap-20261118`, HEAD
`3f2bc6e6209e1f2813caac8077c8d7a7198832cc`. Preserve the thirteen staged
planning/definition additions and their index bytes. No commit, push or task 066
execution is included.

## Outcome and acceptance

Complete the original R1-01 through R1-04 clauses in task.yaml and the v0.4
requirements. Audit every direct built-in against the maintained companion;
explain supported facts, unknown parameters and rationale gaps; characterize
promotion/custom-pack boundaries; provide bounded correction proposals and impacts.
Unknown empirical justification is an audit finding, not an omitted audit.

Dependencies 028 and 056 are done. Reconcile their applicability through current
source hashes, maintained companion and later repair records; preserve old results.
Do not infer current results merely from their done states.

## Approach and compatibility

Reuse the companion unchanged and add a source-bound task-local supplement. This
keeps historical evidence and runtime contracts intact. Rewriting the companion
or implementing corrections is unnecessary to complete this research task.
No rules, Risk, Severity, confidence, 0.45 threshold, catalog, schemas, CLI or
report behavior will change. D5 proposals remain unapproved; D1-D7 decisions and
implementation belong to subsequent authorized work. No external probe is needed.

## Steps and checks

1. Record HEAD, index/worktree changes and hashes; enumerate actual YAML rules.
2. Inspect every companion row and source predicate; add per-rule parameters,
   base-value rationale/explicit unknown, promotion disposition and current FP/FN
   status with source references. Reconcile historical repairs.
3. Check loader/detector custom-scope contracts and promotion gates. Reuse frozen
   judgment expectations with disclosed authorship and compatibility limitations;
   run current targeted Rust adapters and independent comparison as needed.
4. Verify unique/exact population, snapshots, hashes, references and deliberate
   duplicate/missing/extra/stale-source rejection. Use existing Rust pattern and
   loader tests; no replacement validation framework or committed script.
5. Write correction impact analysis, R1 acceptance reconciliation, commands/exits,
   reuse limits and next eligible task. Check final diff, links, preserved index
   and final-source hashes before lifecycle closure.

## Reviews, risks and human decisions

This is a research/prose/data-record change with no executable changes or examples;
under docs/agent/REVIEWS.md it needs neither independent implementation nor test
review. Running existing tests does not change that classification. If executable
changes become necessary, reassess before editing and apply both required stages.
Do not delegate investigation, evaluation or documentation.

Primary risks: treating historical prose as current fact, pattern controls as
end-to-end truth, heuristic confidence as probability, or preservation as calibration.
Keep these limits explicit per rule. Any correction affecting rule/crypto semantics
or storage contracts requires concrete impact analysis and human review before
implementation; this task only prepares proposals.
