# Task 038 local distribution preparation

Status: preparation implementation; dependencies and acceptance gates remain open.
The user delegated technical selection on 2026-11-10. This plan selects source
checkout installation plus unpublished local macOS Apple Silicon archives, with
static and combined TLS/SSH probe variants. Version remains 0.1.0. No full OS
support matrix, 1.0 claim, publication, signing or credentials are included.

## Execution plan

1. Record the actual channel/reporting inventory and retain explicit rules paths.
2. Test deterministic tar/gzip output, checksum tampering and unsafe path/notice
   rejection before implementing the preparation tool.
3. Bundle the exact supplied binary, reviewed rules, bilingual operating guides,
   project license/security policy, resolved dependency license inventory and
   filename-selected notices. Record dirty snapshot/source and binary hashes.
4. Package twice with fixed source/binary inputs, compare bytes, and verify all
   member hashes. Keep independent build evidence separate from archive identity.
5. Install into an empty prefix, list installed rules and scan an external
   nonsecret fixture with all network operations denied. Retain logs under target.
6. Prepare a manual contents-read-only artifact workflow without executing it.
7. Request independent source/license review; leave final task acceptance blocked
   on dependencies, notice gaps, build binding and platform/remote evidence.

## Security and limitations

The tool uses subprocess argument arrays, explicit source assets, normalized tar
metadata and an offline Cargo metadata query. It rejects escaping paths, control
characters, duplicate/nonregular members, outside/symlink notice files and
non-arm64 thin Mach-O binaries. Verification is bounded to 512 MiB/10,000 members.
Only license-name files are copied from package sources, never entire caches.
Resolved normal/build dependencies are conservative audit evidence, not proof of
exact linked objects or legal certification. Missing notices remain explicit.

`PROVENANCE.json` does not prove the caller's binary was built from the recorded
snapshot. Parent build evidence must bind both hashes. Archive repetition does
not prove independently reproducible binaries. The workflow is prepared only;
its runner and artifact upload have not been exercised.

The adopted maintenance scope is latest verified development snapshot,
best-effort. No SLA, older-release backport or 1.0 promise is made. Reporting uses
the inventory-confirmed disabled private-reporting state and an interim public
request for a private contact without sensitive details. No contact is invented,
no setting is changed and no message is sent.

See [preparation evidence](../quality/TASK-038-PREPARATION.md).
