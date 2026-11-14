# Task 025 report naming follow-up

User-selected name: **Cryptographic Inventory and Evidence Report**
(**暗号資産・検出根拠レポート**). This is pqc-scan's own JSON format, not a shared
standard named Native JSON. Implemented on `codex/task-022-sarif-location` from
clean base `7e4d0ce226fed69fe6573462aa63dde02a71425f`.

## Compatibility

| Selection | Additional output |
|---|---|
| No report flag, including `--format all` | None |
| `--inventory-json` | `scan.inventory.v1.json` |
| Original `--native-json` | Original `scan.native.v1.json` |
| Both flags | Both files, byte-identical, from one capture |

`--repository-scope` accepts either or both flags and remains optional. Scope
without a report flag is rejected. `write_inventory_json` is the new report API;
`write_native_json` retains its signature, filename and missing-capture error.
JSON fields, IDs, core APIs, scanner/detectors, rules, dependencies, default reports
and exit precedence are unchanged. The existing schema path and `$id` are retained;
its only change is the descriptive title. Both READMEs and the
[current contract](../../docs/architecture/INVENTORY-REPORT-V1.md) explain the new name.
The old contract path is a forwarding document. Prior task 025 specification,
review and benchmark records remain historical rather than being rewritten.

## Validation

Executed with Rust 1.93.1, workspace default features and offline Cargo resolution:

- Workspace debug/release builds and `cargo fmt --all --check` succeeded.
- `cargo test --offline --workspace`: **177 passed**.
- `cargo clippy --offline --workspace --all-targets -- -D warnings`: passed.
- Python quality-harness tests: **20 passed**. Node report/projection tests: **22 passed**.
- The new flag initially failed the actual CLI tests with an unknown-argument
  error; the implementation then passed the existing schema/semantic golden and
  new single/combined option tests. Coverage includes optional scope, required
  capture for scope, and exit 0/2/3 with completeness precedence.
- Saved task 025 release binary versus new release binary on the same current
  source/root/rules/one-thread input: six profiles (old default/native, new default/
  native/inventory/both) all produce **814 findings**. The six existing reports and
  diagnostics agree after removing only generation time. Full inventory payloads
  agree after normalizing scan/time/observation IDs and their references. Scoped
  occurrence IDs/assets and all other facts stay exact. Both output files in a
  combined run are byte-identical without normalization.
- CI expectations rebind only **37 existing entries in two files** affected by
  source movement. No new omission is added. Existing source lines, rule IDs,
  columns, counts, paths and reasons remain unchanged. Projection is
  **814 raw = 660 omitted + 154 retained**, with **99 source bindings verified**.

[Source/binary/schema/golden bindings and check results](evidence/task025/INVENTORY-NAMING.json)
identify the exact pre-commit source. Detailed logs and raw artifacts are local
under `target/task025-rename/`, including `verification.json`, `legacy-shadow.json`,
`registry-rebind.json`, `code-scanning-audit.json`, and the original red-test log.
The helper plan is
[2026-11-10-inventory-report-name.md](../../docs/superpowers/plans/2026-11-10-inventory-report-name.md).

[Independent review](TASK-025-INVENTORY-NAMING-REVIEW.md) approved the implementation
without P0/P1/P2 findings. The reviewer independently ran the four CLI tests,
checked schema-only-title change, checked existing source/expectation bindings,
recomputed the CI projection and compared the two combined-run files byte-for-byte.
Coordinator workspace checks are separate evidence from those independent runs.

## Limits and unexecuted work

This follow-up changes naming and output selection. Inventory completeness, weak
occurrence identity, dependency provenance, trust verification and privacy limits
remain as documented in the contract. Existing historical internal/native names
remain in stable APIs, machine fields and schema identifiers for compatibility.
The existing help parser/output/exit behavior is outside this rename and unchanged;
its new descriptive text is checked without freezing that older behavior as a new
contract. Existing files in an output directory are not deleted by option selection.

Performance was not remeasured for this naming-only change; core and detectors are
unchanged, and no new performance claim or numeric budget is made. The original
measurement record remains available. Remote CI, platform matrix, push, PR, merge
and publication were not performed. No HTML/UI behavior was changed.
