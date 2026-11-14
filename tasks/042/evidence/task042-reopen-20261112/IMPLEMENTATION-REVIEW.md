# TASK-042再開: 実装レビューA

判定: **要修正・TASK-042は未完了**。全コーパスのソース注釈監査Verifiedと、当初9受入要求の完了を分ける。

対象HEAD: `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`、ブランチ `codex/quality-assessment-model`、未コミット差分を含む。レビュー担当は `q42_reopen_implementation_review`。委譲は行っていない。Bのラベルファイルは参照していない。

## 指摘

### R1 [P1] Q1-05の「各対応形式の正負と破損例」が、有限コーパスの注釈完全性へ置き換わっている

参照: `docs/quality/TASK-042-REOPEN.md:11`、`tasks/042-quality-corpus-expansion.yaml` のQ1-05、`docs/user/COVERAGE.md:31` と `:106`、両corpus manifest。

元受入は「各対応形式の正負と破損例を含め、未対応を負例にしない」。再開表の行11は全nominated規則×既存入力の注釈監査を主眼にしており、対応形式・版の不足を解消する作業が抜けている。旧57単位とv2のどちらにもTSXがない。依存SBOMはCycloneDX1.5とSPDX2.3の正負・破損に限られ、公開対応範囲のCDX1.4/1.6/1.7ライブラリ入力およびSPDX2.2がない。crypto-only1.7は非対応の試験であり、1.7ライブラリ対応の正負・破損を代替しない。公開しているKubernetes/Ingress、nginx、Apache、Envoy、Istio、HAProxy、Traefik設定の対応形式も、当該コーパスのJava/SSH断片だけでは検証できない。

対応範囲と、正例・負例・破損例の単位/既存の意味ある試験・実測・根拠を結ぶ行台帳が必要。既存試験の再利用は可能だが、その試験が何の受入を満たすかを確認する。不足する対応範囲の必須検証を、単にNot Testedと記載してdoneにしない。適用外の理由と、未実施を明確に区別する。これは182単位のbounded source audit Verifiedを取り消す指摘ではなく、その監査をTASK-042全体の代替にできないという指摘。

### R2 [P1] Q1-03の方式・パラメータの両側または対応外理由が未整備

参照: `docs/quality/TASK-042-REOPEN.md:9`、元Q1-03受入、`tests/quality/expanded-v2/labels-a-final.json`。

元受入は「各方式・パラメータの正負または対応外理由をレビューし、出力を見る前に正解を固定」。再開表の「方式・パラメータの表」と専用規則のないHybridへの留保だけでは、この条件を満たさない。現Aの182単位ではDSA/ML-DSA/SLH-DSAは正例だけであり、方式・変種ごとの負例または対応外理由が示されていない。ML-KEMにも512/1024等の変種ごとに正負の在庫を照合する台帳がない。AES/HMACのような古典的代替、Hybrid/旧称/unknownも、どの規則に対する正負かを対応させる必要がある。

単に各方式名が1回以上出ること、方式別集計が生成できることは、元の両側評価を満たさない。対象の方式・変種・用途・形式と正負単位（または根拠のある適用外理由）を列挙し、足りない必要例を出力前に二者で固定する。未測定のパラメータを未対応と読み替えたり、受入表から両側要件を外したりしない。

### R3 [P2] 同一データ回帰でも新データ初評価と出力する

参照: `tests/quality/report.py:271`。

`newDataEvaluation.status=Verified` と `kind='First frozen evaluation; not a same-data version regression'` が、注釈付きコーパスに常に設定される。同一truth/inputのreferenceが与えられ、`accuracyComparison.status=Passed` となった再測定でも、この初評価宣言が残る。Q1-09の「同一入力回帰、事後tuning、新しい独立系統の初評価を別掲」と矛盾する。選択来歴の動的表示だけではkindの誤りを解消しない。

限定再現は既存EvidenceTestsの合成fixtureのみを使用し、製品スキャナーを実行していない。`EvidenceTests.enrich()`で作ったsummary自身をreferenceに `report.build_assessment(...)` へ渡すと、`accuracyComparison.status=Passed`、`newDataEvaluation.status=Verified`、上記first-evaluation kindが同時に返った（終了0）。

