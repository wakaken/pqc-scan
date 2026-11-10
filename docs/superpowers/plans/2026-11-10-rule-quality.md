# Rule Quality Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline. Independent design, source/classification review and Evaluator acceptance.

**Goal:** Reject invalid rule metadata and incomplete loads; document every built-in rule's provenance without detection changes.
**Architecture:** Existing rule loader plus private iterator seam, source-f64 validation and a development-only JSON companion.
**Tech Stack:** Rust1.93.1, existing serde/serde_yaml/regex/walkdir; existing serde_json/sha2 as dev dependencies only.
**Spec:** ../specs/2026-11-10-rule-quality-design.md

## Global Constraints

No CLI/report format or valid custom-scope change. No rules/default changes, regex tuning, new DSL, runtime metadata download or approved numeric quality budget. Same number-free branch; local commit only.

- [x] Independent design check; record actual119 built-ins and scope compatibility.
- [x] Red nonblank ID/source-f64 confidence/root+late traversal failure tests and compatibility controls.
- [x] Minimal loader implementation; green tests; actual custom-scope and CLI error/exit checks.
- [x] Complete source-bound119-row companion with purpose/algorithm/quantum/classical review, references/limits and honest positive/negative pattern targets; test companion consistency.
- [x] Both READMEs, rules reference/contract, current architecture and verification record.
- [x] Offline pinned build/fmt/workspace/clippy/Node/Python, shadow comparisons/frozen corpus/CI source binding audit.
- [x] Independent source+matrix review; resolve findings.
- [x] Final Evaluator acceptance.

Delivery: one local task028 commit on the current number-free branch; no push/PR.
