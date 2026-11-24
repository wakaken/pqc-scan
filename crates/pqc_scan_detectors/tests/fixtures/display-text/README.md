# Display-reference classification controls

`cases.json` has 29 labelled synthetic positions: 14 candidates, 14 human error
phrases in Java, Go, Python, Rust, Ruby, JavaScript and TypeScript (including UTF-8
column controls), and one Vue error-message array motivated by n8n. A label is
about the action-candidate/display distinction, not deployed cryptographic use.
Unknown calls are conservatively expected to remain candidates.

`oss-sites.json` adds four manually inspected, SHA-256-pinned positions from the
local Keycloak/n8n revisions used by the separate eight-OSS semantic oracle.
These are human error messages. No third-party source text is copied here.

Measure TP/FP/FN/TN at labelled positions (positive = candidate), require raw
references to remain present, and state the denominators. Report the synthetic
and actual-source subsets separately. Do not infer whole-repository precision,
recall, runtime use or safety from these selected controls. The existing 36
positive and 13 negative semantic controls test different assertions and must
remain a separate measurement.
