# Task 052 partial assessment and measurements

Status: unfinished. The fixed-tool Scorecard measurement, read-only GitHub
observations and complete 145-row self-assessment are available. Required human
ownership/operations and other Best Practices truth remain unverified; the task
is not done under the original acceptance and current completion rules.

Source: `f07d782cd68cfac95d93585d5ace049f590160d4` on
`codex/task051-dependency-supply-chain-review`, plus the preserved uncommitted
051/063/064 baseline. [Baseline](evidence/assessment-20261115/BASELINE.json) and
[cited source bindings](evidence/assessment-20261115/SOURCE.json) distinguish
local bytes from the remote committed source. No earlier task evidence was
rewritten. All new explanatory prose is English.

## Original requirements and results

| Requirement | Result and original acceptance coverage | Remaining work |
|---|---|---|
| Q6-05 | Development, contribution, review and maintenance procedures assessed; all medal criteria have written/operational evidence columns and an accountable role | Formal human role/security/backup assignment and several complete operational denominators not confirmed; role labels are not a new appointment |
| Q6-06 | Current branch protections/rulesets, workflow token defaults/job grants, collaborator access and relevant security settings read; observations and recommendations separated; connector 403 retained as Unknown | Account MFA method/continuity cannot be inferred from repository access; no settings changed |
| Q6-07 | Public interim contact procedure, support scope, missing private channel and unexercised intake/fix-notification chain recorded | Actual responder/backup, private report cohort, response intervals and security-release notification practice unavailable |
| Q6-08 | Official Scorecard v5.6.0, 18 checks, remote commit, tool hash/commit, date, exact command/exit, unavailable reason and limitations retained; valid final run exit 0 | Packaging is unscored (-1), with reason; this is an observed tool limitation, not a skipped whole measurement. No detection-accuracy or local-dirty-tree inference |
| Q6-09 | Official Passing 67 / Silver 55 / Gold 23 criteria all compared with evidence; suggestions included; conditional N/A justified; badge lookup/application state separated | 26 Unknown and 43 Partial findings remain. Source evidence alone does not establish personal knowledge, MFA, all-history response/review statistics or current statement coverage |

Finding a missing practice is distinct from failing to evaluate it. This
assessment records known Unmet findings as such. It remains unfinished because
some required truth and ownership verification is still unavailable; it does
not claim every product practice must pass before an evaluation can finish.

## Required deliverables

The following durable files have byte-identical requested copies under
`target/quality-task-052/`:

- [best-practices.md](evidence/assessment-20261115/outputs/best-practices.md)
- [github-controls.json](evidence/assessment-20261115/outputs/github-controls.json)
- [process-review.md](evidence/assessment-20261115/outputs/process-review.md)
- [scorecard.json](evidence/assessment-20261115/outputs/scorecard.json)
- [security-process.md](evidence/assessment-20261115/outputs/security-process.md)

Machine support: [criterion assessment](evidence/assessment-20261115/outputs/best-practices-assessment.json),
[Scorecard interpretation](evidence/assessment-20261115/outputs/scorecard-assessment.json)
and [source-key mapping](evidence/assessment-20261115/outputs/EVIDENCE-REFERENCES.json).
The full table has 62 bounded Met, 43 Partial, 26 Unknown, 3 conditional Not
applicable and 11 Unmet. These are assessment labels, not an official badge
percentage. No badge was applied for or awarded by this work. The separate OSPS
Baseline catalogue fetched during discovery is not claimed as an OSPS assessment.

## Actual measurements and command outcomes

On 2026-11-15 JST, the newly acquired official macOS arm64 Scorecard v5.6.0
binary reported tool commit `61e84743356275f6ffca23b1b2f2cb945e19eb96`.
The download matched the release asset SHA-256
`1537183c0fcc6d8d1572aabc7a949091a36a2ee6be5bcb65f487cbb0a7b40381`.
The [receipt](evidence/assessment-20261115/official/RECEIPTS-3.json) binds binary
bytes, URL and acquisition time. Cryptographic release provenance for this tool
was not independently verified; no system-wide install occurred.

`sandbox-exec -f <recorded-profile> <fixed-scorecard> --repo=github.com/wakaken/pqc-scan --format=json --show-details`
exited **0**, with empty stderr and all 18 default checks. The remote repository
commit is `f07d782cd68cfac95d93585d5ace049f590160d4`; overall score **4.3/10**.
This is OSS process/supply-chain evidence, not detection precision, cryptographic
correctness, public-release acceptance or a scan of the dirty task063 source.
The [exact command, time and bounds](evidence/assessment-20261115/scorecard/FINAL-RUN.json)
and [raw result](evidence/assessment-20261115/outputs/scorecard.json) are retained.

