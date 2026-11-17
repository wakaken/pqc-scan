# Agent comparison and operational recommendation

Date: 2026-11-17. Source: commit
`04a6ef5587684f2204e250b85c0e01e9bbb83688` and the 24 immutable
[app reports](rust-review-app-20261117/RESULTS.md).

The user requested an assistant-led comparison and recommendation instead of
personally inspecting the reports. The main agent read all 24 reports against
their scoped source contracts and frozen labels, then compared the 12 matched
case/repeat pairs. This is an unblinded agent judgment, not independent human
adjudication. It satisfies the newly requested operational comparison; it does
not retroactively complete the original human-blind evaluation. Original truth,
reports, pending human scorecards and collection records remain unchanged.

## Recommendation

Keep condition A, the existing review instructions, as the default. Do not
automatically add the supplement, install BeautifulRust or expand AGENTS.md
based on this pilot. Retain the short supplement as optional reference material.
This is a recommendation under limited evidence, not proof that B or BeautifulRust
cannot help other tasks. No additional trial, installation or policy activation
was performed to reach this recommendation.

## Comparison

| Observation in the six scoped cases, each repeated twice | A | B |
|---|---:|---:|
| Nominated defects identified in positive report opportunities | 6/6 | 6/6 |
| Negative controls without a false actionable claim | 6/6 | 6/6 |
| Adequate cause or control explanation | 12/12 | 12/12 |
| Positive reports with a fix compatible with case scope | 6/6 | 6/6 |
| Disagreement with the prepared Markdown severity | 2 reports | 2 reports |
| Extra finding fragment for the same SARIF root cause | 1 | 1 |
| Median recorded app-turn time | 16.95 s | 18.31 s |

These are the main agent's scoped judgments, not externally validated precision
or population recall. A repeated report is not an independent defect. No extra
novel actionable claim or harmful advice was found in the reports reviewed.

Both conditions explain why distinct ID field tuples yield identical hash input,
rather than alleging a SHA collision; they preserve the need for a separate
compatibility decision before changing IDs. Both give concrete Markdown newline
examples and identify the missing continuation indentation. Both explain the
SARIF zero-coordinate omissions and preserve known positions. Their fixed-case
explanations also identify the relevant framing, indentation or serde omission.

B occasionally adds a useful explicit verification suggestion, such as CR/LF
cases in t03 or the zero/known coordinate combinations in t06. A already provides
concrete triggers, boundary conditions and appropriate fixes, for example t04,
t05, t10 and t15. B did not consistently supply a missing causal explanation,
find a defect A missed, or avoid a false positive made by A.

The line-zero and column-zero findings are split in t05 (A) and t17 (B), and
combined in the other SARIF positive reports. The frozen protocol counts them
as one root cause. This is comparable reporting fragmentation, not a second
detected defect or established false positive.

The prepared Markdown label is P1, while all four positive reports use P2 or
medium. The limited fixture establishes rendering integrity loss but supplies no
browser execution or consuming-renderer threat evidence to settle priority.
Preserve the disagreement as unresolved severity calibration; do not silently
change the label or turn it into an A/B advantage. It does not alter the finding
that both conditions identified the same root cause and proposed the same remedy.

B sends 4,005 additional UTF-8 bytes on every request. Actual billed tokens and
incremental money are unavailable. Timing varies in both directions: B was
faster in five matched pairs and slower in seven. The timing observations do not
prove a speed regression. The recommendation favors A because extra input has
no demonstrated added review value here, not because an actual cost penalty or
statistically significant latency difference was established.

## Limits and disposition

The cases comprise three public historical repairs, and their contracts direct
attention to the nominated behavior. A already succeeds throughout this small
sample, leaving little room to demonstrate an improvement. The experiment cannot
resolve other Rust defect families, unsupplied call paths, full-crate reviews or
current develop's updated guidance. Actual app model/settings and hidden context
were not exposed. Official BeautifulRust skills and MCP tools were never invoked.

No further manual inspection is required from the user to use this operational
recommendation. The original independent-human-blind study remains unfinished;
this record neither calls it complete nor makes it a prerequisite for keeping A.
Do not start more trials or change defaults merely to seek a different outcome.
The [assessment ledger](rust-review-agent-assessment-20261117.json) records each
trial's judgment and source-report hash so this recommendation is inspectable.
