# PQC Migration Scan Report

Generated: 2026-11-24 18:36:23 +09:00 (UTC+09:00)

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
|observation-v1-ec27a1f4aa99f155bd8a1493f67aa24d30d242e7c9983aa01d3b25f7ee47999c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-422c1df4b40edb3b13d23c71a75c8aef09f828798333569012bbb22679e2d902|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7be561c547f39b5087efab078cd38c1a1a6b6630a7f3507fb9fc35d482ca7a83|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-519d2253534c68f510ceab80e56fc14b22264a4fd2c2a0d851445ace1ea4eb15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-949e21dbc730df56f067ba979fa04074560cf40c4803e5e08dfad0a422110fda|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96c6f2535176cfbbbcff42ab613ff5bbd932763f2f832d9db8e7f2c13bd606eb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aa76f5a731fc7da5ee498739bc321f3a4197b18f44d18a0f46112ce441acf03f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cc6bf739cb5f5ba5cc81a1371477ac4d447ad37cc1b43724bd37766b6940b9c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8780f53d5ac44dd34c6a3a51b884f56621db9b7c000ade4641e71bdf94acb791|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2731d61a7370e44fe489ec696e11538f9dc5c7fd8e69fac27d5852d45a31bbab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e2793aaea8c11faaffe5a105e5987d7c63cec939dd69b4ecb2d538dcc86d3f6c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d31a4379ac6ab6adf5507dd6eac043944d5fe6857b8bb7ea921725d7899a2218|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e16f1b21763dda569b0133011e5426b6f24e82682f8dc1e759ea55f3ca0facdb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-11ab3101624f9e0d98cad57c5d5a3438ae7f72f967404e02839d6b4d695e334c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ca816fbc22bde24686d648a94c0920d3d61e6f003c0ad8b7856d8c9496a8a54|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6001f4153ee7d99a52f2278a4563ab61b3f865e78a54a8f75c392d97e757c091|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b2e6378e90142ef5433fcac405b092f85e5a263cf30e7493bb49dc9583c0ca1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f07e816d974508bfd9286f56b1bccc1a3620057a3003847e14bb0767054f3579|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-848763ea87da69d66178ebee1dc67dcbdc78e51fbbfda29fe85b460fb02bfa1b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ccfcc150c89bae69ce7432af4ebe95f3c99724237bc5ea51de43799259b3ca27|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ef0b8525661edb34ef94603d7f5d91d9d71d7ac1049242b2dfaf074f8b19a818|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d04a53b2bbe9b09abf3715451ce8a446ff989d9b6ca6853dc7ad8dddaf0273de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a23d9fdf4f4b7767160189482b479b6f0b1c372e3ecb7bbaeda9bf2432768e16|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b6d313dd1c8c2fa0cd49d49d7b74aab846ec2367ae8fb676ce3187cdfe722e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f240ca57e00db2e33c6749d82d62ca5f0394ab1620a3bb50978593e9d672516|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52e1118ff923901f641060d208b4d7aa8653e85f2a4a2aef2ddf4e1da27bfd44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-81619b9359ebcf18e3c3ef2c74fee789755637191b8a88e7d5d2f633b034ba34|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d989e0ac39b5466bb702bfe52a6d5e766af74c95d4fc2506d1dbdf168619f5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e3cf2ee690484bce172bd246e8505e5a76f881d289eba50fff6d4f3d853f1230|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c74a6230646b0928fc85bcfafc45ea62a6537e1533c44ae4e58b607454b1473e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ab1587cb287cfec430d5aefad57ba10a908dca7ffb1f78e9bbbdbee43881d5a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f8dc5c033947a09e48dcad432ad3e90f10614627a00c8ab4db9cb282f7b619b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-75764711b249b7745879a3d3e75ec163eb7fa0d231c8419017ac6db3c0b33157|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4132051adfc1a58cecca689127f33ebfbaa04f24fd2b061ca87971a54807d0dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac9c89746f9d2d0d43fa8a71b1fba5824323ef58ee9df5a183ac2a83b1a59dad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-591ae53c49bc5cbef7bef5d45d5e65e4f175a8b17d7743bb92f1b2053940b24b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-734c29b6db492abdc7cb0e88eaeae6b3e65c06c58fb5ca92ad3d8f132fd09fa3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a1d9dc5017f871bea743c67b1670cad2be37e8bacfb40d26ebd2c503d272a1b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cedd236c422e7a2c1b186cebe1b88906c1b1a3645ca39a24fbc894c34bef77bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-62aea256b86ca0f952155f8ddebdd796ff48780ffe63a4a4e7db64ed17269d04|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cfbd8e2372573e61527f09bcffc066a022347d4185921cb0b578daa07a927113|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f213dbd84a3957065590b935029b474b83d8def158f429fb99f11063abf79654|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc45161230c36c553968725556c8c80ec14073d349b67982b3d50e5e3d929661|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4465da639ca2cb6171e4281a946c010b20830572f5e6ca1e2a82282d6525c56c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c72c8fc4c98f29aa6f893a96f2aafc502b982ec6893c8de24e30abb9667ea72f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d65392461b39e9f90a21f73450848fbbe1b2ff0d54c45e4cce418fc49b8b5038|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79134255a577f376549fe2cd2b73900ea77fdd4aa1ae029e0fc46baf24aafa85|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b1c0b290d02c62b006e3c501932c508daf36a892c1e6852888f7e3c64ee5a1be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d211d22434ffc15581f713bc7c1991ce144d989bef199c005d35bc3e8124b797|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cb3cc52c9031bf465a6e22eea6aa3fb717d679cf7a875f1361ee2e9647948eca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6b5d1749e8a2198059377d14f12d2a0d6431b27b2267d8ea8832339b197a9c20|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22954cb7d491010ec77193ae53457cb6ad0476c617052d09ef84e03c8a1bb88e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5b1966fd5d7d36aa47f796586a804b2ffe3faf1bea38011662db3bf9405e18c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8cc4b02ad836c541251f06b9f44bc563e4672244d7e29759bd2d1b8f53ed8801|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38c1267fc416caed3384de846baeed1e6112400c38f2820bb66ade87eb88627b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-17e5ecbdf1d9f94185348f557f8cfa2c499d3d35b4cc8ae9c9146219fee37a24|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-04cb3598cc90f107810268ad9297822889c47fbd04225426463b6e3acc948b73|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67e08bb9341cbbe3dbe9d95f02fda66360d10944432891a536feda4ee4a7831d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-efdbdb7e86ab3d66ec1f16dbb9b10e22fa7f0569aa787ec7984850f85cc81ab9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5d21c9fefd6214661176c93b9520ff0e6b5160a1aa26dc1d343b0157c9205a64|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-14a85a32769ce9493877a5ffe457473210be76649e31531ed819465afa3f80bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a175e21d15f04c4d9b76db7a168e07eccf0a0fbd504f72c0225410a2a1ed0c63|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3be17d7f8ff39fd2da931ab326a045c2ad78e27fedf277740fc31e7dc4277792|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dfb4aa9c329db940129ca3fd0951e7691b388f6d778b095f3ea239bd75b566cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f2c8b6ef11b962271c5cdc7362b4073282be01dc3ba9404a658d66e8df8fd9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b2d2f082963deea03e19cc61c27b32232bcbba8ffcb106564af186936ac4427|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fa0291bbc3e73b0689b600875b84cb328efa103fd076b372c7c51eaf22bd082|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-357e12bbd438a592c70c93efa67607e0ccca2836ce723938ddc7329896f6cd76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-24af12e52085b20f087665bfc6d8d3a53f89df71a5b35ab6d53773dc5302d7f8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f44066036a8097522a5c1235c5a6bae05ff275e16b59a702e1a10ff8fd691aa7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-670acf786154d267ef614f5d8bd5b7cf33cab27a67318c9ce3e6120b462e2239|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d309d518686e6fd9c21cfd2da5d614662f42d45e3a7c8bec7cf1e9471c410c18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db337bc3f991a6798ccb69ff85883af8b30f33eb33dccaf8c07b4071ac589648|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65a0d1a4c0045c06f350fd30d2ea3c89b460778e7797edc9de7da3574e189402|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-971bd311d3ef1061fed5ac94b5525b26ddcfd4e725853508695827db279e476d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0d4d01c4d8a6372bf81711f0776a32162e68d8165396a4b2215012542b717810|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-adfddd6ca10d38efa1f83e6532e99ca1e2db01bbbadb7080ad6d182a2decdd9a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6831a17ae012da6faa3f9fc598c90b4e2120b8db0e4bdae521a19b5d34e1c45c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7949225cd3411b2add862a4e40f8c73104b401cb14b791f20bf96e1a70093ea5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-37993c42b623424c38a61a9e3210d2a9602e39cd5bad4956ad4b1e55a10e6ec4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b09578812c3d669e67103e9014d98ca0672cf838924a08e183c39d7c36c7a877|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-44c683ac7c9f4bc7f308ab75873a2bab752cc752e108870a71398378b099cc17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7355862a95b575bcca1986dbcdf6dbb04ce8bebe8dee8a228b48231fc95938d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9d9b22d173a78f2caa43b0c379e4829e85d6b15c4a0caca570d0d48d99fb2c1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61263d643d6e8e51c1866c5901710723b32dd30bf70326c2ad6353c71669a625|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9bc6419837b76043cf0917193a1050d90ae92cd435ecbdf0719ea37c81437e2f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-672fdfee98c1024b4ee5e934ae8c44cff10c77dc4908e5a8504be7c0a7b4b031|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1fe92e31974b1ea8f88a96cf433b8319869be6a02d3251ae55350e96b072bc85|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-131a910b6f88fd816d2282af77071536da28323e7bd54ccb472ed7e1c2d1d4a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a76985f8b390c911d89d6c071f1a3099555221cdc31a8992dd670ea86f0f251|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-33bf0c208beb99e5d1ad768af1d3fd0bbda77e206622212ba39e0aba387e6341|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-431bd60c10b3cd4e14cacad40e96fd283dde3e7664cf30f8dd2ae0ba1e00a624|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-eb20bd5eb9da67fb562fa7fad1e5817483327c44894404e51e59a702b259fb25|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d788630bf66b39af0e059577ccc67202d326cf93b9320baeb84f46396da7459f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-524cbcf9776394919b778708577dde088799fadd7badbac71e6504dd631d275c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a0c45bbfb43b106c4bdc562908f6ab92768128d1eda050e7434c019c6d33bf8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2554872f296a2adb04c12511caca6aae69df1f688090738c26ad053da092e63c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e57a14990d158f995745ef8aef67e52d89e4e8c1d07e9e053e4f9246d0f5b72c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-28a8d2de647ecf1f9c0312b28550076ae66e549c725a4dadd640b44de79fbbc8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-bb00ef9c0520812a89ecc4df8e89c2eb901bdb77e1499079ad376e2eba742222|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39734978ea7dac7f61d2b25bbbec38e822930286a2bbbabbb893ab06b9bd672e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ab27f94097e7c6a878d8ca7569d55747bae078b5e00de3d32fbf291510ec728|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-957c1debcf7ddd517dbc179941b7c77b26f3c20fa696d840fa518e0e1aaaa4bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ebe5a80c73511867c3a5cd086a32845acd31bb25dd4cd9346a7c2980e181479c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-bccb3a21df12906ba998b21c7eed1afc3ad6242ebc734f2cb771c26b92347ad9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-511a9a242e95d61bffa21aff50e92d9ca15443d89e15917e398ad5f5227f93d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8cdde12ef66e5844dad711a504ed978b80650dc8570ce3f3731763f4fafc2394|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b2332a29d1904999951348632e59d99c986bb6e83df10541299180cda807289|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f0064d1ecd19f976845e7e87eb3762ae0000a40944f9231d2de638e0dba38fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2bc24f9b113429136a1807580f4166c8826674720b05e0aaa0bf4b8e1d5ef0df|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b38d6cf9592c0316e88f5278bc480878530f046914133289846e50827f12c687|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-359277fe6f0b117acedcec955cf3019402844089156b780553721ad4e7a55757|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f08097487dd1ec7390d3722abaded7450b3a426abf95fbcd28507f9fc2330c4c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5745b482a8bcda0c0e85e48155b2eb1ed79d5e5ef42ab89d8074a90c30b85d76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-883fcdab03f89ebfed4eb125ae41f39506e0f106911d21105762b11af7f84da3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-acdc7b8a9db022b9a5d66310234077bf98302559c6886ce51f949e87afe37181|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5789a2f25c76e4c1ca4a13ccc269fe88f37c09787b8da32f3c2aa5ca9e9284d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f78bb069e235e65583a8db5d140897f254ab4e14a4cc451b07c132cc238bcc3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50078c8c877a49b3f2693a68cddf64a881cff1327c76ed69a9c5d27af7167f48|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70132d7ed645a36528f3675fc9243c9b3669ff7019de688b5dbf6cf47166ab01|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-46d23881a1fe82764e3f6069af2352dd83ff74bee1319ddc526bd27a1a277005|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05bc942276f3dd29bbb8f56ea92747df1df3409c9ddd1a2890916ca249bc80b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b8f7013c169216fd9f007927b65fba8494860f1c1080640b763f3d63cc4579a8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a0d4bde981c2b3b08c3081f2a295c98c18f6051abb6b9aeefcbb90b160d93890|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-539fd5d0baa95a051756e8db9f95c2dbd256e52b6673f2a57512dae4ed2cb613|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1448457a88b2c3990fe4a49dcdb87000d05330290b5ad7269116f92c16a60a8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c5b289978daf652f6405016671e0bcf74031bafd6559a7a4c3b42df480abd05e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6f3d9aad19e08726326cec8e10713477b35bebb14168bd66ca700839900b3802|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9d65859153f43f0f27faabbd4af57cfc310ba2c72eeac7456f86c5f5a1a38694|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-da4d29ee53bf72523f478a7361e8258c913937c794024688aebed1c54a240308|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5119ebf5250ed7bde937c289b52f615cc328bf0bfbfbaa8d3b7a83cdabf0ae85|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d06dae4e51dbacc65f07c6e18e3487b5779c5d9da92fd5f909982c1f54be5d17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-939d337382f526d3ba3de0e4daa8c985b1588cb2b268e56ccff51266e159d9aa|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-11d4fdfa381310af299932d43ab7b67fc9a95eba92c57e2de7858c59c6eb2ad2|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8e7a2783bcd96156c80b481e506c90dbd7bc637a68053c61baab61c2061adb3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fdfcee0a55fd38a1b49f940da4301c8693a4bcdbd04720cfb6a62ec08ffe6762|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-be00ef3cf1070dafd90b421fd8b6196339e5f89f90ee20baa6ef56ea7e70ba41|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ef648e2b404ef7b9fc03ed6c1bcd4d72c1e27cec690c2e1ed6679317c6625c1a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c5b7e9beb871397461baf1f59187f1811ca77aab92b0049b2837bd04c59ee00f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cd2895ed1ae664433aefbc0cdeecd62317a44c87097fd6c4cc78604c14e4f10|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-30a944baec4281f3aecf654ba0f263f1e8572f7ebba54f38e2cca2f16dfec2ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ec3433f0b8f2e7d9fd3575911710f77c36d1383d4960cfc64aeacee067ceef5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-058217890769bcd085951ab41fb4919e213ed655390bf068d005e869173bc408|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1c64b21f51b2724221f78b75aa0152152ad180c5c01b69ba05799e17e5372c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c763065bd59e5b9b841ca64911f9ac3c7da462308da8ac9da3aae5ff55b442b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d82b85767f85350e49bcd7a173e0b7b1a73d9993fc9da96354586732c52dfb2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-53232a3f84abc7b730e1c3f47561c45d70d6374dfe7b974982280c166b752563|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b55a2bcd5af1ceb94d6d867bb97dc32269754278e825fef6c1f2f374943e906|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a301cdd7a2c3cb32f29c246c7bd07013b08a9efcc72b585e210a23c15044fb9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-920048ad4561cf85e15a6d2c75ec62cb344f7e81f6522e85a67f358ed22833dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ee1400aff064860a5a71e4d6cf2e0b305c0d01bd5c9b895c4ae4280fffa586c8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ffedba8119f3cf696bf09358af6d7a103a5888985f45e0bceca632013405bda|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-253a5ce6db407ec985c190d91b533a0cd49addb9e176b00f56a97a406d1f05fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c4b4af3e1645af69f1516deac3d595f66736351a28dbbac615fd5abaad60780|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e1b690d354b4f85e1589d5ecbc1614fed8c766456b124cf4fec3f25a8edee862|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-81ccd705103a418cb31a52d949339d8bd5afddf313634b119938703f43eb48f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6affaab0e8fe098400348ea0c31c370624f6b19c9821480e791e9f736936074|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-82c786b95257da3ee9c43eb9b06ad4698add2385184d383897b861cc7da6a45c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d617a34a955f589f91bedd0776fa39e9724c9ecafcbb77e21dca333e42af8786|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f2819c01958f9fdfcc44443ef3d07b3dbaddce7f306d4816ce7064114601e30|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b5599a3c233302188fd496d918cb297267e2cec165b32dedab4a1e043b04ad66|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a472992e36ae613fe79368d03487e462f637388e831cd523bea4f3065b76ca5b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff592029c29469f28ba67c3909a3e7e0d272ab8e4ea2641e30b8ef0aa19cfae4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aafbe7e3225d401c26e6ebb077bd13021daa292531daa43f65cc4e45a712e856|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d44143cdb1b336fa0ef71adb1d21ddeb379a0dd5c05c43debf4c3d8e0b85fa7b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-376912af274576034cf371b7a189298f47bcd9a078320f43446d44ca3c2d6745|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa006bfdb5f804e8c60e7ec900ad12ee0b9795bffd6e8cd9e6e7f6040cdbe16b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf98894634ad4e1e94473b1bf1d3bd0201ea787b0feb810d0543e35fe6947785|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3aeab9b02aa46fed56dd92731e55f842429ac8f37e50cbe9b69e839291c6a32d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5471c09f4933fb8b819861fe361e0c0a3e99fcc7c86848d8681bcbe60049afc1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d7afed62ad094b8f4cc7078879c5553bf25e50a4b964c0ab8d4ff02f1e725b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cb07ba2b7677fd8f0d1cd4631157ed9faeffe9401cf4561db205d8ba43347ca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-86b0c2f881e0eed02c6a88db86819537202f2d52159a70af8e8dd8f903c4de7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b2fdf4bb7b0e499764a328ff3adb250acee680bbf76a2aa8569fda760e94cf2c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4b0f95367e4512f671735223dd9a2fd25938e4204e911d426a47b886fce6c4e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc022c654dafd3c9978597d23d8891af5b80c104e70a5594fa86ca005d2a6c52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2652b49846d0f12d8e52bde7f4085874cc041963a6543c4498a569cd3e11e022|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3018e1beed5d8eba244f6dcda2e9a133fcd9570e2429832f99339de421023260|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-23474a7c5b157756efa508262e6b9c712dea27f7e316adff4a1ee66036258eb9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93edd83b8c0d795e3898aaba16be6c0fc3e80af35874eee32c06322c498537f0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1eb731316a7064268932fdff875c99258cff474df0740fb99ddc44b01f4e42ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e9e56299b43952216988b1a855d0aa7d0c16bfa10b70964f3313f7bc8e13ffc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cbb05f126ff7cd5bc739c23d5019c050908ecadccc4308728a27cf5855847cc4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74aac43eb2d4ef2d2200aa58fd5293082583b537078d17659cbce8b3a4c526cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-71d02a0d3a320fafff910ffc126a980fd23958807a66d67ebef947d778bd120c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85bc3e09c685313d1e9be0e2dab0cd47b75d364d17fee487252bc7fb2d339ea7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2681eed50e5d855abe8edc2a655aba79c620d42dc603470e5a7e518a57968960|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4fea72b78e07705aad1844e63ece9c60f02f2c2a51e2736d529bb51a706f9565|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-accd2510a4400297db77fc01725c4ef1bfcdde5fbb6b9e12d3bacf1114801768|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aafb0cb64bcd526b9b9597bee166ef72bebc527fcb7f93caf073844b15fc8d41|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5417adec0a1bde409d704325174a58e3418d19d80ab32bbc6f48e35548dd601|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-211590fcf8e7321afa23b589682299fc3ffc4b1e9badce7a0a7b9016d17f26a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e50bdabe5d42bbcbd149543c396efdb24cce013f4d8ad43c09e5838209dcd9d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5598ad76e9225a0a88ea9ff498e3987930f57d9b806d2112b47e1097605d6e05|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1cd6a0b4c49bbb797f9dca48cc88b300178f885c9323045807b4eb2aefabf04c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12d39ba8f3b8e3c39f5d0797cbd2b96cf7005191485f859758247b2e365479d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5ce26d461afde544b85effac18f425eb1eb9064ccc96133af6d36bfca14588e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9af1374f95b957494743ec7c1f7ae07b2edaedb48114e4b4e5f47699e5ad1a56|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aaa502cb4dd77d253cd9d4c66f5e9e87fecc02617752ff285a0acee864f13cf6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f10a010aa8ff1402d1780c5a396d5244c0c1c9649060f67518a1fb9af1f74872|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-17a32e9e74218376b49bd93975d3908be0ea976b7e1e4638d8ce83e5bee09368|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea2624f6e9f2bbd4859e87c1a49dd59e3e88d5e8e81ec728973c3947397f3f79|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-56551f174069173e77e9e302986370d2b3db455b625df532648fe5732a432686|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-75ad079f686c7aa1418d9d72af2ee9566764df23d848994fe5d9ea392ee343d5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c1aeabc4c2fa29c6d9cc81515a6e26e4e21b46ed966c2d3b25664a3e6c069b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1dd9834e88bc5618aa3aaa300f49917fc5326e83b6440d44de4062e6711974a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d48b298011c406c8252c13f46c1fa09a9aeaad66a1fbf18a1321d0ce88fb31dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8641a4a0fb5789d3167da64e38d3db85263f8bf0d6d42137c8db9bf49250d838|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60087237bda4d60c4b3db8e831174cc9e9cbf7e6dcd7897db876dcf566be5f71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-77cae1ea5e21f83863c597f2ad986d03eef7ce2068cbe6848c97e0c2a23dd8a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8478acf68acb99c8e30c02e1c193a6b99dffea55c8c05cb6f497b320e6f5d337|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d68b40704eaea53e0f6dea3304c41a5c7ae88eeb23094c1d3d427b04b518b415|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c59c0af39f8dcfff73b9eb36bae6c845da78a9ed6f6c366a1e6995d7f21b822d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3a9c66370ecefe1f27042a124f9b54d018db467decc33cf3af8a73061897b99|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-48b9eeba4ba635c3cd1fb8ad7a1b11628cdc4fb2e2cc9012c252d3970941a6fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-05e865db74b710a96cfbcc4804055998cbd572fab2c0cf1cb72e77d0748a848d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a4cbd9d344d5dfa4cf568faba621372b57235508973c163110534d84cd63bcf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-675a2fb757abc546325e53f3657ffbc61df8d9862443454679d8d408a9615c13|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4f84efa9a0b501c404e29e056a000acceb2affcd85c592be8b8aa7b83b81743|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-621184245bfdd19096797eb43a5ee933446927f0f2963a9ef687b5528ca06614|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e55904bbe37c1f4294318a4bed793f92d15b488454e5aa1114115be4449fd101|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a3b9e3358941ca5ee3f5a5a9b3fc16e9a85202c4bed026759cb77d1d112afd59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9085b6622c9c5a9b5fa63197d3855b234e283a70cb2c811388495be65c4e970|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-81548fe6c420209e02c43fa70b4ca03578db1fc0e8b673151911631944c18d95|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e71bd3a149b146a5aab04498bdfb69cc07fa3c9450408adb722d4818bed49c1f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b77ee13b55f7fab30a8cbaa1443e7ddf76907c1b3faa5e6cada9e1e9d0d7e08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c27fc9d4549eefb14436aaad8b9330bbc66d904fd7a199ff1b36baa4b6d959e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-515c598e5c5df0faaf8b77db51a9d1a7555afea13ab0204487116b5a98ddac9b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b4e032a0e4c37317be6da4859af6e4ffbb468173a16b50fa518c342a262be82|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f2beda0a5eebaee5af7e0a4260b7f90fd16b8a2a01eb07f43412ddf0d9ddd53|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00487a9bbf8827286ddad49dc7ed46ca950d2bee7e31074fcfb626f3a452d264|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c83cf9d03ad648730bb8737a064b9ea9544c55fea568bc8a45283dc680c31b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f08702a79f710da47b31e9d27e8bb079187641948ae67ede3764e9c3229a49c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-55dab60c86c56103b054718c4b0aa5d53cbc2df3d92f5d34a127bec8f1f943ed|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-456ca81fd558a9f5024d44f034a29d4a03da769f916546fae60c8fb5d8547c30|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ba24603b54586d16a79907a9ce168bb45871d8167bce2d3060fbe17214463654|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9990fa427c6349145af6176c6416f974f26ecb285ebe17b565afaabf21869f3b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4b489ce40e2aca4bcf11a94d90c6962cefcf16e10ee69b9e4d66aca570bdc902|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-04474e943aca6a43c71d06d6235fed0b9668b9a85a905fb90cdfbd8d95689946|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1197b121fc5c5890aa19ebae2318ef970edce50287a90bbf09a73fd905b710fb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-059f619fd41439d0fbcc20cd1e95be13cce1242706be89db7fabbb810dcea3de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00e5313c82bc9363a18c2caf3c42bc0bc32476cae11c362697c2b2bb99ae8de4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7abba48d9ba7961af94d9a94fab5b85b88563c06825968d6d013f5d3c4277694|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d0b69c0cf57219579b7ed224225fae8b62c31aad56979099ff8ed69b76dc8d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2251c73ef8c975aa654aa868e2a8e31715e649b49cc824b9f6c70a58cc23329|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96c344ee44b9ea4c068f9be8080849e659b9e63b96464acf0a0daed596ee278c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49e7ae23ca832d3aca6c87b402a38e58b0df778e17b7a319bca944f076edff08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61e7446a2cbe29423defa0a806b051f0d9aba76cb737145f843b31777280ddf2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fcbbb9cf1f58c2af12a002befde2786aa269cb7c111354afd5989333356661b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f77d6396b0a0cfb637d8f46a5db2bfae50bb4325d9301963ad810a0e440143f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e38155738abd080af77dba75c38f330584a038da09e3ecfac00fa445333fb0b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3b96e5754d601e2ccdf4ca1434a5a830ed229092b5af7fc432b1d01b19edfee4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-644078e9cc949220f3c325a75707c9c4e1840b2497021eac282c1e466d32e413|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bbc148e948b2feba10b300ad03361e5a261130427df3149afd3c530312d6a42f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3502c2ccc851289d383831b9519891d04f941c35db0fa6acba4dd20954f268db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b12dae9fb997d9b4510637b582a5dee779691032c5744c77617831cdc55d32ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-de47d393af0dc1d842620e4112b1af88b7d5601d6e9cc1adb16740299bac9907|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-185fa2ff86f85ddef00543f8c88b540dfe8629df846f3ba619c58f64a52433be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2102e84db8c1fae83c2d470140669a044401fa689c045b65f84b842fdae25a2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-855f543bebe242291c79a18d9bcb598a4602a49f01bd1cca2f07f4c68257cdd2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8b8c4cffb5c25657613fef4c1af836202e9a8a066714844bc884f88719949c9b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0fe18fc92e9229bbbb8b87edb1e95cbb7c2fc1de27fabfdee3cc88f605167af1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5d5722f2b29955ef50d86590faae01506ddd41cfcb77f21024239fc82a72a7d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-53581226a0d07670059d06cd7b072da28955a09da94dc25bbac094e39b40bcf0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ac3a96f22475f79f81a467e4eadac392f00db5d80792865ab9604667dcdaf8e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5ffac0232ab7ca40b51a0c714ca7140b52c3accdc4b557c87341e61ede16322e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-856dec49d17dfff3dd08cda5308416823bacb6e2a20387a4b554eca21e16fdea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-338d203098cd79532be0b55ae4fc5c4da0b1e19fcc58d7cdaa6d944e640eca62|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18f5640756fd11c94e5b766f575af72657f49ae83519916e41583138559e150c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-793f05f249b91e419f6f5cfdba5bf01f832759a3056d1defb3166aa55871ace8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c37fd1dc6af42d82a9aed2d4aa29206655874255de79edc5649a5c27bcea26b5|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-14ff82d40ff1d9c46c01c559bcd07e5f1b2c9f6d3a80f9753ef7a8e6375e02fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41b52467d3fea1a196790eec5f2f620b90f9cf28e7ea426b634c2c1bc762cea0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-226bdbd2d5f76fc2ab7be965961be4fd0826ca8a1a6af4c3302a4468df584d05|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d559c25e12ad9a0a8f264d25232c4d208e956a2d18a9de127aa5804c8f398f86|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bcf705a0a9b1b9d58b037dad163bd80e623e1c8f0dbb0f4da4c47dd82058a14c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ed975b72c5da91c07b39c57a460070b9b8f9d4b45274f20265e65b5a6f4b8d56|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-baa4bb882e241b8a68756025ca77995230445ff73be17e883b9e0f46038b4f08|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-386acda1ca3eab20e48be6b5e90d6b77d5908bc2c4ab8edda2a913325101655d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3feba96b35f6ffc27f7817552fa508c93d106e0edc31e1a87bcd2ea20aa16a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-42bc634e33c6d5308eec26cc7bb4acc12b6e68b343f38a0e876fc6563d5f8a82|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f1b4adddb04c71ffcdfac1ae7b12ecddbc0b032c30e38d6cd5394399d1e91550|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6ed8d30c4d74a85c3e418e6c7fbbd38aba7cc7f0cdb6127d77ad900754b2beea|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f7aaaa3d46d9f6cf614dd03326cf99fec3c476f114bc5566a474558ccda4fbc1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b68f04c542b874cbb8c0d031de1aab981ad73552b8caa2c0742ac8295ffe295b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36fb19d4d0000bcb80d26de43f05bff802f5b1295c4ba4354d34db0977e18f47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7ba301b2e0d95d1f1080367df8938c88a2c4ec92d255525a571a76a678d83299|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39d3efffea874c21e0b7ad914414aacd4962a2edb2d9192e283192b12fbb6047|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-29f21912c60856dfd52e42d9e161a504aebd42f42808bb92dba161bb7f9bd3a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7173e9c29e9588b6eb15976f6d86d0ab8c8c49b40cb7a378c19784530163f8ef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dda1b08d90963d69a8a0497b5030007aab38f10a11da23c7c3f946855b04ec32|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1118a61c5b9ac158b1e3d02a50a1a977dccc3ed4b28fec586b3cd3cb44e1d296|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-14dbacbee0c5120add20c41bc02cf3e06fb97f551835dd098aa8a2330faa5d60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec567ef72ab6035a953c847d0e7a7fc452469f30a33496eeb2fc5b003249ff4f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-39ff80e6e852e9db5406e1af98558605a4d308d775e34626998cc5a28ee26652|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b37923a88b24c139a67597c7e25b709ac875c7e97c0b2da5c885dcb0412cea44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d5d8cfb3b46b1db215d64ccf31f63d1c62b96cff10a25d5f603551d6fb0c833|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29464a9952b3b1911fd174dc93992dd2f48b92f473a21705f167a1caa055bbff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd0d0734a3010c723892e9de358babc15f1884ba8f6b5a854d8304710d3234e4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c422d62c0d8e5dbae21fd061c2db24610f528e3f27ce989d4fbe964b36901337|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e9b4925b0f49532da437c18d96ccf7a5a7c8f5292dca3e8f3b7dfb9d6c42b72d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68059d72323069f8b445dca5f65eb435670ee6c98c6917be11ac5539ab0869ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd439e79d69f0a2a7b0dcd6d3b95b8be70bc1e8debb367657a229c9c55013dc4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8353c698e2d7ec260e9c366c9516fad132215a64717ce33b4d1bff9746f9c7e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c0d3d271993f860f8d54ac85191cf82a1e1ef2829f881771d00510955c9d7ed|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3deb35ba827a95fbb4abb9af96b401daba845ec0cae7cd545a2bd55fd5d31a59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e3d79292b30ed8aaa7c5ddc18cb8aa444b70752113c44189f42d45d0029b455|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0ff057b7d2e7ad900153585742627acc136f9d5c2dd33bf256b80ea91497e3e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff20d3ccefad1f33eebb6a797d1eca1a96aea82fa7f6224f619c23e42d9452db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c7f336a2864755c623c8ed77733c839d34d9f83e1f8156a3a5c8785f75781b4b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f42909954626d85fa2c195d9ec3c9fd33632b297b8f0c77bab3b23312e8ba34|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc29aaf15f8acf8f7389803ca1d1640148eee9f6e006e807d3a1162363395e37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4c70d08fc2a9af32347d408f2b4e610c1829a772ee7908d990260780e911633a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-44d27a6f8418dcfe6db342a0c5e9ba925c45f84e862541b757c8a50f533f2778|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3bfb7ed317afe8f23f2c19a878d46445e33fea8fde37c19dc30d511def0e6927|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65570d4ad6816f53c99c89a6c2704e22e543477d49ee1ff5f6d01e517eb50e28|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-48c1f9b76d2800c795214adcf74d79893361ae13d78fee689b65801e091736a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3d1cbf9f7a70f4aa0a2fea8a16f32718a76eff909ec46a15774fc6c3d5f52ea4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f69a95d7bf0a6f8b94784e733e71dbd8a9d8acfbdbb94b0e53b24a3986d70bee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-61f2962ac6c86400489cbb8df6462d7ed168d677f08fe0955ae3424d5f9c85b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c346503c4c24bc253971a38d35f05e8a04bda4762484931ff51ffbdbbedb2ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f340caa7ec09d14deee3ad1588a5563cb8964b7036feab2593492cd56d43384|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-86b8d4b5317ffbf34338c17d466ec636a9182c3ca53423116ae77bab2f985dca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a3c78e082eacb5388c7de48c34865f7ce53e3ff5cca8e54583869a88ed3d021|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6513fae974093380041d507220385a2d059e10fc3896d9ae21403990a4185caa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fc5b32eb9af3f41972f08cab8851e1e7ccb8bef9a50cda6fca49e69b80dfd6b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-51aa1e568887d8f9a95fc76e826c48d99cc61d85736715ecc4661161f08521c9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c7e20d95deb824ca943c1ee8c78e6b04881e4cd2e24b15a1fe2399c800aed5b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-36d7288465bf73588ea7137aa7a397971c25ca158aae870df45263cfc43663a0|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-546b3973925694a79043dd5762c7fd4faa31a650c00f0999a56467153545a67e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb47e445b59b4bfb3be251a5a8aceabe1fc5356f1361eacd66abe3b177c0aeb2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c01e10dcfe843b4e3f04f8990aa3b72cdff29dfcf72aa58bfe32f2ccf343fc2f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-abb799930d88b5273b960113d2917efaae9f655f609c26571fd4392759f403b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f637751c67c8e8de6bba32e3776c3da386a7ee080cd00a63c4d56d07088ba512|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd59cde159d6ab8ef112e8528f93cb69e571c83b6f30e342714becfd95520ab2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65d16e9cdabfa98405b355618074244b79dafe13dce40e619bf404650b089715|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8c1a66af1244c162d8fe3794e29ef713bf47af72a024f15ed6454590157ba942|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a667879f5d1b356e2532be1262d6bfca1fd65f5b1ab88b8d317ea174d589fb0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fb980982af59db798ea51b95f1c0964db10ef10f8d3e2ec6c5d7e29deedbd2e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-279b419c44c9411901d4d972df7e25336b0f2f00dd41bd12bf1fc615dd52991f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db4fad225e8785930186378cd06d87ab935a392a9289544980507b38f2f14f75|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-031d44195a2d76e76dad51e15ae8a3aac6f4e2a3a4f5663f1e674084e8d2ac40|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-10c340f076eacf747845e3c68ae30e0984e035ca39611fdf2017b797a49b371f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47f9463e99aa422550f988b66c3b6972dfda922cb99315e3023a58ae3bf1ba5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c232099e717a1f1c1c10201d9d5544d11c19edceb8ed05c2b1ff6c60d7474167|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0222b0c632c1eb217a2b58282ecc4760a427ba47695663041eb4c9abada017a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d32138bc9c57cddbc8f06fd21d7b2c9b92fce274f62a6c7d55a7f95bf6b51191|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-413380688b7e652bbd9d76bf01c7f4acc7bc10730ef776c84b61a71d9f2947ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50588de6d430db1674dfbb185c22e1551e6d922f5fb26a2acf61e6748456f415|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff5b57aaaca7c5122ae566d35cf287d9ab7a2d084f6ee8b6b378ebf31c9ea21d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-90e3b689d7c475f789965df38f6cb9dab1c1f6e0d9003947cdb5d0b2d9afd5ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca538446cc5f3c62390648fbe275bf4d64723c725837363ba27dc69eb48f96e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2008f2986182808a2f4550cfe3d553d75d98ad4b54fcc1f9c70668819a98e382|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f21d81021d35fe283df87b0cd762918b9f18642074d4389aa3aaec720c5aec72|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fbaa3ec949dcf5f99797387e27939fc196c7182b2402f9c38422bc2ec6a97302|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64798e70ccdb998acf32a7958a8fe763f7a2f297ba8b71ebad414af5bd329119|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-89c328f830cff69f54c6a3acabe5bfa2bb09398af56b250cdafa2b5aa24ad904|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ab8a1f5c9a273f02b6aa30ee4a59797913e360e2c2ea0dc60912ba983c2f6880|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b5a5a1484d7bd1e4a9651e30bafbbb8138650221f8c2b42fa1a46d02e63af3f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-362d290845405e697846910ebe128ac0051b81d987b3764279b4a2698477de42|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd765da8df198c9f2b7cb93944678c7d70374a4297f686f5ab59865d704ce4f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-01a1f6e372a63866c29032f12928f90fe3a8b3de314cd57b72d585835c8d6376|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69b4e76dd92d830ededaaa40f1dad0ae1aa635cec0ea71b2b772e8b6e32d0227|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0df4ac31c3a685bc413870d3c05bbeafa7109e227c1f1750378981dcac0283a1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40fb6859a3f4c94b2ed686d41d3cac45e2c2b94ab238f24fee69ac6410ca7871|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c529806683d8bbbaf301f98c0d9c598ecaebda0a36d04e24ee551b117a3d21b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3e35854d3626d99e6f00535525a63aae0a2aaa6882fddab84a30d4cbcc46aac8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bfa3f47b02a49ce09460f16634fd1fe6a2301db799d348ec07199006a9575bc2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74da6646d1abd7a7abf237c8d19ac8f9449eca6c0747248cf23bde78c4db569b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc50e343f8c81133fd796ee0ef481ab428586ca6258f3bd7f485c459a4c69bf2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-09a25de89dd357e8e4a902e642007970077d7fc4d810e4ade8c7536932b68629|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-792f103d6917df069d46a01764d35cbfc806233ee0ad623ec832f216df237cf3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47dd69398d4abe4c47976e9a91300a6adbcfeb2fd68b83554a18bb08d6c20aff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-74273c021bac8feef86029d1d14c3c530b264dc51cac0700a1efb21dd237348a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3cc3f7a51cd5997be29bf6947cfc44910bd46554e6840e9a7c9cb078fbfc1b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e0efe754f54ab7ec99fa63cae9dce490d3b4d2b9d2838306f483531d3c9839c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b982f27d14600d6fd61bba391ed79602dd753404e716f985006c74e2898d1522|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-51d056b01d515e00237625e9b75bfa8de92d1351090a93c46c33dbd6cc9e8ce4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5c3dd0e3e22a1df399a2f088bfb2a3ecd09bebac8f0c8763a469a0a40649116a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f1e2056e8ab9d9473e7ef9fe102d089a3219c5c77841cea192c9cc33eb7c090|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c5d76b731216f1cfbd289289f068dc38bf00f057b16bcb18a5a29cfdfea28c1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f49752441646f586021c5207360b41071181e77b73ddd38ecc1239cea3e3143d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70270dc2ea8b44aabd15cd8fbd88e9e16851d08f8475192af6710fe415ffc8f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ef3e55de5e1ddd508b948e2f318baea53187b1c24d7c078cd4d50f9382f57ffd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cc80b319973fac52eee832f883a61eb112d65e678e1cfe4440b65358d3a71b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8713e943823cb4a2b1803c54832df29b7e26904062b0d467a547121367164e76|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-befa714856cc7927bd775de8abe6fa507213b4e4b667dd29a522c57edab319d9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b53d6204ccf5e97de630b38246a6e45d4e9bec5b297974e41de81c6412c1a973|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4471da087f271547fdebb8e14c30bc07b27517a5173738cdf428aa8d3006096|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b98be695264c7f72142aa534c1e2ce46a0869ccdde01bf2a6312b596df1d3de6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c13a79f3a35f9f04b8d27316efc69f530fde182496312a3f910ba704dca6133a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd0ae26dab695efcc5cab27fb8f9a77361dbfb49107fd129cf174f860eeeda3e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-888bd09415d7389a3cc3a84ee567a5d29bbaf1826d996d89846e8dbbbfd1ff62|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a2a36217eb2447e1831e99304dd433762e377249c8ffaa088ca785a4942c431|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e507231c9184ef188f46d03489ed8a373b4671af0c6b14abff1f1aac4b1298c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3152c153302ff03b9e95fc537d285935033a62a3810f82f8fba21baf5c513021|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2476eb6271d40c487d97a816dc9af8aa1177de6dad6eacce2707661a3654bf44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4aa85816b4c018c02b99e4443cb72f5ae243447c5a22670996e82d2ebfdfec60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-605a8e40316801a9853ad4f8a9a7a90c728b52d6d3b953fc2f04a94e69bc7eed|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ac048b1fa5f2080130a806aedf38558e4323cf18729e84d9bf8c27f1dafd6c3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-285333edb004d21d9754e848cc3e4b3e89f6aa36bbcb886077ced966e52d1db2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9b85716914c192864c7fdfc032427783f9aec4dbc787ce2b17893f5eb1bf1a5d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-402428d8a12515e1ea6390b2e32c75d8fc2240fd70d8637dc7042d7e68ce09f4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-657d8bd34606300534b46516505bd6e5b8fe9b45a4967a39ece6eca8ba265e1f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-40e602601eb804a22e213342801209cf663abc5e0fe4f7079284e89eeb80a668|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b5e672cb8baa7a086f1bb5dec1b57a75b2d54e249f36afb411e163ae08284d62|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b76555839bb306583da21cab1c0ad709b35054bdcf4e8f1dc427d11373cae317|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a24a5442dcd5624ba2cae40cc42a4f2dc09be941c60cefffa2c0d76d7fb6f760|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6aa95dd214b698167b2289f772072be31cec5b4502c35d94aee8008430c1916d|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-cba78150f6660ce5aee9b23e0f66f7cd0a1374141366778afa84b2bb8998dbbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5fb59d3e53ecc4e6b94f996d20b3e717785a3510636fb52646a8fef0e89648bf|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3831a39510c9765f28d068f8773dad3ca059fb921feb157525527e5bb22dddad|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f2aa73ff6721016737bf3331d4c9d308db3fbac6e2bbdb9310c4b356cd45ebcd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-776edc6f97bc69d423bc3ddc427479d47d6fc1d41a82579c95350e60c0e69f11|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31265ec01692c97a8ad9800d07034030955f9ca7e7d613f0158dfb8e338523c3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ccb1176237327ab1c90eaf3e8ffcd094e8e0025565f5f44206e2f08bc29c00fb|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9869df18b4d7f032ccd14170a16be749afa86b440c6e1b23b237b80e16d33f19|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67edb3ec033e618ab6cad752b3e38ad2bb7377b3106f55d4198ca131540f5e68|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-dda16ca7ab336337530ec0e326bcfea554859fc8f4287311e2e21eba2fec2ca3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c088ed9bf1ceecb6fa34961ad9dd3d5cf7e43531b0cbe938ee7a2cf788eeb448|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-62a87964e364139466e0ce9564e4ad0f4a6b31b79d108b9ec74e62d219d93a83|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-80f5d51be802e4777e27e2f84b4cd23a53e341eb3b87ba9cf0d4da4f826425d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0316b462c19a3a4978764633989656243d97e3d6f1c8d9a028452bb55d89e2fd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6b19b0df15c2774674719a23d73016f2ff365242dfd1cffe9c9822e384deabdf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-39fcc79608e3a317b3acc8971c7a64bbcd549247918e27358f4b1b09aa471df8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b7d12864618329973c5c753683805830c70d5c532c1eea18d69a01065e1cbfe7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c639250c312d845031e3a49d7d355d9e605ed1f0b05bebba1c3540b6d52b41d2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3222d5a36d8f7dd8671c0a501bc8b8b1238bf010f0d5137aceb52e1e4b6dfe1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed84cadc14321a3a78b1874066a632147503efcc5403c716f50297b626798af7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-237180b4d1d6dbf5b8f0d5a08706268e483dd487a67ae8a98c1f7c30c5e18b7d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7194130ddd0deecce7c3a47852ec042e8ad7ed5ddd497895a9abb7b81a419fab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f695bbba5c8da27a84114083ea27b4f1d30ffdbed01852a1f6d9f06e41c47d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2011073edc9d0e1800687195760851bd734cae95d6ca819294c3796ea5ca947a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d2e708d768fb40480b2c5dff358690e635e9ca7338c9e0ac5580f183f6833ba0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-93959db4f67a39b5184593294dbb367351ddab74e36ca375f96149c9ca0522c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4096f002adc3aa2586ffb76377b8d323b283a8a1270f986552b9e772462d99d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-666e3527b464eda4feb7fc34ffcc1184271dd4836f9324d3415430a4c3128303|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2485474149405e027629166ca480298478450a632b9522ca4713086b2e1c5df7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5c0779e0b27366dcdb04e813a9bc7e67828de1d88729cd622d590253a38ca583|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a8e9a8792146cfd341755774fbf2e49cb46b92ac300917307002d3b4e4f90403|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-41b8c5bd539f6d884549243c718ac8f45e2eb8f66477661b197a4aabf9243b3c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-10a80127d2b1ad495c44be293d3c1931e9794884fb76d571f985a39b58fba5b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d56e183757da04b245a37fd6214318e8fa2acac16eb89557376c7e0dac94bcc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-440a1cc648c450ef10e5b827687f7444fdfcee9a36466a1ac98fe387156e123e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68cd30ef2e393c26fffedeb443941229e9ef3262042d502f7e04b4e5eb433dd4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c8c16d1738d1a4d00de05033a90a209e2cc80e71187a21ecc9f34dfd74789c26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e0ea20d61c1e19636932665ad1a1a10cb68d3ddcf35b96f85c5f39f17290f88f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b3ebb6eb1e5a7adbdf4ea9d2fc1049e53d8d2bebce3573fed3c19a9b0522c872|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e2645e35365fbbd36e6c5d00e334ff187d1a058b85886c9da189824c4bda3e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fe1a5caa57c69b3fe45620e6d4f95d54aa0590a2ece12429eb78ea7ac6e9d89|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4fe8fd402b7a0ac11909f17de564076060d199440c0ee854fd41808e782c0339|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70a90611c0fbcb29ffcf67d0377cad8d67975e16a2c1882579c497f6a51b9647|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-31de9c2936f74d1ff253d1188b53183bf1aa5cf14b83ecfa8295c89bacd25ae0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af955dccc8a5848c3ff6de23fbf7a783279a3a3a149d39a7553620b9c5d76b5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0125929c6989998a26ed18dc3e8ed1bc39081e7fb37057f790d927e752013f31|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3ff69d2143d9558858f2d59aab4b3b24fcb45c5229546544561b5b0e99d897e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f093793adc9b3e04ee379c4458a1b76a06cb173b27e745cbcf5e33c6a694115|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c0eeb6513d51a6af58b1de8f44eaed2ffd89967539e602e0f8495cc7c557b5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9fcfb76dc42a698f746854865847f42c27c8e79ae514d27efa243053ba88596c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d02544c29c4b301ea23f00a88ab66e98fec417736cec77e9f4ac0cc0bb9a1e82|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9ff0a78be58e6cf3bc88202a37decd9cfb799d9f73e8ce33ad246e90df6c467|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-822f71403d110a8c4fb1227d96398937c2616d2d7b890ba153045201e173d20f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a42434efa1ad5d569128c37a602463c56ce5baa9c3d799fc69d05142c834ac6f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6c67f6b5a838f12fcf629b00284821786260a8a27c1fb54c96d6a3f8ef7c4a06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-301bfe107ac342e17330be61d8031d4f0ddca390d33c4d8d8ddabb6df59a0b26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f29bd5596a2b461e3e646706b1b57312418e9e9c05674effbc7508bf7585e27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1ee7498ce7e6e6b27efdcf9df41fb69a61486bc2e49f3024bcba7d285df4c73a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f0a5c96c79f80e26cd958d204a88a7d37f2da06355f8845bf1a2292ef8dc2ef1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa2f873a935d175823e2740e005e7b5c6d4101e17af0ee55fe673e3928e92c4b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7edf2003318c41838407fab4a2dce50459cbdcbbd554ba67b05e986945816d47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d7ff20cdfcdde97e49d4b68a65962d81cfc45338ec0b7659c3d603fd2c13fe3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f02b5a9c0e2bde783d110b4a84c12a07a8ac2489406fa981af151dfc0179fee5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-339d48840df0519252f1e450c1e1816b1ed87129e5a3898905133f797edeb548|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7dabeda97d74113c09a9a7d8e5edaed659c45d3828540cff5d2b961eb272cb8c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1dbf7b4c047898c604422dc59a5c0d48c22ea7f074614b6485964cd7b609a3e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2efaf67f995df8e55aba62e6e8d93b7428e676da83775dc3e951ddf3f38ec951|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4cf75fb6c800fd9fa35b0e02c908685fbede32ab356f97452fe4b7dfe2cbc0a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-499c8c4186665b76f95a1225ac7b2a272d3f6e05706431f9e2d9f81736aed09c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f473f63273d78d1757035257993465f00ff193fab92ea8d7a98dd900b36bafdb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5144d150591bd3609c57f0b59af3abe7c8889b53bcd0ff8678398b3fafb8050a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-edb55aa73c40b8b83f42aaaf1e51da437d303561140a9387e7513816635b342f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2e5414a1067542ddd2b92079dd7669b2056d4d1cf1d54e3acb258abf5086d84|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0cdd8e83fb116e428bd60b3810f01bbe8f85fe9ade43446db974f38c2324f657|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-95baf61867235b369edddea37e502ea3bd01d7b4de92953e25d7f86e3215a41b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5dfc654e1d2ee64afd8adf0eccaa9ab8755fb308b71f0825862efe5d484c85f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49e4a4efcf2000c91f058d1f6e2b5aabab79dacafa16368375a05b58ac98b5cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-354e4c1a0c44236f34588b44100b18e42d434c4d46ab3c0a72d5639ed7f3cc47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1f241aa33935c81484f90d9ace34d9a1fbae34ac182bc02696313150ee97e5d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be31b9e817cc8f80ad323563e858ad939600fedafdb51e6a6aa590f3e5bf257f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-20223459a23a227eb4e225b7e50b03a3e54c4e259f3ef599a57d0a040649c81a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5cca7beb69d230e7d46f946436f3d0fc56c20c561d5d4dd1c802c3ef9d17470|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-365b57c169c01ceea98375f86988311bb28901527ef5d52cb7375d586e7f584c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe04c376a1e521c731a9eafe85cf327941ba336c8bdc99c919060d0b50325042|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1a696e9ee9f38d2218a8149af8e855bc40390270bdf2a30560946659d187c0cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2394d6b932644373dc9416bca509ae0ebf2ca001efb4652646c6285a5258e491|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0a14fe18cdde02fd5f68d32e65f10b038e071c3283ac01ee7596c97e46664717|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-db57dfb7e1d11551dbfccc9bdadb08ad7feee25faddaeefdb6018ea3904a4d31|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0bc81c3b12886d77ba9f9312583fe622cd33421b2e13e7fda5906e73b460ec37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9719078615aeb356e545d8149aae97c5dbff9c86df49aa7100a0253bc2195f36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-caf2f7199d787e321fa35c0210ffcd30c81f8457b6bb17aa1da4df1f5af77e40|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad023006602bbb6040bcb7e2d72ddf7ae621a58184292192fbbf082b7e681ee7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3bc9ae770942027b000b29d13fd7a6fe1f56d0db79de1174575578130d531057|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd4434c32c6e7a6f1d9bd5d4cc2366ad83f6ecd61617bec15d29127585e6003c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6fde2e0dd41a6ae03b7151e84c86132c499e6de8383411864b42c860e9235e0e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-35c888fdd0778b03fa077f1cae5cd668bc1a12531769c6b03b1c622adf36bfb3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c477085beb25867cf61905dbf937350d9796c45de14394e56a2b3d33030a493d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2193d2fb89f7010d4d53c9eb0e3f1e87a67a624a96580e5e4ee32d5733cf4871|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8ae085aa4d66452c5a4da42671b128e85deaaebee9554a4d281664ad6a15f212|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2da07155269e0b3ab0f2f7df4f2d63070af056c5c6d54f93a3df1bb3825a2e32|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d7e121106e110f066d9fda44a4853f7d36c93d89a6760668345daf06bc4f6a8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-480edd934db222918220577226ec7fb8082182d0f70b6cf5af2c00c9e88679d1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc56da130b7e3eb89340ccbd34d27fdc98c7f62951ecd05deef8134d5c5713b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-19145b3b704f23f234a1813ca2fc5fa4bc44c9b20ded44433bdb7eb8d413b3b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4fb1e47193cd47f45904761d3a19fba14ea464a967359ecc86aa8596f83c8499|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f3b6cac4267eb633b0fb6f8ec4a8499f0f6a8801dd347a2a9af1fd75194784fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-13b7325fe47faca337b237c7a118d5f6aa373dd43c25cd038f5ca233d10a59f0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-030b35b0f68c359c5c72d50a72edcbb4e1963be238ccffd8b964b713334a9b47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-15d3ddfcd79d08f29f25dba61b1d9dd4f69ff5eeeb8b462e85f9412dde8fd630|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f18c783870145bc88fe98a73b21d8be56ef439fb12b6e7e305c6bbf227535ab8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af80c570e4b7ac8169b13300cac27f141598d80a31f68094667bfecf4f367eb0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6e400ae53ffa170053a7a0c6443f3df94bc2d583180e288bc4ae76ba03618306|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-54375ed72b828c9de20d567c477d7d53bb469c4839d17b51bfa959f4a77c97db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a405f6f174628c17bd5a634ed977d8f844edb2df8087b4771e5fe97eb019f20|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c81c5d1e83d58537ce776e1b25b7bbfbc1c407c13633b2dff2891df6cbf6329b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-95fce349851e0ea9d114baf9ef1116d6b0c71540de8fc1b9f4c65276b0c1fec2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-837c93acbfed125abddc73c4ae09ae79c5e08813ea9c01162b6c07b2829bedef|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c0bd08b0e530d7d0f64d5cf40930a3cf805f84d5585b46bb1995ac9c0ae292b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a63c2545e4af5eed274d59ae1bdf7ae3b8f3dc1c51f0b7ef0d523bde28cda8c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0b7fc61867a08a03883227a93862e9595e2d95c62c3ad82f9db6ff1fc615ffc6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd3c2c2a435258d3258d3f915aef0eba588f293b01ebfbd12f8af789f6a875d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a1303ac23e1730d982d68cff98997d37d0b39a665427499292f1013ceb25e3dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ffd9a11fc5dd7c3c8c9a310b58edac996061a326d4216bcb986188356543105d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a6d8ca153431fb408406bc9e9660a6e8ddb552f1dc320b0ff02425264ca50f60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9635524f8a1e902754a04bdbcc621b1eda28719ef79aab55b6478aea37f0478d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3f3b3eaa07dd13139cb94a7e4013f8f9bae24b6efeefb6d79212eb005d80bd7e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-afab2cec98b04480de894314a4487be303d46e77c579836b03537396199f1082|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-87d1bb2ed6c449ba1c75589e37114eeff02a9f99c043cd849b1f99770c670727|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf90da76459cdeffe14860a5e1f2e59ddcd3771272d46c49791f8d69052210f2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02d18330538e2eef3b0121deb00b9f5b34c02ea36a97c27750dfa2cdf26f6e1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8203f9756a307ea22a41ba20b3f3966368dfbb4d2498cd7f230fe200395e9bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-626c800a353d53c2e8d52dab1ef2b8f3f45bc43af5a980a4ad6910063a6930a5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8bb8a857cf432b249d697f8d0e342e306b4be41fb19dd06bdb4e6b0a21c94491|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e97f5f1e7e941a810cbfda87be0bbaf3e2ecada941dc5d0f785c59db0e326c27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-35bcdb4a048b45b827f57a3938c29861bef0210a82afe84efec1b8cd5045f96d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e24e0469cfba405b22e46a189ae95594545aca4638109329c0bf66b91661517|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-89a225cd154b910e3ee05cb93d65f920eecdb4887f3afda3c1b2524cf3e20160|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4aa0fc3cb12234274aefe782a4b555d26c9b8b307f10f439fdb91d46f02fc680|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f572220499c61e869f47faa076cc0a926f24e25f45b60e96a1598011f3e7db33|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a69384e74add227887086dfb923051ea696ba4b142e29f0c6f5a140f821291a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-259d4667e96385acc7dedd96aefa7d388aa0ff89031025b1f5326d0cadc52d50|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f24e9740849ab89db5b035724093fa5739003830e691622e81509e4cde2bd3b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5e04ab4bbc54733201b21c5ca53375fe3a5cac4f40042195a209e0672f57ebaf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e604e9864c44633858e3249027e01f568b8161c4bf1a72cd5cce5dae700484c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8a6135bde214b7a6c1f6b0bdcb60a24bbd40cdf280d516224a76382e6b0fa11f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e90703041a422f511472d57e640b5a5b81990a4934ccf17bd75db1474459a5bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07c6e9e50b7db1e4287a1f978dee08e771f597cf9fe7d8296b488d52c519ab50|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ce6e27fa52076bb67a51bd878c0e691592553b4cd052a624cdc0cf22e56318fc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6ee2d7797df38ad35041cf130ab0b35de331d051cb6fb8569c14aff5eb962b00|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bdbcce590706412271c529e8e6bdd4a22e76b850827e3409e4f6437c5e71ebbe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c35b2176e51f803e188399bfadedfeb264bf98816f08f6f376b4883e3cb8ef2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-328f00fe61296e349b18d581e9faab3325671ab02b5f5a38aff9f8c6648797a9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e8730f0379eb6845479a0ce808108bca63d339acf714dc3862c168ee21c285b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f34e1c41c34e4c2cf76be6b460a240184889d34a72906870d5c63d754df1500e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fd1fb23d6cc5a90cda3be4d885ff227c05d808ac505ecf77111ca4c86b83ec3f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d5a0ef6ab715d77b73c485df3b582f5c1ce864b601ca3df4cd0e3a466ee3ded7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af39758dd99e48c37992bda847b541fd0b39713572b825a91a1bbcb8e91e4e94|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a08c6da3a1f99f4b3e20624ffe0d5850b39d2e6dabddbae53ba7f6036aec5fce|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1646642497e83055a29c0ea7b7bf62b41c0e1a116f943ccb4bb2b4e5ef2b9aba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e490155d8209d0bbe2db6fc4993eb53b9c358a37b70bd2a197dbba958ab45126|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-787918621e8e0d0218fd74c50b98face84cea2a68e0e0bc63902e6905b3281f3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-84dc253630649a500c7eec1c6c90fc1994059b44baddd400b8920e1da8df93b2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a7f13a02589ec863b1b356617276e64974b11199258e50163d251f9695b255c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f93fe6f210389b500864f188604804bda1a21201d4adc65a3d47e4e5ba96900|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-482c9f87b960a90e54d35419c13399186f92353966cd0f5b9dd92c7291fe8ff3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f5c6f40c4ab0a43c9fe2863a3147aad09b691cf0ab508129382fad0c863b9e0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1b9d7d2a4b914fc0b608ac7e006e04de3840e691ff246cc645ed895d3af752df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f567ddb657900462bef9e6e73864a68f34c93be7de7aa2bafcb53676d6ca00d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b198d05165e05bc7214a92d0b841cb3052c7cfa9faf9b9a3a9c57ae47c297cfa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-63fdcaae99ab36557b00a9dfc829f3804aa6daa94733ababa9fc13492bfc9221|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a3902db18d065ea1efde072f50423e20009caaf064991063c66361bf86a7b93e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b880d67cc8d54ab5bc2e27ea7addffcedd6c7283a441072298f3a527651d6aad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f53876cb7713d4b8ca9c3035800b1e48724387e76abe40f5c1f1cbffcd42aa91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-86c1effac2f869f604f9675f0417c4f03656592e9d290dfdb24592a26bb7113c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8750f4dd366cab836dac931c6109b1674d84c1ca94bc03b9bb23585a08c2ac57|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d90aeefe548b59efa7ebf601ab76b00b6984b0ca84ffae326d747be04a957e11|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d83d90a5e31e7098effa1a5d5d36bc31c9186b533689706e638a83ea93ba7195|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f94953ecdf1beb9fb4f89ba1e2a2a7c5eeda0c6b9df824760a68e838ae8ef14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e926b76c8a72ba17b65a4e389e6203875964e799980108b67006456d452afdbb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2097607919910d20de3fdc66ff77e09f40d011f5e5ff2d2a311387ea53575437|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-79ae8e4469fc7a47714d7aa99cb6e7fbc37fde3ef608d465fd48df7cbabdc8a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a21d569270e93c9a59180b98647c9c5d73582bbc9ae4aab1a5a4ac4076d69648|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f4872ecd7c23daf95245c7b2a1155f751c2b0624f65cd80d033700fd61805afb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-75736678a4b2aa21f1647ed97706d6aa7396cdf500ebff9b88cf0d845876fdb9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a4c51f0dcd4d8c5e5352319312d3d0956d99114880059dbfd36a5b4562418624|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-761d81d4d0821f1b325c95f0643196569a38d57968254d364176a57733c8a17f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-535db25618f70b9ba7cb722daaee3170cef4a0e7a0506bbf2d325353b441cda4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e3ef814509e76ba46d29889620efba95f4e802898ce1c8a348adc9b46d58506|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fde88a125e06ae5a815d344c7087783420716051bd268a492a614608b2cfc34d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc5fbf30b65deda3a9d6b57ca992c47b3befd676f57c9ed55482005827129a9f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b03804ef42aca6d6b6e1f6184cb88f6e0ea81a83985a64d773283ab43cc9c1ba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64080b5708ff9c879e10245474af8abb299bbdb8f0cbaba0efd98a6f1c62c81e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b29b3983b6aa5f05abeb3d8a29b0b2ace9a8a9e3160904a27294988fa4028943|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8fc827d127edbc50a4f15dd7fd39898d22ad76325f9d05b43ecfa7911b3f785a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e36ef37e04c7849385bf95efb78d1db56cc79c6ca2e395323aa195d6e78043b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9af64cf87a0d20240c5e61e5fe73704cc66626e72bef55b9b8d6c77230665bf0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-789656c72e75bee871c7516828740ee2bb250ef7111295d105ea5a801472a8b3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d74a995a2933401000d0f3d9bf5d0af1f4a54c997377ee76eec413dbdba55acc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8db4e6114f57cea9de90beb3a51116d9dc6576a83b805131b24daa90a2e2a0aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47d0f7d868da1a927d7cd0586e89d97bf6591941c170a13550109927ba2de773|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-26802ad172276a30b57cee86581f2e7f9b6bb725c421ce72ba0281facf0a1655|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c0ff08546455b9e79475dec3a1ba62f4b0bf7244ae2a9d7e520ce4e333b4ecf8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dd4f8215ba2f17804eab502e80c1bacf64e4d1f2f32bfc1fe6c0bcd0c88fa23|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-969e745254d62a96b7a426af46abc3a6806d82e3bef9abbdfb256a1239b4a3ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cf3f445c50a66565001e16a2a0f73979caacb59b0f15661cc067d7e8128d334e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f58d51ff7ddf5c1ef2702b15e265a01942db34dca651f06af3148d4997c8c8df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e63e865caada4c2027a1ff8d3f63280cce52e969a04657f7ef7cdf50c896e84d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-100320823d94f30bce57b99735c7d2a1e8560799b39a5fd68077f25efa06f4b7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-beb8ee700645f4c512ef6b5ed01eb25935783c94c203dd1f356810dea762ca9b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9facc9bb925863c5b5260d406459be0603be40c67e98205715b899c271dbc5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f2181103616862646de86a9ddfd08f9d033b7abbd7cf023853aa65f09a53014e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ca66b9f82afb74b95f1b6ad66a0affa406cbf53a73d8fc083c56b5153f5ffab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-527aae3eab81427e4249568323c213672818220630220e938a5ae7af3843c4b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8f512cf5461ba917f738ee35d90d4a1f7acee90ac3671b55d677e7b8f6f97f07|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-01d33563b3b06d98bc3fb398300646afbefa39fe8733e5a39dda21e5c32989fa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aedb3fb600b779c26688fdbb17076196ceac984d4b971e583769dbef951e9700|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-065cac03e8c839facc7ac9201b12e7b75c1fefc0352f3762af2c69837a2801ee|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d36ea1080ef91730f75fc484aa9c4461fde8d43bbdb86ae374215e133d451b21|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-68cb63a498ae5731ec8092368371ea37c0167024079abf5415681ef597a8718a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60755068d45abb84933c99beeed54bf9a8309a7001e0a05430c4ca83d4e1725c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d8bb4a87a4365c6ea4482ea841d3908bee91dc1d5a73ef36f5b08b443a96a5e5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e208158d0a73ce5dcf2f5a7596005123b0717344f05125066e7f9fd3bee935e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-14dcbc5429607a9cb14771b68ec82ccff2ff81e0545d8026b198f3d2863f4df2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49e82cb6299cdd73d7fde7eeab2e4e30aa195c08e5e388921adbf4666b3f89a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-48ccd03685ec3c1d2f4b25722d2bc5178e0805368ca1001f430d3fef32c5fd10|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f9e46b6a1340ceae29b2c8ad7149c7460dab92a232f2b355cfff5257e0e0ff7a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3dec7648a9fa67e1d32fd187bdfa24cd610a84cdea36df1653896e6b306d2e8d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f3b503da27ab5b58e762decfb6d9e6395a50b769eba46b76400cecf3c33e2ceb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-22a4f2d792dadcdbd05e19e79a9848c3641862ae711477dab91c6a236db37ac5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c8e15ca58a5258aa0d9eec991a688df2ba848d7c6af82c2d2ccd067eba00c87|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2ddc37f0528c5084eb0741e89e703bc10bd7475312a828f5fe2e6c13338844ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-106baa2ff055d9a06d68f034541c5fc3aa01ce5171151abecae5ab52292da05a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4530858699262716b429111b92ae9306339207ec9c89987f896331d7062dc93b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9d136ad621c4bda3503c4ec8f6ccc3471a9055c8a162a76f3741a4ed19aea85f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a943ba68ce18b5344b64fac88912d04ddf8e44ad5ff4280c513d31cebf1f7b00|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c297d7430e2eb11083bffe3522903c18f79b855ae19fbe4073ad3dfcb6cf212c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-eccbd228bc6eaab92f559e0555a9ef49b5f33e48b05250a74195df3bf26d2bb5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c6130ab4d9fde516373cfd73b9ec6913db85c22d17b4b38ff6fee78e4ab8d3c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-83b99b76b7221e1340b8a477697d8b318222871bcd1c9acea72b542ab26757c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9fd6d88baf8a6c574f54434541f9f4dc4d30168123744b04466e047956e77f1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-962003de60efb71f982f0741143d27931eae1122e7995548a9cfe95c39229a61|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-664030f4b524001f07f26f7f216be9ff00da24a16299adf30bdbc02d6f0fb74a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b9d1e34e3602ed8f003cc1ead70545542e2134995dd88dd6f96a8b10fb02e526|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3abaa8e072937fdf365c8f5aa74ae7db6c0bf6319a167c57f379890d7917dc47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8400f4849a6d9653786c8ad7341585508d8064979e0e2de9172a481a3611a61e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1de086a78e6bf8988fe723815402b0789e3b65738b8ff1d279634bf2c4057707|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a0d4a88292a815b1c78061f1a25a6ba69de439bca9089d57e610f5baf7c49e15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38ea304bc3639055d0a3bb114e9ae500c9ae0a19c86b07b8fedb3d420d52a77d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ca5eae24f89ff12f9a455e7dc7c2c97345e6290f91364324c60988f88ab27d52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f31f9d88de5462278c6a97ab543560ed095dd95a20fbb7f6fcfefafc44c1bb37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-874a3b0eb38c2c2f2b1d254d2c209529396003e24a5ee2ff73b0f10931d3b9c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b0b907e5eb4b9f7740c9088d96372f70bf67102beea2048efd9f905c4ca1f796|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-950a2f4bc3a9889303fff5ed35733ecd3180436ca11e575a2a614a4cae542728|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f3de50920ea78e6b97f93a4a5c5e4675c18bb3def2cff560d000bef0d217f6be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ec306533edd09a422e7924c7f238464da3d231a5fb230847ede3da6fe833789a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80b2e72cae607d495db115f36c592b36e81bfa030667e4012d07d9bdcd5bf198|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-524681f1dd977d628e1fdc70f6bc7678ef6a2a2ef42e2ac9bb78ee2fbc90000f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0bee58d31c0b2dae52e21c402af7d427f6a167b1c38db98e65afcf7f73738d26|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e7aa09c41a4f04943669ee0c60c057af5f755fa96703c7eb79be8fae15df3fcc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd024d6a856ed83fe23b8c6adffab76511d23cd1752f7e41a0d7bb9d4b264f2a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-459d6457482583bf0b82eaf785e33175c3a8ed14b7c0ad4d484804151120dbf6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65016573b6226a3004cd373fcbf2228749668f041889ce06b2f4c0f4a5602798|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-139a9594575dd06f12491d99625f298f76626fd15f95d34352af9fa49629b33a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ac24f65a1a9f863a6e1de229ba4b32948060c7731a1ad402081e4a8a1dfae29d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69715ea2ed28dbe7a5088e10efcd2f3eb64717e7795dfbeda630912e9b320967|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e89713d95fd29b8b744949513f18d2cb9c8bde48c249954fab5a138d69132e37|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a62c823df4431948f862d030b96f20bd39c2618168f6b0e9adf1a6a9adeed4a6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c83bff8ed3dff40a1432de3c03486f4fee71cf63893638ac14fa81d2019c487f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-800f27a208c5589596dbf85c028ed654aab89ce194e8511e86e182ce9f14fa27|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3a6c5c70e8ee65eadd09f1dd1015b672b5bd2a16c882cb1a11ab2964a8d9e544|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa6df2acadf6d1416e29685ce8f612f0add51e4e336be1ce7b8b8e5215b73a9c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-49187ef657f730f6c5c3a7b24d1f87db72a6ad4415a2c879d307e4921249e877|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-53fe60bcbe3b5b59ca9bf8ea0e064a8fb68e45a7709ef5970c5c97448b619306|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-478154b8193ba31f5f93abbe6ca2ef0894ad278f6d43f27a2f84ffe60633d22e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-52ed3db2c119f9194feb058a35698e6e0f1c6a4cbb0e4fab5c1958433635057b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd03921c3aeb091a5cc7dc112d5cbc1c8822627e6c46a6ec47b30c7038ac4007|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-38226f3841f8fdafcebec41f2d5b0b7649de90159df65c0349ac6dea7abf1254|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7326b62be0d1cf745b751097bbaaa678b1881c96ef4dd372fe44b8db4183023e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d7733c733dc92c985dc408f5c71a9c8182835012c4ebb89a71a1fe054b545194|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aea2d8e0185581547a4222fe67f57036e070ec2a4adced775b2954f313d4f5b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e02acce99c7b7d201d75e8afd9cbfe37c1fd7c4da6c1acff31e3d3346d9d1255|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ceb0ceebc39d9015392f5dd2b2f30307f41f7bdf6bf1b970e2c8d29e7647ce4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-659e0f12fc209470f17ba837dca28484058dc401cbae44ed4ee230b75545a5bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9434d3fa451632f7abf7c3fa980bb86591e1a425cb047ba1b2092a35235c04cc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e1aa762376311dac6407eca92b1fc98b760f51ff66b3d326c0877d94ef70fe8e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0740b80aeead46f37e307886ab765e1d774d3b0c75feb4cd6845565ea8e35b32|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-571ca1f60384f4b205ef95fad71b45dd2f83c0979b46c8218b8abf4ac0e275ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4678a3b62c9217b733f73a150f77321d4cb0f28d51f29518e94a6ae293961b74|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-746bbef543b3789d21e037aec7b3173c94595e958afef61d54f5863388ff056d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ce737cf384a8057678d0bb24eeb3b491c82ac736f8db0604ef84af7091d7a71|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1317d0ea6354dfb5465c554639218e0cb94b2bf36d29e3790ebaee7f58711469|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3aa99309200057fdc7aa5d7a70dc3b1f34b32aa12213251bb2f50efd0c8eaf4c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57115670e27ad846c29f384b47cf2ab5666824724f8e4c5178dc537ad7cad7f9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a333c245b14dfa19ebe6b6f498f0d3219d86b85a759071779168cae109b9eba6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f233ad7f84e7f9d735a89c2795274fd19620c432db3063842f9acc6c85abb4e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cdb140f0380b639d5ea4e6a54fec4b7885487dda6e0629a1d18be5f0505c69b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e35554ddf95211f58c8c03696666fb2178c95fa72fc5dd64a8cf63ec5fbe730d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff1e4a557b6f2d88f0866dfd3604f301ccf11b02fb288ab0f81b44468bbc1fba|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f21d1253ad232e34e5a8760922edf38f13a6ac9670b33d707eaf10b8641ac9e9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85596c2244be106ed01b8e5a402d0d346ad57619f74b8c27075a401963349991|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-909a746aceed0f2368d85f407a513b38ebe2803ddfc140ff3c10fd1f6e105941|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-58276320d774cef1e1876fe434174946997ec4ab8355c2472465e2d4152eb523|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4460021cf2be3333bb5794561d465a413364b74270a99b7800047fc2e13cc8a4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e9c1c3a0697fda27e0fba11898c790fa8eaf4b21b026cc7ae1469842b3b6289|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-24364ad7ec4ba9e11052d798b20f72826eb805875ce7f54950f7f2ad758c4907|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bee6c298f99cbafe30976fe300483d3b00a8959fddb98c7442b28d84414081b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8d2d93c960f076c0eafeba71154eca0a6275d6d948adb373d10c3e98c7dad09b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6b7525ca24dac55dce19cc4f39048d91a22dab62f9aa86b354802a61ec76be7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59252a1bb7c2bbc3b1c00ad1b2bb401c270d53fed95080cd2d45c6d81ce1ab06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-111dd3ee28e661c7b3af5f882043464dfc012229e8de12da24448720fb6f9cfa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7f3138ca0b1f7fd9a681671d7c96b3feb353b06a2732c8f3d37f8eb2c9f56b9b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d7d01e794f7c9809c3c2f5bcd8a4aa374b5eb7fd69fa6cfd103c4e5d7824734d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-017a0dff1ec226c756e87e36faef4f21637d3eab4c53c695fece4a065a8e5f9a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-201c8f3e8f72b955dbe18dc53210e0665344baaf64a55a844aee7d3ffbf54abe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0abd0e39413a467e141878bb47a788ccbc296c4b9ad2075619120b6d7c37b689|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e91387884c9692e4f0e930bdfe04e8209cb6f738545ab9ad24e9de7fa915b1a3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e9b3661e5330753ffa61ffb14b97e17fb9ab61bcb30d9e1a60e0b09bd8d2b5cf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bea0b575c4f0963fde64e7f54b7b0a6eb81c017d00ae365e06af36aff63c9860|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60c3897a33e12286d36250925c8ad7d0872747b1cfe97c1aa8532836af7ea803|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12d8c71ad6a70fd5c56fa7b31e5bda809e6891b9ae128f3a4753c3a25d286d47|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7d60beebb580eaa8f1d2fa74a80b37441c6f822744356471ce837f697409a465|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65a78b403740ee54cee9563f9ed777bcc8cdfda898f56557c559a998281dc5da|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e34e2c88e73f2a3153db99dfec5a216cd4afb3c06b73d5498b0176a3672374f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-17859e751461bf9421aee0e20847545fc5aa7f2245d7295f3d0f89931dcee060|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5b046fc221ddd33ada086a918e7075d9f92b4008ad3b2e6d63ccad4915ad8764|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f8621664e0d0a45f1f1980f9ae2ccd3f9ceff6f975b541c8b673e562a4a806f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7ad0a0d9b4efae23142be3a04340fad807d901417551bd586d043e9a1a32a5c9|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9260c5aa948823e4657cdd05d81bd215d738c8933de02e8769d3d808c632b1e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dd81f184d573f22f21bc90d7527b5b6ce2fe72b1ca767e3d41678e3bc8a1a9ec|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fc491e78376611470ab2e8ffa5e9471530cdd19b5cd90ad8f030baef0e82e3d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-33e52547d779d31d5229e27cc28516b6b6473f90fd9e3f4c7eb0e58e1fd5f712|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-59d153809c543bc48140340c59fd5655a2a58c4a63728dd8b67fb711fd5a2700|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af398a468b7e00bd840603921f24634031441a3fdcd96db5190f8ca8635d1cf3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d3ec8bbdc26e7b9ec06c6d654184581325ddd7b52433eb386d65831a45af3a68|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-96b00de1b92acb6bbf3276c9e56b4d0fa8d4b05779a859e425ec58e11bb2c124|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ddc6458287dd2ad768feca45fff1a33507d0222f845e8c6ee0d89b898c4b18d4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e27f87333e40512fe54eb9301738e7bc23d982dbb832ffd08b508e486ec3bdd3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-712193ef84541de2a7b3bd850a88c4d1873e8fd304e496161f9691b5fcb36071|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8a65f3c3f18ce8170bde2e863c69447c72f8beda5ea456905dee43791f7272fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-00bb9e9eb900ad32ab8581a61812a2588f7368a0947045455021d95e98623659|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7cbecd3da5c99559743e62e5edb306a4bd9b76357f20ddbb3132bf7525d5d199|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa1b6013dce5e8b30485d174d0b086f843caa15c144f02caa3f424e54939c877|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-88e3f43dda52b97bf312051100edfcaeff8c4fae677743d9ddece4451e62bb73|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-127ccf47c153be13e33fa802b01d4f1130bf61c66bd9e8ebc8b14a813f25188f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7e54e59d0a8c0a25fa90fcbc9545a81156c622b89254910f4fe5a5999148b2ad|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c55f6ffef7e8d110a0edd5b3fce8842378789b4351da33cb68446c6698b1f992|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d2669660537675f9d4ef46c2183bbd654dcc44c7e7cb80fb5c1907a803f6a0e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd9d3c6e7e5ebe7e48ee8f06ed1430f657ddd933e0716c309021a72b87260285|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d0ead92cd775cf36cf583f5a8a456f70f4bb3f8a59a35f840b58e4c88b34ae8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-69f97e14dcb60fff90e2fb65e01313382239fac79ec9c7775756c3ca0e76bf29|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2e94637b921644a43c4bc0aee4d277cd771fb30a210e6ac9bb6d2867de314354|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4eb1ec4e5d60d92fb8ed11189c6d66a0f3ac53a47ae21bb1043caab677a0695f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d47d12dd0f24979f714ea2435e1b8cf39b09570c46fb3d556b9eccde4ea58881|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d6ef8fa1646c9504fb485bb9038d3e0c73b64a6403dc48618169d5a8ba849889|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ad4fd4af5c1c68b0452dcaa5401d888163baaa91a110d95818d7945e96a9f7a7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5a74b7674fd165298ff8f103b9b9c81450aa9035c9a951804fd4998f281236b5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a08361a9cf6db2c8b866f8f0405d162ad8d5d766b9b1cf639d524118677dedd6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-290d43ea5159b64c63e1924c45d16d421b251b3df9c6dc2c4f184aa44ea21363|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c6eb282c6d24c1a6f270890fd6cb442a9060267bed7fc8580a81dab2e46356de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1c145699c75eb2d84e311f089b45f2ba75f476d4b598313c69b55df8bc0b128a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ab7e861ccef7973ab9f2b026a7a898df5d5f1dda80151471ea430e2b03eba677|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c42b984c86cc762235bf86294e2d5753e90d67ce7224c647f979b1cde739c8ac|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ed4235895c442b708c7dab9179ce8091bbf142cf479eb0cc540b7d2dc4a842bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-458ad0d1555fcf8d8ea9df99ba7152122c225cae2fe251db637e1adc939c7df6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d9b701adbfaa696ac4a4267680205160917f5d807b426dd3054eec097e3e7b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-70ed388428244ac2fe28052999bd9a3f581489f02faae0cf85ddfba759451275|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7b475879250e91dcef12587892cd5d35afaa9cf49c9fd4820622be95957c0ce8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d0fcd60f9d9f078d82ebb42cfb0162421ff7a8d1553ac767f08a762c531ae44|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3011a47cf9fb587ccf2791ce9ad28d907f11fc5d613bee140f9791e66a3b68e3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-50ecb244382f91d3f51dbc58f6139790a7582d5544ff714f69fe9e661b505299|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9a5c3ac36158ad9cb4da059db8211cc472b8968b8248a71bc5a8a96109167935|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bb437c5ab37a46d149f8c0c8ac408d9e82db19fb139dacb071ec6ff4f20a56c4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2d0485fd8c0beb862a73c823eebc03d40794b52b15d45756d98a3aecc9a97567|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-434b51467a9240acbc5aea2610db7d2799027d8df91370ba07eabfa338b8cf16|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f905747bc0c9122bc79eb19401536c5b3cd0ee0cae810a99ef0982312ada64c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e11ba2fcd3912c334a1991225bb9a7648bff100f44c1a7b30907ddfe624f1dd8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a36ff8e93ffd58ff002fa1ec3781b9f216208987507948c71e8944a1c466500c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-489979fac223a025f3206cb83f9ece3f56f0d8dc4ef6efbe90826af163964f4c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d49685e528e60120b66118e147964fb1db12d82a8c558e597ad0149cfc6f02b7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-682d0371bc3c0ed88a993cccad1b0692550b5f83374282da8795e5e5bc02762c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0903c8b99c557e7b039015b11e1b23b52234bd7622eab03ed29c2012437d0c41|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3e2bcf0040f3590799b04bbbed4fe285c610776fd771ec5bdacebf81298cbf19|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-91333e82487708296e1765bf9d075f0c63026fb5f19ae74174b14a321134a5a0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5070e7b33e470d08a4b510ac0d7b500973c87d3bdc8c6c3b84eb0ad99d4ec1d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-65ad300168cbc2a4104a89b253f7d2bd653eaadec903a5ed074620a2594d55fe|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-43bb80373244ddda8daaa47c6fc3eeb59a55c8d05ca63c7a57616f1c90a4c669|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0f73b21f6ac39bffbcfc06d9c0733fdffd6a09176b5429e61fc83e84186a7cf5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc483affe72a157217a38411b14791aa76c8159cfdc35f3770b45596a68d42b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e4d781ce9e212972d0041e4ce9d57012c77c4d3da4feb73f15e9b9c22cc733e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b8d7397e7fb6c57fd598416182ee8886da6796761d8d6e417160553c989ce715|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9f5b03d82a400186654cdf6851b1d3336462dad4e09c44d9197f62b04621f57d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-97ce91697c27093489f9a85e7e251f87a9ab24f5996ed118294fbefed881ad6a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e5becf40bfbfe51fbe6cf1f5d14b0df653cc2f1e62fc82dc5dcefd119352c11f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-19f40e64231ef44ad449b695a40b3c59d7b98b79e6dda5c524a5f38ddd3748d8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c738252033a51fadaa766bbc682a01da5e9d7917b8d661274f2c5e34b4a8d99|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-77fecbe4bf7efcc63f58b4379a4485514b8de7d34e9837c3c7fbdf992b2129de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f5620f8eaf3d39ebd9d5fb49c36a49592169e89fb1e06dbf46a9b3a4cea1b244|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-538cd7e85344b4be38c4cd8fafc1c462c174cb38adc4633bef710c87fd887d28|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b69db9a00d790aa3269e2d2dabb0b044f86d1bffe7c0a65590eadb66b53697b4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b79126d31a6104110e7f0943064f1159980cc10fd6da389adc80a66af5499d3a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ff4be66f434074f5b0076ab761951e20a5b72a6f430239e14fdbba4869f74ba7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9c2b16d4b8c9a7437ebea4047216f812fc3b58eaed722c74ef01772cce609bf1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9f89c686187aa1c568fb2894255fe8a6452d5ff2059ee4b870019f383bd6f52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e41968d7f133fa82882ac5280325c930f3feae43d5eb7539a694be477d45d9b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d2f1bac4ea2535112287ea164127edf0da63488d12865613e5fcf04b5478ff05|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-16b787e2eb19a51792f6c727f89e1b620fbc788693137c72047f7efbb63e848b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-18be876a19304ae7676f5c414ded8a142d798a20a53e4d586e5307daa51bea60|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bbf5099982d061d66f8138c912a31c1164c2acc1eb025fb639b5f2aec40e6e6c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d97677b60bf8908d4281e8b3b43e1ddb765db5325e9c07cf002b8e349c28cd36|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-934ae6ed9f5cfb4b18a87a0db4ce34aa28c52e616dacc10f8bda31ccb82233b6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8064aa10e2dfe704bfa8cec2f6cc4b2053e664f6fdf2912881890e3a53dedb5c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a1d5a4ba7e87924106a0678e382452ee50e8c93f62b35a3fc9cb5560e9b4e47f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3324b78c69c60c3390daec70ac5953f58a86c2168fb20a4f30645436e800ca67|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-80f2cd5ac18e30a4a25886249c19ed891d9fc91bf12503e3f4a4a63261e79561|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-02415fc928d23913955ca7dc707b378974f29f371652d576662f86f90fbac32c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-883f196e6f6973626089cb4b0a3e84481aa0836c3858a96516d3cc5d22f2cf22|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4e7c6f6f69b3512fec033838a3166f0f76f2661863a8e124c4f3cd563055ae3c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9bbd2c4345837f78c236a014f359625d24fac02d89b624c7b2fada7c46e2b335|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1f218cf2958891156e48d2b3dd18c1d1e1e68587671df7970588e3ec1305769|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6d92f299d1610ffd73503498396e8e4f6e323e012c8d067b42baf9f456721812|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d4416ba8f567a3dd39c4778ac55801a4da7dc20e44bee56754cf6dc55b58af5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e1368fe3242e1de77a9ecb28fd58b318f2dc6671c26ddcc414db112454747dc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a2e2ff00955fdc8f13bf8dfe114229e9562e55ac6a199f482cd654f5e8bedd5e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1bd9924ec1959f770183f2c92e0dae4052bf86d9094fadf79504c9ab075299f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a028ae36ba046bb04e6de588abf130860376c66024a1beeccb74ffde27ed03e7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-64031bd6e5e3aa343fc61b6b72f2058684fd544f707ce50bad2cf51f29dcdde1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-268bdb5fcbe4448ae930b3f44ed943579c815e760ecec9033c31953e97a902aa|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4ac2cce91806b99f94fcbaafcd64afe0af2e8ec3c600e09a8a49942a4808cb5f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-062c94bcbeacd7338ff7515268e3d49cb7fba16e42e51f133265be9d00b09076|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b71d0509ae7b011cbe506a2dbc92e4f1cd58f8befd6af375e5c1b3c2a359a1af|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a18e331e3a342c534e78ecc956125baf9cf63bea719c0926b09e34efde28653d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f4a15319730d14352cd4b134e1ba8cdd80ae3381155d93175ca32a36d9ed893|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-536a10efc8cbfb249d776133733fa5cec602fa13e36f5de289f5fe3236d45124|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3e7a050e58e0ec720004b029042e2238ceedc074046906b6424b84be5e261b19|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7c64d97b1df0380bc4a34c024cea95228c83f8e185094e918841a6b723cc14db|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3c765b11dfe90ec9c99a5e51d8520496470857eb2b1f5de99f33c8e85b2a204c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8bb81c5927b8bbb476de97c47d70fe7e849115e8cb539691ce8b4e1fcb2f333|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-be9575ea16e5358e11c9008cb29082370b781a8c1b55dd1006fddabf33370fbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1f8a9c9994d17d99b60e589c006f2ca107539caab12723e8f8433001e067d52|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-622b8e28c951a98a93306769397a9a3491b157256587a1db58b1fa29fc2882d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4dfe7ca354544d470e4fa9e58b3863af5eabe80ccf572be608753cc09b26bdcc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c4e01edf8db097cde8d0f8d4dcbab6b4207f73bd9a119ca85ba03dfb6e18278f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cd51082f8a96b51c9baebfc52fc1b0b88922b5dc6c85159ec705dc1626f9eb59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e4050909baaff601a1d826c2a6bdaf63d8aa524040b0fca97c5347cc396da484|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5cc535da8afef47e52499c35c409bff8c7f91186935bcf56f145c2ab6f93f8ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-295fdf971c2c0149ebcf5666ff6e8279b0dd85c900a815f2c4bf22f3896918c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9755a48aba783f60f67d32f8124625e4ab2e6d5d035e21daed67c480827d8dd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3536002b11ded5baa38f6b60ec68a92e8cfc7801b0223a22b6d88622e5cb679c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-25e04e4cdf4c3de80dcfe77fbbe6c1e33ded0a7379fb0d7758cfda1343cec40f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2082fe8253b8880f2e5cc4f87b52e07d6cc1ef5548aff6116ff9076ebc22188e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4f280559a2beb5ca012560bfedebfc89da5e2643bc464475d79683bc6aa5436a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e32627215a300426b7231ad4e406c7510e5a4b08b20a82d0c0824ea00b508991|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5f74aa4fa7936da24164dcd0d31e0ffe5dedb70c29afc0308447e736316705af|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c2ccc2e666fa32c68ab4135df108a2fadf6900d467d8e84a1956ac563ce9456e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a389ece08d630adfe180a60f6e7b596dc48cce00f01003e659478308f99d6dea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-dc42c86c5d9d17e2c867ae9666a2963178626c230c55c0e6c424a57569b87228|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-cfa35a0a21b47a44c453c797680d0ceeca71c42e4545ddd629d6ae8edaa221c0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8319bd8915ff5fbfdb12067204c8cb9963cf71b2ca4140fd2ab0a6b35fd36206|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-98058d565d3cb76f2420cddd8bbe904d9fd33b85f8ecb804dff3d2d7d14d4a3e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e4474ca8fa3a46e2510469f42159489f27cbf7a367c008ca101e3c3e54504b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c75d06aaaeda3773ed1bd0e769de8f411cf79980c58da693a788943f3e19b558|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1eb413d74211d00526f5857dbbdb340de05117c4d330656d541dde8e87a36b1a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bf22b60a9100a70a43ce2fe54fb0691d1913aaa935888c8530d78c0793b46b15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5e50730e74325c338ced6d16d2459d4aabcc5f9677340dc7f5109a849d4f877e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c1b9f6bb891250c70ab05799b0738856120ae7f245ba50892e456cef9a6a6fa0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-956498ad6cc4a071f2693c570a03ca751ed4046d091a9c148607dbb05177c73f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-284d78400b2a8eeb1f494c13f8fb76188e91126fa0e9e704580ad43c5dbae687|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f672e17724344986949b77e23e61632ae876e8f98a6dc77554a32400ef42ace8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b571b9227a4c7b5fb902a030a63e49cc3ab090d52da4bef4a0d835e30a46535d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d987f3043c0fd88268f665751f02336b913331618417214e68c3bf9107040a7e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-57b3090870c6aa9b0f380fc862ac4c74cf67000760b634824e18acf26f436c9a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e74133ff8036db67ac50c2cb038a7e73e133dd558200a8d6004504fb876aa439|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-5eee31f2aa4944ba9830e3e5b6629f801a0bf1495d32c2c7f33381bac4e71a8b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-7a1b09e6adfe9ed6fd1468f5a2200468affe0fb6ad6ed1fbee926e50eb5790f4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd22d7e0e6ec0b6301aaa3ee31f0d9f17e5f3ddde2a0510767f607a12ec98c02|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-524237f45eaf87fba33fa0300bdcdd456bff9a4ae2f9f997847d994198a0f36d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-f18979c8a895be8847a2d26135107e91796631fb3b495089a1470affc8900315|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ea643b944f0778c3f3f1420108793952e24d43a6e705323c49332aafaa7f34f6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-99a474b198f896f18c87af64bdb6ecf75dae2c0c3858d5117375dacd891e8636|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e2e8360ad011e098991bb32c78deb6992cbb79ffb50170690aa0b731bc60131b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4efbb35159a3c43f066832157807667298b1cc1020e57deb6cdd87f53ef9fcc0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c9639ae265d79b8a68b7bc80454f5aac5e6281203096c59c29b093b50a9bea75|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-67dd71913e87cbae8227e5c2c2252113338724d2ccc5330ed7383173e6d7f9b1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e8d81a2b202821133573c2d87592a4c6ab3428e6c210cd02454373256b076e14|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
