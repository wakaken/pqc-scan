# Independent review — task 022

Reviewer: independent Reviewer agent (separate context from implementer).
Date: 2026-11-10.
Base/working HEAD: `1335033fb3ffd96632f9f39a51bf8f2a115f2ead`.
Branch: `codex/task-022-sarif-location`.

## Initial review

Reviewed the uncommitted implementation, writer and actual CLI contract tests,
vendored schema/provenance/license, dev-only validator dependencies, shell helper
and Node regression, both README changes, task specification and implementation
plan. Consulted AGENTS.md, prompts/REVIEWER.md and the required architecture,
knowledge and lifecycle documents. RESTART-LOOP.md applies only to task 014 and
the restart baseline, as its own scope explicitly states.

No actionable correctness, security or compatibility findings in this scope.

The writer preserves native locations and finding IDs by projecting only at the
SARIF boundary. Unknown lines omit regions even with nonzero native columns;
unknown columns omit startColumn; empty paths omit locations. UTF-8 bytes and
literal percent/query/fragment/colon delimiters are encoded once. Unix literal
backslashes and conditional Windows drive/UNC behavior are consistent with the
documented contract. Existing source snippet generation preserves the unmasked
prefix; ASCII-only checking through the native AST offset is conservative for
the legacy mixed byte/code-point positions, and missing/masked/clipped evidence
causes abstention. Source files are not read by the report writer.

The nullable expectation column does not act as a wildcard: it matches only an
absent SARIF column, while source hash, complete canonical result hash, rule,
line and total count remain required. Encoded or traversal paths are ineligible.
No production detector, public native model or ID algorithm changed.

## Independently executed checks

- `cargo test --locked --offline -p pqc_scan_report --test sarif_contract -p pqc_scan_cli --test sarif_locations`: exit 0, 5 writer tests and 1 actual CLI test passed.
- `node --test shell/prepare-code-scanning.test.cjs`: exit 0, 19 tests passed, including actual scanner and exact null-column binding cases.

The schema test validates emitted SARIF against the pinned official schema with
formats enabled, asserts its digest, and rejects zero-position negative controls.
Semantic URI and Unicode fixtures provide assertions beyond schema validity.

## Remaining review boundary

CI registry rebindings and the final quality/contract evidence document are being
prepared by the coordinating agent and have not yet been reviewed. This initial
review is not final whole-change approval. Windows conditional code was inspected
but was not executed on a Windows host. Full build/workspace/fmt/clippy and old/new
artifact comparison are coordinator validation gates, not independently executed
by this reviewer in this pass.

## Final whole-change review

Final outcome: **APPROVED**, with no actionable findings.

Reviewed the final expectation registry, CI-SELF-SCAN.md, task quality/contract
record and appended malformed-column controls. Independently compared the old
registry from HEAD to the new registry: all old paths remain; old rule IDs,
lines, counts and review reasons are unchanged; exactly 109 existing groups
change from a known column to null and receive new complete-result digests.
Independently matched every one of the 618 old approved observations to the old
raw scan and its new raw counterpart. Each counterpart is byte-equivalent as a
JSON object, permitting only removal of startColumn for the rebound groups.
The only changed old source binding is the appended helper test. All 84 current
source hashes match their actual files.

The sole new source registration is sarif_contract.rs, with six observations on
lines 114 and 116. Inspected these lines: they are the synthetic Unicode RS256
input and token lookup in the column calculation test. They are appropriately
classified as test input, and no production path is newly registered.

Re-executed `node target/task022/verify-projection.cjs`: exit 0; 752 raw, 624
reviewed omissions, 128 retained, 84 verified sources. Its metadata, raw bytes,
complete retained objects and previous retained population checks passed.
Re-executed the helper Node test suite after final negative controls: exit 0,
19 tests passed. The new controls retain explicit null, zero and negative SARIF
columns even when complete result hashes are explicitly matched.

Checked all seven SHA-256 bindings published in the quality record against the
current source files; all match. Reviewed compatibility/summary.json and final
build/test/lint logs, and ran `git diff --check` successfully. Coordinator's full
workspace/build/fmt/clippy and all-Node results are documented as coordinator
evidence; this reviewer independently executed the narrower checks above.

The remaining limitations are accurately stated: no hosted Actions/upload,
remote consumer acceptance, Windows execution, broad platform matrix or
performance/accuracy benchmark. They do not prevent approval of this local-only
SARIF projection correction. Final commit identity is assigned after review;
the record correctly binds the reviewed uncommitted code by source digests.

## Final fixture portability amendment

Reviewed the subsequent test-only amendment removing the Windows-forbidden `?`
from the actual CLI fixture filename (`日本 😀 #%?.rs` to `日本 😀 #%.rs`)
and removing the corresponding `%3F` from that test's expected URI. The writer
string fixture still tests question-mark encoding, so semantic coverage remains.
No production implementation changed. The updated CLI source digest is
`240337072dce8fa9e830ce2de7acf673db4ba52c95dfe6112e1b1492ccdb95da`;
both the quality record and source-digests.json match the current file.
Independently reran `cargo test --locked --offline -p pqc_scan_cli --test
sarif_locations`: exit 0, 1 test passed. Approval remains unchanged.
