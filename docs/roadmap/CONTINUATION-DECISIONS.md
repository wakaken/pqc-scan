# Tasks 030–039: implementation decision record

Status: **030 option A and 034/035 contracts approved by maintainer**, 2026-11-10.
The maintainer selected both recommendations in the continuation conversation.
This initial approval record is historical. The user subsequently delegated
technical selection and continuation; see the selected
[runtime contract](../architecture/RUNTIME-PROBE-IMPLEMENTATION.md) and
[local validation](../quality/RUNTIME-INTEGRATION-VALIDATION.md). Neither this
record nor that delegation authorizes publication or declares all tasks complete.
Continuation baseline: `3c357cef879f69ae4058fefe7a03b4f074283b3f`, branch
`codex/inspection-format-support`, clean worktree at entry. Preserve that local
commit. No push, PR, merge, package publication or signing setup is authorized.

## 030: concrete safety proposal for review

Keep `scan` entirely local. Reserve separate optional commands:

```text
pqc-scan probe tls HOST --port PORT [--allow-cidr CIDR]...
pqc-scan probe ssh HOST --port PORT [--allow-cidr CIDR]...
```

These are proposed interfaces for 032/033, not executable commands added by 030.
TLS identity override, local trust roots and protocol output options must be
specified with the TLS implementation after the backend study. There is no
generic connect-only product command. Port is mandatory, one host per invocation.

### Alternatives

| Choice | Behavior | Trade-off |
|---|---|---|
| A: public unicast default (recommended, original design) | Public unicast may connect; other unicast requires explicit CIDRs | Simple public-endpoint use; requires complete special-purpose address classification |
| B: explicit CIDRs for every address | No address is authorized without a matching CIDR | Narrower authorization; DNS users must authorize the actual IP ranges |

Maintainer selection: **A**, including the command shape and ceilings below.

Both choices permanently reject unspecified, multicast and broadcast destinations;
neither a hostname nor a broad CIDR bypasses these checks. Review special-use
IPv4/IPv6 ranges, IPv4-mapped IPv6, IDNA, scoped IPv6 rejection and mixed-answer
handling explicitly; do not infer public unicast from `!is_private()` alone.
Reject the entire answer set if any answer is denied or if the count exceeds the
limit. Normalize before CIDR matching. Pin one validated `SocketAddr`, verify its
actual peer, and retain TLS identity separately. No proxy/SSH configuration,
redirect, alternate endpoint expansion, retry or second DNS resolution.

### Proposed reviewed ceilings

| Resource | Default and hard ceiling for initial release |
|---|---|
| Targets / active sockets | 1 / 1 |
| DNS answers | 8 (excess is rejection, never truncation) |
| Connection attempts | 1 initially; no comparison profiles in the initial implementation |
| DNS / TCP / protocol / aggregate wall time | 5 s / 3 s / 5 s / 15 s |
| Received / sent bytes | 512 KiB / 64 KiB per target at the socket boundary |
| TLS certificate chain | 16 elements, 256 KiB aggregate DER |
| SSH pre-identification / framed packet | 8 KiB / 256 KiB |

Limits may be reduced but cannot be increased above these ceilings. The original
design's possible three explicit comparison attempts are deferred; enabling them
requires shared-budget tests and a separate decision. Protocol-specific ceilings
apply in 032/033; 030 owns finite target/transport accounting. DNS answers include
duplicates for bounding work before normalization/deduplication.

Proposed new-command exits: 0 means the requested observation completed, 1 means
setup/output failure, 3 means denied, failed, partial, cancelled or unsupported
inspection. No new policy-threshold option is proposed initially. Existing scan
exits 0/1/2/3 and their precedence remain unchanged.

### Builder constraints and proof obligations

Core policy and counters must not import TLS/SSH providers. A pinned endpoint
cannot be constructed by bypassing policy. Socket I/O wrappers debit every
successful read/write, bound caller buffer lengths before I/O, and use the
minimum of the phase deadline and aggregate deadline for each operation. Phase
deadlines are absolute from phase entry; successful reads, partial writes and
interrupted I/O cannot reset deadlines or budgets. Cancellation must close sockets
and release resources.

System `getaddrinfo` in an abandoned worker thread is not proof of cancellable
DNS. Before selecting a resolver implementation, demonstrate bounded answer
collection, deadline enforcement and cleanup, or record a blocker. Do not claim
an I/O timeout proves interruption of an underlying synchronous resolver.

Tests must use mocked resolution/dialing and network-isolated loopback peers with
egress denied. Assert mixed denied answers, mapped IPv6, peer mismatch, no
re-resolution, zero implicit retries, exact-byte boundaries, slow reads/writes,
cancellation and cleanup. Report the actual isolation mechanism and platforms;
loopback binding alone is not network isolation. Platform support follows evidence.

## Independent static work

### 034: proposed export contract (approval required)

Add `scan --cyclonedx-17`, independent of `--format all`, producing only the
additional `cbom.cdx-1.7.json` and `dependency-sbom.cdx-1.7.json`. Capture existing
native observations internally as required; leave the six legacy outputs,
inventory schema and existing flags unchanged. Pin official CycloneDX 1.7 schemas,
references, digest and license for offline validation. Report conversion losses;
do not turn lexical algorithm references into runtime protocol/keys/certificates,
declared dependency constraints into installed versions, or unknowns into strength
or certification claims. This new format remains gated by AGENTS storage/output
review before implementation.

### 035: proposed comparison contract (approval required)

