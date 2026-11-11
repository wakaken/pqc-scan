# Finding identity統一計画

利用者指示: 全finding_idを新方式へ統一。ベータ版のため旧方式併用・移行処理・移行の注意書きは不要。
主担当が実装・検証し、実装後1名・最終検証後に別の1名がレビューする。既存差分は保持する。

仕様: identity::native_idのdomain= finding、version1、長さ付きtuple（rule_id、出力と同じ相対file、行、列、evidence match）からfull SHA-256を生成し、finding-v1-<64hex>とする。位置/文字列の結合境界は曖昧にしない。出力フィールドやasset/observation IDの意味は変えない。重複除去も同じIDを用いる。

- [x] pipeline境界衝突・rule/path境界・Unicode・root非依存・完全重複のテストを先行実行。既存native_captureの衝突・delimiter collapse期待を、各所見とassociationを保つ期待へ変更。
- [x] 旧make_finding_idアルゴリズムとdelimiter dedupを置換。JSON/HTML/native associationの新ID整合をCLIで確認し、既存SARIFがfinding_idを持たないことも確認。
- [x] 日英READMEと現行architecture記述を事実に合わせる。旧ID併用や移行警告は追加しない。
- [x] 実装レビュー1名。指摘があれば主担当が修正。
- [x] fmt、build、workspace test、all-feature test/clippy、Nodeと自己スキャンの変更行照合を実施。期待値の無条件緩和はしない。
- [x] 別の1名が最終テストレビュー。記録更新と最終説明。
