# PQC Migration Scan Report

Generated: 2026-11-15 07:21:21.420299 UTC

Risk candidates: 3 / Inventory findings: 0 / Scanned files: 2

Cryptographic references found in source code and configuration.

|Severity|Count (all findings)|
|---|---:|
|critical|3|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[TLS_RSA_CIPHER_SUITE](#rule-0) RSA-based TLS cipher suite detected.|critical|quantum-vulnerable|1|1|
|[TLS_RSA_KEY_EXCHANGE](#rule-1) TLS RSA key-exchange suite detected.|critical|quantum-vulnerable|1|1|
|[TS_TLS_RSA_CIPHER_LITERAL](#rule-2) RSA-family TLS cipher suite literals detected in AST nodes.|critical|quantum-vulnerable|1|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## TLS_RSA_CIPHER_SUITE

RSA-based TLS cipher suite detected.

Check: Identify static RSA key exchange versus RSA authentication in DHE/ECDHE suites; inspect active use before planning compatible hybrid key agreement and PQC authentication.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[app.py:1:17](#finding-0)|TLS_RSA_***BC_SHA|Review the TLS reference before changing effective settings / [Review the TLS reference before changing effective settings](#action-1) / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Verify deployed runtime and protocol PQC capabilities](#action-2)

<a id="source-0"></a>

### app.py:1:17

    > 1 cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"

<a id="rule-1"></a>

## TLS_RSA_KEY_EXCHANGE

TLS RSA key-exchange suite detected.

Check: Evaluate standardized hybrid key exchange such as X25519MLKEM768 with compatible peers; draft Kyber groups have different wire formats.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[app.py:1:17](#finding-1)|TLS_RSA_***BC_SHA|Review the TLS reference before changing effective settings / [Review the TLS reference before changing effective settings](#action-1) / [Source](#source-1)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Verify deployed runtime and protocol PQC capabilities](#action-2)

<a id="source-1"></a>

### app.py:1:17

    > 1 cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"

<a id="rule-2"></a>

## TS_TLS_RSA_CIPHER_LITERAL

RSA-family TLS cipher suite literals detected in AST nodes.

Check: Identify the suite role: RSA_WITH uses static RSA key exchange; ECDHE_RSA and DHE_RSA use RSA authentication. Review hybrid key agreement and PQC authentication separately.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[app.py:1:17](#finding-2)|TLS_RSA_***BC_SHA|Review the TLS reference before changing effective settings / [Review the TLS reference before changing effective settings](#action-1) / [Source](#source-2)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Verify deployed runtime and protocol PQC capabilities](#action-2)

<a id="source-2"></a>

### app.py:1:17

    > 1 cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Adopt a phased hybrid PQC migration plan (near-term)

The detected primitive is vulnerable to cryptographically relevant quantum attacks. Hybrid rollout minimizes compatibility risk.

1. Inventory all call sites using this algorithm and owner teams.
2. Check for an existing classical + PQC composition; if migration is needed, evaluate compatible staged or hybrid modes before hard cut-over.
3. Define retirement dates for legacy classical algorithms.

References:

- https://www.nist.gov/itl/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
<a id="action-1"></a>


#### Review the TLS reference before changing effective settings (planned)

A source name or token does not prove enabled configuration or a negotiated connection. ECDHE_RSA/DHE_RSA use RSA authentication, not static RSA key exchange.

1. Distinguish constants, comments, tests and active settings.
2. Inspect key agreement, authentication and protocol version separately; confirm existing hybrid support and peer compatibility before migration.

References:

- https://www.rfc-editor.org/rfc/rfc8422.html#section-2.2
<a id="action-2"></a>


#### Verify deployed runtime and protocol PQC capabilities (planned)

Source declarations and primitive availability do not establish deployment or peer support.

1. cryptography 48 adds OpenSSL 3.5+ support for ML-KEM/ML-DSA; 47 introduced APIs requiring AWS-LC/BoringSSL. Confirm resolved version and backend.
2. Check resolved versions, provider/features and interoperability before changing production policy.

References:

- https://cryptography.io/en/50.0.2/changelog/

## Inventory exports

- [CBOM](cbom.json): 3
- [Dependency SBOM](dependency-sbom.json): 0


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
|Only part of inspection completed (detector_partial)|1 records; Gemfile|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-4fcba5b28acaa776844f113635a3cc173652e69099a79b62ce6592bb799f47c5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-72a1d6f30f073f26e3cdf76ab6236cabb8fa26d766b5a3fc2a67f55666fbec87|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-00aa99ee59c85129e552ec5f008689461b3f926077e6fbe4e28b53a8f06bdcc9|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|

## Local advisory assessment

This versioned local policy uses collected facts and unverified operator context. It does not guarantee runtime operation or migration completion. A pass applies only to the stated policy conditions.

|Item|Value|
|---|---|
|Profile|migration-review-v1|
|Overall decision|Indeterminate (indeterminate)|
|Reason codes|inspection_incomplete_or_unknown, inventory_not_exhaustive, no_complete_positive_policy_evidence|
|asset-v1-f467ff14d5b6e191402e81ef19b8356787c8158d1df3c1397124aee7fe310efa|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-48efc8829f6bc2f221265523ba14c1e52958837d810b3d33f541ed66acb4d17a|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-c66d9decaab3e3d21a46e1697dcf30a1da822246a92899697b426c080fb25a13|Not applicable (not_applicable); not_a_primitive_policy_target|
