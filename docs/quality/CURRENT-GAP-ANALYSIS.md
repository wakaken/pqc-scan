# Current implementation gap analysis

Reviewed on 2026-11-09 against `b3104f9a4193b1b21ea11ca060f06301de2e936b`, branch `codex/japanese-reports`. Initial `git status --short` was empty. No Rust, rules, existing reports or existing task statuses were changed for this analysis.

## Reading this historical snapshot

The findings below describe the 2026-11-09 design baseline, not the current unresolved backlog. Tasks 018–029 subsequently implemented the Phase 0/1 work, with task 023 still in `review` for maintainer protocol/budget approval. Current rule count is 119 rather than this snapshot's 117. Use the [implementation alignment](PHASE-0-1-ALIGNMENT.md), [roadmap checkpoint](../roadmap/OSS-ROADMAP.md#implementation-checkpoint-2026-11-10) and task-specific validation records to distinguish addressed gaps from remaining limits. Original source line references and source-only evidence claims are retained as historical provenance, not newly verified runtime findings.

## Evidence limits

**Source-confirmed** means a path was read, not executed. All reproductions below are instructions for future isolated tests, not results from this session. No external target was scanned. No build, workspace tests, benchmark, rule accuracy experiment, browser acceptance or current-HEAD remote CI was executed/verified here. Historical validation in tasks 015/017 is historical evidence only. Security-audit guidance requires isolation for executing reviewed target code; that complete isolation was not established in this session. Documentation validation is separate from product validation.

Priorities below describe correctness/product sequencing, not vulnerability severity or CVSS. No exploit or secret disclosure is claimed merely from incomplete defensive coverage.

## Reusable baseline

| Area | Source-confirmed capability | Important limit |
|---|---|---|
| Workspace | Five crates: CLI, core, rules, detectors, report (`Cargo.toml:1-9`) | No runtime probe modules |
| Detector composition | `Detector::detect -> Result<Vec<Detection>>` (`core/src/lib.rs:19-22`); five registered detectors (`detectors/src/lib.rs:13-21`) | Existing architecture prose omits the Result; code is authoritative |
| Languages | Java/Go/JS/TS/Python/Rust/Ruby paths in `tree_sitter_detector.rs:234-243` | Candidate AST node text is regex-matched; TS uses JS grammar |
| Rule loading | Enum/required-field deserialization, regex precompilation, duplicate-ID rejection (`rules/src/lib.rs:75-108,122-158,192-198`) | No accuracy/calibration claim follows |
| PQC signals | Seven-language fixtures, final/draft distinction, exact dependency/ecosystem checks, independent certificate signature/SPKI OIDs (`detectors/tests/pqc_readiness.rs:16-197`) | Static presence does not prove use/negotiation; synthetic OID certs are not verified PKI |
| Report presentation | Full occurrence/action retention and full-payload shared guidance (`report/src/grouping.rs:22-83`) | Keep this implementation; no new UI framework needed |
| Localization | English/Japanese HTML/Markdown, built-in catalog, machine-output parity (`report/src/lib.rs:45-108`, `report/tests/japanese.rs`) | Localization is already implemented; extend new diagnostic copy only |
| Safety controls | Masking plus HTML escaping and URL scheme checks (`report/src/lib.rs:111-154`, `compact.rs:344-382`) | Protection is heuristic and field-specific |
| CI | Rust 1.93.1 fmt/test/clippy/Node checks; Linux build, self-scan and artifact upload (`.github/workflows/pqc-scan.yml:8-62`) | Workflow presence is not current CI success |

Paths abbreviated as `core/`, `detectors/`, `rules/`, `report/` below mean `crates/pqc_scan_core/`, `crates/pqc_scan_detectors/`, `crates/pqc_scan_rules/`, `crates/pqc_scan_report/`.

