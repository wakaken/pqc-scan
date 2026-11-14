# Reviewed development baseline

Run the existing scanner; never execute or compile the corpus. Python 3.12 and
standard library only; the public Rust CLI/report contracts are unchanged.

```sh
cargo build --workspace --release --locked
python3 -m unittest discover -s tests/quality -p 'test_*.py'
python3 tests/quality/run.py --binary target/release/pqc-scan --build-profile release --out-dir target/quality-baseline
```

Choose a new target subdirectory for every run. The harness refuses to overwrite
evidence and copies fixture bytes into a temporary root outside scanner exclusions.
The source-bound manifest, labels and protocol are outside scan inputs. It never
fetches schema/dependencies, resolves packages, contacts peers or executes fixture
code. Building the scanner may initially require normal Cargo dependency cache
population; `--offline --locked` is supported afterward.

- `corpus.json`: current 023-v2 file digests/lineages/splits and 57 truth units.
- `labels-final-a.json`, `labels-final-c.json`: independently assigned current labels.
- `truth.json`: source-only adjudication and exact label/corpus SHA bindings.
- `LABELING.md`: blinded review instructions, without previous adjudications.
- `PROTOCOL.md`: truth/scoring/repeatability/coverage/performance definitions and history.
- `evaluate.py`: one-to-one matching and integrity/semantic normalization.
- `run.py`: real CLI measurements, per-child resources, output/canary checks.
- `test_*.py`: evaluator and resource/timeout sanity controls.
- `corpus-initial.json`, `labels-a.json`, `labels-b.json`: immutable original review
  evidence, retained after lineage audit invalidated the first proposed holdout.
- `holdout-additions.json`: independently authored new project provenance and spans.

Physical `corpus/holdout/` names belong to the initial construction. They do not
define current splits: all readiness-derived variants are now tuning, as declared
by corpus.json. The fresh approval-adapter project was authored without scanner,
matcher, metric or other-label access; two source-only reviewers labeled it before
its first measurement. Existing dependency/SBOM and malformed/unsupported groups
retain their distinct lineages. No output-driven rule tuning was performed.

All original authored fragments and public certificate fixtures are distributed
under the repository [MIT license](../../LICENSE). Readiness fragments are reused
from examples/pqc-readiness unchanged. Certificate bytes and independent OpenSSL
facts come from crates/pqc_scan_detectors/tests/fixtures/certificates; its README
records origin and discarded ephemeral private keys. The two new private-marker
bodies are visibly synthetic canaries, not keys. The fresh holdout includes SPDX
MIT headers. Do not replace them with private data or generate/commit real keys.

These units score a bounded set of nominated rules. Other findings remain counted
as out-of-scope. Null denominators and unsupported/ambiguous populations remain
visible. Small synthetic corpus percentages are descriptive, not global accuracy
estimates. Resource samples include startup/report writing and do not measure
true OS cold cache. CI enforces integrity, sanity and repeatability only; numeric
accuracy/performance proposals require maintainer review after measurement.

See [the measured baseline and proposed budgets](../../tasks/023/TASK-023-BASELINE.md).

## Source-bound quality report (PQS-QAM)

After a measurement, generate a separate development report using the same
scorer and frozen truth. Use a new output directory; existing evidence is never
overwritten. This command reads raw accuracy reports, measurements and source
bindings. It does not scan, execute corpus code or fetch dependencies.

```sh
python3 tests/quality/report.py --baseline target/quality-baseline \
  --reference tasks/023/evidence/task023/BASELINE.json --out-dir target/quality-assessment
```

`quality.json` retains denominators, split/family and language/rule/detector/input
strata, FP/FN, coverage, resources and source hashes; `quality.md` is a short view.
FPR counts negative truth units with a hit, separately from duplicate/extra
prediction FP counts. Zero denominators stay null. Algorithm-specific truth is
not available in the unchanged TASK-023 data. Q3/Q4/Q6 require separate evidence and remain Not Tested in
this automatic report; release acceptance remains Unknown. A failed numeric
comparison is reported without a new numeric CI failure threshold.
TASK-023's local nonregression and 2x investigation criteria were approved;
see [acceptance](../../tasks/023/TASK-023-ACCEPTANCE.md). The original runner's
historical `budgets: unapproved` string is not the current policy authority.
No tool silently rewrites that historical metadata or old measurement files.
See [metric definitions](../../docs/quality/QUALITY-METRICS.md) and the
[current assessment](../../tasks/041/QUALITY-ASSESSMENT.md).

## Additional reviewed data (TASK-042)

The optional data directory reuses the runner, matcher and reporter. Omitting
`--corpus-dir` still selects the original 57-unit TASK-023 corpus. Data are never
imported, installed or executed. The expanded corpus binds two source-only label
records, adjudication, protocol, upstream commits, licenses and exact input bytes.

