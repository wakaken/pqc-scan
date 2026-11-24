# Task 067 results

Current disposition: done after renewed independent reviews and final069 checks.
See the renewed-completion section below.

Historical reopening entry: reopened on2026-11-18 during task069 integration. A focused
negative test found that contradictory exact AES size / known EC curve-size
records could be admitted. Original R2-01/R2-04 require correction and renewed
review/verification; no criterion is narrowed. The prior completion and evidence
below describe the earlier source and are preserved as history. See
[the reproduced failure](../069/evidence/parameter-contradiction-before.log).
The same two reviewers will assess the new-only correction in the combined
067-follow-up/069 package. No legacy rule/catalog/policy changes are selected.

Status: done. Every original R2-01..05 clause and deliverable is implemented
and verified for this technical-reader component, with both review stages accepted. Baseline: c7e76b59da44ffdc8a60da86efd5393e70a039e3 with the
uncommitted task067 changes. The selected worktree/branch and task066 adopted
contracts are retained. No existing user changes were present at selection.

## Behavior and purpose

The core `migration` module reads saved Inventory/TLS/SSH offers/SSH KEX v1 into
content-bound technical targets without scanning, connecting or initializing a
protocol provider. Fixed roles distinguish certificate subject/signature, key
establishment, host signatures and advertisements. The original pure primitive
assessment is reused; only the separately selected `migration-facts-v1`/1.0.0
dispatch adds adopted exact protocol/component facts and DSA/DH parameter gaps.

Bounded duplicate-aware JSON validation precedes the existing schema/reference
and new semantic gates. Native extension tolerance is preserved. TLS valid partial
records retain independently reported verified negotiation; failed/contradictory
success cannot promote assurance. Unknown encoding, names and missing facts remain
unknown. Stable new target IDs bind exact input bytes and original source roles.
Known-safe facts omit raw evidence/arbitrary metadata. Source records expose
adopted editions, clauses and limits. See the [plan](TASK-067-PLAN.md),
[developer notes](../../docs/architecture/SAVED-EVIDENCE-READERS.md) and paired
[user guide](../../docs/user/SAVED-EVIDENCE.md).

## Original acceptance reconciliation

| Clause | Implementation and checks | Current disposition |
|---|---|---|
| R2-01 | Reused primitive axes plus new-only DH/DSA parameter checks; exact/draft/OID/namespace/parameter, RSA/PSS/EC/KEM/signature/AES/hash assertions in migration067 | Verified; both independent stages accepted |
| R2-02 | Explicit native/TLS certificate and KEX roles; final/draft TLS and finite SSH components, combiner/protocol sources; unknown encoding and no authentication inference | Verified; both independent stages accepted |
| R2-03 | Static method/assurance/usage, declaration/resolution provenance, advertisement and reported session remain separate; valid partial/failure facts and canary tests | Verified; both independent stages accepted |
| R2-04 | Four default-build offline readers; bounded regular-file/UTF-8/duplicate/depth/fields/bytes/count/version/reference/semantic gates; no-endpoint-connection control | Verified; both independent stages accepted |
| R2-05 | Explicit new catalog/version dispatch and resolvable source records; old evaluator/policy/rules/schemas unchanged; retained assessment and CLI goldens in existing suites | Verified by retained old tests/goldens, required configuration checks and both reviews |

## Evidence and limits

The final product/test sources match
[evidence/IMPLEMENTATION-FIX-SOURCE.json](evidence/IMPLEMENTATION-FIX-SOURCE.json).
Later lifecycle/review/result prose does not affect executable inputs. Exact
commands, logs, exits, environment and98 binary hashes are in
[evidence/COMMANDS.json](evidence/COMMANDS.json).

| Final check | Result |
|---|---|
| Targeted migration067 + retained assessment_policy | exit0;24 +9 passed |
| `cargo test --offline --locked --workspace` | exit0;644 passed,3 existing ignored supervisor entrypoints |
| Isolated core `--all-features` tests | exit0;182 passed |
| `cargo build --offline --locked --workspace` | exit0 |
| `cargo fmt --all -- --check` | exit0 |
| Strict workspace/all-target default clippy | exit0 |
| Strict workspace/all-target/all-feature clippy in separate target directory | exit0 |

The initial all-feature clippy attempt exited101 for nonminimal_bool. The
semantically equivalent predicate correction and passing reruns are preserved;
failed attempts are not relabelled. Initial logs retain their earlier source
scope. I067-01/02 were fixed and accepted by the same implementation reviewer;
see [review history](TASK-067-REVIEWS.md). The independent test reviewer accepted the final scope and checked all final log/binary hashes; no actionable finding remains.

Boundary tests generate otherwise valid inputs below, at and above each new
reader resource limit, plus existing smaller certificate/profile/SSH limits.
The no-endpoint-connection control reads an immutable saved file aimed at a held
loopback listener and observes no connection. Core has no protocol-crate/provider
or network/schema-retrieval dependency enabled. No live protocol interoperability
is claimed. Existing primitive/policy/rules/native-schema/identity source bytes
remain unchanged; current workspace suites exercise old CLI/report goldens,
custom rules and0.45 behavior. No new report presentation requires browser work
in this component-only task. Node/distribution/browser results from066 are not
claimed as new reader measurements.

The task066 frozen expectations remain specification truth with disclosed source
exposure, not independent practitioner evidence. This task does not implement
business policy/context/reporting, demonstrate protocol interoperability, validate
collector authenticity, or change experimental collection/support/publication.
No numeric product score, external contact, recruitment, spending, release or
publication was performed. Task068 is now eligible because066 and067 are done;
task071's missing practitioners/independent data remain a later dependency.

## Completion decision

The main agent reconciled each original R2 clause with the implementation,
executed final-source checks and two independent acceptances. Documentation
links/parity, task066 freeze preservation and absence of legacy evaluator/rule/
schema/identity changes are recorded in
[evidence/documentation-and-source-checks.log](evidence/documentation-and-source-checks.log).
Final artifacts are bound by [evidence/FINAL-SOURCE.json](evidence/FINAL-SOURCE.json).
Later task/result/review metadata does not change executable test inputs; its
applicability is explicit rather than rewriting historical source manifests.
No mandatory task067 criterion is deferred to a later task. This completion is
not v0.4 product acceptance or release. The work remains uncommitted on the
specified branch unless a later publication instruction says otherwise.

## Renewed completion after069 integration

The reopened original R2-01/R2-04 consistency gap is corrected and verified.
Same two reviewers accepted the combined067 correction/069 source; renewed
completion applies to all original067 criteria. [069 final checks](../069/evidence/COMMANDS.json)
record7 exits0: workspace685/3 existing ignored, isolated core all-features216,
featured CLI5, build/fmt/both strict clippy. Positive exact AES/EC, contradictory
size/curve, unknown encodings and certificate numeric/interval cases pass.
[Review-fix snapshot](../069/evidence/REVIEW-FIX-SOURCE.json) identifies the final
executable bytes; earlier results, failed reproduction and reviews are preserved.
