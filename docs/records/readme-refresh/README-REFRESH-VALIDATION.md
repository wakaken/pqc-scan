# README and roadmap refresh validation

Date: 2026-11-10. Branch: `codex/scanner-quality-improvements`. Runtime checkpoint: `1f61dfa6b3fa6050c4ec4af5f2efe9883cfbe242`. The initial worktree was clean. The existing task 028/029 commits were pushed to this feature branch at the user's request before the documentation refresh. No PR or merge was requested or performed.

## Scope

The refresh covers the English/Japanese READMEs, the roadmap checkpoint, a task-to-phase alignment table, a historical-snapshot notice in the gap analysis, and current architecture descriptions of output names/privacy limits. The source of truth for lifecycle remains the task YAML. No code, rules, schemas, task status or numeric budget was changed. Japanese prose follows the requested local `humanizer-jp` skill; technical identifiers, commands, counts and references were checked separately from wording.

Task 023 remains review. Phase 0 has eight done tasks plus that open approval; Phase 1 has three done tasks. Later tasks used its reviewed technical measurement baseline without approving the protocol/budgets. The current report name and removed CLI switch, 119-rule count, opt-in artifact conditions and limited privacy contract now agree across the current documents. Historical task specifications are retained as provenance.

## Fresh local verification

Executed through a temporary development verification script; exact command arrays and exit statuses are saved in `target/readme-refresh/commands.json`, with per-command stdout/stderr and `facts.json`. These generated artifacts are local and ignored by Git.

| Check | Outcome |
|---|---|
| Recompute all 76 task029 runtime-source SHA-256 bindings and release binary hash | Match accepted task029 state |
| `cargo fmt --all --check` | exit 0 |
| `node --test shell/*.test.cjs` | 28 passed, zero failed |
| `python3 -m unittest discover -s tests/quality -p 'test_*.py'` | 20 passed |
| `pqc-scan rules list --rules-dir rules` and `rules show JWT_RS256` | exit 0; 119 loaded rules |
| `scan examples --format all --lang en/ja --diagnostics --inventory-json --repository-scope pqc-scan/readme --policy-profile migration-review-v1` with separate fresh output directories | Both exit 0, nine expected files each, 289 findings each |
| English/Japanese JSON, CBOM, dependency SBOM, SARIF, diagnostics comparison | Equal after generation-time normalization; no content-based masking differences between languages |
| `scan examples --format sarif` | exit 0; SARIF and two flat inventories only |
| Default `scan examples` | exit 0; six legacy reports, no opt-in artifacts |
| `--require-complete --fail-on high --diagnostics` | exit 3 after reports; precedence retained |
| `--fail-on high` | exit 2 |
| Removed `--native-json` | exit 1, no output directory created |
| `scan --help` | Usage matches documented options; existing CLI returns 1 and prints help through the error path |
| Parse phase/status from task018–039 YAML; locate task headings within roadmap phase sections | Twelve task assignments match; 023 review, eleven done, 030–039 pending |

The help exit is existing behavior, not a new passing exit-0 claim or a fix in this refresh. An initial check incorrectly assumed `rules list` output was one line per rule; it actually contains a summary and entries. The script was corrected to verify its explicit `rules loaded: 119` summary and rerun successfully. The initial assertion failure is preserved in `target/readme-refresh/verification-initial.log`; no production change was made to satisfy it.

Product build/workspace Rust tests/clippy, full performance measurement, official schema validation and browser acceptance were not rerun for this documentation-only change. Task029's accepted build/test/clippy/schema/corpus evidence is retained as historical evidence for the identical source hashes, not reported as newly executed here. The fresh checks above exercise documented CLI selection and exit semantics; they do not measure general detection accuracy or prove universal secret removal.

## Documentation checks and review

Local Markdown targets, relevant roadmap anchors, fenced code blocks, bilingual command examples, the humanizer punctuation/wording checks and `git diff --check` are checked before the documentation commit. Independent review covers implementation-record alignment, current versus historical claims, source-level CLI/report selection and English/Japanese fact parity. The final review decision is recorded separately in `README-REFRESH-REVIEW.md`.

Current-head hosted CI, other operating systems, an actual global `cargo install`, public endpoint/product scans, PR creation and merging are not part of this verification. Pushing the feature branch may trigger hosted CI; this document does not claim that run has passed.
