# Real OSS source oracle

`oss-oracle.json` pins repository revisions, exact file hashes, original source
links and expected source locations/algorithm references for thirty-six selected
controls across eight OSS projects and seven languages. It contains no copied
third-party source text. These controls are not a complete ground truth corpus,
a detection-recall measurement or evidence of deployed configuration.

The November 21 follow-up expands the positive controls from 25 to 36 and adds thirteen source-backed negative controls. These
distinguish API invocations from comments, documentation, name maps, imports and
different cryptographic roles at explicitly reviewed locations. They are bounded
semantic checks, not repository-wide precision/recall estimates. Full repository
scans with test exclusion validate the controls without replacing the input by a
hand-selected file subset. Production-source inspection gaps are accounted for
separately from these reference expectations.

The executable portable reductions in `../../framework_source.rs` exercise the
Ruby/Rust data-flow shapes using original synthetic examples and negative controls.
They deliberately rename wrapper classes/methods and do not recognize product
names. Ruby branch candidates remain references; dynamic overrides, caller argument
propagation and generated methods are unresolved. Optional defaults remain
candidates rather than supplied argument values. `../../source_unknowns.rs` adds
normalization, same-call JWT binding, default-parameter and adversarial controls. Rust TLS API references retain
unknown negotiated algorithms. Existing AST occurrence tests cover distinct spans
in all seven language scopes.

For the 2026-11-20 acceptance run, exact unmodified OSS files were copied to an
isolated temporary input tree (outside the scanner's ignored `target` directory),
then inspected by the before/after binaries with recorded rules. Artifact paths:
`target/oss-scanner-20261120/measure.py`, `oracle.py`, and `evidence/*-oracle.json`.
The measured source hashes must match this manifest before applying expectations.

The 2026-11-21 unknown-record continuation adds column-qualified expectations for
all prior recoverable unknowns and preserves Vector unknown-algorithm controls.
Its source/measurement/report evidence is under
`target/oss-scanner-20261120/unknowns-20261121/`. Historical measurements use their
recorded manifest snapshots; do not rerun them against changed expectations.
