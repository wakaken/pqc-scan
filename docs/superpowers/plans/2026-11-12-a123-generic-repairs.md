# A1–A3 generic inspection repairs implementation plan

**Goal:** Fix the seven observed input failures through general grammar/format handling, retain existing changes, regenerate ten reports for all 14 OSS projects, and prepare one reviewable branch for a later develop PR.

**Architecture:** Retain bounded static detection, original evidence/coordinates, native identity and serialized fields. Extend the existing parse-only compatibility layer for TS/TSX, parse Bundler platform suffixes against declared PLATFORMS without guessing versions, and route Dockerfile names before extension-based AST selection. No target execution/decryption or repository-specific exceptions.

**Stack:** Rust 1.93.1, pinned tree-sitter grammars, existing offline report validators and confined Linux scanner.

**Approved scope:** User approval in this conversation of A1–A3, following `target/pqc-residual179-lvwvto5l/SUMMARY.ja.txt` (original workspace). Main agent implements; only two reviewer identities: implementation then final-test review. No automatic commits/push/PR/merge in this preparation request.

## Constraints and review focus

- Keep current `codex/inspection-coverage-fixes`; snapshot all pre-existing dirty files before edits.
- Never special-case OSS names, paths or the seven exact filenames.
- Preserve source bytes/positions in parse-only adaptation and evidence; invalid syntax remains incomplete.
- Empty type aliases differ from empty function/class type parameters; dynamic imports differ from import types.
- JSX quoted attributes differ from JS expressions/comments/templates and malformed entity-like text.
- Bundler resolved registry versions differ from original platform-qualified reports, GIT/PATH and undeclared/invalid platforms.
- Dockerfile/Dockerfile.* naming differs from ordinary source suffixes and similarly named directories/files.
- Keep previously approved non-decryption, privacy, IDs, report formats and unrelated worktree changes.

## Steps

- [ ] Add positive/negative and coordinate/privacy/native-provenance regressions in `crates/pqc_scan_detectors/tests/a123_repairs.rs` and CLI integration controls. Observe expected failures first.
- [ ] A1: extend `tree_sitter_detector/syntax_compat.rs` for empty alias parameters, reliable JSX quoted attribute ampersands and import types in assertions/function-return positions. Validate bounded type-context candidates, preserve original source, retain malformed controls.
- [ ] A2: extend `dependency_detector/bounded.rs` to recognize declared non-ruby platform suffixes, retain original reported values and canonical resolved registry versions, and accept standard two-space Bundler metadata. Verify unknown platforms and GIT/PATH do not gain fabricated resolution.
- [ ] A3: general Dockerfile naming recognition in `tree_sitter_detector.rs`; lexical inspection remains applicable. Test Dockerfile.java/go/ts and ordinary sources/near names.
- [ ] Update both README/coverage docs and durable evidence describing exactly the changed behavior.
- [ ] Recheck the seven original fixed-source cases with the final detector. Start one implementation reviewer, fix findings and reuse that reviewer.
- [ ] Run fmt, workspace/default/all-feature tests, clippy all-target/all-feature -D warnings, Python/Node quality checks. Freeze/hash final source and build offline Linux scanner.
- [ ] Fresh default-branch checkouts into a temporary directory; fixed SHAs, no target execution. Run 14 confined scans, product-prefixed ten-format files; check consistency, official schemas, privacy and real browser desktop/mobile/keyboard/loaded-offline behavior. Keep original visible report tab.
- [ ] Start a different final-test reviewer; resolve findings with that identity. Package reports, exact residual counts/categories and per-change explanations.
- [ ] Audit branch ancestry/uncommitted patch application against fresh origin/develop. Record session branches and already-merged predecessors, keep independent quality-assessment worktree untouched, and provide a single-branch PR preparation path.
