# Scanning repositories

[日本語](SCANNING.ja.md) · [README](../../README.md)

Use this guide after the first scan when you need to select outputs, change CI exit behavior or understand incomplete inspection. See [report interpretation](REPORTS.md) for inventory and policy output, and [supported inputs](COVERAGE.md) for format-specific limits.

## Contents

- [Command and options](#command)
- [Inspection diagnostics](#diagnostics)
- [GitHub Actions](#github-actions)
- [Running from source](#from-source)

<a id="command"></a>

## Command and options

```bash
pqc-scan scan <path> [--format json|html|md|sarif|all] [--lang en|ja] [--out-dir ./pqc-report] [--rules-dir ./rules] [--fail-on high|critical] [--threads N] [--diagnostics] [--require-complete] [--inventory-json] [--repository-scope ID] [--policy-profile migration-review-v1] [--assessment-context PATH]
pqc-scan rules list [--rules-dir ./rules]
pqc-scan rules show <rule-id> [--rules-dir ./rules]
```

Defaults are `--format all`, `--lang en`, `--out-dir ./pqc-report`, `--rules-dir ./rules` and `--threads 0` (automatic thread selection). No finding threshold or completeness gate is enabled by default. Paths are resolved from the current working directory.

`--repository-scope` requires `--inventory-json`; `--policy-profile` also requires it. `--assessment-context` requires an explicit policy profile. Use `pqc-scan scan --help` for the complete option list.

### Recommended usage from your product repository

When running `pqc-scan` inside your own product repository (not inside the `pqc-scan` repo), use an **absolute path** for `--rules-dir`.

```bash
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

### Common Commands

```bash
# Full report set

pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# SARIF format (for GitHub Code Scanning)

pqc-scan scan . --format sarif --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# CI failure gate

pqc-scan scan . --fail-on high --rules-dir /path/to/pqc-scan/rules

# Inspect one rule in detail

pqc-scan rules show JWT_RS256 --rules-dir /path/to/pqc-scan/rules
```

<a id="diagnostics"></a>

## Inspection diagnostics

Read failures and detector errors retain other findings and produce typed inspection
diagnostics. The CLI prints one completeness warning to stderr when inspection is
partial, failed or unknown. Default exit codes remain unchanged: 0 for success,
1 for an operational error and 2 when `--fail-on` is reached. No findings does not
prove that inspection completed.

Use `--diagnostics` to additionally write `diagnostics.json` in `--out-dir`.
This separate artifact has `schema_version: "1"`, generation time, declared scope,
coverage counters and safe reason codes. Arbitrary detector/I/O error contents and
file contents are not included. The six existing report filenames and machine
schemas remain unchanged; `--format all` does not implicitly enable diagnostics.
`--lang` continues to affect HTML/Markdown only.

```bash
pqc-scan scan . --diagnostics --require-complete --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

`--require-complete` returns **3** when inspection is not known to be complete,
after writing the requested reports. Exit precedence is operational error (1),
requested completeness failure (3), then finding threshold (2). Neither option
requires the other. A valid empty scope can complete, but this says nothing about
cryptography outside that scope.

Regex, AST and key-marker detectors report applicability and bounded inspection
outcomes. Accepted-hit limits remain **32 / 32 / 8 per rule and file**. Regex
counts accepted hits after the existing comment filter, so comment-only matches
no longer consume the accepted-hit quota. Existing language/comment policy is
unchanged. Recovered source hits are an intentional detection correction.

Exactly 32 (or 8) hits does not by itself prove truncation: the detector checks
remaining candidates within a finite work budget. An additional eligible hit or
an unfinished search at a work limit yields `detector_truncated`, retaining earlier
hits and making inspection partial. No omitted-hit count is inferred. Regex/AST
inspect at most 4096 match candidates per rule/file, plus one look-ahead. AST also
retains at most 4096 semantic candidates and schedules at most 100000 tree nodes;
its existing 2000-character candidate clipping now reports truncation. These are
engineering safety bounds, not measured performance targets. AST parsing with
error nodes retains available hits and reports `detector_partial`; initialization
or no-tree failures are detector errors. Binary input, unsupported source
languages and no applicable rules are outside the corresponding detector scope.
`Complete` refers only to the supported pattern/marker strategy, not general
symbol resolution, runtime use or discovery of every key format.

Certificate detection also reports applicability and parsing/attribute completeness.
Malformed or mixed/unknown-attribute certificates yield `detector_partial`; unsupported
containers yield `detector_unsupported`. A container finding does not prove that its
contents were inspected. Unmatched but parsed certificates need not produce a finding.

Dependency inspection now distinguishes valid empty input, malformed supported input,
usable partial records, unsupported versions and resource limits. Supported structured
subsets can report completion; unrelated files are not applicable. Ruby/Gradle/XML,
requirements and Go lexical fallbacks remain `detector_not_reported`. Completion is
within the supported subset, not package-manager validation or installed-state proof.
`--require-complete` keeps its exit-code contract, but corrected coverage can change
which code a scan returns. See [dependency provenance](../architecture/DEPENDENCY-PROVENANCE.md).

The legacy `scanned_files` count still means files selected by the walker. New
counters distinguish selected, read-ok, read-failed, analyzed, partial,
analysis-failed, unsupported, not-applicable and unknown files. Enumerated policy
exclusions and enumeration failures are separate; ignore-filtered populations are
not counted and are explicitly unknown. Completeness is only within declared
scope, including the existing ignore rules, default exclusions and size limit.

<a id="github-actions"></a>

## GitHub Actions

For a CI scan of your product, generate SARIF with `--format sarif` and choose `--fail-on high|critical` if findings should fail the job. Add `--require-complete` only if incomplete inspection must also fail; its exit 3 takes precedence over the finding threshold. The selected scope and unsupported formats still matter.

The checked-in [workflow](../../.github/workflows/pqc-scan.yml) shows GitHub Code Scanning upload and artifact storage. Retain the raw report so findings remain available for review.
This repository applies an additional source/result-bound policy to its own reviewed test/catalog references; it is not a general exclusion of test directories. See [self-scan policy](../quality/CI-SELF-SCAN.md) before reusing that preparation step in another project.

<a id="from-source"></a>

## Running from source

If you do not want to install yet, you can run directly from source:

```bash
cargo run --manifest-path /path/to/pqc-scan/crates/pqc_scan_cli/Cargo.toml --bin pqc-scan -- scan /path/to/your-product --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```
