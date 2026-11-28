# Supported inputs and inspection limits

[日本語](COVERAGE.ja.md) · [README](../../README.md)

This guide describes supported inputs and inspection limits. Use a matching
scanner and rule bundle. See the [input overview](../../README.md#targets) and
[scanning guide](SCANNING.md).

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

There are 139 built-in rules. Use the matching bundled `rules/default` directory.

<a id="source-limits"></a>
<a id="source-facts"></a>

## Source-code inspection limits

Checks cover cryptographic API and algorithm references in Java, Go, JavaScript,
TypeScript/TSX, Python, Rust, Ruby, PHP, C# and C/C++. Code is not executed. Supported usage
patterns are limited, so not every cryptographic operation can be found.
Finding the name of a cryptographic algorithm in code does not confirm that the
operation is actually used or that PQC migration is complete.

<a id="privacy"></a>

<a id="privacy-and-excluded-files"></a>

## Privacy and inspection exclusions

<a id="before-sharing-a-report"></a>

### What to check before sharing a report

The tool masks recognized private keys and credentials in reports, but does not
detect every secret. Reports can retain source excerpts containing the matched
location, file paths and public certificate information. Inspect the actual report
before sharing it with your organization or a third party.

### Conditions that exclude input

| Condition | Default or control |
|---|---|
| Git ignore settings | Respected when selecting files |
| Directories and formats | Exclude `.git`, `node_modules`, `target`, `dist`, `docs`, Markdown and known image, archive and binary formats |
| File size | Up to 2 MiB by default; CLI and folder-picker settings accept 1–64 MiB |
| Test code | Included by default; omit with `--exclude-tests` or **Exclude test code** |
| Custom paths | List them in `.pqcignore` |
| Symbolic links | Link targets are not read |

See [file-size limits](SCANNING.md#file-size-limit) and [file selection](SCANNING.md)
for settings. Remember that zero findings do not prove safety: some input may be
excluded or unsupported.
