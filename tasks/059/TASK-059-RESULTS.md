# Task059 platform matrix results

Complete against originalQ6-11. The existing maintainerselected macOSAppleSilicon
(aarch64-apple-darwin) target was measured on actual macOS27.2.1/arm64 forstatic
andnetwork. OtherOS/CPU/OSversions are not labeled tested or newlysupported.
Static/public and network/unpublishedpolicy stays unchanged; network's four
notice flags/publicationhold remain. No release, signing or remoteCI was run.

Source:HEADf07d782cd68cfac95d93585d5ace049f590160d4 plus actualsource maps. Fresh058
static clean-source-install and networkreleasebuild binaries were reused after
matching all204compiledsource/rule/Cargo/toolchaininputs and exactbinaryhashes.
Default555passed/0failed/3ignored proof remains applicable. A fresh native059
allfeature workspace run passed614/0/5,exit0. Existing explicitworkers/OpenSSH
ignored cases are not relabeled as passes.

Both variants were packaged, separatelyverified and installed in newprefixes;
all six actualcommands exit0. Installedhelp/version/rules and external synthetic
scan succeeded with3findings each; networkprobehelp/pinnedKEXinterface also
passed. Prefixes were removed. Archive/binary/rules/notices and actualinstalled
results are bound under evidence/platform-20261115/package-r2/packages/.

The initial nested sandbox was refused by macOS beforehelp execution; retained
as a harnessfailure. An exacthash-bound scratchcopy of the existinginstallchecker
changes only packagerlocation and redundantinnerlauncher. Every originalassertion
is preserved under inherited readonlysource/cache,scratchonlywrites and allnetwork
denial. Product/checker/packager files were not changed.

[Platformmatrix](evidence/platform-20261115/platform-matrix.json) is byteidentical
to target/quality-task-059/platform-matrix.json. [Commands](evidence/platform-20261115/package-r2/PACKAGE-COMMANDS.json),
[fullnetworktests](evidence/platform-20261115/native-r1/COMMANDS.json),
[buildreuse](evidence/platform-20261115/package-r2/BUILD-REUSE.json),
[reviews](TASK-059-REVIEWS.md) and [finalbindings](evidence/platform-20261115/EVIDENCE-FINAL.json)
record the originalcriterion, actualsource/applicability and bothaccepted stages.
Task052 remains unfinished and053 interrupted. Existingwork/history is preserved.
