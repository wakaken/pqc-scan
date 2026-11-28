# PQC Migration Scan Report

Generated: 2026-11-24 18:36:17 +09:00 (UTC+09:00)

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
|observation-v1-5d4ef7a333ca99d6b2dc4bb4cb785786d3803e8ab1d36200989acd0914f898d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3454a0a849faf63782fdfff5c1a22851f5f0bfe602aa90f36292b369ab97165|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca4b2bcb19ba4d88cf47e0c20695cce0051fd65130d99021926a739f43d0cdfd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8280b912302abe49fb465faa49b053f19cc7bf2a8d3e225fdf55ffa4e8ac251e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f133b347b9a374d1beb6a810d85927590bdf20f7adc29a84f1a32af2d2909efa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a2148da0d14ed63e0cd614a05ab5db9d1dd6a2b4b685e4d2daa40213e65d3a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f0a6fa957761c828759322fb49a1d5ba5d6df95aa4a971edcbf4328a74906fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aebbf946cb042bd132c0ca6b7cc12997ef4bc70416c19d34f632a9ac2a8d295e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-142d974126e1f0ce3092bec8588eeb83b3f4edada79ad8a7114a23f3f373ecac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3bb2c99d92bef43bb4ddc13b684c310fe306b71c1dc890c8dd3d65720dcfeb9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-227e0e129689b8c5ba732f2f630c1c4fc41177d10fbadc6be05bfc58688faa5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf45e724541a12c8d47f9e9f54523ad5813eb53868b8a12974c3892b0cc71b3b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-edde51bbd20ea3b5f22bbab5f1b80217694a4c42cdb1f4c03abff6fa41c2c09a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ecd60834c183dd91d52ce4047680410ee874a0771c7c4542db535c46f4bf21ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b0af2a6a7d38844bf63f3140cc869b019df63ed7d720a29abdb2c48de9704a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fefd447cba43adb0d0318a7a80201a789f2e86abe404449505ed0da44e4f1dfd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c19a223395252e81c6f14cb7feb13b55491c6c290d336e5b0a327c5eb515daa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55fec191a56adc3999e8c1b6e785d0a0e04e1af7c1fcbdb6ecc4dc3e64571c1d|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-05e69f2a807e7a6b941f95333021282bb3726b8ddafe977e1d1e397ee2e4be6a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d38387accbc458593875cb9338512684307cb5a2d03ad575f84dc16da7ab2b82|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-18919185e3e8ea994e081cb0ff3d34549502cf716849bb38b9ca5560cae98b2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e225dbdc7ce6fd68f49b1a6d72c09d27d193dd7a54dac0e12059b18177e2d9b7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d50078b372b55445200e8e7cff23b057f6c9a97bc21eeff0dc87de32878abc8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f935df420db170981b1872e6058cdbf531bc40ba299eda88c8f416f601554c27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f98e84646f6288c08618fead21625e27941f53ccb8d7f4934edca607593dbf4e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a942167e5d85588f8f955c6ca6173ea5278e43bd274afa243330d5bbdfa27f8c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-625c490f0f6c0cd4bde976770bce27a6706d5a1760f44882b973d79528b7425d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f47786f1ad2012fe7fb3282b81364c159aafec295fe389b1bb60b8d1aab49c4f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f425de1ec449eda04b634f605161abd64556b59defe34f19ebe7d3cf2d3d98ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a1da5f5c19055a38d80a665baafdbb873ab65c49331c46667e1c442d68619996|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-caab5b9778021960b55c38a3f543753d084596cdd933820266db16209f3a65e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9e2196c3ce925b2e50b99576e715d0f9130d596493c6df081c0dd3ff2b2148d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-657d4cf04dbeb062fc740c4a150035d29dd279d85be94746caf893b3eeb54874|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bae9e4a8f1feca3687e99809f7e122804d909daa818822277aaa650fb10f026e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-20f596017705ff50245327e1dd29649e7dc589dd60092118d5b195baf3a6a3f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-21aed7bc252f295bff255457ed2d1521cb11fba21febbad5f1339b3c4587ec86|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7dbecf6007bae1b10a5248df615189183f2ced27bb8462243b37e6a7c9b21f95|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d902bfe34c5f1368009d0b0dc1817aa8f7895aa75d3258a90eae1e0e42e70c8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-825e4f712dc814464e9fe4f17280af5951b93a4a2122ec63a0fb9c78d1c4e10a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bfb5343af325f65df0de636cda950c2d32f8f272fb6d9b24c47aae2e5ed40c59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1985c0d28d94d9fa87bd25101b14bbb6f553b430fd6241ceacdd0c120b48983c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3991482246088ab833f787662219f7cbe34f9c413bb41278739d2b3896705d3f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f08da876118228f373c6f2e57b9084a0332622da824cb79db589391fe8c4e4e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2eb172723c3b84579bfdda0ee0cbc8d913e9284ce1ab789a603b51aacc98db26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70ec9de5dddcbc8e188e4a85e5f14c0a55843c56a2bf00786ecb57d6aff6338f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d233ea89d5c85cf978d593a39d6d90b9cda6e9988c552bf5812f4875ee2f45b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84a0f077a1ba384a9d9eb66fb6e21b69d5f3a35ab406114d684fe831378b62e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e2e273dc151d8ddfe9e7365a1afb0338c2fce6e831fb8b6bdbb82e38f49d8fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa9bf614d43af35f426c1e9235b3cf34a97aa232c8a18ba3c61295644447adac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c2087913d18269c4736041fcefe1d8777303f75ab594d23a533189b6fd2ded5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11695d5747f65f2f3d8ec6894db069c80019a18bb7adba398c605440b27a1de9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf09a5056b403190e4301456bc8d30982afc938697683d305abb5e207278e30b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c1f46930191f3e1ac5bc84a7ca29b6b1de3de4a9617cdcd0b79311231cc3874|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ca731fe910a472b09d413d1792d551fc0931e5ed714fccf84ac65dd8b9e0e0f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e572695229798733c534b7c57302cd89e8f07329d3903467d8d909cd424d6c1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da0101dc24893b7ca06c2e2caee676a177409295cb60c269c44f91974e2f53ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2596d097a5577f6333bffa608c3a8ef65040c73994c60b4b6d1a0f37039aea74|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd7ff23faf8d857dbb8551bee0dca56e765a0d1f7e8767be4f9f35a475b0f6bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-33262c1b8c2804da40772976e88ae4ca4a297353dbc417ce881dfe47fe4a1ddd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-720d3c11235181ae690f9c82943fac3e30650f1ada612960dc75fab010f0db71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7bc34fbc9bff08ecaa3aecf78a8b445cc6044cdeca93e26814cc5bf26d7f5b59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-465fcd446cad3f752ffb7d5eba7adcbee1b46321817b5c480b42f7764fb79227|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-063f559efec6075e7dde9c3d4a0446334373e0ab74e2e937964bf0987b9921cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-733a7cee3dcfbaf85b3100995212411d39481b552526e8ec298493a975c35704|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7aa49573319e28e538b21b684830a30711d7ee09f5d193d5d2c74dccf25a085c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be84332b544ee773ad69925303b39a9fbdedb0d85b68ffd408cc4dd2c5f010c2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65357edb4f8f946d70a2651da669d999a5f9efaebb8219559b2814d11de78a4a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-895c37c8c588e474024b8cf4e01f04996a84740d6387657a34a20e2960a71ae8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fffafd5c0cf5886959504f3eb35754d46686ec0031ee69cea274e92d0a4bb55e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c810abe13534e0829476711e19a1f71e3f6c620a6c2f66f873ff4e1fcb8d63e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be827640ed5bbe524ec924aee7752ca774c63da7ae1f2cb1832a18fb56bc2ee0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-520fb9589fc856b893a51de48675b95eb8555473fe3d65d5cf3db1e170c648b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c7c038c6b584215c4ff5b3ee23af221fe0c265c2edafa460a10629457d1dd0a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b6a828f57f8176a14ba711bfc790a03184587de962d2607f5573dbc9a3d5e6f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47f1aa6a65cf0dca22006be52bd524405a1be04ffaafa106e3c79e1af8dd49f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-16e0b0386e9bfe9b8202e97a525e22144d1de5f1280891a2b91d2f2f79143124|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-66e8222f0de8b393b06780a0b036ca725f561d07bf2efc758b0b6651328d813b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40f7e7203de8a5e1019ca3902dc2064edda1724d16623a950fda39749588efdb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b847fafa0d9c3405fe17d445a88ee95da661d8b81882e2953dedaa0f1db9bf7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db8d2c59bd4fad2a7681c1b594f65a40de806e5533a5041804f07d0df0227a72|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-628956536ad50079fc4b3840998e13928ea80b00dadb9cec315aaa97187f6ee3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3bcdb403e3e492c23131d906f3a01c1749f67c65aaae8b4443bc4c7da76dd18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-23fe29b8e127773915b679b1a933edc1f10877077b9c8c26b8e46271d22b47e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c77d55560c639941cc9676674c5e2f0a49c70364cc261010bef9e5fc4e98a715|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f394ecb6a7a9ec57dbb1b6eb906653d009cb0426690bfa79b8ba986a9d670cbe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6db2e1cd47154774689731b7f57ee5e54023e6e9dd4c0a5b61039dd60f810e15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02b3d3d3208320656a6ae03affdfc76d07616a52109ed06c784444b9cd961014|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3e9f875bf96b5bf506061ecd69bc87b0ffcea3fd06adb20e814e83485a66ac70|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6994e9ea5e12ae4fd0c3546347c4c1e89e96908dee6e2df41646cf46d9d2b463|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b5c59ebe2e15ec0cb4e4fbbd37cb271671e012310c944df36bdf578719ec95c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a352b5e10b093b187ae477ed47245e3f49ad1fc7f5fab7f870da21c5b32b900b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8788a21e25d9ef98546e66f87bf0c64af0bb48174c1749feb97975625fdb166b|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-259c733e6223c7ba235d698c0ce887977084fddc679beef3508fb945de6b3735|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55e9d8c991c4604e6c3f2dc64be1ed3f77ae6a7488f952077246cdd62cec5672|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a82a782f7f7bcedcba45edd8db1da39a77c32f7d63767004c4cac82452ddc53|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25d8d8703fcf1601f0acca8a472ae2d480ffdd4ffca9becfeb62eab9ef62ff20|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ba2f4c4438b469f23d572bed7f4bee0ba4d53fad8cde2879a9789119cf460c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68bcd4ff62bc6094ca7cfd044a2226c0777695c070d8bacbb82b43212c14af2f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ec6130114a9f5dd175dba0d1f1d88a08b1fe763aedc5f4396e297187424a69d9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-201f9738ab16c5c10c820fa145a4733a7d9e9c4c4b9da911e27f7486d7793f4a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-181671eb8ae3e01f2dbfe19134b0e35cbc3c9fafc59a8d32ba735d81ead378b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-88dddc92901ea3f3c44c7fd9cd26156731d34d9165089ccc9a5734b39df15a33|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a3787ef80ba5fe0d2f23bd61cf3b7c4c80dbf7cdfa0a114c26a0aabb1215c87|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57ae67aea9793972db1c4d68030df2689f7cf2a1b2629b0727816cb45dfedc60|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a78606e4200a06238dbe86dc88ad9e8fcfcd8e939ab32dcc66f336b0bcda5683|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bdbc74955d1eab282b60539f67440bfb0a9f9db8a8d03002efbf145eba978d0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2de1e82ee28cee3f7115bb52855143db84257399b86d4563da524c8a2b782ace|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-632e3882f1197a788e37cc30f54b96cfd3dc54f8dc1a2d4bbb1507972de76814|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-483f54977eb8eabf08d9480724030b0659903dbaaacab081e7fd7e937373d864|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de002f60cfbe009277427d20f66511f92d3793a366d9ba1cd875a05f1a2c47ad|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5b756ffd7abf4c5a5db65f1f8b91766f925bc64ec3b08701047b6f86dac8396c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e53091e7fd1e90bcb7c13eb8edb3dc4e54e852b2d51d33b3fdfffdce75897a5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3da63563da7d35ef7dc702850a6b1336f9a4b1ba27d61645cd73fe0ad1565c58|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-77707be86d5f6ff1cd5656086561b7dd78f6c1fe7301f36f11ab69fe31abdbdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a008112552a08a065cd327112fa1f81177cea2456a95c7738f66f5521f43dce0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c8f1a29cd35d13d7237ee34e18e5d04db2dbb08b278ccb9eedb6d62cca1f8907|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-58ef4a561131c40e472ff117dc0aaec3d24adf27f0855cb92af71abc34d80883|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6634887260586ccfcb63fa60122a13c010aaf10e50867db0dcba6ecdc36fb88|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c86eabe599c138d5530d4baff0c86de65a1340038b1db8ad28eb5103c7362372|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9cc2928d67a34df04d62a38cb2eccb9fe9c928f15d16c478096fa1d0da1f9439|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e74a5a86c8be450e1874ee4e9b4ffc59cd4abc684ad0d45b6dddf76487e564a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25b11bc250726b6edc121b037d45e421c021dd3222a0baf0bb6df58330ae3598|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-229b8407f47bcda6c12d0ae2c742154f1e78e0b257bdc0975f7b5c2c64e87c4a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-eda17af95c553e0ed963c2789109426bea7f52cea656fab83d25673a1eca6268|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7132f57be26e9f1a7e30f3f2de4710e393b668d62bcf84c86a008e7d069e4682|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-668c1979064a650ae420db0f115ef7bb113700c95e96637daac82b860b8419ce|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2d900053fa2448b733165454152c0883289496e16fa9311214edac8d93b3fe69|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-35b227e5abdc4b4955fcba1b0e02864f70aaae62f5b3aa2a1a4e46ba835280cf|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a0f0be9c4360b79cb200fb310268d035239e630c4dfd89299968f9a94c98ad49|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f6e58e837d15e2ad6b4c7cefab312d1989a17af406d3feb2b65a3acaf242aad|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-dffdd828f2b0837cc03b4fcadf16772ab87b0ce02cb57f6dc703b038c38caea2|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5cdd9862a1b286d3f012bc706b678edc02c17066ffddcf71dfd2a2b66b8acbaf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-488fabd34b428e643cb9c9013e436509d2ccea44d4f7ac9f3d88561c05c0545e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b594e2ba550f9e33f644f293df2de0846778394ca77f8281934329c4ccfd565a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1cfc1ef43d977422c6aaf9546889a755ce4915b6d91027c0d62e4531c1e36db2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0574e0d0e5c0b68d5bda925210554c975ff593cf8bbf576745a8fe19404f5ef|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9dd5af7817b24ecd33ebad2a8f41abdab59e70719a9992fea4271eeecaf1b32a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-95106fa78ac25214127487366c2606233f768ed3e89ccb9e3ce834eb3c686172|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8e615f65f434c89d4500dc31b68d86c0bed74b7dff280edd3cde94ffd3ad17b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a15fb48e6198cb92804d0518b5c9e6d1b5ef63c23a1258ee3de37e8824fb013|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f4807532241000ef0c1bff65d273a60c92a48b01d1c1652af36ebb61018bf93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bf53fe7397149e1cc0699a7b397823de82ec13d3832545df2ec80332a3f42b61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae41d5f89feaa4f09ced62d5e6c2c82cd0b80c419cb506db2423472c0857f3dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15c492f78fdd98cbc91f3f7f5192b2fed69fcf9cc6b23e95334bff0fb4d497c7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3450244e42c2847cd187631f2fd86f448339b7690d442ad2ea1296855766acd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6cd93d7b66cc9c3f9f76af15162db868385cc32cf49b1f2a7983155ae1394e77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2936071172616f33338632c68f5ad57e80288b49cf122c821b897d01f721d81|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-444629276968446da57b5dcd547d697f29c8397851c614af78beadaee2853bd3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-09ca8e42e20b54870cc48d58bb3c453fd7f969a72c7f73738ec5dd1e905ac43e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bec1696a0159fbc0daef661e68924abcaf4b56791daaf38aa19a690c0c099b7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37d4e72fefd40abcbe04254f48c8dd74e8a5c753bcc8ced3d7146c8e53a0eca2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-063595ab67ef4f1ff9fda85397c33291f10cb26b9959e4bcaac85b757ab6d980|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ef55bdd03bbda76d44bb66123f59cf214ac5114815dfce914fe5ce6c7bd83be8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e7deee9ae074275d386d02a85dd0458738ae43ee1160cd14b410a61aa87e950|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-391e8f155acd808c4f2f0137346b735021fb30b238ea13c140ad9613abbec453|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2b3ecc39c400a954ce4483cbeda0afd234c5cb4dfadd1ec9d16df2d6eb3f9dcc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ebe9fca48502a1795a2f18a1fb3162994f20c2941a319ee765c304fd3f8bef9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-abea81cd132bf22a88866d29a7dc14a4289b0496c74cdbd14aa746b8c19cff45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c94e1384ada26155e366cd23b53431a20073dffe2c4396fd8b16268047e40f4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c0fb7fda9b15552d3d5bc18a7d8b35a245e053f32deec163808714f2dc4ed18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae2b27a11a78352e574f737c03b1febc60a38e3f502a705a1ca89fba4947cfa6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3fc30004af892c1a7fb21afc5d55a8cf483fb16622363887a47b2f5d70020068|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d29aa7e2941af975d26e05e46da2052be6eeb74f79a1826825e2b02a89570051|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cfa8ab9948f6ed5c45ac4fa17b1b6e055b429417b8706fc99fe3d2e7a9b190db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f24442d4cd936486e3c505a7c3b7196161612fb13c551fad587f5dee69e57cee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5e9ff52ce04cc0c535bb077580e757501231e54794f24cb8686fc2992e97841|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64f2b1a30000f4e64303faafce1ba5c2cedf4eeee9aa03401c1b9612edfbd98c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15a2beb410063993edbb1a325806d859cdc68e681bb19a6cae174d52fcaa6b50|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de96962173c1dade04f2571dfd1cab86ff8c212d349f43bd4d9bcc0415f94391|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de6b91f3c4a27576da314ca345bb6cac26525e4d4e2e41b989bc55e416bcf4ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-acc8e2d9f480505ea00b30cb66261770d2b922c6d6aeba72848007ca022434a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed8dba431523340357893199d2a6111cf3b2ff7be35649dbf0c6f69bf6077788|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f98d5b3a723b5d40e49f1f209432f67f552f68d4cacad70b290e31aab5bc2fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f91f464f6254dca3977de3faca0d26f8ee0e78fe98d208210d14fc3bb2b683a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06293714ee231f018bc50339c3ec1246cb6552f014cdd59f1836ea145488f1d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4786c42199866820eb3e02fb0361453c1d659963f1ac1bbb9895711d131f285c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0142e0efcc8e483cf3c02393d4bc1210f53e89f6bd144b3f134ee81ac36ab514|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f3744303d6f07e3f7b9bde88bab82b3b2a9412e32ccbb8b953133e0c6c78c828|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7248d277781371f4a5ca029d5e069eb455285bed5f592c66036b198dd818a5fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f907ba6a5c1645f15c7f971045ff05ee5b0df4550214f67a33528b2208695cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-94279684192b2b6f1c293afa5e2db40e914e49e0215cbbc3b555ebddbbbbe779|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0aa3c441a9d388998786537df692632b5cfe5a1ab81aff66056ce2c30ae02a68|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c7dbe467319ffcece87c08f12b1ceab0708727c3098dd721954b8b361de4d072|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ff61fa37d07612a329ec39a9f88dc798a09ff6baf21e83aabe957bbe98644f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b71e835f9726e4716d9c1e86cd219f0ea2a7ea70282b49aba98cb37b7a254b6b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3da510d1feb507b586851c77340367ac3f8068be4a6b39b0ff0c6d90d525b97|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c033cc80882463933dc849e5aaa537841d13f39a3c11585b185b1888cd2d6fae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-81cfc1c9de0a6ce54dd044fc6cd42d00973f409f63deaae7b4767a010e41e9bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27ff9e795fe6c9cf45ab130185af3f8dda86d507552170f9de3334d4cb516466|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca324c6b1315020e0e750729b777cc8c710baca6445f4c9b038efd4075c7b3fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9db0e8c1f1d2a29b85d98f35bc7aa4ed191e579bf328f00671b5e8cd20e710a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b50e10715e5023da9d56f76daf4223d47d0e943a03d68edeaf05584e92000d99|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-507f1380c3f11a3b739f388ae11784b4dc47cb0b410fd744f2b9287db5b600da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e9be338bd8a1129a75e27015aaa09521ad15386047299f206be1d0f403d3344|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f93da14c2f40d1f3b06aa4fcd9353b627188f9d28b031019ef8ed1fad1d463d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0379d2e2bb692463a827b7a6bd06204e902b12e41ed55a187b9eea7cbc28860|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b33f017bd85b672df4675bec3522053e22a5f4d4c689d3cd9c82f1561dfed6c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac92625581bba0f5b903d21cc5689f6f35482fece7b6b6fd9276449bce25ea61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7791dafec8086246e49e60963302080888b2fa85a3b4b437fcb0b42e4fb47ed9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-efd57ed51291d49aa4e7db6c6dd35135d9fcba9b057763d946c5a43a52a90a1f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b59ec0cbbab693b23f3cbacfc57ddda0e029800b47422e97462bdfd981cc408|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e4d1516d71cc118f594fb69b5a9759b72092a5c5ac64276a4b1192b0d6f2387e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49e10ab95bf23dc14179af2e6b40c58dc985413cf940f722da48ff350bfec793|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-666f6ec8f2161a5bff527143538259efddf1b9c72d78aec2a8e1939b18984db6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5eb0a04c84a94f4bc67b471a7c5a5e8bce42e041a55ec7f4cfb4696b847d97cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6498ddcce2037bb561e1579ddd57b1de0578be5d2e38b7699c1251fb76732771|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6a299a5fe66f5f00c52a4550b72ffbf86ed29b62d6301a6810965bcbd7feadc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2d4e444d6baf25682ebea5b1469930f12393b5e927e9e00f5dbb95e5591ceea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b8ce9526a8ef2c99beef961bcf375749590618a7359fcef67b21e3843808f5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a56b24754e47af6cd65fa890c2c6f52dd58a6f20a48fe23a76ea2607b62cde2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b246d1193f99de88a5d4341986eff0ebb980ae65e503caecba88161c39934979|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4266a8688e8749c9a17431189f736c6d156f421daac363a0f1252d8ea7703561|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3af61df52c79a3c735aad4e0bb53cdc50533cdb02d84587027da59dd8978a45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c64ed8cfe00c624520154787cabf34166ab90dd459175233a6468ee2624bfe6a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83f93ec70364be1387df21d111cc7be1cca954b80093bb1e7da6365f425771a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c9b1c31eb7fcbe32ca20f96a92c1ee4f410b2a186a73f11f1ec853f8a2e140d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e34cb4b6d10a83da01769ce34cef1b6a608dda0fac27270ecc566a457b1a59a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3fbb532d47baaecb6e3e6553073a081549f9d9eb55d763db1a830bc30498f899|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-08a581533c88c3dec56ca0a7f9e473c310d41b20d5d8a7a97a445d10bc08bc7b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a361fa6653ab81a47e00cef1e29c386319976e98554e80f3f51e0e728ad70b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9f25594ebb486024694bdf8e2ab44b5e0c5f1b55514c02cd0522769626cf4b7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f17c31b273a843bfdbad4317b9c3ec89f55606cde4cb4f5762824411d341d74a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dcc14466df18f2f4bd39cf9e75655071f336b4215065ae9013d3fb021b759f21|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0bb4665b579ac5104169fb218369e92ae9669a2f6ffb43ff2a437404ba3a038|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-91d6118522efcc8fc360af5e9342d4b7ba1192ae12c649b6ba147a742efdd6ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3d1c39812d9caa729ac24767ec19ff8470274e8528dbfb68a6c531bc1c44681e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c75fe0179c700cc8e9a1242ca67cb9caca688577a7225d8b1b8ddaead6087c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76e2f21c9b479b4388fadc302f92c8ffa214cb733c6db5c93a1ee33109e5b884|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2105b2cf7f72fd5f3fa91dbf814f5d1b8a2c3290f6d3334dfdb4262dd319bbea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-71722159300bf39093901daf2c46024f48f36db30aa2c56486e49cc74b435d8d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9bf0c370ab02bc4ef78720a933634b2a9e79f890dd339cec6f2fddd8d4acf305|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cf76572c6fe04b8094093499020a5464b44347fe3383526f0623b6223b42ee5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-417f3c52bdbc90986741455cf95637251f7bdfbb7dc174b1d1fc9bc6344a244e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18496bed2c5602fcd1c8ba386e556ce82f2e674282082a885a4f887adacb8a84|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60dfecdf04b491fd405156ab636f458d678bc0a8f8233c7e5bc0e35bbc7b24b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9409a087258a4263caa8b671cf9a4f7aa61df40f15b2473acc80e2746bb8eeec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ba964632b028f8f12d947819939e2487bd8a2bcd44d8f7c7087790361bd0b8c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-689ee603c2241f2473f9b5004b96fcc455d05721696b251c178931f71f6b1599|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9765e968f1facbc6d2a9d427a921665db4655f2e373262e939318bd433a20880|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70976f5dda885ecdafabdab3af3b58de6327917dfb508af6aa9d2dfd83698519|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6593b32f3e01e8d42a0c10ef8cdca2179ad12b49101cb5e99a6da54002f7ee5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cb3ab40f01f4b6c89bd5bc800f9d52baf6b1701be2509fe80cfa78f21b410e32|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b27ad1178ff8b6fa109fc90ca023e6f08da634442302c7d066fc7e452ef8c2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fdd9afdc0cfe98fce8e690061cfbe254c0ca7236190b7745b01b58759563f8fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29d6697940ae3cd3f922a4e542e1c1f9e23dbc79b91d9142b95cfdd622b7dab9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-481d4189c164803c4315d109e08d896e748b6e6be0abf542c876fa657714cedf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5dda91bb6291442d35be28ca0dfb1d37364e87e30404a681f2fd19dbe989b69|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc48090364a74e93a5bcaded298dea06e4596d48c1bcce4e5a182dee3608fe5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ada7136340dcab201937dc30d7578861f7374277603bcdf74e2ad220d65840ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b6bd252c791bcf4844f425cd3a0a0df3c360149e609620fbf4483fbde5d9650|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a3535c900f4613c1b85b6cdaa2ad0681ef69a973b8c8e550d72b9527b766c2a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55b552ea50620fa960e591bc72813bef2eea23274a9fbe73f29f694213907b99|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50c4e7b35d8eb80b99f414cd1cccf45eead304471a8680911a065abe18cea9f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0fb473d6e2a136a13472db72a564288fe13c28e693e06b93ea834b936ded469d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c370351ab76b516231b1fcad0517e93e91d1574ce1e89e5e8206827edf24664c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2e4846c291a935623286467ddeec954b62d1e1903abc4a2118abeff1e49a244a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0958f3f32feac5b1e4fdcd3f18e74cd85cb2f8b9a908b3ccfc395246ca76731|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9aec8455d07315a91546ef9854e950f662aa42f0bf38ede4b9acabd3142d17fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-575e3772a38541842b28848172f4b046cd2f771d0b13b905210a30c3246be7c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06edee069664fadc806fb0d3e035d88b2fc1881eed1756b31798040179cd8669|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a018bcd5508ce7d7228dcb55ac19b3dbfa5ce5256a1091cd2b6dbf68b71960d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96c39b1fa1c41da825134b8070fade20998a1ac641b2ad7816a2d862f4b7de5c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6123084bbfefa73500bb59ec3b76d46778f0d02c3903a3b35b532111d531a49a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d13f7db86481cedf912c61a78887c91aa7df0484f4bafae09a2797d992dc477d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dea000e838d749862cfab40cbdbc00563c9c454ad0242c9bb80eee9a3dac2dbe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a63274e1a61762eadcfaa466983332ed68031948d5ae70731acfd5cb7d07f5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce435cbc9863d09b7ab10306407159857c54525ea1617ca6cb1d4f78ea54df02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a90b977c246d7d022e3cfa52a06ccf9c7db77cfd64ce2d7bc37363bfe31f809|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8353546ee2063e4974a32d16f08e5dcd4e0f11ea4efc4272f7c95cde52282824|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-623fbab03d68c1700a1c68a6654f44c2be69d6ed8587dfd00ae52866a2f6c39f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4dc787b7abf2c25ac5e69274d67f005bae7eec633a842152e062ab8a426ec534|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b0aa0e7b5c438602d0a24a2daba0c0f9a5d5e21504dce4534254da85c6beccdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3b43f57aa5fdd7760cefbc732c9b3627d711de3e174e300b39d654f2ee294f3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4125890c5acc03f2e892690e074194da9ec608998945af289ce59acac360a693|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96a24f6b23f8b35e4fbfe4bb893b77e6260a53779b2721b6152584d3be4eaf65|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-505e5dc363968466e325489847225d9c38c173f05edb65ea3deb6abbfe448920|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1d57c6f8be5d58a52eb1767763f30449c256c4b1c2553706751d13585c012dc5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-866f09300566c739b1d62de13d084faf44780aae169437492cdb04898880a8a2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f23f0ecc1fee2ed0e45103c9321ff497cf52804d7d52998e4c4b03caa71c10eb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db3798a4025cd98c8a17d26fcb47ea5a6cbeed1991c680c9245ab0306faf7299|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31d2d825df4ba12f8407206f9354a9fa3c27a4e826df94064dbb0117c853e72d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13102ac1c3854efb5d554b12fcd32697f0fc3b3975605316d1572538aeec79cd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c3a8cd8e03fd8b40dfafecea105736ebfe6f8657b66f8d67e3abe72e6e0bbd08|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-08682c239dee2a4e9062ff095afb16c0d834b0f16b04c4a345f40d801fa0a629|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aaa383ac9d15c163acb96bbd413f6f62efaac1d92539b6cc8fe15d86e71f152b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fab6a88495dc4c822dd60f9df5eaa14053caaaee858067bfea04b6b9459d48ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f22bce67ef2ad61c8829eccd9b5458beb34eb7a9526b57b00f47363fb911fd3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8dcc6d903dc8c58f6a17f723729fdc90d18691820ddaae77b2c73524e789c840|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f161a67652bce013b1517e565fabf395bca09afd1582538d9b390f04a7c85d16|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6915a112f81b94c2d35850362243cfb961be0e557156609c9373692560185b7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b63c345d80f981ad6a50076b62c5771ce457980a945afabb589d61d24910d76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e560bf3ac6bb2e618e05075b5b4c92ece01d83a4cd281fd7c36eb6b773b40f58|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca32a98297abc0c5389bfd1dac52c8145290f630ec7feb11d3332de0c2f9375a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-198b1b06083d9656d2a416c9a95f341889482a31e636bfc4f2b9743ba96b2855|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8fef9e1c5580d57f3214c18b7a2a493aad10b3a761626dd67d2c05610963250f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6e40c035911095154ac03c73c54ecebb39842c9219126703b3b3f8fae469a3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3d5cecb1873578c3586ffda1f132eb63ba79d2f7070040c1d7b5e8ef0a086192|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69dcc0f5a9e01dae41e38b98595538fed79dcbbd57af609ad80154f7e4544fb0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-acc064fc1a59d624806a7a1ed6f68c13f91b83e31b605fceaf603ab44716d0f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d139a9ead57cedd1604cfb50f1bff2d4d0118bc9a59041aa0b0caa127ad5d820|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a5703726207fda0c06b5669210ea87fa1cfa59e41cdfec58bdbfd0f0b6d9c8c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fc869ff641d1c8d1bd6b7df0c1c022fa5fd0a295e85c1e3ebf4b93614c67de6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67f51df3d33bf19e34305e55809c521dbba37beda5c839c97f68ce40688191cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a34630e239eabc4a2e56326fedb8da1b1c4834fc4a7ea77bb77cfcf10d4c9dd3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-051ac1a80f6fcaf7683b2ab847170a4489c77a51f4c60eea1107008949ba9dc7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ccfbb83286db720bbba83ddbebe5ead0eb41354f7655e8b97c662a05085a86b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7d51d74d3712842fb13bdf16f6c5dc9ff569682f1ba57509ac661f017fbdfdd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-407a9a9568e2d58b6d29352c184ff066ea32dc1ceabc9eb0cd2b38fdda168319|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8aea791b7f2c5f2e2e059500112ba57efa39f7ae19c3fc6daae89c0657b01daf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0eb31e14df055d3a55c1e940dc9132fae0d91bf10d31f4327c1b4b9f6fe9d949|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e10c659e35a309b5b9231e3bd49f809e8f1e96de2d6010d61f2b954c3ba0bc41|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a60365a1e6294d93d996660f9914b5244de1ca45f93f19963aa0a73ba23c2ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-563603bf98b5762cc5d6427d2fcbf3cf41fc03ed2b921b80b561a147ae86fc66|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-28e2be313c40d141cda0013ae442a8a38b51a1a74373ab75c3a99b8da1ab155e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d11fc4316a90d73287316fa405be35ff0bc40b511166da2f85722a80349882d9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-420326917aa56045de7e5b8231560385bcb7aa8bf15d00e99ce92a95591ada61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3abde6fae2fb5d4b59033a73ff55b05d91d434f8c4f4bc9ed4b68e1d38fd0c0a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76d6424f135e57d446a238ee59011f06a9047fc5ab1abab75cc94c77e8257728|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cafa34bd4157a48202986aaa10654c545021b4a25353113513441ae7386d9f8e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6200206d54fea1559beb0c71c32272dce3f93723b4d69b8ed28f9f3ddbd4b81e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e6a4d60c9928e960800509da3110e63d456cb081b1fffc6a466b510919a6aa5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b37d9dd13031716f01865a4d57b6b9a319850cab20a5c210aa6ed887f71bddc4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-745765721ee046a89a24c26a59b6ee0e9c214d9caf39171a5a5a995a1821a362|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f333e56d1cf50f934a2a2fffd725f3eb16676ac8d1bbd2c443764de8bb94422|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-20cb4762d1329fd65e18fb7bd7390fcb7aca86b79a4348c6ba6f1467a6845ddd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-575b36e07c902021e29ed2187b2a129b14ae736d902c92efdb6507e7f4474cdc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2810479008c47ef35efc2059447c5df27267840358786dc43bf1765e16be32cc|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f2501d8202631bac8c5643d959db03af623685c47d0b719fdd6e6a4453a46bd4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f670dad965c84241d1f86fe7f19d42802a22d1720597182f48c3ec1624fcecc|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9136f7d44e8e10a63e66ca2982e9555e15d22e07a5ebd46f87effc041fe4e191|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dfdfe81c6d5866ebe65d1723117cf13a86899e876f659cc1dd3044992a73735e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a3218c017993ac778035ff3b66b159806787a7100e76b0cd7021a3cf17a4ab6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-094c5b6852c5635813fb9c328ff870e6755c2b70e3502e9175d36fd265c0a249|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5deb3cdc546d3f4f6e6aa886bef3562411a351fc0e0f80c71c9069d2014c23c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b94a4e2e405fdb079d4051d052240bf172755aacfed3765f0d6d17f48b214684|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2cf87e0494617f49413666d54c3d5174f402d2ba8339ab78d0d9428ccf1ba0ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f440cf4deb0059d492c738f3cf62593ae9536a189e0cf9e888706acc532b44f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc8ebd9595af45eee8a057d8240f08acaeabbd68f45b2ebfdfe5859a48815330|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59d1d81981d24fccda23d79d83d8fc180f94ceeee54d4959687028e660efb67c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c8c8f959aacd62ffbfa04c4adab4cc223fa33587c9cd8a1d4703583b951fe056|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f09322cb60cc36fb57d105ffc045c2f8340679617d7046ae5f1ebfd9d65e951|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be76bdef759e001fdf55d41e18c62988d7f73a45be62623b0d2f719f840b149b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a3fa7b9e80e676be4d1dc361b5d3b1aeaaa757a8d827699d4a067d795fca77e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a53c33e80e939d49c2006b9e9a61608a6ed84babacde9a7c87bb56922aa02939|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38d47fcd3697b212d48c66fe7e3c907a5cb3ab39a6254e6ff9ea7b99c6d0cf5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-21ffc9670739a17243b0fb915c47fb275b77d01250dbdd970a413a5efb1e4171|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b9cf6688d5cc30cb418f5c99f9f2dcafec5147db182dedfe2d32cb0fef321a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7b1be3f98de29e3301e7d1578b5766c41f46c7c2f82fa038c8f4d7331bb27152|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b47a3aea1ffd69f4291ca3f06aa5ec06efc170cd609235c8f73b9ec5ccc1430a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8baf07ff7f95dc028987edc73e88e0ad122806082cb8924b3ba5b9f0429475c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a2c481d2b9e85fdcafe0028ae051889d59ccaadf3217ac2e193e87e63b41e4a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1215af0abec3b2d5372bacf887c46e63e8859978e3689713bb255c6f3e11cef4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8cc6869bf7740ba4dd934f75f9fbf9826b8a15d432afd5d709007f49819eafc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fcdcab7a8f0ab75cf3c28dbf5dc714a312983ebded5999c9793aceb96795ca0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8dad8f3004af1ddc3f998e4950f1158bdf02d0425912a815e7463d9278a31933|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d6cd8d723b2022258beb533d39b596784fb8d421ac1080a43e450de8a591383|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8dc45939645a02095176793194fa7edfa0b75eeb3f8b33de8307383813db7bf8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7b98b4abb3c6f35fe38b3f53365cdafeb7583bd518f0b251a9897d487cbd56d9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1761c3075e2786e5497172f8762f1dcafce3bca28ef29045ae9c64d419408e7d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4534cd7ec8ce7acff5d02d6d55e59945467b6f6997d2cf570ef49cd7ff2520cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd514ee377bcc314ac932972794cd7212bac8eddc65fbdb5f518b525e930c03b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e52c5f1751ce79731d083f99212fb0cf0fd9e2f9a76fe740e32fd4661d93ad6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2a46a48fec8e25fcb43ac97657f5147e148bb88da1ca3aed07b95d06ed40ca3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9af71057d664e5955f9be140bf4d8a425ab9fb6c19210cf351156db5ca3f3e24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a50c0f038b503bbb93e40158ea4c329f929f09a361276589b40540e437db6791|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f64dda36a5b760e158ba409bee2c611911752f863d21df366de228501b02c66|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b8013f7aac0358a3891d8b679e86067d60a5e4b2a4a9d8187e393eb977fa6208|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7abd3930976ddc221971416c8af94ba402aee14e10173c70390ee7d26715e784|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f4844464f39ce27a26183e4e2deae38378e9a078bb2515964ad5107dd1c9334|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4248923df450ff8c01313fa0b893a9c85ac07abbdaf8e4e28cb412f3f7f4aee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2604093a70dc0883d3281c21f4418c632a6e3bd76204557ef5c9f191be03eca0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b9d0cc88687ea044c200cbe0262435cd8f65e2d132cc72fb8cdc13c851124da9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0f78243ea1614e6181b5f6c51e0ddd10f37b81f863777d4d67b336aa961873b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c91da7b451d62eac7be8e20370b7018df707cc069c6dc603a1e71a36a888228|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-277813e2ff75bd6950bea1afa3c46d247d1c4b6562924ca47d5e1d74098c2f1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32494593288ade415691c30d75ec9819df792ce3204933d67572457db83ecbfd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-87adb712492e07079777ae6294fcf0890ab1fbd9950bddbc097c76b5aef14407|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-151f3781b9b6d37cdf60123b0c70a4134158dd720337a2add9ba29f1353b03a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d87e0211b4d2ebbc14ee2855b856c980d6f21e3333e26ef56a9b1d5d4080b545|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6757c7ef1d8e0596af3bdeb18d619bf5a9641be2c17de096e21c79c188c7ad4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b12fc8fcaf59781d4260f8ce922d046c94897f8c75329e71fb4b600af09cf72|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c83ba580a6ecddefca3de0735f7c55bd6cc021f24d61651572cb2c8daa57cf71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36fa2181d0f7e83823f1f4012d908d87bca9f24dccf32cde063e96269a8ef6a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d5fea14505d17373b213b8d430e5098e58222b3a9c7a59bb0a0711f5dd525ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3ac3870e49bbeb0ed9a34f4de558a47041e29bd496c00807f71791b3852517f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de3c54e6b433d5381c9ba67f010e9bb0d7a28a8cf798240a3f2b1b91b84e4d40|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38b3240e4d126e15346a934b4626750c12be584c6d50ae426679a744cf01da54|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1fb0484f591b0933a2e7fb896e423581e848fc5dd331d3711faebefd0d8b3d60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9b3f1a2a620505aed49aa562e0205d5ef5e54c83adcf15e2f340f865fa601bd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31465907d542ed54714217ce8036a44be1b9e1b4c17ed8cf6b9247e3967320d0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d627ebe21b2657d3164f91a488f3add558bab6ee00df3706f3cf1cf98382fbf6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ce7e4b9f195fbc323d115acb00c9259b70a2f01c3b8170338296c90d0deb986|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e85e8d0344ee13b03523e8a81fdb3aa97ecb25da0624bc3634ab356ff36214f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67946fc384e433ee60a95b71a97467ba83da80c0cdbb09aa2efb256ecd938ffd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f3df4c7444efa03d15747f51cc80efb73ca3902f82b7e3c3794c64b7feaf9355|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff47d9f5e80a4b6144105e3d8a6558e05d9deb9c50e0ae671e0a31098a14da08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-45c93a0717023e6460acd79acc794ab10440c72f8d756d4072c7c65ee39777eb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d471530918323aad91dacdbdfb736cb4bcd991b1d307df7a315130a0074a0f7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f223efbc3c04c1fd38c3ad8a5f423a16045f5486f375b045a03113e9fecf62b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02590dad33de308f6d8dfa14beb40b60e23dfe797bf613a5f4dbd5ba71fc43bd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3d5e739b11d229c0594152f0b2195a265ada0a005de29b454a1716f909f13ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-94e5c447322d86bbcf04bc0828f448f904ad5dd1ba7b5228a8ea9cd5406b1aac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-599646e168ac7787632bffa252664897c4ba9afed36bcc28ed14fdbcf74d5299|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e204f05fc637e80d356ff2c6e9bd9f4ecd6dac4234640b4d26516a3d1d49cfd2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60769133a1ef77729b3f4978461a90eab1adbfef0ce9489848871c78d6dc1cf2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d0aa173d1d741841bea8849abab80c0d8b088d5ec5cf8f227a4e0c6fb62c71c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6669b9d7a224830a3a4a7e914cb67b59de254d4335fda490086500bf4c2ab88c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67b1708d6364a53c7d688828a900e78e03a095a6004ac7ee388360bb74826b72|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a545ce77e0d0d272c8d09b7420b9b86df9f357ab646ef7109fe3f49e71e39433|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-82eba6c6006f3428ead8e4e8b02290f629e940be6ad2a9152978981a408f2666|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b7016c1a61e5bb63ddc2c2cdbfe9a7f2dfd7c5d32a3e2a6310d80cd1efcf06b|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-be2fa505bda3cdde8493ab33d48a2c32110960c007a6c7c623d185f3fee3d02a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-57a0f8244a67943f8c1ffe219f4ed98dd25e406867149ea96d874b56caa4f129|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e2c7f10c58595664df4406c9c8eb0ebd6a6a4ddfae50491962b9137ef30e916b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f56122fe1312faae2a41930e005e51a014476fcfa2674186874260d4140782d7|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-fdf1cb7c180402b6148495bf41c55922e9b44d07b787b91dab2fac7f0380859e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2b8addef51cd8febd3c63d1e63ecbc9a5e510013a32155912bb67ebbb9cfc825|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f8129830d6f2218f71868e0f55f87053e64b3231a0fb8dd567a9ce59896d1b26|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7ad128703091456341677d6146e9fb1dedd3596c26054812ebeb108e0941582d|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2afdf312e6df96c948bdba4cae0fc0db5dc8d154cdc53c0f5f1d47349c65944b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1a50a84830fa43bc07f58882d5dfd5592d365b7edfbc8422783fa29eb110735|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8884fc8c2191b806be8a73008c69c003d0c5fa1e17f10f3ba039ec1a6725054d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06570d5b74d342b9d5a85750efc1535fcefc57a282a95b7b56e87ca58316199c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d35eaa32437ad6fb97ad5f6476d2fefe7844efeacf855fd930cb8839daa89ff5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d72ee54e4754f1e594c6e1ae3183145f9bc6b44220d4abe46680b92f8037d67|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4c97eb106bcf10b6a9dc49509ed93d71e67bf329de65970ae451d5d7731d946|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b885a206c40730bf7048398dedc914835bfd136b71a9e38ae1febfb707356797|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d75415c95cbe55141aa5d69e96a1dbdb3bc6178967b8a1c921dd5cf9a48139a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9ddfdd8ef248926d22915696383236a45fafe3417dd78906cc7ab60f511e54f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8bd0842e934e55faead83eccdb1ba793527cee33652e8df0e4058ca3f5b6ab1e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c4eb8fcf17ed7a2b29d22ee029477b41c6587570d7a5dc455ebc922952bcd65|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a96e8ce3515eed5c3b6fb390a06a1387432ecf8434ed6882865bbd904f875a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cad5a668e856e1b088132bbbd3f71b4dcd7877d15e692f0c3378169d9aef7ba9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c2e474b5af32a25dcb2e34062e9430eb66a3cfc4d5d2f41d01d68dbe75b44899|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-43543ae77282d40ca30e05da32e60f6e07a81878a295b011c27a868016ff01e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93ef0b81a362f6833ae8e04e5baeb8799d5decccd73df4ea0253fcb02bcdc195|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22654d5d5bda87b9739d9827cdb3662e88147cd11087ad93f55bd5e7601fe04d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83429c3707d241e566a5b67b0e95b325f9a1dec454011050add3ea8a737bd5a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f62a18f5d477c7f5a379e3cbc52842ba7a6eed25ac569c5f74b9494a5be7fcf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e4871419924fe5fee174ffe2a23381b5ef2aaf60bcee015ac7cf41addf240715|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3aaa8fcf3da12f57fdc85bdbd3dbfeabb7ac62bf6d577cb15793cca3a163e51|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-624acbeec4b972f631567cab0d9d7c3ff1d4546ff9ed3f893d2b712b769fa3e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d71cabfbe2e52400ba47cdd78c96461b3d86a4b883f593d589fa3500448d5db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e088cc515eb8ad150deb54d6e1e0e983681e6de6f68df15d3b74617b04ef1c10|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a530f63a49be4dc05854539fea3f4cb7175302ba325772287670d78c8b9cd731|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-51a20b5d2bc042a40d283cf8fbd5c45c5666472c3c55b9039256f6490dfdd85e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b924f5989848cf058bb8c104984b50c787fe762b520e6528433cb8cb752c2455|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc5605205e250bfcf5cf3014fa0d2dd178b4fbf9e2095c3383b71a9fa786c731|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07f8c195084775978b076646aad25167b97f4dda10a64efc660624ab33c4bb15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d5e44efc5949eca36de147b496f0609221219ccf2bf2f3d94a516e26aa6a07a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e68a0412973cc7844b8e16f35507f73b06a73b8340aea6e539d79369b3fda6c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ae70bca01cad72b18d714e3269cf759871667c9c305ae61c21b8a7ea16133f0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-867d7378e2a003b90ebec344aed87df3bbb03fd07fe962c4ec409ad9d9e36bde|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2682b147ace96ab00a710df38ce5d80d0b205ee8edf3c969ea7b1e94efc5eb2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-acc1b1de5ffa142d64a3f88836fcfddccdd709ac49c5df5245a6ac6cd6e95c02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-08a7f4bcb56478a938d7e1550f71e7616ef209424b704f1b9e5369a3f3202281|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fbb0edc38dc15166f28837f1014303095fcfbc64b70b9b79ec0c439f45a9b8b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0075a6c518839ee6181e0843536730e5666bf8a0b14d2ac36e302cf295e0467f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-066ec1e08f002ce9c1ee714661f2cfec0c44e3bdd66c80d019142603fb554f0c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25186f7b1f32e6e2ab7fff22a9ae8cef1e29b9846f11aa2f0250184b79dd18b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b337dde60a09d47ac53326211462b3aeac05a140091bd7b80967cf1b544f47a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02d7a50aca253d92df3134ec72807bd1b5c6e2a558eb6cd29467a999302ac9f4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4da88047347059044c53522b46c801aa27bd12a3f723f0eef697f712790d96c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98047a7f9c402fdecaf8abc9465e7cd667ae07231b67d4dd853dd4074f4726d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4dcb5de41d425bfa89286423b12ca4bf35bdcd540abeb4000a79e2dfd6db25b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-33fe2050fc3847c7c97191169034bec48cd12d4bcaaf1a06dd00baa49aa4c6a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce1eb59b9522ec1ea3b1fccba98e5f8ba04ce5ae0b1c54331873653339219be2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8a9a06c350efb3e9bd53dd884b07eabceb89d2278aacf73f01e8ccf784b739ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b0de74af054a0430e36edcf79d915671cfa09fdff5fad29b6cc1a03b2d12516|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59f310d91f8d5e9da99010242eef8b8474e4093d206ad0ebc0ac702ed8307aca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3fd55d0f3b55b8a7020921cf2d94f09abd0d47e21373e091f86bfb78ebdceefa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4bbaac2feb97beb725c50987e6d235d68f1dde428271f1eb0af99cf108199354|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ee8fe631f2d882dc8afa83aac9a1de627d0a9546944014e74bd5485c50a634b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-306a28e2a30fde7a04f3f445457b955ace2caab54eba9ed9c219ab0a0116d073|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d96378221f6d7016f2e8dec66b613023baaf833a5e8a18bacea8563f0ca3cf01|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db6be97cfe6e264ed8c27975fa83f4c566f67e4745e998beb624b6358efcd2be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa79275b4cc4eb267380fddb4eeec2635f21223eaf4b172c8f36566144f665b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c0da8b34852450f09c8c3692769ea474802f45272739bec64dbbca22691f911|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-323ab806b92a76ff405b515f046a7dce0f7d90970b6651bf1ad642921d79e023|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7001f19fc3e12a93d8940447697fc67520457a8b421bd33d962473163b134665|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5e0f91adae73934360b9b323717d9cf8cf40f07660324df2c645a885258bf295|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-53b2c401eb89a187c7826ba63c3e62251aa03a46d92015e4a214106591412735|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d27a84966f6856925eaf85f6cefa26cdfcd057ce5fb0921ac7563eb2f9f67756|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de938d69678548579df88dda7a6316c31296382187a6174667314ba6943210cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e9960eca8df9c14140c90630c6e9a774190b7c8c95cbf5955d0a1f28c76a41a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fed90d77dcaaf1e7a125e90bb7a56416da5cb24e027812266a5ff6ea88dde0ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-45234ddb550bf8e128fab585c8f4a52fce5a388ededd66a6bda6de2375318657|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc8439c93a5fab090ff43c78edab72cc82df057ac4ddb34c45d0747568651a89|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce61e78f23d9ee505bfb7c0910e5ee42b2e4010f21efab5a58eb692201033f7f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8c3b07155055f2335b5e36e136c1cf551c07519af628291eae107e4cca6e8d0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e2de7437ac33b9730be8aac9748c0e8be4f436466b7d2fa373f9b25092b2c74|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-175d1ba19c32db0f0caa96561e3a71c17487d6dedfdf42f08bbbdf69d37d0854|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0780c68e883e0142fd987210d6087c84db9ff4d9fc402f772d6b0a26ed8183a2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69db517dbbaa42ebb76eb8f10e711d463df4f90fe835fe1f5eba90db66a3c0d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-548b07d544e0a82223026ee732508cf7e4906aace0449d42017dcf28f20416b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e075a57b89e81f8d470d3c6245c7e40c35ef2ffa8ed654707166658083b77f31|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93829e1bd73e1fe2f5360e423c85483bd805817f15002ca939bceebdf742df15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-35e1e6087645e83cb5b16cc5a89400f2da0ab537f69af5a036327cc52fc293c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3bfa223b61a9e0464722f5ff567421cf4beae87e3eeea8567ef6e9ba4a2111d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed0b9bcc154489cc2ffc68c68779b310b7900b29d79c96771273e3d24a939dfb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-029cae44900502b70a0ede8daa042a22689404b457d1c9e943514fee00833661|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae31ae8d667bea59bb5fceead20509d5e29ad97be68f1adc27163e4ddbf19631|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0d2ce261565bd98a07e32b5fc0a4728a493b0ab4eff0dcec367a468d16a899ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f57edebaa855e6d742ca8f7c052b68ca685dc49161bfa83cb3554d82ea10825|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b384b48431da0d98377e5c69298acb09657017f2f34af10919e9263561d029f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a0501a39fa8a162e0dd68e6e0fa628a490e4870f57d995a84651e57c1a64adc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12b222c0c345760740a904ecc2c2e812ac2a6b706a417634bbfb0c2cdefecc8d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1161a4e670b313cb3131811a9b666fee5c49f697f6e7f140cfb29a62535c9fb9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-42105eddb17ab5562b0df2bf027bd88c2a551251cffc33d23004fe566581033f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd91e36ba064fed72da074fb3beb5bf1053b7e7a481848ea480ada2b2ee96e6c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00a23a05252ad8170b40c7fdf718e68dc539e73fd3512762048118872a7a0cd1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d8ecb6caf37a5438cb64977dd9453c857b17ab44e11336e1dc56ca77f91cb460|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-443e326c505fd4352bffb93cbc11fcbff8ad19747d36892af1454397ec67d8e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4dda171a179ed43c0503e337e73e7825666af91820e7e47979b104b91965063b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e248d8a07cec35dbc38cba5da87766c2ebfcbb63ae5f1543cf6b3403736da97|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-541150bb2c088f94432ebf9c1e8f4e356e7149e00e4cb0dc07e9e32305ce079a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5d6189b47272238154f37a9b0fea32d9613f58b08bcf2897afc87f00bba6e46|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1bc29ad216d38a6556740201c23750dcaffa5f54a65d16fd4c12fa6900878f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59436bba2ce994e71e4ba0d858d324dabadde6a25e1afe1bc95df7c6597f2841|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9579f825b54dfa58d2ed5f3a669d923fd636860297e637dc9e802fb2cf03e9de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83cbd58b0da8258f57ad8db39b4c7ecadad93bfdd16852551b4fd9a08e6c4d4a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f06b9e7ce559751670ac1bd05efbf3f1f04d3d90fb224331af43aaa4cfadafda|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6098645fbb5f415ced5d5102254eab6ec35589d90ad0b4214a173f843353c163|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c429fb63c2e11700b71d6e672286ccf334ec49a2b6dacec6f9a20f9489bec200|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-067547a86a2cbca6fe7814ed239b21306bfaca8e25d1e991a7392e5a50caf7da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-491364c9425ef690602265dbcfa24ae658400711255146051a0684bc1acee60c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-89cb63008fa81b176698140b272358e2b197f9b10643b9f6a4e8314af1986c07|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8dfd26b832aab71485c210d9dc18ab075d34661b0afad3ae575c149fd21d815|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32ae7d68b437b7319dde6d2919b0816f12f26113aefc2720c3fc704258af5c93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff250559520a45beee976e34e00e4c47e7c9624a0e0cbb12471a897ddd873715|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-000c0e5b2d1b3bd823b58dc65c120d77b44528928c70fcc74d7e73899d7c3314|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea4bf15d23b8d2f0ac782b969decb17d8161bf594464a5ea2c5db1a648f5319e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8df58048da9ca3ec50342399d417b6ade329bfca01425573b84afcb1ed358bb3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b27361faf77ffd1fe0972b09da592213896c425ff05f0a7d11fe61c068e3719a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9c258a5286aae6ae91722151418d9ec116306aa20240b67e4dc96c1583fb450|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eb43e2e825c4fceb50cda89eab9baf53e8a0c0ae2f952ba38c553fb7569c3d10|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e0a5d91a026ef034efdb895e8985a28987db16b36c13f87a367a5d18669c73c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-adcd819e342909d02f418be31b19ed8b130d4df22562f3743121b963575df6e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0867dcb3d8b62d795e6f6f682e15e68978df726113e11a4b8fd71837d9fcd5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5d22b365cea9f104d79bdf262a741428fd5f3700e307acb338541718ee4a9d9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ea8e1b196e62d38a8008d9d1cbf818282752545509b5897edd5a237e6d69239|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2059e27edb819457a3ee7ad8b11934a34a59cee0ad340c800256b69b66e76309|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12c4777ba8f4bfa2d358a6768686cb593809fec07c12a6f31a28cea06511fcf3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e62ff3121920962cf957145dce00a93f8c620da79236ceee6df2846b1fcd918a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-972446d4ab3754201ad7860527ac21e89f07a43ca7d28f4449ddda8940320d73|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d21612c770c635b3b9e0baeae31ba82845c3ff9c0b2605406fa4477f75f9dc4f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b2de77d19deefad76c92c7339d318cbe0e5cd1d3d710c361be8c32e60bcb8d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37d05d98effe58943fab22f3aa6b0fb7e0089e97296049e21697d7d24e1e827c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f443941bb94c6a13bf78f783b89650dc3e17deaad41ea71fcbc65e5ad95c777|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da52c126234c2a96417692598478b8b1d0cff638ac410f7b21f7cecc0c5795a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6415f3deffc2339b048c2b4031c2ca71c93bd2ee4059c7851a69e38ef2f84ee8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-151d523afec0b86ee01c8a8ebca78145e291b0e6bf6094f650af264a2ee74e3f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-143e5ea4fd648fb8398237745531b275dfa09f02b11472a1eef2e1e9728c48e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e336adffa50ffdc7e7c11193d36e0826290888198229bd93426f804579a3752|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bbbecba63925c15c15a7e70e73db94ab88a96f83b0cd51ab07c8d8d446c971d2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-997d1265cad84e50e7ee144d38fb5d58d9b2188dc71c9c3bc0648ec7716857e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5b0aa7b26836d16e9521879c61dee236b28e94164dc3e631826e806d112680a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f198432f956dac047198e0dc343694ee41254248e80860faa57750ecbff9881|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bbd9ab57fdb29e1cd0da7fd00b8e45d81326a12e770e9e83e987d68249819fc3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f60b9043135482d1dbb4fcd1dd37b9481985b990f78a670586af423dda9bf04|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec25de9bcde8df7fbb5992e03a25812cf9cb3852d4ddee80f8748585cab0fd49|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db365f7fb281dd97f485701e3b6b42c001685fb7ac971c1ba4dd470869957f8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-746da52e3aabbe0fa5c80905edac166a2770ca27244d935cb82bec59e0be2d9b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa18d650d8c1ac7c06183f7827231534e663f56c8409a225291440df2266d786|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a8a8d1a4199e237dce8271b4eea17ee34a44e02225b15947846c2318a8719ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d14bddeafb506c636c851c7db835eb4b2b5cac9a3af98d220903529ae10463d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5237bcf802cae126bf4d8415452729fb85e6e7642821b6e7f31c3b330130bbeb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f5c3dec43e7e4101ca4b3544d2fc83165750a80f28e1404c2de7e5b999aa2c2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-48a3d4f701de8a34abcfe0865ed1ec18acccf1c93474ca690817948b9d853dac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60416f4231865a20569f7a4772182a910acbbb611a14cd034ace79fdc344970a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80491a813c9520441b449868dfbbc55425bd45497ef9d55048c50821a8e22cb2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0feb918d3718da02d59fcb74d570d8a1fd79743b88d03c43d87656fdd1d93dd2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b2d88e13f3705eac8e14ed11e4a69f18ef1a8a8e3b4c40b63375a98ad2a26e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5124e98c8741596361746c1b0934884bb7d731f6c28263134cfb6ce736812c66|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-46686840b6316c97361a70583c4587b5522573627bf7c6b0ad63912f9e31c882|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74da9884309977d6fe8e15eaccaf86f8c8b45418c50ed4a5451852647e7f0ce0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ed4007121b611b41d895a8a8035f556a6b7498acf844eee6a6caafdeb601a67|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d54541289a188083c81df42df5a43a69d3fe6110c9d6ece322a2c9d28eb00ebb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4b784c5ca891a3b514d2a950f4eb200eb05600388b0573a6e03dd0a5baf21a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-569caecc5b3b2220979ee1b4a321fa5f17c9ea20deee867fb9462878956bc154|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5640fadc8fd992f6dd3e37928db9ca53a30b71b8377ecab26ea69cae0d4bc455|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dbc1c31bea0bba683b80ce2154963bc56d3dae9b03d484e9d1d62f35dd716843|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1cfe7acc86937edb207cd440c94a7d5b3594d3fd29d9b6063f919a8663655d11|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c60973c9a178afb6fdcc00a232e4d3f48ab82618eff01189cd1b5c11050d34f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e08438824ae91f49b714e8a471d6db6d3160bfd9865d0c1a901878471d32859|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ba5b3caf3c95b40721ac13323be60c7707420894852dd1e8dbb0fa4cf5e78172|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-abde0d6ea002902644aa2dc066ebcd758a12ad65e616f3752eb9491c71283ad1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f60caab7e11a0801e61926a82bc518ad115c95afbd29ce339d15f98ab8456a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2001fcad0ccc773aaf0a9a639f95fb3324d95dca7f99e6c2a67a975ca50eda49|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea165687d00b1342526ba328a08e7c5758ed141cf85dd7e831a0355acb826365|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1923195f4a987098a1196d946b7914bd9bc06102f2c9fb2aaa2fdc0fcff4410|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fcacbc169337f45704189fe6387fd1a53e0ac70fcc85475e79cfa27c21810946|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bcc71f8d611f9c7ecd29d8674f32524c00ac37233b58eafa0dcc8bfef9fa7841|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6809dfdfab9482cd59ea1651d18ea320df1f6088a278eda438e84743fe0146f4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36ae84a695e4dbbb3aa383de8ae6758039972415cf4431e98ced9425da1e94d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e59b857f3f9100bcd7467cb53f186be245de09e200fbb503f1a3b3c40fdaf8d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69ca646831f1b885e5696eb5439eecc271a9e09ccd42abf581e1820e406d5dc8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-26d37c97157ec61b072a02f07ca090038f26a1f3effb0ecf9949f6ecbe84b166|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b00eefc905e4ee32c6c3ec8944740c62f461f85e5eb60083b1b11d54cb500f7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db9af46cb8fcbb177ce9f26d344a0a9cb49a6b5c172637d382aaacb569cc752f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79eb70c47033c9b519c3f79d09592af344fb9e11a4f575202e3adcf9ee675925|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c34f273fbb7ef4b1cb4cd5a16a9dbe0212daa2f90b9a4742f2852544ebbccdc4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b859ae3f85dfda8d8108fb655d04b779c20530a64d0407cbe8aff1ab13470fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0f18cb549278f487586bfe069e787b4325c25e35bad133433dd000473944335|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b423fea4f07e1375d4471d6bbb77584534009e1c3b30c54821fa3589c5c5491|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-138a4cf6a507d3b5f588240714d071753f1230bdaf14d0199afafe10d8cc500b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca60ecec8ead2ac15b1d9b17d9e3bcccb7e373da3911a615cc1c29782143e6e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5ce924c659e96d989bc201feb8a387ccba9dc416d167cd477426d4519c58a5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f670d472a06a71ec666cf0e9786fff84dbc7fb618a57f83c04a9b53dcd75fcd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-edd00a9d233a338e922da65a4406109a25861359e7293260409e3ba05fad1fcb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2bc8976aab0b5a5910fa37b75be7724f7cde8699fe73c4c6de9f3f7c7a31e454|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b7e0104ab598e695088ae3b937cd332bc37f5cc15559f5f79211a6cf1970ee4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c47675c027874e8a496d788fc2ce637b6e3383ab864c96893b8c1bb3508954f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55b7868f9a55e50fbbfc3306b135d40604cf82619e99e95450981c119e062d9a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5250dc16160dba045c919bd647bef2a5c8249da5cb66a3829a23407e9e9ad812|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5709f003ca082f6ed7098de58c4e3cf5ac95e7cef847389099a7e1b0b9e7f7d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a66616a7bb0b94826f28eb33f8e0fc102a7258ef270b9c697f1c442052011a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-197911b69ee0e9a2733435e7ccb9a6f8c2f2394367e05870c3f007bd1149bff7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d03346f0bbca666a1a79b23b21e01d08c74cbe6b538f2b21bfcaebea0d7fe686|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-440906d76b53abe52261adf0f05fc1f04915001ac6079362457c0a6202210757|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fee0ff537a5de3cc25638263497c3b852aab8fe5491b00716ea43843af037176|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b6a6709308ed8d23ca2a11bd84f7be2479cfc4fe3db84f62d2741f03c946cacd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c54d0a77c131e0a7ebc9481cbe90cd0857193529e4523a54cb8d9780812e96de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fbfd77f00a70265f9bd0bde8a5e43af2fab9375f9955e89e4159cdde0dcb871|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c108817f7b017187f5bd3fad1223b47ae9d94c7d58ce6f041f32be57287d0d79|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fbf91676a21c0954914fc786908ae8e4b8b89648816cebc4dce4f5cabbc56e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d17e0e1a70fd1ad4ae4ff0f54b1fe4fd8d906059f15d6bd6dc3f1f312a7d844|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f902348e074bab295125a8eb403697298364ac527a13450d03c8e50a44e4df3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da196ea68941be69123eb034dcb9b02d408d06db8dbb15da6ea582df34c71bfc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85def2b65537719b7695a97e5d2dd106854e3d58630c5cb2d929ee9dc110eac7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93b7efad5d07721cb37a76d905c9f7288f567b0bfe9b339b1ca2378b7d479330|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1961e0da55f92809d996a5df53a5b70ec86b6656d64db3ad36827bf74284060d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cc129d1dbce5a438a0be4fe6a126f91c90b42e6ec7f7225728eda7cbeb14cf6e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d843543f7c8325ff9639c801aa28230bbd4f2775b3c894b06e4de4f52658377|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a42b75b1676fcd2dcfc0c5cf54e52e0d816e9c3ac38e2a1c9c641f4d9b26ff15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-879dc1ecff90e99f38a87f1fae766bd677436b0bbca2bc95b7f779f411121dd3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae6e814fdc7125a83e1ece5bc4a9a1e9b6d834b25c64613da84160cfc232d1cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e04c959cd708ed4d5d319c6cee1a615649858fc674b2894467509fa90b3b3196|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0dd82f802f36f46b12c4edf218dfea607f618cd46a0c1cc97d7c8a174bcfebab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3768293e2deb4c69af1073adc6fef6de4c03c16b91915a40d8290a49777ebd68|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-052b5152e5a6018ddb489fd4f2fe5822a2c8cb26482b458dc251af1ec2082c6a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a03f0b1ac61ea25d7d0d804f29e73238a388aabd6cf5675cb08b55538c792fe1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f2c5f995369b20c3ac42a1120e62af131bd64b4faeb52a39257a8d235329e77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0fa212d860e45c32244f4824874bd9b7f564462d1492a7148e61de68fb405ac1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-82d16c8f59ad5b36507fa575357e906dc451e1837cec585c763b2510c81f88e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-700dae7f6b8437d68bdfde3d8444e4b375f3174c00ab9c2f959f229b9e002f1a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7cbbbb0433652d4bbca7b3e069b9f3c1e214ca20777dd33e2515e6e65f5dcb76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1007d01b450b90e427a2c33b0d042ebdbf1bf8174d1adf01e5787ee4dbc0ba10|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a327b25ac75544d81966df043e174aac623cf20321402c7ee7397f63ae51190f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1fcaf46697351052d92decb838ce71aa1c7a4eefb4b165b05a7ad912de080216|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52de4f3bd6339dd7aed8782fba0c2e76cdb1e5b1a0672d6a6e5197d1daede22f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84447885d252b7a6564c28f1b9ed45d3ec928a6797c1eed905227a001136bc2a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c896b97e83147bc8616b2f43ffc3ef6e4157fa2226da71a7aa3dbe267c746f3c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4ecf07ea8748d5222388d53dabbb50bc995d51dc2019eb4f1bcbdac03f2ba15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7620bd5e2b524bcb8cb7a38a62b8409e352dfcd4b72d19a8c50e4ad8cd773252|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4309da791f8c4aacf65359c50dcef244860bde6cc5a85c248dd60481c9961e64|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dac2af2e61ecf0157e53668801e132b56544ce3f4f7eb5a5ab368a57d1f8973|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0236f2d5d62f29cb9434cfe8659b6277b9d6eaa386416b5f17c7ddfeaf520fbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1570b1c79362e3f6072c6ae9d63516cb1e3aa75c169de47794a0bcdf427cea4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f7ad569c1388617ce7e29197ecdf185352c16e70fdd2ca51c6bc570c2d33471|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f8034ead9b71756b0d1473ff541a0ab5ca38a34f9d29281afbc80b0a66726fc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-90681fb75999f363a8a0133c31690aeb64c8f72228fc12ad651b870d575d93df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30a9835918a8a1cd2ebc1c7939f02214e27a32ecb1e2bd117ec46082f78475f0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6124f255d8a67a41f53ef6acbb1888e77f8bfba993e5ec86b881400218653126|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bccb3150445add607c74e4d174098f3f8828c525104034818ea94bf28b71818d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-26a2bcd2e6bee7cdf9ba22073942ced4f8abe7be424d44817f2955c376ab83a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc8820f017ae43233cca8756e0c6c6aec461ca1baf154b45a71701374e199277|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eb082222b87fb9ec10b6fb18a207b1e8393151a6a6c85773b8f6bac8abfbf0cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38ccbad88c3b7ab25dde96470054f9567a73e15e5a0cf3adda2138f2bdee962a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c7d3c56d7bdba8633b3ed882b350dd1309572e278d4faa0076483b358ec885d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4d84da22b7b5f26db9cfbc86d52f031bcf00fe1e4470f23dd90fc740821f4a13|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d7fe7fcb362812df7db5d92de0463deb9d991436d25b74e188343b513f113763|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af73191748ff5a58326e216efd6cc97310631327e1ea7caecde3faf9c90aa65e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7648d7768af628be8454141db6e0e2a041ff9263c8f2b23a121f6234f54c6b98|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da5c6759d1a23cf81202c0c8a560caa3e53fe743b1198cdbf0474d3eeac2dae0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-601480454234aa3f9a0839dbc1b51a56a840ea785f1605d4ce302933246640a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf7351cdbee34eb19bae6b822d348e6eada8d5b2eac10d5d16b679254c83dfff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6712835cb380f52ff047a1b764b5958581b35a9b173ded3e1c18d53e946be119|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0ca385812d9258cc0ddbb41bd64dc838df3eea8cd88498d8ef230f7b2965dde|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e6edaff4299d45ef4aa3273609382a6b6cbc23ccd11c5bd9bd2723449b36fb50|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-75a44c8dd1254ac2b8f3e19e353ebf7cfc0410b12bc3d9075e5dee3ce24ea08d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05b201a697a22379c121df03448c7fa47544b17d43022e284e6fddb5a94b23e8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8ed606892d1050e40db7ecb683d51ed12b9b9cfb44d49adb7a97fca0ba56970|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6dd8ab1c5a713bb94fdf225be7b70f57b84272853802c29b45e7367df92656e6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-006b916fe29055d1c65a5fa4fb23f5b6cbf8162cf1bb0b41546aaaa273d50cd7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2cfe89a7ed998d2979922e0f3ae9f03575c07c665525df5c0afd9aa3db232da3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-87e81140639e1e6fe30285129c44fb70e7118462340ba78214b7b03a68ade6d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9921996270d64f2a589fe6fd715f8262f64f2ebd9e7a27e2503a98631131889f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cfe554b92b5c287961991965ef2258e0d2ba45b68fbdc0eb5521fe8ee8307858|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d921778e558eb7c3137712451b47db004ba670f93f679846f19373d8ded3f4ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-728f3b58716821f013fa8e59977eef7340e5195f258520768bbc85c175117e6c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36806d4763f7a1bf7580542a8a5a77e39b77ec43f62705187a993df91fd3375e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30ec7149c42ca34d10741990a32cc2a63e7829ae4df111a8569643960feeeaf7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c53961e3aa43b9b04d725798532865b01c1b95fdddace26cbe3e49d755424d9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e8d867590560ba8d015e8fc186034a17028eccdad153c5465b52d99cbfd1939|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b727f0aef96500dcd03b57506962e65f71b581483c972092b756929d46a52855|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b4147da1920d6b403c3067cbcbb14860ce9d95b8ae3a50d7ac9b2ab5d8fe7b9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4831a9d7276e501b7d3ffe570bdfa6b239aa6263232aac49bb92f0adf15bc0ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d32e1bccf993bd390e94963273b43b265bbc71c55de3eca8b60bed91c809b02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c3bb20530eeb612d06fb51d94187df35113e2e59b6e1983e94512db9753d81c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8dcf0e440b407c9e1dd6050d859a64a6890dff029726f4366c75f321cedc8fb9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22372831cd2d5e8addb6ce9c8f0eeafa9bbbb2b168bbde2a7d2693cc26178342|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aaac40dfca4aead8c1d22888892e58b5f8ef90352194ad9a728560a943005b24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c15454cfa3b96d401d44539cf5ceb4e5513bb664bfcb715082156c2f2375e318|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7002e45b037ecff53e2b906c7eec90dcb5ac880e32a5f1f015ae1b2a03d65330|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-945fd9097fb5824f8e4bb45eb27e2b2e4612739045d8ea1fd9860bb23c55f077|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc3aed5833c16e87782babcd12cc9db05b4fceeb95e2b911afac07436a6413f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3fd25d7d37f82042a5e185cc3f83ab49257422c8829c000488845bd7d31c27a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60bbfdb36bd67b77287ca3baa5113eeddc37e138050b023b086e0a4a1e9505dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2c7eb667e6953a872551610603a44b1a6da7ebc147d8c88d788e255cee3a9e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f0b3e7ed9cc5cc611cc22c3b89d3af27dd87eac70896e3c6ece706abb29c7af|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c2bc2768777856c1593483e9f72c9598cfc400c94a83bc3e7b20583c67add56d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6335c34f1d5223ebeadcf905bf195a1b0d4573660a3eca5f547be3b9b8e3bbc0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d189dbcd3f6bfea5838840a34f9b96a116e0ee2368add2a9a9538ccfc9f0c96|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-58380fd9c72d0e9700f84ac1c4c99650affe5d870415951c06827f37a0cbe88a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4cc07f3839350e2ec126f8612b54b39c74aac7beb0ac46a1233580b715304d0c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12feadbfe4a919691d515d9ddf4094ccc7054119880bdcb9199d7aeccc065967|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-440404938cbbc06544a5de2999c630f7e268461805cc5bcd5e349ae9947e8a23|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80ee23be36dca38b12ba52d4b78c9a49d9f094ceb8465024f7bf5345e75fb9a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-212e5279f41fc3549e6a6dd9812fbeb42048f15d2cc462962599d29178419a71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e14a4853bd5235a27ea35032ab5207de7078aed2be6b40a0c809818a3eb5d67|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e9d299066edf85f91f73b065e8cac483ede66cd5ca41fe3ea6b6694c22b4888b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d51aabc96b0a953b500fdd8ed362f1917338636637d9edd1c0e564b318d18f85|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d98d7dac9bbb07980554f766626ca85c04e77df70be2d97ef42c7b259ae0c1b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b12b8c0bc30d4877be64fbdde57412c26a8989e3e8d7df6ec30d08cec06598f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5ffa6bb6a367220012b27ac4efb086e9df468c0a4c01107a31063f9fe4400b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d8755b8f7d06063aaeb1d3da4ac65cf8983ebfa1e58a2e4fb4ed34760743ef3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-317ad85b0ca9e94175e524a9b194e328d5e6d245dd7222ec38629f861d0e2c8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d0e3eb9b9f744a2bd3bb42275de24e08d6937dab4a9b1cd8e5f4c71b70d368a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ea8240be6f935e0b894f03d5a4837404d955820ed59cb63123d06543e258f31|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e4581c629309936c5dbf3498497e25aded225546e46721a011cefdf4dbc3e64c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98a4817183c2591eb1fafc0f190d3973f1c3744f41d745e35ea59a67271c391b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b5f734d1b84d584b5dc24c621013ed8335bbea8bcc2adf125a1b238e1175481|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5be3c3d841395319476b3833a95444a2b88301cc030974965934ed75eb07ee27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80e4c53171a80bfa90debcb1477159ea36157a9a3bcac493ceda74700ea9ecfc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-317a25569400c28b593e62af8f0d20110402f4fd358b06534e153fb56880e21b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1774ccb9c14cd43b2b3c60f334bef513c2d789db89b4185239f846d6bab5040b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-01a36e7caea3e39ce3ed24d283ba6c3bf755946f35fa21ad5e6df6fb7dbd8c06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7e346709d3287a48523b0875cdfd3fc82c7932e02f82b366b29690df0d3de17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39821efce5fe10a0fa5a617dd293cdbec2e9abcb85b3f4bbf6d116f5c193418b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f64e6bc07c38b39fd36fa696a6b02ba7364cedf939cfbbb4cf90e6447f1f3e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-03b7bc8e36d1cc237a03a4771433571e5e2f91d3254857859bf69672cfcf13e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-720fd16f3793733a53bda692fd203f1a18cfa051644f8c6faf56172345fd675d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93d98cf4617574e514dcd1d08c4945795ec22f84f9372f92c906e7d4611e6f77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e16b251f5402320fd4febf968705137aac368f85ec54cde8711e5fb08b505e8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b09b6a663866b2aaec5ccde2665664df5153dfef56e90564ca705bbbd8ab08c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2931256afa776c402de04cd1a6768c85edc46bf45bf739e1862bb9185bc1a90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b62c8eb22fc5b875481a8c1b889e14066a2ce9a65ff7f7c1f100cee11c6924d2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-285a5a06f6af202e104783eb0ae0de197d1d957f8894b8e6543a2ce679e87658|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a0c99fb0a468972a73ebb8bb1bc66e780398cf39bf95a314ff0e903c3585fce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b929c9198f63d13dcfdcb2cfbddd6d1c063783d29dc7689c8a4094fb997e155|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a35d49546904c987d2859b2cb4fd32be7120726fb87f8122f28619c611676bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a18ccd517eabd8d77ad8f00a113df1da36d6774628181e7f9f6a8b64750313b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea5d018ca6d71d738908c27a6b2a12ae1dfde1aaf9db876a4849cebd48fb5fc1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c426278d03659c90856d5d2d1ebde17f798e9478038bb787795f398561f9ef9d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d989cb6ba448614a745bf7e57537c64396ab28ce789edcc1101c4477e439a1a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1c5c433631c3da0f1e943e14dde9807966d5c78837abe40ba4bae8221098b256|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f441301a3243942fbda363df559b78780412223b22ae242dae289d177a3fd7ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65c4d6cf260394e0040bc1b541b35a184d64b2e4f4004a6b32c9d92b8c310b2d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da5010da08de477b2660927d4b306198f388d4a96e82eb1095d2c0f0b54ad92f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b528d6b060c29922e5c2d79f145da8748940368eb1cc67831847b4399adcf26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-493b1dc3c5887f063a37507d413608a9b549ded0ae14600c1be2d84be2a8297f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7807f2060ab599d38c13217e46179820433e06e937310405b18ff816cd2f240|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-450474a9c2e9e5586b6ae87be79e99d987b50673812482c42e3aeaf499027386|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3288d6f5ad7baf4c61e8b17ddab0691814a7a50f2da61e1c5d488e3d731d6b35|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15851023bdeb23f15a70d5cf3a168c90bbb8052a26b2002d20651254ac01cd14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f4e307b2e8b737e376f806018b38214ea611fa424c0d9a54762e86024efdf89|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12e9db2b2ea0d8a061f62d0f0eeb09695eed404a53d78125b043babbfd6a3e99|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b35011df62bb2b0e380e6639a8c394d7c25fe8ab71034ac307d0f15197137bd0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8550160fc5e70bd51c5f19da87a5a864b4cc19b317404bac26884a5633a678f5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe3abc1553e92f71bb856c2311def00b7cfda8b0d7203cf7189479b65b8596e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-418da4688db345a253b1d0375f8cd64be67ccd16ac82ae1a48d7e2bb1b0373be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-380566b8c02f58c035d1b861c74bf3138bdb439607975ffa9373cad049b66241|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-44c84e5bddfd9b435b15c4f169d068fe2a39e81c44f3ac1dea4cc971e3d4bc8a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9d5111e25e182cdfa4d1cb0d1bb72a1d06f0ce5c0f2813e6ea249fd5527e5dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83737470e1a10149fc8de8577a00b9e1cd1e8d67ab583b0835e63e642f38eb8b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac97c8333e388ef6d2ddf7026c0bcaffe1d3e1d2bcc1b1fe222d2cdf78bed4e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-532fcc625d692ea0173dc7e975ed16cbc21b3dff6648272fa5d69c0d017758af|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0729e2eb2e8f0aa7b83945d0015adf97196ea2547b40908d34ca19dd981292d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ecc7e6f970cc20226fc8b1e0498f67f521f8264098e30629200e47de69ef1bce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4492afb0fcf096fa6247fb4051befd942d1bd7e69995ac0602763826f478a6b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02df5d035eb4dc637f36d4781321abb2e7695c869d6eab7d6e6c92553324855f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3d8a7393b1b4ecffeab2d2edc5186d853b49704302eca93919397fa60a96cfe2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15aba23c8d1256f6a6d42f5b6193ed3b478a1e04db64c73ab6c69f39c01addd5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-81c6b21658601f2e58b9cfeb0c050e212fb17e4c427638e4fb727f15d3f7c2a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1bfd0711357e1020e83a63fb1097e11cb8da044440f5d22b38c7fa5689bdd0dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1129ead003eefc9501695248aac1e386ec191e5cff7d22c6da07dcc9a2496b46|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70d296ff50b71e33018453b69cb0c1643fef3b25b862d5114f209e8606120683|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd976a7e821d1e51a886b8989f874edc29d58a0d93d32b3e07d4f99ddb763098|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-58c7af72505b5a37ab9d8a76b80eae0976213aae78f596617f8e0083e397c247|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79e7842d1afd4471a1f46e284f16a77ebaa8f53aa997171b07d031b923f77f9b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed309e4576c2699eeceabc6afd64f2278a53140f66d151f535c6948ac71efa35|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-441ef53cded5a3d5eeab2956a855a33a4e13cc58a96573883632165e1e1cc28e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a08f0c947983257ffceb89b5843abef41aecfb9f9231cce8274e731dababe7eb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-759accabf939f933e6c3a4062aadc8c2b53c9a8adb749d1d43a3c7ee7edc62d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11cced32f1f3491769949b3d58c086905119a9bda007be1dafee18f77aebe67f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-131602771746fd1576f5a108be536b158281aa27803353534937d35f8b0a7a77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-20912bd86c88230e5e2c4ea59d2e352ec3af083631153040819c6976998b4553|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-594d0290256979417dde313338bd24434bc2a42f2c5bc1c4bdd9f4b68dbfd798|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c00ffd3f42c140fcf1aa1388bc95dc123b4252cda90e874d7f72d53ee506b873|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9348670e270c426ade5f8d45fa21866e7fcfca41e556c99861b4fb7db200c14e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec29c569e06d2fa10a01a3b0b17462140377bd63e477843a158c0602a249b661|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed28628c15d10911bbab989a3d925b2b1487852c16ce64807f37ebff7a9dbe94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9aa29f6f0382f782f946a46dcde30b6e3c898a4b36258e71896f0b900365215|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69e55c9ddb2466d32a9d28c9b591d1e6e97e5f1278e0e05a08cb2b59eac618e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52e83635b7d18042683bb82a74d93cd75e273b783449bfcb009fbdeef8ed5431|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eebfd2588978e3a66c663dd8314fb2a8cd6b3ee95f5281d3ec3010e8086995c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4c1a5ac3019a648796c426a97e45418860f49bf63c153134ea45d610f279ddc5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ebbfb93410aa9475c1c10a95e42e49bca3d5c6137f3e6afe8e1c85b8daccd9ce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8789657184fe27d58845182eba3f1a9f4794e8d0dd522e3640f6cbe2bfc12cb2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6f5517ef9ed5469ad71df2bc5fd2ab3f6c8fe9500605641b3f5f06888c90578|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5bbe788e54fe73495c066820fe9e60ed7ecd71cf5f12045278ce3d84f0b73523|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-739eea911094bad4a7b5fbc31a6a27941dc818f8acd5fefdf66bbf3440c8e772|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce1f78e978d1acb0b8b8ae80bfb20b359a732d2240bd0b981257858c764022cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d7c2314b8a13560e9827228247d1b4ad1e37d1a7e78b71dd132e8a43e4d86f44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec4439e6aed4ddc542b9027ca20d206c6e45dfe714be000a2a3360a8bca0b935|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f11035b399aa3187b6beae01a7f2452adde52098ad686d67a67c0eef8688d87c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7928718d31e52a38d2a28108f2a44595f423e63916dcada4ac9ed34121bcf359|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-72aee9861067c7fc0d7368ccc56cc39135c8ca99c77c6ed81022ee6da5131914|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac35c2b016d4543c931ac4f82fd39d3babf4b2e5b837b5056f519b1befc0387f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d94f1f8fde251ef9fd9d8f47794e6784f18f38bcf7ba2fafc64d8e9547a0a979|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a265c6fe43d6d9f5a87845c08cbff4ba22a39f5e6f6fdc4446872e3eefbaf3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b1a3d80420c86ca6ed85cb48233a1a998a139bde2940cf6840c946a29248106|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b6d89599cc06839e542bb76a1647153c92219bbdb934333205ac6ed2a5b07b76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0051dc1ecaedba514b94a61f75002df8634bbb53bcd2237ea0042ab236c22aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-86a6c64ad0232dcd59eee5f1ad7d2a43b559a19682f20f5a6a693fa1003c8a6f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ccb9193a1ff8d27566ecebd820354b8c60e732f1537ca6311437c19753509df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0a9950f300be7e36a0977f9b2c15191f6074d18ba56c8471677e222e0cfd7f3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-696ce258519dcae7d33b8b7ae6769803104f10eec45a6d9786c94d8df6cc04bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a12f3262ff720145078a091140a03f13a6d4a570cb4fff881e70365e1a925076|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-138ca0c1aa86f645320385f535201dbfad886510889e4960416395f3a3a1064f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9975f9af2b17a3f74fc25f4bbe11c0e494c494c39409298f491c5db09adcede|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b43e9ae35020d627d5b5ee4203d78178683a935c75cdd58fb4692c9817a688d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cca405e4c5a10a51b58f6469b7b79306448fe4c9463630cecc69b22ad9d5d36c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f6116db3128cb123e5179d86ff229a3e61595af8803da57409a4985f88e912e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0a7065d05180b49adcd89fdc4c77773e9550368cbfd89cf2b62545a23c23b1b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2ec02878fbc6b377b6e1fab8cd35986be0a2ca66d9428286e6dcb7c693fb79d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0036c092e3b5310d60e548ae542aec5c949a92d0d2ad70e5b74ab0836a33c220|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a13c8ecf9c3f965403ec5867327b85f730b4c6ce5a39d451d5d1af6d3a44273|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d2c5f3b790d0575f5f57f06a2976d07f9ca26a6b19cb30b84707ad909828c5f9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3d9800d4340aa3406638887e6f33a92a7f09aaaed883647c3e9b7baf1ae36d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eca9678910a3ece63a91dca291a92010914a05c19cb2603f496fb55e1e29e5a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-951643122016f89adf77a1ce3ed469512ca4a8bad3adbf6eb1ee342dd1f15ff8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a0756b914695dc4457b40843a1d860fe1b289380664530524be5f41134864220|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db1da4c87acd9cb836a86c46744019c6d0532dcfc7d082785cfa261c8ac0aea9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7d0f789a83b13709cd7dbb0de9c32bbde7a2ca9f497c0f295ac64603cf974eb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-895b746c5bc7cad21e052b64b2fed9a92796e50c1811661775f3e24e716c318b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1ab0a5fc08603651f44cd4e7d2bc4255482063be4c566d1abb591e63c12cfa11|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7a709cd7247d16fd58f6a6327b1c7ff9733a9c82f2dbc5db4b10768b8a0a830d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9904a3afdbbd3941ba9ef05085f3bb9623dfc51f19230c4d15eaae7a2b5ceef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6955ca8404f35ff74938cb38c5f2d14f414d3643d8b178c99861a6ff94d6b696|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b9fd641d8534c91ddf0cfeebc109f90bda89f31d14b41345443b092402ef166|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a00e0b0bcc7139600cdb775be9eca26ee1f26c3bb65484482b2cc685bf8c199|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7abf9728f5bd96857979d527b18a0e854bd901ba18637c28e4e2012a48b8f57|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f03038312a475a46319ead910cdf698135f4454f78861b0728c01b4b04d98e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6470f0f816ab565443aa31c662d6c29db58bdc2585e268c13e2e014a9e0dfd64|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ef730a8b4aec90138c278579d19d52c9154d93b5a3c3b5ee9e912717c0f7e02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-652600715a3b75804e2056f498c06402c0481c7b48eccc4425802fd2cc38588c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6620db40e65ea5d0ac30725aed3872de419fe6ce405bfc0722aecb21e7325f45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7853196b411615d332415041c19966d71524803daa72eef6394fca0c500dbfbb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d542c337610d34b5ea75f7b9d3fc225f0627624971add6ff9865fb048b51cf94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da31a4a7b4b1c5441104ae04428c5fa264c2723cf7bbe3b22bc4df023dbe33ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b16d257f9e59f007428c8f5b755ce20b8bbdf4ad29b71314803e6420d9782a46|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07f8dc2d4d26ba002d70eee164d3629145dfb73419af911e229f57d0ec9d06bd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34f23bd8d3fee43e3ea9471bdf92c4238952dbc5706068f0d8a6238750e3200e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d409b0de1b21a50b6185dc600d2bfa2a1cc58a5db7907660694cca49f3d9511e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-460a7fcb7666e2a424899750ed6180b066cc35a3787e186af49a0ea3348aa8ce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8da35275081f5438f848f49683cf1f39e893ea3d4415fdd60b2a349fbaf805ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ebd25ae567ca3e10063c53200c779f52ad47386f999b3c37eb2e2e58782efa2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b605e1aba06d1f3ceda83fb1d0a950e1d9f3f79cd655f1b69f5f26ac767f8fc2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9f078a1517963cbf4b9f4d7e752125909f9d33c1eec2dcc78424496f794eee7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-700ad062d7d1fbbbd6ae09e7f7eb7cec1d053fb5a5358336ba6b7191b5855f3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c18df41a5b72a0b0a7dc713cc94055598e96c3c8aab8040be8350796bc1d9be2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1bcc6b5385190e9dffb6c05c5dba27590e94947147f5145bf236203550d16d2e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0bb2e81155415f2c9c4b01fae55f5148af83836c29cb80f36d15a2d93be05ee3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d275ba20998aeff92cc8a69bf6fd501f0c86580bc7700a4e1db9642e365dfb4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b48ddb17f42ef471ea30f26e70fc46032f195662ccd556d47c7b77c6b4a42165|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13cf58c1163d9cd7510282054c16ebc71c10510d0c0f1acb27a7cbe35c9ad8f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c70af0d4d4c975522c32588c849f125f54c95f11c4735da164ba986583a022e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc3bb26e08da30dcdaea26f1295360b0d197791988581fc6cb8b8de91f9a0401|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b1d13e38162d0d50907abecb100ee758718d776c929cb295af9cfa038233c62|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7a8a1b99e3fd41ff99199adc01effd623bd429584814e66bc7f439d42773f698|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98b0c6275c14426cf14b4d3818fd82e86ef160dcb4d8d5d0b947323570936c02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7787969a809bd528b2eee7d69ec3f250efaf0e42a6e91a2d6b4f9a41aac4ff7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec1423d3beb437167b1f1e2b1aa3028dd4ed3d12c27919bcb9859c5a507c0444|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ae4394f58ee7b55074ecc78efee2e98a24591bda9eed747c4482e54a3c42dea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2eb187d1f5e126b529a95f5647679972c27943780371fbea458aad40aabbb51|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5bd8d101023f221cc442aa320cec22df4261662ea90420057fa17aa24fad651d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd221ef258c564cf03e8c9218f4298bb7d315acfb3ea417fab7ff652c4af9bde|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84d71a7f491640efeca70cdc6c951ed31ed51dafcd354a0634e4b59bf5f74c92|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1306b2a796588df97ebe343f27cec9eeb50e98385e9e125bd4e0d6e244edc81|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a8943a134914a1172b04b8a80ec1eb01fd2db780628b04b469d616fb6cbd04d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c519ecf3d3b0c810c7daa339ed1060d9916c097d04ad78c1b177c4c465f0300|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
