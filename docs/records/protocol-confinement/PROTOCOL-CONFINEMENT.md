# Protocol confinement — 2026-11-11

## Current execution boundary

The current protocol CI path uses `run-isolated.py` with a locally prepared,
immutable tool image and a Linux cgroup v2 container. It covers repository
compilation, all-feature tests/clippy, report previews and product OpenSSH cases.

- Nonroot UID65534, capabilities dropped, no-new-privileges, read-only root/source/cache.
- No host HOME, credentials, SSH agent, Docker socket or host network namespace.
- Environment constructed explicitly; only loopback is active. External traffic has no route.
- Whole process tree: 6 GiB memory, no added swap, 2 CPU quota, 128 PIDs.
- Aggregate data writes: 4 GiB scratch tmpfs plus 1 MiB shared memory. Root and source are read-only; build/temp/HOME/evidence use scratch.
- 1200-second host deadline; container removal covers success, failure and timeout.
- Source snapshot: allowlisted inputs, no-follow directory/file descriptors, regular files only, 32 MiB/file and 128 MiB total.
- Artifact export: 64 MiB raw/aggregate cap, 4096 entries, no traversal, links or special files. Regular outputs are validated before extraction.

Incremental caches and debug symbols are disabled to keep a full workspace build
within the same finite scratch capacity. Test/debug assertions remain enabled.
Image/cache preparation is a separate network-capable trusted-tool operation:
only manifests/stubs enter its new layer and Cargo fetch does not execute target
build scripts/tests. Actual tool image ID, source hashes, environment, limits,
executables and logs are recorded. No weaker fallback is used.

The native macOS runner is retained as `run-native-smoke.py` for development smoke
checks. It is not the strict resource gate. Existing macOS records and frozen
quality measurements retain their historical source bindings.

## Verification

Development evidence is under `target/isolation-fix/`. Actual resource checks
triggered child-process memory OOM, aggregate multi-file ENOSPC, PID exhaustion,
read-only source/root denial, external-network denial and deadline cleanup.
`kernel-green.log` records the first seven checks. The first full strict run
(`suite-bounded/`) passed workspace/all-feature executables, clippy, eight
synthetic protocol/report fixtures and five real OpenSSH product cases.
Final source passed `suite-complete/`: 407 Rust tests, one existing OpenSSH test
ignored, 55 exact test executables, all-feature clippy, eight previews and five
OpenSSH cases. `kernel-complete.log` has 11 passing checks. Memory pressure caused
two cgroup OOM kills at 64 MiB; four completed files plus a further write reached
the aggregate 16 MiB ENOSPC test; 15 children reached the 16-PID cap. Deadline and
lost-create-response cases both left no container. Native workspace356, Node28,
quality Python21 and distribution8 also passed. Current source bindings, image,
limits and evidence digests are collected in PROTOCOL-CONFINEMENT-VALIDATION.json.
One implementation reviewer approved after the lost-create-response and
readiness-race fixes. A separate final-test reviewer approved with no required
improvements, independently checking source/evidence hashes, assertions, all
executable logs and generated report hashes. The final source/evidence record
is PROTOCOL-CONFINEMENT-VALIDATION.json. Optional future coverage includes the
4096-entry exact boundary and direct source leaf/FIFO/size-limit tests; these
were not required for this acceptance.
The container remains running only until bounded evidence extraction completes,
then is removed. A UUID name is fixed before docker create so lost create replies
still have a known cleanup target; an actual daemon-created container plus a
simulated response timeout verifies that branch.

Initial failed attempts are preserved: stopped tmpfs could not be exported, so
completion now uses an atomic status while the container remains alive for
bounded extraction; inactive kernel tunnel devices required checking active
interfaces instead of all sysfs names; scratch explicitly permits execution of
compiled build scripts; unbounded debug/incremental outputs hit ENOSPC, prompting
the bounded build profile above. Tool-image preparation tags cached immutable
image IDs for BuildKit resolution. None of these failures is counted as a pass.

## Quality decisions

QUALITY-GATES and OSS-ROADMAP now state the approved local comparison signals in
the present tense. Task023 is done for the development baseline; numeric CI
threshold enforcement is not introduced. Earlier reviewer/measurement records are
marked historical without changing their measurements, labels or judgments.
Tasks014/016 remain cancelled, tasks001/037 remain blocked. Formal static scope,
experimental probes and existing user-facing ID decisions are preserved.
Hosted CI, publication, signing and legal clearance are separate from this local
verification; this work does not perform those actions.
