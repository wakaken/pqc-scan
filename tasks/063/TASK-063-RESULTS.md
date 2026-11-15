# Task 063 remediation and provenance investigation

Status: the authorized remediation and investigation are complete, including
final-source checks, native packaging/install checks and both independent reviews. No publication,
release, upstream message, risk waiver or crypto-policy change has occurred.
Task 051 remains the original historical evaluation and its evidence is unchanged.

## Changes and initial verification

- crossbeam-epoch: 0.9.18 -> 0.9.20, through the lockfile's indirect dependency.
- anyhow: 1.0.102 -> 1.0.103; raise the declared minimum as well as the lock version.
- tokio: explicit pin 1.49.0 -> 1.51.6; required tokio-macros 2.6.1 -> 2.7.2.
  Retain russh 0.65.1 and crypto provider versions/policy.
- YAML: replace serde_yaml 0.9.34+deprecated / unsafe-libyaml 0.2.11 with
  yaml_serde 0.10.7 / libyaml-rs 0.3.0 under the existing serde_yaml Cargo alias.
  The YAML organization's maintained fork preserves the existing API. Its
  low-level implementation still contains translated unsafe Rust; this fixes the
  maintenance issue and does not certify memory safety. serde_yaml_ng was rejected
  because it still depends on unsafe-libyaml; serde-saphyr would require a larger
  API/behavior migration.

An isolated old/new comparison initially covered 71 repository YAML inputs and 12 explicit
edge cases (duplicates, aliases, custom tags, nonfinite floats, numeric forms,
empty/multiple/malformed documents, YAML1.2 booleans, Unicode, overflow and merge
keys). All 83 accepted values or error strings matched. An expanded run also
included `.yaml` fixtures, bringing the final total to 75 repository inputs plus
12 edge cases: all [87 values/errors matched](evidence/remediation-20261114/yaml-probe/RESULTS-r2.json).
The original RESULTS.json preserves the earlier 83-case run. Directory-shaped YAML fixtures were
excluded because they are not parser inputs. No observed specification
change requires approval. Two additional product-level rule loader regression
tests preserve aliases/multiline/YAML1.2 string behavior and reject duplicate keys
and multiple rule documents. Existing bounded dependency/parser tests also ran.

Initial final-validation-r1 passed default/all-feature workspace build, test and
strict clippy, 21 distribution Python tests and five actual SSH KEX CLI/OpenSSH
cases (success in both languages, wrong pin, modified signature, no overlap).
The refreshed task-051 evaluator reported zero RustSec findings for the current
348-package lockfile and all three variant closures. Final evidence is
retained under this task, never over the task-051 originals.

## Final verification evidence

Source baseline is `f07d782cd68cfac95d93585d5ace049f590160d4` plus the preserved
task-051 worktree and the task-063 changes. Current compiled product/Rust-test inputs
are bound by Linux source hashes and native package provenance/binary hashes;
final Python/report inputs are separately bound by the r4 dependency review.

| Configuration | Workspace Rust tests passed | Existing ignored helpers | Build / strict clippy |
|---|---:|---:|---|
| default | 546 | 3 | exit 0 / exit 0 |
| combined-probes | 580 | 5 | exit 0 / exit 0 |
| all-features | 605 | 5 | exit 0 / exit 0 |

Default/combined evidence is in
[validation-default-combined](evidence/remediation-20261114/validation-default-combined/RUN.json),
with the final all-feature rerun in
[validation-all-features](evidence/remediation-20261114/validation-all-features/RUN.json).
Ignored entries are the existing fuzz-supervisor worker entrypoints and explicit
OpenSSH helper, not newly disabled tests. Five actual CLI/OpenSSH cases separately
verify handshake success, pin/signature failures, no overlap and no authentication.
The all-feature run also passes all 21 distribution Python tests.

The final internal license-review wording was made neutral: dirty=true alone must
not override a completed source comparison. A dedicated Python regression and
fresh evaluator run pass in the final dependency-review (r4): 22 Python tests,
exit 0. The earlier r3 review is retained separately. Rust/Cargo/package inputs
did not change; [FINAL-CHECKS.json](evidence/remediation-20261114/FINAL-CHECKS.json)
verifies their continued applicability as well as fmt and diff checks.

Linux execution uses read-only source/root, a fresh allowlisted environment,
network none, 2 CPUs, 6 GiB memory with no extra swap, 128 PIDs and 4 GiB scratch.
Actual boundary checks, before/after source hashes and container cleanup passed.
The first three-config run exhausted scratch while linking the last configuration;
default/combined had already passed. Keep that failed run and run the all-feature
configuration separately with unchanged source, rather than weakening the limits.

On native macOS arm64, all three variants passed release build, actual archive
preparation and clean-prefix offline installation. Each installation checked help,
version, exact optional command availability, rules and a positive/negative scan
fixture (three expected findings on the positive file). All-probes also checked
the host-key pin interface. See [native commands](evidence/remediation-20261114/native/COMMANDS.json)
and the three INSTALL-RESULTS.json records beneath that directory. Tested local
archives remain under `target/quality-task-063/native-r3/`; hashes, inventories and
provenance are retained durably, without storing build caches or publishing assets.

