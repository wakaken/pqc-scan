# PQC Migration Scan Report

Generated: 2026-11-24 16:06:27 +09:00 (UTC+09:00)

Findings to review (items): 8 / Informational findings (items): 12 / Files inspected: 6

|Severity|Count (all findings)|
|---|---:|
|info|12|
|medium|8|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[C_FAMILY_CRYPTO_API](#rule-0) C/C++ cryptographic API or configuration reference; effective primitive and runtime use require veri...|medium|quantum-uncertain|8|4|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[C_FAMILY_PQC_API](#rule-1) C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active prote...|info|quantum-uncertain|12|6|

<a id="rule-0"></a>

## C_FAMILY_CRYPTO_API

C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification.

Check: Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[digest-out.c:1:80](#finding-0)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-0)|
|<a id="finding-1"></a>[digest-out.cpp:1:80](#finding-1)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-1)|
|<a id="finding-2"></a>[pointer-out.c:1:28](#finding-2)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-2)|
|<a id="finding-3"></a>[pointer-out.c:1:93](#finding-3)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-3)|
|<a id="finding-4"></a>[pointer-out.c:1:242](#finding-4)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-4)|
|<a id="finding-5"></a>[pointer-out.cpp:1:28](#finding-5)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-5)|
|<a id="finding-6"></a>[pointer-out.cpp:1:93](#finding-6)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-6)|
|<a id="finding-7"></a>[pointer-out.cpp:1:242](#finding-7)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-7)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### digest-out.c:1:80 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-0"></a>

### digest-out.c:1:80

    > 1 [masked-sensitive-file-content]

### digest-out.cpp:1:80 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-1"></a>

### digest-out.cpp:1:80

    > 1 [masked-sensitive-file-content]

### pointer-out.c:1:28 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-2"></a>

### pointer-out.c:1:28

    > 1 [masked-sensitive-file-content]

### pointer-out.c:1:93 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-3"></a>

### pointer-out.c:1:93

    > 1 [masked-sensitive-file-content]

### pointer-out.c:1:242 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-4"></a>

### pointer-out.c:1:242

    > 1 [masked-sensitive-file-content]

### pointer-out.cpp:1:28 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-5"></a>

### pointer-out.cpp:1:28

    > 1 [masked-sensitive-file-content]

### pointer-out.cpp:1:93 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-6"></a>

### pointer-out.cpp:1:93

    > 1 [masked-sensitive-file-content]

### pointer-out.cpp:1:242 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-7"></a>

### pointer-out.cpp:1:242

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## C_FAMILY_PQC_API

C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection.

Check: Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-8"></a>[bad-case.c:1:21](#finding-8)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-8)|
|<a id="finding-9"></a>[bad-case.c:1:60](#finding-9)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-9)|
|<a id="finding-10"></a>[bad-case.cpp:1:21](#finding-10)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-10)|
|<a id="finding-11"></a>[bad-case.cpp:1:60](#finding-11)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-11)|
|<a id="finding-12"></a>[digest-out.c:1:28](#finding-12)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-12)|
|<a id="finding-13"></a>[digest-out.c:1:128](#finding-13)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-13)|
|<a id="finding-14"></a>[digest-out.cpp:1:28](#finding-14)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-14)|
|<a id="finding-15"></a>[digest-out.cpp:1:128](#finding-15)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-15)|
|<a id="finding-16"></a>[pointer-out.c:1:140](#finding-16)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-16)|
|<a id="finding-17"></a>[pointer-out.c:1:213](#finding-17)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-17)|
|<a id="finding-18"></a>[pointer-out.cpp:1:140](#finding-18)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-18)|
|<a id="finding-19"></a>[pointer-out.cpp:1:213](#finding-19)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-19)|

### bad-case.c:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-8"></a>

### bad-case.c:1:21

    > 1 [masked-sensitive-file-content]

### bad-case.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-9"></a>

### bad-case.c:1:60

    > 1 [masked-sensitive-file-content]

### bad-case.cpp:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-10"></a>

### bad-case.cpp:1:21

    > 1 [masked-sensitive-file-content]

### bad-case.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-11"></a>

### bad-case.cpp:1:60

    > 1 [masked-sensitive-file-content]

### digest-out.c:1:28 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-12"></a>

### digest-out.c:1:28

    > 1 [masked-sensitive-file-content]

### digest-out.c:1:128 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-13"></a>

### digest-out.c:1:128

    > 1 [masked-sensitive-file-content]

### digest-out.cpp:1:28 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-14"></a>

### digest-out.cpp:1:28

    > 1 [masked-sensitive-file-content]

### digest-out.cpp:1:128 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-15"></a>

### digest-out.cpp:1:128

    > 1 [masked-sensitive-file-content]

### pointer-out.c:1:140 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-16"></a>

### pointer-out.c:1:140

    > 1 [masked-sensitive-file-content]

### pointer-out.c:1:213 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-17"></a>

### pointer-out.c:1:213

    > 1 [masked-sensitive-file-content]

### pointer-out.cpp:1:140 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-18"></a>

### pointer-out.cpp:1:140

    > 1 [masked-sensitive-file-content]

### pointer-out.cpp:1:213 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-19"></a>

### pointer-out.cpp:1:213

    > 1 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography

## Inventory exports

- [CBOM](cbom.json): 10
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|6|
|Read successfully|6|
|Read failed|0|
|Analyzed|6|
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
|observation-v1-8fba58530f15d03a8966519069c88895bf44dc1a82518b84740e07f4f649d59a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4ba3a769e0fcb1ce38a4c7a25c56c676432711c598da6fd7fdcffb85aaa14c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40e2391579c78acbfc62b133407eb0b820a934108a83586e4e40f7e6d96f8d94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41b44495613b1137fb1a1f260496e4e6b94931c4555e2ab25dcaa1353242f29a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-95298781a23ca6637ecea25fd35ccf44ffe302814bed57f73360ce6a91d62e03|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4df8fc3bdaafc1193be0fe7fd52ca293ef5b4e747e5ab0414786262c307ce5d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3482b9e01c037eb2010a514f7f4db1c9ebbf5bc3940baffd7d6e7204b06e563c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-21da685cda00b5a6de04f3648ce4fb87db820bc631d8a5d5b627eee6e53634b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4209ff8ba46162652c9849d3ae22b229c74277700e0196ad02d4332694c5acbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a67ecf8baec350c7cad4719f3fb95667c989e74ef4c26dc795f8a53e835ffc27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9666cba774bf98bdf7cd45aa77bd840442c3b8c059dd4830724a7ab0c1521cec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-979b2e4e03fc08ad40b8afae239b1054f6281d8a37b306e3ed2889601060728b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-204f03cea6d989c021c0163841d84a4310bb2480eb2d060a198775547df278e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ca7896959a48c8a1b95af72a42bf6f49b961c799cdc26ae078f1cc57c7c6e1e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-afbec86ddef5cac9bd3556fa5fd1e3f77babb4cffdd1c5d239a9f6f574d91ed0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c36326701df98a4469aa270f4ad02f135f78bd9f618355f77f7592c92eeda3bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-199d285ac84ee08c2c51ec21babf1f74d7d20549473a5281b29462d0fff3e6c7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69ff6ea42e5d3316edde009af2f3068011368657dd6d802131cf61f5648e719b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3cb008ead677443bcc0c51e0696d36d21e77e3b5a85af87467ba4cb61e9ec2df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c95d1cd4f2aa2d8a52cd1e000adfa2586313c3d706f28f79ee4af5ca2641a0e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
