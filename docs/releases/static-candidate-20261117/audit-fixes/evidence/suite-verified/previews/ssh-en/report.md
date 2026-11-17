# SSH advertised algorithm observation

Unauthenticated advertisements describe the server's claims when lists are available. Lists retain their original order. Client lists are diagnostic proposals, not implemented cryptographic capabilities. Key exchange, host identity verification and authentication were not performed.

## Overview

- Inspection outcome: Requested observation collected (completed)
- Connected peer: 127.0.0.1:56929
- Requested host: 127.0.0.1
- Requested port: 56929
- Received socket bytes: 316
- Sent socket bytes: 632

|Field path|Field|Value|
|---|---|---|
|/attempt_id|Attempt within this invocation|1|
|/details/assurance|Assurance|unauthenticated_advertisement|
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
|/details/extension_markers/0|Extension marker|kex-strict-s-v00@openssh.com|
|/details/first_kex_packet_follows|Following guessed KEX packet declared|false|
|/details/host_key_candidates/0/construction|Construction classification|classical|
|/details/host_key_candidates/0/name|Name|ssh-ed25519|
|/details/host_key_candidates/0/namespace|Identifier namespace|unqualified|
|/details/host_key_candidates/0/provenance|Provenance|rfc_defined|
|/details/host_key_candidates/0/reference|Reference|https://www.rfc-editor.org/rfc/rfc8709.html|
|/details/host_key_candidates/1/construction|Construction classification|classical|
|/details/host_key_candidates/1/name|Name|rsa-sha2-512|
|/details/host_key_candidates/1/namespace|Identifier namespace|unqualified|
|/details/host_key_candidates/1/provenance|Provenance|rfc_defined|
|/details/host_key_candidates/1/reference|Reference|https://www.rfc-editor.org/rfc/rfc8332.html|
|/details/kex_candidates/0/construction|Construction classification|hybrid|
|/details/kex_candidates/0/name|Name|sntrup761x25519-sha512@openssh.com|
|/details/kex_candidates/0/namespace|Identifier namespace|vendor_namespaced|
|/details/kex_candidates/0/provenance|Provenance|openssh_defined|
|/details/kex_candidates/0/reference|Reference|https://www.openssh.org/releasenotes.html|
|/details/kex_candidates/1/construction|Construction classification|classical|
|/details/kex_candidates/1/name|Name|curve25519-sha256|
|/details/kex_candidates/1/namespace|Identifier namespace|unqualified|
|/details/kex_candidates/1/provenance|Provenance|rfc_defined|
|/details/kex_candidates/1/reference|Reference|https://www.rfc-editor.org/rfc/rfc8731.html|
|/details/kex_candidates/2/construction|Construction classification|hybrid|
|/details/kex_candidates/2/name|Name|mlkem768x25519-sha256|
|/details/kex_candidates/2/namespace|Identifier namespace|unqualified|
|/details/kex_candidates/2/provenance|Provenance|rfc_informational|
|/details/kex_candidates/2/reference|Reference|https://www.rfc-editor.org/rfc/rfc10042.html|
|/details/kex_candidates/3/construction|Construction classification|unknown|
|/details/kex_candidates/3/name|Name|unknown-preview-kex|
|/details/kex_candidates/3/namespace|Identifier namespace|unqualified|
|/details/kex_candidates/3/provenance|Provenance|unknown|
|/details/kex_candidates/3/reference|Reference|null|
|/details/offers/compression_algorithms_client_to_server/0|Compression candidates: client to server|none|
|/details/offers/compression_algorithms_server_to_client/0|Compression candidates: server to client|none|
|/details/offers/encryption_algorithms_client_to_server/0|Encryption candidates: client to server|aes128-ctr|
|/details/offers/encryption_algorithms_server_to_client/0|Encryption candidates: server to client|aes128-ctr|
|/details/offers/kex_algorithms/0|Key exchange candidates|sntrup761x25519-sha512@openssh.com|
|/details/offers/kex_algorithms/1|Key exchange candidates|curve25519-sha256|
|/details/offers/kex_algorithms/2|Key exchange candidates|kex-strict-s-v00@openssh.com|
|/details/offers/kex_algorithms/3|Key exchange candidates|mlkem768x25519-sha256|
|/details/offers/kex_algorithms/4|Key exchange candidates|unknown-preview-kex|
|/details/offers/languages_client_to_server|Language candidates: client to server|\[\]|
|/details/offers/languages_server_to_client|Language candidates: server to client|\[\]|
|/details/offers/mac_algorithms_client_to_server/0|MAC candidates: client to server|hmac-sha2-256|
|/details/offers/mac_algorithms_server_to_client/0|MAC candidates: server to client|hmac-sha2-256|
|/details/offers/server_host_key_algorithms/0|Host-key candidates|ssh-ed25519|
|/details/offers/server_host_key_algorithms/1|Host-key candidates|rsa-sha2-512|
|/details/pre_identification_lines|Pre-identification lines|0|
|/details/server_identification|Unverified server identification|SSH-2.0-pqc_preview synthetic fixture only|
|/diagnostics|Diagnostic code|\[\]|
|/generated_at|Generated at|2026-11-17T06:30:53.471684923+00:00|
|/limits/dns_timeout_ns|DNS timeout (ns)|5000000000|
|/limits/max_dns_answers|Maximum DNS answers|8|
|/limits/max_received_bytes|Receive limit (bytes)|524288|
|/limits/max_sent_bytes|Send limit (bytes)|65536|
|/limits/overall_timeout_ns|Overall timeout (ns)|15000000000|
|/limits/protocol_timeout_ns|Protocol timeout (ns)|5000000000|
|/limits/tcp_timeout_ns|TCP timeout (ns)|3000000000|
|/outcome|Inspection outcome|Requested observation collected (completed)|
|/protocol|Observation method|ssh_offers|
|/redaction_applied|Secret-like text omitted|false|
|/scanner_version|Scanner version|0.3.0|
|/schema_version|Schema version|1|
|/target/connected_peer|Connected peer|127.0.0.1:56929|
|/target/host|Requested host|127.0.0.1|
|/target/pinned_address|Pinned destination|127.0.0.1:56929|
|/target/port|Requested port|56929|
|/target/resolved_addresses/0|Checked DNS answers|127.0.0.1|
|/traffic/received_bytes|Received socket bytes|316|
|/traffic/sent_bytes|Sent socket bytes|632|
