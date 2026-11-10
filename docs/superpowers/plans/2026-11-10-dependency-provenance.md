# Task 027 implementation plan

> Execute inline with superpowers:executing-plans and test-driven-development;
> independent design/source review plus evaluator acceptance.

**Goal:** Separate constraints/resolution/provenance and expose honest parse outcomes.
**Architecture:** Existing detector with focused bounded structured-parser helper;
existing core dependency facts and assertion identity; unchanged legacy field shapes.
**Tech:** Rust1.93.1, existing serde/serde_json/serde_yaml; pinned TOML parse dependency.
**Spec:** ../specs/2026-11-10-dependency-provenance-design.md.

- [x] Review source formats and additive parser/identity design independently.
- [x] Red declaration/lock/unknown/outcome/canary tests; bounded typed parsing and
  one shared detector inspection; legacy array and rule compatibility checks.
- [x] Native typed facts and source-aware IDs without schema extension; explicit
  legacy projection losses and safe purl emission.
- [x] Ecosystem/empty/malformed/partial/unsupported/privacy/identity regressions;
  update affected existing expectations for intentional semantic corrections.
- [x] Both READMEs, current contracts, sources and evidence; frozen study, CI
  reviewed source-only rebind and independent source/Evaluator acceptance.
- [x] Final pinned build/fmt/test/clippy/Node/Python.
- [x] Evaluator acceptance.

Delivery: one local task027 commit on the current branch; no push/PR/merge.
