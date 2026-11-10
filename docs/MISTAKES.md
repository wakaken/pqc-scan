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
