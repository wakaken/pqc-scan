# Task 022 SARIF location implementation plan

> Execute inline with superpowers:executing-plans; obtain a fresh independent whole-change review.

**Goal:** Correct SARIF 2.1.0 locations without changing native findings or CLI.
**Architecture:** Keep projection in the existing report writer. Preserve zero-valued native unknowns, omit unknown SARIF regions/columns, encode filesystem paths as URI references, declare Unicode code point columns. Do not read source files during reporting.
**Tech Stack:** Rust 1.93.1, serde, pinned jsonschema (development only).
**Spec:** tasks/022/task.yaml and docs/roadmap/v0.2/ROADMAP.md#task-022.

## Constraints and design

Relative paths retain their spelling/separators except percent encoding; POSIX absolute paths become file URIs. On Windows normalize native separators and handle drive/UNC paths. On Unix a backslash is a literal filename byte, not a separator. Encode UTF-8 bytes, reserved characters and literal percent signs; preserve slash and unreserved ASCII. Empty file paths omit locations. Unknown line omits the entire region; known line/unknown column retains startLine only. Columns are one-based Unicode code points, not visual or UTF-16 columns.

AST legacy positions mix byte and character offsets. Preserve native IDs/values; emit its column only when the existing unmasked source snippet proves the prefix through that position is ASCII and within the unclipped prefix. Otherwise retain line only. Missing or masked snippets cannot prove a column. Other built-in detectors already count code points or use ASCII indentation/column one. External callers must provide code point columns.

## Task 022 (one delivery unit)

- [x] Add writer tests for unknown line/column combinations, empty paths, URI reserved characters/Unicode/absolute paths/platform cases; run against old implementation and observe failures.
- [x] Implement projection; verify semantic Unicode/code point and conservative AST cases, deterministic writer output, non-location result/rule stability, secret masking.
- [x] Vendor official OASIS SARIF 2.1.0 schema with upstream revision/license/digest; add pinned offline validator and negative controls.
- [x] Run old/new CLI on identical temporary input roots and compare all formats (allow generated timestamp only); exercise actual unknown SBOM and Unicode source positions.
- [x] Re-run repository self-scan and inspect CI expectation source/result bindings; rebind only previously reviewed test expectations affected by source changes, retain all unknown/product findings.
- [x] Update both READMEs and task evidence; independent review, build/fmt/test/clippy/Node/schema checks, then local commit only.

## Review focus

Percent double encoding and fragments; Unix backslash versus Windows separators; unknown line with nonzero column; AST mixed units/clipped/masked snippet; registry suppression must remain fail-closed. These classes belong in writer and self-scan acceptance checks above.

## Execution ledger

Base: 1335033fb3ffd96632f9f39a51bf8f2a115f2ead. Worktree initially clean. User supplied detailed implementation authorization and local-commit endpoint, so no additional design/plan approval pause. No additional tasks or PRs.

Outcome: all steps complete; independent review approved, 145 Rust and 22 Node tests passed. Artifacts and limitations are in tasks/022/SARIF-LOCATION-CONTRACT-022.md.
