# Task 030: explicit target policy and bounded transport

**Goal:** authorize and pin a single endpoint without protocol dependencies.
**Spec:** `docs/records/v0.2-roadmap/CONTINUATION-DECISIONS.md` (maintainer-approved A).
**Baseline:** `3c357cef879f69ae4058fefe7a03b4f074283b3f`.

## Constraints

- Keep static scans offline, existing machine formats and exits unchanged.
- No TLS/SSH provider, proxy/config expansion, retries, discovery or authentication.
- DNS 8 answers; 1 attempt/socket; phase 5/3/5 seconds; aggregate 15 seconds;
  receive/send 512/64 KiB; smaller nonzero limits only.
- All answers authorized before choosing one; normalize mapped IPv6; pin and check peer.
- Cancellation/deadlines remain absolute; no abandoned DNS worker thread.
- Parent integrates `core/lib.rs`, Cargo and optional CLI helper. This worker owns
  only new core modules/tests, this plan, task 030 and task-local quality evidence.

## Implementation and verification

- [x] Add meaningful table-driven policy tests first and verify expected failure.
- [x] Implement strict host/IDNA/CIDR parsing, special-address classification,
  private-field authorized target and bounded resolver contract.
- [x] Add deadline/attempt/traffic/cancellation tests first; verify failure.
- [x] Implement validated budgets and a socket wrapper that cannot expose raw
  streams or reset phase deadlines; verify peer and debit exact successful I/O.
- [ ] Parent integrates kill-and-wait DNS helper adapter behind optional feature;
  no core implementation invokes an arbitrary resolver automatically.
- [ ] Run targeted tests and record commands/exits and unexecuted isolation/OS
  acceptance. Independent review and Evaluator determine done; worker never does.

## Review focus

Mapped addresses and mapped CIDRs must share one canonical address family;
broad allowlists never permit unspecified/multicast/broadcast; mixed DNS answers
reject in full; duplicate answers count against 8 before deduplication; repeated
partial I/O does not restart either deadline; cancellation closes underlying
connections. Native system resolution cannot be declared cancellable merely
because a calling thread times out.
