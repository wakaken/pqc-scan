# PQC Migration Scan Report

Generated: 2026-11-24 14:41:57 +09:00 (UTC+09:00)

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
|observation-v1-7e1a80f362bc8dbc3ea94dfb01069e2ee769d36fb8efad5c32b75169ea40e8d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bc89c1ffbe241ffe5796413d1da6da18ee7228afcc63b5cd8efe892482529edc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-960b640ce1053f7ce19bb018f62d7b240ae5e87097e5376552fa03a1ebdde12c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-35f9250a605d93986e227fb84bbc97b5b965403038eeb994881740ee2669e8bf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-752ca013a22905088a520bbcbedd1145f62b75a16746ef0b722c019de61c0042|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0c189631fadcd110d75eed2d90be19ef1cda95176a88b65ac86695f537f0ebb3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c434db24ee735941390b9be29233afa75bc3a5bb4fd68c5a547fcd7a6b24962a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1e468e11599a9c2a944fd0719addcc6cf42ead2018894d83c98ce85d72373bc0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
