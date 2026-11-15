# Task061 prepared artifact trust results

Complete against original Q6-14 and Q6-15. Source revision is
`f07d782cd68cfac95d93585d5ace049f590160d4` plus hash-bound current inputs from
059 and [reviewed sources](evidence/integrity-20261115/REVIEW-SOURCE.json).
No product, rule, package verifier or production trust policy changed.

The isolated offline non-root container ran actual OpenSSL 3.0.22 Ed25519 fixture
signing and recipient verification: 32 cases matched their independent expected
outcomes, with 2 correct signatures accepted and 30 invalid cases rejected.
Each variant covers binary/rules/notices corruption and omission, binary mixing,
recomputed untrusted checksums, signature alteration, wrong trust root and
correctly signed but unauthorized source/builder/target/variant or field type.
The existing checksum verifier rejects corrupt or missing members; deliberately
self-consistent forged checksums pass hash-only verification and are rejected
by the complete authenticated subject procedure.

Verification anchors in an independently selected test public key, then checks
exact statement policy, actual subject digest and archive members. The container
exit result was 0; per-case commands and exits are retained in
[RESULT.json](evidence/integrity-20261115/container-r2/RESULT.json).
[RUN.json](evidence/integrity-20261115/container-r2/RUN.json) binds the immutable
image, sources, inputs, enforced and observed resource limits and cleanup.
Ephemeral private keys were deleted and the container removed. Only bounded
parent-constructed public JSON facts/hashes were retained; no private key,
credential or container scratch file was exported.

The required [integrity JSON](evidence/integrity-20261115/artifact-integrity.json)
and [provenance preparation](evidence/integrity-20261115/release-provenance.md)
are byte-identical to target/quality-task-061 copies. The initial 30-case run
remains historical evidence; the final run adds binary omission and signature
measurement facts. [Reviews](TASK-061-REVIEWS.md) accept both required stages;
[final bindings](evidence/integrity-20261115/EVIDENCE-FINAL.json) tie them to the
unchanged final executable sources.

This completes prepared fixture validation, not real release signing, hosted
attestation, production identity certification or a SLSA level. No remote
workflow, tag or publication occurred. The existing four network notice flags
and publication hold remain. Task052 stays unfinished and task053 interrupted.
