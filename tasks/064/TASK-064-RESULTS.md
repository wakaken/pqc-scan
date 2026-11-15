# Task 064 results

New preparation accepts exactly `static` and `network`. Network means static
scanning with explicit TLS and SSH endpoint diagnostics, including both SSH
advertisement collection and pinned hybrid SSH key exchange. The name replaces
`all-probes`; the intermediate combined distribution is no longer prepared.
Cargo feature switches and product commands are unchanged.

Packaging, dependency assessment, the manual CI workflow, installation checks,
Python tests and paired user documentation use the two names. The preparer rejects
both old selectors; verification and installation still accept historical
static/combined-probes/all-probes archives. No schema structure was changed.
The workflow retains static-only attestation and performs no release upload.

## Verification

Source: `f07d782cd68cfac95d93585d5ace049f590160d4` on
`codex/task051-dependency-supply-chain-review`, plus the uncommitted changes
bound in [SOURCE.json](evidence/consolidation-20261114/SOURCE.json).
The [baseline](evidence/baseline/SOURCE.json) and preexisting diff separate the
completed task051/task063 changes from this request.

- `python3 scripts/run-dependency-review.py --image sha256:16a329d30bd8f2b8d12353b678777392203d3e3f567505709a8ca4a408f95bb1 --preparation target/quality-task-063/preparation/audit --out-dir target/quality-task-064/dependency-review-r1`: exit 0. Bounded offline container: 25 distribution tests passed and real dependency evaluation exited 0. Static: 139 packages, 0 notice flags. Network: 271 packages, 4 notice flags. Both: 0 RustSec findings against the recorded database snapshot. [Run and boundaries](evidence/consolidation-20261114/dependency-review/RUN.json), [test/evaluation exits](evidence/consolidation-20261114/dependency-review/artifacts/checks.json).
- `python3 target/quality-task-064/run-native.py`: exit 0. Two actual macOS arm64 binaries packaged twice each, with byte-identical archives per variant; new static/network archives installed and exercised successfully. Three historical archives also passed current installation checks. [Commands/exits](evidence/consolidation-20261114/native/COMMANDS.json), [completion](evidence/consolidation-20261114/native/COMPLETION.json), [local archive hashes](evidence/consolidation-20261114/native/LOCAL-ARCHIVES.json). Archives remain local under `target/quality-task-064/native-r2/`.
- No Rust/Cargo inputs changed in task064. Reused task063's built/tested native binaries after verifying original archive/binary hashes and exact equality of the complete crates/rules/Cargo/toolchain file set and hashes. [Reuse bindings](evidence/consolidation-20261114/native/BINARY-REUSE.json) link the earlier build/workspace test evidence. Packaging provenance omits some extension types; the reuse check supplements them from task063's complete validation source map. The initial controller stopped before packaging on that overly broad set assertion; it did not accept a mismatched file. [Initial-run explanation](evidence/consolidation-20261114/INITIAL-RUN-NOTE.md).
- Workflow YAML syntax, Python compilation, relative documentation links, matching English/Japanese distribution command examples, `git diff --check` and final-source hashes: exit 0. [Final checks](evidence/consolidation-20261114/FINAL-CHECKS.json). Workflow semantics are covered by the distribution tests; hosted Actions were not dispatched and no hosted pass is claimed.

## Publication conditions

The four objc2-family notice questions remain and network publication remains on
hold under the existing maintainer decision. Renaming does not resolve these
conditions. Preparing hosted provenance for network and final release validation
would also be needed for a future publication, outside this request. No publishing,
release, signing, tag, upstream message, commit, push or merge was performed.

Independent review outcomes are recorded in [the review record](TASK-064-REVIEWS.md).
