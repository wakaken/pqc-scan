# Applicability revalidation after the approved F054-01 guide correction

This is a source-based, post-observation revalidation of the existing expanded-v2
public-support subset. The F054-01 fix added an exact Cargo RSA/JWT package-name
paragraph to the paired coverage guides. Do not infer a change to PKCS#12
support, change nominal truth, create new holdout labels, or silently rebind the
old review. Preserve every historical record and its expected hashes.

Compare the current English/Japanese coverage contract and complete current
PKCS#12 parser with the previous CI record at
`tests/quality/applicability-document-relocation-20261114/`. Establish the source
applicability of the previous bounded decisions from its existing original-byte,
conversion-provenance and parser records; do not repeat resolved investigations.
A changed support claim, parser predicate, corpus unit, truth label or unresolved
scope conflict prevents carrying the decision forward.

Independently revalidate candidate unit IDs x154, x156, v2-audit4, v2-audit5,
v2-audit6 and v2-audit7 from the current contract and the existing source-based
evidence. Distinguish outer container signals from extracted inner certificate
attributes. Confirm every other original unit retains its scope. Keep the raw
nominal result and the supported-input view separate. Do not execute/import any
corpus specimen or choose scope from the scanner's result.

The main agent freezes one current source-based record privately first. The
existing implementation-stage reviewer independently returns the second record
without seeing the main agent's current record or proposed current decisions.
Historical reviews/results and the earlier F054 assessment were already exposed;
disclose that fact. These are current source-scope revalidations, not new blind
labels or new accuracy evidence. The otherApplicabilityReviewSeen flag describes
access to the other current record at authorship, not access to old records.

Use the existing version-1 fields without changing the loader: reviewer,
timestampUtc, corpusSha256, truthSha256, protocolSha256, contractBindings,
sourceBased=true, postObservation=true, historicalMeasurementsSeen=true,
otherApplicabilityReviewSeen=false, remainingUnitsReviewed=true, unresolved,
and dispositions=[{id,status,reason}]. Bind current paired coverage documents,
PKCS#12 parser, unchanged expanded-v2 corpus/truth and this protocol. The main
agent transcribes the reviewer's record exactly and freezes the aggregate no
earlier than either authored review. Both required review stages still apply
to the combined CI selection/test change. Never weaken source/review/hash,
nominal scoring, numeric thresholds, or failure behavior to make CI pass.
