# Task 031: pinned TLS feasibility evidence

Date: 2026-11-10. Status: **independent spec and quality source reviews accepted**; not a
product backend selection or a supported-platform release declaration. Baseline
product HEAD: `802f2c6aa7436ce02332c2c3013c835d8c85d904`.
The [machine record](evidence/TASK-031-TLS-FEASIBILITY.json) binds source/lock/binaries,
commands, feature graphs, safe session facts and unexecuted checks.

## Candidate and actual build

The unpublished, independent [harness](../../tests/feasibility/tls/README.md) pins
rustls **0.23.46**, aws-lc-rs **1.18.0**, rcgen **0.14.7** and its own Cargo.lock.
Resolved native provider is aws-lc-sys **0.44.0**; webpki is **0.103.16**.
Rust **1.93.1**, aarch64-apple-darwin, macOS **27.2 (26B123)** and Apple clang
**17.0.0** compiled and ran the study. No Rust toolchain change was made.

The first dedicated-target debug test build took **12.96 s wall / 47.58 s user /
16.49 s system**, including native/provider compilation. Registry/OS caches were
already populated; this is neither a cold-cache measurement nor a performance
budget. Raw build evidence is `target/task-031/build.log`.

Both variants disable rustls default features and enable explicit `std` and
`aws_lc_rs`; the default harness feature adds `tls12`. The second variant omits
`tls12`. Neither enables rustls logging, prefer-post-quantum, FIPS or ring-provider
features. Exact group lists are supplied explicitly, so these results do not
demonstrate the provider's default negotiation preference.

**Feature qualification:** rcgen's AWS-LC dependency also enables aws-lc-rs default
features, producing the actual union `alloc`, `aws-lc-sys`, `default`,
`prebuilt-nasm`, `ring-io`, `ring-sig-verify`. These last two names are AWS-LC
compatibility APIs, not evidence that rustls selected the ring provider. The
machine record preserves the actual union; do not call this an experimentally
verified minimal product AWS-LC feature graph. Product build choices must be
reviewed and tested after the maintainer selects them.

## Isolation and executed matrix

Compilation/cache acquisition occurred outside the runtime sandbox. Every TLS
test executable ran through `sandbox-exec` with this policy:

```text
(version 1) (allow default) (deny network-outbound)
(allow network-outbound (remote ip "localhost:*"))
```

The [runner](../../tests/feasibility/tls/run-isolated.py) builds offline/locked,
extracts the exact test executable, then applies that policy. Only ephemeral
127.0.0.1 listeners are bound. Actual client sockets use task 030's explicit
loopback CIDR authorization, pinned peer, byte metering, absolute deadlines and
watchdog. Private CA/server keys stay in memory; no key, session secret or wire
payload is persisted. Neither peer writes application plaintext, and both check
that no application plaintext was received. No client certificate is configured
or presented; server-side peer certificate state remains absent.

| Matrix / check | Executed outcome |
|---|---|
| AWS-LC + TLS1.2/TLS1.3 | 10 tests passed inside sandbox |
| AWS-LC + TLS1.3 only | 9 tests passed inside sandbox |
| TLS1.2 classical | Full verified X25519, group 29, suite 49196 |
| TLS1.3 classical | Full verified X25519, group 29, suite 4866 |
| TLS1.3 X25519MLKEM768 | Full verified group **4588**, suite 4866 |
| TLS1.3 secp256r1MLKEM768 | Full verified group **4587**, suite 4866 |
| HelloRetryRequest | FullWithHelloRetryRequest, actual hybrid group 4588 |
| Ordinary verifier negative controls | Untrusted issuer, wrong name and expired leaf all rejected |
| Before handshake | Version, suite, group, chain and early-data handle all absent |
| Reused client configuration | Two independent connections both Full, never Resumed |
| Certificate collection | Count/byte ceilings checked without cloning certificate bytes |
| 17-element real presented chain | Post-handshake collection count limit reported |
| 201-element real presented chain | rustls upstream handshake/message resource limit reported |
| Harness clippy / format | Both exit 0; exact commands in machine record |
| Product default normal dependencies | Neither rustls nor aws-lc present; root workspace unchanged |

