# Pinned SARIF schema

Vendored unchanged from the OASIS SARIF Technical Committee repository:

- Edition: SARIF 2.1.0, OASIS Standard plus Errata 01, JSON Schema draft-04.
- Published source: https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/schemas/sarif-schema-2.1.0.json
- Pinned repository source: https://github.com/oasis-tcs/sarif-spec/blob/1742a253eb654bf8a4ed7c60eaef7412b2c23553/sarif-2.1/schema/sarif-schema-2.1.0.json
- Retrieved 2026-11-10. Published and repository bytes have identical SHA-256:
  `c3b4bb2d6093897483348925aaa73af03b3e3f4bd4ca38cef26dcb4212a2682e`.
- Upstream repository license/IPR notice is preserved verbatim in `LICENSE.md`.
  This OASIS artifact is not relicensed under the application's MIT license.
- Specification: https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/sarif-v2.1.0-errata01-os-complete.html
  Sections 3.4 (artifact location), 3.14.27 (column units), 3.30 (regions).

Validation uses development-only `jsonschema = 0.29.1` (MIT; MSRV 1.71.1),
with default features disabled (no HTTP/file schema retrieval). All schema
references are local fragments. Transitive dependencies are fixed by Cargo.lock.
Runtime report generation has no validator or network dependency. Schema updates
require a deliberate edition/provenance/digest review.

After initial `cargo fetch --locked`, run without schema/network downloads:

```sh
cargo test --locked --offline -p pqc_scan_report --test sarif_contract
cargo test --locked --offline -p pqc_scan_cli --test sarif_locations
```

The writer test asserts the exact schema digest, enables format validation,
validates real output (including empty runs and unknown regions), and rejects a
negative control with zero positions. Literal URI expectations and actual CLI
Unicode/AST/SBOM fixtures independently test semantics that schema cannot prove.