```sh
python3 tests/quality/run.py --corpus-dir tests/quality/expanded \
  --binary target/release/pqc-scan --build-profile release \
  --out-dir target/quality-expanded --repeats 3 --skip-benchmarks
python3 tests/quality/report.py --corpus-dir tests/quality/expanded \
  --baseline target/quality-expanded --out-dir target/quality-expanded-assessment
```

Use fresh output paths. Additional JSON files include algorithm/use truth,
algorithm/parameter/use/case/format strata, the complete rule coverage ledger,
FP/FN and comparison scope. Rates retain denominators; empty cells are null.
Algorithm-conditioned reference detection is not classifier correctness,
execution or enabled cryptographic protection. Unmatched FP stay unattributed;
mixed-algorithm units occupy combined cells without multiplying their counts.

The locally authored portion is tuning. Public Node.js source predates this
scanner but is a single correlated upstream lineage selected by a rule-aware
coordinator. It is an authorship-independent first evaluation, not an unbiased
estimate of population accuracy. Equivalent filename/encoding cases and parameter
loops do not create independent samples. See [source-only protocol](expanded/PROTOCOL.md)
and [provenance](expanded/provenance.json). Keep these data immutable; tuning on
their results requires a new held-out lineage. Performance is Not Tested when
`--skip-benchmarks` is used. Existing numeric gates remain unchanged.

The first 042-v1 measurement exposed incomplete annotation coverage: nominated
findings outside annotated spans count as FP under the unchanged protocol, but
can include legitimate facts. Generated `annotationInterpretation` separates
negative-unit hits, duplicate-positive predictions and unattributed predictions.
Precision/F1 describe agreement with these fixed annotations; unattributed FP
are not established product false positives. Preserve v1 and investigate under
TASK-043; any post-observation annotation correction is tuning evidence.

### Source-audited correction and remaining format controls

`expanded-v2` preserves v1 and adds source-reviewed facts and format/parameter
controls. All inherited data, including Node files whose paths still start with
`holdout/`, are tuning. Only the separately pinned RustCrypto excerpt is fresh
upstream-authored holdout. `expanded-v2-lock-controls` is an additional two-file
tuning supplement, frozen after v2; it does not rewrite the measured v2 data.
Each corpus requires two hash-bound reviews of every nominated rule against
every file. Missing source facts, unresolved definitions, wrong populations,
changed protocol/review bytes or removal of the required audit flag are rejected.
Zero findings alone never establishes annotation completeness.

Use the commands above with either new directory and a fresh output directory.
The existing CI baseline job measures all versions separately. No new numeric
hard gate is introduced. A reference is a same-data regression only if the
truth, corpus and materialized input bindings match; supplying no reference
does not itself prove a first independent evaluation.

For v2, also pass
`--applicability tests/quality/applicability-document-relocation-20261114/assessment.json`
to `report.py`, as selected by the current CI workflow.
This adds `supported-scope.json` after checking two independent, source-bound
applicability reviews. The original object-property truth and counts remain
visible. Six attributes inside encrypted certificate contents are outside the
documented public subset; their post-observation scope correction is separate
from the original nominal-property measurement. It is not detector improvement
or a new blind evaluation.

The current applicability review binds the link-updated public contract and the
unchanged parser. Earlier `applicability-042-v2/` and
`applicability-044-develop-integration/` records remain immutable historical
assessments; their old contract hashes are not current CI inputs. A changed
contract requires a new source-based applicability review, preserving the old
records, rather than replacing hashes inside past reviews or bypassing checks.

The [reopened TASK-042 results](../../tasks/042/TASK-042-REOPEN-RESULTS.md)
link the original nine acceptance criteria, every supported format, algorithm
controls, known errors, exact commands and independent review records.

## TASK-043 fixes and metadata replay

`task043-corpus-v2` reuses all 042-v2 inputs/questions as tuning, adds two
previously unannotated verification references and seven questions from two
pre-scanner upstream authors. Six encrypted attributes are unsupported under
the existing public scanner contract; original object truth stays immutable.
Original 043-v1 source-only labels were frozen before measurements. The v2
change only corrects upstream hashes versus connector copies with an additional
trailing LF. It retains exact inputs, definitions, labels and original blind
records under `inherited-v1/`; reviewers disclose post-freeze output exposure.
The evaluator rejects replay with changed input hashes, questions, definitions
or labels, and the reporter never calls it a new independent trial.

```sh
python3 tests/quality/run.py --corpus-dir tests/quality/task043-corpus-v2 \
  --out-dir target/task043-measurement --build-profile release --repeats 3 --skip-benchmarks
python3 tests/quality/report.py --corpus-dir tests/quality/task043-corpus-v2 \
  --baseline target/task043-measurement --out-dir target/task043-assessment
```

Use a new directory for each run. Source selection is targeted, not random;
seven holdout questions include correlated rules and one ambiguous control.
These data cannot establish population accuracy or Q3–Q6 assurance.