Static YAML enumeration found 60 regex, 22 tree-sitter, 16 dependency, 16 certificate and 3 key rules (117 unique IDs). Categories are CryptoAPI 24, Middleware 19, TLS 18, JWT 16, PKI 14, PQC 13, SSH 10 and KeyMaterial 3. This demonstrates breadth of configured patterns, not exhaustive cryptographic coverage. Middleware is lexical configuration matching; manifests are dependency presence/resolution signals; certificate OIDs are parsed attributes; key detection is a three-header marker search. No checked-in runtime probe or general symbol/data-flow analysis is present. Rule-by-rule supported constructs, known negatives, source editions and error types must be measured in tasks 023/028 before claiming coverage or accuracy.

## G01: Errors become zero findings (first priority)

Source-confirmed: `core/src/pipeline.rs:37-45` converts failed reads to empty vectors and detector errors through `unwrap_or_default`. `core/src/walker.rs:35-55` counts queued files before reads/detection; its statistics feed `ScanSummary` at `pipeline.rs:65-70`. Malformed dependency JSON can also become empty output (`detectors/src/dependency_detector.rs:338-342,720-724`).

Consequence: no findings cannot distinguish successful inspection from a failed stage. Counts cannot substantiate inspection completeness. Ignored files never enumerated by the walker are not a measured skipped population.

Future verification: inject a read failure; provide one Detector returning `Err` and one returning a known hit; compare with a successful empty Detector. Malformed manifest/SBOM and a disappearing file must retain valid partial hits and a typed failure. Reconcile selected/read/analyzed/partial/failed/excluded counts; do not use OS permissions alone for portable tests.

Change: **existing implementation extension/correction**, task 018. Preserve `scanned_files` semantics in legacy output; expose accurate named counters separately.

## G02: Silent detection limits

Source-confirmed: regex takes 32 matches before comment filtering (`detectors/src/regex_detector.rs:30-33`); AST stops at 32 (`tree_sitter_detector.rs:75-81`); key markers stop at 8 (`key_detector.rs:48`). No truncation outcome is returned.

Future verification: 32 matching comments followed by one code occurrence; 40 valid occurrences for one rule; nine synthetic key markers. Check a bounded diagnostic and preserved partial findings. Count accepted hits separately from examined candidates; detect an additional eligible hit before asserting truncation, or conservatively state `limit_reached` with unknown omitted count.

Change: **existing implementation correction/extension**, task 019. Retain bounded scanning; do not remove safety limits to improve recall metrics.

## G03: Certificate key length and matching fields are incorrect

Source-confirmed: `detectors/src/certificate_detector.rs:52,67` calculates BIT STRING encoded bytes times eight. That is neither RSA modulus length nor EC curve size. `:119-133` combines filename, signature, OID, this size and expiry for legacy rules; `rules/default/cert.yml:23-54` matches bare numeric sizes without an RSA key-type condition. File names can supply matching numbers.

Future verification: reviewed public certificate fixtures for RSA 1024/2048/3072, EC P-256 and Ed25519; compare expected modulus/curve attributes, not DER lengths. Rename identical bytes to `rsa-1024.pem`; key assessment must remain invariant. Keep signature and subject-public-key algorithm separate.

Change: **existing implementation correction**, tasks 020/021. Reuse x509-parser/pem. Expose encoded length only with that exact meaning; unknown algorithms have unknown semantic key size.

## G04: Certificate classification and time heuristics

Source-confirmed: `cert.yml:67-76` contains `(?i)dsa`, matching `ecdsa-with-SHA256` produced at `certificate_detector.rs:182`. PQC-name avoidance at `:110-118` does not avoid ECDSA. ECDSA is `quantum-uncertain`, while DSA has only `non-quantum-risk` (`cert.yml:56-76`); the single `Risk` enum (`rules/src/lib.rs:49-56`) cannot express both quantum vulnerability and classical concerns. `CERT_EXPIRY_2026_OR_BEFORE` uses a fixed year regex over the combined string (`cert.yml:111-120`), not a current-time validity calculation.

Future verification: ECDSA must not emit DSA findings; DSA retains its own finding. Use fixed-clock expiry boundaries, not-yet-valid certificates and filenames containing 2026. Review a purpose/parameter-specific classification matrix against the [standards register](../product/STANDARDS-REGISTER.md).

