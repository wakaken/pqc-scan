# Quality checks with performance held

Performance, CPU, RSS, load and swap observations remain held by the operator.
Current-source performance is unknown. Historical measurements remain preserved.

## Guarded selection

`scripts/quality-with-performance-held.py` checks exact Python source paths,
SHA-256 pins and the static/discovered test inventory before selecting tests.
The policy binds 135 sources under `benchmark`, `tests` and `scripts`.
Of 459 original quality methods, 414 are selected and 45 observation methods
remain held. Held methods do not count as passing. Ordinary skips are recorded
separately.

Public `bounded_process.execute` and `task047.study.run_target` retain their
original input validation. Their private execution continuations are guarded
before directories, clocks or child processes can be used. This allows the two
original negative-input validators to run without observation. The original
test bodies remain unchanged. Other observer entries and low-level resource
observers remain guarded; a caught forbidden attempt still fails the run.
This is a fixed reviewed source selection, not a sandbox for arbitrary code.

## CI entry points

The ordinary `.github/workflows/pqc-scan.yml` PR quality job and the
`.github/workflows/release-quality.yml` workflow use the guarded selector.
The latter accepts pull requests and manual dispatch so its registration can
be checked before default-branch integration. It includes default and isolated
all-feature Rust checks, distribution/report checks, synthetic guard controls,
temporal validation and offline evidence aggregation.

The original baseline measurement job is preserved but explicitly disabled
while the hold applies. The separately paused protocol job remains paused;
neither skipped job is represented as passing.

The launcher starts under `python3 -I`, removes inherited `PQC_*`, `PYTHONHOME`
and `PYTHONPATH`, then execs the selected interpreter with `-B` and bytecode
suppression. The runner inserts canonical quality and benchmark paths once.
The receipt preserves held tests, ordinary skips, forbidden attempts and a null
performance result. Output directories are exclusive and historical evidence
is not overwritten.

## Local verification and evidence

The previous 133-source selected run recorded 410 passes and two ordinary skips;
its 47 held methods included two negative validators then blocked at entry.
The later 135-source integration and list-only review are historical evidence
for their own frozen inputs. They are not relabeled as execution of this change.

The current input-validation separation has 24 controller tests on Python 3.12
and 3.14, including copied original negative tests and valid-call rejection before
effects. Independent implementation/test review is assigned to
`codex:gpt-6.1-sol@high`; the decision-trail audit uses
`codex:gpt-6.1-terra@high`. The current actual selected result and final source
binding belong to the publication evidence index, not the historical receipts.

## Remaining limits

A guarded result is not a full original quality pass, a performance result,
hosted CI success or release approval. Hosted checks and exact-source
static/network distribution provenance must be checked on the actual published
source. Missing, failed and held checks remain explicitly distinguished.
