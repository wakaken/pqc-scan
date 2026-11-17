# Scoped source review

Assess whether distinct tuples of rule ID, path, line, column and evidence are encoded without ambiguous field boundaries. Treat the supplied path as already repository-relative. Do not grade ID prefix/length changes or normalize_findings outside these excerpts. Compatibility changes require a separate explicit decision; these are historical snapshots, not a request to change current IDs.

The files are verbatim historical excerpts supplied as inert text, not a complete
crate or an executable example. Missing surrounding imports/types and the absence
of a buildable manifest are intentional. No patch is proposed against production.
Review the supplied snapshot within the stated contract and report no actionable
findings if none are demonstrated. No build or test result is supplied or claimed.
Use static reasoning and distinguish an unverified hypothesis from a defect.

Source location map (1-based original lines):

- source-1.rs.txt: crates/pqc_scan_core/src/pipeline.rs:425-442.
