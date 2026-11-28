# Security policy

[English](SECURITY.md) | [日本語](SECURITY.ja.md)

This checkout prepares the unpublished v0.5.0 candidate. Its intended binary
scope remains macOS Apple Silicon static scanning and the network variant's TLS,
SSH advertisement and pinned hybrid SSH KEX diagnostics. See the
[candidate release notes](docs/releases/v0.5.0.md). Local preparation does not
establish successful final-source hosted checks, publication or release approval.
The recorded best-effort maintenance for verified 0.2.x/0.3.x snapshots and v0.4
remains unchanged. No response SLA or old-version backport guarantee is declared.
Unsupported inputs and incomplete observations must not be interpreted as a clean
security assessment.

## Reporting a vulnerability

Use [GitHub private vulnerability reporting](https://github.com/wakaken/pqc-scan/security/advisories)
and select **Report a vulnerability**. Private reporting was enabled and verified
on 2026-11-16. Do not post exploit details, private source, credentials, keys or
sensitive reports in a public issue. If you cannot access private reporting,
open an issue asking only for help reaching the private channel; wait for a
verified channel before sharing sensitive details.

Include the affected revision/features, a minimal nonsecret reproduction,
expected/actual behavior and impact. Use synthetic keys and endpoints. Redact
reports before sharing: source and endpoint metadata can remain sensitive even
when recognized key material is masked.

## Responsibility and response

The repository maintainer is the primary vulnerability-report responder. This is
a solo-maintainer project with no backup responder. Reports may be delayed during
the maintainer's absence and will be checked in order after their return.
Immediate responses and fixed response or resolution deadlines are not guaranteed.

The maintainer reviews the report, assesses its impact and applicability, and
coordinates investigation and a fix through the private reporting channel.
Normally, the fix is validated and a corrected version is prepared before public
disclosure. When there is an external reporter, disclosure timing is discussed
with them. Any warning needed before a fix is decided individually.

Security fixes are announced through both GitHub Security Advisories and release
notes, identifying affected versions, the corrected version and update
instructions. This procedure does not promise a fix or release by a particular
date.

## Dependency vulnerability monitoring

The dependency-check workflow is configured to run weekly on Mondays at 09:23
Japan time (00:23 UTC), on pull requests changing the workspace dependency
manifests or root Cargo.lock, and after those changes reach develop. GitHub
schedules may be delayed; scheduled runs start only after this workflow is on the
default branch. It can also be run manually from GitHub Actions.

It checks all packages in the root Cargo.lock, including locked optional and
build/development dependencies, with pinned cargo-audit and a newly fetched
RustSec advisory database. It does not build or execute the project. Separate
example/fixture lockfiles, unknown vulnerabilities and flaws in project code are
outside this check. A clean result is not proof of security.

Known vulnerability findings and an incomplete/failed check make the workflow
fail. Informational warnings remain in the report for review. Default-branch
failures create or update a GitHub Issue assigned to the repository maintainer;
the notification contains only a link to the run. PRs show the check result and
do not create Issues. Notification delivery depends on GitHub availability and
the recipient's notification preferences. Detailed reports are retained as Actions
artifacts for 30 days; private vulnerability details must not be posted in Issues.

The maintainer checks applicability, prepares an update or other appropriate
response, and reruns the check and affected tests. The notification Issue is
closed after verification. Fixes are not merged or published automatically, and
no fixed response deadline is promised. Workflow configuration alone does not
establish a history of successful scheduled runs or delivered notifications.

## Distribution and updates

Local packaging alone does not establish publication or authenticity. The
[release-preparation workflow](https://github.com/wakaken/pqc-scan/blob/develop/.github/workflows/release.yml) produces build-provenance
attestations for both static and network candidates. Verify the repository,
workflow and exact source commit alongside each archive digest before publication.
SHA-256 manifests detect changes relative to supplied hashes; they do not prove
authenticity. Follow the [installation and update guide](docs/user/DISTRIBUTION.md)
or [日本語の導入・更新手順](docs/user/DISTRIBUTION.ja.md). Keep binary and rule
versions together and retain the previous verified bundle for rollback.

## Codex Security assessment context

This section defines repository-wide assessment context for Codex Security.
It supplements the reporting and distribution guidance above; it does not
authorize execution, network access, disclosure or remediation.

### System and Scope

This checkout contains candidate product version 0.5.0. Assess the selected source revision and enabled
features rather than assuming that a published release represents this checkout.

pqc-scan is a local Rust CLI and library for advisory inspection of cryptographic
usage and PQC migration risks. Static scanning is the primary product surface.
Explicit, feature-enabled TLS, SSH-offer and SSH hybrid-KEX probes are separate
network operations within the documented macOS Apple Silicon scope.

Assess relevant boundaries across `crates/`, `rules/`, report generation,
`scripts/`, packaging and `.github/workflows/`. Include developer tools, tests
and fixtures when they can expose product behavior or cross a host, artifact or
CI authority boundary; their directory names alone do not exclude them.

The repository threat-model references are:

- [Repository threat model (English)](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/security/threat-model.md)
- [脅威モデル（日本語訳）](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/security/threat-model.ja.md)

These are dated reference snapshots with their original source revision,
worktree digest, findings and verification limits preserved. Recheck changed
boundaries against current source. A historical observation, proposed mitigation
or documented control is not proof of current behavior, a completed test or an
accepted risk. This root policy defines assessment expectations; the model
supplies architecture and threat context. Identify material disagreements rather
than using them to silently suppress findings.

### Threat Model and Trust Boundaries

Protect host files outside the intended input/output authority, private source
and credentials, report and assessment integrity, scanner/CI availability, probe
peer identity and build/distribution provenance.

Treat scanned checkouts, paths, manifests, public containers, imported inventory
and explicitly contacted peers as potentially attacker-controlled. Do not assume
that an attacker already controls the operator account, trusted executable,
external rule pack, public CA/host-key trust inputs or release authority.
Operator-selected roots, rules, output paths, CIDRs and trust inputs define the
authorized operation; they do not grant checkout content additional host rights.

The ordinary CLI runs with its invoking account's authority. Rust crate
boundaries are not sandboxes. Do not infer a public service, tenants or remote
job submission from this local CLI; state any additional deployment prerequisites.

### Security Invariants

- Static inspection must not execute scanned source, evaluate templates, resolve
  dependencies over the network or automatically contact discovered endpoints.
- Repository and local-parent reads must stay within selected, bounded regular
  files and the intended root, including link replacement and file-growth paths.
  Output writes must likewise preserve the intended destination authority across
  file/parent links and races. Reader hardening does not establish writer safety.
- Preserve the documented masking of recognized private-key and credential
  syntax across applicable evidence and export fields. Keep diagnostics safe.
  The [privacy contract](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/user/COVERAGE.md#privacy) is bounded; do not infer
  universal secret detection or that redacted reports are safe for all recipients.
- Render source-derived and peer-derived text as data in HTML, Markdown and
  machine reports, with encoding appropriate to each context.
- Apply finite applicable parsing, traversal, collection, traffic and deadline
  budgets. Keep malformed, unsupported, truncated, failed and unknown inspection
  distinct from complete inspection. Per-file caps are not whole-run guarantees.
- Explicit probes must authorize the numeric destination actually connected,
  preserve peer/budget binding and respect cancellation. TLS verification must
  retain the selected chain/name checks. SSH offers remain unauthenticated claims;
  verified SSH KEX requires the separately trusted host pin and signature path.
  Do not imply login, exhaustive server support or completed PQC migration.
- Preserve observation identity, scope and provenance across inventories, diffs
  and advisory policy. Missing detections or records and operator-declared support
  are not proof of remediation, runtime use or deployment safety.
- Bound archive/import handling and reject unsafe destinations and member types.
  Distinguish checksums from authenticity, supplied binaries from independent
  rebuilds, and preparation/provenance authority from release publication.
- Treat report upload and sharing as separate disclosure boundaries. Account for
  actual contents, recipients and effective CI permissions before claiming impact.

### Reportable Findings and Severity Context

Report source-backed violations that give a realistic attacker a meaningful
capability beyond their initial authority: unintended host reads/writes, protected
secret exposure, active-content injection, exploitable parser/resource failures,
probe destination or authentication bypass, assessment-integrity failures or
misuse of build/artifact authority.

For each finding, establish attacker control, entry point, dataflow, broken
control, sensitive operation, prerequisites, effective mitigations and concrete
impact. Retain the strongest counterevidence and material uncertainty. Static
evidence can establish a finding without a deployed instance or runtime PoC;
do not claim that an unexecuted test or hypothetical deployment was verified.

Calibrate severity using demonstrated impact and likelihood, not the cryptographic
algorithm name or a fixed mapping from a directory or CWE. Reserve Critical for
clear, immediately actionable severe compromise. High impact with high likelihood
can warrant High; with medium/unknown likelihood it generally warrants Medium,
and with low likelihood Low. Medium/unknown impact generally warrants Medium
only with high likelihood, otherwise Low. Low impact remains Low. Explain how
local-only exposure, required operator actions and actual containment affect the
rating; keep confidence separate from severity.

A weak-algorithm or private-key-marker fixture is not by itself a vulnerability
in pqc-scan. Ordinary authorized behavior and same-authority, self-only effects
without meaningful gain are not findings. Conversely, a test/example filename,
an experimental feature or absence of runtime reproduction does not invalidate
a supported boundary violation. Triage suspected embedded credentials from their
format, context and useful access beyond source reading; never use them to check
validity and never reproduce secret material in a report.

### Out of Scope, Exclusions, and Accepted Risk

No new blanket component exclusion or accepted-risk exception is established by
this policy. A known finding remains eligible for reporting; an unresolved issue,
usage recommendation or proposed fix is not acceptance or remediation.

### Known Limitations and Compensating Controls

Detection coverage, secret masking and optional probes have documented limits.
Zero findings do not prove safety or exhaustive inspection. Preserve partial
coverage, missing measurements and unexecuted platform/runtime checks as such.
The linked models retain historical partial coverage and the output-destination
finding observed in their original snapshot. Later scan reports must state their own source
revision, reviewed surfaces and verification limits. A policy or threat-model
update does not close a finding or establish that unreviewed code is safe.

Current Unix report writers use held directory handles, no-follow traversal and
exclusive staging followed by atomic replacement. They do not truncate existing
linked inodes. See [output destination safety](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/user/SCANNING.md#output-destination-safety)
for link rejection, private file modes, supported platforms and concurrency limits.
A fresh, trusted output directory remains recommended; this does not make an
attacker-writable destination tamper-proof. Historical models/findings remain
unchanged, and a later security assessment must verify its own exact source.
