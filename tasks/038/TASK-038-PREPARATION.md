# Task 038 preparation evidence

Date: 2026-11-10. This is local preparation, not a released package or task
acceptance. Parent worktree snapshot/HEAD and product build evidence remain
separately recorded by the parent evaluator. This preparation has no commit.

## Tests and preliminary execution

- `python3 -m unittest discover -s tests/distribution -v`: initial RED failed
  because the new script did not exist. Initial GREEN attempt found an absolute
  path masked by prefix joining; validation now checks inputs before joining.
  Final targeted run: 3 tests passed. Assertions cover byte-identical archives,
  changed-member checksum rejection, unsafe names and escaping license symlinks.
- Preliminary static packaging succeeded twice with the same supplied
  `target/release/pqc-scan`; archives were byte-identical, SHA-256
  `e7ef8f8c3b68fc6dd66533e472afd8606337a35946ba3149cacdd4398ab6efa3`.
  These archives precede the final packaging-script/source freeze. They are
  superseded preparation samples, not final release provenance.
- Preliminary inventory: 139 resolved packages; five have no license-name file
  in the local source cache: asn1-rs-impl, tree-sitter-java, tree-sitter-ruby,
  tree-sitter-rust and tree-sitter-typescript. Declared license expressions stay
  in the inventory, but missing notices are a publication-review gap.
- Initial installation attempt observed current static binary `--help` exit1
  with usage on stderr. No product code was changed by this task; acceptance
  uses functional rules-list and scan commands instead. The guide recommends
  inspecting help during upgrades but does not assert its exit status.
- A SHA1 fixture produced no findings because that pattern is not an applicable
  rule; it was replaced with a known TLS_RSA rule fixture. A fixture below target
  was excluded by the checkout ignore policy; the harness now creates an external
  OS temporary root and removes it after execution.
- `python3 tests/distribution/check-install.py target/task-038/static-first/pqc-scan-0.1.0-aarch64-apple-darwin-static.tar.gz --out-dir target/task-038/static-install`:
  exit0. Clean empty prefix, explicit installed rules, external synthetic fixture,
  `rules list` and JSON scan passed under macOS sandbox-exec `(deny network*)`.
  Exact archive/fixture/binary/source bindings and captured stdout/stderr are in
  ignored `target/task-038/static-install/INSTALL-RESULTS.json`.

## Outstanding acceptance

Re-run both variants after final source freeze and parent release builds; compare
archive bytes and record exact final source/build/binary bindings. Combined probe
archive installation, independent packaging/license/security/doc review, remote
manual workflow, other platforms, independent binary rebuild reproducibility and
external release verification have not run here. Task032/033 protocol acceptance,
task036 actual-browser acceptance and task037 quality/release scope remain
separate gates. No gate is satisfied by writing a workflow.

Missing license notices must be resolved or explicitly excluded by maintainer
review before publication; no network fetch or copying unreviewed cache trees
was used to conceal them. No release/tag, registry upload, signing credential,
message, push, PR or repository setting was created or modified.

## Frozen dirty-snapshot two-variant run

Parent supplied both release binaries and froze runtime sources/README changes.
HEAD at execution: `cae2730fccce1d10198a49464f699a7df8f2d4d2`;
`source_dirty: true` is recorded explicitly. This is an uncommitted snapshot,
not a clean release. Both variants bind source-input digest
`c84520f5b60c371910704feb7c020b8e388048a2657652e9c17ba415d4aef49c`.

| Variant | Supplied binary SHA-256 | Archive SHA-256 | Inventory |
|---|---|---|---|
| static | `48e4096216794cc85e0dcf9b7585ed605a1e1dd902ac65e7779d447464574fb2` | `f39f7b0d935c1a6e9ea8a2e2986c342b29edb2c59692b1cda9f530c1768abe27` | 139 packages, 5 missing notice-file sets |
| combined-probes | `7cebb433c007c174affe3946b15030837ef0a46c8c9d1d28451e0493240d86fe` | `ce659ccc31bb62a2a5353ecbfaf1cfe33d6fb142e348f1a024a5822c7c022e73` | 168 packages, 9 missing notice-file sets |

For each variant, two `package-release.py` invocations exited0; `cmp` and a
separate Python byte-equality assertion succeeded. Each generated archive was
internally checksum-verified during preparation. Each `check-install.py` invocation
exited0: installed `rules list` exit0, external synthetic `scan` exit0, 2 findings,
with all networking denied by macOS sandbox-exec. Fixture digest, exact dirty
provenance and command outputs are captured in:

- `target/task-038/final-static-{first,second}.log`
- `target/task-038/final-combined-{first,second}.log`
- `target/task-038/final-static-install/INSTALL-RESULTS.json`
- `target/task-038/final-combined-install/INSTALL-RESULTS.json`

Archives and adjacent external hashes are under
`target/task-038/final-{static,combined}-{first,second}/`. Empty install roots and
external synthetic fixtures were removed after checks. The combined inventory's
additional missing notice files are block2, dispatch2, objc2-encode and objc2.
These gaps are visible, not a claim of a complete license bundle. The final
workflow has not been dispatched and no artifacts were sent externally.

