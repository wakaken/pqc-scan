# Task 020: certificate attribute parsing implementation plan

**Goal:** correct certificate/subject-key facts and expose parsing completeness without changing CLI switches, report structures, rule definitions or risk policy.

**Architecture:** extend the existing certificate detector using pinned `x509-parser` 0.18.1 and `pem` 3.0.6. Add owned public attribute helpers in that module for future TLS reuse; do not add a crate, PKI framework or runtime probe. Add the already-workspace-pinned SHA-256/hex dependencies to this crate for public-object fingerprints.

**Spec:** [task 020](task.yaml), [domain model](../../docs/architecture/DOMAIN-MODEL.md), [quality gates](../../docs/quality/QUALITY-GATES.md). User explicitly authorized implementation, tests, documentation, review and local commit. Task 021 classification/rule migration is excluded.

Baseline: latest `origin/develop`, `321e5c9` (task 019 merged). Branch `codex/task-020-certificate-attributes`; initial checkout clean. Continue in this clean checkout on a dedicated branch; no concurrent implementation/worktree is needed. One task/PR unit, one local implementation commit. No push, PR or merge.

## Corrected attribute specification and compatibility boundary

- Signature OID/name is the certificate's outer signature algorithm; subject-key OID/type/parameters come only from SPKI. Neither establishes trust, runtime use, on-curve validity or a verified signature.
- RSA size is the positive INTEGER modulus bit length, excluding DER sign padding and counting leading bits exactly. Do not use `subject_public_key.data.len()*8` or the pinned library's generic `key_size()` helper (its RSA implementation assumes a leading zero). Parse the RSA structure with the existing ASN.1 parser; invalid/zero/negative integers are not measured keys. Support rsaEncryption and RSASSA-PSS key OIDs; retain their distinction and parameters.
- EC reports the named-curve OID and known field size (P-256/P-384/P-521), never encoded point bits as key size. Missing/unknown/explicit parameters have unknown semantic size and an incomplete attribute outcome. Point representation/length is checked for known curves; mathematical point validation is out of scope.
- Ed25519/Ed448/X25519/X448 names and encoded lengths are distinct from RSA modulus/EC field size. No inferred security-strength bits. Unsupported/PQC/unknown keys retain their exact OID, public SPKI fingerprint and encoded size, with unknown semantic size and an incomplete attribute outcome; never assume RSA.
- Record DER byte size, SPKI DER byte size and subjectPublicKey BIT STRING payload byte size with explicit names. Preserve parameter presence/type, OID where applicable and a digest of parameter content; do not export arbitrary parameter/key bytes.
- `parse_certificate_der(&[u8])` and `parse_public_key_der(&[u8])` return owned attributes or fixed safe parse errors, require complete consumption and do not verify signatures/chains. The public-key helper accepts SPKI, not raw PKCS#1 or private-key structures. Standalone public-key file detection is not added by this task.
- Certificate file candidacy retains the existing extension policy. Within `.pem/.crt/.cer/.der`, choose PEM versus DER by content, not suffix. `.p12/.pfx` remain unsupported interior inspections, preserving existing container signals. This is not a claim to discover arbitrary extensionless certificate files.
- PEM bundles parse each framed block independently; keep valid certificates before/after malformed or unsupported blocks. Record a 1-based block/object index, 0-based byte offset, real PEM line/column and SHA-256 of exact public certificate DER and SPKI DER. Distinct PEM certificates survive existing pipeline dedup through their real positions, without changing core IDs/dedup. DER retains legacy 1:1 location, with byte offset 0; trailing bytes make inspection partial rather than being silently accepted. DER bundle parsing is not added.
- Detailed outcomes: valid supported attributes Complete; no certificate rules/noncandidate NotApplicable; no parseable certificate in a candidate is Unsupported for an unsupported container/block or Partial for malformed input; any valid+malformed/unsupported/unknown-attribute mixture is Partial. Keep legacy container findings, but never use those to claim successful parsing. Dependency detector alone remains NotReported after this change.
- Preserve broad custom/default certificate rule matching inputs (path, signature, OID, expiry), replacing only the erroneous size with known semantic size or the literal `unknown`. Signature and public-key OID scopes remain separate. Metadata adds facts inside the existing string map; report shapes/version/filenames stay unchanged. Corrected sizes, object positions and evidence intentionally change affected certificate findings/IDs. Unrelated findings must remain identical.
- Do not fix filename-spoofed size/expiry matches, ECDSA-to-DSA pattern confusion, risk tags or time assessment here; those are explicitly task 021. Document these residual limitations instead of silently changing rule semantics.

