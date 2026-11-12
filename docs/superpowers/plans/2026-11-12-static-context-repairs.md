# Static context and source grammar repairs

**Goal:** Address the 69 residual files classified as static-format gaps (56) or context-dependent inputs (13), using generic bounded static inspection; demonstrate retained findings in malformed input. Preserve the current branch, prior dirty changes, IDs, privacy and report schemas.

**Architecture/design:** Extend the existing detector adapters with an optional repository inspection context. It exposes only walker-selected files through the existing bounded, no-symlink reader. Maven may follow validated local parents and Ruby may inspect local gemspec declarations; neither may read excluded/out-of-root sources or run package tools. Keep declaration provenance and original source locations. V8 compatibility is parse-only and offset-preserving, guarded by lexical/AST context. Legacy DSA support extracts public attributes only under a documented ASN.1 layout. Template/conditional declarations are inventoried as conditional candidates, never as installed/effective dependencies. Missing runtime/renderer context remains explicitly partial with a fixed safe explanation.

**Alternatives:** Executing Maven/Ruby/templates would introduce effects and invented environment assumptions, so is excluded. Dropping errors or excluding test directories would conceal gaps, so is excluded. Chosen approach follows the user's request for generic static fixes and preserves approved non-execution/non-decryption decisions.

**Tech:** Rust 1.93.1, existing tree-sitter grammars, roxmltree, x509-parser, source-offset-preserving compatibility.

## Main-agent implementation/test tasks

- [ ] Save the current source baseline, the 69-file list and excluded categories; demonstrate malformed-source partial detection with original output.
- [ ] Add failing regressions for V8 prefix/sloppy identifiers/tagged-template/runtime-negative forms, malformed controls and coordinates; implement bounded lexical/AST compatibility without accepting strict/module invalidity.
- [ ] Add legacy DSA public-key positive/malformed ASN.1 tests and documented extraction; keep cryptographic validation/trust claims unchanged.
- [ ] Add native pipeline tests for local-parent property inheritance, coordinate mismatch, cycles/depth/size, scope exclusion/symlinks, multiple profiles and property conflicts. Implement context and per-declaration conditional provenance; unresolved identities remain incomplete.
- [ ] Add Ruby DSL/gemspec positive/negative tests, local delegated-file provenance, dynamic/ENV branches and opaque evaluation boundaries. Static declarations survive without pretending to evaluate Ruby.
- [ ] Add template source/requirements tests, conditional branch candidates, original coordinates and malformed controls. Inspect static source while explicitly preserving unknown generated content.
- [ ] Update both README/coverage docs with supported behavior and exact remaining context needs; inspect all 69 unchanged OSS files.
- [ ] One implementation-review identity reviews the combined change; main fixes findings and reuses that reviewer.
- [ ] Main runs fmt, workspace/default/all-feature tests, all-target/all-feature clippy -Dwarnings and relevant Python/Node checks; freeze source and build scanner offline.
- [ ] Rescan the same pinned 14 OSS snapshots into a new ten-format result set, validate coverage/identity/schema/privacy, reclassify residuals and check real browser output without changing the existing visible report tabs.
- [ ] One different final-test-review identity checks meaningful assertions and evidence-to-source matching. Package examples, changes, residuals and branch readiness. No automatic commit/push/PR.

## Review focus

- Never confuse a parsed conditional declaration with an effective selected/installed dependency or fabricate resolved versions.
- Never read outside root, excluded files, links/FIFOs, or unbounded parent/include chains.
- Never mask arbitrary syntax errors to claim full inspection; preserve strict/module and invalid untagged-template controls.
- Preserve original evidence bytes/locations, new finding IDs, opaque encrypted containers and secret masking.
- Preserve the 75 malformed/special, 26 encrypted and 2 experimental cases except truthful reclassification backed by evidence; do not implement them merely to reduce counts.