Selected findings: Branch-Protection 0, Code-Review 0, CI-Tests 8,
Pinned-Dependencies 8, Token-Permissions 10, Security-Policy 4; Packaging -1
because no publishing workflow was recognized. A score of 10 is a tool heuristic,
not proof there are no workflow write grants: local scan uses security-events
write and preparation uses OIDC/attestation write, as independently recorded.

Vulnerabilities reports **48 OSV identifiers for the remote repository**. The
tool output lacks path attribution, and the repository contains historical
evaluation inputs and fixture locks. Their contribution is possible, not proven
by this output. Do not call these 48 current shipped-product vulnerabilities.
The remote Cargo.lock differs from task063's remediated local lock. No new
remediation, finding suppression, waiver or automatic task creation occurred.
Signed-Releases 0 is preserved even though public provenance assets exist;
tool recognition and actual signature/provenance verification are separate.

Read-only `gh api --method GET ...` observed develop/main as unprotected (404
with explicit Branch not protected, matching branch metadata), no effective
rules/rulesets, read-only default workflow permissions and private reporting
disabled. The connector's protection GET returned 403, correctly retained as
Unknown for that transport. Existing CLI access supplied the separate observation.
Full commands, exits and response bodies are in
[the CLI ledger](evidence/assessment-20261115/github-cli/COMMANDS.json) and its
numbered follow-ups. Existing credential scopes were broader than necessary;
no credential was logged or new scope granted.

The first unauthenticated Scorecard attempt returned exit 0 but no JSON due to
native TLS initialization restrictions. The first authenticated attempt exited
1 after the archive file cap and ancestor .gitignore read restrictions; its 2.9
is invalid measurement evidence and is not used. Both failures, timeout
diagnostics, the source-byte size check and the successful narrower environment
are explained in [attempts](evidence/assessment-20261115/scorecard/ATTEMPTS.md).
TLS verification was never disabled. A native filesystem probe passed four
allow/deny controls; its first shell-reexec failure remains separately retained.

## Verification and reviews

One-off assessment validation checks the independent official criterion set,
required fields and artifact mirrors, missing/duplicate rejection, conditional
N/A permissions, actual tool/run identities, empty-JSON/failed-run rejection,
negative-score handling, API error distinctions, empty report populations,
remote/local source separation, source/tool hashes and baseline preservation.
The initial 15 checks passed; final-source results and command exits are recorded
in [validation](evidence/assessment-20261115/VALIDATION-r2.json).
Final artifact/source hashes and the diff/link checks are recorded alongside it.

No Rust, Cargo, rules, product tests, reusable script, executable example, CI or
packaging behavior changed in task052. Rust checks were therefore not rerun.
No review subagent was started: current AGENTS.md/REVIEWS.md normally exempt
research/prose-only work; this is not a substitute for implementation/test review
of future executable changes. Earlier reviewer identities were not treated as
having reviewed task052. Raw authoring/validation controller text is retained as
measurement evidence, not installed as executable product tooling.

## Blocker and concrete next action

The maintainer was asked for factual ownership/backup, private-report/response
history and MFA enforcement/method information. The reply asked whether that was
necessary for evaluation; the distinction was explained: it is unnecessary for
Scorecard execution, but unavailable operational facts cannot be labelled Met.
No affirmative facts were inferred from that reply.

To resolve the outstanding truth, provide existing dated evidence or a factual
maintainer statement for the relevant Unknown items, including primary-developer
knowledge and complete response/review/fix populations where applicable. Some
items need specifically scoped technical evidence (e.g. current statement
coverage or independently rebuilt binaries); existing line/archive results do
not replace it. This record does not authorize unrelated implementation or
silently narrow the original criteria. No secrets or incident details need be
posted in chat. If these facts cannot be established, keep 052 unfinished;
changing acceptance criteria requires explicit maintainer agreement.

Network distribution's existing publication hold and unsent upstream draft
remain unchanged. No settings mutation, external message, badge application,
hosted Actions dispatch, publication, release, signing, tag, commit, push, PR,
merge or work on tasks053 and later occurred.

## Authorized reassessment —2026-11-16

The maintainer explicitly authorized the bounded resumption proposed in054.
The original dated assessment above remains unchanged historical evidence.
See [the new reassessment](TASK-052-REASSESSMENT-20261116.md) and
[evidence](evidence/reassessment-20261116/BASELINE.json) for new confirmed facts,
current source comparisons, fresh fixed-toolScorecard and remaining criteria.
This pointer does not declare052complete or revise the original measurements.
