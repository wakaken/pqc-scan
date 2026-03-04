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
