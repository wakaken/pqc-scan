# PQC Migration Scan Report

Generated: 2026-11-24 11:18:23 +09:00 (UTC+09:00)

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
|observation-v1-68b6775a83f9f92b4226439743bace0a3f441b4891c37b11205adff41636b0ab|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-87d0f0c1e2d2581ca8e49d478b1b30b8cbb232f5235f8a32d1c4b374ddef6290|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-504d93a18022a19f401e26330f87626c11828142881bd445e6b6d23c1c2ebf5c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6370826a7cef1e668a9f1c683656d273fa33153ca06cadef37b1df6f2d806608|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-9ad858f631728c7003729f2f6c2c96300333d8ed0605faee9f48d6f358ede1be|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d5e79fa96257b9079e1b2f1e79359a97ee7a03253d140ac49644045b811e4e1|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c663991349d4871c9b1a1e5f95e35a8467340255f64fb23b4c6b91dfef93e4b8|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-2f4c2c6b653b04df1a82c2cc64c15adc9cd334770ad2e488ceec1391111e4b1c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-990aef220507f1ce82df38f908be2f4fe20e0de239aef67d02df41760e051d15|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-a5d8bea559dbb395483dc4e445f4abfdf905e2e9e1f34d76146848e5a18fac8e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-c7f3d3f6d4665b19da34584e3cd91406d37bdb0b7465e1a043b0d9e9071dcd48|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-377e13bf884154146d39395b91def7eac0e654e45da63b5ba69855a3102a1e12|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
