# Task058 compatibility and usability results

Task058 is complete against originalQ6-10/Q6-12. Source:HEAD
f07d782cd68cfac95d93585d5ace049f590160d4 plus the actual dirtyworktree bound in
[evidence/compatibility-20261115/native-r2/SOURCE.json](evidence/compatibility-20261115/native-r2/SOURCE.json).
Task052 remains unfinished and053 interrupted; all priorwork is preserved.

The only productchange wraps longHTML referenceanchors. Actual390px device
emulation formerly expanded to718px due to thoseanchors; a pre-fix strict390
check failed. The minimal CSSfix preserves linktext/href and machinecontracts.
Paired reportguides explain it. ActualChrome tests now retain390px viewport and
scrollwidth in bothlanguages, with nativekeyboard and loaded-offline filtering.
The external publicRustclient test exercises an oldDetector(name/detect only),
three scanentrypoints, singlecall/defaultunknown assurance and original sixfile
writer interface. Eight publiccontract modules and twelve JSONcontract assets
are byteidentical to v0.2.0's actualcommit4ee5ceb2ca6bab3cf3336fae17fb9f42f3446d65.
The annotatedtag objecteecbf1... is distinguished from thatcommit; preliminary
metadata and the review correction are retained rather than rewrittenhistory.

Native checks all exit0:targetedpublicclient/report/048exports, workspace555
passed/0failed/3ignored, strictdefault/allfeature clippy, workspacebuild, clean
cargo install, optionalCLIinformation and networkrelease build. Rustfmtcheck0.
[Exactcommands](evidence/compatibility-20261115/native-r2/COMMANDS.json),
[binarybindings](evidence/compatibility-20261115/native-r2/BINARY-BINDINGS.json).
No hostedCI/publication action was performed.

The installed binaries exercised24 actualcommands:help/version/rules, default6
outputfiles, selected11 extendedfiles, English/Japanese parity, exits0/1/2/3,
strictincomplete-over-severity precedence and retention ofreports beforegateexit.
A separate dynamicGemfile project produces selected2/analyzed1/partial1; both
actualhumanreports name the unfinishedfile and reason. Fixturecode was not run.
[Userresults](evidence/compatibility-20261115/native-r2/USER-RESULTS.json).
135 localdestinations,83 localanchors and46 documentedCLIcommands were checked;
eight externalHEADs returned200. This does not authenticate releasedartifacts.

The two originaldeliverables are retained with byteidentical targetcopies:

- [compatibility-matrix.json](evidence/compatibility-20261115/outputs/compatibility-matrix.json)
- [usability-review.md](evidence/compatibility-20261115/outputs/usability-review.md)

The matrix maps supportedpublicsurfaces to actualtests, explicitly retaining the
maintainer-approved --native-json removal, previousguidance/detection changes and
static/network naming consolidation. No newAPI/schema/rule/crypto/severity or
platform-support promise was introduced. The evaluation is finite, not proof for
every downstreamclient, browser, input or assistivetechnology.

Both isolated revieweridentities were reused without delegation or extraagents;
[reviewrecord](TASK-058-REVIEWS.md) records acceptance and the resolvedP2.
Initialevidence-capture collision and inactive-helper statuscopy mismatch remain
failedharness attempts, not hidden passes or productregressions. Finalsource,
tool/binary/artifact hashes and historicalpreservation are indexed in
[EVIDENCE-FINAL.json](evidence/compatibility-20261115/EVIDENCE-FINAL.json).
