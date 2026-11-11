# Task 031: isolated TLS API/provider feasibility

**Goal:** measure pinned rustls/provider feasibility without shipping TLS probes.
**Baseline:** continuation HEAD recorded in the generated bindings.
**Spec:** `tasks/031-tls-feasibility.yaml`, approved 030 safety policy and
`docs/architecture/TLS-SSH-PROBE-DESIGN.md`.

## Constraints

Rust 1.93.1; isolated development-only workspace, no root product dependency or
CLI changes. No public peers, insecure verifier, application writes, resumption,
0-RTT, client credentials, key log or retained private/session secrets. Only local
ephemeral CA/server keys generated in memory. Parent performs independent review.
Product backend/features/platform decision remains a maintainer gate after study.

## Steps

- [x] Pin rustls 0.23.46 with explicit std/aws_lc_rs/tls12 features and an in-memory
  certificate generator in `tests/feasibility/tls/`; use a separate target path.
- [x] Compile on Rust 1.93.1, recording exact lock, native compiler/build features
  and elapsed build cost rather than claiming MSRV from package metadata alone.
- [x] Build a local harness using 030's authorized bounded transport. Assert
  pre-handshake unknown facts, fresh TLS1.2/1.3, actual suite/group IDs, bounded
  certificate access, classical and available standardized hybrid groups.
- [x] Assert ordinary trust/name/expiry rejection, no client certificate, disabled
  resumption/0-RTT/key logging and zero application bytes from either side.
- [x] Run the compiled test binary under a deny-egress/allow-loopback sandbox;
  save sanitized result facts only. Build outside sandbox may fetch dependencies.
- [x] Record candidate support/unsupported/unknown outcomes, root-default feature
  isolation, source/binary/toolchain bindings and unexecuted platforms.
- [x] Set review only after worker evidence is available. Never select a product
  backend, mark done, commit, or start 032 from this study.

## Review focus

Negotiated group must be read after a completed full handshake; TLS1.3 suite alone
is insufficient. Certificate declarations do not prove quantum-safe authentication.
No failed verifier may yield a successful verified session. A certificate
collection cap can bound exported copies but may not bound rustls internal chain
allocation; report that distinction and its implications for 032 explicitly.
