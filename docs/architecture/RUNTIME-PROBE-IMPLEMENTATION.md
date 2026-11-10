# Initial TLS and SSH probe implementation contract

Status: **selected under explicit maintainer delegation**, 2026-11-10.
After reviewing the adoption questions, the maintainer requested that the agent
select the approach and continue. Adopt the conditional backend, unchanged static
contracts, separate runtimev1 envelopes, and bounded SSH discovery specified below.
Task030 safety is approved and implemented; task031 compiled evidence and its
independent source review support this choice. Publication and negotiated SSH
remain outside this authorization.

## Why a separate versioned runtime envelope

Current inventory v1 has static file targets and a static profile. The domain-model
document's endpoint/ObservationBatch examples remain proposals, not available Rust
APIs. Encoding endpoints as file paths or silently extending a strict v1 schema
would misrepresent scope or break consumers.

| Approach | Trade-off |
|---|---|
| Separate runtime v1 envelope, recommended | Typed endpoint facts with a new, explicit schema; static inventory and six legacy outputs stay byte-compatible |
| New combined inventory schema version | A larger migration and identity/coverage design; unnecessary before a real combined consumer exists |
| Human text only | Smallest initial interface, but loses the requested structured runtime evidence and reproducible semantic checks |

The recommendation adds thin protocol observation types in core and optional
protocol crates. It does not extract another inventory/model framework, duplicate
static inventory builders, or add a service/database/history system. Certificate
parsing reuses existing typed public-certificate helpers through a narrow parse-only
entrypoint. Core imports no TLS/SSH provider.

## Backend and feature proposal

Adopt pinned rustls0.23.46 / AWS-LC1.18.0 as a conditional **macOS aarch64** initial
TLS implementation candidate. Enable rustls `std`, `aws_lc_rs`, `tls12` explicitly;
do not enable logging, FIPS, early data, key logging or an alternate provider.
Retest the product feature union: rcgen in the development study added provider
defaults, so the study is not proof of a minimal production dependency graph.

The default CLI remains a static scanner. CLI `probe-tls` and `probe-ssh` features
activate their optional protocol crate and shared `probe-transport` helper. New
workspace members themselves default to protocol runtime disabled, so a default
workspace build does not require TLS provider/native dependencies. Run default,
each protocol and combined feature matrices explicitly. Linux/Windows support is
unknown until separately built/run; this is not a new cross-platform release claim.

