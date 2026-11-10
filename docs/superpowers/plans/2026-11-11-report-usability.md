# Human report usability correction

The user explicitly requested less technical HTML and clearer inspection coverage,
with delegated routine choices from the continuation. This is a presentation-only
follow-up to036/032, not a new storage/rule/network contract or Monitor task.

Chosen approach: remove per-observation and per-asset internal ID tables from
HTML; retain exact technical records in Markdown/JSON. Merely folding the old
hundreds of duplicate rows still inflates the document and does not explain them;
removing the underlying records would harm audit/automation and is not selected.

1. Reproduce saved sample sizes/rows; preserve old artifacts. Explain030/031/
   032/034/035 without claiming each changed static HTML. Actual browser access is
   blocked by URL policy; do not bypass it or label a mockup as a capture.
2. Add failing presentation regressions: coverage precedes finding filters,
   actual exclusions/size limits are readable, failures/unknowns stay distinct,
   HTML omits observation/asset ID repetition while Markdown retains exact binding.
3. Root owns static inspection/compact/CSS and narrow readonly scope accessors.
   Coverage starts with counts/status/reasons; diagnostics grouped by code with
   bounded example paths; advisory policy shows aggregate decisions and limitations.
4. Probe-report worker owns runtime HTML/test files: concise protocol-specific
   facts and next check, complete technical fields in closed native details.
5. Update both READMEs/report guides, regenerate same-input English/Japanese static
   samples and protocol rendering examples. Compare sizes and machine facts.
6. Run fmt/workspace tests/clippy, Node/quality Python, relevant optional-feature
   tests, independent review and exact source-bound self-scan refresh if needed.
   Record real-browser captures as unavailable, never passed; keep acceptance gates.
7. Commit the reviewed presentation change locally; no external publication.
