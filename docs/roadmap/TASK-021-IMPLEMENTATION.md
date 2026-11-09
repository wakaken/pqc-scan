# Task 021: exact certificate classification and validity

## Scope, authorization and sequencing

User explicitly requested task 021 implementation, compatibility, tests, documentation, review and a local commit. This is authorization for the certificate-classification corrections already specified by task 021 and the standards register; it is not authorization for a new report schema, general risk-engine rewrite or automatic certificate deployment. Review the concrete specification below before production changes. Preserve the existing CLI/options, serialized structures/enums, legacy IDs and custom rule behavior.

Baseline `dc7b63c361263dd3ef44e03855b983c485c9fde6` (`origin/develop`, task 020 merged); branch `codex/task-021-certificate-policy`, initial checkout clean. Task 040 remains intact on its separate local branch and is not included. No push/PR/merge.

Planner sequencing ruling: task 023's broad, cross-language labeled accuracy/performance study is still pending. Exact OID/type/time correctness does not require that whole study. Before these fixes, capture a certificate-only reviewed truth table and old results using task 020 public fixtures plus a public DSA fixture and parser-only mutations. Implement/evaluate only those narrowly defined certificate truth units here, with independent review and before/after results. Record task 023 as related broader quality work, not falsely completed; no product-wide precision/recall or budget claim. Task 020 remains the implementation dependency.

## Corrected predicates and expected policy deltas

All default signature predicates match exact parsed signature OIDs using existing `signature_oid`; subject-key predicates never inspect filenames/signature text. Existing custom `certificate` and other legacy scopes continue their old composite-text behavior, except the three newly reserved opt-in names `rsa_modulus_bits`, `certificate_validity`, `legacy_not_after_2026`. No current repository pack uses those names. Migrate any external pack that previously used those exact strings as opaque legacy labels before opting into this version. No global rule-ID special cases or custom-pack rewriting.

| Rule | Input and condition | Risk / base severity | Intended delta |
|---|---|---|---|
| CERT_RSA_SIGNATURE | Exact RSA signature OIDs: MD5 `.1.1.4`, SHA-1 `.5`, PSS `.10`, SHA-256/384/512/224 `.11/.12/.13/.14` under `1.2.840.113549` | quantum-vulnerable / high, unchanged | No filename match; include known MD5/PSS/SHA-224 RSA signatures |
| CERT_SHA1_SIGNATURE | Exact RSA SHA-1, DSA SHA-1 `1.2.840.10040.4.3`, ECDSA SHA-1 `1.2.840.10045.4.1` | non-quantum-risk / critical, unchanged | Positive hash classification from signature identifier only |
| CERT_MD5_SIGNATURE | `1.2.840.113549.1.1.4` | non-quantum-risk / critical, unchanged | No filename/name substring match |
| CERT_ECDSA_SIGNATURE | `1.2.840.10045.4.1` and `.4.3.1` through `.4.3.4` | **quantum-vulnerable** / medium | Previously uncertain; existing PKI promotion changes final medium to high; never DSA-only |
| CERT_DSA_SIGNATURE | `1.2.840.10040.4.3`, `2.16.840.1.101.3.4.3.1/.2` | **quantum-vulnerable** / high | Previously non-quantum; existing PKI promotion changes final high to critical. Legacy-signing concerns remain in guidance, distinct from quantum vulnerability |
| CERT_OID_RSA_PSS | Exact signature `1.2.840.113549.1.1.10` | quantum-vulnerable / high, unchanged | Subject-key PSS alone is not a PSS signature |
| CERT_RSA_1024/2048/3072 | New `rsa_modulus_bits` scope: known positive RSA/PSS subject modulus, exact decimal length regex | Existing risk/severity/confidence unchanged | EC/unknown keys and filenames cannot satisfy RSA-size predicates |
| CERT_EXPIRY_2026_OR_BEFORE | New `legacy_not_after_2026` scope: valid parsed interval and `not_after < 1798761600` (2027-01-01 UTC) | Existing non-quantum / medium / 0.25 | Preserve fixed cutoff/ID, explicitly label as legacy; no reinterpretation as current expiry |
| CERT_EXPIRED (new) | New `certificate_validity` scope, target `expired` when `now > not_after` | non-quantum-risk / medium / 0.90 heuristic | Explicit current-time expiry fact, not a runtime-use/trust verdict |
| CERT_NOT_YET_VALID (new) | Same scope, target `not_yet_valid` when `now < not_before` | non-quantum-risk / medium / 0.90 heuristic | Separate temporal fact |

