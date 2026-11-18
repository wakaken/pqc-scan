# Frozen expectation coverage

The original65 semantic cases remain unchanged. These are specification conformance tests, not independent practitioner observations. Fixture descriptors are encoded as valid native/runtime/context JSON in the referenced tests; the fixture helper contains no decision oracle. Legacy A17 uses the existing v0.3 integrated goldens and boundary tests.

| Frozen cases | Executed assertions |
|---|---|
| A01, A02, A01-unrelated-missing, A02-ready, A09, A09-missing-peer, A09-known-false-plus-missing | migration069::a01_a04_a06_* / a02_a09_*; explicit axes, minimal claim set and false-plus-missing precedence |
| A03, A03-historical | migration069::a03_*; no criticality requirement and historical unused case |
| A04, A04-draft, A04-pure-prehash | migration067::a01_a04_* / a04_exact_*; migration069::a01_a04_a06_* |
| A05, A10, A11, A05-partial-certificate-collection, A05-partial-missing-group | migration067::a05_a10_* / a11_*; migration069::a05_a10_a11_* |
| A05-unknown-group, A05-draft-group, A05-id-name-mismatch | migration067::a05_all_* / a05_a11_contradictory_*; migration069::a05_unknown_* |
| A06, A06-resolved | migration067::a06_dependency_*; migration069::a01_a04_a06_*; declared/resolved facts never deployment |
| A07, A07-purpose-conflict | migration069::a07_unknown_* / a07_purpose_conflict_*; migration068::a07_* |
| A08, A08-before-boundary, A08-after-boundary, A08-expired, A08-missing-lead, A08-zero-lead, A08-plan-unknown-readiness | migration069::a08_deadline_* and a07_unknown_*; exact D<=as_of+L, same context different clock and normalized UTC determinism |
| A08-signature-lifetime | migration069::a08_signature_*; separate certificate end and signature lifetime |
| A11-contradictory, A11-partial-safe | migration067::a05_a11_contradictory_* / a11_*; migration069::a11_partial_* / a05_a10_a11_* (safe certificate concern preserved independently of collection assurance) |
| A12, A12-override, A12-duplicate-owner | migration068::a12_* / a13_a14_*; migration069::a12_a13_*; CLI invalid_context_* |
| A13, A13-repeat, A13-policy-version | migration069::a12_a13_* / a18_not_applicable_*; CLI actual_cli_reassesses_* |
| A14, A14-unknown-id, A14-ambiguous-anchor, A14-reformat | migration068::a13_a14_*; CLI invalid_context_*; report migration_output content-substitution control |
| A15, A15-at-expiry, A15-at-start, A15-self-verified, A15-no-valid-until | migration068::a15_*; migration069::a15_stale_* |
| A16-offers, A16-kex, A16-kex-unverified, A16-unselected-format | migration067::a16_*; migration069::a16_*; CLI each_saved_format_* / invalid_context_* |
| A17, A17-custom-threshold | existing assessment_v1 + native_v1 CLI/report goldens and confidence boundary tests in full workspace; default old CLI path untouched |
| A18, A18-duplicate-key, A18-null-bool, A18-over-context-bytes, A18-over-depth, A18-before-collection, A18-unix-overflow | migration067 bounded/json_resource_*; migration068 strict_claims_* / shared_resource_* / context_boundaries_*; migration069 a18_*; CLI invalid_context_* |
| A18-input-output-alias, A18-no-network, A18-not-applicable | CLI input_context_and_hardlink_* / no_endpoint_connection_*; migration068 references_deadlines_*; migration069 a18_not_applicable_* |

New integration controls also reject forged report targets/claims/candidates/source metadata, normalize offset-equivalent clocks, keep PKI prerequisites separate from hashing, and reject contradictory exact AES/EC parameters and unrepresentable certificate facts. Main tests check observed public behavior and invalid-input outcomes; writer guards do not implement the decision policy.

Frozen expectation SHA-256: `8ab654f386c25ba136f6c4620c5a03ee59c269344099bd563f4b66caf3874fc4`.
