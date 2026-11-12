# Semantic and inspection coverage repair plan

Goal: correct the confirmed misses, false classifications, syntax/dependency/certificate gaps; adjudicate 113 source files; regenerate all ten report artifacts for 14 OSS products.

Architecture: retain the current branch, native identity scheme, export schemas, privacy policy, bounded inspection and approved release decisions. Extend existing detectors with language-aware evidence and narrowly validated parsing support. Partial, malformed and encrypted inputs must retain truthful completeness. No execution of target programs, builds or package managers.

Execution: main agent investigates, implements, tests and documents. User explicitly requests execution on the existing branch. Exactly one implementation reviewer after the combined implementation and a different test reviewer after final testing; reviewers do not delegate. Reuse each for fixes.

Baseline: preserved file hashes and source copy in /tmp/pqc-semantic-repairs-run.json. Previous investigation: target/inspection-followup-20261112 in the original workspace.

- [ ] Reproduce misses/false positives with default rules; add failing positive/negative regression tests. Trace the 113 unresolved source errors and record per-file evidence and disposition.
- [ ] Fix Python/Ruby calls and aliases, classical EC/EdDSA and Node RSA key generation; prevent HS256/AES, unrelated identifiers and non-Go build paths from being called RSA/Go usage. Preserve coordinates, deduplication and masking. Update rule reference and both READMEs.
- [ ] Add bounded grammar support for confirmed valid syntax (Go expressions, TS import types/JSX, Ruby methods, Rust editions/macros, V8/source-phase dialects as appropriate). Prefer upstream fixes; do not turn arbitrary parse errors into Complete. Raise the finite AST candidate capacity to handle the measured generated file. Keep malformed positive controls.
- [ ] Improve static dependency declaration grammar (npm/catalog/alias/patch/GitHub, Go directives, Bundler sections, Maven nested metadata/properties, pip hash options). Distinguish parsed declarations from resolved/effective versions; no dynamic evaluation. Correct non-SBOM classification without accepting malformed real SBOMs.
- [ ] Distinguish certificate applicability from key/CSR/CRL/parameter inputs; validate known PQC public key shapes from authoritative specifications, preserve encrypted PKCS#12 uncertainty, mixed-container privacy and malformed certificate failures.
- [ ] Run targeted regressions; invoke implementation-review subagent on combined delta and prior changes. Resolve findings and reuse that reviewer if needed.
- [ ] Run fmt, workspace/default and all-feature tests, all-target/all-feature clippy, relevant Python/Node quality checks. Freeze/hash final scanner, build isolated Linux executable.
- [ ] Obtain current upstream checkouts in temporary storage, record exact SHAs, scan 14 products in bounded offline read-only containers; preserve old artifacts, product-prefixed ten-format outputs. Reconcile counts and remaining diagnostics, validate schemas, report privacy and real browser layouts/interactions from a dedicated loopback server.
- [ ] Invoke distinct final test-review subagent with final source/test/report evidence, resolve findings, repeat only affected tests/scans. Package reports and record limitations accurately. No commit/push/PR requested in this task.

Review focus: direct/aliased Python and Ruby RSA without algorithm literals; negative HS256 and unknown JWT algorithms; Go import versus unrelated path text; valid syntax around real crypto references and invalid nearby constructs; finite resource limits; no fabricated dependency versions; malformed and encrypted certificate controls; native source offsets and private material redaction; final reports bound to the final reviewed source.
