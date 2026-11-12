# v0.2 Release Closeout Implementation Plan

> Main agent executes the approved four closeout groups. Exactly one implementation reviewer after implementation and one distinct test reviewer after final tests; reuse each identity. No delegated implementation.

**Goal:** Close the current inspection repairs as a v0.2.0 candidate, commit by content, push, verify CI, open and merge one PR to develop, then verify merge-commit CI.

**Architecture:** Keep scanner/schema/identity and accepted limits. Fix stale distribution assertions, manually review exact self-scan observations, rename the release roadmap and update current release intent. Static scanning is the formal scope; probes remain experimental and their notice-review flags remain truthful. Public tagging/publication is a later release action.

**Tech Stack:** Rust1.93.1, Python standard library, Node22, GitHub Actions/CLI.

**Spec:** User-approved four findings and v0.2 retargeting; target/v1-readiness-audit-20261112/ROADMAP-READINESS.json.

## Constraints and review focus
- Preserve the current branch and all previous implementation/approved scope;014/016 cancelled,023 baseline criteria retained.
- Use 0.2.0 for product version; do not rename versioned report schemas, TLS1.0 literals, API format versions or immutable historical measurements.
- Rename docs/roadmap/OSS-ROADMAP-v0.2.md to OSS-ROADMAP-v0.2.md; resolve all active file links and mark historical1.0 planning references as superseded.
- Classify each retained result from its actual file/line/content. Only exact reviewed test/reference matches may be omitted; dependency inventory/unexpected production observations remain.
- Assert exact positive rule IDs/locations and negative control, with an actual clean archive install. Unknown and changed observations must stay actionable.
- Bind final tests/performance/source/package to actual source; performance2x is an investigation signal, not a new numeric fail threshold.
- Prepare provenance/authenticity verification without representing unsigned local snapshots as official releases or clearing unresolved licenses.

## Execution
- [ ] Update install acceptance and meaningful regression controls; demonstrate existing failure then corrected clean installs.
- [ ] Version0.2.0, roadmap rename, current-scope guides/security/release notes and preparation workflow; preserve historical evidence.
- [ ] Build final scanner; individually review self-scan results and update finite registry/ledger/tests.
- [ ] Implementation review combined changes; fix findings using the same reviewer.
- [ ] Final fmt/build/workspace/allfeature/clippy/Node/Python/schema/baseline/protocol and chosen packaging tests; verify unchanged previous OSS content evidence where only version changes.
- [ ] Separate test review; fix/reverify using same reviewer.
- [ ] Commit by content, push current branch, verify exact latest commit hosted checks and hosted distribution preparation.
- [ ] Create PR to develop with full scope/evidence, merge, verify latest origin/develop CI; record links/SHAs and remaining publication-only actions.
