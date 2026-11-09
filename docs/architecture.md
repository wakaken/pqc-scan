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
- `detect(file, rules) -> Result<Vec<Detection>>`
- Optional `detect_detailed(file, rules) -> Result<DetectorOutcome>`; the default
  calls `detect` once and reports `NotReported`, preserving existing implementations.

### Inspection diagnostics (task 018)

`scan_repository_detailed` returns a private-field `ScanDocument` with the legacy
`ScanResult`, diagnostics and coverage. `scan_repository` remains a wrapper over
the same execution and returns the existing result; detectors are not run twice.
Normalization, finding IDs, severity, recommendations and existing report schemas
are unchanged. Per-file read/detector failures retain other results. Root/setup
and report-write failures remain fatal. The detailed walker accounts for policy
exclusions, metadata failures and soft ignore-policy errors separately from selected files.
Iterator-level traversal failures, including root enumeration failures after
successful metadata access, remain fatal as in the legacy walker. Soft errors
attached to valid directory entries retain other results but prevent a complete
inspection verdict.

Coverage has disjoint file terminal states and reconciles selected = read-ok +
read-failed, and read-ok = analyzed + partial + analysis-failed + unsupported +
not-applicable + unknown. Enumeration failures are an additional entry count;
ignore-hidden populations are not enumerable. Complete means complete within the
declared scope, not absence of cryptography or product security. The dependency
detector still uses `NotReported`, so full nonempty scans generally
have unknown completeness unless known partial inspection takes precedence.

CLI adds a stderr notice and opt-in `--diagnostics` (`diagnostics.json`, schema 1)
and `--require-complete` (code 3). Existing default output and codes remain; fatal
errors take priority, then requested completeness, then finding threshold. Report
writers expose a separate `write_diagnostics` function; existing writer calls do
not add the artifact. Diagnostic messages are fixed safe descriptions, never raw
error chains or scanned contents. This is a diagnostics schema, not the proposed
Phase 1 native inventory schema.

### Bounded detector outcomes (task 019)

Regex, tree-sitter and key detectors override `detect_detailed`; legacy `detect`
consumes that same outcome without rerunning detection. No serialized fields,
CLI switches, rule patterns, finding IDs or risk classification change. Accepted
hits remain capped per rule/file at 32 (regex/AST) or 8 (keys). Regex eligibility
uses the existing comment policy before consuming its accepted-hit budget. AST
keeps its existing duplicate/embedded-identifier filters and traversal order.

Exactly reaching an accepted-hit cap can still Complete if bounded look-ahead
exhausts remaining candidates without another eligible unique hit. Regex/AST
allow 4096 examined regex matches per rule/file plus one iterator look-ahead;
key matching uses its ninth match as look-ahead. AST collection additionally
retains at most 4096 semantic candidates and schedules at most 100000 tree nodes.
Its existing 2000-character clipped candidate text stays compatible, with
Unicode-safe bounded clipping and a Truncated outcome. Work-limit truncation
means inspection stopped, not proof that a particular number of hits was omitted.
The existing `detector_truncated` reason and partial coverage communicate both
proven extra hits and conservative resource stops without adding a new schema.

Valid supported inspection can Complete with zero hits; no applicable rules,
nontext input or unsupported source language are NotApplicable. Error-containing
ASTs keep hits and report Partial; parser setup/no-tree errors propagate through
the existing safe DetectorFailed path. Complete is about the supported pattern
strategy, not full semantic analysis or exhaustive private-key format support.

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

### AST occurrence identity (task 024)

Private candidates retain their parsed-source byte start and the real prefix
length before any clipping ellipsis. Deduplication is per rule on absolute byte
intervals, trimming only edge whitespace/quote wrappers so parent/child versions
of one token share identity. Different same-line spans survive. It does not use
line/text identity, change rule patterns or export a new span model. Legacy
native line/column/evidence choice and the general finding-ID algorithm remain;
AST columns still mix byte-based node starts and code-point relative offsets.
SARIF retains its task022 conservative handling. Traversal/candidate/work/hit caps
are unchanged; the dedup set now lives within each rule. Matches extending into
a synthetic clipped suffix are rejected while Truncated stays visible.

The JavaScript grammar remains the TypeScript/TSX parser. Syntax recovery and
nominal API matching do not resolve aliases, local namesakes, actual execution
or all comments inside ancestor text. Frozen corpus and grammar-subset tests
record these limitations rather than silently expanding dependency support.
See [the task024 evidence](quality/TASK-024-AST-OCCURRENCES.md).

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

- Existing extension candidacy is preserved; PEM/DER is selected by content for
  `.pem/.crt/.cer/.der`. PKCS#12/PFX interiors remain explicitly Unsupported.