Any later source/commit change requires fresh packaging for that revision.
These successful local archive/install checks supersede the earlier unexecuted
combined-install item only; protocol/browser dependencies, independent review,
notice completeness, remote CI and actual release verification remain open.

Independent review found the initial workflow label `macos-14` retired on
2026-11-02. It was corrected to `macos-15`, explicitly listed as Arm64 by the
[official runner-image matrix](https://github.com/actions/runner-images/blob/main/README.md)
(checked 2026-11-10). The native architecture assertion remains. This correction
is source review, not an executed remote workflow; current runner availability
must still be checked when dispatch is separately authorized.

Independent review also found `tarfile` iteration had materialized members before
applying the advertised count cap. The verifier now parses only fixed-size USTAR
regular-file headers, hashes payloads in 64 KiB chunks, rejects PAX/GNU extensions
before their payloads are decoded, and limits compressed bytes and total decoded
bytes (including headers/padding) to 512 MiB, members to 10,000 and checksum text
to 2 MiB. Duplicate, nonregular, malformed terminator/padding and trailing-data
members fail. Regression tests cover count/decoded limits, PAX rejection and
nonregular/duplicate rejection; initial RED reproduced the absent decoded-bound
control, then all 5 targeted tests passed after the correction. Previously
produced local USTAR archive verification still exited0. Final packages must be
regenerated after the corrected script and review findings freeze.

## Independent preparation review

The independent SSH reviewer reviewed packaging, installation, provenance,
licenses, security policy and the manual workflow. Two blocking findings were
fixed: the retired macos-14 runner became the current arm64 macos-15 label with
a native-architecture guard, and archive verification no longer materializes all
headers before enforcing limits. A bounded USTAR-only reader now checks fixed
headers and streams contents with member/compressed/decoded/manifest caps,
rejecting PAX/GNU/nonregular entries before their bodies. Five tests passed; the
reviewer independently rejected corrupt header checksums, truncated header/body,
nonzero trailing data and excessive padding. No blocking source finding remains
in this preparation scope. Generated modes are fixed to 0644/0755; verification
is an integrity check, not an authenticity or comprehensive permission policy.
Notice gaps, browser/dependency acceptance, remote execution, other platforms and
external publication remain unapproved or unexecuted. Final clean-source package
records will supersede the preliminary archives above.

## Final clean-commit local preparation

The following clean-commit evidence supersedes dirty-snapshot packaging bindings
above. Source-review corrections were accepted before execution. At HEAD
`9201dc5022ea918e1416db7abf710db58a4bd855`, `git status --porcelain` was empty
before each package and after all packaging/install commands. Every provenance
records `source_dirty: false` and the SHA-256 of an empty tracked diff.
No tracked documentation was edited until both variants' two archive generations
and installation checks finished.

| Variant | Archive SHA-256 (both repeats) | Supplied binary SHA-256 |
|---|---|---|
| static | `ef60fab5c8807796e99eafe9327d5ab275c3149391e3cc44149c49798b67dc7e` | `48e4096216794cc85e0dcf9b7585ed605a1e1dd902ac65e7779d447464574fb2` |
| combined-probes | `4828e5c3a48e19edca474fd41bf77b40efd845e3af2dc5a34c9f0efd6d606f01` | `7cebb433c007c174affe3946b15030837ef0a46c8c9d1d28451e0493240d86fe` |

Each variant executed two packaging commands, `cmp`, verification of both archives
and an offline clean-prefix installation harness; all 12 commands exited0.
Both installed `rules list`/`scan` command pairs exited0, produced 2 expected
synthetic findings and ran under macOS sandbox-exec with `deny network*`.
The external fixture SHA-256 was
`0367002dc310edee63f28dda54ea17fa290a9817da5e2ca92af1652b12dffb44`.

The packaging input manifest contains 125 explicitly selected source/assets/docs
with aggregate digest
`fcd8f74c22d27b22fb7a7e8d7395d7e5be748ba909920dd9b42718375b36e965`.
This is the packaging tool's declared input scope, distinct from the parent's
full product-build source inventory. Full per-file hashes, English/Japanese
DISTRIBUTION/PROBING and SECURITY hashes, command arrays/exits, toolchain and exact
archive/binary bindings are in [the durable JSON record](evidence/TASK-038-FINAL-CLEAN.json).
Raw logs and installed command outputs remain in ignored
`target/task-038/FINAL-CLEAN-RESULTS.json`, `final-{static,combined}-*.log` and
`final-{static,combined}-install/INSTALL-RESULTS.json`. Archives and adjacent
external hashes are in `final-{static,combined}-{first,second}/` under that root.
These paths now contain the clean-commit results, replacing preliminary artifacts.

The static/combined dependency inventories remain 139/168 packages with 5/9
missing notice-file sets. Task038 remains blocked for notice review and dependency
acceptance, including actual-browser task036. The manual workflow remains
unexecuted; no remote CI, publication, signature authenticity, other-platform
support or independent binary reproducibility is established by these results.
