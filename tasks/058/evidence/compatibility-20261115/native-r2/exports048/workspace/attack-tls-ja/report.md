# TLS接続先の観測結果

検証状態と通信で決まった事実は、この接続先とクライアント設定に対する1回の観測を示します。収集上限と診断情報を併せて確認してください。サーバーの全能力やPQC移行完了を保証する結果ではありません。

秘密情報に見える文字列を省略しました。該当箇所の原文はこの出力から復元できません。観測完了は収集の状態を示すもので、この省略がなかったという意味ではありません。


## 観測の概要

- 診断コード: address_denied
- 観測の完了状況: 要求した観測を完了できませんでした (failed)
- 実際の接続相手: null
- 要求したホスト: localhost
- 要求したポート: 443
- ソケットで受信したバイト数: 0
- ソケットで送信したバイト数: 0

|項目パス|項目|内容|
|---|---|---|
|/attempt_id|今回の実行内の試行番号|1|
|/details/credential|credential|\[masked-sensitive-content\]|
|/details/untrusted|untrusted|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " &amp; 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;|
|/diagnostics/0|診断コード|address_denied|
|/generated_at|作成日時|2026-11-15T07:16:05.978227+00:00|
|/limits/dns_timeout_ns|名前解決の制限時間（ナノ秒）|5000000000|
|/limits/max_dns_answers|名前解決結果の上限|8|
|/limits/max_received_bytes|受信上限（バイト）|524288|
|/limits/max_sent_bytes|送信上限（バイト）|65536|
|/limits/overall_timeout_ns|全体の制限時間（ナノ秒）|15000000000|
|/limits/protocol_timeout_ns|プロトコル処理の制限時間（ナノ秒）|5000000000|
|/limits/tcp_timeout_ns|TCP接続の制限時間（ナノ秒）|3000000000|
|/outcome|観測の完了状況|要求した観測を完了できませんでした (failed)|
|/protocol|観測方法|tls|
|/redaction_applied|秘密情報に見える文字列の省略|true|
|/scanner_version|スキャナーの版|0.2.0|
|/schema_version|スキーマの版|1|
|/target/connected_peer|実際の接続相手|null|
|/target/host|要求したホスト|localhost|
|/target/pinned_address|固定した接続先|null|
|/target/port|要求したポート|443|
|/target/resolved_addresses|確認した名前解決結果|null|
|/traffic/received_bytes|ソケットで受信したバイト数|0|
|/traffic/sent_bytes|ソケットで送信したバイト数|0|
