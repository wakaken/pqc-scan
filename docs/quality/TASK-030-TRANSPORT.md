# Task 030: policy and optional transport evidence

Date: 2026-11-10. Baseline HEAD:
`3c357cef879f69ae4058fefe7a03b4f074283b3f`.
Status: worker implementation evidence; **not task completion**. The parent
integrates the DNS helper and performs independent review, isolated acceptance
and workspace validation. The worktree contains concurrent 034–036 changes;
these worker checks do not independently validate those tasks.

Parent acceptance update: Evaluator accepted task030 after the DNS integration,
independent source review and isolated checks below. The worker record remains
historical evidence; final integration is described separately.

The optional CLI uses a self-executable hidden resolver helper, not an abandoned
DNS thread. The child emits at most nine IP lines, below pipe capacity; the ninth
answer causes rejection. Parent output reads are capped at 513 bytes. A fixed
deadline/cancellation check polls the child, and all exits kill and reap it. Unix
tests directly assert `waitpid` returns `ECHILD` after cleanup. Normal `scan` does
not call this helper; no TLS/SSH protocol command ships in task030.

Under macOS `sandbox-exec` with outbound sockets denied except loopback, policy13,
transport11 and resolver4 all passed. Exact binaries/logs are bound in
`target/roadmap-continuation/030-isolation-results.json`. The first isolated resolver
attempt failed because another default Cargo build replaced its shared executable;
the successful rerun uses `target/probe-transport-build`. Both attempts are retained.
Independent review accepted the fixed-deadline watchdog and direct child-reaping
assertions. This establishes local macOS aarch64 evidence, not Linux/Windows support.

See [integration evidence](ROADMAP-CONTINUATION-VALIDATION.md) for workspace321,
transport338, Node28/Python20, builds/fmt/clippy and remaining unexecuted checks.

## Implemented boundary

`target_policy` parses host/port and strict IDNA using pinned idna 1.1.0, normalizes
IPv4-mapped IPv6, bounds explicit CIDRs (64), checks every bounded DNS answer,
retains checked addresses separately from identity, and creates a private-field
`AuthorizedTarget`. Empty results fail; answer 9 fails before deduplication. A
budget permits one resolution and one connect attempt. Authorized targets are
bound to the creating budget, preventing deadline reset through budget substitution.

Public-unicast classification deliberately requires explicit CIDRs for
special-purpose and tunnel ranges, including all 2001::/23, NAT64, 6to4 and
documentation prefixes. This conservative default is not a routability claim.
Unspecified, multicast and limited broadcast addresses are never authorized.
Unknown network-specific directed broadcast addresses cannot be inferred from a
host address alone. Scoped IPv6 input is rejected; actual peers must match the
pinned address and port without scope expansion.

`probe_transport` is compiled only with the `probe-transport` feature. socket2
0.6.4 creates a nonblocking socket; libc supplies the Unix EINPROGRESS constant.
Only that pinned socket is connected. Five-millisecond polling checks cancellation
and the minimum absolute phase/aggregate deadline. There is no fallback, retry,
proxy/config processing or exposed unmetered stream. Read/write buffer lengths
are clipped before I/O, and actual successful bytes are debited immediately.
Partial I/O never resets the protocol deadline. Error/Drop closes the socket;
cancelling an idle connected socket also shuts it down through a weak reference.
A single watchdog per connected socket also shuts down an idle socket at the
absolute protocol/aggregate deadline, even when the protocol makes no I/O calls.
The watchdog performs no DNS or socket I/O except shutdown: it waits on a Condvar
in bounded intervals with only a weak socket, deadline and cancellation state.
Close/Drop notifies and joins it; a blocked or detached DNS thread is not used.
The standard Read/Write adapters preserve typed errors and use ConnectionAborted
for cancellation so standard read_exact/write_all do not retry cancellation as
Interrupted. No TLS/SSH protocol, key, authentication or application operation is
implemented by these modules.

The resolver trait explicitly requires deadline/cancellation cleanup. Core does
not run synchronous system resolution in a detached thread. The parent-owned
optional CLI process adapter is a separate integration requirement; a trait mock
does not establish real DNS cancellation.

## Executed checks

Rust 1.93.1, aarch64-apple-darwin. Raw logs are in ignored `target/task-030/`.

| Command | Result |
|---|---|
| `cargo test -p pqc_scan_core --test target_policy` before implementation | 101; missing target policy module, with concurrent missing diff module also present |
| `cargo test -p pqc_scan_core --features probe-transport --test probe_transport` before transport | 101; unresolved transport import |
| Added budget-substitution and idle-cancellation tests before fixes | 101; both expected assertions failed |
| Reviewer idle-deadline test before watchdog fix | 101; idle peer stayed open until its test read timed out |
| Added expired flush cleanup test before fix | 101; peer remained open |
| Added late DNS error precedence test before fix | 101; late DnsFailed masked elapsed deadline |
| `cargo test -p pqc_scan_core --features probe-transport --test target_policy --test probe_transport` | 0; 13 policy and 11 loopback transport tests passed (including reviewer idle-deadline fixes) |
| `cargo clippy -p pqc_scan_core --lib -- -D warnings` | 0 on final worker sources (`clippy-final-default.log`) |
| `cargo clippy -p pqc_scan_core --all-targets --features probe-transport -- -D warnings` | 0 on final worker sources, including watchdog (`clippy-watchdog.log`) |

Loopback tests prove actual peer binding, exact byte caps, cancellation and closed
peer behavior, phase/aggregate deadline precedence and typed adapter failures.
Worker execution was **ordinary loopback, not network isolation**. The parent
must run these tests under an actual egress-denying policy before marking 030 done.

Final worker source bindings (also saved in `target/task-030/source-bindings.json`):

| File | SHA-256 |
|---|---|
| `src/target_policy.rs` | `8c7b7902bc1bfcb415057199610c86f33d19a15c2d50c6b9ef05bc090b36bddc` |
| `src/probe_transport.rs` | `c4ba5b36ef1b4a83b9b12cdfdb6025aec54d544da6fa4d2228c88dfc343d4158` |
| `tests/target_policy.rs` | `a2dc124551bf52d9e1482280c3e1d1a38975b13a4a4066ee2a4c4e875c606208` |
| `tests/probe_transport.rs` | `c1388f8060286f3479062418b689ad352d7d7fee4c2b5d936413e920a8cc9279` |

Paths in this table are relative to `crates/pqc_scan_core/`. Later changes require
new bindings and cannot reuse this final-source claim unchanged.

## Remaining acceptance

Real DNS helper failure/timeout/cancel/kill-and-wait/output-limit tests, isolated
execution, independent review, default-static dependency evidence, workspace
build/test/fmt/clippy, current source/self-scan bindings and parent integration
remain separate checks. Other OS/architectures, hosted CI and Internet endpoint
tests were not executed. No commit, push, PR, package publication or release was
performed by this worker.

## Primary references checked

- [IANA IPv4 special-purpose registry](https://www.iana.org/assignments/iana-ipv4-special-registry/)
- [IANA IPv6 special-purpose registry](https://www.iana.org/assignments/iana-ipv6-special-registry/)
- [idna 1.1.0 strict conversion API](https://docs.rs/idna/1.1.0/idna/fn.domain_to_ascii_strict.html)
- [socket2 0.6.4 Socket API](https://docs.rs/socket2/0.6.4/socket2/struct.Socket.html)
