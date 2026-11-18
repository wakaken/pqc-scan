# Technical fact, edition and applicability mapping

Status: D5 mapping proposal adopted by the maintainer on 2026-11-18. Sources are pinned by the
[primary-source receipts](evidence/official/RECEIPTS.json),
[existing task056 source register](../056/evidence/assessment-20261115/SOURCE-REGISTER-v2.json)
and task066 source/reuse evidence. Source review specifies facts, not new product
implementation, crypto validation or legal compliance. Existing v1 judgments and
source-exposed task056 cases remain frozen regression evidence.

## Exact proposed fact contract

| Fact ID | Edition/clauses and authoritative input | Mapping and boundary | Cases / downstream obligation |
|---|---|---|---|
| F01 classical public-key identity | Existing catalog and task056 source register; SP800-131A Rev2 (2019), sections3/5/6; RSA/EC/DSA/DH/EdDSA fact expectations | Keep purpose, quantum exposure and classical parameter concerns separate. Preserve local RSA2048/EC224 thresholds for v1. Unknown/malformed parameters remain unknown; larger RSA/EC does not remove quantum exposure | A01/A02/A03/A07/A12; no rule/category/confidence rewrite |
| F02 ML-KEM names/purpose | FIPS203 Final 2024-08-13, sections1.1/3.1/7 and section8 Table2 (PDF page48), exact saved name/OID | ML-KEM512/768/1024 are KEM parameter sets, not signatures. A bare family has unknown parameters. Standardization does not prove execution/provider/peer/implementation | A04/A05/A09; preserve old primitive helper behavior |
| F03 ML-DSA names/purpose | FIPS204 Final 2024-08-13, section1.1, section4 Table1 (PDF page25), signing/verification interfaces | ML-DSA44/65/87 identify signature parameter sets. Do not substitute KEM, old Dilithium names or prehash encodings | A04/A09; unknown encoding remains explicit |
| F04 SLH-DSA names/purpose | FIPS205 Final 2024-08-13, sections10.2.1/10.2.2 and section11 Table2 (PDF page53) | Twelve SHA2/SHAKE128/192/256 s/f names remain exact. Pure versus HashSLH-DSA differs; parameter name is not interoperable PKI construction or working SDK evidence | A04; task065 Java/Ruby annotation augmentation, no new detector claim |
| F05 PQC OIDs and namespaces | Task056 pinned NIST CSOR register snapshot plus existing pure-OID mapping in assessment.rs | KEM OID suffixes1..3, signature ML-DSA17..19/SLH-DSA20..31, exact selected namespaces. Unlisted/prehash/mismatched namespace and unknown ASN.1 parameters are not silently equated | A04/A14; preserve parameter-encoding uncertainty missing in TLS metadata |
| F06 final TLS hybrid groups | RFC10024 (Standards Track, August2026), sections4.1-4.3,7.1-7.4; RFC9954 (Informational, July2026) sections3.1-3.3/6 | Numeric4588 = X25519MLKEM768;4587 = SecP256r1MLKEM768;4589 = SecP384r1MLKEM1024. Keep each exact component set and specified combiner reference.25497/25498 are obsolete pre-standard Kyber groups, never aliases for final ML-KEM | A05/A10/A11/A16; new catalog only, no assertion of verified combiner implementation or PQC authentication |
| F07 TLS protocol roles | RFC9846 (Standards Track, July2026), sections4.2/4.3.7/4.3.8/4.5.1-4.5.3; pinned old RFC8446 remains old-v1 provenance; existing ProbeReport/TlsObservation contracts | TLS1.3 cipher suite is not the key-exchange group. Verification/Finished/full-handshake gates remain distinct from accessors. Save explicit numeric version/suite/group and collector-reported assurance. Missing CertificateVerify scheme stays unknown | A05/A10/A11; no new transport, PSK/resumption, trust or revocation verification |
| F08 certificate role/time facts | RFC5280 (2008), sections4.1.1.2,4.1.2.5,4.1.2.7; RFC4055 (2005), sections1.2/3; existing certificate parser/relations | Signature algorithm and subject key are separate. RSA-PSS is signature-only. Inclusive captured validity endpoints; explicit as_of validity can be additional. OID/size/time/fingerprint do not prove signature/chain/trust/math. TLS lacks DER/encoding details and cannot reconstruct them | A02/A08/A10/A11; no guessed TLS signature scheme or absent ASN.1 parameters |
| F09 saved SSH offers | RFC4253 (2006), section7.1; current SshObservation/SshOffers schema/model; RFC4251 sections5/6 | Ten ordered advertised lists and extension markers, unauthenticated, not selected methods or host trust. Candidate labels are recomputed from finite exact names | A16 offers; D3 selects ingestion, unchanged collector/support |
| F10 saved SSH KEX | RFC10042 (Informational, August2026), sections2.3.1-2.3.3/2.4/2.5/5; RFC8731 sections3/4 and RFC8332 section3 from existing register/model; existing SshKexObservation contract | Exact mlkem768nistp256-sha256, mlkem1024nistp384-sha384, mlkem768x25519-sha256 methods retain components/protocol provenance. Collector-reported pin verification, KEX, host signature and no-user-auth gate remain separate. Existing sntrup vendor names retain vendor/unknown-primitive limits, no invented final-standard equivalence | A16 KEX; no new backend, authentication support or provider claim |
| F11 symmetric/hash concerns | Frozen task056 FIPS/FAQ/BHT1997/RFC6151 facts and existing assessment.rs query model | AES/hash/preimage/collision models remain separate; no public-key break claim for AES128, no KEM-as-hash replacement, no query fraction as operational security bits. Collision concern depends on relevant use | A07/A09 controls; unknown use cannot fabricate a weakness |
| F12 capture vs inference vs claims | Inventory and local assessment v1 contracts, runtime envelopes, task065 per-rule limits | Lexical/AST/config/reference/declaration/resolution/parsed fields/advertisement/reported negotiation/operator/external-review claim remain distinguishable. No origin signature is added by hashing a saved file | A01/A04/A06/A11/A15/A16; unknown or partial capture does not erase a known concern |
| F13 lifetimes and deadlines | Existing standards register: IR8547 Final2026-10-27 sections2.2/3.1/4.1, transition context; RFC9954 section1.2, retrospective-decryption motivation | Separate confidentiality, historical ciphertext, signature-verification lifetime, certificate validity, migration lead and sourced applicable deadline. No universal federal deadline or quantum arrival parameter. Local comparison D <= as_of+L is explicit planning pressure, not a quantum-break equation | A03/A08/A09/A13; policy source is D2, not an external mandated ranking |
| F14 local decisions | Human-approved task066 policy table/claim contract, after D1/D2; no normative cryptographic source prescribes this organization's urgency | P1-P8, no-low-rank unknowns, readiness independence, claim scope/freshness and actions are disclosed local advisory decisions. External source facts do not establish practitioner usefulness | All A-cases; task071 separately evaluates usefulness |

