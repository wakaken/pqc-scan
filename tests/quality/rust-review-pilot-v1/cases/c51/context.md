# Scoped source review

Assess only native unknown coordinates projected to SARIF: line 0 means unknown and must omit region; known line with column 0 must omit startColumn, retaining startLine. Known positive regex coordinates remain unchanged. Source snippets may be absent. Do not assess URI encoding, AST Unicode column policy, unrelated properties or schema retrieval.

The files are verbatim historical excerpts supplied as inert text, not a complete
crate or an executable example. Missing surrounding imports/types and the absence
of a buildable manifest are intentional. No patch is proposed against production.
Review the supplied snapshot within the stated contract and report no actionable
findings if none are demonstrated. No build or test result is supplied or claimed.
Use static reasoning and distinguish an unverified hypothesis from a defect.

Source location map (1-based original lines):

- source-1.rs.txt: crates/pqc_scan_report/src/sarif_report.rs:51-95.
- source-2.rs.txt: crates/pqc_scan_report/src/sarif_report.rs:145-187.
- source-3.rs.txt: crates/pqc_scan_report/src/sarif_report.rs:200-221.
- source-4.rs.txt: crates/pqc_scan_report/src/sarif_report.rs:225-256.
