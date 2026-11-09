# Standards and implementation reference register

Checked: 2026-11-09. These are source-review references, not interoperability test results. Recheck status, errata and dependency releases when selecting implementation versions. Dates for RFCs are publication months unless an exact date is stated by the publisher.

## Normative and product references

| ID | Document/version and publication | Primary source | Application and limits |
|---|---|---|---|
| S203 | NIST FIPS 203, Final, 2024-08-13 | [Publication and errata](https://csrc.nist.gov/pubs/fips/203/final) | ML-KEM key establishment; not a signature scheme or complete application protocol. Page has a 2025-11-17 potential-update note. |
| S204 | NIST FIPS 204, Final, 2024-08-13 | [Publication and errata](https://csrc.nist.gov/pubs/fips/204/final) | ML-DSA signatures; not key exchange. Page has a 2026-07-31 potential-update note. |
| S205 | NIST FIPS 205, Final, 2024-08-13 | [Publication](https://csrc.nist.gov/pubs/fips/205/final) | SLH-DSA signatures; assess size/performance and protocol fit separately. |
| S8547 | NIST IR 8547, Final, 2026-10-27 | [CSRC Final](https://csrc.nist.gov/pubs/ir/8547/final), [publication date](https://www.nist.gov/publications/transition-post-quantum-cryptography-standards) | Transition guidance, especially federal systems. Do not substitute the November 2024 initial public draft. Do not impose its federal timelines as universal OSS compliance deadlines. |
| T10024 | RFC 10024, Standards Track / Proposed Standard, 2026-08 | [RFC](https://www.rfc-editor.org/rfc/rfc10024.html) | TLS 1.3 hybrid groups X25519MLKEM768, SecP256r1MLKEM768, SecP384r1MLKEM1024. Preserve numeric group identity and distinguish obsolete drafts. |
| T8446 | RFC 8446, Standards Track, 2018-08 | [TLS 1.3](https://www.rfc-editor.org/rfc/rfc8446.html) | Cipher suites and key exchange are separate. Read with subsequent updates, including the references in RFC 10024. |
| T9325 | RFC 9325 / BCP 195, 2022-11 | [TLS recommendations](https://www.rfc-editor.org/rfc/rfc9325.html) | Prefer TLS 1.3; keep securely configured TLS 1.2 only for a documented interoperability need. TLS version alone does not establish PQC. |
| H4253 | RFC 4253, Standards Track, 2006-01 | [SSH transport](https://www.rfc-editor.org/rfc/rfc4253.html) | Identification, packet framing and KEXINIT; read with later SSH updates. Algorithm name lists are unauthenticated until KEX authentication completes. |
| H10042 | RFC 10042, Informational, 2026-08 | [ML-KEM SSH hybrid methods](https://www.rfc-editor.org/rfc/rfc10042.html) | Published SSH hybrid definitions, including mlkem768x25519-sha256. Publication is not Standards Track status. |
| HOPEN | OpenSSH release notes, 9.9 / 10.0 / 10.1 / 10.6 / 10.6p2 | [Official notes](https://www.openssh.org/releasenotes.html), [PQC guidance](https://www.openssh.org/pq.html) | 9.9 added ML-KEM hybrid KEX; 10.0 (2025-04-09) made it preferred; 10.1 (2025-10-06) added non-PQ KEX warnings. 10.6 (2026-10-06) and 10.6p2 (2026-11-05) include composite-signature changes. Do not claim all current SSH authentication is classical; local build/configuration and draft signature status still matter. |
| C17 | CycloneDX 1.7, 2025-10-21 | [Overview](https://cyclonedx.org/specification/overview/), [JSON reference](https://cyclonedx.org/docs/1.7/json/), [schema](https://cyclonedx.org/schema/bom-1.7.schema.json) | Inventory serialization. Schema validity is necessary but insufficient for truthful crypto semantics or referential integrity. |
| C17EX | CycloneDX specification tag 1.7, official fixtures | [Full crypto](https://raw.githubusercontent.com/CycloneDX/specification/1.7/tools/src/test/resources/1.7/valid-cryptography-full-1.7.json), [certificate](https://raw.githubusercontent.com/CycloneDX/specification/1.7/tools/src/test/resources/1.7/valid-cryptography-certificate-1.7.json), [implementation](https://raw.githubusercontent.com/CycloneDX/specification/1.7/tools/src/test/resources/1.7/valid-cryptography-implementation-1.7.json) | Mapping references, not claims that example values are safe or all sample references resolve. Preserve licenses when vendoring. |
| SARIF | OASIS SARIF 2.1.0, 2020-03-27 | [Standard](https://docs.oasis-open.org/sarif/sarif/v2.1.0/os/sarif-v2.1.0-os.html) | Source findings, valid locations and invocation diagnostics; runtime endpoint evidence remains authoritative in native JSON. |

## Library documentation snapshots

| Candidate | Reviewed source/version | Available information and selection limits |
|---|---|---|
| rustls | [CommonState](https://docs.rs/rustls/latest/rustls/struct.CommonState.html), docs displayed 0.23.46 | Negotiated protocol, suite, KX group and peer certificates. KX group is optional and unavailable in some handshake states. Public page is mutable; pin exact crate/provider versions in the Phase 2 spike record. |
| rustls AWS-LC provider | [KX group module](https://docs.rs/rustls/latest/rustls/crypto/aws_lc_rs/kx_group/index.html), displayed 0.23.46 | X25519MLKEM768 and SecP256r1MLKEM768 appear; availability of every RFC group is not implied. |
| aws-lc-rs | [Crate API](https://docs.rs/aws-lc-rs/latest/aws_lc_rs/), [build resources](https://aws.github.io/aws-lc-rs/resources.html) | Provider candidate, not a TLS client. Exact dependency version, native build requirements, MSRV and supported target matrix remain spike outputs; no FIPS validation claim follows from selecting it. |
| russh | [API](https://docs.rs/russh/latest/russh/), displayed 0.65.1 | Full asynchronous SSH library. Verify access to the complete remote KEXINIT list and negotiation details before adopting for discovery. |
| ssh2 | [Session](https://docs.rs/ssh2/latest/ssh2/struct.Session.html), displayed 0.9.7 | `methods` returns negotiated algorithms; `supported_algs` is local library capability, not the server offer list. libssh2/provider capability requires separate verification. |

## Classification and guidance policy

| Observed family | Quantum assessment | Separate issue / candidate migration |
|---|---|---|
| RSA, finite-field DH/DSA, ECDH/ECDSA, EdDSA | Vulnerable to a sufficiently capable quantum adversary | Classical strength depends on parameters/use; select KEM for establishment or signature replacement for signing. Larger RSA/EC does not remove quantum exposure. |
| AES, hashes and MACs | Assess primitive, parameters and attack model; do not apply public-key break claims | Quantum search effects differ from Shor attacks. SHA-1/MD5 collision issues remain classical concerns; do not prescribe ML-KEM as a hash replacement. |
| ML-KEM / ML-DSA / SLH-DSA | PQC-standardized primitive at the observed parameter set | API presence and standardization do not prove secure implementation, protocol compatibility, runtime use or product-wide readiness. |
| RFC hybrid exchange | Assess both constituents and exact protocol identifier | Successful negotiated group is session evidence, not evidence of PQC certificate authentication. |
| Kyber/Dilithium draft aliases, unknown OIDs or vendor names | Preserve exact identifier and unresolved mapping | Never silently canonicalize an old draft into a final standardized wire format. |

NIST IR 8547 informs configurable profiles; it is not a universal legal assessment. Do not freeze unreviewed date-based failure gates into rules. A policy profile must record source edition/section, scope, effective dates and local applicability. Specific deadline tables require a separate review of the full Final report before implementation.

The [Final report body](https://doi.org/10.6028/NIST.IR.8547), sections 2.2, 3.1 and 4.1, was also consulted: protocol/library/PKI integration and confidentiality versus authentication lifetimes affect migration order. Tables 2-5 separate vulnerable public-key families from PQC replacements; section 4.1.3 treats symmetric cryptography separately. These support purpose-specific assessments, not an automatic universal deadline or a claim that every primitive has a deployable protocol replacement.

`RecommendedAction` remains usable. Internally attach purpose (establishment/signature/hash), candidate status, applicability, prerequisite library/runtime/provider/peer support, validation steps and source IDs. Keep JOSE/JWT identifiers, encodings and verifier rollout separate from primitive choice. Certificate migration requires ecosystem and trust-store support; do not mandate an unverified hybrid-chain deployment.

These classifications are design guidance for [DOMAIN-MODEL.md](../architecture/DOMAIN-MODEL.md), [probe design](../architecture/TLS-SSH-PROBE-DESIGN.md) and the rule-review tasks. They do not change existing `Risk`, rule IDs, severity or output today.