The two hybrid numeric identifiers correspond to the mechanisms specified by
[RFC 10024](https://www.rfc-editor.org/rfc/rfc10024.html). The same TLS1.3 suite
4866 appears in both classical and hybrid sessions; the group accessor, verified
handshake completion and exact group ID are the evidence. No claim about other
server capabilities, universal PQC readiness or quantum-safe certificate
authentication follows.

## Important API result: group accessor after verifier failure

The online [CommonState documentation](https://docs.rs/rustls/latest/rustls/struct.CommonState.html)
describes optional negotiated group state. The pinned compiled candidate returned
**Some(X25519 / 29) after all three certificate verification failures**, even
though `is_handshaking()` remained true and `peer_certificates()` was absent.
The machine log labels this `unverified_accessor_group_id`, with
`verified: false` and `handshake_complete: false`. Before starting any handshake,
the accessor was genuinely None.

An additional assertion expecting None after verification failure exposed this
behavior and failed. The study now records the actual result instead of forcing
the documentation assumption. Task 032 must publish negotiated/verified facts
only after a successful ordinary verifier and completed fresh handshake; the
mere presence of a group, version or suite accessor cannot establish success.
No failed verifier bypass was introduced.

## Certificate and other capability limits

Public certificate-chain access is available after successful verification and
was bounded before copying/exporting. The client collection policy is 16
certificates / 256 KiB aggregate; the tested oversized chain correctly became
incomplete, not an unsafe-server verdict. This collection bound does not control
rustls's prior internal decoding allocation. Pinned rustls source separately
limits handshake/certificate message buffering to approximately 64 KiB; the
201-element test reached that local limit before collection. Larger PQC chains
may therefore be unsupported locally even below the approved transport ceiling.
Task 032 must retain a local-limit/incomplete outcome and cannot claim support
for every chain below 256 KiB.

The provider's `ALL_KX_GROUPS` returned IDs `[29, 23, 24, 4588, 4587, 513, 514]`. The two
tested RFC hybrid groups are available; SecP384r1MLKEM1024 (4589) is absent from
that provider list. Pure ML-KEM entries 513/514 were listed, not exercised or
promoted to additional standards/protocol claims. Unknown groups remain unknown;
a local unsupported profile is not evidence of remote absence.

CertificateVerify scheme has no general accessor demonstrated here. Revocation
validation, PQC certificate signature verification, TLS1.0/1.1, alternate providers,
OpenSSL comparison and FIPS validation were not performed. Local CA trust in this
test is not a claim about OS/public trust-store behavior. Linux/Windows and other
architectures were not built or run. Hosted CI was not run.

## Reproduction and corrections

Run the commands in the harness README. Safe raw evidence is under ignored
`target/task-031/`; binaries live under the distinct `target/task-031-build/`.
All command exits and SHA-256 bindings are in the machine record. Package
metadata initially failed in offline all-platform mode because an Android-only
dependency was not cached; host-filtered metadata succeeded. No other-platform
validation is inferred from that host result.

The initial runtime harness failed with peer EOF: on this macOS host, accepted
sockets inherited the nonblocking listener mode. Investigation found server
`WouldBlock` before any handshake. Setting the **test server's** accepted socket
explicitly blocking with finite read/write timeouts fixed the fixture. The client
continues using the production bounded nonblocking transport. No cryptographic
or verifier behavior was relaxed.

## Proposed decision, still awaiting maintainer choice

Accept rustls 0.23.46 / AWS-LC 1.18.0 as a **conditional macOS aarch64 candidate**
for an explicit optional probe, based on the compiled facts above. Keep the
default static product free of protocol/provider dependencies. Decide exact
product features, supported OS/architectures and trust inputs before 032; retest
the resulting product graph. Reject any fallback to custom TLS cryptography or
insecure certificate/name verification. This study does not authorize or implement
the product TLS command, and it does not itself satisfy that maintainer gate.

Independent review verified all11 bound source/lock files and both test binaries,
the19 isolated cases and the stated constraints, with no blocking finding. Its
wording correction distinguishes provider `ALL_KX_GROUPS` from wire advertisements.
The maintainer subsequently delegated selection and continuation. The agent selected
the conditional rustls/AWS-LC candidate, explicit std/aws_lc_rs/tls12 features,
macOS aarch64 evidence scope, pinned local Mozilla roots and explicit local-CA
replacement; product feature isolation will be tested in032. Evaluator accepted031.
No push, PR, merge, package publication or external endpoint connection was performed.
