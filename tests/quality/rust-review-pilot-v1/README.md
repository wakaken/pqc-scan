# Historical Rust review pilot: prepared inputs

Status: prepared inputs; app execution is now authorized separately. This is a six-case, text-only
review dataset. No new product feature, synthetic Rust implementation, runnable
test, skill, dependency, Clippy configuration or CI job is added. Historical Rust
is stored as `.rs.txt`; never compile or execute these excerpts. The repository
[MIT license](../../../LICENSE) applies to the copied project source.

The current request authorizes fixture/label preparation and a concrete method
and estimate. The later user instruction authorizes the app comparison; see the active runbook. See the
[experiment design](../../../docs/quality/RUST-REVIEW-EXPERIMENT.md),
[operator procedure](RUNBOOK.md), [cost estimate](COST.md) and
[preparation handoff](PREPARATION.md).

## Contents and visibility

- `cases/<opaque-id>/`: exact historical source excerpts and the bounded review
  contract. Each source map identifies original file/line ranges.
- `common/`: byte-identical snapshots of existing project review guidance.
- `requests/`: twelve self-contained request packets, six each for A and B.
  These embed the existing `prompts/REVIEWER.md`; they are filled review packages,
  not a replacement review prompt. B adds only the frozen supplement.
- `private/truth.json`: six source-based labels, three known-defect snapshots
  and three controls where that nominated defect was repaired. No control is
  asserted free of all possible defects. Human truth freeze is pending.
- `private/provenance.json`: full source/fix/parent commits, Git blob IDs,
  SHA-256 bindings and exact extraction ranges. Source bytes are unedited.
- `private/schedule.json`: frozen seed 20261117, 24 sequential trials, reversed
  arm order in the second repeat; all outcomes remain `not_run`.
- `private/common-bindings.json`, `private/input-size.json`: policy bindings
  and rough request sizes. See `private/manifest.json` for artifact integrity.

Only one selected request packet may enter a trial. Never mount this directory,
the repository, `.git`, private labels/provenance, the schedule or both arms into
the model environment. Filenames for staged inputs/outputs use trial IDs, not
arm/case IDs. The preparer and preparation reviewers necessarily know truth;
they are not blind experiment participants or independent holdout authors.

## Case selection and label scope (operator-only)

| Pair | Historical repair | Nominated root cause |
|---|---|---|
| c17 / c42 | `f993691d3cce90920b8ce2706b0a1c508672d5ab` and its first parent | Ambiguous tuple concatenation in finding IDs. |
| c63 / c08 | `b0e386e9d8503cf422c310e85fbb2a429bfb57d2` and its first parent | Embedded source newlines escaping Markdown code indentation. |
| c29 / c51 | `35d988e03629cb15b17a94af81b1e74266a344ff` and its first parent | Native unknown coordinate zero serialized as a SARIF position. |

The first ID in each row is before repair; the second is after. These targeted,
public historical examples can have appeared in model training. They are not
independent samples or blind holdout. Current production contains later changes;
the old defects are not findings against today's worktree. Copies exclude fix
tests and commit messages from trial packets to avoid directly disclosing labels.
Source comments remain verbatim, even when they explain a repaired invariant.

Score review precision/recall separately from scanner Q1-Q6. The pilot does not
test generated-code quality, actual browser behavior, cryptographic judgments,
secret-redaction completeness, public API migration or product performance.

## Preparation verification

Check every excerpt against its full Git revision and line range, every packet
against its common source plus its case and optional supplement, and every
schedule item against the packet hash. Verify exactly three positive labels,
three scoped negative labels and balanced arm/repeat coverage. Keep validation
results in `private/validation.json`. These are preparation integrity checks,
not model trials, Rust test runs or evidence that review quality improved.

Original preparation receipts remain historical. The active runbook and cost
note now describe the authorized app method. Source, labels, packets and planned
schedule remain unchanged; actual run outcomes are stored separately.
