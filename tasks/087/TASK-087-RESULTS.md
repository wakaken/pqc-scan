# Selective source masking

Status: complete. All four original criteria, final-source checks, four-report browser acceptance and both retained independent review stages are accepted.

## Source and behavior

Baseline commit31a463803446e42d7d4e989087feb3f11337b4f3 plus the19owned files in [source09 manifest](evidence/IMPLEMENTATION-SOURCE-09.json), SHA256edf1f4bede643843c7d3006bb8e402b5f1862420cae6a465b3a80d26fe6f01f8. The manifest also binds336runtime/input files. [Combined patch](evidence/implementation-09.patch). Changes remain uncommitted; no new commit/push, release, version bump, rule or storage-schema change is selected. Prior task086 API-only results remain historical and unchanged.

The user explicitly requested original benign arguments and surrounding source. PHP/C#/C/C++ no longer blanket-mask all source findings. Core uses the inspected source snapshot, sanitizes recognized spans before clipping and constructs an original five-line window (up to two lines before and after). A recognized call retains its actual variables, public selectors, messages and other benign arguments, rather than an `arguments omitted` substitution. Existing display length limits remain.

Protection covers recognized private-key blocks, named credential values, URL credentials, literal private API inputs and their bounded local initializers. Adjacent public code remains readable. Metadata preserves its existing arbitrary-prefix credential suffix policy. Named initializer handling has a256wrapper bound; key-reference/alias and inspection limits retain conservative fallback. PHP privacy reuses canonical imported names from the existing resolver. None of these display spans feed typed source facts. Unknown/uninspected source regions remain conservatively protected; this is bounded recognized-syntax masking, not a universal secret detector.

Java/Go and the other pre-existing languages already had conditional credential/private-key protection. They did not have the new four-language unconditional flag. Regressions verify normal source remains readable and real recognized secret patterns still disappear. The fix does not offer a privacy opt-out.

## Original acceptance criteria

1. Ordinary source arguments and surrounding context: targeted27 includes real scanner tests for all four new languages, existing seven-language normal fixtures, Java/Go conditional privacy, and UTF16 C++.
2. Recognized secret values protected before clipping: actual scan regressions cover key API roles, named/positional inputs, scoped initializers/aliases, C arrays, C# byte initializers, PHP array keys/imported aliases, deep wrappers, public verification keys and computed expressions. Core covers multiline/Unicode credentials, exact private blocks and caller metadata. Every selective scan asserts nonempty findings and default/native parity, including native absence of synthetic secrets.
3. Compatibility: detection rules, parser selection budgets, structural storage fields and native typed-fact policy remain unchanged. Original source call excerpts are presentation evidence and explicitly excluded from CBOM algorithm restoration. Real report identity/coverage and CBOM comparisons are recorded below.
4. Required checks, consumer scans, real browser and two stages: implementation source09 accepted by the retained reviewer; final checks, consuming scans and browser acceptance below are complete. Independent test review accepted the same final source.

## Verification and measurements

All nine checks in [CHECKS-09.json](evidence/CHECKS-09.json) exit0 against final source09. Each exact command, duration, log and logSHA is recorded. CARGO_INCREMENTAL=0; configured rust-toolchain preserved.

- `cargo test --workspace --locked`:983passed,0failed,3ignored.
- `cargo test --workspace --all-features --locked --target-dir target/v05-feature`:1042passed,0failed,5ignored.
- `cargo fmt --all -- --check`, default/all-feature Clippy with all targets and warnings denied, workspace build and release build: exit0.
- All nine Node test files:134passed,0failed,0skipped.
- `git diff --check`: exit0.
- Meaningful RED failures and subsequent corrections are preserved in the [review history](TASK-087-REVIEWS.md). In particular, source08’s final run was interrupted after exposing a performance regression. Final source09’s unchanged large Unicode header test passes in0.35seconds. No interrupted or superseded run is counted as a final pass.
- [Six-language consuming export controls](evidence/EXPORT-PRIVACY-CONTROLS-09.json): final release CLI scan, all formats, inventory and diagnostics exit0. HTML/Markdown/JSON/SARIF/CBOM/SBOM/native/diagnostics contain no synthetic secret canaries; normal message text remains visible. Includes UTF16 C++, PHP imported aliases/named seed/keypair/quoted credential arrays, C array initializer, C# byte initializer, nested private conversions, password URI-like payloads, Java and Go. The first export fixture placed its C# public label outside the five-line window; its failed assertion and output are preserved in EXPORT-CONTROL-DIAGNOSIS-09.json. The corrected fixture puts the public label and private array within the actual consumed window; privacy assertions are unchanged. This is consumer evidence, not external accuracy or runtime cryptography evidence.

## Four displayed pinned scopes

[Rescan receipt](evidence/OSS-RESCAN-09.json) binds the final binary/source, exact commands, original task085 receipt/inventory/report/rule hashes, full input-tree validation and selection bindings before/after, scope/coverage equality and findingID/rule/location/severity/risk signatures. Only four previously displayed scopes are selected. All original inputs and task085/086 artifacts remain unchanged.

| Scope | Findings before/after | Readable highlighted source locations before/after |
|---|---:|---:|
| XMLSecLibs PHP |27 /27|12 /17|
| ITfoxtec C# |95 /95|40 /87|
| curl C |169 /169|20 /164|
| Bitcoin C++ |69 /69|0 /69|

The readability comparison uses original task085 reports, before either display fix. It counts nonempty highlighted snippet lines not starting with a mask, not precision/recall or a promise of every finding being verbatim. Remaining protected findings include actual private-key fixtures/containers and conservative context. Bitcoin src/key.cpp:219 exactly matches the original highlighted source line; its window contains217–221. Normal arguments and public neighbouring statements are present.

[Complete CBOM comparison](evidence/CBOM-COMPATIBILITY-09.json) preserves row counts and every field except recovered public algorithm-display values: PHP5of27rows, C#39of87, curl144of169, Bitcoin69of69. Main inspected all distinct recovered values: fixed public source/library labels and existing lexical selectors (TLS/SSH/RSA), with no original call or argument excerpts. This is explicit value recovery, not complete CBOM byte equality or newly inferred typed crypto facts. Selective scan native/default parity and frozen882 source tests pass.

[Displayed copies and URLs](evidence/DISPLAYED-REPORTS-09.json) bind the eight served report/rule files to new immutable outputs. The dedicated loopback server57385 is retained. [Actual browser observations](evidence/BROWSER-09.json) include Japanese source windows for all four languages, English C++ evidence, screenshots,1280x720 layout with no horizontal document overflow, Enter collapse/open, and a loaded duplicate that paginates/opens embedded rule explanation/closes with Escape after its separate57487 server is stopped. The duplicate is closed; the four requested report tabs remain open. [C++ original source screenshot](evidence/browser-cpp-ja-context.jpg).

No new full30benchmark, accuracy estimate, migration-assessment rerun, cryptographic runtime validation, release-readiness decision or new commit/push was selected. Both retained independent review stages accept the same final source with no actionable findings. [Final artifact audit](evidence/ARTIFACT-INTEGRITY-09.json) verifies source, logs, final/release binaries, all generated and served report bytes, synthetic inputs/outputs and browser artifacts, and preserved task086 records/binary. [Review decisions](evidence/REVIEW-DECISIONS-09.json) and [completion record](evidence/FINAL-INTEGRITY-09.json). No required work remains.
