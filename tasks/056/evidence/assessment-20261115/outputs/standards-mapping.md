# Standards and judgment applicability mapping

Status: Partial. Official sources were retrieved on 2026-11-15 JST; URLs, HTTP
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
| R8725 / R7519 | [RFC8725](https://www.rfc-editor.org/rfc/rfc8725.html), 2020 / [RFC7519](https://www.rfc-editor.org/rfc/rfc7519.html), 2015 | JOSE identifier/encoding and verifier rollout prerequisites; no KEM-for-signature substitution | Primitive availability alone is not a complete JOSE rollout; latest extension/errata coverage remains incomplete |
| R5280 | [RFC5280](https://www.rfc-editor.org/rfc/rfc5280.html), 2008 | Captured-time validity and certificate role separation | OID/key size/time do not verify signatures, chain, trust or mathematical correctness |
| LOCAL026 / ARCH | `docs/architecture/LOCAL-ASSESSMENT-V1.md`, `docs/architecture.md` | Advisory applicability/order, support claims, uncertainty and documented legacy severity strategy | Local policy is not an external cryptographic standard; exact undocumented/implementation-derived behavior is not independent gold |

Per-case source IDs are present in `assessment-oracle.json`. Unselected judgment combinations and incomplete full-implementation/model
coverage prevent a full all-judgment normative-conformance claim. The fourteen
acquired potential corrections do not change the purposes/parameter names tested
here, and this classifier does not implement their affected internal algorithms.
They are potential updates, not automatically enacted new technical requirements. No standards register,
crypto decision, severity meaning or public contract was changed.
