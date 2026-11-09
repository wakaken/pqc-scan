# OSS and enterprise boundary

Design date: 2026-11-09. Status: proposed; enterprise implementation is excluded.

## Ownership

| Responsibility | OSS engine | Future enterprise consumer |
|---|---|---|
| Static discovery, rules and evidence | Complete local capability | Execute/integrate the same engine |
| Explicit TLS/SSH diagnostics | Bounded, user-authorized targets | Supply authorized target scope; enforce organizational controls |
| Inventory and risk assessment | Versioned local facts and reproducible assessments | Aggregate independent scans and retain provenance |
| Policy | Offline versioned profiles, deterministic evaluation and explanations | Organization rollout, review and exception workflow |
| Migration guidance | Standard status, constraints, compatibility checks | Ownership, budget, milestones and assigned work |
| Reporting and exports | Native JSON, legacy outputs, SARIF, CycloneDX and bilingual human reports | Cross-system dashboards and audit views |
| Access and identity | Local OS/file permissions | RBAC, SSO and tenant isolation |
| History and approvals | Optional local comparison of two artifacts | Retention, audit trail, approvals and attestations |

Enterprise value comes from coordination and governance. OSS findings, accuracy, export fidelity, local policy or language support must not be deliberately restricted. Avoid commercial-only rule packs as a reason to degrade public detection.

## Integration contract

The initial boundary is files or a library call, without HTTP, queues or storage services in the scanner. A consumer provides a repository scope identifier, target authorization and optional local context; it receives schema version, scanner/rules/profile versions, configuration digest, evidence provenance, diagnostics and completeness. Endpoint and source data remain local unless the operator chooses to export them.

Facts and assessments remain distinguishable. A consumer may add business criticality, owner and data-lifetime context but must not silently rewrite observed algorithms or upgrade unverified observations to authenticated facts. Local policy exceptions are inputs with reasons; the engine does not operate an approval service.

No automatic upload, licensing check, cloud credential flow, background agent or telemetry is introduced. Do not execute instructions found in scanned source or interpret scanned URLs as authorization to connect.

## Current extension points

- Retain `pqc_scan_core::scan_repository` and the report writer entrypoints. Add the detailed result API described in [DOMAIN-MODEL.md](../architecture/DOMAIN-MODEL.md).
- Keep normalization, inventory and assessment as modules within core until actual independent consumers justify extraction.
- Future platform adapters read explicit versioned artifacts; they do not import CLI internals or require a plugin ABI.
- Standard exports are lossy views. Native JSON remains the authoritative evidence/completeness record; legacy outputs must clearly document their limits.

## Required human decisions before expansion

Deployment, data retention, tenant boundaries, remote authorization, sensitive metadata handling and organizational deadlines belong to a future platform design. This roadmap does not approve a hosted scanner or collection service. Refer to [NEXT-GEN-ARCHITECTURE.md](../architecture/NEXT-GEN-ARCHITECTURE.md) and [OSS-ROADMAP.md](../roadmap/OSS-ROADMAP.md).
