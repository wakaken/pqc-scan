# 組織の暗号移行レポート

保存済みの検出結果と業務上の申告をもとに、対応が必要なシステム、移行を妨げる条件、次の確認事項を整理します。

対象数: 18 · 評価時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code>

## 対応の一覧

| システム・場所 | 方式・役割 | 緊急度 | 次の対応 |
|---|---|---|---|
| [<code>未設定</code>](#target-1) · <code>packages&#46;lock&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-2) · <code>composer&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-3) · <code>unknown&#46;c&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-4) · <code>packages&#46;lock&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-5) · <code>kem&#46;php&#58;2</code> | <code>ML&#45;KEM&#45;768</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-6) · <code>composer&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-7) · <code>vcpkg&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-8) · <code>signature&#46;cpp&#58;1</code> | <code>ML&#45;DSA&#45;65</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-9) · <code>signature&#46;c&#58;1</code> | <code>SLH&#45;DSA&#45;SHA2&#45;128s</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-10) · <code>vcpkg&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-11) · <code>composer&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-12) · <code>packages&#46;lock&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-13) · <code>composer&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-14) · <code>packages&#46;lock&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>system&#45;g1</code>](#target-15) · <code>rsa&#46;php&#58;2</code> | <code>RSA</code> · 暗号方式への参照 | 早急に対応 | 阻害条件を解消する |
| [<code>未設定</code>](#target-16) · <code>signature&#46;cs&#58;1</code> | <code>ML&#45;DSA&#45;65</code> · 暗号方式への参照 | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-17) · <code>vcpkg&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |
| [<code>未設定</code>](#target-18) · <code>composer&#46;json&#58;1</code> | <code>unknown</code> · 参照のみ | 緊急度を判断できない | 調査 |

<a id="target-1"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>packages&#46;lock&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>lock&#95;resolved</code>

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
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;45e1832092b6a173d070c47b47e17945ec57671df54c1bd8cbf3fa12f89eaaae</code>

判断: <code>migration&#95;decision&#45;v1&#45;e19104fdfa4e3a423be7b9d441d398d5af2186efdcdd9b27c86506fe7c1c848a</code>

</details>

<a id="target-2"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>composer&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>プラットフォームの仮想設定を明示的に無効化</code>

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

<a id="evidence-2"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;69352d88032089c87937b8a9e5edde93c9d618540b4c2470f363113ddadb621e</code>

判断: <code>migration&#95;decision&#45;v1&#45;568ed1afcc856c4d2684c61f96a337c8b803451d1cc9442eff45fc3c9b84491f</code>

</details>

<a id="target-3"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>unknown&#46;c&#58;1</code> (参照のみ)

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

<a id="evidence-3"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;0dbb409bad474253a1554d0fadb3113f131e55cf9d7f964b1217ecb63f255a0e</code>

判断: <code>migration&#95;decision&#45;v1&#45;693c0e0e9a86ab64db015da3621328b89fb5a92a43419e03afeb580f80a93844</code>

</details>

<a id="target-4"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>packages&#46;lock&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>lock&#95;resolved</code>

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

<a id="evidence-4"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;1801c8e56ea3ed30196d31d482b3bb9477171938f6b335596455615f562dc129</code>

判断: <code>migration&#95;decision&#45;v1&#45;9dc63694b437bca2d8c14fd78d9ac3952bc34597a4edea06cb10214a2795a9fe</code>

</details>

<a id="target-5"></a>

## <code>システム未設定</code> · <code>ML&#45;KEM&#45;768</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>kem&#46;php&#58;2</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- 利用中・利用予定の状況
- 必要な用途

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-5"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>ML&#45;KEM&#45;768</code> |
| 量子計算に対する評価 | <code>耐量子性を意図した方式・配備は未検証</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;d78e1ea85930368324c4a72ccba0463c2fa644f0d86c64cf265e89bcd99ff5bd</code>

判断: <code>migration&#95;decision&#45;v1&#45;8e0e7f00b540cd0922e6cd56723b602677cd1983df3222909469ac33e2253bf4</code>

</details>

<a id="target-6"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>composer&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>manifest&#95;dev&#95;declared</code>

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

<a id="evidence-6"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;12f917cd391a411afa0f2c58a12b23db19cb0902ebc5a0ed110349a656eb1670</code>

判断: <code>migration&#95;decision&#45;v1&#45;f9013260a0c6159a7485b286cc08242f864e126ad06af90d09e2e1ac13572de9</code>

</details>

<a id="target-7"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>vcpkg&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>manifest&#95;declared</code>

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

<a id="evidence-7"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;7b336895a6c82185545440adc6848f78eb6c9e347233a2c9efcc3756cb743ccc</code>

判断: <code>migration&#95;decision&#45;v1&#45;93a33f60db759ef5788bd7cb10cbc2bbea3cb4a5b8aff1be79c29f6ff0dd8222</code>

</details>

<a id="target-8"></a>

## <code>システム未設定</code> · <code>ML&#45;DSA&#45;65</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>signature&#46;cpp&#58;1</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- 利用中・利用予定の状況
- 必要な用途

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-8"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>ML&#45;DSA&#45;65</code> |
| 量子計算に対する評価 | <code>耐量子性を意図した方式・配備は未検証</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S204</code>](#source-6)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;5446526b774f73eee99d2dcd12030a4ff571446df9ad204a7befa771d5a284e2</code>

判断: <code>migration&#95;decision&#45;v1&#45;cdfa8919cf853305da54f0f553ab4e4465cf2bfe61f1d28944747f5906bdaba6</code>

</details>

<a id="target-9"></a>

## <code>システム未設定</code> · <code>SLH&#45;DSA&#45;SHA2&#45;128s</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>signature&#46;c&#58;1</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- 利用中・利用予定の状況
- 必要な用途

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-9"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>SLH&#45;DSA&#45;SHA2&#45;128s</code> |
| 量子計算に対する評価 | <code>耐量子性を意図した方式・配備は未検証</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S205</code>](#source-7)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;3e48e37ecd6b94b857bebeda008c5de9ec3246449a56facf5738370faa6c5367</code>

判断: <code>migration&#95;decision&#45;v1&#45;3c35943a8cdf7cc45d8430056056030b0316ac02f4cc07c3d00b78ef2fb4ef22</code>

</details>

<a id="target-10"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>vcpkg&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>manifest&#95;declared</code>

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

<a id="evidence-10"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;3ee68beead419d421a750e37637146cb8f4ce7d068613cf0e80a1a2c6b2989cd</code>

判断: <code>migration&#95;decision&#45;v1&#45;99e0173b3235893b8fa3bfefb4a67756cb79ee0d13d5cadb8e7ae3f7fde9cefa</code>

</details>

<a id="target-11"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>composer&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>プラットフォームの要求宣言</code>

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

<a id="evidence-11"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;e87f20b64dcad498ca9d05df2409a06f78d677f8498802369b970755df7e1d8b</code>

判断: <code>migration&#95;decision&#45;v1&#45;e32cdb25498b5bffce54d44bc93fd4c436a9407df848d20fd67d0c36d3c17810</code>

</details>

<a id="target-12"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>packages&#46;lock&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>lock&#95;resolved</code>

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

<a id="evidence-12"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;1dfb6e3b17609304ce1de394d14160918db331682eb574f9f3c95d3c4da2223d</code>

判断: <code>migration&#95;decision&#45;v1&#45;172c1c2035778dc0b795c2017edaefbe4b88447a9af4fe145a86a7ea1fa13a3c</code>

</details>

<a id="target-13"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>composer&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>manifest&#95;dev&#95;declared</code>

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

<a id="evidence-13"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;f54d0bd445afb2adcb373d598a63d70c1b75bf37f067b8d7bf2547442f35c4be</code>

判断: <code>migration&#95;decision&#45;v1&#45;f393e93c3f146054401ad08b6433b23219b9287c599b1091e573552688ab5005</code>

</details>

<a id="target-14"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>packages&#46;lock&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>ロックファイルに記録されたフレームワーク</code>

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

<a id="evidence-14"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;9bc4e6ccfe554907853e3d7ad4067a4a650b1c8f3440d5e5227b7a6130f83b7c</code>

判断: <code>migration&#95;decision&#45;v1&#45;42b02a91248a454411d00d07a269fe2fe28d9abbc00440b9a6505b08ef3a2ff2</code>

</details>

<a id="target-15"></a>

## <code>system&#45;g1</code> · <code>RSA</code>

**早急に対応** · 不足情報あり · 未対応の条件あり

場所: <code>rsa&#46;php&#58;2</code> (暗号方式への参照)

### 次の対応

阻害条件を解消する

業務上重要な署名用途での利用中・利用予定が申告されています。

**申告された阻害条件**

- 実行環境の対応

**判断に必要な追加情報**

- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
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
| 実行環境の対応 | 申告上は未対応 |
| プロバイダーの対応 | 不明 |
| 正確なパラメーターへの対応 | 不明 |
| 接続相手の対応 | 不明 |
| プロトコルの対応 | 不明 |

<a id="evidence-15"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>RSA</code> |
| 量子計算に対する評価 | <code>量子計算に対する懸念あり</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。

- [必要な用途](#claim-1)
- [利用中・利用予定の状況](#claim-2)
- [実行環境の対応](#claim-3)
- [業務上の重要性](#claim-4)
- [プロバイダーの対応](#claim-5)
- [検討する方式・設定の申告](#claim-6)
- [適用範囲](#claim-7)
- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;5c5192237138dab73254385d5217745c7a9e771b79b89da102c14b79fa81260d</code>

判断: <code>migration&#95;decision&#45;v1&#45;e12442975e8aed6ca55a78eb244f2b90985bcf56f97a4353bfaca2abef2c9eae</code>

</details>

<a id="target-16"></a>

## <code>システム未設定</code> · <code>ML&#45;DSA&#45;65</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>signature&#46;cs&#58;1</code> (暗号方式への参照)

### 次の対応

調査

現在の情報では、緊急度を裏付ける判断条件が揃いません。

**判断に必要な追加情報**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- 利用中・利用予定の状況
- 必要な用途

**次の確認**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

### 移行の検討候補

この判断で選んだ候補はありません。

<a id="evidence-16"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>ML&#45;DSA&#45;65</code> |
| 量子計算に対する評価 | <code>耐量子性を意図した方式・配備は未検証</code> |
| 確認できている範囲 | <code>構文上の参照</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S204</code>](#source-6)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;8774562c336b4d0916d0d3304e18bbf769c170c66da466fe71ff8201847617e6</code>

判断: <code>migration&#95;decision&#45;v1&#45;d259182bb4c165cd751c1604f133119d1d66a8b48d34d0dd081c62f8abb8d419</code>

</details>

<a id="target-17"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>vcpkg&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>manifest&#95;declared</code>

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

<a id="evidence-17"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;d0fc1f1071fb9185725c01cd225bc6b7e88519f4ca6e70f252f5e768deb1ba8d</code>

判断: <code>migration&#95;decision&#45;v1&#45;1bd838e41ed10becaec47375aaffa9ba8b9a338fc0045efa13e8178aa166a658</code>

</details>

<a id="target-18"></a>

## <code>システム未設定</code> · <code>unknown</code>

**緊急度を判断できない** · 不足情報あり · 準備状況は不明

場所: <code>composer&#46;json&#58;1</code> (参照のみ)

依存関係の宣言: <code>プラットフォームの仮想設定を明示的に無効化</code>

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

<a id="evidence-18"></a>

### この対応とした根拠

| 確認項目 | 記録された内容 |
|---|---|
| 暗号方式 | <code>unknown</code> |
| 量子計算に対する評価 | <code>不明</code> |
| 確認できている範囲 | <code>解析した属性</code> |
| 収集状況 | <code>記録された範囲で収集完了</code> |

- 参照の検出だけでは、実環境での利用は確認できません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>照合用ID</summary>

対象: <code>migration&#95;target&#45;v1&#45;c8d176ad477b006ae673bbf6b0dbc65b8e7a4650a31e57c0ddf4f7d7419b19ec</code>

判断: <code>migration&#95;decision&#45;v1&#45;1a84f5ff58ba48814152f1bbdaf4a81030d2a26a892b1d910c6d8335d9dd5eda</code>

</details>

## 判断に使った申告と由来

外部レビュー由来のものを含め、申告は未検証です。移行準備の表示は有効な申告に基づき、配備の検証結果ではありません。

<a id="claim-1"></a>

### 必要な用途

<code>署名・認証</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### 利用中・利用予定の状況

<code>利用中</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

<a id="claim-3"></a>

### 実行環境の対応

<code>申告上は未対応</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

<a id="claim-4"></a>

### 業務上の重要性

<code>はい</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

<a id="claim-5"></a>

### プロバイダーの対応

<code>不明</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

<a id="claim-6"></a>

### 検討する方式・設定の申告

<code>ML&#45;DSA &#47; ML&#45;DSA&#45;65 · 署名・認証</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

<a id="claim-7"></a>

### 適用範囲

<code>対象</code> · 申告期間内

申告者: <code>test&#45;owner</code> · 利用者の申告（未検証）

申告時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;57Z</code> / 有効期限: <code>2026&#45;12&#45;31T15&#58;00&#58;00Z</code>

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

**<code>S203</code> · <code>FIPS203 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 3&#46;1&#59; 7&#59; 8 Table2</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;203&#47;final</code>

<a id="source-6"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

<a id="source-7"></a>

**<code>S205</code> · <code>FIPS205 Final 2024&#45;08&#45;13</code>**

<code>10&#46;2&#46;1&#59; 10&#46;2&#46;2&#59; 11 Table2</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;205&#47;final</code>

## 評価の範囲

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。

入力SHA-256: <code>a2d70e3632b8f87ffcd0b9849ab511b4b23437b980be27e72bb25effbe98d46d</code>

収集時刻: <code>2026&#45;11&#45;24T09&#58;23&#58;43&#46;730406&#43;00&#58;00</code>

[機械処理用の根拠・照合データ（JSON）](migration.assessment.v1.json)

