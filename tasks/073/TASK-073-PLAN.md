# Task073 plan and D4 supplement

Selected by maintainer on2026-11-18 after commit c1f8a4bc5478eefb2b1babb6b2aa86295f9c8d34,
in the existing worktree and branch. U01/U02 and original criteria remain unchanged.
071/072 remain untouched;075 final reconciliation remains dependent on them.

## Outcome, compatibility and choices

Generate the adopted migration-context v1 without hash/ID transcription or invented
claims. Reuse SavedEvidence, typed ClaimSet/Provenance validation, bind, and safe_output.
A shared typed generation API accepts explicit target selections and optional typed
claims, generates default aliases and validates final bytes against the same reader
and evaluator binding. No new stored schema, policy, rule or cryptographic behavior.
Import/export uses the adopted closed context and preserves all supported information;
no lossy form projection. No CLI conversation, web implementation or default scan change.

D4 supplement: add explicit context-template INPUT --input-format FORMAT. --list-targets
prints a one-based readable list (algorithm, role, source/location and bindability).
Generation requires --out FILE and --as-of TIME; repeat --target N to select only those
rows from this exact input. No selections produce a valid unbound empty template;
claims without selected targets reject. Default aliases are organization/system-1,
customizable via --organization and --system. Optional repeated --set FIELD=JSON_VALUE
and --unknown FIELD provide known/explicit-unknown claims; omitted fields stay omitted.
Known strings use JSON quoting. Support fields use support.peer, etc. Any answers
require --actor and --asserted-at; --valid-until is optional and never invented.
No guessed positive values or all-target selection. Invalid/duplicate/ambiguous indices,
fields, provenance and stale imported input bindings fail using existing fixed errors.

Use existing1MiB context and256-system/4096-binding ceilings. Bound parsing applies
before generation/import. Complete rendering and input/context alias checks precede
safe atomic file replacement. No new dependency, output bypass or outbound operation.
A cosmetic JSON example would leave ID transcription unresolved; generating defaults
for every target would silently widen scope. Explicit selections and shared validation
meet both usability and ownership requirements.

## Steps and acceptance evidence

1. Add shared claims/selection generation and complete context export over existing
   typed records. Keep normalized semantic values and all provenance/references.
2. Add thin headless CLI, readable list and safe write; document EN/JA examples.
3. Test sparse/empty, unknown/false/N/A, exact grouped scope, malformed answers,
   duplicate/ambiguous targets, stale/substituted inputs, roundtrip and output aliases.
   Run actual generation followed by assess, retained legacy contracts and features.
4. Inspect combined diff, reuse original implementation/test reviewers, run required
   fmt/workspace/build/strict clippy and affected optional-feature checks, bind final
   source/logs/binaries and record results. Main agent closes only all criteria passed.

Main agent owns all development/testing/docs. Same reviewer identities are
/root/v04_implementation_review and /root/v04_test_review; no extra delegation.
