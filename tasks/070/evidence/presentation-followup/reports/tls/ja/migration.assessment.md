# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 3 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>system&#45;a</code>](#target-1) · <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> | <code>X25519MLKEM768</code> · 通信の鍵確立 | 対象の役割を継続確認 | この役割の変更を継続確認する |
| [<code>未設定</code>](#target-2) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> | <code>RSA&#45;SHA256</code> · 証明書の署名 | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-3) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> | <code>RSA</code> · 証明書の公開鍵 | 緊急度を判断できない | 調査 |

<a id="target-1"></a>

## <code>system&#45;a</code> · <code>X25519MLKEM768</code>

**対象の役割を継続確認** · 必要情報あり · 移行準備の判断対象外

場所: <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> (通信の鍵確立)

### 次の対応

この役割の変更を継続確認する

識別できた事実と、この役割の範囲では既知の懸念は確認されていません。

**次の確認**

- 事実・対象範囲・申告が変わった際に再評価してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-1"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>X25519MLKEM768</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>収集記録がセッションの検証成功を報告</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- ハイブリッド方式の結合処理の実装は未検証です。

- [適用範囲](#claim-1)
- [必要な用途](#claim-2)
- [<code>F06</code>](#source-1)
- [<code>F07</code>](#source-2)
- [<code>F12</code>](#source-4)
- [<code>F14</code>](#source-5)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;7f0437243eb67d95e9c589931213def7ffbdf88da00db46029f56c898e2a3578</code>

判断: <code>migration&#95;decision&#45;v1&#45;64f7e09c5f6cea5012a513bd3677cfeb67ddc0f58146050109125e0e508425a6</code>

</details>

<a id="target-2"></a>

## <code>システム未設定</code> · <code>RSA&#45;SHA256</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> (証明書の署名)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

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
- 正確なパラメーター
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
| 暗号方式 | <code>RSA&#45;SHA256</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F08</code>](#source-3)
- [<code>F12</code>](#source-4)
- [<code>F14</code>](#source-5)
- [<code>LOCAL026</code>](#source-6)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;cfb6eff3499d8d9a3b29da39ae787362ed425ffaf6f34c7388b7f700d341fe00</code>

判断: <code>migration&#95;decision&#45;v1&#45;090adf97fe32da6be39c3ee92731e5323cd94a6ea948dd5298b828cb12391ce9</code>

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
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F08</code>](#source-3)
- [<code>F12</code>](#source-4)
- [<code>F14</code>](#source-5)
- [<code>LOCAL026</code>](#source-6)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;24c6384f7d935a4495728e19b225cd92a65d68625b15d8413ad6c5dbbe53b99d</code>

判断: <code>migration&#95;decision&#45;v1&#45;af716d0aed4c59b4a54d56412a8d4e6c2be1fdc11525b4248e718b67a207a6d6</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

<a id="claim-1"></a>

### 適用範囲

<code>対象</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### 必要な用途

<code>鍵確立</code> · 申告期間内

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

## 判断の出典

<a id="source-1"></a>

**<code>F06</code> · <code>RFC10024 August2026&#59; RFC9954 July2026</code>**

<code>10024&#58;4&#46;1&#45;4&#46;3&#44;7&#46;1&#45;7&#46;4&#59; 9954&#58;3&#46;1&#45;3&#46;3&#44;6</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc10024&#46;html</code>

<a id="source-2"></a>

**<code>F07</code> · <code>RFC9846 July2026</code>**

<code>4&#46;2&#59; 4&#46;3&#46;7&#59; 4&#46;3&#46;8&#59; 4&#46;5&#46;1&#45;4&#46;5&#46;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc9846&#46;html</code>

<a id="source-3"></a>

**<code>F08</code> · <code>RFC5280 2008&#59; RFC4055 2005</code>**

<code>5280&#58;4&#46;1&#46;1&#46;2&#44;4&#46;1&#46;2&#46;5&#44;4&#46;1&#46;2&#46;7&#59; 4055&#58;1&#46;2&#44;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc5280&#46;html</code>

<a id="source-4"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-6"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-7"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-8"></a>

**<code>S203</code> · <code>FIPS203 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 3&#46;1&#59; 7&#59; 8 Table2</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;203&#47;final</code>

<a id="source-9"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## 評価の範囲

- これは保存された1回の試行であり、接続先全体の確認結果ではありません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>d2460ea7e75165d9b0e0fc27c4b654c9b66a66c82c97baf8a0556ad4aa1f8f26</code>

収集時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

