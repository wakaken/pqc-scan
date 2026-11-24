# Explicit TLS and SSH diagnostics

[日本語](PROBING.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/develop/README.md)

The macOS Apple Silicon v0.4 `network` variant includes three diagnostic commands.
The `static` variant has no `probe` command. See [choosing a download](DISTRIBUTION.md#release-status).

| Goal | Command | What it establishes |
|---|---|---|
| TLS selection and public certificates | [`probe tls`](#tls) | Results of a successfully verified handshake |
| SSH advertised methods | [`probe ssh`](#ssh-offers) | Unauthenticated advertisements; no key exchange |
| Hybrid SSH key exchange | [`probe ssh-kex`](#verified-hybrid-ssh-key-exchange) | Match to a trusted host-key pin and initial exchange using a fixed profile |

Use `probe` only for a host/port the administrator has authorized you to inspect.
It exchanges traffic with the specified endpoint and may be logged there.
Repository `scan` and report generation run locally; source URLs, certificate
SANs and findings do not cause connections to external services.

<a id="install-the-optional-commands"></a>

## Enable the probe commands

The `network` binary includes all three commands. For TLS and SSH offers in a
source installation, enable the following features with `--features`:

```bash
cargo install --locked --path crates/pqc_scan_cli --features probe-tls,probe-ssh
```

Hybrid SSH key exchange uses the [separate feature below](#verified-hybrid-ssh-key-exchange).
For all commands, pass `probe-tls,probe-ssh,probe-ssh-kex` to `--features`.

<a id="tls"></a>

## 1. TLS

Run the command with the hostname and port as shown below.

```bash
pqc-scan probe tls service.example --port 443 --out-dir /path/to/new-tls-report
```

One fresh TLS 1.2/1.3 handshake records the selected version, suite, key-exchange
group, bounded public certificate facts and configured client profile.

### Select a trusted CA

The default uses a fixed local Mozilla root bundle. To replace it with a local public CA:

```bash
pqc-scan probe tls 127.0.0.1 --port 8443 --allow-cidr 127.0.0.1/32 --server-name localhost --ca-file /path/to/public-ca.pem --out-dir /path/to/new-local-tls-report
```

| Item | Behavior or condition |
|---|---|
| `--ca-file` | Replaces the root bundle; public certificate PEM only, up to 16 blocks/256 KiB. No private keys or passwords |
| Certificate validation | Ordinary chain/name/time validation is mandatory. OS trust constraints and revocation checking are not supplied |
| `--server-name` | Selects the verification name. DNS names also provide SNI; IP verification sends no SNI. The pinned destination does not change |
| Certificate collection | Up to 16 certificates/256 KiB; other processing limits may be reached first |
| Operations not performed | Application requests, client credentials, session resumption, 0-RTT and key logging. Client-authentication requests are unsupported |

Verification failures are not shown as verified negotiated results. Failures and
limits mean incomplete inspection, not that the server lacks PQC or is unsafe.
A TLS 1.3 suite alone does not distinguish classical from hybrid key exchange.

<a id="ssh-offers"></a>

## 2. SSH offers

```bash
pqc-scan probe ssh service.example --port 22 --out-dir /path/to/new-ssh-report
```

Identification and KEXINIT are exchanged; all ten ordered remote name lists and
extension markers are recorded. The connection closes before executing key exchange.

| Recorded information | Interpretation |
|---|---|
| Remote methods and banner | Unauthenticated advertisements, not proof of a selected method, verified host key, completed session or server-wide readiness |
| Diagnostic client proposal | Listing an algorithm does not claim that the client implements it |
| Banner version and unknown identifiers | The version is a peer claim; unknown identifiers remain unknown |

No service/userauth/channel requests or `none` authentication are sent. No username,
agent, private key, SSH configuration or system-SSH backend is used.

<a id="verified-hybrid-ssh-key-exchange"></a>

## 3. Verified hybrid SSH key exchange

For source installation, specify `--features probe-ssh-kex` as shown below. Supply one OpenSSH
Ed25519 **public host key** from a trusted administrator or another independent
trusted channel. Do not fetch a key from an untrusted server and treat it as trusted.

```sh
cargo install --locked --path crates/pqc_scan_cli --features probe-ssh-kex
pqc-scan probe ssh-kex service.example --port 22 --host-key-file /path/to/trusted-host.pub --out-dir /path/to/new-ssh-kex-report
```

| Check | Condition |
|---|---|
| Host public key | Pins the explicitly requested endpoint. Up to 4096 bytes, validated before DNS/connections; Unix symlinks and nonregular files are rejected |
| Key exchange | Fixed to `mlkem768x25519-sha256` (ML-KEM768+X25519), without classical-only fallback |
| Host signature | Fixed to `ssh-ed25519`. Verify the exchange signature, match the pin and complete the initial exchange, then disconnect |
| Inputs and operations not used | Private keys, host certificates, known_hosts, ambient SSH configuration/agents. No authentication service request, username, password, `none` authentication, login, channel or remote command |

Unknown or mismatched keys are rejected; failed/partial observations leave verified
selection fields null. Failure or no overlap does not establish server-wide lack
of PQC. Ed25519 authentication is classical, so this does not establish PQ host
signatures or completed migration. The shared conditions below cover the single
attempt, pinned peer, address authorization, time/traffic limits and cancellation.

## Authorization, limits and outputs

### Allowed addresses

| Destination | Condition |
|---|---|
| Public unicast | Allowed by default |
| Internal/loopback/special-purpose unicast | Requires `--allow-cidr` |
| Unspecified, multicast and limited broadcast | Cannot be allowed |

Every DNS answer is checked, including mapped IPv6; mixed denied results fail
closed. The validated socket address is pinned and its actual peer checked.
No proxy, redirect, config expansion, address retry or re-resolution is used.

### Processing limits and cancellation

| Limit | Initial ceiling |
|---|---|
| Targets / attempts / active sockets | 1 / 1 / 1 |
| DNS answers | 8 |
| DNS / TCP / protocol / overall time | 5 / 3 / 5 / 15 seconds |
| Protocol socket received / sent bytes | 512 KiB / 64 KiB |
| SSH banner / complete unencrypted frame | 8 KiB / 256 KiB |

Counters measure bytes delivered through the protocol socket, excluding OS DNS
traffic and TCP overhead/retransmissions. Cancellation closes the connection, but
OS name resolution already in progress may continue. Unsupported or oversized
data never silently becomes complete.

### Output files and exit codes

| Command | JSON | Default directory |
|---|---|---|
| `probe tls` | `probe.tls.v1.json` | `./pqc-probe/tls` |
| `probe ssh` | `probe.ssh-offers.v1.json` | `./pqc-probe/ssh` |
| `probe ssh-kex` | `probe.ssh-kex.v1.json` | `./pqc-probe/ssh-kex` |

Each also writes `report.html` and `report.md`. SSH KEX HTML presents the result
and reason first, with technical details closed. Its JSON contract is
`schemas/probe-ssh-kex-v1.schema.json`. Choose a fresh directory to avoid stale
outputs. `--lang ja` changes human copy only; outputs do not replace the six static
reports or `scan.inventory.v1.json`. Known secret syntax is masked in every export;
`redaction_applied` indicates that affected original strings are unavailable.

| Exit code | Meaning |
|---|---|
| 0 | Requested observation completed; not a safety verdict |
| 1 | Setup/output error |
| 3 | Denied, failed, partial, cancelled or unsupported inspection |

Existing `scan` exit codes 0/1/2/3 remain unchanged; see [scan exit codes](SCANNING.md#diagnostics).
