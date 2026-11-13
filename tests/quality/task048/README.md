# TASK-048 検証手順

元の計画・受入条件は`tasks/048-quality-output-contracts.yaml`、結果は
`docs/quality/TASK-048-RESULTS.md`を参照してください。

Rustは`rust-toolchain.toml`の1.93.1、validatorは既存のjsonschema 0.29.1です。
Schema取得機能を無効にし、同梱のSARIF・CycloneDXと独自v1 Schemaだけを使います。
`schema-pins.json`はlicenseを含むhashと由来を固定します。
依存の準備後は`--locked --offline`で試験し、検証時にSchemaを取得しません。

主担当の保存用runner:

```sh
python3 tests/quality/task048/verify.py target/quality-task-048
```

通常・全機能のtest/clippy/build、fmt、Python品質試験、既存のcompact.js試験、
集約を逐次実行します。`SOURCE.json`と`VALIDATION.json`、生ログを保存し、
失敗時は非0で停止します。再実行時は過去の証拠を上書きしない新しい出力先を使います。
個別試験は`PQC048_EVIDENCE`を指定しなくても通常のcargo testで実行できます。

Markdown変換にはこの評価環境に既存の`markdown-it-py==3.0.0`を使います。
製品の依存には追加しません。CommonMarkのraw HTMLとtableを有効にし、
サニタイザーを通す前の構造を検査します。集約だけを再実行する場合:

```sh
python3 tests/quality/task048/contracts.py \
  --raw target/quality-task-048/raw --out target/quality-task-048
```

生成物は`schema-results.json`、`output-semantics.json`、`render-structural.json`と
`browser/`です。`render-security.md`には主担当が実ブラウザーでの観察を記録します。
生の検査成功だけではブラウザー受入を満たしません。

配信は生成レポートだけの専用ディレクトリを指定します:

```sh
python3 -m http.server 8048 --bind 127.0.0.1 \
  --directory target/quality-task-048/browser
```

日英の静的・TLS・SSH offers・SSH KEX、HTMLと変換Markdownの16ページで、
攻撃文字列がテキストとして残り実行されないことを観察します。
`control.html`は検証器の陽性対照としてタイトルを変更する合成ページです。
キーボードによる詳細展開・ページ切り替え、検索、読込後に配信を停止した状態も確認します。
実際のbrowser/viewport、URL、スクリーンショットと配信原本を一緒に保存してください。

probeのSchema試験は実CLIのaddress_denied出力を使います。
別の攻撃writer試験は未知項目入りの合成値であり、protocol Schemaの正例とは呼びません。
交渉・認証の網羅検証、任意の外部宛先の診断は行いません。