## Alternatives

Extend the existing parser/detector (chosen): smallest scope and preserves rule/API/report contracts. A new PKI/inventory crate or replacement parser adds migration cost without solving a current need. Exact typed rule predicates would solve classification problems but exceed task 020 and its authorized compatibility boundary; retain them for task 021.

## Files and ordered implementation (one PR)

- Existing `crates/pqc_scan_detectors/src/certificate_detector.rs`: framing/provenance, detailed outcomes and existing rule adapter. New private submodule `certificate_detector/attributes.rs` holds typed owned attributes and re-exported parser helpers, keeping ASN.1 extraction separate from file/rule orchestration within the same crate.
- Existing `crates/pqc_scan_detectors/Cargo.toml` and `Cargo.lock` only if dependency graph metadata changes: reuse workspace SHA-256 and hex, no new package versions.
- New `crates/pqc_scan_detectors/tests/certificate_attributes.rs` and public-only fixtures/README: expected facts, negative/bundle/privacy cases and helper/detector/pipeline integration.
- Existing CLI tests: update only the now-one legacy detector diagnostic expectation when required; no CLI implementation change.
- Existing task 019 integration test: update the same remaining legacy-detector count from two to one; no task 019 lifecycle/acceptance changes.
- Existing `core/src/pipeline.rs`: extend only file-wide private-marker snippet masking for EC/DSA/encrypted PEM, following the reproduced certificate-container canary leak. No dedup/ID/CLI/rule changes.
- Both READMEs and `docs/architecture.md`: corrected sizes, explicit supported containers, identity/diagnostics and limitations. This plan/task record stores review/evidence.
- `classifier.rs` is inspected but need not change: current case-insensitive candidate suffixes already cover the requested files.

- [x] Capture pinned offline build/test baseline, saved binary and both-language all-format example outputs under ignored `target/task020/`.
- [x] Independent read-only review of this attribute specification before production implementation; resolve material concerns.
- [x] Add tests using known public certificate/SPKI DER: RSA 1024/2048/3072 and non-byte-aligned modulus, EC P-256/P-384/P-521, Ed25519, unknown/PQC OID and mixed signature/key algorithms. Watch regressions fail against baseline before production changes.
- [x] Implement narrow DER attribute helpers and real decoder-based facts, preserving unknown values and safe errors. Run focused tests.
- [x] Test first then implement content format selection, damaged/mixed bundles, unsupported PKCS#12/private-key blocks, exact fingerprints/positions and pipeline retention. Preserve legacy container signals.
- [x] Update docs and run all-format old/new corpus comparison. List intentional certificate deltas separately; require unrelated findings and language-independent machine data to match.
- [x] Run fmt/build/workspace tests/clippy and report Node checks; independently review full diff, correct material findings and rerun affected/full checks as appropriate.
- [x] Evaluator records tested source/fixture digests, logs, unexecuted checks and done status only after evidence; commit locally and verify clean tree.

## Review focus / required negative fixtures

Unsigned/negative/non-byte-aligned RSA integers; EC unknown/missing parameters and malformed point; Ed25519 with unexpected parameters/length; PQC/unknown payload never measured as RSA; signature RSA + subject EC independence; identical SPKI in distinct certificates; leading whitespace/CRLF PEM positions; malformed block between valid objects; malformed-only/unsupported-only candidate; DER trailing bytes; private-key canary absent from attributes/evidence/reports/diagnostics. Preserve existing OID-only synthetic PQC tests as identifier observations, not valid PQC key assertions.

## Primary references and applicability

Consulted 2026-11-09; these specify attribute encodings, not deployment readiness or a new classification policy.

