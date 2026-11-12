# pqc-scan

Release target: **v0.2.0**. Static scanning is the formal scope; TLS/SSH probes remain experimental. See the [v0.2 roadmap](docs/roadmap/OSS-ROADMAP-v0.2.md) and [release notes](docs/releases/v0.2.0.md).

[日本語](README.ja.md)

`pqc-scan` is a Rust command-line tool for investigating migration to post-quantum cryptography (PQC).
It reads local project files, finds references to cryptographic algorithms and libraries, and reports the locations and suggested follow-up work.
Use it to decide what to investigate before changing JWT, TLS, PKI, SSH or other cryptographic components.

Scanning and report generation run entirely on your machine using local rules and parsers. No LLM or AI service is required: pqc-scan makes no outbound connections during these operations and does not send source code or scan results to external services. No AI-service API key or subscription is needed.

Downloading build/install dependencies, uploading reports through CI you configure, and opening external reference links are separate network activities. Once the tool and rules are available locally, scanning works offline.

## Contents

- [What can I scan?](#targets)
- [What does it do?](#features)
- [Run your first scan](#quick-start)
- [Read the results](#results)
- [User guides](#guides)
- [Limits and privacy](#limits)
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

## Run your first scan

Use the Rust toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml) (currently Rust 1.93.1) and install from this checkout:

```bash
cd /path/to/pqc-scan
cargo install --locked --path crates/pqc_scan_cli
```

Keep the checkout's `rules` directory available after installation. The binary does not bundle that directory; use an absolute `--rules-dir` path when scanning another project.
If the command is not found, check that `~/.cargo/bin` is in `PATH`.
Use `pqc-scan --help` or `pqc-scan --version` to check the installed command; both exit successfully without scanning or networking.

```bash
pqc-scan scan /path/to/your-product --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

Use a new output directory outside the scanned repository when comparing scans, so old/generated reports do not become part of the input or get confused with current output.
To produce Japanese HTML/Markdown, add `--lang ja`. [Running from source](docs/user/SCANNING.md#from-source) is also supported.

<a id="results"></a>

## Read the results

HTML starts with inspection scope and unfinished checks; detailed observation and asset ledgers remain in JSON/Markdown.
Advisory policy decisions are kept in explicitly requested JSON/Markdown exports.

Open `report.html` first. Review a finding's file location, source context and suggested checks before deciding on a change.
Search and filters help you work through repeated findings; `report.md` provides a portable text version.
Both show inspection status. Local assessment decisions appear only in Markdown and explicitly requested JSON; HTML does not display them.

`--format all` writes six files: `report.html`, `report.md`, `report.json`, `report.sarif`, `cbom.json` and `dependency-sbom.json`.
The last two list cryptographic findings and software dependencies respectively. Their purpose and relationship are explained in [choosing a report](docs/user/REPORTS.md#choose-report).
Additional inventory, assessment and diagnostic files require explicit options.
Use `--cyclonedx-17` for separate CycloneDX 1.7 exports, or `diff` to compare
inventories with the same explicit repository scope. See the [report guide](docs/user/REPORTS.md).

Finding IDs use `finding-v1-<SHA-256>` with length-prefixed fields and repository-relative paths. Identical findings keep the same ID across checkout locations.

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

<a id="license"></a>

## License

MIT

The scanner reads regular files only; in-tree symbolic links and special files are
reported as read failures. Reads, including source context and runtime hints, are
bounded by the configured file-size limit. CLI scope notes explain that ignore
and traversal policies may leave files unenumerated. This does not change exit
codes or the scope-based meaning of `--require-complete`.

Distribution preparation now supports three macOS arm64 variants: `static`,
`combined-probes` (TLS and SSH offers), and `all-probes` (also trusted SSH KEX).
All probe modes remain experimental. See [distribution preparation](docs/user/DISTRIBUTION.md).

### Inspection scope and binary exclusions

Known media, archives, compiled artifacts and non-text binary data are excluded before inspection by default. Certificates, private-key inputs and key/trust stores remain in scope. Use `--include-non-scannable` to explicitly include the normally excluded formats for custom inspection. HTML/Markdown summarize diagnostic counts with at most three example paths per reason; `--diagnostics` retains the full JSON records.

Source inspection matches full candidate text, with finite repository-sized work budgets. UTF-16/UTF-32 source text is decoded strictly (BOMs, or a conservative BOM-less ASCII-lane signature); previews and masking use the same decoded text. See [coverage](docs/user/COVERAGE.md#source-limits) for syntax and budget details.

### Source inspection coverage

Supported Python imports and `as` aliases, Ruby OpenSSL RSA/EC calls, EdDSA references, and Node RSA key generation produce investigation candidates. This is bounded static inspection: computed names, ambiguous bindings and runtime configuration may remain unknown. HS256/AES references and a library dependency alone do not establish RSA use. Go import indicators require Go import syntax. Dependency declarations, catalog/workspace references and Git/path sources are inventoried separately from resolved versions; no package manager or Ruby DSL is executed. Known non-certificate key/CSR/CRL objects are distinguished from certificate failures, while encrypted PKCS#12 remains incomplete. Supported syntax compatibility preserves original evidence and positions; incomplete or malformed inspection is still reported. See [coverage](docs/user/COVERAGE.md) and the [rule reference](docs/rules-reference.md).

Dockerfile variant names are corroborated by their first instruction before overriding a source extension; lexical cryptography inspection continues. Bundler lock records retain platform-qualified reported versions while resolving corroborated registry versions separately. TypeScript import types, empty type-alias parameter lists and quoted JSX URL attributes retain original evidence positions.

### Local declaration context

Repository scans can read walker-selected local Maven parents and gemspec files through the same bounded, no-symlink reader. Maven parent coordinates must match; excluded files, external parents and network resolution are not used. Profile and Ruby platform/group/conditional declarations remain candidates, with explicit uncertainty instead of an effective or installed version. Unrendered JavaScript/requirements templates retain static findings and available branch declarations, but generated contents and dynamic Ruby evaluation remain incomplete with an explanatory diagnostic. Selected V8 script grammar compatibility preserves source positions and strict/module boundaries. Recognized legacy DSA identifiers do not make missing public-key parameters available.
