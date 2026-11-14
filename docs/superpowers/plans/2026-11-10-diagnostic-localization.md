# Existing human-report diagnostics and assessments

> **For agentic workers:** execute task 036 with meaningful failing tests first,
> independent review and the repository role lifecycle.

**Goal:** Render existing scope-limited diagnostics and advisory assessments in
English and Japanese HTML/Markdown without changing machine contracts.

**Architecture:** Add a document-aware report entrypoint, retaining old writers.
Pass optional context through the common compact renderer and render escaped
sections outside occurrence/action groups. Core exposes only read-only getters.

**Tech Stack:** Existing Rust 1.93.1 workspace, static HTML/Markdown catalogs.

**Spec:** tasks/036/task.yaml and
docs/records/v0.2-roadmap/CONTINUATION-DECISIONS.md, section 036. The continuation request
authorizes this existing-report extension; no network or new storage format.

## Global constraints

- Preserve six legacy machine filenames, JSON schemas, IDs and scan exits.
- Translate human copy only. Preserve facts, code, paths, IDs and reason codes.
- Keep assurance separate from usage, runtime evidence and certificate trust.
- No runtime sections before probes; no translation API or new framework.
- Keep full-payload action sharing and old writer entrypoints unchanged.

## Review focus

- Unknown/empty/incomplete scope never becomes an assurance of safety.
- Dynamic filenames and future vocabulary must be escaped in both renderers.
- Policy results must belong to the displayed inventory and keep exact reasons.
- Every occurrence/action remains accessible with and without JavaScript.
- Language changes must not alter machine artifacts.

## Task 036: document-aware human rendering

**Files:** core policy/observation/inventory read-only getters; report lib,
compact and new inspection module; CLI main; report integration tests; both
READMEs and report guides; task YAML and quality record.

**Interfaces:** `write_document_reports(document: &ScanDocument,
assessment: Option<&AssessmentReport>, diagnostics: bool, out_dir: &Path,
format: ReportFormat, language: ReportLanguage) -> Result<Vec<PathBuf>>`.
Legacy writers call the shared implementation without context. The CLI passes
its already evaluated optional assessment and existing diagnostics selection.

- [ ] Add integration tests using a real ScanDocument plus fixture detectors:
  complete/partial/failed/unknown/empty, optional diagnostics/assessment,
  localized status and counts, escaped path, immutable machine outputs.
- [ ] Run targeted tests and record the missing API/assertion failure.
- [ ] Implement read-only getters and escaped common human sections. Always
  display inspection status/counters in CLI human reports; detailed reasons only
  with --diagnostics; assessments only when selected. Preserve raw state codes.
- [ ] Run targeted tests, existing compact/Japanese and CLI tests.
- [ ] Update bilingual entrypoint/guide text, run fmt/build/workspace tests/clippy,
  Node/Python and relevant offline schema checks.
- [ ] Perform desktop/mobile/keyboard/offline browser checks, independent review,
  source-bound self-scan refresh and durable evidence. Evaluator alone sets done
  only if every required check is available and passes; then local task commit.
