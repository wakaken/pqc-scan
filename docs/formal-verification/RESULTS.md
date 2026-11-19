# PoC results

Status: the selected local PoC proofs pass; hosted CI and final review are pending.
No project-wide security or completeness claim.

## Source and environment

Base: `origin/develop` at `22aa9f9fb3e368c24d005060015252a2800b422a`.
Working branch: `codex/formal-verification`. All ordinary Rust checks use the
repository's Rust **1.93.1**. Local platform: macOS arm64. Kani **0.68.0** bundles
CBMC **6.11.0** and uses its own pinned **nightly-2026-08-21** compiler; that compiler
is distinct from the product toolchain. Lean **4.36.0** reports compiler commit
`611c117bfaf7b7ae1cfcf246ce6fba31e8f509e9`. Elan **4.4.0** manages Lean.

## Evidence already obtained

* The unmodified latest-develop baseline: `cargo test --workspace --locked`,
  exit 0, **620 passed**.
* Lean: all seven named theorems in [Model.lean](Model.lean) compile with warnings
  treated as errors. The dependency audit reports only Lean standard logical
  axioms, with no `sorryAx` or project-added axiom.
* Rust/Lean: **10,662 identical rows**, including 4,092 policy cases (all decision
  lists of length 0–4, all three applicability states and both completeness
  booleans) and 6,570 coherent coverage cases (0–2 terminal counts plus nine
  large-u64 boundary cases). Vector SHA-256:
  `5340482a2bb192cdbe7743bec948d593d26be19c32eaa14e8422f3c3a37a0675`.
* Minimal production extraction: all **4,092 complete policy records**, including
  reasons and their ordering, are byte-identical before/after extracting
  `aggregate_decision`. Record-table SHA-256:
  `762433c3fed0ee40bbee2b109cae8d361a0b8bea12c2d7f4a5a76e2df1a5bd81`.
* Actual verifier negative controls: Kani rejects `assert!(false)` and Lean rejects
  a proposed proof of `False` (both nonzero exits). Runner tests additionally
  distinguish nonzero exit, missing executable, timeout, missing/empty/mismatched
  vectors, invalid theorem dependencies and changed source manifests.

## Proved properties

| Evidence | Exact claim |
|---|---|
| Kani policy, lists of length 0–4 | Pass iff complete/exhaustive positive evidence exists without Fail/Indeterminate; Fail dominates; no NotApplicable in the applicable kernel |
| Kani coverage, bounded coherent u64 counters | Classification agrees with failure/incomplete/unknown conditions; counters remain unchanged |
| Kani eight classical catalog names | The actual assessment function classifies those names as quantum-vulnerable with an empty parameter map |
| Kani ML-KEM / ML-DSA / SLH-DSA | Generic names without parameter sets have unknown exposure and parameters_known=false |
| Lean seven theorems | Independent policy specification is satisfied for all finite lists; Pass equivalence, incomplete/Indeterminate blockers and Fail dominance; coverage complete iff every blocker is zero, for all Nat inputs |

The thirteen default Kani harnesses and seven Lean theorems pass. This proves
classification of the listed names, not the security of an underlying algorithm.
The symbolic unknown-name harness is explicitly outside the default proof claim.

## Scope and limitations

The [specification and assumptions](README.md#specifications-and-trust-boundary)
are part of every result. Lean proves its model for arbitrary finite decision
lists and natural-number counters. Kani checks the actual production code only
within its stated domains. Finite Rust/Lean comparisons are tests, not a proved
translation or a proof for all Rust inputs. The pipeline's creation of counters,
report binding, reason generation, catalog completeness, Unicode/OID parsing,
certificate composition and cryptographic soundness are not formally proved.

Direct verification of the allocation-heavy public `summarize` wrapper caused
excessive symbolic expansion in `record`/String allocation paths. The initial
unwind-128, unwind-8 and unwind-5 attempts were interrupted; a separate 25-second
verbose diagnostic run timed out. None is recorded as success. The existing
decision expression was extracted into a small allocation-free private function
used by the public wrapper and Kani; no stubs or copied decision implementation
are used. The successful kernel proof does not cover allocations or reason text.

Assessment feasibility: the five-byte symbolic unknown-name trial timed out at
120 seconds, and the reduced two-byte (`?` plus one ASCII byte) trial also timed
out at 60/45 seconds. They are not proofs. The latter harness remains opt-in via
`--assessment`; its default status is `not_run`. Eight classical names in one
harness also exceeded 120 seconds; separate per-name harnesses succeed. Initial
unwind-3/8 trials failed on builtin `memcmp` unwinding assertions, rather than
showing a classification counterexample. Final classical bounds are 8 and generic
PQC bounds are 11; unwinding assertions and default safety checks remain enabled.
Kani also warns about unsupported constructs elsewhere in core; the successful
harnesses do not reach them. This is not proof of whole-crate Kani support.

## Next expansion

First expand `assess_primitive_policy` with fully supported contexts to prove that
unknown exposure and missing parameters independently prevent Pass. Then extend
catalog/OID and parameter interpretation coverage in `assessment.rs`. Next verify
the pipeline's per-file terminal accounting and the binding of inspection status
to inventory evaluation. Consider Creusot/Verus only when an identified unbounded
Rust invariant justifies another annotation/toolchain system. Accuracy studies,
fuzzing and normal regression suites remain separate work.

## Final checks and reviews

Local command:

```sh
python3 scripts/formal-verification.py --mode all --out-dir target/formal-verification/verified --timeout 300
```

Exit **0**, source unchanged, about **93 seconds** of recorded command execution
with caches already populated. Both verifier lanes passed. Full input manifest
SHA-256 (sorted compact JSON):
`c9fcfdfa9c1df5859ffa2561acbafd4b191b87cf5f5bb97453815ddb0c4275f2`.
`target/formal-verification/verified/results.json` SHA-256:
`0897ec1ef521fe33c4027490e467eafb8ba0f9c7872c01bcf814cf64642b9d99`.
It records the baseline HEAD plus every final proof/build/runner input hash;
committing those same bytes does not invalidate the local proof evidence.

| Command | Result |
|---|---|
| `cargo test --workspace --locked` | Exit 0; 621 passed, 3 existing supervisor workers ignored |
| `cargo test --workspace --all-features --locked` | Exit 0; 680 passed, 5 existing ignored entries (supervisor workers and explicit OpenSSH environment test) |
| `cargo build --workspace --locked` and `--all-features` | Both exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` and corresponding `--all-features` run | Both exit 0 |
| `cargo fmt --all --check`, `git diff --check` | Exit 0 |
| `python3 -m unittest discover -s tests/formal-verification -v` | Exit 0; 9 tests |
| `actionlint` 1.7.12 on the new workflow | Exit 0 |

Raw local checks are under `target/formal-tools/`; the verification runner produces
fresh source-bound logs rather than relying on this prose. Default/all-feature
Rust results remain applicable: all 180 normal Rust/Cargo inputs compared with
the completed-test interval's source manifest are unchanged; subsequent changes
were cfg(kani)-only harness bounds, runner/CI fixes and documentation. Changed
harnesses and runner tests were rerun; none of the five ignored entries is claimed
executed. Probe-specific external integration is outside this change's scope.

The implementation reviewer found an invalid Elan installation command, a Lean
version-selection dependency on the user's default, and a contradictory
`--mode lean --assessment` combination. The installer now uses a fixed official
release and verified digest, version selection runs beside `lean-toolchain`, and
the contradictory combination is rejected. Both runner defects have observed
red/green regression tests. Hosted CI and final two-stage review confirmation are
still pending; the request is not yet declared complete.
