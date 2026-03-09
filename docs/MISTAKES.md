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
