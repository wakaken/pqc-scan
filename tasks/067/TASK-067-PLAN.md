# Task 067 implementation plan

Selected by the maintainer on 2026-11-18. Baseline: c7e76b59da44ffdc8a60da86efd5393e70a039e3,
clean `codex/v04-roadmap-20261118` in the existing worktree. Task066 is done;
its adopted D1-D5 contract, frozen 65 specification cases and source mappings
remain authoritative. No scope or acceptance changes are selected.

## Outcome and acceptance

Implement R2-01..05: bounded offline ingestion of Inventory/TLS/SSH offers/SSH KEX
v1; occurrence-specific content-bound targets; distinct primitive, parameter,
purpose, protocol/component, quantum/classical and assurance facts; versioned
technical dispatch and sources. Preserve every legacy catalog/policy/schema/ID,
rule value, confidence and 0.45 behavior. Business decisions, context and human
rendering belong to tasks068-070.

## Approach

Use one provider-independent core module with small format-specific adapters,
the existing NativeScan validator and pure primitive evaluator. Validate runtime
shape against the existing embedded schemas with the already pinned, offline
JSON Schema dependency (default features disabled), then enforce semantic
envelope/role/assurance consistency. A bounded duplicate-aware JSON reader runs
before schema validation. Do not amend published schemas, construct inventory
from runtime records, initialize providers, resolve DNS or repeat cryptographic
verification. This implements the adopted narrow-adapter approach; a replacement
evaluator or integrated-inventory migration is rejected by the adopted design.

## Steps and checks

1. Restore existing evidence and source applicability; record lifecycle selection.
2. Implement bounded immutable input, duplicate/depth/count/string checks, fixed
   errors, exact byte hashes and target identities. Preserve Native extension
   tolerance while validating all known structure/references.
3. Project fixed Inventory roles and runtime roles. Enforce numeric/name pairs,
   partial/verification gates and SSH advertisement/negotiation separation.
   Add only new-dispatch mappings and source edition/clause references.
4. Exercise frozen technical cases A01/A04-A06/A10-A11/A16-A17, exact/draft/OID,
   malformed parameter/reference/substitution/privacy/limit boundaries; retain
   old goldens. Update paired user guidance and developer architecture notes.
5. Inspect the combined diff, run initial targeted tests, obtain implementation
   review. Complete fmt/workspace test/build/strict clippy and affected optional
   configurations, then obtain test review. Reuse both reviewer identities for
   later material changes and tasks in this authorized request; no delegation.
6. Bind final source, inputs, commands, exits and reviews; reconcile every R2
   clause before marking done. Then select task068 only after prerequisites pass.

## Boundaries and risks

Safe partial facts must survive incomplete collection, but contradictory success
must reject. Unknown IDs/parameters never become resistance or successful use.
Certificate subject/signature, KEX, authentication and offers remain distinct.
Hashing binds bytes without attesting collector truth. Existing static/runtime
collection and output paths are untouched. No publication, probing, external
contact, spending, participant recruitment or numeric-score adoption is authorized.

Task071 lacks practitioners and independent data. This dependency does not block
067-070; it prevents claiming 071/072 or v0.4 complete without actual resources.
