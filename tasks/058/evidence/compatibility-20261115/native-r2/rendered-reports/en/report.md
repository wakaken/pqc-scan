# PQC Migration Scan Report

Generated: 2026-11-15 00:00:00 UTC

Risk candidates: 4 / Inventory findings: 0 / Scanned files: 4

Cryptographic references found in source code and configuration.

|Severity|Count (all findings)|
|---|---:|
|medium|4|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[ENVOY056](#rule-0) Synthetic migration reference|medium|quantum-vulnerable|1|1|
|[JAVA_CURRENT056](#rule-1) Synthetic migration reference|medium|quantum-vulnerable|1|1|
|[JAVA_UNKNOWN056](#rule-2) Synthetic migration reference|medium|quantum-vulnerable|1|1|
|[PKI056](#rule-3) Synthetic migration reference|medium|quantum-vulnerable|1|1|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## ENVOY056

Synthetic migration reference

Check: Review actual use before migration

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[envoy.yaml:1:1](#finding-0)|synthetic reference|Prefer TLS 1.3 and review legacy key exchange / [Validate Envoy PQC signature compatibility in control/data planes](#action-0) / [Prefer TLS 1.3 and review legacy key exchange](#action-2) / [Set modern TLS policy in Envoy](#action-3) / Source unavailable|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-1)

<a id="rule-1"></a>

## JAVA_CURRENT056

Synthetic migration reference

Check: Review actual use before migration

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-1"></a>[current/Example.java:1:1](#finding-1)|synthetic reference|Use Java standard JCA names for ML-DSA / ML-KEM / [Use Java standard JCA names for ML-DSA / ML-KEM](#action-4) / Source unavailable|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-1)
- [Verify deployed runtime and protocol PQC capabilities](#action-5)

<a id="rule-2"></a>

## JAVA_UNKNOWN056

Synthetic migration reference

Check: Review actual use before migration

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-2"></a>[unknown/Example.java:1:1](#finding-2)|synthetic reference|Provide BouncyCastle PQC fallback path / [Check Java runtime baseline for standard PQC APIs](#action-6) / [Provide BouncyCastle PQC fallback path](#action-7) / Source unavailable|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-1)
- [Verify deployed runtime and protocol PQC capabilities](#action-5)

<a id="rule-3"></a>

## PKI056

Synthetic migration reference

Check: Review actual use before migration

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-3"></a>[certificate.pem:1:1](#finding-3)|synthetic reference|Review actual use before migration / Source unavailable|

Shared checks for these locations (not verified facts about each line):

- [Adopt a phased hybrid PQC migration plan](#action-1)
- [Plan certificate algorithm and trust-store migration](#action-8)

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Validate Envoy PQC signature compatibility in control/data planes (advisory)

Envoy TLS behavior depends on configured providers and peer capabilities; PQC signature readiness can vary between mesh edges, sidecars, and external clients.

1. Harden min/max TLS versions and retire static RSA key exchange where configured; ECDHE_RSA uses RSA authentication, not static RSA key exchange.
2. Review cipher_suites for TLS 1.0-1.2, not TLS 1.3; review certificate signatures and peer compatibility separately.
3. Verify PQC certificate-signature compatibility in both downstream and upstream clusters.
4. Gate production rollout with staged canary tests and explicit fallback paths.

References:

- https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/security/ssl.html
- https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/tls.proto
<a id="action-1"></a>


#### Adopt a phased hybrid PQC migration plan (near-term)

The detected primitive is vulnerable to cryptographically relevant quantum attacks. Hybrid rollout minimizes compatibility risk.

1. Inventory all call sites using this algorithm and owner teams.
2. Check for an existing classical + PQC composition; if migration is needed, evaluate compatible staged or hybrid modes before hard cut-over.
3. Define retirement dates for legacy classical algorithms.

References:

- https://www.nist.gov/itl/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
<a id="action-2"></a>


#### Prefer TLS 1.3 and review legacy key exchange (near-term)

RSA key exchange and legacy protocol versions are high-priority migration blockers for PQC readiness.

1. Prefer TLS 1.3; restrict connections to TLS 1.3 when all peers support it. Disable TLS 1.0 and 1.1.
2. Retain securely configured TLS 1.2 only where interoperability requires it, and record the compatibility reason.
3. Prefer ECDHE today and plan hybrid KEM suites for next phase.
4. Continuously test interoperability across clients and load balancers.

References:

- https://www.rfc-editor.org/rfc/rfc8446
<a id="action-3"></a>


#### Set modern TLS policy in Envoy (near-term)

Envoy control-plane config supports strict min/max protocol and cipher suite policy.

1. Prefer \`tls_minimum_protocol_version: TLSv1_3\`; use TLSv1_2 only when interoperability requires it.
2. Review \`cipher_suites\` for TLS 1.0-1.2, not TLS 1.3. Retire static RSA key exchange where configured; ECDHE_RSA uses RSA authentication, not static RSA key exchange.
3. Review certificate-signature migration and peer compatibility separately before removing RSA authentication.

References:

- https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/tls.proto
<a id="action-4"></a>


#### Use Java standard JCA names for ML-DSA / ML-KEM (near-term)

Modern JDKs expose standardized PQC algorithm names through JCA, reducing vendor lock-in.

1. Select an explicit parameter set and generate a new ML-DSA key pair; never reuse RSA key material.
2. Provision matching public keys and required JOSE/PKI encodings to all verifiers before switching signers to ML-DSA where policy allows.
3. Adopt ML-KEM for key establishment in hybrid TLS/key exchange designs.
4. Keep dual verification during migration to preserve compatibility.

References:

- https://openjdk.org/jeps/496
- https://openjdk.org/jeps/497

Before (java)

    Signature sig = Signature.getInstance("SHA256withRSA");
    sig.initSign(privateKey);
    sig.update(payload);
    byte[] signature = sig.sign();

After

    // JDK 24+: provision a new ML-DSA key pair for the selected parameter set.
    // mlDsaPrivateKey is its private key, never the previous RSA key.
    // Provision the matching public key and encoding to verifiers before rollout.
    Signature sig = Signature.getInstance("ML-DSA");
    sig.initSign(mlDsaPrivateKey);
    sig.update(payload);
    byte[] signature = sig.sign();
    // Keep legacy verification only under the explicit migration policy.

<a id="action-5"></a>


#### Verify deployed runtime and protocol PQC capabilities (planned)

Source declarations and primitive availability do not establish deployment or peer support.

1. JDK 24 adds ML-KEM and ML-DSA primitives. Check deployed JSSE hybrid TLS support separately and inspect jdk.tls.namedGroups restrictions.
2. Check resolved versions, provider/features and interoperability before changing production policy.

References:

- https://docs.oracle.com/en/java/javase/27/security/java-secure-socket-extension-jsse-reference-guide.html
<a id="action-6"></a>


#### Check Java runtime baseline for standard PQC APIs (planned)

The Java version is unknown; verify the deployed runtime before selecting a PQC API baseline.

1. Target JDK 24+ for ML-KEM and ML-DSA support planning.
2. Pilot compatibility testing in CI before production migration.

References:

- https://openjdk.org/jeps/496
- https://openjdk.org/jeps/497
<a id="action-7"></a>


#### Provide BouncyCastle PQC fallback path (near-term)

BouncyCastle can be used as a pragmatic transition option when standard runtime support is unavailable or incomplete.

1. Verify the resolved BC release and deployed JDK support before provider initialization; the example uses BC Java 1.85 and the BC provider, not BCPQC.
2. Select explicit ML-DSA / ML-KEM parameter sets for the intended purpose. Generate new algorithm-compatible keys and provision matching public keys and encodings to verifiers; never reuse RSA key material.
3. Document provider ordering and FIPS/compliance impact.

References:

- https://www.bouncycastle.org/java.html
- https://downloads.bouncycastle.org/java/docs/bcprov-jdk18on-javadoc/

Before (java)

    Security.addProvider(new BouncyCastleProvider());
    Signature sig = Signature.getInstance("SHA256withRSA", "BC");

After

    // BC Java 1.85 example: verify the resolved release/provider supports this name.
    Security.addProvider(new BouncyCastleProvider());
    // Provision a new ML-DSA-65 key pair and its public key to verifiers.
    // mlDsaPrivateKey must not be the previous RSA key.
    Signature sig = Signature.getInstance("ML-DSA-65", "BC");
    sig.initSign(mlDsaPrivateKey);

<a id="action-8"></a>


#### Plan certificate algorithm and trust-store migration (near-term)

PKI updates require coordinated CA, chain, and relying-party compatibility planning. FIPS 204/205 standardize signature primitives, not a universal hybrid certificate format.

1. Inventory certificate signature algorithms and key sizes from CBOM/report output.
2. Identify the exact certificate construction and encoding, then verify provider, CA, client and trust-store support before planning issuance.
3. Pilot only a supported construction in non-production trust stores; if format or ecosystem support is unavailable, keep issuance unverified and evaluate another supported migration path.
4. Define reissuance plan for quantum-vulnerable roots/intermediates/end-entity certs.

References:

- https://csrc.nist.gov/pubs/fips/204/final
- https://csrc.nist.gov/pubs/fips/205/final

## Inventory exports

- [CBOM](cbom.json): 0
- [Dependency SBOM](dependency-sbom.json): 0

