# PQC Migration Scan Report

Generated: 2026-11-24 16:07:17 +09:00 (UTC+09:00)

Findings to review (items): 6 / Informational findings (items): 1 / Files inspected: 3

|Severity|Count (all findings)|
|---|---:|
|info|1|
|medium|6|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[API_OPENSSL_RSA_NEW](#rule-0) OpenSSL low-level RSA API usage detected.|medium|quantum-vulnerable|4|2|
|[C_FAMILY_CRYPTO_API](#rule-1) C/C++ cryptographic API or configuration reference; effective primitive and runtime use require veri...|medium|quantum-uncertain|2|2|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[C_FAMILY_PQC_API](#rule-2) C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active prote...|info|quantum-uncertain|1|1|

<a id="rule-0"></a>

## API_OPENSSL_RSA_NEW

OpenSSL low-level RSA API usage detected.

Check: Prefer high-level OpenSSL EVP APIs to ease PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[template-rsa.cpp:1:16](#finding-0)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-0)|
|<a id="finding-1"></a>[template-rsa.cpp:1:34](#finding-1)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-1)|
|<a id="finding-2"></a>[using.cpp:1:15](#finding-2)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-2)|
|<a id="finding-3"></a>[using.cpp:1:33](#finding-3)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-3)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### template-rsa.cpp:1:16 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-0"></a>

### template-rsa.cpp:1:16

    > 1 [masked-sensitive-file-content]

### template-rsa.cpp:1:34 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-1"></a>

### template-rsa.cpp:1:34

    > 1 [masked-sensitive-file-content]

### using.cpp:1:15 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-2"></a>

### using.cpp:1:15

    > 1 [masked-sensitive-file-content]

### using.cpp:1:33 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-3"></a>

### using.cpp:1:33

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## C_FAMILY_CRYPTO_API

C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification.

Check: Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-4"></a>[template-rsa.cpp:1:34](#finding-4)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-4)|
|<a id="finding-5"></a>[using.cpp:1:33](#finding-5)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-5)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### template-rsa.cpp:1:34 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-4"></a>

### template-rsa.cpp:1:34

    > 1 [masked-sensitive-file-content]

### using.cpp:1:33 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-5"></a>

### using.cpp:1:33

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## C_FAMILY_PQC_API

C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection.

Check: Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[template.cpp:1:39](#finding-6)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-6)|

### template.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-6"></a>

### template.cpp:1:39

    > 1 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Adopt a phased hybrid PQC migration plan (near-term)

The detected primitive is vulnerable to cryptographically relevant quantum attacks. Hybrid rollout minimizes compatibility risk.

1. Inventory all call sites using this algorithm and owner teams.
2. Check for an existing classical + PQC composition; if migration is needed, evaluate compatible staged or hybrid modes before hard cut-over.
3. Define retirement dates for legacy classical algorithms.

References:

- https://www.nist.gov/itl/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
<a id="action-1"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography

## Inventory exports

- [CBOM](cbom.json): 5
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|3|
|Read successfully|3|
|Read failed|0|
|Analyzed|1|
|Partial|2|
|Analysis failed|0|
|Unsupported|0|
|Not applicable|0|
|Unknown|0|
|Enumerated exclusions|0|
|Enumeration failures|0|
|Unenumerated population|Unknown|

## Inspection diagnostics

Counts and up to three example paths per reason; full records remain in diagnostics JSON.

|Item|Value|
|---|---|
|Only part of inspection completed (detector_partial): Source syntax could not be fully interpreted. Parsed regions were inspected; unresolved syntax remains unknown. Check generated syntax, language version and the source file separately.|2 records; template-rsa.cpp; template.cpp|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-a1d6f1450d6cef74029888501c61d501663ca85bce88609de614d683ef82916e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05f4cfe5e61f5b1350d3863d0763598c884611821c0e852085e5efbdc7be406d|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7235de44eb898b3e371ee06b5ac228f8e4863d2846dca3b58b56a8ed5d6bed25|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e83e2da662a6820174fb6a5cacf38faf44dae052cb3651c0eca611fee70d4d86|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57a43e0055ee0625fcf0993fa459215695553705728581853c20ca74c2485177|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dc9f89bfba753c7294470bea21198e8bffb6a79e7ab3e73a10cf5fcc9c4675f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0e3c1a615ae0414e7ff2aaab0b2e14e826f6c969ddf0a6fe688444d6da897650|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
