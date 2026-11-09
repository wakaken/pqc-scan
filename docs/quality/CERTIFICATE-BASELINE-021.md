# Focused certificate baseline for task 021

This is a small, deliberately bug-focused regression corpus, not the cross-language
accuracy/performance study in task 023. That task remains pending. No product-wide
precision/recall estimate, held-out evaluation or quality budget is claimed.

## Protocol and provenance

- Baseline: `dc7b63c361263dd3ef44e03855b983c485c9fde6`, saved CLI binary and rule pack
  before production changes. Compare with the task 021 working revision using the
  same input bytes, path spelling, default scanner and separate frozen/current rules.
- Corpus: [25 literal cases](../../crates/pqc_scan_detectors/tests/fixtures/certificate-policy/cases.json),
  reusing task 020 public-only certificate fixtures plus one public DSA/SHA-256 fixture.
  [Provenance and limits](../../crates/pqc_scan_detectors/tests/fixtures/certificate-policy/README.md)
  and [DSA manifest](../../crates/pqc_scan_detectors/tests/fixtures/certificate-policy/manifest.json)
  record independent OpenSSL attributes and hashes. Temporary keys were removed.
- Truth unit: `(case, built-in certificate rule ID)` observation of the encoded
  signature OID, measured subject key or parsed validity condition. Expected lists
  were written before classifier changes, from the reviewed standard-based matrix.
  A rule absent from the old pack, including the two new validity IDs, counts as a
  miss against the desired behavior. Risk/base/final severity changes are asserted
  separately; these membership counts do not measure calibration or severity quality.
- Mutation helpers only rebuild public DER envelopes/declared identifiers/timestamps.
  Mutations invalidate signatures; trust/signature verification is never a label.
  Unknown/PQC identifier cases are reference observations, not conformance claims.
  The PSS case declares SHA-256/MGF1-SHA256/salt32 explicitly; empty/default PSS
  parameters would imply SHA-1 and are not mislabeled as a safe-hash case.
  DSA subject-key extraction remains partial as in task 020; its explicit signature
  OID is still observable. Unsupported/malformed cases are not removed from the corpus.
- Default ranges 2020-2040 and far past/future cases avoid wall-clock edge ambiguity
  during actual CLI comparison. Fixed-clock Rust tests separately inject exact Unix
  seconds, including endpoint equality, 2026/2027 cutoff, reversed ranges and reuse.
  The table's fixed reference instant is 2026-11-09 00:00:00 UTC (1794182400).
- Every case has its own input root; scan `.` from that root so existing `target`
  ancestor exclusions do not turn the test into an empty scan. Outputs are outside
  input roots. Compare complete certificate-ID sets, retaining extra/missing lists
  and input digests. Other detectors still execute; membership evaluation selects
  the declared certificate rule family only.

## Executed before/after membership counts

| Revision | Cases | Expected positive units | TP | FP | FN |
|---|---:|---:|---:|---:|---:|
| Before task 021 | 25 | 48 | 40 | 25 | 8 |
| Task 021 | 25 | 48 | 48 | 0 | 0 |

This demonstrates correction of these specified cases only. Cases were selected
to reproduce known errors, not randomly sampled. Counts must not be advertised as
100% scanner accuracy. No TN denominator or general precision/recall target is
inferred. Final source/fixture digests, review and full validation are recorded in
[task 021](../../tasks/021-certificate-classification-policy.yaml).

Important deltas include removing ECDSA-as-DSA and filename-spoofed algorithms,
size and expiry; recognizing the exact DSA SHA-256 and known RSA/PSS/hash OIDs;
requiring an actual RSA/PSS subject modulus; and separating current expiry/future
validity from the retained legacy cutoff. Unknown signature OID suffixes do not
inherit a recognized prefix classification. Malformed/unsupported containers never
establish typed algorithm/time facts.

## Risk and compatibility evidence

| Observation | Old risk / final severity | Corrected risk / final severity |
|---|---|---|
| ECDSA signature | quantum-uncertain / medium | quantum-vulnerable / high |
| DSA signature | non-quantum-risk / high | quantum-vulnerable / critical |
| SHA-1 / MD5 signature | non-quantum-risk / critical | unchanged |
| 1024-bit RSA subject | non-quantum-risk / critical | unchanged, now type-checked |

Base severities/confidences and the global risk engine are unchanged. Quantum
vulnerability and classical weak-digest/key or legacy-signing concerns are distinct;
the single legacy Risk enum is not replaced here. Full assessment separation remains
task 026. All previous rule IDs remain; current validity uses two additional IDs.
Custom legacy scopes retain their composite semantics except newly reserved explicit
scope names. Typed evidence intentionally changes affected certificate Finding IDs
and legacy CBOM strings; ID algorithms and output structures/enums remain stable.

Local raw case reports, before/after JSON, input digests and logs are under ignored
`target/task021/`. Raw artifacts are local evidence, not remote CI acceptance.
See the [implementation specification](../roadmap/TASK-021-IMPLEMENTATION.md) for
primary standards, supported identifiers and clock/parameter limitations.