RFC 5280 endpoints are inclusive: `not_before <= now <= not_after` targets `valid`. A reversed interval is Partial and produces no assertive validity/cutoff result, while retaining parsed signature/key findings. An unavailable clock makes required current-time assessment Partial, never assumes time zero. Signature/chain/mathematical key validation remains excluded. PSS digest parameters, unlisted OIDs, standalone EdDSA certificate rule coverage and general assessment-model separation are not expanded here; absence of a weak-hash finding is not proof of a safe digest. Confidence remains an uncalibrated rule heuristic, not a measured probability.

Container findings remain low-certainty presence signals only. When parsing fails, typed signature/key/time scopes must not fall back to a filename. Preserve legacy custom fallback and container rule IDs. OID-only PQC observations remain separate and unchanged.

## Clock and compatibility design

Keep the public unit struct `CertificateDetector` and required Detector APIs. Add the associated factory `CertificateDetector::at(assessment_time_unix: i64) -> impl Detector` for deterministic library/test/pipeline use. Default detector composition captures one Unix-second clock snapshot **at registry construction** through a small private wrapper, with unchanged detector name and a single parse execution. CLI constructs a fresh registry per scan command. Library clients reusing a registry deliberately reuse its assessment instant; construct a fresh registry for a fresh-time scan. Direct legacy unit-detector calls use a per-call snapshot. No ScanConfig field, CLI switch, dependency or crate is added.

Only current-time validity findings add assessment time/status inside the existing evidence string map. Typed evidence reflects signature identity, subject modulus or temporal condition separately, rather than combining signature and subject size in the match text. Affected certificate Finding IDs/legacy CBOM algorithm strings therefore intentionally change; the ID/dedup algorithms and unrelated findings remain unchanged. Other parsed certificate attributes are retained. The existing risk engine stays intact; changed final ECDSA/DSA severities follow existing promotion rules and are recorded explicitly. The single legacy Risk enum cannot fully represent simultaneous concerns; task 026 remains responsible for the richer model. SHA-1/MD5/weak-RSA findings and DSA guidance preserve classical concerns without mislabeling the primitive as quantum-resistant.

Alternatives rejected: repurpose the date-named legacy ID, rewrite all custom predicates, or add an assessment crate/new serialized risk fields. Those would create unnecessary compatibility migration. Additional validity rule IDs are deliberate small extensions needed to preserve legacy semantics while evaluating the actual clock.

## Files and one-PR implementation sequence

- `rules/default/cert.yml`: opt built-ins into exact scopes, preserve IDs/base severities/confidences except the two documented risk tags; add two validity IDs with explicit descriptions/guidance.
- `crates/pqc_scan_detectors/src/certificate_detector.rs` and `lib.rs`: typed scope dispatch, safe fallback, clock entrypoint/snapshot, temporal metadata; reuse task 020 parser unchanged.
- New certificate-policy integration tests/public DSA fixture and baseline truth table; existing public fixtures reused byte-for-byte. No private keys retained.
- `docs/rules-reference.md`, READMEs, architecture and Japanese certificate copy: exact semantics/deltas/time boundaries/custom migration. No renderer or unrelated catalog changes.
- Task 021/roadmap dependency rationale; task 023 remains untouched/pending. `core/src/risk.rs` is inspected and intentionally unchanged.

- [x] Capture pinned build/tests, binary/rule snapshots and both-language example outputs under ignored `target/task021/`.
- [x] Write/review labeled certificate-only baseline and this standards-based specification before implementation.
- [x] Add failing tests for filename spoofing/malformed containers, ECDSA/DSA pairs, RSA subject type, PSS signature/key distinction, old custom-rule behavior and quantum/base/final classification.
- [x] Add explicit-clock tests for before/at/after endpoints, reversed ranges, legacy 2026/2027 boundary and temporal metadata. Implement the minimal scopes/clock wrapper only after test failures.
- [x] Measure exact certificate-case old/new TP/FP/FN and result deltas, separate from broad task 023; retain unrelated sample/native format behavior and all existing entrypoints.
- [x] Update docs/localization and rule counts; run fmt/build/workspace tests/clippy/Node checks and actual CLI report/privacy/time acceptance.
- [x] Independent whole-diff review, resolve material issues, evaluator records digests/results/unexecuted checks, then local commit and clean-tree verification. No publication.

## Primary references