同一データ比較が成立する場合は再評価/回帰として表現する。単にreferenceがないことから初評価とも断定せず、初測定の根拠を束縛するか中立的な測定種別にする。v2の既観測由来tuningとRustCryptoの新規holdoutを分離し、同一データ・異なるtruth・fresh holdoutの試験を追加する。

## 確認できた実装

- AGENTS.mdの追加は、未達必須条件があればdoneにしない、過去証拠を保持して再開理由を記録する、結果と未実施を区別するという要求を明記している。
- tasks/042はin-progress、roadmapは再開と未完了を明記し、過去の限定受入を全要求の完了根拠にしていない。
- annotation_audit_statusは2名のdistinct identity、record SHA、protocol SHA、全規則×全ファイルscope、missingFacts/unresolvedDefinitions空、理由、正しい145/23/3335を検証する。
- validate_freezeはcorpusが要求するauditをtruth側で落とすことを拒否する。既存のload_truthによるcorpus binding・二者ラベル・裁定・freeze時系列・sourceOnly等の検証と接続される。
- 予測ゼロはauditの代替にされず、sourceAuditStatusとpredictionAttributionStatusが分離されている。未帰属があれば総合解釈をPartialに維持する。
- 既存v1をunflaggedのまま扱う場合も注釈完全性はPartialであり、過去v1をVerifiedへ引き上げない。
- スキャナー・規則・採点の変更は今回のコード修正対象外。実装変更からこれらの意味変更を要求する事情はない。

## 検証証拠と限界

主担当の `python-audit-final.log` は45件成功。`NATIVE-COMMANDS.json` は cargo build/fmt-check/test/clippy（locked/offline、Clippy -D warnings）の終了0と対応ログを記録している。全スイートは重複実行しなかった。R3の具体的懸念だけを既存合成fixtureで再現した。

今回のsource-only A最終記録は182単位・全3335組の注釈監査Verified。初回Partial記録・質問文参照履歴を保存し、出力非参照の範囲とtuning/fresh holdoutを明記した。この記録の妥当性と、未実測の最終成績・元9条件の全充足は別問題である。

v2はレビュー時点で未測定。二者の最終正解・裁定・freeze後にのみ測定する。Q1-01/02/04/06/07/09/10も、最終成果物・コマンド・hash・当初の受入行に対応する証拠を揃えるまで完了としない。最終試験妥当性はもう一方の指定担当によるレビューを必要とする。

## レビュー対象ソースのSHA256

- `AGENTS.md`: `2837947b6eb51216bb712cc15e4802ce2a85caf98088f6409956e58406fee314`
- `tests/quality/taxonomy.py`: `be7a90ad270ba5ede8986091322764cf2f8ea78ee36e9a97706efd75f6bb8858`
- `tests/quality/report.py`: `1406736384a7994728949828798b239c5145bb837be1b52b320932558d030acb`
- `tests/quality/test_report.py`: `b43703f0bf17a246f0f88f43b36a8c3c39e66045879cc4070db437912a7a4069`
- `docs/quality/TASK-042-REOPEN.md`: `7f5e6ed0f01244af2cf973d5f1430980ddc5348489201cbcbdfd7967c68af685`
- `tasks/042-quality-corpus-expansion.yaml`: `1be5913a177b905733d913f8769ac03093bdb7b5dee16aa00b706a68e2b42f1b`
- `tests/quality/expanded-v2/corpus.json`: `60234321633f7d6d68ab47a0a06cbdc68da83ffb2c34b5a977ead916ec3c274e`
- `tests/quality/expanded-v2/PROTOCOL.md`: `f61cb81685c9fbc49b52c16281acf945ecae68ceee68a82a5626b7663ec7866e`
- `tests/quality/expanded-v2/labels-a-final.json`: `0d5809b8515d06a870f6f1155a3df23f6a277442834099eb43262f3f65dfe1f2`

## 幅追加・既存試験再利用の追確認（2026-11-12）

R3は修正済み。report.pyはreferenceなしでも初評価を断定しない中立kindを使用し、同一truth/inputのreferenceが成立するとSame-data regression or validation replayを明示する。test_same_data_reference_does_not_claim_first_independent_evaluationは同一summaryでPassedとなる経路とkindを統合検証する。今回の指摘は解消した。全試験の再実行はしていない。

