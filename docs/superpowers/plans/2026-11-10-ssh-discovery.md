# Task033 SSH advertised-offer discovery

**Spec:** docs/architecture/RUNTIME-PROBE-IMPLEMENTATION.md, selected under
maintainer delegation. Task030's typed authorization/transport is reused.

**Goal:** Collect initial SSH2 identification/KEXINIT advertisements from one
explicitly authorized endpoint, with no key exchange execution or authentication.

**Ownership:** pqc_scan_probe_ssh crate, its schema/tests and task/quality record.
Root Cargo, CLI, core and report integration belong to the parent agent.

## Interfaces

`SshProbe::new() -> Self`, `failed(&self, ProbeTarget, &ProbeLimits,
ProbeDiagnostic) -> ProbeReport<SshObservation>` and
`run(self, AuthorizedTarget, ProbeBudget) -> ProbeReport<SshObservation>`.
Pure parsers and profiles compile without the runtime/OS-random dependency.

## Steps

- [x] Add failing fixtures for exact ten-list parsing, markers/unknown names,
  framing/length/padding/ASCII/boolean limits and identification boundaries.
- [x] Implement bounded parser/profile/classification from primary RFC/OpenSSH
  sources; preserve order/duplicates and report offers as unauthenticated.
- [x] Add failing local-peer runtime tests proving split/coalesced reads,
  wait-for-client KEXINIT, no packets beyond identification+message20, cancellation,
  traffic/time limits, malformed frames and immediate DISCONNECT termination.
- [x] Implement runtime with BoundedTcpStream and OS-random cookie/padding; never
  import full SSH/crypto/auth libraries, configuration or system-ssh backend.
- [x] Validate separate v1 JSON schema offline, plus source/semantic/privacy checks.
- [x] Run isolated local OpenSSH classical/hybrid fixtures where available,
  recording version/config/sanitized counters and cleaning ephemeral host keys.
- [x] Format, default/runtime tests and clippy; record exact source/binary evidence,
  unexecuted platforms/CI/UI, then move to review. Parent reviews/evaluates/commits.

## Review focus

Oversized allocation before frame checks; noncanonical boolean true; duplicate or
extension names; SSH1/1.99 compatibility boundaries; guessed KEX packet confusion;
server claims promoted into verification; stale socket after failure; leaked raw
ignored messages or ephemeral test keys; feature dependency leakage.
