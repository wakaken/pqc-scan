# Regression investigation

Existing TASK-023-ACCEPTANCE.md conditions are unchanged: fixed eligible truth TP cannot decrease; FP/FN cannot increase. Same-condition warm maximum time/RSS >2x previous warm maximum is an investigation signal, not automatic CI failure. No threshold was revised. Any revision requires measured variation and maintainer approval.

Comparable >2x signals: []

If any signal exists, retain it and examine raw sample distribution, host load, launcher bounds, source/binary/config and repeated controlled measurements before calling it product regression. Historical 2026-11-11 large-workload 2x finding and safety-I/O investigation remain in FINAL-GAPS-VALIDATION.md; they are not overwritten by this new matrix. Accuracy nonregression is checked using the unchanged 023-v2 truth in separate accuracy runs; the new product-source workload has no accuracy truth and is not used for TP/FP/FN.

## Actual immutable-truth comparison

The default and all-feature 023-v2 runs both measure TP28/FP4/FN2/TN19. Against the immutable historical baseline, holdout/source_api and holdout/config FP increase: the approved numeric nonregression result remains **Failed**. See ACCURACY-COMPARISON.json and existing TASK-043-FIXES-RESULTS.md for the previously investigated annotation/interpretation mismatch. This is not a new product acceptance, a threshold revision, or a detector fix. Evaluation completeness and measured product quality are separate.
