# Pinned CycloneDX 1.7 schemas

Upstream: [CycloneDX specification](https://github.com/CycloneDX/specification),
tag `1.7`, commit `4b3f59453366e27c8073fd24e98bf21ef8892c8e`.
`SOURCES.json` records exact raw URLs and SHA-256 digests. Files are unmodified.
The official schema and example are Apache-2.0; see `LICENSE` and upstream schema
copyright/license comments. The SPDX schema embeds the upstream SPDX attribution.

`bom-1.7.schema.json` references the local SPDX, JSF and cryptography definitions.
Tests register these exact resources in memory using their official base URIs;
the jsonschema dependency has network retrieval features disabled. No schema
network fetch is performed during export or offline validation.

The public certificate example is a pinned upstream compatibility fixture, not an
assertion that pqc-scan extracts every field shown there. Exporter mappings and
omissions are documented in `docs/quality/TASK-034-CYCLONEDX.md`.
