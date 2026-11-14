# Finding identity

すべてのスキャン所見は `finding-v1-<64桁の小文字SHA-256>` を使用する。
`identity::native_id` の共通エンコーダでdomainを`finding`とし、rule ID、
リポジトリ相対パス、行、列、元のevidence matchを長さ付きtupleとして符号化する。
同一tupleは同一IDとなり、異なるフィールド境界を混同しない。
重複除去もこのIDを使う。asset/observationはそれぞれのdomainを使う。

JSON、HTMLのdata-finding-id、native associationは同じ生成値を参照する。
SARIFは従来からfinding_idを含めない形式であり、今回も追加していない。
スキーマのassociation IDパターンは新形式のみを許可する。

## 検証 — 2026-11-11

`target/finding-id/` に以下の証拠を保存した。

- identity-red.log: 旧実装でフィールド境界の衝突とcheckout root依存を再現。
- capture-red.log: 旧実装で異なる所見のID衝突・delimiter dedupの欠落を再現。
- core-green.log / capture-green.log: 固定SHA-256 vector、境界・Unicode・空値・root非依存・完全重複・関連付けを検証。
- exports-final.log: JSON/HTML/nativeの新ID一致、スキーマの旧形式・誤version・短いIDの拒否、秘密情報のmask維持。
- workspace.log: 356成功。
- protocol/RESULTS.json: 全feature407成功・既存OpenSSH fixture1件ignore、55実行ファイルのhashとsource前後一致。
- clippy.log: 全feature・全targetで警告なし。
- fmt.log / build.log: formatとworkspace build成功。
- node.log / python-quality.log: 28件 / 21件成功。

自己スキャン期待値は、開始時source snapshotと旧raw SARIFを現物照合し、
未変更行32記録・1ファイルだけline/hashを更新した。
新規expectedは0件。新raw1113件のうち既存expected1004件、保持109件。
self-scan/REBIND-LEDGER.jsonとrebind-self-scan.pyに対応を記録。
精度測定用コーパス、truth、凍結した測定値、既存の取消・承認判断は保持した。

実装レビューは1名が承認し、必須指摘0。
最終テストレビューも別の1名が承認し、必須改善0。
source/evidence/binaryのSHAと自己スキャン32記録を独立照合した。
詳細はFINDING-IDENTITY-VALIDATION.jsonとtarget/finding-id/VALIDATION.jsonに保存した。
