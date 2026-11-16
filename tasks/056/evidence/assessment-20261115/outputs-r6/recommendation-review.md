# Recommendation content and routing review

Evaluation complete against the current source; both independent reviews accepted.
The product was not corrected as part of this validation request.

Automatic checks exercise category/risk/evidence routing, all six language
families, file-extension fallback, special reference IDs, three explicit PQC
configuration overrides, eight middleware targets, and older/current/unknown
runtime hints. Each individually false/missing runtime, provider, parameter,
peer, protocol, JOSE/verifier and PKI support prerequisite prevents bounded Pass.
Source declarations and primitive availability do not establish deployment.
The 140 original characterization inputs are reassessed unchanged for routing;
action-ID agreement is not evidence that their prose is correct.

The main agent separately reviewed all38 unique complete action JSON payloads,
including code examples, by payload hash against the acquired primary source
editions. This is post-observation manual review, disclosed separately from the
frozen oracle. `RECOMMENDATION-SEMANTICS.json` contains each rationale, source
mapping, affected case IDs and verdict.32 payloads have no identified semantic
finding;6 payloads expose5 distinct findings:

| Finding | Observed problem | Required condition missing or misstated |
|---|---|---|
| F056-01 | Java version unknown receives rationale that an older version was detected | Unknown version must remain unknown; JEP496/497 availability begins at JDK24 |
| F056-02 | BC example explicitly selects ML-DSA with BCPQC | BC Java1.85 registers MLDSA in BC; its BCPQC provider list does not load it. A supported version/provider and matching keys must be identified |
| F056-03 | Java JCA before/after changes RSA to ML-DSA while retaining privateKey | Migration advice must require a new algorithm-compatible ML-DSA key pair and verifier encoding/rollout; an RSA key cannot initialize ML-DSA |
| F056-04 | PKI advice proposes a hybrid-chain pilot without identifying construction/encoding availability | FIPS204/205 standardize primitives, not a universal hybrid certificate format. CA/chain/trust-store/provider support must be checked separately |
| F056-05 | Two Envoy actions refer broadly to RSA-dependent/legacy RSA suites | Static RSA key exchange differs from ECDHE_RSA authentication. Envoy cipher_suites controls TLS1.0–1.2, not TLS1.3; peers/certificates need separate review |

The remaining actions preserve KEM versus signature purpose, conditional runtime
support, provider/features and peer interoperability, JOSE identifiers/encoding
and all verifiers, and nonproduction PKI/middleware pilots. Go1.24 ML-KEM versus
1.27 ML-DSA, Node crypto24.6/24.7/24.8 versus WebCrypto24.7, cryptography47
AWS-LC/BoringSSL versus48 OpenSSL3.5, rustls0.23.22/0.23.27 aws-lc-rs defaults,
and Ruby/OpenSSL gem4 constraints use acquired edition-specific facts.
Java unknown-version advice and examples are findings, not silently accepted.

Illustrative snippets and suggested service abstractions were not compiled or
executed, nor treated as verified cryptographic implementations. This review
assesses prerequisites and factual guidance; it does not certify production
support. Fixing findings or defining new cryptographic policy requires separate
scope/human review. No follow-up task was automatically created.
