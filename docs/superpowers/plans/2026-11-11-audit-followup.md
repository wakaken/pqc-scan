# Cursor監査対応計画

目的: 2件の監査を作業ツリーと照合し、既存判断と互換性を保持して確認済みの不備を修正する。
主担当が実施し、実装後1名・最終検証後に別の1名だけがレビューする。
参照仕様: /tmp/pqc-scan-audit-handoff-2vf5uf7i/HANDOFF.md、両Cursor監査のFINDINGS.md。

- [x] core: walkerで特殊ファイルとリンクを読取失敗として記録。通常ファイルの読取は上限+1 byteで打切り、Unixはopenat/O_NOFOLLOW/O_NONBLOCKで各相対成分を確認する。検出・snippet・推奨情報の読取を同じ制限にする。FIFO、外部リンク、途中ディレクトリ差し替え、サイズ境界、既存ignoreをテストする。
- [x] CLI: 未列挙母数の留保をstderrに表示する。既存の範囲内完了・require-complete・fail-on終了コードは維持し、無所見のignore fixtureで確認する。日英READMEの範囲とHTML/Markdown説明を一致させる。
- [x] TLS: 成功詳細の設定前に必要なversion/suiteを検査し、失敗時の成功フラグ残留を防ぐ。暗号方式や真正性検証は変更しない。アクセス値欠落を単体テストで再現し、実TLS回帰も確認する。
- [x] 管理記録: task IDをすべて引用文字列にし、YAML1.1/1.2の解析・依存整合を検証。001の古い014再開指示とSTATUSの誤った衝突説明を訂正。既存004/014/016/023差分と凍結測定値は保持する。
- [x] protocol: runnerの環境・ファイル境界を限定し、HOME読取・source書込・外部接続の拒否を実測する。ビルド時と実行時の隔離範囲を別記し、過去証拠を厳格な包括隔離とは呼ばない。OpenSSHの過去source bindingを照合し、再実行可能性を確認する。
- [x] 実装レビュー1名。指摘を主担当が修正し必要な回帰を実行する。
- [x] fmt/build/default+all-feature tests/clippy、feature 8組、関連Python/Node、必要なCLI検証を記録する。
- [x] テストレビュー1名。最終ソースに対する検証の十分性を確認し、監査照合表を仕上げる。

範囲外: legacy finding_id/SARIF同一性の変更、新しい保存形式、公開/push/署名、014/016再開、ルール・暗号方式変更。legacy IDの衝突は認めるが、互換方針の人手確認が必要として残す。CIとobjc4件は未完了を維持する。

結果と未達条件は docs/records/cursor-audit-20261111/CURSOR-AUDIT-FOLLOWUP-2026-11-11.md に集約。最終テストレビューは承認・必須改善0で完了し、証拠JSONへ記録した。
