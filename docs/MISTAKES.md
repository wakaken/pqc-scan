# Known Mistakes

Recurring issues discovered during development.

---

## Example

Mistake:
Breaking API compatibility.

Cause:
Response format changed.

Lesson:
Maintain backward compatibility.

---

## Example

Mistake:
Migration failure.

Cause:
Missing schema constraints.

Lesson:
Test migrations locally.

---

## CLI Testability

Mistake:
CLI command logic was coupled directly to `std::process::exit`, which made exit-code behavior hard to test.

Cause:
Argument parsing, command execution, and process termination were handled in the same path.

Lesson:
Return an explicit exit status from command logic and keep `main` as a thin wrapper.

---

## Documentation Drift

Mistake:
Agent-facing docs referenced files that no longer existed or had different names.

Cause:
Documentation paths changed without updating all workflow and knowledge documents together.

Lesson:
When doc locations change, verify every referenced path and update all agent guidance in the same patch.

## Shared target directories can invalidate feature-specific protocol evidence

Cause:
An isolated optional-feature resolver test failed while default builds shared its
CLI executable path. Concurrent replacement was suspected, but the failed CLI's
hash was not captured, so its cause remains unconfirmed. A dedicated target-dir
rerun succeeded.

Lesson:
Use a separate Cargo target directory for each protocol feature matrix. Bind both
the test executable and the launched CLI binary to hashes before execution, and
read completed logs before reporting success. Preserve failed attempts separately
from successful reruns.


## LLVM coverage counters need a bounded and attributable collection path

Cause:
LLVM diagnostics (including debug dumps on stdout) contaminated machine JSON.
Accumulating per-process raw profiles exceeded the study's artifact budget.
A source module compiled through both the CLI and a path-included test also
showed ambiguous file aggregation; env-cleared children did not inherit the
profile destination.

Lesson:
Verify a small compile/run/merge/export round trip before a workspace run.
Separate machine output from diagnostics, merge profiles incrementally and bind
executables/source hashes before retiring scratch caches. Preserve ambiguous or
missing counters separately from proven execution gaps; never weaken a product's
child environment boundary merely to collect coverage.

### Miriのbuild script失敗を依存crateの非対応と早合点しない

固定版Cargo/Miriでは汎用target runnerがhostのbuild scriptにも適用され、native ELFをMiri用JSONとして誤読する場合がありました。依存なしのbuild.rs有無で再現を比較し、生成物の形式と同版工具のhost/target処理を照合します。今回の固定版ではhost runnerの明示で解消しました。対処後も、元crateでの試験、工具の正負対照、OS/FFIの制約と時間上限は別に確認します。[044の追加調査](../tasks/044/TASK-044-MIRI-FOLLOWUP.md)を参照してください。
