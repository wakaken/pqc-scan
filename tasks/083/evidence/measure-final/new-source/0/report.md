# PQC Migration Scan Report

Generated: 2026-11-24 18:36:13 +09:00 (UTC+09:00)

Findings to review (items): 241 / Informational findings (items): 652 / Files inspected: 882

|Severity|Count (all findings)|
|---|---:|
|high|12|
|info|652|
|medium|229|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[OPENSSL_RSA_API](#rule-0) OpenSSL RSA API call detected.|high|quantum-vulnerable|12|12|
|[API_EVP_PKEY_RSA](#rule-1) EVP_PKEY configured for RSA detected.|medium|quantum-vulnerable|16|16|
|[API_OPENSSL_RSA_NEW](#rule-2) OpenSSL low-level RSA API usage detected.|medium|quantum-vulnerable|8|8|
|[CSHARP_CRYPTO_API](#rule-3) C# cryptographic API or qualified library reference; effective key use requires verification.|medium|quantum-uncertain|47|32|
|[C_FAMILY_CRYPTO_API](#rule-4) C/C++ cryptographic API or configuration reference; effective primitive and runtime use require veri...|medium|quantum-uncertain|120|88|
|[PHP_CRYPTO_API](#rule-5) PHP cryptographic API or library reference; key type may be unresolved.|medium|quantum-uncertain|38|30|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[CSHARP_CRYPTO_REFERENCE](#rule-6) C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks.|info|quantum-uncertain|2|2|
|[CSHARP_PQC_API](#rule-7) C# PQC or composite/HPKE reference; source alone does not establish platform support or active prote...|info|quantum-uncertain|147|101|
|[C_FAMILY_PQC_API](#rule-8) C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active prote...|info|quantum-uncertain|399|241|
|[PHP_CRYPTO_REFERENCE](#rule-9) PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved.|info|quantum-uncertain|2|2|
|[PHP_PQC_API](#rule-10) PHP PQC algorithm or X-Wing reference; runtime support requires verification.|info|quantum-safe|102|74|

<a id="rule-0"></a>

## OPENSSL_RSA_API

OpenSSL RSA API call detected.

Check: Replace with EVP_PKEY-based implementation suitable for PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[c-openssl-rsa_generate_key_ex-negative-comment.c:1:4](#finding-0)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-0)|
|<a id="finding-1"></a>[c-openssl-rsa_generate_key_ex-positive.c:1:21](#finding-1)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-1)|
|<a id="finding-2"></a>[c-openssl-rsa_private_decrypt-negative-comment.c:1:4](#finding-2)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-2)|
|<a id="finding-3"></a>[c-openssl-rsa_private_decrypt-positive.c:1:21](#finding-3)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-3)|
|<a id="finding-4"></a>[c-openssl-rsa_public_encrypt-negative-comment.c:1:4](#finding-4)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-4)|
|<a id="finding-5"></a>[c-openssl-rsa_public_encrypt-positive.c:1:21](#finding-5)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-5)|
|<a id="finding-6"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4](#finding-6)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-6)|
|<a id="finding-7"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21](#finding-7)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-7)|
|<a id="finding-8"></a>[cpp-shared-c-openssl-rsa_private_decrypt-negative-comment.cpp:1:4](#finding-8)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-8)|
|<a id="finding-9"></a>[cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21](#finding-9)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-9)|
|<a id="finding-10"></a>[cpp-shared-c-openssl-rsa_public_encrypt-negative-comment.cpp:1:4](#finding-10)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-10)|
|<a id="finding-11"></a>[cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21](#finding-11)|\[masked-sensitive-content\]|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-11)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-0)
- [Adopt a phased hybrid PQC migration plan](#action-1)
- [Review the TLS reference before changing effective settings](#action-2)

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-0"></a>

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_generate_key_ex-positive.c:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-1"></a>

### c-openssl-rsa_generate_key_ex-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_private_decrypt-negative-comment.c:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-2"></a>

### c-openssl-rsa_private_decrypt-negative-comment.c:1:4

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_private_decrypt-positive.c:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-3"></a>

### c-openssl-rsa_private_decrypt-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_public_encrypt-negative-comment.c:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-4"></a>

### c-openssl-rsa_public_encrypt-negative-comment.c:1:4

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_public_encrypt-positive.c:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-5"></a>

### c-openssl-rsa_public_encrypt-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-6"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-7"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_private_decrypt-negative-comment.cpp:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-8"></a>

### cpp-shared-c-openssl-rsa_private_decrypt-negative-comment.cpp:1:4

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-9"></a>

### cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_public_encrypt-negative-comment.cpp:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-10"></a>

### cpp-shared-c-openssl-rsa_public_encrypt-negative-comment.cpp:1:4

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-11"></a>

### cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## API_EVP_PKEY_RSA

EVP_PKEY configured for RSA detected.

Check: Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-12"></a>[c-evp-id-rsa-negative-comment.c:1:62](#finding-12)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-12)|
|<a id="finding-13"></a>[c-evp-id-rsa.c:1:59](#finding-13)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-13)|
|<a id="finding-14"></a>[c-known-evp_digestsigninit-negative-comment.c:1:64](#finding-14)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-14)|
|<a id="finding-15"></a>[c-known-evp_digestsigninit-reassigned.c:1:61](#finding-15)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-15)|
|<a id="finding-16"></a>[c-known-evp_digestsigninit.c:1:61](#finding-16)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-16)|
|<a id="finding-17"></a>[c-known-evp_digestverifyinit-negative-comment.c:1:64](#finding-17)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-17)|
|<a id="finding-18"></a>[c-known-evp_digestverifyinit-reassigned.c:1:61](#finding-18)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-18)|
|<a id="finding-19"></a>[c-known-evp_digestverifyinit.c:1:61](#finding-19)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-19)|
|<a id="finding-20"></a>[cpp-known-evp_digestsigninit-negative-comment.cpp:1:64](#finding-20)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-20)|
|<a id="finding-21"></a>[cpp-known-evp_digestsigninit-reassigned.cpp:1:61](#finding-21)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-21)|
|<a id="finding-22"></a>[cpp-known-evp_digestsigninit.cpp:1:61](#finding-22)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-22)|
|<a id="finding-23"></a>[cpp-known-evp_digestverifyinit-negative-comment.cpp:1:64](#finding-23)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-23)|
|<a id="finding-24"></a>[cpp-known-evp_digestverifyinit-reassigned.cpp:1:61](#finding-24)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-24)|
|<a id="finding-25"></a>[cpp-known-evp_digestverifyinit.cpp:1:61](#finding-25)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-25)|
|<a id="finding-26"></a>[cpp-shared-c-evp-id-rsa-negative-comment.cpp:1:62](#finding-26)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-26)|
|<a id="finding-27"></a>[cpp-shared-c-evp-id-rsa.cpp:1:59](#finding-27)|\[masked-sensitive-content\]|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-27)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-0)
- [Adopt a phased hybrid PQC migration plan](#action-1)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### c-evp-id-rsa-negative-comment.c:1:62 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-12"></a>

### c-evp-id-rsa-negative-comment.c:1:62

    > 1 [masked-sensitive-file-content]

### c-evp-id-rsa.c:1:59 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-13"></a>

### c-evp-id-rsa.c:1:59

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-negative-comment.c:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-14"></a>

### c-known-evp_digestsigninit-negative-comment.c:1:64

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-reassigned.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-15"></a>

### c-known-evp_digestsigninit-reassigned.c:1:61

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-16"></a>

### c-known-evp_digestsigninit.c:1:61

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-negative-comment.c:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-17"></a>

### c-known-evp_digestverifyinit-negative-comment.c:1:64

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-reassigned.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-18"></a>

### c-known-evp_digestverifyinit-reassigned.c:1:61

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-19"></a>

### c-known-evp_digestverifyinit.c:1:61

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-negative-comment.cpp:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-20"></a>

### cpp-known-evp_digestsigninit-negative-comment.cpp:1:64

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-reassigned.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-21"></a>

### cpp-known-evp_digestsigninit-reassigned.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-22"></a>

### cpp-known-evp_digestsigninit.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-negative-comment.cpp:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-23"></a>

### cpp-known-evp_digestverifyinit-negative-comment.cpp:1:64

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-24"></a>

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-25"></a>

### cpp-known-evp_digestverifyinit.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-id-rsa-negative-comment.cpp:1:62 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-26"></a>

### cpp-shared-c-evp-id-rsa-negative-comment.cpp:1:62

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-id-rsa.cpp:1:59 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-27"></a>

### cpp-shared-c-evp-id-rsa.cpp:1:59

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## API_OPENSSL_RSA_NEW

OpenSSL low-level RSA API usage detected.

Check: Prefer high-level OpenSSL EVP APIs to ease PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-28"></a>[c-openssl-rsa_generate_key_ex-negative-comment.c:1:4](#finding-28)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-28)|
|<a id="finding-29"></a>[c-openssl-rsa_generate_key_ex-positive.c:1:21](#finding-29)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-29)|
|<a id="finding-30"></a>[c-openssl-rsa_new-negative-comment.c:1:4](#finding-30)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-30)|
|<a id="finding-31"></a>[c-openssl-rsa_new-positive.c:1:21](#finding-31)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-31)|
|<a id="finding-32"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4](#finding-32)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-32)|
|<a id="finding-33"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21](#finding-33)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-33)|
|<a id="finding-34"></a>[cpp-shared-c-openssl-rsa_new-negative-comment.cpp:1:4](#finding-34)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-34)|
|<a id="finding-35"></a>[cpp-shared-c-openssl-rsa_new-positive.cpp:1:21](#finding-35)|\[masked-sensitive-content\]|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-35)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-0)
- [Adopt a phased hybrid PQC migration plan](#action-1)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-28"></a>

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_generate_key_ex-positive.c:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-29"></a>

### c-openssl-rsa_generate_key_ex-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_new-negative-comment.c:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-30"></a>

### c-openssl-rsa_new-negative-comment.c:1:4

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_new-positive.c:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-31"></a>

### c-openssl-rsa_new-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-32"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-33"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_new-negative-comment.cpp:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-34"></a>

### cpp-shared-c-openssl-rsa_new-negative-comment.cpp:1:4

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_new-positive.cpp:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-35"></a>

### cpp-shared-c-openssl-rsa_new-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

<a id="rule-3"></a>

## CSHARP_CRYPTO_API

C# cryptographic API or qualified library reference; effective key use requires verification.

Check: Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-36"></a>[cs-bc-classical-dsasigner-generatesignature.cs:1:37](#finding-36)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-36)|
|<a id="finding-37"></a>[cs-bc-classical-dsasigner-generatesignature.cs:1:86](#finding-37)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-37)|
|<a id="finding-38"></a>[cs-bc-classical-dsasigner-verifysignature.cs:1:37](#finding-38)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-38)|
|<a id="finding-39"></a>[cs-bc-classical-dsasigner-verifysignature.cs:1:86](#finding-39)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-39)|
|<a id="finding-40"></a>[cs-bc-classical-ecdhbasicagreement-calculateagreement.cs:1:37](#finding-40)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-40)|
|<a id="finding-41"></a>[cs-bc-classical-ecdhbasicagreement-calculateagreement.cs:1:97](#finding-41)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-41)|
|<a id="finding-42"></a>[cs-bc-classical-ecdsasigner-generatesignature.cs:1:37](#finding-42)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-42)|
|<a id="finding-43"></a>[cs-bc-classical-ecdsasigner-generatesignature.cs:1:88](#finding-43)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-43)|
|<a id="finding-44"></a>[cs-bc-classical-ecdsasigner-verifysignature.cs:1:37](#finding-44)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-44)|
|<a id="finding-45"></a>[cs-bc-classical-ecdsasigner-verifysignature.cs:1:88](#finding-45)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-45)|
|<a id="finding-46"></a>[cs-bc-classical-rsaengine-processblock.cs:1:37](#finding-46)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-46)|
|<a id="finding-47"></a>[cs-bc-classical-rsaengine-processblock.cs:1:86](#finding-47)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-47)|
|<a id="finding-48"></a>[cs-bc-dsa-positive.cs:1:29](#finding-48)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-48)|
|<a id="finding-49"></a>[cs-bc-ec-positive.cs:1:29](#finding-49)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-49)|
|<a id="finding-50"></a>[cs-bc-rsa-positive.cs:1:29](#finding-50)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-50)|
|<a id="finding-51"></a>[cs-provider-dsacng-positive.cs:1:29](#finding-51)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-51)|
|<a id="finding-52"></a>[cs-provider-dsacryptoserviceprovider-positive.cs:1:29](#finding-52)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-52)|
|<a id="finding-53"></a>[cs-provider-dsaopenssl-positive.cs:1:29](#finding-53)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-53)|
|<a id="finding-54"></a>[cs-provider-ecdiffiehellmancng-positive.cs:1:29](#finding-54)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-54)|
|<a id="finding-55"></a>[cs-provider-ecdiffiehellmanopenssl-positive.cs:1:29](#finding-55)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-55)|
|<a id="finding-56"></a>[cs-provider-ecdsacng-positive.cs:1:29](#finding-56)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-56)|
|<a id="finding-57"></a>[cs-provider-ecdsaopenssl-positive.cs:1:29](#finding-57)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-57)|
|<a id="finding-58"></a>[cs-provider-rsacng-positive.cs:1:29](#finding-58)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-58)|
|<a id="finding-59"></a>[cs-provider-rsacryptoserviceprovider-positive.cs:1:29](#finding-59)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-59)|
|<a id="finding-60"></a>[cs-provider-rsaopenssl-positive.cs:1:29](#finding-60)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-60)|
|<a id="finding-61"></a>[cs-rsa-decrypt.cs:1:37](#finding-61)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-61)|
|<a id="finding-62"></a>[cs-rsa-decrypt.cs:1:80](#finding-62)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-62)|
|<a id="finding-63"></a>[cs-rsa-encrypt.cs:1:37](#finding-63)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-63)|
|<a id="finding-64"></a>[cs-rsa-encrypt.cs:1:80](#finding-64)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-64)|
|<a id="finding-65"></a>[cs-standard-dsa-positive.cs:1:37](#finding-65)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-65)|
|<a id="finding-66"></a>[cs-standard-dsa-signdata-positive.cs:1:37](#finding-66)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-66)|
|<a id="finding-67"></a>[cs-standard-dsa-signdata-positive.cs:1:80](#finding-67)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-67)|
|<a id="finding-68"></a>[cs-standard-dsa-verifydata-positive.cs:1:37](#finding-68)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-68)|
|<a id="finding-69"></a>[cs-standard-dsa-verifydata-positive.cs:1:80](#finding-69)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-69)|
|<a id="finding-70"></a>[cs-standard-ecdiffiehellman-derivekeymaterial-positive.cs:1:37](#finding-70)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-70)|
|<a id="finding-71"></a>[cs-standard-ecdiffiehellman-derivekeymaterial-positive.cs:1:92](#finding-71)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-71)|
|<a id="finding-72"></a>[cs-standard-ecdiffiehellman-positive.cs:1:37](#finding-72)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-72)|
|<a id="finding-73"></a>[cs-standard-ecdsa-positive.cs:1:37](#finding-73)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-73)|
|<a id="finding-74"></a>[cs-standard-ecdsa-signdata-positive.cs:1:37](#finding-74)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-74)|
|<a id="finding-75"></a>[cs-standard-ecdsa-signdata-positive.cs:1:82](#finding-75)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-75)|
|<a id="finding-76"></a>[cs-standard-ecdsa-verifydata-positive.cs:1:37](#finding-76)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-76)|
|<a id="finding-77"></a>[cs-standard-ecdsa-verifydata-positive.cs:1:82](#finding-77)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-77)|
|<a id="finding-78"></a>[cs-standard-rsa-positive.cs:1:37](#finding-78)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-78)|
|<a id="finding-79"></a>[cs-standard-rsa-signdata-positive.cs:1:37](#finding-79)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-79)|
|<a id="finding-80"></a>[cs-standard-rsa-signdata-positive.cs:1:80](#finding-80)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-80)|
|<a id="finding-81"></a>[cs-standard-rsa-verifydata-positive.cs:1:37](#finding-81)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-81)|
|<a id="finding-82"></a>[cs-standard-rsa-verifydata-positive.cs:1:80](#finding-82)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-82)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-4)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### cs-bc-classical-dsasigner-generatesignature.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-36"></a>

### cs-bc-classical-dsasigner-generatesignature.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-dsasigner-generatesignature.cs:1:86 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-37"></a>

### cs-bc-classical-dsasigner-generatesignature.cs:1:86

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-dsasigner-verifysignature.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-38"></a>

### cs-bc-classical-dsasigner-verifysignature.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-dsasigner-verifysignature.cs:1:86 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-39"></a>

### cs-bc-classical-dsasigner-verifysignature.cs:1:86

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-ecdhbasicagreement-calculateagreement.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-40"></a>

### cs-bc-classical-ecdhbasicagreement-calculateagreement.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-ecdhbasicagreement-calculateagreement.cs:1:97 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-41"></a>

### cs-bc-classical-ecdhbasicagreement-calculateagreement.cs:1:97

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-ecdsasigner-generatesignature.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-42"></a>

### cs-bc-classical-ecdsasigner-generatesignature.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-ecdsasigner-generatesignature.cs:1:88 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-43"></a>

### cs-bc-classical-ecdsasigner-generatesignature.cs:1:88

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-ecdsasigner-verifysignature.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-44"></a>

### cs-bc-classical-ecdsasigner-verifysignature.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-ecdsasigner-verifysignature.cs:1:88 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-45"></a>

### cs-bc-classical-ecdsasigner-verifysignature.cs:1:88

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-rsaengine-processblock.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-46"></a>

### cs-bc-classical-rsaengine-processblock.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-classical-rsaengine-processblock.cs:1:86 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-47"></a>

### cs-bc-classical-rsaengine-processblock.cs:1:86

    > 1 [masked-sensitive-file-content]

### cs-bc-dsa-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-48"></a>

### cs-bc-dsa-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-bc-ec-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-49"></a>

### cs-bc-ec-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-bc-rsa-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-50"></a>

### cs-bc-rsa-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-dsacng-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-51"></a>

### cs-provider-dsacng-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-dsacryptoserviceprovider-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-52"></a>

### cs-provider-dsacryptoserviceprovider-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-dsaopenssl-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-53"></a>

### cs-provider-dsaopenssl-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-ecdiffiehellmancng-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-54"></a>

### cs-provider-ecdiffiehellmancng-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-ecdiffiehellmanopenssl-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-55"></a>

### cs-provider-ecdiffiehellmanopenssl-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-ecdsacng-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-56"></a>

### cs-provider-ecdsacng-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-ecdsaopenssl-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-57"></a>

### cs-provider-ecdsaopenssl-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-rsacng-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-58"></a>

### cs-provider-rsacng-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-rsacryptoserviceprovider-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-59"></a>

### cs-provider-rsacryptoserviceprovider-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-provider-rsaopenssl-positive.cs:1:29 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-60"></a>

### cs-provider-rsaopenssl-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-rsa-decrypt.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-61"></a>

### cs-rsa-decrypt.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-rsa-decrypt.cs:1:80 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-62"></a>

### cs-rsa-decrypt.cs:1:80

    > 1 [masked-sensitive-file-content]

### cs-rsa-encrypt.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-63"></a>

### cs-rsa-encrypt.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-rsa-encrypt.cs:1:80 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-64"></a>

### cs-rsa-encrypt.cs:1:80

    > 1 [masked-sensitive-file-content]

### cs-standard-dsa-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-65"></a>

### cs-standard-dsa-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-dsa-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-66"></a>

### cs-standard-dsa-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-dsa-signdata-positive.cs:1:80 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-67"></a>

### cs-standard-dsa-signdata-positive.cs:1:80

    > 1 [masked-sensitive-file-content]

### cs-standard-dsa-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-68"></a>

### cs-standard-dsa-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-dsa-verifydata-positive.cs:1:80 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-69"></a>

### cs-standard-dsa-verifydata-positive.cs:1:80

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdiffiehellman-derivekeymaterial-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-70"></a>

### cs-standard-ecdiffiehellman-derivekeymaterial-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdiffiehellman-derivekeymaterial-positive.cs:1:92 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-71"></a>

### cs-standard-ecdiffiehellman-derivekeymaterial-positive.cs:1:92

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdiffiehellman-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-72"></a>

### cs-standard-ecdiffiehellman-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdsa-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-73"></a>

### cs-standard-ecdsa-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdsa-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-74"></a>

### cs-standard-ecdsa-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdsa-signdata-positive.cs:1:82 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-75"></a>

### cs-standard-ecdsa-signdata-positive.cs:1:82

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdsa-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-76"></a>

### cs-standard-ecdsa-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-ecdsa-verifydata-positive.cs:1:82 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-77"></a>

### cs-standard-ecdsa-verifydata-positive.cs:1:82

    > 1 [masked-sensitive-file-content]

### cs-standard-rsa-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-78"></a>

### cs-standard-rsa-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-rsa-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-79"></a>

### cs-standard-rsa-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-rsa-signdata-positive.cs:1:80 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-80"></a>

### cs-standard-rsa-signdata-positive.cs:1:80

    > 1 [masked-sensitive-file-content]

### cs-standard-rsa-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-81"></a>

### cs-standard-rsa-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-standard-rsa-verifydata-positive.cs:1:80 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-82"></a>

### cs-standard-rsa-verifydata-positive.cs:1:80

    > 1 [masked-sensitive-file-content]

<a id="rule-4"></a>

## C_FAMILY_CRYPTO_API

C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification.

Check: Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-83"></a>[c-draft-dilithium3-positive.c:1:21](#finding-83)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-83)|
|<a id="finding-84"></a>[c-draft-kyber768-positive.c:1:21](#finding-84)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-84)|
|<a id="finding-85"></a>[c-dynamic-evp-positive.c:1:39](#finding-85)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-85)|
|<a id="finding-86"></a>[c-evp-dh-evp_pkey_keygen-positive.c:1:39](#finding-86)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-86)|
|<a id="finding-87"></a>[c-evp-dh-evp_pkey_keygen-positive.c:1:83](#finding-87)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-87)|
|<a id="finding-88"></a>[c-evp-dh-positive.c:1:39](#finding-88)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-88)|
|<a id="finding-89"></a>[c-evp-dsa-evp_pkey_keygen-positive.c:1:39](#finding-89)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-89)|
|<a id="finding-90"></a>[c-evp-dsa-evp_pkey_keygen-positive.c:1:84](#finding-90)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-90)|
|<a id="finding-91"></a>[c-evp-dsa-positive.c:1:39](#finding-91)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-91)|
|<a id="finding-92"></a>[c-evp-ec-evp_pkey_keygen-positive.c:1:39](#finding-92)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-92)|
|<a id="finding-93"></a>[c-evp-ec-evp_pkey_keygen-positive.c:1:83](#finding-93)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-93)|
|<a id="finding-94"></a>[c-evp-ec-positive.c:1:39](#finding-94)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-94)|
|<a id="finding-95"></a>[c-evp-generic-positive.c:1:21](#finding-95)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-95)|
|<a id="finding-96"></a>[c-evp-id-dh.c:1:39](#finding-96)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-96)|
|<a id="finding-97"></a>[c-evp-id-dsa.c:1:39](#finding-97)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-97)|
|<a id="finding-98"></a>[c-evp-id-ec.c:1:39](#finding-98)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-98)|
|<a id="finding-99"></a>[c-evp-id-rsa.c:1:39](#finding-99)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-99)|
|<a id="finding-100"></a>[c-evp-rsa-evp_pkey_keygen-positive.c:1:39](#finding-100)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-100)|
|<a id="finding-101"></a>[c-evp-rsa-evp_pkey_keygen-positive.c:1:84](#finding-101)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-101)|
|<a id="finding-102"></a>[c-evp-rsa-positive.c:1:39](#finding-102)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-102)|
|<a id="finding-103"></a>[c-evp_set_default_properties-positive.c:1:21](#finding-103)|C/C++ property configuration reference: provider=oqsprovider|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-103)|
|<a id="finding-104"></a>[c-known-evp_digestsigninit-reassigned.c:1:41](#finding-104)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-104)|
|<a id="finding-105"></a>[c-known-evp_digestsigninit-reassigned.c:1:81](#finding-105)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-105)|
|<a id="finding-106"></a>[c-known-evp_digestsigninit-reassigned.c:1:130](#finding-106)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-106)|
|<a id="finding-107"></a>[c-known-evp_digestsigninit-reassigned.c:1:168](#finding-107)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-107)|
|<a id="finding-108"></a>[c-known-evp_digestsigninit-unknown.c:1:21](#finding-108)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-108)|
|<a id="finding-109"></a>[c-known-evp_digestsigninit.c:1:41](#finding-109)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-109)|
|<a id="finding-110"></a>[c-known-evp_digestsigninit.c:1:81](#finding-110)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-110)|
|<a id="finding-111"></a>[c-known-evp_digestsigninit.c:1:130](#finding-111)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-111)|
|<a id="finding-112"></a>[c-known-evp_digestsigninit.c:1:157](#finding-112)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-112)|
|<a id="finding-113"></a>[c-known-evp_digestverifyinit-reassigned.c:1:41](#finding-113)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-113)|
|<a id="finding-114"></a>[c-known-evp_digestverifyinit-reassigned.c:1:81](#finding-114)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-114)|
|<a id="finding-115"></a>[c-known-evp_digestverifyinit-reassigned.c:1:130](#finding-115)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-115)|
|<a id="finding-116"></a>[c-known-evp_digestverifyinit-reassigned.c:1:168](#finding-116)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-116)|
|<a id="finding-117"></a>[c-known-evp_digestverifyinit-unknown.c:1:21](#finding-117)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-117)|
|<a id="finding-118"></a>[c-known-evp_digestverifyinit.c:1:41](#finding-118)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-118)|
|<a id="finding-119"></a>[c-known-evp_digestverifyinit.c:1:81](#finding-119)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-119)|
|<a id="finding-120"></a>[c-known-evp_digestverifyinit.c:1:130](#finding-120)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-120)|
|<a id="finding-121"></a>[c-known-evp_digestverifyinit.c:1:157](#finding-121)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-121)|
|<a id="finding-122"></a>[c-openssl-dh_compute_key-positive.c:1:21](#finding-122)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-122)|
|<a id="finding-123"></a>[c-openssl-dh_generate_key-positive.c:1:21](#finding-123)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-123)|
|<a id="finding-124"></a>[c-openssl-dh_new-positive.c:1:21](#finding-124)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-124)|
|<a id="finding-125"></a>[c-openssl-dsa_generate_key-positive.c:1:21](#finding-125)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-125)|
|<a id="finding-126"></a>[c-openssl-dsa_new-positive.c:1:21](#finding-126)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-126)|
|<a id="finding-127"></a>[c-openssl-dsa_sign-positive.c:1:21](#finding-127)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-127)|
|<a id="finding-128"></a>[c-openssl-dsa_verify-positive.c:1:21](#finding-128)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-128)|
|<a id="finding-129"></a>[c-openssl-ec_key_generate_key-positive.c:1:21](#finding-129)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-129)|
|<a id="finding-130"></a>[c-openssl-ec_key_new_by_curve_name-positive.c:1:21](#finding-130)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-130)|
|<a id="finding-131"></a>[c-openssl-ecdh_compute_key-positive.c:1:21](#finding-131)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-131)|
|<a id="finding-132"></a>[c-openssl-ecdsa_sign-positive.c:1:21](#finding-132)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-132)|
|<a id="finding-133"></a>[c-openssl-ecdsa_verify-positive.c:1:21](#finding-133)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-133)|
|<a id="finding-134"></a>[c-openssl-rsa_generate_key_ex-positive.c:1:21](#finding-134)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-134)|
|<a id="finding-135"></a>[c-openssl-rsa_new-positive.c:1:21](#finding-135)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-135)|
|<a id="finding-136"></a>[c-openssl-rsa_private_decrypt-positive.c:1:21](#finding-136)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-136)|
|<a id="finding-137"></a>[c-openssl-rsa_public_encrypt-positive.c:1:21](#finding-137)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-137)|
|<a id="finding-138"></a>[c-openssl-rsa_sign-positive.c:1:21](#finding-138)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-138)|
|<a id="finding-139"></a>[c-openssl-rsa_verify-positive.c:1:21](#finding-139)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-139)|
|<a id="finding-140"></a>[c-ossl_provider_load-positive.c:1:21](#finding-140)|C/C++ provider configuration reference: oqsprovider|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-140)|
|<a id="finding-141"></a>[c-ssl_ctx_set1_groups_list-positive.c:1:21](#finding-141)|C/C++ TLS group configuration reference: X25519MLKEM768:P-256|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-141)|
|<a id="finding-142"></a>[c-ssl_set1_groups_list-positive.c:1:21](#finding-142)|C/C++ TLS group configuration reference: X25519|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-142)|
|<a id="finding-143"></a>[cpp-known-evp_digestsigninit-reassigned.cpp:1:41](#finding-143)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-143)|
|<a id="finding-144"></a>[cpp-known-evp_digestsigninit-reassigned.cpp:1:81](#finding-144)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-144)|
|<a id="finding-145"></a>[cpp-known-evp_digestsigninit-reassigned.cpp:1:130](#finding-145)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-145)|
|<a id="finding-146"></a>[cpp-known-evp_digestsigninit-reassigned.cpp:1:168](#finding-146)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-146)|
|<a id="finding-147"></a>[cpp-known-evp_digestsigninit-unknown.cpp:1:21](#finding-147)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-147)|
|<a id="finding-148"></a>[cpp-known-evp_digestsigninit.cpp:1:41](#finding-148)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-148)|
|<a id="finding-149"></a>[cpp-known-evp_digestsigninit.cpp:1:81](#finding-149)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-149)|
|<a id="finding-150"></a>[cpp-known-evp_digestsigninit.cpp:1:130](#finding-150)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-150)|
|<a id="finding-151"></a>[cpp-known-evp_digestsigninit.cpp:1:157](#finding-151)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-151)|
|<a id="finding-152"></a>[cpp-known-evp_digestverifyinit-reassigned.cpp:1:41](#finding-152)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-152)|
|<a id="finding-153"></a>[cpp-known-evp_digestverifyinit-reassigned.cpp:1:81](#finding-153)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-153)|
|<a id="finding-154"></a>[cpp-known-evp_digestverifyinit-reassigned.cpp:1:130](#finding-154)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-154)|
|<a id="finding-155"></a>[cpp-known-evp_digestverifyinit-reassigned.cpp:1:168](#finding-155)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-155)|
|<a id="finding-156"></a>[cpp-known-evp_digestverifyinit-unknown.cpp:1:21](#finding-156)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-156)|
|<a id="finding-157"></a>[cpp-known-evp_digestverifyinit.cpp:1:41](#finding-157)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-157)|
|<a id="finding-158"></a>[cpp-known-evp_digestverifyinit.cpp:1:81](#finding-158)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-158)|
|<a id="finding-159"></a>[cpp-known-evp_digestverifyinit.cpp:1:130](#finding-159)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-159)|
|<a id="finding-160"></a>[cpp-known-evp_digestverifyinit.cpp:1:157](#finding-160)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-160)|
|<a id="finding-161"></a>[cpp-shared-c-draft-dilithium3-positive.cpp:1:21](#finding-161)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-161)|
|<a id="finding-162"></a>[cpp-shared-c-draft-kyber768-positive.cpp:1:21](#finding-162)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-162)|
|<a id="finding-163"></a>[cpp-shared-c-dynamic-evp-positive.cpp:1:39](#finding-163)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-163)|
|<a id="finding-164"></a>[cpp-shared-c-evp-dh-evp_pkey_keygen-positive.cpp:1:39](#finding-164)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-164)|
|<a id="finding-165"></a>[cpp-shared-c-evp-dh-evp_pkey_keygen-positive.cpp:1:83](#finding-165)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-165)|
|<a id="finding-166"></a>[cpp-shared-c-evp-dh-positive.cpp:1:39](#finding-166)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-166)|
|<a id="finding-167"></a>[cpp-shared-c-evp-dsa-evp_pkey_keygen-positive.cpp:1:39](#finding-167)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-167)|
|<a id="finding-168"></a>[cpp-shared-c-evp-dsa-evp_pkey_keygen-positive.cpp:1:84](#finding-168)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-168)|
|<a id="finding-169"></a>[cpp-shared-c-evp-dsa-positive.cpp:1:39](#finding-169)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-169)|
|<a id="finding-170"></a>[cpp-shared-c-evp-ec-evp_pkey_keygen-positive.cpp:1:39](#finding-170)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-170)|
|<a id="finding-171"></a>[cpp-shared-c-evp-ec-evp_pkey_keygen-positive.cpp:1:83](#finding-171)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-171)|
|<a id="finding-172"></a>[cpp-shared-c-evp-ec-positive.cpp:1:39](#finding-172)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-172)|
|<a id="finding-173"></a>[cpp-shared-c-evp-generic-positive.cpp:1:21](#finding-173)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-173)|
|<a id="finding-174"></a>[cpp-shared-c-evp-id-dh.cpp:1:39](#finding-174)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-174)|
|<a id="finding-175"></a>[cpp-shared-c-evp-id-dsa.cpp:1:39](#finding-175)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-175)|
|<a id="finding-176"></a>[cpp-shared-c-evp-id-ec.cpp:1:39](#finding-176)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-176)|
|<a id="finding-177"></a>[cpp-shared-c-evp-id-rsa.cpp:1:39](#finding-177)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-177)|
|<a id="finding-178"></a>[cpp-shared-c-evp-rsa-evp_pkey_keygen-positive.cpp:1:39](#finding-178)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-178)|
|<a id="finding-179"></a>[cpp-shared-c-evp-rsa-evp_pkey_keygen-positive.cpp:1:84](#finding-179)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-179)|
|<a id="finding-180"></a>[cpp-shared-c-evp-rsa-positive.cpp:1:39](#finding-180)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-180)|
|<a id="finding-181"></a>[cpp-shared-c-evp_set_default_properties-positive.cpp:1:21](#finding-181)|C/C++ property configuration reference: provider=oqsprovider|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-181)|
|<a id="finding-182"></a>[cpp-shared-c-openssl-dh_compute_key-positive.cpp:1:21](#finding-182)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-182)|
|<a id="finding-183"></a>[cpp-shared-c-openssl-dh_generate_key-positive.cpp:1:21](#finding-183)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-183)|
|<a id="finding-184"></a>[cpp-shared-c-openssl-dh_new-positive.cpp:1:21](#finding-184)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-184)|
|<a id="finding-185"></a>[cpp-shared-c-openssl-dsa_generate_key-positive.cpp:1:21](#finding-185)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-185)|
|<a id="finding-186"></a>[cpp-shared-c-openssl-dsa_new-positive.cpp:1:21](#finding-186)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-186)|
|<a id="finding-187"></a>[cpp-shared-c-openssl-dsa_sign-positive.cpp:1:21](#finding-187)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-187)|
|<a id="finding-188"></a>[cpp-shared-c-openssl-dsa_verify-positive.cpp:1:21](#finding-188)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-188)|
|<a id="finding-189"></a>[cpp-shared-c-openssl-ec_key_generate_key-positive.cpp:1:21](#finding-189)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-189)|
|<a id="finding-190"></a>[cpp-shared-c-openssl-ec_key_new_by_curve_name-positive.cpp:1:21](#finding-190)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-190)|
|<a id="finding-191"></a>[cpp-shared-c-openssl-ecdh_compute_key-positive.cpp:1:21](#finding-191)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-191)|
|<a id="finding-192"></a>[cpp-shared-c-openssl-ecdsa_sign-positive.cpp:1:21](#finding-192)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-192)|
|<a id="finding-193"></a>[cpp-shared-c-openssl-ecdsa_verify-positive.cpp:1:21](#finding-193)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-193)|
|<a id="finding-194"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21](#finding-194)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-194)|
|<a id="finding-195"></a>[cpp-shared-c-openssl-rsa_new-positive.cpp:1:21](#finding-195)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-195)|
|<a id="finding-196"></a>[cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21](#finding-196)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-196)|
|<a id="finding-197"></a>[cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21](#finding-197)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-197)|
|<a id="finding-198"></a>[cpp-shared-c-openssl-rsa_sign-positive.cpp:1:21](#finding-198)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-198)|
|<a id="finding-199"></a>[cpp-shared-c-openssl-rsa_verify-positive.cpp:1:21](#finding-199)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-199)|
|<a id="finding-200"></a>[cpp-shared-c-ossl_provider_load-positive.cpp:1:21](#finding-200)|C/C++ provider configuration reference: oqsprovider|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-200)|
|<a id="finding-201"></a>[cpp-shared-c-ssl_ctx_set1_groups_list-positive.cpp:1:21](#finding-201)|C/C++ TLS group configuration reference: X25519MLKEM768:P-256|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-201)|
|<a id="finding-202"></a>[cpp-shared-c-ssl_set1_groups_list-positive.cpp:1:21](#finding-202)|C/C++ TLS group configuration reference: X25519|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-202)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### c-draft-dilithium3-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-83"></a>

### c-draft-dilithium3-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-draft-kyber768-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-84"></a>

### c-draft-kyber768-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-dynamic-evp-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-85"></a>

### c-dynamic-evp-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-dh-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-86"></a>

### c-evp-dh-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-dh-evp_pkey_keygen-positive.c:1:83 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-87"></a>

### c-evp-dh-evp_pkey_keygen-positive.c:1:83

    > 1 [masked-sensitive-file-content]

### c-evp-dh-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-88"></a>

### c-evp-dh-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-dsa-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-89"></a>

### c-evp-dsa-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-dsa-evp_pkey_keygen-positive.c:1:84 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-90"></a>

### c-evp-dsa-evp_pkey_keygen-positive.c:1:84

    > 1 [masked-sensitive-file-content]

### c-evp-dsa-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-91"></a>

### c-evp-dsa-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ec-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-92"></a>

### c-evp-ec-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ec-evp_pkey_keygen-positive.c:1:83 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-93"></a>

### c-evp-ec-evp_pkey_keygen-positive.c:1:83

    > 1 [masked-sensitive-file-content]

### c-evp-ec-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-94"></a>

### c-evp-ec-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-generic-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-95"></a>

### c-evp-generic-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-evp-id-dh.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-96"></a>

### c-evp-id-dh.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-id-dsa.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-97"></a>

### c-evp-id-dsa.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-id-ec.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-98"></a>

### c-evp-id-ec.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-id-rsa.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-99"></a>

### c-evp-id-rsa.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-rsa-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-100"></a>

### c-evp-rsa-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-rsa-evp_pkey_keygen-positive.c:1:84 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-101"></a>

### c-evp-rsa-evp_pkey_keygen-positive.c:1:84

    > 1 [masked-sensitive-file-content]

### c-evp-rsa-positive.c:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-102"></a>

### c-evp-rsa-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp_set_default_properties-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-103"></a>

### c-evp_set_default_properties-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-reassigned.c:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-104"></a>

### c-known-evp_digestsigninit-reassigned.c:1:41

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-reassigned.c:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-105"></a>

### c-known-evp_digestsigninit-reassigned.c:1:81

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-reassigned.c:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-106"></a>

### c-known-evp_digestsigninit-reassigned.c:1:130

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-reassigned.c:1:168 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-107"></a>

### c-known-evp_digestsigninit-reassigned.c:1:168

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit-unknown.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-108"></a>

### c-known-evp_digestsigninit-unknown.c:1:21

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit.c:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-109"></a>

### c-known-evp_digestsigninit.c:1:41

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit.c:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-110"></a>

### c-known-evp_digestsigninit.c:1:81

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit.c:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-111"></a>

### c-known-evp_digestsigninit.c:1:130

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestsigninit.c:1:157 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-112"></a>

### c-known-evp_digestsigninit.c:1:157

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-reassigned.c:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-113"></a>

### c-known-evp_digestverifyinit-reassigned.c:1:41

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-reassigned.c:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-114"></a>

### c-known-evp_digestverifyinit-reassigned.c:1:81

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-reassigned.c:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-115"></a>

### c-known-evp_digestverifyinit-reassigned.c:1:130

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-reassigned.c:1:168 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-116"></a>

### c-known-evp_digestverifyinit-reassigned.c:1:168

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit-unknown.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-117"></a>

### c-known-evp_digestverifyinit-unknown.c:1:21

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit.c:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-118"></a>

### c-known-evp_digestverifyinit.c:1:41

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit.c:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-119"></a>

### c-known-evp_digestverifyinit.c:1:81

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit.c:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-120"></a>

### c-known-evp_digestverifyinit.c:1:130

    > 1 [masked-sensitive-file-content]

### c-known-evp_digestverifyinit.c:1:157 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-121"></a>

### c-known-evp_digestverifyinit.c:1:157

    > 1 [masked-sensitive-file-content]

### c-openssl-dh_compute_key-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-122"></a>

### c-openssl-dh_compute_key-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-dh_generate_key-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-123"></a>

### c-openssl-dh_generate_key-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-dh_new-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-124"></a>

### c-openssl-dh_new-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-dsa_generate_key-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-125"></a>

### c-openssl-dsa_generate_key-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-dsa_new-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-126"></a>

### c-openssl-dsa_new-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-dsa_sign-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-127"></a>

### c-openssl-dsa_sign-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-dsa_verify-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-128"></a>

### c-openssl-dsa_verify-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-ec_key_generate_key-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-129"></a>

### c-openssl-ec_key_generate_key-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-ec_key_new_by_curve_name-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-130"></a>

### c-openssl-ec_key_new_by_curve_name-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-ecdh_compute_key-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-131"></a>

### c-openssl-ecdh_compute_key-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-ecdsa_sign-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-132"></a>

### c-openssl-ecdsa_sign-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-ecdsa_verify-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-133"></a>

### c-openssl-ecdsa_verify-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_generate_key_ex-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-134"></a>

### c-openssl-rsa_generate_key_ex-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_new-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-135"></a>

### c-openssl-rsa_new-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_private_decrypt-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-136"></a>

### c-openssl-rsa_private_decrypt-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_public_encrypt-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-137"></a>

### c-openssl-rsa_public_encrypt-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_sign-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-138"></a>

### c-openssl-rsa_sign-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-openssl-rsa_verify-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-139"></a>

### c-openssl-rsa_verify-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-ossl_provider_load-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-140"></a>

### c-ossl_provider_load-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-ssl_ctx_set1_groups_list-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-141"></a>

### c-ssl_ctx_set1_groups_list-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-ssl_set1_groups_list-positive.c:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-142"></a>

### c-ssl_set1_groups_list-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-reassigned.cpp:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-143"></a>

### cpp-known-evp_digestsigninit-reassigned.cpp:1:41

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-reassigned.cpp:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-144"></a>

### cpp-known-evp_digestsigninit-reassigned.cpp:1:81

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-reassigned.cpp:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-145"></a>

### cpp-known-evp_digestsigninit-reassigned.cpp:1:130

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-reassigned.cpp:1:168 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-146"></a>

### cpp-known-evp_digestsigninit-reassigned.cpp:1:168

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit-unknown.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-147"></a>

### cpp-known-evp_digestsigninit-unknown.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit.cpp:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-148"></a>

### cpp-known-evp_digestsigninit.cpp:1:41

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit.cpp:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-149"></a>

### cpp-known-evp_digestsigninit.cpp:1:81

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit.cpp:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-150"></a>

### cpp-known-evp_digestsigninit.cpp:1:130

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestsigninit.cpp:1:157 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-151"></a>

### cpp-known-evp_digestsigninit.cpp:1:157

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-152"></a>

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:41

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-153"></a>

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:81

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-154"></a>

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:130

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:168 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-155"></a>

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:168

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit-unknown.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-156"></a>

### cpp-known-evp_digestverifyinit-unknown.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit.cpp:1:41 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-157"></a>

### cpp-known-evp_digestverifyinit.cpp:1:41

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit.cpp:1:81 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-158"></a>

### cpp-known-evp_digestverifyinit.cpp:1:81

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit.cpp:1:130 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-159"></a>

### cpp-known-evp_digestverifyinit.cpp:1:130

    > 1 [masked-sensitive-file-content]

### cpp-known-evp_digestverifyinit.cpp:1:157 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-160"></a>

### cpp-known-evp_digestverifyinit.cpp:1:157

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-draft-dilithium3-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-161"></a>

### cpp-shared-c-draft-dilithium3-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-draft-kyber768-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-162"></a>

### cpp-shared-c-draft-kyber768-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-dynamic-evp-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-163"></a>

### cpp-shared-c-dynamic-evp-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-dh-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-164"></a>

### cpp-shared-c-evp-dh-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-dh-evp_pkey_keygen-positive.cpp:1:83 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-165"></a>

### cpp-shared-c-evp-dh-evp_pkey_keygen-positive.cpp:1:83

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-dh-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-166"></a>

### cpp-shared-c-evp-dh-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-dsa-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-167"></a>

### cpp-shared-c-evp-dsa-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-dsa-evp_pkey_keygen-positive.cpp:1:84 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-168"></a>

### cpp-shared-c-evp-dsa-evp_pkey_keygen-positive.cpp:1:84

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-dsa-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-169"></a>

### cpp-shared-c-evp-dsa-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ec-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-170"></a>

### cpp-shared-c-evp-ec-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ec-evp_pkey_keygen-positive.cpp:1:83 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-171"></a>

### cpp-shared-c-evp-ec-evp_pkey_keygen-positive.cpp:1:83

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ec-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-172"></a>

### cpp-shared-c-evp-ec-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-generic-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-173"></a>

### cpp-shared-c-evp-generic-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-id-dh.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-174"></a>

### cpp-shared-c-evp-id-dh.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-id-dsa.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-175"></a>

### cpp-shared-c-evp-id-dsa.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-id-ec.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-176"></a>

### cpp-shared-c-evp-id-ec.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-id-rsa.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-177"></a>

### cpp-shared-c-evp-id-rsa.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-rsa-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-178"></a>

### cpp-shared-c-evp-rsa-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-rsa-evp_pkey_keygen-positive.cpp:1:84 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-179"></a>

### cpp-shared-c-evp-rsa-evp_pkey_keygen-positive.cpp:1:84

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-rsa-positive.cpp:1:39 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-180"></a>

### cpp-shared-c-evp-rsa-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp_set_default_properties-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-181"></a>

### cpp-shared-c-evp_set_default_properties-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dh_compute_key-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-182"></a>

### cpp-shared-c-openssl-dh_compute_key-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dh_generate_key-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-183"></a>

### cpp-shared-c-openssl-dh_generate_key-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dh_new-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-184"></a>

### cpp-shared-c-openssl-dh_new-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dsa_generate_key-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-185"></a>

### cpp-shared-c-openssl-dsa_generate_key-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dsa_new-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-186"></a>

### cpp-shared-c-openssl-dsa_new-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dsa_sign-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-187"></a>

### cpp-shared-c-openssl-dsa_sign-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-dsa_verify-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-188"></a>

### cpp-shared-c-openssl-dsa_verify-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-ec_key_generate_key-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-189"></a>

### cpp-shared-c-openssl-ec_key_generate_key-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-ec_key_new_by_curve_name-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-190"></a>

### cpp-shared-c-openssl-ec_key_new_by_curve_name-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-ecdh_compute_key-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-191"></a>

### cpp-shared-c-openssl-ecdh_compute_key-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-ecdsa_sign-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-192"></a>

### cpp-shared-c-openssl-ecdsa_sign-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-ecdsa_verify-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-193"></a>

### cpp-shared-c-openssl-ecdsa_verify-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-194"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_new-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-195"></a>

### cpp-shared-c-openssl-rsa_new-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-196"></a>

### cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-197"></a>

### cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_sign-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-198"></a>

### cpp-shared-c-openssl-rsa_sign-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-openssl-rsa_verify-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-199"></a>

### cpp-shared-c-openssl-rsa_verify-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-ossl_provider_load-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-200"></a>

### cpp-shared-c-ossl_provider_load-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-ssl_ctx_set1_groups_list-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-201"></a>

### cpp-shared-c-ssl_ctx_set1_groups_list-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-ssl_set1_groups_list-positive.cpp:1:21 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-202"></a>

### cpp-shared-c-ssl_set1_groups_list-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

<a id="rule-5"></a>

## PHP_CRYPTO_API

PHP cryptographic API or library reference; key type may be unresolved.

Check: Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-203"></a>[php-generic-sign-positive.php:2:1](#finding-203)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-203)|
|<a id="finding-204"></a>[php-known-derive-dh-reassigned.php:2:12](#finding-204)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-204)|
|<a id="finding-205"></a>[php-known-derive-dh-reassigned.php:2:80](#finding-205)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-205)|
|<a id="finding-206"></a>[php-known-derive-dh-reassigned.php:2:157](#finding-206)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-206)|
|<a id="finding-207"></a>[php-known-derive-dh.php:2:12](#finding-207)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-207)|
|<a id="finding-208"></a>[php-known-derive-dh.php:2:80](#finding-208)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-208)|
|<a id="finding-209"></a>[php-known-derive-dh.php:2:140](#finding-209)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-209)|
|<a id="finding-210"></a>[php-known-derive-ec-reassigned.php:2:12](#finding-210)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-210)|
|<a id="finding-211"></a>[php-known-derive-ec-reassigned.php:2:80](#finding-211)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-211)|
|<a id="finding-212"></a>[php-known-derive-ec-reassigned.php:2:157](#finding-212)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-212)|
|<a id="finding-213"></a>[php-known-derive-ec.php:2:12](#finding-213)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-213)|
|<a id="finding-214"></a>[php-known-derive-ec.php:2:80](#finding-214)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-214)|
|<a id="finding-215"></a>[php-known-derive-ec.php:2:140](#finding-215)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-215)|
|<a id="finding-216"></a>[php-openssl-dh-positive.php:2:6](#finding-216)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-216)|
|<a id="finding-217"></a>[php-openssl-ec-positive.php:2:6](#finding-217)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-217)|
|<a id="finding-218"></a>[php-openssl-pkey_derive-positive.php:2:1](#finding-218)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-218)|
|<a id="finding-219"></a>[php-openssl-private_decrypt-positive.php:2:1](#finding-219)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-219)|
|<a id="finding-220"></a>[php-openssl-private_encrypt-positive.php:2:1](#finding-220)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-220)|
|<a id="finding-221"></a>[php-openssl-public_decrypt-positive.php:2:1](#finding-221)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-221)|
|<a id="finding-222"></a>[php-openssl-public_encrypt-positive.php:2:1](#finding-222)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-222)|
|<a id="finding-223"></a>[php-openssl-rsa-positive.php:2:6](#finding-223)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-223)|
|<a id="finding-224"></a>[php-openssl-sign-positive.php:2:1](#finding-224)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-224)|
|<a id="finding-225"></a>[php-openssl-verify-positive.php:2:1](#finding-225)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-225)|
|<a id="finding-226"></a>[php-oqs-ffi-sig.php:2:5](#finding-226)|mir-evgenii/liboqs-php FFI wrapper reference|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-6) / [Source](#source-226)|
|<a id="finding-227"></a>[php-oqs-native-sig.php:2:5](#finding-227)|Muzosh oqsphp wrapper reference|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-7) / [Source](#source-227)|
|<a id="finding-228"></a>[php-sodium-box-positive.php:2:1](#finding-228)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-228)|
|<a id="finding-229"></a>[php-sodium-box_keypair-positive.php:2:1](#finding-229)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-229)|
|<a id="finding-230"></a>[php-sodium-box_open-positive.php:2:1](#finding-230)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-230)|
|<a id="finding-231"></a>[php-sodium-kx_client_session_keys-positive.php:2:1](#finding-231)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-231)|
|<a id="finding-232"></a>[php-sodium-kx_keypair-positive.php:2:1](#finding-232)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-232)|
|<a id="finding-233"></a>[php-sodium-kx_server_session_keys-positive.php:2:1](#finding-233)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-233)|
|<a id="finding-234"></a>[php-sodium-scalarmult-positive.php:2:1](#finding-234)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-234)|
|<a id="finding-235"></a>[php-sodium-sign_detached-positive.php:2:1](#finding-235)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-235)|
|<a id="finding-236"></a>[php-sodium-sign_keypair-positive.php:2:1](#finding-236)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-236)|
|<a id="finding-237"></a>[php-sodium-sign_verify_detached-positive.php:2:1](#finding-237)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-237)|
|<a id="finding-238"></a>[phpseclib-dh-positive.php:2:1](#finding-238)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-238)|
|<a id="finding-239"></a>[phpseclib-ec-positive.php:2:1](#finding-239)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-239)|
|<a id="finding-240"></a>[phpseclib-rsa-positive.php:2:1](#finding-240)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-240)|

Shared checks for these locations (not verified facts about each line):

- [Verify PHP extensions and their native cryptographic backends](#action-5)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### php-generic-sign-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-203"></a>

### php-generic-sign-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-known-derive-dh-reassigned.php:2:12 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-204"></a>

### php-known-derive-dh-reassigned.php:2:12

    > 2 [masked-sensitive-file-content]

### php-known-derive-dh-reassigned.php:2:80 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-205"></a>

### php-known-derive-dh-reassigned.php:2:80

    > 2 [masked-sensitive-file-content]

### php-known-derive-dh-reassigned.php:2:157 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-206"></a>

### php-known-derive-dh-reassigned.php:2:157

    > 2 [masked-sensitive-file-content]

### php-known-derive-dh.php:2:12 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-207"></a>

### php-known-derive-dh.php:2:12

    > 2 [masked-sensitive-file-content]

### php-known-derive-dh.php:2:80 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-208"></a>

### php-known-derive-dh.php:2:80

    > 2 [masked-sensitive-file-content]

### php-known-derive-dh.php:2:140 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-209"></a>

### php-known-derive-dh.php:2:140

    > 2 [masked-sensitive-file-content]

### php-known-derive-ec-reassigned.php:2:12 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-210"></a>

### php-known-derive-ec-reassigned.php:2:12

    > 2 [masked-sensitive-file-content]

### php-known-derive-ec-reassigned.php:2:80 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-211"></a>

### php-known-derive-ec-reassigned.php:2:80

    > 2 [masked-sensitive-file-content]

### php-known-derive-ec-reassigned.php:2:157 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-212"></a>

### php-known-derive-ec-reassigned.php:2:157

    > 2 [masked-sensitive-file-content]

### php-known-derive-ec.php:2:12 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-213"></a>

### php-known-derive-ec.php:2:12

    > 2 [masked-sensitive-file-content]

### php-known-derive-ec.php:2:80 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-214"></a>

### php-known-derive-ec.php:2:80

    > 2 [masked-sensitive-file-content]

### php-known-derive-ec.php:2:140 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-215"></a>

### php-known-derive-ec.php:2:140

    > 2 [masked-sensitive-file-content]

### php-openssl-dh-positive.php:2:6 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-216"></a>

### php-openssl-dh-positive.php:2:6

    > 2 [masked-sensitive-file-content]

### php-openssl-ec-positive.php:2:6 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-217"></a>

### php-openssl-ec-positive.php:2:6

    > 2 [masked-sensitive-file-content]

### php-openssl-pkey_derive-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-218"></a>

### php-openssl-pkey_derive-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-openssl-private_decrypt-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-219"></a>

### php-openssl-private_decrypt-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-openssl-private_encrypt-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-220"></a>

### php-openssl-private_encrypt-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-openssl-public_decrypt-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-221"></a>

### php-openssl-public_decrypt-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-openssl-public_encrypt-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-222"></a>

### php-openssl-public_encrypt-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-openssl-rsa-positive.php:2:6 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-223"></a>

### php-openssl-rsa-positive.php:2:6

    > 2 [masked-sensitive-file-content]

### php-openssl-sign-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-224"></a>

### php-openssl-sign-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-openssl-verify-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-225"></a>

### php-openssl-verify-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-oqs-ffi-sig.php:2:5 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-226"></a>

### php-oqs-ffi-sig.php:2:5

    > 2 [masked-sensitive-file-content]

### php-oqs-native-sig.php:2:5 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-227"></a>

### php-oqs-native-sig.php:2:5

    > 2 [masked-sensitive-file-content]

### php-sodium-box-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-228"></a>

### php-sodium-box-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-box_keypair-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-229"></a>

### php-sodium-box_keypair-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-box_open-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-230"></a>

### php-sodium-box_open-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kx_client_session_keys-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-231"></a>

### php-sodium-kx_client_session_keys-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kx_keypair-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-232"></a>

### php-sodium-kx_keypair-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kx_server_session_keys-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-233"></a>

### php-sodium-kx_server_session_keys-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-scalarmult-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-234"></a>

### php-sodium-scalarmult-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-sign_detached-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-235"></a>

### php-sodium-sign_detached-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-sign_keypair-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-236"></a>

### php-sodium-sign_keypair-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-sign_verify_detached-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-237"></a>

### php-sodium-sign_verify_detached-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### phpseclib-dh-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-238"></a>

### phpseclib-dh-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### phpseclib-ec-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-239"></a>

### phpseclib-ec-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### phpseclib-rsa-positive.php:2:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-240"></a>

### phpseclib-rsa-positive.php:2:1

    > 2 [masked-sensitive-file-content]

<a id="rule-6"></a>

## CSHARP_CRYPTO_REFERENCE

C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks.

Check: Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-241"></a>[cs-oqs-official-reference.cs:1:7](#finding-241)|C# obsolete official dotnetOQS wrapper reference|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-8) / [Source](#source-241)|
|<a id="finding-242"></a>[cs-oqs-openforge-reference.cs:1:7](#finding-242)|C# OpenForge community OQS wrapper reference|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-9) / [Source](#source-242)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-4)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### cs-oqs-official-reference.cs:1:7 — Matched rules

- `CSHARP_CRYPTO_REFERENCE` (info): C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks. — Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

<a id="source-241"></a>

### cs-oqs-official-reference.cs:1:7

    > 1 [masked-sensitive-file-content]

### cs-oqs-openforge-reference.cs:1:7 — Matched rules

- `CSHARP_CRYPTO_REFERENCE` (info): C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks. — Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

<a id="source-242"></a>

### cs-oqs-openforge-reference.cs:1:7

    > 1 [masked-sensitive-file-content]

<a id="rule-7"></a>

## CSHARP_PQC_API

C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection.

Check: Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-243"></a>[cs-alias-positive.cs:1:11](#finding-243)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-243)|
|<a id="finding-244"></a>[cs-alias-positive.cs:1:88](#finding-244)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-244)|
|<a id="finding-245"></a>[cs-bc-keypair-mldsa.cs:1:37](#finding-245)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-245)|
|<a id="finding-246"></a>[cs-bc-keypair-mldsa.cs:1:247](#finding-246)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-246)|
|<a id="finding-247"></a>[cs-bc-keypair-mlkem.cs:1:37](#finding-247)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-247)|
|<a id="finding-248"></a>[cs-bc-keypair-mlkem.cs:1:248](#finding-248)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-248)|
|<a id="finding-249"></a>[cs-bc-keypair-slhdsa.cs:1:37](#finding-249)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-249)|
|<a id="finding-250"></a>[cs-bc-keypair-slhdsa.cs:1:258](#finding-250)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-250)|
|<a id="finding-251"></a>[cs-bc-ml_dsa_44-positive.cs:1:37](#finding-251)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-251)|
|<a id="finding-252"></a>[cs-bc-ml_dsa_65-positive.cs:1:37](#finding-252)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-252)|
|<a id="finding-253"></a>[cs-bc-ml_dsa_87-positive.cs:1:37](#finding-253)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-253)|
|<a id="finding-254"></a>[cs-bc-ml_kem_1024-positive.cs:1:37](#finding-254)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-254)|
|<a id="finding-255"></a>[cs-bc-ml_kem_512-positive.cs:1:37](#finding-255)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-255)|
|<a id="finding-256"></a>[cs-bc-ml_kem_768-positive.cs:1:37](#finding-256)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-256)|
|<a id="finding-257"></a>[cs-bc-mldsakeypairgenerator-positive.cs:1:29](#finding-257)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-257)|
|<a id="finding-258"></a>[cs-bc-mlkemkeypairgenerator-positive.cs:1:29](#finding-258)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-258)|
|<a id="finding-259"></a>[cs-bc-operation-mldsasigner-generatesignature-positive.cs:1:37](#finding-259)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-259)|
|<a id="finding-260"></a>[cs-bc-operation-mldsasigner-generatesignature-positive.cs:1:154](#finding-260)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-260)|
|<a id="finding-261"></a>[cs-bc-operation-mldsasigner-verifysignature-positive.cs:1:37](#finding-261)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-261)|
|<a id="finding-262"></a>[cs-bc-operation-mldsasigner-verifysignature-positive.cs:1:154](#finding-262)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-262)|
|<a id="finding-263"></a>[cs-bc-operation-mlkemdecapsulator-decapsulate-positive.cs:1:37](#finding-263)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-263)|
|<a id="finding-264"></a>[cs-bc-operation-mlkemdecapsulator-decapsulate-positive.cs:1:152](#finding-264)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-264)|
|<a id="finding-265"></a>[cs-bc-operation-mlkemencapsulator-encapsulate-positive.cs:1:37](#finding-265)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-265)|
|<a id="finding-266"></a>[cs-bc-operation-mlkemencapsulator-encapsulate-positive.cs:1:152](#finding-266)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-266)|
|<a id="finding-267"></a>[cs-bc-operation-slhdsasigner-generatesignature-positive.cs:1:37](#finding-267)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-267)|
|<a id="finding-268"></a>[cs-bc-operation-slhdsasigner-generatesignature-positive.cs:1:164](#finding-268)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-268)|
|<a id="finding-269"></a>[cs-bc-operation-slhdsasigner-verifysignature-positive.cs:1:37](#finding-269)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-269)|
|<a id="finding-270"></a>[cs-bc-operation-slhdsasigner-verifysignature-positive.cs:1:164](#finding-270)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-270)|
|<a id="finding-271"></a>[cs-bc-slh_dsa_sha2_128f-positive.cs:1:37](#finding-271)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-271)|
|<a id="finding-272"></a>[cs-bc-slh_dsa_sha2_128s-positive.cs:1:37](#finding-272)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-272)|
|<a id="finding-273"></a>[cs-bc-slh_dsa_sha2_192f-positive.cs:1:37](#finding-273)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-273)|
|<a id="finding-274"></a>[cs-bc-slh_dsa_sha2_192s-positive.cs:1:37](#finding-274)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-274)|
|<a id="finding-275"></a>[cs-bc-slh_dsa_sha2_256f-positive.cs:1:37](#finding-275)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-275)|
|<a id="finding-276"></a>[cs-bc-slh_dsa_sha2_256s-positive.cs:1:37](#finding-276)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-276)|
|<a id="finding-277"></a>[cs-bc-slh_dsa_shake_128f-positive.cs:1:37](#finding-277)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-277)|
|<a id="finding-278"></a>[cs-bc-slh_dsa_shake_128s-positive.cs:1:37](#finding-278)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-278)|
|<a id="finding-279"></a>[cs-bc-slh_dsa_shake_192f-positive.cs:1:37](#finding-279)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-279)|
|<a id="finding-280"></a>[cs-bc-slh_dsa_shake_192s-positive.cs:1:37](#finding-280)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-280)|
|<a id="finding-281"></a>[cs-bc-slh_dsa_shake_256f-positive.cs:1:37](#finding-281)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-281)|
|<a id="finding-282"></a>[cs-bc-slh_dsa_shake_256s-positive.cs:1:37](#finding-282)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-282)|
|<a id="finding-283"></a>[cs-bc-slhdsakeypairgenerator-positive.cs:1:29](#finding-283)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-283)|
|<a id="finding-284"></a>[cs-construction-compositemldsa-positive.cs:1:29](#finding-284)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-284)|
|<a id="finding-285"></a>[cs-construction-compositemlkem-positive.cs:1:29](#finding-285)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-285)|
|<a id="finding-286"></a>[cs-construction-hpke-positive.cs:1:29](#finding-286)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-286)|
|<a id="finding-287"></a>[cs-mldsa-importfrompem-positive.cs:1:29](#finding-287)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-287)|
|<a id="finding-288"></a>[cs-mldsa-importpkcs8privatekey-positive.cs:1:29](#finding-288)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-288)|
|<a id="finding-289"></a>[cs-mldsa-importsubjectpublickeyinfo-positive.cs:1:29](#finding-289)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-289)|
|<a id="finding-290"></a>[cs-mldsa-mldsa44-positive.cs:1:37](#finding-290)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-290)|
|<a id="finding-291"></a>[cs-mldsa-mldsa44-signdata-positive.cs:1:37](#finding-291)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-291)|
|<a id="finding-292"></a>[cs-mldsa-mldsa44-signdata-positive.cs:1:138](#finding-292)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-292)|
|<a id="finding-293"></a>[cs-mldsa-mldsa44-verifydata-positive.cs:1:37](#finding-293)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-293)|
|<a id="finding-294"></a>[cs-mldsa-mldsa44-verifydata-positive.cs:1:138](#finding-294)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-294)|
|<a id="finding-295"></a>[cs-mldsa-mldsa65-positive.cs:1:37](#finding-295)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-295)|
|<a id="finding-296"></a>[cs-mldsa-mldsa65-signdata-positive.cs:1:37](#finding-296)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-296)|
|<a id="finding-297"></a>[cs-mldsa-mldsa65-signdata-positive.cs:1:138](#finding-297)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-297)|
|<a id="finding-298"></a>[cs-mldsa-mldsa65-verifydata-positive.cs:1:37](#finding-298)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-298)|
|<a id="finding-299"></a>[cs-mldsa-mldsa65-verifydata-positive.cs:1:138](#finding-299)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-299)|
|<a id="finding-300"></a>[cs-mldsa-mldsa87-positive.cs:1:37](#finding-300)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-300)|
|<a id="finding-301"></a>[cs-mldsa-mldsa87-signdata-positive.cs:1:37](#finding-301)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-301)|
|<a id="finding-302"></a>[cs-mldsa-mldsa87-signdata-positive.cs:1:138](#finding-302)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-302)|
|<a id="finding-303"></a>[cs-mldsa-mldsa87-verifydata-positive.cs:1:37](#finding-303)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-303)|
|<a id="finding-304"></a>[cs-mldsa-mldsa87-verifydata-positive.cs:1:138](#finding-304)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-304)|
|<a id="finding-305"></a>[cs-mlkem-importfrompem-positive.cs:1:29](#finding-305)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-305)|
|<a id="finding-306"></a>[cs-mlkem-importpkcs8privatekey-positive.cs:1:29](#finding-306)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-306)|
|<a id="finding-307"></a>[cs-mlkem-importsubjectpublickeyinfo-positive.cs:1:29](#finding-307)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-307)|
|<a id="finding-308"></a>[cs-mlkem-mlkem1024-decapsulate-positive.cs:1:37](#finding-308)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-308)|
|<a id="finding-309"></a>[cs-mlkem-mlkem1024-decapsulate-positive.cs:1:140](#finding-309)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-309)|
|<a id="finding-310"></a>[cs-mlkem-mlkem1024-encapsulate-positive.cs:1:37](#finding-310)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-310)|
|<a id="finding-311"></a>[cs-mlkem-mlkem1024-encapsulate-positive.cs:1:140](#finding-311)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-311)|
|<a id="finding-312"></a>[cs-mlkem-mlkem1024-positive.cs:1:37](#finding-312)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-312)|
|<a id="finding-313"></a>[cs-mlkem-mlkem512-decapsulate-positive.cs:1:37](#finding-313)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-313)|
|<a id="finding-314"></a>[cs-mlkem-mlkem512-decapsulate-positive.cs:1:139](#finding-314)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-314)|
|<a id="finding-315"></a>[cs-mlkem-mlkem512-encapsulate-positive.cs:1:37](#finding-315)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-315)|
|<a id="finding-316"></a>[cs-mlkem-mlkem512-encapsulate-positive.cs:1:139](#finding-316)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-316)|
|<a id="finding-317"></a>[cs-mlkem-mlkem512-positive.cs:1:37](#finding-317)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-317)|
|<a id="finding-318"></a>[cs-mlkem-mlkem768-decapsulate-positive.cs:1:37](#finding-318)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-318)|
|<a id="finding-319"></a>[cs-mlkem-mlkem768-decapsulate-positive.cs:1:139](#finding-319)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-319)|
|<a id="finding-320"></a>[cs-mlkem-mlkem768-encapsulate-positive.cs:1:37](#finding-320)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-320)|
|<a id="finding-321"></a>[cs-mlkem-mlkem768-encapsulate-positive.cs:1:139](#finding-321)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-321)|
|<a id="finding-322"></a>[cs-mlkem-mlkem768-positive.cs:1:37](#finding-322)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-322)|
|<a id="finding-323"></a>[cs-rebound-positive.cs:1:37](#finding-323)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-323)|
|<a id="finding-324"></a>[cs-selected-import-mldsa.cs:1:37](#finding-324)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-324)|
|<a id="finding-325"></a>[cs-selected-import-mlkem.cs:1:37](#finding-325)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-325)|
|<a id="finding-326"></a>[cs-selected-import-slhdsa.cs:1:37](#finding-326)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-326)|
|<a id="finding-327"></a>[cs-slhdsa-importfrompem-positive.cs:1:29](#finding-327)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-327)|
|<a id="finding-328"></a>[cs-slhdsa-importpkcs8privatekey-positive.cs:1:29](#finding-328)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-328)|
|<a id="finding-329"></a>[cs-slhdsa-importsubjectpublickeyinfo-positive.cs:1:29](#finding-329)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-329)|
|<a id="finding-330"></a>[cs-slhdsa-sha2_128f-positive.cs:1:37](#finding-330)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-330)|
|<a id="finding-331"></a>[cs-slhdsa-sha2_128f-signdata-positive.cs:1:37](#finding-331)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-331)|
|<a id="finding-332"></a>[cs-slhdsa-sha2_128f-signdata-positive.cs:1:148](#finding-332)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-332)|
|<a id="finding-333"></a>[cs-slhdsa-sha2_128f-verifydata-positive.cs:1:37](#finding-333)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-333)|
|<a id="finding-334"></a>[cs-slhdsa-sha2_128f-verifydata-positive.cs:1:148](#finding-334)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-334)|
|<a id="finding-335"></a>[cs-slhdsa-sha2_128s-positive.cs:1:37](#finding-335)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-335)|
|<a id="finding-336"></a>[cs-slhdsa-sha2_128s-signdata-positive.cs:1:37](#finding-336)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-336)|
|<a id="finding-337"></a>[cs-slhdsa-sha2_128s-signdata-positive.cs:1:148](#finding-337)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-337)|
|<a id="finding-338"></a>[cs-slhdsa-sha2_128s-verifydata-positive.cs:1:37](#finding-338)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-338)|
|<a id="finding-339"></a>[cs-slhdsa-sha2_128s-verifydata-positive.cs:1:148](#finding-339)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-339)|
|<a id="finding-340"></a>[cs-slhdsa-sha2_192f-positive.cs:1:37](#finding-340)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-340)|
|<a id="finding-341"></a>[cs-slhdsa-sha2_192f-signdata-positive.cs:1:37](#finding-341)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-341)|
|<a id="finding-342"></a>[cs-slhdsa-sha2_192f-signdata-positive.cs:1:148](#finding-342)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-342)|
|<a id="finding-343"></a>[cs-slhdsa-sha2_192f-verifydata-positive.cs:1:37](#finding-343)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-343)|
|<a id="finding-344"></a>[cs-slhdsa-sha2_192f-verifydata-positive.cs:1:148](#finding-344)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-344)|
|<a id="finding-345"></a>[cs-slhdsa-sha2_192s-positive.cs:1:37](#finding-345)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-345)|
|<a id="finding-346"></a>[cs-slhdsa-sha2_192s-signdata-positive.cs:1:37](#finding-346)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-346)|
|<a id="finding-347"></a>[cs-slhdsa-sha2_192s-signdata-positive.cs:1:148](#finding-347)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-347)|
|<a id="finding-348"></a>[cs-slhdsa-sha2_192s-verifydata-positive.cs:1:37](#finding-348)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-348)|
|<a id="finding-349"></a>[cs-slhdsa-sha2_192s-verifydata-positive.cs:1:148](#finding-349)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-349)|
|<a id="finding-350"></a>[cs-slhdsa-sha2_256f-positive.cs:1:37](#finding-350)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-350)|
|<a id="finding-351"></a>[cs-slhdsa-sha2_256f-signdata-positive.cs:1:37](#finding-351)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-351)|
|<a id="finding-352"></a>[cs-slhdsa-sha2_256f-signdata-positive.cs:1:148](#finding-352)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-352)|
|<a id="finding-353"></a>[cs-slhdsa-sha2_256f-verifydata-positive.cs:1:37](#finding-353)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-353)|
|<a id="finding-354"></a>[cs-slhdsa-sha2_256f-verifydata-positive.cs:1:148](#finding-354)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-354)|
|<a id="finding-355"></a>[cs-slhdsa-sha2_256s-positive.cs:1:37](#finding-355)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-355)|
|<a id="finding-356"></a>[cs-slhdsa-sha2_256s-signdata-positive.cs:1:37](#finding-356)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-356)|
|<a id="finding-357"></a>[cs-slhdsa-sha2_256s-signdata-positive.cs:1:148](#finding-357)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-357)|
|<a id="finding-358"></a>[cs-slhdsa-sha2_256s-verifydata-positive.cs:1:37](#finding-358)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-358)|
|<a id="finding-359"></a>[cs-slhdsa-sha2_256s-verifydata-positive.cs:1:148](#finding-359)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-359)|
|<a id="finding-360"></a>[cs-slhdsa-shake128f-positive.cs:1:37](#finding-360)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-360)|
|<a id="finding-361"></a>[cs-slhdsa-shake128f-signdata-positive.cs:1:37](#finding-361)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-361)|
|<a id="finding-362"></a>[cs-slhdsa-shake128f-signdata-positive.cs:1:148](#finding-362)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-362)|
|<a id="finding-363"></a>[cs-slhdsa-shake128f-verifydata-positive.cs:1:37](#finding-363)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-363)|
|<a id="finding-364"></a>[cs-slhdsa-shake128f-verifydata-positive.cs:1:148](#finding-364)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-364)|
|<a id="finding-365"></a>[cs-slhdsa-shake128s-positive.cs:1:37](#finding-365)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-365)|
|<a id="finding-366"></a>[cs-slhdsa-shake128s-signdata-positive.cs:1:37](#finding-366)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-366)|
|<a id="finding-367"></a>[cs-slhdsa-shake128s-signdata-positive.cs:1:148](#finding-367)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-367)|
|<a id="finding-368"></a>[cs-slhdsa-shake128s-verifydata-positive.cs:1:37](#finding-368)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-368)|
|<a id="finding-369"></a>[cs-slhdsa-shake128s-verifydata-positive.cs:1:148](#finding-369)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-369)|
|<a id="finding-370"></a>[cs-slhdsa-shake192f-positive.cs:1:37](#finding-370)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-370)|
|<a id="finding-371"></a>[cs-slhdsa-shake192f-signdata-positive.cs:1:37](#finding-371)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-371)|
|<a id="finding-372"></a>[cs-slhdsa-shake192f-signdata-positive.cs:1:148](#finding-372)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-372)|
|<a id="finding-373"></a>[cs-slhdsa-shake192f-verifydata-positive.cs:1:37](#finding-373)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-373)|
|<a id="finding-374"></a>[cs-slhdsa-shake192f-verifydata-positive.cs:1:148](#finding-374)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-374)|
|<a id="finding-375"></a>[cs-slhdsa-shake192s-positive.cs:1:37](#finding-375)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-375)|
|<a id="finding-376"></a>[cs-slhdsa-shake192s-signdata-positive.cs:1:37](#finding-376)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-376)|
|<a id="finding-377"></a>[cs-slhdsa-shake192s-signdata-positive.cs:1:148](#finding-377)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-377)|
|<a id="finding-378"></a>[cs-slhdsa-shake192s-verifydata-positive.cs:1:37](#finding-378)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-378)|
|<a id="finding-379"></a>[cs-slhdsa-shake192s-verifydata-positive.cs:1:148](#finding-379)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-379)|
|<a id="finding-380"></a>[cs-slhdsa-shake256f-positive.cs:1:37](#finding-380)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-380)|
|<a id="finding-381"></a>[cs-slhdsa-shake256f-signdata-positive.cs:1:37](#finding-381)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-381)|
|<a id="finding-382"></a>[cs-slhdsa-shake256f-signdata-positive.cs:1:148](#finding-382)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-382)|
|<a id="finding-383"></a>[cs-slhdsa-shake256f-verifydata-positive.cs:1:37](#finding-383)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-383)|
|<a id="finding-384"></a>[cs-slhdsa-shake256f-verifydata-positive.cs:1:148](#finding-384)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-384)|
|<a id="finding-385"></a>[cs-slhdsa-shake256s-positive.cs:1:37](#finding-385)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-385)|
|<a id="finding-386"></a>[cs-slhdsa-shake256s-signdata-positive.cs:1:37](#finding-386)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-386)|
|<a id="finding-387"></a>[cs-slhdsa-shake256s-signdata-positive.cs:1:148](#finding-387)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-387)|
|<a id="finding-388"></a>[cs-slhdsa-shake256s-verifydata-positive.cs:1:37](#finding-388)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-388)|
|<a id="finding-389"></a>[cs-slhdsa-shake256s-verifydata-positive.cs:1:148](#finding-389)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-389)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-4)

### cs-alias-positive.cs:1:11 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-243"></a>

### cs-alias-positive.cs:1:11

    > 1 [masked-sensitive-file-content]

### cs-alias-positive.cs:1:88 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-244"></a>

### cs-alias-positive.cs:1:88

    > 1 [masked-sensitive-file-content]

### cs-bc-keypair-mldsa.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-245"></a>

### cs-bc-keypair-mldsa.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-keypair-mldsa.cs:1:247 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-246"></a>

### cs-bc-keypair-mldsa.cs:1:247

    > 1 [masked-sensitive-file-content]

### cs-bc-keypair-mlkem.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-247"></a>

### cs-bc-keypair-mlkem.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-keypair-mlkem.cs:1:248 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-248"></a>

### cs-bc-keypair-mlkem.cs:1:248

    > 1 [masked-sensitive-file-content]

### cs-bc-keypair-slhdsa.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-249"></a>

### cs-bc-keypair-slhdsa.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-keypair-slhdsa.cs:1:258 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-250"></a>

### cs-bc-keypair-slhdsa.cs:1:258

    > 1 [masked-sensitive-file-content]

### cs-bc-ml_dsa_44-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-251"></a>

### cs-bc-ml_dsa_44-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-ml_dsa_65-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-252"></a>

### cs-bc-ml_dsa_65-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-ml_dsa_87-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-253"></a>

### cs-bc-ml_dsa_87-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-ml_kem_1024-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-254"></a>

### cs-bc-ml_kem_1024-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-ml_kem_512-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-255"></a>

### cs-bc-ml_kem_512-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-ml_kem_768-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-256"></a>

### cs-bc-ml_kem_768-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-mldsakeypairgenerator-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-257"></a>

### cs-bc-mldsakeypairgenerator-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-bc-mlkemkeypairgenerator-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-258"></a>

### cs-bc-mlkemkeypairgenerator-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mldsasigner-generatesignature-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-259"></a>

### cs-bc-operation-mldsasigner-generatesignature-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mldsasigner-generatesignature-positive.cs:1:154 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-260"></a>

### cs-bc-operation-mldsasigner-generatesignature-positive.cs:1:154

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mldsasigner-verifysignature-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-261"></a>

### cs-bc-operation-mldsasigner-verifysignature-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mldsasigner-verifysignature-positive.cs:1:154 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-262"></a>

### cs-bc-operation-mldsasigner-verifysignature-positive.cs:1:154

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mlkemdecapsulator-decapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-263"></a>

### cs-bc-operation-mlkemdecapsulator-decapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mlkemdecapsulator-decapsulate-positive.cs:1:152 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-264"></a>

### cs-bc-operation-mlkemdecapsulator-decapsulate-positive.cs:1:152

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mlkemencapsulator-encapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-265"></a>

### cs-bc-operation-mlkemencapsulator-encapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-mlkemencapsulator-encapsulate-positive.cs:1:152 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-266"></a>

### cs-bc-operation-mlkemencapsulator-encapsulate-positive.cs:1:152

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-slhdsasigner-generatesignature-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-267"></a>

### cs-bc-operation-slhdsasigner-generatesignature-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-slhdsasigner-generatesignature-positive.cs:1:164 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-268"></a>

### cs-bc-operation-slhdsasigner-generatesignature-positive.cs:1:164

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-slhdsasigner-verifysignature-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-269"></a>

### cs-bc-operation-slhdsasigner-verifysignature-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-operation-slhdsasigner-verifysignature-positive.cs:1:164 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-270"></a>

### cs-bc-operation-slhdsasigner-verifysignature-positive.cs:1:164

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_sha2_128f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-271"></a>

### cs-bc-slh_dsa_sha2_128f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_sha2_128s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-272"></a>

### cs-bc-slh_dsa_sha2_128s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_sha2_192f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-273"></a>

### cs-bc-slh_dsa_sha2_192f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_sha2_192s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-274"></a>

### cs-bc-slh_dsa_sha2_192s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_sha2_256f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-275"></a>

### cs-bc-slh_dsa_sha2_256f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_sha2_256s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-276"></a>

### cs-bc-slh_dsa_sha2_256s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_shake_128f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-277"></a>

### cs-bc-slh_dsa_shake_128f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_shake_128s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-278"></a>

### cs-bc-slh_dsa_shake_128s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_shake_192f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-279"></a>

### cs-bc-slh_dsa_shake_192f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_shake_192s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-280"></a>

### cs-bc-slh_dsa_shake_192s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_shake_256f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-281"></a>

### cs-bc-slh_dsa_shake_256f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slh_dsa_shake_256s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-282"></a>

### cs-bc-slh_dsa_shake_256s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-bc-slhdsakeypairgenerator-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-283"></a>

### cs-bc-slhdsakeypairgenerator-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-construction-compositemldsa-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-284"></a>

### cs-construction-compositemldsa-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-construction-compositemlkem-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-285"></a>

### cs-construction-compositemlkem-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-construction-hpke-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-286"></a>

### cs-construction-hpke-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mldsa-importfrompem-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-287"></a>

### cs-mldsa-importfrompem-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mldsa-importpkcs8privatekey-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-288"></a>

### cs-mldsa-importpkcs8privatekey-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mldsa-importsubjectpublickeyinfo-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-289"></a>

### cs-mldsa-importsubjectpublickeyinfo-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa44-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-290"></a>

### cs-mldsa-mldsa44-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa44-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-291"></a>

### cs-mldsa-mldsa44-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa44-signdata-positive.cs:1:138 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-292"></a>

### cs-mldsa-mldsa44-signdata-positive.cs:1:138

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa44-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-293"></a>

### cs-mldsa-mldsa44-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa44-verifydata-positive.cs:1:138 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-294"></a>

### cs-mldsa-mldsa44-verifydata-positive.cs:1:138

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa65-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-295"></a>

### cs-mldsa-mldsa65-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa65-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-296"></a>

### cs-mldsa-mldsa65-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa65-signdata-positive.cs:1:138 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-297"></a>

### cs-mldsa-mldsa65-signdata-positive.cs:1:138

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa65-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-298"></a>

### cs-mldsa-mldsa65-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa65-verifydata-positive.cs:1:138 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-299"></a>

### cs-mldsa-mldsa65-verifydata-positive.cs:1:138

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa87-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-300"></a>

### cs-mldsa-mldsa87-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa87-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-301"></a>

### cs-mldsa-mldsa87-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa87-signdata-positive.cs:1:138 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-302"></a>

### cs-mldsa-mldsa87-signdata-positive.cs:1:138

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa87-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-303"></a>

### cs-mldsa-mldsa87-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mldsa-mldsa87-verifydata-positive.cs:1:138 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-304"></a>

### cs-mldsa-mldsa87-verifydata-positive.cs:1:138

    > 1 [masked-sensitive-file-content]

### cs-mlkem-importfrompem-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-305"></a>

### cs-mlkem-importfrompem-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mlkem-importpkcs8privatekey-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-306"></a>

### cs-mlkem-importpkcs8privatekey-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mlkem-importsubjectpublickeyinfo-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-307"></a>

### cs-mlkem-importsubjectpublickeyinfo-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem1024-decapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-308"></a>

### cs-mlkem-mlkem1024-decapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem1024-decapsulate-positive.cs:1:140 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-309"></a>

### cs-mlkem-mlkem1024-decapsulate-positive.cs:1:140

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem1024-encapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-310"></a>

### cs-mlkem-mlkem1024-encapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem1024-encapsulate-positive.cs:1:140 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-311"></a>

### cs-mlkem-mlkem1024-encapsulate-positive.cs:1:140

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem1024-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-312"></a>

### cs-mlkem-mlkem1024-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem512-decapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-313"></a>

### cs-mlkem-mlkem512-decapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem512-decapsulate-positive.cs:1:139 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-314"></a>

### cs-mlkem-mlkem512-decapsulate-positive.cs:1:139

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem512-encapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-315"></a>

### cs-mlkem-mlkem512-encapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem512-encapsulate-positive.cs:1:139 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-316"></a>

### cs-mlkem-mlkem512-encapsulate-positive.cs:1:139

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem512-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-317"></a>

### cs-mlkem-mlkem512-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem768-decapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-318"></a>

### cs-mlkem-mlkem768-decapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem768-decapsulate-positive.cs:1:139 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-319"></a>

### cs-mlkem-mlkem768-decapsulate-positive.cs:1:139

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem768-encapsulate-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-320"></a>

### cs-mlkem-mlkem768-encapsulate-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem768-encapsulate-positive.cs:1:139 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-321"></a>

### cs-mlkem-mlkem768-encapsulate-positive.cs:1:139

    > 1 [masked-sensitive-file-content]

### cs-mlkem-mlkem768-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-322"></a>

### cs-mlkem-mlkem768-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-rebound-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-323"></a>

### cs-rebound-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-selected-import-mldsa.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-324"></a>

### cs-selected-import-mldsa.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-selected-import-mlkem.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-325"></a>

### cs-selected-import-mlkem.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-selected-import-slhdsa.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-326"></a>

### cs-selected-import-slhdsa.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-importfrompem-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-327"></a>

### cs-slhdsa-importfrompem-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-importpkcs8privatekey-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-328"></a>

### cs-slhdsa-importpkcs8privatekey-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-importsubjectpublickeyinfo-positive.cs:1:29 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-329"></a>

### cs-slhdsa-importsubjectpublickeyinfo-positive.cs:1:29

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-330"></a>

### cs-slhdsa-sha2_128f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128f-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-331"></a>

### cs-slhdsa-sha2_128f-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128f-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-332"></a>

### cs-slhdsa-sha2_128f-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128f-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-333"></a>

### cs-slhdsa-sha2_128f-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128f-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-334"></a>

### cs-slhdsa-sha2_128f-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-335"></a>

### cs-slhdsa-sha2_128s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128s-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-336"></a>

### cs-slhdsa-sha2_128s-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128s-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-337"></a>

### cs-slhdsa-sha2_128s-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128s-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-338"></a>

### cs-slhdsa-sha2_128s-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_128s-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-339"></a>

### cs-slhdsa-sha2_128s-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-340"></a>

### cs-slhdsa-sha2_192f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192f-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-341"></a>

### cs-slhdsa-sha2_192f-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192f-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-342"></a>

### cs-slhdsa-sha2_192f-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192f-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-343"></a>

### cs-slhdsa-sha2_192f-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192f-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-344"></a>

### cs-slhdsa-sha2_192f-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-345"></a>

### cs-slhdsa-sha2_192s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192s-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-346"></a>

### cs-slhdsa-sha2_192s-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192s-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-347"></a>

### cs-slhdsa-sha2_192s-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192s-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-348"></a>

### cs-slhdsa-sha2_192s-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_192s-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-349"></a>

### cs-slhdsa-sha2_192s-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-350"></a>

### cs-slhdsa-sha2_256f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256f-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-351"></a>

### cs-slhdsa-sha2_256f-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256f-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-352"></a>

### cs-slhdsa-sha2_256f-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256f-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-353"></a>

### cs-slhdsa-sha2_256f-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256f-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-354"></a>

### cs-slhdsa-sha2_256f-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-355"></a>

### cs-slhdsa-sha2_256s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256s-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-356"></a>

### cs-slhdsa-sha2_256s-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256s-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-357"></a>

### cs-slhdsa-sha2_256s-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256s-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-358"></a>

### cs-slhdsa-sha2_256s-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-sha2_256s-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-359"></a>

### cs-slhdsa-sha2_256s-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-360"></a>

### cs-slhdsa-shake128f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128f-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-361"></a>

### cs-slhdsa-shake128f-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128f-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-362"></a>

### cs-slhdsa-shake128f-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128f-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-363"></a>

### cs-slhdsa-shake128f-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128f-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-364"></a>

### cs-slhdsa-shake128f-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-365"></a>

### cs-slhdsa-shake128s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128s-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-366"></a>

### cs-slhdsa-shake128s-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128s-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-367"></a>

### cs-slhdsa-shake128s-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128s-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-368"></a>

### cs-slhdsa-shake128s-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake128s-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-369"></a>

### cs-slhdsa-shake128s-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-370"></a>

### cs-slhdsa-shake192f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192f-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-371"></a>

### cs-slhdsa-shake192f-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192f-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-372"></a>

### cs-slhdsa-shake192f-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192f-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-373"></a>

### cs-slhdsa-shake192f-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192f-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-374"></a>

### cs-slhdsa-shake192f-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-375"></a>

### cs-slhdsa-shake192s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192s-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-376"></a>

### cs-slhdsa-shake192s-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192s-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-377"></a>

### cs-slhdsa-shake192s-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192s-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-378"></a>

### cs-slhdsa-shake192s-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake192s-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-379"></a>

### cs-slhdsa-shake192s-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256f-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-380"></a>

### cs-slhdsa-shake256f-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256f-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-381"></a>

### cs-slhdsa-shake256f-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256f-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-382"></a>

### cs-slhdsa-shake256f-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256f-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-383"></a>

### cs-slhdsa-shake256f-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256f-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-384"></a>

### cs-slhdsa-shake256f-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256s-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-385"></a>

### cs-slhdsa-shake256s-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256s-signdata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-386"></a>

### cs-slhdsa-shake256s-signdata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256s-signdata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-387"></a>

### cs-slhdsa-shake256s-signdata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256s-verifydata-positive.cs:1:37 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-388"></a>

### cs-slhdsa-shake256s-verifydata-positive.cs:1:37

    > 1 [masked-sensitive-file-content]

### cs-slhdsa-shake256s-verifydata-positive.cs:1:148 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-389"></a>

### cs-slhdsa-shake256s-verifydata-positive.cs:1:148

    > 1 [masked-sensitive-file-content]

<a id="rule-8"></a>

## C_FAMILY_PQC_API

C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection.

Check: Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-390"></a>[c-conditional-positive.c:3:14](#finding-390)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-390)|
|<a id="finding-391"></a>[c-evp-keygen-init.c:1:39](#finding-391)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-391)|
|<a id="finding-392"></a>[c-evp-keygen-init.c:1:91](#finding-392)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-392)|
|<a id="finding-393"></a>[c-evp-ml-dsa-44-evp_pkey_keygen-positive.c:1:39](#finding-393)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-393)|
|<a id="finding-394"></a>[c-evp-ml-dsa-44-evp_pkey_keygen-positive.c:1:90](#finding-394)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-394)|
|<a id="finding-395"></a>[c-evp-ml-dsa-44-positive.c:1:39](#finding-395)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-395)|
|<a id="finding-396"></a>[c-evp-ml-dsa-65-evp_pkey_keygen-positive.c:1:39](#finding-396)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-396)|
|<a id="finding-397"></a>[c-evp-ml-dsa-65-evp_pkey_keygen-positive.c:1:90](#finding-397)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-397)|
|<a id="finding-398"></a>[c-evp-ml-dsa-65-positive.c:1:39](#finding-398)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-398)|
|<a id="finding-399"></a>[c-evp-ml-dsa-87-evp_pkey_keygen-positive.c:1:39](#finding-399)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-399)|
|<a id="finding-400"></a>[c-evp-ml-dsa-87-evp_pkey_keygen-positive.c:1:90](#finding-400)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-400)|
|<a id="finding-401"></a>[c-evp-ml-dsa-87-positive.c:1:39](#finding-401)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-401)|
|<a id="finding-402"></a>[c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.c:1:39](#finding-402)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-402)|
|<a id="finding-403"></a>[c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.c:1:92](#finding-403)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-403)|
|<a id="finding-404"></a>[c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.c:1:39](#finding-404)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-404)|
|<a id="finding-405"></a>[c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.c:1:92](#finding-405)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-405)|
|<a id="finding-406"></a>[c-evp-ml-kem-1024-positive.c:1:39](#finding-406)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-406)|
|<a id="finding-407"></a>[c-evp-ml-kem-512-evp_pkey_decapsulate-positive.c:1:39](#finding-407)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-407)|
|<a id="finding-408"></a>[c-evp-ml-kem-512-evp_pkey_decapsulate-positive.c:1:91](#finding-408)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-408)|
|<a id="finding-409"></a>[c-evp-ml-kem-512-evp_pkey_encapsulate-positive.c:1:39](#finding-409)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-409)|
|<a id="finding-410"></a>[c-evp-ml-kem-512-evp_pkey_encapsulate-positive.c:1:91](#finding-410)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-410)|
|<a id="finding-411"></a>[c-evp-ml-kem-512-positive.c:1:39](#finding-411)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-411)|
|<a id="finding-412"></a>[c-evp-ml-kem-768-evp_pkey_decapsulate-positive.c:1:39](#finding-412)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-412)|
|<a id="finding-413"></a>[c-evp-ml-kem-768-evp_pkey_decapsulate-positive.c:1:91](#finding-413)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-413)|
|<a id="finding-414"></a>[c-evp-ml-kem-768-evp_pkey_encapsulate-positive.c:1:39](#finding-414)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-414)|
|<a id="finding-415"></a>[c-evp-ml-kem-768-evp_pkey_encapsulate-positive.c:1:91](#finding-415)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-415)|
|<a id="finding-416"></a>[c-evp-ml-kem-768-positive.c:1:39](#finding-416)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-416)|
|<a id="finding-417"></a>[c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.c:1:39](#finding-417)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-417)|
|<a id="finding-418"></a>[c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.c:1:98](#finding-418)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-418)|
|<a id="finding-419"></a>[c-evp-slh-dsa-sha2-128f-positive.c:1:39](#finding-419)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-419)|
|<a id="finding-420"></a>[c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.c:1:39](#finding-420)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-420)|
|<a id="finding-421"></a>[c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.c:1:98](#finding-421)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-421)|
|<a id="finding-422"></a>[c-evp-slh-dsa-sha2-128s-positive.c:1:39](#finding-422)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-422)|
|<a id="finding-423"></a>[c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.c:1:39](#finding-423)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-423)|
|<a id="finding-424"></a>[c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.c:1:98](#finding-424)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-424)|
|<a id="finding-425"></a>[c-evp-slh-dsa-sha2-192f-positive.c:1:39](#finding-425)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-425)|
|<a id="finding-426"></a>[c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.c:1:39](#finding-426)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-426)|
|<a id="finding-427"></a>[c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.c:1:98](#finding-427)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-427)|
|<a id="finding-428"></a>[c-evp-slh-dsa-sha2-192s-positive.c:1:39](#finding-428)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-428)|
|<a id="finding-429"></a>[c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.c:1:39](#finding-429)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-429)|
|<a id="finding-430"></a>[c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.c:1:98](#finding-430)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-430)|
|<a id="finding-431"></a>[c-evp-slh-dsa-sha2-256f-positive.c:1:39](#finding-431)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-431)|
|<a id="finding-432"></a>[c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.c:1:39](#finding-432)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-432)|
|<a id="finding-433"></a>[c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.c:1:98](#finding-433)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-433)|
|<a id="finding-434"></a>[c-evp-slh-dsa-sha2-256s-positive.c:1:39](#finding-434)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-434)|
|<a id="finding-435"></a>[c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.c:1:39](#finding-435)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-435)|
|<a id="finding-436"></a>[c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.c:1:99](#finding-436)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-436)|
|<a id="finding-437"></a>[c-evp-slh-dsa-shake-128f-positive.c:1:39](#finding-437)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-437)|
|<a id="finding-438"></a>[c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.c:1:39](#finding-438)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-438)|
|<a id="finding-439"></a>[c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.c:1:99](#finding-439)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-439)|
|<a id="finding-440"></a>[c-evp-slh-dsa-shake-128s-positive.c:1:39](#finding-440)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-440)|
|<a id="finding-441"></a>[c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.c:1:39](#finding-441)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-441)|
|<a id="finding-442"></a>[c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.c:1:99](#finding-442)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-442)|
|<a id="finding-443"></a>[c-evp-slh-dsa-shake-192f-positive.c:1:39](#finding-443)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-443)|
|<a id="finding-444"></a>[c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.c:1:39](#finding-444)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-444)|
|<a id="finding-445"></a>[c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.c:1:99](#finding-445)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-445)|
|<a id="finding-446"></a>[c-evp-slh-dsa-shake-192s-positive.c:1:39](#finding-446)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-446)|
|<a id="finding-447"></a>[c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.c:1:39](#finding-447)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-447)|
|<a id="finding-448"></a>[c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.c:1:99](#finding-448)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-448)|
|<a id="finding-449"></a>[c-evp-slh-dsa-shake-256f-positive.c:1:39](#finding-449)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-449)|
|<a id="finding-450"></a>[c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.c:1:39](#finding-450)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-450)|
|<a id="finding-451"></a>[c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.c:1:99](#finding-451)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-451)|
|<a id="finding-452"></a>[c-evp-slh-dsa-shake-256s-positive.c:1:39](#finding-452)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-452)|
|<a id="finding-453"></a>[c-oqs-constant-ml_dsa_65-positive.c:1:21](#finding-453)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-453)|
|<a id="finding-454"></a>[c-oqs-constant-ml_kem_768-positive.c:1:21](#finding-454)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-454)|
|<a id="finding-455"></a>[c-oqs-constant-slh_dsa_pure_sha2_128s-positive.c:1:21](#finding-455)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-455)|
|<a id="finding-456"></a>[c-oqs-ml-dsa-44-keypair-positive.c:1:34](#finding-456)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-456)|
|<a id="finding-457"></a>[c-oqs-ml-dsa-44-keypair-positive.c:1:60](#finding-457)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-457)|
|<a id="finding-458"></a>[c-oqs-ml-dsa-44-positive.c:1:34](#finding-458)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-458)|
|<a id="finding-459"></a>[c-oqs-ml-dsa-44-sign-positive.c:1:34](#finding-459)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-459)|
|<a id="finding-460"></a>[c-oqs-ml-dsa-44-sign-positive.c:1:60](#finding-460)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-460)|
|<a id="finding-461"></a>[c-oqs-ml-dsa-44-verify-positive.c:1:34](#finding-461)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-461)|
|<a id="finding-462"></a>[c-oqs-ml-dsa-44-verify-positive.c:1:60](#finding-462)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-462)|
|<a id="finding-463"></a>[c-oqs-ml-dsa-65-keypair-positive.c:1:34](#finding-463)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-463)|
|<a id="finding-464"></a>[c-oqs-ml-dsa-65-keypair-positive.c:1:60](#finding-464)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-464)|
|<a id="finding-465"></a>[c-oqs-ml-dsa-65-positive.c:1:34](#finding-465)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-465)|
|<a id="finding-466"></a>[c-oqs-ml-dsa-65-sign-positive.c:1:34](#finding-466)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-466)|
|<a id="finding-467"></a>[c-oqs-ml-dsa-65-sign-positive.c:1:60](#finding-467)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-467)|
|<a id="finding-468"></a>[c-oqs-ml-dsa-65-verify-positive.c:1:34](#finding-468)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-468)|
|<a id="finding-469"></a>[c-oqs-ml-dsa-65-verify-positive.c:1:60](#finding-469)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-469)|
|<a id="finding-470"></a>[c-oqs-ml-dsa-87-keypair-positive.c:1:34](#finding-470)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-470)|
|<a id="finding-471"></a>[c-oqs-ml-dsa-87-keypair-positive.c:1:60](#finding-471)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-471)|
|<a id="finding-472"></a>[c-oqs-ml-dsa-87-positive.c:1:34](#finding-472)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-472)|
|<a id="finding-473"></a>[c-oqs-ml-dsa-87-sign-positive.c:1:34](#finding-473)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-473)|
|<a id="finding-474"></a>[c-oqs-ml-dsa-87-sign-positive.c:1:60](#finding-474)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-474)|
|<a id="finding-475"></a>[c-oqs-ml-dsa-87-verify-positive.c:1:34](#finding-475)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-475)|
|<a id="finding-476"></a>[c-oqs-ml-dsa-87-verify-positive.c:1:60](#finding-476)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-476)|
|<a id="finding-477"></a>[c-oqs-ml-kem-1024-decaps-positive.c:1:34](#finding-477)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-477)|
|<a id="finding-478"></a>[c-oqs-ml-kem-1024-decaps-positive.c:1:62](#finding-478)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-478)|
|<a id="finding-479"></a>[c-oqs-ml-kem-1024-encaps-positive.c:1:34](#finding-479)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-479)|
|<a id="finding-480"></a>[c-oqs-ml-kem-1024-encaps-positive.c:1:62](#finding-480)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-480)|
|<a id="finding-481"></a>[c-oqs-ml-kem-1024-keypair-positive.c:1:34](#finding-481)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-481)|
|<a id="finding-482"></a>[c-oqs-ml-kem-1024-keypair-positive.c:1:62](#finding-482)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-482)|
|<a id="finding-483"></a>[c-oqs-ml-kem-1024-positive.c:1:34](#finding-483)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-483)|
|<a id="finding-484"></a>[c-oqs-ml-kem-512-decaps-positive.c:1:34](#finding-484)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-484)|
|<a id="finding-485"></a>[c-oqs-ml-kem-512-decaps-positive.c:1:61](#finding-485)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-485)|
|<a id="finding-486"></a>[c-oqs-ml-kem-512-encaps-positive.c:1:34](#finding-486)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-486)|
|<a id="finding-487"></a>[c-oqs-ml-kem-512-encaps-positive.c:1:61](#finding-487)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-487)|
|<a id="finding-488"></a>[c-oqs-ml-kem-512-keypair-positive.c:1:34](#finding-488)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-488)|
|<a id="finding-489"></a>[c-oqs-ml-kem-512-keypair-positive.c:1:61](#finding-489)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-489)|
|<a id="finding-490"></a>[c-oqs-ml-kem-512-positive.c:1:34](#finding-490)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-490)|
|<a id="finding-491"></a>[c-oqs-ml-kem-768-decaps-positive.c:1:34](#finding-491)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-491)|
|<a id="finding-492"></a>[c-oqs-ml-kem-768-decaps-positive.c:1:61](#finding-492)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-492)|
|<a id="finding-493"></a>[c-oqs-ml-kem-768-encaps-positive.c:1:34](#finding-493)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-493)|
|<a id="finding-494"></a>[c-oqs-ml-kem-768-encaps-positive.c:1:61](#finding-494)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-494)|
|<a id="finding-495"></a>[c-oqs-ml-kem-768-keypair-positive.c:1:34](#finding-495)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-495)|
|<a id="finding-496"></a>[c-oqs-ml-kem-768-keypair-positive.c:1:61](#finding-496)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-496)|
|<a id="finding-497"></a>[c-oqs-ml-kem-768-positive.c:1:34](#finding-497)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-497)|
|<a id="finding-498"></a>[c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.c:1:34](#finding-498)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-498)|
|<a id="finding-499"></a>[c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.c:1:73](#finding-499)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-499)|
|<a id="finding-500"></a>[c-oqs-slh_dsa_pure_sha2_128f-positive.c:1:34](#finding-500)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-500)|
|<a id="finding-501"></a>[c-oqs-slh_dsa_pure_sha2_128f-sign-positive.c:1:34](#finding-501)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-501)|
|<a id="finding-502"></a>[c-oqs-slh_dsa_pure_sha2_128f-sign-positive.c:1:73](#finding-502)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-502)|
|<a id="finding-503"></a>[c-oqs-slh_dsa_pure_sha2_128f-verify-positive.c:1:34](#finding-503)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-503)|
|<a id="finding-504"></a>[c-oqs-slh_dsa_pure_sha2_128f-verify-positive.c:1:73](#finding-504)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-504)|
|<a id="finding-505"></a>[c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.c:1:34](#finding-505)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-505)|
|<a id="finding-506"></a>[c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.c:1:73](#finding-506)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-506)|
|<a id="finding-507"></a>[c-oqs-slh_dsa_pure_sha2_128s-positive.c:1:34](#finding-507)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-507)|
|<a id="finding-508"></a>[c-oqs-slh_dsa_pure_sha2_128s-sign-positive.c:1:34](#finding-508)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-508)|
|<a id="finding-509"></a>[c-oqs-slh_dsa_pure_sha2_128s-sign-positive.c:1:73](#finding-509)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-509)|
|<a id="finding-510"></a>[c-oqs-slh_dsa_pure_sha2_128s-verify-positive.c:1:34](#finding-510)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-510)|
|<a id="finding-511"></a>[c-oqs-slh_dsa_pure_sha2_128s-verify-positive.c:1:73](#finding-511)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-511)|
|<a id="finding-512"></a>[c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.c:1:34](#finding-512)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-512)|
|<a id="finding-513"></a>[c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.c:1:73](#finding-513)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-513)|
|<a id="finding-514"></a>[c-oqs-slh_dsa_pure_sha2_192f-positive.c:1:34](#finding-514)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-514)|
|<a id="finding-515"></a>[c-oqs-slh_dsa_pure_sha2_192f-sign-positive.c:1:34](#finding-515)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-515)|
|<a id="finding-516"></a>[c-oqs-slh_dsa_pure_sha2_192f-sign-positive.c:1:73](#finding-516)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-516)|
|<a id="finding-517"></a>[c-oqs-slh_dsa_pure_sha2_192f-verify-positive.c:1:34](#finding-517)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-517)|
|<a id="finding-518"></a>[c-oqs-slh_dsa_pure_sha2_192f-verify-positive.c:1:73](#finding-518)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-518)|
|<a id="finding-519"></a>[c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.c:1:34](#finding-519)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-519)|
|<a id="finding-520"></a>[c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.c:1:73](#finding-520)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-520)|
|<a id="finding-521"></a>[c-oqs-slh_dsa_pure_sha2_192s-positive.c:1:34](#finding-521)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-521)|
|<a id="finding-522"></a>[c-oqs-slh_dsa_pure_sha2_192s-sign-positive.c:1:34](#finding-522)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-522)|
|<a id="finding-523"></a>[c-oqs-slh_dsa_pure_sha2_192s-sign-positive.c:1:73](#finding-523)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-523)|
|<a id="finding-524"></a>[c-oqs-slh_dsa_pure_sha2_192s-verify-positive.c:1:34](#finding-524)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-524)|
|<a id="finding-525"></a>[c-oqs-slh_dsa_pure_sha2_192s-verify-positive.c:1:73](#finding-525)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-525)|
|<a id="finding-526"></a>[c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.c:1:34](#finding-526)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-526)|
|<a id="finding-527"></a>[c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.c:1:73](#finding-527)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-527)|
|<a id="finding-528"></a>[c-oqs-slh_dsa_pure_sha2_256f-positive.c:1:34](#finding-528)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-528)|
|<a id="finding-529"></a>[c-oqs-slh_dsa_pure_sha2_256f-sign-positive.c:1:34](#finding-529)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-529)|
|<a id="finding-530"></a>[c-oqs-slh_dsa_pure_sha2_256f-sign-positive.c:1:73](#finding-530)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-530)|
|<a id="finding-531"></a>[c-oqs-slh_dsa_pure_sha2_256f-verify-positive.c:1:34](#finding-531)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-531)|
|<a id="finding-532"></a>[c-oqs-slh_dsa_pure_sha2_256f-verify-positive.c:1:73](#finding-532)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-532)|
|<a id="finding-533"></a>[c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.c:1:34](#finding-533)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-533)|
|<a id="finding-534"></a>[c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.c:1:73](#finding-534)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-534)|
|<a id="finding-535"></a>[c-oqs-slh_dsa_pure_sha2_256s-positive.c:1:34](#finding-535)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-535)|
|<a id="finding-536"></a>[c-oqs-slh_dsa_pure_sha2_256s-sign-positive.c:1:34](#finding-536)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-536)|
|<a id="finding-537"></a>[c-oqs-slh_dsa_pure_sha2_256s-sign-positive.c:1:73](#finding-537)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-537)|
|<a id="finding-538"></a>[c-oqs-slh_dsa_pure_sha2_256s-verify-positive.c:1:34](#finding-538)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-538)|
|<a id="finding-539"></a>[c-oqs-slh_dsa_pure_sha2_256s-verify-positive.c:1:73](#finding-539)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-539)|
|<a id="finding-540"></a>[c-oqs-slh_dsa_pure_shake_128f-keypair-positive.c:1:34](#finding-540)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-540)|
|<a id="finding-541"></a>[c-oqs-slh_dsa_pure_shake_128f-keypair-positive.c:1:74](#finding-541)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-541)|
|<a id="finding-542"></a>[c-oqs-slh_dsa_pure_shake_128f-positive.c:1:34](#finding-542)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-542)|
|<a id="finding-543"></a>[c-oqs-slh_dsa_pure_shake_128f-sign-positive.c:1:34](#finding-543)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-543)|
|<a id="finding-544"></a>[c-oqs-slh_dsa_pure_shake_128f-sign-positive.c:1:74](#finding-544)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-544)|
|<a id="finding-545"></a>[c-oqs-slh_dsa_pure_shake_128f-verify-positive.c:1:34](#finding-545)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-545)|
|<a id="finding-546"></a>[c-oqs-slh_dsa_pure_shake_128f-verify-positive.c:1:74](#finding-546)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-546)|
|<a id="finding-547"></a>[c-oqs-slh_dsa_pure_shake_128s-keypair-positive.c:1:34](#finding-547)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-547)|
|<a id="finding-548"></a>[c-oqs-slh_dsa_pure_shake_128s-keypair-positive.c:1:74](#finding-548)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-548)|
|<a id="finding-549"></a>[c-oqs-slh_dsa_pure_shake_128s-positive.c:1:34](#finding-549)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-549)|
|<a id="finding-550"></a>[c-oqs-slh_dsa_pure_shake_128s-sign-positive.c:1:34](#finding-550)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-550)|
|<a id="finding-551"></a>[c-oqs-slh_dsa_pure_shake_128s-sign-positive.c:1:74](#finding-551)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-551)|
|<a id="finding-552"></a>[c-oqs-slh_dsa_pure_shake_128s-verify-positive.c:1:34](#finding-552)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-552)|
|<a id="finding-553"></a>[c-oqs-slh_dsa_pure_shake_128s-verify-positive.c:1:74](#finding-553)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-553)|
|<a id="finding-554"></a>[c-oqs-slh_dsa_pure_shake_192f-keypair-positive.c:1:34](#finding-554)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-554)|
|<a id="finding-555"></a>[c-oqs-slh_dsa_pure_shake_192f-keypair-positive.c:1:74](#finding-555)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-555)|
|<a id="finding-556"></a>[c-oqs-slh_dsa_pure_shake_192f-positive.c:1:34](#finding-556)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-556)|
|<a id="finding-557"></a>[c-oqs-slh_dsa_pure_shake_192f-sign-positive.c:1:34](#finding-557)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-557)|
|<a id="finding-558"></a>[c-oqs-slh_dsa_pure_shake_192f-sign-positive.c:1:74](#finding-558)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-558)|
|<a id="finding-559"></a>[c-oqs-slh_dsa_pure_shake_192f-verify-positive.c:1:34](#finding-559)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-559)|
|<a id="finding-560"></a>[c-oqs-slh_dsa_pure_shake_192f-verify-positive.c:1:74](#finding-560)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-560)|
|<a id="finding-561"></a>[c-oqs-slh_dsa_pure_shake_192s-keypair-positive.c:1:34](#finding-561)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-561)|
|<a id="finding-562"></a>[c-oqs-slh_dsa_pure_shake_192s-keypair-positive.c:1:74](#finding-562)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-562)|
|<a id="finding-563"></a>[c-oqs-slh_dsa_pure_shake_192s-positive.c:1:34](#finding-563)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-563)|
|<a id="finding-564"></a>[c-oqs-slh_dsa_pure_shake_192s-sign-positive.c:1:34](#finding-564)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-564)|
|<a id="finding-565"></a>[c-oqs-slh_dsa_pure_shake_192s-sign-positive.c:1:74](#finding-565)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-565)|
|<a id="finding-566"></a>[c-oqs-slh_dsa_pure_shake_192s-verify-positive.c:1:34](#finding-566)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-566)|
|<a id="finding-567"></a>[c-oqs-slh_dsa_pure_shake_192s-verify-positive.c:1:74](#finding-567)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-567)|
|<a id="finding-568"></a>[c-oqs-slh_dsa_pure_shake_256f-keypair-positive.c:1:34](#finding-568)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-568)|
|<a id="finding-569"></a>[c-oqs-slh_dsa_pure_shake_256f-keypair-positive.c:1:74](#finding-569)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-569)|
|<a id="finding-570"></a>[c-oqs-slh_dsa_pure_shake_256f-positive.c:1:34](#finding-570)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-570)|
|<a id="finding-571"></a>[c-oqs-slh_dsa_pure_shake_256f-sign-positive.c:1:34](#finding-571)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-571)|
|<a id="finding-572"></a>[c-oqs-slh_dsa_pure_shake_256f-sign-positive.c:1:74](#finding-572)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-572)|
|<a id="finding-573"></a>[c-oqs-slh_dsa_pure_shake_256f-verify-positive.c:1:34](#finding-573)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-573)|
|<a id="finding-574"></a>[c-oqs-slh_dsa_pure_shake_256f-verify-positive.c:1:74](#finding-574)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-574)|
|<a id="finding-575"></a>[c-oqs-slh_dsa_pure_shake_256s-keypair-positive.c:1:34](#finding-575)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-575)|
|<a id="finding-576"></a>[c-oqs-slh_dsa_pure_shake_256s-keypair-positive.c:1:74](#finding-576)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-576)|
|<a id="finding-577"></a>[c-oqs-slh_dsa_pure_shake_256s-positive.c:1:34](#finding-577)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-577)|
|<a id="finding-578"></a>[c-oqs-slh_dsa_pure_shake_256s-sign-positive.c:1:34](#finding-578)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-578)|
|<a id="finding-579"></a>[c-oqs-slh_dsa_pure_shake_256s-sign-positive.c:1:74](#finding-579)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-579)|
|<a id="finding-580"></a>[c-oqs-slh_dsa_pure_shake_256s-verify-positive.c:1:34](#finding-580)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-580)|
|<a id="finding-581"></a>[c-oqs-slh_dsa_pure_shake_256s-verify-positive.c:1:74](#finding-581)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-581)|
|<a id="finding-582"></a>[cpp-oqs-keyencapsulation-decap_secret-positive.cpp:1:17](#finding-582)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-582)|
|<a id="finding-583"></a>[cpp-oqs-keyencapsulation-decap_secret-positive.cpp:1:56](#finding-583)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-583)|
|<a id="finding-584"></a>[cpp-oqs-keyencapsulation-encap_secret-positive.cpp:1:17](#finding-584)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-584)|
|<a id="finding-585"></a>[cpp-oqs-keyencapsulation-encap_secret-positive.cpp:1:56](#finding-585)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-585)|
|<a id="finding-586"></a>[cpp-oqs-keyencapsulation-generate_keypair-positive.cpp:1:17](#finding-586)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-586)|
|<a id="finding-587"></a>[cpp-oqs-keyencapsulation-generate_keypair-positive.cpp:1:56](#finding-587)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-587)|
|<a id="finding-588"></a>[cpp-oqs-keyencapsulation-positive.cpp:1:17](#finding-588)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-588)|
|<a id="finding-589"></a>[cpp-oqs-signature-generate_keypair-positive.cpp:1:17](#finding-589)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-589)|
|<a id="finding-590"></a>[cpp-oqs-signature-generate_keypair-positive.cpp:1:48](#finding-590)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-590)|
|<a id="finding-591"></a>[cpp-oqs-signature-positive.cpp:1:17](#finding-591)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-591)|
|<a id="finding-592"></a>[cpp-oqs-signature-sign-positive.cpp:1:17](#finding-592)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-592)|
|<a id="finding-593"></a>[cpp-oqs-signature-sign-positive.cpp:1:48](#finding-593)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-593)|
|<a id="finding-594"></a>[cpp-oqs-signature-verify-positive.cpp:1:17](#finding-594)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-594)|
|<a id="finding-595"></a>[cpp-oqs-signature-verify-positive.cpp:1:48](#finding-595)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-595)|
|<a id="finding-596"></a>[cpp-rebound-positive.cpp:1:17](#finding-596)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-596)|
|<a id="finding-597"></a>[cpp-shared-c-conditional-positive.cpp:3:14](#finding-597)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-597)|
|<a id="finding-598"></a>[cpp-shared-c-evp-keygen-init.cpp:1:39](#finding-598)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-598)|
|<a id="finding-599"></a>[cpp-shared-c-evp-keygen-init.cpp:1:91](#finding-599)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-599)|
|<a id="finding-600"></a>[cpp-shared-c-evp-ml-dsa-44-evp_pkey_keygen-positive.cpp:1:39](#finding-600)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-600)|
|<a id="finding-601"></a>[cpp-shared-c-evp-ml-dsa-44-evp_pkey_keygen-positive.cpp:1:90](#finding-601)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-601)|
|<a id="finding-602"></a>[cpp-shared-c-evp-ml-dsa-44-positive.cpp:1:39](#finding-602)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-602)|
|<a id="finding-603"></a>[cpp-shared-c-evp-ml-dsa-65-evp_pkey_keygen-positive.cpp:1:39](#finding-603)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-603)|
|<a id="finding-604"></a>[cpp-shared-c-evp-ml-dsa-65-evp_pkey_keygen-positive.cpp:1:90](#finding-604)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-604)|
|<a id="finding-605"></a>[cpp-shared-c-evp-ml-dsa-65-positive.cpp:1:39](#finding-605)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-605)|
|<a id="finding-606"></a>[cpp-shared-c-evp-ml-dsa-87-evp_pkey_keygen-positive.cpp:1:39](#finding-606)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-606)|
|<a id="finding-607"></a>[cpp-shared-c-evp-ml-dsa-87-evp_pkey_keygen-positive.cpp:1:90](#finding-607)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-607)|
|<a id="finding-608"></a>[cpp-shared-c-evp-ml-dsa-87-positive.cpp:1:39](#finding-608)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-608)|
|<a id="finding-609"></a>[cpp-shared-c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.cpp:1:39](#finding-609)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-609)|
|<a id="finding-610"></a>[cpp-shared-c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.cpp:1:92](#finding-610)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-610)|
|<a id="finding-611"></a>[cpp-shared-c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.cpp:1:39](#finding-611)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-611)|
|<a id="finding-612"></a>[cpp-shared-c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.cpp:1:92](#finding-612)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-612)|
|<a id="finding-613"></a>[cpp-shared-c-evp-ml-kem-1024-positive.cpp:1:39](#finding-613)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-613)|
|<a id="finding-614"></a>[cpp-shared-c-evp-ml-kem-512-evp_pkey_decapsulate-positive.cpp:1:39](#finding-614)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-614)|
|<a id="finding-615"></a>[cpp-shared-c-evp-ml-kem-512-evp_pkey_decapsulate-positive.cpp:1:91](#finding-615)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-615)|
|<a id="finding-616"></a>[cpp-shared-c-evp-ml-kem-512-evp_pkey_encapsulate-positive.cpp:1:39](#finding-616)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-616)|
|<a id="finding-617"></a>[cpp-shared-c-evp-ml-kem-512-evp_pkey_encapsulate-positive.cpp:1:91](#finding-617)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-617)|
|<a id="finding-618"></a>[cpp-shared-c-evp-ml-kem-512-positive.cpp:1:39](#finding-618)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-618)|
|<a id="finding-619"></a>[cpp-shared-c-evp-ml-kem-768-evp_pkey_decapsulate-positive.cpp:1:39](#finding-619)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-619)|
|<a id="finding-620"></a>[cpp-shared-c-evp-ml-kem-768-evp_pkey_decapsulate-positive.cpp:1:91](#finding-620)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-620)|
|<a id="finding-621"></a>[cpp-shared-c-evp-ml-kem-768-evp_pkey_encapsulate-positive.cpp:1:39](#finding-621)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-621)|
|<a id="finding-622"></a>[cpp-shared-c-evp-ml-kem-768-evp_pkey_encapsulate-positive.cpp:1:91](#finding-622)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-622)|
|<a id="finding-623"></a>[cpp-shared-c-evp-ml-kem-768-positive.cpp:1:39](#finding-623)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-623)|
|<a id="finding-624"></a>[cpp-shared-c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.cpp:1:39](#finding-624)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-624)|
|<a id="finding-625"></a>[cpp-shared-c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.cpp:1:98](#finding-625)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-625)|
|<a id="finding-626"></a>[cpp-shared-c-evp-slh-dsa-sha2-128f-positive.cpp:1:39](#finding-626)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-626)|
|<a id="finding-627"></a>[cpp-shared-c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.cpp:1:39](#finding-627)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-627)|
|<a id="finding-628"></a>[cpp-shared-c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.cpp:1:98](#finding-628)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-628)|
|<a id="finding-629"></a>[cpp-shared-c-evp-slh-dsa-sha2-128s-positive.cpp:1:39](#finding-629)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-629)|
|<a id="finding-630"></a>[cpp-shared-c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.cpp:1:39](#finding-630)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-630)|
|<a id="finding-631"></a>[cpp-shared-c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.cpp:1:98](#finding-631)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-631)|
|<a id="finding-632"></a>[cpp-shared-c-evp-slh-dsa-sha2-192f-positive.cpp:1:39](#finding-632)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-632)|
|<a id="finding-633"></a>[cpp-shared-c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.cpp:1:39](#finding-633)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-633)|
|<a id="finding-634"></a>[cpp-shared-c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.cpp:1:98](#finding-634)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-634)|
|<a id="finding-635"></a>[cpp-shared-c-evp-slh-dsa-sha2-192s-positive.cpp:1:39](#finding-635)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-635)|
|<a id="finding-636"></a>[cpp-shared-c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.cpp:1:39](#finding-636)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-636)|
|<a id="finding-637"></a>[cpp-shared-c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.cpp:1:98](#finding-637)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-637)|
|<a id="finding-638"></a>[cpp-shared-c-evp-slh-dsa-sha2-256f-positive.cpp:1:39](#finding-638)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-638)|
|<a id="finding-639"></a>[cpp-shared-c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.cpp:1:39](#finding-639)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-639)|
|<a id="finding-640"></a>[cpp-shared-c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.cpp:1:98](#finding-640)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-640)|
|<a id="finding-641"></a>[cpp-shared-c-evp-slh-dsa-sha2-256s-positive.cpp:1:39](#finding-641)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-641)|
|<a id="finding-642"></a>[cpp-shared-c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.cpp:1:39](#finding-642)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-642)|
|<a id="finding-643"></a>[cpp-shared-c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.cpp:1:99](#finding-643)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-643)|
|<a id="finding-644"></a>[cpp-shared-c-evp-slh-dsa-shake-128f-positive.cpp:1:39](#finding-644)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-644)|
|<a id="finding-645"></a>[cpp-shared-c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.cpp:1:39](#finding-645)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-645)|
|<a id="finding-646"></a>[cpp-shared-c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.cpp:1:99](#finding-646)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-646)|
|<a id="finding-647"></a>[cpp-shared-c-evp-slh-dsa-shake-128s-positive.cpp:1:39](#finding-647)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-647)|
|<a id="finding-648"></a>[cpp-shared-c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.cpp:1:39](#finding-648)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-648)|
|<a id="finding-649"></a>[cpp-shared-c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.cpp:1:99](#finding-649)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-649)|
|<a id="finding-650"></a>[cpp-shared-c-evp-slh-dsa-shake-192f-positive.cpp:1:39](#finding-650)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-650)|
|<a id="finding-651"></a>[cpp-shared-c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.cpp:1:39](#finding-651)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-651)|
|<a id="finding-652"></a>[cpp-shared-c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.cpp:1:99](#finding-652)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-652)|
|<a id="finding-653"></a>[cpp-shared-c-evp-slh-dsa-shake-192s-positive.cpp:1:39](#finding-653)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-653)|
|<a id="finding-654"></a>[cpp-shared-c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.cpp:1:39](#finding-654)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-654)|
|<a id="finding-655"></a>[cpp-shared-c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.cpp:1:99](#finding-655)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-655)|
|<a id="finding-656"></a>[cpp-shared-c-evp-slh-dsa-shake-256f-positive.cpp:1:39](#finding-656)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-656)|
|<a id="finding-657"></a>[cpp-shared-c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.cpp:1:39](#finding-657)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-657)|
|<a id="finding-658"></a>[cpp-shared-c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.cpp:1:99](#finding-658)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-658)|
|<a id="finding-659"></a>[cpp-shared-c-evp-slh-dsa-shake-256s-positive.cpp:1:39](#finding-659)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-659)|
|<a id="finding-660"></a>[cpp-shared-c-oqs-constant-ml_dsa_65-positive.cpp:1:21](#finding-660)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-660)|
|<a id="finding-661"></a>[cpp-shared-c-oqs-constant-ml_kem_768-positive.cpp:1:21](#finding-661)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-661)|
|<a id="finding-662"></a>[cpp-shared-c-oqs-constant-slh_dsa_pure_sha2_128s-positive.cpp:1:21](#finding-662)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-662)|
|<a id="finding-663"></a>[cpp-shared-c-oqs-ml-dsa-44-keypair-positive.cpp:1:34](#finding-663)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-663)|
|<a id="finding-664"></a>[cpp-shared-c-oqs-ml-dsa-44-keypair-positive.cpp:1:60](#finding-664)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-664)|
|<a id="finding-665"></a>[cpp-shared-c-oqs-ml-dsa-44-positive.cpp:1:34](#finding-665)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-665)|
|<a id="finding-666"></a>[cpp-shared-c-oqs-ml-dsa-44-sign-positive.cpp:1:34](#finding-666)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-666)|
|<a id="finding-667"></a>[cpp-shared-c-oqs-ml-dsa-44-sign-positive.cpp:1:60](#finding-667)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-667)|
|<a id="finding-668"></a>[cpp-shared-c-oqs-ml-dsa-44-verify-positive.cpp:1:34](#finding-668)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-668)|
|<a id="finding-669"></a>[cpp-shared-c-oqs-ml-dsa-44-verify-positive.cpp:1:60](#finding-669)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-669)|
|<a id="finding-670"></a>[cpp-shared-c-oqs-ml-dsa-65-keypair-positive.cpp:1:34](#finding-670)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-670)|
|<a id="finding-671"></a>[cpp-shared-c-oqs-ml-dsa-65-keypair-positive.cpp:1:60](#finding-671)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-671)|
|<a id="finding-672"></a>[cpp-shared-c-oqs-ml-dsa-65-positive.cpp:1:34](#finding-672)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-672)|
|<a id="finding-673"></a>[cpp-shared-c-oqs-ml-dsa-65-sign-positive.cpp:1:34](#finding-673)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-673)|
|<a id="finding-674"></a>[cpp-shared-c-oqs-ml-dsa-65-sign-positive.cpp:1:60](#finding-674)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-674)|
|<a id="finding-675"></a>[cpp-shared-c-oqs-ml-dsa-65-verify-positive.cpp:1:34](#finding-675)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-675)|
|<a id="finding-676"></a>[cpp-shared-c-oqs-ml-dsa-65-verify-positive.cpp:1:60](#finding-676)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-676)|
|<a id="finding-677"></a>[cpp-shared-c-oqs-ml-dsa-87-keypair-positive.cpp:1:34](#finding-677)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-677)|
|<a id="finding-678"></a>[cpp-shared-c-oqs-ml-dsa-87-keypair-positive.cpp:1:60](#finding-678)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-678)|
|<a id="finding-679"></a>[cpp-shared-c-oqs-ml-dsa-87-positive.cpp:1:34](#finding-679)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-679)|
|<a id="finding-680"></a>[cpp-shared-c-oqs-ml-dsa-87-sign-positive.cpp:1:34](#finding-680)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-680)|
|<a id="finding-681"></a>[cpp-shared-c-oqs-ml-dsa-87-sign-positive.cpp:1:60](#finding-681)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-681)|
|<a id="finding-682"></a>[cpp-shared-c-oqs-ml-dsa-87-verify-positive.cpp:1:34](#finding-682)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-682)|
|<a id="finding-683"></a>[cpp-shared-c-oqs-ml-dsa-87-verify-positive.cpp:1:60](#finding-683)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-683)|
|<a id="finding-684"></a>[cpp-shared-c-oqs-ml-kem-1024-decaps-positive.cpp:1:34](#finding-684)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-684)|
|<a id="finding-685"></a>[cpp-shared-c-oqs-ml-kem-1024-decaps-positive.cpp:1:62](#finding-685)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-685)|
|<a id="finding-686"></a>[cpp-shared-c-oqs-ml-kem-1024-encaps-positive.cpp:1:34](#finding-686)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-686)|
|<a id="finding-687"></a>[cpp-shared-c-oqs-ml-kem-1024-encaps-positive.cpp:1:62](#finding-687)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-687)|
|<a id="finding-688"></a>[cpp-shared-c-oqs-ml-kem-1024-keypair-positive.cpp:1:34](#finding-688)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-688)|
|<a id="finding-689"></a>[cpp-shared-c-oqs-ml-kem-1024-keypair-positive.cpp:1:62](#finding-689)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-689)|
|<a id="finding-690"></a>[cpp-shared-c-oqs-ml-kem-1024-positive.cpp:1:34](#finding-690)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-690)|
|<a id="finding-691"></a>[cpp-shared-c-oqs-ml-kem-512-decaps-positive.cpp:1:34](#finding-691)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-691)|
|<a id="finding-692"></a>[cpp-shared-c-oqs-ml-kem-512-decaps-positive.cpp:1:61](#finding-692)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-692)|
|<a id="finding-693"></a>[cpp-shared-c-oqs-ml-kem-512-encaps-positive.cpp:1:34](#finding-693)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-693)|
|<a id="finding-694"></a>[cpp-shared-c-oqs-ml-kem-512-encaps-positive.cpp:1:61](#finding-694)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-694)|
|<a id="finding-695"></a>[cpp-shared-c-oqs-ml-kem-512-keypair-positive.cpp:1:34](#finding-695)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-695)|
|<a id="finding-696"></a>[cpp-shared-c-oqs-ml-kem-512-keypair-positive.cpp:1:61](#finding-696)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-696)|
|<a id="finding-697"></a>[cpp-shared-c-oqs-ml-kem-512-positive.cpp:1:34](#finding-697)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-697)|
|<a id="finding-698"></a>[cpp-shared-c-oqs-ml-kem-768-decaps-positive.cpp:1:34](#finding-698)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-698)|
|<a id="finding-699"></a>[cpp-shared-c-oqs-ml-kem-768-decaps-positive.cpp:1:61](#finding-699)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-699)|
|<a id="finding-700"></a>[cpp-shared-c-oqs-ml-kem-768-encaps-positive.cpp:1:34](#finding-700)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-700)|
|<a id="finding-701"></a>[cpp-shared-c-oqs-ml-kem-768-encaps-positive.cpp:1:61](#finding-701)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-701)|
|<a id="finding-702"></a>[cpp-shared-c-oqs-ml-kem-768-keypair-positive.cpp:1:34](#finding-702)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-702)|
|<a id="finding-703"></a>[cpp-shared-c-oqs-ml-kem-768-keypair-positive.cpp:1:61](#finding-703)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-703)|
|<a id="finding-704"></a>[cpp-shared-c-oqs-ml-kem-768-positive.cpp:1:34](#finding-704)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-704)|
|<a id="finding-705"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.cpp:1:34](#finding-705)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-705)|
|<a id="finding-706"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.cpp:1:73](#finding-706)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-706)|
|<a id="finding-707"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-positive.cpp:1:34](#finding-707)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-707)|
|<a id="finding-708"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-sign-positive.cpp:1:34](#finding-708)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-708)|
|<a id="finding-709"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-sign-positive.cpp:1:73](#finding-709)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-709)|
|<a id="finding-710"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-verify-positive.cpp:1:34](#finding-710)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-710)|
|<a id="finding-711"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-verify-positive.cpp:1:73](#finding-711)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-711)|
|<a id="finding-712"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.cpp:1:34](#finding-712)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-712)|
|<a id="finding-713"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.cpp:1:73](#finding-713)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-713)|
|<a id="finding-714"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-positive.cpp:1:34](#finding-714)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-714)|
|<a id="finding-715"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-sign-positive.cpp:1:34](#finding-715)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-715)|
|<a id="finding-716"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-sign-positive.cpp:1:73](#finding-716)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-716)|
|<a id="finding-717"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-verify-positive.cpp:1:34](#finding-717)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-717)|
|<a id="finding-718"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-verify-positive.cpp:1:73](#finding-718)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-718)|
|<a id="finding-719"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.cpp:1:34](#finding-719)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-719)|
|<a id="finding-720"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.cpp:1:73](#finding-720)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-720)|
|<a id="finding-721"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-positive.cpp:1:34](#finding-721)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-721)|
|<a id="finding-722"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-sign-positive.cpp:1:34](#finding-722)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-722)|
|<a id="finding-723"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-sign-positive.cpp:1:73](#finding-723)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-723)|
|<a id="finding-724"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-verify-positive.cpp:1:34](#finding-724)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-724)|
|<a id="finding-725"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-verify-positive.cpp:1:73](#finding-725)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-725)|
|<a id="finding-726"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.cpp:1:34](#finding-726)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-726)|
|<a id="finding-727"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.cpp:1:73](#finding-727)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-727)|
|<a id="finding-728"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-positive.cpp:1:34](#finding-728)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-728)|
|<a id="finding-729"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-sign-positive.cpp:1:34](#finding-729)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-729)|
|<a id="finding-730"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-sign-positive.cpp:1:73](#finding-730)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-730)|
|<a id="finding-731"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-verify-positive.cpp:1:34](#finding-731)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-731)|
|<a id="finding-732"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-verify-positive.cpp:1:73](#finding-732)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-732)|
|<a id="finding-733"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.cpp:1:34](#finding-733)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-733)|
|<a id="finding-734"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.cpp:1:73](#finding-734)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-734)|
|<a id="finding-735"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-positive.cpp:1:34](#finding-735)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-735)|
|<a id="finding-736"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-sign-positive.cpp:1:34](#finding-736)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-736)|
|<a id="finding-737"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-sign-positive.cpp:1:73](#finding-737)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-737)|
|<a id="finding-738"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-verify-positive.cpp:1:34](#finding-738)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-738)|
|<a id="finding-739"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-verify-positive.cpp:1:73](#finding-739)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-739)|
|<a id="finding-740"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.cpp:1:34](#finding-740)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-740)|
|<a id="finding-741"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.cpp:1:73](#finding-741)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-741)|
|<a id="finding-742"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-positive.cpp:1:34](#finding-742)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-742)|
|<a id="finding-743"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-sign-positive.cpp:1:34](#finding-743)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-743)|
|<a id="finding-744"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-sign-positive.cpp:1:73](#finding-744)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-744)|
|<a id="finding-745"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-verify-positive.cpp:1:34](#finding-745)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-745)|
|<a id="finding-746"></a>[cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-verify-positive.cpp:1:73](#finding-746)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-746)|
|<a id="finding-747"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-keypair-positive.cpp:1:34](#finding-747)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-747)|
|<a id="finding-748"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-keypair-positive.cpp:1:74](#finding-748)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-748)|
|<a id="finding-749"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-positive.cpp:1:34](#finding-749)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-749)|
|<a id="finding-750"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-sign-positive.cpp:1:34](#finding-750)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-750)|
|<a id="finding-751"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-sign-positive.cpp:1:74](#finding-751)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-751)|
|<a id="finding-752"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-verify-positive.cpp:1:34](#finding-752)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-752)|
|<a id="finding-753"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128f-verify-positive.cpp:1:74](#finding-753)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-753)|
|<a id="finding-754"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-keypair-positive.cpp:1:34](#finding-754)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-754)|
|<a id="finding-755"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-keypair-positive.cpp:1:74](#finding-755)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-755)|
|<a id="finding-756"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-positive.cpp:1:34](#finding-756)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-756)|
|<a id="finding-757"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-sign-positive.cpp:1:34](#finding-757)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-757)|
|<a id="finding-758"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-sign-positive.cpp:1:74](#finding-758)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-758)|
|<a id="finding-759"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-verify-positive.cpp:1:34](#finding-759)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-759)|
|<a id="finding-760"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_128s-verify-positive.cpp:1:74](#finding-760)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-760)|
|<a id="finding-761"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-keypair-positive.cpp:1:34](#finding-761)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-761)|
|<a id="finding-762"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-keypair-positive.cpp:1:74](#finding-762)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-762)|
|<a id="finding-763"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-positive.cpp:1:34](#finding-763)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-763)|
|<a id="finding-764"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-sign-positive.cpp:1:34](#finding-764)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-764)|
|<a id="finding-765"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-sign-positive.cpp:1:74](#finding-765)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-765)|
|<a id="finding-766"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-verify-positive.cpp:1:34](#finding-766)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-766)|
|<a id="finding-767"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192f-verify-positive.cpp:1:74](#finding-767)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-767)|
|<a id="finding-768"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-keypair-positive.cpp:1:34](#finding-768)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-768)|
|<a id="finding-769"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-keypair-positive.cpp:1:74](#finding-769)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-769)|
|<a id="finding-770"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-positive.cpp:1:34](#finding-770)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-770)|
|<a id="finding-771"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-sign-positive.cpp:1:34](#finding-771)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-771)|
|<a id="finding-772"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-sign-positive.cpp:1:74](#finding-772)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-772)|
|<a id="finding-773"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-verify-positive.cpp:1:34](#finding-773)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-773)|
|<a id="finding-774"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_192s-verify-positive.cpp:1:74](#finding-774)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-774)|
|<a id="finding-775"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-keypair-positive.cpp:1:34](#finding-775)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-775)|
|<a id="finding-776"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-keypair-positive.cpp:1:74](#finding-776)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-776)|
|<a id="finding-777"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-positive.cpp:1:34](#finding-777)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-777)|
|<a id="finding-778"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-sign-positive.cpp:1:34](#finding-778)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-778)|
|<a id="finding-779"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-sign-positive.cpp:1:74](#finding-779)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-779)|
|<a id="finding-780"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-verify-positive.cpp:1:34](#finding-780)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-780)|
|<a id="finding-781"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256f-verify-positive.cpp:1:74](#finding-781)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-781)|
|<a id="finding-782"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-keypair-positive.cpp:1:34](#finding-782)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-782)|
|<a id="finding-783"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-keypair-positive.cpp:1:74](#finding-783)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-783)|
|<a id="finding-784"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-positive.cpp:1:34](#finding-784)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-784)|
|<a id="finding-785"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-sign-positive.cpp:1:34](#finding-785)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-785)|
|<a id="finding-786"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-sign-positive.cpp:1:74](#finding-786)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-786)|
|<a id="finding-787"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-verify-positive.cpp:1:34](#finding-787)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-787)|
|<a id="finding-788"></a>[cpp-shared-c-oqs-slh_dsa_pure_shake_256s-verify-positive.cpp:1:74](#finding-788)|\[masked-sensitive-content\]|Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection. / [Source](#source-788)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-0)

### c-conditional-positive.c:3:14 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-390"></a>

### c-conditional-positive.c:3:14

    > 3 [masked-sensitive-file-content]

### c-evp-keygen-init.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-391"></a>

### c-evp-keygen-init.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-keygen-init.c:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-392"></a>

### c-evp-keygen-init.c:1:91

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-44-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-393"></a>

### c-evp-ml-dsa-44-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-44-evp_pkey_keygen-positive.c:1:90 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-394"></a>

### c-evp-ml-dsa-44-evp_pkey_keygen-positive.c:1:90

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-44-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-395"></a>

### c-evp-ml-dsa-44-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-65-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-396"></a>

### c-evp-ml-dsa-65-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-65-evp_pkey_keygen-positive.c:1:90 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-397"></a>

### c-evp-ml-dsa-65-evp_pkey_keygen-positive.c:1:90

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-65-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-398"></a>

### c-evp-ml-dsa-65-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-87-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-399"></a>

### c-evp-ml-dsa-87-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-87-evp_pkey_keygen-positive.c:1:90 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-400"></a>

### c-evp-ml-dsa-87-evp_pkey_keygen-positive.c:1:90

    > 1 [masked-sensitive-file-content]

### c-evp-ml-dsa-87-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-401"></a>

### c-evp-ml-dsa-87-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-402"></a>

### c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.c:1:92 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-403"></a>

### c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.c:1:92

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-404"></a>

### c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.c:1:92 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-405"></a>

### c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.c:1:92

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-1024-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-406"></a>

### c-evp-ml-kem-1024-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-512-evp_pkey_decapsulate-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-407"></a>

### c-evp-ml-kem-512-evp_pkey_decapsulate-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-512-evp_pkey_decapsulate-positive.c:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-408"></a>

### c-evp-ml-kem-512-evp_pkey_decapsulate-positive.c:1:91

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-512-evp_pkey_encapsulate-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-409"></a>

### c-evp-ml-kem-512-evp_pkey_encapsulate-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-512-evp_pkey_encapsulate-positive.c:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-410"></a>

### c-evp-ml-kem-512-evp_pkey_encapsulate-positive.c:1:91

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-512-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-411"></a>

### c-evp-ml-kem-512-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-768-evp_pkey_decapsulate-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-412"></a>

### c-evp-ml-kem-768-evp_pkey_decapsulate-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-768-evp_pkey_decapsulate-positive.c:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-413"></a>

### c-evp-ml-kem-768-evp_pkey_decapsulate-positive.c:1:91

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-768-evp_pkey_encapsulate-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-414"></a>

### c-evp-ml-kem-768-evp_pkey_encapsulate-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-768-evp_pkey_encapsulate-positive.c:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-415"></a>

### c-evp-ml-kem-768-evp_pkey_encapsulate-positive.c:1:91

    > 1 [masked-sensitive-file-content]

### c-evp-ml-kem-768-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-416"></a>

### c-evp-ml-kem-768-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-417"></a>

### c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.c:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-418"></a>

### c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.c:1:98

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-128f-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-419"></a>

### c-evp-slh-dsa-sha2-128f-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-420"></a>

### c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.c:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-421"></a>

### c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.c:1:98

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-128s-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-422"></a>

### c-evp-slh-dsa-sha2-128s-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-423"></a>

### c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.c:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-424"></a>

### c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.c:1:98

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-192f-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-425"></a>

### c-evp-slh-dsa-sha2-192f-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-426"></a>

### c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.c:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-427"></a>

### c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.c:1:98

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-192s-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-428"></a>

### c-evp-slh-dsa-sha2-192s-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-429"></a>

### c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.c:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-430"></a>

### c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.c:1:98

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-256f-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-431"></a>

### c-evp-slh-dsa-sha2-256f-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-432"></a>

### c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.c:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-433"></a>

### c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.c:1:98

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-sha2-256s-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-434"></a>

### c-evp-slh-dsa-sha2-256s-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-435"></a>

### c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.c:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-436"></a>

### c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.c:1:99

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-128f-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-437"></a>

### c-evp-slh-dsa-shake-128f-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-438"></a>

### c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.c:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-439"></a>

### c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.c:1:99

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-128s-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-440"></a>

### c-evp-slh-dsa-shake-128s-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-441"></a>

### c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.c:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-442"></a>

### c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.c:1:99

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-192f-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-443"></a>

### c-evp-slh-dsa-shake-192f-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-444"></a>

### c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.c:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-445"></a>

### c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.c:1:99

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-192s-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-446"></a>

### c-evp-slh-dsa-shake-192s-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-447"></a>

### c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.c:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-448"></a>

### c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.c:1:99

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-256f-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-449"></a>

### c-evp-slh-dsa-shake-256f-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-450"></a>

### c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.c:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-451"></a>

### c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.c:1:99

    > 1 [masked-sensitive-file-content]

### c-evp-slh-dsa-shake-256s-positive.c:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-452"></a>

### c-evp-slh-dsa-shake-256s-positive.c:1:39

    > 1 [masked-sensitive-file-content]

### c-oqs-constant-ml_dsa_65-positive.c:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-453"></a>

### c-oqs-constant-ml_dsa_65-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-oqs-constant-ml_kem_768-positive.c:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-454"></a>

### c-oqs-constant-ml_kem_768-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-oqs-constant-slh_dsa_pure_sha2_128s-positive.c:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-455"></a>

### c-oqs-constant-slh_dsa_pure_sha2_128s-positive.c:1:21

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-456"></a>

### c-oqs-ml-dsa-44-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-keypair-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-457"></a>

### c-oqs-ml-dsa-44-keypair-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-458"></a>

### c-oqs-ml-dsa-44-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-459"></a>

### c-oqs-ml-dsa-44-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-sign-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-460"></a>

### c-oqs-ml-dsa-44-sign-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-461"></a>

### c-oqs-ml-dsa-44-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-44-verify-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-462"></a>

### c-oqs-ml-dsa-44-verify-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-463"></a>

### c-oqs-ml-dsa-65-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-keypair-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-464"></a>

### c-oqs-ml-dsa-65-keypair-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-465"></a>

### c-oqs-ml-dsa-65-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-466"></a>

### c-oqs-ml-dsa-65-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-sign-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-467"></a>

### c-oqs-ml-dsa-65-sign-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-468"></a>

### c-oqs-ml-dsa-65-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-65-verify-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-469"></a>

### c-oqs-ml-dsa-65-verify-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-470"></a>

### c-oqs-ml-dsa-87-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-keypair-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-471"></a>

### c-oqs-ml-dsa-87-keypair-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-472"></a>

### c-oqs-ml-dsa-87-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-473"></a>

### c-oqs-ml-dsa-87-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-sign-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-474"></a>

### c-oqs-ml-dsa-87-sign-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-475"></a>

### c-oqs-ml-dsa-87-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-dsa-87-verify-positive.c:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-476"></a>

### c-oqs-ml-dsa-87-verify-positive.c:1:60

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-decaps-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-477"></a>

### c-oqs-ml-kem-1024-decaps-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-decaps-positive.c:1:62 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-478"></a>

### c-oqs-ml-kem-1024-decaps-positive.c:1:62

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-encaps-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-479"></a>

### c-oqs-ml-kem-1024-encaps-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-encaps-positive.c:1:62 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-480"></a>

### c-oqs-ml-kem-1024-encaps-positive.c:1:62

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-481"></a>

### c-oqs-ml-kem-1024-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-keypair-positive.c:1:62 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-482"></a>

### c-oqs-ml-kem-1024-keypair-positive.c:1:62

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-1024-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-483"></a>

### c-oqs-ml-kem-1024-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-decaps-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-484"></a>

### c-oqs-ml-kem-512-decaps-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-decaps-positive.c:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-485"></a>

### c-oqs-ml-kem-512-decaps-positive.c:1:61

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-encaps-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-486"></a>

### c-oqs-ml-kem-512-encaps-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-encaps-positive.c:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-487"></a>

### c-oqs-ml-kem-512-encaps-positive.c:1:61

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-488"></a>

### c-oqs-ml-kem-512-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-keypair-positive.c:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-489"></a>

### c-oqs-ml-kem-512-keypair-positive.c:1:61

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-512-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-490"></a>

### c-oqs-ml-kem-512-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-decaps-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-491"></a>

### c-oqs-ml-kem-768-decaps-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-decaps-positive.c:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-492"></a>

### c-oqs-ml-kem-768-decaps-positive.c:1:61

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-encaps-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-493"></a>

### c-oqs-ml-kem-768-encaps-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-encaps-positive.c:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-494"></a>

### c-oqs-ml-kem-768-encaps-positive.c:1:61

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-495"></a>

### c-oqs-ml-kem-768-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-keypair-positive.c:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-496"></a>

### c-oqs-ml-kem-768-keypair-positive.c:1:61

    > 1 [masked-sensitive-file-content]

### c-oqs-ml-kem-768-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-497"></a>

### c-oqs-ml-kem-768-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-498"></a>

### c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-499"></a>

### c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-500"></a>

### c-oqs-slh_dsa_pure_sha2_128f-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-501"></a>

### c-oqs-slh_dsa_pure_sha2_128f-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-sign-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-502"></a>

### c-oqs-slh_dsa_pure_sha2_128f-sign-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-503"></a>

### c-oqs-slh_dsa_pure_sha2_128f-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128f-verify-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-504"></a>

### c-oqs-slh_dsa_pure_sha2_128f-verify-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-505"></a>

### c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-506"></a>

### c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-507"></a>

### c-oqs-slh_dsa_pure_sha2_128s-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-508"></a>

### c-oqs-slh_dsa_pure_sha2_128s-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-sign-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-509"></a>

### c-oqs-slh_dsa_pure_sha2_128s-sign-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-510"></a>

### c-oqs-slh_dsa_pure_sha2_128s-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_128s-verify-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-511"></a>

### c-oqs-slh_dsa_pure_sha2_128s-verify-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-512"></a>

### c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-513"></a>

### c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-514"></a>

### c-oqs-slh_dsa_pure_sha2_192f-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-515"></a>

### c-oqs-slh_dsa_pure_sha2_192f-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-sign-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-516"></a>

### c-oqs-slh_dsa_pure_sha2_192f-sign-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-517"></a>

### c-oqs-slh_dsa_pure_sha2_192f-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192f-verify-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-518"></a>

### c-oqs-slh_dsa_pure_sha2_192f-verify-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-519"></a>

### c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-520"></a>

### c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-521"></a>

### c-oqs-slh_dsa_pure_sha2_192s-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-522"></a>

### c-oqs-slh_dsa_pure_sha2_192s-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-sign-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-523"></a>

### c-oqs-slh_dsa_pure_sha2_192s-sign-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-524"></a>

### c-oqs-slh_dsa_pure_sha2_192s-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_192s-verify-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-525"></a>

### c-oqs-slh_dsa_pure_sha2_192s-verify-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-526"></a>

### c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-527"></a>

### c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-528"></a>

### c-oqs-slh_dsa_pure_sha2_256f-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-529"></a>

### c-oqs-slh_dsa_pure_sha2_256f-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-sign-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-530"></a>

### c-oqs-slh_dsa_pure_sha2_256f-sign-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-531"></a>

### c-oqs-slh_dsa_pure_sha2_256f-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256f-verify-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-532"></a>

### c-oqs-slh_dsa_pure_sha2_256f-verify-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-533"></a>

### c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-534"></a>

### c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-535"></a>

### c-oqs-slh_dsa_pure_sha2_256s-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-536"></a>

### c-oqs-slh_dsa_pure_sha2_256s-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-sign-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-537"></a>

### c-oqs-slh_dsa_pure_sha2_256s-sign-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-538"></a>

### c-oqs-slh_dsa_pure_sha2_256s-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_sha2_256s-verify-positive.c:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-539"></a>

### c-oqs-slh_dsa_pure_sha2_256s-verify-positive.c:1:73

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-540"></a>

### c-oqs-slh_dsa_pure_shake_128f-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-keypair-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-541"></a>

### c-oqs-slh_dsa_pure_shake_128f-keypair-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-542"></a>

### c-oqs-slh_dsa_pure_shake_128f-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-543"></a>

### c-oqs-slh_dsa_pure_shake_128f-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-sign-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-544"></a>

### c-oqs-slh_dsa_pure_shake_128f-sign-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-545"></a>

### c-oqs-slh_dsa_pure_shake_128f-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128f-verify-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-546"></a>

### c-oqs-slh_dsa_pure_shake_128f-verify-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-547"></a>

### c-oqs-slh_dsa_pure_shake_128s-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-keypair-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-548"></a>

### c-oqs-slh_dsa_pure_shake_128s-keypair-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-549"></a>

### c-oqs-slh_dsa_pure_shake_128s-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-550"></a>

### c-oqs-slh_dsa_pure_shake_128s-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-sign-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-551"></a>

### c-oqs-slh_dsa_pure_shake_128s-sign-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-552"></a>

### c-oqs-slh_dsa_pure_shake_128s-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_128s-verify-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-553"></a>

### c-oqs-slh_dsa_pure_shake_128s-verify-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-554"></a>

### c-oqs-slh_dsa_pure_shake_192f-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-keypair-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-555"></a>

### c-oqs-slh_dsa_pure_shake_192f-keypair-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-556"></a>

### c-oqs-slh_dsa_pure_shake_192f-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-557"></a>

### c-oqs-slh_dsa_pure_shake_192f-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-sign-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-558"></a>

### c-oqs-slh_dsa_pure_shake_192f-sign-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-559"></a>

### c-oqs-slh_dsa_pure_shake_192f-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192f-verify-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-560"></a>

### c-oqs-slh_dsa_pure_shake_192f-verify-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-561"></a>

### c-oqs-slh_dsa_pure_shake_192s-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-keypair-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-562"></a>

### c-oqs-slh_dsa_pure_shake_192s-keypair-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-563"></a>

### c-oqs-slh_dsa_pure_shake_192s-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-564"></a>

### c-oqs-slh_dsa_pure_shake_192s-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-sign-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-565"></a>

### c-oqs-slh_dsa_pure_shake_192s-sign-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-566"></a>

### c-oqs-slh_dsa_pure_shake_192s-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_192s-verify-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-567"></a>

### c-oqs-slh_dsa_pure_shake_192s-verify-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-568"></a>

### c-oqs-slh_dsa_pure_shake_256f-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-keypair-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-569"></a>

### c-oqs-slh_dsa_pure_shake_256f-keypair-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-570"></a>

### c-oqs-slh_dsa_pure_shake_256f-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-571"></a>

### c-oqs-slh_dsa_pure_shake_256f-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-sign-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-572"></a>

### c-oqs-slh_dsa_pure_shake_256f-sign-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-573"></a>

### c-oqs-slh_dsa_pure_shake_256f-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256f-verify-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-574"></a>

### c-oqs-slh_dsa_pure_shake_256f-verify-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-keypair-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-575"></a>

### c-oqs-slh_dsa_pure_shake_256s-keypair-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-keypair-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-576"></a>

### c-oqs-slh_dsa_pure_shake_256s-keypair-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-577"></a>

### c-oqs-slh_dsa_pure_shake_256s-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-sign-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-578"></a>

### c-oqs-slh_dsa_pure_shake_256s-sign-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-sign-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-579"></a>

### c-oqs-slh_dsa_pure_shake_256s-sign-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-verify-positive.c:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-580"></a>

### c-oqs-slh_dsa_pure_shake_256s-verify-positive.c:1:34

    > 1 [masked-sensitive-file-content]

### c-oqs-slh_dsa_pure_shake_256s-verify-positive.c:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-581"></a>

### c-oqs-slh_dsa_pure_shake_256s-verify-positive.c:1:74

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-decap_secret-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-582"></a>

### cpp-oqs-keyencapsulation-decap_secret-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-decap_secret-positive.cpp:1:56 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-583"></a>

### cpp-oqs-keyencapsulation-decap_secret-positive.cpp:1:56

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-encap_secret-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-584"></a>

### cpp-oqs-keyencapsulation-encap_secret-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-encap_secret-positive.cpp:1:56 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-585"></a>

### cpp-oqs-keyencapsulation-encap_secret-positive.cpp:1:56

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-generate_keypair-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-586"></a>

### cpp-oqs-keyencapsulation-generate_keypair-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-generate_keypair-positive.cpp:1:56 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-587"></a>

### cpp-oqs-keyencapsulation-generate_keypair-positive.cpp:1:56

    > 1 [masked-sensitive-file-content]

### cpp-oqs-keyencapsulation-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-588"></a>

### cpp-oqs-keyencapsulation-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-generate_keypair-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-589"></a>

### cpp-oqs-signature-generate_keypair-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-generate_keypair-positive.cpp:1:48 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-590"></a>

### cpp-oqs-signature-generate_keypair-positive.cpp:1:48

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-591"></a>

### cpp-oqs-signature-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-sign-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-592"></a>

### cpp-oqs-signature-sign-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-sign-positive.cpp:1:48 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-593"></a>

### cpp-oqs-signature-sign-positive.cpp:1:48

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-verify-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-594"></a>

### cpp-oqs-signature-verify-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-oqs-signature-verify-positive.cpp:1:48 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-595"></a>

### cpp-oqs-signature-verify-positive.cpp:1:48

    > 1 [masked-sensitive-file-content]

### cpp-rebound-positive.cpp:1:17 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-596"></a>

### cpp-rebound-positive.cpp:1:17

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-conditional-positive.cpp:3:14 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-597"></a>

### cpp-shared-c-conditional-positive.cpp:3:14

    > 3 [masked-sensitive-file-content]

### cpp-shared-c-evp-keygen-init.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-598"></a>

### cpp-shared-c-evp-keygen-init.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-keygen-init.cpp:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-599"></a>

### cpp-shared-c-evp-keygen-init.cpp:1:91

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-44-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-600"></a>

### cpp-shared-c-evp-ml-dsa-44-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-44-evp_pkey_keygen-positive.cpp:1:90 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-601"></a>

### cpp-shared-c-evp-ml-dsa-44-evp_pkey_keygen-positive.cpp:1:90

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-44-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-602"></a>

### cpp-shared-c-evp-ml-dsa-44-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-65-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-603"></a>

### cpp-shared-c-evp-ml-dsa-65-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-65-evp_pkey_keygen-positive.cpp:1:90 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-604"></a>

### cpp-shared-c-evp-ml-dsa-65-evp_pkey_keygen-positive.cpp:1:90

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-65-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-605"></a>

### cpp-shared-c-evp-ml-dsa-65-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-87-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-606"></a>

### cpp-shared-c-evp-ml-dsa-87-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-87-evp_pkey_keygen-positive.cpp:1:90 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-607"></a>

### cpp-shared-c-evp-ml-dsa-87-evp_pkey_keygen-positive.cpp:1:90

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-dsa-87-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-608"></a>

### cpp-shared-c-evp-ml-dsa-87-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-609"></a>

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.cpp:1:92 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-610"></a>

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_decapsulate-positive.cpp:1:92

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-611"></a>

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.cpp:1:92 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-612"></a>

### cpp-shared-c-evp-ml-kem-1024-evp_pkey_encapsulate-positive.cpp:1:92

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-1024-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-613"></a>

### cpp-shared-c-evp-ml-kem-1024-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-512-evp_pkey_decapsulate-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-614"></a>

### cpp-shared-c-evp-ml-kem-512-evp_pkey_decapsulate-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-512-evp_pkey_decapsulate-positive.cpp:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-615"></a>

### cpp-shared-c-evp-ml-kem-512-evp_pkey_decapsulate-positive.cpp:1:91

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-512-evp_pkey_encapsulate-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-616"></a>

### cpp-shared-c-evp-ml-kem-512-evp_pkey_encapsulate-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-512-evp_pkey_encapsulate-positive.cpp:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-617"></a>

### cpp-shared-c-evp-ml-kem-512-evp_pkey_encapsulate-positive.cpp:1:91

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-512-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-618"></a>

### cpp-shared-c-evp-ml-kem-512-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-768-evp_pkey_decapsulate-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-619"></a>

### cpp-shared-c-evp-ml-kem-768-evp_pkey_decapsulate-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-768-evp_pkey_decapsulate-positive.cpp:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-620"></a>

### cpp-shared-c-evp-ml-kem-768-evp_pkey_decapsulate-positive.cpp:1:91

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-768-evp_pkey_encapsulate-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-621"></a>

### cpp-shared-c-evp-ml-kem-768-evp_pkey_encapsulate-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-768-evp_pkey_encapsulate-positive.cpp:1:91 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-622"></a>

### cpp-shared-c-evp-ml-kem-768-evp_pkey_encapsulate-positive.cpp:1:91

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-ml-kem-768-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-623"></a>

### cpp-shared-c-evp-ml-kem-768-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-624"></a>

### cpp-shared-c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.cpp:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-625"></a>

### cpp-shared-c-evp-slh-dsa-sha2-128f-evp_pkey_keygen-positive.cpp:1:98

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-128f-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-626"></a>

### cpp-shared-c-evp-slh-dsa-sha2-128f-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-627"></a>

### cpp-shared-c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.cpp:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-628"></a>

### cpp-shared-c-evp-slh-dsa-sha2-128s-evp_pkey_keygen-positive.cpp:1:98

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-128s-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-629"></a>

### cpp-shared-c-evp-slh-dsa-sha2-128s-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-630"></a>

### cpp-shared-c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.cpp:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-631"></a>

### cpp-shared-c-evp-slh-dsa-sha2-192f-evp_pkey_keygen-positive.cpp:1:98

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-192f-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-632"></a>

### cpp-shared-c-evp-slh-dsa-sha2-192f-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-633"></a>

### cpp-shared-c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.cpp:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-634"></a>

### cpp-shared-c-evp-slh-dsa-sha2-192s-evp_pkey_keygen-positive.cpp:1:98

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-192s-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-635"></a>

### cpp-shared-c-evp-slh-dsa-sha2-192s-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-636"></a>

### cpp-shared-c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.cpp:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-637"></a>

### cpp-shared-c-evp-slh-dsa-sha2-256f-evp_pkey_keygen-positive.cpp:1:98

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-256f-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-638"></a>

### cpp-shared-c-evp-slh-dsa-sha2-256f-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-639"></a>

### cpp-shared-c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.cpp:1:98 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-640"></a>

### cpp-shared-c-evp-slh-dsa-sha2-256s-evp_pkey_keygen-positive.cpp:1:98

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-sha2-256s-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-641"></a>

### cpp-shared-c-evp-slh-dsa-sha2-256s-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-642"></a>

### cpp-shared-c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.cpp:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-643"></a>

### cpp-shared-c-evp-slh-dsa-shake-128f-evp_pkey_keygen-positive.cpp:1:99

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-128f-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-644"></a>

### cpp-shared-c-evp-slh-dsa-shake-128f-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-645"></a>

### cpp-shared-c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.cpp:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-646"></a>

### cpp-shared-c-evp-slh-dsa-shake-128s-evp_pkey_keygen-positive.cpp:1:99

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-128s-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-647"></a>

### cpp-shared-c-evp-slh-dsa-shake-128s-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-648"></a>

### cpp-shared-c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.cpp:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-649"></a>

### cpp-shared-c-evp-slh-dsa-shake-192f-evp_pkey_keygen-positive.cpp:1:99

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-192f-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-650"></a>

### cpp-shared-c-evp-slh-dsa-shake-192f-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-651"></a>

### cpp-shared-c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.cpp:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-652"></a>

### cpp-shared-c-evp-slh-dsa-shake-192s-evp_pkey_keygen-positive.cpp:1:99

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-192s-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-653"></a>

### cpp-shared-c-evp-slh-dsa-shake-192s-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-654"></a>

### cpp-shared-c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.cpp:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-655"></a>

### cpp-shared-c-evp-slh-dsa-shake-256f-evp_pkey_keygen-positive.cpp:1:99

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-256f-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-656"></a>

### cpp-shared-c-evp-slh-dsa-shake-256f-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-657"></a>

### cpp-shared-c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.cpp:1:99 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-658"></a>

### cpp-shared-c-evp-slh-dsa-shake-256s-evp_pkey_keygen-positive.cpp:1:99

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-evp-slh-dsa-shake-256s-positive.cpp:1:39 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-659"></a>

### cpp-shared-c-evp-slh-dsa-shake-256s-positive.cpp:1:39

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-constant-ml_dsa_65-positive.cpp:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-660"></a>

### cpp-shared-c-oqs-constant-ml_dsa_65-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-constant-ml_kem_768-positive.cpp:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-661"></a>

### cpp-shared-c-oqs-constant-ml_kem_768-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-constant-slh_dsa_pure_sha2_128s-positive.cpp:1:21 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-662"></a>

### cpp-shared-c-oqs-constant-slh_dsa_pure_sha2_128s-positive.cpp:1:21

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-663"></a>

### cpp-shared-c-oqs-ml-dsa-44-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-keypair-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-664"></a>

### cpp-shared-c-oqs-ml-dsa-44-keypair-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-665"></a>

### cpp-shared-c-oqs-ml-dsa-44-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-666"></a>

### cpp-shared-c-oqs-ml-dsa-44-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-sign-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-667"></a>

### cpp-shared-c-oqs-ml-dsa-44-sign-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-668"></a>

### cpp-shared-c-oqs-ml-dsa-44-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-44-verify-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-669"></a>

### cpp-shared-c-oqs-ml-dsa-44-verify-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-670"></a>

### cpp-shared-c-oqs-ml-dsa-65-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-keypair-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-671"></a>

### cpp-shared-c-oqs-ml-dsa-65-keypair-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-672"></a>

### cpp-shared-c-oqs-ml-dsa-65-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-673"></a>

### cpp-shared-c-oqs-ml-dsa-65-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-sign-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-674"></a>

### cpp-shared-c-oqs-ml-dsa-65-sign-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-675"></a>

### cpp-shared-c-oqs-ml-dsa-65-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-65-verify-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-676"></a>

### cpp-shared-c-oqs-ml-dsa-65-verify-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-677"></a>

### cpp-shared-c-oqs-ml-dsa-87-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-keypair-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-678"></a>

### cpp-shared-c-oqs-ml-dsa-87-keypair-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-679"></a>

### cpp-shared-c-oqs-ml-dsa-87-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-680"></a>

### cpp-shared-c-oqs-ml-dsa-87-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-sign-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-681"></a>

### cpp-shared-c-oqs-ml-dsa-87-sign-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-682"></a>

### cpp-shared-c-oqs-ml-dsa-87-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-dsa-87-verify-positive.cpp:1:60 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-683"></a>

### cpp-shared-c-oqs-ml-dsa-87-verify-positive.cpp:1:60

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-decaps-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-684"></a>

### cpp-shared-c-oqs-ml-kem-1024-decaps-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-decaps-positive.cpp:1:62 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-685"></a>

### cpp-shared-c-oqs-ml-kem-1024-decaps-positive.cpp:1:62

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-encaps-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-686"></a>

### cpp-shared-c-oqs-ml-kem-1024-encaps-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-encaps-positive.cpp:1:62 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-687"></a>

### cpp-shared-c-oqs-ml-kem-1024-encaps-positive.cpp:1:62

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-688"></a>

### cpp-shared-c-oqs-ml-kem-1024-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-keypair-positive.cpp:1:62 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-689"></a>

### cpp-shared-c-oqs-ml-kem-1024-keypair-positive.cpp:1:62

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-1024-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-690"></a>

### cpp-shared-c-oqs-ml-kem-1024-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-decaps-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-691"></a>

### cpp-shared-c-oqs-ml-kem-512-decaps-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-decaps-positive.cpp:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-692"></a>

### cpp-shared-c-oqs-ml-kem-512-decaps-positive.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-encaps-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-693"></a>

### cpp-shared-c-oqs-ml-kem-512-encaps-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-encaps-positive.cpp:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-694"></a>

### cpp-shared-c-oqs-ml-kem-512-encaps-positive.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-695"></a>

### cpp-shared-c-oqs-ml-kem-512-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-keypair-positive.cpp:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-696"></a>

### cpp-shared-c-oqs-ml-kem-512-keypair-positive.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-512-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-697"></a>

### cpp-shared-c-oqs-ml-kem-512-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-decaps-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-698"></a>

### cpp-shared-c-oqs-ml-kem-768-decaps-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-decaps-positive.cpp:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-699"></a>

### cpp-shared-c-oqs-ml-kem-768-decaps-positive.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-encaps-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-700"></a>

### cpp-shared-c-oqs-ml-kem-768-encaps-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-encaps-positive.cpp:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-701"></a>

### cpp-shared-c-oqs-ml-kem-768-encaps-positive.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-702"></a>

### cpp-shared-c-oqs-ml-kem-768-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-keypair-positive.cpp:1:61 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-703"></a>

### cpp-shared-c-oqs-ml-kem-768-keypair-positive.cpp:1:61

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-ml-kem-768-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-704"></a>

### cpp-shared-c-oqs-ml-kem-768-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-705"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-706"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-keypair-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-707"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-708"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-sign-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-709"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-sign-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-710"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-verify-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-711"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128f-verify-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-712"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-713"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-keypair-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-714"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-715"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-sign-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-716"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-sign-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-717"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-verify-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-718"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_128s-verify-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-719"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-720"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-keypair-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-721"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-722"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-sign-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-723"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-sign-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-724"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-verify-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-725"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192f-verify-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-726"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-727"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-keypair-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-728"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-729"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-sign-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-730"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-sign-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-731"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-verify-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-732"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_192s-verify-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-733"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-734"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-keypair-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-735"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-736"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-sign-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-737"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-sign-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-738"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-verify-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-739"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256f-verify-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-740"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-741"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-keypair-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-742"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-743"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-sign-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-744"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-sign-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-745"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-verify-positive.cpp:1:73 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-746"></a>

### cpp-shared-c-oqs-slh_dsa_pure_sha2_256s-verify-positive.cpp:1:73

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-747"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-keypair-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-748"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-keypair-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-749"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-750"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-sign-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-751"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-sign-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-752"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-verify-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-753"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128f-verify-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-754"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-keypair-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-755"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-keypair-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-756"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-757"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-sign-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-758"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-sign-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-759"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-verify-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-760"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_128s-verify-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-761"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-keypair-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-762"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-keypair-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-763"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-764"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-sign-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-765"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-sign-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-766"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-verify-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-767"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192f-verify-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-768"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-keypair-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-769"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-keypair-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-770"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-771"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-sign-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-772"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-sign-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-773"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-verify-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-774"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_192s-verify-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-775"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-keypair-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-776"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-keypair-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-777"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-778"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-sign-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-779"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-sign-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-780"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-verify-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-781"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256f-verify-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-keypair-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-782"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-keypair-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-keypair-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-783"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-keypair-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-784"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-sign-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-785"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-sign-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-sign-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-786"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-sign-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-verify-positive.cpp:1:34 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-787"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-verify-positive.cpp:1:34

    > 1 [masked-sensitive-file-content]

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-verify-positive.cpp:1:74 — Matched rules

- `C_FAMILY_PQC_API` (info): C/C++ PQC source reference; requested parameter sets do not prove enabled mechanisms or active protection. — Verify the exact standard or draft parameter set, linked OQS/OpenSSL version and build flags. Requested algorithms are not proof of enabled or active protection.

<a id="source-788"></a>

### cpp-shared-c-oqs-slh_dsa_pure_shake_256s-verify-positive.cpp:1:74

    > 1 [masked-sensitive-file-content]

<a id="rule-9"></a>

## PHP_CRYPTO_REFERENCE

PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved.

Check: Identify the specific third-party OQS wrapper and native library. A qualified reference does not establish installation, algorithm selection or enabled functionality.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-789"></a>[php-oqs-ffi-reference.php:2:5](#finding-789)|mir-evgenii/liboqs-php FFI wrapper reference|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-6) / [Source](#source-789)|
|<a id="finding-790"></a>[php-oqs-native-reference.php:2:10](#finding-790)|Muzosh oqsphp wrapper reference|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-7) / [Source](#source-790)|

Shared checks for these locations (not verified facts about each line):

- [Verify PHP extensions and their native cryptographic backends](#action-5)
- [Introduce crypto abstraction boundary for algorithm agility](#action-3)

### php-oqs-ffi-reference.php:2:5 — Matched rules

- `PHP_CRYPTO_REFERENCE` (info): PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved. — Identify the specific third-party OQS wrapper and native library. A qualified reference does not establish installation, algorithm selection or enabled functionality.

<a id="source-789"></a>

### php-oqs-ffi-reference.php:2:5

    > 2 [masked-sensitive-file-content]

### php-oqs-native-reference.php:2:10 — Matched rules

- `PHP_CRYPTO_REFERENCE` (info): PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved. — Identify the specific third-party OQS wrapper and native library. A qualified reference does not establish installation, algorithm selection or enabled functionality.

<a id="source-790"></a>

### php-oqs-native-reference.php:2:10

    > 2 [masked-sensitive-file-content]

<a id="rule-10"></a>

## PHP_PQC_API

PHP PQC algorithm or X-Wing reference; runtime support requires verification.

Check: Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-791"></a>[php-class-alias-positive.php:2:29](#finding-791)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-791)|
|<a id="finding-792"></a>[php-compat-mldsa44-keygen-positive.php:2:1](#finding-792)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-792)|
|<a id="finding-793"></a>[php-compat-mldsa44-sign-positive.php:2:1](#finding-793)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-793)|
|<a id="finding-794"></a>[php-compat-mldsa44-verify-positive.php:2:1](#finding-794)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-794)|
|<a id="finding-795"></a>[php-compat-mldsa65-keygen-positive.php:2:1](#finding-795)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-795)|
|<a id="finding-796"></a>[php-compat-mldsa65-sign-positive.php:2:1](#finding-796)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-796)|
|<a id="finding-797"></a>[php-compat-mldsa65-verify-positive.php:2:1](#finding-797)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-797)|
|<a id="finding-798"></a>[php-compat-mldsa87-keygen-positive.php:2:1](#finding-798)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-798)|
|<a id="finding-799"></a>[php-compat-mldsa87-sign-positive.php:2:1](#finding-799)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-799)|
|<a id="finding-800"></a>[php-compat-mldsa87-verify-positive.php:2:1](#finding-800)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-800)|
|<a id="finding-801"></a>[php-compat-mlkem1024-decaps-positive.php:2:1](#finding-801)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-801)|
|<a id="finding-802"></a>[php-compat-mlkem1024-encaps-positive.php:2:1](#finding-802)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-802)|
|<a id="finding-803"></a>[php-compat-mlkem1024-keygen-positive.php:2:1](#finding-803)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-803)|
|<a id="finding-804"></a>[php-compat-mlkem512-decaps-positive.php:2:1](#finding-804)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-804)|
|<a id="finding-805"></a>[php-compat-mlkem512-encaps-positive.php:2:1](#finding-805)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-805)|
|<a id="finding-806"></a>[php-compat-mlkem512-keygen-positive.php:2:1](#finding-806)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-806)|
|<a id="finding-807"></a>[php-compat-mlkem768-decaps-positive.php:2:1](#finding-807)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-807)|
|<a id="finding-808"></a>[php-compat-mlkem768-encaps-positive.php:2:1](#finding-808)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-808)|
|<a id="finding-809"></a>[php-compat-mlkem768-keygen-positive.php:2:1](#finding-809)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-809)|
|<a id="finding-810"></a>[php-compat-xwing-decaps-positive.php:2:1](#finding-810)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-810)|
|<a id="finding-811"></a>[php-compat-xwing-encaps-positive.php:2:1](#finding-811)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-811)|
|<a id="finding-812"></a>[php-compat-xwing-keygen-positive.php:2:1](#finding-812)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-812)|
|<a id="finding-813"></a>[php-ext-mldsa44-positive.php:2:1](#finding-813)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-813)|
|<a id="finding-814"></a>[php-ext-mldsa44-sign-positive.php:2:13](#finding-814)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-814)|
|<a id="finding-815"></a>[php-ext-mldsa44-sign-positive.php:2:51](#finding-815)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-815)|
|<a id="finding-816"></a>[php-ext-mldsa44-verify-positive.php:2:13](#finding-816)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-816)|
|<a id="finding-817"></a>[php-ext-mldsa44-verify-positive.php:2:51](#finding-817)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-817)|
|<a id="finding-818"></a>[php-ext-mldsa65-positive.php:2:1](#finding-818)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-818)|
|<a id="finding-819"></a>[php-ext-mldsa65-sign-positive.php:2:13](#finding-819)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-819)|
|<a id="finding-820"></a>[php-ext-mldsa65-sign-positive.php:2:51](#finding-820)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-820)|
|<a id="finding-821"></a>[php-ext-mldsa65-verify-positive.php:2:13](#finding-821)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-821)|
|<a id="finding-822"></a>[php-ext-mldsa65-verify-positive.php:2:51](#finding-822)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-822)|
|<a id="finding-823"></a>[php-ext-mldsa87-positive.php:2:1](#finding-823)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-823)|
|<a id="finding-824"></a>[php-ext-mldsa87-sign-positive.php:2:13](#finding-824)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-824)|
|<a id="finding-825"></a>[php-ext-mldsa87-sign-positive.php:2:51](#finding-825)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-825)|
|<a id="finding-826"></a>[php-ext-mldsa87-verify-positive.php:2:13](#finding-826)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-826)|
|<a id="finding-827"></a>[php-ext-mldsa87-verify-positive.php:2:51](#finding-827)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-827)|
|<a id="finding-828"></a>[php-ext-mlkem1024-decapsulate-positive.php:2:13](#finding-828)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-828)|
|<a id="finding-829"></a>[php-ext-mlkem1024-decapsulate-positive.php:2:53](#finding-829)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-829)|
|<a id="finding-830"></a>[php-ext-mlkem1024-encapsulate-positive.php:2:13](#finding-830)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-830)|
|<a id="finding-831"></a>[php-ext-mlkem1024-encapsulate-positive.php:2:53](#finding-831)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-831)|
|<a id="finding-832"></a>[php-ext-mlkem1024-positive.php:2:1](#finding-832)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-832)|
|<a id="finding-833"></a>[php-ext-mlkem512-decapsulate-positive.php:2:13](#finding-833)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-833)|
|<a id="finding-834"></a>[php-ext-mlkem512-decapsulate-positive.php:2:52](#finding-834)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-834)|
|<a id="finding-835"></a>[php-ext-mlkem512-encapsulate-positive.php:2:13](#finding-835)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-835)|
|<a id="finding-836"></a>[php-ext-mlkem512-encapsulate-positive.php:2:52](#finding-836)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-836)|
|<a id="finding-837"></a>[php-ext-mlkem512-positive.php:2:1](#finding-837)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-837)|
|<a id="finding-838"></a>[php-ext-mlkem768-decapsulate-positive.php:2:13](#finding-838)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-838)|
|<a id="finding-839"></a>[php-ext-mlkem768-decapsulate-positive.php:2:52](#finding-839)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-839)|
|<a id="finding-840"></a>[php-ext-mlkem768-encapsulate-positive.php:2:13](#finding-840)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-840)|
|<a id="finding-841"></a>[php-ext-mlkem768-encapsulate-positive.php:2:52](#finding-841)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-841)|
|<a id="finding-842"></a>[php-ext-mlkem768-positive.php:2:1](#finding-842)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-842)|
|<a id="finding-843"></a>[php-ext-xwing-decapsulate-positive.php:2:13](#finding-843)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-843)|
|<a id="finding-844"></a>[php-ext-xwing-decapsulate-positive.php:2:49](#finding-844)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-844)|
|<a id="finding-845"></a>[php-ext-xwing-encapsulate-positive.php:2:13](#finding-845)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-845)|
|<a id="finding-846"></a>[php-ext-xwing-encapsulate-positive.php:2:49](#finding-846)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-846)|
|<a id="finding-847"></a>[php-ext-xwing-positive.php:2:1](#finding-847)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-847)|
|<a id="finding-848"></a>[php-polyfill-mldsa44-positive.php:2:1](#finding-848)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-848)|
|<a id="finding-849"></a>[php-polyfill-mldsa44-sign-positive.php:2:13](#finding-849)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-849)|
|<a id="finding-850"></a>[php-polyfill-mldsa44-sign-positive.php:2:61](#finding-850)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-850)|
|<a id="finding-851"></a>[php-polyfill-mldsa44-verify-positive.php:2:13](#finding-851)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-851)|
|<a id="finding-852"></a>[php-polyfill-mldsa44-verify-positive.php:2:61](#finding-852)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-852)|
|<a id="finding-853"></a>[php-polyfill-mldsa65-positive.php:2:1](#finding-853)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-853)|
|<a id="finding-854"></a>[php-polyfill-mldsa65-sign-positive.php:2:13](#finding-854)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-854)|
|<a id="finding-855"></a>[php-polyfill-mldsa65-sign-positive.php:2:61](#finding-855)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-855)|
|<a id="finding-856"></a>[php-polyfill-mldsa65-verify-positive.php:2:13](#finding-856)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-856)|
|<a id="finding-857"></a>[php-polyfill-mldsa65-verify-positive.php:2:61](#finding-857)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-857)|
|<a id="finding-858"></a>[php-polyfill-mldsa87-positive.php:2:1](#finding-858)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-858)|
|<a id="finding-859"></a>[php-polyfill-mldsa87-sign-positive.php:2:13](#finding-859)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-859)|
|<a id="finding-860"></a>[php-polyfill-mldsa87-sign-positive.php:2:61](#finding-860)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-860)|
|<a id="finding-861"></a>[php-polyfill-mldsa87-verify-positive.php:2:13](#finding-861)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-861)|
|<a id="finding-862"></a>[php-polyfill-mldsa87-verify-positive.php:2:61](#finding-862)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-862)|
|<a id="finding-863"></a>[php-polyfill-mlkem1024-decapsulate-positive.php:2:13](#finding-863)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-863)|
|<a id="finding-864"></a>[php-polyfill-mlkem1024-decapsulate-positive.php:2:63](#finding-864)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-864)|
|<a id="finding-865"></a>[php-polyfill-mlkem1024-encapsulate-positive.php:2:13](#finding-865)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-865)|
|<a id="finding-866"></a>[php-polyfill-mlkem1024-encapsulate-positive.php:2:63](#finding-866)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-866)|
|<a id="finding-867"></a>[php-polyfill-mlkem1024-positive.php:2:1](#finding-867)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-867)|
|<a id="finding-868"></a>[php-polyfill-mlkem512-decapsulate-positive.php:2:13](#finding-868)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-868)|
|<a id="finding-869"></a>[php-polyfill-mlkem512-decapsulate-positive.php:2:62](#finding-869)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-869)|
|<a id="finding-870"></a>[php-polyfill-mlkem512-encapsulate-positive.php:2:13](#finding-870)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-870)|
|<a id="finding-871"></a>[php-polyfill-mlkem512-encapsulate-positive.php:2:62](#finding-871)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-871)|
|<a id="finding-872"></a>[php-polyfill-mlkem512-positive.php:2:1](#finding-872)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-872)|
|<a id="finding-873"></a>[php-polyfill-mlkem768-decapsulate-positive.php:2:13](#finding-873)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-873)|
|<a id="finding-874"></a>[php-polyfill-mlkem768-decapsulate-positive.php:2:62](#finding-874)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-874)|
|<a id="finding-875"></a>[php-polyfill-mlkem768-encapsulate-positive.php:2:13](#finding-875)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-875)|
|<a id="finding-876"></a>[php-polyfill-mlkem768-encapsulate-positive.php:2:62](#finding-876)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-876)|
|<a id="finding-877"></a>[php-polyfill-mlkem768-positive.php:2:1](#finding-877)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-877)|
|<a id="finding-878"></a>[php-polyfill-xwing-decapsulate-positive.php:2:13](#finding-878)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-878)|
|<a id="finding-879"></a>[php-polyfill-xwing-decapsulate-positive.php:2:59](#finding-879)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-879)|
|<a id="finding-880"></a>[php-polyfill-xwing-encapsulate-positive.php:2:13](#finding-880)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-880)|
|<a id="finding-881"></a>[php-polyfill-xwing-encapsulate-positive.php:2:59](#finding-881)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-881)|
|<a id="finding-882"></a>[php-polyfill-xwing-positive.php:2:1](#finding-882)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-882)|
|<a id="finding-883"></a>[php-reassignment-positive.php:2:6](#finding-883)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-883)|
|<a id="finding-884"></a>[php-slh-reference-positive.php:2:1](#finding-884)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-884)|
|<a id="finding-885"></a>[php-sodium-kem_dec-positive.php:2:1](#finding-885)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-885)|
|<a id="finding-886"></a>[php-sodium-kem_enc-positive.php:2:1](#finding-886)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-886)|
|<a id="finding-887"></a>[php-sodium-kem_keypair-positive.php:2:1](#finding-887)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-887)|
|<a id="finding-888"></a>[php-sodium-kem_mlkem768_dec-positive.php:2:1](#finding-888)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-888)|
|<a id="finding-889"></a>[php-sodium-kem_mlkem768_enc-positive.php:2:1](#finding-889)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-889)|
|<a id="finding-890"></a>[php-sodium-kem_mlkem768_keypair-positive.php:2:1](#finding-890)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-890)|
|<a id="finding-891"></a>[php-use-function-positive.php:2:68](#finding-891)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-891)|
|<a id="finding-892"></a>[php-utf8-positive.php:3:1](#finding-892)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-892)|

Shared checks for these locations (not verified facts about each line):

- [Verify PHP extensions and their native cryptographic backends](#action-5)

### php-class-alias-positive.php:2:29 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-791"></a>

### php-class-alias-positive.php:2:29

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa44-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-792"></a>

### php-compat-mldsa44-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa44-sign-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-793"></a>

### php-compat-mldsa44-sign-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa44-verify-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-794"></a>

### php-compat-mldsa44-verify-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa65-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-795"></a>

### php-compat-mldsa65-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa65-sign-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-796"></a>

### php-compat-mldsa65-sign-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa65-verify-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-797"></a>

### php-compat-mldsa65-verify-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa87-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-798"></a>

### php-compat-mldsa87-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa87-sign-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-799"></a>

### php-compat-mldsa87-sign-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mldsa87-verify-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-800"></a>

### php-compat-mldsa87-verify-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem1024-decaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-801"></a>

### php-compat-mlkem1024-decaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem1024-encaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-802"></a>

### php-compat-mlkem1024-encaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem1024-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-803"></a>

### php-compat-mlkem1024-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem512-decaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-804"></a>

### php-compat-mlkem512-decaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem512-encaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-805"></a>

### php-compat-mlkem512-encaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem512-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-806"></a>

### php-compat-mlkem512-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem768-decaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-807"></a>

### php-compat-mlkem768-decaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem768-encaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-808"></a>

### php-compat-mlkem768-encaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-mlkem768-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-809"></a>

### php-compat-mlkem768-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-xwing-decaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-810"></a>

### php-compat-xwing-decaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-xwing-encaps-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-811"></a>

### php-compat-xwing-encaps-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-compat-xwing-keygen-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-812"></a>

### php-compat-xwing-keygen-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa44-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-813"></a>

### php-ext-mldsa44-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa44-sign-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-814"></a>

### php-ext-mldsa44-sign-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa44-sign-positive.php:2:51 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-815"></a>

### php-ext-mldsa44-sign-positive.php:2:51

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa44-verify-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-816"></a>

### php-ext-mldsa44-verify-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa44-verify-positive.php:2:51 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-817"></a>

### php-ext-mldsa44-verify-positive.php:2:51

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa65-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-818"></a>

### php-ext-mldsa65-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa65-sign-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-819"></a>

### php-ext-mldsa65-sign-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa65-sign-positive.php:2:51 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-820"></a>

### php-ext-mldsa65-sign-positive.php:2:51

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa65-verify-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-821"></a>

### php-ext-mldsa65-verify-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa65-verify-positive.php:2:51 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-822"></a>

### php-ext-mldsa65-verify-positive.php:2:51

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa87-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-823"></a>

### php-ext-mldsa87-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa87-sign-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-824"></a>

### php-ext-mldsa87-sign-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa87-sign-positive.php:2:51 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-825"></a>

### php-ext-mldsa87-sign-positive.php:2:51

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa87-verify-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-826"></a>

### php-ext-mldsa87-verify-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mldsa87-verify-positive.php:2:51 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-827"></a>

### php-ext-mldsa87-verify-positive.php:2:51

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem1024-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-828"></a>

### php-ext-mlkem1024-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem1024-decapsulate-positive.php:2:53 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-829"></a>

### php-ext-mlkem1024-decapsulate-positive.php:2:53

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem1024-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-830"></a>

### php-ext-mlkem1024-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem1024-encapsulate-positive.php:2:53 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-831"></a>

### php-ext-mlkem1024-encapsulate-positive.php:2:53

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem1024-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-832"></a>

### php-ext-mlkem1024-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem512-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-833"></a>

### php-ext-mlkem512-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem512-decapsulate-positive.php:2:52 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-834"></a>

### php-ext-mlkem512-decapsulate-positive.php:2:52

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem512-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-835"></a>

### php-ext-mlkem512-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem512-encapsulate-positive.php:2:52 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-836"></a>

### php-ext-mlkem512-encapsulate-positive.php:2:52

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem512-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-837"></a>

### php-ext-mlkem512-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem768-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-838"></a>

### php-ext-mlkem768-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem768-decapsulate-positive.php:2:52 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-839"></a>

### php-ext-mlkem768-decapsulate-positive.php:2:52

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem768-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-840"></a>

### php-ext-mlkem768-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem768-encapsulate-positive.php:2:52 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-841"></a>

### php-ext-mlkem768-encapsulate-positive.php:2:52

    > 2 [masked-sensitive-file-content]

### php-ext-mlkem768-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-842"></a>

### php-ext-mlkem768-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-ext-xwing-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-843"></a>

### php-ext-xwing-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-xwing-decapsulate-positive.php:2:49 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-844"></a>

### php-ext-xwing-decapsulate-positive.php:2:49

    > 2 [masked-sensitive-file-content]

### php-ext-xwing-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-845"></a>

### php-ext-xwing-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-ext-xwing-encapsulate-positive.php:2:49 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-846"></a>

### php-ext-xwing-encapsulate-positive.php:2:49

    > 2 [masked-sensitive-file-content]

### php-ext-xwing-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-847"></a>

### php-ext-xwing-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa44-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-848"></a>

### php-polyfill-mldsa44-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa44-sign-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-849"></a>

### php-polyfill-mldsa44-sign-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa44-sign-positive.php:2:61 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-850"></a>

### php-polyfill-mldsa44-sign-positive.php:2:61

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa44-verify-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-851"></a>

### php-polyfill-mldsa44-verify-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa44-verify-positive.php:2:61 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-852"></a>

### php-polyfill-mldsa44-verify-positive.php:2:61

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa65-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-853"></a>

### php-polyfill-mldsa65-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa65-sign-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-854"></a>

### php-polyfill-mldsa65-sign-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa65-sign-positive.php:2:61 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-855"></a>

### php-polyfill-mldsa65-sign-positive.php:2:61

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa65-verify-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-856"></a>

### php-polyfill-mldsa65-verify-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa65-verify-positive.php:2:61 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-857"></a>

### php-polyfill-mldsa65-verify-positive.php:2:61

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa87-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-858"></a>

### php-polyfill-mldsa87-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa87-sign-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-859"></a>

### php-polyfill-mldsa87-sign-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa87-sign-positive.php:2:61 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-860"></a>

### php-polyfill-mldsa87-sign-positive.php:2:61

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa87-verify-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-861"></a>

### php-polyfill-mldsa87-verify-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mldsa87-verify-positive.php:2:61 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-862"></a>

### php-polyfill-mldsa87-verify-positive.php:2:61

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem1024-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-863"></a>

### php-polyfill-mlkem1024-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem1024-decapsulate-positive.php:2:63 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-864"></a>

### php-polyfill-mlkem1024-decapsulate-positive.php:2:63

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem1024-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-865"></a>

### php-polyfill-mlkem1024-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem1024-encapsulate-positive.php:2:63 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-866"></a>

### php-polyfill-mlkem1024-encapsulate-positive.php:2:63

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem1024-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-867"></a>

### php-polyfill-mlkem1024-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem512-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-868"></a>

### php-polyfill-mlkem512-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem512-decapsulate-positive.php:2:62 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-869"></a>

### php-polyfill-mlkem512-decapsulate-positive.php:2:62

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem512-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-870"></a>

### php-polyfill-mlkem512-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem512-encapsulate-positive.php:2:62 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-871"></a>

### php-polyfill-mlkem512-encapsulate-positive.php:2:62

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem512-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-872"></a>

### php-polyfill-mlkem512-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem768-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-873"></a>

### php-polyfill-mlkem768-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem768-decapsulate-positive.php:2:62 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-874"></a>

### php-polyfill-mlkem768-decapsulate-positive.php:2:62

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem768-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-875"></a>

### php-polyfill-mlkem768-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem768-encapsulate-positive.php:2:62 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-876"></a>

### php-polyfill-mlkem768-encapsulate-positive.php:2:62

    > 2 [masked-sensitive-file-content]

### php-polyfill-mlkem768-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-877"></a>

### php-polyfill-mlkem768-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-polyfill-xwing-decapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-878"></a>

### php-polyfill-xwing-decapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-xwing-decapsulate-positive.php:2:59 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-879"></a>

### php-polyfill-xwing-decapsulate-positive.php:2:59

    > 2 [masked-sensitive-file-content]

### php-polyfill-xwing-encapsulate-positive.php:2:13 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-880"></a>

### php-polyfill-xwing-encapsulate-positive.php:2:13

    > 2 [masked-sensitive-file-content]

### php-polyfill-xwing-encapsulate-positive.php:2:59 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-881"></a>

### php-polyfill-xwing-encapsulate-positive.php:2:59

    > 2 [masked-sensitive-file-content]

### php-polyfill-xwing-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-882"></a>

### php-polyfill-xwing-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-reassignment-positive.php:2:6 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-883"></a>

### php-reassignment-positive.php:2:6

    > 2 [masked-sensitive-file-content]

### php-slh-reference-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-884"></a>

### php-slh-reference-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kem_dec-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-885"></a>

### php-sodium-kem_dec-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kem_enc-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-886"></a>

### php-sodium-kem_enc-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kem_keypair-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-887"></a>

### php-sodium-kem_keypair-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kem_mlkem768_dec-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-888"></a>

### php-sodium-kem_mlkem768_dec-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kem_mlkem768_enc-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-889"></a>

### php-sodium-kem_mlkem768_enc-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-sodium-kem_mlkem768_keypair-positive.php:2:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-890"></a>

### php-sodium-kem_mlkem768_keypair-positive.php:2:1

    > 2 [masked-sensitive-file-content]

### php-use-function-positive.php:2:68 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-891"></a>

### php-use-function-positive.php:2:68

    > 2 [masked-sensitive-file-content]

### php-utf8-positive.php:3:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-892"></a>

### php-utf8-positive.php:3:1

    > 3 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Verify C/C++ library, provider and build capabilities (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. The deployed native library, build options, provider configuration and peer capabilities are unknown. vcpkg/Conan declarations, baselines and CMake references are not installed libraries or a reconstructed build graph.
2. OpenSSL 3.5 supplies ML-KEM, ML-DSA and SLH-DSA in its default provider; oqsprovider is not required for every OpenSSL PQC use. Check the exact OpenSSL build, enabled algorithms and provider/property selection.
3. For liboqs, liboqs-cpp and oqsprovider, verify separate compatible versions, disabled/enabled algorithms, native build flags and maintenance status. OQS integrations are prototypes; a wrapper/include reference does not establish deployment suitability.
4. TLS groups and provider loads are requested configuration only. Verify peers, negotiated groups, authentication signatures and protocol compatibility independently. Preserve appropriate symmetric encryption rather than replacing AES wholesale with a public-key KEM.

References:

- https://docs.openssl.org/3.5/man7/EVP_KEM-ML-KEM/
- https://openquantumsafe.org/
- https://github.com/open-quantum-safe/oqs-provider
- https://github.com/open-quantum-safe/liboqs-cpp
<a id="action-1"></a>


#### Adopt a phased hybrid PQC migration plan (near-term)

The detected primitive is vulnerable to cryptographically relevant quantum attacks. Hybrid rollout minimizes compatibility risk.

1. Inventory all call sites using this algorithm and owner teams.
2. Check for an existing classical + PQC composition; if migration is needed, evaluate compatible staged or hybrid modes before hard cut-over.
3. Define retirement dates for legacy classical algorithms.

References:

- https://www.nist.gov/itl/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
<a id="action-2"></a>


#### Review the TLS reference before changing effective settings (planned)

A source name or token does not prove enabled configuration or a negotiated connection. ECDHE_RSA/DHE_RSA use RSA authentication, not static RSA key exchange.

1. Distinguish constants, comments, tests and active settings.
2. Inspect key agreement, authentication and protocol version separately; confirm existing hybrid support and peer compatibility before migration.

References:

- https://www.rfc-editor.org/rfc/rfc8422.html#section-2.2
<a id="action-3"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography
<a id="action-4"></a>


#### Verify .NET target, package and operating-system support (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. Deployed runtime, OS, provider and native backend versions are unknown. Target frameworks, global.json SDK selection and NuGet package declarations are separate source claims, not the installed runtime.
2. .NET 10 documents MLKem, MLDsa and SlhDsa. Check the selected parameter, OS/backend support and IsSupported or IsAlgorithmSupported in the deployed environment; Linux facilities may require OpenSSL 3.5 or later. Do not generalize platform support across Windows, Linux and macOS.
3. .NET 11 CompositeMLKem and Hpke facilities require review of the exact API and experimental diagnostics. HPKE alone does not identify a PQC KEM. Check Bouncy Castle package/version independently of the .NET target.
4. ASP.NET/Kestrel TLS depends on the OS/TLS backend and peer negotiation; cryptographic API availability does not enable PQC TLS. Data Protection AES payloads are not a reason to replace symmetric encryption with a KEM.

References:

- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10/libraries
- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-11/libraries
- https://learn.microsoft.com/en-us/dotnet/standard/security/cross-platform-cryptography
- https://learn.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel/endpoints?view=aspnetcore-10.0
<a id="action-5"></a>


#### Verify PHP extensions and their native cryptographic backends (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. Deployed PHP, extension and native library versions are unknown from a source or manifest reference. Composer config.platform is emulated dependency resolution, not the deployed PHP version.
2. PHP 8.6 Sodium KEM APIs require libsodium 1.0.22 or later. Keep X-Wing separate from pure ML-KEM-768 and TLS hybrid groups; confirm the actual loaded extension and operation.
3. Check ext-pqcrypto and paragonie/pqcrypto_compat independently; a Composer package is not a native extension. Verify each documented algorithm, version and maintenance/audit status.
4. For PHP OpenSSL calls, verify the linked OpenSSL/provider and exposed PHP API. OpenSSL PQC primitives alone do not establish PHP KEM or TLS support. Framework encryption using AES does not call for blanket replacement with a public-key KEM.

References:

- https://www.php.net/manual/en/migration86.new-functions.php
- https://github.com/paragonie/ext-pqcrypto
- https://github.com/paragonie/pqcrypto_compat
- https://www.php.net/manual/en/ref.openssl.php
<a id="action-6"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. mir-evgenii/liboqs-php is an independent Composer/FFI wrapper; verify FFI configuration and the separately installed liboqs library.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/mir-evgenii/liboqs-php
<a id="action-7"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. Muzosh/liboqs-php supplies the native oqsphp extension and OQS_* classes; it is distinct from the mir-evgenii Composer/FFI wrapper.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/Muzosh/liboqs-php
<a id="action-8"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. The official open-quantum-safe/liboqs-dotnet dotnetOQS wrapper is discontinued. Do not infer an official NuGet identity from a similar package name; evaluate maintained alternatives with their own ownership and native dependencies.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/open-quantum-safe/liboqs-dotnet
<a id="action-9"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. OpenForge liboqs-dotnet is an independent community wrapper. Check the exact modular NuGet/native packages, version and architecture; it is not the discontinued official OQS wrapper.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/openforge-sh/liboqs-dotnet

## Inventory exports

- [CBOM](cbom.json): 646
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|882|
|Read successfully|882|
|Read failed|0|
|Analyzed|868|
|Partial|14|
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
|Only part of inspection completed (detector_partial): Source syntax could not be fully interpreted. Parsed regions were inspected; unresolved syntax remains unknown. Check generated syntax, language version and the source file separately.|14 records; c-conditional-positive.c; c-dynamic-evp-positive.c; c-known-evp_digestsigninit-reassigned.c|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-96d9a565581e704262f90b6780acdd9a120e48a6567ecac9429af61c851c8898|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2b9aa6908071a5a4024a4534b447c4f3816d0aac8349c667b18bd5b10b9a3b9e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7adbe910047b90d8f8f8ecc441c62cc1ac039ba6effcfba5efcf294a481dae6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea556f3023b22733439c2f1ee135401f1dac6e293d7b7d4fca2698cccda6c828|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c9a5f358fd02e83b78dbccad2b6c7051dd5bc1b93ef342277a1fc9ee9668cf2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7170f07608a6547f1eaf9d0f31ab7a66b36f46214ce3971d6ac6022fc959a75f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-656cbb460155fa1fb4a49f6d1be9872c0b5a073b7dd6be3ea6a7b1067de78fda|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f08b62486b629da114deb76bc2e48e6013a7f56cd73ef5201cb13e1df71217f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-09622f2cdda1f6e537c67b8aa4f97426f86c3da354026dae6aa6d97d48abab9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b5698a96781ea08579cb8e9bc6e32a351e0566e385898236a8c078a9db0a125f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b62a5e23566a431ffb4dc71e5d5529e9f4dee4a79694337dcff98320b95f0cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cb32a26c5ec0ea869a1a1ff4f39b4648a88d61c6fb765756b2fd69c722543d7e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bad3715b9350ca70230b6a6591762350e5c0881dea606c9f8fb17cc477c4f7d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a9a1aa6b0b0b768432d54ffd54aef5d39f2c0fc17b68e1c124d3abeae6112db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-993e39cd43a19c348043fa2e68f36e7f1e33286fb9f2ca29389b36972974ea9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e4257f7ae1c53fd3362c5fb353b7ce1562aa7671d0f470dbc1abb9b170f5e21|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d30d6c52f00a39b0b9b0a6b426040c8d3d5d6be227cac058a8240dd2f4543ae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f8f2b1b37e8df5a3fb35c0b506f2cbbfff11a3190d12be684bc91027cba82c11|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b04a518b72d1935dab10b96d9c20faf10423b3788d16cae1709f3d479877cc5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f320706da705f011ce2283c6ebfb5a9fb584cffae731128ed468df581ece40e3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7306df5deecf5c29bb387370dfed09e6bef2f72b7b176e91177ff2e7a8aa79ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e7736d8f0d56774a79f44387e843e68d407dde468fd223c519ca91fac81f118|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85962f97c92a6c815f9796be551d239588cd1e0521ddbf22c12e70a618a4ea58|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b9a569643f5937f69de993d8a1fb8bdc3863d00e3d3a121918a75a34669b332|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3027fd7acdf864cc7f8aa74eee07965cd76498a440492d6bc16778b5b294a89d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ba92b33075c1dd4096f0f301169dfca14b0874b0dfc4dd752b29d8f169a53078|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06e97abfeb0e990fc6af86b2f199e21ecadeb654eff1b1d5b4d05d467d24dc1e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a20a56b3d256c016f8ac393fa12f7fea040bb3a6a54282b32207fe02160c742|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22a3085029ebc228b53b50e536f33a979159f90de52dba9c6ff0c0c5adc973d0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f63261de9a48f3c2c39f3f8ab9b01ef34bb038ef9de37984a030a178ecb61a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-666b2ea352979725d301dba4599281b8f51fa4016371187454c202eff7ddce8b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25c3b85de1554226e6de18894e8ca5ddde1f62045baefe7c2799ef871ba5fc94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-88beab220c3a1c0a7648561aaeaaa6630b7ee9042c49f463e413c8a1dcabdedb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6617ee0592388366e244dfaa63bfa833f2fe3721bd9679eb336e623bd22693a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-262ba05576019f4d24f149b8a49f7c202ef6c79f6fb75b1d1bd4acbaab402990|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e6aca447168e9e9bd0a444af93f08e16c1bd671dd80bf1367afc9669904b2e29|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31f1113984394fcdf50f6651074e7919508e2ff0d12b06ad00b88a33f2f366ed|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-025ef5e7edf8cd6d26c56bab24ccbe2cecc8108f70e44f8229fad19496d69eee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f779d23bee9ea7abdb63c9bace77a23da83043eb572b5d9c7de97004f652b52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85e63622189d7e824005aeb7f94e170373166adf86d34059f44609a3c6853bc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e7b300c9fbc55a71a69eb6fbb7df097163c6310bc7a814a26e7461f36d75f86|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0c638a0e12703579209c5ab44a68689e23952a22ac27142207ca131851120ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2f0909abc529928dffce4291516892fd69e8031365166fedbf050adcf2d3b93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c7eea10d4d9278eaea44adca4ace7b9f273919efee9a492a945fbbdb0160fa5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-62613deff7397f8078e75e8fa830b3884be7c0cdf95894f40f807bf37992320b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf05337eeb0a2245e84b1a6713a026955cb62d38839402319d254c1c21a074e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-581acb564cda96412dc4333d8bd8cdc991e8aba57e48183f8f466cd2fd60d86d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6aa47e583ffe39898d7277c9cfe74a45f772afbf2dea1d3622f9761ca5f092c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d3de50bfd9693105f0fdb835329fb743bac4113c7845aa781c704064a372845|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fea392b46c094836d5f3a5ef6da283432bd796b99793342470d62f4ef97e641|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c48368a02cc27193700438099a90635ff08aa8dfc92b69f86ff4e5792f0d6f44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f455149b6eda5bae6606dc056a8295bd9d5dfda1ae5a06095328b9eca741c886|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5c459e5cf172bbf7cef8a596bfd3b00122a301f1e823240815c622864676797|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-418a67e53e0bc19d6670f6091499dc966053298176fbdd9720740d44fec24c2a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7467fb318808abc986768d017dcd48f930ff8cae8b7a85a88cb6bd85e79e8cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d47d08ffe045eff82dd05e03e1e9d8bd11da31cb589e0e496c78724d49b5d8ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b797094bb49ac456d3e6c93a311d880c17214a3d7d817f9ade2560e05ef4947b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c1f67374d69922fec36c97eb7613a7292e14c353102b95c460b4d51542c1bf3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-093ed0defa2e001fec2984326ea36f650e79ba66de3bbc468715d56f5c8139f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52f048e5c1f437b4c33ea53dfc998db829f4f1359e679859fc566b8808685348|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2419276839d8708fd56b125e30f1a536f931138970435c26aeb7faa929e6dd7b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f72f03e2509260a0ed2ad37d97fb3dec6c74d78980c9c487d440b8867427ca26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f1c36ff2540caedf7f50398de0bb1f923cc3e548e70412ef115b979466d7bd7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b24ed6384c296673d0b71363fc349d3d8793a0812338b630a310d163ab60773f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f622b923e17d3c4c6a9428e01b582ee4e7d93ba3adaa94670012e074c86083f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7102a80fd207cc82190d8b744250ce3cf8833c1c25c3c0db10ddb0b94f15049|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-10aa41f259a1441a4a49f55cb0789d07a11839e6b45ccfc52d08cd8bdfbc37a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96b9529c772983321c96ea6389b66b9943ca3b0963dfa378458aaf4df344dbf9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5495fd93dc6f210bdbaeddef86f0b788d2193c85668a7d6e2dbacad317eea5d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-92c45f4f1a31918d81cf8f67b891766531beb7ce19371e99026305c0d9f7d01f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dc3b7d2173b98e754d023df30f401bca1cbc4e4140a8b5dfa99ee5ecffbc938|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83425a09436284842a67dbd1b4e98bf85707d83647ce4cb043e5accb576aedb2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-03def111affe04cdbedc45c5bc1367da86181f458af28b8e25db55734f8ffa9e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-585f9e71ced221d615b5754399e04b665b786f57595a11309077f1f637c562ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad1d78deb308f3366eaf9cfa19db2365fd3f2a50f87fe6fb358963c3bbdc444e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3004c99d946d86d26123a05f92ca279102d3305829a3cd31f9115dd6b1a77334|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-734930fee1f1b7c0642da252a43959e7665ae30b7379829356bd5882d4671702|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fcabd9f158c023b04ff76e43141f71b1dddcb866ea072e9c245e5796ddd7ff5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8101b195eb991239ce8595d2fdda4790a86cafd351df5c72a57cfe67f2918558|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f5bcb7939f9127a30bef85a029ac3c5ce6d8eaae6b8675a52b1ba8238ca2774|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f37ffc463344ee9cdcc5623a34aaa939432e7e6f268240033b26c373a64b6e32|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c8a3cb8fcda05da7c08da198023e601ea02ab8511729a1776c82f893adc8dc1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c482d7fe9631e07dc7a8123780e3b9e330d4b644a5709b21d1a2821d087347bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31175033c744ae1b17b3167dcf60c3a0b1e5ebfddb36a5a1a00fe53f0d293527|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fce991388cc30f8dd966e02e6140520da2f0c567d0b349994d3e1e4dd4505b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-382e188cb97f0ea5c65afe485ebfaf40899b4472fd9c52ee3077f0dc1886ea9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a7daa72ffa4dd67965cb04824368d2bdfe469d6adcde7999b7f2d0412a13f5c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-571fe9f1414674cc6f65a90fc6b56e29399c6173478023d87d2ba6a20a6d2030|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4aa51ac8b2c250975576d08ab3e3f3bac1dc83e09262b040069cff0c4f9b8dde|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f06fd5b9fdaf6a0860b98c1d97785d88582dc8d08547918b89ddef818d079fa9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dbbd2fdbbbbfef25d22d121df168f44ff18dca27b7bc7915448f3f294589e631|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d6a0376513fbf9fe09ac0c5b55c16c2f98bb1b91beff6270d7dbd88bd753aac|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0329a0c4f9cdb8f2da851098db273b61c886674c78ecb4aeaccf6c3acd2f0df6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a31e2c1b2c10a99e202aefdbf9b0cc851ddfe528f178b9f29469159a05332a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf2b4d81aae4bc14ff183711e5dc1ad195d72f13758730bbb61caa8a5beecda7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e31f399acd2f6b74b7e2f5f6587f3c978e615e175a1646d200d5c6db7c65316e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2953e0c57ec464ab01765fea47d9c046aead2645dd7f2ba0886dd67615674aaa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-938d8d08f27f307e936dd33f98955716ba989dd719d1c4dcccf468ae9e7cd2e3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3d6dc8542e4605ac04313a7ebcfeaee91eccdc2309fd9a4861d148bcc71459e5|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a44fd4c66580f1a61f568dce07f4b8f7c51c1c0cff58d5fd533b918bf8f1c92d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-99485d7f6525cdb522e2c5e2f5a9f49e20a67e4ec7627ed678f239f4be3ea157|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b60a2c6333a8b469d8b4a388df64153aa59c62cef9d581d0e6222d4c183dae39|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8379a59ffede5953e4d88f4d9b447bc1a413addef76f3a3531e14139c698e554|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e21dced918782ecd13cf7890527f69b0f6f50d77c1f37667b39ff8688d73f5b|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b0803fa799b842877e7fc94a2acda9162ae346ea983df117efe83bf1410153c8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1494831bec4a9b8f2cd20aedc7aad7483f872b3fa8b9a31bbf44a3002e78057b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f7abc3b0bb97618b87d9a8bdd41a9b6682dea7ed3dcff99b573f734d6720b56|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74c52855bc64d2e6b7820fc7ee99c88f26f2714a068e089ee0584128839344c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0d88743f6dd5291ea4b0414456ce03bfef5a26ebb99397b166b45fcd80f38c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d228447628791af9a5f50c2d8955cc24cf5da886bac71bc79e9c7b66a7440707|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1122ac1e20dbb224a968ae18a9f6975efe7aba332b447cb119c1748bae399b4e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-23740840751c2b453af8c53473259e7d2a094f1762563b6e6ed9d971e34f6bb1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34d03a4de403e98ea8ee384740751e4d9f7bcb9bce70af331a800124bd96622e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b33b64de7145b141f4d22b37000cb6a2f7729245441f8248945142033f4bc34|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb4dee4eb5ddc6634a9b0875bf623333740bb31a42e578a02d6be17383744e39|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a44882115db72c1e32ed204478f8c26465f56e11d2efe8bef89cb6d30a788b5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9fbb71bb07bd6685c62606ae5d22cf5aba157f12d432f0e51774b13552d01395|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-51eed95f1c8e12025c8cb7f45c47f89753ae244a511711988c16f1341c40f880|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-63f2880f0520f4b86cf4647893d9164720f504b47012e40f3948d44088ea10ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05e55f9c31dee388d36f78685770c440289854d3afabd3db57f7550b2b58f4ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-92bb79d3b54bbd3bb1bfa57e26943fe15c37a15fd87ea4af1918f571dab36545|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74bcdb808481c29a81720b1afef1bba7574cb22ed2bd7e4e5f52425c06ff016f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3d405078374317e266c1fe1ddd870e26462186091f57dfc67470e3399f0afd3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-af53092327cc1a002383bbb22c1db28edcadcb5ec8f9ba4db8d0842607166964|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a25af7ab168760df6a640162ee1d953b6fa6963c0fadb4c93b01457487ef108c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad90e5464a706ef17889a7d666c07c7a3913aa2fce1335d9c7b8249c285d3ab1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5792c797201b6fabdde5cd23be251b413c2657b15c983aab73dbcd80de78c22f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-35571931d25e85a3056f3d69c3ec0f5f72120c0ccc3242fe7249edf5892f1d7c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5a8b55b60d14695cf29fe0cd4eefda52df84dea18a635979f4c3cd70b24d9df4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a97b18c03aa8f8bf260b6d7e55ac2f93106916bc5e46944cd1c39a76bb03003|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f77033f4b822234202da327597eb94f119d70b8a2bf184f86baa8b8335f8774e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7fcfc3b0cb2cc4e5ed495fc2c77b56c6aec22f425224127a58dec0679f570497|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d315f3f7bee090f6e1c20f0f110ce0cf2520ff46e1315ff88b77ebceb2cf1563|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-88f960b4cd1b45cef8757404fcdf8245c7124ad073447f359443fe16efd95c0f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-4649f59ef094c4f22eb4b978a415ada872c73b7f443e184281b4252cf453dc90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b858b421f297d228e2bcfcbc9db521470282c67a2e3a26a688dcabe1e741f23f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ab9ce41a550f99e7a795b1403f58fdd5af47e3ac00e7d28a81b9e5e81df64800|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73491002aba1f71bb7dbe743fd4e14af63e050c98b2f16a7acbab97a3f400f18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25ad817d8632a2b908135d674064d1c45dd6ce96250b3af4747f0056404e4521|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd9bb241b890bd66ee95dc15f9075075c3dcadc6dc2fcb317e3e0dbd2e89ce98|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbaf3fb5d630e1503b25564be6919c37d3cac21e7146f33af101612a9686d8c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eba1d65442f9db0610146d269cc8878d091a3c36438b6df046366df1101826a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2709e82c4be8835717f8bdceea3d7f3dc4b76abcc78305c6cb52b4a07d1535b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47362082e671437e7832f7eb68071fe70e310dac5cfc4ca27d2a27032da6911b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bf89332259346db6796a3ac2b52107dfc83c0a70688c6a5fae3f0ce5b6683063|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a5a7858e21b45f4e20a56251eb8a0e0bd8e8a2b726851e1ec95d92395594621|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec7f2a50008e7f49d01e61c87ff6e8e92c1729d92dec26c2c4f80c66ef263477|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e870e77175174de4efb06f867e33ef6568cae39fc5b28269019ad483f531a641|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-87dec890c12a2bf8df04286cb451bffad5a781923ef38bb3a05d5b178171ecae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4d183d003375e553f4807098c7ba39f6fedfbf5bc2deee7493793dea506d1fdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dae96024b21d7e36750ff19703f5e982f1c4863ebfe6f81261c1d6f17a5e706|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fad4bb53b47136b81ed6bccbd867736039288d95f17b8c0795d420f73b3c8311|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-331e45e4057c20030595fd8199571828f1b082aa96a853c0a3c50e00630b959d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f867b58c51358bc697f5d08e5f62cf8dfcf531a78f0abd83d6ff6859c921b290|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c4c350d74dcce43427939d962a5a1092d6a22ab23d41b034fd13007e2500578|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac01850651fa221f6eb119d683985e1ec29b0c2408cd6537aca8d33708887179|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-129b3d218e5cf7115091fbbeaf8647eec7a134da7fbb82cd16fffc37b9b0bcbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a272f1476cb6237c9b831fdbc487274a4ab4967627a4efde8c2b331afc0baff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8207b2702c64c125f422839501e0918f5c45aba88e0ccab335f69e3525e81d8f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e84af53be8f8a13b46a87374343c7d7dfa54ac60595fe848c49589d474a667e6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0445e9750346082adfa8ad3ace112b486a81d600a1f51fb785a029b4438de8ca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d41b26c5626060da21ceba1a99b2e9ed8b09c13a98f26425f2cadac581996d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c6dde2fa5b5e8a61f62324e41094c584cb28022a9cd1be42916d4a31820e277|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0552084594ed7916d02cd4f5d18dc37f90d90c87063312254d105197c16e414|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4da1197beb35e229bca60fb8ed7620c63cedea53dad3f643a2282a55ef605582|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ddbc4671affbeab79924c106f95a5e6245d2ceb00aeb23d275ede19de193d241|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3eb4c77182a8ec1c6006ba57266dc0e5f3cf91d898c2a126f69b74e2380b4cc4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd3055cb87bfbc118abe4cf13cc269829350654c9275eb5e1aa108ed15983ef5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-149b1b4089ab124e9096f900c8fe0c66db6a246c19bfae70e2bea608975dafbe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b8b617b676192f0e1376b63b77a1ba868cf35af4474154badb7825809178741|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e673f3605303cef9d1066820550464cd9f33953643a133da1d124abcc2497f16|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96ee1eb153476a412c0b223d418adb97609cb2bb380a2d0afd65b896a1c3da91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-45235d510605658362ff780d85eb0c0ba0e147995b2d4c89c8432c697bdf6be7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0390602c9f3c46dae08412531a894360cdbf0966fc975c30ba6a150aa30ecc71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6cba9d1584fde6f9dd66f4d84da783938d91289a647c61b1a31d0ce4c967fe03|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3499093c05d5e8101f00ff018013f85e9a76465db116b41973910ee5b7e39e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-53215dca3e4328a7d75bdeffe1cd783f72a634043acdd5fcd12ffd41c3e48724|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d846a7e8627762a20dc3c1ac6ef54d5ef8a7b2f917f33d4404fd8f3c56b1018|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c41679021a387e8a477de36780ef1240d3f1a119990d6b4e70229361aae04cee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e0d1006342ff41b07b5fb4c1fe46ee6c92ed3e62485e0c7e5ccbf3479e309954|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af28adf49d54e6d92a5d2bb8f9ccc4ad7cc8c354abbe5b789d1d26c164585b69|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2765fc76a8ec9adc36b4fd10767dafd5664b15c09eb838199eef87c8a458f771|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-913fb805c42b972f544149c9f31b0d84a628986ed61f987936f417001458f629|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-516dcf3c981690b365335c224305480306efc590e81c82c732373c66c9d59d8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e518f3b01f199974161c3793687b6327f735f3987ca988ab46f30f462d670cc2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a3d40bb832e14617fda3c5c535840614d308038e37fe026afce814e5df1430d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-66abba47cff5914764c8896433514f61abd1a3f460527726f2bb89baea7096e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32ab9289feaf03e505326c6f52db6ea245e2f6726ff357edc80234022e06b681|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6029d4ec02a2c966c0081236e4761bbdafd770f7eb6af8192383ce87a05a1719|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-82a2aee5cc942dfdac1065c83a3127406269a947dab6a735364ae52c45acc4c2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d928947051255da7796c4e12ffb286ff8661856eb0817e3020107e03882cf34a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8dcd8590de0e839cb9bea20eaa958f90bbf72b45dc078e863d74c116d2cf29e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-798b64d22f21335724d2b0d95a5b4c97bf61823081956069eaecfc5e8c9038a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3cae85868df6445b7573be2a32dec7e95a44e98d70ba27edc50ae0deca04e35|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2baca172d3c494df0240e105c68861ddbfd1daf92174a548703df1759d10d686|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d05d3e3d723d25090034be634340482bc068a126a5dcc877b192a0093e34bb1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae759e4c1c42b481cad2d6fcb256529be878d112484fd400d0c704db788322e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a88e006509eb90a6790871cc1b879c878311d1c056b1a33e06ac4d17351189be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bfb98b7fa102467cc458033f76702d23dfa34190ed126c0f9590891bfb9fb16f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e9cf3a71eaf435298bde01ad2c1f4beccc6f2886f4fd8589c262dc67d1bcf36c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc3dc54c858c5b12a9878363fc176deba00860b5cb834a93532bc8fc1c2b4460|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a20c2e186f207413298343cdd398c6e1f0b794e4ab7b16650a22e2b57eff0533|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-09198a6fef656e77d725b70cca1fa3ee7d241c85be0532360c776f496883e537|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1957d41c56d001a935e6334ff7505629c7de235e808a7ced749b4514d9f768b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ac1985cdfea80468a08d427b2357929042327c7dbecc9bdadcdd7dad72ed875|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27fd979b8d2b7cf00a1076b73e26653a6085729a8055553a58903b330cc29225|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-932ed11d971690aa232b5995b410e4777c793a0ae563d0817065db9ffe35fdb4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39bd2b93006f6952fbe0d1c6cee35f11591c141b52d6051095322a7ba592fa77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-372ac74f74dbac3b38902d095b7fa75c53e2307712d7330b4d18768380cb1054|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61aae8c8b766749d8950ffa10d2e5e73d4b989aa263fceace4d832eb93c81fc5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1717151922714942a61a3309ac2cdc47d452f96e3bc01b72438382125381d1d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e5688ca5bf3130e76db72acc87eb82596b0be3046a24f53e44150a473c87b90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa705a4e5ed4d759776e54d5e12e0737e894a515b34c3eaf32f3484a3b8602b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec940cc2419460bb2fba8b401897b48b3d763fa7c756722268055e36d323fd29|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b622aca6b112ad50fc6704d8e262a5758074828af375a3684d732b1a2848ccc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6b8fb4ed664331ab428e76c8a4a2f86c5da6e25c64eb9efc227b890eaf3439fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-793352743dff25dd0bb96157ecff9dd16632d64286bb6baad381ffb630a9f5b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36730993683e27f119c452de3965bd7fa92eeeee25fec9bf114db205f507eabc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f9d8d8e7986785dafd304eaf3e76d12e0f93790ea4eaa637257156d5d27d972|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5c0462bf2f3a8745aae894786ed297a0004fcef2cae16ab0efddb95a5f859aa5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15d23dc1baeb3974c90646b38c03726de1b96bffca114d48115186ce17b58344|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0bb7e8db69af526d3585083bf5c64e9c4d1652d8175df6ce1e04825a012774bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b4bc45e7a8ef1e077c161976db8b3d04d2f5580f647e2f4dfe93b42a4809249f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7a149323151a583a111f2fe8db0f1c4bfb394c8b9bb339d630efaa2da02a5b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bda7c1514bc77c09526e02b2302b7530ec0465410651252598168093444255aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00f087175f7b14ad646d98db02d769943be306ccf4e532342fc5c27689934fd8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af955351b3db111a19fff5fdd52dd50254aaac588a5c31406d85c16c6d849cc9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9bf695e04cf14ddee77a8b73b74b5ef6a19554ad746c7bacd62fd316c3ddde1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68abec84cb2240179f782f13587529057f108b32188f9224a3c86c299dd7614f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cb7c10ec83da61018ad6f121cb923e7775b33dc8947ae1b901aaa358503c2812|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-faf20b1b169b9edd4686d374b2c48cb818617f07bab29345025b8d896004f276|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a11eadf4db5fa27716a90080fb9acc7f3c070cea6fee683bd548d91e759b24a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe596de5e5e7082f20636b61d0a0cc150e772a10c8bb9035f52e7722d29426e8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b7e20278235d4e847ed693e424cd7ca56e3beea98ecfc54fa12855ac6c19af5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d51fd6871dcc24a9183c5711960e61a8320aee7398d23031719dc6f660040d86|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6b1c5182448a4702f0d1dcfb139c7c2e87ebe3f1e0b3fcc200dc015bdb077672|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eb79f021c501c8aea5f52b427fdf82d8be23ced55161ce0163aa8fc0c96d38c7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-33aeefd222c4e96ebfc10b0e4922b654865f655c0d86a4ed5060b80acec068d0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-036d41b3878d1168d8359ea50b62cc55d67bdd7169be203384f0070a29b5d7d2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0883ff4ceed27246d5b83692cd5bb3c2268a699941038d987b41d9c1ee4d622|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-466f989c7c6f60fe5cba3aebdf1e63638e919b13f7cfbe82be7d930228d6977b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60e2b47be7f614054e4be0eed586fad1ac14b1d9309008d7fb5f079bfeb4937c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-486c6741472ea566ef04df35745e937b89e9577293104467da019f53dcbed932|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-585ff3d968f1a797dbcd4f316a119f9500b4dfbc18d9028a9881d8150581fc5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-683dddfcae6ecfac92a4c539b31d0345a1ce80ac2936b72ab2fc2b9df1642aec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a1a4a82fa3367ab94dd6e4fc7c5a966425606d6d2696149cf3033aeade32b6e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ce8a9758a3d4186ade8de367bedb61f860d6e588ec366d00bb537a95ee2d672|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84d6d42b3ee7a7ba61606a439651d18b51242a5c38d7312d82df8a286e74c44a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f738b1d56d6a0ea31f40c389de66116fc2d91f95c0c7ee5a79b1657c0f029517|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e36bcaadf794063872585adea358a5215bd40e6d98a50aa69d76225d2fcd1574|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a6114f9580a6ba192a273f862e94547ac77a2abd060459d6cc5633f2e14a5a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d75be725e64e09e834ba8176401bcc1c00d800d4b30449a6cf37c950be634a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-613f4a17afd6b21a37ec6cf91706fea6409c5e053dc21174a7af5a2294b5f6de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c2b0955514e4bec0ae230accf71fcbe2044f7f167272dc29a024f9da4ffba08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6d1f96a4a885227dbe0b6831adfec2cc0a2ef46317b9aeca4a5dfb31f28e756|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d346a0ff26c174b6e20918fa0c292b2507055a1da68f2a5d8b282545473d5e48|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-87165a523cfe029cb56d1cfabb65e1f4a3c2712e2490125e284d832b2211524b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-325eb6d587392728ee9599d8af67808b182a882b0c8ba50c7e6b07f11d07aa94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5484c6216a2af3935940b064de4c828a6e3b49aba3b97d4b5bd2bf8cc990127|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d40e34c1493a8c83b4c08c0ff39eb391c1ba663751506986aa25f526d329a671|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f506ddfe6883953c3a58362e46d317d5801edf1f15acc45f1b0fd341c9054be6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79e2e05604b5b067bf9cd18cb28223c7f6668b4edf76a95ed022756a976d77d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5efb6ad88355217882adc03574b4aac68091224271e74a5a3be4f1557fbc0304|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea67ce5f13c297a657bdbecb6f4e2b3ff594aa64168eb37f77e74f190dc3cd36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8cc09d34a79349eba3b0c1c1715fb5169c02070d3f300a2193d81b8fe03ee966|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b623d94e1461ca254851f04287a27776071be2c4eae201b8073ce0e0e6a3050b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-445183cb7c5ae802d11e1248d1364925a6a2bd3731e88f549557abac155b420a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a57ca8b28a23ef8d4d0089bce476af9d362aeba70e0e40d93c5943f9cd29d90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-356f767e3ffccb042baaee1fc7edd8c789bb077df5cd268f1790bbed86d2128d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e253b26d04937f92d459d6e88d21aa339cbc8817787c799b521fb6ac4c1ec36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0db0190fbb260833211c31bb7a266d0721e5686dd0d3415b824ec9efad892667|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-fb04e073540e35ecef56e3bdf6974e9d3f724ed3e3e168278b02e3edde0b8e5c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-194ca25c717f3546d0a7e7f5bc730e878ff4ba3f462d2f15e8cc3873d7cab93d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0dbedc27c7215b6e1daad9d247105e9b7c39994cb3f2e0d1ad2f4eb30acc1317|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-09e67c664657f660f96fc2f844e633c106eb1ca7b9ac333e43a959cd000f83a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b147f6a7337c4457f7a3638060a5f1847226b75ebc5a62b1178f1b3b32e3b121|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6ba47ed3485d786305c3c3b56b6d46de246368a54fd79d1eb05bb65ecaa027c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce5cba6f0cc6b44295a0e5b3039a32b229eb80bb95ce63d21bdd54ec1a96de06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea17c3443a53a1591b03904a233f5d131be4781654b923099cf1401132ab529f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c38384ea6e0e8b6440e2edd6ff8fcfd006d7826c50d1a9a2f545ea1767ea860|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9abba6f86f37d670d4c2a7e25f442619a66705ddb3ae4a030a17db8066192d7f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c65e00d91e6fdc08a53480e1958071f603afadeba2804378d413139149869b5c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-47a8b87ec75259cc27e686c88a66776569bd5f5149540a9e0267e8d874d0f0d4|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-432a18d208b546a53493f4920386a05b9087b4e8d43bece1f200334045ee3588|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52841f3e946ce137b96bddfe59ad6b0309c8064c6e62147fc7543e975de45afa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc5b22f00def3215c94857d89251dfe6e47c8ae20b28c9a7e33d402676483b47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7afdc0ccfdb3d03b4e976249c29767701db39219936d0b9efeb030c7bfebafbe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-836916c28c0b396d79dab27dbaee107b2279adf12667c8aa9774acfd0d9279c1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d25516213071485c1094195c285f7aa9295061f55180095100dc0393d39b29e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5045b2791a66a1fa515d0c372d8f7dabf5bdfb96c8b6921a6d01bf5a6afff560|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-163584ba07019f83dcca5d01a192c5bebd5167cc9f4a677e994d1cfd7be296b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f190ed3d718f25f55d69819e9e08db3ee46c8df195ad1a23d4d75a0d2e930fcd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-661f5a347eedfd0c2b6a25a8f34a3776a234dd05a48f1d8bf610674c13383d7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c9755c5e9925638da50c411e2faf928d8772edc654301f3d11eab3eb2db2445|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-78d528b8ec3100e9e745701db0be8577bc20d1dbe4bfb60527ecd4026320bc91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e3c597744ada61678201f0b4a532f3fd7401dd048e6f978c5fb73c71e0e0f45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a46c39681b6d8deed6ddad1960f988f78d87362f1ecc1c3bace776b1ca2b33a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e700bf4fdadfd9090a753d4d2656ca3d80e15afd3cf018b64dd3d2292306bf56|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-426a48a92036491eb4a2eeba8e372226bd3202aa485054f8d0fdacb1b0b13be5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0e4ab34c11e818a3ccce43b531e34ca9ac45d10c2fa19b504d2bd42ca3183ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a52c695ffb9ea85fb1c3401fc48e4584172e0200dd0c60b896e8e9785ef0e5b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-659a0d4e2f872de472ee3b333042ff0b4da69c4b19100efdface5ae2bed0c5b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cfc6d03fbebdb1949a62761acc6b5b425be7d529e89519676ddc63b684962f1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb76eda985dede6036bfa55a9a6118cda0a8d6f91d05fc1dea352c101fb07a3d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5eb5271dd2e56da8dd02ca49a156471c6d206199654e2d3fa5fd9932b184db4e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2baec443a3752692f929a0d279437fa339a26bd9d373eae7ca47c0873dc0608|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f6a2e23870523e5d292f1592994238fd5416a661153fb65d5ded3f19fb2ebafb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-481e9547237ce1f73420ccfdefe531147292b4f250f0453c133b0159849ed2bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d096e46a44fda6382f1a26c15dad4837c1994aa6c88212b34f3501050d199d70|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b4184d3fae05c6d2a91b71d718b760f28a7642611980c7462bac77212354653|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff162c8c8d9448576b418ef8bcc8d96ea2388c9f763ab77737a4c8afd0e8ead0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc1a57a86140545b1f3b636d2b3ad9385bc8336c25a6f0ceb3e6b530196fa529|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e2b187b71cb0651ed4811766c396335900c3a5b555f9aa0988d5d59ccd0f6a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6dc4d3e226f82d6ff684f854e13cb3e2cccd7e71c6626830c1e6c72bdb9ca0db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e9279fcfdcda68331c74f3625f3bda1c886ae4f18d22e69526f6c80ff79c3844|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-515442746fd65606d9ca5b8d5d8054d1dee888d4578850ff12f3ab2a25b822f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b8d46403c1f312760125d38885541fb579a97ca6440c22434787832846c6d630|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-529fd72795b5460a4de7f991d2bea8da893eceb64df0d212922358f14d9386c7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2167aeb02a4c11a6869ac522d7ac9f17157912c24462586cbf2405bd676abaa3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d4fd6e35ac77b2ece99a6e5f513bd76ee24b678f3587ba08b5faa82fe5e7358|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-acf1705a884d996022c07fa3599492568dbff213a079ce676877e1f29685c2b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b4b0d5fc435d304e61de80f4552379820a12be373742d6a9a181e8e00ffd9220|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1e43f7352589e406218faca00874887b16b82ca44c791897f7bbb10adec4ff1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff3e3c58089ffeef68921b076cb218b5b572a3e56b1768817d55cb22c752eb6a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61b89a8c5803482354e636375625998aafafd9f6bd40f151f31f5793c738946c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7569fc16fe541ee70d2664f0205f0bddddfa6ce821d54675b494212ef83b79e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3913f3dc068fa23c6804bc01b0d376102d5374aae7965a8e151edae85c5b4979|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-966aa4935fe51e2f1f4c79d3fac96809f5f7525c6d66014d3bcbdd5b905986e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d3d33dde64a9ecf0c15b4537c7ee402281623934e62bc091d5f2a488739491a8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-972bb17d4f761e1bbba32858b702b62786f73f7a5a77c1aeadf88de355a0e085|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6afda4b9c937b242f3784d0d7bced664587ab1b193969f8e3e6960446fd34263|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c9819ae7c1758cbe3ee0258f9e5945237e1285cb946a9964be0130639e1142a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6edf273030881d542cd7b740104c75163014a76d3087fcdf87790ba46b0324ae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ab1196252dcf91a8f45ceee171ea4c34c2eb41a51514772653b02bc8b09874aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0d9ee2063652c84db680507439d9e0823a2507e5f15ffbad26fd7c5b210aeb78|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83081bc3bc8c6e121a5b1d48e253e9ee128f60f8d664b11d7f47c03403558692|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-981f0e269efedb565ceb9905e2bed09b8094effd30f7330d2f024e75a86c94ed|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eb3128f38c6ac11f227f69376ddf2e30644c4b13eda50f35dd30a19d24a959ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2cc2c307e47e544100f0eb3a65612ae068e1809ad7f73ec829c1ca653b402ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dba1f6a8d2f6aee33ddf56c1134f2422d8f55877807a6f20f31475976a6259c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06fdbefffe422df5976d1fbc3111dc48e37d17117da2f9b1f960bc49a34635b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36aaa66e56227f5c44650fec542e81c206d9f2059797a0cef0676fb3b92c23e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-537732284dde2b0de3d5cfaa80bbfa352547179110e0daac00a3d3bfe63a998b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5b27ce6d5cca6fb3bf0d8fb2c5ef7cbcbb08d196462aed10ed635d1ac20ccef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f38c0a27cd96372b716102375389f96c57361004a09e581ceb1cbaae0c7e65c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59ff50599d3e8c4770c0e91995af7a8505da507db4f282d286942e8f186790dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-599b4e8e184f0425963bd89de95bdedc404c86e1522473fe8891cc29ce244e63|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-504527e8acdb3dcb4f4405501cc0f66d66daedebff25d5dbdae3e8f066fda0bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-685f97009bf27530001c29ab78dad592e8b93b82abf84395993c2544e324bd20|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59c0b2dcfc31d3c81835aa5b67682c42f8f030908dce3cbad4d12c8ace519649|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8a0d1b8dda1e39f0fa86d8b6ce404d04af45f9583f84cb47ac46375325eb0fc3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-54807baf6d25ca28028fe4fcfde794c34c3c504314c9a1e6ed1a01dd4e6bfb7f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-912b8e28f90bcfe2c2f13b65a8a5efbb41ad27e232e417ff5799b3a6342970b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe1e4e466e8d829150a9b0fd0fd114f73280b4c764cb18939cf40c2c2afd414a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9306fa5b389a04ed0bc74e226da3c47c5e3306de1865964be38ec36c94e0c4d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b529ef50a236283ae507d3ea4608e9e936c0c59faea920b3a9b5f6c7df85c134|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-66b7e32d828e7a1509f2e255c73929facd42b0703f557bd5c59ecba0cd1eca42|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-016f993f397ff90b68b8ad1f078fbe8ad0782ceca53bd4f8ab1d350e889d3bc7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1bc5459856d6d9daf0164ee3ca03050496a282e0b114ef1ee469d1f2a350dc90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f715f2406a3ce374dad316054b3b7e434262be038c0063ab1f9a9b31741d60a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e02eb10af6a4b05b14168e4739a54c8afe2e19b9cfbf4b07e23da6555f07ddd9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b46dd876011657d66cae8e90afee87c71cfd65023b18c1af40f05502e0b40c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d5d31490d4da6db6666d9d9e4599988ea352ff832847370997d194a0dde174f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-78b8f09985d8ef9275a16960b4a15a1c809209f34d611dcda13183a0ac77bbb5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-edc1c0a36de898e409ebc500930ea8f49e73e4a432bb3a3b5490db0cefd04c6d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d9dc4c7ceafada76d6bf253a71e63f4cb6a511295c1bf18aae20cf4d0bb7ea5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9dda09a09802425d598edbeab3d153c9c9fe1239a0062dea17740e797d0402a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-837190b970ae2f6fcaa1acebd213c7bdafc46e7e7b397d55557240317bf1ccbd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ef9fecb41516c61726fe7364a497c8b4e26898f4a7f4ec1de448bdf38b4e8985|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-644a35295e9b89b99921feac1f2866e0a471ab96a1dc2049f08d4f1379181978|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b3f057c34be70dfae32a07c6163bd10595daae2932bdf28bf2424f737cb22fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c5d3982c416a95e43465b615602ad73c0b393faaa07768f450791d70e2f3fe7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6715b6b2a30b22342d97b29d6196144cd9d85812949140bbae97be5de4471a82|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fdaad64af7b8e5079af5d5bb967e619be516c9919d5496c63201d0e2f1a77053|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4a6a41b212739a3f8c83423b80ea6cd0e6ea2aefefedadcd24b2481bbbc44a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a3aaf707077d11a9e5e36f693914951d32b34a46f1bca9cebef5b31bf1ba27c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ee9f1fae67239bf82f6b4c7bdeb6a6dd38f5543be1f818fd3d2ac9bd9227746|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb09c80d74110d91038cf7c66523007af66724b6ee2d0f59fe792fbaa76a4dc1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-216fd83714bd21583c7661bf95eed305e3d38efcf86dc2bd07515408fe9b5f1f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f38107cd926d833f5932eb648e71d348dbe42e232a8ab3d3311c132afb24817|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6ba6706eaad83f8d3648b01f6aa5e05e62acd9103d9372b1a36f2e56b69a6ab6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-387cccc1cb4af5f493ed75b623a613ce65acb8e4e458dcec7dcd07f734007498|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a4e997db2333613bb8792cd6de65af74fd6d4992626576ad6db328ee8b5d6b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84cce89c9eb522a981b1531f7b20694fe13041ae892cc126dcb89bcf7e882d3f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-367b7d25d982f8ca8963a6f7e140b73ab40844744e4210a8045ad8ad8d3e7329|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f45d0ce9f5c98b1b6dc7482376048ff24982a7338b85b369f6078f86b2b2c09|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b18b644c995e7d4f250bb5a53595f185d4ecf7083d10c30580a892214520763|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ce915b1bc41ebe75bdfc48bdfcbd758dbc9ad26fb68bc7a1955c191a4d0dffa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02bc7e70894f2130e5c9fffe22e157c158f983c8fb18ba77cceea7c94e91019c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3d8da507230611dea6195d635d41c411d1209197f59a890a07cd8746e3d1048|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-975fe324bda866d09feb26d593b41c115a5f4661fa6ab30535b6553c3151be5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc215e20ba9e86e85c03e297d51a8f224b99b84e0346d7df515d95b46ab8bf02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e73cebb207199f683ce35c7b4e117da90c052a9558a3b9cd029f6cf68052a694|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b3fea875f1b6af7c906214394992289f2254a0b4bfbbd25a845a2fc89bb3ce4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e824193f40c2007a17b6ef88f2ebbd9c5540cf07ffb15dc4b53139b4824bddd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d785012b0cb787300aa026e8e384b616bbfd6577c1b83965588ba363e291d5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ab727181e64c353248111ae885ff7d79eb8ae65e0761559c3486f14841e047d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73532e525f661734c62b3d8e79725a93a6b53b0eb804bf1b0d306359116de177|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f68955f77a74c87a3ca820b37f3833c5232134288e68ff8c2987c922d7a47195|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a2b3a842895d61dd470571260f4c8d0d9026a3d12dfdf611f255f24c03f49a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7501dae05001ed844b64d6d792cc81ab9a57bfcead4a1a55a2a7741f355b44a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c6fe5f2f354b9ac7e69fa52fd5cbc17585e64afd7a487da90473d5ef2f32d2c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69c091ea2a7dafc6a96c260acd81104f39de26268116c5f2279807f9041098ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-129509d053b9a11029a9c46b4ef5611a1deb02804a6d49c8001c2e05b7bebc93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e48a3be1c94263ab2a0c203ca39d5b7d7f1407e9ca229db98e7ccb8416736db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4758d10a7a97c641e824f8e893afc8e7824a2007c0aae5499e98d623cd9eaa52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b12889169656c1e0e9eda835b7ef3db2f7d7a745341212606582c4eb6f93902f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-546d4adda06d9402c0be78da954e353c762307bbffb6d432b11f2f4756069aaf|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-25b525d9b83e7abc5d2106b71b727e1e7bca8cf9c65ac936fdafb7b37f82a1ce|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b64db5ac94b52b88d79f3bdac61e8a1dd95134e79491a5c52f39308443f5d444|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc431c0cf78acb5260799301b7bdb2728ed532b29ca9b15f553550dbfbec347f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a14fea891936bbfa732a46d6eba7b2d82aa17e343fb82421939a8d22557fc8dd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9d915bf8fc0c2d902b9bb84e4ff4a65a90eee4d8c707a26ceb6f92dbc39567c3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-793192ca4a8a49012b7c9a0e06eb1026771ed9c48927a6a57eea0d9a8c2583b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f912e46ae21c1aacd97960d241665c9ecf2ef7028172099ada5953762b9bb3dc|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b23d65a381f8edb34e59e31d4c1970c048161ff7347d958453722b6238c64080|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-286dee263205da33bbcca40f4be2212082a72b3438639b8a1e1032b8f673b092|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-315360da30f1382f5497914d69696412a1936d0189b4bf12ccdc0a3052783c70|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e9bef6136fa3042682f8e585e1369cda76d5c8e80d50eb4cc1b845d21559db65|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-fab7a8a87b9e5a2012148d18abd8aea8d6cdb1315c5bb5ab5b33b4f14bae62e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a87586000a775c7b46949a2f1b6d30336c7d6091507329c29aaae45367ebb7bd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-4dc5277026ba6a8f4a47b5b558f11dbcb285ef40651c99635f278b6065ad0567|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-afd2f285e4ecf5ae0c6047d81cac009bb7413c861aed81021d52df77487fc30b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-728f4f9a565f8b3c66871971bcb23a9d890f1c7a67e039321fbbd1c09fe9d756|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6bfb445e4d6481a8e3ad2112732362acf6f8faf0a74207e3562e737015b966b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25b36eef89a20289e8f6fba3e8798d3c7f308dae8a02712d81e795bff95084be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c32a401e46c0507a4db1bde2eba0317d34ac6940eb816118f1bea5802547571|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d8aa3cd6e728f56b38521353f0a58778e2146a97225339e1ed0d125c40704934|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5bbf732801dc62e4dff68b1cb4041285b4d5527ee38b31de3ff485d4bb5890ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-825ad23f479820e050b82a136e775d430f1ac36f76e40eb70c0b46df03755fd2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b017628f744e5feecc11151b3bcc7fa0d2417df29b66948c97af5a1a6e1012f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f74ff2949828f039adbef1f41e62a6ff05460064483188486ea5fa8c7f77ee9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5c8d1a1f17b192acfc8e6e7c40b68e968ec087669bee909e4e5728f2dfa4e981|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3fc80fe56d69ad794ec653dc946d263d7c7dd401cdeccb5a17913d8686cc8c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b976597ab5c5842c2d685dd4571b11d0e2cc44a87ac8ed072f588fd15fc915a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e3d9908daabe401858dd448a79ea7e150d08a7b30742d10b89d5c343a02c103|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-10fad13b8dbe7a58072b963c2dbc85bf5f2a0aa25e5a430bf0633196437b430f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bea82aca1aa8a46994f62f1ca792879323634e1d592167ba9e6fd18d5360351b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-78daec37584fca4ae0ba9c969a88aa8959171d15a0befbef8ac0c6388aac6eec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d632dc2fb4de3eacecf355ca5c6446d3751a0d58a6c49adbb72dbaceb31238f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4974c97fb80800228c141cf09e3fe710865d1a752328dc143968573902283696|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fee975eed2c7097e3c432026059ca8c7f37f573a740ba30915e4c6d00fa32553|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-448350d7129790c382d19751397b9d978b48f305510c725d6b84b47d2dbac380|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0d6c3c53d77081ca91978b8f1d2b419bdb8beb840fbd486899988fea571352ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-19ced72e9f13e53403f2f5ac4e85fd4d6aa81046fc1c73153646e11d924a2d5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-56230e2a4dca8e4303d39ddc91f58d5647b899ae8c77535531c49355bed387cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-033aabacef832899017e6ec615b5cb8de3bc802a0fd2279f993aea9e51dc1070|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07dfe2d522a5628d30fec32b786f6b5bb349cf641e2a6b89191f0e3213f7d9f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6351b462b927bf73f4003525d32f129b60476d04f614f4cc8184bb6d7f08134b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-206d2c1c5e955dbd142cab492c27ab04310e639be7ad10309f586f47dbdab69e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-071d2f06deb999845d1c7d8af6820083a8095ba3804d6f7e421d7ceccfe5a36c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-46b58a32ea24ad3d7df52f8b8d650116dab9f13d4ec539702401a1885bb4bd01|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-986649d5cdf224abfecd07876ba41e55ac9e13ac8f387763a9e56dd8959d7895|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b95ebf78a463d1b1df60b5c6cd4e60c172d919a2ccbb8f8a9ca7585381d67fd7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c54db024df1a391dbd3bf442ca61294b73c0e0671f7dccfc4176dcc53895f0a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db372bf0452b7169eafee142cf3866e6ef35f995abcfef74cbe34d31eba79fa9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd030d79799c2bae0b3405d53c7161769f4485a0cffb53dd3aae720c4c50c8a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb5b2a7de7668ff55284cef8b6355c557efbd1ed53a3cdb2a8c0feed5fbdd067|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-35016d08b456cd1e9b3aab4102756747ec2bc0987d57a77444b9716720f13d52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a869c2d7aff40e7c157ab0fb5445f4f85e1c9eea5e2204e4c03dcc1d6ef9a61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3b7132dc87f0144c9062ff9caf4190a89d1276886d890d163a3439470090269|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79b1dff5c15d2857805e7dddb780b64c9791241e1373c57e30fe8022de8ad58f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-21be37a828bdd8cfae2fade733d64f3bef23d2654de42e3d09cf4e963b6c5854|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6eb02f5cbae72cca6d264c51f8a0ad34eeb755ea257e47630609844c50d6de1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ae562902c357416aa5bad557424095539634ceafce22a24c1bff6d58dac78f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-891041521da8665a148caa06ebc5b58b1889abff56dda1efb61e9ae30446798a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d317f865d836aa78f3977c3f2525aad738006a4eefe8371adc8a34aa1be2f320|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2b03c007946f6e5d5f0d45cfd71331b9119c7cb4d713da3e12da103490df8b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f22c0e22707622aee861bc81312b2e9746ae4211c8ef41c220a270973a5f1cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c8f49fb76711bf944e7f1d09cf8adc2e82382f02a642449a0a0278e5de70587d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b64aecc0cf01dab17b4f25ff96e9ac06e016180ddeb40592688bdab16d267d6f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4784dc47fc6377bc30ea1dc69686789de78f86aba7bb13dd84019a0f9fccfa99|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a51639bb6761df477b01708305932bc4e5a13196272b24e8e8b0f5a0fa28833|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-46bf33332208d01d9b080c8da3d4e94abf4ccc852b575e3881ff1bdda8c2e31e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1f15b2f509839fe4ad0d09d4ff42d43629b92ea71ebdbefb0118cc9b993e34b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64fb4b3f4cff23dd5e20f5d5b85a003e8292420b678a181caafd407e074b2d7f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6cf8305b4d594ba665c84af5e8b3997f0e6a9d1fc3cf8676c9676b0146849866|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29e926f04457844a0748752d89c54ce2044a6326d42ca3dd8ebe0f733134ce89|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49a396d655f140fcef1ebfbf1f72b3599c5a4e1db269c98a6652425d8365378f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1ad2c00582428b939a96b4b9a2074a5319337a9b8afd818158bafdb715c9713|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-23bb9131df70ddd0f3905d65d192bd74434002685cf94e797796f85209d81b2a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13f8d7a02e8211498dd3655fa3c91513abe38e2f9c43b806463157a2138fe786|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2076c8220524ebab92b1cfdb0dbbb0bb542e315ab613eaea423055222a428c29|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-521e0672564c7d8e2dcf33aa383501745d779f8c8e40e417550221b2f2db9249|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49f076279b58d105cb256f74338ad43fd5b1f145798b23008095ffa15e39e9d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-91390b08d55bce0793102797cff81c1c07aa79c9dbebde1ff1153bf84dc951e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7ba99e8401bc44317c2d24b4d7bd66c691a77712587a6995ba9022c310b8c9da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49ba6cae50daea6e1f60ba706f5598269594925a481215c6bacbbb92636a5f17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ef66c10200e7fb58692b5945b85a1711fa61bc63baf1a9b514bcf9acc5ace03|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b32efe1adbb26365c0342d5ece0a0ca1d98321f29d69d948a82815256b215fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c86d1ab463c139427b3f87c101b29a4a45fd153e509c199a4a00a7ee7be27d63|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d1964ec3b838571b8068f573d9294ffaea2ffb956299d8866ab6fe27ff099da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ab705477be10db01c6be4b43d082ff7b604f22793ff704805c66aff11b859d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1465519cc6b07e67ce9790de008caa85be0eb744a903552221651abaa5ad9321|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be99d6701f886dbd8c295fc39f6723bd94fdec7f94b210cd2fab301d316c0a19|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dab6b25a06d84cb39923d132f9c238f967b056fc013fdb6f329df9de04d40935|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d543d17a0aff6bb4cc34e6fddae622b46ad5137b000fb4545c9529289724a7c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fa00dbbc6dcd8f0d3cb521c6418d770d9c0e482d72b4614299ce6ab92a8bc6e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a850d1de35ab2377d12a58de1e584216e1effc1fb5e86eea7a56901a0e00715b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e73842d9f5fbb9049c1dfb512025377e3914c9cd96b8f948f355f9297b2339e6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-103b192c3c196e1ab06fbbc5dd77eb1ded4e448139e43778900c6e30df3129d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-734374ca95005eb11964900820b071427d36a8508df4c4959fc2fa602da7e857|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65b147a3105d0290b06450183d9c04ffd9d544d68978e171f47866909e2cd39a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5088a1bd1a3da931ca65e7f4bccd2ee5bf8f2a09f76cbac628f4ea47a261ad79|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5858e2498b35c5f290886949ba8db88691bad210ff9ac095ab9078acc3775887|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a42ad7d56198013d0353e2f1ad1a992e54c15d659280a9b052de3a606460cbb1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a71b2270d01b274f92fe3c1d89c4f8b0363e152bb5cb2dccb58d14f32b0a0306|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a0e38afa81d5d4ee3f2d00e12424545cbe63a1f1b9dd31fa941dd6f9ba7c9b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-541d94828d9cc8ed03bcf24191736dab28cf1479ba4c6b1ba028408b9a8bd2cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e91528ab68543ee003aef66fb5d0016a2809296ea54211e1fbe9bd5638a58c28|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb5b069f7bc2dde237dd847969433255e24cb2665d7bcc5bf8fdb621f574b23a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c720c5eec046fd0e7475b2b275a0d49b98bb4d2fa66c0649f810094d2d061b26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9af2ed149a9b1e2993858a990f33e429a07e076eacb54940a8860a181f29a1a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8966716f7d20cb42e7d3fecc20f67017fd604676b947d1bd9c80ad578e1fe6c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5f1a16d4ff18a5b813ed683ce791ab9731646dc5c378ebf29092f164f832927|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-354728a189894b1c2e54188021e9c2ff681e7daac7f1aaef834476a466dabbd2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eea50f63a4c38403c8a7bed4f0e34f44bba258e107e0a3ed91fcdb1bf1ec066d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c0b283813c210cf297836c48bfd3ae5fd82b7c134edf331b1de419916e063d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9429c045837ef118da1d6a3d384db2877d03f2a03257ba64daf36ae57e437265|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4fdec88d37d4cc373c681957898bd92142db35cfa86b992ea55eff7bab947307|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12cc4c7857d09e7f5e5112f9348797ddcfe20400f33361cd5f02a07af90c7672|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ef2a9744d3b4bb23a93c9bb20ac4dc31305c6565074295e0e4118c2f6db4db6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ba875b96b5c7e7d9805021b70861a869a23ab7b8cd12be890939244a184c414d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-43ac2b6e8680e7143e7894a1be290e33feeda546a6ae86ce15280cbaa83e3876|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e953edf651c5584bde5c5afd17e05d97bc3171b21567c18b21726196c9066d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7dbe0f1c63548a8d044bd22ca135ec2965de0ab8ba66f23844620e20b2abc6e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41b85a4795902459af153f7de53475ae62ec86b8d9fc813539ea7934c7dcc190|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b27f70cd8f85cf920e7ef70eff382e17a5d9dbd217527022a3aae59e6ad3ad13|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18d7d793b20293a2ee8374bc76518e73848a5327bb18f54f342e48b63dc0d41c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de16aa37c956f83600b6bb97a6e8c0ae6bf6a515160ec62b27489f53efdefff9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d12863524153007a642b338d0a57b6959bced729b0ca28596409b99699ec157|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d94b7d908a42ae1cbbb6c2751551445d2266b3b0db11e4c79951a9f065f5f241|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b21eb0f40c39e0672baffded73899f49c36b88ece41c66c4086414dae86e61ae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-576a744a4389c1554c463d4886e1e322a8c53fa4350bf33d51aae1e366f1d15e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-214a295d57f32f78fd51e465791a8127f58237ac889d867e4092f82a3a6f6810|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af1466928adc496ed05ad2eb10fcb53c285067c4e97ba665d8b4d6781257697a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbbfe961302e4a1805d6b56e16c995ea7ed1e76c76da68134562accfd8e1db14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e803b7fab92325e0f93171dc6ba53b0cfa738c253e1bcf02dceb0fa09d2c300c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-394b56c2a930f3482518c721d0f25b1314db42f6416c9f497f0b3f4d9a067e6c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e57ae14895702225fbd1131994de2c548309a865f671c7143c30238c3903ff3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3e00c378b0a7376ffca84da790ec2e1c50263799cd3ea82197a7da12b43fa303|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1f953801f1d16c6fbb3f7c288d5effc3dab70fa4597de05442da7bfb23c1014|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f3bb6d293fb1c7fe3eacebde45f6c5d4e2b7b414dff3d9704f00105b5eec36dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1c0a245bfcf0d218dd42bd5c36c9b53fd5c72d1ecfcf32b81a4725810e49024|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34c3d505aeef968b091f9b9c26538df06e92027a274d8059ed2e3b103e4ca652|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c8cd61667573f8c957c2cacc8661ad36c2f5687a0efe9563dfcafd64e18f8ebf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b47b96aefc02d11303098db75dc2e3eb15db7215f19578d8f4222144f3735ede|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f25d8f7596de8c484d7c75dfa7e3c64da7552657b4e64089f4411f2596caaaa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bf63b199a58bf3412469fdd30c68be0973445c536e72b4b3746c53264c0695e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-453e96d0a3f05211c66e2a430a25d8c72123c397ef439665eab7dd6b23d6099f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ba83d85ef575cd24896fbc820ada53e8048fe71be3c841a7df9ced130898274f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-012708851e1bdcb87f28d5c5fbeb042e01732030a6bca61295205fcd754ebb7d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-527472ba42f965e809a509c7756befa00348731dd2dc234ce69bfd884c9e2dc5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6763fa903e151bb002c40b31259a4c87ab9d1d7df4aace7ad00f5439ec67a61b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76b46f9d38591e4e0d392aeed98d19c317f5965c2d3339570d0d55d0f8eefa67|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-881b43edf59e66b037850eb56136a8adc367b14a92a626d562e5862a9c4d4279|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37af9303efc2ede1fac5de568b146ce0b1630b30e436afe82c65552afeeccde2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-513e5ed16f0c9093aeb27b42961ff6273c1e91cc8e17ada2fe4a701796894370|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e371f3d0a29adb5b4c303b28960bb6be80ef43d7a4a3ea615d6a377c96fd2c17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bba0b3d6cd332eca8d775869c6ea8c7d7d87d1a21b13f8d47e5479b1e3f27932|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40bc7de9fb9841ab7cfb49758f8c027de81609709ff478f106a76986a27c791e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6f22438c4d4c804a09cb0e545f87b29535d0af67bbe7e2503136b116f038a17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d2f96371e8e2a1ca4b69fdb9f792c237eb1857a8996765b86cd71e4c6949b7c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bbc17ce825d6024d3d1a21a0320ed199fc3c37d1fcbd55479d2954a4e6ec350a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f520f35ae047b43d094e4bd4c0cd345e0dc10a2c491cdee60744e6bdc8912288|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b62bba6223876239a0a879bedfc6a8bbda73ed2a452d624808b605bd2d1e3e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-598defe87cac715c1968a7fcdc15f0832301319b5ccedb73dc2000f87e425e60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-63b04f04f87981cb0daad00ef0aa15082ec2abb70cf4431cf57710c68eafeb2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c81e2fd0bea05b6327482a73deeba19039c58ad7239571fdbcf31796e1b80ebf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af72ce6f6505da6de1fa17c3c81316c27e0ce1583bb620a0d09f8a2bdac9e4fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ebd737b5a01cb1f2da03796bac2b12840a054946083070e29cf3e04297759a34|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76d4e4905a8f3ba6b78bfc360cf1e9e64b2d0f66db7726f17d5e522a5c2495d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc8bec4937637aeb50ee63675596e8614b1b90982cec89902f3f3ed2aca1ac27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15abb7d88bcd03c988b1ed79e95364c5bf97d0e21948fa8c10aa6f22a696ee9e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a5ad80c20b15948cc5e7e6fd25bb31a0540a62f4792e77db094201775213969|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa957530c5e69605abee0bc4c9377645abf13d69ac79ea781cd36e1102434b40|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07b12e5657dd24e30be23cd89afc2a302046ba71b5cd4954424db1457ef4abd4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bcadcc9015974cb7936078200763b9d4b901073f48c25677362aef0bbc87d8d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f964470378a6d946a0d661d857e4cafc1b45bae1a5107177702429c7b1a2962|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bda9ef6b273c4969472ee57214cc8716e2f0ddbee13733654d694a32210b9acc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9188cfb2f412dd34c5879646f5b24e3b13d2f4e8fe85a3ef6a2cfbcd5de5ab9d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6385b8cf76dadf7243c1e33c3164e623c9c83efd3e7194d04367e13132192672|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f26700c1b2ae99a91b8eb09ad73bd28a30a11a9cfad4857d6764c3709a1d5995|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70776ae0140357861cb54385051c35201c99731c8674c2750c86440164bfda07|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8dc4af494d4abc4f6d9b39813c24d638602193a8bc8a50efc4b4d6a7cf815ed8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e1eac8b9957c0f2e280b8e34bc3a4814250caec6cf64abf846296caa4d21ee22|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e5878958d25092c4b532768ae43daa4cf92e15df9ec3ab1daf702101dcd9b7d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ef45cb2e7ca2ea40c106068866345ac25849728034751af9257d94d9a93dfea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2db7534f1180585763b0a1307c142917367b0b24dc8972eee3d1cbef8d16b514|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f8377cdcc42c45d402c9abbef60782a55c17519914c26eb986cf90ad54aa9d04|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-71642e90e78289d870102b6368beba8e7d10b31033c1b735853a2081d92375ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52b908707d0e326e031060e6110f8e32529c4aa9c25617d07e2edb2b9fed421c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-161b218f2c092b91b96450930e025577675e53b67443be024c08575a25436262|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1bea7e500ad23975720b5b4657d1aa16c0cc9d36f4854a8b004bc11ea66d3ffa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-925147734837fda7c79c967e3cb69e524fadc3a74cd78b2affb6631f4511218d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-515973bd874e10b7c18ffdd62a9a3e1453afa9e39f8bcf7e847a7cfd7d254b5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7836041c2203284f5430ef995516f36ecf3bf5db18d1b21a64911cd01b2e90bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a0fd0e2d700b2fdfa92c31954b9884c7956c3c7823b55bfc122d9669fa237094|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49d5c093e0cfba017f6c0f3a97dfef3727d6e24b3b445cdf948d6dae97938bfc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dc632ba0e08dce900fb770186a75376df0e08ff4064e6b331b4251a5f22c319|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2815fc4a9d8a7b61991b7f6305a255e6060c28b4e285ce398569bbb150b9506|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f282e50b3c2f6a9f39cf75199b7b8a6468c19bea7b8b82e5fcea4f5cf404a71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50d6019b7f831e1a039cda5e1cf21a1f4656818a61d418a7be4e6881ca860412|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ea7cb3fe9edd564a14927f0cee7f259abb66caebfe6463a774a9007b7a4f7d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7da359359ae933101985b06e9d7c8d5d469e70ce0ea823c256a00f5541b64f08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff6f826b80803f43705fe84b9dcd637cbc78502a30b77f313321a32cba9a6c08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a8d72883d71205264f8f4e0d351a965e5aade9c2889b0641a638c19954b5447|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff836a6d34e2697f072daf27d126ffdaf3d4ecf9ea7549809805217d3131478e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed20b2fd770891fcea8760443dd153875e8589aacd9cc64ac192c28280388bd0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-156fb3925726493cb26980d0a1361ef2809d59ee2a7bc5e27e76b8b0931c711c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ea049e4e93bb32fc9e4285ff44f2a2ef2f78bff1cfdcfe6e419b0f96e022853|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15b6d10d802b98e7deab265d04f03825408adb0bf216d628d9d21c55002f3fe3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c98c4dc769b6eaf403f5554d930850977dd4115303047c8553c6e16cb6f4f738|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f5065b5b127557c23b77884f8add47b2fa537e9407dc7af1e01568e73194140|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-99cad19d807bf25cf51932be72a2233db76ebebb94aa3d9dc75df4e2314e7479|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-211e1be0405f13e52a42b9ccafcaa58842c4b35c4d1041c93c847730529e4876|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0fc40005d18f451755aa0222c4df6847b4098d947834508b9d77340b90abc18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd65efc8f156bb7a00aedce6ff9b8f7cda4008be4e31d92f338b51d8ecb798b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5dc328138b0acce273f99452d15f0d5395f64a28cf03f4e29a7539622641d099|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af8f9f7bfe0353288217724b776c0e8d5e7461925e217e9178fbac8074c0a0f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b6fe87416294b3493717d1d0956bf079fd722b470a861a2509d3778d95a17846|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a38c2effbf1b67270ffa6c9ff22cb30c97a787e4592dd3126acde30afdf4f86|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50c057a688ea3182143c8b7ca48a840cbd934d2a848bdd491ecf51b441b5d544|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9040dcfeb94a5151cd1285faadfc10df3ccfe39ab5cfb27cf55232e8fb63944f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64cfa5bd0559b530c25829b99179bc5a39373dd0f418797deecbe1d023c6adb3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b16994cc95b93b1f5e347dda9039de3e3f7f690a08b358ffb889780efb41c2dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f8d66cdb5eadc6fa118b0ee899580d25dffc334917fd3248193527c6cbd5ae37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-88e8e9c50d82db173bbc1fdc4bb203ef8c1cda4fea6f5efc16ce8f2dfccee4c8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3cae43061b87e4fb2a614e2dcce1903205e07be528f3ed7e867a9f531078375|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bf6306b5d4c2031390249b8960a4e8769fbf14802fc297cd19224a72f56251c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-157ff8b56ac8334fd6f53fcd53502dd4e76fbe15df4919115305aa889c0cf586|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec783c43f9fc91a6f4d623c5737bf2ac1224c715c4331dc9a0ff0549bde7eb94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3248b5b0d7aea2ba557ee66f686958ebe9a07f5b9667356091ce655f962dcad5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55b8f938dc36a29981ea1d93d3a368c7832871daff7a262bc30c42188f6712e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4bf9733b19822e29e6fc43b1c8088dbc9617265995d67c655413ce6ead8d4fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e738b9c6fb3f58bd9bb5503e94a3021c2435ac50e4cd7ca5f68181ff2c8f2307|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d00cc0ce70ce2212557b880efdc905f164ce2ee0b0fd53d6af033d330afedd1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b77ca558253e10dca3b408c13c15e4d274750f6f48c1053e7c5094fbe184405d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f2eb4c1817f7d8b6e1f85b5a239b449cd3f713b55da7101554d92f232c21b9e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f16d2b5207f9ae7cd95363d35a495c20edd593636e97136a2610a5a247dbda4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f12e66d02e0a6e852bc1213ce6cd9fa135a35306cd08a6a4963e7786cba3ca3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ff58d6285fed98c3f13c44019cebde11dd87fdacddba94229bbd898a07e2d6f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3d5c01b156cca5c98fcdf6b355c16b5eef86ceab2eedf18cab19906a7be0e198|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c559d48236a48cf04063211b30e43d6cae6bcf52bdd2c5298725ef73f206a047|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf558ef45dee45ffd71718e28caa54080b182ce88086c058f82b0fec1f849e84|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aca30c61fac3de08366778bc64b1d2d033431354159c868cc5ace7213b561b8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61529301c335608807d260b3ed6f5d2791d081b7a75ba8709dd33fc462ea4cdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8eb7ae240d6252bbd0a04f4d5086e3052da8c00adb24481d69a1ce3ed6b9744b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-efdf6d1df88e7a00282b6cffcc9542fd4a15ab0aa0612e31268e4d2f5dabc2f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d874a79d10dbb51cc915eb9c62e9f18762df101c2c6c5aa67ea74a0c70487dbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c80e3962387a1cc8a1436e808bd10cb5508a20d6882d599682992eacaaa4e10|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c000191b6e6cdf5c3e8b1eff709f846f647ced7502cb3ee7eb5332f8a2256c69|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a126984053204786e2a52f9353c6bab049f44512671e168c94f34b74a9f39ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f6be236bde3e565f724076263f177d1af1a064c4265a100cb3de74c5f61eef33|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce25bc3e2a98a72760efca5b5307ca1c73adbbb2c45575fe84fe8bbfec2037c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ffa4bf29b20488da31efd630f75a2b90654f31f65c7b85879e2c5e9025be26a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3986c59e7ad8ad8a9c3850b2f11c6e5e5dcdcfeb2150c1de64364d491e090522|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69ad0835a2b477ff764f7192c87f527c24cd1590b31d6e236438470b7079e623|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9b3e26f5fabcabb05f42205cb3605c212f246d243fc4c6dfe22e1dc67592edd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-289a77d85f3f4faef1e9f0bd475a2de1c014a42a8433ef3a87b6883291cb2aef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0f6e1f0368ffaa66947eb65940ccc633810de04cebc614f5ca8e14754c8a925|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-384dee07363ddc1641173d87ffdb2e6ca578d68f0b844b7b91fb4dbf3e04c5ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7fe2e52818031295655ceba855e5f572655a8707e2c6e981a268619b037e5ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dcb90760df7247b1a2627eb7636d90fb3f50fc1a4cedf85d75cac824b246e194|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b219082dcb1e68d5b69e65843a55620b08287bcd9d9da06a1b378fd02cfbb156|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30924b698587e437e4c94621e5e60832063bd8f507a82e3efe037970dfccd47e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6478f83eb4228f64873193d802c0d5c731897af8cdda7d97c3aebb440f4d649d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c241be0ecc15bb704f3d2afa7fef3100ef87f814d8da7701925f2c1b4476685|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-adf7106f5631151a899df8cbc241563b8ff27efa132cc3cb2b7a22e5176cca9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e886859f6b6031099b6c060b37e31ebb50ce49e0fed2987c8b98022bf04eb59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8cf250e3b878069a5629ac479c07c2cd7cbbe01cd647514a56712d3cb8fd9a81|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e386d25c7ea9c4250b375c6ca31de40f79b8f4bcc1f497474ddb5870868dd45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-72f47df72e9135d8a09126434d1d6b9c940e3e38da79327c6fbdb8dc1cf25199|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9afd35e40d53efe451e7d8c2731eec91d626099a7a90bf75c4ddf96f90c7df3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9d48feda0188eb66407d6487dab8738c5283210629509c82e294c07cdbafe2a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-442cfbdc79365ee176e4c7b6e965e940a7f484bf080c29ec6529d2465b4ed135|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8423e24c84568bc0b436c8f69a8e138bfac7df8ab20e802e6863063f4d6c584|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64663f29b816677e54e6c3b46d954b55d7ebe632172094715233f55239dc79ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4490f45cc6f23a9f4dcbb2ab7534f2481157677843ba7c8b7bbc4ca6d771ee8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bda31d694649f036feef6f26c8987e3141a044fdfa80efe59db0ccf856480399|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fadd226608c61915e13198ddd61ea38f60aee6af7b20deb161ff52fee8b5f50d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1c22006182af893eebb121e41d738131d7114bd456dc718c0d2e440d60a90fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c80cd56c7b7966d30ed96a3ac4dd7a86c419d08aa4561aeb7dc54b4e2e6134d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d2f511d8e60f592c184756f1f3c6c81df533799f2fa2a3d612c065ecd7c5c967|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40ed6f39e5aa327d350f1bdb3c8d92ac19d72113c83a255b6aefcfe1e9eb2349|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0882a4a1757b3e9b3a082a9f55cfbe11d4226b0e87078a278bf4128671bd30a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-563dc2e9de687c0d5e2c690e0bcdd9d51f221df328395a95d23e3bb34a3605cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60158e1ad186c91372dc5770af47bc5697a3e11222dd173ccef4ce7b47e6415b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f12f8c37d5cceb95782f7b482034ba3a109b8d0ab96207cfe50036fd252a7a35|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b889802830427eb8ed5b1fdd63ade9141a9cf990b4c20d048a30f265a273518d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70515ee1fe9dc7038e03afc180c95ae5a47b26bd701487fabf324c64f9176a4b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-63326c32c98c2d3956a89a1a82309d88cce8a4287801a2ddf52117bd94cf4911|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-182d01b4b8dcedc1bd6d3a7598ee37ede8051f77f3e199d6fce3761a28398151|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a68d9cd69bf8d31bf81a86a9577b331a8890a405d210484e2a5142ee25771aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-acd5f98627d40b2bfee10ce028a74376ae742278a64756b30b4ba57995068bbc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d539d1fa2dfaeb568588873fcdabc44ecc4016b602f6c706f79b283733c8bb1d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b0011aeb705f759e6c628eb5dc52a39e28f98cb70482c9f38bafca4725ff5a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ee130f3f9340a2cdf8575183d16518c8e8c1d069f9a815b8a49fcb73fc06a43|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c06dfeed5b8f02e54f297efb4b6b49f0c3dd2931ee4531e31e3b8ebf9b246a42|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3345f9616196a266d0c144d5b0472765d548f7b5d182b3b15a51ac4dd21c36d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ef741d85101ed480143830def828a21314147ad471a9a27177e52ee4cbbc8e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b88ce2cf5c9607b28a1bb582f02c247fdecebc0b7e6bcfbcef136984da917b2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93c1e40dbadaeab486d696d47457345403236827e8bae9b0d0bdf5dac086de57|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fbdc4478fd2b096598614658ca6169a47703494e4590219ffdce15b35e0de931|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-731c90d983955eec34c3275c8c17c924ff1a646bbdf2c2c55c41d8bdfa22cda2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e6d34e50e9f29f5232c8451bd7ade93abf3b2a361889d1b1e75f1ada3cc7818|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-567cfefdbd3ae593c35c9617b90cbc21a5af32764797e854d2fae7a9c1339ad9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad388a2062c1675f8daf73ed1b376fbc195cfef186fa9cf76513c62796c7d175|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-744c6888910420b4f140f4bbef2d2ab764a29dbcde63ef241577c9959f014d62|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-302cc57c3d394b7ca507c17cfed7995227a16eaef63bdcef37cdbbe0eb23097c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e251c0071ab8073071d15e6e70dfb8269ec7d2ad6b2fb4888f3366b873dc3b1d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7906f81ab2a188e0525a966a0c8261f611889fc4f93b1b666414dd66bdee8696|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd4f46e97acad7b64193682552665dae7d4fab940a1e4794b09695a9a503c56d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-043fd2037fcec86f6b2eda0a3013746178131e0490d487a4fbb8d55ad5de8a46|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-068e7a127cc120d698411bf8ef435b4d990264c12f0238ed20d0b1f852348434|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d8edce1bdf68f7722c19ce8453adc7296c86e2f8281577319074d8f85b9f851c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db6c662338c07f43fa9662dd35f7ecf35619e8c26d6fdd7f10c09f091ad3a641|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84fb88f2214c08529d4460a64c048d165a88a1620fa08cb7b2b791b7477354cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0780a671801fbcfd5d4511e6695c14ed239ca8fcb9c2c083d2c14ff8bccf5292|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c02d17d3bb611a6fab4bd71cc74b395aa0dfc992a406169ebd672fce43c9a25b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8394a2600bc80a003c5400562e174ca8775cc16ae727743b8a3717ed1e7c4fe2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3895786c73b19e18e2aeec1d395420b5485502641c02e38e455e0e704c98dfbb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2e9bb436ceec48fbd88ece22d6325930fe01803d8ff375fe718e48c81050709c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db5c8f96aa69b1b045c15b2e1257bc36eb882d0838ad0d1ad926d9f62b49bae5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36af38fd8062c9a659e7b7b21ade0a0dd27b62365223339a54d0f93d6308bc77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e13dcac2e36acd678c7c6c66c6d925282585150142a62571e4a2bf8d1369e312|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e1589fefddf0da627f918d174b63a6688e5c7bb43066f4be4d39d9e64c4ff0c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59e2f3f1ed9f920bd7de1cb7f3dacae8a1f71f0212e35d09a4e3267b3b1edb74|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-571b6d23fa15a0a64f3ce01043d08641798f354ed9843ff4fad7e7ac78bca9ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2289e7810d6a8b618c3a2d3e30539728a789a2f01437e52ee3deb0d65334e3a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d7d903f03cadfcd7e6afdaaa9a009b116210c6e2d2d36ab6fee25f13dcbdc5c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-454ad05c9b3dd220b0896ab62cde9832ba2018d2df6cf4824426ef90335392a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69268f87998a89437cc0d49dd56b1ae7037da33669ba5b4e9269544393833e60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c66cb3db34a50fec63dea00081d8a0648f87ce827c036a1b860a8db1ff6e68cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d568b54221e9c1c7c1c7677c5f3143f318cbd735b3ae7686de3bebfb4c0a256|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69c16bfd78ff7a73ed94a9ebfab4c6fad9d9fb63a746641e2ede6dd21d33dba2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ddb560cc03e6cbaa97479f50a6974e2d339bf008095c78a0ad1a1fdee4cb29f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f8a70e891ddec5fada81e1d7bb2a27b2ab846a0587f0f0ca764030df771e1ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49516793348780e4db2594753e1ac8ce83f1e47e30658ca358e69f02e7a6ed66|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b58f5c9138e08d2ed2937081b6c89d5a7a7c12700e481264e0fd77e99b73235|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c492940d41cbcc491b81c0fd9058758fdcdb5c66e2f478c72305bdd6279ba4d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69883f960f7b6e2c658919f4a3388dc06edeca06220b36d61002c51a21b57ad7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b7183fa300b8d12eb282b05fe4f5ff5818029205ac1338c9a02eb93865d8ab3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80a772e756d8bdd24744d9b3a65bf989ea33d7645305671b24fd5a5f435d9c8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb53042b5d4370d4b9bb6f201476c8ab4568625d0c4b68633de12a22b55b3d91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c262285d2c639e841897b97cffdb23dc764032cbe554d47c508a936cedc68a5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4c1e25456a50d61a85d188d01b4f2da7a2baf5cb303c2a8f409165977051e9d2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1c89478630e251245483839928b93ab85caac5839f3148f04fdd6b4592c81a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b70a39031d4236ee915d44f3e9f70d2d5a9886cb6721dde3d6aeb2510f0d20a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d35a5b58c152960e7163176353aa5b139d2dd3bee5984236960ba81fd994e61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c91ebcae76a5555620c91735ab4f1c265a53fb2c02600b8efc214428394cfd17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1dc4acdae13273aeb4bfdfcfd1aaac76e8806c14cb2a16872262e7e0fdd546b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-542dc76d8b64238104890dff52e124f1535887685a8e01e7f86907763ce54f5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9fab3bd899f402f52e0d17e1cc19c72afca779fc82f7e00086f39ea015919a5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7290e31bc420a82c8e1464ffb251c3328322b9346c492e498a3eb056a35f5a6f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57bc84692c7114611a53625666c630232684761a142aba70a53529adbb0c2c5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb9c950d0a8d647fe846947851c4a1135fa4b10d218cfc1a786759677764445b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7de4d11bc135a29b217cb6651d171307c1acf06bb1667cdb5769124d6f2fa68d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-795b06deb2139ee1bddf4e069c7dcbec78cdffddc9d1a63909d667193d5639e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7db1836b5e350d34e0386cd4e140cf8ded5f02ad5b2234927c588be24fd4f35c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f694794ca86a82087d31956d0584182488d38ddabc9914d55497f3e86ac40744|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-209a3a00c1eca9a2f096f2d8931c031967e2cdbbd4f3335ab354b02aea681f3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4e1e8aedc785e9417f7d3af565376b938f64e4f171fed52f0b6fd96678d6923|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-28f180878a3052fc3e8037be87cc29802d4a0361eab4cc14ce9d8ca8eb09744d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1c4195129e08734b0aaced0b5304b669631fb6c06b1a9074bbe216cba11f51b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed01ae9ff325dc30273c3827bd62f6aae7e142029368d23c1e4e2d761c46dd5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40789777114a16f85f3a2e83413e9158eeffca6231cc6132fed909d6daed2eeb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4528f205415f0824684a52e99ac8da949e830399a30acfe75ebb64830f868555|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-54b5245c18e2a7139ebfb2922b6d806907fb15b5cfc23d0c64f511800a8ca19b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1ff784cd44800eac3611396e25d2c08ea757e847a1c0dac15a36057a7bfb26a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d01f333284cca9aac0cce33d503c99e36ed61d67fc4a4dd98668e50483dbb96|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca19baf159bd6b94d367573b5043e3d5147618e46d3ecbe4f99329ec944a05c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b4f6267388261bdea9656e7719de5bc0c2f3ae0f5fc5d9b60050318dfe0d5d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39d6783f8d2d7aeecbdca0791cddc4814519e623e0dfe305f1c479de77e22d01|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3577e53c54ff47d51cd71c7a67e2d024b53120e25692291bc5243d14fc5db846|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a46077c260966bc71a6efc70d565f52a41bb7ab6df43aed2dbf965d55f73afeb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a98235e23882602cca898f443beb6cd9b1629619718257e240e26ae728e8809|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-505597dbe7135549bdecd84899c207e45c72bda36bdd973cb6f434997ac2109e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34c4aa63e6ba1364076a638ad16e384a78b9c007a4c3f1cfa3ae09b2618b69a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3020a9fd2975c3b6210bc994a50e46433b6929f9380f407dcca276f25169539|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ba756e91dea33bb7807292a35cb31777e43c1b7db020649e65d587dfdee9a671|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fffe0b7af94a4b8e6dcd68ce23283952cefe2f8d961ab5046e425a05d9c3183|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-08555349a4173cffb85a40c14f05229d9177213cbfe754f567c8536add1db4f5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f7e71b5a53f7e3e3855b3cbfe295b71f9bad5d507d6165627feb3144c421fc74|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4fec48e303b5735616538505e4aaa2a81a0b786ad8a0af2cbeeed583a692c13|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5a163e93df0cfed45c3e9d58e256444ed8dc318b2c200c8032497241e706b23|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e9ae6648bccba6dae799b47a36eccddb42eaf3f86babbc823279bf71017a7f9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b70ff882300b55ca94f93d56a7292fbe707f5a08394749a14d726b6394a91844|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4852a1361c5ba47f01fb12a8779baa266e1b194301410131e9eaefa7bcf04d2d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aa550e332c739e9055170e303f102fd754835178db91f593ee8cb6ac5f73a104|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb241b0172eba116862d539fbe3611cc062462d0f452935b4d9dded1f9ecb10e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e2cf2211effdde61ff43ddff7a05c72d42e700fa0194ddb1b02ea090baaaeeb4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13832913a34983370a411186bbdffdf6cf19627d130a11dc74df183c39c92f8f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e21fa0555ce47649e66faa8a51ca9072402281b6fa06cc47fd64d8eb31bc666f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d159fbf4dbad9e6292b3b131d941d98e179a41c646bbb38013d3a9fa9aca9ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ab052067bc7c772ca8b08ee310ebfbc0657c0bdbf7f19ee92a73fe19a3eed71d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b328bccf751b9578cb0156d2da13f3fc60dc07ea15658cf23ffb26c178effda|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-949c3642205e9cb25cd5578434352eb7f34fd9c0fb870f5e6c89b1c950bd0cd0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-003594a91d98065a6178571b63d983dc5f465b07966d5242e9d8cf5e1d440660|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1afe1dd7faa646fbdba5b321c318cdc46fa48a233e813850d561ee311c335ab9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b93247a2a55c6dd8b5fcd8e2f5d1a52940554957f906a3b19361139eabc4efbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d2a69b942f4df589380d81de16e21b2ed4b35de3c266cfece2cd5fcd9f6c8710|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-23d3b2ab7b5eed065cacaaf7f6236fbb21b5d2301f68490f1bf0775c231462ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25f943426f35988f171a35e4d7466f08a78c9547bd6fcc77047a8de2b3402334|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd4bd6afec0417ec299502713f1d19db02adf2afe45f7eb9ea8188ef0f69024d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b971a0d32a05eaec668c994aa1289f29436c8cd5ba3a01fa90620c52f2a622b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-23c06a54834839e88450b4b7d2a171083d4833c32ca07c5b64ad238547de408f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec5b5089aa093e1e5e27e9cc187b5622d8085c180f710ca92736adffc4b4d45f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c241ef1dd2bcc5dc0c38c5483a1f2b5d4e39518a8ed973f6ebf3137534a046f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-870d4c73efa5e274c48127152c5e9005149df5f0e0155587fb20451e5a1da8c2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-018c285915d364903ff946d17516d405f7efd41ea46c6fec5a5a338e911107eb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-314e09129a742ede7c727d27c35abc1c1f25ad9bb16c284d85d4fefc13de9230|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0928381e862af7074a72311e527d2898c90e687901e8703ca8b5aa8d9bf25661|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0929565f1472b4d201a0b3877ca96b0dc83d56cdc70635edf0631f8dbbdffaf3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31a6b87bd72366f97f14f263a6bb4ca31c69e0d8104ecf15bb8038a818f6eee6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be9c42d40607f7084948669add899bc434eb59725555e141de6ccacf838ed022|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9b4130c7ac22bf29122a5167575ae8c2ac26349faf133e9728ae05f5d1c19b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-045906b4aac2fb8f69bf6e91360255da712ec9e413f3d1e7e6405338c35bf02a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-595044b0b6e46e4f2cf5452f71fe31fd2a245fc44dcb1215601e10b89f8137fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-689ac3504a6f4422c1f391bf9d65547a96ba73dbc9df1ccf1cab7ef35554424c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-01594af26cd383e135b6d286b759521d6e2dce598f72ee20766cee237b7c55fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d79457ed04cfef80a9ddb754ceac01856b284b61df43f0f5406b6740ad99979|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ba5dd91125741a66990f3713f83a123af210e322e2d79629144be9eb9f50cf7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79429f3df1ebd8fa754e5501649ac6d14ca47d3e436dd4f1448f7855bbd5238b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f02972f4bb01a837a3f2de87fe80551dbc13b2c6e0f263704747706b097b4d6b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0afb5a90c803ffb277a4ee139280d00ee6f95bc8d83a5e494d28b4096f83b7e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1da4ee6829336c744b775d4988c293893c91fc927dafdbfa35a7130baaa269a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a523828906c6d8e6460abd47441e3be735ac8d2a9488a2a612fa999e60cbc14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-342f317900db02f55350851ff8635156510487bd8cc462847a6c9fdd49758a0e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-164cc607ff16974bbadc135b7871986a7acdd34e7cbdffbf043f0e133919fdb6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18bd878bf276fd24545433254120b6ec04303663e6163104dc39927636ea5aeb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4908360fc00abe7e26ec5e611d025597aadafad37dcfcd4a197d9af033de689b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9bf5f21709861fcf9d092773d984c6e1bfe3a1115aa44f228855125751aa10a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a31b7e92681784430e5057ca38e2e0421a11f7c652dda6f4760476f1f0e8402|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f4c9732f8c3d22d4bc1d16bb3dd759181fe74eda184f328e6b04783c1cfee77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fece6a6eee83e648f00a4fe6943c622996216de4fde4aa4a5d616048dd31640|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d2f87d6334489bcfbfff3c78ec460fc2e1c314bd8d0aaf5f2012bd31fb8ff4c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea551d88cba0ddd0caecef57697367b85c2863d35b254203dfdaebc3b8ed5999|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb0cb8b7651e3b6b4bd164d496233f6f3413afbffd1c3709b7fda859462e719e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-001a5d3d64fef1106c8a6eb5f637f8e819d875bc2f93d0b7b9165122021978d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3594959a98ab4a94e527829af0fc280cfae15a1da9729cc252246e8b231c3d79|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a305b9f2e646c68ffc7fc34fdee6e3dbcaf9e94dd63dbfa563749f916727af64|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bdfbf705983b24cf91fd17459848bed37196a7ba791cb1eac921c3903cb5b3fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22dd47e72f70a98dcdff323004807273df12d396b8484982378e906967d2a86a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a37a5abee5897dff507861f999539162f5930955e588c8630181e57656f31ca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6eabb5b4a1f0774ae9fb8cab67fa6b060d58e7952d1df4a92b39deb2eef7cdb6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32e3a28284c8b7df134c245ea35a21361faeef50c345a8d14c7526627247fedd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-181d4bd53f58386c4bf510aa43991b190335da62a66c140f75622abbb5f2b54d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-213d5dd593f415623e08475ddfd3bc3d1853a8e943be12346d57a6f14aa2f868|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b84ba26a560d18d16fdc3da375a1536be66662fc7dd3097b3f05abcce401f5bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fb0acdf313947cd409b0c4ca43f1044bb30397532dd7081effbe5088d9d57bd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc2b4a021a020858c2b40efd0f63e8e3f6f57886b57f1a9a11e37e51d674ab42|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9cde40107df2d533c57c751c727ba4956f262ba4504187775248ce3a7994991a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2fa9e7011758c27d29e17a5ec73ceebaf07964e18c48e5ea46c38b970fe8dfbe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c829c05bee010fa42309321600681c721ddcb5edfd767260ea40335dfc68fc1d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98b62fcfafe11fa3d095d7da3074051f814f7e3bf46c53fb2200f96f2b4465ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e2928a35877c1dbf467e4cb65732fc171d153424eefbf8166a700fad2b80cce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b6c64fe71f16a9ad66d85ed7af914a7312cb919b3200f42e356fd8357f581b25|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0631e4ab125d4b9f5fd5bd60e6046ff57bef72ddb863f6ea432659829d0d660|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15ff2107e324d58bf9f57bf812c2fe62464a0f199b1ac62f3c541549fe5142ce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b85273575d1fb3c54da6cbc1d3897e62d629b598aba48fddce3ff3f59ba5bec0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b69dcda354186a97f8182c1af89a26a8fa48e6a70ea03126a91f175bc8587a02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fe016eeda59e404818c6014a16eb6ea6bf26e58385f9d885e2128897897d53f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-362ca8a2c8821a69671252f0f29eeb7d745a8a18585877efc33789d041637b4a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6b813439585bc624d3e1ac7beefd01fe1c1a38eedfe0f5a0efb9de52ebcd441|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b62c461ec596f898ad49556e1396cee94cf24b9f0bb308a06e3b49449243e0f5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7218e48dad81d77050f5b8bdf5aa698c63edae996be051facc2337c6aaec2f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2697efa93454e7c9bb47f517afdbb8cddfb7b2c3f5442ccdda066d99a8c6d18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be58b3b82de63ea4810cd7f534050e253c31b2ae6cff45ecd83dbdb8fa2fa689|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac128433f6ca4a43c3542d443ba23f77a8567fed395b3c2912a015f0db710470|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1f0717e2d1153c40f4a4fb7c360872bb0208382d4ce97ad95a42a672d030392|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a77f5062ba82f53e103e5e413e2a0b30dceb942f5b4b37db2867e7bc817f8f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbf88e046a3c91830b358608271b26186b4cc2d9ab4580e1dc38e24ee2e1e257|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96c4988ba87b9f5b521fe4a9382624cfea9cfb3e15994918240240c9ed398b2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ce4a26fc075e581ba1eb37ace4f11378a486e150b13ef1c538deb434c72c9be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b68ba40aa221e4bd72d46c40c81624a8a21023b9f635cac57964afe61aad1434|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8816559fe7563705fea3fa6703558647a771522e294f018fefad24e72b32812e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-530ca581984e4834b7ae894f9c93567fb50b8739a3f464cf60a03cf40f946a08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e9e2dac1b7291e9c9af9dd9c5af597946e938ed58c01c169010f1e1bd5e0a83|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3382304bd6cd55a27831149d559a0025cb8927a34fe894a1ddf9d001b4467deb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4e3b3d9d1a7f8543abc3800109fb3517e5957307a6b59c90eb50d0571abd6cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d4facc0820e5cd87e7622d157e6f1ad71072b6018e0f9ae66b229c5658feb4c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b13bf5f70d0606cefb6f9c086b5ff40eff966a415bc1821aebd54b0a821a0f1d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-24a003ecefbce04edc1f6c47fd370c1edadc06ece98a8cba4642687a3a7aaef3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98f1ac58bded7c8a691f030fd7ec11c568713a8952be15b0da15473070417057|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1db3c935958a29d647509cbbbf703200dc91cc50db32f20f30b76b6273c377f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e6420bfd0892c943bbb2d2910144eef49da1589b6ddbf9683639753c822bcb0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-78c0e6395908319d048292cd17d155fb03dd63a3fa358214df51cd0ce1115656|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe8a4d9dfa7862f19c7b4d2c7f432a03b40e8842b637ce17c1a4c10ada14569b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7ea91e2c44aea0153480d6fdcf680c534d8101c631640701e1bd45fc38ec3f35|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0e3b37a99eea292d1f59598363dbced28e7e7a4c1c2ef904b7b946411b0199c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b16efb992b803c121614ea5540db6596faf6ae0c10d1937a77d7695b2c5c6508|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e54e0a2531af2599b850e284835c0b4523008fb5c0d8cd9e1a6862b0a1477ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
