# Corpus and OSS quality

## Sub-features

Reviewed detection truth, per-family confusion counts and denominators, inspection coverage, repeatability, privacy, source/input bindings, and separate actual OSS populations.

## How to get to it (user POV)

The checkout's quality harness is `tests/quality/run.py` and `tests/quality/report.py`. Its frozen in-tree corpora contain synthetic cases and licensed OSS excerpts. The independent full OSS coordinator is a separate capability and is unavailable in this checkout. `examples/scripts/scan_oss_products.sh` is a convenience clone-and-scan script, not a frozen accuracy evaluation.

## Driving it with the quality harness

Set `VERIFY_BINARY` to the exact built CLI from a run manifest and `VERIFY_RUN` to a fresh evidence directory. Use the actual profile of that binary. Do not overwrite an old output directory.

```sh
python3 tests/quality/run.py --binary "$VERIFY_BINARY" --build-profile debug \
  --corpus-dir tests/quality/expanded-v2 --out-dir "$VERIFY_RUN/expanded-v2" \
  --repeats 3 --skip-benchmarks
python3 tests/quality/report.py --corpus-dir tests/quality/expanded-v2 \
  --baseline "$VERIFY_RUN/expanded-v2" \
  --applicability tests/quality/applicability-docs-readability-20261124/assessment.json \
  --out-dir "$VERIFY_RUN/expanded-v2-assessment"
```

Read `.github/workflows/pqc-scan.yml` for the current applicability file before each run. An outdated README is not authority to bypass a source-binding failure. Additional current CI populations are the default baseline, `expanded`, `expanded-v2-lock-controls`, and `task043-corpus-v2`. Reuse the same runner with a separate output for each, pass `--skip-benchmarks`, and use the matching `report.py` invocation from CI. Keep old corpus versions and historical measurements separate. Run relevant quality-harness tests when changing those scripts.

For actual OSS evaluation, locate an explicitly supplied checkout containing `benchmark/README.md`, `benchmark/entry.py`, `benchmark/registry.json`, frozen input manifests, and protocol. The currently observed separate worktree is `/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan`; its existence is not a portable precondition. Read its README and WORKFLOW before using it. Its entry command accepts a selected population, frozen binary and rules, matching build receipt, protocol, and a unique output. Bind dirty source, binary SHA-256, rules digest, feature list, and build profile. Preserve fixed upstream commits, file manifests, licenses, subset decisions, and unavailable inputs. Do not copy that work into this branch or change another task's outputs. Do not replace frozen commits with latest clones.

## Gotchas

Accuracy and completed scanning are different claims. Keep TP, FP, FN, eligible denominators, ambiguity, unsupported/skipped cases, completeness, and deterministic semantics explicit. Do not turn no predictions into 100% quality. `--skip-benchmarks` intentionally omits performance evidence. Full OSS coordinator runs keep accuracy unknown until independent truth and evaluation exist. Missing external assets remain unavailable, never an empty successful population. Do not silently lower thresholds, edit truth to match results, or claim CI passed from local runs.
