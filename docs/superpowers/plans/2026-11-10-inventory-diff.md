# Inventory comparison implementation plan

**Goal:** Compare existing inventory-v1 documents without treating absent or
uninspected observations as resolved cryptographic risks.

**Spec:** [approved continuation decisions](../../roadmap/CONTINUATION-DECISIONS.md#035-proposed-comparison-contract-approval-required)

**Architecture:** A pure core module accepts validated `NativeScan` values and an
explicit repository scope, producing a separately versioned serializable report.
The CLI owns bounded reads and output writing; existing inventory/output formats
and identity construction remain unchanged.

**Global constraints:** Rust 1.93.1; no new dependencies, network or cryptographic
operations. Preserve unknown string vocabulary. Use exact asset and occurrence
keys, never legacy IDs. Duplicate anchors and missing target inspection proof
remain unresolved. No inferred rename or risk resolution.

## Task 035

Owned files: `crates/pqc_scan_core/src/diff.rs`,
`crates/pqc_scan_core/tests/inventory_diff.rs`, task035 YAML and evidence record.
Parent integrates `pub mod diff`, CLI, shared docs and report entrypoints.
Additional parent delegation: own `crates/pqc_scan_cli/src/diff_command.rs` and
the dedicated CLI integration test; parent retains `main.rs`/Cargo integration.

Interfaces:

- `parse_inventory(input: &[u8]) -> anyhow::Result<NativeScan>`: bound input to
  16 MiB, deserialize v1, reject invalid references/duplicate IDs and invalid
  comparison identifiers; never include input fragments in errors.
- `compare(before: &NativeScan, after: &NativeScan, scope: &str) -> anyhow::Result<DiffReport>`:
  require matching explicit scope and supported identity scheme, order results
  by exact keys, expose metadata drift separately.
- `DiffReport::summary_markdown(&self) -> String`: fixed explanatory copy and
  deterministic counts; JSON retains per-record identity and reason codes.

Review focus: duplicate anchors; schema/identity mismatch; global completeness
mistaken for target-level coverage; forged references; same anchor at multiple
moved paths.

- [x] Write paired-input tests for root/line independence, added and missing
  observations, duplicate ambiguity, unknown vocabulary, exact-key content
  changes, drift, incompatible scope/schema and malformed references.
- [x] Run tests and record expected missing-API failure.
- [x] Implement bounded input validation and pure comparison. Compare observation
  facts without run-local IDs, physical positions or checkout roots; compare
  referenced content IDs. Missing-after items are unresolved because inventory
  v1 has no per-target exhaustive native capture proof.
- [x] Add move candidates only for a unique one-to-one semantic evidence anchor
  and different paths, retaining both unresolved/added records; never merge.
- [x] Run focused tests and clippy; record code/input SHA and evidence limits.
- [x] Move Executor status to review. Independent Reviewer, Evaluator, workspace
  checks, CLI/docs integration and local commit are parent responsibilities.

## CLI adapter continuation

- [x] Adapter `run(&Path, &Path, &str, &Path, &mut impl Write) -> Result<()>`:
  read at most 16 MiB + 1 byte, validate both inventories, compare before creating
  the output directory, then write only `scan.diff.v1.json`/`scan.diff.v1.md`.
- [x] Observe a valid-input adapter test fail before implementation and pass after;
  verify malformed/scope/size/read/output errors with no output on invalid input.
- [x] Add a binary integration test for scan → inventory → diff, line-insertion
  correspondence, offline output schema, mandatory scope and scope mismatch.
- [x] Verify the binary test after the parent adds the `main.rs` command hook.
