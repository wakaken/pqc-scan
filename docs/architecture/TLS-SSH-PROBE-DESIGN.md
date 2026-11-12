# Explicit TLS and SSH runtime diagnostics

Design date: 2026-11-09. Status: original proposal; at that checkpoint no runtime
probe, remote scan or compiled library spike existed. See the [standards register](../product/STANDARDS-REGISTER.md) for versions, dates and sources.

Continuation checkpoint (2026-11-10): task030's safety foundation is implemented
under its approved one-attempt policy; task031's
[pinned isolated study](../quality/TASK-031-TLS-FEASIBILITY.md) has passed independent
source review. Its backend/features/platform adoption and the
[concrete product contract](RUNTIME-PROBE-IMPLEMENTATION.md) remain maintainer gates.
No product TLS/SSH command or public endpoint scan is claimed. In particular,
the study found that a group accessor may be Some after verifier failure: only
successful verification plus a completed fresh handshake may yield verified facts.
The original three-attempt/profile proposal below is superseded by the approved
initial ceiling of one; it is not permission to add comparisons or retries.

## Entry points and scope

Proposed commands are `pqc-scan probe tls HOST --port PORT` and `pqc-scan probe ssh HOST --port PORT`. Port is mandatory in the initial design to keep scope explicit. These commands are new features, independent of `scan`. Static input files, manifests, URLs, SAN entries and recommendations never cause a connection. Initial commands accept one host/port; bulk target input is deferred.

No user authentication, passwords, private keys, agent sockets, channels, remote commands, file transfer, configuration changes, HTTP requests or application payloads. TLS/SSH diagnostic traffic itself is explicit network activity and can be logged or rate-limited by the peer. Commands report partial/failed results rather than declaring server-wide compliance.

Use candidate crates `pqc_scan_probe_tls` and `pqc_scan_probe_ssh` only when their implementation tasks start. Their public APIs return core `ObservationBatch`; static detectors and core never import protocol libraries. Compile features isolate networking/native build dependencies from the default static product. No common async framework is required merely to connect a single endpoint.

## Target authorization and connection binding

1. Parse a host and port, not an arbitrary URL. Reject userinfo, paths, wildcard/network ranges, embedded commands, control characters and out-of-range ports. Normalize DNS/IDNA with a reviewed library; parse bracketed IPv6 unambiguously. `--server-name` is an explicit TLS identity override, never another destination.
2. A typed target policy checks the supplied destination and all bounded A/AAAA answers. Default proposed policy permits public unicast only. Private/loopback/link-local destinations require explicit `--allow-cidr` scope (offline fixtures use loopback permission). Block unspecified, multicast and broadcast classes regardless. Link-local metadata ranges receive no implicit exemption. The authorized DNS name alone does not authorize every future IP.
3. Resolve once per target invocation under a deadline; record answer set and checked addresses. Fail closed on any disallowed answer or excessive answer count. Normalize IPv4-mapped IPv6 before CIDR matching. IDNA/IP zone and special-use address handling need table-driven tests.
4. Pin the validated `SocketAddr`; call `connect` on that address, never re-resolve through a TLS/SSH library hostname API. Compare the socket peer address to the selected IP/port. If another resolved address is attempted, validate it and debit the same target budget. Retain DNS name for SNI/identity verification independently.
5. Ignore proxy environment variables, SSH config, ProxyCommand, redirects and alternate endpoint hints. TLS and SSH have no HTTP redirect step in this design. A redirect-like banner is data, not an instruction. Certificate AIA/CRL URLs must not trigger network retrieval. No remote DNS-over-HTTP service is introduced; use configured system resolution and record its scope.
6. No automatic re-resolution, address expansion, port fallback or retry. A new explicit invocation establishes fresh authorization. This prevents a DNS rebind from changing an already pinned connection; it does not prove DNS ownership or replace peer identity checks.

The local CLI operator may intentionally authorize internal networks. This is not a future enterprise SSRF policy: a hosted wrapper must enforce a stronger organization/tenant authorization policy separately. The OSS command does not implement an authorization server.

## Proposed initial safety budgets

These are conservative engineering starting points, not measured performance/accuracy goals. The library spike must prove enforcement, and maintainers must review defaults before shipping. Exceeding any budget produces a diagnostic, never an unsupported-algorithm conclusion.

