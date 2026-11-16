# Development, contribution and maintenance process review

Assessment date: 2026-11-15 JST. Scope: Q6-05, with controls from Q6-06 and
the complete 145-item Passing/Silver/Gold assessment in `best-practices.md`.
All file paths in this report are relative to the repository root. Evidence keys
resolve through `EVIDENCE-REFERENCES.json` in the requested output set.

Local baseline: `f07d782cd68cfac95d93585d5ace049f590160d4` plus the preserved
task051/063/064 worktree. `tasks/052/evidence/assessment-20261115/BASELINE.json`
binds the actual files. Live GitHub observations and the remote Scorecard source
are separate. No product, CI, executable examples or reusable scripts changed
in task052; one-off acquisition/assessment commands do not add product behavior.

## Responsibility and evidence

| Activity | Written procedure / accountable role | Actual evidence | Gap or limit |
|---|---|---|---|
| Propose a change | AGENTS.md, task lifecycle, best practices; contributor proposes, maintainer resolves scope/approval questions | Public API returned 29 closed PRs; all are authored by wakaken; latest merge is PR29 on 2026-11-14 | No general CONTRIBUTING guide or clear README contributor entrypoint; agent guidance is not a complete public contribution policy |
| Implement and validate | Main agent owns investigation, implementation, meaningful tests and documentation; pinned Rust toolchain and change-specific checks | Task063 records dependency/YAML changes, differential controls and feature-specific builds/tests/clippy; task064 records packaging/install results | Historical evidence has its own source and date. This assessment does not claim new Rust checks or validate dirty source using remote CI |
| Review implementation | AGENTS.md and REVIEWS.md require one isolated implementation reviewer, then one test reviewer for executable changes | Task063/064 review records identify reviewers and scoped acceptance | These are AI review identities. PR28/29 GitHub review lists are empty; Scorecard observed 0/10 approved changesets. No claim of non-author human approval |
| Run CI | pqc-scan.yml: quality for eligible PR/manual; baseline/scan for develop/manual; protocols explicitly false | Run43564396104 at remote f07d782: baseline/scan success, quality/protocols skipped. PR run43564067952 at be09d1eee: quality success; others skipped | Passing overall run does not mean every job passed. No hosted run covers local uncommitted 051/063/064. Prose path filtering remains; do not require filtered checks before addressing pending-check behavior |
| Enforce merge policy | Maintainer; current source policy and live settings are separate | Branch lists protected=false; CLI says Branch not protected for develop/main; effective rules/rulesets empty | No observed mandatory checks, reviewer enforcement, protected branch or bypass policy. Connector-only protection reads were 403 Unknown; fallback observations are retained independently |
| Manage dependencies | Cargo manifests/lock; exact-version notice manifest; existing dated review/remediation process | Task051 fixed tool/DB assessment; task063 compatible fixes; task064 static/network inventories | No periodic update bot or scheduled audit observed. Point-in-time audit is not ongoing monitoring. Network publication hold remains |
| Prepare and release | Distribution/release docs; task lifecycle and maintainer publication decisions | One v0.2.0 public release with static archive, checksum, provenance and paired notes. Task064 packaged/installed two current variants and three historical archives locally | Task064 hosted Actions were not run. Archive repetition reused binaries and does not establish independent reproducible compilation. Release tag is unsigned. No new release/push/tag/signing occurred |
| Handle security reports | SECURITY.md publishes interim safe contact-request procedure; maintainer role is implicit | Private-reporting API enabled=false; no verified private contact documented; public advisory list returned empty | No explicitly appointed security responder, backup, triage/escalation timeline or exercised fix-notification chain established |
| Preserve continuity | Maintainer governance decision | Collaborators API returned only wakaken with admin role | Repository ownership is observed; security role, backup, one-week continuity and MFA methods are not established by this result |

Actual CI job responses are in `CONNECTOR-PROCESS.json`; raw permissions and
errors are in `github-cli/`. Full query URLs, acquisition times and exits are
preserved under `tasks/052/evidence/assessment-20261115/`. GitHub collections
returned fewer than their requested page limit (100); recent run sampling is
explicitly only ten runs. This is not an exhaustive all-history CI review.

## Quality and maintenance interpretation

The process has public source, standard build/test tools, active development,
written compatibility/security rules and source-bound implementation evidence.
The root MIT license does not resolve each dependency's redistribution conditions.
The existing four objc2-family questions and network publication hold remain;
renaming the variant does not clear them.

Developer documentation, measured operations and remote enforcement are different
evidence classes. Review policy cannot prove a review happened; a passing job
cannot prove merge protection; a historical coverage number cannot establish
current statement coverage. Task044 line/region rates are not statement rates,
and its approximately 74% branch result is below the gold 80% criterion for its
own historical scope. No new coverage threshold or current coverage result is
introduced.

## Recommendations only

1. Make general contributor entrypoints and human decision/security roles explicit,
   using the existing procedures rather than adding parallel agent roles.
2. Consider develop/main merge controls only after selecting required jobs,
   reviewer/bypass policy and correcting workflow-level path filtering.
3. Establish a verified private reporting channel and named response/backup roles
   without publishing an unapproved SLA or unsupported backport promise.
4. Agree periodic dependency-monitoring scope/frequency and distinguish product
   closure from fixture/evidence inputs before adding automatic gates.

These are proposals, not newly created tasks, policy changes or accepted risks.
No settings were changed, reports sent, workflow runs dispatched or badge
applications made. Tasks053 and later were not started.

## Assessment status

All official medal criteria have an entry and evidence/owner-role mapping.
Formal human assignments and some required operational facts remain unavailable;
therefore complete verification against the original task052 criteria remains
unfinished. The missing facts do not prevent the completed Scorecard run. They
prevent claiming fully evidenced Best Practices compliance or task completion.
