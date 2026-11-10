# Explicit TLS and SSH diagnostics

[日本語](PROBING.ja.md) · [README](../../README.md)

Use probes only for a host/port you are authorized to inspect. A probe exchanges
protocol traffic with that endpoint. Repository `scan` and report generation stay
local; source files, URLs, certificate SANs and findings never start a probe.

## Install the optional commands

The default installation includes static scanning. Enable either or both optional
commands at build time using the pinned Rust toolchain:

```bash
cargo install --locked --path crates/pqc_scan_cli --features probe-tls,probe-ssh
```

The initially exercised protocol environment is macOS on Apple Silicon. Other
OS/architectures are not yet qualified. TLS adds a native cryptographic provider
build; static builds/tests do not require that provider. No AI service is used.

## TLS

```bash
pqc-scan probe tls service.example --port 443 --out-dir /path/to/new-tls-report
```

Replace the example host with your authorized endpoint. Port is mandatory. One
fresh TLS1.2/1.3 handshake records the actual version, suite, key-exchange group
and bounded public certificate facts. The configured client profile is included.
No application request, client credential, session resumption, 0-RTT or key log
is used. A TLS1.3 suite alone does not identify classical or hybrid exchange.

Trust uses a fixed local Mozilla root bundle. To use a local public CA instead:

```bash
pqc-scan probe tls 127.0.0.1 --port 8443 --allow-cidr 127.0.0.1/32 --server-name localhost --ca-file /path/to/public-ca.pem --out-dir /path/to/new-local-tls-report
```

`--ca-file` replaces the bundle; it accepts at most16 public certificate PEM
blocks/256 KiB, never private keys or passwords. Ordinary chain/name/time validation
is mandatory. The effective DNS verification name is also SNI; IP verification
sends no SNI. `--server-name` never changes the pinned destination. OS trust
constraints and revocation checking are not supplied by this bundled-root mode.

A failed verifier never yields verified negotiated fields, even if internal
accessors contain provisional values. Client-authentication requests are locally
unsupported. Chain collection is limited to16 certificates/256 KiB; the TLS library
may hit its smaller internal message limit first. Failures/limits are incomplete
inspection, not evidence that the server lacks PQC or is unsafe.

## SSH offers

```bash
pqc-scan probe ssh service.example --port 22 --out-dir /path/to/new-ssh-report
```

The collector exchanges identification and KEXINIT, records all ten ordered remote
name lists and extension markers, and closes before executing key exchange. It
sends no service/userauth/channel requests, including no `none` authentication.
There is no username, agent, private key, SSH configuration or system-SSH backend.
The diagnostic client proposal is recorded; it does not claim implemented crypto.

Results are **unauthenticated advertisements**. They do not prove a selected
method, verified host key, completed session or server-wide migration readiness.
A version banner is an unverified peer claim. Unknown identifiers remain unknown.

## Authorization, limits and outputs

Public unicast is allowed by default. Internal/loopback/special-purpose unicast
requires explicit `--allow-cidr`; unspecified, multicast and limited broadcast
addresses remain denied. Every DNS answer is checked, including mapped IPv6;
mixed denied results fail closed. The selected socket address is pinned and its
peer checked. No proxy, redirect, config expansion, address retry or re-resolution.

| Limit | Initial ceiling |
|---|---|
| Targets / attempts / active sockets | 1 / 1 / 1 |
| DNS answers | 8 |
| DNS / TCP / protocol / overall time | 5 / 3 / 5 / 15 seconds |
| Protocol socket received / sent bytes | 512 KiB / 64 KiB |
| SSH banner / complete unencrypted frame | 8 KiB / 256 KiB |

Counters measure bytes delivered through the protocol socket, excluding OS DNS
traffic and TCP overhead/retransmissions. Cancellation closes sockets and reaps
the resolver child; already initiated OS resolver service work is outside that
process boundary. Unsupported or oversized data never silently becomes complete.

TLS writes `probe.tls.v1.json`, SSH writes `probe.ssh-offers.v1.json`; each also
writes `report.html` and `report.md`. Defaults are `./pqc-probe/tls` and
`./pqc-probe/ssh`; choose fresh directories to avoid stale outputs. `--lang ja`
changes human copy only. These outputs do not replace the six static reports or
`scan.inventory.v1.json`. Known secret syntax is masked consistently in exports;
`redaction_applied` indicates that affected original strings are unavailable.

Exit0 means the requested observation completed, not that the endpoint is safe.
Exit1 is setup/output failure; exit3 is denied/failed/partial/cancelled/unsupported
inspection. Existing scan exit0/1/2/3 behavior is unchanged.
