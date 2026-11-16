# Verification follow-up interpretation

## Build variables and debug preservation

The initial existing image lacks currentyaml_serde. A second attempt selected only
existing locked crates/index records, but the probe directed Cargo extraction to
a read-only cache. The third run corrected scratch placement and reached actual
all-feature compilation, which failed at link with ENOSPC. Static unoptimized and
release/all-dependency-debug attempts also reached link and exhausted4GiB.
All failures are retained, never passed. No network fetch or product fix occurred.

The successfulr6experiment uses normal release source installation for the static
variant, requesting project debug withCARGO_PROFILE_RELEASE_DEBUG=1and explicitly
disabling third-party package debug through a Cargo profile override. This avoids
retaining all dependency debug information in the same4GiB scratch. The installed
40,862,248-byte ELF retains.debug_info/.debug_line, the Rust linker marker symbol,
and its exact hash. RequestedCC/CFLAGS are observed in101compiler calls. C++ was
not compiled; CXX/CXXFLAGS are unexercised. GenericLDFLAGS, macOS archive install
and all-feature debug installation remain unverified. These are Partial criteria,
not universal build conformance. Independent debug/no-debug controls pass.

## Branch and statement coverage

The first coverage probe inherited a32MiB/file bound appropriate for small evidence
but too small for instrumented executables. A second run uses the existing044
reviewed1GiB/file limit while keeping6GiB memory/no swap,4GiB scratch,128PID,2CPU,
read-only inputs and offline network. Default builds/tests/export succeed:
4105/5454mapped LLVMbranches,75.26585991932527%. Lines and regions are separate.
Its sequential all-feature build fills scratch. The third attempt runs only the
unchanged existing044all-feature measure in a fresh bounded environment; this is
configuration separation, not an increased resource limit or disabled tests.
No source statement rate is derived from LLVMline/region counters.

## Evidence applicability

All199declared060source bindings still match; its raw binary repeatability remains
bounded to its original environment. The newly splitSECURITY policies and Japanese
entry links are separately hashed. Earlier validation receipts bind older policy
bytes and are historical, not current whole-root self-scan verification.
Current observations do not modify frozen053/catalog evidence or publish policy.

## Final collected observations

The all-feature-only third attempt successfully compiles, then fails because the
unchanged044collector assumes a previous default-binary inventory. In the fourth
attempt the measurement driver explicitly supplies the actual empty initial
inventory of its fresh container, rather than inventing a default result. The
unchanged measure function then completes,exit0,280.28seconds; source hashes
match and the container is removed. The earlier attempt is retained separately.

Default test summaries total555passed,0failed,3existing ignored; all-features
total614passed,0failed,5existing ignored. Default mapped branches4105/5454
(75.26585991932527%); all-feature mapped branches4248/5658(75.07953340402969%).
Both observed rates are below80%. These are fixed-nightly exploratory LLVM
measurements, not a change to the product Rust1.93.1pin. Default optional modules
are unmapped; all-feature unmapped files are core/error.rs and two crate lib.rs
shims. The previously documented duplicate-source resolver and env-cleared child
collection limitations still need attribution review; no raw counter is silently
removed or repaired. The whole sequential second host run exits1after the later
all-feature build, even though its preceding default measurement succeeds.

The r6installed-debug positive/negative controls and actualsource install exit0
underRust1.93.1. The earlier32MiB/file bound was unsuitable for large instrumented
binaries; final probes use the existing reviewed1GiB/file cap, with unchanged
6GiB memory/no swap,4GiB scratch,2CPUand128PIDcontrols. Bounded artifact exports
remain64MiB. No aggregate resource limit was raised to pass the measurements.
