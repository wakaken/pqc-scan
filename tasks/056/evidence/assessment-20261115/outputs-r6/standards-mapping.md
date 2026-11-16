# Standards and judgment applicability mapping

Evaluation applicability mapping complete; both independent reviews accepted. Official sources were retrieved on 2026-11-15 JST; URLs, HTTP
results and exact bytes are bound in `official/RECEIPTS.json` and numbered-purpose
receipts under the owning task evidence directory. This is judgment validation,
not validation of a cryptographic implementation or legal compliance.

| Reference | Edition and source | Applied judgment | Limits / remaining work |
|---|---|---|---|
| S203 | [FIPS203 final, 2024-08-13](https://csrc.nist.gov/pubs/fips/203/final) | ML-KEM-512/768/1024 are key establishment, not signatures | Potential-update notice dated 2025-11-17 confirmed. Initial spreadsheet GET403 was resolved with a normal browser User-Agent/Referer; two recorded potential corrections concern internal pseudocode/comments, not algorithm purpose/parameter-set names |
| S204 | [FIPS204 final, 2024-08-13](https://csrc.nist.gov/pubs/fips/204/final) | ML-DSA-44/65/87 are signatures | Potential-update notice dated 2026-07-31 confirmed. Twelve recorded potential corrections were acquired and reviewed; they concern notation/internal algorithm details rather than the classifier's names/purposes |
| S205 | [FIPS205 final, 2024-08-13](https://csrc.nist.gov/pubs/fips/205/final) | Twelve SLH-DSA SHA2/SHAKE parameter names are signatures | A named primitive is not provider/protocol/peer support; unlisted encodings stay unassessed/unknown |
| CSOR | [NIST algorithm registration](https://csrc.nist.gov/projects/computer-security-objects-register/algorithm-registration), acquired snapshot | Exact pure PQC OIDs and parameter-set identity | Hash/prehash encodings are not silently equated; namespace/parameter interpretation is separate |
| S8547 | [IR8547 final, October 2026](https://csrc.nist.gov/pubs/ir/8547/final) | Public-key transition context | Final report is not the 2024 initial draft; federal transition context does not create a universal project deadline or new hard gate |
| S131A2 | [SP800-131A Rev2](https://csrc.nist.gov/pubs/sp/800/131/a/r2/final), 2019 | Frozen local RSA2048 / EC224 reference thresholds | Thresholds belong to local advisory profile; no current-law/sufficient-strength claim for every use |
| NIST-PQ-FAQ | [NIST PQC FAQ](https://csrc.nist.gov/projects/post-quantum-cryptography/faqs), acquired snapshot | Public-key vulnerability vs generic symmetric/hash search | Ideal query counts are not operational attack cost, calibrated security bits or NIST categories; AES128 is not declared broken |
| BHT1997 | [Original collision paper](https://arxiv.org/abs/quant-ph/9705002), referenced by existing register | Generic collision model is separate from preimage | Original abstract checked: black-box r-to-one collision queries scale with the cube root and have a space-time tradeoff; not a physical resource estimate or a reimplementation proof |
| R6151 | [RFC6151](https://www.rfc-editor.org/rfc/rfc6151.html), 2011 | MD5 collision concern | Not every HMAC-MD5 use has the same collision attack |
| R4055 | [RFC4055](https://www.rfc-editor.org/rfc/rfc4055.html), 2005 | RSA-PSS signature purpose vs unrestricted RSA | Does not supply an unimplemented PSS parameter interpretation |
| R8725 / R7519 | [RFC8725](https://www.rfc-editor.org/rfc/rfc8725.html), 2020 / [RFC7519](https://www.rfc-editor.org/rfc/rfc7519.html), 2015 | JOSE identifier/encoding and verifier rollout prerequisites; no KEM-for-signature substitution | Primitive availability alone is not a complete JOSE rollout; all43 relevant RFC errata records in the acquired official snapshot were dispositioned separately; no JWT validation is performed |
| R5280 | [RFC5280](https://www.rfc-editor.org/rfc/rfc5280.html), 2008 | Captured-time validity and certificate role separation | OID/key size/time do not verify signatures, chain, trust or mathematical correctness |
| LOCAL026 / ARCH | `docs/architecture/LOCAL-ASSESSMENT-V1.md`, `docs/architecture.md` | Advisory applicability/order, support claims, uncertainty and documented legacy severity strategy | Local policy is not an external cryptographic standard; exact0.45 preservation is an explicit maintainer compatibility requirement, not empirical optimality |

Per-case source IDs are present in the canonical oracle. `SOURCE-REGISTER.json`
binds all source IDs (including aliases) to exact acquired/current bytes and
edition scope. `DECISION-COVERAGE.json` maps eighteen inspected semantic decision
classes to explicit case IDs; it is not instrumented branch coverage or every
arbitrary input combination. Local policy is distinguished from external norms.
IR8547 supplies transition context, not a universal deadline or extra Pass rule.

`ERRATA-APPLICABILITY.json` reviews all fourteen acquired FIPS203/204 potential
corrections. They affect internal algorithms/notation, not the purposes/parameter
names tested here; no cryptographic implementation is validated. The original
GET403 and successful User-Agent/Referer retry receipts are both retained.
`RFC-ERRATA-APPLICABILITY.json` dispositions all43 records for RFC4055/5280/7519/
8422, retaining Verified/Held/Reported/Rejected statuses. The acquired snapshot
contains no records for RFC6151/8725/9325. Erratum8060 reinforces JOSE header
validation; the local evaluator does not validate JWTs. RSA-PSS parameter and
X.509 path/name/extension corrections do not turn extraction into verification.
Reported/potential corrections are not silently enacted normative amendments.

Additional primary sources are bound in `official/SUPPLEMENT-RECEIPTS.json`:
Cargo feature/resolution documentation separates lock/declaration from enabled
execution; RFC9325 and Envoy TLS configuration separate RSA authentication from
key exchange; BC Java r1rv85 provider registration establishes the recommendation
example finding. Runtime editions and old-version boundaries are recorded in
`official/RUNTIME-RECEIPTS.json` and the source register. Acquired documentation
is evidence of public capabilities, never evidence of this repository's deployed
provider, actual peer support or successful cryptographic operation.
