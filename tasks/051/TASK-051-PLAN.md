# Task 051 execution plan

Baseline: `f07d782cd68cfac95d93585d5ace049f590160d4`, fetched from
`origin/develop` on 2026-11-14. Clean worktree; new branch
`codex/task051-dependency-supply-chain-review`.

## Outcome and original criteria

Q6-03 records exact dependency versions, resolved features, sources, RustSec
database retrieval time/hash, vulnerabilities, maintenance advisories, exception
expiry and actions. Q6-04 maps all distribution variants to licenses and notices;
only the maintainer may accept unresolved publication issues. Required primary
outputs remain `target/quality-task-051/dependency-review.json` and
`target/quality-task-051/license-review.md`. Retain durable copies and raw evidence
under `tasks/051/evidence/`.

The current package script and release workflow declare `static`,
`combined-probes` and `all-probes`, all targeting `aarch64-apple-darwin`.
Other platforms/build reproducibility belong to 059/060. E41 did not execute a
current RustSec audit. ED and ER provide reusable notice validators and historical
five review flags, not a current dependency assessment.

## Approach, compatibility and cost

Reuse package-release.py's notice collection and exact-version fallback checks.
Add an offline evaluation script, rather than altering scanner output, package
schemas, dependencies or CI gates. Evaluate the complete lockfile with a pinned
cargo-audit binary and an explicitly acquired RustSec Git snapshot; then map its
findings to each variant's resolved normal/build dependency closure. Retain
lockfile-only findings separately. A missing advisory is not proof of maintenance
or safety; an unknown/malformed audit must never become a clean result.

Use the existing immutable Linux tool/cache image for bounded, offline execution
with read-only source, empty allowlisted environment and scratch-only writes.
Cargo metadata filters the declared macOS target; no cross-platform binary build
or protocol execution is claimed. Trusted preparation downloads only the official
fixed audit binary and advisory DB, with hashes/timestamps. No paid service,
publication, remote configuration, signing or system-wide installation is needed.
Expected local cost: minutes to tens of minutes, bounded CPU/memory/disk/time.

## Steps and checks

1. Record source/tool/DB identity and acquisition evidence, including actual DB
   commit time rather than assuming its freshness from the client date.
2. Implement evaluation and synthetic tests: normal/build vs dev reachability,
   cycles/shared nodes, source/version matching, clean/vulnerable/unmaintained
   audit data, failed/malformed audit, missing/tampered notices and retained flags.
3. Run initial Python checks and real offline evaluation; inspect combined diff.
4. Obtain one isolated implementation review, address findings, then finish the
   applicable distribution/script checks and obtain one isolated test review.
   Record and reuse exactly those two reviewer identities for material fixes.
5. Bind final source, inputs, commands, exits and artifacts by SHA-256. Apply the
   original completion gates; keep unfinished any required maintainer decision
   or missing validation. Preserve failures alongside successful reruns.

No exception is pre-approved. Findings will carry proposed actions, an explicit
unapproved decision and no invented expiry; a release-time acceptance requires a
maintainer decision with a real expiry/recheck condition. The five existing notice
flags must remain unless exact-version evidence resolves them.
