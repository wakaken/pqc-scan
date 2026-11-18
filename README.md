<!-- README hero -->
<p align="center">
  <img src="docs/images/pqc-scan-banner.webp" alt="pqc-scan — Post-Quantum Cryptography Migration Scanner" width="100%">
</p>

<p align="center">
  <a href="https://github.com/wakaken/pqc-scan/actions/workflows/pqc-scan.yml"><img alt="CI" src="https://github.com/wakaken/pqc-scan/actions/workflows/pqc-scan.yml/badge.svg?branch=develop"></a>
  <a href="https://github.com/wakaken/pqc-scan/releases"><img alt="Release" src="https://img.shields.io/github/v/release/wakaken/pqc-scan?display_name=tag"></a>
  <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-blue"></a>
  <img alt="Rust" src="https://img.shields.io/badge/built%20with-Rust-CE422B?logo=rust&logoColor=white">
</p>

<p align="center"><strong>Discover cryptographic references. Investigate migration risks. Prepare for the post-quantum era.</strong></p>

<p align="center">
  <a href="#quick-start">Quick start</a> ·
  <a href="#report-preview">Report preview</a> ·
  <a href="docs/user/COVERAGE.md">Coverage & limits</a> ·
  <a href="README.ja.md">日本語</a>
</p>

| Discover | Investigate | Report |
| :--- | :--- | :--- |
| Identify supported cryptographic references in local source, config, certificates, and manifests. | Review locations, migration guidance, and inspection completeness. | Explore offline HTML/Markdown or export JSON/SARIF for automation. |

> **Scope:** Static, advisory analysis. Findings are not confirmed vulnerabilities; zero findings do not guarantee safety or complete coverage. TLS/SSH probes are experimental and separately invoked.
<!-- /README hero -->

# pqc-scan

This checkout contains **v0.3.0** for static scanning on macOS Apple Silicon (`aarch64-apple-darwin`). See the [release notes](docs/releases/v0.3.0.md) for changes and compatibility. The maintainer has accepted its bounded scope and authorized publication after final build-provenance verification.

