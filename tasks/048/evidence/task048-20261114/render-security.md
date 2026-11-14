# TASK-048 表示と秘匿の検証

対象ソースはHEAD `c1bc58e59baffdee73beae73f79b1f9a68804bd2`と今回の未コミット差分です。正確なファイルhashはレビュー記録と最終ソース台帳で結び付けます。

## 実測

- 静的writerとTLS・SSH offers・SSH KEX writerに、script終端・script要素・img/onerror・svg/onload・javascript/data URI・引用符・Markdownリンク・パイプ・バッククォート・CRLF/CR・日本語/絵文字を入力しました。probeの攻撃入力はwriter境界の合成オブジェクトであり、protocol Schema適合や通信成功の正例とは区別します。
- 日英8群のHTMLとMarkdown変換結果、計16ページをGoogle Chromeで開きました。viewportは1466×1381です。全ページで攻撃文字列はテキストとして存在し、注入された実行要素・イベント属性・危険なリンクは0、合成秘密の残存も0でした。ページ横幅はclient/scrollとも1466でした。
- Markdown変換は既存のmarkdown-it-py 3.0.0のCommonMarkにtableを有効化し、raw HTMLも許可しました。サニタイザーで不具合を隠していません。HTML/Markdownの原本・変換結果・hash・スクリーンショットは`browser-observations/`に保存しています。
- 別の陽性対照ではscriptがタイトルを`EXECUTED048`へ変えました。検証対象16ページではそのタイトルになりませんでした。
- 日本語HTMLで検索の0/25表示と解除を確認し、Enterでルール詳細を開き、次の箇所へ切り替えて21–25/25を確認しました。検索欄からTabも操作しました。
- loopbackサーバーを停止した状態でも、読込済みページの検索・詳細展開・ページ切り替えが機能しました。新規ナビゲーションは接続拒否となり、停止状態と整合しています。確認後は閲覧用に専用サーバーを再開しました。

## 修正と限界

Markdownのsource snippetは、埋め込まれた改行の後を字下げしていませんでした。code exampleはLFだけを字下げし、単独CRを取りこぼしていました。修正前の実出力をCommonMarkで変換すると、imgとscriptがコード外に現れることを再現しました。source/codeをCRLF/CR正規化後に全行字下げし、通常テキストのCRも空白にします。

JSON/SARIFはJSONとして解析し、秘匿と所見保持を別に確認します。JSON文書をHTMLのscriptへ直接埋め込む使い方は対象ではありません。合成秘密は既存の対応構文`password=...`を使い、未知の秘密構文の保証には広げません。

ブラウザーはChromeの上記1 viewport、Markdownは上記1 rendererでの実測です。全ブラウザーや全Markdown処理系の保証ではありません。製品の公開Schema・ルール・暗号判断は変えていません。

閲覧URL: http://127.0.0.1:8048/attack-static-ja/report.html
配信対象は`target/quality-task-048/browser/`のみ、bind先は`127.0.0.1`です。
