# PQC Migration Scan Report

Generated: 2026-11-24 13:21:50 +09:00 (UTC+09:00)

Findings to review (items): 6 / Informational findings (items): 1 / Files inspected: 4

|Severity|Count (all findings)|
|---|---:|
|critical|2|
|info|1|
|medium|4|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[TLS_RSA_CIPHER_SUITE](#rule-0) RSA-based TLS cipher suite detected.|critical|quantum-vulnerable|1|1|
|[TLS_RSA_KEY_EXCHANGE](#rule-1) TLS RSA key-exchange suite detected.|critical|quantum-vulnerable|1|1|
|[PHP_CRYPTO_API](#rule-2) Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved.|medium|quantum-uncertain|4|2|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_PQC_API](#rule-3) Exact PHP PQC family and local receiver reference; parameters, provider and active use require separ...|info|quantum-safe|1|1|

<a id="rule-0"></a>

## TLS_RSA_CIPHER_SUITE

RSA-based TLS cipher suite detected.

Check: Identify static RSA key exchange versus RSA authentication in DHE/ECDHE suites; inspect active use before planning compatible hybrid key agreement and PQC authentication.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[privacy.php:1:51](#finding-0)|TLS_RSA_***ARY078|Review the TLS reference before changing effective settings / [Review the TLS reference before changing effective settings](#action-1) / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)

### privacy.php:1:51 — Matched rules

- `TLS_RSA_CIPHER_SUITE` (critical): RSA-based TLS cipher suite detected. — Identify static RSA key exchange versus RSA authentication in DHE/ECDHE suites; inspect active use before planning compatible hybrid key agreement and PQC authentication.

<a id="source-0"></a>

### privacy.php:1:51

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## TLS_RSA_KEY_EXCHANGE

TLS RSA key-exchange suite detected.

Check: Evaluate standardized hybrid key exchange such as X25519MLKEM768 with compatible peers; draft Kyber groups have different wire formats.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[privacy.php:1:51](#finding-1)|TLS_RSA_***ARY078|Review the TLS reference before changing effective settings / [Review the TLS reference before changing effective settings](#action-1) / [Source](#source-1)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)

### privacy.php:1:51 — Matched rules

- `TLS_RSA_KEY_EXCHANGE` (critical): TLS RSA key-exchange suite detected. — Evaluate standardized hybrid key exchange such as X25519MLKEM768 with compatible peers; draft Kyber groups have different wire formats.

<a id="source-1"></a>

### privacy.php:1:51

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## PHP_CRYPTO_API

Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved.

Check: Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[computed.php:1:90](#finding-2)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-2)|
|<a id="finding-3"></a>[computed.php:1:115](#finding-3)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-3)|
|<a id="finding-4"></a>[spread.php:1:73](#finding-4)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-4)|
|<a id="finding-5"></a>[spread.php:1:98](#finding-5)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-5)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-2)

### computed.php:1:90 — Matched rules

- `PHP_CRYPTO_API` (medium): Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-2"></a>

### computed.php:1:90

    > 1 [masked-sensitive-file-content]

### computed.php:1:115 — Matched rules

- `PHP_CRYPTO_API` (medium): Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-3"></a>

### computed.php:1:115

    > 1 [masked-sensitive-file-content]

### spread.php:1:73 — Matched rules

- `PHP_CRYPTO_API` (medium): Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-4"></a>

### spread.php:1:73

    > 1 [masked-sensitive-file-content]

### spread.php:1:98 — Matched rules

- `PHP_CRYPTO_API` (medium): Bounded PHP primitive API or opaque signature-wrapper reference; generic keys remain unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-5"></a>

### spread.php:1:98

    > 1 [masked-sensitive-file-content]

<a id="rule-3"></a>

## PHP_PQC_API

Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence.

Check: Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[keyedwrite.php:1:17](#finding-6)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-6)|

### keyedwrite.php:1:17 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-6"></a>

### keyedwrite.php:1:17

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

- [CBOM](cbom.json): 7
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|4|
|Read successfully|4|
|Read failed|0|
|Analyzed|2|
|Partial|2|
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
|Only part of inspection completed (detector_partial): Source syntax could not be fully interpreted. Parsed regions were inspected; unresolved syntax remains unknown. Check generated syntax, language version and the source file separately.|1 records; keyedwrite.php|
|Inspection limit reached (detector_truncated): Inspection limit: php_same_name_bindings; limit=64; processed=64. Retained findings remain available; omitted findings are unknown. Inspect the remaining source separately.|1 records; privacy.php|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-6160e3267c19eb59e11760c392ef748fcfe13a0d4f58c89d682f55ea009036e8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-497962c14c0d96626fb14c9f54a5e588f1d43f63b7ae9d13e2540422424c27bb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-07170779bea11c6ca56f6111a46e1c129d3f94c1e76bebd5316442b0ebb56442|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1039ceb04807c5cc94399a1227878e1bc284044f5c4fd2d5d5068ac196cc3377|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-4cb449fdf02b377857a3647d0dd533c3513dfc14c177949c5314ac8f4f3b5894|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-5d24c2bfd1c9ec854d31b711b5402b642d5c3682b254804af20b8f50483af949|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2b7da7c0953039fba1ea2a8ab48564e75014ba9c7ffce53ea41690c77496065f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
