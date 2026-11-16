# Original-wording reconciliation notes

These are evidence-backed corrections/proposals for the final criterion table,
not blanket completion or a change to acceptance. The pinned official extract
remains31325acd7963e0c9203da0ff798802a766237517.

-0.contribution explicitly presumes GitHub uses issues/PRs unless otherwise
noted. A separateCONTRIBUTING file is not required.31actualPRs exist.
-0.english requires English docs and ability to accept English reports; a past
English incident is not required. Existing English docs and GitHub inputs apply.
-0.build_floss_tools asks whether the project is buildable with FLOSS tools, not
whether every supported OS SDK is FLOSS. The actual Linux builds are relevant.
-1.documentation_current requires effort and ordinary defect handling, not
perfect documentation or a new full documentation audit. Local/current/public
policy versions must still be distinguished.
-1.coding_standards_enforced allows formatter/static-tool enforcement. Do not
automatically add a branch-protection requirement absent from its text; public
CI's actual formatting checks and source-change applicability must be assessed.
-1.build_standard_variables says relevant variables; examplesCC/CXX/LDFLAGS do
not force a Rust project to implement every language-specific flag. Explain Rust
RUSTFLAGS/linker and actualC compilation; do not pass an unexercisedC++path.
-1.build_preserve_debug has actual source-install evidence. Investigate whether
unchanged packaging's byte-copy semantics cover the remaining staging path; do
not assert unmeasured macOS debug execution.
-1.installation_common permits a commonly used package manager, includingCargo.
It does not require a custom installer or a single bootstrap command.
-1.documentation_roadmap requires at least a year; existing version roadmaps do
not establish that temporal horizon. No plan expansion is authorized.
-1.signed_releases applies to source and generated results; an archive attestation
does not automatically sign source/tag. Annotated tag inspection is separate.
-2.hardened_site explicitly identifies GitHub/GitLab as known conforming sites.
Read the actual project-site inventory and preserved headers; do not demand
unrelated external reference sites conform as project-hosted services.
-2.security_review explicitly requires a human security review. Maintainer
confirms none personally, and earlier none from another human. This is a known
Unmet practice, not missing human truth or a requirement to recruit for052.

No known confirmed project-vulnerability case was reported by the maintainer.
That does not retrospectively classify the existing safety defects or clear their
severity/timing ledger.48OSVrecords now have actual primary-source responses;
current rootCargo includes only three affected package names, at fixed versions.
Other ecosystem/test/feasibility matches require their own scope attribution.

A new branch validity issue is recorded inBRANCH-ATTRIBUTION-INVESTIGATION.json:
inline #[cfg(test)]code is retained by path-only filtering. Neither an improved
threshold result nor another identical run would resolve that mapping defect.
Genuine source-statement metrics remain unestablished by the checked tool docs.
No new tool was installed and no product/shared executable was changed.
