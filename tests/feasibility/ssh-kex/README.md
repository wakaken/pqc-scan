# Completed SSH KEX feasibility (task039)

Developer-only standalone study, outside the product workspace/lockfile. No
product command, schema, rule, runtime dependency or report renderer is changed.

Use Rust1.93.1 and Docker. Run from the repository root:

```sh
python3 tests/feasibility/ssh-kex/run-docker.py --out-dir target/task-039/final
```

Build preparation downloads public registry dependencies and a SHA256-pinned
OpenSSH10.2p1 release from its official mirror. The base Rust image is digest-pinned.
Actual cases run under Docker `--network none`, no host mounts, nonprivileged,
no-new-privileges, NET_RAW dropped, finite memory/CPU/PID limits. Only loopback is
active; unused kernel tunnel interfaces may remain down. No host SSH config, username, private key, agent or credential is supplied.
OpenSSH retains its own default privilege separation/seccomp sandbox. Native macOS
sshd inside sandbox-exec cannot initialize its nested sandbox in this environment;
that failed preflight is not evidence of unsupported KEX and protections are not
disabled as a workaround.

Ephemeral Ed25519 fixture keys/config are kept in an OS temporary directory and
removed. Only message type numbers and typed outcomes are retained; encrypted
post-NEWKEYS packets are observed using the server's decoded DEBUG3 type events,
not inferred from a plaintext capture. Raw server/client debug logs, complete
packets and secret bytes are never persisted. The tamper proxy flips one byte of
the initial signature solely to exercise the library verifier; it implements no
cryptographic operation. Stalled peer/client subprocesses have finite deadlines.

Study profiles: ML-KEM768+X25519 hybrid, classical X25519, wrong/unknown trust pin,
invalid signature, incompatible KEX, stalled banner, and ssh2's automatic-service
negative control. Success requires both NEWKEYS directions, prior key pin match,
and exact selected KEX; failures expose no verified selected KEX. Server receive
numbers must show no SERVICE_REQUEST(5), userauth/global/channel messages (>=50)
for russh. The ssh2 control intentionally demonstrates type5 and is rejected for
this requirement even if a build has ML-KEM capability.

The trust pin is the exact separately provisioned public host key for this fixture.
A key fetched from an untrusted connection is not a trust anchor. OpenSSH host
certificates are rejected by this minimal study; CA/principal/expiry/known-host
patterns are future integration work, not silently accepted by embedded-key match.
Classical Ed25519 host authentication does not become PQ authentication because
the key exchange is hybrid. Only this specific session/profile is established.

This harness is not production-hardened: the production030 address/traffic/
cancellation policy must wrap a future backend. No arbitrary target support or
all-platform compatibility is inferred from a single Linux arm64 fixture study.
