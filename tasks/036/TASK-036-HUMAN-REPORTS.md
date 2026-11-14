# Task 036: human diagnostics and local assessment rendering

Continuation baseline: `3c357cef879f69ae4058fefe7a03b4f074283b3f`.
Implementation and source review are complete; **task acceptance is blocked** on
real-browser checks. No runtime protocol section exists or is claimed.

The document-aware writer retains old entrypoints and machine outputs. The common
English/Japanese compact renderer adds scope-limited status/counters, optional
safe diagnostics, observation assurance and usage as separate facts, and selected
local advisory policy decisions with exact reason codes. Dynamic text is escaped;
custom-rule fallback, source facts and full-payload action sharing are preserved.

Independent review reproduced an initial public-writer defect: a policy generated
from different inventory content could be accepted when its run ID was unchanged.
A failing same-ID/different-facts regression was recorded, then corrected through
`AssessmentReport::validate_for_inventory`. The independent external harness
changed from `writer_accepted=true` to `false` for the same invalid pairing.
Additional review tests bind four assurance/usage combinations and per-asset
decisions/reasons to their exact HTML/Markdown rows in both languages. All six
targeted tests passed. Fixed table layout and wrapping address long identifiers.

The integration tree passed default workspace321 and optional-transport338 Rust
tests, Node28, Python20, build/release/fmt and both clippy configurations. See the
[integration evidence](../../docs/records/roadmap-continuation/ROADMAP-CONTINUATION-VALIDATION.md) for the exact source
snapshot and logs; these are local checks, not hosted CI or browser evidence.

English/Japanese examples were generated at
`target/task-036/report-{en,ja}/report.html`. Browser automation could not display
them: the in-app browser is unavailable, and Chrome's automatic URL review denies
the `file:` protocol, permitting only HTTP/HTTPS. No workaround was attempted.
The user received local file links; no manual desktop/mobile/keyboard/offline
acceptance has been received. Node's simulated DOM tests do not fill that gap.

Required next acceptance: inspect both languages at desktop and narrow widths;
check long-ID wrapping, keyboard disclosure/search/filter navigation and offline
operation. Keep `036` blocked until this evidence is available. Adding future
runtime sections must follow the actual probe contracts rather than invent them.
