# Scanning repositories

[日本語](SCANNING.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/README.md)

This guide describes static v0.3.0. Detected references, evidence and IDs can differ between versions. See the [release notes](../releases/v0.3.0.md) for compatibility and [GitHub Releases](https://github.com/wakaken/pqc-scan/releases) for published assets. Local preparation and Actions artifacts are not published Releases.

Use this guide after the first scan when you need to select outputs, change CI exit behavior or understand incomplete inspection. See [report interpretation](REPORTS.md) for inventory and policy output, and [supported inputs](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/docs/user/COVERAGE.md) for format-specific limits.

## Contents

- [Command and options](#command)
- [GitHub Actions](#github-actions)
- [Running from source](#from-source)
- [Inspection diagnostics](#diagnostics)

<a id="command"></a>

## Command and options

```bash
pqc-scan scan <path> [--format json|html|md|sarif|all] [--lang en|ja] [--out-dir ./pqc-report] [--rules-dir ./rules] [--fail-on high|critical] [--threads N] [--max-file-size-mib N] [--diagnostics] [--require-complete] [--include-non-scannable] [--exclude-tests] [--cyclonedx-17] [--inventory-json] [--repository-scope ID] [--policy-profile migration-review-v1] [--assessment-context PATH]
pqc-scan rules list [--rules-dir ./rules]
pqc-scan rules show <rule-id> [--rules-dir ./rules]
pqc-scan diff <before-inventory> <after-inventory> --repository-scope ID --out-dir DIR
```

Defaults are `--format all`, `--lang en`, `--out-dir ./pqc-report`, `--rules-dir ./rules` and `--threads 0` (automatic thread selection). No finding threshold or completeness gate is enabled by default. Paths are resolved from the current working directory.

`--repository-scope` requires `--inventory-json`; `--policy-profile` also requires it. `--assessment-context` requires an explicit policy profile. Use `pqc-scan scan --help` for the complete option list.

Add `--cyclonedx-17` to write separate CycloneDX 1.7 inventories. The [report guide](REPORTS.md#cyclonedx)
explains these documents and the completeness limits of [inventory comparison](REPORTS.md#diff).


### File size limit

`--max-file-size-mib N` selects a per-file limit of 1–64 MiB (whole numbers; default 2). For example, `pqc-scan scan . --max-file-size-mib 16 --threads 1 --diagnostics` includes files up to 16 MiB. One MiB is 1,048,576 bytes. The configured value appears in the existing diagnostic/inventory scope; files above it remain explicitly excluded. Other ignore/type/test policies and detector work limits still apply. A larger read limit does not guarantee complete parsing; inspect diagnostics. More selected data and parallel workers can increase time and peak memory.

`serve --max-file-size-mib N` supplies the initial value in the folder picker. The “File size limit (MB)” field can change it for the next batch; all folders in that batch use its captured value. The control is locked while scanning. No saved report or history is rewritten by changing it. MB is the picker’s display label for the existing binary-sized setting: one displayed unit remains 1,048,576 bytes, consistent with the unchanged CLI MiB option and byte limit. The field accepts integers 1–64; empty or invalid values show an error and disable scan submission.

### Output destination safety

On Unix (including macOS and Linux), report writers reject symbolic links in the
output directory path and existing symbolic-link or non-regular-file destinations.
The root-controlled macOS aliases `/tmp`, `/var` and `/etc` remain usable; links
inside them receive the same checks. Use the physical path for other directory
aliases. This applies to scan, inventory, assessment, diagnostics, CycloneDX, diff
and probe reports.

Missing directories are created. Existing regular report files can be overwritten,
including on repeated runs. Each file is replaced atomically rather than truncated;
an external file sharing a hard link is not modified. Published files have private
permissions (0600, subject to umask); replacing a file does not preserve its inode,
old permissions, ACLs or extended attributes. Directory write permission is needed.
A failed report set may leave earlier files from that run; the set is not one
transaction. Concurrent modifications by someone who can write the destination
can still remove or alter reports; this protection prevents redirected writes,
not tamper-proof storage. Unsupported non-Unix platforms fail explicitly instead
of falling back to unsafe output.

### Recommended usage from your product repository

When running `pqc-scan` inside your own product repository (not inside the `pqc-scan` repo), use an **absolute path** for `--rules-dir`.

```bash
pqc-scan scan . --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

### Common Commands

```bash
# Full report set

pqc-scan scan . --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules

# SARIF format (for GitHub Code Scanning)

pqc-scan scan . --format sarif --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules

# CI failure gate

pqc-scan scan . --fail-on high --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules

# Inspect one rule in detail

pqc-scan rules show JWT_RS256 --rules-dir /path/to/pqc-scan/rules
```

<a id="github-actions"></a>

## GitHub Actions

For a CI scan of your product, generate SARIF with `--format sarif` and choose `--fail-on high|critical` if findings should fail the job. Add `--require-complete` only if incomplete inspection must also fail; its exit 3 takes precedence over the finding threshold. The selected scope and unsupported formats still matter.

The checked-in [workflow](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/.github/workflows/pqc-scan.yml) shows GitHub Code Scanning upload and artifact storage. Its `quality` job runs on PRs, while `baseline` and `scan` run on develop pushes; manual runs enable all three. Documentation-only automatic runs are skipped, and `protocols` is temporarily disabled even for manual runs. See [repository CI](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/README.md#repository-ci) for the event conditions. These are this repository's CI choices; select exit gates explicitly for your own product. Retain the raw report so findings remain available for review.
This repository applies an additional source/result-bound policy to its own reviewed test/catalog references; it is not a general exclusion of test directories. See [self-scan policy](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/docs/quality/self-scan/CI-SELF-SCAN.md) before reusing that preparation step in another project.

<a id="from-source"></a>

## Running from source

If you do not want to install yet, you can run directly from source:

```bash
cargo run --manifest-path /path/to/pqc-scan/crates/pqc_scan_cli/Cargo.toml --bin pqc-scan -- scan /path/to/your-product --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

<a id="diagnostics"></a>

## Inspection diagnostics

Zero findings does not mean every selected input was inspected. Read failures and detector errors retain other findings.
When inspection is partial, failed or unknown, the CLI prints a warning to stderr. A separate scope note explains that ignored or otherwise unenumerated files may exist; it is not itself an inspection failure.
CLI HTML/Markdown also display inspection status and counters. Detailed safe
reasons appear with `--diagnostics`; language selection changes human copy only.

### Save diagnostics and optionally fail CI

Use `--diagnostics` to investigate inspection gaps, and `--require-complete` to make incomplete inspection fail a CI job.
Either option can be used independently.

| Option | Behavior |
|---|---|
| `--diagnostics` | Adds `diagnostics.json` in `--out-dir`, recording scope, counters and safe reason codes |
| `--require-complete` | Returns exit 3 if inspection is not known to be complete, after writing the requested reports |

To select both:

```bash
pqc-scan scan . --diagnostics --require-complete --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

`diagnostics.json` also contains `schema_version: "1"` and generation time. It omits file contents and arbitrary detector/I/O error text.
`--format all` does not enable it; the six legacy filenames and machine schemas stay unchanged. `--lang` affects HTML/Markdown only.

Read exit codes as follows:

| Code | Meaning |
|---|---|
| 0 | Successful execution; not proof of complete inspection or security |
| 1 | An operational error, such as setup or output failure |
| 2 | The severity threshold selected by `--fail-on` was reached |
| 3 | `--require-complete` was selected and inspection was not known to be complete |

If several conditions apply, precedence is 1, then 3, then 2. Requesting diagnostics alone does not enable the completeness exit gate.

### What “complete” covers

Completeness applies only within the declared scope after Git ignore settings, default exclusions and the size limit.
A valid scope with no target files can complete; this says nothing about cryptography outside that scope.
`Complete` means the supported patterns or formats were inspected, not general symbol resolution, runtime use or discovery of every key format.

Distinguish file selection from completed analysis. `scanned_files` means files selected by the walker.
Diagnostics separately count selected, read-ok, read-failed, analyzed, partial, analysis-failed, unsupported, not-applicable and unknown files.
Enumerated policy exclusions and enumeration failures are separate; the ignore-hidden population remains unknown.

### Finding and work limits

Default accepted-hit limits are 4,096 per rule/file for regex and AST inspection,
and 8 for private-key markers. Regex counts hits after its language-specific
comment filter. Exactly reaching the cap does not prove truncation: an additional
eligible hit or unfinished search reports `detector_truncated`. Earlier findings
remain available, and omitted counts are not inferred.

Regex and AST each examine at most 65,536 match candidates per rule/file. AST also
limits semantic candidates to 262,144, scheduled tree nodes to 2,000,000 and
cumulative candidate text examined across rules to 256 MiB per file. Matching uses
full candidate text; clipping a displayed preview does not limit inspection.
Large numbers of accepted findings can increase report size, memory use and output time even within these limits. These are finite resource bounds, not performance guarantees. See the
[coverage guide](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/docs/user/COVERAGE.md#source-limits) for syntax and encoding limits.

### Parsing errors and unsupported formats

AST syntax errors retain available findings and yield `detector_partial`. Parser initialization or no-tree failures are detector errors.
Binary inputs, unsupported source languages and absence of applicable rules are outside that detector's scope.

For certificates, malformed input, unknown key attributes or mixed unsupported blocks yield `detector_partial`; unsupported containers yield `detector_unsupported`.
Detecting a container does not prove its contents were inspected. A parsed certificate that matches no rule need not produce a finding.

Dependency inspection distinguishes valid empty input, malformed supported input, partial success, unsupported versions and resource limits.
Supported structured subsets can report completion; unrelated files are not applicable. Go modules, Maven POM, requirements and Ruby declarations/locks can complete within their explicitly parsed subsets. Literal Gemfile/gemspec dependency calls use bounded Ruby syntax inspection, including selected local gemspec references. Dynamic or unsupported declarations remain incomplete; Gradle lexical fallbacks remain `detector_not_reported`.
Completion is not full package-manager validation or proof of installation.

The `--require-complete` exit-code contract is unchanged, but actual returned codes depend on parser outcomes and supported scope.
See [dependency declarations and resolution evidence](https://github.com/wakaken/pqc-scan/blob/11ff0b0c3e6f8c9a17148fe0551256f29dee74f0/docs/architecture/DEPENDENCY-PROVENANCE.md) for format-specific details.

## Including normally excluded binary formats

`scan` excludes known media/archive/compiled-data extensions and otherwise non-text binary inputs before selecting files. These exclusions are counted separately from unfinished checks. `--include-non-scannable` restores their inclusion for forensic/custom-detector runs; it does not add parsers for those formats and can increase “not applicable” counts. Existing directory, Markdown, Git-ignore and size policies still apply. Cryptographic container extensions are retained by default even when unsupported, so their inspection gaps remain visible. Full exclusion paths remain in diagnostic JSON, while human diagnostics show counts and at most three examples per reason.


### Reference positions

Explicit Python PQC imports are located at the module reference after `from `;
the evidence range includes imported symbols and continuation lines. Only regex
rules whose `scope` is exactly `config` omit leading whitespace from a nonempty
reference range. This is not a rule for every configuration file or every regex.
Whitespace-only custom matches retain their full range.

The exact built-in `TLS_VERSION_1_0` pattern/scope selects the protocol token and
reuses its trailing delimiter for an adjacent reference. Modified patterns/scopes
and unrelated custom IDs retain generic matching ranges and non-overlapping
iteration. A name reference does not establish effective configuration or a
negotiated protocol. Locations use original decoded text and character columns;
see [version-to-version identity changes](REPORTS.md#versions-and-identity).

### Java RSA, Maven and certificate facts

Java `RSAKeyGenParameterSpec` constructors are inspected for explicit
1024/2048/3072-bit RSA references. They do not prove an effective key size or
runtime use. Existing candidates are inspected before additional constructors
consume the remaining budgets; incomplete inspection stays explicit.
Rejected Maven dependency coordinates cannot change the context annotations of
earlier valid declarations.

Certificate parameter OIDs and named curves require the corresponding universal
ASN.1 type and encoding shape. Content bytes alone do not establish an OID, curve
or key-size fact. Uninterpreted or malformed attributes retain inspection limits;
no signature/trust validation follows. A NULL parameter has no parameter OID, so
the optional `parameters_oid` metadata entry is absent, rather than an empty
string. See the [candidate compatibility notes](../releases/v0.3.0.md).

## Local web scanning

```bash
pqc-scan serve --root /path/to/projects --rules-dir /path/to/pqc-scan/rules
```

The default browser opens the session's local page. Click folders or their checkboxes to select multiple folders, then **Scan selected folders**. Click again to deselect; review exact paths below the list, remove individual selections or clear them all. Double-click to open and select a folder; use its **›** button to browse without selecting it. Navigate with **Back**, **Forward** or the path above the list. Selection survives navigation; clicking the current path toggles selection of the current directory, including `--root`. Use Up/Down to move focus, Space to toggle selection, Enter to open, and Alt+Up to go to the parent. The startup boundary is separate from the selected folder; no folder is preselected when the page first opens. The modal dialog blocks other page actions and Escape throughout the queue, shows the folder and its position, and reports the current phase: finding files, inspecting files (processed/selected counts), aggregating results and writing reports. Counts include read/inspection failures and do not measure successful inspection or total elapsed-time progress. A single default-mode scan opens its HTML report automatically when generation succeeds; multiple scans finish on the results screen with individual results listed. Completed scans remain listed on the start page; **Back to results** in a served report restores that scan without running it again. The report shows incomplete inspection separately from successful report generation. Scanner errors leave a failed entry and allow retry; they are not presented as zero findings.

The picker groups selected folders and Clear selection together; test exclusion and the per-file size limit are grouped under Scan settings. Remaining session capacity is no longer displayed continuously; the 20-job limit still applies.

`--root` defaults to the current directory; the picker cannot leave that boundary and omits symbolic links. `--port` defaults to 8787; use `--port 0` for an available port. `--threads` defaults to 4. `--no-open` prints the URL without opening a browser. The picker UI offers Japanese and English. Reports inherit the current language and light/dark theme; changes in the report also carry back to the results screen. Theme preferences are applied before report content renders, including when browser preference storage is unavailable. Offline exports keep their language/theme selectors but do not include a live-session return link.

The server binds only to `127.0.0.1`. Open the full random session URL printed in the terminal; `localhost` aliases or a bare `/` URL are not accepted. Keep that URL private. No source file contents are served. Each scan uses the same scanner pipeline as the CLI and writes the seven `--format all` outputs plus `diagnostics.json`; file size and test exclusion are editable above, while rule sets, severity thresholds and network probing cannot be configured from this picker. One scan runs at a time, with at most 20 scans per server session. Folder listings show up to 1,000 subfolders and state when entries were omitted. Select at most 20 folders and fit the remaining session capacity (saved-result imports also consume a slot); the UI rejects an oversized selection before starting. Folders scan in selection order, including separate scans of overlapping parent/child selections. A failed scan stays listed and the next starts. A failed or uncertain start stops the remaining queue without automatically retrying the request. During connection loss, the modal remains open until the server status is available. Pending folders stay only in page memory: keep the page open until completion; reloading or closing abandons pending requests but does not cancel an already running scan or remove its server result.

Reports are retained in the private `pqc-scan-web-*` session directory printed in the terminal. Stop with Ctrl+C. To keep an offline report, retain both `report.html` and `report.rules.html`; exports are also available at the corresponding filenames under the report URL. This mode does not upload source or reports. It does not run project build scripts or package commands. A folder containing the session output directory cannot be selected for scanning.

### Optional migration context and reassessment

```console
pqc-scan serve --migration --root /path/to/projects --rules-dir /path/to/pqc-scan/rules
```

This opt-in mode records saved scan results for new web scans. On completion,
the results screen offers **Open scan report ↗** and **Review migration**
action. You can also choose **Load saved results** on the input screen and select
`scan.inventory.v1.json` (32MiB maximum). Files stay within the local session.
The folder picker supports multiple selections; each queued job uses one folder and keeps its own result.
Ordinary `serve` retains automatic scan-report navigation for one selected folder.
The migration review has three views: **Overview**, **Confirm use** and
**Response work items**. Overview counts and algorithm/location distributions come
from the selected saved results, for any scanned product. It does not repeat the
response report. **Next** opens common input; you may also proceed without context.

Scope proposals use observed purpose, exact role and folder/file hints. Select
matching scopes and, if needed, a first-level module. Test-folder labels describe
paths, not verified test-only use. The summary shows eligible records, excluded
ambiguous records and records already assigned elsewhere. **Next** applies the
answers to the displayed exact membership and generates the report; no extra
confirmation checkbox is required. Different uses remain separate systems in the
saved context. Scope size is not a quality or urgency score.

**Inspect records / split scope** opens the detailed selector, with algorithm/
filename search, role filters and20-row pages. At most4096 targets across256 uses
can be assigned; oversize selection is rejected rather than partly applied.
Targets that cannot be distinguished within the saved input cannot acquire declarations. Repeated source references with complete unique saved positions can be selected explicitly. **Edit individually** creates a
full per-target exception; later shared edits preserve it. **Restore shared context**
explicitly removes it. Groups are editing conveniences, not organization defaults.
Unsaved drafts are excluded and stay in page memory; reload loses them and the
editing groups. Save canonical context to retain declarations. Personal/organization
guidance does not change policy or criticality; company/department names are not required.

**Unanswered** omits a declaration; **Unknown** explicitly records that the situation
is not known. False and Out of scope are distinct. Choosing Out of scope hides other
migration questions and excludes those answers from generated context, while retaining
the draft for restoration. Purpose-dependent questions follow the same rule.

Choose a candidate algorithm and a key-size/configuration preset, or **Other (enter
manually)**. Presets name available standard parameter sets; they are not deployment
recommendations. Custom configuration validity is not verified. Support questions
refer to that purpose, algorithm and exact configuration. Changing their basis requires
resetting known support answers before saving. The selected configuration and its
provenance are retained in context and assessment evidence; declarations do not
establish implementation correctness, protocol compatibility or verified deployment.

Answers require an actor alias and assertion time. Inputs use Japan time (UTC+09:00)
and serialize whole seconds with an explicit offset; reviews show Japan time as well.
The current-time button is explicit, and expiry is optional and never generated.
Without expiry, freshness is unknown. Enter migration duration in years and months
with an explicit calendar anchor in Japan time. The UI adds the months to that date,
clamps month ends to the destination month and shows the exact day/second conversion.
It saves only the existing exact seconds (maximum 315,576,000); it never assumes
30 days per month. The initial anchor is the evaluation date; later evaluation-time
edits do not silently change a saved estimate. Imported seconds remain unchanged.
The separate goal month (`YYYY-MM`, years 0001–9999) is planning information,
without a day or timezone conversion. It does not set a policy/contractual deadline
or change urgency or readiness. Review the evaluation time separately. You may continue without
context; known concerns and missing information remain visible in the report.

**Use saved input** preserves every supported field, default, reference
and provenance (1MiB maximum). Imported context is read-only; edit the original file
and import it again to change it. Selecting another scan input resets bindings and
requires a new review. Existing context without a selected configuration retains its
previous generic support meaning. A configuration cannot silently adopt known support
from another declaration scope; such a mixed binding is rejected.

**Next** validates the canonical context against the exact saved input and starts
background generation using the real evaluator. No rescan or outbound connection
occurs. A spinner shows loading, evaluation, JSON/Markdown/HTML preparation and
saving; file counts are shown only during actual writes. These are processing
stages, not a guessed overall percentage. Evaluation cannot be cancelled after it
starts. Failure preserves input, shows no completed result and allows a new attempt.

Response work items combine decisions only when their system, observed purpose/
role, statuses, reasons, missing facts, actions, blockers and candidate support
semantics agree. Their number depends on the data, including zero. Next checks are
visible; **View evidence** opens exact records20 at a time with one selected detail.
The report does not infer migration completion from a group. **Edit use**
returns to the retained input. Language/theme remain available. **Save report (HTML)**
exports a self-contained bilingual HTML file with the current initial language/theme;
**Save input (JSON)** retains canonical input for CLI `assess` with the same saved bytes
and evaluation time. See [report navigation](SAVED-EVIDENCE.md) for offline limits.
Each output replacement is atomic individually, not a transaction across files.

The private job subdirectory migration/ retains the3 migration report files and,
when supplied, migration-context.json. Only completed allowlisted reports/context
are served. Imports count toward the existing20-job session limit; one context draft
per job is bounded, with a3MiB request and16MiB response limit. The ordinary serve
mode and scan/report defaults retain their behavior. Existing context inputs retain
their meaning; migration context and assessment schema versions remain1. The
approved optional planning-month claim extends their accepted fields. Original
practitioner evaluation and release approval are separate from this workflow.


The input method choices are **New input** and **Use saved input**. Choose scope,
purpose and usage first; candidate/configuration/support prerequisites appear where
needed. Unanswered declarations can inherit defaults; explicit unknown can override
an inherited known value. Neither state alone determines urgency. Scope exclusion
records a declaration, not deletion or proof of safety; technical concerns remain.
Location-filtered categories and counts use the exact server scope, excluding
ambiguous and already assigned records from eligible counts. A saved use can be
deleted with its individual overrides and restored with **Undo** while this page
remains open. Undo refuses conflicting reassignment instead of overwriting another
use. Edits, deletion and undo invalidate the current generated view, without deleting
existing artifact files. Download a new HTML report and input JSON after changes.
The session result list is newest first and shows project/import name, request time
in Japan time and status; it is not persistent multi-session history. Unknown
historical names/times are labelled rather than invented. Scan reports open in a
separate tab carrying the current language/theme, preserving this page's inputs.


Use Cancel remaining scans in the progress dialog to stop all unsubmitted folders. The active folder finishes and its result remains in Scan history. Migration loading and preparation also show a blocking dialog. Opening Response work items does not generate anything; use Prepare review items explicitly. Evaluation time appears beside the input method; optional declaration expiry is folded into details. Explicitly choosing a duration or goal month clears the other field. Existing imported formats and meanings are preserved.

Omitting declaration expiry leaves purpose/support claims with unknown validity. Entered declarations and currently usable assessment claims are distinct; supply the intended expiry explicitly.


Migration HTML separates observed purpose/algorithm from current declarations of purpose, usage and proposed configuration; declarations alone do not verify deployed cryptography. Concrete missing facts, locations and next checks precede the full original assessment reasoning/evidence. Evaluation time uses Japan time. Search appears for multiple work items and matches algorithms, purposes, filenames and related text (e.g. RSA, encryption, package.json). A single urgency category has no dropdown. Undetermined urgency does not mean low risk. JSON decisions, declarations and UTC timestamps remain unchanged.

### Follow-up report usability

The optional **Organization or contractual deadline** section accepts the actual scoped deadline, its organization-policy/contract basis, and non-secret issuer/document/edition/clause identifiers. It uses the existing context reference schema; it is separate from goal month and can affect urgency together with lead time. No deadline, omitted answer and explicit uncertainty remain distinct. Source records that cannot be uniquely bound are disabled with an explanation: adding normal answers cannot resolve that scanner limitation.

Migration work items show applicable declared deadlines in Japan time and lead time in days/hours, retaining exact UTC/seconds in JSON. Record counts and distinct recorded locations are separate; repeated observations are not independent uses or vulnerabilities. Evidence lists identify each observation method. Web reports carry the selected relative scope as a display label. CLI `assess --report-label "Project / scope"` can name an HTML report (non-secret text, at most 256 UTF-8 bytes, no control characters). This display metadata and known binding limitations do not alter canonical assessment JSON.

### Exclude test code

Tests are included by default because they can reveal supported algorithms and compatibility requirements. Keep them for a full inventory; use `--exclude-tests` to focus migration review on product files. In the browser, select “Exclude test code” below “Clear selection”. The setting is captured at scan start for every folder in the sequential batch.

```bash
pqc-scan scan . --exclude-tests --diagnostics --out-dir /path/to/reports/product-only --rules-dir /path/to/pqc-scan/rules
```

Matching is case-sensitive and relative to the selected root. Files below `test/`, `tests/`, `__test__/`, `__tests__/`, `spec/`, `specs/`, `testsuite/`, `test-suite/` or `test-framework/`, and filenames `test_*.py`, `*_test.py`, `*_test.go`, `*_test.rb`, `*_spec.rb`, `*.test.{js,jsx,ts,tsx,mjs,cjs,mts,cts}`, `*.spec.{js,jsx,ts,tsx,mjs,cjs,mts,cts}`, `*Test.java` and `*Tests.java` are excluded. Ancestor names outside the root do not affect selection; the role of the root itself is not inferred.

The standalone names `tests.py` and `tests.rs` are also excluded. For syntactically valid Rust, exact source matches entirely inside explicit `#[cfg(test)]` items or `#[test]` functions are omitted. Mixed `cfg(any(test, ...))`, custom macros, functions where imports/globs or unresolved attribute macros may override or remove `test`, and matches with unresolved positions or normalized/masked text remain. Nested `#[cfg(test)]` regions under an unresolved outer attribute macro are also retained. Mixed files are still analyzed, so their file counts do not decrease; surrounding source snippets can include adjacent tests. Unusual test names may remain, and exclusion does not establish production use of every remaining finding. Root dependency manifests remain selected. Applied exclusions are recorded in the inspection scope, diagnostics.json and native inventory. Do not directly compare totals from different scopes.

### Custom excluded paths

Place a UTF-8 `.pqcignore` at the selected directory root to apply Gitignore-style patterns in both CLI and browser scans. For example, omit custom validation files while retaining one control:

```gitignore
/integration-fixtures/*
!/integration-fixtures/production-control.rs
```

Only the selected root's policy is loaded; ancestor and nested `.pqcignore` files are not inherited. The policy file itself is excluded. Invalid patterns/encoding or a read failure stop the scan with a generic error; policy text is not exported. Symlink policies are not followed. The existing inspection scope records that custom exclusions applied, and enumerated excluded files are counted. Negations cannot restore Git-ignored files, default exclusions or `--exclude-tests` matches. Excluding a mixed file by path also excludes its product code.

Default directory exclusions such as `target` and `docs` are relative to the selected root. Names in ancestors outside that root do not discard the selected project.
