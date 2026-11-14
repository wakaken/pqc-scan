# Product SSH hybrid KEX verification — 2026-11-11

Explicit maintainer implementation request after task039's feasibility study.
Base207f425. This follow-up ships an optional experimental mode, unlike the earlier
study. Static scan remains offline; default build excludes russh/Tokio. Existing
TLS and SSH-advertisement schemas/files remain unchanged. No push/publication.

## Contract and implementation

Feature `probe-ssh-kex`; command `probe ssh-kex HOST --port PORT --host-key-file
FILE`. The endpoint-specific pin is one separately provisioned OpenSSH Ed25519
public key, at most4096 bytes, checked before DNS/connection. Unix O_NONBLOCK and
O_NOFOLLOW plus regular-file checks reject FIFO/symlink/private-key input without
blocking or printing input. No known_hosts, TOFU, certificates, agent, credentials,
ambient SSH config or system ssh client is used.

russh0.65.1, default features off, AWS-LC1.18.0 (same pinned provider as existing
TLS), Tokio1.49.0. The old study used AWS-LC1.19.0; product validation therefore
runs the actual new executable rather than reusing the study's result. Root lock
adds optional dependencies; existing libc/typenum resolve to newer compatible
versions, so full default/feature regressions are required. Upstream russh agent
allocation and EXT_INFO advisories are patched by this version; this is not an
assertion of universal vulnerability absence.

Only mlkem768x25519-sha256 plus ssh-ed25519 is offered. No classical KEX fallback.
The library verifies the exchange signature; an exact trusted key must match and
initial NEWKEYS must complete. No userauth/global/channel API is invoked. Keepalive
and compression are disabled, repeated completion is rejected, and the connection
is disconnected. A synchronous call from an existing async runtime fails locally
without connecting rather than panicking on nested runtime entry.

BoundedTcpStream retains exclusive policy control. New nonblocking try-I/O methods
meter bytes and check cancellation/deadlines without exposing a raw socket. The
Tokio adapter uses bounded timer wakeups; its polls do not perform blocking reads.
Final closure and runtime destruction end background work before traffic is read.
Existing one-attempt/address/DNS/TCP/protocol/overall/socket-byte caps apply. The
pinned russh parser independently bounds identification lines and transport packet
allocation (256KiB baseline plus transport overhead); oversized header tests check
rejection without reading an oversized body. No cryptographic implementation is
added to the transport adapter.

Output: separate `probe.ssh-kex.v1.json`, HTML and Markdown, defined by the new
self-contained schema. Verified KEX/signature fields stay null on any failed or
partial result, even after an intermediate successful callback. HTML requires a
completed trusted exact-hybrid result before displaying success; the outcome and
next action precede closed technical details. Host authentication uses classical
Ed25519, not PQ signatures. The observation covers one endpoint/profile only.

## Validation evidence

Initial real-product OpenSSH matrix: `target/ssh-kex-product/interop-first/`.
Five cases: Japanese/English trusted hybrid success; wrong trusted key rejected;
exactly one altered signature byte rejected; hybrid-only vs classical peer rejected.
OpenSSH10.2p1's decoded receive events show20,30,21,1 for successful connections,
without SERVICE_REQUEST5 or userauth/global/channel messages50+. The server sends
and receives NEWKEYS and decodes the encrypted disconnect, so no-auth is not merely
inferred from an encrypted byte capture. Temporary host private keys are removed.

Docker runtime uses network none, no host mounts, nonprivileged/no-new-privileges,
NET_RAW dropped, bounded memory/CPU/PIDs; OpenSSH's own sandbox remains enabled.
Build preparation downloads official checksum-pinned OpenSSH and registry inputs;
runtime does not access external endpoints. Context is allowlisted, includes no
home/config/keys, and records every input hash. CI definition includes this product
matrix; no hosted execution is claimed until actually run for the target commit.

Additional Rust checks cover malformed/missing/private/oversized pin before any
connection, denied addresses with zero traffic, exact bilingual JSON/schema parity,
nonblocking metered transport, send/receive clipping, cancellation and deadlines,
FIFO/symlink rejection, repeated-KEX trust guard, oversized SSH packet header,
no false verified HTML summary and synchronous-runtime misuse. Full known_hosts,
host certificate/PQ signature support and exhaustive malicious encrypted rekey or
channel scenarios are not claimed. Linux arm64 real peer and macOS arm64 bounded
fixture coverage are separate from untested platforms.

Final evidence index: `evidence/SSH-KEX-PRODUCT-VALIDATION.json`, code commit `b0d70efbc5eb6f7c92efed656dda572b8642105b`.
The tested pre-commit source hashes match that committed source exactly; records
retain their actual execution state and are not rewritten as clean-source runs.

Final default workspace347 tests pass; isolated all-feature matrix397 pass with
one ignored legacy OpenSSH test,54 test executables and55 hash-bound binaries.
All-feature clippy/format pass, Node28 and distribution Python8 pass. Actual
product CLI/OpenSSH5 cases pass at `target/ssh-kex-product/interop-final/` with
163 frozen context inputs. The schema checker independently validates all five
real artifacts and rejects contradictions involving auth/trust/failure facts.
The default normal dependency tree excludes russh, Tokio and socket2.

Actual in-app browser observations over loopback HTTP: Japanese/English success
and failure pages; technical details initially closed, Tab focus and Enter opening;
measured document widths1280 desktop and390 narrow, no horizontal overflow with
expanded details. No external assets. An already-loaded English failure page's
native details toggled while the dedicated server was stopped; this is not an
OS-wide offline test. The server was restarted and user report tabs preserved.
Page hashes, actions and actual screenshots are under
`target/ssh-kex-product/browser/`; browser version was not exposed by the supported
API, so no cross-browser/device guarantee is claimed.

Implementation reviewer `ssh_kex_product_impl_review` and separate test-adequacy
reviewer `ssh_kex_product_test_review` accepted, with no blocking findings. The
latter independently checked sources, inputs, executed binaries and evidence
hashes. Exactly two reviewers; no delegated implementation or automatic suite
repetition. Evaluator accepts this product integration within the documented
experimental/platform scope. No actual current-commit hosted CI is claimed.

Self-scan keeps1091 raw findings (previous1087). Existing34 main.rs test results
were rebound only to identical unchanged lines; source/result bodies match under
that line map and no new exception was added. Unmatched raw delta has10 additions
and6 removals (including shifted unregistered fixture/lockfile locations), retained
in `SELF-SCAN-DELTA.json`. No new observation was hidden by policy expansion.

Existing prepared distribution archives predate this feature; they are not
relabeled as containing it.

Primary advisory references:
[RUSTSEC-2026-0154](https://rustsec.org/advisories/RUSTSEC-2026-0154.html),
[russh EXT_INFO fix](https://github.com/Eugeny/russh/security/advisories/GHSA-4p9f-7p9m-5vfg).