R1/R2のために追加した候補4は203files/246units/30rulesで二者freeze後に測定済み。A側の独立監査は6090組Verifiedで、原v2のtruth/原文/初測定は保持する。完全な要求対応表は主担当作成中のため、全体受入は未判定。

個別の不足確認:

- Ruby: 旧57のu022 puts ML-DSA-65はv2のliteral定義ではpositiveとなるため、旧negative判定を新定義の陰性根拠として流用しない。一方、pqc_readiness.rsのinventories_pqc_in_all_seven_languages_but_not_comments_or_similar_namesは、Rubyの # OpenSSL::PKey.generate_key('ML-DSA-65') と無関係な同名identifierで当該規則が出ないことを明示assertする。これは現在の定義に整合する意味ある既存負例として再利用できる。native-2.log:479に当該試験の成功が残る。単体試験の件数をv2精度母数へ加算しない。
- PEM/CRT/CER: v2内で各形式のRSA属性positive、既知RSA証明書に対するECDSA属性negative、壊れた文書のunsupportedが揃っている。negativeは完全に同一の証明書属性へ相対化されたものなので、RSA署名とSPKIサイズを混同しない。Q1-05の形式単位の両側・破損要求について、これ以上の例を不可欠とは判断しない。
- package.json: jose6.0.0はambiguousのまま保持する。pqc_readiness.rsのinventories_dependencies_without_claiming_pqc_is_enabledは@noble/post-quantum0.7.1の明示宣言に対して本番DEP_PQC_CAPABLE_LIBRARYをassertするので、別の意味ある既存positiveとして再利用できる。native-2.log:480に成功。公式tag付きREADMEで同ライブラリの標準PQC実装という根拠を確認した。特定の実行・有効化・インストールは主張しない。
- package-lock/pnpm: 既存dependency_provenance.rsのopenssl|ringによるDEP_TESTは抽出の検証であり、本番DEP_PQC_CAPABLE_LIBRARYのpositiveを代替しない。別版expanded-v2-lock-controlsへ認識対象@noble/post-quantumの2対照が追加された。最初のpnpm候補はline1が実宣言line3を覆わなかったが、初測定前に実行長に合うline3へ修正された。旧候補は別ファイルに保持。A記録は修正済みcorpusSHA55d668218071458baadd4857f8bac3043b1f5a4b48e583a0a08722b580cd5f73に対するpositive2件・全60組監査Verified。旧v2結果や製品規則の既知曝露を開示し、補遺の他者ラベル・補遺出力は未参照。二者固定・裁定・補遺測定は主担当が後続で行う。補遺はtuningのみで、独立holdoutでも原v2の初測定修正でもない。

上記の形式については、補遺2件の測定が揃えば具体的に把握している正負不足は解消できる。全受入判断では、R1/R2の行台帳、元9条件の最終証拠、実測と未実施の区別、最終試験妥当性のレビューが必要であり、この個別確認を全体完了の承認へ拡張しない。

- 追確認SHA `tests/quality/report.py`: `9035f436d9a720bfbcaf7748184bb6db1bbf84c321a185b239e95ae000111681`
- 追確認SHA `tests/quality/test_report.py`: `72f46d6b3edc50da99b28e6c7deb7bf70e1c6ca9648754d0847c55ddd2a6a074`
- 追確認SHA `tests/quality/expanded-v2-lock-controls/corpus.json`: `55d668218071458baadd4857f8bac3043b1f5a4b48e583a0a08722b580cd5f73`
- 追確認SHA `tests/quality/expanded-v2-lock-controls/labels-a.json`: `ef4f7fe6e74b6afd5d70a70a3a52cd107634e0d047a8d9ae5a1f364313c6a359`

## 形式・方式の最終対応表の追確認（2026-11-12）

FORMAT-MATRIXの49行、ALGORITHM-MATRIXの14方式、UNIT-LEDGERの248単位を確認した。各ledger単位の質問・位置・規則・注釈全フィールド・source/inputFormat/splitはv2またはlock-controlsの固定corpus/truthと一致した。形式行の正負・破損・曖昧・非対応IDと、方式行の所属ID/パラメータごとの注釈区分も全件照合し、一致した。043-FOLLOWUPは20原因群に元の50観測を重複なく保存している。

