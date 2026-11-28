# OSS corpus evaluation workflow

Goal: evaluate 90 primary OSS projects and eight comparison projects, repair confirmed defects, and verify the integrated result without changing historical evidence.

## Pstack checklist

- [x] Read the Principles section of the poteto-mode skill.
- [x] Phase A: Frame
- [x] Phase B: Design the workflow
- [ ] Phase C: Run the loop
- [x] Phase D: Keep the audit trail
- [ ] Phase E: Verify and hand back

## Work units

1. Preserve source state and reconcile fixed-SHA historical acquisitions and v0.5 coverage.
2. Add the benchmark registry and resumable entry point over tests/quality, without duplicating scoring or truth.
3. Freeze protocol, input manifests, baseline binary and rules before product modifications.
4. Verify a small input, then the priority 27, then all 90 and the separate comparison eight.
5. Establish source-backed truth with exposure and independence recorded. Leave unsupported accuracy metrics null.
6. Reproduce confirmed detection and report defects, implement bounded compatible repairs, and add behavioral regressions.
7. Compare all machine outputs, run isolated repeated performance samples, and inspect representative reports in a real browser.
8. Run required verification and independent correctness/test-adequacy review on integrated source.
9. Publish a local Japanese result with all states and unmet predicates. Do not push, open a PR, merge or publish externally.

## Throughput checkpoint

Initial registry has 98 projects, nine groups and eleven individual languages. Reuse acquisitions only after URL/SHA/input verification. Execute source acquisition and scans under explicit limits. Keep performance runs serial and free from builds and concurrent scans. Reassess expansion after the small pilot and after priority 27; an incomplete or failed scan is never a completed evaluation.

## Completion predicate

Every required acquisition, source/input binding, baseline, truth scope, after-run comparison, report check, mandatory validation and independent review is supported by preserved evidence for final source. Historical 26 measurements and 64 observations are context, not a new baseline or independent truth. Integration of v0.5 support does not establish release readiness.

## Architecture exploration checklist

- [x] Ground
- [x] Sketch
- [x] Agree
- [x] Implement
- [ ] Scrap. Apply only when experiments invalidate the design.

## Design arena checklist

- [x] Frame
- [x] Fan out
- [ ] Cross-judge. The configured pool is Codex only, so no other-provider judge is available.
- [x] Pick
- [x] Graft
- [x] Verify

The design rubric rewards reuse of tests/quality, immutable source and input bindings, resumable non-overwriting runs, explicit partial and failed states, bounded resources, and a small maintainable interface. Data shape comes first. Registry records identify population/group/repository/pin/license/scope. Run records bind source/binary/rules/input/configuration and link to measurements and artifacts. A run receipt never certifies independent truth.

## Current checkpoint

Both 90-project primary and eight-project comparison phases executed successfully
before and after the product repairs. Current inspection is Complete13/Partial77
for primary and Complete1/Partial7 for comparison. Four historical assessment rows
are corrected by source-version-aware reassessment without rewriting receipts.
Both original full output audits pass98/98. The user explicitly excluded only the
two quarantined Node/Discourse test fixtures. All98 projects remain registered;
the amended comparison8 pairs and16 output audits pass.

A further67 source intervals were frozen before fresh16-project paired scans.
All32 executions and output audits pass. An all67 anchor-contract audit found
two originally eligible annotation defects, not two missing supported APIs.
Original truth and scores remain immutable. A separate validity supplement keeps
the source facts and sets affected official metrics to null. Its independent
assessment and the integrated admission guard are approved. No corrected posthoc scope
is treated as a new independent holdout or as population accuracy.

The final accessible product binary is source-bound in
`evidence/final-accessible-build-receipt.json`. Default987, all-features1049,
Node30 and Python262 tests pass, with existing ignored/skipped cases retained.
The only product difference from the preceding integrated build is compact.js.
The dialog endpoint fix has independent implementation/test approval and actual
Chrome observations on final CLI outputs. All98 HTML derivatives replace only
that exact asset, preserve all other bytes, and are explicitly not full CLI
rerenders. Other exports remain bound to immutable original outputs.

The task remains unfinished. Controlled representative performance measurements
still require drained owned work and observed natural quiet conditions on this
Mac. Historical110 samples and excluded CPU-reader diagnostics do not satisfy
that predicate. The fresh Darwin reader has independent approval; actual quiet
admission and five accepted before/after pairs remain outstanding. Final scope, admission and the current report are independently assessed.
The performance result and final different-family trail assessment must be
completed before handback. Do not promote this checkpoint to v0.5 release
readiness or change historical task states.

The first drained-work controlled attempt exhausted900 seconds of quiet-window
admission. No scanner measurement started and zero of five pairs were accepted.
The approximately915-second observer record averages3.959 logical cores, with
nominal thermal state, normal memory pressure and zero swap-counter change.
The task remains unfinished pending a quiet Mac and a fresh bounded attempt.
See `evidence/completion-index.json` for the additive current evidence pointers.

## User-directed deferral, 2026-11-26

The user deferred new performance measurements because a quiet Mac is difficult
to provide. Do not start load monitoring or new measurement attempts automatically;
wait for a subsequent instruction to resume. Keep the original measurement
predicate unmet and preserve all prior attempts. Other mandatory work in the
existing evaluation/repair scope is complete. Null population accuracy expresses
the qualified truth scope, not zero detection quality or an unfilled numeric field.
Documented unsupported APIs and partial inspection remain product limitations.

## Accuracy evaluation reopened, 2026-11-26

The user correctly challenged the preceding completion wording. Completing the
limited67-interval annotation did not establish sufficient accuracy coverage for
the original corpus objective. That completion statement is superseded. Expand
qualified truth and executed evaluation, and quantify deduplicated product
improvement candidates with affected-project counts. Performance measurements
and load monitoring remain deferred. The current expansion workflow is saved
externally under the run directory named in evidence/completion-index.json.
