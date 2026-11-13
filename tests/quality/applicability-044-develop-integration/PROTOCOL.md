# Source-based applicability correction

Review the six candidate units against their original bytes, conversion provenance,
the bound public support contract and source. Original nominal object truth and
all v2 measurements are immutable. This is post-observation applicability review,
not a new blind ground truth or a new holdout. State historicalMeasurementsSeen
truthfully; derive scope from the contract, not whether a prediction succeeded.
Assess each candidate as retained or unsupported, with a source reason. Confirm
all remaining units retain their original scope and that legacy filename signals
remain distinct from certificate extraction. Report any other scope conflict.

Own review JSON: reviewer, timestampUtc, corpusSha256, truthSha256,
protocolSha256, contractBindings, sourceBased=true, postObservation=true,
historicalMeasurementsSeen, otherApplicabilityReviewSeen=false,
remainingUnitsReviewed=true, unresolved=[], dispositions=[{id,status,reason}].
Two independently authored matching decisions are bound before computing the
separate supported-scope result. Original counts stay visible; no comparison
improvement may be attributed to the binary. Do not edit any frozen v2 record.
