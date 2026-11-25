# PQC Migration Scan Report

Generated: 2026-11-24 17:19:54 +09:00 (UTC+09:00)

Findings to review (items): 2 / Informational findings (items): 2 / Files inspected: 2

|Severity|Count (all findings)|
|---|---:|
|info|2|
|medium|2|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[DEP_BOUNCYCASTLE](#rule-0) BouncyCastle dependency detected.|medium|quantum-uncertain|2|2|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[DEP_NUGET_CRYPTO](#rule-1) NuGet cryptographic package assertion; wrapper ownership and runtime support require verification.|info|quantum-uncertain|2|2|

<a id="rule-0"></a>

## DEP_BOUNCYCASTLE

BouncyCastle dependency detected.

Check: Verify BouncyCastle PQC support status.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[Directory.Packages.props:1:1](#finding-0)|bouncyca***@2.8.0|Verify BouncyCastle PQC support status. / [Source](#source-0)|
|<a id="finding-1"></a>[a.csproj:1:1](#finding-1)|bouncyca***@2.8.0|Verify BouncyCastle PQC support status. / [Source](#source-1)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-0)
- [Track PQC readiness of dependent libraries](#action-1)

### Directory.Packages.props:1:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-0"></a>

### Directory.Packages.props:1:1

    > 1 [masked-sensitive-file-content]

### a.csproj:1:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-1"></a>

### a.csproj:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## DEP_NUGET_CRYPTO

NuGet cryptographic package assertion; wrapper ownership and runtime support require verification.

Check: Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[Directory.Packages.props:1:1](#finding-2)|bouncyca***@2.8.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-2)|
|<a id="finding-3"></a>[a.csproj:1:1](#finding-3)|bouncyca***@2.8.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-3)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-0)
- [Track PQC readiness of dependent libraries](#action-1)

### Directory.Packages.props:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-2"></a>

### Directory.Packages.props:1:1

    > 1 [masked-sensitive-file-content]

### a.csproj:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-3"></a>

### a.csproj:1:1

    > 1 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Verify .NET target, package and operating-system support (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. Deployed runtime, OS, provider and native backend versions are unknown. Target frameworks, global.json SDK selection and NuGet package declarations are separate source claims, not the installed runtime.
2. .NET 10 documents MLKem, MLDsa and SlhDsa. Check the selected parameter, OS/backend support and IsSupported or IsAlgorithmSupported in the deployed environment; Linux facilities may require OpenSSL 3.5 or later. Do not generalize platform support across Windows, Linux and macOS.
3. .NET 11 CompositeMLKem and Hpke facilities require review of the exact API and experimental diagnostics. HPKE alone does not identify a PQC KEM. Check Bouncy Castle package/version independently of the .NET target.
4. ASP.NET/Kestrel TLS depends on the OS/TLS backend and peer negotiation; cryptographic API availability does not enable PQC TLS. Data Protection AES payloads are not a reason to replace symmetric encryption with a KEM.

References:

- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10/libraries
- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-11/libraries
- https://learn.microsoft.com/en-us/dotnet/standard/security/cross-platform-cryptography
- https://learn.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel/endpoints?view=aspnetcore-10.0
<a id="action-1"></a>


#### Track PQC readiness of dependent libraries (planned)

Dependency-level crypto libraries strongly influence migration options and operational timelines.

1. Record current versions in dependency-sbom.json and define upgrade targets.
2. Subscribe to release notes for PQC feature availability and API changes.
3. Run integration tests for PQC or hybrid modes before promoting to production.

References:

- https://cyclonedx.org/
- https://spdx.dev/

## Inventory exports

- [CBOM](cbom.json): 4
- [Dependency SBOM](dependency-sbom.json): 2


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|2|
|Read successfully|2|
|Read failed|0|
|Analyzed|1|
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
|Only part of inspection completed (detector_partial): Detector completed only part of the inspection.|1 records; a.csproj|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-0f2157758b408636fc107042c4e1f9d4f56a298d5df1321176096731437a3de3|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-60da681ec35d0fe37f8a538cec4611c7fce1593c2d61e3518b47a358ad903543|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-89f9d7c0e8b68731f6e953619f48efb0a6699995b329d0dfdd2a44125dff3d66|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f9954ff8d4f578e0e8ba3d633b3de6dd9391713bd00db817d18ca1ba3de98a98|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-46ed4fdef9557584d61dca0b78bece74b92bc904c02b9387879d377d74e9d640|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-dc49ab12e93216120cdeb24bdba23cea671441a9dd0728ed92bb923a03392467|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
