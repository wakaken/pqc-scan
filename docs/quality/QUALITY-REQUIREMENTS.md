# Q1〜Q6の要求・証拠・担当タスク対応表

対象ロードマップ: **pqc-scan v0.3**（品質評価・検証基盤の包括的強化。[位置付け](QUALITY-ROADMAP.md#v03-scope)）。

計画更新日: 2026-11-11。対象は最初の品質強化依頼のQ1〜Q6と、それを検証するコーパス、Rust、標準出力、TLS/SSH、OSS運用、CI、報告、受入の要求です。
今回は計画とタスク整理だけを行い、追加実装・工具導入・製品測定は行っていません。

各行の担当は主担当タスクです。関連作業は[ロードマップ](QUALITY-ROADMAP.md)の依存関係で結びます。同じ要求を複数タスクで独立に実装しません。
既存証拠は対象版・入力・環境が限られます。「既存実装あり」は新しい範囲の検証完了を意味しません。
必須成果物は今後の実行で `target/quality-task-<担当ID>/` に保存する予定のファイルです。まだ存在する成果物として扱わないでください。

## 既存証拠の参照先

- [E23: 固定コーパス・正解ラベル・反復測定](../../tests/quality/PROTOCOL.md)
- [E41: TASK-041の実測・コマンド・レビュー](assessment-20261111/EVIDENCE.json)
- [E18: 検査状況・範囲の仕様](../roadmap/TASK-018-IMPLEMENTATION.md)
- [E19: 検出器の件数・作業量制限の試験](../../crates/pqc_scan_detectors/tests/bounded_outcomes.rs)
- [E21: 証明書の分類・境界試験](../../crates/pqc_scan_detectors/tests/certificate_policy.rs)
- [E26: 独立したローカル評価方針の試験](../../crates/pqc_scan_core/tests/assessment_policy.rs)
- [E28: 119ルールの根拠と既知の誤一致](RULE-QUALITY.md)
- [E29: 秘匿処理の契約と試験](TASK-029-EVIDENCE-PRIVACY.md)
- [E34: CycloneDXの構造・意味の検証](../../crates/pqc_scan_report/tests/cyclonedx_17.rs)
- [ES: SARIFの公式Schema・位置検証](../../crates/pqc_scan_report/tests/sarif_contract.rs)
- [EN: 既存の公開Schema群](../../schemas/native-scan-v1.schema.json)
- [EP: 隔離TLS/SSH試験の実行境界](../../tests/protocol/README.md)
- [ED: 配布物の構築・導入試験](../../tests/distribution/test_package.py)
- [EC: 既存CIの定義](../../.github/workflows/pqc-scan.yml)
- [EG: 既存の承認済み条件・受入方針](QUALITY-GATES.md)
- [EU: 利用者向け説明](../../README.ja.md)
- [ER: 過去の技術受入と残存課題](FINAL-GAPS-VALIDATION.md)

E41の実測対象は `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf` と記録された作業ツリーです。元の57正解単位、過去の測定JSON・レビュー記録は変更しません。
計画の網羅性と測定の網羅性を区別します。以下に全項目の担当があっても、実施済みの証明にはなりません。

042の実行後の証拠は[E42: 追加正解データと評価の結果](TASK-042-RESULTS.md)に追記しています。
E42は方式・用途別の固定注釈への一致評価です。未注釈所見が残るため精度評価の完全性はPartialであり、製品全体の精度保証ではありません。
E43は[既知の不一致50件の原因と修正候補](TASK-043-ERROR-TRIAGE.md)です。規則変更や修正効果の測定は含みません。追加承認後の実装・44件の受入・再評価はE43Fとして[TASK-043の修正結果](TASK-043-FIXES-RESULTS.md)に分けて保存します。

E44は[到達率と追加検証の適用判断](TASK-044-RESULTS.md)です。2026-11-13のsource-bound測定と局所Miri試行、対象外・未記録理由を保持します。[Miriの追加調査](TASK-044-MIRI-FOLLOWUP.md)ではhost runnerの衝突を解消し、元crateでの実行範囲を確認しています。[現行の計測基盤](TASK-044-RETIREMENT.md)ではcoverageを独立させ、Miri専用実装は調査資料として保管します。全製品の安全性や形式的証明の合格には一般化しません。

E55は[検査範囲と統計の集計](TASK-055-RESULTS.md)です。制御8状態、既知9ファイル、74入力×5検出器、境界試験でQ2-01〜08を検証しました。未知母数と不完全性を保持し、製品全体の網羅性には一般化しません。

## Q1: 検出精度

| 要求項目 | 既存実装と証拠 | 不足 | 担当タスク | 受入条件 | 必須成果物（予定） |
|---|---|---|---|---|---|
| Q1-01 TP・FP・FN・TNとPrecision・Recall・F1・FPR・FNR、評価単位・分母 | E23/E41: 57単位のうち適格50単位で計算済み | 評価範囲を広げたときの分母と重複所見の扱い | [042](../../tasks/042-quality-corpus-expansion.yaml) | 手計算の正負・重複・分母0の例と一致し、全ての率に分母を付す | `metrics.json` |
| Q1-02 言語別・検出器別・ルール別の精度 | E41: 現在の指定19ルールを層別集計 | 他100ルールと新たな入力系統の検証 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 対応範囲の全ルールを評価済み・未評価・対象外に分類し、空の層を隠さない | `case-coverage.json` |
| Q1-03 暗号方式別の精度 | E28: 方式の説明はあるが精度の正解ではない | RSA/DSA/ECDSA/ECDH/DH/EdDSA/ML-KEM/ML-DSA/SLH-DSA・hybrid・旧称・unknownの正解分類 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 各方式・パラメータの正負または対応外理由をレビューし、出力を見る前に正解を固定 | `algorithm-truth.json` |
| Q1-04 暗号利用形態別の精度 | E23: API参照等の既存分類 | 署名・鍵交換・KEM・暗号化・JWT等の用途別分類 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 参照と実行を分け、各用途の評価単位・適格数・誤検出を報告 | `usage-truth.json` |
| Q1-05 ソース・設定・証明書・依存関係・SBOM/CBOM別の精度 | E41: 既存入力種別の集計 | 形式・版・利用形態を組み合わせた評価の不足 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 各対応形式の正負と破損例を含め、未対応を負例にしない | `artifact-metrics.json` |
| Q1-06 正例・負例・境界・意図的に紛らわしい入力 | E23/E19: alias等と制限例は一部存在 | コメント・文字列・同名関数・alias・shadowing・動的指定・未使用コード・無効設定・混在・同一行複数・型/引数・構文破損の不足セル | [042](../../tasks/042-quality-corpus-expansion.yaml) | 各ケースを対応表に載せ、未実行セルと既知FP/FNを残す。件数だけで受け入れない | `adversarial-cases.json` |
| Q1-07 独立正解と調整用/独立評価用データの分離 | E23: 独立ラベル2組と系統修正履歴 | 追加データの由来・版・独立性の確認 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 出力非参照の作成・二者ラベル・裁定・系統監査・固定時点・ライセンスを記録 | `label-review.md` |
| Q1-08 既知の誤検出・見逃しの原因と改善候補 | E23/E28/E42/E43/E43F: 原調査50観測と、追加承認された44観測の修正台帳を保持 | 44観測の修正・比較・二段階レビューを受入済み。注釈補完と出典訂正は別版で追跡し、旧数値信号を保持 | [043](../../tasks/043-quality-detection-triage.yaml) | 元の失敗例を保持し、承認済み全44件を修正。同一データと独立データ、互換性、二段階レビューを証拠で照合する | `error-triage.md`、`TASK-043-FIXES-RESULTS.md`、全44件の受入台帳 |
| Q1-09 同一データ回帰と新規独立データ評価の区別 | E41: 固定データの回帰測定済み | 新たな独立評価とデータ版が違う場合の比較 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 同一入力差分と新データの結果を別掲し、調整に使ったholdoutを独立評価と呼ばない | `comparison.json` |
| Q1-10 評価器の正しさ・統計的不確実性・データの制約 | E23/E41: 手計算・改ざん拒否の試験あり | 追加分類・データ相関・評価費用の確認 | [042](../../tasks/042-quality-corpus-expansion.yaml) | 評価器と正解を別々にレビューし、少数母数・同系統重複・未評価範囲と追加件数の根拠を説明 | `evaluation-review.md` |

## Q2: 検査網羅性

| 要求項目 | 既存実装と証拠 | 不足 | 担当タスク | 受入条件 | 必須成果物（予定） |
|---|---|---|---|---|---|
| Q2-01 入力の選択・正常検査・部分検査・失敗件数 | E18/E41: 41=33完了+7部分+1不明 | 評価範囲拡大時の集計と試験 | [055](../../tasks/055-quality-inspection-coverage.yaml) | selected/read/terminalの等式を満たし、重複計上と失敗の成功扱いを拒否 | `file-outcomes.json` |
| Q2-02 未対応形式・未対応版 | E18: UnsupportedとNotApplicableを区別 | 全形式・版の対応表と実結果の対応 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 未対応の理由と対象を記録し、未対応入力の所見0を安全判定にしない | `format-coverage.json` |
| Q2-03 サイズ・候補数・時間等の制限による打ち切り | E19: bounded outcomesの試験 | 制限別件数・停止理由の集約 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 境界値と超過例で部分結果を保持し、打ち切りを通常完了と区別 | `limit-outcomes.json` |
| Q2-04 解析対象外・除外済みファイル | E18: 除外件数・scopeあり | 明示除外と列挙できない対象の分離 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 設定した除外と実際に列挙した除外を区別して再現試験で確認 | `exclusions.json` |
| Q2-05 未列挙・未知の対象範囲 | E41: 未列挙母数を不明と表示 | 既知の全入力集合による列挙漏れの試験 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 管理された入力一覧で列挙漏れを検証し、任意リポジトリの未知母数を0に置換しない | `enumeration-review.md` |
| Q2-06 検出器別の適用可能性・完了・部分・失敗 | E18/E19: 個別outcomeはある | 開発用の検出器別集約が未実装 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 検出器×入力形式ごとの適用・非適用・未報告・失敗を集計し、想定値と照合 | `detector-coverage.json` |
| Q2-07 列挙率・適用率・処理完了率の分母分離 | E18: file partitionは定義済み | 三つの測定を分けた報告 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 計算可能な率だけを分母付きで示し、未知の列挙総数で率を作らない | `coverage-denominators.json` |
| Q2-08 読み取り・解析失敗時の結果保持 | E18/E19: 結果保持の仕様あり | 故障注入と報告の対応確認 | [055](../../tasks/055-quality-inspection-coverage.yaml) | 読み取り不能・不正構文・一検出器失敗でも他の結果と診断を保持 | `failure-retention.json` |

## Q3: PQC判断の妥当性

| 要求項目 | 既存実装と証拠 | 不足 | 担当タスク | 受入条件 | 必須成果物（予定） |
|---|---|---|---|---|---|
| Q3-01 アルゴリズム種別・署名/鍵交換/KEM/暗号化の用途 | E21/E26/E28: 分類台帳と表形式試験 | 全ての判断分岐に対する独立正解 | [056](../../tasks/056-quality-assessment-oracle.yaml) | 規格と公開事実から正解を作り、matcher正解と別に正誤・未評価数を示す | `assessment-oracle.json` |
| Q3-02 量子脆弱性と古典的安全性の分離 | E26/E28: 軸を分けた説明あり | 鍵長・digest・用途・unknownの組合せ検証 | [056](../../tasks/056-quality-assessment-oracle.yaml) | 古典的な弱さと量子耐性を取り違えず、情報不足を安全と断定しない | `risk-matrix.json` |
| Q3-03 PQC標準との関係・旧称・hybrid・unknown | E28: 原語と旧称を区別 | 規格の版・errataと実装判断の全対応 | [056](../../tasks/056-quality-assessment-oracle.yaml) | FIPS203/204/205・IR8547・関連SP/RFCの適用範囲と未確認事項を判断ごとに結ぶ | `standards-mapping.md` |
| Q3-04 暗号の存在・名目的参照と実利用 | E23/E26: 実利用の証明ではないと明記 | 存在から実行済みへ飛躍しない試験 | [056](../../tasks/056-quality-assessment-oracle.yaml) | 文字列・dead code・API参照だけで実利用/安全を断定しない | `assurance-cases.json` |
| Q3-05 依存の存在・宣言版・解決版と機能有効化 | E26: 有効化と存在を分離 | backend/features/未使用依存の判断ケース | [056](../../tasks/056-quality-assessment-oracle.yaml) | 依存名だけでPQC有効とせず、宣言と解決の来歴を保持 | `dependency-assessment.json` |
| Q3-06 広告機能と交渉・認証済み結果 | EP: offersとtrusted KEXを分離 | 観測状態と判断の対応表 | [049](../../tasks/049-quality-protocol-matrix.yaml) | 未認証offerを実交渉や認証成功へ昇格させない | `protocol-assurance.json` |
| Q3-07 証明書・公開鍵の属性と信頼性 | E21: signature/subject/timeを区別 | 属性抽出と署名/chain/trust保証の境界確認 | [056](../../tasks/056-quality-assessment-oracle.yaml) | OID・鍵長・期限だけから数学的妥当性や信頼を捏造しない | `certificate-assessment.json` |
| Q3-08 Severityの意味と境界 | E26/E28: 既存方針あり | category/risk/confidence/evidenceの全分岐検証 | [056](../../tasks/056-quality-assessment-oracle.yaml) | 既存仕様を独立表で検証し、意図的変更は影響分析と承認後に分ける | `severity-cases.json` |
| Q3-09 Confidenceの意味・確率との区別 | E28: heuristic値と説明 | 公開文言と下流利用の確認 | [056](../../tasks/056-quality-assessment-oracle.yaml) | 未校正の値を正答確率として扱わない。確率化する場合だけ別仕様・独立校正を要求 | `confidence-review.md` |
| Q3-10 移行推奨の正しさ・適用条件 | E26/E28: 推奨と規格参照あり | 用途・provider/相手側・JOSE/PKI制約・旧版推奨の検証 | [056](../../tasks/056-quality-assessment-oracle.yaml) | KEMと署名等を混同せず、必要条件・検証手順・根拠版・利用不能時の留保を示す | `recommendation-review.md` |
| Q3-11 公式Schemaによる構造検証 | ES/E34/EN/EP: offlineのSARIF2.1.0・CycloneDX1.7・native/probe Schema試験あり | 全出力と全probe結果Schemaの正負試験の一覧 | [048](../../tasks/048-quality-output-contracts.yaml) | 公式Schemaの版・license・hash・参照先を固定し、外部取得なしで実出力を検証。不正例も拒否し、legacy bare-array CBOMをCycloneDXと呼ばない | `schema-results.json` |
| Q3-12 全出力形式間の意味的一貫性 | ES/E34/EN/E29: 位置・実体・秘匿・exportの個別試験あり | HTML/Markdown/JSON/SARIF/CBOMの横断集約 | [048](../../tasks/048-quality-output-contracts.yaml) | algorithm/certificate/key/usageを区別し、位置・ID安定性・欠落/重複・証拠来歴・unknown・宣言/解決版・export lossを形式間で照合 | `output-semantics.json` |


## Q4: 堅牢性と安全性

| 要求項目 | 既存実装と証拠 | 不足 | 担当タスク | 受入条件 | 必須成果物（予定） |
|---|---|---|---|---|---|
| Q4-01 異常終了・panic・ハング・timeout | E19/EP: 決定的境界試験あり | 故障・異常入力の横断試験表 | [057](../../tasks/057-quality-security-failure-matrix.yaml) | 終了状態を区別し、hangを時間内に停止、panicや結果消失を個別報告 | `failure-matrix.json` |
| Q4-02 メモリ・CPU・PID・ディスク・通信量の制限 | EP: container上限と試験あり | 対象処理別の上限・実行証拠の対応 | [057](../../tasks/057-quality-security-failure-matrix.yaml) | 準備と対象実行を分離し、子プロセスを含む上限と終了後の後始末を検証 | `resource-boundary.json` |
| Q4-03 不正・破損入力、制限超過時の診断 | E19/E21: 一部の破損例と制限試験 | 全parser・出力経路への対応 | [047](../../tasks/047-quality-bounded-fuzzing.yaml) | DER/PEM/PKCS12/SBOM/設定/ソース/出力/protocolに上限付き探索を行い、最小再現例を保存 | `fuzz-results.json` |
| Q4-04 情報漏洩・マスキング | E29: canaryと秘匿契約あり | 全収集経路から全出力・ログまでの対応表 | [057](../../tasks/057-quality-security-failure-matrix.yaml) | 合成秘密の流出と例外時漏洩を検証し、パス等の対象外範囲を明記 | `privacy-matrix.json` |
| Q4-05 HTML等のXSS・URI・エスケープ | ES/E29: 既存writer試験 | 実ブラウザーと悪意ある入力の横断確認 | [048](../../tasks/048-quality-output-contracts.yaml) | HTML/Markdown/JSON/SARIFで構造を壊さず、実ブラウザー上で文字列が実行されない | `render-security.md` |
| Q4-06 ファイルパス・symlink・FIFO・境界外読み取り | E18/ER: link-safe readと試験あり | OS差・競合・特殊ファイルの要件表 | [057](../../tasks/057-quality-security-failure-matrix.yaml) | 対象外読み取りとブロックを防ぎ、hard link/内容同時変更等の限界を明示 | `filesystem-safety.json` |
| Q4-07 ネットワークアクセス境界 | EP/EU: 静的scanとprobeを分離 | オフラインscan/名前解決/接続先/準備工程の対応 | [057](../../tasks/057-quality-security-failure-matrix.yaml) | 静的scanの外部通信なしとprobeの明示先・許可CIDR・固定IP・中止を試験 | `network-boundary.json` |
| Q4-08 TLS1.2/1.3・classical/hybrid・実Negotiated Group | EP: 任意featureとlocal peers | client profile/provider/RFC9846差分の網羅表 | [049](../../tasks/049-quality-protocol-matrix.yaml) | 実交渉groupを確認し、cipherや設定名から推測しない。未知groupは不明のまま保持 | `tls-matrix.json` |
| Q4-09 TLS証明書検証・失敗・部分handshake | EP: 検証失敗等の試験あり | trust/name/expiry/mTLS/HRR/refused/timeout/巨大messageの対応 | [049](../../tasks/049-quality-protocol-matrix.yaml) | 検証失敗で成功済み暗号事実を捏造せず、各状態と限界を記録 | `tls-failures.json` |
| Q4-10 SSH KEXINIT offer・交渉済みhybrid ML-KEM・host pin | EP: 既存039の実装と試験 | peer設定と観測・認証状態の対応 | [049](../../tasks/049-quality-protocol-matrix.yaml) | pin不一致・unknown/古典/hybridを分け、未認証情報と検証済み事実を区別 | `ssh-matrix.json` |
| Q4-11 SSHの認証・ログイン・コマンド実行禁止 | EP: no-auth試験あり | 送信packet種別と全失敗経路の対応 | [049](../../tasks/049-quality-protocol-matrix.yaml) | service/userauth/channel/commandを送らず切断することを実peerで確認 | `ssh-no-auth.json` |
| Q4-12 SSH不正frame・timeout・通信量・接続先制限 | EP: parser/runtimeの境界試験 | fragment/巨大/途中切断・接続拒否を含む全対応 | [049](../../tasks/049-quality-protocol-matrix.yaml) | 割当て前の長さ検査、有限の通信/時間/接続数、中止と後始末を検証 | `ssh-failures.json` |
| Q4-13 コードカバレッジと重要な未到達処理 | E41の未測定を保持。E44: 通常/全featureのline・region、固定nightlyのbranchを測定 | resolverの集約・子process、別OS/arch・FFI等の制約を分離して記録 | [044](../../tasks/044-quality-coverage-feasibility.yaml) | 取得可能なline/region/branchを区別し、安全上重要な未到達箇所と対象外理由を列挙 | `coverage-review.md` |
| Q4-14 変異試験によるassertionの有効性 | E45: [20件の変異試験](TASK-045-RESULTS.md)。検出14・非等価の見逃し5・等価1、時間切れ/ビルド不能0 | 全機能有効のLinux arm64に限定。見逃しの意味と分母を記録し、全製品の品質へ一般化しない | [045](../../tasks/045-quality-mutation-study.yaml) | caught/missed/timeout/unviableを分離し、残存変異の意味とscore分母を説明 | `mutation-review.md` |
| Q4-15 生成試験による不変条件 | E46: [生成試験](TASK-046-RESULTS.md)。各設定192例の統計・再現性、384例のID/不明情報、縮小の負対照を確認 | 固定3seed・通常/全機能・Linux arm64の有限範囲。全入力・全環境の証明へ一般化しない | [046](../../tasks/046-quality-property-testing.yaml) | 意味的再現、失敗≠成功、unknown非捏造、検査統計、ID安定/一意/連結境界を検証 | `property-results.json` |
| Q4-16 Miri・形式検証の適用判断 | E41の未導入を保持。E44: 原sourceのscope helperでMiri3試験と工具の正負対照 | 当初のcore build script停止は追加調査で解消。元crateの結果とFFI/OS依存・時間上限・Kani未試行を分離し、代替検証を記録 | [044](../../tasks/044-quality-coverage-feasibility.yaml) | 適用可能な小範囲を試し、実行不能・費用過大なら根拠と代替検証を明記。導入数を目標にしない | `verification-feasibility.md` |

## Q5: 性能と再現性

| 要求項目 | 既存実装と証拠 | 不足 | 担当タスク | 受入条件 | 必須成果物（予定） |
|---|---|---|---|---|---|
| Q5-01 wall time・CPU time・peak RSS | E23/E41: wait4による固定workload測定 | 測定対象拡大とOSごとの単位・取得範囲 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 時刻・CPU・RSSの単位/対象プロセスを明示し、取得不能値をnullで保持 | `resources.json` |
| Q5-02 ファイル数・入力bytes当たりの性能 | E23: 既存コピー入力と入力hash | 実製品相当のサイズ・構成 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | files/bytesを実入力と照合し、起動/出力時間込みのthroughputと分母を示す | `throughput.json` |
| Q5-03 入力規模・thread数による性能変化 | E41: 最大通常183872 bytesの合成入力 | 異なる規模・並列度・構成での測定 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 認可済み/再配布可能な入力を固定し、増加傾向とばらつきを記録 | `scaling.json` |
| Q5-04 同一条件の反復・cold/warm・日間変動 | E23: first-measuredとwarm3、cold未測定 | cache条件と実行日差の測定 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 本当のcoldと初回を区別し、sample数・分布・環境差を開示 | `noise-study.md` |
| Q5-05 意味的同一性・所見ID・重複保持 | E41: 3回の意味一致 | 新入力・feature・環境にまたがる再現 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 除去する変動項目を限定し、事実・ID・重複の変化を検出 | `semantic-reproducibility.json` |
| Q5-06 出力順序の安定性 | E41: 順序を意味と別に測定 | 並列度・環境・全出力形式での確認 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 意味が同じでも順序差を別報告し、安定性を主張する条件を定める | `ordering.json` |
| Q5-07 環境差・build/features・binary/input/rules hash | E23/E41: 環境とhashあり | tool/provider/OS/arch差の比較条件 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 比較できる条件とできない条件を識別し、環境変化を製品退行と混同しない | `environment-comparison.json` |
| Q5-08 失敗・timeout・制限時の資源使用 | E19/EP: 安全上限の試験 | 失敗時の資源測定 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 失敗を高速成功に数えず、原因・使用資源・保持結果を併記 | `failure-resources.json` |
| Q5-09 承認済み回帰・性能調査条件 | EG/ER: 既存nonregression・同環境2x信号 | 測定拡大後の比較と調査手順 | [050](../../tasks/050-quality-performance-reproducibility.yaml) | 既存条件を保ち、2x超過は調査として残し、改定には測定変動と承認を要する | `regression-review.md` |

## Q6: OSS製品としての成熟度

| 要求項目 | 既存実装と証拠 | 不足 | 担当タスク | 受入条件 | 必須成果物（予定） |
|---|---|---|---|---|---|
| Q6-01 CI品質・PRと定期/手動実行の分担 | EC/E41: 既存4jobと報告追加 | 追加測定の費用・頻度・実行枠 | [053](../../tasks/053-quality-ci-evidence-integration.yaml) | 既存jobを再利用し、PRの軽量検証と高コスト測定を分け、全軸の証拠を保存 | `ci-measurement-plan.md` |
| Q6-02 テスト品質・評価器の独立レビュー | E41: 実装/試験妥当性の二段階レビュー | 追加検証の実測とレビューの結合 | [053](../../tasks/053-quality-ci-evidence-integration.yaml) | 最終ソースに結ぶ結果を保存し、ログ成功だけで試験妥当性を合格にしない | `test-evidence-index.json` |
| Q6-03 依存関係・脆弱性管理 | E41: lock固定、最新audit未実行 | 工具・advisory DBの取得と定期確認 | [051](../../tasks/051-quality-dependency-supply-chain.yaml) | 版/feature/供給元とDB取得日/hash、脆弱性・未保守・例外期限・対処を記録 | `dependency-review.json` |
| Q6-04 ライセンス管理・notice | ED/ER: 既存noticeと5要確認事項 | 公開前の調査と許容判断 | [051](../../tasks/051-quality-dependency-supply-chain.yaml) | 全配布variantの依存とlicense/noticeを対応付け、未解決許容は維持者判断として残す | `license-review.md` |
| Q6-05 開発プロセス・コードレビュー・maintainer文書 | EG/EU: agent手順等あり | 貢献・変更・保守手順の実態確認 | [052](../../tasks/052-quality-oss-process.yaml) | Best Practicesの各項目に根拠と担当を示し、文書の存在と実行実績を区別 | `process-review.md` |
| Q6-06 GitHub branch protection・required checks・最小権限 | E41/EC: read権限確認、protectionは403 | 管理設定とeffective権限の確認 | [052](../../tasks/052-quality-oss-process.yaml) | 取得できた設定と推奨変更を分け、権限不足はUnknown。設定変更は別承認 | `github-controls.json` |
| Q6-07 セキュリティ報告・脆弱性対応 | EU: SECURITY.mdへの案内あり | 受付・対応・修正版案内の手順確認 | [052](../../tasks/052-quality-oss-process.yaml) | 公開窓口/対応範囲/担当/連絡経路を確認し、未運用部分を明記 | `security-process.md` |
| Q6-08 OpenSSF Scorecard | E41: 未導入、公開API404 | 固定工具での実測 | [052](../../tasks/052-quality-oss-process.yaml) | commit/工具版/取得日と各check・取得不能理由を保存し、検出精度とは別表示 | `scorecard.json` |
| Q6-09 OpenSSF Best Practices | E41: 基準参照のみ | 根拠付き自己評価 | [052](../../tasks/052-quality-oss-process.yaml) | 全適用項目を根拠と照合し、申請・取得の有無を区別。badge申請は別指示 | `best-practices.md` |
| Q6-10 CLI・公開Rust API・出力の互換性 | EN/ES/E34: 契約試験あり | サポートする利用形態の全対応 | [058](../../tasks/058-quality-api-usability.yaml) | 旧版の呼出し・Detector adapter・終了コード・file名・field/enum/ID/版を回帰試験し、意図的差分は承認付きで示す | `compatibility-matrix.json` |
| Q6-11 クロスプラットフォーム・feature・導入・rule配布 | ED/ER: macOS arm64の3variant準備 | 対象OS/archとclean installの実測不足 | [059](../../tasks/059-quality-platform-install.yaml) | 対象環境を維持者と確定し、各組合せのbuild/test/install/rulesを実測。試していない環境を正式対応としない | `platform-matrix.json` |
| Q6-12 文書・利用しやすさ・日本語/英語・accessibility | EU/E41: 両言語説明と既存画面記録 | 新規利用者の操作と現行画面の確認 | [058](../../tasks/058-quality-api-usability.yaml) | 導入→scan→所見/未検査理解の手順を確認し、help/API例/リンクと日英の事実一致、keyboard/狭い画面は実ブラウザーで記録 | `usability-review.md` |
| Q6-13 ビルド再現性 | ED: 同binaryのarchive再現は別証拠 | 別のclean環境での再build比較 | [060](../../tasks/060-quality-clean-rebuild.yaml) | 同じsource/依存/tool/条件を別環境で構築し、binary差分と埋込情報/非決定性を説明。archive比較で代用しない | `rebuild-comparison.json` |
| Q6-14 配布物の完全性・検証手順 | ED: packageとoffline install試験あり | 取得後の改変検出・release manifestの確認 | [061](../../tasks/061-quality-release-integrity.yaml) | binary/rules/noticesをhashで結び、欠落/改変/混在を受け手の検証手順で拒否 | `artifact-integrity.json` |
| Q6-15 署名・生成来歴・信頼できるrelease手順 | ER: 未公開/未署名 | 鍵・権限・provenance・署名検証の設計と試行 | [061](../../tasks/061-quality-release-integrity.yaml) | 認証情報を記録せず、準備物の署名/検証と改変拒否を隔離試行。公開・tag・実release署名は別承認 | `release-provenance.md` |
| Q6-16 全品質軸の自動報告・証拠保持 | E41: Q1/Q2/Q5を部分自動集約 | Q3/Q4/Q6等の結果の取り込みと保存期限 | [053](../../tasks/053-quality-ci-evidence-integration.yaml) | 全要求IDと実行/未実行・根拠hashを機械的に参照でき、工具障害と製品退行を分離 | `quality-evidence-index.json` |
| Q6-17 1.0受入・残存リスク・継続改善 | EG/ER: 037の過去技術受入あり | 次の対象版の全要求別判定 | [054](../../tasks/054-quality-release-acceptance.yaml) | 全要求に結果/証拠/範囲を付け、必須検証未実行・未解決欠陥は自動合格にせず、除外/条件変更は維持者承認に結ぶ | `release-assessment.md` |
| Q6-18 品質レポートの内容と利用者への説明 | E41: 初回のJSON/Markdownと手動評価書あり | 全軸の新測定と判定を一つの報告経路へ統合 | [053](../../tasks/053-quality-ci-evidence-integration.yaml) | 対象commit/版・機能範囲・規格・方法・環境・実測・Q1〜Q6・制約/未確認・重大懸念・過去比較・優先順位・受入判定を入力/コマンド/成果物へ結ぶ。機械JSONと人向け報告の事実が一致 | `quality-assessment.md` |


## 全要求に共通する完了条件

各タスクは実行対象の版・入力・規則・工具・環境と、コマンド・終了状態・ログ・成果物hashを保存します。正解ラベルの独立性、未実行理由、既知の欠陥、評価範囲外も残します。
構造Schemaへの適合と内容の正しさ、静的参照と実利用、静的scanと実行時probe、合成入力と実製品、同一データ回帰と新規独立評価は別々に判定します。

対応範囲内の必須検証が未実行、または必要な測定機能が未実装なら、その要求は未完了です。UnknownやNot Testedの表示機能だけで完了にしません。
適用外・環境の制約・残存リスクによって受入範囲を変更する場合は、根拠と維持者の判断を残します。権限不足を回避するために設定を変更したり、既知の失敗例を削除したりしません。

報告には指定された8状態を使い、判定と証拠への参照を付けます。全体の総合点や安全率は作りません。
最終の054が全行を照合し、各軸の結果とリリース可否を判定します。過去の037の技術受入と、新しい対象版の判定は別記録です。

057のQ4-01・02・04・06・07は[検証結果](TASK-057-RESULTS.md)と5成果物・二段階レビューを当初条件に照合し、評価を完了しました。Windows等の未実行範囲や秘匿・ファイルの限界も同記録に残しています。

047のQ4-03は[ファジング結果](TASK-047-RESULTS.md)の20枠・14,547入力、対照故障の保存/再現/縮小、255入力のcoverageと二段階レビューを当初条件に照合し、評価を完了しました。固定seed・対象範囲と未計測branchを製品全体の保証とは区別します。

048は[出力契約の検証](TASK-048-RESULTS.md)で、Q3-11・Q3-12・Q4-05を当初条件と二段階レビューに照合しdoneとしました。実CLIのSchema正負23例、形式間の意味・来歴・省略の照合、Chromeの日英16ページの観察を保存しています。有限入力・1環境の評価と製品全体の保証を区別し、他タスクや過去の測定は変更していません。
