# Static v0.3.0 acceptance and publication preparation

On 2026-11-17, after the source-specific release assessment of develop
395368622e5941eefbae732f370831223aa6e8fe, the maintainer instructed this chat to
complete the three stated remaining actions: explicit bounded product acceptance
and publication authorization, final hosted build-provenance preparation and
verification, and release-facing documentation/maintenance policy. This records
that instruction as acceptance and conditional publication authorization for
static macOS arm64 v0.3.0 only. No network publication, cross-platform support or
quality-criterion relaxation is authorized. Actual publication remains separate
from this preparation; no Release is claimed published by this record.

The maintainer separately selected the same best-effort maintenance policy as
0.2.x for 0.3.x: latest verified snapshot in each series, no SLA or older-version
backport guarantee. Paired SECURITY, distribution guides and release notes record
this decision. Historical assessments and approvals remain unchanged.

All 73 original clauses retain the bounded assessment in audit-fixes/REQUIREMENTS.
The F-01/F-02 fixes, independent implementation/test reviews, current CI and exact
Code Scanning receipts remain applicable to unchanged product/rule/Cargo inputs.
Known limits remain explicit: nine unobserved invariant directions, source-aware
nominal controls rather than independent population accuracy, auxiliary LLVM dump
failure and additional fuzz Partial, same-host rebuild scope, broad evaluator
Unknown, historical Code Scanning failure/2135-newness Unknown, and bounded
operational/continuous-monitoring evidence. Acceptance does not relabel these.

Publication documentation changes only prose and release/maintenance status.
Current applicability binding is independently revalidated because the complete
paired COVERAGE files are hash-bound. No support paragraph, parser, corpus/truth,
scoring, threshold or source/review rejection guard is weakened. The selected
record and its negative regression receive both existing review stages.

The final merged source must receive hosted CI and the existing release.yml
preparation workflow, including static repeated archives, clean installation,
attestation and exact source/workflow verification. Hosted receipts and final
archive digest belong in the PR and retained artifact receipt; they are not
claimed executed at this document's authorship. The workflow additionally builds
network diagnostic artifacts; their presence does not remove four notice holds
or authorize their publication.
