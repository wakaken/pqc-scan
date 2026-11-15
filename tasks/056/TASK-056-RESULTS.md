# Task 056 judgment validation results

## Decision and scope

Task056 evaluation is complete against all nine original acceptance criteria.
Both independent review stages accepted the evaluation and final-source checks
passed. The measured product has five recommendation findings; no product logic
or rules were changed to hide those findings. Evaluation completion does not
assert that the measured product is free of defects.

The maintainer explicitly selected keeping0.45 and completing remaining056
validation on2026-11-15. This is a compatibility requirement, not empirical
endorsement or calibration. Task052 remains unfinished and task053 interrupted;
their definitions are byte-identical to the056 baseline. No other task was
started, created or resumed. No commit, push, PR, publication, release, tag,
settings change or external message was performed. Earlier051/063/064 work and
all earlier measurements are preserved.

## Source and actual verification

HEAD:`f07d782cd68cfac95d93585d5ace049f590160d4`, branch
`codex/task051-dependency-supply-chain-review`, plus the actual uncommitted source
bound by [native-r6/SOURCE.json](evidence/assessment-20261115/native-r6/SOURCE.json)
and [SNAPSHOT.json](evidence/assessment-20261115/native-r6/SNAPSHOT.json).
The only post-run change in that manifest is protocol prose, recorded separately.
The new Rust adapters and frozen inputs match executed bytes. All preexisting
baseline files except task056's own definition remain unchanged.

Rust1.93.1 and Python3.12.7 are recorded with executable hashes in
[TOOLCHAIN-FINAL.json](evidence/assessment-20261115/TOOLCHAIN-FINAL.json).
Actual native commands, exit results, feature configurations and per-command
measurement/CLI binary hashes are in
[native-r6](evidence/assessment-20261115/native-r6/COMMANDS.json):

| Check | Actual result |
|---|---|
| Core public-interface observation test, default | exit0;1,991 raw observations |
| CLI source/dependency observation test, default | exit0;16 real scan fixtures |
| cargo test --workspace --locked --offline | exit0;548 passed,0 failed,3 ignored |
| cargo clippy --workspace --all-targets --locked --offline -- -D warnings | exit0 |
| cargo build --workspace --locked --offline | exit0 |
| Core and CLI targeted tests, --all-features | both exit0; same asserted judgments |
| workspace all-targets all-features strict clippy | exit0 |
| cargo fmt --all -- --check | exit0 |
| Python quality unittest discovery | exit0;102 tests,1 skipped (optional PyYAML unavailable) |
| Independent property comparison | exit0;1,531 canonical cases and16 CLI cases agree |
| Deliberately stale source-hash control | expected exit1; current corrected bindings then exit0 |

The confinement uses readonly source/cache, allowlisted fresh environment,
offline Cargo, external-network denial and scratch-only writes with finite
per-command/per-process limits. It does not claim aggregate Linux cgroup memory
or disk enforcement. Earlier confinement/setup failures remain under original
run directories; none is relabeled as a passing measurement or a product bug.
Python tested-byte bindings and the correction of one stale log-path reference
are recorded in PYTHON-SOURCE-BINDING.json.

The Rust tests emit observations without consulting expected answers. Their exit0
means adapter completion. The independent Python comparison determines property
agreement; manual recommendation content review is separate. Eight hand-derived
comparator controls include wrong/missing/duplicate/extra results, absence/null,
booleans/numbers, f32 transport, empty denominator and invalid reassessments.
Negative-first development logs are retained.

## Measurements and original criteria

The initial1,345-case oracle remains unchanged, including460 historical
characterizations. Separately frozen follow-up cases reassess exactly those
same inputs against explicit compatibility/routing expectations. Canonicalization
rejects changed inputs, double replacement or replacement of already-assessed
rows. The final1,531 canonical cases consist of198 algorithm,108 policy,
960 severity,210 recommendation-routing,46 inventory and9 aggregation cases.
There are0 property mismatches and0 unassessed canonical cases. The460 original
Not Tested labels remain historical and never become duplicate passing rows.

The source-exposed main author is not a blind or independent-author dataset
creator. The expectations derive from cited normative/public local-policy facts
or explicit maintainer compatibility instructions, independently of matcher
labels. Synthetic cross-product rows are correlated, not population samples;
no accuracy confidence interval, calibrated probability or universal safety
claim is made.

