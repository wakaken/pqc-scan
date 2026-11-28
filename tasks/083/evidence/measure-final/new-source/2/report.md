# PQC Migration Scan Report

Generated: 2026-11-24 18:36:20 +09:00 (UTC+09:00)

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
|observation-v1-c98204481e0694be888c50c3dc4e8b9ff52cfb67be5679015cd96cf83f0d0585|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-decc730f77789dce7824834ce0b93b194955697a873c0ced1d4d81012a39810d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c64d659d82bb90403c010ce5701203a78cf616b680367d4751b5a73badab844|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70b4856635f1867dba6da786d9174e500c639b683ce9fbe1cda2f506799cbfb9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-378696ca4083cdd7a13f4f5165863cf33aea634a65160f9d9dd90f9edea09eec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-911a037e66bb59aba77b762b39500a17b34eec8aa344556fd1dcbe2dac2c3db8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1338486c72d7cc76339f1d0e07fd86b821d1441909196d1b1f6a91b1eec98927|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aac65b5b3cd7213694dd1d3611c7280960cf4cacbdbf1bb6ea99165ccfc238cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb49715ebed82ec321f133142dcacea2faa66c528d5d337611d255cdae27f0b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a627cf8c6a4859014d0f821c13c35db278a3e212b1850f5d5a9f8dedb69cc7fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7486a65394c8f562441d0b63b67ce1060685f309dc800dddb43624f8e846fda1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e9a2519d69484157d1b53f9086ce475fc7be6c674e8110137b2247b7f0edfae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fadd1572440a9311cfaa5a096c2cf546fd14109d59d8ee4892d261ced81e0ada|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85a96aff69f61c7278678ce5605d165cb608a81263a45739990ea6698e7a0f3d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0626cb9713bf9cb7936e5105cb51a08678dacce37746ce2b55c499f4448e299b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f13e91ae657d9ed25eda8aabd9a798d67854b09e9bf1dbd825fc94c3fcb0126|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68027ffe21ae290ce1809021fc2ba6579d7c880591a519e6281bc9056c19d626|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-955e1d8016a1290dc0b592aece73eae47a20f1e1cf8968fceee183e833e5eff7|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-523a8a4c77583ce490daf90865526ac5a929d77e5921c0c4bd5998f03e1cdf06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76fef5513fec6ac08a9496888ff4d3b6ef0a7709fe62e27cf5232866616cf736|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d4f3206f905d25a5fb8c7534096607667542c6dfc07bede8fb7f10b79cbf57b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0b34b299cd57c3c5074850cde27132261778fa1f09941886c0a23a98af36b9e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0fdeee697b3d85b5f4dde844403e8d70459a1c176a364ea7c3837323ae141ee1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa96ae3d50978965f192d4c5dda1bdfd2948a51b6fa366fcb938331f1f75462d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c6d72aaa07a484177dc0c806c2d69468739e77a3a578219413ad558f7771b2c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f723558b990497dc8276cd5fca9e53db3d391ea51cb8bdcf9251ca55f3c07e7f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fdb8888b7eef8b8509b63f9ccb7b787168248fe329cf5c73ce6de0096fe2d49|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1cb2f4aec5dae780e48ffd6c7d9241909c1decf975826e0ebcd5d7f52058d4f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7f737506d86033ce6d2f6ebf959ddc671b938aadbe3cb4f5e4e0c90f9fcd245|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73f275b533a0d9c64be0f0180e7feea4bc4bf609c64429a5b776d957c4bebbfd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e408590fd611e0f7ba7a5a47fc25b312a4783a6ce58d2590583a6a69d4793f14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae23b45343197b123f48a2f1737951d1ae653749270b0ed911910d069c3c1752|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-43c8a922f9e4cce261c548b8e7e8c401a16ae746a5611375066f8ce211deb84a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3cfb4fd4a94d38bab42726d38bb04e29841a034f4b7f98bfdb98aa6439e5f5be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e46626b489d781a033efd2c819f042f41dfc710a68961df6f8edcdc324e4d59c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59e45c10b46ff75f325e1cebab996f78d86c07c5ba635476ea9ce82b21087a0f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61b69e4e0e7ea4506ba6acf337f17a525053a1e631e307251cf011ef0387bfdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e44ba33be7855fce7128797f196663c85dc23540d2619ed8b2c7bf670272b75d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d83fea2572b2968243776116ab032b94b5ec907867d3169a3d43f0df8c07665|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c4699eccdf122d1e560094c52ac6aebd40a65f9f930a88a5f360b688b002ee8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3383e98700475a3be9516188e845d67d60c82de6d38c65bb97c50ae7c8a41970|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-09b8613aae5707e9ae6dfa27af2e0a9d615ec320fb4f112d7e6cf7da77c6e46f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-977bf2ee90cb96289c5fbe63aec3246dcd0cc2291dd57cd34a209f53c2006fe5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c7d2f39ebd8dcc23a44e93bce711a5b3651ce4c412d7ac13f7fda56f7660e59b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b119baa3d23becb2a672724d075c3375b11e93494e7d243602c39065e4b4e908|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93dfb3db039a6bd4d9dc0d89f8568c894786099c8f85d9741f144970d0287036|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7651254cde92907ce8a43cd335ef0f54b85d174ddba61ee669a68d717f57ddad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27e2bdc9c131dd4b2f02d85db77d5dea759ec0ec9647494fbe222642adf02503|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30194ad72dfd27536f068e0b619f61657e0961e87cd8168e073384595501f766|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ceac7829f5c82ea7738f5d15afd7b6110dda575b0eb92d9bfd79ad0a9ae7dca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c72b23cdb825c4ea9b8c36451cc3a67f6370f057cd39d237fab8f7a185e448f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-51744838a9831dc15b6af2459c9edf9fa219a5accbf65e097fc57b304c9232bd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73f0c2f8e0c5cf773dc07b9c3bb13f38e3113ec6d13faa9ba069589e5ea47678|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39fec2b6af32115bdd3036125e776c5abd1531789da0d4b635b86ad335d7fa19|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ee807f68fc4193646f0417e958d15fad41e5a8cf3acb67ee2374ee3f2c550291|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61af9daaeb54e0d46d49d628915e1a8b5ae110c877c9388ca3a3ed3efa030ad6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7aaf0fecf0766515df23f06aa689ab117b2dce606fdb63c38753dc113b8d6dfc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd79f25cb60458c9561e5575e78eb4706f387266e6edf173806536ddf942da91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c85a44e6a4474d606a770fb9f0cfe6639cd02848a3ac83be1ffe9444d7cd4d0f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-baf85222cd904b749b3b9894aaebdb1800984fd203eb221ced46e20334a93ff0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e92540791e727e5c30e1238ccc62862279ef7212fe103211bea8369024a368f9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d51194d2076d9f94e1f95185f71e77aef580e6811b1a7741ff69d632a8c8b025|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bcc04ffa6b50138e60ef6780117f2dc665ceb92e225f129ddbb2c83e8b72d697|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d5f0ce3c4e38cfba59a7f5a470712711b6ca82caadb63f9686577d7b414636f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-469d69e54efec7cb7c028add61c34292d1366da8502ca0a77dd754fa8e60d5be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6559502842650d51fc005b91c0b72e93ee9a81821dccf08e2082e624948e7e24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d7798b1951cdc0abda02af7dfe73b68492dc59cd09d92ca63b7c2a561618aaf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93f4afe2c9db38bc2e480274ce7f2707bb6dbd05541b75e36c2302db279f72a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00678e384adec722f3f22b59e2cd770038f652f9f29063d5360d14ec91b9d483|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d9575dd00b7daf34dec8ab1036b0b09ecfbf82cc40dfc9b64f2923067f37790|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bfc2cd3cabd632eabec38170a2b4da2ea910a0215522699ea73cadfe4ee2b21a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-159b070d11f08b57c02f9e348b8456437de15ef4bac39d5ae1bcda76bca8872f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d550fd8af3a17b188a6305a4b64df276cda85650e8e72c71b41eeafd6f3ff5a2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a889a5a482f3bb89c1f28011ce577baa0115213bdd7dc23d422a589ffe19fb6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e5e3c107642bf6b82c73f7d360af42c1852699b42d80d3317ff41c75ddf5d00|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38c8c3bc25a65d9dcabceef9551de3cc8e94609cedeca3e7867926010c4a1385|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c675b8a180eaf69a3a199f39ac24130525f75958b522916eb0953e9fe41fe9a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-afbe7f407313d185dbf23df9436fd2f5b503c51791130a4ceffea3316999bacc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-904e49eefbf1cb872a4f2924c12193d622b2553dbccbaa67df1cab1738ac6c36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7108bd11db42ea567926587399dd84fc129aea49d1b37eb0873e983c27eeec3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bebdd02ccb6dacea1aa520b59eb6ecd1a6acb0889e61c32048049e3ac37ddc5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1992acbc203b3f6dcdf9bf10010b01a32cff2ea21f2251b4b4b9036064688a9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bed64a90ba80ba6bbabdee5e68625fa8c2f63f826267f5fedd2ea408e28e8c3c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5cf1d51af5299c4c200bb87ab5918e6cd5d1069b345ecf2303dbdeca716c4db1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-725195b9ed9573846ad17e524bcd057c43dd59f35926a7d86b7ff1460411c60b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-541c97381fcd2ee1944f44b368ca4a2993aa32a8d2ba04ea76ef63e3e5a17255|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8db79aac2e284508b3d9e1a2ec44c4447ce46522a553e29d6c83b3db17eb8083|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-61fe3f6605f75be0fa4ca40a297906106da75ab8d93b2ad289988e568b541595|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7896bfeae80b5b8577f041e15363b3629bd7b0722a3bef3fa2386aeb66fe0170|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e5ac709c18d19ee1b460f2593c8dff187df5e35e4980a0ff563223ed8997fb8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b07201a48fc9d5435ff1d34d3da33c60bbc588be5bc1c22ae30ba01a3d2e57ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-44ff23b589cdfb0e865504080d3f6657c2c5161377e704324d72176191102c3d|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-846e256ca63e28fbeb33ffa8bf8a8e543934af2e09fc66e2d410e3aa58426e1a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1ac77957ac5d211f3bf6167ebf364ac03ddfdc15672997d658687c4146f59dfe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5245a5603ca46bea9c68dd08bc523910651a5d9057f1e1dbf881416b37c2c98d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca40af37ed93ad10a158eafeece4a9638fbade64776697e661fa4311b4f78c4d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2fa1a9cda7ef6c368333634f1bde81287a5485e896693ce2f4ef43870e0bb7c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8d5474773ddd9071612252eeb073540ed408b97864465bd74e5be815fd5cc84|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-12c161aafe503d2d3f8e4ca196bc585e90547b36ed9cbdabf1a660a181d26655|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8d6b36041cf9539bf8f353631feb091938ca67d2ec55ce3dbb1803d6173b79fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7cfde196a09611a58b585fdd18c2a41c0ccb31acfc8b6096262997e4c5ee4aa2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8823207895fcadc0d05c78a60b3858f99df7acef4518bfd3d2316a936df1f75|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d3145e784b57f587879572a6594e1897b1a4214b863716433874ad268ec94cc7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c8523424e76ca4e4d41d943e8be15a3776834a10367e99d178d585d8b2864b9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c2d0c8efaa6b4d1864e76acdacfbfaf3b4ddd482bcd7235dd535c5309d2814e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5341e75f09f6dd5f57668c8ef2b71b97041440898b4245a42f66b529a4fa6da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d85311f454b3d1f0417517c9f068054463276c8ae595f8e3f3e510f0762e52b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e1c71b029fa5ba27619098775381d38b663bb20b1ef90243ce0c77d4baf231a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1dbec84a51396b12f6f47a4ef53bd76fffa3893419f0ddcf26400b7d8588bf65|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3067904f6521bf9e34ee127671bdbbdb0ec15ffab7f703f1901a541a530a03ef|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5d799d7f93b431b752d59e85be13f361b95609116a77bbfb7d1bbec938b5c1fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37de9239fba5787c1739bdada5afae234792cd6842ff95d5ecb7669dd62e0b25|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e9e91e5287aba670cf7b5ce27342b8abff56abfc4b38361c8887893fe871e90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b58e9d01d00aaaee17b4bb90e0db07d30603a568255ac617718acbd1c5d4c75c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7ba605ff1196135d711e329d9e9c6303672b0384b28182308dff8df770761fc1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-711c80efc6439c53719043acafe50d6673ac96687af9420dc43b79237733fb36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b19a27d9cf131228e84525556226d1c2db80e289cfafd95c5ab123c48d03ba96|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5485ab95ba30a5770c26e16fc5c0f95b1549f934747c94322bff3bec7d070e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3541a8108ab1ecab260862858b2b60b2004b9e414e05549f66d84ac1b9a5fe24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dfc0849fa407f9f794c508827f000080532ab935fc32b5afb9b01a830331b782|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fbd8a23ac2d69b761e8e95ba87216c62b1f50d1df22a2852d84944cf52970ef2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d209488ffef8b6e0a8fe0c44f239bd050898f1a94faff99580d13316dc0ccca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39f9558ad2f3fbe52dad85e858ef1a296fed0f2b68461545cacb21ca976e1069|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6118e6243c202737020c9597927362f592ade0c282a44ea691d03ac59d4a7223|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-620c232c0ab232fe13e06196d41c01458fa4daad40bd1d6c1c92df095607f727|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4509bc5013d34e4ac48629d51500862fef7e40f2f0c55003b05fa00dbd39cca|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6e5a3b33b6a0235b19245a4542900ba3877d4807b5b67e3323413f5ed7caa737|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9e1db65f07f994c3dfdd14358fd6b70c185af41f0fb3269ed4d9b26058de7e0f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-de86f59c2b041c700d7b08c35a6e8bb965706d38a334d0fa05048b108480b4ce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ae84d228bbb9e732dfb6200cf9df3eb7bf40ce7ca6fbcb79207b23364c28fd19|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ef89f168c28916cedd1cd06774a518046b1b7e4559bcbb077d5522f266ca793f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3a1756d0cef09d955744b4870520c0b87f73df65388747df9121fda33572e885|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-10e82b421ba30462e76a798551c536c77c7fde22bf096d9537f8135fbad540f6|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-cde4106252738db1e341c1177c7df5726c2c051abfd2ee710c44fb2a6b440005|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6be2e3fcbb7227494e97097b85ba63556c98cb3897e1f9fe60fffbc82a9c095e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-75fe8f012ee37c64a5b26f116ef8ef64746ec3f54b5d3e33d563d9d7a21ab9c8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8456e01f364ea86b7b70b7546878d3d63018507891479e0c710785c5a231c480|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a73877fa1887a85a45a2330b69ba620e09f20998b6d293d1ca036d6632f45b45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41e2b2b6a8edaef0ebd45c9f3d1431659101af1fc4831dde7a683871357a94bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d8a59022c0c0e453d7abd277efed519227ce2d771838ad730ba4f7d8ffe53d0e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-832de1bae44ff33314c872184b5555e13c1d3a3abef98ce4c7b08550c28982e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4fffbe2c478c8c057ce8b6cb61afca3aae8e0a3e379bf8c354f035a526892d0f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b306cae78be3a8c0266492cea93e5d38cffcc0a31499473ee1495cc518aaad37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d62a18dad64bc7a9d65a845e1f286c782a20e694c47a51740e5a3026765a1e2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-674f535b89fcf91be9da9dd88cd0c33195a9b48db32c2f9bff4db293afb2ff18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc8d6769db949284bd64ae1ac628944f9d604daed20274feaa470bcac3ddf6c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96430d565b06832ec95cbed6bdc5f9609abbfbb5a4242980f403728a2f90bd9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ea4498d78ed6b0f0e5abd57907c6904e316d5a2001bc2151b7f0d62b9822149|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1808853471f40ef389bcc9aabeaca0f7147b2ff2e4647a4cdf245f10009815df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c22dd9a7b659df93ddd91891e395936c8cde7d176bc84441d3e49f39c79e192|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0f3999fcb1c395d374ed00350ea706d84744ee36f1661bdf43696ab2108c010|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06d45e46060128a8fc904499748dc125d7f3c3d9fbd4ca0eb276bb74ca374db4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-696c164503f5037212fbab553bd360f721f2f988a0f241549e76819edd878fc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c2c951bbdf6db392551727cdb398d9fbb13e8fa4e7d5b837fed0be6f6a91071|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a027b4620f415aa2a3592c2b144cc78f58e3f45e1f3f771a075228149f655fa8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3aa4cf76cc4a3cada1af2ee79fb56969c38d3fdf1acc0f5e3fb5db7545ce01d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7421775bfae12d4ca47c11eb210cadbc9b1f7c42ced5d76116967538176b61a2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-906b82a6855197f28f1b7d6dc51c44c098201cd92b1c129fbf25fac789cc9f13|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02b573fe524c2650348bf2735fddb9ca080713aaa09e430cfa77a4cd74077280|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05792c9b948a318b99521f4c0f00bd9328b93fceb22f08ec51d8d2553391ecdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4f74470c292a2ffb7e58f025c76bff733d76500315e17050c82811b6ba5e779|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3329e4c48f13931669145996e05f2adee0b3b31e225e205dbe1fd0be1a79373a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a080504ee342f3fa051cd3f2a40ee2484b1930f41466e622bdaa75e8685b95a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93d16716cf9fb79f7924912091e99cceaf6447b62eaf35707bc5577230ccfec6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00a81c34fb8f94334b2e53809fe34cb11a5ca84f1f7c107a0cb3607f91a36f9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6183a3eab8f0b336593ef7c0b0d5598d7517d9afec600f9fddfa449b767a5928|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-964dff1c24886338b1b879eba4eb399bd25a2f8e788b68d4d206bec247f83173|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa1519d5895e0e09df6bbe25a194730e9b25db27720187f8644d8c477b0f870b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0fba740cab7cf5100c82425491eb6749b39944127030de11b7307be3b535cb3e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bba1ae413cc74c248ff74d2271009477ed98fdf234ee1b195cea996b25b822ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1bd5f21803d6a7517d87fbf930399740699ca239e211f467a061c81ef80f16c2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32234db202f20c9506c1e8acb4c7599ac29606cc9c56759c600a84d31aae644c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-412c7772153c8808087e2c458928cc38438752c846720d97c2642741a94169fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3332ce92b28686df72e60d0c85113d3484e68e9552aa0fee975a1f84d8162274|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf486b3d360f329dde736b619e656d1f33b6cc18cb64332c1a9ab96025eb9607|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1376c086f1e9655bd5f8b9c26f886af512d86ae39c2408280b8f1f37186f7fe1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d0d8e825b5e068c84f1ad967262f4681be99d2ee08a175026753f0a6b2b5def|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8030542c15145203eedbdcab5e155d8f6b0baef8b923dcebfaab11451f8b654|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-968d847a72a427d7275e8a895d1278e776b9d1488b81a60b3f3db15502856e36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3844b43722b7281f40b355ae036fa2791f73d9b69923def4e97455478acfd539|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-24fb9380a3a7865daab496d865420dba8fcf79b0ed9ff2952e57e807c32019dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-99a8731ceeb4d0c30c6f6563a8ba976ad12b2707017e8cea9800cb5939133846|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-052468a35204158fb33b15f18cdd5cc7a72b64b082427befbdfae6206de05250|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96fb2d782f4723f13e1bbf4ccc86a94edf03117a5e5f31d055e82a3c1112d2aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bf2b707245836e4ae1482db5afd7f708da12bdc749bf3ab2fa50b36c6bd7ae71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f6ddc1e97e270c8cdcac8674e386002fa6419510209ec4d567372bb40ac55874|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-904d32422c3dd6a9fc4eaed9cfdd02cc32c4c9aa181400b5a723406e3a4bfbc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ddace7c1229c402adeffa06edb26ce4fd28215f3cfffdc81076a4d0311f73c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4b067acb867f0f4d8ffa1db6da49d85f5eace5631948f46d5129da809548255|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a88de801d06d181e300f3176b6b485a8412cb07046f8bc093396904b1fff36e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aff9eb6624a6e41897274894e189c309d9d949fe84efd2eab4ff90251bfc344d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-97d938ef8b66c471318d8ccf644c9ce1963beb6cab43f2fb9865359746bf85cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-607c2cb9a382fa59a81f592a06b5317c9927c8e8002ffef3668f9e5d2278a504|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6029227343b8f1aea89279116a498facc1c9c22038ddbbafc15fa9c028ffb41d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-002844a459d8f392ff35b5b63ada14d6e6efd455fa3888278694d39cae73eccf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-43ea720565e2cd2a135cacba96387fd9bfa4bea4e7fec29ba09f16d12d8d81c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37ab3d9c3daef4f70573a65763bb9194d61aa686875f2255a8db6ccca82f0b5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c889fa7d227368c03e60e5c03b4508fb100eb2b76c73aaa488a323dc2f5002a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-590f6c4c4181e48dce362b8d29ecf71ada235e2a51509b6b4e83660d7d8d166f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea5b0672141fce34f61d49af0b54fa21330e05ded5c5dbf2d852fd0933803b79|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4215b64c68f330d34a486572e5ad3b3590f8eccff4f51b71ec0da769c311e877|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-df5a0487fbfac6aa97892b0fa49aa59b594e82b2ef3320d127dac21a00767d93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1070cf5caf86665c49331547dd4395f062911fa907275977921254a9744cb717|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7899c1e6580bf62698ae810ad6e68e60066da37db5d53fafd012083da5888aad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1212013bd257081cc17da9e7bd590c1d5560228d0708d39e2b1b0c903b1fdd8d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f31e2273ebecdeb727a5ae5a40b0666a5280dd4060a52f6a43c01504874ceb5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29ecfb4cc8195da85aec1280d9d56f9c3499a8a238a63f22c8c969ef3f058222|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27dbce3246498d8dc9483df5f682859b53a81e4f9b8e6bf55e4d5683e305ff07|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-532a73c3a9bb3064f29977ad09a460b9347905f2153b80914738211ded681421|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b43939205285a7479b84e8e7d34cd740845b2055b90cc3b68f3fb89ef8e80c81|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d3d1d41e3045ea8d18f9652117c2c2385ee50b08871d8bce81e56c6c434c528|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac05f465d1eb4cb948ddf611822d9b93aee63e4ed21bf9640ca7ae8f5a4a45ed|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-42e1e6f63e27aaa8175a1012b1200e53ccd50fddbf0bf055ffde009898d7a2b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e15aa3e418ed579c8f47e690fb13396bcba6efc39c6b993971df8d76e7f7155b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c33f93ec98b433105e3707b713d02a6a9d9acb4161775346a542928cb7be8b31|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e4b0ad598cf4e026624a44c4b19302612145f8646f6145337e7af9f36fd40af5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5bde261ca0d1f9ba6ce0703c89b9ab005db9b5815a64d14e3cd7f155f06a2e8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-275eebe51065fdfbf0158726ad21872ffe7f13b94adfa1ab3c337da16a4f283c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b5a23587825b570fcc916248a7d4d9bf670dc80d1009795806dcf29f77f31c4d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf264519927aac26c106015d896e6ff26054268208b1440adda75051d2952e1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-901de172f6d7562a9ec349e4f16d98a86d4c3552ba926438b0738e4260ca570c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-512079310d194a0f96b80d4687ed38e58ca1da6839f95e246b0f516b23c4b7c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49b234c38fc474764dff883bd2c8ade4204b03e910d12bda110974f85fbff8d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73d65c105f95b773d147734031889d7e45f0eac41b758a64e0540afcdec47b63|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-86ad7c5c6a6ede8206cd9eda86f65bf5fa836ff11aee19c06a2305f4d0077dd8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac05bd09338fa41b7481dbb80845ca2608f68d57f15cc1fbb2d4540e334f95e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27263a47f9dfce83efbdcfd47809f17100add5466c164d02a84efaf244121036|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a62e29c788447fee547df3d2fcf07885356147b58a801bf060aa030dfdb44bec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-01ee37ccd86fd18376bdf100c43fa9c2f0fca15d50561bca761348e4b3dc82b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dadafe13aa3a720cdf63d4360966d6797e5966f098ad56a4490f5f3e23575220|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da08ea4d3200d8e589c873aa58b23660373f75a53293459626b750d22a466bd6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2df036174bd774c7a9e7954859766d3bfbc7fc55d2a0091efa7eb81b4532e39e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-956cb8661ebec2f605b2edc2ddff2082648216e62ad82a64fec08bf588954cf3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d33c8392524f30a64471d138461f6408f0391fa9dcda3e5353cbd678ffda8b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-21dbf00b29441b799d64527d3d60ebc2ba3e0e9745ba21b9de3a8670229b8892|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bae05d630f92fd2c7e0ed1e4594ca925bcd00195ae35c88bb33a65a436532abc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-99b73818bb52f726641d50193be3ffb2b3d4b1141aeb9cce6f91a794b96a538e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-baed5a245af2feb445e85fbb31e12d534506499144ecd428f5afde06574926bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b269f8aef80c69f16799e2d41e694856ada3308a3f53921c22342c1f8202682c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-563796a74abd0cf3ce6e3947faa3c0bf79963a6a4dc3ea2ab97b388c2a05687a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50f180054abdfc57a49e7289e1464d06e412b96558287c0d35de75e45c9f394c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b9fc0e32f667d69d50ce48d893c51e9bc996e5aec5b09ee6e49f5d1df10b0053|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-04181e2a30d903fcafac8699183a580f37081f71ac07b3e053686ce7fc5bfc0f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd9d5d3a9ead693fe37b10b2ec58f0701b01592039069437969980227d70cafd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f522d958e3e3c3da0974acaff9d4b0cdfff42fdf9cb2a167f01ebaaabc4772d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e39fd9e29bc77a4a7df3069e7ceb060bdab5e6b310707f0b8d896f209e7aa578|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4518fc751eca39457e52cebdd558c6c076b841d44ceec856ceb7a3420bc03608|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b6d18cf6dc2777517c2164d1bb3a69ec3dddbf373887491b70944c48c0d4fc8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aeabb619c6fea1e407dbdf1614bc2eb867f08a47fe518088cde67a82140bc718|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-330447dcab1723e21b98091e115562831afe95ab920be08bcdc820eac1218757|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-578797fd3308d04a9508149488b913e9f5b6b25a80ca602e9da728bc6b2313ca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-99d0c85a52c0eba3ad50e3551a8b03fca3da6755530400a099eada071645ee50|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34e847ffad313f8983ed8913cd7d25b13c8da7786494c13b44d65e0b1f7721ce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9becca82049ff276ef3dec19e4bd087b1fb6f589b7fdf1fe022a1c629cbc2f49|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-991dfb39bc4ce2bc545fef2c20e3801da645608579bd305bced5fd40bb78ae87|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11a2b70c8e4cb28227622fddceb739daa17329e69ccd2cac235dbff93ff9098c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6ad8829ba397abd5e7ca84c6eb3cc2a5df244703107940f9f0801626888120da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-24f47f91d2c159238c3cdde11c4c287f6e18d61247fee67ab043290bf060ff3e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e297c645eb8ea0755d1ab9417b48f785b63879d8f629a8bec9029b9442bcb121|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-82118862defda18f5281bfb224efe1ad986023093ea8913497a1db3822a07bdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a2bc323b60a91cd1025a108d43ceda0250ddb18635c10a11fe5359176020daf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e790d1f6ebd69e512140409d12d535e341a680648f569f058ec49a82901178c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12886d151919f548fa1c802a95abfa8c6ef13c6357f7978d70e8e4c4b47412ae|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9559269ebd7f4ad018123762af2874115debbfa103a3577b2245a5781a19e712|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ad82b112bfffb141f7be0915854f4fbcea7f5e20000b107cbbdc4e9a7cf9eb6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fc07a32c1232dc5fff091b48448d0330b96fca74a35d5dc56ee46b5613da1e8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd05170ec9fc1f8019c0b3c049523d0e6eb4443dc84dd7e2355042e82eb55696|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f6bb4e7ea83241933558dfbc946f6b96d14838ef07a87aa7a1635fbef0fd793|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-311a9d8d3693504ed4b99fc8ff391174294f42afe8fa61f7452aa3518c377440|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed87908f4ac704b535d7916363d981e8191c916ca0fb92afe59b52fe07bc8ed1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67cf3c7e1fcc78d2acd0b5c41ae6f652d634ce64b7f613f81f7b60448331b0c8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a9f4a4b06bb40d4f8b657f6c78e58f289e3edacf5e8503892ab1be6bb00c6071|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c03b6ac790482173a109891998e906c347a839fb5dcc43c15af71764fa47de2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c68789ed9f1d0c64aca72b5fb613f2eca3bf063aba15f8337cd7b24a9a7732c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c1869667ef1f0571b6fe34953e79b47bc774264c083de7c436e1d27474c0b92|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41f26b6f3ab4813f75a030e3e09ea9546ca7d3e3e29d3e81eefd3d5efd54db9c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6887038b0b58b8dcc04fec02eeae0c266a3f5ede03f976eaf545f49394a67514|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4528312756ead8f2245dd5e5fd77edb6c82d48fd0d4fd5c6dcb4b0eed603478|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3aaf9b8603c3f32259bf4215bfa94c873a2f5757cb5491e2bdd9a1a91b16e3e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d069483caad792228cb5020135a4492bb64674cac95cc5acf15a13c763eb1c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-137662b6b0b23f73076f4055b3b197f8f530a4675409496f451885a71d67d6f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7edaf1a96f04f8b2b96e8e79682535abce84bb916fb88a84ba9f19b001f1920|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-26b7fbd91056cafafacae7486766029d88babfe1103af2c7387205446756ab81|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-527947ee70071c69506b073e46113825248f0b5a07d27508afc855e91277fd1b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1bdb250c1b80eddf9b4e8e2494edf03c4cdeac5e994d06c39d904c722f05d59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c52b6291a6f92e5fee57ed4ff657b86f7eb1e105431995d479c8f88eb434ed3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2a0bdfa5bba18ceb0cbdc49f0924db46b9259fe12adf9b29a9020ef28e8a11a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4715281acd9fa559d91c2d9bee6c873909cb8e79e2ec20153021cf7145e20ea3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6aa854216fc08b4222e987c4ff2883fc51d921309505f0e82c8b1f97369a90b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc8c4c45236304e82d6c87134a0a04cecb683f2dee9d43e8d59d86f8cfaf29df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f7394c9efccf22406b08f4d650f426502d1f8bab7e36d77b89f4289d9d20e546|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-71473917a639919850f13a6c0efe8fbebdbaee788fc9c6e626a9e39f4fe749ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f632bf8c950ba608f2d9c6a41107e53dd1e88cdb77ceeab7503535b42c9e98f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3315b28cf8c01f99b725293561f78bfe44c5833d662d82cb417a624b763bb179|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ee29148da46e258d3ee76e8aaca6d476864bb52a132048a260c7a25d0ad483ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c12e2eff6c030e8f30a2115316517a2700552d715e4131ea02873f4f13ac786a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b9e312dcb117a624198ae8dbb93db8c79270531b39db28098af4bc8f5b2706cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c86c0e2a5603d4096f78e057b7eb5496c7e19d4f4ade1de4c336dd67cc20af55|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-931b9173fc743df06b0ff965d6a300eeeb86045f0d3bc65b7228250bfbff0312|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-315ee956f54446ccaf5ff8e35766c0ba92aade4ac77577a1e706195449a2cbd4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c47a4854c683b6c5c676a6fd8ef251e5674e07a3fb544e1a4cbfe52f0e2735b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31f0c467c8f9b71f64be8961cbdf12c1c3f97fb2e5d85a5e94728e6e2344838b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7196217ad2f6fd238c3ebd309fe6d877f7cb88b0878ddb838c4fa659fa4b963e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b619e07efd76db1d9734d6dafe890eecf087ab63442add4db63bb02e70a383f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3639aa20caae3c45012473a68541c45b30d6ec08f57084f387fee85ad67bcb0d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e05fad7b3b6ee54486a1b11bf40354466b16bbdee150b371f40227c77ca8dd7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05a012d04b2b49ef49e8e49ba2df4198c89f587f63ba8b5e8fe8f216398c9b9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b386a20f5b03230e628e15cc676de21400b61eaec6c4e67f36367dd6794f3252|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf3b8186e315f444433378c7d63cd6b408b36516345a935801ae5e8ea2281d16|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32d316ef1578cc4f7e50793d32c223ecb1109df8038c5e296a02ee4d37762100|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd871ba98472bafc71e16c2a2564c5c29a2a1a5772d374deb289f4c98ee07104|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96ce6670482d1eb875b1c82ffc120d0b4fe9e7dfb4afee57f018aebcb3c0f68d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5c223a81459e4f31d3a93c0adff3d7053fd87003ae331c9a3876f8be6a59708|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da6091a1c1583916b77ce87b37447f4143b375f5e3178c57180495a240410a6b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cc47a5f032e0e9db9663a0e0046adeef4c782ed1df359d0793d2dcbb9d7037d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ca9f3a299c1d941b6851220f7f148da38b0de1b207fc422ae559e53d95c57a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1aea191ef672cf203dc5d74eaa0b8594376f13cd728de5d3fad21a22795836ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fca91043989135a1f7189200a18c8e0a43e3d7e2e5f9156602c38a34f388f876|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12c78373160a5dd126ce1939619b0a00799e622413bc0ade7ede17e56c766573|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8409bfff61859296b64629a44802d212f59908df79ca31ce2bac8820d6abb2ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ed0e0c14d69a8dd4ce58351483d1690a876d6564ae3950eb75cb0595a65a613|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7a3f0b34c7654675c8620779bf9f813e1710f78d96c676b0f355cbbc0134b69b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ef9c8c3dd9bc5dcaa439ab4015aa3c3b00789255fed229ad46c995b4dfcfbb78|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b8076d7b2dd0c5645a14d9f91d4fe143ba474b2cd71e6a1bcf69bfbaad836c8f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-463b05bb8f362f30719af1dc404a6531dda7b710cdfd2116950f49272028c92d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2f7235324954522f367a7c013d11686500ba87f44828537559e238e22a33af7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad5ff7705a01bf80bc4f902f61d13dae9e91268a9881573309f9720e987a7a56|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-275b0c8757518435e284404e28f87023775d2282546ffcf50735e83c306a2acd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55b31d263a827a01da8a76a4f1d6c23d4316bc2aa0ea5c5b4acef2f19c044b87|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f6b441555f5bc44fb2a564287fad8c1d164313b09c1c8fe396d880cf6a34b46d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-03805d0a8dbdd2b608daef6fa675bc1d7b06be90b443c10a2ec6741fc9a78a76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15524696a42c189c241e1697c2d1873696dbd30441e8d80e178dcb60dd67ed0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73d4a6625cdcec7f18fc2677e6627258c80f6fa3393221f8b21062a137d8224b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-496329d35a0b713bf29107eaf906b71a6c4fe6d65734a1a12c864d807a8d96a2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da7b59ee6ad94b89bc8e473794b0754109b99f38ec8b8138e0e7be0084b0d591|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0aff46863fec111449ced55535c798fa08b1cbdddca94f48ddd9f42e87dbc0ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d21646c504f2e6f3d1e0572b2689230513d0d849dd5e5b26b97bade6d71f6bdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-248aeddfa0ca136571cb39d8e2373a5c029996f357df56059c819bf2b13496a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7c9ccee97f95329940118a2a2a1ebab4cdeadadaff1608937cd0857b3b1b0cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cef86a0a7e298f5322f665584b838a38248352c73787dcc5925e87afbbf5aba9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ccbb4790b810dec7f161cb054a1ded3a8d40cde91513176d07c52dac9bf9d77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1df5d5bd025ab0f3aa0baeea1461609667e0df2c2f751c8a6de71362c0c9f24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-19025da733e3c1d068daf3ef3af8b039df540e1d3d33c0d9945b1cec95e6c918|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2a35d1d8814162655b41034caeb3d893a1e4c7d171c9384df4a49d7091dff0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a86efb9d5badab2bbfe03beabd5c0aef1f2944ad152c9e3de073f7c22b9ec53|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c50133cfb362f3f3ef740d28ab2338b4a12de5f73d0e1ffe1118703ee6be926d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-28c1af1110fe8aae0a9ac2a8dc28ac6efa8273852f2920a97673fc06171a2aee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-483389ea3ab179b20defe3b481b277c2d682cbadb6ae2d0f96d17aa02ecef85f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ff161cfaf7acdd4d58bd78dc6554bddb7becbf5423debe921b35beeb6b6e339|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-91b25c72a4dd671fe871a20b2136bacf720389cf78ff1653dd144901db784953|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da7821f3225021911d99ca71de7458eb45092dc50043e2aa0035425f7cc2cc9a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ba24ea91100002e2b4779603c42e804e66dd90cfccd59045449dc5994d0d62a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2386ccef1d7f52bbcd8ee4d5caf385f9c1bed321f0ce9db7f3dca936aa679503|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ebd21af1f27d7b409fa00617a732d0efe5ec277c519951e65c0504610745f695|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2263d9c1e4984ac47b621ac4b43a02c94c7515c2a43e4bae4a6719f65937fd94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a38037c7922913dd53e190f4bd8946c9dabe1b2a41e0e96b2b7efa5831cf581|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7379daffa7a3ab0005c3bd3d2351eb961e7c977807488fcf5077155d30ad952c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4ad5298e86b28c0098ba038b5bd9db727617e7c9ed5da6f5bfd42044f7f7ba4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e23fe46c3f7ec1b667b2048a3e97380b25bfc17bab0a6252088b51fad7bc8e37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9cb459adbe8774ea39dc3cd0e30aa74d70e83ae28c717c5ca60e77d8d8e4b515|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ecdb676810cc3592ad729ae24f252d3e1643c88201e6cff5a6614ed5a17a432e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9450006501405c51dba1d6d9852b25f6b4245b3a333249e5039b69493bd5963b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f49333d5c6dcb2909f35a9ef810271b829f1d1af6e04d13545de4e73fd25f24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e0a09b018c99266b1df4ccc59bb1bef11ae8ce846f0d0db3adf8560269c2e5e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b0aa1d92859c448ddac78945a11f47b8e831f952c1b89744f18ed1c3ff1f532|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e4fc956efc1c805a3a61e99354136745757251420d1b1ab3954c568ef6ee80a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c1ed47a2b907b07224d40908a99bffbd469bb31a4957c79ac99d263487918c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-900b1d31af4307108eb870df41c363da1524ff70938178246ccff751de5392c1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b21d3cbe2a9ed28f013f7d9a1561b9d732cadf6b984915f2dba06756a67b031e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f141d1c7cf369fee2ff45cdec2a74c63b14e4f7390ea0a1283e766b36fd3af1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7be22ea196803f737d525635ffe25ce319b6c0cbc6a8542c2338df358145e30|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e08776402de263657a69c50015d1bb48a21a20ee60534bf2a5030fbb363786fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31195379bcac64694e2855391ffa9dc36a105480fbd6580d9119f1105ee401bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8742dd26ee7563415ab6d435b1e78aab33248810be0fc1add3480a4c35605e3d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f3209d7c54f959c40a1c944562f0546343d9be3299b2275d068f9d34e1114f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2192b9dd221057af3c12fc7c67c8eb00a08818825e76b4bb2839d8682bbd2f5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73d243862e8e3df02d2ad156eb0b7e0222ce40198b366cbaf2902370d1fcb1a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12933ac0bea673264f53931d1f903b2a9ba259d4ed8bc4f533eb8660a306819b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-879ae9950c0e2273a245f46b930042bc94a51ffe9453fd7683f1a409443b4ff9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74c7bedad27eb13c9a29179929c54dd89ed9d41e89b94ce11d95fb54e126e64a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93c6a9d794a83da8d0ca34c853b82d7a9e77c0be8057f603c5ca8403ff6ad4ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-04b7d2f4d67e27a0448990c6a6ff88f6d1ba0d19ec5ab433e2e7fbff010873e6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3cb62f33d20f3a748c7a5a0154dfc3c0ec604831cad99a1774389018fc5fcc15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07904165d99227f5395f91575f04e039e7149cb5aab2aa6749c8c46653a4be53|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2ee15f7502b5ad573d2d7e79861c798c969c7afdfe3873075e1d387557245da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11efc1a98d2ffe0979eda40ff80b5908f5fb27832f0511cd97896cc133ca4b46|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a932f5038bda0385fd9251d2380c0f03c84711325a128d985a11bf9ff6f09a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be922d1d21058b975bebf1bbad3840badc39080a8b8fb1478052e0571f48054c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d345fed88452d9e067cf1b23859e9a9f28b656ea41ab1c7513f69241213c0931|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5e1a032f5219ebc88c432dc0b6e6c7d46741c8aa514dc10797b393df757782b7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4fca211da239b294a16a01ee5f4563af1f0847d850ff859659c1bddb1d13e0cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b3c8daea88cdfb06d26168b5d2fb6b12f8cafcae1fa98f54e610de721b01e08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ce7b2d2ebf6851687afcae1c0743807e9937d422afd0ac9b9f1634d76007635|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cd20822fc518d9c36563b2cedecfb69dd8c80f01b7f4d5465b631942d842b8d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-75fb23c9b7672fa84210ee67507aba84b73ac5025fa496e114b513443a014c45|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73c93b1d89b5d11eee0b23e5fb894363cde678029e48188bdbf513c16db86ed3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e57a3d43e355a88d3b91df2282fbb41f373b1e14fa8890495dedf4bf44811089|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d8ea98c88154d974d916053d8f2316907b2f1be3b3d2e18e3122f0dd29713f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e468751d952679b282e316d0174ed42205642030f712b4517daf54583dde6345|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ebc8a1b7b2fccdfa62cca185c0fdf29ab27f5cb6e76d5437ac13fd4dc1b26706|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-86430ceeb39979bcccf2369d91d27fa05a3b904caadefcb79fbb8fa130c05ecf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1774e681ef0ab067628a095b53be2f01b826ce11cfd742a1be0481364239c560|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd5064cbb2e253689b8505201d3d95aa493a495814493f22b2d163e6e30af84e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-26b9444a615f46248decacf5ab5f0ed4215da73e4c44ff392abd793b1590bb03|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-891a05260591c4a9ea946d74315af6a95593e9e238e9330913873ac02a9748dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f91bfc162ba6ce696cd149d33825e0d08f3ac07bd7bc604aac41472df11c610|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84a7bc52c1de3da4685cc92add2064f55bef183343139f7ea5488b3638c47988|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d0c906e75116de1cbf012f927d9e3b724e000dc66478e8f5d9250ce0f33249f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c60ce360738160b8bff9c7bbff287c813929c862a0ad53a340b2432d407734bb|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ce179161cf8b721a42f9107ab8f5683cc235bbd9785c08b9be6a52a5f5c2849b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7080ee44dbd1de9dbf2500525f71917c1887fe8bf6d0ac2a51217454aac4c03c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0b6c0aec17c27063c9dd8b23832df58f982d81ed690a0ec4d05415eb098d77ec|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a9c4674753599ca5a902e278cfc5443cdcf669dba60828500bdda1fe73e731cf|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1272242a9ca43a747de7f15debfb920c0c039c8aef3087bf38b884ec1ba2a685|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a0bb1ea7c25c8d2aff2d457519bff8b741af61240af5897d3256c2097afc8b71|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f907decfb706ebfe1675936f04e37df7791922e8350bdb4535604eac2892f287|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-95a754614ae815f5d735b63c9b678d23efc7c6b64536f59ff45c7361373aad7f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80aeab7e27d446d6a1979d82584ea209af03b1298244b9565f57e4f41e056f76|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e019d72008b32f9703f27605a4ab38799149e475bf1c5ccdfce2acd6c25fc5c4|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-dc830bc7f5989bd0684fbe34708e5ab1202a5c823848faeee5954549dc05cdfe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-faf8d24ab0559e35bb3021f232bb38792a78f752215b5707224305d0ce9ffca2|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d38940dd11041453b43356e90272ced128a09dd2a69caab554de32127992c95f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a9052d628f3ed338fff15948c52949abab1a3ee8fc629d65926e7ad15b34130|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4ba9d1fa3b81fdc8dcc0461552a1fbea1669fbf5a34cc0a0ee5849bfca76875|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d362a0eb2902b49872dad9f146a4fcfd18156b4923f9f48d7c34ab0e37a1350e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f034e4649c7f0936ad08a965dc3c310a3f598fbc5e48b2b835dbff2d110d415a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38ae05433f18714bc532943bfdae9c6156c3f33fb8d1b2150d6ca3a14d39855d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57e624f91d3220f5290ac801157104b57079a17f128439f340eefb5449876a5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f45ebd30a4ba69ee5fc652ea9bde51883ba55715bda7e3b74533d932a524a85|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be5115772402c02c78d2eeb39e60251eba2367c4a445f01b94bde26dc96e79d9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-292367ce0c90ee8731459a334c0c7d1643026ccd55ff46848c0f48959ef51fe2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-56e93c590fae4947331a1fcc6fd70418c347523ccce4671d52dddfb284affe1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c123aff9ac0bdc8d4d4ccad8ff3b51cf5aee90b38366d0eec040db4e8f61fe60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e2383a0ec72f45e56917fdfa8e249179822b968d575059062d73b5edf6870df8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1ba1635c301df00c516abb7b787a4064994f599b0768adb4472b9f5425678a94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3618d1472e4b76b50dff565b005b3b55664cedf345c747648963bacdd8439e06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c09c0a5646571ef148bcdf2e7c87dac64361fc3bda91b8417a81fec711816fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-405c099d8b2a602c54228032f288c966dd0be138977952bb77b1cabe31e6b68a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27ed05f9e7fdd756c50854612cc53a350b6288b8be15179e8c9afef004c0137e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-312c8336f2a16c495a06f6b4d44dd273b5b300f480ef9643c746f96788e6faa4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da7dbb6e6cc0150867ee1a49ce0b46e8c243477c2f629e6fed2f334773f72f26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9df2629245c271bd4336277371916790e499d8641e3468b47ea0a615aa0001b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4f44cd47355b801660df992b3f70ae425a582b596c0804ce52e247ea9139812|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbe7c56d0a8849554d5747ba33e51f49bdb6e04a5afa4481f8480150862aa0f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-103bc3e9772f68af738b09dd0345bc6effca376eeb78310aa1dce1198ed49f35|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-644c98d3981e455dc2bd2d1f1ca43e9fbc7799c147be29c5dba04610e914b620|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f4cb21b50cc12a09356175b77ae720fda204ca9b13249be5c8f8b266afe850e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b71689c52729e2f59c6550fff3f1444afdc7b898e14a8c4efba07168300e192|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c8b72db278790793016d2e72413d2528e8c626df0283731f240f6852a85ee94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b566f58eb70ed63245a063893960d5127c9c775a6feaf8e16ad6b2a47c3c1cce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-19e9571f8215cdeba03c0ef213eb4af97f6e899390f39614fd8980c76c7dc88b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b42d64ab15547520d1423d5b72cb10f540d36eccbbcc47fb06d8ca485d17dd15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9545056737e864b7087b7721c9b0d4a01a8096238c22ed804ada20180776aa6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-171912ce98918f81e2f6a536c856aa61ab2c0b27102d421f03f379e174ac81f7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e769a05a8bea09581da120f9752dd87b09d1df994e183c8d36b11f92bc2673b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2fc83e40be37762a3013cb0699596ece2bbb3e83e5a05991a3d85cd2917bcbb5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d0b2ac47ecb96559102d0a240bee4f6e7a3dea1dc9d12d4598054fc42702c69|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-439fd3e41f2cb504e8c4da234e3323d78520a61a6ea953f777e741a602d18dba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce98f8a5ec727ec5cfe5c85160fbce5fd6b316e565d6ac641b71ef170b3851c8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d371a8fa29407788101f43b9a751482be8ffe32b1c9a637f268196d5c6ccbf72|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67a08be1204295fa4bb9eaab51f91c2efbbe931470e4ec44e2a074a38e968d80|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a1b588394a00e1c01e7d7cc81c72168e5c37cc081d2c849d2627821e0f91bde|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a3e7cb325379933f8101025f9733a428f6fb84d17472a22aeb4f88fd6746ea5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b1043403cb9eaf3fa2961d695b24c4bf0f88f8eddfe76c6166698ad9d1337b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-519e63eecb5e84d3aa3d22f0eb17f199d8094c5df0eaf02b23651c4e31e77b02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29ce58d872c3d18db85cc1407c8a5a78654ce2e18cb417ad2e1f4b5a66142079|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ab0eeeeba3121d14fc113c35583a419d3ceb08779ed8e7b96fb6ab110b4aec3b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af46f2787ff1db1844265b2fd1ae59217bbf0346c2eacfc6839e5820fe76dec4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f2da4aa070ad38ff683a8fc41bc97c5178f01073185cd45f74a97aff97fc211|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-249689bb418c8de0185fe29eb0d489016d0e2fe839e68b40868c4cd9c8343391|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3fa3db11b0506e6012b4da190d5bce2421d97147e8458258c607e89c6dbd8123|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-adac718a7a634687250b0b7b69668fe139e6511fc6a79bc8de99da78b12a9d44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5bc80152a2c40e19eabb1166d35c0fcba41c0aabbbb62b0acab1c2f8c64fa25|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cc5cd8ee7fefb7431ba6f9d5ea3926f09f8bc72142ebfea01581e771c82c812c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30372f2d6a975f7f9f32c38d50d40d26ad9c984752f984032f597124ecab328a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aca0412f6269155c904d22ac8d5c8cf70cbd193ff544074083d275b1376e3c17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6040d44f910066a1602b5f91bd8e7d804cf2e32336ffabdeb2dc4cc0ded4398c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c88a3d3fc3c480c12b779b7d81812f581eee521ce98b4bbf62a451e8dbb5356e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-091c9466269757bacfb5da9148a62461e5671bc5602bbef2622533a208ebda6e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c99372b32acf72d1f9ed09394ada44b427b6060a1bf6a06d202c67a2f071de2e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a1d97f5cfeed8d66f689c689d474b344cecffdd85ae4b23d3cd2f1c005d03619|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-17f022d4a2c3d04f8f1c3f0bdc5f559654aefea2066189257f68ce2313d8d0d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-283c5f5cae351c77cc5d57a1c7ade4344737caae119c0575280de1b666282355|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e6e2e63bb4956e695a00f9a175aadde47cdb04a697ff71f1bf621e8f5798fc52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0fc747f85850b3f0a91e2170aaef162b201f9fae59fb3b6799feabc45f06c892|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4a48cf2c0d10390aa8277c471fd12ce0b71f708be490d137b4bdfa4a1b2fca3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b77ca6d7997f7821a30a261ec03c5e27a353454fe5c9c45d65c0d8f19965a2e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6634c109b0531e8b964eb67f1c0bfad80cde709b3309a362cc16606bc34ed40d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4be57213b7172ff720411661d44fb3aa2892f54cf0ac1211e27e8f91b100797|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2eaf61405b2473e5a80e14282e58fed37d533755f6e4125ba1c63668fde61a3c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e2a89ab9f95614b0c6736092d8c40d5f5f8da2ef058469efbb41c4cb8a7cf338|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1ec1b4dc18f3a3c84057aeae570c1264804ea8d75a909b50ce7d394f11067da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4f7ec697b4c054f33689c1c7f3e26c5c803e01fc8bc5c4d799f01290a229c93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7984f8c83d646046dff4b18eb6d93a17de3d9ce98d57fc9c3836520b507e34dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f82716f502496e0442ab1625dbb1723a7050ecc497884f43afbfaf7ceabffcf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc953a9620bb83a126c3b84554192f6ae2c69120bbe89bd58ecf1d3b2dab6afc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a8799bb68634d8ffda17474c9928f927ef605b06c543d9a3f44929eb8da039f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-91894ff04d3471f82d9083002dbdb88e49a09883afcd8db283e52f400c271358|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb0a8dbf466024ea9eb6939f209b7995d56feab049c7efc67212e67ca9e0bd7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c209f7a952fa795641a785e91a8b77266f3e1f522063348898e720c3b4ca93be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5748b0041e1028bc31a7ef493ec3261815ff182ef685b61eedcb14aa6e07115e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-03f5f068bb7facc6547fd03c185d0279539b42e90b3c91b214cd09d9cda79489|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0d21e7df04c9536de3126928419aa074e48bfd52f895ca398509542a8244a8b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f5e00cedcc9dd9a3d8e63125ab039733bd4976c4a35e57c6cb5bbc032fa28b6e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d19cb9db61940bbd2c5ba74025c6b06f34ace408664bbeff557de344cb029f3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41540a68aac5b1c4bc90869061e47deecbfa97843c36a2dc1bd8e0b2d8589fa9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f12368a9ae5bd7e512fcdd4e84794197579104b2d2770f770f404f055d5632ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4c27f73ba69439532463be0a152d771c46fb77152db02241044ca4a1418e3e8f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-858c794b9895494ff6f3dbca6033d75d54993f845031be4401d405e7c8051dc4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b6f7836b68da0ef4ff4b9318900fc018030ff9c57e966f6043273a4f057912d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b49a1b7b45c94330215750e3e627ed3dcf315bbc17a4699faec3e07929a3530|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-82f4562bcc5c4388e8710779e32d93835f2dd5d9ed000bd11a80f221369ea2c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8782f86efd83feca63d1ae2fa3a9849c736cab96c8fb6ca4175b8654a864a1c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5939ee058648199118788a253be6e8a4f766d692d33f8c871859308fa6298b37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2ceddb622bc10c8aed0f2242166b2e97c16be010abdf79a391172d163e0a9e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d00592aad101f448d9918b5604f49d4ce0943abde3bf63419a0900843cf92bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8005ff172753910681d4b550bb4af8b48c44c7dbb815bd0ebfe2d3440cdceb27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b896980ad0ea6d4839cf770cc8bc0c8c668a18cdf8e0d990efb3c10ea032afe6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-48b40e66211adcc8436c76d0ba88bae473d8c2fd5191d8e7304c991272b084bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3822f3e8d32f73d59301165cf53a1bc7ad74705b6ccd5e29de41250711cdf37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-689850c1943241b438ebd880b62763b23f457777351ae2f13bb0ad17711eca8e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9645aeadfc622327531a25a95df37da82fe3135eb19f76db83e127688f72b85d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cd1c55b8107f666d9b1dbde5b7ba75e0986676db15db79ed05b57f03b2ff1d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-118ca3af4a1b2ad08ddb1037fb7fc2bf1c54f2dae7b71199f416584092cf828b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7fd13267c37eff168211d494049eaffcfff56be04f606000726030bb9a51083a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd54c91f577226567680584f5257e4a3b27ed0b0e565f9c6462f989dbdb55ff8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbc1730b935ea9e6ba997007e8a8a42dabf37190569d28e8e242c9551efcb1fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9cf8f4b47d458909c82f832f4407c41cb7e0a08e752ac2dd0876c0980337b949|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39430b8f2c1abb31c5586cabb6c29fc629241642acd42dfc66186d592c703214|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-800d1eba43db3aefb7c94403405f9c6c013d7ebe1080ac28f7175b2b1469d05e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22f3df6908ca6b503ab68778a1c1b4823ecaac72d17cd9660e7bbda30ada6a8d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-398e55baae70538b9d1a6dc8b87db6b7d2d974e882d980f8a26ecffb8066cd23|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce2bd9f516d743061e2d945a5c12cb4508cd0bd3acb838589427b89fa61ac2fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-78991dc9b1d4d37aeec5c3ebe9b6222a4cec23f04f639c320b86f2aadbadf51d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f03a61543b815ace9560bdcb4f5d0a4f46ff4da2b80020e4c17e38786737897c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d792c6852062297f6466993af59a1b1a42951c6b9daa45a45d678bbe1970a12|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c54bfd2131d9d60163372e65654103608df18194ece0a02f8367efe296a1fe9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-289335934ee9b79ece576ed01f2e623ba9577485b4c687d598bcc4f317860f87|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-08a2aefc0fe626be91f15823fce67b11b7e2882a1976fef85dea80bef26cfdff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e56d349caf4a181e70b85d96da24b8a1c650fe0f969cf1b81112363c8d5ab485|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4d1259a09614f8ed02df78fe90d22b41dee312e9dbc3c6d7e8a2169ca4ea9070|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f691ec7da64fc0a77973193c95cd619a16d77ecca850de86d4c5ad99bf1d9d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e018e294202b8d802d124b54c13aa2c416157e827e5585869337027d7f292e57|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5831bbeadbb737e133b5d1391acb142886b736ed1e9dc545030b8b1ab16ada2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-835318e3c320fd35cc422ceb1da67a4f070a3cc62e6b93d047ab632a468cc83c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98e2c58c23c7999b030e6c27c1a1ec4d7690a8e033a72887011f508821d2ebbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40862034bfdb4a7c3f395debc7f0f64591ad8618b0ca1d5b8465ec8d8c6cc2fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76f2f53ec25eb4627f4687f3eb25149ce1ec20062009819e942a27f695225677|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d274c13f46406a662bc9b3ba90e949a7b380efb27e87730a12426ec702d62bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55e7c5ebd338fcb5debbc2eca46df4019e49370b73ecedeebe634325dce18ba0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8e12c8d6331b7ba00fb003926a3cbc4906833ebbb6b9e441e4faff0c021b1fd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d2e48cb75d21771aaf72f932f92f887346e1dcab67595f9f9434e36708ee106b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f97116626ea36f7f5a31470bb8632529b1618e7b7f5ca986566ec7a4a680adfb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-113a269fd1d240a5cc5f753de7a0ef4a03b0a9f1e02465b865026b5d44a5cdbc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b671591618fe2b573d0d9fd74b176f468e413d89525729cdb41a99e660a73442|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-832d3f142b6dafe765b11feede0c47233d13876dabe8dd9a787ca16d960a6971|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a59f18bbf4f965373dbbffa6270ef2b4284757faaa069818120a493f10313b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c1cb381a02840a3a40e638423aae14dc4c4e5a36e62f25628f3d414111d87ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-050e8e506cfa153dd80d1e7399c868dc6f52f597c96d2c92617ac563e3bfb2be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22293e02aa72838796bcf2ae12d6168cebf588b1ad137304c0fc846b4f4c8e90|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27823f7e1e9b98ffe3ed6a96390ca86a8f2d667beb86b53099aa24ccec313f81|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2896337666e058ec7c0bf2b6e1d75a0a563b5ad5e306591d57b85083117f8a57|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2631821f49df2f64fff9e479fae3a0472db31751c0da3bd316afde7ab3d4f9a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a47c631453ef6bcea1bf29c4d58bfd94067693f83d073e3fb2886c03cf6a73c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-72c28c7f88c25b60f6bd1ca9cc2a5bbe8eb09176a451ab18e59e83b3d827b4d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0d2fc65a77eafd0d42d1db3ab08ae86878be6a69a98f7e058ec2648fafea81d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a309515c7db98aa653c9b45074945d702310670679b0712299500b54cf4c2692|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61fd4749f0c03d35a75d963d5b26500d1dfe1dc1f508e7984392be89f587a32d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff2fafc2c123abf6229ca3d2b4a4889db5f65a74efda23c307c7bfde8382d650|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c31a2cdd62065e997edc4237748cb47801083608536d7929dc7495dbbe5ebc8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3543edd25959b61a68ff0ea9215e610b9f211cea339e0518e1b6984f6fec0df8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2e072cc12ce0550a67839c03405d170fede574c8ad9f54daf5a5c36ae8b66def|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-16a090a47f381b1c58069f8773ee1a63fa659355797ec16379aef153789fb731|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9944f63d023f5f7cfd2a6a8f32c9874ccdf71d4c9bda5173c263ea7897af880|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27daa72f0f768c7b6bdc2c8f78344e087ddede04771cf5a5582cf7fde1a15ead|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-33cb353c005ca6f20556dad5d4a4d056d1862fc670756d52538cd65d7be44a1e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bba499ecdc52f50b0b94bfeb808a0aaf936b60de03558774e19f9ded60a4dd59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-343e8f13387a3aa6101847143ad369e9c6cd216f3cc83821a3e0762d09f06a28|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8a7d8980ff65a2413467c2206a888a9eb5bde8714b3f07d25b0ccc7a3968d56a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76a73f25da932e22583c11741fe8effebef375cd2ca401cbd49b28413d01ba43|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-780d6d01e505b930cdbbcbf81e9023b9ebb992c9066a0033129f96ad7febc722|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c9bfaf19f6ccc6e80556452eb514c50a1ad240ac31421ebbad07a3bc93b9533|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a316150cc851e1b42f2256dd5f51373704bdee511caa793b2f6ebffdca74b0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f4da035509fb8b210eba79dd07f2df197e54c265efac199bfe96e33b22ffa36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-765f87187c9c5b642b4d40817b781f9e6425dcf5ef0bfccf82ed3950e6cea7a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1afd260f33983647308e957b27bc69fec493841bb1fb30aa2501034176275b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-88dc3e12a86275f89e30371d7fbcf67e24419c30b1db6e31da11ae4b2e20e428|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-214ee716eb96421ba612597900f70d7de4dd8c4ffb57d55818141e90df850422|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f084d5de49c141c5a586392b4d3dbc612b6f8aca4c80989e1fb0c1cf34fdafb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f8f13710535f66e7d50117eca389cbbc5fe3522d9c7eb9b2456bcbb64f63cc2b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb0bd9b9a19df56b10ab6e959504366ff11beafe995cecbb94357817c6451229|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-850fd8d72582c1fe771d48aa857b01e5f5b1621431a6411c3687760447b40627|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e59b5f91b49d9199056eae2e3c7ece2cf4f3b2f6c3c63f78473c1fd7e5d44e2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-999a801d7f495645d09c410df68a050c77e525f35633a89977e6b96e3f8b79ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d6f4f0b3a0d0c581919f5684dc68ad020f6f74cd1416b2d327381c66434ac06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-43e26eebfec8d02fc2c406960940c9b7c3aac98fa164258822a9405e5c2d9ba2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a285163e597fe2a7cb2fdce083e884120043057dc1f799ef74258c53ad1c318c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ffca7e1e9615e3dc02afa86bda3299d2be1f2fd6168831402618e7252ac522b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-133f18b081851a85f8289bbbece4f1679bae07f9791535b752b188505f2c06cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-901f077d13faa8d2b4f186d9f5c8c736a46b7c90aec2519a066a03b440caec3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38d68bcbd9c3aee0912d94c120759226f0d2538c2163e581ec7b3787c748bff8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de1426a5f4dacd0da2a89eed00285a365f608f65a2f8a696e56abd58f7163df0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-abb5dbd587f58189035eced7c408be3c41339685be4d11b7f1269ff07f8ccd1e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad045ce54596b9f5264a4f02765760e5a09acc3328620cad52313440ea33822b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-058b6d5b7ea1eb7029763ce0da70404a74cd351e1fb814c169e9e85316c8bd74|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1ac7bb126da71b8a4c7ef2c16d79454c3c864cceecb058134c2178be4ed5394|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c99ce0ddcfce27446dd18462cff6d0e157d68714cd924ebfb00f4f551433502|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a05ebf38ec448b5655539eec383b02dbcd6e5732088c18122bf3bf281f928e6d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c888e883bb053407e2066fc2b7759fccc7c54b1e5ea920286a83e2855d54fd7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c24f03b6ce1db02eef1378c85156fe54d0e330e1610de80cf7b66c7ac0d05d87|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22361c5177bda9d3807f62fdc6118b53b3a3a21b77f0ce3c9b198c4ccc84fd33|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-abd61a505f55c7dac2c91705f70b53b512f7d142b7c3b9bbf5b238525ff5fe18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a4b4dfb6dad615d63a1b9b82cf2422aef295ec72ec1366336425d0a958cf686|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1098232c955791e182980ec0412e52a9d5ccea3eaed6f189deb85e5979364c0c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-381696c7dbc636abf7f806e6ad6c6a7038db890aefb3f0731100b204407e26d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4e1a5d2545b7fa3896ae476e0c4a727c35054c716dbbd4b7b7426a09d2b7134|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4b1703fc764524fbdd122ec6c5e3e49c888503ffb593ada34836d4abe73a3b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e4e1313ed97d7bab02f818e8a2b9f5bfa335620fe7ef90b2b222c05cdd3e5ddb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5568a48706aeb07f50b2e2767562a684c1cb3eee16604ead25e472e2558bd529|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0817ec16e99cf152941dcc3dd5093bd225514c2ebbb92e67ee960b9aab341465|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1958cf3b1ab85dbd79cf61d48fed083ea48f02e9bd556bd46bc99c8c3e2a32d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37dd765e11cd1b58e0fdf040a3bd07fd2d5b85fcd71e9e012b287e3b0d9c3e4b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-53cfbc9b82cb1bf621b3c392d6c67bb03895a1a83898f8ce515e351f987611ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d382311e26a7023808002dd071b6a9c58623489f4137d1a4d0f14bc1d10e328f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2dbe5ebfe31278eef2e134e9ba013bb5cfd672d693f4a960a4fd5858bbba4259|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1ed63d0efdf6c5541f3962e156199804a8fc12178c8fcc15ca4811fde92264b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18c0afd53667bc24d6adfd03372a83fa158440417346b24117aa025ed2908af9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68c91c5a425f85f3592a78e852fee0c19b129bc82b98c8ec80e42e03fc8ec49f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-714e479a9a33b6a308cad098e21a5f1e02368b3e2bad175700fc7cdce1ec65ca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be489b8be7909893b210d847808eee0e4ceab4609d95706e6599b3fdf42bd7d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fbab3edc438a0f362246906ec11868c1db1525b96a1e0c3957aedd57699a0e2d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1ff6bcb6cf2254205c3f119031ec4696ae4fdabf6c48e73d4cd31f1e678b865|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-127e5e8acefa87db759e87eca59443a8af0a7ca357c29ae5813ddec523753cb9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce6ef13300a9e0dd23ed53748044ed887c40e1032ccf708eab4423799df247cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-deb2396600366baaffc19f032f34745aba41900f5a46ba4ac4fe3b2ed611bde2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69b8bcfa6879d2379d90b5c23c122d63e0f7d4299fb3e5f8366531b8f7118196|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2245c4995330f6bb5a05793be34e94659f0cbff37baf6081272b1feb99ac6fb2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-432098e6e4d0cfe1729c5deb159dd303acd341cdcff98b0b984e54ef4e77c4ce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc1a432cffaf8af48526b1fc3efc4463b52dd35ec6e9c4cbb8f4d90cddedc60a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c6e6e937517ede539496162b0eeb695bb752096bb9bf985d63ba820a8278e0b9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37a5247e4e82aa0f1798934a737f3791649c555ac1eab8b0aae312f54fd29253|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34a2a60a709a59152af67b26268085fee999828dcfc36956307c3ffb363bd670|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cdbb34c46ee74d6a3e9c2d0c53ae66d71d7cae4070d368cba697f8981bd90b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9f74b2c4b464768cdf5e5fb26ab0678b8bb9b24593d1019204c88ab007cfd0e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6899b94ccdd31d72cc693b9d46ff646f9f8801f382b64d0ddc690444fb4e74dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15325a8bac9e5b9bc97af35d512cd95c795f5dacafe8ebef46489e6011776fb5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-563a650c03795e654c5ce0d28b5ee5ac6b1d2ea529d2f7a29447243644893246|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47c3319362f7168b3155807549dcbbb32f3a56c2b7f30e3c7ac80e9866e20839|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fce2547aa22e9dc8a38dc1f93398d70959388a08b66def0568a5f12639c6e1c1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc189128a10ad0d906b79ae565467f31c8e99389fb293f2c03a664f79a5bdc69|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-191dce769d9e2b5ad790cf44f8764c961a26869f8d2f004bd4e41c707c3e0860|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fad77e0010dfaa2056281cdeb7fb78625bfc3b382094138183e6a25dc49ebc5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1069dd55249a6ebc2065ab80d66ddf9400c2c4b96d833260e8c176cef356f2a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d5d906ca4562529263541354283f798846e7efe9d6330009f02b089af33386f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c752d75c17d5c24b09bfc531f6392798474252b6698a642987794e2fa951a329|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c81502c196d3b721506226cd5c101956e7da3d2de1bb991dc67dfd65c3b7454d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fabdd6e704fd824684b559e97c175ebff8c9aed264b357ef6a483c62f254e630|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-753ae0f22daca27caac825061109d7829b387791bf8083e7052dbe7ffc6d09e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55f2abd13bf0b4d443c86a3c2eee8b4ae22c6f7d73920e483879f2f4d5371dd9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fbfbad459589af4681b5885101b25ef5d8f563f7b65e425f421342669993765|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-26b348d39ff6608325cdc0fc8d978fbfa7393c575972cad68a62fe509f024eff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25886196396063caf07c1528119c6705d700f22f94a4c742dc2370789d75a83c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a15701ec7b635bd95b4a22d43c45d4d74f6a82869ef49f494cb9bf0044cee910|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6345b09e2a09448d593bde167f06890c90f480b63052cbb4197adae4e4d9a7fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c28d023e08db5ffc17c3ab8e958d28ec2c02737a1245868a70ab16927dbc1e33|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4a80acda65f21f7b58e1ef715f90bc780f1f457e68a59eaa50b7836935a5a54|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b93c54c0c68b4a5f410af7808d3013d9a08ce77584fed344964586e94a7c0b89|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f0579eaf780ff254a1b9f2a83bb691da48ecf731844ade2467352e5d1ef330c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a5bef421043b5515e8071002fb6298b969e99d07327426d70a2883e7c4b9d91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8b1eb2014ff58f3513305eaae0e192ac6d1e73f91f9c12cfb36da6a79ca45c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6edbe78c8a4f839e8b41a1c2a590245270852bef97d0dbfa390121e1db6e7071|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8be75c043d0b8895efc7104ad054d6e204808a306e5b6a0d170f8818928a6c2d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55c14767817351b87ef5ec9803aeb22cfed0342b3da1f2ab1c46901b568dcad9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9102814a6c9bef0657ffc8c6e1b3a24cc5a7438c62f502e77497a374747ac1f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41f9ebfdff5c5e55f792c7f2105d8c55209a2c1e219fdc57de7364df6bb1adcd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1026b4103dfada40e2f1adcee0e56fd24cffe8ccc79db422b4038c3576252279|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c0f99cd56ec19621d6d1527257ee498a0d2caa7c2897995a525c5197370fb37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cc10b396a7cb8350d44196467163e023149d61825b13baa3fcef54e63c18ee3d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-046c2359305ba92512dc628d68dd2f83de059277665d841a8fde02ca7dec5630|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c87b5a3f79534356f97e255a49dc0c00dc57be777505415d46cae7163d8b5c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-51a854848771713c463aa4ad29a0273a365d5fa3fc83d0059e6d1e24ebcbe224|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e5bd87792b4e7643385d0b368ac776b92909b03eb060a830e4532506696e7cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f3b17dac7e3aecce18468fd765d6d61f771f3aacf2b6d062e3f8fed6a4e8d7c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-46c98eb3e21c2d81dfec6dfacbe00651afc4af2556c55e0b9cffdafb6d5b23a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-454e1281678e802ab763601705d707a855399a97b7d02c1e1765b2e5e6c2281f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0a6b6a53f8b520a25a37b948995766ef25ab88bff717c0e071489904657bdb0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e749d460a7c5b18f5c17f0d35a5814d086ea546f13ae22ed55b3a46c808b6120|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6ee15deee01f573b160c7b2d63566b4ff294dacdbcc477689c43566417effcc0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-baa5836e2b7eb37452120d07e3152322d7dfcc553d87772b4ccbeb8d60edc7cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6b6c17dbe31eb826163d1b67cb8e62bd8db76ef1b1f85cea6124c6389fc36f93|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e1be8fb8473be124f9659a0a2445806070cb5933b11c516fe98674a4c3f51e82|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-63ef3b7113eb7fab904a7f7016d06b20c30575a8f386b5b32fbb49a11dda8bdc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6ffd28d937801d6a1a2f8fbd3dc56f3d9d385738f9393f962b8c86ca5dbace03|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57dfd48bd4b038a623fb5d402a32d4c5c83a21ff29a3c4458f42fa1a5ffe43e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2871012d53bfd46b66a25722e0ff3d98ad2d3ef39261bb1a2f20eff7e8cffaa9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3e8598c9d9c8fec503f54be526b5d5f1bb6d4873883ea5428f8706b0f7b95b12|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-871b7735e4ba526db9c49e7fe63027ed12286b1c50a0f896533aeb2e8b054575|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b30525538a126ba9cb7a965f2bec0142e3f1b1c3f592aa68f0897ceb883a1265|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f75d2c5762f62518b68ab10c15a7b89ae646353afafd645056a5e0dbfcfd0c77|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4983172bb86e4c6f917d5c3c044b25ac8a4a7f8cc74887eff8504259eab397f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a7abf3d97845b3634536576d3a9ae33da66c5b4ec7082873c88d47815a919772|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8382f61c02129d1fbf11498f872112eeb1490b590a76b899fc9f7e8e9aa54b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13ecb78adf0f457c377cd17de3494a49d4ba17894d529015046a7d5add562b56|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-32f68eae1147a384f9b8a2e3914f96d87dd82204c67c68fae637e272bfa7e589|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55d734ff598dd341ec6bda408d40655b0c3ee441c39f57c6de8f0639b1e500b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39c85d38230cafd9c8ee01b1cc792924b44cfae48a2a25eac45b7839433caf7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cb3a810f673091095b438cf1c96f168f959319a0ccf2dfafa2057952f5d3bd33|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2e2caa0163d6f69cecd30a78207af145f5ca371e8232eb26422f14a3c6e65e49|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d62ac7bda11dd6778b9384458278db036cb95c3a1d32ea4479a31b4d4847abe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d3e1e68155e13f83a24057f54f76a06d89099e37afde6d58d7e5da78de5629c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eb0a90b89677c9d70a4caa8be7f8f2d2a01aaa377edd0a15d671bb022580c207|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b66629e2bff626996474d8abb87d46b3da47fbae15a12a2d8ef583e3b4e06f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c7e2b1ef0ff0f170bc2adebc85525a25ab1639b1c343c545b0dc2dd7ba785770|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b840fc9d980442ba34b57c2091ce81dccf5c631d08eb2a4bb4fac8460b6f26f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dbb5f46b6b3ae45ddbf8c097a5d6d1b9df1204183e01e890922256d78e1eddba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc3ae1f3e6f6556a9007e2c3b0c0f1ea7febd1567a50c45c19268087c2911a98|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-edcb0fe9c74f79eea2d0fca9e2dd2f7f79343f9a29fd70fb16a8c69228567160|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-feecd5ae0f78e24f2fecfac69f0012475b7a66fe136087751eb18fb223e86af0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06b874b24b6a766fede8e0a8a5cd962a40dcd527052dfb3b48205b96a482df60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cdce2850a867c177c0c4d6ed762d924c662b75b8de75ffc0e0775bc56ea8dba4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38a79062f18fbeb1f613e480def493cfb331497aa3683f4bfe75524fc9783dd0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1529055b93032443b4627c5390fb616cc3b2c86439e191b33dfb223ea48cd8d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c34b11c365c9cff4b9184f467af6d3a255a238faa1171fde427f17fed4f35719|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ee4deeac4e8bb9c7c0025b8509b5362a81e4c1aeec2221f0fdd6fc6c802bffe4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1972fc6b80210bc60a7e599aeb533fe5a454bfdf188881e42f0b519ef9226d31|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2682d95b857b4e7950efa72d62b73bb45ccac9cdb5745b74a8ba707c383e2e2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-382c6417012ade317454b7fd5c36626bf66b15330551afa0478b1a7e3a431122|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-935b4e41710c00d67324cc75cba51499b614ff806408827af41b2244ec2d0203|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c78001ef4089c84ddd7bcc66f49a7bf03f7f3cbd9162461ff465a528e0f0f6ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec5f3eeafd45737df9a1f77867cc4211635095b033c3404ef6276e3011f09653|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-df42a5e09e12ad70294d0bd73f303c38d751cbcb6d21751c1aef05b2051e556b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f119f67eadb38e685e54c8bab54a6b9024fc3aca37167891adb99131fa40a30|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d484bf28cc6c678ebb90f0cd8d18e946a199df45417f46468e24c7bbc2563897|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-94c5de53f5fe108cf65312d055bf8f8b7188c63e72bd4c7282e555ce7351b971|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11bf2d65bffc20ed09b5f422db453a221d4855f8e3aefdb057c5e107728efb67|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-436417e0bd31b915577020d880e569c10b3a90853536e77147d39b9de309e64f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-627223aa549655b3d3fe60c9f1748298508647093b21f0a371319a87d572ef0f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55d5c9a09e40d3f2827f4a7016034ee854b999f1dd03e130587279d989aa362c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1625daed3cfb72976449be96734bcd16c604763f499b34a30cdfe673c36559b7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3628648f630d5037460f29f00a2599297e7f3680bb0b57b36e54f8052624375|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b794fd4b0a778369088b7aebe4dd159a90661c68047f19601ead485c9e3363a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-561064148a5fe91e3fd680aadf5e7ec006897e0efbf3e9a2e78b2a64b67bb14b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-036df33040f288455b98ad1a0f559fe68a00dd070357412374e5cc8526cd3145|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3bfa78cadd40467bc1bad2c477c6857a92e9ecb3c8fc14c940f15b4478686a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b92311a372089d0a63c610cf6a508029757bee14c0ede9ada8838df9cd1086c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f08225a5cfcfdb559fb30a42c7200aeebca6f89545bb9ba508f603b0eb517961|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-533f591f268dfa0f236c056264d306f3cdebb8225989bdd5fb8d1ead294579ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55f20772159b584914575ffaca526117f65902df9e602b04c6d6523fd064880d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-044e44045de116966401441bcd468106385f7e997d847201235bde1d74019d32|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40267b6f676e6f676cd493bd0461d716998af8b8d83d0e84e9d3227454bd99bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1708b68d9d6043cc03c346110b66a8dea782fa0bf5e7b2ab264d07a2ab0cb404|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8c042448039dc3f72c7d17d8c9a8fcddaf345f0d63e1a0f158ec4d542f03370|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f56f4791affbd9ddfaa809049c763295143287257eef79f968f2154521b7f27c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4700a283a2ce695121dfdb1d03ea4b3d92cd1bfc39170123344767eff820e7f5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d444d2c847b8cd518c648020749d9d84fe998d634fd6e2e7bc457f567ea98b5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b4e10bbbb3223a9481b568e2135c489feb0f12a8eac595e59d9c588b638abbff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7ee277556ec9dbe57beafd921274e6f5bfafe3afc489c6bf28062bb52772087|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cc9bf94cdd946da45cc7d77f32b57f1152f1220dc2ee48cf1a7d7243ef76e5a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2b1408bc0321ae6c9f50c3e727c50f8f7b965daeb5672395b7e2296eb2b2dfa8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29df5edd79250ac1b0c034b954a9eb8cac11eaefd46629762012e5193edbb0da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98ae868a30401f96d9d7d9e6475d8352f6547c924411124a5ab3071abadeee65|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-786fc90030e925f1f05a8a11a1e92a05606beb44500db4b1c8a1e8cb7b573191|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2bf0a9c17287f8a3c97d633b7d496c183101173e0635aaf1281c7531413a8fe4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b24246be4dbee58ff5f2c23bc927b9c3d141a02ac9136948309453ec777738e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c8975ba1ac1cd2abbd8ab6c9968d121df0bc53ab4fc349e48bd11e2b90c6ddcf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-105d53c2ab6a69c04266b858871d5259c9f4301c5038cd6139fbcdaf8f098e04|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c40d22b901f2a2659a88ce107ec989075881bcb5691d55a4b2675c71c7bffdfb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11e06fb9d8f120f050849a9b39c1ea292a7c48ccba397e74fd27c341a67ce8fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b4578f9663732a2e308ea7bf2f95abeae1d37ed27358057c719a485c7843ee05|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7a7ffe2f97ec835553bdc2a458dbb98e187debb2626d0311561eb0b921321be3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-300e32c512b2e31aa118cd75ad71c3953d75ce82d762bf71928a21e9aa15a204|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c17ce915a3b482e763b32e0e6cae9e50a8f64bfa61c941968bbe7be492d07681|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13587e067609201cf386dcf4ccafc518c0dc147c2aac912af9fbe989da55c6c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2380dac62df272ef581bef8ede1ff76b6d627f2f4ff591691dbc6eab66a0aa13|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-42076ecef4ae0004a0b6020faac2d9de36faf0e33fb5ac8209db084a015a3534|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-988baa2b8cf673b575302008cd4a5d1c1147d6ed07cac46dc16af28b4efe640f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-818680cb1c770b60083a7df4f6903aa240ef81992722c70aec3ccd0f809a9480|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-784905597044e4c5cf2eb72c7aea121e174a8985e3dcb5cdf6cbaad764d3863c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-793f85830faf411b0637d763c81081ce567fad2e7485fc04ded3762e584e7921|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18cc01efb73305691e251685c1a8bf7813d67d3c44e8015e1d409395af57536b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d28b679fa7f350adae7bef5b9b66818ed50741cb021fef50abe80614eb47deb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4cd0b476019ca2615496fb5a04a80c574006fd5d19fa8ce8a934dac2e6819bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7732ebfb6d6b11d797cf027777b60f49d3c16e7cba76588f646fc4029196a4c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5cc54d34fbc1e5374e3b42be87e62a61371a0b2de51aa5b1d270a198d8a68e7b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-76d0025dd20903f7a8f1fb74bd8150e77825dd8c66860f75f46bf0ee770ecde0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bfe2c23b039bc95b9c97fc7c4caac42db944b316e9a0f3ded159746325d861b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1354a1b0c02b3e25d9b4ffa864ca7accc07e663fb989973c04e5257434e7bda2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b06b6b5b15a6c333c0dce6dced143b8960d2e1d35bd7cd3a2f446afce00d1885|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f2fd01fcd8e3cbeef94c456487fdd37ae879e21fb10abb15f1991ebd9c1e1ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4700c55f9cf42ac977497bcbe1b142351aee923cfc1eadbde7e369d8be29189b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4bc770a9af2fc52fb3d15d0dfc96c25ac4761ca01471af62eaf14ae918bf0691|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5169a2d93c4c486a37dc9d08269575e4548740e12deef1ebd5fb47b7125a5dee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-92a3ec455a31d3b4fae8fc68aa0eeb70a2b61aa78067261c4c369cae4bbe817b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ada4c1e7a67d86b1ed74c3a7e9dcc7411e68df4c5b9cf82433d458d71237582c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d162b188ed893ecef274f16e3ece4c3d4fb069a1cf78fcca7d13a756d78f6b4f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4974b14db1d4b7b70a1c64c3cc7d337f2765a311f0f37eb750d9c446fc0129f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05252f750290e2f12eb3c089999d7343725494be3ad5cf706c3d2bfbc22c6047|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ad008468595f2ee5d4d1c9212d259d139935a4eb47fc124b38023317472e203|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-44486227f14ed8c084c4da227de76a16eb2e25343e4b7751f184f372711ea115|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41bae48862a7bee5a0c1c0a4ee1a69c4d1d679796c217e6004413e856ed1147d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec396f102c779aa31e51073935a781eb51f3b677f1b1e517ae6a4f89bd635808|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d0cda625a4fbdb768c952c4fbfb64363fc76547005be2985c8c5c38e775eb7a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-89538069cd2814e38b0f839ea9f312ca7d78bad351fa26114a0ec8261f6335a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3648518ce9489ffe1a44141018d9b053f339257c10c798c643d5a7d38f925efa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-783181a5e9719504ba7001633a91f51012a6ba2c5475ef3a9fc168dfeea5d674|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bcdf48cd8903f670648b3b3acbf54c894f5d20f239deae7a1fcad30b6b6af3c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e40f06668ff4ce3f6689da502db988a333846886cb60aad36060ac16507f516|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca462ce8b0383fe64cd7af18de97671395c008c47721a1afeeca5e0493f0d100|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-353c381cbf03f3582cd0048408b3926bc2c26cc4185a1aba23929bb4bf7ad305|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed3ce8093c5e82d16a540b3186cf7ce4eada1059b04cd549d0e619f9fea0cac8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2facd7c63851743a96d5340f55f606f9a1739f0ac056ec04b5bff44242686545|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9284160d33688a31a20d955ece747974e25dbd1eea48abb2b514006699f3f73b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a783edc2d34affc08f99802c691c2da826ef21a33bb63cc10abf2cdc0538dbc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-925fdcc4892e66a1b27141d0109643d5c59d53d192f112097a99173e7e2f9c27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b271187cfef1cdb1801f0b6af834c2aaa821c169ed42d8d217092b773417e7c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ce88798c9e867ad2b653e313ccc249e8a91013e35926eb8d298b50efc084f72|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-34e04e49de8dd57fe876a92ca2e67cf5b2b0e651f84bd0782908da1fbec77b30|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47be1003af0ab9f80d3a658910e7b20d5fb43a6170727d23801736916b75a4a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c3702958c52740776a544ae047f0a66a82c555222db555c5c9c3269e15fd517|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a01dae38be15497c86266e99a5419f36769f479db08564ac98ee4643aa8c813a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-72d779e85e1d7b0876b11cbcd96b22091653c7789be09c9765270172812d34ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61da9134c2830e45e4b2ae5f79eefee570aa96c62680cea5a294a24430f8b142|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38e7ef87bceabec7629948e23c7ebce91bb1c5794de4c8a2daca985eae024467|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f7cb4691a087d410b42bb4a7a6f63f27d001054fdba73a16801380adbe7e279|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d5d07de8624d7b33e7540b97bb8cc4864685b49689d1942b658e8d5eb53accde|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ee8f0c8cc55caa9590b7bb9692ad3ac03f414a0db17614cdc7415d4ad65ec42c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-522753cc0e906b4b16e69918b463743b65dc8e34b1a501d1ef1ce1f928182088|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b31a67d3dc2d06816d6b8feded3b2edf4beb60d9ffb1da4779e1a3d6385034ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0045161c0e9a5ccd4f434fbfedb1307c4ad413ea5e914352feea736e10cd7126|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-06a7d0243901ee6a697da84530c47d1aaf42206b99a2c3012e26fc5baa696462|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f22eb01cc56f59631951ae49918fee7445bcde7f313ec2689e89603c8edc4468|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-464b64eb6e3c88b3c9c08d0517b82fc0cf1e786a427780cfc06496d4d18c3eff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-63d08fb74af3dc998572a83f62a93110b7415e3e8eb287eedae7fa9aec0e5f71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70f1a844211a901b6beb34828d8bae2c545416fffc248f95564293f323f31d14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47ffd6030adad4f70cdbd17a7de65a7f32a7d8f2eaec903857e59b38bd557d83|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbbd7adc0aba5d7230e09ea385d53983afd339222b55b06f5330c7bf10be3bdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b636bd2e6a535fc8d41b5fd271daa0af36866943848b882d13b5dd1cb512198|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-146c78f423def4fcb2ba041456c8f783fcd8883c60ee6b8cd9a25980cafd6e5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d2be0dd1107f4c78c464c115485529389032699edfcfe77b0cdafd678ddd431|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-403c849f2506a5120447d9d9fee95d210ebba9de3224722e5c7f08c9f4b104c2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ce40018aa43f74cb5cc5aa32e15ab2465ca734386387421d45e228bbf39225d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12a8c060c2457d12d2e6034d8779d55dad1016aea0f507a66fe244dabe40374c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3f346c578c0ac55809fa00b4b9c6e7e178291e9b271731085d283050f51d684|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25960e65b3deaa224a5d7b5f0e9020a825db73cbf203a4c852e11021de136ab1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1716222ec2e14f819ee2bbc6e6a8cea65612faa0f2c5b6756bf0e5c3a65dca79|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30f7ca9eaa6d44143f4ce858be3d416d04e400c08d12799edfd3a83c06ced487|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc6b674442f577de4302817f923e8ed8d076fd5fe6fb25db971377b2f207d098|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9391ea1ea3660c52276aa1e874fb51f84c31cb9d7352c140d7822abed4b66b21|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dfadc8fe16bde71fb3d32297e4c9891bfd291ae3ce3ff82029f19956998bf0df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e427e4eea53540e47a27315e5c04a06e8cf53cdac9a3929b6d448eab459ac626|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b6e26f3557c70ce77be266d1c4a08f19a4202f768ed996616c7b5f257a85011|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69561b76cc3c6460292d5927f50addee9c8e98301a4967b3a3f3f24e484f908f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cb087780e349f6b0e611f116d86ab1f9b57f83bc8e0da65da12b0bfabdce0d4c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-860a807da146dc1e51d46ac7d3b7092d240105bedd1664ca5bae8c1ceaa4dfe7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e5f1ea5c82316df15a37529f80a5b9457459fee38ec4ff6a19a304eaa0f04f3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9666b588ae2dc215151429df418ff6f55d7016ad0c682904aaad833f9b29c1ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e70c08a822f13bfd6c91889fa7ba8e1aa7b9d93618ca967ab53d4dc622e79e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41e404e8ce12e849856fb22edeebc3d4d573f6131441280c67ace0e19bf0cba2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6e2c3b60d68fee2bbad85682d4d42a3042f6fc12e29ca6ed0a9b6f72d997645|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fd7b7a15f4ceb0f2ca1e83be1e4bcb776f2593e7cc08631d6c66bfb5ba04464|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-28f44966cefb73ea79b7aed8572d1f349a2c40c249c741b86a15dc67a760e274|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-94c960564d2b9d29a9904fe6e325d36c3d5e7b8d59c6408cb6f6be893fe10409|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73fecd35d89c791276e9b670ba82ae90de8de4274ffd249f33345143cb4e6200|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-858b2abbc892a4a9fe83e11fbf6c367611aa40ec23027527eaab80bef721eadf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9abf2fd17945eba2fccf6f30d46c9ed0496a90583c8ab52ab6ef8656012e3b0b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d953ad2af84f6a9858fab43ffaeec7846f7cbda4afb2ac3417afa91eac8e829c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de9cb25a1d249f9e658252c959f72a0078456dcb003a58147349cd102b13ee7c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9088095d73ae03f589b65e8ee6fe75d9573bee87754c9c0b55d074115e2939a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f12a3901f08786f4d93615b2355c6592d3ef2c64a71842513b97df15fe2df308|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7713fadf659abe969ea252f93878721be629d7bd85ede305c1c9702afa7f33ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-344e946e3ab67d67f34a89b8f1a8b0fce61775f7c51253c6dcd9fcd42d1c588c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bdcd1a2e48698f6ace57a0010e94cabeeb37f123715d35e038b11da01a6920d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f620c01396225aa2451ccab3ce9dbe179be5d720a4a6db76f863101c68ef83be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1bfc61849d06a0433aaacbd4747e947df2c28d8f118ef676f1d28f4ae208e60d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d647e294e22bf6260a9b303795cb4eb5b394fcbd721cfc4060b61cce66b8f519|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1dfa14c48486f9b2d60d35c36d6ce8f22bed88a3239127958f35f33e0ce5b98|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e1fad4a47d272cf2c636a04feeb04e618b4e8ee6340cf52f9ed653065e10d14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-46304abf466790ebbaabd7425e95f2d4d1aee16689c5c19d079f3df6eb60fe6a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e41f17334a689fc645c1af41a9aa8c6fa1a1b77f827556809b928878a825fc24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0b71f6a914d15033ec2a38453a9579ff7f57c8e0b0c82341c0adf880fec5e71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8e3ab2c12de75c9871b01ae6c05b6502cfac70f3bc4637439db6901c72b19a61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b9860ff41faecb27ad0d0b672b751087a129bda063e5f3d53a340119907be55b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-56c8c3b1957112bb78ac64c5032407031a14ff10a347d7745a0b0a262f7f68f0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd5475ca7e094de7ca452fb9f8168ac1de6ece9b197e76f334ff459da7be5db9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e5b33c3f49bd85f20ca23eacf794c43c53d9234f1658f7f1aeacd4a364c613b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f0cf35a68f6ea54aa410c53fb96bbe739299421dd2054bc16982ad9157fefe8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a26246923a972ec5de4e11976bffa835b8ef22acdd1ac4048d146fa77a0fc1df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3a309b67c7354e9877f680b55ccd0fdb07a7918e29403c35c093bd68feb118d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2dbaff3b41c096050f4494942bd234047b2b968d38dbd55f321a41b1418d56c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a8a4ac32b1d2a393c0898766ba01f8ea2974cc165994b09fa48382e019c04c1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-56c14b750ada0decde1a44035e59da7d4a8295c01894f8812e2ca1f4ce18ed6e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1dfb8228b33f648c546ae9101eba7ffd867ae96f552b7264e85db728a511ce95|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed71dea2babceb3b6b359f39093d177c76eca832f45680b1fefe8002d6ae100d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
