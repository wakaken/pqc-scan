# Codex app comparison: collection results

Status: 24/24 reports collected. Quality evaluation and independent human
adjudication remain unfinished. No skill adoption or product quality improvement
is established. Collection ran through native Codex app chats; no Codex CLI,
direct API call, trial subagent, installation, Clippy/CI/product change, purchase
or usage reset was performed.

## Collected evidence

The six cases, A/B packet bytes and schedule were reused unchanged. All 24 app-
stored initial message texts match the scheduled packet hashes. All 24 distinct
projectless chats returned complete final reports, with zero observed model tool
calls and no truncated final messages. Input transport markers are not model tool
calls. Raw app receipts, final text, thread IDs and timing are retained in this
directory; manifest.json is the operator schedule, not a scored quality report.

The app's actual model, reasoning settings, client version, hidden instructions,
per-trial billable tokens and money were not exposed by the collection API and
remain Unknown. Fresh chats separate conversation history but do not demonstrate
OS isolation. Thus the authored input control is verified, while full causal
isolation and cost comparison are not. The historical guidance baseline is the
prepared f07d782 worktree snapshot, not current develop guidance.

## Provisional observations, not blind scores

The truth-aware main agent observed the nominated root cause in all six positive
review opportunities per arm, and no nominated-defect claim in all six controls
per arm. These are observations on three public historical pairs with two repeats,
not 24 independent defects or final precision/recall estimates. No improvement
from the supplement is demonstrated by this unblinded read.

The prepared Markdown severity is P1; the four relevant outputs use P2 or medium.
That discrepancy needs human adjudication, not a post-output label adjustment.
Both arms also have one report splitting SARIF line/column into two findings;
these map to one nominated root cause under the frozen protocol.

Collection timing medians were 16.95 seconds for A and 18.31 seconds for B; totals
were 225.436 and 228.850 seconds. These app-turn timings are observations, not a
controlled speed/performance conclusion. Median response lengths were 477.5 and
503.5 characters respectively; characters are not billed tokens.

Account usage showed 53 percent consumed before the method update and 54 percent
after the 24 trials, still allowing ordinary usage. This is a shared, rounded
account window including parent/reviewer/other work, not experiment-only cost.
The run made no purchase or reset; the dollar charge cannot be determined from
these snapshots. The old API-equivalent budget is not this app run's cost.

## Remaining evaluation

The blind/ folder contains 24 unchanged shuffled reports, source/label reference
records and empty human scorecards. Keep operator-only/blind-mapping.json withheld
until human scores are fixed. Human scoring, novel-claim adjudication, severity
assessment, configuration uncertainty and final adoption decision remain open.
Do not enlarge the corpus, retune the supplement or run additional trials without
new authorization. No historical measurement or quality gate was rewritten.
