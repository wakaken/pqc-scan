# Inspection format support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Correct nine example incomplete inspections with supported parsers and honest remaining limitations.

**Architecture:** Extend the existing detectors and detailed outcomes. Keep transport-free local parsing, use existing certificate attribute logic and typed dependency provenance; no public storage or CLI changes.

**Tech Stack:** Rust1.93.1, tree-sitter0.26, pinned TS/TSX grammar0.23.2, existing ASN.1/X.509 and serde parsers; bounded XML parser if required.

**Spec:** docs/superpowers/specs/2026-11-10-inspection-format-support.md

## Global Constraints

No private keys, decryption, password CLI, runtime network, package-manager/script execution. Keep accepted-hit/input/work limits, existing report shapes and identity algorithm. Unsupported/dynamic input must not become Complete simply to remove warnings.

## Review Focus

- Mixed public certs/encrypted P12 content: retain supported certs and incomplete status; no MAC/trust claim.
- Malformed/oversized/deep/nested unknown ASN.1 and unsupported bags: bounded and safe, no payload diagnostics.
- TSX/type-only/malformed/comment/namesake source: grammar improvement without hidden rule changes.
- Dynamic/include/property/workspace/lock conflicts: no fabricated resolution or successful partial parsing.
- SBOM display name vs encoded purl identity: strict identity validation with source-claim provenance.

## Work units

- [ ] TypeScript owner: failing actual TS/TSX tests, dedicated grammars, existing occurrence regressions; no manifest edits (parent owns dependencies).
- [ ] Dependency owner: failing nine-format/subset/provenance tests, bounded parser modules and SBOM identity handling, negative/unsupported cases; no manifest edits.
- [ ] Certificate owner: failing PFX certificate/mixed/encrypted/malformed/limit tests, unencrypted cert extraction, existing certificate semantics regressions; no manifests/examples edits.
- [ ] Parent: dependency/lock updates and reviewed sample generation, tests/docs integration, red/green evidence, full workspace/fmt/clippy/Node/Python, old/new example/corpus/performance/CI comparisons, independent reviews and local commits. Do not push/PR/merge new feature without explicit request.

Each owner self-reviews their files; a separate reviewer checks source correctness and scope before final acceptance. Tests should fail before each product change. Store raw artifacts under ignored target/format-support.
