# SSH KEX product integration

Explicit maintainer request: implement the selected russh backend, no userauth or
login, verify actual PQ/hybrid KEX and pretrusted host. Main implementation/testing;
one implementation reviewer then one separate test reviewer, no further agents.
Existing product static/TLS/SSH-offer bytes/contracts remain stable.

Contract: opt-in `probe-ssh-kex` feature, `probe ssh-kex HOST --port PORT
--host-key-file FILE`; bounded single public host-key input, no known_hosts/TOFU,
certificates/agent/config/private keys. Initial profile exactly
mlkem768x25519-sha256 + ssh-ed25519, no classical fallback. Per-invocation supplied
key is the trust pin for that explicit endpoint. No claim of PQ host signatures.
New separate probe.ssh-kex.v1.json plus existing style HTML/MD, protocol ssh_kex.
Failures expose no verified selection; clear diagnostic for untrusted key,
invalid signature, no overlap, budget/cancellation. Existing schema files unchanged.

1. Tests first for nonblocking bounded transport and pin/command prerequisites.
2. Extend BoundedTcpStream with metered try-I/O, preserving blocking callers.
   Tokio AsyncRead/Write adapter polls those methods with bounded timers, never
   exposes an unmetered socket or blocks an async worker. Shared stream is closed
   and runtime dropped on every outcome; existing watchdog/cancellation apply.
3. Implement optional russh0.65.1/AWS-LC backend, pinned lockfile, exact public-key
   trust, library signature/NEWKEYS then pin-checked completion, reject rekey and
   unrequested server channels. Disable keepalive/auth/channel/compression usage.
4. CLI, separate versioned schema and guarded bilingual summary with details
   closed. Preserve legacy formats. Update bilingual README/probing docs.
5. Local isolated real OpenSSH CLI matrix: trusted hybrid, wrong/unknown/invalid
   input, signature tamper, no overlap, timeout/cancellation/traffic, rejected
   rekey and oversized peer packets; server-decoded message types prove no5/50+.
   Unit tests for adapter/accounting/privacy and report typed-envelope guards.
6. One source review; frozen-source fmt/default/all-feature checks, protocol
   matrix and actual HTTP browser verification; one test-adequacy review. Record
   exact hashes/commands/remaining platform and hosted-CI limitations. No push.

Study prerequisite and provider/release limits: TASK-039-SSH-KEX-FEASIBILITY.md.
030 default caps stay unchanged; private/loopback requires explicit CIDR.
