# Exact-version upstream notices

These files supplement registry archives that omit their repository-root notices.
`manifest.json` binds package name/version/source/license expression and Cargo.lock
registry checksum to the archive's `.cargo_vcs_info.json` revision/path. Each copied
file is byte-for-byte upstream text, with its SHA-256 and immutable retrieval URL.
Packaging never fetches these URLs; mismatched packages/revisions/hashes fail closed.

The five static dependencies have full upstream license texts: asn1-rs-impl's
MIT/Apache texts and four tree-sitter grammar MIT texts with original attribution.

The four objc2-family entries are **upstream license statements**, not complete
permission texts. Their exact repository documents link license texts and state
uncertainty about Apple SDK-derived works. They remain `review_required: true`
in both this manifest and package inventory. Collecting these statements does not
resolve that publication-review question or establish legal clearance. No author,
copyright ownership or permission text was invented to fill the missing material.

When upgrading a dependency, review its exact new archive/revision and upstream
notices, update bindings deliberately, and rerun tamper/staleness/package tests.
Do not update versions/hashes automatically or copy an unrelated latest license.

Prepared binary archives store the selected notice bytes under `licenses/` and
map source paths to archive members in `DEPENDENCY-LICENSES.json`. The source
manifest is retained for provenance; the original `texts/` tree is not duplicated
inside the binary archive.
