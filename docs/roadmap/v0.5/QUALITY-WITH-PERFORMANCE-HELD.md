# Quality checks with performance held

This internal reference describes the manually dispatched
`.github/workflows/release-quality.yml` workflow and its guarded quality subset.
Performance, CPU, RSS, load, and swap observations remain held by the operator.
The performance result remains unknown.

## Guarded selection

`scripts/quality-with-performance-held.py` reads
`scripts/quality-performance-held-policy.json`. Before project imports, the
runner checks the complete Python path set and SHA-256 values under
`benchmark`, `tests`, and `scripts`. The integrated policy binds 135 sources,
including the new runner and its synthetic controller. A path, digest, or test
inventory change requires policy reassessment and independent review.

The policy lists 459 original quality methods. The runner selects 412 and holds
47. Held methods remain unexecuted and are never counted as passing.
The two `heldReasons` entries identify unchanged negative-input tests held
because their observer entry points are unconditionally guarded. These two
validators have not been reverified or waived.

The runner checks static and discovered inventories and guards the reviewed
observer entry points before quality discovery. Any recorded observer attempt
fails the run, including an attempt caught by a test. Receipts keep ordinary
skips separate from held methods. This selection trusts fixed reviewed source.
It is not a sandbox for arbitrary or concurrently changing code.

## Manual workflow

The workflow has only a `workflow_dispatch` trigger. It runs formatting,
workspace tests, clippy, all-feature checks in a separate Cargo target,
distribution tests, report tests, the guarded quality subset, the temporal
witness, and offline evidence aggregation. It first runs the 21 synthetic
selector controls with:

```sh
python3 -I -B tests/ci/test_quality_performance_hold.py
```

The guarded quality step starts a stdlib-only launcher with `python3 -I`.
The launcher removes inherited `PQC_*`, `PYTHONHOME`, and `PYTHONPATH`
variables, enables bytecode suppression, and executes the selected interpreter
with `-B`. The runner inserts the canonical quality and benchmark paths once.
The quality receipt is written under `target/quality-performance-held/`.
The workflow preserves that directory and `target/quality-integrated/`.

Workflow registration and dispatchability at an authorized ref remain pending.
The local workflow file is not evidence that GitHub can dispatch it.
No remote workflow has been dispatched as part of this integration.
Fresh independent review of the integrated source and policy is required
before running the integrated selector against the real quality modules.

The existing `.github/workflows/pqc-scan.yml` remains unchanged by this
integration. Its original full quality suite and default baseline include
measurement paths. That workflow is not appropriate while the observation
hold applies.

## Source-bound local evidence

Before integration, the reviewed external v3 runner completed the selected
412 original methods against 133 pinned Python sources. The result was 410
passes and two ordinary skips, with no failures, errors, or recorded observer
attempts. The 47 held methods did not run. Performance remained unknown.

The preserved receipt is
`guarded-quality-local-canonical-path/selected/RESULT.json` in the local
`review-resumption-c13be11edbc44a13bb7791e194fa5b21` evidence directory.
`guarded-quality-local-canonical-path/EXECUTION-RESULT.json` records its
commands and source binding. The earlier `guarded-quality-local` attempt
failed before discovery because the parent Python path duplicated a namespace
path. Both attempts remain preserved.

Integration adds two Python files and binds all 135 current sources.
The original 133 sources remain byte-identical. The prior selected-run result
continues to describe its original snapshot. It does not certify execution
of the integrated 135-source version.

The integration author runs only the 21 synthetic controller cases on Python
3.12 and Python 3.14. Those cases use private fixture repositories.
They do not import or discover the actual project quality suite.
The author receipt, source freeze, commands, and fresh logs are retained under
`integration-author/` in the same local evidence directory.

## Remaining limits

Synthetic controls and a guarded subset do not establish a full original
quality pass, hosted CI success, or release approval. Original required
validation remains outstanding where it is held, missing, or unavailable.
No held test, historical evidence, scorer, gold data, or original test has been
changed to turn an incomplete result into a pass.