Consulted 2026-11-09: [RFC 5280](https://www.rfc-editor.org/rfc/rfc5280.html), May 2008, 4.1.2.5 (inclusive validity) and signature/SPKI separation; [RFC 3279](https://www.rfc-editor.org/rfc/rfc3279.html), April 2002, signature OIDs; [RFC 5758](https://www.rfc-editor.org/rfc/rfc5758.html), January 2010, DSA SHA-224/256 and ECDSA SHA-2 identifiers; [RFC 4055](https://www.rfc-editor.org/rfc/rfc4055.html), June 2005, RSA SHA-2/PSS identifiers; [FIPS 186-5](https://csrc.nist.gov/pubs/fips/186-5/final), Final 2023-02-03, DSA retained for legacy verification, not new signing; [NIST IR 8547 Final](https://csrc.nist.gov/pubs/ir/8547/final), October 2026, vulnerable public-key family classification. No federal deadline or automatic production migration is introduced.


## Specification review record

Independent `task021_review` verified OID sets against RFC 3279/4055/5758, inclusive boundaries against RFC 5280, cutoff epoch and unchanged-engine ECDSA/DSA final severities. The focused-baseline sequencing was accepted without claiming task 023 complete. The construction-time clock contract was clarified to avoid implying a reused registry obtains a new time, and new scope-name reservations were made explicit. The user's explicit task 021 implementation request authorizes this already-defined correction scope; any additional CLI/schema/general scoring or unlisted migration-policy change still requires a separate decision.

## Final review and local evaluation

The independent reviewer found two material issues: the literal truth-table test used
wall time instead of its declared fixed instant, and custom `legacy_not_after_2026`
rules matching the post-2026 target received an incorrect pre-2027 evidence string.
The latter was reproduced by a failing regression before correction. The former now
uses the table's injected clock; custom cutoff evidence distinguishes `notAfter <
2027` from `notAfter >= 2027`. Re-review approved the full difference, independently
ran all 12 policy integration tests and confirmed the explicit PSS hash/MGF/salt
profile with OpenSSL. No remaining material review finding was reported.

Evaluator ran the pinned offline build, format check, all 139 Rust tests, warnings-
denied clippy, three Node tests, 25-case same-digest old/new CLI comparison and
English/Japanese real report acceptance. All passed. Existing examples retain 290
findings, 55 scanned files and two skipped files; all six report formats and
schema-1 diagnostics match baseline after removing only generation timestamps.
Machine outputs also match across languages. Six synthetic private-PEM marker
families do not leak their canary payloads into any format, diagnostics or process
output. Strict completeness still exits 3 on the existing incomplete examples.

Reflection: rule predicates should consume typed parsed attributes; keep filename
fallback explicitly separate. Pin the assessment clock in truth-table tests as well
as endpoint tests, and test both outcomes of custom predicate scopes. Construction-
time snapshots need an explicit reuse contract. These narrow patterns reuse the
current parser/trait/report flow without a new crate or global scoring abstraction.

Exact source/fixture digests, commands, review and limitations are in
[task 021](../../tasks/021-certificate-classification-policy.yaml); raw local evidence
is under ignored `target/task021/`. Remote CI, new browser acceptance, general
accuracy/performance budgets and certificate trust/signature validation were not
executed. Task 023 remains pending. Task 040 is preserved on its separate local
branch; if later combined, re-review its exact self-scan expectation registry against
these intentional certificate-rule/evidence changes. No scan finding is suppressed
by task 021 and no publication is authorized here.

## Publication preparation (separately authorized)

The user subsequently authorized pushing both task 040 and task 021 and preparing
PRs. To preserve one task per PR, task 040 targets `develop`; task 021 is stacked on
`codex/ci-self-scan-expectations`. Retarget task 021 to `develop` after the CI
prerequisite merges. No merge is authorized by this publication request. The local
prerequisite merge preserves both reviewed implementation commits and resolves the
two README additions without conflict; no Rust/rule/helper behavior was added.

Task 021 now carries its exact self-scan registry maintenance in the same PR:
refresh the moved inline-test source/result binding, eight existing public
certificate expectations and two new test/public-fixture file registrations. Remove
the three obsolete ECDSA-as-DSA expectations. The registry has 83 files and 618
exact groups; raw 746 = reviewed omissions 618 + retained 128. All six product
signature-name mapping observations remain present in the upload copy. No blanket
suppression, automatic CI learning, cryptographic weakening or threshold change.

Independent `ci_self_scan_review` approved this integration registry update,
recomputed every source/full-result binding and partition, checked public DSA facts
with OpenSSL and independently passed all 21 Node tests. Fresh combined local
build/fmt/workspace tests (139)/clippy/Node tests (21) passed. Logs and exact
raw/upload/audit evidence are under ignored `target/publish-021/`. Remote Actions
and Code Scanning conclusions remain pending at local preparation time.
