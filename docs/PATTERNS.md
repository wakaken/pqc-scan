# Patterns

This document captures recurring repository-specific design and implementation patterns.
Use it to understand how this codebase is typically extended.
For normative rules and contributor expectations, see `docs/BEST-PRACTICES.md`.

---

## Thin CLI Entrypoints

Pattern:
- keep `main` focused on argument parsing, top-level error reporting, and process exit
- move command behavior into helpers that return explicit results or exit status values

Why it works here:
- CLI behavior stays compatible while remaining easy to test
- stdout and stderr can be asserted without spawning a subprocess

Current example:
- `crates/pqc_scan_cli/src/main.rs`

---

## Pipeline-Centered Scanning

Pattern:
- keep orchestration in the core pipeline
- let repository walking, detection, normalization, recommendation enrichment, and reporting remain distinct stages

Why it works here:
- security-sensitive logic stays inspectable
- new detection capabilities can be added without rewriting the whole scan flow

Current example:
- `crates/pqc_scan_core/src/pipeline.rs`
- `crates/pqc_scan_core/src/walker.rs`

---

## Detector Extension Pattern

Pattern:
- implement the shared detector trait
- keep detector-specific parsing and matching local to the detector crate
- register new detectors through the default detector list rather than wiring them ad hoc
- add targeted tests for both detection hits and sensitive-output handling when a detector touches key or certificate material

Why it works here:
- new detector kinds remain isolated
- the scan pipeline can treat detectors uniformly
- regressions in masking or fallback behavior are caught close to the detector implementation

Current example:
- `crates/pqc_scan_detectors/src/lib.rs`
- `crates/pqc_scan_detectors/src/regex_detector.rs`
- `crates/pqc_scan_detectors/src/tree_sitter_detector.rs`

---

## Rule-Driven Matching

Pattern:
- encode detection logic in YAML rules where possible
- compile and validate rule data at load time
- keep rule IDs unique and update rule reference docs with rule changes

Why it works here:
- detection coverage can evolve without invasive code changes
- invalid rules fail early instead of producing silent scan drift

Current example:
- `crates/pqc_scan_rules/src/lib.rs`
- `rules/default/*.yml`
- `docs/rules-reference.md`

---

## Report Writer Extension Pattern

Pattern:
- add new output formats in the reporting crate
- keep sanitization centralized before format-specific rendering
- preserve existing output paths and field stability unless a planned compatibility change is approved

Why it works here:
- sensitive data handling remains consistent across formats
- downstream tooling can rely on stable report contracts

Current example:
- `crates/pqc_scan_report/src/lib.rs`
- `crates/pqc_scan_report/src/json_report.rs`
- `crates/pqc_scan_report/src/sarif_report.rs`

---

## Report Contract Test Pattern

Pattern:
- keep deterministic writer-level tests next to JSON, HTML, and Markdown report generators
- assert grouped finding rendering, format-specific escaping, stable serialized field names, recommended-action sections, and masked evidence output
- cover inventory-oriented outputs such as `cbom.json` and `dependency-sbom.json` when report contracts are extended
- use synthetic findings with masked placeholders instead of raw sensitive fixtures

Why it works here:
- report outputs are compatibility surfaces and can regress during refactors even when core finding data stays correct
- escaping and masking behavior are security-sensitive and should be checked in the final rendered format, not only in lower-level helpers

Current example:
- `crates/pqc_scan_report/src/json_report.rs`
- `crates/pqc_scan_report/src/html_report.rs`
- `crates/pqc_scan_report/src/markdown_report.rs`

---

## Documentation Synchronization Pattern

Pattern:
- when renaming docs or changing workflow terms, update all agent-facing references in the same patch
- keep task lifecycle terminology aligned across `AGENTS.md`, `docs/TASK-LIFECYCLE.md`, prompts, and task files

Why it works here:
- agents depend on these files as operational inputs
- inconsistent wording causes workflow mistakes and duplicate repair work

Current example:
- `AGENTS.md`
- `docs/TASK-LIFECYCLE.md`
- `prompts/EVALUATOR.md`

---

## Task Queue Pattern

Pattern:
- keep tasks small and role-driven
- let Planner move tasks to `planned`, Executor to `in-progress`, Reviewer review the implementation, and Evaluator close the loop with `done`
- use `blocked` when progress depends on unavailable validation or human decisions

Why it works here:
- responsibilities stay auditable
- unfinished work remains visible without overloading normal completion states