Add `pqc-scan diff BEFORE AFTER --repository-scope ID --out-dir DIR`, accepting
version-compatible inventory documents with exactly that explicit scope in both.
Reject missing/mismatched scopes or incompatible identity schemes; never infer
identity from an absolute checkout path or legacy finding ID. Write separate
`scan.diff.v1.json` and a human summary; this is a new format requiring review.

Use exact asset/occurrence keys; compare semantic content excluding documented
run/time fields. Report rule/profile/scope drift. Changed duplicates and ambiguous
anchors remain unresolved. An absent record is removal from the compared
observation set only when coverage can establish the target was inspected;
otherwise unresolved. Removal never means cryptographic risk resolution. Preserve
move/similarity pairs as explicit heuristic candidates with no identity merge or
resolved claim. Builder rule: propose a move only for a unique one-to-one pair
with the same semantic algorithm/evidence anchor and a changed path; duplicates
or multiple candidates remain unresolved. No fuzzy text scoring or automatic
rename is introduced. If inventory v1 lacks the target-level proof needed, fail closed
to unresolved rather than changing its schema implicitly.

### 036: authorized existing-report extension

Use existing `ScanDocument`, local `AssessmentReport`, catalogs and renderers.
Display scope-limited inspection status/counters and optional advisory policy
decisions in English/Japanese human reports. Preserve machine outputs, source
facts, identifiers, custom fallback and complete occurrence/action associations.
Existing assurance and usage states are presented as separate facts; static
references/settings/parsed public data never become runtime or trust verification.
No runtime sections until probes exist. Do not add new report formats, CLI flags,
translation services or UI framework. Establish minimal rendering interfaces from
current code before implementation; test fact binding, escaping and language
parity, then actual offline browser/keyboard behavior.

## Later decisions (not prerequisites for independent static work)

| Task | Required decision/evidence | Next action |
|---|---|---|
| 031 | After 030, compiled pinned TLS/provider/API/platform study | Approve/reject backend from actual local isolated harness results |
| 032 | 030 safety decision plus 031 backend decision | Implement only verified fresh TLS observations |
| 033 | 030 safety decision plus parser/cap review | Discovery only; zero KEX execution/service/userauth/channel requests |
| 038 | Dependencies 032–036; release channel, OS/arch, support window | Inventory channels, propose packaging before declaring support |
| 037 | 038 first; baseline-derived regression budgets/release scope | Keep measured results separate from approved thresholds |
| 039 | After 033, maintainer-selected concrete negotiated-session use case | Defer study until selected; no negotiated SSH mode is authorized |

The presence of tasks is not approval of their decision gates. Keep statuses
truthful and task-local commits separate. Existing measured results are historical
evidence, not validation of this continuation or of hosted CI.

## Subsequent delegated selection and actual checkpoint

The maintainer requested agent selection and continued implementation after the
backend questions. The tested rustls/AWS-LC backend, bundled roots with explicit
custom-CA replacement, separate runtime-v1 JSON and bounded SSH discovery were
selected and implemented. New runtime storage is separate from unchanged static
inventory/legacy outputs. The local distribution scope is source installation
and unpublished macOS arm64 static/combined archives, with best-effort support
for the latest verified development snapshot; no stable 1.0/SLA/backport promise.

Task031 is done. Task032/033 source/protocol and common integration reviews are
accepted, but new human reports still require actual browser acceptance. Task038
clean-source repeatability/offline installs passed; notice and dependency gates
remain. Task037 does not approve task023 numeric budgets or declare 1.0 ready.
Task039 is deliberately not selected: the current advertisement-only use case
does not justify completed KEX or additional SSH cryptographic libraries. This
is deferral, not feasibility success or rejection of untested candidates.

## Local acceptance checkpoint — 2026-11-11

This checkpoint supersedes earlier browser/notice blockers. Source `75b9afd` and
source-bound evidence in [REMAINING-ROADMAP-VALIDATION.md](../quality/REMAINING-ROADMAP-VALIDATION.md)
passed one implementation review and one separate test-adequacy review. Actual
loopback HTTP browser checks now cover both languages and static/TLS/SSH pages;
032,033,036 are done. 038 technical preparation is done: both local macOS arm64
variants repeat byte-identically and clean-install offline. Static missing notice
sets were completed; four combined-variant upstream license statements still
require publication review and do not constitute legal clearance.

037 remains blocked on approved budgets/1.0 support scope and actual current-HEAD
hosted CI. The concrete existing-baseline proposal and measurements are in the
validation document; no approval is inferred from silence. 039 remains conditional
pending, without a selected completed-KEX consumer. No push, publication, signing
or stable multi-platform support claim was made.

## Maintainer acceptance decision — 2026-11-11

The maintainer explicitly approved the proposed regression signals and 1.0 scope.
For unchanged eligible truth, per split/family TP must not decrease and FP/FN must
not increase. On the same host/profile/config/input, wall time or peak RSS above
twice the previous observed warm maximum requires investigation. Existing local
measurements are within these signals; this does not establish a universal accuracy
or performance guarantee or silently enable threshold enforcement in CI.

Approved scope: static scanning is the formal 1.0 target; TLS/SSH probes remain
experimental; local distribution validation covers macOS arm64. This approval
supersedes the earlier pending-budget/scope statements, without declaring 1.0
released or promising other platforms. 037 remains blocked on successful hosted
CI for the actual target commit. Four combined-probes notice reviews remain
publication prerequisites. Push, publication and signing were not authorized.
