# Task081 retained reviews

Reuse only /root/implementation_review and /root/test_review across076-083. Main owns all investigation, planning, code, tests and docs. No reviewer delegation.

## Initial implementation review: changes requested

Retained implementation reviewer verified all62 hashes for30cd15dadf85faa83ae6d70c7510debeaf38e3ba69ff95fceaabf1b08a17cd61. Three P2 findings: conflicting explicit project Version/VersionOverride became missing and inherited a central value; trusted_central bypassed the10,000record cap, including values omitted by the central file's own Truncated inventory; new JSON's50k cap was applied after the existing500k decoder construction (main identified, reviewer confirmed). Narrow inputs/commands/exit0/outputs are preserved under evidence/review1-reproduction. No full suite independently repeated. No other clear privacy/compatibility issue found. Main adopts all three and must reproduce/fix/check them, obtain the same reviewer's scoped confirmation, then complete final-source checks and retained test review. Pre-review passing full suites/CLI/package are historical and not claimed as final-source acceptance.

## Scoped confirmation requested

Updated manifest includes63source hashes and four original reviewer inputs. Both central bugs have preserved failing tests,14current parser tests pass, construction limit unit passes and old default remains500k. Clippy/fmt/diff pass. XML line offsets use a finite newline index; safe NuGet identity canonicalization preserves original unique source-key lines. The retained reviewer must confirm the three fixes and material source delta before acceptance. Full suites/CLI/package from the previous source are historical only; final-source checks remain.

## Scoped implementation acceptance

Retained /root/implementation_review accepted8c2c3670531e82bf9d70faf5fcc6e363f7fb1c8c64b3393488209613b521f83f:63source and four reproduction hashes match. All threeP2 issues resolved, newline indexing/UTF8/UTF16 controls reviewed; no new findings. No full suite independently repeated. Final-source broad/CLI/package/browser and retained test stage remain required.

## Test review1 and test-only correction

Retained /root/test_review verified63source/fourinputs/271observations, all scope evidence and requested oneP2: UTF8/UTF16/XML/uppercaseJSON position controls and escaped duplicate case could pass with zero matching detections. Main now assertsComplete, exact1newNuGet match andline4/column1 for each encoding/JSON, and exact2PHPmatches before unknownline0/column0.14tests pass underdefault andworkspaceallfeatures; Clippy/fmt/diff pass. Only that integration test changed. Everyproduct/doc/fixture/binary hash remains8c-identical, so the prior implementation acceptance and final broad/CLI/package/compat/browser evidence retain applicability under the tests-only reuse rule. Updated manifest/observation hashes await the same testreviewer's scoped acceptance; no product change or new reviewer is needed.

## Final test acceptance

Retained /root/test_review accepted510e0e99bd9ea783db5b439b6179cadd64fc20619fd859e73f64087360438d79:63source/fourinputs/278observation hashes matched. Exact detection counts now prevent vacuous position/encoding success; default/allfeatures14tests and Clippy/fmt/diff pass. Prior product/CLI/package/compat/browser reuse is valid because only the integration test changed. No original081 test blocker remains.082-083 and whole-v0.5 acceptance are excluded.
