# v0.4 experimental network distribution readiness

Status at this checkpoint: verification in progress; publication and risk acceptance remain pending.
The maintainer requested a README checkpoint, then remaining work toward v0.4
publication, including probes if publishable. README checkpoint
`7e8cd91426fa21609a91c0511de44252eaa890d4` is pushed. Preserve the
[earlier static readiness record](../v0.4-readiness-20261119/README.md) as historical
source-bound evidence, not current results.

## Scope and compatibility

Prepare `static` and experimental `network` archives for macOS Apple Silicon
(`aarch64-apple-darwin`). Network includes explicit TLS, SSH offer collection and
pinned hybrid SSH KEX. No Linux/other-platform binary support or crates.io scope
is added. Probe targets remain explicit and authorized; trust/pin verification,
traffic/deadline bounds, no-authentication behavior, report schemas and exit codes
remain unchanged. Hosted `protocols` remains disabled; a skip is not a pass.

Unix SIGINT delivery now uses signal-hook 0.4.6 on a worker, calling the unchanged
Cancellation token/socket shutdown path outside the asynchronous signal handler.
Non-Unix retains ctrlc 3.5.1. Native preflight has 300 network packages, no missing
notices and no review-required flags. The four historical Objective-C entries and
their flags remain intact; they are absent from this selected Unix graph. This
resolves their selection in this distribution, not their licensing for other graphs.
New registry notices are hash-checked by the existing packager.

## Verification gates

Required final-source checks include default/all-feature tests, strict Clippy,
builds, new actual-process SIGINT/closed-socket regressions, distribution and
quality checks, existing confined loopback-only protocol suite with previews and
OpenSSH pin/signature/no-overlap/no-authentication cases, dependency review,
repeatable archives and offline installs for both variants. Existing implementation
and test reviewer identities continue; no replacement reviewers are introduced.

Both variants require final clean-commit hosted preparation, exact-source build
attestations and verification of the actual downloaded archives. A version label,
local package or successful static step alone is insufficient. Preparation does
not create a public GitHub Release. Final source/run/archive/checksum/review evidence
is retained under `target/v04-publication-20261124/` and the corresponding Actions
artifacts; passing interim checks do not close remaining gates.

## Dependency maintenance decision still required

The 2026-11-24 audit with cargo-audit 0.22.2 and RustSec DB
`52384ab5581a4487b62c549eef95cf7ba7e8bad5` found zero vulnerability entries and one
unmaintained warning: [RUSTSEC-2026-0543](https://rustsec.org/advisories/RUSTSEC-2026-0543.html),
for chrono 0.4.44 in both variants. It has no patched version. The
[upstream notice](https://github.com/chronotope/chrono/issues/1768) describes winding
down maintenance. This is a maintenance warning, not a vulnerability-free guarantee.

Chrono is used for collection/report timestamps, RFC3339 input normalization,
assessment time/deadline handling, certificate time conversion and local display.
It also appears in public Rust data types. Removing it is not a version bump;
it needs separate API/output/time-boundary compatibility analysis. The audit
finding remains visible with `accepted: false`; no ignore or exception was added.

Proposed decision for the final candidate: retain chrono for v0.4 only with a
maintainer-approved exception expiring on 2027-01-23 (60 days from this record),
recheck advisories before publication, and re-evaluate earlier if a concrete
vulnerability is reported. This is a proposal, not acceptance, a created follow-up
task or an automatic release authorization. If the maintainer declines, keep
publication pending until a compatible replacement is implemented and verified.

Final maintainer product/support/publication acceptance must name the candidate
and both actual archives, acknowledge bounded experimental probe support and
resolve this warning explicitly. No public tag, merge or release has been created.

## Subsequent maintainer direction — supported probe scope

After reviewing the successful technical verification, the maintainer requested
that probes become supported v0.4 features if all verification was complete and
there were no remaining technical release blockers. The implementation/test
reviewers accepted source `4e800e811cb7c60b2b377796e69bffc6326be1c8` and both
actual hosted archives. The exact-source CI/preparation receipts are
[runs 45304498763](https://github.com/wakaken/pqc-scan/actions/runs/45304498763) and
[45304503424](https://github.com/wakaken/pqc-scan/actions/runs/45304503424).

On that basis, current README, installation/probe guides and v0.4 notes describe
TLS, unauthenticated SSH advertisements and pinned hybrid SSH key exchange as
supported features in the macOS Apple Silicon network distribution. This changes
the product designation, not algorithms, supported platforms, authentication,
trust verification, command behavior or output contracts. The earlier experimental
checkpoint above remains historical. The chrono maintenance-warning decision and
actual publication authorization remain pending. Documentation changes after this
source require refreshed archive/source binding; the old archives are not the new
publication assets. No old measurements or audit exceptions are rewritten.
