# PQC Migration Scan Report

Generated: 2026-11-17 06:26:07.721993 UTC

Risk candidates: 3 / Inventory findings: 0 / Scanned files: 1

Cryptographic references found in source code and configuration.

|Severity|Count (all findings)|
|---|---:|
|high|3|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[TLS_VERSION_1_0](#rule-0) TLS 1.0 name referenced; effective configuration and runtime use are not established.|high|non-quantum-risk|3|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## TLS_VERSION_1_0

TLS 1.0 name referenced; effective configuration and runtime use are not established.

Check: First distinguish constants, comments and test references from effective configuration. Prefer TLS 1.3; restrict to TLS 1.3 when all peers support it. Retain securely configured TLS 1.2 only for a documented interoperability need. Disable TLS 1.0 and 1.1.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[tls.conf:2:1](#finding-0)|TLSv1.0|First distinguish constants, comments and test references from effective configuration. Prefer TLS 1.3; restrict to TLS 1.3 when all peers support it. Retain securely configured TLS 1.2 only for a documented interoperability need. Disable TLS 1.0 and 1.1. / [Source](#source-0)|
|<a id="finding-1"></a>[tls.conf:2:9](#finding-1)|TLSv1.0|First distinguish constants, comments and test references from effective configuration. Prefer TLS 1.3; restrict to TLS 1.3 when all peers support it. Retain securely configured TLS 1.2 only for a documented interoperability need. Disable TLS 1.0 and 1.1. / [Source](#source-1)|
|<a id="finding-2"></a>[tls.conf:3:1](#finding-2)|TLSv1.0|First distinguish constants, comments and test references from effective configuration. Prefer TLS 1.3; restrict to TLS 1.3 when all peers support it. Retain securely configured TLS 1.2 only for a documented interoperability need. Disable TLS 1.0 and 1.1. / [Source](#source-2)|

Shared checks for these locations (not verified facts about each line):

- [Review the TLS reference before changing effective settings](#action-0)

<a id="source-0"></a>

### tls.conf:2:1

      1 
    > 2 TLSv1.0 TLSv1.0
      3 TLSv1.0

<a id="source-1"></a>

### tls.conf:2:9

      1 
    > 2 TLSv1.0 TLSv1.0
      3 TLSv1.0

<a id="source-2"></a>

### tls.conf:3:1

      1 
      2 TLSv1.0 TLSv1.0
    > 3 TLSv1.0

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Review the TLS reference before changing effective settings (planned)

A source name or token does not prove enabled configuration or a negotiated connection. ECDHE_RSA/DHE_RSA use RSA authentication, not static RSA key exchange.

1. Distinguish constants, comments, tests and active settings.
2. Inspect key agreement, authentication and protocol version separately; confirm existing hybrid support and peer compatibility before migration.

References:

- https://www.rfc-editor.org/rfc/rfc8422.html#section-2.2

## Inventory exports

- [CBOM](cbom.json): 2
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

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-f1f7f9528817d447ef715026185d86f781d79588320ee4150b72f33a24a12929|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-6fe3c32750f30ccea49b679b94650375184cb961eec309ba8d9ec820e804b546|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-43e56672c75f31d073be339caef37037936d4a7b7f1e6a6598b7bed2f305bdc6|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
