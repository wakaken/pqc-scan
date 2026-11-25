# PQC Migration Scan Report

Generated: 2026-11-24 18:35:50 +09:00 (UTC+09:00)

Findings to review (items): 0 / Informational findings (items): 12 / Files inspected: 8

|Severity|Count (all findings)|
|---|---:|
|info|12|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[PQC_GO_API](#rule-0) Standard Go PQC package imported; import alone does not prove negotiated protection.|info|quantum-safe|1|1|
|[PQC_JAVA_API](#rule-1) Standard PQC algorithm requested through Java APIs; runtime success is not established.|info|quantum-safe|1|1|
|[PQC_NODE_API](#rule-2) Node-style PQC key generation call; bindings and actual execution require confirmation.|info|quantum-safe|1|1|
|[PQC_PYTHON_API](#rule-3) Python PQC key generation reference; backend support is not established.|info|quantum-safe|1|1|
|[PQC_RUBY_API](#rule-4) Ruby PQC algorithm literal referenced; key generation or signing success is not established.|info|quantum-safe|1|1|
|[PQC_RUST_API](#rule-5) Rust PQC crate symbol referenced; execution and protocol use require confirmation.|info|quantum-safe|1|1|
|[PQC_STANDARD_REFERENCE](#rule-6) Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance...|info|quantum-safe|5|5|
|[PQC_WEBCRYPTO_API](#rule-7) WebCrypto-style PQC algorithm request; this is not proof of browser support.|info|quantum-safe|1|1|

<a id="rule-0"></a>

## PQC_GO_API

Standard Go PQC package imported; import alone does not prove negotiated protection.

Check: crypto/mlkem requires Go 1.24+; crypto/mldsa requires Go 1.27+. Verify actual call sites.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[example.go:3:8](#finding-0)|"crypto/mlkem"|crypto/mlkem requires Go 1.24+; crypto/mldsa requires Go 1.27+. Verify actual call sites. / [Source](#source-0)|

### example.go:3:8 — Matched rules

- `PQC_GO_API` (info): Standard Go PQC package imported; import alone does not prove negotiated protection. — crypto/mlkem requires Go 1.24+; crypto/mldsa requires Go 1.27+. Verify actual call sites.

<a id="source-0"></a>

### example.go:3:8

      1 package example
      2 
    > 3 import "crypto/mlkem"

<a id="rule-1"></a>

## PQC_JAVA_API

Standard PQC algorithm requested through Java APIs; runtime success is not established.

Check: JDK 24+ provides ML-KEM and ML-DSA; verify provider and protocol support independently.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[Example.java:3:9](#finding-1)|Signatur***L-DSA"|JDK 24+ provides ML-KEM and ML-DSA; verify provider and protocol support independently. / [Source](#source-1)|

### Example.java:3:9 — Matched rules

- `PQC_JAVA_API` (info): Standard PQC algorithm requested through Java APIs; runtime success is not established. — JDK 24+ provides ML-KEM and ML-DSA; verify provider and protocol support independently.

<a id="source-1"></a>

### Example.java:3:9

      1 class Example {
      2     void inspect() throws Exception {
    > 3         Signature.getInstance("ML-DSA");
      4     }
      5 }

<a id="rule-2"></a>

## PQC_NODE_API

Node-style PQC key generation call; bindings and actual execution require confirmation.

Check: Node crypto requires 24.6+ for ML-DSA, 24.7+ for ML-KEM, and 24.8+ for SLH-DSA. Check runtime support.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[example.js:1:1](#finding-2)|generate***m-768'|Node crypto requires 24.6+ for ML-DSA, 24.7+ for ML-KEM, and 24.8+ for SLH-DSA. Check runtime support. / [Source](#source-2)|

### example.js:1:1 — Matched rules

- `PQC_NODE_API` (info): Node-style PQC key generation call; bindings and actual execution require confirmation. — Node crypto requires 24.6+ for ML-DSA, 24.7+ for ML-KEM, and 24.8+ for SLH-DSA. Check runtime support.

<a id="source-2"></a>

### example.js:1:1

    > 1 generateKeyPairSync('ml-kem-768');

<a id="rule-3"></a>

## PQC_PYTHON_API

Python PQC key generation reference; backend support is not established.

Check: cryptography 47 added ML-KEM and ML-DSA; 48 adds OpenSSL 3.5+ support. Confirm backend capability.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-3"></a>[example.py:1:1](#finding-3)|mlkem.ML***erate(|cryptography 47 added ML-KEM and ML-DSA; 48 adds OpenSSL 3.5+ support. Confirm backend capability. / [Source](#source-3)|

### example.py:1:1 — Matched rules

- `PQC_PYTHON_API` (info): Python PQC key generation reference; backend support is not established. — cryptography 47 added ML-KEM and ML-DSA; 48 adds OpenSSL 3.5+ support. Confirm backend capability.

<a id="source-3"></a>

### example.py:1:1

    > 1 mlkem.MLKEM768PrivateKey.generate()

<a id="rule-4"></a>

## PQC_RUBY_API

Ruby PQC algorithm literal referenced; key generation or signing success is not established.

Check: Verify openssl gem and linked OpenSSL/provider capabilities, not Ruby version alone.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-4"></a>[example.rb:1:28](#finding-4)|'ML-DSA-65'|Verify openssl gem and linked OpenSSL/provider capabilities, not Ruby version alone. / [Source](#source-4)|

### example.rb:1:28 — Matched rules

- `PQC_RUBY_API` (info): Ruby PQC algorithm literal referenced; key generation or signing success is not established. — Verify openssl gem and linked OpenSSL/provider capabilities, not Ruby version alone.

<a id="source-4"></a>

### example.rb:1:28

    > 1 OpenSSL::PKey.generate_key('ML-DSA-65')

<a id="rule-5"></a>

## PQC_RUST_API

Rust PQC crate symbol referenced; execution and protocol use require confirmation.

Check: Confirm crate version, features and provider; a RustCrypto reference alone does not enable PQC TLS.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-5"></a>[example.rs:1:5](#finding-5)|ml_kem::MlKem768|Confirm crate version, features and provider; a RustCrypto reference alone does not enable PQC TLS. / [Source](#source-5)|

### example.rs:1:5 — Matched rules

- `PQC_RUST_API` (info): Rust PQC crate symbol referenced; execution and protocol use require confirmation. — Confirm crate version, features and provider; a RustCrypto reference alone does not enable PQC TLS.

<a id="source-5"></a>

### example.rs:1:5

    > 1 use ml_kem::MlKem768;

<a id="rule-6"></a>

## PQC_STANDARD_REFERENCE

Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance are not established.

Check: A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[example.js:1:22](#finding-6)|ml-kem-768|A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security. / [Source](#source-6)|
|<a id="finding-7"></a>[example.rb:1:29](#finding-7)|ML-DSA-65|A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security. / [Source](#source-7)|
|<a id="finding-8"></a>[example.rs:1:13](#finding-8)|MlKem768|A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security. / [Source](#source-8)|
|<a id="finding-9"></a>[example.ts:1:28](#finding-9)|ML-DSA-65|A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security. / [Source](#source-9)|
|<a id="finding-10"></a>[negative.js:3:16](#finding-10)|X25519MLKEM768|A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security. / [Source](#source-10)|

### example.js:1:22 — Matched rules

- `PQC_STANDARD_REFERENCE` (info): Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance are not established. — A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security.

<a id="source-6"></a>

### example.js:1:22

    > 1 generateKeyPairSync('ml-kem-768');

### example.rb:1:29 — Matched rules

- `PQC_STANDARD_REFERENCE` (info): Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance are not established. — A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security.

<a id="source-7"></a>

### example.rb:1:29

    > 1 OpenSSL::PKey.generate_key('ML-DSA-65')

### example.rs:1:13 — Matched rules

- `PQC_STANDARD_REFERENCE` (info): Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance are not established. — A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security.

<a id="source-8"></a>

### example.rs:1:13

    > 1 use ml_kem::MlKem768;

### example.ts:1:28 — Matched rules

- `PQC_STANDARD_REFERENCE` (info): Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance are not established. — A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security.

<a id="source-9"></a>

### example.ts:1:28

    > 1 subtle.generateKey({name: 'ML-DSA-65'}, false, ['sign']);

### negative.js:3:16 — Matched rules

- `PQC_STANDARD_REFERENCE` (info): Standard PQC algorithm or hybrid group name referenced; API execution, enabled state and conformance are not established. — A standardized PQC name or hybrid group is referenced. Check implementation, parameter set, provider and actual use; this is not proof of execution, FIPS conformance or product security.

<a id="source-10"></a>

### negative.js:3:16

      1 // generateKeyPairSync('ml-kem-768');
      2 const ML_KEM_helper = 1;
    > 3 const group = 'X25519MLKEM768';

<a id="rule-7"></a>

## PQC_WEBCRYPTO_API

WebCrypto-style PQC algorithm request; this is not proof of browser support.

Check: Node WebCrypto supports ML-KEM and ML-DSA from 24.7; browser support must be checked separately.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-11"></a>[example.ts:1:8](#finding-11)|generate***SA-65'|Node WebCrypto supports ML-KEM and ML-DSA from 24.7; browser support must be checked separately. / [Source](#source-11)|

### example.ts:1:8 — Matched rules

- `PQC_WEBCRYPTO_API` (info): WebCrypto-style PQC algorithm request; this is not proof of browser support. — Node WebCrypto supports ML-KEM and ML-DSA from 24.7; browser support must be checked separately.

<a id="source-11"></a>

### example.ts:1:8

    > 1 subtle.generateKey({name: 'ML-DSA-65'}, false, ['sign']);

## Action details

Each location links only to its applicable action. Identical payloads are printed once.


## Inventory exports

- [CBOM](cbom.json): 12
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Inspection complete within declared scope (complete_within_scope)|
|Selected files|8|
|Read successfully|8|
|Read failed|0|
|Analyzed|8|
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
|observation-v1-ba3e11d164953ff931f4c996b7f71bcb1f40b71bea87e72724be10752cd2dcbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a032eb787a3271039cdff69dc954fecfc8d31ae70492343f6790423b0879a86e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-978cdabf782e9e4cf1618cee93f9cbecea111d9a5900dfdac86808d3e151d22a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9180c804d4ea1bdd20eca638a973ac5f953e1ef32437554370023eca636f2811|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-060d7b167085d58c59db3eb44283217d47836a823086fc1982db5680ee9b1973|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d1f499cf591b07ce21df3e3f139fb2d7f6370e753e9787669d44d9917da21001|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c36628e2be26c5fffa91ac952a74668505eb794d169a9bece78fa613f407613f|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-936f0c467ee96adb2bce52bf459abdfcbf7e29e7a3fc3448bd3e1700020fe986|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-153c5050e2261b737dbdc164c4a238cd3c31309c4e8395d88fd59c8276821d06|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-47a307403f801f2c2760a59859816a7df857cc8912a5a7fd4406c9c238b71e7c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-27a0ac09290f3adb0ea2c067d3fafe4a8bf50bcef472a099ed4ab1724003a0b0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b5d3ad174c8b81b87783d8edb8127cc229af917e7342d79bc886c1faa7cb9bca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
