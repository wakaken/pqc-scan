# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 1 · 評価時刻: <code>2026&#45;11&#45;18T13&#58;36&#58;10Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>system&#45;a</code>](#target-1) · <code>app&#46;txt&#58;1</code> | <code>RSA</code> · 暗号方式への参照 | 早急に対応 | 調査 |

<a id="target-1"></a>

## <code>system&#45;a</code> · <code>RSA</code>

**早急に対応** · 不足情報あり · 準備状況は不明

場所: <code>app&#46;txt&#58;1</code> (暗号方式への参照)

### 次の対応

調査

期限到来の申告が失効しています。早急に調査し、申告を更新してください。

**判断に必要な追加情報**

- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

<a id="candidate-1"></a>

**<code>ML&#45;DSA</code> · 署名・認証**

組込みは未検証です。採用前に正確なパラメーターを選び、互換性を検証してください。

| 必要な条件 | 申告上の対応状況 |
|---|---|
| 実行環境の対応 | 不明 |
| プロバイダーの対応 | 不明 |
| 正確なパラメーターへの対応 | 不明 |
| 接続相手の対応 | 不明 |
| プロトコルの対応 | 不明 |

<a id="evidence-1"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>RSA</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 判断に関係する申告が現在有効ではありません。更新が必要です。

- [対象範囲の期限](#claim-1)
- [プロバイダーの対応](#claim-2)
- [必要な用途](#claim-3)
- [適用範囲](#claim-4)
- [利用中・利用予定の状況](#claim-5)
- [<code>F12</code>](#source-1)
- [<code>F13</code>](#source-2)
- [<code>F14</code>](#source-3)
- [<code>LOCAL026</code>](#source-4)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)
- [<code>context&#58;org&#45;deadline</code>](#source-7)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad</code>

判断: <code>migration&#95;decision&#45;v1&#45;d65af84c906ba2e69d1101b7d0fa2894bcb0088a04e472351f6da1721ea790f8</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

<a id="claim-1"></a>

### 対象範囲の期限

<code>2026&#45;11&#45;16T00&#58;00&#58;00Z · 参照資料&#58; org&#45;deadline</code> · 申告は失効

申告者: <code>owner</code> · 外部レビュー由来の申告（未検証）

申告時刻: <code>2026&#45;11&#45;16T00&#58;00&#58;00Z</code> / 有効期限: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### プロバイダーの対応

<code>不明</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-3"></a>

### 必要な用途

<code>署名・認証</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-4"></a>

### 適用範囲

<code>対象</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-5"></a>

### 利用中・利用予定の状況

<code>利用中</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

## 判断の出典

<a id="source-1"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-2"></a>

**<code>F13</code> · <code>task066 adopted time contract 2026&#45;11&#45;18</code>**

<code>time and policy semantics&#59; FACT&#45;SOURCE&#45;MAPPING F13</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-3"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-6"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

<a id="source-7"></a>

**<code>context&#58;org&#45;deadline</code> · <code>v1</code>**

<code>3&#46;1</code> · <code>https&#58;&#47;&#47;synthetic&#46;invalid&#47;never&#45;fetch</code>

## 評価の範囲

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>ea072032ff581bb287e010de83d5c9be0f72debdce0962925dfa3c5dbf29f81f</code>

収集時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

