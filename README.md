# pqc-scan - Post Quantum Cryptography Scan Tool

Japanese version: [README.ja.md](README.ja.md)

`pqc-scan` is a Rust CLI that helps teams prepare for post-quantum cryptography (PQC) migration.
It scans repositories for cryptographic usage patterns, identifies quantum-relevant risk signals, and generates actionable migration reports for engineering and security teams.

## Why This Tool

Many organizations already use SBOMs for software supply-chain visibility, but PQC migration also requires visibility into **cryptographic usage in code and configuration**.
`pqc-scan` provides that visibility with practical outputs for triage, planning, and CI automation.

## Quick Start

Scan your product repository (recommended real-world usage):

```bash
pqc-scan scan /path/to/your-product --format all --out-dir /path/to/your-product/pqc-report --rules-dir /path/to/pqc-scan/rules
```

Generated files:

- `/path/to/your-product/pqc-report/report.json`
- `/path/to/your-product/pqc-report/report.html`
- `/path/to/your-product/pqc-report/report.md`
- `/path/to/your-product/pqc-report/report.sarif`
- `/path/to/your-product/pqc-report/cbom.json`
- `/path/to/your-product/pqc-report/dependency-sbom.json`

## Installation

### Option A: Install as `pqc-scan` command (recommended)

```bash
cd /path/to/pqc-scan
cargo install --path crates/pqc_scan_cli
```

Then run from your product repository:

```bash
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

If `pqc-scan: command not found` appears after installation, ensure `~/.cargo/bin` is in your `PATH`.

### Option B: Run without installing (source checkout)

If you do not want to install yet, you can run directly from source:

```bash
cargo run --manifest-path /path/to/pqc-scan/crates/pqc_scan_cli/Cargo.toml --bin pqc-scan -- scan /path/to/your-product --format all --out-dir /path/to/your-product/pqc-report --rules-dir /path/to/pqc-scan/rules
```

## Core Capabilities

- Hybrid detection pipeline: `tree-sitter` + `regex`
- Rule-driven findings with severity, confidence, and quantum risk tags
- Migration guidance per finding (runtime/language-aware recommended actions)
- Rule and severity reference: [docs/rules-reference.md](docs/rules-reference.md)
- Source snippet extraction with highlighted matched line (HTML + JSON)
- CBOM export (`cbom.json`)
- Dependency inventory export (`dependency-sbom.json`)
- CI/CD-friendly outputs including SARIF for GitHub Code Scanning

## CBOM vs SBOM

| Artifact | Purpose | Main Question |
|---|---|---|
| `dependency-sbom.json` | Software dependency inventory | "What packages/libraries are present?" |
| `cbom.json` | Cryptographic usage inventory | "What algorithms and crypto usages are present, and what is their quantum risk?" |

In practice:

- Use SBOM data to track component and package exposure.
- Use CBOM data to prioritize cryptographic migration work (JWT/TLS/PKI/SSH/etc.).
- Use both together for end-to-end PQC migration planning.

## Detection Coverage

### Detector Modules

- `tree_sitter_detector`
- `regex_detector`
- `dependency_detector`
- `certificate_detector`
- `key_detector`

### Tree-sitter Language Support

- Java
- Go
- JavaScript
- TypeScript
- Python
- Rust
- Ruby

### Dependency and Manifest Support

- `Cargo.toml`, `Cargo.lock`
- `Gemfile`, `Gemfile.lock`, `gems.rb`, `gems.locked`, `*.gemspec`
- `pom.xml`, `build.gradle`, `build.gradle.kts`, `gradle.lockfile`
- `go.mod`
- `package.json`, `package-lock.json`, `pnpm-lock.yaml`
- `requirements.txt`, `Pipfile.lock`, `poetry.lock`

### SBOM Ingestion Support

- CycloneDX JSON (`*.cdx.json`, `*bom*.json`)
- SPDX JSON (`*.spdx.json`, `*sbom*.json`)

### Certificate and Key Support

- Certificate files: `.pem`, `.crt`, `.cer`, `.der`, `.p12`, `.pfx`
- Private key markers:
  - `BEGIN RSA PRIVATE KEY`
  - `BEGIN PRIVATE KEY`
  - `BEGIN OPENSSH PRIVATE KEY`

### Middleware/Config Coverage

- Kubernetes / Ingress
- nginx
- Apache httpd
- Envoy
- Istio
- HAProxy
- Traefik

### Built-in Rule Set

- Default rules live under `rules/default/`
- Current built-in rules: **100+**
- Full reference (all rule IDs + severity definitions): [docs/rules-reference.md](docs/rules-reference.md)

## CLI

```bash
pqc-scan scan <path> [--format json|html|md|sarif|all] [--out-dir ./pqc-report] [--rules-dir ./rules] [--fail-on high|critical] [--threads N]
pqc-scan rules list [--rules-dir ./rules]
pqc-scan rules show <rule-id> [--rules-dir ./rules]
```

### Recommended usage from your product repository

When running `pqc-scan` inside your own product repository (not inside the `pqc-scan` repo), use an **absolute path** for `--rules-dir`.

```bash
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