| Budget | Initial proposal | Enforcement / qualification |
|---|---|---|
| Targets / active connections | One / one per invocation | No implicit bulk scan or address fan-out |
| DNS answers | At most eight | Reject excess or mixed-disallowed results; no silent truncation |
| Attempts | One by default; at most three total for explicit comparison profiles | Includes failed address attempts; no retries; every attempt gets an ID |
| Time | DNS 5 s, TCP 3 s, handshake/discovery 5 s, overall target 15 s | Overall deadline wins; interrupt blocked I/O and cancel remaining work |
| Traffic | 512 KiB received and 64 KiB sent per target across all attempts | Count at socket boundary; close on limit, retain safe partial observations |
| TLS certificates | 256 KiB aggregate certificate data, at most sixteen chain elements | Record collection limit; larger PQC chains are partial, not unsafe |
| SSH banner | 8 KiB pre-identification aggregate; identification line within protocol bound | Bound line count/length, reject invalid encoding for names; escape display |
| SSH packet | 256 KiB maximum framed packet, bounded allocations before reading body | Discovery accepts only the required early protocol messages; no decompression |

Configurable lower limits are useful. Increasing above reviewed hard caps requires a documented local option and still finite bounds. Future explicit target lists must separately define max targets, global concurrency, rate and total traffic before release; they are not part of the initial PR.

## Error and completeness model

Use stable diagnostic codes for invalid target, denied address, DNS failure/timeout, connect refused/unreachable/timeout, protocol mismatch, malformed peer data, TLS alert, certificate/identity validation failure, local provider unsupported, peer rejection/no overlap, unsupported inspection, resource limit, cancellation and partial success. Preserve a sanitized reason and attempt context, not opaque dumps of peer traffic.

TLS alerts and SSH negotiation failures can have multiple causes. `no_overlap` applies to the tested client profile, not every capability of the server. An unknown algorithm identifier is preserved as unknown. A cap or timeout is incomplete inspection. One failed PQC-only handshake never establishes global PQC absence. `Pass/Fail/Indeterminate` policy evaluation must account for these distinctions.

Default probe status reports its outcome and returns a nonzero operational status for a failed/partial requested inspection; finalize the new-command exit contract in task 030 without changing existing `scan` codes. The detailed scan design proposes 1 for fatal setup/output errors, 3 for requested completeness failure and 2 for policy threshold, with that precedence. Successful SSH offer discovery means discovery completed, not that KEX/authentication completed.

## TLS observations and API feasibility

Initial implementation uses a fresh handshake with no resumption, 0-RTT, client certificates, key logging or application writes. Trust roots and optional custom CAs are local inputs. Retain standard certificate signature and identity validation; initial releases do not provide a global insecure-verification switch. A failed verifier may yield bounded presented-certificate metadata, but must label it unverified and the handshake incomplete. No promise that every failure exposes a certificate chain.

| Requested fact | Source / API feasibility | Reporting rule |
|---|---|---|
| Negotiated TLS version | rustls `CommonState::protocol_version()` | Present only when available; distinguish offered client profile |
| Negotiated suite | `negotiated_cipher_suite()` | Exact suite ID/name; TLS 1.3 suite does not identify KEX/authentication |
| Actual KX group | `negotiated_key_exchange_group()`, then group identifier | Optional value; read after completed full handshake. `None` stays unknown, not classical |
| PQC / hybrid group | Exact group registry plus provider capability | Match standardized codepoint/name, not substrings or TLS suite names |
| Peer certificates | `peer_certificates()` and bounded X.509 parsing | Signature OID, SPKI algorithm/parameters, semantic key size, validity, fingerprint; do not confuse certificate signature with handshake signature |
| Certificate trust | Standard verifier outcome, name, time and local root set | Separate signature validity, chain/name validation and revocation status. Revocation checking is not claimed without explicit local evidence |
| TLS CertificateVerify scheme | Not a general accessor on the reviewed CommonState API | Defer initial collection; a later wrapper may record verified `DigitallySignedStruct.scheme` in the verifier path only after a tested design |
| All server versions/groups | Not available from one ordinary client handshake or these public accessors | Record tested profiles and observed selections; never invent an exhaustive offer list |

