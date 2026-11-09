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

See [the measured baseline and proposed budgets](../../docs/quality/TASK-023-BASELINE.md).
