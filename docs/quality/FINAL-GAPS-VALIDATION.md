# Final gap validation — 2026-11-11

Base: merged6e098de; source changes are tracked in codex/final-quality-gaps.
Scope: state reconciliation, individual self-scan triage and all-probes technical
packaging including SSH KEX. Publication/signing and existing maintainer decisions
are preserved. Implementation reviewer final_gaps_implementation_review approved the combined
change with no required fixes, checking actual result/source digests and new
notice bindings. Final-test review and command/source/archive bindings are recorded
after execution. Earlier evidence remains historical.

Code Scanning:109 retained results reviewed,81 exact expected results registered,
28 dependency observations retained (14warning/14note). Full raw1113 results retained.
No scanner/rule/severity/threshold change or live alert dismissal.

Packaging:static and combined-probes preserved;all-probes adds probe-ssh-kex to
build,metadata,provenance,notices and exact clean-install command-surface checks.
All-probes includes271 packages with no missing notices. Five publication-review
flags remain: the original four objc-family flags plus russh-util dirty-VCS provenance.
