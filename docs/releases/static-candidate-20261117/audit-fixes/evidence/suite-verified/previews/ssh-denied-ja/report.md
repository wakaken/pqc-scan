# SSH提示アルゴリズムの観測結果

収集できた提示一覧は、サーバーによる認証前の申告として元の順に記録します。クライアント側の一覧は診断用の提案で、暗号方式の実装対応を示すものではありません。鍵交換・ホストの検証・認証は実施していません。

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
|/details/assurance|確認根拠|not_observed|
|/details/client_profile/first_kex_packet_follows|後続の推測KEXパケットの申告|false|
|/details/client_profile/identification|診断クライアントの識別情報|SSH-2.0-pqc_scan_0.3.0|
|/details/client_profile/offers/compression_algorithms_client_to_server/0|圧縮候補：クライアントからサーバー|none|
|/details/client_profile/offers/compression_algorithms_server_to_client/0|圧縮候補：サーバーからクライアント|none|
|/details/client_profile/offers/encryption_algorithms_client_to_server/0|暗号化候補：クライアントからサーバー|chacha20-poly1305@openssh.com|
|/details/client_profile/offers/encryption_algorithms_client_to_server/1|暗号化候補：クライアントからサーバー|aes256-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_client_to_server/2|暗号化候補：クライアントからサーバー|aes128-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_client_to_server/3|暗号化候補：クライアントからサーバー|aes256-ctr|
|/details/client_profile/offers/encryption_algorithms_client_to_server/4|暗号化候補：クライアントからサーバー|aes128-ctr|
|/details/client_profile/offers/encryption_algorithms_server_to_client/0|暗号化候補：サーバーからクライアント|chacha20-poly1305@openssh.com|
|/details/client_profile/offers/encryption_algorithms_server_to_client/1|暗号化候補：サーバーからクライアント|aes256-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_server_to_client/2|暗号化候補：サーバーからクライアント|aes128-gcm@openssh.com|
|/details/client_profile/offers/encryption_algorithms_server_to_client/3|暗号化候補：サーバーからクライアント|aes256-ctr|
|/details/client_profile/offers/encryption_algorithms_server_to_client/4|暗号化候補：サーバーからクライアント|aes128-ctr|
|/details/client_profile/offers/kex_algorithms/0|鍵交換方式の候補|mlkem768x25519-sha256|
|/details/client_profile/offers/kex_algorithms/1|鍵交換方式の候補|sntrup761x25519-sha512|
|/details/client_profile/offers/kex_algorithms/2|鍵交換方式の候補|sntrup761x25519-sha512@openssh.com|
|/details/client_profile/offers/kex_algorithms/3|鍵交換方式の候補|curve25519-sha256|
|/details/client_profile/offers/kex_algorithms/4|鍵交換方式の候補|curve25519-sha256@libssh.org|
|/details/client_profile/offers/languages_client_to_server|言語候補：クライアントからサーバー|\[\]|
|/details/client_profile/offers/languages_server_to_client|言語候補：サーバーからクライアント|\[\]|
|/details/client_profile/offers/mac_algorithms_client_to_server/0|MAC候補：クライアントからサーバー|hmac-sha2-256-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_client_to_server/1|MAC候補：クライアントからサーバー|hmac-sha2-512-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_client_to_server/2|MAC候補：クライアントからサーバー|hmac-sha2-256|
|/details/client_profile/offers/mac_algorithms_server_to_client/0|MAC候補：サーバーからクライアント|hmac-sha2-256-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_server_to_client/1|MAC候補：サーバーからクライアント|hmac-sha2-512-etm@openssh.com|
|/details/client_profile/offers/mac_algorithms_server_to_client/2|MAC候補：サーバーからクライアント|hmac-sha2-256|
|/details/client_profile/offers/server_host_key_algorithms/0|ホスト鍵方式の候補|ssh-ed25519|
|/details/client_profile/offers/server_host_key_algorithms/1|ホスト鍵方式の候補|ecdsa-sha2-nistp256|
|/details/client_profile/offers/server_host_key_algorithms/2|ホスト鍵方式の候補|rsa-sha2-512|
|/details/client_profile/offers/server_host_key_algorithms/3|ホスト鍵方式の候補|rsa-sha2-256|
|/details/client_profile/purpose|用途|diagnostic_proposal_only|
|/details/client_profile/reserved|予約フィールド|0|
|/details/early_messages|初期メッセージ数|0|
|/details/extension_markers|拡張マーカー|\[\]|
|/details/first_kex_packet_follows|後続の推測KEXパケットの申告|null|
|/details/host_key_candidates|host_key_candidates|\[\]|
|/details/kex_candidates|kex_candidates|\[\]|
|/details/offers|offers|null|
|/details/pre_identification_lines|識別前の行数|0|
|/details/server_identification|未検証のサーバー申告|null|
|/diagnostics/0|診断コード|address_denied|
|/generated_at|作成日時|2026-11-17T06:30:53.400131256+00:00|
|/limits/dns_timeout_ns|名前解決の制限時間（ナノ秒）|5000000000|
|/limits/max_dns_answers|名前解決結果の上限|8|
|/limits/max_received_bytes|受信上限（バイト）|524288|
|/limits/max_sent_bytes|送信上限（バイト）|65536|
|/limits/overall_timeout_ns|全体の制限時間（ナノ秒）|15000000000|
|/limits/protocol_timeout_ns|プロトコル処理の制限時間（ナノ秒）|5000000000|
|/limits/tcp_timeout_ns|TCP接続の制限時間（ナノ秒）|3000000000|
|/outcome|観測の完了状況|要求した観測を完了できませんでした (failed)|
|/protocol|観測方法|ssh_offers|
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
