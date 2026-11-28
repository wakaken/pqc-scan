# PQC Migration Scan Report

Generated: 2026-11-24 11:15:22 +09:00 (UTC+09:00)

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
|observation-v1-38ac87345644fca97d101c6e330077930756411e26a6138f862d268658562e20|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-4f0a9d18e732dcf1efcdca8118867fb7fd949400dbd7b98275af9f08f090f67e|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e393c05ce1d81231d2a1939a74fdde38cd0b6a294b48305a811caf05b61d90bc|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-84d91249a8cc1db52ceefb21e6ee672ef66bdac2038ea731d16a39b1f3c7e909|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-0c5579d1763655bfa94f32196a7a3bb4f082c246c642bc257e0d8a0f4932d8df|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-9cf342922ee37a0c69c3f89ef4f6972a949332c049847d3aa16447aa50d52edb|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-d4bcffe901779b85d6c40064a5ca424437707df1d4fa19b4398b9c30e4598460|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e8f96ec7765563f047164d9dffdd35416bb6401e57060e4622e800e2ecbfa750|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-527f3724193ce84534ad99d88cd09868318a2f55382e644b9a79dd94ea80d7c5|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6247ae6c9aece9e968d601db059f168c15466904b810f1f88d839c0a619e6294|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