Change: **existing implementation correction**, task 021; **extension**, task 026 for separated assessments. Existing rule IDs remain; changed classification results require explicit before/after review, not a silent compatibility claim.

## G05: Certificate container and identity limits

Source-confirmed: `.p12/.pfx` are candidates (`core/src/classifier.rs:34-43`), but only PEM for `.pem/.crt/.cer` and DER for `.der` are parsed (`certificate_detector.rs:34-71`). Parse failures fall back to filename/container signals (`:77-107`); these are not successful interior inspection. All PEM cert hits are at 1:1 without fingerprint or block index (`:138-166`), so same-rule/same-description certificates can collapse in normalization (`pipeline.rs:107-117`). No certificate signature or trust-chain verification is performed by this detector.

Future verification: identical DER bytes under `.der/.crt/.cer`; malformed PEM; unsupported PKCS#12; a bundle with two distinct same-algorithm public certificates. Require explicit parsing/unsupported outcomes and distinct certificate assets/observations. Do not use real private keys or add PKCS#12 password handling.

Change: **existing implementation correction/extension**, task 020 (parsing) and 025 (inventory identity).

## G06: AST semantics and occurrence retention

Source-confirmed: `tree_sitter_detector.rs:59-134,143-203` applies regexes to selected node text. It does not resolve imports, symbols, types or data flow. `.ts/.tsx` use JavaScript grammar (`:234-243`). The dedup key omits column (`:103-109`), merging same normalized matches on one line.

Future verification: real API vs same-named local function, alias imports, dead code, unrelated string, TS-only syntax, two calls on one line and nested candidate nodes. Measure error categories before selecting individual structural improvements.

Change: **existing implementation correction/extension**, tasks 023/024. Retain tree-sitter and regex; no general interprocedural-analysis rewrite is justified.

## G07: Confidence and severity are heuristics

Source-confirmed: rule confidence is copied to findings (`core/src/pipeline.rs:133-147`). `core/src/risk.rs:5-28` promotes sensitive categories with quantum-vulnerable risk and confidence at least 0.45. Private-key escalation sits inside those conditions, not an unconditional global path; built-in key rules already carry critical severity.

Future verification: a table of category/risk/confidence/evidence combinations and a held-out labeled corpus. Do not interpret 0.8 as an 80% measured probability. Keep severity behavior stable until its explicit policy migration is reviewed.

Change: **existing implementation extension**, tasks 023/026. No new numeric score is proposed without calibration evidence.

## G08: Finding identifiers are location-dependent and ambiguous

Source-confirmed: `core/src/pipeline.rs:120-145,156-172` hashes the detector path before relativization, line, column and masked match, with no field boundaries; digest output is 12 bytes. For identical rule/file/evidence, `(line=1,column=23)` and `(line=12,column=3)` both contribute `123`. This is identical constructed hash input, not a cryptographic collision attack.

Future verification: call the ID helper with those tuples; compare copies of one fixture under two checkout roots, an inserted line and a moved file. Define exact vs heuristic matches separately.

Change: **existing implementation extension**, tasks 025/035. Preserve old `finding_id`; add length-delimited, versioned IDs and diff correspondence rather than changing legacy hashes in place.

## G09: Current inventory is a finding view, not normalized assets

Source-confirmed: `core/src/pipeline.rs:184-211` derives CBOM `algorithm` from masked evidence text and `component` from the first path segment. `CbomEntry` lacks algorithm-vs-instance identity (`core/src/model.rs:99-107`). Inventory depends on rule findings, not independently observed assets.

Future verification: two uses of one primitive, one cert seen statically and over TLS, two certs with one public key, unknown algorithm and no-risk dependency. Normalize reusable algorithm definitions while preserving every usage observation and provenance.

Change: **existing implementation extension**, task 025. Keep current CBOM projection and report grouping.

## G10: Export conformance and unknown locations

