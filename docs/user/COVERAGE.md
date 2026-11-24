# Supported inputs and inspection limits

[日本語](COVERAGE.ja.md) · [README](../../README.md)

This guide explains supported v0.4.0 inputs and how to interpret their results.
See the [input overview](../../README.md#targets) and [scanning guide](SCANNING.md).

## Contents

- [Dependency files](#dependencies)
- [SBOM input](#sbom)
- [Certificates and key markers](#certificates)
- [Configuration files and rules](#configuration)
- [Source code](#source-limits)
- [Privacy and excluded files](#privacy)

<a id="dependencies"></a>

## Dependency files

The following files provide relevant library and version information:

- Rust: `Cargo.toml`, `Cargo.lock`
- Ruby: `Gemfile`, `Gemfile.lock`, `gems.rb`, `gems.locked`, `*.gemspec`
- Java: `pom.xml`, `build.gradle`, `build.gradle.kts`, `gradle.lockfile`
- Go: `go.mod`
- JavaScript/TypeScript: `package.json`, `package-lock.json`, `pnpm-lock.yaml`
- Python: `requirements.txt`, `Pipfile.lock`, `poetry.lock`

Declared versions are distinguished from versions recorded in lockfiles.
Dynamically determined values are not treated as confirmed values. Finding a
library does not establish actual use of its cryptographic algorithms.

<a id="sbom"></a>

## SBOM input

Supported inputs are CycloneDX JSON (1.4–1.7, `*.cdx.json`/`*bom*.json`) and
SPDX JSON (2.2–2.3, `*.spdx.json`/`*sbom*.json`). These describe recorded
components and packages; they do not verify installation state.

<a id="certificates"></a>

## Certificates and key markers

Public certificates in `.pem`, `.crt`, `.cer` and `.der` provide signature and
public-key algorithms, supported key sizes/curves and validity periods.
Certificate signatures and public keys are shown separately. Expiry checks use
the scan time; they do not establish trust or use by a live endpoint.

For `.p12`/`.pfx`, readable public certificates are inspected. Password entry and
decryption are not supported. Check incomplete-inspection notices for unreadable contents.

RSA, PKCS#8 and OpenSSH private-key headers are also detected. This does not
decrypt private keys or verify their validity.

<a id="configuration"></a>

## Configuration files and rules

Supported configurations include Kubernetes/Ingress, nginx, Apache httpd, Envoy,
Istio, HAProxy and Traefik. Checks cover TLS, cipher suites and key-exchange groups.
These are file settings, not confirmed algorithms in a live connection.

There are 125 built-in rules. Use the matching bundled `rules/default` directory.

<a id="source-limits"></a>
<a id="source-facts"></a>

## Source-code inspection limits

Checks cover cryptographic API and algorithm references in Java, Go, JavaScript,
TypeScript/TSX, Python, Rust and Ruby. Code is not executed. Supported usage
patterns are limited, so not every cryptographic operation can be found.
An algorithm reference does not establish runtime use or completed PQC migration.

Syntax errors, unsupported constructs and processing limits can leave inspection
incomplete while retaining available findings. Review the stated reasons and next
checks in [inspection status](SCANNING.md#diagnostics).

<a id="privacy"></a>

## Privacy and excluded files

Recognized private keys and credentials are masked in reports, but this is not a
universal secret scanner. Source snippets, paths and public certificate details
may remain. Review reports before sharing them.

Git ignore settings are respected. Default exclusions include `.git`,
`node_modules`, `target`, `dist`, `docs`, Markdown and known image, archive and
binary formats. The default file-size limit is 2 MiB; CLI and folder-picker
settings accept 1–64 MiB. See [file-size limits](SCANNING.md#file-size-limit) and
[file selection](SCANNING.md).

Tests are included by default. Use `--exclude-tests` or **Exclude test code** to
omit them, and `.pqcignore` for custom paths. Symbolic-link targets are not read.
Excluded and unsupported inputs mean that zero findings do not prove safety.
