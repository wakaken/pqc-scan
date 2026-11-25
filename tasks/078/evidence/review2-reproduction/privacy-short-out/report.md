# PQC Migration Scan Report

Generated: 2026-11-24 13:22:20 +09:00 (UTC+09:00)

Findings to review (items): 1 / Informational findings (items): 0 / Files inspected: 1

|Severity|Count (all findings)|
|---|---:|
|critical|1|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[JWT_RS256](#rule-0) RS256 algorithm name referenced; active signing and effective policy are not established.|critical|quantum-vulnerable|1|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## JWT_RS256

RS256 algorithm name referenced; active signing and effective policy are not established.

Check: Confirm whether this reference belongs to active signing, verification, tests or documentation; assess JOSE formats and peer/library support before selecting PQC signatures.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[short.php:1:51](#finding-0)|RS256|Review whether the JOSE reference is used for signing, verification, tests or documentation / [Review whether the JOSE reference is used for signing, verification, tests or documentation](#action-1) / [Source](#source-0)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-2)

### short.php:1:51 — Matched rules

- `JWT_RS256` (critical): RS256 algorithm name referenced; active signing and effective policy are not established. — Confirm whether this reference belongs to active signing, verification, tests or documentation; assess JOSE formats and peer/library support before selecting PQC signatures.
- `API_JOSE_RSA_ALG` (high): RSA-family JOSE algorithm detected. — Update JOSE algorithms to PQC-ready alternatives.

<a id="source-0"></a>

### short.php:1:51

    > 1 [masked-sensitive-file-content]

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


#### Review whether the JOSE reference is used for signing, verification, tests or documentation (near-term)

JWT signatures using classical public-key algorithms become long-term high risk under quantum adversaries.

1. Restrict accepted alg values and explicitly deny weak/legacy defaults.
2. Introduce short-lived tokens during the migration phase.
3. Prepare verifier rollout before signer rollout to avoid compatibility outages.
4. Confirm JOSE algorithm identifiers, key encoding, JWT library support and all verifiers before changing token signatures; primitive support alone is insufficient.

References:

- https://www.rfc-editor.org/rfc/rfc7519
<a id="action-2"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography

## Inventory exports

- [CBOM](cbom.json): 2
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

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-8c503f931f98ced83ce756bbe109485f1c913b93bb4d47bb50100e0eb89cb6e7|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-e9093f4c9ca95dc665d75b598626cc5dec87bfd155f30ad114cca1141e1c452f|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
