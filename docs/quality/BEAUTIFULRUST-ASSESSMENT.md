# BeautifulRust assessment and disposition

Decision date: 2026-11-17. The maintainer accepted continuing **condition A**, the
existing pqc-scan review process, and deferring BeautifulRust adoption for this
iteration. No skill/plugin/MCP installation, default supplement activation,
AGENTS.md expansion, Clippy/CI change, product refactor, API/CLI change or report
format change is part of this work. No additional experiment is planned.

## Investigation and fit

Investigated [BeautifulRust](https://github.com/morganlinton/BeautifulRust/tree/b9718436d77ee7bcf17cdcb6aa27628feb95d97a)
at commit `b9718436d77ee7bcf17cdcb6aa27628feb95d97a`, with plugin metadata v1.6.0.
This records the inspected revision, not a claim about the latest release.

| Component | Purpose | Disposition for pqc-scan |
|---|---|---|
| `rust-best-practices` | Broad Rust ownership, API, errors, concurrency, unsafe, docs and test guidance. | Useful reference concepts; no automatic/full-skill adoption. |
| `rust-review` | Scoped, prioritized findings with locations, reasons and fixes. | Informed the optional short supplement; the existing review template and stages remain authoritative. |
| `rust-port` | Behavior-preserving migration from other languages. | Outside the current Rust review objective; not adopted. |
| Clippy tables/configuration | Pedantic and individual lints, panic/unsafe/docs checks, restrictive bool-argument threshold. | No bulk import or lint change; existing pinned-toolchain checks remain. |
| Plugin MCP connection | External guidance/lint/crate lookups. | Not installed or invoked in the comparison. |

Source references: [writing skill](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/skills/rust-best-practices/SKILL.md),
[review skill](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/skills/rust-review/SKILL.md),
[porting skill](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/skills/rust-port/SKILL.md),
[lint table](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/lints/lints.toml)
and [Clippy configuration](https://github.com/morganlinton/BeautifulRust/blob/b9718436d77ee7bcf17cdcb6aa27628feb95d97a/lints/clippy.toml).

The project already has strict Clippy in CI, deterministic contract tests,
source-bound Q1-Q6 evidence and two-stage independent review. Generic Rust
material can add explicit checks for tuple/trait contracts, reachable panics,
unsafe invariants and error documentation, but does not replace that foundation.
Blindly following API-shape advice can break existing public fields, bool
parameters, error types or detector adapters. Automatic tool reruns conflict
with evidence reuse; a second report template conflicts with the single project
review prompt. Extra text and repeated validation also consume context and time.

Rust idioms and scanner assurance are distinct: borrow/clone/error/trait advice
does not establish detection truth, inspection completeness, PQC classification,
secret redaction, bounded hostile-input handling or output compatibility. Those
remain governed by existing project contracts and [PQS-QAM](QUALITY-MODEL.md).

## What was actually compared

**This was not a trial of the official BeautifulRust skills or MCP tools.**
Condition A used frozen project instructions and existing Rust excerpts.
Condition B added a project-specific, roughly 500-word supplement informed by
BeautifulRust. No official skill invocation or MCP tool call was performed by
the experimental subjects. Both conditions were instructed to use static reading
only, and zero model tool calls were observed.

Three historical repairs supplied six source snapshots: ambiguous finding-ID
field encoding, Markdown continuation indentation, and unknown SARIF coordinates,
each before and after repair. Two arms and two repeats produced 24 separate
projectless Codex app chats. The source/labels/packets, raw receipts, reports,
hashes and failed validation attempt are retained. The CLI/API proposal was
superseded by app execution; its original text remains historical evidence.

| Main-agent assessment within the nominated cases | A | B |
|---|---:|---:|
| Positive review opportunities with the root cause identified | 6/6 | 6/6 |
| Negative controls without a false actionable claim | 6/6 | 6/6 |
| Adequate cause/control explanation | 12/12 | 12/12 |
| Scope-compatible fix guidance on positive reports | 6/6 | 6/6 |
| Median observed app-turn duration | 16.95 s | 18.31 s |

B supplied 4,005 additional UTF-8 bytes per request without a demonstrated
incremental finding or consistent explanation advantage. Time varied in both
directions; it is not evidence of a causal performance penalty. Actual per-trial
tokens and monetary cost were unavailable. The shared account allowance moved
from 53 to 54 percent consumed, including parent/reviewer/other work; that delta
is not an experiment-only bill. No purchase or usage reset was performed.

## Interpretation and limits

The user requested an assistant-led operational recommendation instead of
personally grading all reports. The main agent read all 24 reports and the
12 matched pairs, knowing both labels and conditions. The resulting assessment
supports keeping A under the current evidence. The maintainer accepted that
decision; this closes the adoption decision for this iteration.

It does not establish independent human-blind quality acceptance or a general
absence of benefit from BeautifulRust. In particular:

- Only three public historical defect families were sampled, with correlated
  before/after snapshots and two repeats. These are not 24 independent defects.
- The contracts explicitly point at the behavior to inspect; A already succeeds
  throughout, leaving little room to distinguish B. Other Rust issues and current
  develop's updated guidance were not evaluated by these frozen packets.
- Actual app model, reasoning settings and hidden instructions were not exposed.
  Fresh chats separate conversation history but do not demonstrate OS isolation.
- Original Markdown labels use P1 while both arms report P2/medium. That priority
  discrepancy remains unresolved and is not silently corrected after observing
  results. SARIF line/column splits count as one nominated root cause.
- Independent human-blind scorecards remain unfilled. The assistant assessment
  is a separate, explicitly unblinded decision aid; no historical result or
  unfinished formal evaluation is relabeled complete or cancelled.
- No full-crate implementation/refactor trial, official skill/tool trial, scanner
  accuracy improvement, crypto assurance or product performance gain was measured.

## Evidence and retained material

- [Design and execution history](RUST-REVIEW-EXPERIMENT.md).
- [Six source cases, provenance, truth and frozen A/B requests](../../tests/quality/rust-review-pilot-v1/README.md).
- [App collection, limitations and raw report index](../records/rust-review-app-20261117/RESULTS.md).
- [Unblinded agent comparison and per-report assessment](../records/rust-review-agent-assessment-20261117.md).
- [Unactivated supplement](../agent/RUST-REVIEW-SUPPLEMENT.md), retained as study material.

Frozen source and response data intentionally retain original wording, languages,
Markdown hard-break spaces and source-ending blank lines. They are evidence,
not instructions to restore the superseded CLI method or old review-prompt paths.
The original reports, labels, pending scorecards and artifact hashes are preserved.
