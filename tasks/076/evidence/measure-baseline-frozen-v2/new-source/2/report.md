# PQC Migration Scan Report

Generated: 2026-11-24 11:16:27 +09:00 (UTC+09:00)

Findings to review (items): 10 / Informational findings (items): 0 / Files inspected: 588

|Severity|Count (all findings)|
|---|---:|
|high|6|
|medium|4|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[OPENSSL_RSA_API](#rule-0) OpenSSL RSA API call detected.|high|quantum-vulnerable|6|6|
|[API_OPENSSL_RSA_NEW](#rule-1) OpenSSL low-level RSA API usage detected.|medium|quantum-vulnerable|4|4|

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

<a id="rule-1"></a>

## API_OPENSSL_RSA_NEW

OpenSSL low-level RSA API usage detected.

Check: Prefer high-level OpenSSL EVP APIs to ease PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[c-openssl-rsa_generate_key_ex-negative-comment.c:1:4](#finding-6)|RSA_generate_key_ex|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-6)|
|<a id="finding-7"></a>[c-openssl-rsa_generate_key_ex-positive.c:1:21](#finding-7)|RSA_generate_key_ex|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-7)|
|<a id="finding-8"></a>[c-openssl-rsa_new-negative-comment.c:1:4](#finding-8)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-8)|
|<a id="finding-9"></a>[c-openssl-rsa_new-positive.c:1:21](#finding-9)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-9)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-2)

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-6"></a>

### c-openssl-rsa_generate_key_ex-negative-comment.c:1:4

    > 1 // RSA_generate_key_ex(arg);

### c-openssl-rsa_generate_key_ex-positive.c:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-7"></a>

### c-openssl-rsa_generate_key_ex-positive.c:1:21

    > 1 void sample(void) { RSA_generate_key_ex(arg); }

### c-openssl-rsa_new-negative-comment.c:1:4 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-8"></a>

### c-openssl-rsa_new-negative-comment.c:1:4

    > 1 // RSA_new(arg);

### c-openssl-rsa_new-positive.c:1:21 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-9"></a>

### c-openssl-rsa_new-positive.c:1:21

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

- [CBOM](cbom.json): 10
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|588|
|Read successfully|588|
|Read failed|0|
|Analyzed|588|
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
|observation-v1-bb961ec6bde9c37c9740f3cc8fbb0d4b16220f76955aeb8d67c1004a70b91519|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-fb32168897bb4774ec8f8ea8159b9c64e4a5e1c0e38cf004259938e29381e04f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-15f6129d7f70c293ab8617ff1a91c2f93721915cbb19cf44fa082940506c561f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6e62447f33d3d9fda421bec08a7f73df2cb52e8285c5b626b81421a8cd8f8e52|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-f850194f3bd137a56ea363508124d92c90db3e6a2ac751d73166c7321174d921|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-b4323aaea260d11bf1eb2db80ad7ab92eb42c704b7ef180f5605bd3245375715|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5ee00a333aee99875a37f2d145ae578ce6a10d09a87ed9f4a3f6c9a2eab76546|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-57ac53592ac6b483623f227363a010aa5019284bda8b59477cd3bd825b65e4db|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9e0e2875bfda61171f5931a4c241af5276694de503778ac2366ed5d91180dd1e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-28cf5ac1a8a8bad9588a315db386f15468b1be2a4e20f28622e345bd6201f881|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
