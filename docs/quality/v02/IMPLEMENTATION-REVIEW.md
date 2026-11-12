# v0.2 closeout 実装レビュー

日付: 2026-11-12。担当: v02_implementation_review（実装レビュー専任、追加委譲なし）。
対象: INITIAL-SOURCE.jsonを起点とするCURRENT-REQUEST-FILES.jsonの54ファイルのcloseout変更と既存修正との接続。現在ブランチcodex/inspection-coverage-fixesと既存実装を保持。

## 判定

実装を承認します。今回の対象に、正確性・要件・互換性・公開範囲について修正必須の指摘はありません。最終テストの適切性、実際の候補コミット/マージコミットCI、ホスト型配布の実行結果は後続の独立したゲートであり、本レビューの承認から成功を推定しません。

## 確認内容

- package-release.pyの共通validatorとcheck-install.pyの呼び出しを確認。3つの承認済みIDの集合一致と件数一致を組み合わせ、欠落・重複・余分な検出を拒否します。全検出をexample.py:1に制限し、2ファイルの走査を要求するためplain.pyの陰性対照が検出されれば失敗します。既存variant/command-surface検証も維持しています。
- 自己scanのpolicyはパス、現ソースSHA、完全SARIF resultのcanonical SHA、rule/location、出現数に限定されます。sourceの変更/不明、未登録result、過剰重複を保持する既存fail-closed処理は変更されていません。生成用build-review.cjsはtarget配下のオフライン記録器でありCIの自動学習経路ではありません。
- ledgerの全1357行で実ファイルのsource SHA、canonical result SHA、path、line、ruleIdを独立照合し不一致0件。行番号のある全行でsourceLineSha256も実データと照合しました。DER等は記録器と同じUTF-8置換表現の行ハッシュで、バイナリ本体は完全source SHAで別途拘束されます。行なしinventoryはline/hashがnullです。
- 新規のinline_test/identifierを実際のcfg(test)配下の原文と照合し、通常コード側の除外は観測値の正規化表、OID表示名、説明辞書、レポート中の参考例、ルール定義に限られることを確認。例/テストファイルの具体的な登録と依存inventoryの保持を確認。1317 expected/40 retainedの分割は有限の個別観測に対するもので、ディレクトリ全体の免除ではありません。
- workspace各crateとstandalone feasibility lockの製品版だけを0.2.0へ更新。inventory v1、SARIF 2.1.0、CycloneDX 1.7等を製品番号として置換していません。古い測定/ソースbinding JSONに残る旧ロードマップパスは履歴であり、現行リンクに旧ファイル参照は残っていません。
- README両言語、配布ガイド、SECURITY、release notes、ロードマップ冒頭は静的scan正式/probes実験的/macOS arm64公開対象と一致。014/016 cancelled、023 baseline done、固定真値と同一環境2x調査基準を保持しています。タグ/Release公開やライセンス確認済みとの虚偽表明は追加していません。
- release.ymlは手動起動の準備workflowのまま。static archiveのみGitHub来歴を生成し、repo/signer workflow/source digestを実行時の対象コミットに制限して検証します。actionはSHA固定、権限はcontents read/id-token write/attestations write、秘密鍵やRelease upload処理はありません。未解決の実験的probe notice flagsは維持されています。

## 検証証拠と限界

main提供のtarget/v02-closeout/distribution-unit.log（11件成功）、node-early.log（34件成功）、install-red.log→install-green.log（新validator欠落で失敗後成功）を読みました。これらは事前/局所テストの証拠であり、最終source全体の受入結果とは混同していません。レビュー担当はフルsuiteを重複実行していません。レビュー中にRust/Python/Node本体や公開文書を変更していません。

mainが進行中の最終検証、実archiveのoffline clean-install、基準/性能比較、別identityのテストレビュー、正確な最終コミットのCI/配布workflow成功および公開文書への結果追記が完了してから全体をcloseしてください。
