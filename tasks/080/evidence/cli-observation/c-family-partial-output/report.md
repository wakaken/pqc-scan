# PQC Migration Scan Report

Generated: 2026-11-24 16:30:07 +09:00 (UTC+09:00)

Findings to review (items): 1 / Informational findings (items): 0 / Files inspected: 1

|Severity|Count (all findings)|
|---|---:|
|medium|1|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[C_FAMILY_CRYPTO_API](#rule-0) C/C++ cryptographic API or configuration reference; effective primitive and runtime use require veri...|medium|quantum-uncertain|1|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## C_FAMILY_CRYPTO_API

C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification.

Check: Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[unknown.c:1:10](#finding-0)|\[masked-sensitive-content\]|Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation. / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### unknown.c:1:10 — Matched rules

- `C_FAMILY_CRYPTO_API` (medium): C/C++ cryptographic API or configuration reference; effective primitive and runtime use require verification. — Verify the primitive, purpose, linked provider and build configuration. A source request or provider/group setting does not establish runtime capability or negotiation.

<a id="source-0"></a>

### unknown.c:1:10

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

- [CBOM](cbom.json): 1
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|1|
|Read successfully|1|
|Read failed|0|
|Analyzed|0|
|Partial|1|
|Analysis failed|0|
|Unsupported|0|
|Not applicable|0|
|Unknown|0|
|Enumerated exclusions|0|
|Enumeration failures|0|
|Unenumerated population|Unknown|
