# Task033 SSH advertised discovery evidence

Baseline HEAD: `cae2730fccce1d10198a49464f699a7df8f2d4d2`.
Status: implementation frozen; independent review and parent integration evaluation pending.

## Scope and guarantees

The optional runtime uses task030's authorized, pinned `BoundedTcpStream`.
It sends one identification and one diagnostic KEXINIT, then records the peer's
initial advertisement. It never executes KEX or sends service, userauth or channel
requests. Ten ordered lists retain duplicates and unknown names; extension markers
are separated without implying negotiation. Peer identification and offers remain
unauthenticated. Default builds provide pure parsers/types without transport or RNG.

The chosen caps, deadlines, client lists, send timing and nonzero boolean semantics
follow `docs/architecture/RUNTIME-PROBE-IMPLEMENTATION.md`. Classification uses exact
names from RFC4251/4253/8731/10042 and OpenSSH release documentation; unrecognized
names remain unknown. RFC10042 is Informational. No host key verification is claimed.

## Executed checks

- Parser RED: missing parser APIs, followed by five passing tests covering bounded
  names/lists/framing, truncation/mutations, markers and identification boundaries.
- Runtime RED: missing runtime module; nine passing local-peer tests covering split
  and coalesced input, peer waiting for client KEXINIT, only outgoing message20,
  terminal disconnect/unexpected messages, early-message/banner caps, partial EOF,
  cancellation/deadline/actual socket traffic limits and peer-secret omission.
- Additional real RED regressions: declared frame with absent body misclassified;
  valid SSH1.99 LF-only identification rejected. Both now pass.
- Independent review found overstrict ASCII validation of optional banner comments.
  A UTF-8 comment regression failed before the fix; softwareversion ASCII,
  control-character rejection and the 255-byte bound remain enforced.
- Schema RED: absent schema; offline schema validation now passes, including
  forbidden negotiated facts, fixed client semantics, secret-mask marker, and zero
  ns/traffic/DNS limits rejected. Traffic observations may correctly be zero.
- Final macOS sandbox executions: parser5, runtime9, schema1, all exit0.
  `target/task-033/isolated-final-results.json` binds exact executable hashes,
  verified unchanged before/after execution. Outbound connections are denied except
  localhost by sandbox-exec; no public endpoint is used.
- `cargo test -p pqc_scan_probe_ssh --no-default-features --target-dir target/task-033-default-build`: exit0.
- `cargo clippy -p pqc_scan_probe_ssh --features runtime --all-targets --target-dir target/task-033-build -- -D warnings`: exit0.
- `cargo fmt -p pqc_scan_probe_ssh --check`: exit0.

Logs are under `target/task-033/`, including original RED and final GREEN records.
No package-not-found setup failure is counted as a behavioral RED.

## Installed daemon interoperability

The explicit ignored fixture creates temporary host keys, binds only loopback,
disables authentication/forwarding, reads sanitized DEBUG3 message counters and
cleans up its process group and key directory. The test-only hybrid environment
value accepts only three fixed names; product runtime never reads it.

macOS OpenSSH10.5p1 could not run its preauth Darwin sandbox inside the outer
sandbox-exec (`sandbox_init: Operation not permitted`). That attempt is failed,
not acceptance evidence; neither sandbox was weakened.

Linux interop passed after the final UTF-8 comments correction: both classical
`curve25519-sha256` and hybrid `sntrup761x25519-sha512@openssh.com` advertised
offers were collected, with server receive types `[20]` and zero service/userauth/
channel requests. Debian OpenSSH9.2 / Rust1.93.1 ran on Linux arm64. Source/binary,
base/image digests, settings, safe reports and counters are preserved in
`tasks/033/evidence/TASK-033-SSH-DISCOVERY.json`.

Linux container evidence is recorded separately by `tests/protocol/ssh-interop.py`.
It copies only git-listed Cargo/toolchain/crates paths into an ephemeral build
context (rejecting symlinks, target, .git and .env); repository-wide COPY is absent.
Only build/package preparation uses network. The actual ignored fixture runs with
Docker `--network none`, default seccomp, no privileged option or capability adds.
Ephemeral host keys are created and removed inside that container. The runner
records image/base digests, architecture, versions, source/binary hashes, container
network settings, test outcome and two safe reports. This is evidence for these
SSH fixture profiles, not all Linux product behavior.

## Remaining evaluation

Parent owns shared CLI/report/core integration, workspace checks and independent
review/evaluation. External CI and other platforms are not executed by this worker.
Task is not marked done until those acceptance gates pass.
