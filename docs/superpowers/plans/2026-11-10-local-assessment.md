# Task 026 local assessment implementation plan

> Use superpowers:executing-plans inline and test-driven-development;
> independent reviewers check the design and final change.

**Goal:** Add pure multi-axis assessment/local advisory policy with explicit offline CLI.
**Architecture:** assessment/policy core modules, separate assessment-v1 artifact;
legacy pipeline, inventory-v1 contract and recommendation output remain unchanged.
**Tech:** Rust1.93.1/current serde/chrono/sha2/jsonschema; no dependency changes.
**Spec:** ../specs/2026-11-10-local-assessment-design.md.

- [x] Independent design review and sources: versioned profile applicability;
  strict inventory-v1 compatibility and conservative quantum/classical semantics.
- [x] Red policy/catalog/context table tests; pure modules with fixed safe vocab,
  deterministic linked assessments and purpose-specific candidate prerequisites.
- [x] Red actual CLI/schema/reference tests; explicit profile/context selection,
  separate report writer/schema. Keep all existing output/exit defaults unchanged.
- [x] Update both READMEs, current contract/standards register, exact local evidence;
  old/new semantic shadows, frozen corpus and CI expectation bindings.
- [x] Independent final review, pinned offline build/fmt/test/clippy, Node/Python;
  evaluator acceptance and local commit, no publication.

Review focus: false clean pass; scope ambiguity; KEM/signature substitution; hybrid
combiner inference; symmetric/hash attack confusion; trusting dependency presence;
unsafe context/unknown strings; schema/reference drift; changed legacy severity.

## Execution evidence

[Validation and limitations](../../../tasks/026/TASK-026-LOCAL-ASSESSMENT.md) and
[independent reviews](../../../tasks/026/TASK-026-REVIEW.md) bind the final source,
inputs, command outcomes, compatibility and still-unapproved numeric budgets.