Native build/packaging uses the existing macOS sandbox with read-only source/cache,
scratch-only writes, explicit environment, offline Cargo and finite CPU/process/
file/wall limits. This does not claim Linux-style aggregate memory/disk cgroups.
Initial native attempts failed on Xcode wrapper cache writes outside scratch;
direct configured clang/ar/git and SDK paths resolved them without changing the
Xcode license or widening scratch permissions. Clean-install product commands
are separately constrained offline by the existing installation verifier.

The final [dependency review](evidence/remediation-20261114/dependency-review/artifacts/review/dependency-review.json)
and [license review](evidence/remediation-20261114/dependency-review/artifacts/review/license-review.md)
report zero RustSec findings and notice flags 0/4/4 for 139/168/271 packages.
The current DB acquisition has the same RustSec commit as task 051, so the change
from four findings to zero is not an advisory suppression or DB-version change.
No finding is ignored. Zero findings is not proof of absence of all vulnerabilities.
The task-051 historical evidence hashes remain unchanged.

## Source provenance findings

All downloads are from public exact-version registry archives or recorded
GitHub commits; registry archive SHA-256 matches the task-051 lock. Comparisons
read archive bytes without installing or executing downloaded code on the host.

- russh-util 0.52.0: all three Rust files match recorded upstream
  `3d09c20c52a542ffa327723d3a44cb1edd9b0ca4`. Cargo.toml.orig differs only by version
  0.51.0 -> 0.52.0. Normalized tokio declarations match the upstream workspace's
  version 1.17.0 and package feature additions. The Apache-2.0 notice remains bound
  to that revision. Clear this technical source-investigation flag while retaining
  the archived dirty=true fact and granting no publication approval.
- idna 1.1.0: recorded commit `91377f48bf35011d042aa5abef9e7f2a0a625aaa` matches all
  15 comparable archived files. Original manifest differs only by registry keyword
  no_std -> no-std. Archive-only files are Cargo.lock and the workspace license
  texts; their root-repository provenance is checked separately. No implementation
  difference was found.
- ring 0.17.14: all 282 comparable archived files match recorded upstream
  `2723abbca9e83347d82b056d5b239c6604f786df`. Archive-only files are Cargo.lock and
  106 pregenerated assembly/header/object files. Re-executing unchanged upstream
  generation functions with a small recorded driver in offline confinement
  reproduced all 89 assembly/header source files byte-for-byte. The 17 Windows
  object files were not assembled (NASM unavailable) and are not used for the
  declared macOS arm64 distribution target. Do not extend this result to Windows
  binary reproducibility or infer that dirty=true means malicious code.

## Remaining objc2 notice questions and maintainer decision

Exact registry archives for block2 0.6.2, dispatch2 0.3.1, objc2-encode 4.1.0 and
objc2 0.6.5 contain no conventionally named license/notice file and no copyright
occurrence in their text files. The checksum-bound archives and the exact pinned
upstream LICENSE.md statements were reacquired and verified. Those statements
link generic license texts but explicitly leave Apple SDK-derived-work licensing
uncertain. Do not invent copyright holders, substitute a generic MIT template as
package-specific permission, or clear the four review flags.

The dependency path is pqc_scan_cli -> ctrlc -> dispatch2 -> the objc2 family.
Replacing ctrlc would require separate compatibility investigation of signal
handling and Ctrl+C cancellation. On 2026-11-14, the user selected: keep both
probe distributions on hold and prepare an upstream clarification draft without
sending it. This preserves current distribution policy and accepts no unresolved
license condition. The four flags remain; this is a known unresolved product
publication condition, with an explicit decision on the next action.

## Upstream clarification draft (not sent)

Subject: Exact-version license texts and Apple SDK redistribution guidance

Hello objc2 maintainers,

We are preparing optional macOS arm64 probe bundles for pqc-scan. Through ctrlc,
the dependency closure includes block2 0.6.2, dispatch2 0.3.1, objc2-encode 4.1.0
and objc2 0.6.5. We have not published these probe bundles.

We verified their registry archive checksums and the LICENSE.md statements at
the Cargo-recorded upstream revisions. The archives do not contain standalone
license/notice files, and the upstream statement links license texts while
noting uncertainty about Apple SDK-derived works.

Could you point us to:

1. The canonical complete license text and applicable copyright/attribution
   notices for redistribution of each of these exact versions?
2. Any documented clarification of the Apple SDK-derived-work conditions for
   binary distribution, and whether the intended distribution requires additional
   notices or permissions?
3. Any later release that resolves these documentation questions, with a link to
   the relevant change?

We are retaining the upstream statements and keeping the optional bundles on
hold until we can document the applicable conditions. We can provide the exact
archive hashes and recorded revisions if helpful.

Thank you.

## Completion decision

Both [review stages](TASK-063-REVIEWS.md) accepted the final changes and evidence,
including the r4 wording regression and the durable-copy confirmation. The main
agent marks task 063 done for the authorized compatible remediation and completed
investigation. Remaining objc2 publication conditions are explicitly unresolved,
with the user-selected hold and unsent draft recorded above; they were not waived.

All required checks have final-source evidence. Windows object regeneration,
release attestation, new platform certification and publication were not part of
this request and are not claimed. Local changes remain uncommitted/unpushed.
