# TLS接続先の観測結果

検証状態と通信で決まった事実は、この接続先とクライアント設定に対する1回の観測を示します。収集上限と診断情報を併せて確認してください。サーバーの全能力やPQC移行完了を保証する結果ではありません。

## 観測の概要

- 観測の完了状況: 要求した観測を収集しました (completed)
- 実際の接続相手: 127.0.0.1:49029
- 要求したホスト: 127.0.0.1
- 要求したポート: 49029
- ソケットで受信したバイト数: 1408
- ソケットで送信したバイト数: 322

|項目パス|項目|内容|
|---|---|---|
|/attempt_id|今回の実行内の試行番号|1|
|/details/certificate_collection/collected_count|収集した証明書数|1|
|/details/certificate_collection/presented_bytes|提示された証明書のバイト数|852|
|/details/certificate_collection/presented_count|提示された証明書数|1|
|/details/certificate_collection/status|状態|complete|
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
|/details/handshake_complete|ハンドシェイク完了|true|
|/details/handshake_kind|ハンドシェイクの種類|full|
|/details/negotiated/cipher_suite/id|識別子|4866|
|/details/negotiated/cipher_suite/name|名前|TLS13_AES_256_GCM_SHA384|
|/details/negotiated/group_classification|グループ識別子に基づく分類|classical|
|/details/negotiated/key_exchange_group/id|識別子|29|
|/details/negotiated/key_exchange_group/name|名前|X25519|
|/details/negotiated/version/id|識別子|772|
|/details/negotiated/version/name|名前|TLSv1_3|
|/details/peer_certificates/0/attribute_status|公開属性の検査状態|complete|
|/details/peer_certificates/0/certificate_der_sha256|公開証明書DER全体のSHA-256|c805f7ca420bb265c8aa962663eabd81b57ea348baf3e2b3506c221f6fcf386d|
|/details/peer_certificates/0/key_bits|鍵長（ビット）|2048|
|/details/peer_certificates/0/named_curve_oid|名前付き曲線のOID|null|
|/details/peer_certificates/0/not_after_unix|有効期間の終了（Unix秒）|1794983453|
|/details/peer_certificates/0/not_before_unix|有効期間の開始（Unix秒）|1794897053|
|/details/peer_certificates/0/public_key_oid|公開鍵OID|1.2.840.113549.1.1.1|
|/details/peer_certificates/0/signature_oid|証明書の署名OID|1.2.840.113549.1.1.11|
|/details/peer_certificates/0/spki_sha256|公開SPKIのSHA-256|07b2647b0002831ed5296ce5c029bd547891ad3486f0bf529c901d2e4ecd7de8|
|/details/verification/certificate_chain_and_name|証明書チェーンと名前の検証|verified|
|/details/verification/revocation|失効確認|not_checked|
|/details/verification/root_set_digest_basis|root_set_digest_basis|sorted_length_prefixed_trust_anchor_der_fields_sha256_v1|
|/details/verification/root_set_sha256|公開ルート集合のSHA-256|84ccca61fab8373515eb42260578a839df5d5f85afb65745dc1b094b835f955e|
|/details/verification/sent_sni|送信したSNI|localhost|
|/details/verification/trust_bundle_version|trust_bundle_version|null|
|/details/verification/trust_source|信頼元|local_ca_file|
|/details/verification/verification_name|検証する名前|localhost|
|/diagnostics|診断コード|\[\]|
|/generated_at|作成日時|2026-11-17T06:30:53.340397464+00:00|
|/limits/dns_timeout_ns|名前解決の制限時間（ナノ秒）|5000000000|
|/limits/max_dns_answers|名前解決結果の上限|8|
|/limits/max_received_bytes|受信上限（バイト）|524288|
|/limits/max_sent_bytes|送信上限（バイト）|65536|
|/limits/overall_timeout_ns|全体の制限時間（ナノ秒）|15000000000|
|/limits/protocol_timeout_ns|プロトコル処理の制限時間（ナノ秒）|5000000000|
|/limits/tcp_timeout_ns|TCP接続の制限時間（ナノ秒）|3000000000|
|/outcome|観測の完了状況|要求した観測を収集しました (completed)|
|/protocol|観測方法|tls|
|/redaction_applied|秘密情報に見える文字列の省略|false|
|/scanner_version|スキャナーの版|0.3.0|
|/schema_version|スキーマの版|1|
|/target/connected_peer|実際の接続相手|127.0.0.1:49029|
|/target/host|要求したホスト|127.0.0.1|
|/target/pinned_address|固定した接続先|127.0.0.1:49029|
|/target/port|要求したポート|49029|
|/target/resolved_addresses/0|確認した名前解決結果|127.0.0.1|
|/traffic/received_bytes|ソケットで受信したバイト数|1408|
|/traffic/sent_bytes|ソケットで送信したバイト数|322|
