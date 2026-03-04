# Architecture

`pqc-scan` is organized as a Rust workspace with clear module boundaries for CLI, core scanning, rule loading, detectors, and reporting.

## Workspace Modules

- `pqc_scan_cli`
  - CLI parsing (`clap`)
  - command dispatch (`scan`, `rules list`)
  - fail-on threshold behavior (`high`, `critical`)
- `pqc_scan_core`
  - repository walking
  - pipeline orchestration
  - finding normalization and deduplication
  - risk/severity adjustment
  - runtime/dependency profiling
  - recommendation enrichment
  - source snippet extraction for findings
- `pqc_scan_rules`
  - YAML rule loading and validation
  - regex compilation at load time
  - rule indexing and filtering by kind
- `pqc_scan_detectors`
  - `tree_sitter_detector`
  - `regex_detector`
  - `dependency_detector`
  - `certificate_detector`
  - `key_detector`
- `pqc_scan_report`
  - report writers for JSON / HTML / Markdown / SARIF
  - CBOM and dependency SBOM output

## End-to-End Scan Flow

1. Load rules from `rules/default/` (or custom `--rules-dir`)
2. Walk repository files with `.gitignore` awareness
3. Exclude default paths and large files (`> 2 MB`)
4. Run detectors in parallel (file-level concurrency via `rayon`)
5. Convert detector output (`Detection`) into normalized findings (`Finding`)
6. Apply risk-aware severity adjustment
7. Deduplicate repeated hits
8. Build dependency inventory from detector metadata (`dependency-sbom.json`)
9. Build runtime/dependency profile from repo manifests + dependency signals
10. Enrich findings with recommended migration actions
11. Attach source snippets (line window + highlighted hit line)
12. Build CBOM from findings (`cbom.json`)
13. Emit selected report formats

## Rule System

Rules are YAML-defined and loaded dynamically.

Supported `kind` values:

- `regex`
- `tree_sitter`
- `dependency`
- `certificate`
- `key`

Rule fields:

- `id`
- `kind`
- `category`
- `severity`
- `risk`
- `confidence`
- `migration_hint`
- `pattern`
- `scope`
- `description` (optional)

Compilation strategy:

- Regex patterns are compiled once during rule loading
- Invalid regex fails fast with rule id context
- Rule IDs are deduplicated and indexed for O(1) lookup

## Detector Layer

All detectors implement the shared `Detector` trait:

- `name() -> &'static str`
- `detect(file, rules) -> Vec<Detection>`

### Tree-sitter detector

- Parses source code and inspects semantic node types
- Applies tree-sitter scoped rules per language
- Current language support:
  - Java
  - Go
  - JavaScript
  - TypeScript
  - Python
  - Rust
  - Ruby

### Regex detector

- Fast lexical matching over text files
- Produces line/column offsets and masked evidence previews

### Dependency detector

- Parses common manifests and lock files
- Parses CycloneDX/SPDX JSON files as dependency input
- Emits both:
  - dependency inventory entries
  - rule-based dependency findings

### Certificate detector

- Parses PEM/DER certificate data (`x509-parser`, `pem`)
- Extracts signature algorithm, OID, key size, expiration
- Matches certificate rules against extracted metadata

### Key detector

- Uses efficient marker matching for private key blocks
- Classifies private-key hits as sensitive
- Never emits raw key material

## Core Data Model

- `Location`
  - `file`, `line`, `column`
- `Evidence`
  - `type`, `match`, `snippet_preview`, `metadata`
- `Finding`
  - rule/risk/severity/confidence metadata
  - `migration_hint`
  - `recommended_actions[]`
  - `source_snippet` (optional)
- `RecommendedAction`
  - `action_id`, `title`, `priority`, `rationale`
  - `steps[]`, `references[]`, `code_examples[]`
- `SourceSnippet`
  - 4-5 line context window (default: hit line ±2 lines)
  - explicit highlighted line flag
- `CbomEntry`
  - component/algorithm/usage/location/quantum_risk/migration_hint
- `DependencySbomEntry`
  - name/version/ecosystem/source_file/source_type/purl

## Risk and Severity Engine

Risk categories:

- `quantum-vulnerable`
- `quantum-uncertain`
- `quantum-safe`
- `non-quantum-risk`

Severity levels:

- `info`
- `low`
- `medium`
- `high`
- `critical`

Adjustment strategy:

- Sensitive categories (`TLS`, `PKI`, `JWT`, `auth`) can be severity-boosted
- Only high-confidence vulnerable findings are promoted
- Private key evidence is forced to highest severity path

## Recommendation Engine

The recommendation engine enriches each finding with concrete next actions.

Inputs:

- finding attributes (rule/category/risk/evidence/location)
- language signal (from AST metadata and file extension)
- runtime profile (Java/Node/Go/Python/Rust/Ruby hints)
- dependency signals (for example BouncyCastle presence)

Behavior:

- adds generic migration planning actions for quantum-vulnerable findings
- adds category-specific actions (JWT/TLS/PKI/CryptoAPI)
- adds language/runtime-specific migration suggestions
- includes references and optional before/after code examples

## Source Snippet Enrichment

For each finding:

- read source text once per file (cache-backed)
- extract local line window around the matched line
- mark the hit line for highlight rendering in reports
- sanitize output:
  - private key lines are masked
  - overly long lines are clipped
  - binary files are skipped

## Reporting Layer

### JSON (`report.json`)

- full machine-readable scan result
- includes findings, recommendations, source snippets, summary

### HTML (`report.html`)

- analyst-friendly report UI
- severity badges, finding metadata, recommendation blocks
- line-numbered source snippets with highlighted hit line

### Markdown (`report.md`)

- human-readable text report
- suitable for artifact archives and PR comments

### SARIF (`report.sarif`)

- GitHub code scanning compatible output
- maps severity to SARIF levels (`error`/`warning`/`note`)
- includes migration hint and recommended action summary

### CBOM / Dependency SBOM

- `cbom.json`: cryptographic bill of materials from findings
- `dependency-sbom.json`: normalized dependency inventory from manifests/SBOMs

## Performance Characteristics

- file-level parallel detector execution (`rayon`)
- precompiled rule regex for repeated scans
- bounded scan scope:
  - `.gitignore` respected
  - default directory exclusions
  - max file size guard
- low-overhead deduplication to reduce report noise

## Security Considerations

- sensitive material is always masked in evidence/snippets
- private key content is never exported
- detector output is normalized before reporting
- scan is read-only against repository contents

## Extensibility Guidelines

- Add a new detector:
  1. implement `Detector` trait
  2. register in `pqc_scan_detectors::default_detectors()`
  3. add matching rule kind/patterns
- Add new rule packs:
  - place YAML files under a rules directory
  - load with `--rules-dir`
- Extend recommendations:
  - add templates in core recommendation module
  - bind by category/language/runtime/dependency signals
- Add a new report format:
  1. implement writer module in `pqc_scan_report`
  2. register enum variant and output path mapping
