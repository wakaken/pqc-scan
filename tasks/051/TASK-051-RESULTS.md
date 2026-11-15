# Task 051 dependency and license assessment

The evaluation is complete against Q6-03/Q6-04 and both independent review stages
have accepted the final source and evidence. Dependency quality has findings
requiring action. This record grants no publication approval
and accepts no unresolved finding or license/source issue.

Baseline/current HEAD: `f07d782cd68cfac95d93585d5ace049f590160d4`, plus
the task-051 worktree changes on `codex/task051-dependency-supply-chain-review`.
Exact executed files are bound in [the final run](evidence/dependency-20261114/final-run/RUN.json).
The added scripts evaluate dependency metadata and notices; scanner behavior,
Cargo.lock, rules, package formats, CI gates and release policy are unchanged.

## Required outputs and scope

Primary outputs are `target/quality-task-051/dependency-review.json` and
`target/quality-task-051/license-review.md`. They are byte-identical copies of the
durable [JSON](evidence/dependency-20261114/final-run/artifacts/review/dependency-review.json)
and [license review](evidence/dependency-20261114/final-run/artifacts/review/license-review.md).
Relative notice links resolve in both locations. The durable run retains the
normal/build closure, resolved features, source/checksum/VCS metadata, declared
licenses, exact notice bytes, raw metadata/audit, commands and exit statuses.

| Declared macOS arm64 variant | Packages, including workspace crates | Existing notice review flags | Missing collected notices |
|---|---:|---:|---:|
| static | 139 | 0 | 0 |
| combined-probes | 168 | 4 | 0 |
| all-probes | 271 | 5 | 0 |

These are the three variants in package-release.py and release.yml, targeting
`aarch64-apple-darwin`. Cargo metadata ran on Linux with an explicit macOS target
filter, using the same normal/build traversal as packaging. These counts are not
exact binary linkage or new cross-platform build/install certification.

Registry archive SHA-256 values match Cargo.lock. The normalized Cargo manifests,
VCS records when present, and the complete set and bytes of collected notice
files match those archives. Packages without archived notice files use the
existing exact-version fallback validator, including upstream revision and byte
hash checks. No registry-cache checksum metadata was invented.

## Q6-03: tool, DB and findings

The official cargo-audit 0.22.2 Linux aarch64 release asset was downloaded on
2026-11-14 and checked against its GitHub asset SHA-256:
`c6603814ddaa45e51263dafd31c0ac98808f688d26f7395804f9670b0fd599dd`.
The extracted binary hash, URL and retrieval time are in the
[tool receipt](evidence/dependency-20261114/preparation/tool.json).

The RustSec DB was acquired on 2026-11-14 at commit
`4ef2df8e806a65e2f1ecc14f56f51249062b23d6`, whose commit time is
2026-11-13T13:33:03+01:00. The
[database receipt](evidence/dependency-20261114/preparation/database.json)
records retrieval time, per-file hashes and the canonical content-manifest hash.
The actual snapshot, including Git identity, is retained in
[advisory-db.tar.gz](evidence/dependency-20261114/preparation/advisory-db.tar.gz).
Raw cargo-audit reported null Git metadata; the separate verified receipt supplies
identity rather than substituting an assumed date. `--stale` permits reproduction
of this explicitly dated snapshot; a fresh acquisition is required before release.

The complete 348-package lockfile audit returned exit 1 because it found one
vulnerability, plus one unmaintained and two unsound warnings. The evaluator
returned exit 0 because assessment succeeded and retained all four findings.
Tool/assessment success does not mean dependency quality passed.

