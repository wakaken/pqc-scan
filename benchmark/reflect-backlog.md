# Long-running evaluation follow-ups

Status: recorded only; no implementation selected by this reflection.

These local devex items come from the current pstack reflect review. They do not authorize scans, cleanup, builds, publication, or additional release gates. No external tracker was configured in the inspected active references; external contact remains outside the authorized scope. Reuse the existing runner and evidence metadata rather than constructing parallel frameworks.

## REFLECT-001: Compute the remaining evidence obligations

- Status: open.
- Tags: devex, evidence-reuse.
- Evidence: decisions.tsv:115 retained authentic observed timestamps for 41 subjects instead of requiring additional proof for all 98.
- Mechanism: derive a small remaining-work list from requirements, existing evidence subjects, dependencies, and documented applicability decisions.
- Acceptance: applicable evidence removes only satisfied obligations; changed source or conditions preserve affected obligations; missing or invalid evidence cannot become a pass.
- Limit: do not build a new general provenance platform or duplicate gold/scorers.

## REFLECT-002: Isolate failures of independent subjects

- Status: open.
- Tags: devex, execution, resume.
- Evidence: decisions.tsv:146 records two 600-second GitLab scan timeouts with 69 other subjects unstarted; this was not established to be an acquisition or network failure.
- Mechanism: record per-subject dependencies, deadlines, attempts, and retry conditions; retain failed subjects as unfinished while continuing independent subjects.
- Acceptance: a timed-out subject does not prevent an independent subject from completing; a retry needs a justified condition change; source/input/privacy/invariant failures still stop affected execution; completion excludes failed subjects.

## REFLECT-003: Admit work against retained-output capacity

- Status: open.
- Tags: devex, storage, validation.
- Evidence: decisions.tsv:152 records all-features validation failing with no space before the first test executable ran.
- Mechanism: use existing retained-output sizes, selected Cargo feature configurations, a bounded pilot's disk growth, and retry allowance to determine whether to start the next unit.
- Acceptance: insufficient capacity prevents additional launches while preserving unfinished required validation and all protected evidence; the check does not delete automatically or weaken validation; du accounting is not treated as uniquely reclaimable APFS space.
- Limit: use disk and operational metadata only; deferred CPU/RSS/load/swap/performance measurements remain deferred.

## REFLECT-004: Require parent-retrievable process results

- Status: open.
- Tags: devex, delegation, process-ownership.
- Evidence: decisions.tsv:147 records an unavailable child exec session and a subsequent parent-owned launch.
- Mechanism: delegation declares the process owner, whether its handle is operable by the parent, and durable paths for logs, progress, and terminal results; launch in the parent when cross-agent control is unavailable.
- Acceptance: the parent can read authentic progress and completion after the child's turn ends without relying on a transferred session identifier or a self-reported pass; ownership remains isolated.

## Preserved constraints

Required validation, independent technical review, source/input identity, privacy, bounded static reads, compatibility, and historical evidence remain mandatory. These items are not permission to weaken a gate or make every follow-up a prerequisite for the next bounded evaluation.
