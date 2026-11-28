# Task078 reviews

Exactly two retained identities for the combined076-083 request: implementation `/root/implementation_review`, test `/root/test_review`. Main owns all changes and verification. HEAD7e8cd914 plus source-bound dirty manifests; no new reviewer budget is created.

## Implementation review1 — changes requested

Reviewed manifest v1 SHA256 `9a1511da582fb55500d428de8401405bb251e98c847e6d8a1fc785e9a8aacc5e`. Reviewer reproduced four issues; raw inputs/outputs are retained in evidence/review1-reproduction (synthetic, no real keys).

- P1: Names/Values early truncation returned before sensitive argument registration, leaking a key literal through another detector's source context.
- P2: foreach/catch bindings, array writes and unknown method arguments kept older types.
- P2: required phpseclib/SLH import and class-constant references were omitted.
- P2: use-function alias of class_alias bypassed the raw-name dynamic-class guard.

Main fixes: add an ephemeral source-context protection flag at the outer PHP detector boundary independent of parsing/binding success; retain argument-range protection for overlapping evidence. Invalidate additional writes, explicit OpenSSL output arguments and opaque method calls; test63/64/65 same-name bindings and alias7/8/9. Preserve exact canonical/reference-only imports and ::class spans. Resolve function imports before poisoning dynamic class identities. No serialized/public required field change or crypto computation. Nine PHP tests including immutable180-case cohort and25 report tests pass; Clippy passes.

The first broad default and all-feature suites exposed missing Japanese rule copies (not a detector failure). Add all three copies and verify existing all-rule localization assertion. The unrelated source-order assertion in the new reference test incorrectly assumed native serialization order; replace it with exact original-span/algorithm assertions. Failed logs remain historical. Updated broad checks and required browser copy observation remain pending.

## Final decisions

Implementation fix confirmation: pending. Test review: pending. Task078 is unfinished until every original criterion and required final-source evidence/review is accepted.

## Implementation review2 — changes requested and main corrections

Reviewed85609c3a060cfa6fa0e88b3d9b2f836d8b6edbe1d762ad7c2ad3ae7207af138c. Four earlier cases except the broader P1 evidence path are resolved. Reviewer retained P1 for legacy evidence.match exposing RS256/TLS-shaped key data at binding truncation and added P2 for array spreads/computed keys overriding a literal RSA selector. Raw reproduction saved under evidence/review2-reproduction.

Main extends the ephemeral PHP protection to all matching evidence and raw metadata on selected PHP sources; source context/previews remain masked. A private, nonserialized typed-source bit preserves finite explicit API identities, while opaque token-derived algorithms are cleared on this source path. Occurrences, legacy IDs and association inputs are preserved; sanitized CBOM rows use the existing context sanitizer. No change applies to old-seven files, stored schemas, confidence/risk policy or saved inventory assessment. RS256 can still be a substring of a preserved public rule ID; the payload assertion checks evidence fields and exact algorithm-asset identifiers rather than deleting public IDs to satisfy a substring search.

OpenSSL options now reject dynamic/spread/computed entries rather than skip potentially overriding keys. All11 PHP tests (180 frozen cases plus supplements), six core privacy units, current quality companion and Clippy pass. EN descriptions remove implementation vocabulary, with snapshots/source hash synchronized. Broad pre-review2 runs passed but are preserved as historical and will be rerun against the final corrections. Existing Japanese-copy observation will be refreshed for final generated reports.

## Implementation acceptance and final documentation applicability

Implementation reviewer accepted0911e0ccc2f6d532b7f21343b6eee5052ac03478ba624b80f3587d23a76c3993 after review2 corrections, with no unresolved findings. A final four-document-only amendment updates the root EN/JA source support tables, aligns Japanese privacy copy with English and records PHP capture/privacy behavior in architecture. Product/tests/rules/report/package inputs are unchanged; no README/COVERAGE/architecture include-str reference exists in product code. Reviewer confirmed acceptance applies to current manifestc42ef837e89af5988c33353fb1dc40a882a08d975f380aafd18f0012a053bf1e, all hashes matched, no additional findings. Existing binary/browser/broad evidence remains applicable by this explicit limited reuse.

## Test review and final closure

Test reviewer `/root/test_review` accepted current manifestc42ef837e89af5988c33353fb1dc40a882a08d975f380aafd18f0012a053bf1e, with no required fixes or uncovered078 criteria. All36 source hashes and66 saved output hashes matched. It confirmed the180 frozen focal assertions,11 supplementary tests, source/privacy/binding/parameter controls,128-rule quality evidence,25 localized report tests, both final workspace configurations, CLI/package/compatibility and actual browser captures. The four-document-only evidence reuse was accepted. No independent full-suite rerun was needed.

Main reconciles all five original078 acceptance clauses against final source and closes078. Acceptance is scoped PHP implementation only;079-083, cross-language milestone closure, independent population accuracy and crypto runtime capability are not claimed.
