# Explicit TLS and SSH diagnostics

[日本語](PROBING.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/develop/README.md)

All three probe modes remain experimental in the v0.3 development checkout, as in v0.2.0. The published v0.2.0 binary is
the static variant, which has no `probe` command. Probe bundles have completed
technical preparation but retain publication checks; see [distribution status](DISTRIBUTION.md#release-status).

Use probes only for a host/port you are authorized to inspect. A probe exchanges
protocol traffic with that endpoint. Repository `scan` and report generation stay
local; source files, URLs, certificate SANs and findings never start a probe.

## Install the optional commands

The default installation includes static scanning. Enable either or both optional
commands at build time using the pinned Rust toolchain:

```bash
cargo install --locked --path crates/pqc_scan_cli --features probe-tls,probe-ssh
```

The published binary targets macOS on Apple Silicon. An isolated Linux protocol/OpenSSH test harness is available, but the repository's `protocols` CI job is currently paused, including manual workflow runs. Do not infer a fresh protocol pass from a successful `quality`, `baseline` or `scan` job. The harness does not qualify a Linux binary distribution or other OS/architecture combinations. TLS adds a native cryptographic provider
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

## Verified hybrid SSH key exchange (experimental)

Enable this separate optional feature when installing:

```sh
cargo install --locked --path crates/pqc_scan_cli --features probe-ssh-kex
pqc-scan probe ssh-kex service.example --port 22 --host-key-file /path/to/trusted-host.pub --out-dir /path/to/new-ssh-kex-report
```

The file must contain one OpenSSH Ed25519 **public host key**, obtained from a
trusted administrator or another independent trusted channel. It is the pin for
this explicitly requested endpoint. Do not fetch a key from the untrusted server
and treat it as trusted. Private keys, host certificates, known_hosts files and
ambient SSH config/agents are not used. Input is limited to4096 bytes and checked
before DNS or connections; Unix symlinks/nonregular files are rejected.

The fixed profile offers only `mlkem768x25519-sha256` (ML-KEM768+X25519 hybrid)
and `ssh-ed25519` host signatures. No fallback to classical-only key exchange.
The library verifies the exchange signature; the peer key must match the supplied
pin and the initial exchange must complete. The library may send NEWKEYS before
the application checks the pin, including on a later pin mismatch. A mismatch
never produces a verified result. `user_authentication_attempted: false` records
the client mode; it is not a per-connection packet trace. The connection is then closed without
an authentication service request, username, password, `none` authentication,
login, channel or remote command. Ed25519 authentication is classical: this does
not establish PQ host signatures or complete PQC migration.

One attempt, pinned peer, address allowlist, DNS/time/traffic/cancellation caps
above also apply. This mode is explicit network activity and may be logged by the
server. Failure/no overlap means this exact profile could not be verified; it does
not enumerate every server capability. Unknown or mismatched keys fail closed.

Separate outputs: `probe.ssh-kex.v1.json`, `report.html`, `report.md`; default
folder `./pqc-probe/ssh-kex`. JSON contract:
`schemas/probe-ssh-kex-v1.schema.json`. Verified selection fields remain null on
failed/partial observations. HTML presents the result/reason first, with technical
details closed. Existing `probe ssh` remains advertisement-only; no existing
static/TLS/SSH-offer output format changes. Enable `probe-tls,probe-ssh,probe-ssh-kex`
for all three commands. The `static` archive has no probes; `combined-probes` includes TLS and SSH offers;
`all-probes` also includes SSH KEX. All three variants have been prepared and
installation-tested for the v0.2 candidate; this does not clear the probe bundles
for publication.
