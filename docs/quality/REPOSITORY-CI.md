# Repository CI

The `pqc-scan` workflow runs automatically for pull requests and pushes to
`develop`. Working-branch and tag pushes do not start it; updating an open PR
still triggers PR validation.

| Event | Jobs |
|---|---|
| Pull request | `quality` |
| Push to `develop` | `scan` |
| Manual execution | `quality`, `scan` |

On `develop`, `scan` runs independently of `quality`; PR validation provides the
pre-merge quality check. Manual execution runs both enabled jobs. The `baseline`
job is disabled for all triggers while performance measurement remains held.

The CI self-scan reads a separate product input: `crates/*/src/**`, Cargo manifests
and the root lockfile, build scripts when present, and `rules/default/*.yml`.
Standalone tests, examples, task evidence, CI scripts and documentation are outside
this scan. Source bytes and repository-relative locations are preserved; colocated
Rust unit tests remain in their source files. This scope applies to repository CI.

The summary separates results requiring review from exact reviewed references.
The `pqc-self-scan-evidence` artifact contains the input inventory/hashes, scoped
expectations, all results for that scope, upload results, diagnostics and summary.
New or changed results still appear for review. These counts describe detected
references, not confirmed vulnerabilities.

The [self-scan expectations](self-scan/SELF-SCAN-EXPECTATIONS.json) identify exact
reviewed references using source hashes, full-result hashes, locations, and counts.
Only matching results are omitted from the upload copy; unfiltered results remain
in the artifact. Changed or unmatched observations remain available for review.
Policy updates require source and result review, rather than blanket registration
of current findings. The dated directories under `self-scan/` contain data used
by regression tests, including source snapshots and before/after SARIF.

The `quality` job aggregates all six quality axes offline from recorded evidence. Missing facts and stale measurements remain
visible; aggregation does not run the high-cost studies or grant release acceptance.
Integrated reports are uploaded even after a failed step, with 30-day retention.
See the [measurement and retention plan](../../docs/quality/CI-MEASUREMENT-PLAN.md).

It skips eligible pushes and pull requests whose changed paths all
match its documentation list: the root README files, `AGENTS.md`, `SECURITY.md`,
Markdown under `docs/` and `prompts/`, and `tasks/README.md`.
Other changes in eligible events, including mixed documentation/code changes,
rules, workflow files and JSON data under `docs/`, still run CI. The exact list is maintained in
[the workflow](../../.github/workflows/pqc-scan.yml). Run it manually from GitHub Actions
when validation is needed for an otherwise skipped change.

The `protocols` job is disabled for every trigger, including manual runs. Its
`if: ${{ false }}` condition remains in place. The generic isolated suite's
read-only-target write failure has been corrected and its tests, Clippy, previews
and SSH KEX completed in the [candidate validation](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/releases/static-candidate-20261117/AUDIT-FIXES.md). This is separate evidence from the dedicated protocol049 path and does not resume the hosted job. Re-enabling it requires a maintainer decision and validation of its current inputs, environment and resource limits; the enabled jobs do not establish a protocol pass.

GitHub limits the number of changed files considered
by path filtering; for large changes, manually run CI if it was omitted.

Before making these checks required for merging, replace workflow-level path
filtering with job-level gating so skipped documentation changes do not leave
required checks pending.