| Advisory / exact dependency | Variant mapping | Proposed action; no exception granted |
|---|---|---|
| [RUSTSEC-2026-0204](https://rustsec.org/advisories/RUSTSEC-2026-0204.html), crossbeam-epoch 0.9.18 | All three | Review affected pointer-formatting use and dependency paths; propose a compatible update to a patched range (`>=0.9.20`), then rerun applicable checks. |
| [RUSTSEC-2026-0190](https://rustsec.org/advisories/RUSTSEC-2026-0190.html), anyhow 1.0.102 | All three | Review context/downcast_mut use; propose a compatible patched update (`>=1.0.103`) and verify error-handling behavior. |
| [RUSTSEC-2024-0449](https://rustsec.org/advisories/RUSTSEC-2024-0449.html), unsafe-libyaml 0.2.11 | All three | No patched range is reported. Evaluate replacement of the YAML parser chain with rule/config compatibility tests; no parser change is authorized by this assessment. |
| [RUSTSEC-2026-0386](https://rustsec.org/advisories/RUSTSEC-2026-0386.html), tokio 1.49.0 | all-probes | Review AsyncFd use in the dependency chain; evaluate compatible patched ranges (`>=1.51.6, <1.52.0` or `>=1.53.3`) and repeat protocol/feature/package checks. |

These are exact version matches against the fixed DB, without target-based
advisory suppression. Runtime reachability/exploitability is not established.
No finding was ignored, downgraded, fixed or accepted. Every JSON finding has
`accepted=false`, no maintainer decision and no exception expiry. A future
exception must explicitly identify a maintainer, rationale, scope and expiry.

Recheck before publication, after dependency/feature/target/notice changes and
monthly during release preparation. This is a documented procedure, not a new
scheduled job. Offline execution disables registry yanked checks. Absence of a
RustSec advisory does not establish that another dependency is safe or maintained.

## Q6-04: license and source decisions

Every selected package maps to declared license expressions and retained notice
bytes in the license review and JSON. The assessment preserves compound AND/OR
expressions, legacy slash notation and all collected attributions. It does not
choose an alternative license silently or certify legal compliance.

Four objc2-family statement-only fallbacks remain review-required for both probe
bundles: block2 0.6.2, dispatch2 0.3.1, objc2-encode 4.1.0 and objc2 0.6.5.
They require the full applicable texts/attributions and an exact-version
redistribution review, including the upstream Apple SDK-derived-work questions.
russh-util 0.52.0 remains the fifth flag for all-probes: its archive records
dirty=true and comparison with the upstream revision remains uncleared.

The captured VCS records also identify dirty=true for idna 1.1.0 and ring 0.17.14.
Their registry archives are checksum-verified and their archived notices are
collected; that does not prove equality to an upstream Git tree. The license
review lists all three dirty records explicitly. Existing publication restrictions
and notice flags are preserved; this task accepts none of these source questions.

No maintainer acceptance is needed to record a negative assessment without
allowing publication. Any future decision to accept unresolved issues belongs to
the maintainer. This task does not expand into automatic dependency fixes or
later roadmap tasks.

## Actual verification and retained failures

Command, exit statuses, actual Rust/Cargo/Python versions and output hashes are
retained in [commands.json](evidence/dependency-20261114/final-run/artifacts/review/commands.json)
and [checks.json](evidence/dependency-20261114/final-run/artifacts/checks.json).
The final command was:

```sh
python3 scripts/run-dependency-review.py --image sha256:7fe1ac9c7a27d03c783c046a2446674883a45fad4593d71f7bfaff3c0728a252 --preparation target/quality-task-051/preparation --out-dir target/quality-task-051/run-r6
```

- Distribution Python suite: 21 tests passed, no skips, exit 0. The new cases cover
  clean/vulnerable/unmaintained/unsound audit records, malformed/failed/filtered
  audit rejection, normal/build/dev closure with cycles/shared nodes, version and
  source isolation, notice bytes/features, partial missing notice, archive/manifest
  tampering, DB content/commit/receipt mismatches, and immutable image/output guards.
  Existing packaging and exact-version fallback tests retain their negative cases.
- Real offline evaluator: exit 0, all three variants assessed. The independently
  published advisory facts and retained archives/notice bytes provide external
  checks beyond synthetic self-consistency.
- Confinement: nonroot, read-only source/root, empty allowlisted environment,
  network none, 1 CPU, 1 GiB memory with no extra swap, 64 PIDs and 512 MiB scratch.
  Actual cgroup/readonly/network boundary checks passed. Source hashes matched
  before/after execution; the container was removed.
- Python syntax parsing and `git diff --check`: exit 0; receipts in
  [VERIFICATION.json](evidence/dependency-20261114/VERIFICATION.json).
  Rust builds/tests/clippy and hosted CI are not applicable to these script-only
  changes; no Rust, Cargo, CI or packaging implementation was modified.

Retained attempts r1-r5 document the tmpfs export lifecycle error (r1), missing
macOS cache (r2), invalid assumption that fetched crates contain vendor-style
.cargo-checksum.json (r3), and successful earlier runs (r4/r5). The implementation
review then identified partial notice omission; r6 fixes it using the actual
archive notice set. Failed attempts are not counted as product regressions or
replaced by passing logs. Trusted cache preparation used only manifests and empty
source stubs with cargo fetch --locked; no repository build scripts ran online.

## Reviews and completion

Implementation reviewer `/root/dependency051_implementation_review` accepted the
scoped fix and final executable r6 source bindings. Test reviewer
`/root/dependency051_test_review` accepted the 21-test evidence and independently
checked all three inventories, findings, DB contents and final-source bindings.
Neither reviewer found remaining required corrections. Details are recorded in
[the review record](TASK-051-REVIEWS.md).

The main agent marks task 051 done for the completed evaluation, not for dependency
remediation or publication clearance. Both original acceptance criteria and
deliverables have final-source evidence. Findings and source/license restrictions
remain explicit, with no exception accepted. No later task, commit, push, PR,
merge, tag or release action was performed.
