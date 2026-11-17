# TLS接続先の観測結果

検証状態と通信で決まった事実は、この接続先とクライアント設定に対する1回の観測を示します。収集上限と診断情報を併せて確認してください。サーバーの全能力やPQC移行完了を保証する結果ではありません。

## 観測の概要

- 診断コード: address_denied
- 観測の完了状況: 要求した観測を完了できませんでした (failed)
- 実際の接続相手: null
- 要求したホスト: 127.0.0.1
- 要求したポート: 443
- ソケットで受信したバイト数: 0
- ソケットで送信したバイト数: 0

|項目パス|項目|内容|
|---|---|---|
|/attempt_id|今回の実行内の試行番号|1|
|/details/certificate_collection/collected_count|収集した証明書数|0|
|/details/certificate_collection/presented_bytes|提示された証明書のバイト数|null|
|/details/certificate_collection/presented_count|提示された証明書数|null|
|/details/certificate_collection/status|状態|not_available|
|/details/client_profile/cipher_suites/0/id|識別子|4866|
|/details/client_profile/cipher_suites/0/name|名前|TLS13_AES_256_GCM_SHA384|
|/details/client_profile/cipher_suites/1/id|識別子|4865|
|/details/client_profile/cipher_suites/1/name|名前|TLS13_AES_128_GCM_SHA256|
|/details/client_profile/cipher_suites/2/id|識別子|4867|
|/details/client_profile/cipher_suites/2/name|名前|TLS13_CHACHA20_POLY1305_SHA256|
|/details/client_profile/cipher_suites/3/id|識別子|49196|
|/details/client_profile/cipher_suites/3/name|名前|TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384|
|/details/client_profile/cipher_suites/4/id|識別子|49195|
|/details/client_profile/cipher_suites/4/name|名前|TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256|
|/details/client_profile/cipher_suites/5/id|識別子|52393|
|/details/client_profile/cipher_suites/5/name|名前|TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256|
|/details/client_profile/cipher_suites/6/id|識別子|49200|
|/details/client_profile/cipher_suites/6/name|名前|TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384|
|/details/client_profile/cipher_suites/7/id|識別子|49199|
|/details/client_profile/cipher_suites/7/name|名前|TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256|
|/details/client_profile/cipher_suites/8/id|識別子|52392|
|/details/client_profile/cipher_suites/8/name|名前|TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256|
|/details/client_profile/key_exchange_groups/0/id|識別子|29|
|/details/client_profile/key_exchange_groups/0/name|名前|X25519|
|/details/client_profile/key_exchange_groups/1/id|識別子|23|
|/details/client_profile/key_exchange_groups/1/name|名前|secp256r1|
|/details/client_profile/key_exchange_groups/2/id|識別子|24|
|/details/client_profile/key_exchange_groups/2/name|名前|secp384r1|
|/details/client_profile/key_exchange_groups/3/id|識別子|4588|
|/details/client_profile/key_exchange_groups/3/name|名前|X25519MLKEM768|
|/details/client_profile/name|名前|default_secure_fresh|
|/details/client_profile/protocol_versions/0/id|識別子|772|
|/details/client_profile/protocol_versions/0/name|名前|TLSv1_3|
|/details/client_profile/protocol_versions/1/id|識別子|771|
|/details/client_profile/protocol_versions/1/name|名前|TLSv1_2|
|/details/client_profile/provider|プロバイダー|aws_lc_rs|
|/details/client_profile/provider_version|プロバイダーの版|1.18.0|
|/details/client_profile/rustls_version|rustlsの版|0.23.46|
|/details/handshake_complete|ハンドシェイク完了|false|
|/details/handshake_kind|ハンドシェイクの種類|null|
|/details/negotiated|negotiated|null|
|/details/peer_certificates|peer_certificates|\[\]|
|/details/verification/certificate_chain_and_name|証明書チェーンと名前の検証|not_completed|
|/details/verification/revocation|失効確認|not_checked|
|/details/verification/root_set_digest_basis|root_set_digest_basis|sorted_length_prefixed_trust_anchor_der_fields_sha256_v1|
|/details/verification/root_set_sha256|公開ルート集合のSHA-256|86c02d21c2b203d30bf8f4b3782c0a8c7695828d8d25b6744474e9381d65bc00|
|/details/verification/sent_sni|送信したSNI|null|
|/details/verification/trust_bundle_version|trust_bundle_version|1.0.11|
|/details/verification/trust_source|信頼元|bundled_mozilla|
|/details/verification/verification_name|検証する名前|127.0.0.1|
|/diagnostics/0|診断コード|address_denied|
|/generated_at|作成日時|2026-11-17T06:30:53.394822006+00:00|
|/limits/dns_timeout_ns|名前解決の制限時間（ナノ秒）|5000000000|
|/limits/max_dns_answers|名前解決結果の上限|8|
|/limits/max_received_bytes|受信上限（バイト）|524288|
|/limits/max_sent_bytes|送信上限（バイト）|65536|
|/limits/overall_timeout_ns|全体の制限時間（ナノ秒）|15000000000|
|/limits/protocol_timeout_ns|プロトコル処理の制限時間（ナノ秒）|5000000000|
|/limits/tcp_timeout_ns|TCP接続の制限時間（ナノ秒）|3000000000|
|/outcome|観測の完了状況|要求した観測を完了できませんでした (failed)|
|/protocol|観測方法|tls|
|/redaction_applied|秘密情報に見える文字列の省略|false|
|/scanner_version|スキャナーの版|0.3.0|
|/schema_version|スキーマの版|1|
|/target/connected_peer|実際の接続相手|null|
|/target/host|要求したホスト|127.0.0.1|
|/target/pinned_address|固定した接続先|null|
|/target/port|要求したポート|443|
|/target/resolved_addresses|確認した名前解決結果|null|
|/traffic/received_bytes|ソケットで受信したバイト数|0|
|/traffic/sent_bytes|ソケットで送信したバイト数|0|
