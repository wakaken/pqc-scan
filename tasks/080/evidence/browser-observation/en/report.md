# PQC Migration Scan Report

Generated: 2026-11-24 16:30:09 +09:00 (UTC+09:00)

Findings to review (items): 6 / Informational findings (items): 12 / Files inspected: 3

|Severity|Count (all findings)|
|---|---:|
|info|12|
|medium|6|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[API_OPENSSL_RSA_NEW](#rule-0) OpenSSL low-level RSA API usage detected.|medium|quantum-vulnerable|1|1|
|[C_FAMILY_CRYPTO_API](#rule-1) C/C++ cryptographic API or configuration reference; effective primitive and runtime use require veri...|medium|quantum-uncertain|5|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[C_FAMILY_PQC_API](#rule-2) C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active prote...|info|quantum-uncertain|9|3|
|[C_FAMILY_TLS_REFERENCE](#rule-3) C/C++ cryptographic include or TLS library reference; linking and negotiation remain unverified.|info|quantum-uncertain|3|2|

<a id="rule-0"></a>

## API_OPENSSL_RSA_NEW

OpenSSL low-level RSA API usage detected.

Check: Prefer high-level OpenSSL EVP APIs to ease PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[api.c:3:411](#finding-0)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### api.c:3:411 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-0"></a>

### api.c:3:411

    > 3 [masked-sensitive-file-content]

<a id="rule-1"></a>

## C_FAMILY_CRYPTO_API

C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification.

Check: Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[api.c:3:220](#finding-1)|C/C++ provider configuration reference: oqsprovider|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-1)|
|<a id="finding-2"></a>[api.c:3:259](#finding-2)|C/C++ property configuration reference: provider=default|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-2)|
|<a id="finding-3"></a>[api.c:3:311](#finding-3)|C/C++ TLS group configuration reference: X25519MLKEM768:P-256|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-3)|
|<a id="finding-4"></a>[api.c:3:364](#finding-4)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-4)|
|<a id="finding-5"></a>[api.c:3:411](#finding-5)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-5)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### api.c:3:220 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-1"></a>

### api.c:3:220

    > 3 [masked-sensitive-file-content]

### api.c:3:259 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-2"></a>

### api.c:3:259

    > 3 [masked-sensitive-file-content]

### api.c:3:311 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-3"></a>

### api.c:3:311

    > 3 [masked-sensitive-file-content]

### api.c:3:364 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-4"></a>

### api.c:3:364

    > 3 [masked-sensitive-file-content]

### api.c:3:411 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-5"></a>

### api.c:3:411

    > 3 [masked-sensitive-file-content]

<a id="rule-2"></a>

## C_FAMILY_PQC_API

C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection.

Check: Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[api.c:3:28](#finding-6)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-6)|
|<a id="finding-7"></a>[api.c:3:98](#finding-7)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-7)|
|<a id="finding-8"></a>[api.c:3:124](#finding-8)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-8)|
|<a id="finding-9"></a>[api.cpp:2:10](#finding-9)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-9)|
|<a id="finding-10"></a>[api.cpp:2:40](#finding-10)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-10)|
|<a id="finding-11"></a>[api.cpp:2:113](#finding-11)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-11)|
|<a id="finding-12"></a>[api.cpp:2:151](#finding-12)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-12)|
|<a id="finding-13"></a>[common.h:1:21](#finding-13)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-13)|
|<a id="finding-14"></a>[common.h:1:49](#finding-14)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-14)|

### api.c:3:28 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-6"></a>

### api.c:3:28

    > 3 [masked-sensitive-file-content]

### api.c:3:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-7"></a>

### api.c:3:98

    > 3 [masked-sensitive-file-content]

### api.c:3:124 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-8"></a>

### api.c:3:124

    > 3 [masked-sensitive-file-content]

### api.cpp:2:10 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-9"></a>

### api.cpp:2:10

    > 2 [masked-sensitive-file-content]

### api.cpp:2:40 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-10"></a>

### api.cpp:2:40

    > 2 [masked-sensitive-file-content]

### api.cpp:2:113 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-11"></a>

### api.cpp:2:113

    > 2 [masked-sensitive-file-content]

### api.cpp:2:151 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-12"></a>

### api.cpp:2:151

    > 2 [masked-sensitive-file-content]

### common.h:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-13"></a>

### common.h:1:21

    > 1 [masked-sensitive-file-content]

### common.h:1:49 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-14"></a>

### common.h:1:49

    > 1 [masked-sensitive-file-content]

<a id="rule-3"></a>

## C_FAMILY_TLS_REFERENCE

C/C++ cryptographic include or TLS library reference; linking and negotiation remain unverified.

Check: Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-15"></a>[api.c:1:1](#finding-15)|C/C++ OpenSSL include reference|Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms. / [Source](#source-15)|
|<a id="finding-16"></a>[api.c:2:1](#finding-16)|C/C++ liboqs include reference|Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms. / [Source](#source-16)|
|<a id="finding-17"></a>[api.cpp:1:1](#finding-17)|C/C++ liboqs-cpp include reference|Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms. / [Source](#source-17)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### api.c:1:1 — Matched rules

- `C_FAMILY_TLS_REFERENCE` (info): C/C++ cryptographic include or TLS library reference; linking and negotiation remain unverified. — Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms.

<a id="source-15"></a>

### api.c:1:1

    > 1 [masked-sensitive-file-content]

### api.c:2:1 — Matched rules

- `C_FAMILY_TLS_REFERENCE` (info): C/C++ cryptographic include or TLS library reference; linking and negotiation remain unverified. — Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms.

<a id="source-16"></a>

### api.c:2:1

    > 2 [masked-sensitive-file-content]

### api.cpp:1:1 — Matched rules

- `C_FAMILY_TLS_REFERENCE` (info): C/C++ cryptographic include or TLS library reference; linking and negotiation remain unverified. — Check included libraries, exact TLS groups and provider configuration separately from deployment and negotiated connections. Includes alone do not prove linking or enabled algorithms.

<a id="source-17"></a>

### api.cpp:1:1

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

- [CBOM](cbom.json): 13
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|3|
|Read successfully|3|
|Read failed|0|
|Analyzed|3|
|Partial|0|
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

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-e4e32974ccc6d905029a1fb39653e78f67ff7d0a6b79c6215262d9786a389211|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e49bf962a318a3dc52fb2a989ae074beb58adfea19aead3efeff0f33401a3cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-45ffac493ea739ddef511eb4cc106cebc6d8a89571684de5dff4dc5e8a32d0a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-10d920bf5f87444f8c0706203e79885862041ed1bb3b133385eae3fb9db79364|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cfb491a769e753b72b0196ba787edf7c4f78f35f4fb8198cbf63188bd3df36ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd8d37c4b2c191cfc60323940584e2cd7fa30346defe7c3a84a11d1a9b492dce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5e30dbf9dd45b395716ba747bbae73b08ec22a084c9594749afca00c6da0fa0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f6867b5251441a652960516cd0e629b21ceb86dc9d4f99a7e80fc66f85ce518f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea405bc5dbf41e5b90e15c1e3634f4389c2370a5169978550d90eaa030ae8f54|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2851a4bdb1491abca222929eb4e7af0dcb1017c648d9b598f05659b98907f5b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-512507d01b7b3fc5cc42125acc26ab60f61486680d2e4de5fb3b5f4af6e36dba|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1de3e1b0d1516ae42286dba7f789e3bac340e1055c060265aa20c535f4219579|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-109e70f16e9ddf91095188e0365a080a085f09eb193ec70fd7e382e0f1525424|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5c997fb23ab070f4551206d8a1b047ecb901db843e74c6633fdf6f6dee0e65fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4044a4007d8b2e3b7d54e3bd39916b55b8c5151f6e00005ac4dfe85b5d0e34ae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9802d1cd64ec57626320ebcf974390a234a7c44302098408245f7dd978b5f41d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-560210e00b261c697cbce8e0f1d7e8569a94727f6372e96a1d93fbb9e6d7fc54|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31432b859bb2b87e244ee6f64d1aff1b36a5153aa9c162ce6c914a88b491834d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
