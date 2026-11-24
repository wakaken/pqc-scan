# Task 068 implementation plan

Selected under the maintainer's task067-onward request on 2026-11-18 after066/067
completed. Same worktree/branch; baseline HEAD c7e76b5 plus the preserved, reviewed
task067 implementation. The adopted task066 contract and frozen expectations
remain authoritative; no acceptance or storage-scope changes are selected.

Implement R4-01..04/R4-06 as a bounded, closed `migration-context-v1` parser and
explicit immutable-evidence/target binding. Reuse067 bounded JSON, hashes and
target identities, and existing length-framed IDs. Preserve Context v1 and its
scan-wide semantics. Organization/system/target defaults merge complete claim
records, support entries individually; omitted inherits, explicit unknown replaces,
false remains false. Provenance and asserted/expiry intervals remain claims, not
scanner verification. Reject bad/duplicate/ambiguous ownership/references/content.

The alternative of reusing Context v1 globally is rejected by adopted D1/D2;
automatic discovery/tracking and external certification are outside scope.
Typed claims/record shapes and finite limits follow the adopted contract exactly.
Time is explicit; collection retains067 semantics, new UTC claim/as_of values use
whole-second offsets and normalize to UTC. No current policy/score is evaluated.

Steps: (1) record selection/source; (2) add closed typed parser/schema and bounded
validation, safe errors, normalization/provenance; (3) bind only matching content
and exact manifest targets, resolve scoped precedence/freshness and semantic
purpose conflicts; (4) test A02/A07/A12-A15/A17-A18 parsing/binding assertions,
false/unknown/missing, scope/substitution/ambiguity, all limits, privacy and legacy
context compatibility; (5) update paired guides, inspect combined change and
reuse /root/v04_implementation_review; (6) run applicable fmt/workspace/build/
strict clippy/configuration checks, reuse /root/v04_test_review, reconcile original
criteria and bind final evidence before completion. Existing067 code/results and
historical evidence are preserved; downstream changes receive explicit review
and evidence applicability rather than rewriting067 snapshots.

Task069 owns full business decisions, CLI and report reassessment; no outcome is
claimed from context parsing alone. No external contact, costs, probing, release,
publication or numeric-score adoption is authorized. Task071 resource acquisition
remains independent of this implementation.
