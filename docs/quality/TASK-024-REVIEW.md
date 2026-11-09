# Task 024 independent review

Final status: APPROVED for the reviewed task 024 working-tree change. Earlier pending notes below describe interim snapshots and are superseded by the final review.

Reviewed on 2026-11-10, against base HEAD `757d067bff36f6e0538c1482ae4123611a9aab81`, on branch `codex/task-022-sarif-location`. Review is of the working-tree diff, not a committed revision or remote CI result. No source/task statuses were modified by this reviewer.

## Scope and method

Read the repository instructions, Reviewer prompt, task 024 and implementation plan, architecture/knowledge/lifecycle documents, quality gates and task roadmap. Inspected the full detector change, new occurrence tests, readiness regression, unchanged pipeline deduplication/finding-ID code, and existing bounded-outcome tests. Checked correctness, security, compatibility, bounded work and test coverage independently of the implementer.

## Findings

No blocking correctness or security findings in the reviewed implementation.

- A per-rule `(absolute normalized source start, absolute normalized source end)` key distinguishes separate occurrences while merging parent/child representations of the same source interval. Source byte offsets are private and independent of the existing native coordinate calculation. Trimming is restricted to the specified edge whitespace and quotes; case differences do not merge different source positions.
- The clip guard rejects matches whose end enters the synthetic ASCII ellipsis. Unicode clipping computes source length in bytes from the actual clipped UTF-8 prefix, consistent with regex/tree-sitter offsets. The existing Truncated outcome is retained.
- Candidate traversal, language selection, parser recovery handling and work/hit limits are unchanged. Only accepted unique source intervals consume the 32-hit cap. Dedup state stores at most 32 pairs per rule; a 33rd eligible unique match marks truncation. Existing later-rule continuation tests still pass.
- Existing first-representative selection, native location calculation, evidence masking/metadata, public structs, CLI/report schemas and general finding-ID algorithm are unchanged. This intentionally does not correct the existing mixed byte/Unicode native-column limitation.
- No crypto rules, cryptographic operations, secret exposure paths, dependency versions or grammar additions were introduced. New source matches are still passed through the existing comment/identifier eligibility filters and evidence masking.

## Independently executed checks

`cargo test --offline -p pqc_scan_detectors --test ast_occurrences --test bounded_outcomes --test pqc_readiness` exited 0: 5 occurrence tests, 15 existing bounded-outcome tests and 6 readiness tests passed. Coverage includes seven language scopes, nested quoted candidates, distinct case/Unicode/multiline positions, exact 32 versus 33 unique hits, synthetic ellipsis rejection and the frozen task 023 same-line regression.

`git diff --check` exited 0 at the initial review snapshot.

## Pending / limitations

Final before/after metrics, corpus projection/intentional-observation registry changes and bilingual documentation were not yet available at this snapshot; no final whole-change approval is asserted here. Workspace-wide build/test/clippy, report schemas/canaries, remote CI and performance measurements are not independently re-executed by this reviewer. Their evidence must be reviewed separately when available. Passing finite tests does not establish whole-program/API resolution or complete grammar coverage.

## Follow-up review: characterization, CLI and registry

The implementation detector code is unchanged. Independently reran the expanded detector suites (7 occurrence + 15 bounded-outcome + 6 readiness tests) and the new CLI end-to-end test (1 test); all 29 passed offline. The CLI test confirms separate native IDs/columns, repeated finding stability, six legacy output names and the pinned official SARIF schema. New grammar checks accurately distinguish typed TypeScript recovery (Partial), plain JS-compatible TypeScript (Complete) and the tested TSX/JSX subset (Complete); they do not claim universal TypeScript coverage. Standalone-comment checks cover all seven language scopes.

Inspected the self-scan expectation diff and explicit guards in `target/task024/rebind-registry.cjs`. Six exact existing fixture groups increase multiplicities and seven exact fixture positions are added, accounting for 15 newly visible synthetic observations. Only the changed readiness-test source digest is rebound. No production file is newly exempted. The additional production-source results at `recommendation.rs:720` and `japanese.rs:89` remain retained; the audit reconciles raw 793 = reviewed fixture expectations 660 + retained 133. The raw result inventory records all 17 additions.

Inspected `target/task024/compatibility.py` and its summary. The comparison invokes both binaries on the same temporary root/config and canonicalizes only documented timestamps for unchanged rendered output. Unchanged controls contain 12 files / 18 findings in both versions; occurrence controls contain 42 files / 41 -> 44 findings, retaining old native finding payloads/IDs and SARIF/CBOM entries. Additions are constrained to AST occurrences in twice.js and unicode.js; canary assertions cover all generated files and process output. These checks are implementer-executed evidence inspected by the reviewer, rather than independently repeated performance/corpus runs.

Final measurement prose and bilingual documentation remain pending at this follow-up snapshot. No blocking findings in the expanded code/test/registry scope.

## Final whole-change review and approval

Final decision on 2026-11-10: **APPROVED**. No blocking correctness, security, compatibility, scope or evidence-integrity findings remain in the complete task 024 change reviewed against base `757d067bff36f6e0538c1482ae4123611a9aab81`.

Reviewed both README additions, architecture explanation, task/plan record, CI self-scan maintenance text, `TASK-024-AST-OCCURRENCES.md` and `task024/COMPARISON.json`. The prose accurately describes private source identity, unchanged native coordinate/ID behavior, finite limits, nominal API/alias/local-shadow/string/comment limitations and the JavaScript grammar's tested TypeScript/TSX subsets. No grammar addition or silent numeric-budget approval is claimed. Real-product/global accuracy, remote CI and platform/cold-cache performance claims remain explicitly unexecuted or excluded.

Independently checked all 55 final runtime source bindings and all 45 harness bindings against current file SHA-256 values: all match the final measurement environment. Recalculated medians from every recorded warm sample distribution and checked the documentation table values: consistent. Checked accuracy groups directly: only tuning/source_api changes (TP 9 -> 10, FN 5 -> 4, FP remains 3); every other group is exactly unchanged, including all holdout cells. The documented precision 10/13, recall 10/14, 41 files/5746 bytes, three stable accuracy repeats and performance/diagnostic limits are consistent with the machine record.

Inspected all nine final coordinator command records in `target/task024/command-results.json`: every command exited 0. Inspected test logs and independently summed 154 passing Rust tests; Python reports 20 and Node 22 passes. This is review of coordinator verification evidence, distinguished from this reviewer's independently executed 29 focused Rust tests. Final `git diff --check` independently exited 0. No source or lifecycle status was changed by this reviewer.

The frozen corpus and final measurement bindings, same-root compatibility check, exact CI expectation adjustment and retained production observations provide adequate evidence for this bounded occurrence correction. Approval does not grant task 023's outstanding maintainer protocol/budget decision, represent hosted CI validation, or change the documented residual limitations. Evaluator lifecycle completion and an authorized local commit may proceed; no push/PR/merge is authorized by this review.
