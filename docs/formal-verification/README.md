# Formal verification PoC

## Purpose and acceptance

Provide reproducible evidence for conservative security decisions without changing
the CLI, public API, report schemas, catalog or policy semantics. Preserve ordinary
tests. A successful Lean model proof is not a proof of the Rust implementation.
The original request covers Kani harnesses, independent Lean specifications and
models, Rust/model comparisons, pinned repeatable execution, CI and honest results.

## Architecture and selection

Use Kani **0.68.0** on the actual `pqc_scan_core` crate, with no stubs or copied
production functions. Use Lean **4.36.0**, its bundled standard library only,
for an independent mathematical policy specification and a model of coverage
classification. Rust unit tests export exhaustive small-domain vectors; Lean
executes its own models on those same inputs and the runner compares the results.
Normal builds do not depend on these verifiers.

Kani is the smallest way to check existing Rust, including runtime safety within
declared bounds. Lean adds unbounded list/natural-number reasoning, at the cost
of a separately reviewed translation. Creusot **0.15.0** now verifies the actual
policy aggregation loop for arbitrary representable slice lengths, including
termination. Its [contracts, toolchain, trust boundary and results](CREUSOT.md)
complement Kani's bounded checks and Lean's independent model. Verus remains a
future alternative; adding a fourth prover is not needed for this PoC.
None of these tools proves detector
accuracy, active cryptographic use, cryptographic primitive security or deployment
readiness. Fuzzing, regression tests and accuracy studies remain separate evidence.

Existing assets reused: `policy::summarize` (with a small internal
`aggregate_decision` extraction), `CoverageSummary::finish`,
`assessment::assess_algorithm`, the task-026 local assessment contract and existing
pipeline counter tests. Task 044's historical Kani non-execution in its Linux
environment is preserved; it does not establish current macOS feasibility.

## Specifications and trust boundary

### Policy

Inputs are a list of `Pass`, `Fail`, `Indeterminate`, `NotApplicable`, two
completeness booleans, and applicability. Explicit exclusion yields
`NotApplicable`; unknown applicability yields `Indeterminate`. Otherwise:

* A `Fail` anywhere requires aggregate `Fail`, even with incomplete inspection.
* `Pass` is permitted **iff** inspection is complete, inventory is exhaustive,
  at least one relevant result is `Pass`, and neither `Fail` nor `Indeterminate`
  occurs. `NotApplicable` entries alone provide no positive evidence.
* All remaining inputs require `Indeterminate`.

There are no list-length preconditions in Lean. Kani directly checks the allocation-free `aggregate_decision` used by
`summarize` for applicable profiles, with Rust lists bounded to 0–4 entries.
Applicability overrides and the wrapper are checked by Rust/Lean comparison tests. The report evaluator always supplies `inventory_exhaustive=false`
for inventory v1; the harness verifies the aggregation function, not the complete
inventory/report pipeline or reason strings.
Creusot compiles that same function from `src/policy/aggregation.rs` through the
proof-only Cargo manifest in this directory. Its quantified specification describes
membership, independently of the implementation's flags and traversal order.

### Inspection coverage

Let F = read failures + analysis failures + enumeration failures and
I = partial files + unsupported files. The model requires `Failed` iff F > 0 and
read-ok equals analysis-failed; otherwise positive F or I requires `Partial`.
With F = I = 0, positive unknown files requires `Unknown`; otherwise the result is
`CompleteWithinScope`. In particular, any failure, partial, unsupported or unknown
count prevents complete status. Zero selected files can complete within scope;
this does not mean the unenumerated population was inspected.

Coherent counters satisfy selected = read-ok + read-failed and read-ok = analyzed
+ partial + analysis-failed + unsupported + not-applicable + unknown. `finish`
classifies counters; it does not validate arbitrary externally inconsistent
counters or prove the pipeline produced them. Kani constructs coherent counts
with each terminal count and enumeration failures at most `u64::MAX / 8`, so all
additions are representable, then checks classification and count preservation.
Lean uses unbounded natural numbers; the Rust overflow boundary is separate.

### Algorithm assessment

The experimental Kani domain is ASCII identifiers of exactly two bytes beginning with `?`
(the second byte is symbolic: all 128 ASCII values): they must yield unknown
quantum exposure and unknown parameters. These intended properties are
not yet proved within the local time budget. This is a bounded unknown-name
family, not a proof for every unrecognized string. Eight concrete classical
public-key names (RSA, EC, DH, DSA, ECDH, ECDSA, Ed25519, X25519) have individual
harnesses and must remain quantum-vulnerable. Generic ML-KEM, ML-DSA
and SLH-DSA names without a parameter set must remain unknown. These are catalog
classification properties, not mathematical proofs of cryptographic security.
Other unknown-name families, arbitrary-length Unicode identifiers, OIDs,
certificate composition, parameter parsing, hybrid combiners and primitive-policy prerequisites are outside this PoC's
formal proof boundary; existing tests still apply.

