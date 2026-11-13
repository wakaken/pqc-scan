# TASK-042 lock-format controls

This is a separate frozen tuning supplement to expanded-v2, whose bytes/truth
remain unchanged. Only dependency_presence is assessed: an explicit declaration
of a recognized library with version-dependent PQC capability, not an installed,
resolved, enabled or runtime-safe configuration. This is the same property as
crates/pqc_scan_detectors/tests/pqc_readiness.rs,
inventories_dependencies_without_claiming_pqc_is_enabled, whose package.json
example names @noble/post-quantum 0.7.1. The two new files test that declaration
in the other supported npm lock formats. They are data only; never install/run.

Review all 30 nominated rules against both source files before any supplement
scan. Supply independently source-derived labels with algorithms, usage,
artifact, caseKind, parameters, reason. Positive/negative are measured reference
properties, ambiguous/unsupported are excluded. Retain exact declaration version;
no version resolution or provider assurance. Unknown algorithm is not safe.
Two source-only reviews, adjudication and hash-bound freeze precede first output.
All files are tuning, authored by a rule-aware coordinator after v2 output. Prior
exposure must be disclosed; independence refers only to other supplement labels
and supplement scanner outputs. No new external authorship independence claimed.

The same full source audit contract as v2 applies: coverageAudit must contain
status, scope all_nominated_rules_all_files, fileCount, nominatedRuleCount,
reviewedFileRulePairs, missingFacts, unresolvedDefinitions, reason and protocolSha256.
Zero output cannot verify annotation completeness. Existing scorer is authoritative.
No new numeric gate, rule or product semantics.
