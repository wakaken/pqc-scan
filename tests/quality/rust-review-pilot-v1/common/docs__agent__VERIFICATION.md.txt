# Verification and Completion

These are mandatory details of [AGENTS.md](../../AGENTS.md). Read this file
before any change and before claiming completion, including documentation-only
work. Apply every relevant change category; overlapping categories accumulate.

# Development and Verification

Use the Rust toolchain specified in `rust-toolchain.toml`.

Typical commands:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

Example CLI run:

```bash
cargo run --bin pqc-scan -- scan ./examples \
  --format all \
  --out-dir ./target/pqc-report \
  --rules-dir ./rules
```

## Verification by Change Type

For Rust implementation, Rust tests, Cargo configuration or dependency changes:

- Run relevant targeted tests during development.
- Run `cargo fmt --all` and inspect resulting changes.
- Run `cargo test --workspace`.
- Run `cargo clippy --workspace --all-targets -- -D warnings`.
- Run `cargo build --workspace` when validating executable behavior,
  packaging or a required build deliverable.
- Exercise affected optional features and supported configurations.
  Default workspace tests do not establish coverage of disabled features.

For rule changes:

- Validate rule schema and unique IDs.
- Run relevant positive and negative detection tests.
- Run workspace tests and required task-specific acceptance checks.
- Run Rust checks above when Rust source or test code also changes.

For documentation-only changes:

- Check references, links, commands and consistency with actual behavior.
- Check agreement between English and Japanese documentation where applicable.
- Rust builds and tests are not required unless the documentation change
  affects executable examples or its acceptance criteria require them.

For CI, scripts or packaging changes:

- Run the relevant syntax checks, script tests and packaging checks.
- Validate affected workflows or produced artifacts as required.
- Local checks do not substitute for required CI execution.

For user-facing behavior:

- Exercise the affected CLI or report workflow.
- Verify compatibility and relevant error paths.
- Follow the browser verification requirements for report presentation
  or interaction changes.

## Verification Evidence

- Record the source revision, relevant uncommitted changes, command,
  feature configuration and exit result.
- Preserve logs or artifacts needed to assess required acceptance criteria.
- A HEAD SHA alone is insufficient when the worktree contains tested changes.
- Reuse passing results while their source, inputs and environment remain
  applicable. Repeat checks when changes or unresolved concerns invalidate them.
- Do not write tests that merely mirror implementation details.
- Use assertions that establish observable requirements and relevant failures.
- Do not claim an unexecuted, unavailable or failed check passed.

# Completion and Partial Results

Mark a task `done` only when every original acceptance criterion and required
deliverable has been implemented and verified with evidence matching the
final source, including the required review stages.

- An artifact list, passing tests or a limited review does not establish
  completion of requirements outside its scope.
- Do not mark a task done while required work is Partial, Unknown, Not Tested,
  failed or deferred.
- Recording a limitation or transferring work to another task does not satisfy
  the original criterion.
- Do not narrow acceptance criteria or reinterpret scope without explicit
  user agreement.
- Distinguish completion of an evaluation from measured product quality.
  Finding a product failure can be a valid completed evaluation only when
  the evaluation itself is complete and valid for every required criterion.
- Incomplete truth, missing measurements, unresolved required reviews and
  unverified assessment validity remain unfinished work.
- Continue work on resolvable issues.
- If a blocker prevents completion, keep the task unfinished and explain:
  - what is blocked and why
  - what was attempted
  - which acceptance criteria remain unmet
  - the concrete action or decision needed to proceed
- When reopening an incorrectly closed task, preserve historical evidence,
  record why the previous completion decision is superseded, and reconcile
  the task queue and roadmap without rewriting old measurements.
- Cancellation is an explicit maintainer decision, not implementation completion.
  Preserve its reason and history. Reopening requires an explicit new request.
