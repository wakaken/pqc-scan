# Historical catalog reference repair — 2026-11-16

Two artifact references addressed mutable current documents even though their
SHA-256 values identify historical source bytes. Later authorized edits changed
the Japanese security-policy link and appended a task052reassessment pointer.
Those edits do not invalidate the original study, but the original artifact bytes
must remain readable for strict evidence validation.

`CATALOG-BEFORE.json` preserves the complete preceding catalog byte for byte.
`PROVENANCE.json` records the original Git blob identities, original logical
paths and unchanged SHA-256values. `README.ja.md` and `TASK-052-RESULTS.md` here
are immutable historical source snapshots, not current end-user instructions.
Their internal relative links describe their original repository locations.

The active catalog points those historical artifact descriptors at these exact
snapshots. It does not update their expected hashes, sourceBindings, measurement
population digest, results, reviews or commands. Reversing the path mappings
and removing the explicit repair metadata reconstructs the entire frozen catalog.

The active catalog population and active test source now differ from the old
hosted study. The existing aggregator correctly reports the old integration
record's current source/input applicability as Partial; no old successful run is
promoted to current-source validation. Tests require that conservative distinction,
validate the old population digest against the frozen catalog, verify exact
reconstruction, and reject tampered snapshot bytes. Existing synthetic positive
controls still require Verified applicability for genuinely matching populations.

New PR/merged CI validates the new source separately. Original completion records
and task states remain intact; task054stays in-progress and releaseDecision remains
Unknown. No numeric acceptance gate or missing-measurement requirement is waived.
Machine-local paths and physical backup locations are not recorded here.

The updated regression test was itself an artifact of the old hosted study. Its
original bytes are therefore preserved as `test_evidence.py.txt`, with the same
old declared SHA-256, while its live source binding remains historical. The test
update cannot silently replace a measured old source artifact with new bytes.
