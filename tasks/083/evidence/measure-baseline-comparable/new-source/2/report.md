# PQC Migration Scan Report

Generated: 2026-11-24 18:35:52 +09:00 (UTC+09:00)

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
|observation-v1-5e4d5cc0f741533bcdb91fcc1783fba6d9b3cee5e79fe9f87a373ee4909fa907|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0c98603ef749eb139672402a7aa33594a8dc86748e44db05800a5030c05964f5|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2b87e028602b9a8cbaf2c185a45a32cb1cbf5d238e8d085e285b9199a3fc4e8f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-bb01cdc597ce0479053387d35d8e9f476b6c63765c754e202ae050a860cffe5f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-509b334d51064f8e461302f07756524fa7742e44c0abd5a67454bde2952c2a4a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-cdb8452714e449a4b5e15ac433bdece4707f506d9946ea24f9d3f35fd20c3e7e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5fa66eb2f2a5c62fd26fadaf98a0c8ec9c540968076ddc8f38753d89fef4f421|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0e8efc2471e229d3040ca61fc1abaad458bb73751bff8e403b0f7d26864dc41e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-752a2fe03b250d942ed8becad9c1381add27fd42d1f80c0ead9a8770f5ee7a66|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e08fc2fa8c2748979eca553ad06efc3f53d0c530a906a4cba4d7237ba2c970fc|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9e8acd8bf9496c131622e9716f46c9f516e2453c4fc4d698612982d7c6414f0e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a885dbf35ef04bd91664e33e7cdf277d1ed0137f5968db020afc092bd2b11b92|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-75a97b876f6afbe8e49419b1c0d4000ba173208decbba7eb59bf929ca283c7e2|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2260fbc8e8c7f8b7537ea99c29f643ce6a8f7c78433759291d70c5f7e62021f6|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9dcfe5652d247c7675d71bfa1aeb3d1f60b8798a2e8195b1e3eb920e51cee15c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a7042e58b5a99bad5a2876d882057813081a34fdba3614e18ccdb48e2ed39c95|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-84bca18b259058e207afcf8f7bf0714ca055e4f4dc8090b2768275e1f1c0b11a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3742e1e0272d2083c8acd0c87ab265c2172edd5e923af87ef70391d69ec316c9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d54fed1c2d4ed36cb0b7c5038957d5054f773a915f7dcd335c8a60719c0d49e7|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-330a4c444e0b5605f2076ebfcd8c98136025281ae36ce45f3ca88251336a9058|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b9f8d338a197664b96490aa6d1d77503dc0b00b77841bd1aecaba63a82459a6e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-40423ad67fb9b7678898e976fbd74df2bef1231fa551c13b410c4d3eb57df297|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f158d44b918e10ecab0eb680925401f294cecdda3dffd7c796925e70063995ce|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5650a6ea7d64ced8bd7257f0fc4a5a1c1820d6206af2309b722495a8ef15b91a|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-87c668c0b637588ac514f9bb698c88c00f33175056e71efbdda34175c6ff0c35|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3b9547b3aa40fb92aca2bb0995bbda5944fa5ff88654b018f58050b5c6f110e4|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3c70f3fc2298192d3b292f44aec7985e86a357a1a48d15144a623a1e391f73e9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ce72c9b1d6017fd4a9e8196d6389ddfce69bab3c1ac193391c6fbf3ab064d3f9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-1c67e08375f4157716b318bd1a6e135cd9338bec4e0b2fa38261581075304898|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d347957cfa30cd8af92959d7d489b3683adfaccdb85fa78d0a79fa83727d8abe|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-be514997b5dc004b774735b5f27aa14deb1223449d13373d6b6a63c63bbf36e0|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-569ca9d3ca13cf5c5698727a1b6e4c9b6582c6c03e4cfa4daa8ce97678133391|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e3b7e86743d9a515d1026576f9255b6e09c7868176a7a283b842924a98da84d8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f1e9fe038e4d67bdb59fca2649d15cdb66b29ef9ab4f5f083f7a22604c9f07da|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9a341690c018508beba7eda7744f86f4e3da9a5b11344609bf0f682c89411ac3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-603b225657113b1db1d4dd9c4838ff619cd055023a57fc0bfd64f2af40b3bce3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
