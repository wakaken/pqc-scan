# Migration recommendation review

Status: Partial. 66 source-cited local-policy cases exercise candidate purpose,
applicability, required support and JWT/PKI conditions. All compared expected
judgments matched in the first bounded measurement. KEM candidates remain key
establishment; signature candidates remain signatures. Operator support claims
are not observed runtime/provider/peer capabilities.

140 legacy recommendation combinations were observed across five categories,
seven language labels, two evidence kinds and two risk classes. They produce
18 unique action payloads. They are characterization, not independently correct
gold; older/known-runtime versions, middleware branches and complete provider/
feature/unused-dependency combinations remain unmeasured by this table.
The source-hashed action payloads and originating case IDs are in
`RECOMMENDATION-ACTIONS.json` under the task evidence directory.

## Source facts checked

Official snapshots confirm JDK24's ML-KEM/ML-DSA JEP delivery, Go1.24 ML-KEM and
Go1.27 ML-DSA additions, versioned Node26.13.1 crypto/WebCrypto history, and
cryptography50.0.2's versioned changelog. Rustls0.23.46 defaults and the Ruby
OpenSSL history were acquired; all precise historical version/provider claims
still need targeted reconciliation before declaring every recommendation valid.
Tool/browser transport failures are not negative API-support findings: OpenJDK
browser fetches returned 403 while direct official GETs succeeded and were saved.

JWT actions explicitly call out JOSE identifiers, encoding, libraries and verifier
rollout. Node actions state ML-KEM does not replace JWT signatures. Runtime
verification actions retain resolved version/provider/feature/peer checks.
TLS reference guidance separates RSA authentication from static RSA KEX, and
source references from enabled/negotiated connections. These are bounded text
observations, not deployment/interoperability tests.

## Concrete finding and proposal (no product change)

For an unknown Java runtime profile, an observed action says the detected version
appears older than JDK releases with standardized PQC APIs. The probe supplied no
Java version. Missing version information cannot establish an old detected
version. This is a wording/provenance finding, separate from the true JDK24 API
availability fact. It occurs in the `java.runtime-upgrade-for-pqc` payload among
the bound actions. The original source is
`crates/pqc_scan_core/src/recommendation.rs`, function `add_java_actions`.

A possible correction is to distinguish unknown runtime from a detected pre-24
runtime and retain a verification step before upgrade advice. No wording,
decision, runtime predicate or recommendation contract was changed by this
evaluation. The concrete source/output evidence is preserved for a scoped
maintainer decision; no new task was created.

Code examples and PKI hybrid pilot advice require explicit key-type/parameter,
certificate-format, issuer, trust-store and verifier checks. A primitive name
change alone is insufficient, and this run did not execute those example programs
or establish complete chain/interoperability correctness. No unverified hybrid
construction, feature availability or production rollout is declared safe.

Before Q3-10 can be fully verified, independently assess the characterized
payloads and remaining version/middleware/provider cases, resolve the unknown
Java inference, bind relevant primary-source editions/errata, and retain any
required decision for intentional changes. Passing the measurement adapter or
66 candidate checks does not discharge these original requirements.
