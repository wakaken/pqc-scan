# Task 034: CycloneDX 1.7 conversion evidence

Date: 2026-11-10. Implementation baseline:
`3c357cef879f69ae4058fefe7a03b4f074283b3f`. The shared continuation worktree also
contains tasks 030/035/036; this is task-local evidence, not their validation.
No push, hosted CI, publication or release claim is made.

## Contract and mapping

The maintainer approved separate opt-in outputs through `--cyclonedx-17` in the
continuation conversation; see [decision record](../../docs/records/v0.2-roadmap/CONTINUATION-DECISIONS.md).
The exporter consumes validated native inventory v1 through a local read-only
serialization adapter, without changing its schema or exposing mutable core fields.

| Native kind/fact | CycloneDX representation | Qualification |
|---|---|---|
| Algorithm | `cryptographic-asset`, `assetType=algorithm`, exact name and optional numeric OID | No inferred primitive, strength, parameter set or certification |
| Parsed public key | `related-crypto-material`, `type=public-key`, SPKI SHA-256 and optional bit size | Requires public fingerprint; never exports key bytes or infers private key availability |
| Parsed certificate | `certificate`, representable validity timestamps, typed key/signature references; full DER hash in `pqc-scan:certificate-der-sha256` property | Standard certificate `fingerprint` omitted: its definition excludes the signature, whereas inventory records full DER SHA-256 |
| Dependency source assertion | `library` component in the separate software BOM | Source presence is not installed/runtime use; no inferred dependency graph |
| Accepted lock resolution | `version` only for `lock_resolved` plus existing exact version validation | Declared constraints and unverified reported versions remain provenance properties |
| Usage/observation | Explicit method/assurance/usage/path/position properties on projected components | Static references never become runtime protocol objects |
| Unknown/future asset kind | Omitted with counted conversion loss | No raw unknown attributes are copied |

Asset IDs remain `bom-ref` values. Relations require one unambiguous target of
the expected kind in the same document; absent, ambiguous or wrong-kind links
are omitted and counted. Public objects without valid public fingerprints are
omitted and counted. Component order is sorted by exact ID. Private-key marker
observations do not create key components. Observations with no projected asset
are counted as unprojected, including marker-only cases.

Observation positions preserve `path`, `line`, `column` and `column_kind` together.
UTF-8 byte columns and Unicode-scalar columns retain their original values and
units; no position-unit conversion is inferred. The semantic golden retains the
unit alongside the numeric column.

Top-level `pqc-scan:export-loss` records counted omissions and fixed notes covering
the incomplete observation projection, unsupported fields, missing graph/runtime
proof, absent strength/certification/state inference, and nonprojected legacy
associations/assessments/diagnostics. The original inventory remains the richer
source. `native_completeness=not_exhaustive` is retained, including empty exports.

The existing native public-fact/identifier allowlist remains the input privacy
boundary. Arbitrary attributes, snippets, raw evidence and private data are never
copied; exported text also uses the existing recognized-syntax redactor. Structural
IDs, paths and package names retain the existing operator-sensitive metadata limit.

## Pinned standard and offline consumer validation