## Implementation and verification steps

1. Establish the latest-develop baseline and run the existing workspace suite.
2. Add cfg-gated direct Kani harnesses and executable Rust comparison vectors.
3. State the independent Lean specification, model, and complete proofs without
   placeholders or added axioms; execute the model on the comparison domain.
4. Add a pinned runner with separate pass/fail/not-run/timeout states, source
   hashes, logs and a separate cost-bounded Actions workflow.
5. Check negative controls, existing tests/build/clippy/fmt, and the combined diff;
   obtain implementation review followed by test review and record the results.

The main agent performs these steps. Exactly two independent reviewers assess
the combined change under the repository review policy. No product rewrite,
cryptographic logic change, storage-format change or rule change is planned.

## Reproduce

Prerequisites: Python 3.10+, Rustup and the root `rust-toolchain.toml`; native
macOS arm64 or Linux x86_64 for the tested verifier configurations. Install Kani
with `cargo install --locked kani-verifier --version 0.68.0` and `cargo kani setup`.
Install the [Elan version manager's official **v4.4.0** release](https://github.com/leanprover/elan/releases/tag/v4.4.0)
for your native target, verify its release asset SHA-256, extract it, and run
`./elan-init -y --no-modify-path --default-toolchain none`. Then run
`$HOME/.elan/bin/elan toolchain install leanprover/lean4:v4.36.0` and add
`$HOME/.elan/bin` to PATH. The CI workflow contains the exact Linux x86_64 URL
and digest. Elan is not installed with `cargo install elan`.
The runner also locates Elan's conventional `lean` executable when it is not on PATH.
Downloads/installations need network access; the prepared proofs and model
comparisons do not call AI services.

From the repository root:

```sh
python3 -m unittest discover -s tests/formal-verification -v
python3 scripts/formal-verification.py --mode all --timeout 300
```

For `all`, also install the pinned [Creusot stack](CREUSOT.md#reproduce).
Use `--mode kani`, `--mode lean` or `--mode creusot` for separate lanes. Each command gets its own
timeout. A fresh UTC-named directory below `target/formal-verification/` contains
`results.json`, command logs, the two vector tables and the compiled Lean module.
An explicit `--out-dir` must not already exist, preventing stale-success reuse.
Every requested check must pass for exit 0. Step statuses distinguish `passed`,
`failed`, `not_run` (could not start) and `timeout` (the child process group was
killed). Lane/overall failure includes unavailable or timed-out required steps;
an unrequested lane remains `not_run`. Kani zero-harness success is rejected.
The default Kani lane selects the two policy/coverage contracts plus eight
classical catalog entries and three generic PQC names (13 harnesses);
`selected_harnesses` records the exact scope. The costly symbolic unknown-name
harness is opt-in with `--assessment`; `experimental_assessment` defaults explicitly
to `not_run`, not `passed`. Opting in makes it a required proof for that run.
CI runs the default proof scope. A successful default run makes no proof claim
about unrecognized names. See [measured results and limitations](RESULTS.md).
`--assessment` requires Kani (`--mode kani` or `--mode all`); using it with
`--mode lean` or `--mode creusot` is rejected before any tool runs.
Lean warnings are errors; all seven theorem dependencies are audited against
Lean's standard logical axioms `propext`, `Quot.sound`, `Classical.choice`, with no
project-added axioms or native computation shortcuts permitted.

The runner binds results to the HEAD plus SHA-256 hashes of Rust/Cargo/toolchain,
proof, runner, runner-test and workflow inputs, and rejects source changes during
execution. This is a drift guard and reproducibility record, not proof that the
translation is correct. CI reruns all three verifiers for source/dependency/proof input
changes, on pushes, pull requests and manual invocation, as separate jobs from
ordinary tests. Each job has a 45-minute ceiling (including installation) and artifacts survive verification
failure. A cancelled job or installation failure without `results.json` means
verification did not complete; it is never proof success. Fork PRs need no secrets.
An unchanged model could miss a Rust change outside the finite comparison domain;
review of translation and proof scope remains necessary.

## References

* [Local assessment contract](../architecture/LOCAL-ASSESSMENT-V1.md)
* [Kani installation](https://model-checking.github.io/kani/install-guide.html),
  [usage](https://model-checking.github.io/kani/usage.html)
* [Lean installation](https://lean-lang.org/install/)
* [Carl M. Kadie's Vibe Validation article](https://levelup.gitconnected.com/nine-rules-for-vibe-validation-of-vibe-coded-algorithms-20db019f5583):
  independently specify, preserve essential decisions in the model, review for
  counterexamples and translation gaps, then machine-check the proof. The PoC
  adopts those correctness guardrails, not the article's multi-agent workflow.
* [Creusot](https://github.com/creusot-rs/creusot),
  [Verus](https://github.com/verus-lang/verus)