### Common Commands

```bash
# Full report set
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# SARIF only (for GitHub Code Scanning)
pqc-scan scan . --format sarif --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# CI failure gate
pqc-scan scan . --fail-on high --rules-dir /path/to/pqc-scan/rules

# Inspect one rule in detail
pqc-scan rules show JWT_RS256 --rules-dir /path/to/pqc-scan/rules
```

## Report Outputs

### `report.json`

Machine-readable full scan result including findings, metadata, recommended actions, and source snippets.

### `report.html`

Human-friendly triage report with:

- severity badges
- finding metadata
- highlighted source snippets
- recommended migration actions

#### HTML Report Preview

The screenshot below shows a real `report.html` example with grouped findings, an occurrences table, highlighted source snippets, and recommended actions.

![HTML report example](docs/html-report-exmaple.jpg)

### `report.md`

Portable text report for audit notes, pull requests, and artifact storage.

### `report.sarif`

SARIF output for code scanning platforms (including GitHub Code Scanning).

### `cbom.json`

Cryptographic Bill of Materials with algorithm/location/risk/migration inventory.

### `dependency-sbom.json`

Dependency inventory extracted from manifests and SBOM files.

## Security and Privacy Controls

- Full private key material is never emitted.
- Sensitive snippet output is masked.
- `.gitignore` is respected during repository walking.
- Default excludes: `.git`, `node_modules`, `target`, `dist`, `docs/`, `*.md`.
- Files larger than 2 MB are skipped.

## Accuracy and Disclaimer

`pqc-scan` performs **best-effort static analysis** using rules and heuristics.
It is designed to accelerate triage and migration planning, but it is **not a formal proof tool** and does **not guarantee 100% detection accuracy**.

Please assume the following:

- False positives can occur.
- False negatives can occur.
- Findings should be validated in your environment before making production or compliance decisions.

By using this tool, you acknowledge that scan results are advisory and must be combined with expert review, architecture context, and runtime/security testing.

## Known Limitations

- Static scanning cannot fully infer runtime crypto negotiation paths.
- Library-specific behavior may differ by version and configuration.
- Some parser-less formats are detected heuristically and may require custom rules.

## Architecture

Workspace crates:

- `pqc_scan_cli`: CLI parsing and command dispatch
- `pqc_scan_core`: scan pipeline, risk engine, deduplication, recommendation enrichment
- `pqc_scan_rules`: YAML rule loading and regex compilation
- `pqc_scan_detectors`: detector implementations
- `pqc_scan_report`: JSON/HTML/Markdown/SARIF/CBOM writers

See [docs/architecture.md](docs/architecture.md) for details.

## GitHub Actions

Example workflow:

- `.github/workflows/pqc-scan.yml`

This workflow builds the scanner, runs SARIF scan output, uploads SARIF to GitHub Code Scanning, and uploads `dependency-sbom.json` as an artifact.

## License

MIT