R1: 解消。公開形式・版の代表例について正負/破損を具体的に対応付け、Ruby陰性とpackage.json陽性の意味あるdefault-rule試験を再利用し、lock-controlsの陽性2件を別版の二者固定・測定証拠に結び付けた。旧Rubyの誤った負例を流用していない。未対応のSwift/crypto-only CBOMを安全な陰性として扱っていない。v2/補遺の形式以外に、元条件へ不可欠な新規例を追加する必要は現時点で確認されていない。

R2: 原ソース・有限参照検出評価に必要な対照は解消。標準Node ML-KEM3種/ML-DSA3種/SLH-DSA12種は正の有限ループとコメント陰性に対応し、同じループを独立標本として増やしていない。DSA正負、Java SLHの正負、RSA/SPKI/署名、その他の古典的代替も示される。ECDH/EdDSA/Hybridに一般source専用規則がないという説明は現在のdefault catalogと整合し、陰性対照・設定制限・実交渉を区別している。これは無限の方式×言語×版の組合せを要求するものではなく、今回の有限代表例の方式・パラメータをレビューした判断である。

最終文書では次の追跡性を整える:

- ALGORITHM-MATRIX JSONのpre_standard/RubyはcontrolUnitsByRuleのnegativeが空のため、意味ある既存負例の再利用もFORMAT行と同じくexistingTestsへ具体関数名・ソース・ログ付きで結び付ける。identifies_draft_algorithms_separately_from_final_mlkemとinventories_pqc_in_all_seven_languages_but_not_comments_or_similar_namesである。新しい検体や製品修正は不要。
- 043-FOLLOWUPから参照するPARSER-DIAGNOSTIC.mdは現時点で存在しないため、最終耐久証拠作成時に作成するか実在ログへの参照へ修正する。parser-diagnostic.logの全3入力はerror=trueのERROR木でcall/method_invocation nodeがなく、PARSER-COMMAND.jsonはexit0とcorpusExecuted=falseを記録する。製品を変更した・検出器を修正した証拠ではない。

R3は引き続き解消。実装上の未解消P1/P2はこの時点で確認されない。元9条件の最終受入は、主担当が残す結果・耐久証拠・最終検証・selfscanと指定試験レビューを照合してから行う。ここでは全体完了の承認を先取りしていない。

- 追確認SHA `docs/quality/task042-reopen-20261112/FORMAT-MATRIX.json`: `b72ee03b4c67451b61d2a2425e43567a40835127aa937b0099ef34b02e0ffd69`
- 追確認SHA `docs/quality/task042-reopen-20261112/ALGORITHM-MATRIX.json`: `fb692c677f8d2b949857e62b721a1acc89e35c6e50e23bf025e135d276d1ced2`
- 追確認SHA `docs/quality/task042-reopen-20261112/UNIT-LEDGER.json`: `7a3d2409ac0d979eb781b51e0f9562d23a4109dcf97bd797d0cb3fe1d740c56f`
- 追確認SHA `docs/quality/task042-reopen-20261112/043-FOLLOWUP.json`: `a024da55bbbf83ef900111e28a9821b2353ba82d5faf14519cff3d1ccfe8e92e`

## 最終証拠レビューでの追加指摘（2026-11-12）

判定: **追加の適用範囲修正が必要、最終承認は保留**。

### R4 [P1] 暗号化PKCS12の属性を、公開対応範囲内のFN/TNと混同しない

対象は既存のpublic-only.p12/pfxに対する6属性単位: v2-audit4/5/6/7（RSAサイズ/RSA署名positive）、x154/x156（ECDSA署名negative）。公開対応は非暗号化のpublic certificate subsetであり、この旧容器の暗号化cert contentsはopaque/unsupportedである。元DERと変換来歴からオブジェクト属性が分かることと、スキャナーがサポートする形式から属性を読めることは別である。

以前のA注釈はこのオブジェクト事実と検査適用性の分離が不十分だった。元truth・二者記録・初測定を変更しない。v2の全nominal-property一致表は歴史的記録として保持するが、未対応形式の4属性を製品の見逃し、2属性を対応内の正しい陰性と数えたまま、Q1-05の「未対応を分母から除外した」と受け入れることはできない。