- [RFC 5280](https://www.rfc-editor.org/rfc/rfc5280.html), May 2008, sections 4.1/4.1.2.7: certificate, signatureAlgorithm and SPKI separation.
- [RFC 3279](https://www.rfc-editor.org/rfc/rfc3279.html), April 2002, section 2.3.1: RSA modulus/exponent public-key encoding.
- [RFC 5480](https://www.rfc-editor.org/rfc/rfc5480.html), March 2009, sections 2.1/2.2: named-curve EC parameters and point encoding.
- [RFC 8410](https://www.rfc-editor.org/rfc/rfc8410.html), August 2018, sections 3/4: Ed/X OIDs, absent parameters and encoded public-key lengths.
- Pinned local source: `x509-parser-0.18.1/src/{x509,public_key}.rs`, `pem-3.0.6/src/lib.rs`. RSA helper inspection is source evidence, not a new cryptographic implementation or runtime benchmark.

## Execution ledger

Specification gate: independent reviewer `task020_review` approved before production changes; specifically requested invalid INTEGER/full-consumption, BIT STRING alignment, Ed/X parameters/length and mixed-bundle coverage. The minimal source-snippet masking extension was approved as necessary to prevent reproduced disclosure, without changing classification.

Baseline build/tests (105 passed) and both-language all-format outputs saved under `target/task020/`. Twelve initial detector/pipeline tests compiled and failed against baseline. A separate EC PRIVATE KEY canary test reproduced raw synthetic payload in certificate-container source snippets (`privacy-red.log`); all three additional marker families are now file-wide masked. Thirteen detector/pipeline cases then passed. The initial Ed25519 test wrongly assumed a built-in inventory rule would emit a finding; corrected it to use an explicit test-only public-key OID rule rather than adding a product rule. Additional helper-level negative cases cover malformed structures and exact encodings.

Ruling: keep public certificate parsing and file/rule orchestration in separate files within the existing module/crate, because independent reusable helpers should not depend on file paths or RuleSet. This is a small module addition, not a new subsystem.


## Final review and evaluation

Independent full review found two material issues: the pinned container parser discards inner remainders, and a whole-file PEM substring can occur inside DER. Two new regressions failed before the fixes. Bounded expected-envelope checks now reject extra Certificate/TBS/validity/AlgorithmIdentifier/SPKI fields and confirm the exact original SPKI range; DER parsing takes precedence over PEM framing. Re-review approved both fixes, with independent 20/20 targeted execution. Signature BIT STRING alignment and inner/outer signature identifiers also prevent a complete attribute verdict on inconsistent inputs.

Final Rust 1.93.1 offline build/fmt/clippy succeeded; workspace 125 tests passed (baseline 105 plus 20 new), Node 3 passed. Both-language example results retain 290 findings/55 selected/2 skipped and all six legacy artifacts except generation timestamps. Schema-1 diagnostic shape remains; dependency alone remains NotReported. Known malformed/unsupported certificates now honestly affect coverage.

Nine public certificate fixtures have independent OpenSSL manifest facts/fingerprints. Actual CLI corpus output changes 12 to 16 findings: exactly four RSA-size rule/file pairs added, none removed. Five private-marker families retain no synthetic canary in seven output artifacts or stdout/stderr. Strict mode returns 3 after report generation. Fixture preparation used ephemeral keys outside the repository and removed them; tests require no OpenSSL/network.

Verification-harness rulings: compare legacy IDs using identical root spelling (the initial absolute/relative comparison changed only IDs); execute scratch corpora with `scan .` from their directory because existing full-path `target` exclusions otherwise select zero files; assert expected nonempty per-object observations. A copied duplicate test-only rule was removed. These corrections change no scanner/walker/rule-loader behavior. The initial Ed25519 test uses an explicit test-only OID rule because no default Ed25519 certificate inventory rule exists; do not invent product findings to satisfy a test.

Raw local evidence: `target/task020/{validation-summary.json,report-validation.json,VERIFICATION-REPORT.md,final-*.log}`. Durable source/fixture digests and unexecuted checks are in task 020. Remote CI, new browser acceptance and performance/precision/recall measurement were not run. The requested final action is a local commit only.
