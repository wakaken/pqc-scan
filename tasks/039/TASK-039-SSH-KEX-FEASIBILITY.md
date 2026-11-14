# Task039 — no-userauth, trusted-host hybrid SSH KEX

2026-11-11. Scope: selected by the maintainer for investigation and backend
selection only. Current product `probe ssh` remains advertisement-only. No runtime
command, static rule, root Cargo dependency/lockfile or report format is changed.

## Decision

Select **russh0.65.1 with AWS-LC**, conditionally for a separately designed future
negotiated mode. A standalone executable demonstrated ML-KEM768+X25519 KEX and
verification against a separately provisioned Ed25519 public host-key pin, then
disconnected before any authentication service request. Do not use ssh2/libssh2
or parse system `ssh -vv` as the backend for this exact no-service requirement.
This is a feasibility decision, not a product implementation or release approval.

The distinction is threefold: the peer offers an algorithm; one algorithm is
actually selected and NEWKEYS completes; the KEX signature is verified and its
host key matches a prior trust anchor. Only all latter checks together justify
verified session facts. A key learned from the same connection does not provide
prior trust. A hybrid KEX with a classical host signature is not PQ host
authentication or proof that every client/server connection uses PQC.

## Candidate comparison

| Candidate | Observed/API result | Decision |
|---|---|---|
| russh0.65.1, default features off, aws-lc-rs on | MLKEM768X25519_SHA256; connect_stream accepts an already connected stream; check_server_key can reject unknown keys/certificates; kex_done exposes selected Names after signature/trust checks; no service/auth/channel request in successful or rejected fixture cases | Select conditionally; integration must wrap existing030 limits and disable unwanted activity |
| ssh2 0.9.7, libssh2-sys0.3.4 | Public handshake/methods/host_key APIs are useful, but handshake itself automatically sends ssh-userauth SERVICE_REQUEST(5). Local capability differs by linked OpenSSL: macOS build lists ML-KEM; Linux OpenSSL3.0.22 does not | Reject unmodified public API for the strict no-service contract; do not infer server PQC absence from a local provider limit |
| OS OpenSSH command | Production client proceeds from ssh_kex2 to ssh_userauth2; -N suppresses remote command execution, not authentication; external config/version/debug text complicates a stable machine contract | Source/API rejection for backend; no claim of an executed no-auth system-client test |

## Exact build and isolation

Standalone files: `tests/feasibility/ssh-kex/`, with independent Cargo.lock.
Rust1.93.1; russh MSRV1.89; locked AWS-LC1.19.0/aws-lc-sys0.47.0,
ssh-key0.7.0-rc.11, Tokio1.49.0, ssh2 0.9.7, libssh2-sys0.3.4.
The libssh2 bundled source identifies1.11.2_DEV; do not equate a stable Rust
binding release with stable native upstream release status. ssh-key's prerelease
status and native build/provider dependency need explicit adoption review before
production integration. No claim of FIPS certification is made.

Both candidate clients compile on macOS arm64. Runtime matrix is Linux arm64 in
Docker, with OpenSSH10.2p1 built from its official checksum-pinned release and its
own sandbox intact. Base Rust1.93.1-bookworm image is digest-pinned. Dependencies
are downloaded only during preparation; runtime has network none, no host mount,
no privileges beyond ordinary Docker defaults, NET_RAW dropped, no-new-privileges,
1GiB/2CPU/128PID caps. Only loopback is active. Each fixture uses one connection;
there is no listener readiness TCP connection. Keys/config exist in a removed OS
temporary directory. No ambient user config, agent or credential is used.

The initial native macOS attempt failed before KEX because nested sandbox_init
was denied. It is explicitly failed evidence, not an algorithm result. No
protection was disabled to make it pass; Docker replaced that test environment.

## Observed cases

First successful complete study: `target/task-039/docker-study-third/`.
Final source-bound rerun and both review acceptances are recorded below.

| Case | Result | Server-decoded client packet types |
|---|---|---|
| Hybrid, trusted pin | Completed, exact mlkem768x25519-sha256, prior pin matched |20,30,21,1|
| Hybrid, wrong pin | Untrusted host, no verified selected KEX |20,30,21|
| Hybrid, unknown pin | Untrusted host, no verified selected KEX |20,30,21|
| Hybrid, tampered signature | Invalid signature, trust callback not reached |20,30|
| Classical-only control | Completed curve25519-sha256; not mislabeled PQC |20,30,21,1|
| Hybrid-only client vs classical-only peer | No overlap; no verified result |20|
| ssh2 negative candidate control | Handshake/trust can succeed but service request occurs, without user authentication |20,30,21,5,1|
| Stalled banner | Outer5s timeout, no verified result; fixture thread stops |No completed handshake|

20=KEXINIT,30=KEX method initialization,21=NEWKEYS,1=DISCONNECT,5=SERVICE_REQUEST.
For successful russh cases the server both sends and receives NEWKEYS and decodes
the subsequent encrypted DISCONNECT. Absence of auth/service/channel packets is
asserted from server-side decoded packet-type events, not from opaque encrypted
network bytes or simply the absence of calls in application source. ssh2's type5
negative control demonstrates that this observer detects post-KEX service traffic.
No payloads, secret key material, shared secrets or raw debug transcripts are saved.
The signature test changes exactly one byte in the unencrypted Ed25519 signature;
it does not bypass the verifier or introduce a cryptographic implementation.

