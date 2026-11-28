# PQC Migration Scan Report

Generated: 2026-11-24 13:27:24 +09:00 (UTC+09:00)

Findings to review (items): 1 / Informational findings (items): 3 / Files inspected: 1

|Severity|Count (all findings)|
|---|---:|
|info|3|
|medium|1|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_CRYPTO_API](#rule-0) PHP cryptographic API or library reference; key type may be unresolved.|medium|quantum-uncertain|1|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_CRYPTO_REFERENCE](#rule-1) PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved.|info|quantum-uncertain|1|1|
|[PHP_PQC_API](#rule-2) PHP PQC algorithm or X-Wing reference; runtime support requires verification.|info|quantum-safe|2|1|

<a id="rule-0"></a>

## PHP_CRYPTO_API

PHP cryptographic API or library reference; key type may be unresolved.

Check: Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[api.php:4:1](#finding-0)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### api.php:4:1 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-0"></a>

### api.php:4:1

    > 4 [masked-sensitive-file-content]

<a id="rule-1"></a>

## PHP_CRYPTO_REFERENCE

PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved.

Check: Identify the specific third-party OQS wrapper and native library. A qualified reference does not establish installation, algorithm selection or enabled functionality.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[api.php:5:5](#finding-1)|\[masked-sensitive-content\]|Identify the specific third-party OQS wrapper and native library. A qualified reference does not establish installation, algorithm selection or enabled functionality. / [Source](#source-1)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### api.php:5:5 — Matched rules

- `PHP_CRYPTO_REFERENCE` (info): PHP OQS KEM wrapper reference; installation and selected algorithm are unresolved. — Identify the specific third-party OQS wrapper and native library. A qualified reference does not establish installation, algorithm selection or enabled functionality.

<a id="source-1"></a>

### api.php:5:5

    > 5 [masked-sensitive-file-content]

<a id="rule-2"></a>

## PHP_PQC_API

PHP PQC algorithm or X-Wing reference; runtime support requires verification.

Check: Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[api.php:2:6](#finding-2)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-2)|
|<a id="finding-3"></a>[api.php:3:1](#finding-3)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-3)|

### api.php:2:6 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-2"></a>

### api.php:2:6

    > 2 [masked-sensitive-file-content]

### api.php:3:1 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-3"></a>

### api.php:3:1

    > 3 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography

## Inventory exports

- [CBOM](cbom.json): 4
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|1|
|Read successfully|1|
|Read failed|0|
|Analyzed|1|
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
|observation-v1-17d11f5c17acf2e5d97de30ded00e57767cb9ae15a2452b717354b797ee3bd17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-e2c98f0730b3ca01a6eefec935c0a911dbd99a11b7ea5edafde927ffffc9dbbc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a718c6f255411088e30dbe2efe02cbfcbab727405df9587784c6441efc703b17|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-520d1ec136bed83096e823fabed6b476c6d4d78e5ffdbb9d7a40df470415369e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
