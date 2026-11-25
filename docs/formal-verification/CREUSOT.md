# Deductive verification of policy aggregation

## Scope and selection

The additional PoC proves a production loop over an arbitrary-length slice.
It preserves the existing CLI, public `policy::Decision` API, serialization,
applicability overrides, policy rules and reason strings. Only the small aggregate
kernel and its enum move into `crates/pqc_scan_core/src/policy/aggregation.rs`.
The kernel now gathers three flags in one pass instead of three membership scans.

| Candidate | Value and applicability | Selection |
|---|---|---|
| Policy aggregation | Security-critical; arbitrary-length membership and loop invariants; no allocation or external calls beyond core slice/integer operations | Prove directly now |
| Diagnostic coverage | Critical scalar classification checked by Kani; counter production is spread across pipeline operations | Next: conservation of terminal file counts |
| Algorithm assessment | Critical unknown handling, but string normalization, catalog maps and parameter parsing need additional library contracts | Keep existing tests/Kani; scope separately |
| Inventory and finding deduplication | Unbounded preservation/idempotence properties would benefit; hash maps, cross references and serialization increase proof boundary | Later, with explicit asset-equivalence specification |
| Identity and reports | Path normalization, hashing and JSON/HTML dependencies require many external specifications | Defer until input/aggregation contracts are established |

The proof-only Cargo workspace in this existing directory uses the **actual
production file** as its library path. It does not contain a second implementation.
`cfg(creusot)` enables annotations and the enum's logical model. Normal builds
retain the existing traits and serde names and have no Creusot dependency or
nightly requirement. The proof workspace has its own lockfile and toolchain;
root Cargo dependencies and Rust **1.93.1** remain unchanged.

## Independent mathematical contract

For a finite input slice D, write E(x) for `exists i, 0 <= i < |D| and D[i] = x`.
Let C mean inspection complete and X mean inventory exhaustive. There are **no
additional preconditions**, including no length bound, nonempty requirement,
coherence assumption or restriction on the placement of uncertain results.

* Result is Fail iff E(Fail).
* Result is Pass iff C and X and E(Pass) and not E(Fail) and not E(Indeterminate).
* Result is never NotApplicable. With the enum's four constructors, this fixes
  the remaining result as Indeterminate.

These membership statements express policy requirements without describing an
algorithm. They include empty/all-NotApplicable slices, any mixture of results,
and both values of each completeness flag. The applicable-profile kernel does
not decide profile applicability: `summarize` retains those overrides.

The loop invariants say `i <= |D|`, and each flag equals the existence of its
corresponding constructor in the prefix `[0, i)`. Initially the prefix is empty.
Reading the next element preserves all three equivalences. On exit `i = |D|`,
so the flags describe the whole slice. The variant `|D| - i` decreases and is
nonnegative. `#[check(terminates)]` requests total correctness, avoiding a
vacuous postcondition for a divergent implementation. Generated conditions also
cover slice indexing and the index increment's overflow boundary.

## Tools and trust boundary

| Component | Pin |
|---|---|
| Creusot compiler, CLI and prelude | 0.15.0, source `fd4d85b2c237ef93147e99f6a0c04acfc89a6a17` |
| creusot-std | `=0.15.0`, proof Cargo.lock with registry checksums |
| Proof Rust | `nightly-2026-08-03` |
| OCaml | 5.3.0, upstream `creusot-deps.opam` |
| Why3 | `05bf4ddf55cb3042f34c3bc0ed3aff56a226978a` (reports 1.8.2+git) |
| Why3find | `89dd8721fb9de7fea5d00e58ebdabdc75a8235b8` (reports 1.3.0+dev) |
| SMT prover selected by why3find.json | Z3 5.3.2 |

The official installer fixes and verifies the solver downloads' SHA-256 hashes.
It also installs Alt-Ergo 2.6.4, CVC4 1.8 and CVC5 1.4.1; this proof configuration
does not use them. The runner checks the selected stack's reported versions.
OCaml build dependencies are resolved by opam; they are not additional proof
solvers. CI starts from a fresh installation rather than caching proof success.

No project `trusted`, assumed axiom, external-function stub or unchecked lemma
is added. The trust boundary includes Rust/Creusot's translation, creusot-std's
contracts for slice length/indexing and integer operations, the logical sequence
and integer prelude, Why3's transformations, the Z3 binary and host toolchains.
Upstream prelude axioms (including integer extensionality warnings) remain trusted;
SMT-based verification depends on the correctness of this trusted toolchain.
The proof crate excludes unrelated dependencies rather than assigning unchecked
specifications to the full application. Serde derives are disabled during proof;
their output compatibility is checked by ordinary regression tests.

Creusot supports the slice, enum, match, integer and loop features used here.
Functions from external crates need available contracts/metadata; arbitrary
dependencies cannot be assumed verified merely because cargo builds them.
The official limitations include architecture-specific proofs and gaps for
allocation/capacity failures. This kernel allocates nothing. “Arbitrary length”
means every valid slice representable on the verified target, not an infinite
collection or all possible architectures. macOS arm64 and Linux x86_64 both have
64-bit usize; results for other targets are not claimed. Rust 1.93.1 production
compilation and the proof nightly are different trusted compilers; tests bridge
them but do not mathematically prove compiler equivalence.

