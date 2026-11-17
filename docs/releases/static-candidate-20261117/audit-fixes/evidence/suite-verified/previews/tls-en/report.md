# TLS endpoint observation

Verification state and any negotiated facts describe this single endpoint and client profile. Inspection limits and diagnostics qualify the result. This observation does not establish exhaustive server capabilities or completed PQC migration.

## Overview

- Inspection outcome: Requested observation collected (completed)
- Connected peer: 127.0.0.1:53829
- Requested host: 127.0.0.1
- Requested port: 53829
- Received socket bytes: 1408
- Sent socket bytes: 322

|Field path|Field|Value|
|---|---|---|
|/attempt_id|Attempt within this invocation|1|
|/details/certificate_collection/collected_count|Collected certificate count|1|
|/details/certificate_collection/presented_bytes|Presented certificate bytes|852|
|/details/certificate_collection/presented_count|Presented certificate count|1|
|/details/certificate_collection/status|Status|complete|
|/details/client_profile/cipher_suites/0/id|Identifier|4866|
|/details/client_profile/cipher_suites/0/name|Name|TLS13_AES_256_GCM_SHA384|
|/details/client_profile/cipher_suites/1/id|Identifier|4865|
|/details/client_profile/cipher_suites/1/name|Name|TLS13_AES_128_GCM_SHA256|
|/details/client_profile/cipher_suites/2/id|Identifier|4867|
|/details/client_profile/cipher_suites/2/name|Name|TLS13_CHACHA20_POLY1305_SHA256|
|/details/client_profile/cipher_suites/3/id|Identifier|49196|
|/details/client_profile/cipher_suites/3/name|Name|TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384|
|/details/client_profile/cipher_suites/4/id|Identifier|49195|
|/details/client_profile/cipher_suites/4/name|Name|TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256|
|/details/client_profile/cipher_suites/5/id|Identifier|52393|
|/details/client_profile/cipher_suites/5/name|Name|TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256|
|/details/client_profile/cipher_suites/6/id|Identifier|49200|
|/details/client_profile/cipher_suites/6/name|Name|TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384|
|/details/client_profile/cipher_suites/7/id|Identifier|49199|
|/details/client_profile/cipher_suites/7/name|Name|TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256|
|/details/client_profile/cipher_suites/8/id|Identifier|52392|
|/details/client_profile/cipher_suites/8/name|Name|TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256|
|/details/client_profile/key_exchange_groups/0/id|Identifier|29|
|/details/client_profile/key_exchange_groups/0/name|Name|X25519|
|/details/client_profile/key_exchange_groups/1/id|Identifier|23|
|/details/client_profile/key_exchange_groups/1/name|Name|secp256r1|
|/details/client_profile/key_exchange_groups/2/id|Identifier|24|
|/details/client_profile/key_exchange_groups/2/name|Name|secp384r1|
|/details/client_profile/key_exchange_groups/3/id|Identifier|4588|
|/details/client_profile/key_exchange_groups/3/name|Name|X25519MLKEM768|
|/details/client_profile/name|Name|default_secure_fresh|
|/details/client_profile/protocol_versions/0/id|Identifier|772|
|/details/client_profile/protocol_versions/0/name|Name|TLSv1_3|
|/details/client_profile/protocol_versions/1/id|Identifier|771|
|/details/client_profile/protocol_versions/1/name|Name|TLSv1_2|
|/details/client_profile/provider|Provider|aws_lc_rs|
|/details/client_profile/provider_version|Provider version|1.18.0|
|/details/client_profile/rustls_version|rustls version|0.23.46|
|/details/handshake_complete|Handshake complete|true|
|/details/handshake_kind|Handshake kind|full|
|/details/negotiated/cipher_suite/id|Identifier|4866|
|/details/negotiated/cipher_suite/name|Name|TLS13_AES_256_GCM_SHA384|
|/details/negotiated/group_classification|Exact-group classification|classical|
|/details/negotiated/key_exchange_group/id|Identifier|29|
|/details/negotiated/key_exchange_group/name|Name|X25519|
|/details/negotiated/version/id|Identifier|772|
|/details/negotiated/version/name|Name|TLSv1_3|
|/details/peer_certificates/0/attribute_status|Public-attribute inspection status|complete|
|/details/peer_certificates/0/certificate_der_sha256|Full public certificate DER SHA-256|c805f7ca420bb265c8aa962663eabd81b57ea348baf3e2b3506c221f6fcf386d|
|/details/peer_certificates/0/key_bits|Key size (bits)|2048|
|/details/peer_certificates/0/named_curve_oid|Named-curve OID|null|
|/details/peer_certificates/0/not_after_unix|Validity end (Unix seconds)|1794983453|
|/details/peer_certificates/0/not_before_unix|Validity start (Unix seconds)|1794897053|
|/details/peer_certificates/0/public_key_oid|Public-key OID|1.2.840.113549.1.1.1|
|/details/peer_certificates/0/signature_oid|Certificate signature OID|1.2.840.113549.1.1.11|
|/details/peer_certificates/0/spki_sha256|Public SPKI SHA-256|07b2647b0002831ed5296ce5c029bd547891ad3486f0bf529c901d2e4ecd7de8|
|/details/verification/certificate_chain_and_name|Certificate chain and name validation|verified|
|/details/verification/revocation|Revocation checking|not_checked|
|/details/verification/root_set_digest_basis|root_set_digest_basis|sorted_length_prefixed_trust_anchor_der_fields_sha256_v1|
|/details/verification/root_set_sha256|Public root-set SHA-256|84ccca61fab8373515eb42260578a839df5d5f85afb65745dc1b094b835f955e|
|/details/verification/sent_sni|Sent SNI|localhost|
|/details/verification/trust_bundle_version|trust_bundle_version|null|
|/details/verification/trust_source|Trust source|local_ca_file|
|/details/verification/verification_name|Verification name|localhost|
|/diagnostics|Diagnostic code|\[\]|
|/generated_at|Generated at|2026-11-17T06:30:53.437313006+00:00|
|/limits/dns_timeout_ns|DNS timeout (ns)|5000000000|
|/limits/max_dns_answers|Maximum DNS answers|8|
|/limits/max_received_bytes|Receive limit (bytes)|524288|
|/limits/max_sent_bytes|Send limit (bytes)|65536|
|/limits/overall_timeout_ns|Overall timeout (ns)|15000000000|
|/limits/protocol_timeout_ns|Protocol timeout (ns)|5000000000|
|/limits/tcp_timeout_ns|TCP timeout (ns)|3000000000|
|/outcome|Inspection outcome|Requested observation collected (completed)|
|/protocol|Observation method|tls|
|/redaction_applied|Secret-like text omitted|false|
|/scanner_version|Scanner version|0.3.0|
|/schema_version|Schema version|1|
|/target/connected_peer|Connected peer|127.0.0.1:53829|
|/target/host|Requested host|127.0.0.1|
|/target/pinned_address|Pinned destination|127.0.0.1:53829|
|/target/port|Requested port|53829|
|/target/resolved_addresses/0|Checked DNS answers|127.0.0.1|
|/traffic/received_bytes|Received socket bytes|1408|
|/traffic/sent_bytes|Sent socket bytes|322|
