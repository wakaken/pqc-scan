# v0.2 release candidate validation

Target: v0.2.0, branch codex/inspection-coverage-fixes, base0a281e9.
The maintainer retargeted the original 1.0-labelled roadmap to 0.2 without changing
approved scope or criteria. Static scanning is formal; probes remain experimental;
public binary scope is static macOS arm64.014/016 remain cancelled and023 done.
This file records candidate validation, not an already published Release.

## Scope and preservation

The combined candidate includes the existing inspection/semantic/A1–A3/local-context
repairs, original evidence and two-stage reviews, plus the current closeout:
correct clean-install rule assertions, exact self-scan review, product0.2.0 and
roadmap/user/security/release documentation, hosted static build-provenance
attestation with exact-source verification. Report schema versions remain unchanged.
Historical source/CI/measurements keep their original version labels and hashes.
The old roadmap filename is replaced by OSS-ROADMAP-v0.2.md; active links use it.

## Self-scan acceptance

V02-SELF-SCAN-REVIEW.json records every current raw result with exact location,
source/line/result hashes, disposition and reason. Prior reviewed registrations
are retained. Source changed/unreviewed results or excess duplicates stay
visible; CI never learns new exceptions. Dependency/capability inventory stays
in the upload copy. Raw SARIF is preserved. No live GitHub alert dismissal,
scanner severity, detection threshold or blanket source exclusion is performed.
The new source-bound regression checks raw immutability, exact partition,
retained inventory and unknown/changed/excess negative controls.

## Clean installation and provenance

The installed positive fixture correctly produces TLS_RSA_KEY_EXCHANGE,
TLS_RSA_CIPHER_SUITE and TS_TLS_RSA_CIPHER_LITERAL at example.py:1. The plain.py
negative must remain clean. Missing/duplicate/extra rules, wrong location or
unread fixture files fail acceptance. All prepared variants retain exact command
surface checks. Static public preparation has no unresolved notice flag.
Experimental combined/all-probes preparation retains four objc-family reviews;
all-probes also retains the russh-util dirty-VCS review. These are not cleared.

Hosted preparation generates a GitHub static archive attestation, then verifies
repository, signer workflow and exact source commit. The action and its upstream
composite dependency are SHA-pinned. No personal signing credential is installed.
A local unsigned archive alone does not establish authenticity. No tagging or
GitHub Release upload is part of candidate merge.

## Performance criterion

Frozen corpus/truth, per-split/family TP/FP/FN nonregression and the same-environment
2x warm maximum investigation signal remain authoritative. Numeric CI thresholds
are not enabled. The pathological repeated-JWT input now retains 128 times more
findings due to the authorized 32→4096 per-rule cap expansion:128→16384 findings,
roughly 1 MB→122MB output in the pre-closeout controlled study. Its measured cost
(~0.82s/~330MB versus~0.024s/~17MB) triggers investigation. It is an explicit cost
of more retained observations, not a secretly revised threshold or a universal
performance guarantee. Ordinary small/medium/large measurements stayed below 2x.
Final candidate measurements and exact-source checks are recorded separately.

## Validation records

Execution results, both independent review stages and hosted links are added
after verification. Generated logs and artifacts live under target/v02-closeout;
committed summaries bind the relevant source, commands and result hashes.

## Executed local candidate checks — 2026-11-12

Pinned build/fmt/workspace475/all-features526(one existing daemon-dependent
ignore)/all-feature clippy succeeded. Node34, quality Python21, distribution11
and actual kernel-boundary11 tests passed. The confined all-feature suite ran60
executables with stable binary/source hashes, eight generated previews and real
OpenSSH cases; cleanup succeeded. The static0.2.0 archive repeated byte-identically
and clean-installed offline with the exact three positive rule IDs and clean
negative. Final self-scan partitions1357 raw into1317 reviewed expected and40
retained inventory observations; all raw content remains available.

The final native0.2 baseline preserved every frozen split/family TP/FP/FN group
and three-run semantic/diagnostic/order/score stability. Warm maximum times were
~0.039/0.138/0.474/0.812s for small/medium/large/pathological; pathological peak
RSS~332MB is the investigated retained-output cost, not a new universal budget.
The14 OSS ten-format study is retained at its original pre-retarget scanner
version; it is not relabelled as a0.2 rerun. Final0.2 schema/CLI and installation
checks are separate. Previously observed browser behavior remains bound to the
unchanged renderers; no new browser execution is claimed in this closeout.

The [machine evidence](V02-RELEASE-VALIDATION.json) binds actual source/commands
and generated logs. Separate implementation and final-test review records cover
the combined candidate. Hosted exact-commit CI and three clean-source prepared
variants with static provenance verification are required after push; their links
and reviewer follow-up are recorded in the PR. Local results do not assert that
these future hosted gates have already passed.

## Independent local reviews

One implementation reviewer approved the combined closeout with no required fixes; a different final-test reviewer approved local adequacy after source, log, projection, baseline, kernel and actual installation checks. See [implementation review](v02/IMPLEMENTATION-REVIEW.md) and [test review](v02/TEST-REVIEW.md). The same test reviewer checks hosted CI/distribution/attestation and merge CI in a scoped follow-up; actual run links are recorded in the PR and final report.

## Archive documentation follow-up

Final archive checks found that newly linked release notes needed bundling. The asset collector now includes release-note Markdown through existing no-link/root-bound validation; tests verify both language tar members. RED/GREEN, all11 distribution tests, repeated-identical static archives and actual offline install passed. Only packaging Python/tests and note links changed; Rust/rules/protocol logic is unchanged from full tests. The same two reviewer identities verify this follow-up. Hosted validation targets the subsequent committed candidate, not the superseded first push.
