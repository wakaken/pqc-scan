# SSHハイブリッド鍵交換の確認結果

事前に信頼したホスト公開鍵との照合と、サーバー署名の検証を行います。ML-KEM768とX25519のハイブリッド鍵交換を調べ、ホスト認証には従来方式のEd25519を使います。ユーザー認証・ログインは行いません。失敗だけでサーバー全体の非対応を断定することはできません。

## 観測の概要

- 観測の完了状況: 要求した観測を収集しました (completed)
- 実際の接続相手: 127.0.0.1:51195
- 要求したホスト: 127.0.0.1
- 要求したポート: 51195
- ソケットで受信したバイト数: 2150
- ソケットで送信したバイト数: 1786

|項目パス|項目|内容|
|---|---|---|
|/attempt_id|今回の実行内の試行番号|1|
|/details/handshake_complete|ハンドシェイク完了|true|
|/details/host_authentication_pqc|ホスト認証のPQC対応|false|
|/details/host_identity_verified|信頼済みホスト鍵と署名の検証|true|
|/details/host_signature|検証済みホスト署名方式|ssh-ed25519|
|/details/negotiated_kex|検証済みの鍵交換方式|mlkem768x25519-sha256|
|/details/offered_host_key|提示した固定のホスト鍵方式|ssh-ed25519|
|/details/offered_kex|提示した固定の鍵交換方式|mlkem768x25519-sha256|
|/details/provider_version|プロバイダーの版|1.18.0|
|/details/russh_version|russhの版|0.65.1|
|/details/trust_source|信頼元|explicit_endpoint_public_key|
|/details/user_authentication_attempted|ユーザー認証の実施|false|
|/diagnostics|診断コード|\[\]|
|/generated_at|作成日時|2026-11-17T06:30:53.659595965+00:00|
|/limits/dns_timeout_ns|名前解決の制限時間（ナノ秒）|5000000000|
|/limits/max_dns_answers|名前解決結果の上限|8|
|/limits/max_received_bytes|受信上限（バイト）|524288|
|/limits/max_sent_bytes|送信上限（バイト）|65536|
|/limits/overall_timeout_ns|全体の制限時間（ナノ秒）|15000000000|
|/limits/protocol_timeout_ns|プロトコル処理の制限時間（ナノ秒）|5000000000|
|/limits/tcp_timeout_ns|TCP接続の制限時間（ナノ秒）|3000000000|
|/outcome|観測の完了状況|要求した観測を収集しました (completed)|
|/protocol|観測方法|ssh_kex|
|/redaction_applied|秘密情報に見える文字列の省略|false|
|/scanner_version|スキャナーの版|0.3.0|
|/schema_version|スキーマの版|1|
|/target/connected_peer|実際の接続相手|127.0.0.1:51195|
|/target/host|要求したホスト|127.0.0.1|
|/target/pinned_address|固定した接続先|127.0.0.1:51195|
|/target/port|要求したポート|51195|
|/target/resolved_addresses/0|確認した名前解決結果|127.0.0.1|
|/traffic/received_bytes|ソケットで受信したバイト数|2150|
|/traffic/sent_bytes|ソケットで送信したバイト数|1786|
