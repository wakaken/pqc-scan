# PQC Migration Scan Report

Generated: 2026-11-24 18:35:51 +09:00 (UTC+09:00)

Findings to review (items): 36 / Informational findings (items): 0 / Files inspected: 882

|Severity|Count (all findings)|
|---|---:|
|high|12|
|medium|24|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[OPENSSL_RSA_API](#rule-0) OpenSSL RSA API call detected.|high|quantum-vulnerable|12|12|
|[API_EVP_PKEY_RSA](#rule-1) EVP_PKEY configured for RSA detected.|medium|quantum-vulnerable|16|16|
|[API_OPENSSL_RSA_NEW](#rule-2) OpenSSL low-level RSA API usage detected.|medium|quantum-vulnerable|8|8|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## OPENSSL_RSA_API

OpenSSL RSA API call detected.

Check: Replace with EVP_PKEY-based implementation suitable for PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[c-openssl-rsa_generate_key_ex-negative-comment.c:1:4](#finding-0)|RSA_generate_key|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-0)|
|<a id="finding-1"></a>[c-openssl-rsa_generate_key_ex-positive.c:1:21](#finding-1)|RSA_generate_key|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-1)|
|<a id="finding-2"></a>[c-openssl-rsa_private_decrypt-negative-comment.c:1:4](#finding-2)|RSA_private_decrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-2)|
|<a id="finding-3"></a>[c-openssl-rsa_private_decrypt-positive.c:1:21](#finding-3)|RSA_private_decrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-3)|
|<a id="finding-4"></a>[c-openssl-rsa_public_encrypt-negative-comment.c:1:4](#finding-4)|RSA_public_encrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-4)|
|<a id="finding-5"></a>[c-openssl-rsa_public_encrypt-positive.c:1:21](#finding-5)|RSA_public_encrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-5)|
|<a id="finding-6"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4](#finding-6)|RSA_generate_key|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-6)|
|<a id="finding-7"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21](#finding-7)|RSA_generate_key|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-7)|
|<a id="finding-8"></a>[cpp-shared-c-openssl-rsa_private_decrypt-negative-comment.cpp:1:4](#finding-8)|RSA_private_decrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-8)|
|<a id="finding-9"></a>[cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21](#finding-9)|RSA_private_decrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-9)|
|<a id="finding-10"></a>[cpp-shared-c-openssl-rsa_public_encrypt-negative-comment.cpp:1:4](#finding-10)|RSA_public_encrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-10)|
|<a id="finding-11"></a>[cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21](#finding-11)|RSA_public_encrypt|Replace with EVP_PKEY-based implementation suitable for PQC migration. / [Source](#source-11)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Review the TLS reference before changing effective settings](#action-1)

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-0"></a>

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4

    > 1 // RSA_generate_key_ex(arg);

### c-openssl-rsa_generate_key_ex-positive.c:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-1"></a>

### c-openssl-rsa_generate_key_ex-positive.c:1:21

    > 1 void sample(void) { RSA_generate_key_ex(arg); }

### c-openssl-rsa_private_decrypt-negative-comment.c:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-2"></a>

### c-openssl-rsa_private_decrypt-negative-comment.c:1:4

    > 1 // RSA_private_decrypt(arg);

### c-openssl-rsa_private_decrypt-positive.c:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-3"></a>

### c-openssl-rsa_private_decrypt-positive.c:1:21

    > 1 void sample(void) { RSA_private_decrypt(arg); }

### c-openssl-rsa_public_encrypt-negative-comment.c:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-4"></a>

### c-openssl-rsa_public_encrypt-negative-comment.c:1:4

    > 1 // RSA_public_encrypt(arg);

### c-openssl-rsa_public_encrypt-positive.c:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-5"></a>

### c-openssl-rsa_public_encrypt-positive.c:1:21

    > 1 void sample(void) { RSA_public_encrypt(arg); }

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-6"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4

    > 1 // RSA_generate_key_ex(arg);

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-7"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21

    > 1 void sample(void) { RSA_generate_key_ex(arg); }

### cpp-shared-c-openssl-rsa_private_decrypt-negative-comment.cpp:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-8"></a>

### cpp-shared-c-openssl-rsa_private_decrypt-negative-comment.cpp:1:4

    > 1 // RSA_private_decrypt(arg);

### cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-9"></a>

### cpp-shared-c-openssl-rsa_private_decrypt-positive.cpp:1:21

    > 1 void sample(void) { RSA_private_decrypt(arg); }

### cpp-shared-c-openssl-rsa_public_encrypt-negative-comment.cpp:1:4 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-10"></a>

### cpp-shared-c-openssl-rsa_public_encrypt-negative-comment.cpp:1:4

    > 1 // RSA_public_encrypt(arg);

### cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21 — Matched rules

- `OPENSSL_RSA_API` (high): OpenSSL RSA API call detected. — Replace with EVP_PKEY-based implementation suitable for PQC migration.

<a id="source-11"></a>

### cpp-shared-c-openssl-rsa_public_encrypt-positive.cpp:1:21

    > 1 void sample(void) { RSA_public_encrypt(arg); }

<a id="rule-1"></a>

## API_EVP_PKEY_RSA

EVP_PKEY configured for RSA detected.

Check: Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-12"></a>[c-evp-id-rsa-negative-comment.c:1:62](#finding-12)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-12)|
|<a id="finding-13"></a>[c-evp-id-rsa.c:1:59](#finding-13)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-13)|
|<a id="finding-14"></a>[c-known-evp_digestsigninit-negative-comment.c:1:64](#finding-14)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-14)|
|<a id="finding-15"></a>[c-known-evp_digestsigninit-reassigned.c:1:61](#finding-15)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-15)|
|<a id="finding-16"></a>[c-known-evp_digestsigninit.c:1:61](#finding-16)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-16)|
|<a id="finding-17"></a>[c-known-evp_digestverifyinit-negative-comment.c:1:64](#finding-17)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-17)|
|<a id="finding-18"></a>[c-known-evp_digestverifyinit-reassigned.c:1:61](#finding-18)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-18)|
|<a id="finding-19"></a>[c-known-evp_digestverifyinit.c:1:61](#finding-19)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-19)|
|<a id="finding-20"></a>[cpp-known-evp_digestsigninit-negative-comment.cpp:1:64](#finding-20)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-20)|
|<a id="finding-21"></a>[cpp-known-evp_digestsigninit-reassigned.cpp:1:61](#finding-21)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-21)|
|<a id="finding-22"></a>[cpp-known-evp_digestsigninit.cpp:1:61](#finding-22)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-22)|
|<a id="finding-23"></a>[cpp-known-evp_digestverifyinit-negative-comment.cpp:1:64](#finding-23)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-23)|
|<a id="finding-24"></a>[cpp-known-evp_digestverifyinit-reassigned.cpp:1:61](#finding-24)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-24)|
|<a id="finding-25"></a>[cpp-known-evp_digestverifyinit.cpp:1:61](#finding-25)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-25)|
|<a id="finding-26"></a>[cpp-shared-c-evp-id-rsa-negative-comment.cpp:1:62](#finding-26)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-26)|
|<a id="finding-27"></a>[cpp-shared-c-evp-id-rsa.cpp:1:59](#finding-27)|EVP_PKEY_RSA|Use EVP_PKEY abstraction to prepare key-type replacement for PQC. / [Source](#source-27)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-2)

### c-evp-id-rsa-negative-comment.c:1:62 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-12"></a>

### c-evp-id-rsa-negative-comment.c:1:62

    > 1 // void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); }
      2 // 

### c-evp-id-rsa.c:1:59 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-13"></a>

### c-evp-id-rsa.c:1:59

    > 1 void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); }

### c-known-evp_digestsigninit-negative-comment.c:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-14"></a>

### c-known-evp_digestsigninit-negative-comment.c:1:64

    > 1 // void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestSignInit(mdctx,NULL,NULL,NULL,key); }

### c-known-evp_digestsigninit-reassigned.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-15"></a>

### c-known-evp_digestsigninit-reassigned.c:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); key=other; EVP_DigestSignInit(mdctx,NULL,NULL,NULL,key); }

### c-known-evp_digestsigninit.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-16"></a>

### c-known-evp_digestsigninit.c:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestSignInit(mdctx,NULL,NULL,NULL,key); }

### c-known-evp_digestverifyinit-negative-comment.c:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-17"></a>

### c-known-evp_digestverifyinit-negative-comment.c:1:64

    > 1 // void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestVerifyInit(mdctx,NULL,NULL,NULL,key); }

### c-known-evp_digestverifyinit-reassigned.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-18"></a>

### c-known-evp_digestverifyinit-reassigned.c:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); key=other; EVP_DigestVerifyInit(mdctx,NULL,NULL,NULL,key); }

### c-known-evp_digestverifyinit.c:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-19"></a>

### c-known-evp_digestverifyinit.c:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestVerifyInit(mdctx,NULL,NULL,NULL,key); }

### cpp-known-evp_digestsigninit-negative-comment.cpp:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-20"></a>

### cpp-known-evp_digestsigninit-negative-comment.cpp:1:64

    > 1 // void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestSignInit(mdctx,NULL,NULL,NULL,key); }

### cpp-known-evp_digestsigninit-reassigned.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-21"></a>

### cpp-known-evp_digestsigninit-reassigned.cpp:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); key=other; EVP_DigestSignInit(mdctx,NULL,NULL,NULL,key); }

### cpp-known-evp_digestsigninit.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-22"></a>

### cpp-known-evp_digestsigninit.cpp:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestSignInit(mdctx,NULL,NULL,NULL,key); }

### cpp-known-evp_digestverifyinit-negative-comment.cpp:1:64 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-23"></a>

### cpp-known-evp_digestverifyinit-negative-comment.cpp:1:64

    > 1 // void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestVerifyInit(mdctx,NULL,NULL,NULL,key); }

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-24"></a>

### cpp-known-evp_digestverifyinit-reassigned.cpp:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); key=other; EVP_DigestVerifyInit(mdctx,NULL,NULL,NULL,key); }

### cpp-known-evp_digestverifyinit.cpp:1:61 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-25"></a>

### cpp-known-evp_digestverifyinit.cpp:1:61

    > 1 void sample(void) { EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); EVP_PKEY_keygen_init(ctx); EVP_PKEY *key = NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestVerifyInit(mdctx,NULL,NULL,NULL,key); }

### cpp-shared-c-evp-id-rsa-negative-comment.cpp:1:62 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-26"></a>

### cpp-shared-c-evp-id-rsa-negative-comment.cpp:1:62

    > 1 // void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); }
      2 // 

### cpp-shared-c-evp-id-rsa.cpp:1:59 — Matched rules

- `API_EVP_PKEY_RSA` (medium): EVP_PKEY configured for RSA detected. — Use EVP_PKEY abstraction to prepare key-type replacement for PQC.

<a id="source-27"></a>

### cpp-shared-c-evp-id-rsa.cpp:1:59

    > 1 void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA,NULL); }

<a id="rule-2"></a>

## API_OPENSSL_RSA_NEW

OpenSSL low-level RSA API usage detected.

Check: Prefer high-level OpenSSL EVP APIs to ease PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-28"></a>[c-openssl-rsa_generate_key_ex-negative-comment.c:1:4](#finding-28)|RSA_generate_key_ex|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-28)|
|<a id="finding-29"></a>[c-openssl-rsa_generate_key_ex-positive.c:1:21](#finding-29)|RSA_generate_key_ex|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-29)|
|<a id="finding-30"></a>[c-openssl-rsa_new-negative-comment.c:1:4](#finding-30)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-30)|
|<a id="finding-31"></a>[c-openssl-rsa_new-positive.c:1:21](#finding-31)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-31)|
|<a id="finding-32"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4](#finding-32)|RSA_generate_key_ex|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-32)|
|<a id="finding-33"></a>[cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21](#finding-33)|RSA_generate_key_ex|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-33)|
|<a id="finding-34"></a>[cpp-shared-c-openssl-rsa_new-negative-comment.cpp:1:4](#finding-34)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-34)|
|<a id="finding-35"></a>[cpp-shared-c-openssl-rsa_new-positive.cpp:1:21](#finding-35)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-35)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-2)

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-28"></a>

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4

    > 1 // RSA_generate_key_ex(arg);

### c-openssl-rsa_generate_key_ex-positive.c:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-29"></a>

### c-openssl-rsa_generate_key_ex-positive.c:1:21

    > 1 void sample(void) { RSA_generate_key_ex(arg); }

### c-openssl-rsa_new-negative-comment.c:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-30"></a>

### c-openssl-rsa_new-negative-comment.c:1:4

    > 1 // RSA_new(arg);

### c-openssl-rsa_new-positive.c:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-31"></a>

### c-openssl-rsa_new-positive.c:1:21

    > 1 void sample(void) { RSA_new(arg); }

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-32"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-negative-comment.cpp:1:4

    > 1 // RSA_generate_key_ex(arg);

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-33"></a>

### cpp-shared-c-openssl-rsa_generate_key_ex-positive.cpp:1:21

    > 1 void sample(void) { RSA_generate_key_ex(arg); }

### cpp-shared-c-openssl-rsa_new-negative-comment.cpp:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-34"></a>

### cpp-shared-c-openssl-rsa_new-negative-comment.cpp:1:4

    > 1 // RSA_new(arg);

### cpp-shared-c-openssl-rsa_new-positive.cpp:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-35"></a>

### cpp-shared-c-openssl-rsa_new-positive.cpp:1:21

    > 1 void sample(void) { RSA_new(arg); }

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


#### Review the TLS reference before changing effective settings (planned)

A source name or token does not prove enabled configuration or a negotiated connection. ECDHE_RSA/DHE_RSA use RSA authentication, not static RSA key exchange.

1. Distinguish constants, comments, tests and active settings.
2. Inspect key agreement, authentication and protocol version separately; confirm existing hybrid support and peer compatibility before migration.

References:

- https://www.rfc-editor.org/rfc/rfc8422.html#section-2.2
<a id="action-2"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography

## Inventory exports

- [CBOM](cbom.json): 36
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|882|
|Read successfully|882|
|Read failed|0|
|Analyzed|882|
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
|observation-v1-b689feee80ff09fb4aa05554c1a808f4260da9585bbccf3920aaf25ba067d212|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-120cb66bb1a64a1076f095401fc131e9ecc33de1760315c00e588862cdea57e8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f5c90cc663f0ed2fa0d6967f0dcabebf329cc899dc24650d7c5a3e6387d82109|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9c0f439198da64b7d4fe8624056b6643fcaa4c3469d334860c80c59156a08685|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-63e78ef3bd64e971535cf0de6ffd13b57c7a9ab526a06417104b7182fad84f13|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-50ce8bd592e01f8dafe8b361876914983d916053c8375b5a5cd490321376b2a3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e5a61b3f7d569e0e043278a2e838db0f42ac86466ae1c8cca053e80593a9070a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-aed2dc105c6143c2de41e279915d0e73ab4525a61d7daee774154aa82ac3a834|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-dfacb72ad2d3b3adccbf31dee27268858524f8164898df779e6980a66d9b1be6|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8bf671674ef6e69c7dca623a54f69362f421be95dd3a4284b4d0b4961a969529|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6196f468dbe949eb86bc1c1dc630153b3a898c5db28731d8477fa8f9f90648fa|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c890961749501e9540a8f7a35c713c1f1a5b12a5f859d4479b5a790c1ec140ad|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-76c519b0ab38389fc2617bb2da91ea043ee1af842b545264c5caea47f2837ed5|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1acc335d170ed5f03834bacef8f637dcb60b89b04d5d110feed755fd118a7d31|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-aa95ef7e0fe30f71036a94a15c6d7ce8febc6414c618e7443c4deee442750009|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-90cbe2e307bb6842e0b52c530158666a06d479ebb274ac6b18161c4ce40a3a24|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7f1567bf555868cc3e531df417fcd1b96f9e3d4f973839e08cdd09422cc638b1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-93b2a853fe39c7882c44a5edc68f7b65822dde020cfac893842a56855df2da1b|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-685ab24c03a6a5122b3a6669f753ec95abea06d395954343668c8cd8688d3752|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-48581ee03b25d61f5d241c9a7592d7e2948c8b2dc58c36aa817f12845eb515ae|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-589f4fccf66263180f09b6177ab6589fda6cd528a2e474c822de47c6014255ba|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-703d633dd6d07ed094b37b2afae2caf1c9b398a7811684d513541bebe0ece41e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-340f0ec58a373c4a5c13e1c96c1c120c42bf15567ad8bb4ed0250935f15935b0|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5446caedd6eeab7fd4f9d663d0566006bf3f72e744efbb11cd17d7368e251b71|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2d4d8b7cc8df439487132b4e96c10dbb427a4bce8c3541e9b4246d1884455232|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0a61b05a102af6b688f5efdf6d3669b19fcccb74617cff6c4dcfc183f067f0c9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-57a3e6e5fb68867ce47e35b652f1a93b3386169bb24884b1d0b97cf6c3fa837c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-48aa47d366769d2c9d9d4d65afa291baade1534fdc430a4e42bad0ca6f2ae205|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e1b64deadf15cccb61eed1f6b879640cbf1524657762ff4a19c5e3a2887174a1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-48d3d1299b529323dbcf418e939930356dd5f69a3013c8a44784bcea118b4d38|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-4dce8d365233a59c6e0a2277d52aa7a8645d4e85e512c0e125e1254f3cabab5b|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1b70b2d9afd80d866f2125e6bc1cb88feb06b95292a115d589f8674ba71f21d1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6d0f58b7c87e60375f3b17ed6aa21aa9c37eda515aaf7f6a54cbcdaef9db39fa|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0785f55fabdc5a1c2fc4988aa8b93dabdf06b6b7732e0e96bffd0386b119e03e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2f92136bf3c1d8958f6207d4b3dfcdb209d1fa2335656593dc3c06041bd1c346|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d285cc5459d2fd699d8e663ec678994eeeea81d1a6893daf3560ffb078b6b4db|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
