# PQC Migration Scan Report

Generated: 2026-11-24 14:27:08 +09:00 (UTC+09:00)

Findings to review (items): 0 / Informational findings (items): 6 / Files inspected: 4

|Severity|Count (all findings)|
|---|---:|
|info|6|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[CSHARP_PQC_API](#rule-0) C# PQC or composite/HPKE reference; source alone does not establish platform support or active prote...|info|quantum-uncertain|6|3|

<a id="rule-0"></a>

## CSHARP_PQC_API

C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection.

Check: Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[capture.cs:1:62](#finding-0)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-0)|
|<a id="finding-1"></a>[capture.cs:1:103](#finding-1)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-1)|
|<a id="finding-2"></a>[capture.cs:1:145](#finding-2)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-2)|
|<a id="finding-3"></a>[capture.cs:1:167](#finding-3)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-3)|
|<a id="finding-4"></a>[generic.cs:1:167](#finding-4)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-4)|
|<a id="finding-5"></a>[property.cs:1:87](#finding-5)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-5)|

### capture.cs:1:62 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-0"></a>

### capture.cs:1:62

    > 1 [masked-sensitive-file-content]

### capture.cs:1:103 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-1"></a>

### capture.cs:1:103

    > 1 [masked-sensitive-file-content]

### capture.cs:1:145 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-2"></a>

### capture.cs:1:145

    > 1 [masked-sensitive-file-content]

### capture.cs:1:167 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-3"></a>

### capture.cs:1:167

    > 1 [masked-sensitive-file-content]

### generic.cs:1:167 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-4"></a>

### generic.cs:1:167

    > 1 [masked-sensitive-file-content]

### property.cs:1:87 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-5"></a>

### property.cs:1:87

    > 1 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.


## Inventory exports

- [CBOM](cbom.json): 4
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|4|
|Read successfully|4|
|Read failed|0|
|Analyzed|3|
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
|Only part of inspection completed (detector_partial): Source syntax could not be fully interpreted. Parsed regions were inspected; unresolved syntax remains unknown. Check generated syntax, language version and the source file separately.|1 records; alias.cs|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-90b68735c76d184f35092260308af202bb452558023ddae705a945098521c6b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3d8f98905cb77e712e0ec35d767970cfbc8a1fc11e0fb6f49dac28ace84c033d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0ac8a66fae17abfa82edcb2a0579f7442d7980fdc724e5f840682034eb756321|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a9ad6bf03c8e0cc862895741b6dc5e87a5c8109da0ed962e8e887a1c7387a3c6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9dd7298efd5efc7e795ff60073cf9218724f0d8412cfe44e9ae7d6a219c181d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-44d5b88154863169289df5524da392f4e5e286fa7493cef048148a2ae73b3c59|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
