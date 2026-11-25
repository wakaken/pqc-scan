# PQC Migration Scan Report

Generated: 2026-11-24 16:30:07 +09:00 (UTC+09:00)

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
|observation-v1-cdfa00e57263f35a829b96e4e6f65f14d07be42290030750944b7a5ddb2f40b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fdf5f3a69951c34d426ab057eaf6eb8b13cd409aa1858b627463135419b5d94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fadc53fbbbf83172651ccac9678f7f39d148672e9be73b043b3739e3e3568f46|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a53c35dc6efcfc01e68839da976a298b5f02010243ee51f5e39fcf9e87a44b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-56dc4cfa6a59bd91484391c2540ce35b5ce613d38f13fc4342417748ff78ec1a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6ad2cc1560bf9989b639c017bb78df878de9bb04eabe3b65636d9f7b699d947a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f22750cdea5f3028441465af6fb572d406862d9786fe37551c1b2cc7b0dd48c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a30f74e85e6f529f90b3b7e5515ded5cc21958310dc347eea37a247b08235af9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3c5a27f14411c4ce9a565140f9db00f09ef24495c99da85640b9d96b1a9ae12|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fdc1c0d5e3910385e985b531713c9c7c1b7916ff27b0e529bbcd1624ef7f51f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8720b73d4a88a86d4baa2b30099898497f3fed5e0d5264151a28dd9585fe16d2|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1bef51794ff4a49da9b7b9a773dd61e6cd0cdde9f9f99d939af7140569cf2acc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9eea2ea2f7b27c80826390ca32ceab71ac95aff292070c7a231f27015bd1ace|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93fe98067c4a0256336e374366883af9a74b103c49059cf8d8ef473222c9e015|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f41ee0413f00f6455629f1aaf5b5dbf8ebf2ad79359eeda2dbe6bc00a1dd6d7d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f8c91704e64d6466513cf5b269e6a9c547e984731dd7b6f787efe22c12b5741|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4145a057ef94cab5e769132f11865d7bd4afdd95831cfaa17f51af468c752d0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13bd7b0a4c0d323ed716289596aa6db3d3eb040cfc8286662c5c55887c6bcb9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
