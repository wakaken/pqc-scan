# Arm-blind human scoring package

Only this folder should be delivered to the independent human scorer. Do not
open the parent directory, operator mapping, raw trial receipts or this parent
chat before scoring. Report IDs are shuffled and omit arm/repeat/thread metadata.
Natural wording or source comments may still reveal conditions; record suspected
unblinding. The preparer knows truth and cannot supply independent blind scores.

Each report has a reference record with the nominated contract, source-based
label and exact source excerpts. A negative label is scoped to that root cause,
not a globally defect-free file. Fill scorecards.json, retaining denominators,
duplicates, novel verified/unresolved claims and severity disagreements. Merge
line-zero and column-zero SARIF claims as the nominated shared root cause; do not
count a split into two findings as two independent defects found. Check Markdown
severity rather than assuming the prepared P1 label is independently validated.

Freeze scores and their digest before receiving the separate arm mapping. These
public historical pairs and repeated observations do not establish population
accuracy. Human scoring and definitive adoption evaluation remain pending.
