# pqc-scan

[日本語](README.ja.md)

`pqc-scan` is a Rust command-line tool for investigating migration to post-quantum cryptography (PQC).
It reads local project files, finds references to cryptographic algorithms and libraries, and reports the locations and suggested follow-up work.
Use it to decide what to investigate before changing JWT, TLS, PKI, SSH or other cryptographic components.

pqc-scan is local-first: scanning and report generation run entirely in your local environment. During these operations, it does not connect to external services over the internet. It does not automatically connect to external AI services or LLMs, and it does not send the scanned source code to external services.

## Contents

- [What can I scan?](#targets)
- [What does it do?](#features)
- [Run your first scan](#quick-start)
- [Scan in your browser](#web-scan)
- [Review migration](#migration-review)
- [Report preview](#report-preview)
- [Read the results](#results)
- [User guides](#guides)
- [Limits and privacy](#limits)
- [License](#license)

<a id="targets"></a>

## What can I scan?

Point the scanner at a local repository. It inspects the following artifacts within its supported patterns and formats:

| Artifact | Languages or file examples | What the scanner looks for |
|---|---|---|
| Source code | Java, Go, JavaScript, TypeScript/TSX, Python, Rust, Ruby | Cryptographic APIs, algorithm names and related usage patterns |
| Configuration files | Kubernetes/Ingress, nginx, Apache httpd, Envoy, Istio, HAProxy, Traefik | TLS/cipher/group settings and other cryptographic configuration signals |
| Public certificates | `.pem`, `.crt`, `.cer`, `.der` | Signature and subject-key algorithms, supported key sizes/curves, validity periods and public fingerprints |
| Private-key markers | RSA, generic PKCS#8 and OpenSSH private-key headers | The presence of supported headers; no decryption or private-key validation |
| Dependency manifests and locks | `Cargo.toml`, `Cargo.lock`, `package.json`, npm/pnpm locks, Python/Ruby/Java/Go dependency files | Relevant libraries, declared version constraints and supported resolution records |
| Existing SBOM files | CycloneDX JSON and SPDX JSON | Reported dependency components/packages; not proof of installation |

File selection defaults to 2 MiB per file. Use `scan --max-file-size-mib 16` or the folder picker’s size field to change it (1–64 MiB). See [file-size limits](docs/user/SCANNING.md#file-size-limit).

<a id="features"></a>

## What does it do?

- Locates cryptographic references and attaches severity, risk labels and source context.
- Provides migration guidance for the affected language or runtime.
- Writes HTML/Markdown for investigation and JSON/SARIF for automation.
- Lists cryptographic findings and software dependencies in separate inventory files.
- Optionally records richer observations, local advisory assessments and inspection diagnostics.
- The explicit `assess` command can [reassess saved evidence offline](docs/user/SAVED-EVIDENCE.md), keeping technical roles, scoped claims and migration decisions separate.
- Add purpose and usage information to scan results to organize migration checks.

A reference to a PQC algorithm is useful evidence, but it does not establish that the application uses it successfully. The [report guide](docs/user/REPORTS.md#pqc-findings) explains these findings with examples.

<a id="quick-start"></a>

## Run your first scan

Use the Rust toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml) (currently Rust 1.93.1) and install from this checkout:

```bash
cd /path/to/pqc-scan
cargo install --locked --path crates/pqc_scan_cli
```

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
2. Select one or more project folders within `--root` using the rows or checkboxes, then **Scan selected folders**. Double-click to open a folder; use Back/Forward or the path above the list to navigate. The picker stays within this root and skips symbolic links.
3. The progress dialog blocks other page actions while folders scan in selection order. Each folder keeps its own result. For multiple folders, use each folder’s card to open its scan report or start migration review when available. Scan history offers the same direct actions. A single default-mode scan opens its HTML report automatically. Review inspection completion as well as findings. Browser Back returns to the start page for another scan.
4. Stop the server with **Ctrl+C**. Reports remain in the private `pqc-scan-web-*` directory printed in the terminal. Keep `report.html` and `report.rules.html` together for offline reading.

The server listens only on `127.0.0.1` and runs the same static scanner as the CLI, without uploading source or reports. Use `--port 0` if the default port is occupied, or `--no-open` to open the printed URL yourself. With an extracted distribution, replace the command and rules paths with `/path/to/bundle/bin/pqc-scan` and `/path/to/bundle/rules/default`. See [local web scanning](docs/user/SCANNING.md#local-web-scanning) for limits and options.

<a id="migration-review"></a>

## Review migration

Add `--migration` to `serve` to move from scan results to optional purpose and
usage input, then **Prepare review items**. For multiple folders or history, choose
the named folder card: **Open scan report ↗** opens its detections and **Review migration**
opens its saved input/result. The migration report groups decisions and shows what
to do and when to proceed. Review the evaluation date, schedule and urgent
investigations first; inspect locations for the underlying evidence.

Follow the [eight illustrated steps](docs/user/SCANNING.md#web-migration-walkthrough).
See [migration input and reassessment](docs/user/SCANNING.md#optional-migration-context-and-reassessment),
[reading the migration report](docs/user/REPORTS.md#saved-assessment), and
[saved evidence and declarations](docs/user/SAVED-EVIDENCE.md). Save input JSON with
the exact inventory used to create it; **Save assessment data** saves the evaluation,
not a reusable context file.

<a id="report-preview"></a>

## Report preview

The scan HTML report includes a severity summary, rule groups, search/filters, and a file tree with finding counts. Expand findings for locations, source evidence and recommended checks; follow the rule guide for explanations. Filters and the tree can be hidden independently. Language (English/Japanese), light/dark theme and browser-local time display work offline.

![Light HTML report with inspection status, file tree and grouped findings](docs/images/v0.4/scan-light-en.jpg)

<details>
<summary>Dark theme</summary>

![Dark HTML report](docs/images/v0.4/scan-dark-en.jpg)

</details>

Actual Keycloak scan with v0.4.0, captured on 2026-11-23 with the default risk-candidate view and filters hidden. Counts describe that scan snapshot, including test fixtures; they are references requiring investigation, not confirmed vulnerability counts. The incomplete-inspection notice remains visible. [Capture source and settings](docs/images/README.md).

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

HTML and Markdown merge equivalent findings at the same position and algorithm, retaining matched rules in details. Inspection totals count files; finding totals count merged items. Machine exports such as JSON retain original rule-level records.

Tests remain included by default. Use `--exclude-tests` or the browser’s “Exclude test code” option below “Clear selection” to omit conventional test paths and filenames. See [scanning](docs/user/SCANNING.md) for matching conventions and limits. Custom exclusions can be configured with `.pqcignore`; syntax-proven Rust test-only findings can also be omitted.

<a id="guides"></a>

## User guides

| Guide | Use it when you need to… |
|---|---|
| [Scanning repositories](docs/user/SCANNING.md) | Select outputs, configure CI exit codes, inspect incomplete scans or run from source |
| [Understanding reports](docs/user/REPORTS.md) | Interpret risk/PQC findings, choose an inventory, understand CBOM/SBOM or use local policy assessment |
| [Supported inputs and inspection limits](docs/user/COVERAGE.md) | Check languages, file formats, certificate handling, rule packs, exclusions and privacy boundaries |
| [Explicit TLS/SSH diagnostics](docs/user/PROBING.md) | Inspect one authorized endpoint with optional protocol commands |
| [Installation and local packages](docs/user/DISTRIBUTION.md) | Rule assets, package verification, updates and support scope |
| [Saved evidence and declarations](docs/user/SAVED-EVIDENCE.md) | Exact-input/context binding, claim expiry and offline reassessment |

For reporting a suspected vulnerability, see [SECURITY.md](SECURITY.md).

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

<a id="license"></a>

## License

[MIT](LICENSE). Bundled dependencies have their own licenses; see the archive’s
`DEPENDENCY-LICENSES.json` and `licenses/` directory.
