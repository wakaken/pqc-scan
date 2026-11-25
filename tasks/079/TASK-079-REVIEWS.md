# Task079 review record

Reuse exactly `/root/implementation_review` and `/root/test_review` for the combined076-083 request; main owns all investigation/changes/tests/docs. Source versions include dirty files; no new agent budget is created.

## Implementation review1 — changes requested

Reviewed manifest09171708f3c411d4b05cbdd417f930db95326a0b9440683fcf157516eeee4750, all42 hashes matched. Reviewer built the target binary and ran narrow reproductions only, not full suites. Three P2 issues:

1. Type parameters/properties could shadow imported native types and still be classified as ML-KEM-768.
2. Scalar selector/parameter-container lookup skipped the mutation barriers used for key values; an invoked local function could alter the selected suite without invalidating768.
3. Legal namespace Alias::Type syntax was rejected alongside malformed colon paths.

Raw reproduction inputs/outputs are preserved under evidence/review1-reproduction. Main added regression tests before fixes, including owning-scope positive controls in an unrelated class, method generic parameters, property names, scalar and container unknown mutation, value-name collision with namespace alias and invalid type-alias :: control. C# type frames and common binding checks correct the issues without touching old language semantics or stored formats. Namespace aliases and global qualifier whitespace normalize with explicit namespace-only qualification guards. All18 tests pass, including218 frozen cases; PHP11/180 regression and Clippy/fmt pass. Earlier red/intermediate logs remain historical. The first broad workspace run passed before review1 fixes and is not labeled final for changed source.

Implementation fix confirmation, final broad checks and independent test review remain pending. Original079 criteria remain unchanged; no080-083 completion is implied.

## Implementation acceptance

Reviewer accepted final932fc7094d0a6c388f9a7a9d5c864330d5d932612b05186a81e5f7ff8939f130, all42 source hashes matched, three P2 findings resolved and no new findings. Main completed final default/all-feature workspace, build/fmt/Clippy, real CLI/strict/package/compatibility and browser checks against this unchanged source; FINAL-CHECKS records commands/exits. Test review remains pending.

## Test review1 — changes requested

Reviewer `/root/test_review` verified all42 source and86 observation hashes for932fc709 and the frozen218 cases,18 supplemental tests, privacy/limits/position controls, quality, broad configurations, CLI/package/compatibility and browser evidence. One P2 blocked acceptance: pre-pipeline labels distinguished BC TLS/official/community wrappers, but final masking removed per-observation project identity. Shared hints alone could not establish which occurrence referenced which project. All other reviewed criteria had no findings.

## Project-identity correction and scoped implementation confirmation

Main reproduced the missing native marker in `csharp-project-identity-red.log` and added final pipeline/CBOM/native per-position label, marker, anchor and legacy-association checks plus private-argument controls for C# and PHP wrappers. C#19/PHP12 tests, including218/180 frozen cases, pass in `project-identity-regression.log`.

The fixed five-project enum carries no arbitrary scanned strings. A private transient draft field binds each finite reference to its exact original legacy key. The pipeline retains only the enum's constant display label after normal redaction; previews and all arbitrary metadata stay masked. Existing open-string `evidence.marker_kind` supplies finite library markers, using the unchanged anchor formula. No schema fields, required contracts, asset kinds, evaluator policy or runtime capability are introduced.

Implementation reviewer accepted manifestc73abf4448fe8172a544cd5ce4dabe3328d2d4c05b27ae0c46d2d5002ce12abb, all43 hashes matched; no new findings. Full suites were not independently repeated. Final post-fix checks and same-identity test confirmation remain required.

## Final test acceptance

The retained test reviewer accepted c73abf4448fe8172a544cd5ce4dabe3328d2d4c05b27ae0c46d2d5002ce12abb, verifying all43 source/92 observation hashes. The project-identity P2 is resolved in final pipeline/CLI/browser outputs; secret arguments/context remain masked. Post-fix C#19/PHP12, default/all-feature workspace, build/fmt/Clippy, CLI/package/compatibility/distribution and fresh EN/JA/offline browser evidence apply. No full suite was independently rerun. No original079 blocker remains;080-083 and milestone acceptance are excluded.
