# PoC results

The original Kani/Lean measurements below are historical. The dated Creusot
extension at the end records the additional scope and its separate status.

Status: the selected PoC is implemented and verified locally and in hosted CI;
both independent review stages are accepted. Unproved scope is retained below.
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
red/green regression tests. The final implementation review and subsequent
independent test review accepted `22aa9f9..5e0c8c7`, with no unresolved required
findings. Reviewers inspected evidence instead of repeating the full suites.

## Hosted CI

[Formal verification run 44491155629](https://github.com/wakaken/pqc-scan/actions/runs/44491155629)
passed on **2026-11-19** for source commit
`5e0c8c72f69e85f49b98ba5423d0c18ae688f588`. Both jobs ran on Ubuntu 24.04 x86_64
with fresh installation of the pinned verifiers:

| Job | Result | Total job time | Downloaded results.json SHA-256 |
|---|---|---|---|
| formal-kani (135446804179) | All 13 selected harnesses passed | 4m 06s | `56b4b01280805f763e6b267ac50b00c34333c143ffc30eb72fc4466e32ec3d61` |
| formal-lean (135446804586) | Seven theorems/audit and 10,662-row comparison passed | 48s | `c9d02ee5d334615f9cbdace3ca7726441fa9a786b007eec0fee54157f190c5d5` |

Each lane reports source unchanged, and both source-file manifests exactly match
the local final manifest above. The experimental unknown-name harness remains
`not_run` in both default CI reports; it is not a successfully proved property.

Artifacts: `formal-kani-5e0c8c72f69e85f49b98ba5423d0c18ae688f588`
(ID 14512224642, ZIP SHA-256
`89675ff8b75073804c94011ea0c3bd4edb2df7d7fce16069ca348011d66229e1`)
and `formal-lean-5e0c8c72f69e85f49b98ba5423d0c18ae688f588`
(ID 14513340970, ZIP SHA-256
`9b6c9a945eb9391f60814e40a902a04cd7a709ab814d694b930fc17c6495fd4b`).
Actions artifacts have 14-day retention; rerun the documented commands or workflow
when a fresh measurement is needed. Local downloaded evidence is under
`target/formal-tools/ci-{kani,lean}/` and workflow metadata is in
`target/formal-tools/ci-run.json`.

Only this results prose changed after those reviewed/executed source bytes.
The report update does not alter proof/build/test inputs, so the checks and review
conclusions remain applicable. No PR merge or develop update is part of this PoC.

## 2026-11-25: Creusot extension

Status: the selected Creusot PoC is implemented and verified locally and in hosted
CI. Both independent review stages accepted the change. This is a proof of the
selected aggregation kernel, not the whole application.

Baseline: `6f94ba692995cb33640dd13af62e4da0606dfead`, including develop
`b41ae987798d2908850c612634d2d2f220c73dc3`. Branch remains
`codex/formal-verification`. See [the Creusot specification and trust boundary](CREUSOT.md).
The final executable input manifest has 244 entries and SHA-256
`b6f5f17479ed5be4b97437026b5b28ad489273ed3609691dce41f4ea9fdbc97d`
(sorted compact JSON). Proof caches are artifacts, excluded from source inputs.

Local macOS arm64 results, Creusot 0.15.0 / Why3 1.8.2+git / Why3find 1.3.0+dev /
Z3 5.3.2 / proof nightly-2026-08-03:

| Check | Actual result |
|---|---|
| Production aggregation kernel, arbitrary representable slice length | Both top-level VCs proved, including `vc_aggregate_decision`; Fail/Pass equivalences, no NotApplicable, index/increment safety, prefix invariants and termination |
| Positive proof command | Exit 0, 4.453s with warm compilation caches; individual main SMT goal 0.012s, slice-length goal 0.003s |
| Omit completeness guards | Proof exit 1, 23.083s; named runtime counterexample exit 101 |
| Forget Fail | Proof exit 1, 16.455s; named runtime counterexample exit 101 |
| Skip final element | Proof exit 1, 19.393s; named runtime counterexample exit 101 |
| Positive runtime witnesses | Three passed, exit 0 |
| Existing Kani / Lean | All 13 harnesses and seven theorems passed; 10,662 comparison rows equal |
| Full policy records before/after the extraction | All 4,092 serialized records byte-identical, including reasons |
| Default workspace tests | Exit 0; 825 passed, three existing ignored entries; 238.399s |
| All-features workspace tests | Exit 0; 887 passed, five existing ignored entries; 232.837s |
| Workspace build and clippy, default and all-features | All exit 0; warnings denied by clippy |
| fmt, runner tests, actionlint, diff check | Exit 0; 13 Python tests |
| Built CLI scan of examples, all output formats | Exit 0; 55 files scanned, 322 findings; reports produced |

These timings are measurements, not performance guarantees; installation is
excluded and two workspace test configurations ran concurrently. Negative VCs
are **unproved**, not positive proofs of their negations. Raw solver responses,
including Timeout and Unknown, are retained; concrete executed witnesses establish
actual policy violations independently of solver incompleteness.

Evidence: `target/formal-creusot-final/results.json` records the first complete
all-lane success with source unchanged. `target/formal-creusot-final-inputs/` reruns
the final source manifest after cache exclusion and precise CI path filters.
That final rerun also passed with source unchanged: positive Creusot command
2.600s, three negative proof commands 16.708s / 14.932s / 26.342s. All three
runtime counterexamples failed as expected. Its `results.json` SHA-256 is
`a0f9f2a8c99a957613ab7d8c98ecc78d8f4489dd2d235b0359cc77657e32696b`.
Ordinary check receipts and logs are under `target/formal-creusot-checks/` and
`target/formal-creusot-feature-checks/`. All Rust/Cargo/toolchain inputs in those
receipts still match final source; only CI filters and generated-cache exclusion
changed afterward. Those changes do not affect the ordinary test/build evidence.

Installation attempts are retained under `target/formal-tools/`: missing macOS
depexts, the initial data-directory mismatch, and the C SDK/linker incompatibility
were failures, not proof success. Installing prerequisites, using Creusot's actual
data path, and selecting the working SDK per command resolved them. An initial
mutant filename containing a hyphen produced invalid generated span syntax;
the runner rejected it as a negative control, and underscore filenames resolved it.
Earlier witness runs with source changed during execution remain failed overall.

Implementation review `/root/creusot_implementation_review` accepted the combined
change, then the scoped cache/filter follow-up, with no unresolved findings.
Reviewed diff SHA-256: `a29cf2e28b5e62c025af918090bdc73e1d759c3928e2ee7148bbf924a62a977d`.
Prose-only measurement updates do not change executable proof inputs.

Independent test reviewer `/root/creusot_test_review` accepted executable source
`f0bed1539b5dc2cdb15ba162f584aa452a1aef01`, with no unresolved findings. The reviewer
independently checked current/local/CI manifests, command-log hashes, full-record
compatibility, ordinary test counts and mutation evidence without repeating the
whole suites. Test binaries' target paths confirm default/all-features isolation.
`target/formal-creusot-checks/provenance.json` additionally records the retained
test/CLI artifacts' hashes and timestamps retrospectively, not as hashes captured
at original execution time. No protocol implementation was changed.

### Creusot extension hosted CI

[Formal verification run 45543461832](https://github.com/wakaken/pqc-scan/actions/runs/45543461832)
passed on **2026-11-25**, source
`f0bed1539b5dc2cdb15ba162f584aa452a1aef01`, Ubuntu 24.04 x86_64. All three downloaded
lane manifests exactly match the 244 final local input hashes above; every lane
reports source unchanged.

| Lane | Actual result | Full job time | results.json SHA-256 |
|---|---|---|---|
| Creusot | Both top-level VCs proved; positive witnesses pass; all three faulty mutants rejected and concrete witnesses fail | 11m 42s | `85b51dbea50a7fe0ef06c6f5c66c9a975c381b9687cdac49e73a7003297b8087` |
| Kani | All 13 selected harnesses pass | 3m 33s | `68d46afa2eaf546a32fc4e2487e3116d5ce14c2b4690cdfc8bb872345ee41306` |
| Lean | Seven theorems and axiom audit pass; all 10,662 rows equal | 1m 10s | `a79c00609b5b6b2225041839377f546cb455b6edf079b17ccd32210242139909` |

Creusot's fresh installation took **9m 00s**. Its positive command took **28.874s**,
including cold Cargo compilation (25.02s). The final named SMT goals were Valid
(main 0.014455s, length 0.002885s). One calibration attempt with `goal="<none>"`
timed out; it is retained in the solver history and is not a missing proof goal.
Negative proof commands took 23.555s / 16.098s / 65.275s, all exit 1, with their
named runtime witnesses each exit 101. No process timeout or compile failure was
counted as a valid negative control.

Downloaded evidence is under `target/formal-creusot-ci/{creusot,kani,lean}/`.
Complete Creusot setup/verification logs are retained beside those directories;
job/artifact metadata is under `target/formal-creusot-review/`. Hosted artifact IDs
are 14991455814 (Creusot), 14989774721 (Kani), 14989838968 (Lean), with the workflow's
14-day retention. Installation cost is isolated from ordinary test jobs. Proofs
are always regenerated and run without reading cached solver success. This final
report update changes prose only; executable input hashes and review conclusions
remain applicable.

No new Creusot claim covers the report wrapper, applicability/reasons, diagnostic
counter production, assessment/catalog parsing, inventory references, identity,
deduplication, detection accuracy or whole-program behavior. Compiler/translation,
upstream library contracts/prelude axioms, Why3 and Z3 remain trusted. The other
tools' earlier limitations, especially the unproved symbolic unknown-name Kani
experiment, remain in effect. Next expansion is diagnostic terminal-count
conservation, then inventory reference integrity and finding-set preservation.
