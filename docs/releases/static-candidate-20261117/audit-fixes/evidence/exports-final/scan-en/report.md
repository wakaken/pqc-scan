# PQC Migration Scan Report

Generated: 2026-11-17 06:26:07.725799 UTC

Risk candidates: 12 / Inventory findings: 2 / Scanned files: 5

Cryptographic references found in source code and configuration.

|Severity|Count (all findings)|
|---|---:|
|critical|3|
|high|2|
|info|2|
|medium|7|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[CERT_RSA_2048](#rule-0) Certificate subject public key is 2048-bit RSA.|critical|quantum-vulnerable|1|1|
|[CERT_RSA_SIGNATURE](#rule-1) Certificate signed with an RSA-based algorithm detected.|critical|quantum-vulnerable|1|1|
|[PRIVATE_KEY_RSA_HEADER](#rule-2) RSA private key header detected.|critical|non-quantum-risk|1|1|
|[TS_RSA_KEY_SIZE_LITERAL](#rule-3) Classical public-key size literals detected in AST nodes.|high|quantum-vulnerable|2|1|
|[API_PY_RSA_MODULE](#rule-4) Python RSA API usage detected.|medium|quantum-vulnerable|1|1|
|[DEP_OPENSSL](#rule-5) OpenSSL dependency detected.|medium|quantum-uncertain|2|2|
|[DEP_RUST_OPENSSL](#rule-6) Rust TLS/crypto dependency detected.|medium|quantum-uncertain|2|2|
|[TS_PY_RSA_USAGE](#rule-7) RSA-related usage detected from Python AST.|medium|quantum-vulnerable|2|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[DEP_PQC_CAPABLE_LIBRARY](#rule-8) Library with version-dependent PQC capabilities inventoried; capability is not an enabled-state find...|info|quantum-uncertain|2|2|

<a id="rule-0"></a>

## CERT_RSA_2048

Certificate subject public key is 2048-bit RSA.

Check: Plan supported PQC certificate signatures with issuer and peer compatibility checks; larger classical keys alone do not remove quantum exposure.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[public.der:1:1](#finding-0)|RSA 2048-bit|Plan supported PQC certificate signatures with issuer and peer compatibility checks; larger classical keys alone do not remove quantum exposure. / Source unavailable|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Plan certificate algorithm and trust-store migration](#action-1)

<a id="rule-1"></a>

## CERT_RSA_SIGNATURE

Certificate signed with an RSA-based algorithm detected.

Check: Verify issuer, verifier and key-format support before evaluating standardized PQC certificate signatures.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[public.der:1:1](#finding-1)|sha256WithRSAEncryption|Verify issuer, verifier and key-format support before evaluating standardized PQC certificate signatures. / Source unavailable|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Plan certificate algorithm and trust-store migration](#action-1)

<a id="rule-2"></a>

## PRIVATE_KEY_RSA_HEADER

RSA private key header detected.

Check: Confirm whether this is a header example, test key or complete live key. If a live key is exposed, revoke or rotate it and review key handling.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[marker.txt:1:1](#finding-2)|\[masked-private-key\]|Review private-key markers and rotate exposed live credentials / [Review private-key markers and rotate exposed live credentials](#action-2) / [Source](#source-2)|

<a id="source-2"></a>

### marker.txt:1:1

    > 1 [masked-private-key-material]

<a id="rule-3"></a>

## TS_RSA_KEY_SIZE_LITERAL

Classical public-key size literals detected in AST nodes.

Check: Shift from key-length based planning to PQC signatures/key exchange.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-3"></a>[app.py:2:3](#finding-3)|RSA.generate(2048|Shift from key-length based planning to PQC signatures/key exchange. / [Source](#source-3)|
|<a id="finding-4"></a>[app.py:3:3](#finding-4)|RSA.generate(2048|Shift from key-length based planning to PQC signatures/key exchange. / [Source](#source-4)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Plan certificate algorithm and trust-store migration](#action-1)
- [Verify deployed runtime and protocol PQC capabilities](#action-3)

<a id="source-3"></a>

### app.py:2:3

      1 from Crypto.PublicKey import RSA
    > 2 a=RSA.generate(2048)
      3 b=RSA.generate(2048)

<a id="source-4"></a>

### app.py:3:3

      1 from Crypto.PublicKey import RSA
      2 a=RSA.generate(2048)
    > 3 b=RSA.generate(2048)

<a id="rule-4"></a>

## API_PY_RSA_MODULE

Python RSA API usage detected.

Check: Evaluate PQC-capable libraries for Python crypto implementations.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-5"></a>[app.py:1:1](#finding-5)|from Cry***rt RSA|Evaluate PQC-capable libraries for Python crypto implementations. / [Source](#source-5)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-4)
- [Verify deployed runtime and protocol PQC capabilities](#action-3)

<a id="source-5"></a>

### app.py:1:1

    > 1 from Crypto.PublicKey import RSA
      2 a=RSA.generate(2048)
      3 b=RSA.generate(2048)

<a id="rule-5"></a>

## DEP_OPENSSL

OpenSSL dependency detected.

Check: Review the OpenSSL PQC roadmap.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[Cargo.lock:3:1](#finding-6)|openssl@0.10.75|Review the OpenSSL PQC roadmap. / [Source](#source-6)|
|<a id="finding-7"></a>[Cargo.toml:2:1](#finding-7)|openssl@0.10|Review the OpenSSL PQC roadmap. / [Source](#source-7)|

Shared checks for these locations (not verified facts about each line):

- [Track PQC readiness of dependent libraries](#action-5)

<a id="source-6"></a>

### Cargo.lock:3:1

    > 3 [masked-sensitive-file-content]

<a id="source-7"></a>

### Cargo.toml:2:1

    > 2 [masked-sensitive-file-content]

<a id="rule-6"></a>

## DEP_RUST_OPENSSL

Rust TLS/crypto dependency detected.

Check: Track PQC migration options for Rust openssl/rustls crypto stacks.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-8"></a>[Cargo.lock:3:1](#finding-8)|openssl@0.10.75|Track PQC migration options for Rust openssl/rustls crypto stacks. / [Source](#source-8)|
|<a id="finding-9"></a>[Cargo.toml:2:1](#finding-9)|openssl@0.10|Track PQC migration options for Rust openssl/rustls crypto stacks. / [Source](#source-9)|

Shared checks for these locations (not verified facts about each line):

- [Track PQC readiness of dependent libraries](#action-5)

<a id="source-8"></a>

### Cargo.lock:3:1

    > 3 [masked-sensitive-file-content]

<a id="source-9"></a>

### Cargo.toml:2:1

    > 2 [masked-sensitive-file-content]

<a id="rule-7"></a>

## TS_PY_RSA_USAGE

RSA-related usage detected from Python AST.

Check: Inventory Python RSA API usage as PQC migration targets.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-10"></a>[app.py:2:3](#finding-10)|RSA.generate|Inventory Python RSA API usage as PQC migration targets. / [Source](#source-10)|
|<a id="finding-11"></a>[app.py:3:3](#finding-11)|RSA.generate|Inventory Python RSA API usage as PQC migration targets. / [Source](#source-11)|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-0)
- [Introduce crypto abstraction boundary for algorithm agility](#action-4)
- [Verify deployed runtime and protocol PQC capabilities](#action-3)

<a id="source-10"></a>

### app.py:2:3

      1 from Crypto.PublicKey import RSA
    > 2 a=RSA.generate(2048)
      3 b=RSA.generate(2048)

<a id="source-11"></a>

### app.py:3:3

      1 from Crypto.PublicKey import RSA
      2 a=RSA.generate(2048)
    > 3 b=RSA.generate(2048)

<a id="rule-8"></a>

## DEP_PQC_CAPABLE_LIBRARY

Library with version-dependent PQC capabilities inventoried; capability is not an enabled-state finding.

Check: Inspect resolved version, features and backend. rustls 0.23.27 prefers hybrid TLS with aws-lc-rs; cryptography 48+ supports OpenSSL 3.5+ PQC. Presence alone proves no protection.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-12"></a>[Cargo.lock:3:1](#finding-12)|openssl@0.10.75|Inspect resolved version, features and backend. rustls 0.23.27 prefers hybrid TLS with aws-lc-rs; cryptography 48+ supports OpenSSL 3.5+ PQC. Presence alone proves no protection. / [Source](#source-12)|
|<a id="finding-13"></a>[Cargo.toml:2:1](#finding-13)|openssl@0.10|Inspect resolved version, features and backend. rustls 0.23.27 prefers hybrid TLS with aws-lc-rs; cryptography 48+ supports OpenSSL 3.5+ PQC. Presence alone proves no protection. / [Source](#source-13)|

Shared checks for these locations (not verified facts about each line):

- [Track PQC readiness of dependent libraries](#action-5)

<a id="source-12"></a>

### Cargo.lock:3:1

    > 3 [masked-sensitive-file-content]

<a id="source-13"></a>

### Cargo.toml:2:1

    > 2 [masked-sensitive-file-content]

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


#### Plan certificate algorithm and trust-store migration (near-term)

PKI updates require coordinated CA, chain, and relying-party compatibility planning. FIPS 204/205 standardize signature primitives, not a universal hybrid certificate format.

1. Inventory certificate signature algorithms and key sizes from CBOM/report output.
2. Identify the exact certificate construction and encoding, then verify provider, CA, client and trust-store support before planning issuance.
3. Pilot only a supported construction in non-production trust stores; if format or ecosystem support is unavailable, keep issuance unverified and evaluate another supported migration path.
4. Define reissuance plan for quantum-vulnerable roots/intermediates/end-entity certs.

References:

- https://csrc.nist.gov/pubs/fips/204/final
- https://csrc.nist.gov/pubs/fips/205/final
<a id="action-2"></a>


#### Review private-key markers and rotate exposed live credentials (immediate)

A private-key marker was found. Confirm whether complete key material is present and whether it belongs to a live exposed credential; the marker alone does not establish an incident.

1. Check whether the signal is a header reference, a synthetic test key, or complete live key material.
2. If a live credential is exposed, revoke or replace it and invalidate affected sessions/tokens as appropriate.
3. Move keys to a secret manager or HSM and enforce commit scanning in CI.

References:

- https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html
<a id="action-3"></a>


#### Verify deployed runtime and protocol PQC capabilities (planned)

Source declarations and primitive availability do not establish deployment or peer support.

1. cryptography 48 adds OpenSSL 3.5+ support for ML-KEM/ML-DSA; 47 introduced APIs requiring AWS-LC/BoringSSL. Confirm resolved version and backend.
2. Check resolved versions, provider/features and interoperability before changing production policy.

References:

- https://cryptography.io/en/50.0.2/changelog/
<a id="action-4"></a>


#### Introduce crypto abstraction boundary for algorithm agility (near-term)

Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.

1. Wrap signing, verification, keygen, and key exchange behind stable interfaces.
2. Add policy controls to select classical, hybrid, or PQC paths at runtime.

References:

- https://www.nist.gov/itl/post-quantum-cryptography
<a id="action-5"></a>


#### Track PQC readiness of dependent libraries (planned)

Dependency-level crypto libraries strongly influence migration options and operational timelines.

1. Record current versions in dependency-sbom.json and define upgrade targets.
2. Subscribe to release notes for PQC feature availability and API changes.
3. Run integration tests for PQC or hybrid modes before promoting to production.

References:

- https://cyclonedx.org/
- https://spdx.dev/

## Inventory exports

- [CBOM](cbom.json): 14
- [Dependency SBOM](dependency-sbom.json): 2


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|5|
|Read successfully|5|
|Read failed|0|
|Analyzed|5|
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
|observation-v1-4b8867a0c4a549cc70903ab05276e42df5395154c50ecc6a300db34ad8cc0c44|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-429df60f1db5a228893e41f5f465a27a5f889bdb699c421e8860ade9a66b2026|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-53acfbcd181b57ff430e87ff10fc21921b9cc641fc2ae4684dea59f8355d6124|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-fa2247b2cd600595e34976255b7f0bf9bb8db0a34444116de889e2b53cab3898|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c8011a0ee5cfb3faad5dcd9365add8f950fa16ef92f558d111ad75da8feb8aa9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-845dc6df8e67a56f9a2585c3269f7b0cc8f6572d93f7d37cb6e8aeb31ea6dee1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-67a247b178fba5ec0f87959cf93f56f6d124d65caf0e0a1fc2cd984947e649fd|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9e6f634be2c49fece530000265f09edc842e4ab164f9f492abe37654914b14ff|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-da7c8c6ffbbe03ff6f86cf830c50f6b6bd33e5bc0a883dd029f98211352771d3|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-fe210ed74530178e446782f469950fd573c040105bc87a9c39b0bfa9a78d9e3f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4bc526f2bb60e6d8b7404e5e7ba4d6240c98497661e0a640644c885a2ab766df|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2739d30a642c9b53ef9f6d75df86eae282fe55ad73431df95dec4476f502625c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-3cd9336d4955aca315eb421b13258192009afe499167f3e7a16f8301d0108d45|Assurance: Lexical match (lexical); Usage: Referenced (referenced)|
|observation-v1-c72ad020ad6accdc6a3048a4f018a0b677b5b296f90d9f92c2a7956824389b72|Assurance: Lexical match (lexical); Usage: Unknown (unknown)|
|observation-v1-caccabb262b3fcb6aa846dee2773e9d8797a66bafb488c528d82f00da0becad3|Assurance: Parsed public data (parsed); Usage: Unknown (unknown)|

## Local advisory assessment

This versioned local policy uses collected facts and unverified operator context. It does not guarantee runtime operation or migration completion. A pass applies only to the stated policy conditions.

|Item|Value|
|---|---|
|Profile|migration-review-v1|
|Overall decision|Local policy concern (fail)|
|Reason codes|inventory_not_exhaustive, applicable_asset_violation|
|asset-v1-dae48197d26ca7335bd9cfb1cb9f7777aaeedd585645f6cf68af6b5450eff8d8|Local policy concern (fail); observed_primitive_quantum_vulnerable|
|asset-v1-21bb03bffb268e7b5726fdb5ff7bfc95036a64a99256d51d55fc7af879da6cb5|Local policy concern (fail); observed_primitive_quantum_vulnerable|
|asset-v1-7bbf1f29e19625b0e55aab993b0f879f9b6e4412b02839f7727cae2aadf29cd3|Local policy concern (fail); observed_primitive_quantum_vulnerable|
|asset-v1-8f840a694ce3a2aabc09a6c926ba150227da1967ad9ea770b8d1da6e8d9e7fa1|Local policy concern (fail); observed_primitive_quantum_vulnerable|
|asset-v1-c107b6bb88260f51b5fec82879d5da9be71caec5d9d2e9a68cd7519c6566478b|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-dc3ec79c66d3d45929703676e81e944c1bcd6c569260c6d1c2a285c4fccb670e|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-70040dd83548ebe9db3bc9797c050f9e331241f6289c246b0d2a0e3c0c3e80a0|Local policy concern (fail); observed_primitive_quantum_vulnerable|
|asset-v1-222f95df2eda5cd7e961041e14962a3da065aec445e5c7fa345f1414cb5cf991|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-2fa9b19ac26dbbe2be3d9f32a3b565671c0b3e76e36915e243de7e353aeb9139|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-36078846a597a7122ccc96c1a60441ae58d885416447099aa6b764f5a5e5a57e|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-1c535d35d99a0dece1474f4262ecc5db18f3a107995760b4d1af82b496807333|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-b7724157d8d35b191705db319379e7cd94448f242341541771013fe1dcf1f8e5|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-c7b120b4e0196a11bc5211143dc260b587d39d0efc02a86d1f1588ea4e05b0fd|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-3aec3730c162169b5c20bb6abf01abc7b1cee005f4580e7671cdd9612aa492e8|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-516b4df7ddf7bfccaf2493f00e08f9761e141a44c51ad6a72062f10344ad0ee1|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-3cebd944749f559480f6688ffc1e60e0c4019c1eaf04cf5bfc2ec58adfe16c17|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-29bef60a61c288d6a4944ec341d20a20f50ed1781926d91f8d05a238695872fc|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-d3eb48213bf72af36df0d89ddcf3cf5d243d8ed2c1e9bfec830a3c87a7754c05|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-07ae183c86ce79c0ef179c061d46c42ecc624806d8efbea7463e194a191b68dd|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-64bdb0f44a5ff674fb62b3aea9372ae7451a93ba6b3c1a167900db31b85bf6b5|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-f2152403a630c9234c9261e4e962e51d73195d02807cd4e58c725c8699952147|Not applicable (not_applicable); not_a_primitive_policy_target|
|asset-v1-3cd221be6cf0a113cc951ff750b100ec7c3ad83d153e0e3c253b14ae033de438|Not applicable (not_applicable); not_a_primitive_policy_target|
