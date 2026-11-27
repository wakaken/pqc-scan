# Measurement harness repair evidence

Scope is run.measure and a focused behavioral test file. No product, rule, schema, scorer, truth or resource-metric semantics changed. Baseline and final source hashes are retained alongside the patch.

The RED command ran eight behavioral tests against the original harness and returned exit 1 with 11 failed subtests. Positive control, existing empty directory compatibility and existing timeout group cleanup passed. The failing behaviors were prior evidence overwrite, leader exit descendant lifetime, interruption cleanup and invalid timeout acceptance.

The initial combined GREEN attempt returned exit 1 because sparse checkout omitted tracked crates/ and examples/ inputs. Its logs are retained. After authorized read-only sparse expansion using `git sparse-checkout add crates examples` (exit 0), the same combined command passed all 27 available tests, with one product-binary opt-in test skipped. Commands and exit codes are in the matching command text files.

The final patch retains the signature, measurement fields, wait4 per-child usage collection, units and classification. Empty pre-created directories are accepted. Nonempty measurement directories are rejected before execution. Exclusive log creation also avoids concurrent truncation. POSIX process groups receive SIGKILL after normal leader exit, timeout or KeyboardInterrupt; an unreaped leader is reaped with wait4 or wait fallback. Finite positive timeout validation occurs before output creation.

Validation is local macOS POSIX with wait4. Windows process-tree supervision and children intentionally escaping their POSIX process group are outside this patch and were not tested. POSIX cleanup reaps the directly owned leader; orphan descendants are terminated by process group and reaped by their OS owner. No full scanner evaluation, Cargo checks, independent approval, commit or push was performed here. Parent integration and independent correctness/test-adequacy assessment remain required.
