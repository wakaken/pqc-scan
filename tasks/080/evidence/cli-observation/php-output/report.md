# PQC Migration Scan Report

Generated: 2026-11-24 16:30:06 +09:00 (UTC+09:00)

Findings to review (items): 1 / Informational findings (items): 2 / Files inspected: 1

|Severity|Count (all findings)|
|---|---:|
|info|2|
|medium|1|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_CRYPTO_API](#rule-0) PHP cryptographic API or library reference; key type may be unresolved.|medium|quantum-uncertain|1|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_PQC_API](#rule-1) PHP PQC algorithm or X-Wing reference; runtime support requires verification.|info|quantum-safe|2|1|

<a id="rule-0"></a>

## PHP_CRYPTO_API

PHP cryptographic API or library reference; key type may be unresolved.

Check: Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[api.php:1:134](#finding-0)|\[masked-sensitive-content\]|Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use. / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### api.php:1:134 — Matched rules

- `PHP_CRYPTO_API` (medium): PHP cryptographic API or library reference; key type may be unresolved. — Review the resolved primitive and purpose; PHP source alone does not establish loaded extensions, key type or active use.

<a id="source-0"></a>

### api.php:1:134

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## PHP_PQC_API

PHP PQC algorithm or X-Wing reference; runtime support requires verification.

Check: Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[api.php:1:12](#finding-1)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-1)|
|<a id="finding-2"></a>[api.php:1:111](#finding-2)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-2)|

### api.php:1:12 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-1"></a>

### api.php:1:12

    > 1 [masked-sensitive-file-content]

### api.php:1:111 — Matched rules

- `PHP_PQC_API` (info): PHP PQC algorithm or X-Wing reference; runtime support requires verification. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-2"></a>

### api.php:1:111

    > 1 [masked-sensitive-file-content]

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

- [CBOM](cbom.json): 3
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
|observation-v1-d846752221d036915ed8b19a8438382760e38f2c6002ebe5997f55648c120926|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-29b1471af85313a8223146f87dc819395ff2dad7c7ec9b657a517bed5f65390a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2c686142182d235c863a27c71559c64440250aa8d008e29a289a9c38e5b3c924|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
