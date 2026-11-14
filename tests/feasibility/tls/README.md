# TLS feasibility harness (task 031)

This independent, unpublished Cargo workspace evaluates pinned rustls 0.23.46
with AWS-LC 1.18.0 on Rust 1.93.1. It is development evidence, not a shipped TLS
probe or an approved product backend/platform support declaration.

Only local loopback servers are used. Test CA/server keys are generated in memory
and discarded; no private key, TLS session secret or wire payload is written.
The client uses the ordinary local-root certificate/name/time verifier and task
030's pinned-address, metered, deadline-controlled transport. TLS application
writers are never used; both peers check that no application plaintext is present.

First populate the dependency cache and compile outside the runtime sandbox:

```sh
cargo test --manifest-path tests/feasibility/tls/Cargo.toml --target-dir target/task-031-build --locked --no-run
```

Then run the two selected feature variants on macOS:

```sh
python3 tests/feasibility/tls/run-isolated.py
python3 tests/feasibility/tls/run-isolated.py --without-tls12
```

The runner builds offline, then runs the exact compiled test binary under a
`sandbox-exec` policy denying outbound networking except localhost. Raw safe logs,
commands, exits and binary digests go to ignored `target/task-031/`; the dedicated
build directory avoids feature/binary collisions with the product workspace.
No Internet endpoint is contacted. Other operating systems require an equivalent
reviewed isolation runner and their own evidence; they are not declared supported.

The default harness feature adds rustls TLS 1.2. Both variants explicitly enable
only rustls std/AWS-LC provider features; they do not enable rustls defaults,
logging, FIPS or prefer-post-quantum. Tests explicitly select exact group lists,
so results do not imply server-wide capabilities or provider-default negotiation.

The matrix covers TLS 1.2/1.3 classical exchange, two available standardized
hybrids, HelloRetryRequest, pre-handshake unknowns, trust/name/expiry failure,
certificate collection/upstream message limits and repeated fresh handshakes.
Read the [evidence and limitations](../../../tasks/031/TASK-031-TLS-FEASIBILITY.md),
especially provisional group access after failed certificate verification.