The [official CycloneDX 1.7 JSON reference](https://cyclonedx.org/docs/1.7/json/)
and official [schema source](https://github.com/CycloneDX/specification/blob/4b3f59453366e27c8073fd24e98bf21ef8892c8e/schema/bom-1.7.schema.json)
were consulted. Vendored source commit:
`4b3f59453366e27c8073fd24e98bf21ef8892c8e`, tag `1.7`.
[SOURCES.json](../../schemas/cyclonedx/1.7/SOURCES.json) pins all resource URLs and
SHA-256 digests, including the Apache-2.0 license and upstream public certificate
example. The original schema files are unmodified.

Offline tests use the existing pinned `jsonschema=0.29.1` consumer, with network
retrieval features disabled, registering the three external schemas from local
bytes under their official URIs. They validate generated crypto/software documents
and the pinned upstream example. This establishes the exercised schema compatibility;
no external product import or broad CycloneDX consumer interoperability is claimed.
Semantic tests independently check references, provenance, safe omissions and a
fixed algorithm-reference golden.

## Actual command outcomes

Logs: ignored `target/task-034/`, outside fixture roots.

| Command/check | Outcome |
|---|---|
| `cargo test -p pqc_scan_report --test cyclonedx_17` before implementation | Exit 101, expected missing exporter module (`red.log`) |
| Wrong-kind relation regression before correction | Exit 101, incorrect key reference reproduced (`relationship-red.log`) |
| Marker-only projection loss regression before correction | Exit 101, missing loss count reproduced (`marker-red.log`) |
| Missing public fingerprint regression before correction | Exit 101, fabricated empty public objects reproduced (`public-facts-red.log`) |
| `cargo test -p pqc_scan_report --test cyclonedx_17` after corrections | Exit 0, 10 tests passed (`green.log`) |
| `cargo clippy -p pqc_scan_report --test cyclonedx_17 -- -D warnings` | Exit 0 (`clippy.log`) |
| Owned-file `git diff --check` | Exit 0 |
| Independent-review regressions before corrections | Exit 101, three assertions failed: certificate fingerprint semantics, omitted position unit and updated unit-bearing golden (`review-red.log`) |
| `cargo test -p pqc_scan_report --test cyclonedx_17` after review corrections | Exit 0, 11 tests passed (`review-green.log`) |
| `cargo clippy -p pqc_scan_report --test cyclonedx_17 -- -D warnings` after review corrections | Exit 0 (`review-clippy.log`) |

The first attempted RED was interrupted by other in-progress core modules; it was
rerun and overwritten only after core compilation was available. The recorded
RED above specifically names the missing CycloneDX module. Shared-worktree compile
interruptions are not counted as meaningful failing tests.

Dedicated checks include official offline schemas/digests, deterministic order,
unique/resolving same-document refs, wrong-type/unavailable reference omission,
public certificate/key separation, declared/reported/resolved versions, absence of
private canaries, marker-only no-key behavior, exact two-file expansion and unchanged
legacy report bytes. Direct `write_cyclonedx_17` without captured inventory fails
before creating its output directory.

## Integration and remaining acceptance

Public exporter entrypoints:

```rust
pub fn documents(native: &pqc_scan_core::NativeScan)
    -> anyhow::Result<(serde_json::Value, serde_json::Value)>;
pub fn write_cyclonedx_17(document: &pqc_scan_core::ScanDocument,
    out_dir: &std::path::Path) -> anyhow::Result<Vec<std::path::PathBuf>>;
```

Controller owns module/public writer integration, CLI capture/flag behavior,
bilingual README/user guides, workspace build/fmt/test/clippy, independent review,
self-scan bindings and final source/commit acceptance. These checks must be
recorded after integration; this task-local record does not assert they passed.
The dedicated integration test initially included the exporter by file path to
exercise it before shared `lib.rs` integration. It now uses the integrated public
`pqc_scan_report::cyclonedx` module.

## Independent-review corrections

The official pinned 1.7 schema defines `certificateProperties.fingerprint` as a
certificate hash excluding its signature. The initial conversion incorrectly
placed the inventory's full-DER `certificate_sha256` there. A failing semantic
regression reproduced this mismatch. The standard field is now omitted, the
original whole-DER value is explicitly named in the component property, and
`certificate_tbs_fingerprint_unavailable` counts the lost standard representation.
No TBSCertificate bytes are collected or hashes recomputed by this exporter.

A second review correction preserves `column_kind` with each observation's column.
The regression exercises byte column 5 and scalar column 3 on the same Unicode
path, asserting exact value/unit pairing. Both regressions and the updated golden
passed together with all dedicated schema/privacy/provenance checks.

At worker handoff, task034 remained `review` pending those parent-owned checks.
The final evaluation is recorded in the parent integration acceptance below.
# Parent integration acceptance

Independent source review accepted the corrected full-DER hash property and
column-unit projection. CLI integration additionally proves default output stays
at six files, `--cyclonedx-17` adds exactly two files without implicitly saving
inventory JSON, and legacy machine content is unchanged after the documented
generation-time normalization. The CLI test passed; eleven exporter/schema tests
passed. English/Japanese README and user guides describe opt-in behavior and loss.
Evaluator accepted task034 with the [integration checks](../../docs/records/roadmap-continuation/ROADMAP-CONTINUATION-VALIDATION.md).
No package publication, external consumer deployment, runtime security assertion
or replacement of legacy arrays is implied.