FIPS parameter tables were visually checked on the stated complete PDF pages;
parameter labels/purposes are adopted, not their internal cryptographic algorithms.
Current acquired bodies and receipts preserve source bytes. Table names/security
categories are not converted into product scores or measured probability.

## Errata and revision treatment

The existing task056 [FIPS applicability record](../056/evidence/assessment-20261115/ERRATA-APPLICABILITY.json)
reviews14 acquired potential corrections (2 FIPS203 and12 FIPS204). Those internal
pseudocode/notation corrections did not change its adopted purpose/parameter-name
facts. Reuse that bounded disposition for F02/F03; do not call potential updates
enacted normative amendments. Current official FIPS publication pages retain the
203 potential-update notice dated2025-11-17 and204 notice dated2026-07-31. No
cryptographic implementation is being validated or copied here.

Task056's [RFC applicability record](../056/evidence/assessment-20261115/RFC-ERRATA-APPLICABILITY.json)
retains43 selected records and their Verified/Held/Reported/Rejected statuses for
RFC4055/5280/7519/8422. Its acquired snapshot had none for6151/8725/9325; this is
historical snapshot scope, not a new assertion of no errata. PSS/path/name/extension
corrections do not turn captured OIDs into validation. Exact reused source hashes
and local-link/current-source applicability are recorded in task066 evidence.

New TLS/SSH references have separate
[errata retrieval/dispositions](evidence/official/ERRATA-RECEIPTS.json).
RFC9954 verified editorial erratum9136 corrects TLS13 KeyShare section references
to4.3.8 in RFC9846, not the old4.2.8 numbering. Use the corrected pointer in F07;
it does not alter group IDs or implement key-share cryptography. RFC9846 explicitly
obsoletes8446; the new technical mapping uses its current section numbering while
old v1 sources/results remain pinned and unchanged. Network retrieval failures
are preserved, never relabeled as an empty errata set. All acquired records were dispositioned: RFC10024 has no matching records in
this snapshot; RFC10042 has Verified9159 (server abort responsibility) and9160
(PQ/T typo); RFC9954 has Verified9136; RFC9846 has Verified9040 (TLS1.2 fallback)
and Reported9042/9043/9161/9157 (downgrade wording, CertificateRequest syntax and
Main Secret terminology). Wire negotiation/key-share/CertificateRequest processing
is outside the offline adapter. Reported corrections are not enacted amendments.
No acquired record changes the selected numeric group/component facts. Any later
unresolved relevant erratum blocks affected mapping approval/implementation rather
than silently adopting speculative semantics.

## Proposed v1-preserving dispatch and implementation checks

`migration-facts-v1` is a separately selected technical contract, not a mutation
of `local-assessment-1`. Reuse primitive results when identical; compose explicit
protocol/assurance facts only in the new path. Keep source identifiers/editions,
unknowns and component roles in the output. Unknown catalog/major versions reject.
New group/encoding handling cannot change old catalog/policy results through a
shared helper. Task067 must compare old goldens and purpose/parameter/unknown
boundaries under both dispatches and default/affected feature configurations.

Actual saved-input validation requires more than generic `ProbeReport::validate`:
reader DTOs must enforce current schemas, resource caps, redundant flag consistency,
known numeric-ID/name pairs and per-protocol assurance gates before fact projection.
No protocol provider or network execution is needed. Readers for unselected formats
must explicitly reject rather than return negative evidence. No numeric calibration,
rule repair or trust/implementation certification is included.
