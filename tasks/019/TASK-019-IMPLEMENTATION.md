# Task 019 implementation plan

Goal: expose bounded static-detector inspection outcomes, recover regex hits hidden behind comment-only matches, and keep existing CLI/report contracts. One task, one local commit/PR unit; push and PR creation are excluded.

Baseline: `15de16ac66fccd9768a4454e1e76aad99c874804` (`origin/develop`, including task 018). Branch: `codex/task-019-detector-limits`. Initial worktree clean. Specifications: [task 019](task.yaml), [domain model](../../docs/architecture/DOMAIN-MODEL.md), [quality gates](../../docs/quality/QUALITY-GATES.md).

## Scope and decisions

- Retain 32 accepted hits per rule/file for regex and AST, and 8 for key markers. Regex counts accepted hits after the existing comment filter. Keep all other matching, masking, deduplication, language/scope and metadata behavior.
- Use the existing `DetectorOutcome` / `DetectorStatus` / pipeline diagnostics schema. Override `detect_detailed` in the three detectors; retain `detect` as a wrapper over the same execution. Expose the existing consuming outcome accessor so wrappers do not clone results.
- A 33rd eligible unique hit (9th key hit) proves hit-limit truncation. Exactly the hit cap is not itself truncation: continue bounded eligibility/dedup checks and report Complete if the remaining search is exhausted.
- Regex/AST inspect at most 4096 regex-match candidates per rule/file, with one look-ahead to establish remaining candidates. A work-budget stop is Truncated inspection even if no additional eligible hit is proven. Never report an omitted-hit count.
- AST retains at most 4096 semantic candidates and schedules at most 100000 tree nodes. Existing 2000-character candidate clipping is preserved and now signals incomplete inspection. These are conservative engineering work bounds, not measured performance targets. Character clipping must remain Unicode-safe and bounded.
- Parser errors keep obtainable hits and report Partial. Parser setup/no-tree failure is an error. Unsupported language, binary text and no applicable rules are outside that detector's declared scope. Complete means the existing supported pattern inspection completed, not resolved API semantics or whole-product safety.
- Dependency/certificate detectors retain NotReported; default full scans can remain Unknown or become Partial. No new CLI option, rule change, output fields, schema version, crate or dependency is needed.

## Files

- `pqc_scan_detectors/src/{regex_detector,tree_sitter_detector,key_detector}.rs`: shared legacy/detailed execution and finite per-rule accounting.
- `pqc_scan_detectors/src/lib.rs`: shared private match-work budget.
- `pqc_scan_core/src/diagnostic.rs`: make the existing consuming outcome accessor public; no serialized type change.
- `pqc_scan_detectors/tests/bounded_outcomes.rs`: real rule/detector/pipeline boundary tests using synthetic text only.
- Existing CLI tests only if their documented diagnostic expectations need updating; no CLI implementation change is planned.
- Both READMEs and current architecture: supported outcomes, work limits, deliberate recovered-hit changes and remaining unknown coverage.

## Implementation sequence (one PR)

- [x] Capture offline build/tests and both-language all-format baseline artifacts under ignored `target/task019/`.
- [x] Write and run failing tests for 32 comments + code, exact/over cap, comment-only work exhaustion, other rules after a capped rule, 8/9 masked key markers, AST duplicate look-ahead, parse errors, clipping and collection limits.
- [x] Implement detailed outcomes with the existing pipeline contract; run focused detector/pipeline tests and keep legacy fixture results stable.
- [x] Verify a custom scan using only the three supported detectors can complete below caps, but truncated/partial scans cannot; verify default scans still expose dependency/certificate NotReported.
- [x] Update English/Japanese docs. Compare all six example artifacts to the baseline, ignoring only generation time and explicitly reviewing any recovered hits. Check default and opt-in CLI exits and diagnostics fields.
- [x] Run fmt/build/workspace tests/clippy/Node checks. Obtain an independent review and correct material findings with regression evidence.
- [x] Evaluator records evidence and task status, then commit locally without publishing.

## Review and compatibility observations

Independent reviewer `task019_review` approved the source changes with no material findings. Two suggested boundary cases (4096/4097 AST candidates and later rules after an AST hit cap) were added and passed. A timestamp-only temporary-directory collision discovered in parallel test fixtures was corrected with a process-local atomic sequence; three parallel reruns passed. This affected the new test harness, not scanner behavior.

English/Japanese example outputs retain 290 findings, 55 selected files and 2 skipped files; all six legacy artifacts match the baseline except generation time. Diagnostics retain the schema-1 field set; only dependency/certificate remain NotReported. One existing TypeScript fixture has error nodes under the existing JavaScript grammar, now honestly reported Partial rather than hidden in Unknown. Synthetic CLI cases demonstrate the intended recovered comment-hidden hit (0 to 1), retained regex/AST/key caps (32/32/8), safe dummy-key masking and strict exit 3. No grammar/dedup/rule fix outside task 019 was applied.

## Final validation

Rust 1.93.1 offline build/fmt/clippy passed. Workspace tests: 105 passed (baseline 90), including 15 new boundary tests; Node report tests: 3 passed. Both-language legacy comparisons and synthetic CLI cases passed. Task 019 records exact code digests, logs and unexecuted checks. The requested completion action is a local commit; no publication is authorized.
