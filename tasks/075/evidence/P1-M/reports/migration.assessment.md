# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 1 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>system&#45;1</code>](#target-1) · <code>app&#46;txt&#58;1</code> | <code>RSA</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |

<a id="target-1"></a>

## <code>system&#45;1</code> · <code>RSA</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>app&#46;txt&#58;1</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
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
| 暗号方式 | <code>RSA</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 判断に関係する申告が現在有効ではありません。更新が必要です。

- [適用範囲](#claim-1)
- [必要な用途](#claim-2)
- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad</code>

判断: <code>migration&#95;decision&#45;v1&#45;d7dc41f1c1e2e62657bae47a47c98f42c361c6d36590d9c91ca72b39da29ab8d</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

<a id="claim-1"></a>

### 適用範囲

<code>対象</code> · 有効期限不明

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>unknown</code>

<a id="claim-2"></a>

### 必要な用途

<code>署名・認証</code> · 有効期限不明

申告者: <code>owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / 有効期限: <code>unknown</code>

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

## 評価の範囲

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>ea072032ff581bb287e010de83d5c9be0f72debdce0962925dfa3c5dbf29f81f</code>

収集時刻: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

