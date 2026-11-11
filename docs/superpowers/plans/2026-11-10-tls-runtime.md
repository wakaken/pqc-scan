# Task032: verified optional TLS session probe

Spec: docs/architecture/RUNTIME-PROBE-IMPLEMENTATION.md, selected by explicit user delegation.
Root CLI/core/report integration is owned by the parent; this worker owns the optional TLS crate, standalone TLS schema and protocol tests.

- Write failing product API tests using memory-only ephemeral local CA/server keys.
- Add default-empty crate; runtime feature selects pinned rustls/AWS-LC/Mozilla roots and existing bounded core transport.
- Prepare roots/name/profile before any network access. Public API new/failed/run matches the assigned contract.
- Publish negotiated facts only after ordinary verification, completed Full/FullWithHelloRetryRequest; preserve failed/partial/local-limit outcomes and exact transport counters.
- Bound public CA files, chain collection and parsed certificate facts. No private keys/client auth/resumption/early data/key log/application writes.
- Run classical/hybrid/HRR/verifier/chain/timeout/cancel/malformed local peers under sandbox egress denial; offline schema assertions and default/runtime graph checks.
- Record commands, hashes, actual outcomes and unexecuted checks; move to review. Parent independently reviews/evaluates/commits.

Parent-owned integration in this same task: core::probe envelope/model regressions, runtime report writer/English-Japanese parity, CLI/root features, both READMEs/user guides, CLI/offline schema tests and whole-workspace/default/runtime/combined acceptance. These remain required before Evaluator completion. Fixture keys use the explicit test-fixtures feature; default workspace build/test and product runtime normal graph exclude rcgen/native test fixtures.
