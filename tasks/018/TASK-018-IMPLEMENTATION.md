# Task 018 implementation plan

Goal: account for read/detector failures and uncertain inspection without changing legacy scan data, IDs or the six existing report contracts. Deliver API, CLI, tests and documentation in one commit/PR unit; no push or PR creation is authorized.

Specifications: [task 018](task.yaml), [domain model](../../docs/architecture/DOMAIN-MODEL.md), [quality gates](../../docs/quality/QUALITY-GATES.md). The user's implementation request authorizes the additive `--diagnostics` / `--require-complete` interfaces described there. Work on the existing clean branch; no worktree or crate split is needed.

## Files and interfaces

- `pqc_scan_core/src/diagnostic.rs`: private-field `ScanDocument`, `ScanDiagnostic`, `CoverageSummary`, concrete `DetectorOutcome`, outcome enums and a versioned diagnostics-only serialization view.
- `pqc_scan_core/src/lib.rs`: additive `Detector::detect_detailed` default calling `detect` once and marking coverage not-reported; export `scan_repository_detailed` and new types.
- `pqc_scan_core/src/pipeline.rs`: one shared pipeline, injectable private file reader for deterministic failure tests, retained legacy normalization/risk/recommendations/IDs.
- `pqc_scan_core/src/walker.rs`: detailed enumerated exclusion/error reasons; retain `WalkStats` and legacy function signature and successful selection behavior.
- `pqc_scan_report/src/lib.rs` / `json_report.rs`: separately called `write_diagnostics(&ScanDocument, &Path) -> Result<PathBuf>`; never add fields to existing serialized results or automatically select the extra artifact.
- `pqc_scan_cli/src/main.rs`: boolean opt-ins, one stderr completeness notice, write partial reports before evaluating gates, codes 0/1/2 unchanged by default; opt-in completeness code 3 takes precedence over finding threshold 2, while setup/output errors remain 1.
- Both READMEs and current architecture: implemented contract, unknown first-party adapter coverage and future parser/cap limitations.

## Constraints and review focus

- Rust 1.93.1, existing dependencies, no networking or rule changes. Error diagnostics use fixed code-based messages; never serialize detector error chains or file contents.
- Selected = read-ok + read-failed; read-ok partitions into analyzed/partial/analysis-failed/unsupported/not-applicable/unknown. Enumerated exclusions and soft enumeration failures stay separate. Iterator-level traversal failures retain the legacy fatal error behavior at every depth.
- Legacy adapters, including current built-in detectors, cannot attest to internal parser/cap completeness. Nonempty scans initially remain unknown; strict mode returns 3. Empty declared scope can complete without claiming absence of crypto.
- Per-detector failure retains other hits. Successful zero-hit detailed inspection is distinguishable from empty legacy output. No detectors must not claim completeness for selected files.
- Ignore-hidden population is explicitly uncounted; excluded inputs are outside the declared scope. Root failure is fatal. Unrelated task states and detector behavior stay untouched.

## Implementation and validation sequence (one task, one PR)

- [x] Capture pinned baseline build/tests and English/Japanese all-format example outputs under ignored `target/task018/`.
- [x] Write failing API/pipeline tests: empty success versus not-reported, injected reads, mixed detector errors/hits, file partitions, explicit unsupported/partial states, safe diagnostics and legacy-wrapper equality.
- [x] Implement minimal diagnostic API and shared pipeline; run targeted core tests.
- [x] Write failing CLI/export tests: new flags, strict precedence, default filenames/shape/IDs, empty scope, invalid root and output failure. Implement the additive writer and CLI; run targeted tests.
- [x] Update both READMEs/current architecture and record implementation scope/remaining unknowns.
- [x] Run fmt, workspace build/tests/clippy, Node report tests, baseline all-format comparisons and synthetic CLI exit checks. Obtain a fresh independent review; correct material issues and record evidence in task 018.
- [x] Evaluator checks evidence and lifecycle before marking done. Commit the whole task locally; no push or PR.

## Review decisions

The independent review identified fatal-root enumeration compatibility and unreported soft ignore-policy errors. Both received failing regression tests before correction. To preserve the existing exit contract, all iterator-level traversal errors remain fatal, rather than introducing a new default success/partial-exit behavior for nested traversal errors. Soft errors attached to valid entries are recorded before skipping directories; no error/glob contents are exported. A contradictory NotApplicable outcome containing hits is conservatively partial. No detector-specific cap/parser behavior was changed.

## Validation outcome

Pinned local build/fmt/clippy passed; 90 Rust tests and 3 Node tests passed. English and Japanese example scans each retained all 290 findings, 55 selected files and 2 skipped files. All six legacy artifacts matched the saved pre-change binary, excluding only generation timestamps. Actual process checks verified codes 0/2/3, strict-before-threshold precedence, diagnostics-write failure 1 and unreadable-root 1 in both baseline/current binaries. The independent reviewer confirmed the two corrections and approved the implementation. Exact code digests, logs and limitations are recorded in task 018. Remote CI, new performance/accuracy measurement and browser acceptance were not executed; push and PR creation remain excluded.
