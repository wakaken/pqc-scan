# PQC Migration Scan Report

Generated: 2026-11-24 14:45:59 +09:00 (UTC+09:00)

Findings to review (items): 2 / Informational findings (items): 6 / Files inspected: 1

|Severity|Count (all findings)|
|---|---:|
|info|6|
|medium|2|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[CSHARP_CRYPTO_API](#rule-0) C# cryptographic API or qualified library reference; effective key use requires verification.|medium|quantum-uncertain|2|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[CSHARP_CRYPTO_REFERENCE](#rule-1) C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks.|info|quantum-uncertain|3|1|
|[CSHARP_PQC_API](#rule-2) C# PQC or composite/HPKE reference; source alone does not establish platform support or active prote...|info|quantum-uncertain|3|1|

<a id="rule-0"></a>

## CSHARP_CRYPTO_API

C# cryptographic API or qualified library reference; effective key use requires verification.

Check: Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[api.cs:1:395](#finding-0)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-0)|
|<a id="finding-1"></a>[api.cs:1:437](#finding-1)|\[masked-sensitive-content\]|Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use. / [Source](#source-1)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### api.cs:1:395 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-0"></a>

### api.cs:1:395

    > 1 [masked-sensitive-file-content]

### api.cs:1:437 — Matched rules

- `CSHARP_CRYPTO_API` (medium): C# cryptographic API or qualified library reference; effective key use requires verification. — Review the resolved primitive and purpose, platform and provider. A .NET or Bouncy Castle source reference does not establish effective runtime use.

<a id="source-1"></a>

### api.cs:1:437

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## CSHARP_CRYPTO_REFERENCE

C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks.

Check: Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[api.cs:1:7](#finding-2)|\[masked-sensitive-content\]|Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately. / [Source](#source-2)|
|<a id="finding-3"></a>[api.cs:1:24](#finding-3)|\[masked-sensitive-content\]|Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately. / [Source](#source-3)|
|<a id="finding-4"></a>[api.cs:1:65](#finding-4)|\[masked-sensitive-content\]|Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately. / [Source](#source-4)|

Shared checks for these locations (not verified facts about each line):

- [Introduce crypto abstraction boundary for algorithm agility](#action-0)

### api.cs:1:7 — Matched rules

- `CSHARP_CRYPTO_REFERENCE` (info): C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks. — Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

<a id="source-2"></a>

### api.cs:1:7

    > 1 [masked-sensitive-file-content]

### api.cs:1:24 — Matched rules

- `CSHARP_CRYPTO_REFERENCE` (info): C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks. — Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

<a id="source-3"></a>

### api.cs:1:24

    > 1 [masked-sensitive-file-content]

### api.cs:1:65 — Matched rules

- `CSHARP_CRYPTO_REFERENCE` (info): C# TLS library or OQS wrapper reference; identity and runtime capability require separate checks. — Distinguish Bouncy Castle TLS, the obsolete official dotnetOQS wrapper and the independent OpenForge project; verify protocol settings, maintenance, native library and selected algorithms separately.

<a id="source-4"></a>

### api.cs:1:65

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## CSHARP_PQC_API

C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection.

Check: Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-5"></a>[api.cs:1:110](#finding-5)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-5)|
|<a id="finding-6"></a>[api.cs:1:314](#finding-6)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-6)|
|<a id="finding-7"></a>[api.cs:1:334](#finding-7)|\[masked-sensitive-content\]|Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component. / [Source](#source-7)|

### api.cs:1:110 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-5"></a>

### api.cs:1:110

    > 1 [masked-sensitive-file-content]

### api.cs:1:314 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-6"></a>

### api.cs:1:314

    > 1 [masked-sensitive-file-content]

### api.cs:1:334 — Matched rules

- `CSHARP_PQC_API` (info): C# PQC or composite/HPKE reference; source alone does not establish platform support or active protection. — Verify the exact primitive, parameter set, .NET or Bouncy Castle version and provider. Composite and HPKE constructions remain unresolved and are not equivalent to a component.

<a id="source-7"></a>

### api.cs:1:334

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

- [CBOM](cbom.json): 6
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
|observation-v1-939ee74bcd751992d80e69765f4931481ed6edafbcc7f7765e5b57ab6f32e83a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-48f7643291d8061476820e97db8a962880bbc652804fe156e820d96124addf1e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-063acac2b25cc0bad09b28d528529485e28874e86b2f9d2846b3bbba27cf6a18|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d9df691f825530a8138a9879f75039261b8d7d0ab6ce0122d0a7623613464d42|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-97030ac6999c97900308b36f35f0b774b78a4a3462624cce581ec76c180d7deb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2a9f7f2cd0664c16a6f5c89fa4718f89f71333cf8ffbdac3634fbf58854a9b3e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe6a23674dad255da2a3076e61b14c8a3294071ba9e5ffab8dcff9db5d260823|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c3777f9a6c225f247a9878cd4a4533ee06d03184f39211871753cdb0fbdc61cb|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
