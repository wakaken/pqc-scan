# SSH hybrid key exchange verification

An explicit prior public host-key pin and the server signature are required. This profile tests hybrid ML-KEM768+X25519 with classical Ed25519 host authentication, without user authentication or login. A failure does not establish global server non-support.

## Overview

- Diagnostic code: address_denied
- Inspection outcome: Requested observation could not complete (failed)
- Connected peer: null
- Requested host: 127.0.0.1
- Requested port: 443
- Received socket bytes: 0
- Sent socket bytes: 0

|Field path|Field|Value|
|---|---|---|
|/attempt_id|Attempt within this invocation|1|
|/details/handshake_complete|Handshake complete|false|
|/details/host_authentication_pqc|Post-quantum host authentication|false|
|/details/host_identity_verified|Trusted host key and signature verified|false|
|/details/host_signature|Verified host signature algorithm|null|
|/details/negotiated_kex|Verified selected key exchange|null|
|/details/offered_host_key|Fixed offered host-key type|ssh-ed25519|
|/details/offered_kex|Fixed offered key exchange|mlkem768x25519-sha256|
|/details/provider_version|Provider version|1.18.0|
|/details/russh_version|russh version|0.65.1|
|/details/trust_source|Trust source|explicit_endpoint_public_key|
|/details/user_authentication_attempted|User authentication attempted|false|
|/diagnostics/0|Diagnostic code|address_denied|
|/generated_at|Generated at|2026-11-17T06:26:07.853420+00:00|
|/limits/dns_timeout_ns|DNS timeout (ns)|5000000000|
|/limits/max_dns_answers|Maximum DNS answers|8|
|/limits/max_received_bytes|Receive limit (bytes)|524288|
|/limits/max_sent_bytes|Send limit (bytes)|65536|
|/limits/overall_timeout_ns|Overall timeout (ns)|15000000000|
|/limits/protocol_timeout_ns|Protocol timeout (ns)|5000000000|
|/limits/tcp_timeout_ns|TCP timeout (ns)|3000000000|
|/outcome|Inspection outcome|Requested observation could not complete (failed)|
|/protocol|Observation method|ssh_kex|
|/redaction_applied|Secret-like text omitted|false|
|/scanner_version|Scanner version|0.3.0|
|/schema_version|Schema version|1|
|/target/connected_peer|Connected peer|null|
|/target/host|Requested host|127.0.0.1|
|/target/pinned_address|Pinned destination|null|
|/target/port|Requested port|443|
|/target/resolved_addresses|Checked DNS answers|null|
|/traffic/received_bytes|Received socket bytes|0|
|/traffic/sent_bytes|Sent socket bytes|0|