The reviewed docs displayed rustls 0.23.46. Public state exposes protocol/suite/KX/certificates; it does not expose a complete server capability catalog. The KX accessor may return None during handshaking or certain resumed sessions. Exact compile feasibility under pinned Rust 1.93.1 remains untested. Sources: [CommonState](https://docs.rs/rustls/latest/rustls/struct.CommonState.html), [provider groups](https://docs.rs/rustls/latest/rustls/crypto/aws_lc_rs/kx_group/index.html).

RFC 10024 (August 2026) defines X25519MLKEM768, SecP256r1MLKEM768 and SecP384r1MLKEM1024 for TLS 1.3. Preserve the numeric NamedGroup and registry edition. Old Kyber/draft IDs are separate. A provider need not implement every RFC group; the reviewed AWS-LC rustls group module lists the first two, not proof of all three. A selected group is evidence for that session only. A classical certificate with hybrid KEX still has classical authentication exposure. [RFC 10024](https://www.rfc-editor.org/rfc/rfc10024.html)

Default profile: one secure provider-supported full handshake, recording exactly offered client configuration and negotiated facts. Optional comparison profiles (at most three total attempts) may use baseline, TLS-1.3-only and a supported standardized-hybrid-only configuration. A provider with only TLS 1.2/1.3 cannot establish a server's complete TLS 1.0/1.1 support; mark those tests unsupported locally. Never bypass authentication to obtain a positive result. Client inability to verify a PQC certificate is a local limitation, not server absence of PQC.

### TLS library choice

| Candidate | Advantages | Costs / limits | Decision |
|---|---|---|---|
| rustls with AWS-LC provider | Public negotiated-group API; standardized hybrid candidates; no application HTTP client needed | Native provider build/target/MSRV checks; limits on schemes/groups; certificate verifier support can differ from KEX | Recommended conditional candidate; task 031 spike must pin features/versions and demonstrate facts locally |
| aws-lc-rs alone | Crypto provider primitives | Not a TLS state machine; implementing TLS would create unnecessary cryptographic risk | Use through rustls; reject building a TLS protocol stack around it |
| Existing rustls alternate provider | Could reduce native build constraints | PQC and group support varies by provider/version; cannot promise hybrid parity | Keep static product usable; unsupported probe capability is explicit |
| OpenSSL command or FFI | Broad ecosystem, potential independent test oracle | External installation/provider variation; subprocess-output parsing and verifier consistency | Optional offline oracle, not initial runtime dependency or automatic fallback |

Use existing `x509-parser`/`pem` certificate helpers after task 020 corrects them; narrowly expose parser-only helpers from detectors for TLS reuse if necessary. This is an **existing implementation extension**, not extraction of a new certificate framework. TLS crate/command is a **new feature addition**. Do not claim FIPS validation from an AWS-LC feature name.

## SSH discovery and later negotiation

Phase 3 initially stops at unauthenticated algorithm discovery. Exchange SSH 2.0 identification, receive/send the minimal required transport packets to obtain the server `SSH_MSG_KEXINIT`, parse its ten name lists and flags, retain ordered KEX and server-host-key candidates (and optionally directional cipher/MAC/compression lists), then disconnect. No `ssh-userauth` service request, `none` authentication, usernames, agent access or channel creation is sent.

Treat KEX extension markers such as `ext-info-*` / strict-KEX markers separately from algorithms. Parse length-prefixed lists with integer overflow/length/ASCII/list-count validation, packet/padding checks and a state machine that tolerates only bounded pre-KEX messages. The identification/banner and KEXINIT are untrusted peer data. Do not implement DH/KEM, signatures, encryption, rekey or general SSH transport in this discovery parser.

If a server waits for client KEXINIT, send a protocol-valid discovery profile with bounded name lists and an OS-random cookie, explicitly identified as a diagnostic client. Stop before key exchange execution and never report the proposed common method as negotiated. Packet framing and cookie generation must be reviewed against RFC 4253; do not turn this narrow parser into a homegrown cryptographic implementation.

Recognize exact identifiers such as `mlkem768x25519-sha256` (RFC 10042, Informational, August 2026) and OpenSSH's sntrup hybrid names. Retain standardized/RFC-defined, vendor-namespaced, draft and unknown provenance separately. Host-key candidate names do not establish a presented public key, verified signature or negotiated method. OpenSSH release numbers in banners are claims, not capability proof.

Current OpenSSH releases include evolving composite PQ signatures; do not freeze a claim that host authentication is always classical. Keep exact wire names and their source status. First-release recognition is an inventory classification, not implementation of those signatures. [OpenSSH notes](https://www.openssh.org/releasenotes.html), [RFC 10042](https://www.rfc-editor.org/rfc/rfc10042.html)

| Candidate | Discovery/negotiation suitability | Decision |
|---|---|---|
| Small bounded banner/KEXINIT parser | Direct remote offers, no cryptographic operations, can stop before auth; must implement/review framing and limits | Preferred initial discovery approach, conditional on parser review and fuzz fixtures |
| russh (reviewed docs: 0.65.1) | Full async SSH transport; potential future authenticated KEX. Public remote-list visibility and stop-before-userauth must be demonstrated, not assumed | Candidate for a separate negotiation spike; do not add Tokio/full SSH only to read offers |
| ssh2/libssh2 (reviewed docs: 0.9.7 binding) | `methods()` reports selected algorithms after handshake; `supported_algs()` is local capability; native/provider matrix varies | Possible future KEX backend, not proof of remote-offer visibility |
| System `ssh -vv` | Useful local test oracle | Config, ProxyCommand/agent side effects and unstable debug output | Never initial scanner backend; isolated oracle only with controlled config |

Future negotiated SSH observations require actual completed KEX and verified host signature, with trusted known-host/pin input and a no-auth stop point demonstrated by packet transcript. A merely computed first common algorithm is `candidate`, not `SshNegotiated`. This negotiated mode is excluded from initial Phase 3; roadmap task 039 is a feasibility record, not permission to ship it.

## Offline verification before release

Use a network-isolated test environment with synthetic public certificates, local CAs and ephemeral test-only server keys. Do not log keys, TLS secrets, full packet transcripts with payloads or ambient environment values. Tests may inspect sanitized message types/counters to prove no auth/application data was sent. No Internet endpoints are test fixtures.

- TLS: classical and supported hybrid full handshakes, HRR path, unsupported local group, certificate trust/name/expiry failures, unknown OIDs, resumption disabled, alerts, slow peer, oversized chain and traffic/time limits. Verify observed group directly, not through suite names. Record backend/build/platform versions.
- SSH: fixture byte streams for split/coalesced reads, truncated or oversized packets, banners, unknown names, duplicate names, extension markers, invalid lengths, disconnect/ignore messages, waiting-for-client-KEXINIT, and malicious extra messages. Local OpenSSH versions/configs advertise classical and hybrid options; no credentials or auth request.
- Network policy: mocked DNS and connected peer binding for public/private/mixed answers, IPv4-mapped IPv6, rebinding between attempts, SNI independent of IP, cancellations, closed socket cleanup, and budgets across all profiles.
- Determinism: compare semantic facts after excluding run time/IDs/ephemeral handshake bytes. Server randomness is not an inventory difference. Failure reproducibility means classified outcomes within bounded conditions, not byte-identical wire exchanges.

All defaults, capability claims and no-auth guarantees require evidence in [QUALITY-GATES.md](../quality/QUALITY-GATES.md). The [roadmap](../roadmap/OSS-ROADMAP-v0.2.md) schedules safety before protocol release. No enterprise management service is part of this design.

## Task039 selected use case and feasibility — 2026-11-11

The maintainer explicitly selected verification of actual hybrid KEX and prior-trust
host authentication without user authentication/login. The completed standalone
[task039 study](../quality/TASK-039-SSH-KEX-FEASIBILITY.md) selects russh0.65.1/AWS-LC
conditionally for a future negotiated mode. In isolated OpenSSH10.2p1 fixtures,
ML-KEM768+X25519 KEX completes with an exact pre-provisioned Ed25519 host-key pin;
wrong/unknown pins and signature tampering fail. The server decodes NEWKEYS then
DISCONNECT, with no service/userauth/channel request. ssh2's public handshake
sends a service request automatically and is unsuitable for the strict contract.

Initial future trust shape: an explicit public-key pin for the requested endpoint,
provided over a separate trusted channel. Do not fetch-and-trust a scanned key,
accept host certificates by embedded-key match, read ambient SSH state, or silently
fall back from requested hybrid KEX. Host-key authentication remains classical in
this study. Full known_hosts/CA validation,030 budget/cancellation integration and
adversarial/rekey/platform tests remain implementation prerequisites. This finding
does not add a product command or change current advertisement-only probe results.

## Explicit product integration — 2026-11-11

The maintainer subsequently requested product implementation. Experimental feature
`probe-ssh-kex` now exposes `probe ssh-kex HOST --port PORT --host-key-file FILE`.
The prior public Ed25519 key is validated before DNS and bound to that explicit
invocation's endpoint. Fixed ML-KEM768+X25519 KEX has no classical fallback; host
signature verification and prior pin agreement precede verified output. The
connection ends without user authentication/login. Known-host patterns, host
certificates and PQ signatures remain unsupported; no ambient trust/config is read.

The async backend uses metered nonblocking BoundedTcpStream operations, not a raw
socket escape hatch. Existing030 one-attempt, target policy, time/traffic and
cancellation budgets remain enforced. The runtime and socket are stopped before
return. Separate `probe.ssh-kex.v1.json` plus guarded bilingual human reports do
not alter existing TLS/SSH-offer contracts or make static scans access the network.
See [product validation](../quality/SSH-KEX-PRODUCT-VALIDATION.md) and bilingual
[usage](../user/PROBING.md). This supersedes the earlier study-only limitation for
the specifically authorized integration; it does not claim stable/hosted-CI or
all-platform release acceptance.
