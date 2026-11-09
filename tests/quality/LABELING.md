# Task 023 baseline protocol, version 1

Purpose: measure the current static scanner; never change rules/detectors to fit
this corpus. These small synthetic/reference inputs do not estimate real-world
prevalence, execution, negotiated capabilities, backend availability or security.
All source fragments are data only, never imported, compiled or executed.

## Truth and scope

A truth unit is one nominal source API/import reference, draft-name reference,
configuration assignment, dependency declaration, certificate object/attribute,
or private-key marker. Each unit names a nominated rule and a one-based line and
half-open Unicode code-point column interval. Separate same-line occurrences are
separate truth units. A source reference is nominal: fragments may omit imports;
explicit local shadowing is negative. An unused/dead-code reference still counts
as static reference. An isolated algorithm string is not an API use (Ruby puts
is a negative API control). Aliases with explicit originating imports retain
positive truth even if unsupported by the current matcher. Dynamically supplied
algorithm values without a resolvable local value are ambiguous.

Dependency truth is declaration/resolution presence, never PQC enabledness.
Certificate truth is the exact public object's algorithm/size from the existing
independent OpenSSL manifest, not trust/signature validation. Key truth concerns
markers only; synthetic invalid bodies must never be described as real keys.
Malformed inputs with no decidable fact and unsupported formats/languages are
labeled unsupported, not true negative. Potentially undecidable provenance is
ambiguous. Filename alone is insufficient for an algorithm attribute.

Two fresh reviewers independently assign positive/negative/ambiguous/unsupported
labels and reasons from corpus bytes and this protocol before seeing scanner
results, each writing only their own label file. No one may inspect the other's
labels. The coordinator adjudicates disagreements in a separate ledger without
looking at scanner results; unresolved disputes remain ambiguous. Label content,
source bytes, lineage, split and licenses are pinned before measurement. The
corpus manifest deliberately has no truth labels. All metadata stays outside the
materialized scan input. No private repository or external endpoint is used.

## Split and scoring

The existing readiness examples share one tuning lineage. Public task020
certificates share another tuning lineage. All readiness-derived transformations share the readiness tuning lineage.
The fresh independently authored approval-adapter project forms a separate
holdout lineage; no lineage may cross splits. These are
public developer evaluation sets, not a secret benchmark. Holdout is frozen
before first measurement and must not be reused for tuning without a new lineage
and fresh held-out set. Duplicated benchmark copies never enter accuracy counts.

Only the rule IDs explicitly listed by family in corpus.json are evaluated;
all other scanner findings are counted as out-of-scope, never silently discarded
or described as correct. Every evaluated prediction outside an annotated unit is
an FP. A prediction matches only its file/rule/line/column interval. Unit matching
is one-to-one, so duplicate predictions become FP, and missing same-line
occurrences become FN. For native unknown dependency positions, exact dependency
name metadata may bind the declared unit; no guessed source line is inserted.
Eligible positive/negative units yield TP/FN or FP/TN. Predictions on ambiguous or
unsupported units are counted separately; these units never improve denominators.
Report per split and family: TP/FP/FN/TN, eligible and excluded populations,
precision TP/(TP+FP), recall TP/(TP+FN), and null for zero denominators. Global
micro counts are supplemental; do not average percentages or claim global
cryptography accuracy. Present missed/false/extra observations by unit/rule and
raw native location. No percent threshold is approved in advance.

## Reproducibility, coverage and privacy

Run the built CLI with explicit checked-in rules, --threads 1, --diagnostics,
--format all, default 2 MiB file cap/default exclusions and declared environment.
Use a temporary root outside ignored target ancestors; assert every manifest
file was read. Use the SAME root across repeats. The canonical semantic digest
sorts finding/inventory arrays without dropping duplicates (scoring sorts input
findings too, so assignments do not depend on report ordering), removes only top-level
generated_at and the built-in CERT_EXPIRED/CERT_NOT_YET_VALID certificate detector metadata
assessment_time_unix field and its identical snippet token (a wall-clock
observation), preserves all other facts/IDs/locations/status and separately
reports raw ordering drift. Only assessment_time_unix is volatile, not time-based
validity status. Report date boundaries can therefore legitimately change facts.
All generated artifacts live under ignored target, outside inputs.

Record diagnostics' real partitions/status, not invented completeness. Require
selected=read_ok+read_failed and read_ok=sum(file terminal states), and report
unknown/not-enumerated populations. Dependency internals still do not report
completeness; overall unknown does not mean complete. No network peers exist.
Use synthetic canaries, never keys; test absence from every output and stderr.

## Performance and evidence

Benchmark fixed generated small/medium/large/pathological workloads, recording
recipe, exact input digest, bytes/files, target sizes, binary/HEAD/source/rules/
catalog/toolchain/OS/CPU/RAM versions, profile/features and every CLI argument.
Use a first-run sample and at least three warm samples. First-run means the first
measured run after materialization, NOT OS cold cache; filesystem caches are not
flushed. Report wall, child CPU and per-child peak RSS min/median/max and raw
samples; process startup/report writing are included, build is excluded.
POSIX wait4 supplies per-child CPU/RSS with explicit platform unit conversion;
unsupported platforms report unavailable rather than process-lifetime maxima.
Timeout/failure is not a fast successful sample. No concurrent benchmark jobs.
A bounded pathological repeated-match fixture must show truncation diagnostics.
No network, package installs, real applications or cryptographic execution.

Propose budgets only after results, clearly marked unapproved and environment/
corpus-specific. CI checks the evaluator, corpus bindings, coverage arithmetic,
and repeatability, retaining measured summaries; it does not enforce an invented
accuracy/performance target or silently update baselines. Maintainer approval of
protocol/budget proposals remains an explicit review decision after measurement.


Corpus version 023-v2 groups all readiness-derived transformations into tuning. A fresh offline approval-adapter project is the independently authored source-only holdout. Do not read previous label/adjudication files or scanner results.
