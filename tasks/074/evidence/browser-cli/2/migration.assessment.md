# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 3 · 評価時刻: <code>2026&#45;11&#45;18T13&#58;33&#58;10Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>system&#45;1</code>](#target-1) · <code>main&#46;py&#58;1</code> | <code>RSA</code> · 暗号方式への参照 | 早急に対応 | 阻害条件を解消する |
| [<code>system&#45;1</code>](#target-2) · <code>main&#46;py&#58;2</code> | <code>RSA</code> · 暗号方式への参照 | 早急に対応 | 阻害条件を解消する |
| [<code>未設定</code>](#target-3) · <code>main&#46;py&#58;2</code> | <code>RSA</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |

<a id="target-1"></a>

## <code>system&#45;1</code> · <code>RSA</code>

**早急に対応** · 不足情報あり · 未対応の条件あり

場所: <code>main&#46;py&#58;1</code> (暗号方式への参照)

### 次の対応

阻害条件を解消する

業務上重要な署名用途での利用中・利用予定が申告されています。

**申告された阻害条件**

- 接続相手の対応

**判断に必要な追加情報**

- 正確なパラメーター

**次の確認**

- 下記の申告された阻害条件を解消してください。
- 実装と相互運用性を別途検証してください。

### 移行の検討候補

<a id="candidate-1"></a>

**<code>ML&#45;DSA</code> · 署名・認証**

組込みは未検証です。採用前に正確なパラメーターを選び、互換性を検証してください。

| 必要な条件 | 申告上の対応状況 |
|---|---|
| 実行環境の対応 | 申告上は対応 |
| プロバイダーの対応 | 申告上は対応 |
| 正確なパラメーターへの対応 | 申告上は対応 |
| 接続相手の対応 | 申告上は未対応 |
| プロトコルの対応 | 申告上は対応 |

<a id="evidence-1"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>RSA</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>文字列の一致</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。

- [適用範囲](#claim-1)
- [接続相手の対応](#claim-2)
- [業務上の重要性](#claim-7)
- [プロトコルの対応](#claim-8)
- [利用中・利用予定の状況](#claim-9)
- [正確なパラメーターへの対応](#claim-11)
- [実行環境の対応](#claim-13)
- [必要な用途](#claim-16)
- [プロバイダーの対応](#claim-17)
- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;833a272f120091d772636f5b99fa87f453ebcbcce476e36d9e06579fc339301b</code>

判断: <code>migration&#95;decision&#45;v1&#45;7da0ed448d690ecc6cc6dfd323e339134482f4f4337f697c74e541920737af8f</code>

</details>

<a id="target-2"></a>

## <code>system&#45;1</code> · <code>RSA</code>

**早急に対応** · 不足情報あり · 未対応の条件あり

場所: <code>main&#46;py&#58;2</code> (暗号方式への参照)

### 次の対応

阻害条件を解消する

業務上重要な署名用途での利用中・利用予定が申告されています。

**申告された阻害条件**

- 接続相手の対応

**判断に必要な追加情報**

- 正確なパラメーター

**次の確認**

- 下記の申告された阻害条件を解消してください。
- 実装と相互運用性を別途検証してください。

### 移行の検討候補

<a id="candidate-2"></a>

**<code>ML&#45;DSA</code> · 署名・認証**

組込みは未検証です。採用前に正確なパラメーターを選び、互換性を検証してください。

| 必要な条件 | 申告上の対応状況 |
|---|---|
| 実行環境の対応 | 申告上は対応 |
| プロバイダーの対応 | 申告上は対応 |
| 正確なパラメーターへの対応 | 申告上は対応 |
| 接続相手の対応 | 申告上は未対応 |
| プロトコルの対応 | 申告上は対応 |

<a id="evidence-2"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>RSA</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。

- [接続相手の対応](#claim-3)
- [プロバイダーの対応](#claim-4)
- [正確なパラメーターへの対応](#claim-5)
- [必要な用途](#claim-6)
- [実行環境の対応](#claim-10)
- [利用中・利用予定の状況](#claim-12)
- [プロトコルの対応](#claim-14)
- [適用範囲](#claim-15)
- [業務上の重要性](#claim-18)
- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;d9755eec083e80e2194e05546c1f874afaa0fee4eaddf93377b14833d4cda89f</code>

判断: <code>migration&#95;decision&#45;v1&#45;630327413d504a7074940524104e897504ac800556ea8741414690325da01d36</code>

</details>

<a id="target-3"></a>

## <code>システム未設定</code> · <code>RSA</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>main&#46;py&#58;2</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応
- 正確なパラメーター
- 利用中・利用予定の状況
- 必要な用途

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-3"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>RSA</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;6d0df6bf319b64a2ca3950337ed166c9c4ec88d855cd61a9a858c24cbc0aaf0d</code>

判断: <code>migration&#95;decision&#45;v1&#45;f6d95265829d5b63ade9f7f75d3df8484cdfc86c0a7829bbc05f9b87618b8ef6</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

<a id="claim-1"></a>

### 適用範囲

<code>対象</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### 接続相手の対応

<code>申告上は未対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-3"></a>

### 接続相手の対応

<code>申告上は未対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-4"></a>

### プロバイダーの対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-5"></a>

### 正確なパラメーターへの対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-6"></a>

### 必要な用途

<code>署名・認証</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-7"></a>

### 業務上の重要性

<code>はい</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-8"></a>

### プロトコルの対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-9"></a>

### 利用中・利用予定の状況

<code>利用中</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-10"></a>

### 実行環境の対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-11"></a>

### 正確なパラメーターへの対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-12"></a>

### 利用中・利用予定の状況

<code>利用中</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-13"></a>

### 実行環境の対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-14"></a>

### プロトコルの対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-15"></a>

### 適用範囲

<code>対象</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-16"></a>

### 必要な用途

<code>署名・認証</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-17"></a>

### プロバイダーの対応

<code>申告上は対応</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-18"></a>

### 業務上の重要性

<code>はい</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

## 判断の出典

<a id="source-1"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-2"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-3"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-5"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## 評価の範囲

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>7bdc870267610e8b3cba99a8a16e6cfb10a3bced9bd5b051019328ea2efe86b5</code>

収集時刻: <code>2026&#45;11&#45;18T13&#58;33&#58;08&#46;805046&#43;00&#58;00</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

