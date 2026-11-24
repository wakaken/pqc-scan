# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 3 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>未設定</code>](#target-1) · <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> | <code>unknown</code> · 通信の鍵確立 | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-2) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> | <code>RSA&#45;MD5</code> · 証明書の署名 | 早急に対応 | 調査 |
| [<code>未設定</code>](#target-3) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> | <code>RSA</code> · 証明書の公開鍵 | 緊急度を判断できない | 調査 |

<a id="target-1"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> (通信の鍵確立)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応
- 正確な暗号方式の識別
- 正確なパラメーター
- 利用中・利用予定の状況
- 必要な用途

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-1"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>検証未完了、または収集情報が不完全</code> |
| 収集状況 | <code>収集失敗</code> |

- この対象の情報収集は未完了です。
- 検証は完了していません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;7dc4de55d949e725fee8cca5d0130e03814c88fddf71b8b5f099a30e993bb2d3</code>

判断: <code>migration&#95;decision&#45;v1&#45;06dd1f8c8cd72f92d491fae6496d501cf5cea0a40444c0bb01a439b893f0dc60</code>

</details>

<a id="target-2"></a>

## <code>システム未設定</code> · <code>RSA&#45;MD5</code>

**早急に対応** · 不足情報あり · 準備状況は不明

場所: <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> (証明書の署名)

### 次の対応

調査

保存された事実に、用途に関係する従来暗号の懸念があります。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- 接続相手の対応
- PKIの対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- パラメーターの符号化
- 利用中・利用予定の状況
- 必要な用途

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
| PKIの対応 | 不明 |

<a id="evidence-2"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>RSA&#45;MD5</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>収集失敗</code> |

- この対象の情報収集は未完了です。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F08</code>](#source-2)
- [<code>F12</code>](#source-3)
- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)
- [<code>R6151</code>](#source-7)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;e872792348a59baca96dd26560dbbb7d0e9bb76943b882703904f1ee984c7604</code>

判断: <code>migration&#95;decision&#45;v1&#45;57af0a250c2273ad6a6333ac09de78dde50df961c5d5f3640b743b42dfb84a96</code>

</details>

<a id="target-3"></a>

## <code>システム未設定</code> · <code>RSA</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> (証明書の公開鍵)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応
- パラメーターの符号化
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
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>収集失敗</code> |

- この対象の情報収集は未完了です。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F08</code>](#source-2)
- [<code>F12</code>](#source-3)
- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;4696c757de0244243d537a84a174637883f59cf7f6c02ed30a8c991bee6cad83</code>

判断: <code>migration&#95;decision&#45;v1&#45;fcda13859d2e6064f6a32e2bb3337103a31f4c0954c1f292f319db3aaaf83e8e</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

## 判断の出典

<a id="source-1"></a>

**<code>F07</code> · <code>RFC9846 July2026</code>**

<code>4&#46;2&#59; 4&#46;3&#46;7&#59; 4&#46;3&#46;8&#59; 4&#46;5&#46;1&#45;4&#46;5&#46;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc9846&#46;html</code>

<a id="source-2"></a>

**<code>F08</code> · <code>RFC5280 2008&#59; RFC4055 2005</code>**

<code>5280&#58;4&#46;1&#46;1&#46;2&#44;4&#46;1&#46;2&#46;5&#44;4&#46;1&#46;2&#46;7&#59; 4055&#58;1&#46;2&#44;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc5280&#46;html</code>

<a id="source-3"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-6"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-7"></a>

**<code>R6151</code> · <code>RFC6151 2011</code>**

<code>2</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc6151&#46;html</code>

<a id="source-8"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## 評価の範囲

- これは保存された1回の試行であり、接続先全体の確認結果ではありません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>538e5773018c4079f6209d78ccad54416ee6c61098bd2120f2d2fc583ab05b4c</code>

収集時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

