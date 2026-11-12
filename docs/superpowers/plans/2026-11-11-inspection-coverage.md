# Inspection Coverage Implementation Plan

> Main agent implements all tasks. Exactly one implementation reviewer after implementation and one different test reviewer after final validation; neither delegates. Preserve the original dirty quality-assessment workspace and its approved decisions.

**Goal:** Repair the measured inspection gaps, deliberately exclude known non-scannable artifacts before inspection, and regenerate all ten formats for the fourteen pinned OSS checkouts stored outside the product.

**Architecture:** Keep report schemas, finding identity, rule definitions and cryptographic judgments unchanged. Use finite but repository-sized detector budgets and borrowed candidate source text instead of 2,000-character truncation. Recognize the specifically observed module syntax in a byte-position-preserving parser view, while matching and redaction always use original decoded source. Use one strict UTF decoder for detection and snippets. Known binary artifact exclusions are a declared scan policy, with an opt-out for forensic inspection; cryptographic containers remain candidates.

**Alternatives considered:** Raising every limit without reducing candidate copies leaves unnecessary memory growth; removing all budgets loses finite resource guarantees; hiding actual failures would misreport coverage. Use borrowed text, finite work budgets, regression tests and explicit incomplete status for genuine limits. Upgrade only if released upstream grammars support the observed syntax; otherwise narrowly adapt parse views, without suppressing unrelated syntax errors.

**Constraints:** No target OSS execution. No rules/recommendation or public storage schema changes. No claim of full semantic analysis or zero remaining unsupported inputs. Keep private content masked, preserve source positions and unchanged finding IDs. Do not modify or mark done the separately planned tasks 041–061. Preserve original reports as baseline.

## Tasks

- [ ] Baseline workspace tests and capture original dirty workspace hashes.
- [ ] Add failing regressions: hits after 2,000 characters and 32 occurrences, larger valid ASTs, type-only star exports, deferred namespace imports and negative syntax cases; valid YAML wildcard versus real aliases/tags; UTF-16/32 decoding/positions/masking and malformed/opaque negative cases; known binary exclusion/override and crypto-container retention.
- [ ] Repair detector candidate ownership/work limits, with configurable reduced limits for exact safety-boundary tests. Keep real limit exhaustion visible and preserve later-rule execution.
- [ ] Implement narrowly validated parser compatibility for observed module syntax and V8 intrinsics if evidence requires it, preserving byte offsets and original evidence.
- [ ] Correct YAML preflight token classification without enabling alias expansion or tags.
- [ ] Decode BOM-marked UTF-16/32 strictly; recognize unambiguous BOM-less wide text conservatively; use decoded source consistently for snippets and privacy. Invalid encoded input must not appear as completed text inspection.
- [ ] Exclude known media/archive/executable and compiled-data extensions before selected counts, with an explicit opt-out. Retain certificates/keys/key stores and unknown extensions. Keep API compatibility through additive entry points/options. Summarize exclusion details in human reports; retain exact machine diagnostics.
- [ ] Update both READMEs and coverage/scanning guides with supported syntax, limits, encodings and exclusion options.
- [ ] Start one implementation reviewer on the combined diff; fix their findings using the same reviewer for rechecks.
- [ ] Run fmt, workspace tests, clippy and appropriate CLI/privacy/schema/feature checks. Build the modified scanner in the confined scan environment, bind source/rules/binary to hashes, rescan the same fourteen OSS snapshots and validate all ten formats. Re-fetch source snapshots only if necessary; record actual SHAs and checkout omissions.
- [ ] Verify generated reports in a dedicated loopback browser, including desktop/mobile, keyboard and already-loaded offline behavior. Preserve the user's existing tab.
- [ ] Start one different test reviewer on final source, assertions and actual run evidence. Fix/retest as needed, reusing that reviewer. Deliver updated reports, per-product names, before/after coverage and residual limitations.

## Review focus

Source locations and privacy on decoded/masked parser views; comments/strings/invalid syntax accidentally accepted; alias/tag false negatives; binary-looking text and extension spoofing; exclusion accounting versus unknown populations; finite work budgets and preserved partial findings; evidence matching final code.