The initial TLS trust source is pinned local `webpki-roots`1.0.11; no root or
revocation network retrieval. Optional `--ca-file PATH` replaces that bundle with
explicit local public CA certificates. Bound custom input to 256 KiB/16 certificates,
reject non-certificate PEM blocks, and never read passwords/client private keys.
Record the trust-source kind and public root-set digest, not private local paths.
OS trust-store integration is deferred. Ordinary chain/name/time validation is
mandatory; no insecure switch or custom cryptographic implementation.
The [bundle documentation](https://docs.rs/webpki-roots/1.0.11/webpki_roots/)
distinguishes its Mozilla roots from platform trust constraints/revocation data;
record this limitation and update the bundle through reviewed releases.

## Commands and output contract

```text
pqc-scan probe tls HOST --port PORT [--allow-cidr CIDR]... [--server-name NAME] [--ca-file PATH] [--out-dir DIR] [--lang en|ja]
pqc-scan probe ssh HOST --port PORT [--allow-cidr CIDR]... [--out-dir DIR] [--lang en|ja]
```

Port is mandatory. Host is one explicit endpoint, never a URL, source-file hint or
discovery range. Default directories are `./pqc-probe/tls` and `./pqc-probe/ssh`;
recommend fresh directories for separate runs. Each command writes its machine
document plus `report.html` and `report.md`. These commands do not write or replace
the six static legacy outputs. `--lang` changes only human text.

| Command | New document | Offline schema |
|---|---|---|
| TLS | `probe.tls.v1.json` | `schemas/probe-tls-v1.schema.json` |
| SSH discovery | `probe.ssh-offers.v1.json` | `schemas/probe-ssh-offers-v1.schema.json` |

Common fields: schema version1, scanner version, generation time, protocol/method,
requested normalized host/port, checked answer set, pinned/actual peer when known,
explicit client profile, attempt1 scoped to this invocation, configured limits,
actual protocol-socket sent/received counts, typed outcome and safe diagnostic codes.
Socket counters exclude OS DNS service traffic and TCP/IP overhead/retransmissions.
The helper's operation is deadline-bound and reaped; already-initiated OS resolver
service work is outside the process's cancellation/accounting boundary. Outcomes
separate completed, partial and failed inspection; collection failure retains
only safe facts. Do not export opaque error text, secret bytes or traffic dumps.

Exit0 means the requested observation completed; exit1 means configuration/output
failure; exit3 means denied, failed, partial, cancelled or locally unsupported
inspection. No policy threshold is introduced. Existing scan 0/1/2/3 is unchanged.
Unknown algorithm classification is not global remote absence or a security verdict.

## TLS facts and success gate

One fresh default-secure TLS1.2/1.3 handshake per invocation, with the exact configured
version/suite/group profile recorded. No comparison retry, resumption, 0-RTT,
client certificate, application write, HTTP request, key log, proxy or redirect.
TLS identity/SNI is separate from the pinned socket; `--server-name` changes the
verification identity and DNS SNI, never destination or address authorization.
The effective DNS verification name is also the sent SNI. An IP verification
identity uses IP certificate validation and sends no SNI. Export both
`verification_name` and optional `sent_sni`, including an explicit override, so the
tested identity can be reproduced independently of the requested destination.

Publish verified negotiated facts only after ordinary verifier success, completed
handshake and a Full/FullWithHelloRetryRequest kind. Task031 proved that group
accessors can be Some after verifier failure; those values must not be promoted
to a verified session. On failure, verified negotiated fields remain absent.

Record exact negotiated version/suite/group IDs and names, unknown group state,
bounded public certificate/SPKI fingerprints and parsed signature/key OIDs,
available semantic key parameters/validity, and explicit verification/revocation
state. No CertificateVerify scheme is promised. TLS1.3 suite names never imply KX.
Hybrid classification uses exact reviewed group identifiers, not substrings.
Separate classical certificate authentication from hybrid key establishment.

The existing 16-certificate/256 KiB collection limits and 512/64 KiB transport
limits apply. The pinned library's prior approximate64 KiB message limit may fail
earlier; report it as a local resource limit/incomplete inspection, never a server
security finding. Missing collection facts and unverified presented certificates
must stay explicitly unknown/unverified, without assuming failure exposes a chain.

## SSH advertised-offer contract

Implement only bounded SSH2.0 identification and initial unencrypted packet/KEXINIT
framing. Preserve all ten ordered name lists and separate extension markers from
algorithms. Record exact wire identifiers/provenance and an **unauthenticated
advertisement** assurance state. There is no negotiated method, presented/verified
host key, KEX result, username, service/userauth/channel request or remote command.

Send the diagnostic banner once after TCP connection, then send one KEXINIT
immediately after validating the peer identification, before waiting for its
KEXINIT. This also handles peers waiting for the client. Use an OS-random cookie
and random padding. The profile is a diagnostic proposal, not a claim that this
collector implements those cryptographic algorithms. Close immediately after
validated offer collection; never execute the next key-exchange packet.

Fixed initial client name lists (ordered):

| KEXINIT field | Diagnostic proposal |
|---|---|
| kex_algorithms | `mlkem768x25519-sha256`, `sntrup761x25519-sha512`, `sntrup761x25519-sha512@openssh.com`, `curve25519-sha256`, `curve25519-sha256@libssh.org` |
| server_host_key_algorithms | `ssh-ed25519`, `ecdsa-sha2-nistp256`, `rsa-sha2-512`, `rsa-sha2-256` |
| encryption_algorithms_client_to_server | `chacha20-poly1305@openssh.com`, `aes256-gcm@openssh.com`, `aes128-gcm@openssh.com`, `aes256-ctr`, `aes128-ctr` |
| encryption_algorithms_server_to_client | Same ordered list as client_to_server |
| mac_algorithms_client_to_server | `hmac-sha2-256-etm@openssh.com`, `hmac-sha2-512-etm@openssh.com`, `hmac-sha2-256` |
| mac_algorithms_server_to_client | Same ordered list as client_to_server |
| compression_algorithms_client_to_server | `none` |
| compression_algorithms_server_to_client | `none` |
| languages_client_to_server | Empty |
| languages_server_to_client | Empty |

Send `first_kex_packet_follows=false` and reserved0. Do not add extension markers
or an additional KEX message. Record this exact configured diagnostic profile;
never label it as implemented cryptographic support or record its first common
method as negotiated.

Enforce task030's 8 KiB pre-identification/256 KiB framed-packet ceilings before
allocation/read, protocol identification-line limits, overflow/padding/alignment
checks, boolean/reserved fields and bounded ASCII name lists. Interpret every
nonzero boolean as true, as required by [RFC4251](https://www.rfc-editor.org/rfc/rfc4251.html#section-5),
without treating first-kex-packet-follows as permission to execute it. Maximum64 preliminary
lines/messages, maximum256 names per list and maximum64 bytes per algorithm/method
name ([RFC4251 section6](https://www.rfc-editor.org/rfc/rfc4251.html#section-6)) are proposed
additional parser bounds; exceeding them is incomplete inspection, never truncation
into a complete result. The64 limits are separate: at most64 pre-identification
lines and at most64 early IGNORE/DEBUG packets before KEXINIT. The8 KiB banner
budget includes all received banner bytes through the identification terminator;
the identification line is at most255 bytes including CRLF. The256 KiB frame cap
means the entire `4 + packet_length` frame, checked before allocation.
DISCONNECT is an immediate terminal result, never an ignored packet to continue
past. No
decompression, rekey, encryption, DH/KEM/signature implementation or system SSH backend.

## Required proof before acceptance

Use task030 authorization, nonblocking metered transport, cancellation/idle shutdown
and fixed phase/aggregate deadlines. No alternate DNS resolution, implicit retry,
proxy environment, SSH config, manifest/SAN-derived target or certificate URL fetch.
Signal cancellation must reach the token, close sockets and reap a resolver child.

TLS: actual classical/hybrid/HRR facts, verifier negatives, missing/unknown group,
large chains, local limits, alerts/slow peers, no application/clientauth/resumption,
product dependency graph and default-static isolation. SSH: fragmentation/coalescing,
malformed/oversized input, unknown names/markers, waiting-for-client KEXINIT,
classical/hybrid local server profiles and zero service/auth/channel message types.
Use offline schemas plus reference/semantic/privacy assertions, deterministic
fixtures and English/Japanese fact binding. Runtime peers must be network-isolated
loopback fixtures with ephemeral test keys, never public targets. Browser acceptance
remains a separate gate; task036's current automation restriction does not disappear.

031's measured backend evidence is in
[TASK-031-TLS-FEASIBILITY.md](../quality/TASK-031-TLS-FEASIBILITY.md).
No negotiated SSH mode, release channel, numeric quality budget, publication or
task039 use case is selected by these proposals.
