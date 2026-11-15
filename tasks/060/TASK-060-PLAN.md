# Task060 independent clean rebuild comparison

Authorized unfinished053 prerequisite work;051/059done,052unfinished053interrupted.
OriginalQ6-13 compares actualbinaries, not repackaging identicalbinaries.
Use two independent fresh macOS OS-confined source/HOME/tmp/Cargo/target roots,
not a second physicalhost. Both have identical frozen source/dependencies and
installed compiler/SDK; only readonly dependency-source caches are shared.
Per-variant targets are distinct; no A buildartifact is accessible to B's policy.

Build static andnetwork in eachenvironment with identical releaseflags and fixed
SOURCE_DATE_EPOCH. The same pair of prefixmapflags covers both physicalroots,
mapping either to the same logicalpath; actualflag strings are equal in A/B.
This controls path variation without altering source/runtime/cryptographic logic.
Compare rawbinary SHA/length/bytes, actualversion, embeddedMachO metadata and, if
unequal, report observed regions/tool evidence and unexplained causes honestly.
No stripping/repacking comparison substitutes for rawbinary comparison.

Main agent owns investigation/code/measurements/docs. Existing sandbox enforces
readonlysource/cache,scratchonlywrites,noegress and finite percommand/perprocess
limits, not a VM or aggregate memory/disk guarantee.72GiB free was observed before
preparation; four bounded native releasebuilds withjobs2 use existingcache and
no new dependency/network/tool installation. Bothcontexts removed oncompletion.

Keep all originalcriteria/deliverables and source/tool/command/feature bindings.
Same two independent reviewer identities cover implementation then testvalidity;
complete only when originalQ6-13 is measured and explained against finalsource.
Source/currentbuild does not become a release or officialplatform guarantee.