## Reproduce

Install prerequisites: Rustup, Python 3.10+, git, curl, opam, automake, GMP, zlib,
ZeroMQ and pkg-config. On Ubuntu 24.04, the exact apt packages and full pinned
installation commands are in the existing
[formal verification workflow](../../.github/workflows/formal-verification.yml).
On macOS, Homebrew packages are `opam automake autoconf gmp zlib zeromq pkgconf`.

Use a fresh temporary clone of the official Creusot repository and check out the
full source pin above. From that clone, initialize headless opam
(`opam init --bare --disable-sandboxing --no-setup -y` and
`opam --cli=2.1 var --global in-creusot-ci=true`). Install the pinned nightly's
`rustc-dev`, `llvm-tools` and `rustfmt` components. Run
`cargo install --locked --path cargo-creusot`, then install `creusot-rustc` with
`--locked --root <data-home>/toolchains/nightly-2026-08-03`. Run
`cargo run --locked --bin creusot-install -- prelude why3 provers why3-conf`.
All these commands run **inside the pinned clone**, not the pqc-scan workspace.
Avoid the optional global Cargo configuration patch.

Linux data-home defaults to `~/.local/share/creusot`; on macOS it is
`~/Library/Application Support/creusot.creusot`. A custom `CREUSOT_DATA_HOME`
must be used consistently for installation and execution. The macOS opam switch
is `~/.creusot`. In the measured local environment, the command-line tools' SDK
produced a C linker error. Setting `SDKROOT` to `xcrun --sdk macosx --show-sdk-path`
for the installer command selected the working Xcode SDK; no global Xcode setting
was changed. An upstream macOS CVC5 version-parser warning is irrelevant to the
Z3-only configuration and is retained in version logs.

From pqc-scan's root:

```sh
python3 -m unittest discover -s tests/formal-verification -v
python3 scripts/formal-verification.py --mode creusot --timeout 300
```

The existing runner creates fresh proof workspaces, points their manifests at the
production source, and runs `cargo creusot --no-cache ... -- --locked`. It requires
nonempty goal summaries including `vc_aggregate_decision`, all goals proved,
generated Coma and actual Valid solver results. Exit zero without goals is failure.
It preserves Coma, detailed goals, solver outcomes (including Timeout/Unknown),
runtime witness tests, source hashes, versions, durations and command logs.
Each subprocess has a timeout and its entire process group is killed on expiry.
An unavailable tool remains not_run; failed goals are failed; neither is success.

Negative controls derive temporary mutants without editing production source:

| Mutation | Concrete counterexample | Required evidence |
|---|---|---|
| Omit both completeness guards | `[Pass]`, C=false or X=false | The corresponding VC is unproved and the named runtime witness fails |
| Forget to record Fail | `[Pass, Fail]`, C=X=true | Same |
| Skip the last element | `[Pass, Indeterminate]`, C=X=true | Same |

Every contract/invariant is retained. A compiler error, missing goal, process
timeout or a passing mutant is **not** accepted as the negative control. Solver
Timeout/Unknown responses can contribute to an unproved goal; they are preserved
as such, not described as solver-discovered counterexamples. Executed runtime
witnesses independently establish that these three mutants really violate policy.
The lane succeeds only if the positive proof/witnesses and all rejection checks
succeed. Finite witnesses test sensitivity; they do not establish mutation coverage
against every possible future defect or prove the specification itself correct.

The existing workflow runs separate Kani and Creusot lanes. Source, contracts,
proof manifest/lockfile/toolchain/config, runner and workflow changes trigger it.
Artifacts survive failure; cancellation or failed installation is not a proof.
Changing the production kernel changes the compiled proof input automatically.
Changing the specification still requires independent review against requirements.

## Results and expansion

See [the dated results](RESULTS.md) for source-bound measurements and review status.
Kani retains bounded direct checks of policy, coverage and fixed catalog entries;
Creusot provides an unbounded direct implementation proof of this one kernel.
The Lean lane was retired at the maintainer's request; its earlier measurements
remain historical. Small-domain Rust regression tests remain. All existing tests, fuzzing and corpus evaluations retain their roles.

No Creusot proof is claimed for profile applicability, reason construction,
coverage-counter production, inventory completeness, detector accuracy, algorithm
classification, identity/deduplication, report serialization or whole-program
behavior. Next, specify conservation of coverage counters across terminal file
states, then inventory reference integrity and finding-set preservation. Those
need explicit domain contracts before library abstractions or trusted boundaries
are expanded. Adding Verus is unnecessary while Creusot covers this use case.

Official references: [installation](https://guide.creusot.rs/installation.html),
[loop invariants](https://guide.creusot.rs/basic_concepts/loop_invariants.html),
[termination](https://guide.creusot.rs/termination.html),
[limitations](https://guide.creusot.rs/limitations.html),
[trusted annotations](https://guide.creusot.rs/trusted.html),
[pinned source](https://github.com/creusot-rs/creusot/tree/fd4d85b2c237ef93147e99f6a0c04acfc89a6a17).
