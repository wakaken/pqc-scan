# Judgment validation protocol (task 056)

The frozen oracle is independent of matcher labels. Its author consulted official
normative/public local-policy sources and inspected implementation for coverage;
this is not a blind or independent-author dataset. Oracle revision 1 used an
undefined certainty label and exact decimal equality for a public f32. Revision 2
corrected those against the published schema and numeric transport before any
product observations; both freeze receipts are retained under task056.

`oracle.json` contains 1,345 synthetic cases. 885 have explicit expected judgment
checks; 460 are characterization only (320 exact legacy confidence-threshold
cases and 140 legacy recommendation cases). Characterization is Not Tested for
correctness, never a passing denominator. Correlated cross-product rows are not
independent statistical samples; no population confidence interval is claimed.

The Rust integration adapter calls public existing interfaces and emits actual
values without reading expected answers. Its test success means completion of
the adapter only. `judgment.compare` separately compares every ID and assertion,
rejects missing/duplicate/extra observations and distinguishes a missing property
from JSON null. Only declared f32 transport checks permit tolerance (1e-7), never
algorithm/policy enums or confidence-as-probability inference. A valid comparison
can find product mismatches; that does not become an infrastructure error.

All identifiers, certificate times and metadata are synthetic or public facts.
No private keys, credentials or remote diagnostic targets are used. Input case
code is never executed. The probe does not implement cryptographic operations or
change policies, risk thresholds, rules, public schema, support or exit behavior.

The nine task056 outputs, raw observations, source hashes, actual commands and
failed/successful confinement attempts live under task056. Required outputs are
also copied to target/quality-task-056. Completeness against the original nine
criteria requires more than these finite agreement counts: missing independent
truth, recommendation/feature/provenance coverage, errata applicability and
required review remain explicit. Do not mark task056 done merely because a
measurement adapter, workspace suite or comparison command exits zero.

## Authorized continuation and separate follow-up tables

On 2026-11-15 the maintainer requested retaining 0.45 while finishing task056;
this is a compatibility requirement, not evidence of calibration or optimality.
`supplement.json` freezes source-based follow-up expectations. Its 460
`reassesses` cases use byte-equivalent JSON input to replace the initial 320
threshold and 140 routing characterization rows in the canonical denominator.
`judgment.canonical_cases` rejects changed inputs, duplicate replacements and
attempts to replace already-assessed rows. The original table/results remain
historical artifacts and are never relabeled as a blind trial.

Supplement version2 adds each individually missing/false runtime/provider/peer/
encoding prerequisite and the explicit long-lived-confidentiality priority
boundary. Both supplement freezes precede their respective observation runs;
version1 bytes and earlier comparison results are retained. `pipeline.json`
contains sixteen real-CLI source/config/dependency fixtures; their code is only
scanned. Neither dependencies nor fixture programs are installed or executed.

Automatic agreement concerns the asserted properties, including legacy routing;
it does not establish that recommendation prose is correct. The main agent
separately reviews every unique complete action JSON payload by hash against
primary sources, retaining product findings. This is a disclosed post-observation
manual content review, not pre-observation independent-author truth. Counts for
manual semantic review, canonical synthetic judgments, raw characterization and
CLI facts remain separate. No confidence intervals or population claims apply.

Task evidence `outputs-r6/` contains the nine current deliverables; `outputs/`
retains the first Partial evaluation. The exact reproducible evaluation program,
commands, input/source/tool/binary hashes, freeze receipts, confinement attempts,
standards/errata dispositions and required review IDs accompany the task record.
