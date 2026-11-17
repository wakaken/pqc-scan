# SSH advertised algorithm observation

Unauthenticated advertisements describe the server's claims when lists are available. Lists retain their original order. Client lists are diagnostic proposals, not implemented cryptographic capabilities. Key exchange, host identity verification and authentication were not performed.

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
|/details/assurance|Assurance|not_observed|
|/details/client_profile/first_kex_packet_follows|Following guessed KEX packet declared|false|
|/details/client_profile/identification|Diagnostic client identification|SSH-2.0-pqc_scan_0.3.0|
|/details/client_profile/offers/compression_algorithms_client_to_server/0|Compression candidates: client to server|none|
|/details/client_profile/offers/compression_algorithms_server_to_client/0|Compression candidates: server to client|none|
|/details/client_profile/offers/encryption_algorithms_client_to_server/0|Encryption candidates: client to server|chacha20-poly1305@openssh.com|
|/details/client_profile/offers/encryption_algorithms_client_to_server/1|Encryption candidates: client to server|aes256-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_client_to_server/2|Encryption candidates: client to server|aes128-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_client_to_server/3|Encryption candidates: client to server|aes256-ctr|
|/details/client_profile/offers/encryption_algorithms_client_to_server/4|Encryption candidates: client to server|aes128-ctr|
|/details/client_profile/offers/encryption_algorithms_server_to_client/0|Encryption candidates: server to client|chacha20-poly1305@openssh.com|
|/details/client_profile/offers/encryption_algorithms_server_to_client/1|Encryption candidates: server to client|aes256-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_server_to_client/2|Encryption candidates: server to client|aes128-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_server_to_client/3|Encryption candidates: server to client|aes256-ctr|
|/details/client_profile/offers/encryption_algorithms_server_to_client/4|Encryption candidates: server to client|aes128-ctr|
|/details/client_profile/offers/kex_algorithms/0|Key exchange candidates|mlkem768x25519-sha256|
|/details/client_profile/offers/kex_algorithms/1|Key exchange candidates|sntrup761x25519-sha512|
|/details/client_profile/offers/kex_algorithms/2|Key exchange candidates|sntrup761x25519-sha512@openssh.com|
|/details/client_profile/offers/kex_algorithms/3|Key exchange candidates|curve25519-sha256|
|/details/client_profile/offers/kex_algorithms/4|Key exchange candidates|curve25519-sha256@libssh.org|
|/details/client_profile/offers/languages_client_to_server|Language candidates: client to server|\[\]|
|/details/client_profile/offers/languages_server_to_client|Language candidates: server to client|\[\]|
|/details/client_profile/offers/mac_algorithms_client_to_server/0|MAC candidates: client to server|hmac-sha2-256-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_client_to_server/1|MAC candidates: client to server|hmac-sha2-512-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_client_to_server/2|MAC candidates: client to server|hmac-sha2-256|
|/details/client_profile/offers/mac_algorithms_server_to_client/0|MAC candidates: server to client|hmac-sha2-256-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_server_to_client/1|MAC candidates: server to client|hmac-sha2-512-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_server_to_client/2|MAC candidates: server to client|hmac-sha2-256|
|/details/client_profile/offers/server_host_key_algorithms/0|Host-key candidates|ssh-ed25519|
|/details/client_profile/offers/server_host_key_algorithms/1|Host-key candidates|ecdsa-sha2-nistp256|
|/details/client_profile/offers/server_host_key_algorithms/2|Host-key candidates|rsa-sha2-512|
|/details/client_profile/offers/server_host_key_algorithms/3|Host-key candidates|rsa-sha2-256|
|/details/client_profile/purpose|Purpose|diagnostic_proposal_only|
|/details/client_profile/reserved|Reserved field|0|
|/details/early_messages|Early transport messages|0|
|/details/extension_markers|Extension marker|\[\]|
|/details/first_kex_packet_follows|Following guessed KEX packet declared|null|
|/details/host_key_candidates|host_key_candidates|\[\]|
|/details/kex_candidates|kex_candidates|\[\]|
|/details/offers|offers|null|
|/details/pre_identification_lines|Pre-identification lines|0|
|/details/server_identification|Unverified server identification|null|
|/diagnostics/0|Diagnostic code|address_denied|
|/generated_at|Generated at|2026-11-17T06:26:07.770444+00:00|
|/limits/dns_timeout_ns|DNS timeout (ns)|5000000000|
|/limits/max_dns_answers|Maximum DNS answers|8|
|/limits/max_received_bytes|Receive limit (bytes)|524288|
|/limits/max_sent_bytes|Send limit (bytes)|65536|
|/limits/overall_timeout_ns|Overall timeout (ns)|15000000000|
|/limits/protocol_timeout_ns|Protocol timeout (ns)|5000000000|
|/limits/tcp_timeout_ns|TCP timeout (ns)|3000000000|
|/outcome|Inspection outcome|Requested observation could not complete (failed)|
|/protocol|Observation method|ssh_offers|
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
