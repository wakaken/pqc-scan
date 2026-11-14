# Task 024 AST occurrence fidelity implementation plan

> Execute inline using superpowers:executing-plans/test-driven-development;
> obtain a fresh independent whole-change review before local commit.

**Goal:** Keep distinct source occurrences while collapsing overlapping AST candidates for the same source match.
**Architecture:** Extend private CandidateNode with source byte provenance and use canonical source intervals for per-rule deduplication. Preserve candidate traversal, budgets, legacy native coordinates/IDs for existing matches, public model/CLI/report schemas and rule semantics.
**Tech Stack:** Rust 1.93.1 and existing pinned tree-sitter grammars; existing Python baseline and Node projection tools.
**Spec:** tasks/024/task.yaml and docs/roadmap/v0.2/ROADMAP.md#task-024.

## Constraints and decisions

Same branch explicitly authorized: codex/task-022-sarif-location. Base 757d067bff36f6e0538c1482ae4123611a9aab81, initial checkout clean. Technical task023 baseline is available. User explicitly requested continuing task024; unapproved task023 numeric budgets remain unapproved and task023 status is not changed. No grammar dependency addition, rule changes, whole-program resolver, dataflow engine, native schema or general ID algorithm change. No push/PR/merge.

Use per-rule absolute byte spans on the parsed source, with edge whitespace/quote normalization only to collapse wrapper/child representations of the same lexical occurrence. Different columns/spans remain distinct even for identical normalized text. Keep the existing legacy location calculation for existing finding-ID compatibility; mixed AST Unicode columns remain the task022 documented limitation. Do not claim byte columns are Unicode columns. Clipped candidate display suffix is not source; matches crossing it must not create invented source occurrences. No general comment-policy change is required: characterize comments and retain existing policy.

Alternatives: adding a column to the old line/text key still relies on mixed-unit coordinates and can conflate source spans. Rewriting detectors or adding a TypeScript grammar would affect dependencies/coverage without a measured cost-benefit justification for this focused correction. Choose private source spans and document grammar/API limits.

## Task 024 steps

- [x] Read task/role/architecture/quality guidance; trace same-line collapse and collect a current before baseline with the existing frozen corpus.
- [x] Add failing tests for distinct same-line calls/tokens across seven grammars, overlapping parent/child matches, quoted wrappers, mixed case and multiline/Unicode provenance, and 32/33 same-line accepted-hit budget. Reproduce actual task023 twice.js miss.
- [x] Implement span-based deduplication without changing public output structures or existing coordinates; exclude synthetic clipped suffix matches; pass focused tests and existing bounded outcomes.
- [x] Characterize aliases/local shadows/plain strings/TS and comments using immutable task023 fixtures plus source-only cases; do not tune to the held-out set or relabel it.
- [x] Compare actual old/new CLI on the same input/root/config, per-family counts and repeated small/medium/large/pathological performance; retain all old findings/IDs/payloads except justified occurrence/clip fixes. Validate report contracts/canaries/SARIF schema.
- [x] Inspect/rebind exact CI expectation changes and new explicit intentional fixture observations, preserving raw evidence/product results and conservative unknown-line policy.
- [x] Update both READMEs, architecture/measurement evidence and task record; independent review, full offline Rust/Python/Node/build/fmt/clippy and diff checks; local commit only.

## Review focus

Same-line identical tokens; parent/child quote-boundary variants; Unicode byte provenance versus unchanged native column units; cap counting of unique occurrences and later-rule continuation; clipped suffix cannot become fake source. Meaningful tests own each class. Existing AST LIFO order and bounds remain explicit; no source-order completeness or universal grammar/API attribution claim.

Execution outcome: all technical acceptance steps passed. Existing same-line
fixture emits columns 1 and 36; unchanged inputs preserve all formats/IDs.
Frozen task023 source_api TP9->10, FN5->4, FP3 and all holdout cells unchanged.
Five detector RED cases plus actual fixture RED were observed before the fix;
seven detector characterizations and one real CLI/schema regression now pass.
Final offline checks: Rust154, Python20, Node22, build/fmt/clippy/diff all exit0.
Independent review024 approved full change, source bindings and measurements.
Evaluator marks task024 done; task023's maintainer budget gate is unchanged.
Only the authorized local task-specific commit remains; no publication.