元v2と別版の適用性記録/評価を作り、6属性をsupported-scope外と明示して二者確認し、元のオブジェクト属性一致とsupported-scopeの値を別掲する。filename/containerのx153/x155と破損filename signalはそのまま正例。新しい非暗号化public-unencrypted.p12/pfxの属性は対応内である。製品のサポート、暗号判断、既存採点仕様を変更する必要はない。

独立の手計算対照: 原表58TP/4FP/44FN/89TN・positive102/negative93・eligible195は不変。6適用外属性（FN4/TN2）を分離した対応内表は58TP/4FP/40FN/87TN・positive98/negative91・eligible189、適用外/unsupported53、ambiguous4。これは製品改善ではなく事後の適用範囲訂正である。

### R5 [P2] selfscanのraw/filtered SARIFを耐久証拠へ含める

RAW-MEASUREMENTS.tar.gzの70membersは4データのaccuracy raw/summaryを含むが、selfscanのSARIFを含まない。code-scanning-audit.jsonにはrawShaと計数・期待省略の監査があるものの、耐久側にはraw/filteredの実体が見当たらない。target削除後も1357raw/1329expected/28retainedと残存28件を確認できるよう、元SARIFとfilter後SARIFの保存先・hashを束縛する。既存の別保存先があればそのリンクでよい。

## その他の最終確認

- ACCEPTANCE-MATRIXは元9要求を維持し、R1/R2を解消した対応表と結果・正解・試験を結び付けている。全119規則を全て検出精度合格にする要件へすり替えていない。
- 4データのMEASUREMENTの全計数は保存quality.jsonのbySplitFamily合計と一致。RAW-MEASUREMENTSには全4summaryおよび反復rawがある。既存57とv1の同一データ回帰はPassed、v2は異なるtruth/inputとして比較合否を出していない。
- selfscan期待値の29具体ファイルは全source SHAが現バイトに一致し、proposalと現在expectationsのfindingリストも一致する。合計137所見はreport.pyの既存3所見のsource再bindと新しい静的対照/公開証明書/ラベルmetadata134件である。report.pyの最終reasonはproposalの一般文より具体的で、findingリスト変更はない。製品/実依存の残存28件を追加期待値へ吸収していない。
- Python47件成功、Node31件成功/失敗0の最終ログを確認。初回Node2失敗の履歴は旧source-binding不一致として別ログを保持し、最終成功と混同しない。workspace build/fmt/test356/Clippy exit0は既存の記録を使用し、再実行していない。
- CI追加は既存runner/reporterを再利用して旧/v1/v2/補遺を別出力へ保存する。数値ハードゲートや製品規則を変更していない。hosted CIで実行済みという主張はしていない。

- 最終追確認SHA `docs/quality/task042-reopen-20261112/ACCEPTANCE-MATRIX.md`: `9e8369199a64ccc6fd622611f6ab1977f0d972c586fa20df15522c086149e2cb`
- 最終追確認SHA `docs/quality/task042-reopen-20261112/MEASUREMENT.json`: `29817e098b01b06bc0843b382af752bdc8017db3b11126d91cef493cd5aa784c`
- 最終追確認SHA `docs/quality/task042-reopen-20261112/FORMAT-MATRIX.json`: `b72ee03b4c67451b61d2a2425e43567a40835127aa937b0099ef34b02e0ffd69`
- 最終追確認SHA `docs/quality/task042-reopen-20261112/ALGORITHM-MATRIX.json`: `721bd89673586d1d0363b0eea9170960a61596c28bde455a6fbd5a59df33bb64`
- 最終追確認SHA `docs/quality/task042-reopen-20261112/PARSER-DIAGNOSTIC.md`: `5bcb934706fc31565a58e130946788b1796d6c2c5ac03299c31c131caf6da2c0`

## 実装レビュー最終署名（2026-11-12）

判定: **現ソースの実装を承認**。未解消の実装P1/P2はない。これは実装レビュー段階の承認であり、最終試験レビュー・耐久証拠の最終整合・当初9受入全体のEvaluator完了判定を代替しない。TASK-042は主担当が引き続きin-progressで保持している。

