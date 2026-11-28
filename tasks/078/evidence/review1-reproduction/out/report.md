# PQC Migration Scan Report

Generated: 2026-11-24 13:15:58 +09:00 (UTC+09:00)

Findings to review (items): 3 / Informational findings (items): 6 / Files inspected: 5

|Severity|Count (all findings)|
|---|---:|
|info|6|
|medium|3|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[API_OPENSSL_RSA_NEW](#rule-0) OpenSSL low-level RSA API usage detected.|medium|quantum-vulnerable|1|1|
|[PHP_CRYPTO_API](#rule-1) Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved.|medium|quantum-uncertain|2|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_PQC_API](#rule-2) Exact PHP PQC family and local receiver reference; parameters, provider and active use require separ...|info|quantum-safe|6|3|

<a id="rule-0"></a>

## API_OPENSSL_RSA_NEW

OpenSSL low-level RSA API usage detected.

Check: Prefer high-level OpenSSL EVP APIs to ease PQC migration.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[privacy.php:1:76](#finding-0)|RSA_new|Prefer high-level OpenSSL EVP APIs to ease PQC migration. / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### privacy.php:1:76 — Matched rules

- `API_OPENSSL_RSA_NEW` (medium): OpenSSL low-level RSA API usage detected. — Prefer high-level OpenSSL EVP APIs to ease PQC migration.

<a id="source-0"></a>

### privacy.php:1:76

    > 1 <?php $k=\PQCrypto\MLDSA44\SigningKey::fromBytes('CANARY078_SECRET_A'); // RSA_new
      2 $a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;$a=1;...

<a id="rule-1"></a>

## PHP_CRYPTO_API

Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved.

Check: Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[arraywrite.php:1:105](#finding-1)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-1)|
|<a id="finding-2"></a>[arraywrite.php:1:130](#finding-2)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-2)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-1)

### arraywrite.php:1:105 — Matched rules

- `PHP_CRYPTO_API` (medium): Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-1"></a>

### arraywrite.php:1:105

    > 1 [masked-sensitive-file-content]

### arraywrite.php:1:130 — Matched rules

- `PHP_CRYPTO_API` (medium): Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-2"></a>

### arraywrite.php:1:130

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## PHP_PQC_API

Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence.

Check: Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-3"></a>[catch.php:1:17](#finding-3)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-3)|
|<a id="finding-4"></a>[catch.php:1:88](#finding-4)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-4)|
|<a id="finding-5"></a>[foreach.php:1:17](#finding-5)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-5)|
|<a id="finding-6"></a>[foreach.php:1:83](#finding-6)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-6)|
|<a id="finding-7"></a>[methodwrite.php:1:17](#finding-7)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-7)|
|<a id="finding-8"></a>[methodwrite.php:1:73](#finding-8)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-8)|

### catch.php:1:17 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-3"></a>

### catch.php:1:17

    > 1 [masked-sensitive-file-content]

### catch.php:1:88 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-4"></a>

### catch.php:1:88

    > 1 [masked-sensitive-file-content]

### foreach.php:1:17 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-5"></a>

### foreach.php:1:17

    > 1 [masked-sensitive-file-content]

### foreach.php:1:83 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-6"></a>

### foreach.php:1:83

    > 1 [masked-sensitive-file-content]

### methodwrite.php:1:17 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-7"></a>

### methodwrite.php:1:17

    > 1 [masked-sensitive-file-content]

### methodwrite.php:1:73 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-8"></a>

### methodwrite.php:1:73

    > 1 [masked-sensitive-file-content]

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


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography

## Inventory exports

- [CBOM](cbom.json): 9
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|5|
|Read successfully|5|
|Read failed|0|
|Analyzed|4|
|Partial|1|
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
|Inspection limit reached (detector_truncated): Inspection limit: php_same_name_bindings; limit=64; processed=64. Retained findings remain available; omitted findings are unknown. Inspect the remaining source separately.|1 records; privacy.php|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-24a5fba673632333079fb0fb045c6d5f0dfcf7291f8490d7ca9b7cc551c641ff|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-8feb0ac72a23d18deb096e125b20fe30e62acafccd6f32dc41cd3f85387a8f91|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-af66769f7c5a7ff38b10d1537830c5b915a5476efb56952a364d3e5f1485db19|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9e55ec3607055a66bbe74bc92ad777a4966925c1d211f4fdb5a8ebdeee3ed682|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fa0d7bea1c553661eb01a6566b434f1d3c69f849a5abad9100d6ee76a78ea825|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47a702a4c045d9da8f18cbd465d22d8deaec2b5c8ddcefbf91f518c610bf2241|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b688d8b53b9caed20618b5ae38828ab0604e352dbe3c9f9faa13dd696cedcf8b|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60a9efdbce812c86eea2a67422379f1708647ed1a54e9e8ae548bae50f07b96c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4de2008c1d6b452bf932c4a4271cd059a4e05be8c48ee5237a7d83c455c944ab|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
