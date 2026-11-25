# PQC Migration Scan Report

Generated: 2026-11-24 13:17:38 +09:00 (UTC+09:00)

Findings to review (items): 0 / Informational findings (items): 1 / Files inspected: 1

|Severity|Count (all findings)|
|---|---:|
|info|1|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PHP_PQC_API](#rule-0) Exact PHP PQC family and local receiver reference; parameters, provider and active use require separ...|info|quantum-safe|1|1|

<a id="rule-0"></a>

## PHP_PQC_API

Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence.

Check: Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[alias.php:1:74](#finding-0)|\[masked-sensitive-content\]|Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection. / [Source](#source-0)|

### alias.php:1:74 — Matched rules

- `PHP_PQC_API` (info): Exact PHP PQC family and local receiver reference; parameters, provider and active use require separate evidence. — Verify the specific Sodium or ParagonIE implementation and version. X-Wing is distinct from standalone ML-KEM and TLS hybrids; source requests prove no runtime protection.

<a id="source-0"></a>

### alias.php:1:74

    > 1 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.


## Inventory exports

- [CBOM](cbom.json): 1
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
|observation-v1-4bde8a654eba0e5b9aee49fa37662410297c1ab649fbc3bafc0f4ac2b9232809|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