Check [GitHub Releases](https://github.com/wakaken/pqc-scan/releases) for published assets; local preparation and Actions artifacts are not published Releases. Historical [v0.2.0 notes](docs/releases/v0.2.0.md) remain available. Static scanning is the formal scope; TLS/SSH probes remain experimental and network binary publication remains on hold.

[日本語](README.ja.md)

`pqc-scan` is a Rust command-line tool for investigating migration to post-quantum cryptography (PQC).
It reads local project files, finds references to cryptographic algorithms and libraries, and reports the locations and suggested follow-up work.
Use it to decide what to investigate before changing JWT, TLS, PKI, SSH or other cryptographic components.

Scanning and report generation run entirely on your machine using local rules and parsers. No LLM or AI service is required: pqc-scan makes no outbound connections during these operations and does not send source code or scan results to external services. No AI-service API key or subscription is needed.

Downloading build/install dependencies, uploading reports through CI you configure, and opening external reference links are separate network activities. Once the tool and rules are available locally, scanning works offline.

Reports show findings, locations and suggested next steps. The inspection overview separately identifies incomplete checks and files needing follow-up.

## Contents

- [What can I scan?](#targets)
- [What does it do?](#features)
- [Run your first scan](#quick-start)
- [Scan in your browser](#web-scan)
- [Report preview](#report-preview)
- [Read the results](#results)
- [User guides](#guides)
- [Limits and privacy](#limits)
- [Repository CI](#repository-ci)
- [License](#license)

<a id="targets"></a>

## What can I scan?

Point the scanner at a local repository. It inspects the following artifacts within its supported patterns and formats:

| Artifact | Languages or file examples | What the scanner looks for |
|---|---|---|
| Source code | Java, Go, JavaScript, TypeScript/TSX, Python, Rust, Ruby | Cryptographic APIs, algorithm names and related usage patterns, with dedicated TypeScript and TSX grammars |
| Configuration files | Kubernetes/Ingress, nginx, Apache httpd, Envoy, Istio, HAProxy, Traefik | TLS/cipher/group settings and other cryptographic configuration signals |
| Public certificates | `.pem`, `.crt`, `.cer`, `.der` | Signature and subject-key algorithms, supported key sizes/curves, validity periods and public fingerprints |
| Private-key markers | RSA, generic PKCS#8 and OpenSSH private-key headers | The presence of supported headers; no decryption or private-key validation |
| Dependency manifests and locks | `Cargo.toml`, `Cargo.lock`, `package.json`, npm/pnpm locks, Python/Ruby/Java/Go dependency files | Relevant libraries, declared version constraints and supported resolution records |
| Existing SBOM files | CycloneDX JSON and SPDX JSON | Reported dependency components/packages; not proof of installation |

PKCS#12 (`.p12`/`.pfx`) supports unencrypted public-certificate extraction. Encrypted contents and private-key bags remain unsupported; no password, decryption or integrity/trust verification is performed.
See [supported inputs and inspection limits](docs/user/COVERAGE.md) for the full file list, supported versions and parser boundaries.
Live endpoints are inspected only through separately enabled, explicit [TLS/SSH probes](docs/user/PROBING.md).
They are not repository scan targets. Arbitrary programming languages are not supported.

<a id="features"></a>

## What does it do?

- Locates cryptographic references and attaches severity, risk labels and source context.
- Provides migration guidance for the affected language or runtime.
- Writes HTML/Markdown for investigation and JSON/SARIF for automation.
- Lists cryptographic findings and software dependencies in separate inventory files.
- Optionally records richer observations, local advisory assessments and inspection diagnostics.

A reference to a PQC algorithm is useful evidence, but it does not establish that the application uses it successfully. The [report guide](docs/user/REPORTS.md#pqc-findings) explains these findings with examples.

<a id="quick-start"></a>

The Rust RSA/JWT dependency rule matches the exact Cargo package names `rsa` and `jsonwebtoken`; names such as `universal-hash` are excluded. Package presence does not prove use of RSA.

## Run your first scan

Use the Rust toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml) (currently Rust 1.93.1) and install from this checkout:

```bash
cd /path/to/pqc-scan
cargo install --locked --path crates/pqc_scan_cli
```

This development checkout prepares two archive configurations: `static` (local scanning) and `network` (static scanning plus TLS/SSH diagnostics). `network` remains experimental and unpublished. Internal feature switches remain available for custom source builds.

For archive verification and installation, see the [installation guide](docs/user/DISTRIBUTION.md). Static macOS Apple Silicon binaries are distributed through [GitHub Releases](https://github.com/wakaken/pqc-scan/releases); probe bundles are not cleared for public distribution.

Keep the checkout's `rules` directory available after installation. The binary does not bundle that directory; use an absolute `--rules-dir` path when scanning another project.
If the command is not found, check that `~/.cargo/bin` is in `PATH`.
Use `pqc-scan --help` or `pqc-scan --version` to check the installed command; both exit successfully without scanning or networking.

```bash
pqc-scan scan /path/to/your-product --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

Use a new output directory outside the scanned repository when comparing scans, so old/generated reports do not become part of the input or get confused with current output.
Output paths containing symbolic links are rejected; existing regular reports remain replaceable. See [output destination safety](docs/user/SCANNING.md#output-destination-safety).
To produce Japanese HTML/Markdown, add `--lang ja`. [Running from source](docs/user/SCANNING.md#from-source) is also supported.

<a id="web-scan"></a>

## Scan in your browser

After installing the CLI above, start the local server:

```bash
pqc-scan serve --root /path/to/projects --rules-dir /path/to/pqc-scan/rules
```

1. The default browser opens the local start page. If it does not, open the complete `http://127.0.0.1:8787/<session>/` URL printed in the terminal.
2. Browse to a project within `--root` and choose **Scan this folder**. The picker stays within this root and skips symbolic links.
3. Wait for the scan to finish; the HTML report opens automatically. Review inspection completion as well as findings. Browser Back returns to the start page for another scan.
4. Stop the server with **Ctrl+C**. Reports remain in the private `pqc-scan-web-*` directory printed in the terminal. Keep `report.html` and `report.rules.html` together for offline reading.

The server listens only on `127.0.0.1` and runs the same static scanner as the CLI, without uploading source or reports. Use `--port 0` if the default port is occupied, or `--no-open` to open the printed URL yourself. With an extracted distribution, replace the command and rules paths with `/path/to/bundle/bin/pqc-scan` and `/path/to/bundle/rules/default`. See [local web scanning](docs/user/SCANNING.md#local-web-scanning) for limits and options.

<a id="report-preview"></a>

## Report preview

The v0.3.0 HTML report includes a severity summary, rule groups, search/filters, and a file tree with finding counts. Expand findings for locations, source evidence and recommended checks; follow the rule guide for explanations. Filters and the tree can be hidden independently. Language (English/Japanese), light/dark theme and browser-local time display work offline.

![Light HTML report with inspection status, file tree and grouped findings](docs/images/report-light-en.jpg)

<details>
<summary>Dark theme</summary>

![Dark HTML report](docs/images/report-dark-en.jpg)

</details>

Actual Keycloak scan, captured on 2026-11-18 with the default risk-candidate view and filters hidden. Counts describe that scan snapshot, including test fixtures; they are references requiring investigation, not confirmed vulnerability counts. The incomplete-inspection notice remains visible. [Capture source and settings](docs/images/README.md).

<a id="results"></a>

## Read the results

HTML summarizes findings, inspection scope and unfinished checks; detailed observation and asset ledgers remain in JSON/Markdown.
Advisory policy decisions are kept in explicitly requested JSON/Markdown exports.

Open `report.html` first. Review a finding's file location, source context and suggested checks before deciding on a change.
Search and filters help you work through repeated findings; `report.md` provides a portable text version.
Markdown keeps multiline source excerpts and code examples inside code blocks, including CR/LF input.
Both show inspection status. Local assessment decisions appear only in Markdown and explicitly requested JSON; HTML does not display them.

See [browser scanning](#web-scan) to select a folder and generate reports from the local web interface.

`--format all` writes six original files: `report.html`, `report.md`, `report.json`, `report.sarif`, `cbom.json` and `dependency-sbom.json`, plus the local rule guide `report.rules.html`.
`cbom.json` and `dependency-sbom.json` list cryptographic findings and software dependencies respectively. Their purpose and relationship are explained in [choosing a report](docs/user/REPORTS.md#choose-report).
Additional inventory, assessment and diagnostic files require explicit options.
Use `--cyclonedx-17` for separate CycloneDX 1.7 exports, or `diff` to compare
inventories with the same explicit repository scope. See the [report guide](docs/user/REPORTS.md).

Finding IDs use `finding-v1-<SHA-256>` with length-prefixed fields and repository-relative paths. With the same binary, rules, selected input and scan settings, identical findings keep their IDs across checkout locations. Position/evidence corrections between versions can change affected IDs and counts; see [versions and identity](docs/user/REPORTS.md#versions-and-identity).

<a id="guides"></a>

## User guides

| Guide | Use it when you need to… |
|---|---|
| [Scanning repositories](docs/user/SCANNING.md) | Select outputs, configure CI exit codes, inspect incomplete scans or run from source |
| [Understanding reports](docs/user/REPORTS.md) | Interpret risk/PQC findings, choose an inventory, understand CBOM/SBOM or use local policy assessment |
| [Supported inputs and inspection limits](docs/user/COVERAGE.md) | Check languages, file formats, certificate handling, rule packs, exclusions and privacy boundaries |
| [Explicit TLS/SSH diagnostics](docs/user/PROBING.md) | Inspect one authorized endpoint with optional protocol commands |
| [Installation and local packages](docs/user/DISTRIBUTION.md) | Rule assets, package verification, updates and support scope |

For reporting a suspected vulnerability, see [SECURITY.md](SECURITY.md).

Selected built-in source rules follow imports, aliases and bounded local bindings. Node checks also follow literal `for...of` algorithm lists and locally generated RSA keys into signing/verification. See [source analysis and its limits](docs/user/COVERAGE.md#source-facts); these observations do not prove execution or enabled PQC support.

For the current candidate's source versions, measurements and remaining limits, start with the [static candidate evaluation](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/releases/static-candidate-20261117/AUDIT-FIXES.md) and [individual rule/branch assessments](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/releases/static-candidate-20261117/QUALITY-COMPLETION.md). These are bounded evaluations, not a guarantee of accuracy for every rule or product. The [2026-11-11 PQS-QAM assessment](tasks/041/QUALITY-ASSESSMENT.md) is historical; it is not the current candidate's quality verdict. The offline benchmark produces separate [JSON/Markdown quality reports](tests/quality/README.md).

Each guide has a Japanese version linked at the top.

<a id="limits"></a>

## Limits and privacy

This is static, advisory analysis. False positives and missed references are possible; a finding does not prove runtime use, and zero findings does not prove complete inspection or safety.
Repository scans do not contact endpoints, validate certificate trust chains, install dependencies or certify PQC migration completion.
Optional experimental `probe ssh-kex` (`probe-ssh-kex` feature) verifies hybrid ML-KEM768+X25519 key exchange against an explicitly supplied trusted Ed25519 public host key, then disconnects without user authentication/login. It writes separate `probe.ssh-kex.v1.json` and bilingual HTML/Markdown; existing outputs remain unchanged. See the [probe guide](docs/user/PROBING.md).

Optional `probe` commands contact only the explicitly selected endpoint; TLS uses
ordinary certificate/name validation, while `probe ssh` collects unauthenticated offers.

Recognized private-key/credential syntax is masked, with deliberately conservative previews. This is not universal secret detection: unknown/obfuscated content and secrets placed in paths or identifiers can remain outside the policy.
Review the [privacy and scan-scope limits](docs/user/COVERAGE.md#privacy) before sharing reports.

Only regular files are read, within the configured file-size limit. Git-ignore rules,
excluded directories and known binary formats limit the selected scope. Incomplete
parsing retains available findings; it does not make the rest of a file disappear.
Use `--diagnostics` to see the reasons, and `--require-complete` if incomplete
inspection should fail CI. See [inspection status](docs/user/SCANNING.md#diagnostics)
and [supported inputs and limits](docs/user/COVERAGE.md).

<a id="repository-ci"></a>

## Repository CI

The `pqc-scan` workflow runs automatically for pull requests and pushes to
`develop`. Working-branch and tag pushes do not start it; updating an open PR
still triggers PR validation.

| Event | Jobs |
|---|---|
| Pull request | `quality` |
| Push to `develop` | `baseline`, `scan` |
| Manual execution | `quality`, `baseline`, `scan` |

On `develop`, `baseline` and `scan` run independently of `quality`; PR validation
provides the pre-merge quality check. Manual execution remains available for
running all three enabled jobs together.

The CI self-scan reads a separate product input: `crates/*/src/**`, Cargo manifests
and the root lockfile, build scripts when present, and `rules/default/*.yml`.
Standalone tests, examples, task evidence, CI scripts and documentation are outside
this scan. Source bytes and repository-relative locations are preserved; colocated
Rust unit tests remain in their source files. This scope applies to repository CI.

The summary separates results requiring review from exact reviewed references.
The `pqc-self-scan-evidence` artifact contains the input inventory/hashes, scoped
expectations, all results for that scope, upload results, diagnostics and summary.
New or changed results still appear for review. These counts describe detected
references, not confirmed vulnerabilities. See [the product scan scope](docs/quality/self-scan/CI-SELF-SCAN.md#product-scope-20261116).

The existing `quality` and `baseline` jobs also aggregate all six quality axes
offline from recorded evidence. Missing facts and stale measurements remain
visible; aggregation does not run the high-cost studies or grant release acceptance.
Integrated reports are uploaded even after a failed step, with 30-day retention.
See the [measurement and retention plan](docs/quality/CI-MEASUREMENT-PLAN.md).

It skips eligible pushes and pull requests whose changed paths all
match its documentation list: the root README files, `AGENTS.md`, `SECURITY.md`,
Markdown under `docs/` and `prompts/`, and `tasks/README.md`.
Other changes in eligible events, including mixed documentation/code changes,
rules, workflow files and JSON data under `docs/`, still run CI. The exact list is maintained in
[the workflow](.github/workflows/pqc-scan.yml). Run it manually from GitHub Actions
when validation is needed for an otherwise skipped change.

The `protocols` job is disabled for every trigger, including manual runs. Its
`if: ${{ false }}` condition remains in place. The generic isolated suite's
read-only-target write failure has been corrected and its tests, Clippy, previews
and SSH KEX completed in the [candidate validation](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/releases/static-candidate-20261117/AUDIT-FIXES.md). This is separate evidence from the dedicated protocol049 path and does not resume the hosted job. Re-enabling it requires a maintainer decision and validation of its current inputs, environment and resource limits; the three enabled jobs do not establish a protocol pass.

GitHub limits the number of changed files considered
by path filtering; for large changes, manually run CI if it was omitted.

Before making these checks required for merging, replace workflow-level path
filtering with job-level gating so skipped documentation changes do not leave
required checks pending.

<a id="license"></a>

## License

[MIT](LICENSE). Bundled dependencies have their own licenses; see the archive’s
`DEPENDENCY-LICENSES.json` and `licenses/` directory.
