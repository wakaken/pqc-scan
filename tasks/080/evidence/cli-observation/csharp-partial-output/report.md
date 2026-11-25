# PQC Migration Scan Report

Generated: 2026-11-24 16:30:06 +09:00 (UTC+09:00)

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
|[CSHARP_PQC_API](#rule-0) C# PQC or composite/HPKE reference; source alone does not establish platform support or active prote...|info|quantum-uncertain|1|1|

<a id="rule-0"></a>

## CSHARP_PQC_API

C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection.

Check: Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[unknown.cs:1:18](#finding-0)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-0)|

### unknown.cs:1:18 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-0"></a>

### unknown.cs:1:18

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
