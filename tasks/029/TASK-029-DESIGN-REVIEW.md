# Task 029 独立設計・データフローレビュー

対象: base `4324d2c31e96ce62de211882192d6c4a8acded6d` の現行ソースと
`docs/superpowers/specs/2026-11-10-evidence-privacy-design.md`、対応 plan。
範囲は synthetic regression の設計と source-only review。実行、漏出再現、
ビルド、テスト、性能測定、最終実装承認は行っていません。

## 判定

post-identity / post-native-association の legacy redaction と、共有 policy を
使う writer 境界の防御は妥当です。公開モデル・ルール・CLI・ID 計算を変えずに
実装できます。下記の行対応・位置解釈・事実保持を実装時の必須条件とします。
未実装なので最終承認ではありません。

## ソースから確認した経路

- `pipeline.rs:140-151`: dependency inventory → normalize/ID → recommendation →
  snippets → sort → CBOM。`make_finding_id` は既存 match をハッシュします。
- `pipeline.rs:179-190`: `ScanDocument` 構築後、同じ未変更 findings を
  `NativeScan::build` に渡します。
- `compat.rs:47-78,108-139`: native の association/loss 判定には rule/path/
  line/column/**match equality** が必要です。redaction を先に行うと links/loss が
  変化します。最終 findings の順番・件数も維持する必要があります。
- `pipeline.rs:540-558`: 既存コードでも private PEM marker 六種および
  dependency/SBOM containers は source snippet をファイル全体でマスクします。
  六種の snippet 未対応を新規問題として扱う根拠はありません。
- `model.rs:208-225`: detector の `mask_preview` は長文を head/tail に短縮します。
  短い body-only match や変形済み preview には marker/assignment が残らない場合が
  あるため、text sanitizer だけでは source context の代替になりません。
- `report/src/lib.rs:156-167`: 現行 writer sanitization は evidence.match /
  snippet_preview / source snippet のみ。metadata/actions/CBOM/dependency 自由文は
  この関数では処理しません。これは source 上の未保護経路の確認であり、
  実際の supported canary 漏出を検証したとの主張ではありません。
- native は `observation.rs` の typed allowlist と `inventory.rs` の検証を経て、
  raw match/snippet/arbitrary metadata をそのまま serialize しない設計です。
  `diagnostic.rs:105-118` の診断 message は固定文で detector error を転記しません。

## 必須実装条件と最小修正案

### 1. CBOM の行対応・件数を保存する

`pipeline.rs:397-427` の CBOM dedup key は rule/location/**元match** です。
redacted findings から CBOM を再生成すると、同じ location にある別 match が
同じ placeholder へ収束して行が失われます。反対に CBOM を単独で text sanitize
するだけでは、marker のない body-only algorithm を context から消せません。

最小案: 元の dedup 順・行対応を保って各 CBOM 行を更新するか、元 key を使って
作成した CBOM と対応 finding の redacted evidence を同期します。raw を返却用
追加フィールドへ保存しません。writer に caller が直接渡した CBOM は独立に
text sanitize します。二つの別 match が同じ placeholder になる synthetic control
で CBOM 行数・順序・location・risk が維持されることを確認してください。

### 2. original read の位置情報から context を作る

byte spans / line spans は detector に渡した `ScannableFile` と同じ text から
作成してください。後段でファイルを再読込して位置だけを参照すると、変更済み
ファイルによって evidence と context がずれます。context は spans のみ保持し、
private payload のコピーや新しい fingerprint を導入しない設計が適切です。

regex columns は `line_col_for_offset` の Unicode scalar 単位ですが、AST は
tree-sitter の byte column と relative scalar column の混合です
(`tree_sitter_detector.rs:145-153,273`)。同じ column を常に byte offset として
処理しないでください。曖昧な場合の保守的 line envelope は raw evidence のみに
適用し、normalized algorithm/OID/key-size/version/location を消さないでください。
token が credential value の途中から一致する例、Unicode prefix、複数行 match、
同一行の benign algorithm と credential value の両立が有用な controls です。

### 3. field provenance に応じた policy を適用する

metadata では `password: short-canary` のように field 名と値が分離されます。
値の text sanitizer に加え、指定した credential key の値全体をマスクする
処理が必要です。key 名の部分一致で `key_size` / `public_key_oid` / certificate
fingerprint まで消さないでください。自由文の assignment は quoted/unquoted/
JSON の supported syntax を具体化し、HTTP(S) userinfo は host/path の事実を
保存しつつ credentials を除去してください。

Finding description/migration_hint、action title/rationale/steps/references と
before/after code、CBOM algorithm/migration_hint、dependency name/version/purl 等の
自由記述欄を同じ helper で扱う方向に賛成します。rule IDs/action IDs/path/enum /
technical numeric facts は structural data として保持してください。metadata key
自体を変更する場合の collision は避け、値の sanitize を基本にしてください。

### 4. 独立経路と限界を区別して検証する

legacy writer を通らない inventory / assessment / diagnostics の保護は、
legacy sanitizer の試験成功から推定できません。それぞれ typed facts / fixed
diagnostics の source provenance を確認し、対応する synthetic fixture で確認して
ください。caller-provided `ScanResult` には元 source context がないため、任意の
短い body-only string を秘密と判断する保証は作れません。この限界と、operators
由来 path/ID を非秘密とする前提を両 README/契約文書にも明記してください。

六種 PEM marker の privacy 対応は新たな key detector/parser 対応を意味しません。
既存 ID hash basis の維持は承認済み互換性要件ですが、新たな secret hash を追加
しない方針と混同しないでください。万能の秘密検出・全ての任意文字列の非出力を
保証する文言は避けてください。

## 最終レビューで確認する証拠

新規 synthetic canary の実失敗→修正後成功、返却 core result と全 legacy outputs、
両言語、CBOM 行対応、inventory references/loss records、safe technical facts、
ID/order/count parity、caller-created freeform fields、diagnostics/stderr。
本レビューはこれらを実行しておらず、未検証項目を成功扱いしていません。
