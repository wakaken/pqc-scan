# Scoped source review

Assess only whether untrusted source_snippet.lines[*].text remains an indented Markdown code block across LF, CRLF and CR. Input text may contain line separators; surrounding sanitization does not remove arbitrary markup. No browser execution claim is required. Other report fields, code examples and HTML writer behavior are outside this case.

The files are verbatim historical excerpts supplied as inert text, not a complete
crate or an executable example. Missing surrounding imports/types and the absence
of a buildable manifest are intentional. No patch is proposed against production.
Review the supplied snapshot within the stated contract and report no actionable
findings if none are demonstrated. No build or test result is supplied or claimed.
Use static reasoning and distinguish an unverified hypothesis from a defect.

Source location map (1-based original lines):

- source-1.rs.txt: crates/pqc_scan_report/src/compact.rs:99-103.
- source-2.rs.txt: crates/pqc_scan_report/src/compact.rs:629-646.
