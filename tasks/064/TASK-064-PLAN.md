# Task 064 distribution consolidation

The user authorized two distribution configurations and delegated clearer naming,
with internal feature switches retained and scripts/CI/tests/docs updated and
verified. No publication or release is authorized.

## Naming and compatibility

Choose `network` (network diagnostics included) alongside `static`. `all-probes`
does not explain what “all” includes; `full` would imply broader completeness;
`with-probes` still requires explaining “probes”. User documentation names the
actual TLS/SSH capabilities and says network also includes static scanning.

Preparation accepts only static/network and emits corresponding filenames and
provenance labels. Network uses the former all-probes feature list unchanged.
Keep TLS/SSH/SSH-KEX Cargo features and commands independent. Removing the
combined distribution does not remove custom source builds or SSH offer collection.
Read-only archive verification/install compatibility recognizes the historical
combined-probes/all-probes labels; do not offer them for new preparation. Do not
rewrite completed tasks, raw evidence, historical release notes or records.
Only labels change in existing archive metadata, as explicitly authorized; do not
change schemas or unrelated archive content contracts.

## Implementation and validation

1. Capture pre-existing source and diffs, then update the packaging variant map,
   exact command-surface validation, installer and dependency assessment.
2. Remove the redundant CI build/package/install block and rename the retained
   network block. Keep existing static-only provenance attestation scope and
   manual/non-publishing permissions; do not add release actions.
3. Update current English/Japanese README, distribution/probing guides and
   current notice guidance. Preserve historical three-variant measurements.
4. Test both selectors, removed-name rejection, exact feature/command mapping,
   old archive reader behavior and CI consistency/security constraints. Run
   relevant Python syntax/tests and source/link checks.
5. Package and install real native static/network binaries, repeat packaging and
   compare archives. Reuse task063's tested native binaries only after matching
   all compiled Rust/Cargo inputs and original binary hashes; labels do not enter
   the executable. No Rust/Cargo source change is planned, so repeat Rust suites
   only if new changes invalidate the source-bound results.
6. Refresh the two-variant dependency assessment. Review the combined change
   using the same two isolated reviewer identities as earlier follow-ups,
   implementation then test adequacy, with scoped confirmation of material fixes.

Hosted Actions are not dispatched by this request: validate workflow syntax and
semantic build/artifact paths locally, and exercise packaging/install commands
on native macOS. Report hosted execution as unexecuted rather than a CI pass.
Retain source/tool/command/exit/output hashes under tasks/064/evidence; keep task
unfinished if any required local verification or review is missing.
