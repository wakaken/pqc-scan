# PQC Migration Scan Report

Generated: 2026-11-24 11:23:27 +09:00 (UTC+09:00)

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
|observation-v1-29d4b035fd6221372d41bd05a9c6a7acb662501c995b6ea0a39a1fe1b0054440|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-36e464c12673a5f8b9aa299f4a4cb5121874265cf62d3b8054e98d3c3244738e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-26a1e528dc6d118ef15288d82a8a7a6d18449627896ad95f68b0e46a957f46e5|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-eb363409026b8dae07574c8baad96544fcd6d8b3673613f8664f19a79a2deb11|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e5662d2f3ce235a25c907e144ce0fa7a251074897d887c80d9899794b8e04e38|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-acbb3673d0f90f93d153908d41d3e8471a1ebe48527a280bcd6e6a1ffb360561|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8486fe93b1155ab092d756ac48acb1013e6bc2f669214a9be44710e23f3485bd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3d15b7ad608ee6e9c489a75b59f71ed057cb0c5189c31a9632771735e3451953|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-912881a6a2ce6c56804525cebea653ef3a631b7c518b2b0b9afa307cefe03d93|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-2e9fbc57e7012fb6ffeaf91fce15c3509793e20bbfbf82e209368e6c7e556fbd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6c59bb55bf09aec3aa6380090e7590d927167bf5f9119d3bd0d8d19de2a2e998|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-cc2d4a870fc676708782c8c44ec9f4282ee6c00a9ef3310516402b9f88173938|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6b7255812420127977fecc5d5fd05ae2813bf2d7d442dd7fa98ac9b4393154bd|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-131315bfc1b056fe8dc663b5d4c7fe486d2092775f2a8f3511cfd684c17525a8|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-4bcf7ebe640a0c728d19652e457ccdbe459f1e22801730dff63d1eea248911fa|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-8be7053061d9b7c50983498e018248f850a263dcb2ae55005981ddeb3d25a1d0|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-7401571cceb214e4f681275c3ddaa4470955949906682686b73199448ce202da|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a3dd565d4a5444cd86358b6de21c14b622e1e227e95898fbb098651197c0602c|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-08d0556db662f9ba75223d05305c40e900a67b3ce524e98bf7626aee7de0b174|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-bcf6b701ba47aa14003ca8a9e6d287f8d9f032e9fd2d9a5b5bb9f699655a06f3|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d8c84de7de2d8ecba5b7ca1bd0135462e55ddccb8af14584941a84cba0703f84|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-165703ca01ddb34ebf4a25d52354d986b0e01f5991eeb9aafb00fdb2d6823fd1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a7079d805944dd0b4f00ca5986f7fa8953eefbd1c1fb83c9627e330e0ec47c76|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-bb826be5993fd4fe34e08e8be00391a91cf311df5576ca9ae39041904ef4e6c7|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3d4ee935f107b86e4c2312367f9fc5838b7b321710a0eb07abac40ccd186b8b1|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-62a78636a6b3fb400dc6873e3f1a3b850933708b7cb8e1260f0a310fec8d3160|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b09b1152f11e65e984636c4c9cbf25bb01f475141f823a287e3d8cf74f2c8242|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a78148c34a4570bb78c2748439bae20f62258184531081883332bf2dbb1e693e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-a55b8c980b733921ec2bd2b4599f10217e79ce13a62e784dc2b96f91a61ba831|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-3035351230aa53dd3aa3527af623c411cc401d53484f202af91b72ceaaba26d0|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f2ad793990398b5e4633f50928f093ba63b1ae67570fb38bec0b9acf04c78293|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-88814c8b45494e93fab3ca269581cee82df14c9425605d9c548c854626acf463|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-bb2b4d01883397eee26e8542026c80d7ee93340b11247c96d8b0fc158f9d4b43|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e311ae624af8b502a86c1dfdda09e539689563793f20da140ee0bd0c9fdf3252|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-27357ea4d18b10fd271fae2afced9ccc4dd4e82e1c7d08797d1f0f41bad94b0d|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-ed8016f6f948bb88bc7135c948357090592849a249e908c5ed29e924d96749cb|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
