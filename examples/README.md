# Examples for pqc-scan

This directory now contains realistic mini-product fixtures grouped by language and middleware.
Each language folder includes source modules, runtime configuration files, and dependency manifests.

## Included language fixtures

- `java`: API/auth gateway style code (`src/com/example/**`) + `pom.xml` + runtime properties
- `go`: service/auth split modules + `go.mod` + TLS/JWT service config
- `python`: application entrypoint + security module + `requirements.txt` + YAML runtime config
- `javascript`: Node service + token service module + `package.json` + OIDC/TLS config
- `typescript`: TS service modules + `package.json` + OIDC/TLS config
- `rust`: service module split + `Cargo.toml`/`Cargo.lock` + TOML runtime config
- `ruby`: app/service split + `Gemfile`/`Gemfile.lock` + YAML runtime config

## Additional fixtures

- `pqc-readiness/*`: seven-language static PQC references, negative cases and explicit TLS overrides (not executable migration recipes)

- `middleware/*`: k8s/nginx/httpd/envoy/istio/haproxy/traefik TLS policy examples
- `pki/*`: certificate and key-material samples
- `sbom/*`: CycloneDX and SPDX dependency SBOM samples

## Scan command

```bash
cargo run --bin pqc-scan -- scan ./examples --format all --out-dir ./pqc-report-examples --rules-dir ./rules
```

## What should be detected

- Tree-sitter detections across Java/Go/JavaScript/TypeScript/Python/Rust/Ruby
- Regex detections for JWT/TLS/SSH/Crypto API patterns
- Dependency detections from Maven/npm/Go/Python/Cargo/Gem manifests and SBOM files
- Middleware-specific TLS policy findings in k8s/nginx/httpd/envoy/istio/haproxy/traefik
- Key material findings from private key headers (masked in output)

## Public-certificate examples

`pki/certs/legacy.pem` and `legacy.p12` contain the public RSA-2048 test certificate
from `crates/pqc_scan_detectors/tests/fixtures/certificates/rsa2048.der`.
The PFX is a certificate-only, unencrypted DER PFX v3 with no private key and no
MAC. It demonstrates structural/certificate inspection, not integrity or trust
verification. Encrypted/private-key/unknown-bag and malformed cases are kept in
detector regression tests and must remain incomplete rather than silently pass.

The SPDX example intentionally keeps the display label `bouncycastle` alongside
its Maven purl: a display label and a validated canonical package identity are not
the same field. SBOM versions remain source claims, not installed resolutions.
