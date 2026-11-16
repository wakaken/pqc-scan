# Task 063 authorized follow-up

The user authorized the proposed sequence: compatible updates for the three
patched dependencies, YAML replacement investigation, then publication notice and
source-provenance investigation. Present choices and effects wherever behavior
changes or unresolved conditions remain. No publication or release is authorized.
Task 051 remains a completed historical evaluation; this task records remediation
without rewriting its measurements or consuming another roadmap task.

## Acceptance and approach

1. Update crossbeam-epoch 0.9.18 to 0.9.20 and anyhow 1.0.102 to 1.0.103 within
   existing semver requirements. Update the explicit tokio 1.49.0 pin to 1.51.6,
   checking russh requirements and protocol behavior. Prefer minimal changes to
   broad dependency refreshes. Retain resolver failures and unexpected new findings.
2. Map serde_yaml use, then compare maintained replacements using actual rule,
   configuration and dependency-parsing inputs plus malformed/edge cases. A fork
   retaining unsafe-libyaml does not remove the maintenance finding. Keep the
   canonical parser unchanged if compatibility cannot be established; present the
   smallest concrete alternatives and their consequences before dependent work.
3. Acquire public exact-version upstream sources and complete applicable notice
   texts; verify identities/hashes. Compare registry archives to their recorded
   VCS source, distinguishing Cargo packaging normalization from source changes.
   Do not infer legal clearance or clear a flag merely because a file exists.
4. Run default and affected optional-feature build/test/clippy/fmt checks,
   protocol regressions, distribution checks and fresh offline dependency review.
   Reuse existing bounded container infrastructure and source-bound evidence.
   Network-capable preparation contains manifests/stubs or trusted tool acquisition
   only; execute target builds/tests offline with read-only source and bounded
   writable scratch. No paid service or remote execution is required.
5. Reuse implementation reviewer `/root/dependency051_implementation_review` and
   test reviewer `/root/dependency051_test_review` for the combined follow-up;
   neither may delegate. Record final-source identities and unresolved decisions.

## Compatibility and completion boundaries

Preserve CLI/report/inspection/rule semantics and exact dependency-notice bindings.
Upgrading a crypto-related library must not silently change crypto policy. If an
update requires such a change, stop that affected path and continue independent
investigation. No risk exception or release decision is pre-approved.

Expected cost: dependency downloads and several bounded build/test runs, typically
tens of minutes. YAML feasibility/source comparisons may expose decisions requiring
maintainer input. Document evidence before asking; keep unresolved required work
unfinished. No new unrelated task is authorized by an empty queue or a finding.
