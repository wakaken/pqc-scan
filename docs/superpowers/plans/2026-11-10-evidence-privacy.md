# Evidence Privacy Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline; independent whole-diff privacy review and final Evaluator acceptance.

**Goal:** Verify bounded supported secrets cannot survive source-to-report collection while retaining technical facts and IDs.
**Architecture:** Shared core text/result policy, context metadata without payloads, post-identity/association legacy redaction, writer defense in depth.
**Tech Stack:** Rust1.93.1; existing regex/serde/sha2; no new package versions or runtime services.
**Spec:** ../specs/2026-11-10-evidence-privacy-design.md

## Global Constraints

Same branch. Existing CLI/model/schema and valid technical facts stay stable. No real keys, decryption, detector/rule coverage expansion, universal-secret claim, push or PR.

- [ ] Trace fields and independently review bounded design/data-flow.
- [ ] Write actual failing all-format/core/CLI canary and benign-fact/ID controls.
- [ ] Implement shared known-syntax redaction and source context; redact after IDs/native links, synchronize CBOM; green targeted tests.
- [ ] Extend formats/URLs/markup/metadata/actions/diagnostics/stderr and multilingual/schema controls; preserve stable IDs/facts.
- [ ] Update both READMEs, architecture/contract and proof/limitations report.
- [ ] Pinned full checks, saved-binary compatibility, frozen corpus and CI source expectations with reviewed synthetic controls only.
- [ ] Resolve independent source review; Evaluator acceptance; one local commit only.