## Selected integration shape and outstanding work

- Explicit separate negotiated mode; retain current discovery behavior and label.
- Require an operator-provided trusted public key/pin bound to the requested
  endpoint. Reject missing/mismatched pins; never auto-enroll a scanned key or read
  ambient known_hosts. This study tests exact public-key pinning, not the full
  known_hosts syntax, revocation or host certificate CA/principal/time validation.
- Use the030 resolved/authorized/pinned socket with connect_stream, rather than
  letting the library resolve or retry a host. A bounded AsyncRead/AsyncWrite
  adapter, cancellation/task cleanup and packet/resource limits need production
  tests; this loopback-only harness is not evidence that all030 caps are integrated.
- Offer a fixed hybrid-only profile for a PQC verification request. Do not silently
  fall back to classical. Record exact selected wire name only after signature,
  pin and completed initial KEX; never derive it from first-common offer matching.
- Do not invoke authenticate_none or any authentication/global/channel API; disable
  keepalives, forwarding/compression/unneeded algorithms, stop after initial KEX.
  Re-key, malicious-server requests, traffic/packet exhaustion and platform matrix
  remain separate production-hardening tests. Ignore the kex_done shared secret.
- Keep endpoint trust and host-signature algorithm separate from PQ/hybrid KEX.
  Ed25519 is classical. PQ host certificates/signatures were not implemented or
  tested. Timeout/provider/no-overlap does not prove global server non-support.
- A production command/output contract needs a new reviewed implementation plan;
  task039 authorizes the study, not an implicit schema change or shipped mode.

## Primary sources

- [russh Handler: signature/trust callbacks and KEX completion](https://docs.rs/russh/latest/russh/client/trait.Handler.html).
- [russh connect_stream: caller-provided stream](https://docs.rs/russh/latest/russh/client/fn.connect_stream.html).
- [ssh2 Session: handshake, methods and host-key APIs](https://docs.rs/ssh2/latest/ssh2/struct.Session.html).
- [libssh2 session source](https://github.com/libssh2/libssh2/blob/master/src/session.c): session startup service request; exact reviewed crate source is hash-bound in study evidence, not inferred from moving master.
- [OpenSSH10.2 ssh_login source](https://github.com/openssh/openssh-portable/blob/V_10_2_P1/sshconnect.c), [ssh manual](https://man.openbsd.org/ssh.1).
- [Official OpenSSH10.2 release checksum](https://www.openssh.org/txt/release-10.2), [OpenSSH PQC explanation](https://www.openssh.org/pq.html).
- [RFC4253: transport host authentication and NEWKEYS](https://www.rfc-editor.org/rfc/rfc4253.html), [RFC10042: hybrid ML-KEM SSH KEX](https://www.rfc-editor.org/rfc/rfc10042.html).

Links may move to newer documentation. The study lockfile and evidence hashes
identify the actual source reviewed/compiled; results are limited to those builds.

## Final source-bound execution

Final rerun: `target/task-039/final/DOCKER-RESULTS.json` and
`results/RESULTS.json`, indexed with source/upstream/evidence hashes in
`evidence/TASK-039-SSH-KEX-FEASIBILITY.json`. All eight allowlisted study files match the
container source hashes and still match the checkout. Both executable hashes are
recorded before/after and agree with independent image metadata hash output.
All eight cases pass, including server-decoded post-NEWKEYS disconnect/control.
The exact final image ID, base digest, compiler/OpenSSH/provider versions,
security settings, case outcomes and cleanup are recorded. The container was
removed and temporary fixture keys were deleted. Built local images are retained
as reproducible evidence, not published.

Both standalone clients compile on macOS arm64; standalone clippy and root/study
format checks pass. Existing product workspace346 tests and Node28 tests pass.
All148 previously bound product source/manifests remain byte-unchanged. The
standalone study is not a claim of extra product test coverage or a deployed mode.

One implementation reviewer (`ssh039_impl_review`) and one separate test-adequacy
reviewer (`ssh039_test_review`) accepted with no blocking findings. The latter
independently verified eight source files/two binaries inside the final image,
seven reviewed upstream files,148 unchanged product files and nine evidence files.
No full-suite duplication was performed. Evaluator accepts task039 as a completed
feasibility/selection study, not as a product implementation.

Self-scan retains1087 raw results (previous1082): five new dependency observations
come from the standalone study lockfile (OpenSSL/RSA/PQC-library references). No
new exception or suppression was added. A lockfile reference is not a claim that
a dependency is shipped in the product binary; root product dependencies remain
unchanged. Raw evidence is `target/task-039/self-scan/report.sarif`.

The exact eight tested study files were committed unchanged as `ec4df5c5d8734b7dde7c7fec542976793840830d`.
Execution records retain the original base4291ae7 plus frozen file hashes; this
commit binding does not rewrite the recorded pre-commit worktree as a clean run.
