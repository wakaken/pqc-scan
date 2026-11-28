# OSS evaluation

Use these tools to evaluate pqc-scan against real open-source projects and compare scanner versions.
For scanning your own project, start with the [scanning guide](../docs/user/SCANNING.md).

## Corpus and results

The [registry](registry.json) catalogs 98 repositories. The v0.5 evaluation uses a selected subset:

| Set | Projects | Purpose |
| --- | ---: | --- |
| Main evaluation | 27 | Cryptographic implementations and API use across nine language groups |
| Comparison and regression | 8 | Check behavior on larger applications and platforms |
| Future main targets | 63 | Cataloged, outside the selected evaluation |

Keep main and comparison results separate. The main projects were deliberately selected for cryptographic coverage, not randomly sampled.

See the [35-project list](../docs/releases/v0.5.0.md#validation-scope)
and [results and evaluation conditions](../docs/quality/oss-corpus/README.md).

## Understand the results

- **Execution** tells you whether the scanner finished and its required output files passed verification.
- **Inspection** tells you which input the scanner could examine. A successful run can still report Partial, Unknown, or Truncated inspection.
- **Accuracy** requires applicable source-backed labels. Detection counts alone are not precision or recall. Unsupported or unassessed metrics remain null.

Static references do not establish runtime use or successful PQC migration. Compare results only when the input commits, file scope, and run settings match.

## Choose a project

Run commands from the repository root with Python 3:

```sh
python3 -B benchmark/entry.py --help
python3 -B benchmark/entry.py select --repo openssl__openssl
python3 -B benchmark/entry.py select --population comparison
```

`select` prints the matching registry entries without scanning or downloading source.
Use repository IDs from `registry.json` to choose individual projects. The historical `--priority` flag does not select the v0.5 main set.

## Run a new evaluation

Prepare the following before starting:

1. Verified source files at the registered commit and input scope.
2. A pqc-scan executable, its rules, and a build receipt matching that executable and those rules.
3. A protocol defining the timeout, file limits, and required outputs. Set `collectPerformance` to `false` while performance measurement is deferred.
4. An output directory outside both this checkout and the scanned source tree.

Replace the example paths with your prepared files:

```sh
python3 -B benchmark/entry.py run --repo openssl__openssl \
  --phase current \
  --binary /path/to/pqc-scan --rules /path/to/rules \
  --build-receipt /path/to/build-receipt.json \
  --protocol /path/to/protocol.json \
  --output /path/to/evaluation
```

The tool executes pqc-scan, not the scanned project's code or dependency installers.
Each attempt gets a separate directory, and the command prints a `receiptPath` for the invocation.
Add `--resume` to reuse completed attempts whose inputs, settings, and saved outputs still verify.

## Compare two runs

Use the invocation receipts printed by the two runs:

```sh
python3 -B benchmark/entry.py compare \
  --before /path/to/before/invocation.json \
  --after /path/to/after/invocation.json \
  --output /path/to/comparison.json
```

The comparison checks the saved files and pairs matching projects and samples.
Changed inputs or settings are incompatible. Missing or failed observations are unavailable.
A comparison describes changes in scanner output; it does not establish accuracy or a performance improvement.
