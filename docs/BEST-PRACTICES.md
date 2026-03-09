# Best Practices

This document contains normative development guidance for this repository.
Use it for rules contributors should normally follow.
For recurring repository structures and extension styles, see `docs/PATTERNS.md`.

---

## Code Quality

- prefer small, readable functions with minimal nesting
- keep public entrypoints thin and delegate real work to testable helpers
- preserve CLI behavior, output stability, and backward compatibility
- prefer explicit return values for control flow over deep process termination

---

## Testing

- add unit tests for new logic
- add integration tests where behavior crosses module boundaries
- keep tests deterministic
- for CLI code, separate argument parsing, command execution, and process exit so exit codes and output can be tested without spawning a subprocess
- prefer repository-local temporary fixtures and checked-in rule sets so scanner tests remain stable
- for security-sensitive detector paths, use synthetic fixtures and assert both positive detection and masking behavior

---

## Security And Compatibility

- validate inputs and avoid exposing secrets
- do not weaken cryptographic checks or compatibility guarantees
- never emit raw private key material or sensitive cryptographic data
- keep report and CLI contracts stable unless the change is explicitly planned and documented

---

## Documentation

- keep agent-facing document references aligned with the actual repository layout
- when renaming or relocating docs, update `AGENTS.md` and related knowledge docs in the same change
- keep this document focused on prescriptive guidance; move reusable repository structures to `docs/PATTERNS.md`
