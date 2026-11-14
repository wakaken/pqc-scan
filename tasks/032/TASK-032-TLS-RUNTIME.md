# Task 032 TLS runtime: Executor evidence

Worker scope is the optional TLS crate, its standalone schema, protocol fixtures and plan. This is a review handoff, not task completion. Parent-owned core/envelope, CLI, human reports, README/user documentation and full workspace acceptance remain part of task 032. The source-bound machine record is [TASK-032-TLS-RUNTIME.json](evidence/TASK-032-TLS-RUNTIME.json).

## Selection and contract

The user delegated selection; the selected specification is [RUNTIME-PROBE-IMPLEMENTATION.md](../../docs/architecture/RUNTIME-PROBE-IMPLEMENTATION.md). This implementation pins rustls 0.23.46, AWS-LC 1.18.0 and webpki-roots 1.0.11, and ran on Darwin arm64 with Rust 1.93.1. `default=[]` keeps this crate provider-free; product `runtime` enables its explicitly selected backend. Fixture keys require separate `test-fixtures`, whose optional rcgen dependency is absent from the normal product runtime graph. Dependency graph evidence is owned by the parent (`target/task-032/default-normal-tree.txt`, `tls-normal-tree.txt`).

`new` prepares identity, CA roots and explicit client profile before networking; `run` uses the existing authorized/pinned bounded transport for one fresh full ordinarily verified handshake. The profile exports actual offered group/suite IDs. Only a verified completed full handshake exports negotiated facts; an error does not declare server-wide PQC absence. DNS SNI and verification identity, IP-without-SNI, trust source/bundle version and a canonical public root-set digest are explicit. Current common limits use exact integer nanoseconds.

No application data, resumption, early data, client credentials or key logging is enabled. A client-auth request is observed by a resolver that has no credentials, and aborts before another client write. The fixture records only TLS record types/counts and asserts no encrypted client auth/Finished response. Fixture private keys remain in memory; temporary files contain only public CA certificates and are removed on drop. Existing certificate parsing supplies typed public DER/SPKI fingerprints, OIDs, key size/curve and validity facts. `certificate_der_sha256` explicitly hashes the complete certificate DER.

## Executed verification

All following commands exited 0 after the final source changes. Generated evidence is under ignored `target/task-032/`, with isolated build output in `target/task-032-build/`.

- `cargo fmt -p pqc_scan_probe_tls -- --check`: `fmt-final.log`.
- `cargo check -p pqc_scan_probe_tls --no-default-features --target-dir target/task-032-build`: `default-check.log`.
- `cargo test -p pqc_scan_probe_tls --features runtime,test-fixtures --no-run --target-dir target/task-032-build --message-format=json`: `artifacts-final.jsonl`, `compile-final.log`.
- Both resulting TLS test executables ran with `sandbox-exec -p '(version 1) (allow default) (deny network-outbound) (allow network-outbound (remote ip "localhost:*"))' <binary> --test-threads=1`: unit 1 passed, runtime 11 passed, zero failures. `isolation-final.json` records exact binary hashes; hashes were checked before and after execution. Complete logs: `isolated-final-pqc_scan_probe_tls.log`, `isolated-final-runtime.log`.
- `cargo clippy -p pqc_scan_probe_tls --all-targets --features runtime,test-fixtures --target-dir target/task-032-build -- -D warnings`: `clippy-final.log`.

The local matrix includes TLS 1.2/1.3 classical, actual X25519MLKEM768 hybrid and HRR, untrusted/wrong-name/expired verifier failures, mTLS request abort, IP/SNI/root-set provenance, custom CA bounds/strict PEM, unsupported local offered profile, oversized records, collection/upstream chain limits, timeout/cancellation, traffic counters and no application/auth payload. Successful and incomplete fixture reports are validated offline against the self-contained TLS schema; a forged completed/unverified report is rejected.

TDD evidence includes initial missing product API (`runtime-red.log`), unsupported signature misclassification (`unsupported-red.log` -> `unsupported-green.log`) and actual oversized TLS record misclassified as protocol mismatch (`record-red.log`, `record-debug.log` -> final isolated runtime success). The latter required rustls `InvalidMessage::MessageTooLarge` to map to a local resource limit. The final assertion formatting difference was applied, then binaries, isolation and clippy were rerun.

## Limits and remaining acceptance

Local CA input is limited to 256 KiB and 16 CERTIFICATE blocks and replaces bundled roots; neither its path nor contents are exported. Certificate fact collection is limited to 16 certificates/256 KiB, with partial diagnostics distinct from rustls's smaller upstream handshake-message limit (approximately 64 KiB). Revocation is explicitly not checked. The default provider does not offer every supported registry group (for example P256MLKEM768); absent client support stays a local limitation, and one failed attempt is not a remote capability verdict.

Source review, isolated local execution and schema validation are distinct evidence. Independent TLS review is parent-owned and pending at this handoff. Whole-workspace/default/feature matrix checks, CLI/report integration and user docs are also parent-owned; this worker does not claim those, browser acceptance, remote CI, other platforms or release approval. No commit, push, PR, publication or task `done` transition was performed.
