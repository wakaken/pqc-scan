# Rust review supplement (pilot)

Optional reference for the treatment arm of the
[review experiment](../quality/RUST-REVIEW-EXPERIMENT.md). This is not a skill,
a second review prompt, or a new acceptance gate. It is not automatically required
for normal reviews. Use [the existing template](../../prompts/REVIEWER.md),
[review policy](REVIEWS.md), and [verification rules](VERIFICATION.md).
[AGENTS.md](../../AGENTS.md) and explicit project safety and compatibility
constraints take precedence over external guidance.

## Review the change and its consequences

- Trace external input to panic, indexing, arithmetic overflow, narrowing/sign
  conversion, allocation and resource limits. An `expect` message does not make
  an input-dependent panic safe; an `unwrap` in a test is not automatically a bug.
- Check that custom `Eq`, `Ord`, `PartialOrd` and `Hash` agree on identity. Look for
  lost or duplicated findings and unstable ordering, rather than trait presence alone.
- Check ownership and borrowing for unintended copies, stale cloned state and
  unnecessary shared mutation. Prefer a small local correction; do not replace
  deliberate ownership, `Arc`, trait objects or parallelism just to match an idiom.
- Preserve actionable errors and partial results. Follow failure paths and cleanup;
  do not replace a documented recovery policy with a blanket panic rule. Error,
  `Debug`, assertion and logging output must not expose sensitive input.
- For changed unsafe/FFI code, check the actual lifetime, aliasing, bounds and
  resource invariants. A `SAFETY` comment describes the argument; it is not proof.
  Check lock scope, callbacks and cancellation where the change involves concurrency.
- Check affected public failure/precondition documentation and tests against the
  behavior. Prefer observable assertions, boundary cases and negative controls;
  passing logs do not establish that the relevant requirement is tested.

## Keep scanner requirements separate from Rust idioms

- Preserve CLI defaults/exit behavior, public signatures, existing detector
  adapters, report fields, finding identity, ordering and reference integrity.
  Do not privatize fields, replace bool parameters, seal traits, change error types
  or add `non_exhaustive` to existing APIs merely to satisfy a general guideline.
- Keep unknown, unsupported, failed, partial and complete inspection distinct.
  No findings is not proof of safety; a legacy adapter is not proof of completeness.
- Follow sensitive data through collection, snippets, diagnostics and final
  exports. Keep read boundaries, finite budgets and non-execution of scanned input.
- Do not infer PQC protection from a name or static reference. Keep declaration
  versus use, offered versus negotiated, and KEM versus signature facts distinct.
  Rust idioms do not replace the [PQS-QAM evidence](../quality/QUALITY-MODEL.md).

## Report only justified findings

Use the existing response format. Give the file/line, reachable cause, observable
impact and relevant requirement or Rust contract. Suggest the smallest compatible
fix when useful. Separate a demonstrated defect from a question or optional style
preference; do not invent a finding to fill a checklist. Reuse supplied test evidence
unless a specific concern needs reproduction. Do not edit files, rerun the full suite
automatically, add dependencies/lints, or expand review scope.

## Source

Project-specific synthesis informed by BeautifulRust's
[rust-review](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/skills/rust-review/SKILL.md)
and [rust-best-practices](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/skills/rust-best-practices/SKILL.md)
at commit `b9718436d77ee7bcf17cdcb6aa27628feb95d97a` (MIT).
This pilot deliberately omits their automatic tool runs, broad skill triggers,
standalone report format and blanket API/style prescriptions.