| Original criterion | Validation and evidence |
|---|---|
| Q3-01 | Source-cited frozen oracle and separately implemented comparator; correct/incorrect/unassessed counts distinct from matcher labels and manual semantics |
| Q3-02 | Purpose, classical weakness, quantum vulnerability, ideal symmetric/hash query models, parameters/curves/unknowns and local collision context verified separately |
| Q3-03 | Per-case source IDs, exact source register/editions,18 inspected semantic decision classes,14 FIPS potential-update dispositions and43 RFC errata dispositions; IR8547 context does not add a universal deadline |
| Q3-04 | Actual CLI string/dead-code/API fixtures and bounded observation vocabulary do not assert execution, complete safety or deployment |
| Q3-05 | Declared/resolved parser provenance, optional/default-features/provider/unused declarations, synthetic dependency facts; no enabled feature graph or PQC deployment inferred from names |
| Q3-07 | Certificate signature/subject roles, missing/ambiguous/wrong-kind links, inclusive captured validity boundaries and unknown clock; mathematical/signature/chain/trust guarantees remain explicitly absent |
| Q3-08 | Full selected category/risk/base-severity/confidence/evidence equivalence classes, including saturation and preserved0.45; local confidentiality-priority boundaries; no intentional behavior change |
| Q3-09 |960 severity confidence-preservation cases and13 source-bound downstream/guide audits; heuristic value is not posterior probability |
| Q3-10 | Each individually false/missing prerequisite, KEM/signature mismatch, JOSE/PKI constraints, old/current/unknown runtime, provider/features/peers, and all38 unique complete action payloads manually reviewed; findings retained |

[DECISION-COVERAGE.json](evidence/assessment-20261115/DECISION-COVERAGE.json)
is a finite semantic-equivalence mapping from inspected judgment branches to
explicit cases, not instrumented line/branch coverage or every arbitrary input
combination. Invalid parser/schema/security checks reuse the executed workspace
suite rather than manufacturing normative labels from implementation output.

## Measured product findings

All38 distinct complete recommendation payloads were manually reviewed by hash
against primary sources.32 have no identified content finding;6 contain5
separate findings. This post-observation manual content review is disclosed,
not counted as blind gold or pooled with synthetic property agreement.

1. F056-01:unknown Java version receives an older-detected-version rationale.
2. F056-02:the BC example selects ML-DSA with BCPQC, while BC Java1.85 registers
   MLDSA in BC; version/provider and compatible key prerequisites are incomplete.
3. F056-03:the Java RSA-to-ML-DSA example retains privateKey without specifying
   algorithm-compatible replacement keys and associated verifier rollout.
4. F056-04:the hybrid certificate-chain pilot omits a concrete supported
   construction/encoding requirement; primitive FIPS standards do not supply one.
5. F056-05:two Envoy payloads conflate or ambiguously describe RSA-dependent
   suites without clearly separating authentication from static key exchange.

See [recommendation-review.md](evidence/assessment-20261115/outputs-r6/recommendation-review.md)
and RECOMMENDATION-SEMANTICS.json for source IDs, exact payload hashes, affected
case IDs and reasoning. These are product measurements, not evaluation failures.
No examples were executed as cryptographic programs and no support certification
is inferred. Corrections require separate authorized scope; no automatic task
creation or resumption of053 is implied.

## Artifacts and reproduction

The nine original deliverables are in
[outputs-r6](evidence/assessment-20261115/outputs-r6/assessment-oracle.json) and
byte-identical copies under target/quality-task-056. The first Partial outputs,
original freezes, raw observations, failed attempts and threshold study remain.

Reproduce comparisons/output generation from repository root with:

```sh
python3 tasks/056/evidence/assessment-20261115/evaluate-final.py.txt
```

The exact native/Python controllers, official acquisition receipts, source,
input, tool, binary and artifact hashes accompany the evidence. The controller
rejects stale registered source hashes; SOURCE-REGISTER-v1/v2 and correction
receipts preserve the history. BHT-BINDING-UPDATE.json strengthens an existing
citation with acquired primary abstract bytes without changing expectations.

[Independent review record](TASK-056-REVIEWS.md) records the two isolated reviewer
identities, source scopes, findings and confirmations. Both stages accepted. The main agent confirmed final-source applicability,
artifact parity, preserved task states and all original completion gates.
