# Task 056 recommendation remediation follow-up

Status:complete. All five authorized guidance repairs are implemented and verified.

## Authorization and scope

The maintainer requested explaining and correcting the five measured guidance
findings on2026-11-15. Keep task056's completed historical evaluation and its
original outputs unchanged. This is an authorized bounded repair follow-up,
not a new evaluation task or a reopening of its original acceptance criteria.
Task052 remains unfinished and task053 interrupted. No commit/publication is
requested. Reuse the same two review identities for this material follow-up.

## Approach and acceptance

Correct the existing templates and Japanese localization, preserving action IDs,
routing, schema,0.45 and all detector/cryptographic logic. Merely deleting the
examples would lose useful guidance; adding a cryptographic implementation would
exceed scope. Instead identify unknown Java versions honestly, show BC provider
selection with an explicitly versioned example, require newly provisioned
algorithm-compatible keys, qualify certificate construction/encoding support,
and distinguish static RSA exchange from RSA authentication in Envoy.

Acceptance:all five findings addressed in relevant English/Japanese output;
unknown/old/current Java cases remain distinct; existing action routing and
judgment properties regress cleanly; meaningful targeted tests, required Rust
checks, actual rendered-report/browser observations and both scoped review
confirmations match final source. Illustrative Java fragments are guidance,
not a newly supported SDK implementation or cryptographic certification.

## Steps and evidence

Preserve baseline and historical evaluation. Add regression tests reproducing
five findings, record expected failures, fix minimal templates/localized copy and
paired report guides. Run targeted/full/default/affected optional checks and
compare unchanged frozen judgment properties. Inspect combined diff and rendered
English/Japanese reports, obtain implementation then test review using existing
identities, preserve source/tool/binary hashes and record outcomes here.

Evidence: `evidence/remediation-20261115/`. Initial status:in progress; final decision is recorded below.

## Implemented changes and compatibility

| Finding | Resulting guidance |
|---|---|
| F056-01 | Missing/nonnumeric Java hints now say version unknown; detected23 still gets upgrade guidance and24 keeps standard JCA guidance |
| F056-02 | Example is explicitly scoped to BC Java1.85, BC provider and ML-DSA-65; resolved release/deployed JDK support must be checked |
| F056-03 | Explicit parameter selection and a new ML-DSA key pair are required; the fragment uses mlDsaPrivateKey rather than the old RSA key; verifiers must receive matching public keys/JOSE/PKI encodings |
| F056-04 | Select exact certificate construction/encoding and establish provider/CA/client/trust-store support before a nonproduction pilot; primitive FIPS standards do not supply a universal hybrid certificate format |
| F056-05 | Both Envoy actions distinguish static RSA exchange from ECDHE_RSA authentication; cipher_suites is scoped to TLS1.0–1.2 rather than1.3, with certificate/peer checks separate |

Japanese action copy and paired user guides were updated together. The39 final
unique action payloads include six changed payloads; one additional unique
payload separates unknown Java from an identified old version. Original38-payload
measurements and their five findings remain historical, not rewritten.

COMPATIBILITY.json compares all1,991 raw public-interface rows with the completed
evaluation:non-recommendation observations are exactly equal. Only the full
recommended_actions field differs in50 recommendation rows; action-ID routing,
risk, severity, confidence, evidence and locations remain identical. Detection
rules,0.45, public schemas and cryptographic judgment logic remain unchanged.
The Java fragments are illustrative code requiring already provisioned keys;
no Java SDK/library dependency or cryptographic implementation was added.

## Actual verification

Source:HEAD f07d782cd68cfac95d93585d5ace049f590160d4 plus source-bound uncommitted
changes. Native runs record full source/snapshot, clean environment/confinement,
commands, exits and test/CLI executable hashes. Compiler/tool bindings reuse the
same Rust1.93.1/Python3.12.7 installed binaries from the previous measurement,
with current equality checked in final evidence.

- Five core regression tests first failed for all five issues (red-r1,exit101).
  The real writer test first failed for unknown-version guidance
  (red-report-r1,exit101). These are expected negative-first evidence.
- native-r1:targeted core/report tests, default/allfeatures observations, workspace,
  strict default/allfeatures clippy and build all exit0. Workspace554 passed,
  0 failed,3 ignored. Original oracle inputs were not changed.
- A later rustfmt check exposed unstable formatting of a nested temporary-path
  expression in the new report test. It was split into local timestamp/name
  variables; product and fixture semantics did not change. Final formatting
  check exits0. FORMAT-CORRECTION.json retains the change reason.
- native-r2 reran the final report test, workspace554/0/3 and strict clippy, all
  exit0. RENDERED-PARITY.json confirms all eight generated EN/JA HTML/Markdown/
  JSON/SARIF files are byte-identical to the browser-tested native-r1 outputs.
- compare.py.txt exits0:1,531 canonical properties plus16 CLI facts agree in
  default and allfeatures; no mismatch or unassessed canonical property. Core
  observations are byte-identical across feature configurations. Passing routing
  properties are distinct from the manual content review of all changed payloads.
- Actual Chrome157.0.8103.51, dedicated loopback reports, fresh profile:English/
  Japanese desktop first screens and key examples inspected; native trusted Enter
  toggles details and search works offline after loading. Six screenshots retained
  under browser-r2. Initial incomplete CDP keyboard setup failed in browser-r1;
  explicit page focus/key text/native keycode and activation wait passed without
  a product change. Every attempt is preserved.

Device390x844 emulation produced effective layout width718 when all details were
expanded; this is recorded rather than claiming exact390px fit. No responsive
layout change was included. The report files remain readable in captured views;
layout behavior beyond this guidance repair is outside the five-item request.
The local server exposed only eight synthetic report files and was stopped with
the temporary browser/profile after testing. It did not touch a user's visible
tab. Actual loopback URLs and request boundaries are in browser-r2/RESULT.json.

## Independent reviews and final decision

- Implementation:`/root/task056_implementation_review`, same identity as original
  stage. Accepted the six-file combined repair with no actionable finding; source
  recorded in REVIEW-SOURCE-v1.json. Only the test temporary-name construction was
  subsequently simplified; all product/doc/expected bytes remained identical.
- Test:`/root/task056_test_review`, same identity as original stage. Accepted the
  final six-file repair with no actionable finding or material verification gap.
  The reviewer checked final hashes, all five negative-first regressions,203 final
  native source bindings, original judgment comparisons, all1,991 compatibility
  rows, eight equal report files, ten primary references and six browser images.
  No extra identity was created.
- Final decision:complete. All five authorized repair criteria, applicable checks,
  both review stages and final-source/preservation checks passed. The historical
  evaluation remains done against its recorded source; task052 remains unfinished
  and task053 interrupted. No other task or publication action was started.

Final hashes, command outcomes, source applicability and historical preservation
are bound in `evidence/remediation-20261115/EVIDENCE-FINAL.json`. Reproduce the
frozen-property comparison with `python3 tasks/056/evidence/remediation-20261115/compare.py.txt`.
Native controllers and browser controllers are preserved beside that manifest;
reproduction uses a new output directory to retain previous attempts.
