# Task058 usability and bilingual workflow review

Measured against HEADf07d782 plus native-r2 source/snapshot manifests. Both independent reviews accepted; final-source and preservation checks passed.

## Actual newcomer workflow

Rust1.93.1 cargo install --locked --offline into a fresh scratch prefix succeeded.
Matching rules were copied from that source snapshot and selected explicitly by
absolute path. The installed binary scanned a separate two-file synthetic project
without executing code, resolving dependencies or contacting network endpoints.
Both static and network binaries were built; network help exposes TLS/SSH/KEX
commands while static stays local scanning. This does not authorize network
publication or broaden platform support.

USER-COMMANDS.json records24 exact commands and actualexpected exits:global/
scan/rules/diff help and version, optionalprobe help, ruleslist/show, defaultscan,
English/Japanese extendedscans, severity/incompleteness/combinedgates, invalidflag,
missinginput and explicitlyremoved nativeflag. The original six outputfilenames
and eleven explicitly requested extendedfilenames match source documentation.
The same sourceinput produces identical legacy findings across languages.
Help stdout/stderr behavior is independently asserted by default/allfeature
cli_information tests. The external publicRustclient compiles old-style name/
detect, ScanConfig/Detection/Evidence constructors, three scanentrypoints and
original write_reports; unknown-assurance/default completeness is preserved.

The dynamic Gemfile fixture is intentionally not fully decidable statically.
Diagnostics report selected2, analyzed1, partial1 and statuspartial. Both actual
human reports name Gemfile and explain incomplete inspection; defaultscan exits0
without claiming all-safe, require-complete exits3, fail-on high exits2 and
require-complete takesprecedence withreports retained. The report guides explain
findings versus actualuse and incomplete inspection, not universal safety.

## Copy, documentation and links

Both rootREADME files and all ten userguides were reviewed for installation,
scan defaults, rulesselection, informational flags, outputfilenames, optional
inventory/assessment/CycloneDX, exit0/1/2/3 and source/deployment limitations.
135 local linkdestinations and83 local heading/explicit-ID anchors resolve.
Eight actualexternal HEADchecks returned200. HEADsuccess verifies reachability,
not externalfacttruth or publishedartifact authenticity.

Paired REPORTS guide sentences now explain longreferenceURL wrapping. No new
CLI/API/rule/crypto/storage contract was introduced. Explicit historical
--native-json removal remains rejected under the recorded task025 maintainer
choice; the stablelibrary nativewriter remains exported. Other intentional
changes and sourcebound tests are mapped in compatibility-matrix.json.

## Actual browser observations

Actual Chrome157.0.8103.51 used only fresh profiles and dedicated127.0.0.1 servers
serving eight intended syntheticreportfiles. It did not alter a user's visible
tab/profile. Server/profile were closed after checks; actual loopback URLs,
request restrictions, keyboard events and runtimeexceptions are in each RESULT.

- browser-baseline confirmed390device emulation expanded to718 layoutpx when
  long referenceanchors were opened; wideelement diagnostics identified anchors.
- browser-red's strict390px expectation failed beforeproductchange. The minimal
  compact.css repair applies overflow-wrap:anywhere to anchors. Linkvalues,
  machineoutput, schema, IDs, rules and severity remain unchanged.
- browser-green verified English/Japanese1280x900 desktop and390x844 mobile
  widths/scrollWidths390. Main agent inspected both mobileimages. Native trusted
  Enter toggles details; typedsearch works afterloaded-offline. LongURLs no longer
  force viewportwidening. Before/after records and siximages are retained.
- browser-user-flow-r2 exercised the actual installed-CLI reports rather than
  only a writerfixture. Bothlanguages show unfinishedGemfile/inspectionreason,
  exact390px layout, keyboard expansion and offline rulefiltering. Its preceding
  attempt used statuscopy from an unusedhelper rather than active renderer;
  diagnostics already correctlyreportedpartial. That harnessfailure is preserved
  in browser-user-flow/FAILURE.json; no product/fixture was altered to hideit.

This is finite actualbrowser acceptance, not allbrowser/assistivetechnology or
productiondeveloper-usability certification. No population accuracy, safeuse,
formalplatform, freshhostedCI, signatureauthenticity or publication claim follows.

## Checks and source provenance

native-r2 commands all exit0:public-client/report/export targets, workspace555
passed/0failed/3ignored, default/allfeature strictclippy, workspacebuild, clean
sourceinstall, optionalCLIinformation and release networkbuild. Initialnative-r1
exportcapture collision failedcollection; per-command scratchcapture resolvedit
without weakening confinement or changingproduct/tests. Logs remainhistorical.
Default/allfeature use distinct buildtargets. Binary/source/input/tool/environment
and cleanup bindings accompany the native evidence. Rustfmtcheck exits0.

The two original deliverables are copied byteforbyte to target/quality-task-058.
Both independentreview acceptances and final-source/reference/preservation checks
passed; the original058 criteria were not narrowed.
