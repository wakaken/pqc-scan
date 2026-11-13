# Understanding reports

[日本語](REPORTS.ja.md) · [README](../../README.md)

This guide describes the v0.3 development checkout. Detected references and findings can differ from the published v0.2.0 release.

After a scan, use the human report to investigate findings, then choose machine-readable files for tracking or automation. A finding records evidence and suggested checks; it is not an instruction to replace an algorithm without investigating its use.

Reports show findings, locations and suggested next steps. The inspection overview separately identifies incomplete checks and files needing follow-up.

## Contents

- [Review findings](#review)
- [Interpret risk and PQC findings](#pqc-findings)
- [Choose a report, including CBOM and SBOM](#choose-report)
- [Capture richer inventory](#inventory)
- [Export CycloneDX 1.7](#cyclonedx)
- [Compare inventories](#diff)
- [Use local policy assessment](#assessment)
- [Read SARIF locations](#sarif-locations)

<a id="review"></a>

## Review findings

Start with `report.html`:

1. Check the inspection overview for selected, analyzed and follow-up file counts, exclusions and limits.
2. Select a rule and a file location. Read the source context and the evidence that triggered it.
3. Check whether the reference belongs to active application code, a setting, a dependency, an example or a test. Static matching cannot settle runtime use.
4. Review the proposed action against the actual runtime, library, protocol and peers before making a change.

HTML reports start with a compact, severity-sorted rule list. Search by rule or
file, filter severity/risk, and switch between risk candidates and informational
inventory. Rules and their occurrences are paginated in groups of 20. Expand a
rule, then a location, to see its source context and applicable actions. Identical
shared guidance is linked instead of repeating it at every location. Different
recommendations sharing an action ID remain distinct. Without JavaScript, all
records remain available through native expandable details. No external scripts
or network connection are required. Markdown provides a linked rule/location index.
Source excerpts and code examples normalize CRLF and CR line endings and indent
every line so embedded markup remains code when rendered as Markdown.

TLS guidance prefers TLS 1.3, retaining securely configured TLS 1.2 only for an
identified interoperability need. PQC override settings receive setting-specific
checks rather than an assertion of RSA usage.

Use `--lang ja` for Japanese HTML and Markdown reports. English remains the default
(`--lang en`). JSON, SARIF, CBOM and dependency SBOM retain their existing content
regardless of this option. Finding IDs, paths, evidence, code and reference URLs
are never translated. Japanese reports explain the findings and next steps using
a built-in catalog; no repository data is sent to a translation service.

```bash
pqc-scan scan . --lang ja --format all --out-dir /path/to/reports/product-scan-ja --rules-dir /path/to/pqc-scan/rules
```

Built-in rule descriptions and recommended actions have Japanese copy. Custom
rules or actions without a translation show an explicit notice and retain the
original text. `quantum-safe` is displayed as a PQC reference, not a declaration
that the product is safe or migration is complete.

<a id="pqc-findings"></a>

## Interpret risk and PQC findings

The scanner reports both migration concerns and informational references. Severity helps prioritize investigation; confidence describes a heuristic match, not a calibrated probability that the application is vulnerable.
Risk labels describe the matched signal:

| Label | How to read it |
|---|---|
| `quantum-vulnerable` | A quantum-related migration concern for the referenced algorithm or usage |
| `non-quantum-risk` | A conventional weakness, such as a weak algorithm or certificate-related issue; it is a separate concern from PQC migration |
| `quantum-uncertain` | The available evidence is insufficient to determine quantum readiness |
| `quantum-safe` | The referenced primitive is classified as quantum-resistant; this is not a verdict on the application |

### Why PQC-related references appear

The results include references to ML-KEM (key establishment), ML-DSA and SLH-DSA (signatures), as well as old draft names such as Kyber and Dilithium. These help locate code and dependencies that may already be involved in migration work.
For example:

| Observed reference | What still needs checking |
|---|---|
| An ML-KEM name in code | Whether the intended runtime/provider supports it and whether that path is actually used |
| A PQC-capable library in a dependency file | The resolved version, enabled features and actual API calls; dependency presence alone proves none of these |
| A Kyber/Dilithium draft name | Which implementation and specification version it refers to; old and final names are not interchangeable evidence |
| A TLS setting that disables PQC/hybrid groups | The effective configuration and peer support; this is a configuration signal, not an observed negotiated connection |

The rule set includes ML-KEM/ML-DSA references across the seven listed languages, selected Java, Node and Ruby SLH-DSA API references, selected libraries, Java/Ruby classical-only TLS group settings and Go `GODEBUG=tlsmlkem=0`.
Certificate signature OIDs and subject-public-key OIDs for ML-DSA/SLH-DSA are checked separately; a filename does not establish the certificate's algorithm. Parsed OIDs do not verify a signature, trust chain or valid PQC key.

Selected built-in rules use [bounded local source analysis](COVERAGE.md#source-facts) to distinguish supported API calls from shadowed names or display strings. Other rules remain nominal pattern checks. In either case, review the evidence and inspection status; local reference tracing does not establish execution.

These references can appear as informational findings in CBOM and SARIF too. Read `quantum-safe` as a classification of the primitive, not “migration complete.” Dependency findings remain `quantum-uncertain`.
Node crypto/WebCrypto, browser and TypeScript runtime support must be checked individually; their version requirements differ. JWT signature changes also need compatible JOSE formats, libraries and verifiers.
See [static examples and references](../../examples/pqc-readiness/README.md) for specific patterns.

<a id="choose-report"></a>

## Choose a report

| File | Selection | Use |
|---|---|---|
| `report.html` | `--format html` or `all` | Investigate findings interactively |
| `report.md` | `--format md` or `all` | Share text findings and source/action links |
| `report.json` | `--format json` or `all` | Process findings, evidence, recommendations and summary |
| `report.sarif` | `--format sarif` or `all` | Send findings to Code Scanning tools |
| `cbom.json` | Every report format | Track the cryptographic findings by algorithm, location, risk and guidance |
| `dependency-sbom.json` | Every report format | Track dependencies extracted from manifests, locks and supplied SBOMs |
| `scan.inventory.v1.json` | `--inventory-json` | Keep typed observations, assets and links to legacy findings |
| `scan.assessment.v1.json` | Inventory plus `--policy-profile migration-review-v1` | Review a separate local advisory assessment |
| `diagnostics.json` | `--diagnostics` | Check inspection scope, coverage and safe failure reasons |
| `cbom.cdx-1.7.json`, `dependency-sbom.cdx-1.7.json` | `--cyclonedx-17` | Exchange separate CycloneDX 1.7 cryptographic and software inventories |

`--format all` selects the six legacy files; additional artifacts require their own options. A single selected report format still writes both flat inventory files. `--lang en|ja` changes HTML/Markdown only.
Use a fresh output directory for comparisons; files from previous runs are not automatically removed.

To request the six default files plus inventory, diagnostics and both CycloneDX
exports (ten files in total):

```bash
pqc-scan scan /path/to/your-product --format all --inventory-json --diagnostics --cyclonedx-17 --lang en --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

A policy assessment is an additional, separate opt-in. The v0.3 development work retains inventory v1, SARIF 2.1.0 and CycloneDX 1.7; the product roadmap version does not rename these formats. Separate
probe reports are described in the [probe guide](PROBING.md).

### What CBOM and SBOM mean here

An SBOM (Software Bill of Materials) lists software components and dependencies. A CBOM (Cryptographic Bill of Materials) lists cryptographic algorithms and usages.
For instance, a dependency record can show that a crypto library is declared, while a cryptographic finding can point to an algorithm reference in a source file. Use both to connect dependency investigation with code/configuration review, without treating either as proof of runtime use.

`dependency-sbom.json` and `cbom.json` are pqc-scan's flat JSON inventories. They are not CycloneDX/SPDX documents or exhaustive runtime inventories. Reading an existing CycloneDX/SPDX SBOM is supported. The separate opt-in CycloneDX export is described below.
Use `scan.inventory.v1.json` when you need richer observations and provenance alongside the legacy findings.

<a id="inventory"></a>
## Capture richer inventory

`--inventory-json` adds `scan.inventory.v1.json` with method-specific observations,
normalized algorithm/public-key/certificate/dependency assets, and links to legacy
findings. This is pqc-scan's own JSON format. It is opt-in even with `--format all`;
existing reports and IDs remain unchanged. Use `--repository-scope team/project`
with inventory capture to enable weak cross-scan source correlation. Omit secret
or absolute identifiers. The report does not assert active use, trust or a PQC
assessment. See the [contract and limitations](../architecture/INVENTORY-REPORT-V1.md).

```bash
pqc-scan scan . --inventory-json --repository-scope team/project --rules-dir ./rules
```

Dependency assets preserve declared constraints (for example `^1.2.3`), optional
lock-record resolved versions and source provenance separately. A declaration or
SBOM claim alone never creates a versioned purl. Path/git/workspace locators stay
unresolved; no dependency graph or package download is inferred. The legacy
`dependency-sbom.json` remains a flat array with the same fields; corrected purls
and inspection diagnostics are intentional semantic changes. Source origins remain
separate in inventory even when the legacy projection collapses them.

<a id="cyclonedx"></a>
## Export CycloneDX 1.7

Use `--cyclonedx-17` to exchange inventory with tools accepting CycloneDX 1.7:

```bash
pqc-scan scan . --cyclonedx-17 --rules-dir /path/to/pqc-scan/rules --out-dir /path/to/new-output
```

`cbom.cdx-1.7.json` carries observed algorithm/public-key/certificate assets;
`dependency-sbom.cdx-1.7.json` carries software dependency assertions. Native
observations are captured internally; writing `scan.inventory.v1.json` still
requires `--inventory-json`. Neither `--format all` nor input SBOM detection enables
this export automatically. The two legacy arrays retain their names and shape.

Read the `pqc-scan` properties and conversion-loss notes as well as the standard
fields. A declared constraint or a version reported in an input SBOM is not proof
of installation. Static references are not runtime protocol observations; missing
facts, trust validation, security strength and certification are not invented.

<a id="diff"></a>
## Compare inventories

Capture both scans with `--inventory-json --repository-scope team/project`, then:

```bash
pqc-scan diff /path/to/before/scan.inventory.v1.json /path/to/after/scan.inventory.v1.json --repository-scope team/project --out-dir /path/to/new-diff
```

Both inputs must use the requested scope and compatible schema/identity versions.
The command writes `scan.diff.v1.json` and `scan.diff.v1.md`. Exact correspondence
uses inventory asset/occurrence keys, not legacy finding IDs or checkout roots.
Rule/profile/coverage changes are disclosed. Duplicate anchors and missing target
inspection evidence stay unresolved; absence alone never proves remediation.
Current inventory v1 cannot prove target-level exhaustive absence, so disappeared
records remain unresolved. Unique changed-path pairs may be suggested as heuristic
moves; they are never merged automatically or treated as verified renames.

<a id="assessment"></a>

## Use local policy assessment

HTML starts with selected/analyzed/follow-up file counts, actual exclusion settings
and file-size limits. Failed, incomplete and unknown checks remain distinct;
unrecorded excluded populations are never presented as a complete total.
With `--diagnostics`, a closed details section groups reasons and shows at most
three example paths per reason. Per-observation IDs, assurance and usage ledgers
are omitted from HTML; exact records remain in inventory JSON and Markdown.
HTML does not display advisory policy decisions. It focuses on inspection
status, finding locations, checks and next steps. Explicitly requested policy
results and reasons remain in assessment JSON and Markdown.

`--inventory-json --policy-profile migration-review-v1` adds a separate
`scan.assessment.v1.json` with quantum/classical concerns, observation certainty,
usage, priority and purpose-specific candidate prerequisites. Existing reports,
inventory JSON, severity and exit codes remain unchanged. This is local advisory
migration review, not compliance or deployed-security assurance, with no deadlines.
Unknown/incomplete inspection cannot yield a clean pass; current inventory v1
cannot yield aggregate Pass.

```bash
pqc-scan scan . --inventory-json --policy-profile migration-review-v1 --rules-dir ./rules
```

Optional `--assessment-context ./context.json` supplies typed local context, e.g.
`{"version":1,"use_case":"jwt"}`. Missing support stays unknown; provided values are
unverified operator claims. Dependency presence does not prove PQC readiness.
See [scope, prerequisites and context fields](../architecture/LOCAL-ASSESSMENT-V1.md).

<a id="sarif-locations"></a>

## Read SARIF locations

SARIF remains version 2.1.0. File paths are percent-encoded UTF-8 URI references
(literal `%`, spaces, `#` and `?` are encoded); absolute paths become file URIs.
Windows native separators are normalized on Windows; Unix backslashes are literal.
An unknown line (`0` in `report.json`) produces an artifact-only location. A known
line with unknown column omits `startColumn`; an empty path omits `locations`.
Columns are one-based Unicode code points (`columnKind: unicodeCodePoints`),
including one column per emoji, combining code point or tab, rather than visual
width or UTF-16 units. Legacy AST columns mix byte and character offsets: SARIF
keeps a column only when the existing unmasked snippet proves an ASCII prefix;
otherwise it retains the line only. Missing, masked or clipped prefixes cannot
prove a column. `report.json` locations/IDs, other reports, CLI flags and filenames
are unchanged. These corrections intentionally replace invalid/misleading SARIF
positions and unescaped paths. See the [contract and validation record](../quality/SARIF-LOCATION-CONTRACT-022.md).
