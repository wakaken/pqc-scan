# Task 038: read-only distribution inventory

Date: 2026-11-10. This is independent preparation while protocol and browser
acceptance gates remain open. Task038 is not implemented or complete; release
channels, support window and supported package variants remain maintainer choices.

## Observed channels and reporting configuration

Read-only GitHub API calls exited0:

```text
gh api repos/wakaken/pqc-scan/releases
gh api repos/wakaken/pqc-scan/tags
gh api repos/wakaken/pqc-scan
gh api repos/wakaken/pqc-scan/private-vulnerability-reporting
```

At this checkpoint, releases and tags were empty arrays. Repository metadata says
public, default branch `develop`, MIT license, issues enabled, no configured
homepage, not archived. Private vulnerability reporting was **disabled**. Raw
selected metadata is in ignored `target/task-038/{releases,tags,repository,private-reporting}.json`.
No setting, tag, release, issue, message or other remote state was changed.

The existing documented installation channel is a local checkout plus
`cargo install --locked --path crates/pqc_scan_cli`; users retain the checkout's
rules and pass an absolute `--rules-dir` outside it. No release workflow or
published binary channel was found in the reviewed checkout. crates.io API pages
could not be accessed by the web tool; publication there remains **unverified**,
not assumed absent. Package names or search hits alone would not establish project
ownership. Root package version remains0.1.0; no1.0 release is declared.

## Concrete packaging choices for later review

Recommended initial preparation: retain the source-checkout channel and prepare
local, deterministic GitHub-style archives for only measured OS/architecture and
explicit static/TLS/SSH feature variants. Do not create or publish a release.
Keep crates.io publication out of the initial plan until package naming and
versioned cross-crate dependencies are deliberately selected.

Each archive should contain the exact binary, reviewed rule assets, MIT/project
and required third-party license notices, English/Japanese installation and
upgrade instructions, source/toolchain/feature provenance and SHA-256 manifests.
Keep current explicit `--rules-dir` behavior; do not introduce implicit project
or environment rule discovery as a packaging side effect. A clean install must
scan a separate local fixture offline using its installed rule path.

Measure archive repeatability and clean rebuild hashes separately. Normalize
archive paths/metadata/time and record the actual binary hash; identical archives
of an identical binary do not prove cross-host binary reproducibility. Never
claim support for unbuilt OS/architectures. A prepared workflow should be manual,
read-only for repository contents, artifact-only, and contain no publication or
signing credential setup.

Recommended support-policy candidate: the latest stable release only, best-effort
maintenance, no invented SLA or promise of fixes for old releases. This is a
proposal, not an adopted maintainer obligation. A supported release/version and
window must be selected before writing a final public policy.

## Security reporting decision still required

Do not document GitHub's private vulnerability flow as enabled while the API says
otherwise, and do not invent a maintainer email. Select a private reporting contact
or separately authorize enabling the repository feature before declaring that
channel available. An interim public issue can request a private contact without
posting exploit details, source secrets or keys; no such issue is sent by this
work. Actual SECURITY.md and release/support guarantees await that decision.

Dependencies032–036 and task038's channel/support decision must pass before its
acceptance. Task037 follows038 and still needs maintainer-approved release scope,
numeric regression budgets and actual supported-platform/current-revision CI
evidence. Task039 remains a separately selected use-case study, not an initial
SSH-release requirement.
