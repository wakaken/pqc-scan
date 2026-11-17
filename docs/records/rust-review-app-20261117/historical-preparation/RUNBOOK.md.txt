# Operator procedure (not executed)

## Before authorization to run

1. Inspect the six fixtures, `private/provenance.json`, `private/truth.json` and
   `private/validation.json`. Have the maintainer/designated human freeze the
   labels against source before any trial. Preserve the authored version and
   record the approved version/digest; do not overwrite historical labels.
2. Confirm the same model, reasoning effort, speed/service tier and client for
   both arms. Proposed model: `gpt-6.1-sol`, medium effort, Standard speed. This
   is an experiment proposal, not a model override for the current work.
3. Agree the run purpose, 24 independent contexts maximum, spend limit and
   evidence destination. Do not treat case splitting as permission for extra
   subagents under [review policy](../../../docs/agent/REVIEWS.md). No experiment
   agent has been launched. Preparation reviews are separate from trial results.
4. Establish a disposable environment with an empty working directory and no
   repository/history, host directory mounts, skills/plugins, MCP servers,
   superpowers, hooks or inherited agent instructions. Expose only one request
   via stdin and a result destination to the host operator. Permit the model
   service connection, but no browsing, shell/file tools or other model actions.
   Provision credentials through the approved runtime without copying secrets
   into packets or logs. Do not edit the user's installed configuration.
5. Verify isolation and loaded instructions without issuing a model request.
   `--sandbox read-only` alone does not restrict file reads or prevent skill
   discovery. `--ignore-user-config` alone does not prove absence of plugins,
   instructions or tools. If the chosen client cannot enforce the boundary,
   leave execution pending and select a compliant harness before running.

## Trial execution after explicit authorization

Local help was inspected on 2026-11-17: `codex-cli 0.144.1`; `codex exec --help`
lists the flags below. Neither a model request nor a CLI review was executed.
This is a command template for a provisioned isolated runtime, not a runnable
host-isolation recipe. Output/billing/tool limits require an external supervisor
or a harness supporting them; these CLI flags do not enforce those limits.

For each item in `private/schedule.json`, stage its verified packet as
`/input/request.txt` in a new isolated runtime and choose an opaque result ID.
Do not send schedule, role, family, commit or expected label metadata as an extra
prompt. The existing review template and case source are already embedded.

```sh
codex exec --ignore-user-config --ignore-rules --ephemeral \
  --skip-git-repo-check --sandbox read-only --cd /trial/empty \
  --model gpt-6.1-sol -c 'model_reasoning_effort="medium"' \
  --json --output-last-message /results/t01-review.md - \
  < /input/request.txt > /results/t01-events.jsonl 2> /results/t01-stderr.log
```

Use a fresh context and runtime for each of 24 attempts, one at a time. Do not
resume a trial. A host supervisor must stop a trial after five minutes, reject
any tool invocation and prevent automatic retries. Four thousand visible output
tokens is a requested report budget; the cost estimate separately includes
reasoning/output headroom. Fix enforceable total billable-token and spending
limits for the actual harness before execution. Retain the supervisor exit,
CLI exit, timestamps, failures and partial outputs. Never put credentials in logs.

Before each attempt check the global spend and reserved cost of that attempt
against the agreed ceiling. Stop if telemetry or enforceable bounds are missing;
do not assume that a short visible answer means low billed usage. If the model,
tier or instruction footprint differs from the frozen configuration, stop the
comparison and preserve the deviation. No automatic replacement runs.

## Scoring and evidence

Retain the packet hash, model/client/tier/settings, actually loaded instructions,
input/cached-input/output/reasoning token receipts, cost basis, start/end/exit,
raw event stream, final report and any attempted tool invocation per trial.
Missing usage is Unknown, not zero. Record a subscription run's credits separately
from the API-equivalent estimate; do not charge both in a single total.

The operator anonymizes/shuffles reports and keeps the mapping private. A human
who did not author the trial outputs scores against frozen truth before seeing
arms. Match one root cause per positive case; merge repeats within a report and
record duplicates. For controls, absence means only that nominated historical
defect is absent. Verify novel claims blind, preserving original labels; do not
automatically mark every unlisted claim false or every historical repair perfect.
Use the definitions and invalid-case handling in the experiment design. Timeouts,
errors and truncated/incomplete reports remain separate and prevent a complete
paired success claim. Do not discard failed trials from a favorable average.

Store source hashes, frozen truth, all 24 attempted outcomes, arm-blind scoring,
mapping and cost/time summaries under the agreed evidence destination. Do not
modify the current quality catalog or prior measurements. Broader validation,
fixture changes, additional trials, skills and lints require subsequent scope.