Source-confirmed: `report/src/json_report.rs:13-21` serializes `cbom` and `dependency_sbom` as bare arrays. They lack a CycloneDX BOM envelope and must not be labeled CycloneDX 1.7 compliant. Unknown SBOM occurrence lines can be zero (`dependency_detector.rs:739`), and SARIF emits line/column directly (`report/src/sarif_report.rs:161-169`). Native `ScanResult` has no schema/version/diagnostics (`core/src/model.rs:119-135`).

Future verification: validate current inventory samples against the official object schema (expected structural rejection); feed line zero into the SARIF writer and validate against SARIF 2.1.0 (region positions are positive). The future writer omits unknown regions rather than inventing line 1.

Change: **existing implementation correction**, task 022; **new feature addition**, task 034 for separately named CycloneDX outputs. Shape incompatibility is established from source/schema inspection; no validator execution is claimed here.

## G11: Dependency constraints are represented as resolved versions

Source-confirmed: package.json parsing uses `sanitize_version` (`detectors/src/dependency_detector.rs:357-365,797-811`); `^1.2.3` becomes `1.2.3` and can enter a versioned purl, despite no resolution evidence.

Future verification: manifest-only, lock-resolved, workspace/git/path dependencies and conflicting manifests; preserve declared constraint and resolved version provenance separately. Do not manufacture a dependency graph from a flat inventory.

Change: **existing implementation correction/extension**, task 027; legacy projection retained, authoritative semantics in native inventory.

## G12: Rule validation and evidence safety need broader verification

Source-confirmed: confidence bounds/finite checks, empty-ID and scope semantics are not validated at `rules/src/lib.rs:75-108,122-158`; directory walk errors are filtered out at `:125-128`. Duplicate IDs and invalid regex already fail. `report/src/lib.rs:111-154` sanitizes finding matches/previews/snippets, not every metadata/action/inventory field. Private-key marker coverage is RSA/PKCS#8/OpenSSH (`key_detector.rs:15-19`, `pipeline.rs:394-409`); format coverage and heuristic base64 matching are limited.

Future verification: empty ID, out-of-range confidence, unreadable rule subtree, legacy/custom scopes; synthetic canaries in evidence metadata, snippets, inventory and all formats; synthetic EC/DSA/encrypted-key headers and inline secret-like values. Review actual data flow before claiming leakage. Never use or print real secrets.

Change: **existing implementation correction/extension**, tasks 028/029. Custom-scope rejection requires compatibility review; start with warnings where previously accepted semantics are unclear.

## G13: Quality/distribution evidence is incomplete

Source-confirmed: tracked source contains 117 built-in rule IDs and 76 Rust `#[test]` declarations, plus Node tests. Counts are static inventory, not tests passed, precision or recall. CI is Linux-focused; no tracked release workflow, benchmark/fuzz harness, formal native output schema or SECURITY.md was found in the reviewed tree. README documents `cargo install --path` and source checkout.

Unknown: external crates.io/GitHub release publication, signed binaries, actual current CI status, production accuracy, memory/performance and multi-platform behavior. No claim that an external release does not exist.

Future verification: labeled holdout corpus and reproducible performance runs; external release inventory; clean-install and rule-pack availability tests on declared platforms. See task 023 and tasks 037/038.

## G14: Task and documentation state need explicit preservation

Source-confirmed: task 014 is `pending`, task 016 is `review`, task 017 is `done`. This design neither resolves nor rewrites those states. Task 017 completing presentation work does not authorize auto-closing 016. New tasks start at 018 as `pending`; they extend, not recreate, completed detector/report tests.

The tracked architecture file is `docs/architecture.md`; this macOS checkout lists it as `ARCHITECTURE.md`. Use the tracked spelling in links and avoid a case rename. `docs/DECISIONS.md` contains a Quarkus example, not an actual pqc-scan backend decision. Restart pilot constraints are scoped to its baseline/task 014; this user-requested design and explicit task creation do not enable the automatic Monitor.

## Sequencing conclusion

Correct errors/limits, certificate semantics, SARIF locations and safety evidence before new protocol reach. Introduce additive core modules and compatibility projections, then isolated probes. Preserve working bilingual reports, rule precompilation, detector composition and tests. No finding here demonstrates a need to replace the workspace architecture.
