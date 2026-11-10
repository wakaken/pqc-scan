# Remove advisory assessment from ordinary HTML

Date: 2026-11-11. This user-requested correction supersedes the advisory-summary
choice in REPORT-USABILITY-VALIDATION.md. [Exact source/log/binary/sample bindings](HTML-ASSESSMENT-REMOVAL.json)
record the worktree at parent18735b9 before committing this correction.

The HTML no longer contains the advisory-policy heading, overall decision, asset
counts, profile identifier or reason codes. Its focus is inspection coverage,
findings/locations and checks/next steps. Explicitly requested policy JSON and
full Markdown remain intact; inventory/assessment mismatch rejection is preserved.
No cryptographic, protocol, rule, schema, identity or machine-writer logic changed.
The user requested understandable investigation results, rather than additional
internal evaluation fields; this is a presentation follow-up to036.

The existing regression now requires byte-identical HTML with/without a policy
for the same document in English/Japanese, retaining policy/reasons in Markdown
and identical legacy machine files. RED was reproduced before removal. Initial
workspace execution then caught an unused CSS class still embedded in the HTML;
the independent reviewer requested its removal. Final default workspace tests344,
Node28, quality Python20, release build/fmt/clippy all exit0. The reviewer accepted
removal scope, Markdown/validation retention and unchanged findings/actions.
All119 existing self-scan expectation source hashes remain verified; no policy
refresh or new exception was needed. The protocol matrix was not repeated for
this static HTML-only change; real-browser and hosted CI acceptance remain open.

The original55-file/294-finding English/Japanese samples were regenerated with
the explicit policy option still enabled, and copied to the currently viewed
`target/task-036/report-{ja,en}` paths. Assertions confirm the HTML contains no
policy section/profile while report.md and scan.assessment.v1.json retain it.
Japanese HTML is916,734 bytes; English795,850. The previous HTML files remain at
`target/report-assessment-removal/before-{ja,en}.html`. No push, publication,
remote message or repository setting change occurred.

Final independent evidence review accepted147 source bindings,51 executed binary
hashes, all final log/sample hashes, the119 unchanged self-scan source bindings
and the retained blocked browser gate. No blocking correction remained. Reviewed
machine-record SHA-256:
`db18b2c69abc6129471327e6a3502ae2469af7d0ad92c99cd9e1108f2a85de5d`.
