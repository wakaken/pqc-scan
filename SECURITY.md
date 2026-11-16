# Security policy

[English](SECURITY.md) | [日本語](SECURITY.ja.md)

The v0.2 release targets static scanning with optional explicitly invoked network
probes. The latest verified 0.2.x snapshot receives best-effort maintenance. No
response SLA or old-version backport guarantee is declared. Optional TLS/SSH
probes remain experimental. Unsupported inputs and incomplete observations must
not be interpreted as a clean security assessment.

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

Local packaging alone does not establish publication or authenticity. Hosted static-candidate archives have GitHub build-provenance attestations; verify the repository, workflow and exact source commit using the distribution guide.
SHA-256 manifests detect changes relative to supplied hashes; they do not prove
authenticity. Follow the [installation and update guide](docs/user/DISTRIBUTION.md)
or [日本語の導入・更新手順](docs/user/DISTRIBUTION.ja.md). Keep binary and rule
versions together and retain the previous verified bundle for rollback.
