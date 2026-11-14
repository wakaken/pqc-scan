# Task 025: inventory-only CLI follow-up

Explicit user decision: remove `--native-json` because two switches for the same
report are confusing. This supersedes the earlier request to retain that CLI
switch. Base: `9a3934c2f49756592a8ee9361955bf411719197d`; same branch
`codex/task-022-sarif-location`, clean at start.

## Result

The Cryptographic Inventory and Evidence Report (暗号資産・検出根拠レポート) has
one CLI switch: `--inventory-json`, writing `scan.inventory.v1.json`.
`--native-json` alone or with the current switch is rejected during argument
parsing before reports are written. It is absent from help. Scope still requires
`--inventory-json` and remains optional. Default report selection, machine fields,
IDs, exit precedence, rules, core/detectors and dependencies are unchanged.

Both READMEs give the migration instructions and current architecture documents
reflect the removal. Stable library APIs, including `write_native_json`, remain
available: the user removed a CLI switch, not existing library entrypoints. The
CLI no longer calls the old writer. Existing files in output directories are not
deleted. Older implementation/review records describe their historical snapshots.

## Verification

Pinned Rust 1.93.1; workspace default features, offline Cargo resolution:

- Workspace debug/release builds, fmt check and clippy with denied warnings passed.
- **177 Rust tests, 20 Python tests and 22 Node tests passed.**
- Actual CLI tests reject the removed switch alone and in combination, assert no
  output-directory creation, and verify its absence from help. New-option tests
  retain schema/golden, private-payload masking, public certificate/dependency,
  optional scope and exit 0/2/3 coverage. Release-binary rejection smoke tests also pass.
- Saved old release versus current release, default and inventory enabled: all
  four self-scans have **814 findings**. Six legacy reports plus diagnostics agree
  after generation-time normalization. Inventory payloads agree after normalizing
  run/time/observation IDs together with references; scoped occurrence/asset IDs
  and other facts remain exact.
- CI expectations rebind **34 existing entries in one file**, reflecting source
  line movement only; no new omission. Projection remains **814 raw = 660 omitted
  + 154 retained**, all **99 source bindings verified**.

[Independent review](TASK-025-INVENTORY-ONLY-REVIEW.md) has no material findings.
[Machine evidence](evidence/task025/INVENTORY-ONLY.json) records exact current source hashes,
base, binaries, commands and outcomes. Detailed local artifacts remain under
`target/task025-inventory-only/`, including `verification.json`, `legacy-shadow.json`,
`release-rejection.json`, `code-scanning-audit.json` and historical red/green logs.
An intermediate private-marker test still read the old filename; that test fixture
was corrected to the new filename before final successful verification.

This intentionally breaks the removed CLI spelling, as requested. The current
[inventory contract](../../docs/architecture/INVENTORY-REPORT-V1.md) retains all prior
coverage/trust/provenance/identity/privacy limits. No performance remeasurement,
remote CI, platform matrix, push, PR, merge or publication was performed. No UI,
cryptographic or detection behavior changed.