- `certificate_detector/attributes.rs` owns public, path/rule-independent
  `parse_certificate_der` / `parse_public_key_der` helpers. They return owned
  attributes and fixed safe errors, require one fully consumed DER object and do
  not perform signature/chain/point validation. The second helper accepts SPKI,
  not standalone private/raw-PKCS#1 keys; standalone key file scanning is not added.
- Signature OID and SPKI OID/parameters are separate. RSA length is the positive
  modulus bit length, with sign padding and non-byte-aligned values handled.
  Known named EC curves supply field bits (including P-521 = 521). Ed/X encoded
  lengths are distinct from semantic size; unknown/PQC keys never inherit RSA size.
  Parameter kind/OID/content digest is retained without arbitrary raw bytes;
  RSA-PSS parameter constraints are not interpreted as a migration/security assessment.
- SHA-256 identifies exact public certificate DER and SPKI DER. Explicit encoded
  byte counts, optional semantic size/curve and parsed validity timestamps live
  inside the existing evidence string map, without new report fields/schema versions.
- PEM framing recovers valid neighbors around bad/unsupported blocks. Real PEM
  line/column, 1-based block index and 0-based byte offset retain object provenance
  through existing dedup/ID generation. DER keeps the legacy 1:1 location and offset
  zero; trailing data prevents complete inspection. DER bundles are not parsed.
- Detailed outcomes distinguish Complete/NotApplicable/Partial/Unsupported. Known
  malformed key structures, unknown key semantics and mixed bundles cannot Complete.
  Container fallback is retained as a finding signal, never successful parsing.
- Built-in signature rules use exact parsed OIDs; new `rsa_modulus_bits` requires
  a measured RSA/PSS subject modulus. ECDSA/DSA risk tags are quantum-vulnerable;
  existing PKI severity promotion remains unchanged. Classical weak-digest/key
  findings and legacy DSA verification guidance remain separate concerns.
- New `certificate_validity` targets expired/not_yet_valid/valid using explicit
  Unix seconds with inclusive endpoints. `legacy_not_after_2026` retains the old
  date-named rule's fixed cutoff before 2027-01-01 UTC. Reversed intervals or an
  unavailable required clock retain static hits and prevent complete assessment.
- `CertificateDetector::at(i64)` supplies a fixed detector. Default registry creation
  captures its instant once; CLI creates a registry per scan. A reused library
  registry reuses its instant; rebuild it for fresh-time analysis. Direct legacy
  unit-detector calls capture per-call time. No ScanConfig/Detector trait change.
- Legacy custom composite scopes remain compatible except the three newly reserved
  opt-in names above. Typed time findings add only assessment time/status to the
  existing evidence map. Signature/subject/time evidence is separated; affected
  certificate IDs and legacy CBOM values intentionally change, not the ID algorithm
  or report structure. Unrelated findings and the task 020 parser remain unchanged.
  Full simultaneous assessment modeling and broad accuracy studies remain tasks
  026/023, and PSS digest-parameter interpretation remains limited.
- Source snippets from files carrying EC/DSA/encrypted private PEM markers are
  masked file-wide, extending the existing RSA/PKCS#8/OpenSSH protection. Bodies of
  unsupported blocks are never decoded/exported by certificate parsing.

See [task 021 specification/evidence](roadmap/TASK-021-IMPLEMENTATION.md),
[task 020 specification/evidence](roadmap/TASK-020-IMPLEMENTATION.md) and
[public fixture provenance](../crates/pqc_scan_detectors/tests/fixtures/certificates/README.md).

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

The internal grouping model retains every occurrence and its own action list,
including missing-snippet findings. Shared guidance is the intersection of full
serialized action payloads, not action IDs. A common compact renderer serves both
languages, with native expandable details and an offline script for filters,
20-item pagination and deep links. Machine serialization does not depend on this
presentation model. Script behavior has dependency-free Node test-double coverage;
this is distinct from actual browser/visual acceptance.

Human-facing HTML and Markdown support `--lang en|ja` (default `en`). The reporting
crate keeps the original `write_reports` entrypoint as an English wrapper and adds
`write_reports_with_language`. Japanese presentation uses a built-in rule/action
catalog, after sanitization, and never rewrites source evidence. Unknown custom
text is retained with an untranslated notice. JSON, SARIF and both inventories
remain language-independent. No runtime translation service is used.

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

Task 025 adds the opt-in Cryptographic Inventory and Evidence Report beside the
legacy pipeline. `--inventory-json` writes `scan.inventory.v1.json`; the original
`--native-json`/filename remain supported. See [INVENTORY-REPORT-V1.md](architecture/INVENTORY-REPORT-V1.md)
for identity, assurance and export losses.
