# Product vision

Design date: 2026-11-09. Status: proposed implementation plan; no capabilities are implemented by this document.

## Intent

pqc-scan is a locally executable OSS scanner that discovers cryptographic usage through static analysis and explicitly requested runtime diagnostics. It reports evidence, uncertainty, quantum exposure and feasible migration options. It evolves the existing Rust product; a rewrite is not a goal.

The current baseline is commit `b3104f9a4193b1b21ea11ca060f06301de2e936b` on `codex/japanese-reports`, with a clean initial worktree. The five workspace crates, detectors, YAML rules, recommendation engine, report writers and bilingual presentation are reusable product assets. [Current evidence](../records/v0.2-roadmap/CURRENT-GAP-ANALYSIS.md) distinguishes implemented behavior from proposals.

## Product promises

- Static scanning works offline, without an LLM, account, telemetry or cloud service.
- A lexical match, an AST pattern, a parsed certificate and an authenticated negotiated session carry different evidential strength.
- No findings does not mean no cryptography, successful inspection, secure software or completed migration.
- Algorithm vulnerability, classical weaknesses, observation certainty, usage, impact, priority and policy decisions are separate claims.
- OSS retains full detection, local assessment, policy and export capabilities. Commercial products must not drive artificial accuracy limits.
- `scan`, `rules list`, `rules show`, documented options and machine output contracts remain available; current finding IDs use the maintainer-approved framed SHA-256 scheme.
- Normal repository scanning never initiates network connections. Probes require an explicit command and explicit targets.

## Users and decisions

Engineers need a source location and a concrete verification step. Security teams need a reproducible inventory, incomplete-inspection diagnostics and defensible assessment provenance. OSS maintainers need small changes with regression evidence. Future integration consumers need a versioned local result, not a mandatory server.

Recommendations identify candidate standards, implementation prerequisites and interoperability checks. An ML-KEM reference is not proof of TLS hybrid negotiation. A PQC dependency is not proof that a feature is enabled. A certificate OID is not verification of its signature or trust chain. No automated cryptographic source replacement is planned.

## Priorities and exclusions

Quality work comes first: inspection completeness, certificate/public-key semantics, precise algorithm classification, safe evidence handling and a labeled evaluation baseline. Add inventory and observation models next, then bounded TLS and SSH diagnostics. Extend standard exports, diffing and existing English/Japanese reports after those foundations.

No dynamic plugin loader, custom ABI, database, web service, distributed jobs, mandatory LLM, automatic discovery of networks, authentication attempts or enterprise control plane is in scope. Do not add abstractions solely for a possible future platform.

## Evolution categories

Every roadmap item and task uses one or more of these categories:

| Category | Meaning | Typical example |
|---|---|---|
| Existing implementation correction | Repair demonstrated incorrect behavior | Certificate modulus size; SARIF unknown locations |
| Existing implementation extension | Extend a retained component | Diagnostic outcome API; structured observations |
| New feature addition | Add an explicit user capability | TLS probe; opt-in CycloneDX export |
| Existing implementation replacement | Remove and substitute an existing subsystem | None currently proposed |

Local helper changes are corrections, not a license to replace a detector framework. If extension proves insufficient, document the failing invariant, compare a retained-architecture alternative, quantify compatibility effects and request human approval before implementation.

## Success criteria

First establish measured precision/recall by detection family, inspection coverage, reproducibility, runtime/memory and export conformance. Then agree improvement budgets using that baseline; rule counts and successful test counts are not accuracy metrics. [Quality gates](../quality/QUALITY-GATES.md) define evidence requirements.

The default delivery unit is one task with its implementation, tests and documentation in one PR; a necessary prerequisite feasibility study can remain a separate task.

This design records recommendations, not human approval of future schema, rule-semantic or high-risk architectural changes. See the [roadmap](../roadmap/v0.2/ROADMAP.md) for task scope and decision gates, the [domain model](../architecture/DOMAIN-MODEL.md) for evidence semantics and the [boundary](OSS-ENTERPRISE-BOUNDARY.md) for platform responsibilities.