R4は解消した。既存report.pyの任意--applicabilityは、二者の独立した事後ソース適用性レビュー、元corpus/truth、契約source/protocolのSHA、全IDとunsupported限定決定、残りscope確認、未解決空、distinct identity、review<=freeze時系列を確認する。原truthと原nominal計数を変えず、コピー上の6ラベルだけをunsupportedへ写して、既存scoreを再利用する。新しい一致判定エンジン・製品規則・リスク判断の変更はない。

生成quality.jsonは元58TP/4FP/44FN/89TN・eligible195を保持し、supported-scope.jsonと同一の別表58TP/4FP/40FN/87TN・eligible189・unsupported53を併記する。別表は全層（言語/規則/検出器/方式/方式parameter/用途/形式/artifact/case）と除外6件の元label/理由、レビュー・契約bindを保持する。事後適用性訂正であり、binary改善や新しい独立評価とは表示しない。CIはv2のみに同じ適用性記録を指定する。手計算TP->excluded、FN/TN->unsupported、ゼロ母数・元結果保持と、binding/identity/unresolved/ID/重複/negativeへの書換え/理由/時系列の拒否試験を確認した。python-scope-final.logの50件成功を確認し、全suiteの再実行はしていない。

R5も解消した。RAW-MEASUREMENTS.tar.gz内にself-scan/report.sarifとself-scan/code-scanning.sarifがある。rawSHAはcode-scanning-audit.rawSarifSha256と一致し、実体件数もraw1357/filtered28である。28件には未登録のテストfixtureも含まれるため、全てを実利用の暗号・本番依存とは解釈しない。最終indexでのarchive hash束縛は主担当の耐久証拠作成に含む。

最後に残っていたcomparison.jsonのreference未指定時の初評価断定も解消した。fallback reasonはNo same-data reference suppliedへ中立化され、test_selected_frozen_annotations_produce_all_task_outputsが出力JSONの当該reasonをassertする。既存同一referenceの回帰表示試験も維持している。

当初指摘R1/R2は先の追確認どおり解消し、R3と最終fallbackも解消。原コーパス・原ラベル・元結果・失敗履歴を保持したまま、対応形式と評価propertyの違いを明示する構成である。残作業は主担当の最終結果/LABEL-REVIEW/証拠index更新、変更後report.pyのselfscan source binding整合、指定試験レビューと全9条件への最終照合である。既存の356 Rust/Python50/Node31終了0という別種の証拠を混同せず、最新変更で必要になった限定検証だけを主担当が行う。

署名担当: implementation-reviewer-a / q42_reopen_implementation_review

- 承認対象SHA `AGENTS.md`: `2837947b6eb51216bb712cc15e4802ce2a85caf98088f6409956e58406fee314`
- 承認対象SHA `tests/quality/report.py`: `00eaf7993453a1d29a2a3c4e47b23af942ad975d5befe5d0cecb6542ba4fbdbf`
- 承認対象SHA `tests/quality/taxonomy.py`: `be7a90ad270ba5ede8986091322764cf2f8ea78ee36e9a97706efd75f6bb8858`
- 承認対象SHA `tests/quality/evaluate.py`: `4b341b0d4c8374615198d18a8ccf449de326f1df197ebc98a9994ddc8333351c`
- 承認対象SHA `tests/quality/run.py`: `7d2733b06c8e329b3c762b992d3308d34e1715d9e541b1faf693888652bf5ce9`
- 承認対象SHA `tests/quality/test_report.py`: `bac8f3d3e452a747b708eeee1b157581a413d0566bad74d02202b0cb5a1c9c31`
- 承認対象SHA `.github/workflows/pqc-scan.yml`: `cc7f7347b77713a5de1581933c6ee91a8316e624aff06b34a858468fd91c8ebe`
- 承認対象SHA `tests/quality/applicability-042-v2/assessment.json`: `b0857e2db9a4fbe7aae917e9af7a5ab7df5436df481516da68105cce6b3ddef5`
- 承認対象SHA `tests/quality/applicability-042-v2/review-a.json`: `6d86b778771e50d2fc7ce398429b9acc83c046cc5fc6a9e37eb08f30c22292d0`
- 承認対象SHA `tests/quality/applicability-042-v2/PROTOCOL.md`: `a3b0bae7f5ae7ac5da14d7430d521ac981642422118f8057aaeec40341f0c349`
