# Inventory report naming implementation plan

> **For agentic workers:** Use superpowers:executing-plans inline; user explicitly
> requested implementation, tests, documentation, independent review and a local commit.

**Goal:** Rename the user-facing report to Cryptographic Inventory and Evidence
Report (暗号資産・検出根拠レポート), selected by `--inventory-json` and written to
`scan.inventory.v1.json`, preserving earlier consumers.
**Architecture:** Existing CLI and report crate; a shared serialization path and
single capture for either/both flags. Core/detectors and JSON payload are unchanged.
**Tech Stack:** Existing Rust 1.93.1/clap/serde, no dependencies added.
**Spec:** User-selected name, flag and filename in this conversation.

## Constraints and compatibility decision

Same branch `codex/task-022-sarif-location`, initially clean at
`7e4d0ce226fed69fe6573462aa63dde02a71425f`. No push, PR or merge.
New flag writes the new filename only. Old `--native-json` and
`write_native_json` retain the old filename. Both flags write identical payloads
to both files from one capture. `--repository-scope` accepts either or both.
Default artifacts, serialized keys/IDs, old Rust types/API and schema URI/path
remain stable. Schema title and current docs use the new descriptive name;
historical implementation/review/measurement records stay historical.

## Steps and review focus

- [x] Add failing CLI tests for new flag/filename/golden, old flag and both flags,
  scope requirements, no implicit artifact and compatible exit behavior.
- [x] Add the new report writer and CLI dispatch; retain old writer behavior.
- [x] Update both READMEs/current contract and schema title; keep old doc links.
- [x] Rebind only existing CI fixture expectations affected by shifted source;
  check actual self-scan projection and old/new artifact shadows.
- [x] Run pinned workspace build/fmt/test/clippy and relevant Node/Python checks;
  independently review the complete follow-up diff and record evidence.
- [x] Commit locally and confirm clean tree; no publication.

## Execution record

Implemented and independently reviewed. See
[the validation report](../../../tasks/025/TASK-025-INVENTORY-NAMING.md) for exact source
bindings, executed outcomes, compatibility and limitations. Final local commit
and clean-tree check are performed after this record is staged.
