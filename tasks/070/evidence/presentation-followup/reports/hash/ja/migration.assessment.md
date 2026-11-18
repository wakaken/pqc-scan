# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 1 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>未設定</code>](#target-1) · <code>app&#46;txt&#58;1</code> | <code>SHA&#45;256</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |

<a id="target-1"></a>

## <code>システム未設定</code> · <code>SHA&#45;256</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>app&#46;txt&#58;1</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応
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
| 暗号方式 | <code>SHA&#45;256</code> |
| 量子計算に対する評価 | <code>汎用の量子探索による影響。鍵・ハッシュの条件は別途確認</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>BHT1997</code>](#source-1)
- [<code>F12</code>](#source-2)
- [<code>F14</code>](#source-3)
- [<code>LOCAL026</code>](#source-4)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;97d0163974543a2fcdefade0751d91200e974bbf49443ea9290263582e269da8</code>

判断: <code>migration&#95;decision&#45;v1&#45;c2202c8f344bcb4f892de00c4d9feaccd19554cf35cf9b06093ba61ab7cd2cf4</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

## 判断の出典

<a id="source-1"></a>

**<code>BHT1997</code> · <code>1997</code>**

<code>ideal quantum collision&#45;query model</code> · <code>https&#58;&#47;&#47;arxiv&#46;org&#47;abs&#47;quant&#45;ph&#47;9705002</code>

<a id="source-2"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-3"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

## 評価の範囲

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>82ea7b77065b43177faf3f46792283e2d442e8c0906a02a4b0fdad83ec8749cc</code>

収集時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

