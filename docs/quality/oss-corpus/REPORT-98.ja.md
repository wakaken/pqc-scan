# 90件の主評価と8件の比較評価

> 2026-11-27の継続評価は[主評価27件・比較8件の報告](REPORT-35.ja.md)と[選定35件の実行条件](PROTOCOL-35.md)を参照してください。以下の98件の計画・測定・裁定は履歴として保持しています。将来対象63件は今回の完了条件に含めていません。

固定SHAのOSSで暗号の検出・分類・検査範囲・出力品質を確認し、再現した問題を修正して同じ入力で再評価することを目的とした記録です。2026年11月25日時点で、主対象90件と比較用8件の修正前・修正後の実行と機械出力の監査は成功しました。全体の評価は未完了です。利用者が指定したテスト用2ファイルの除外を反映し、変更後の取得範囲は揃いました。拡張正解の適用不備2区間は追補と採用ガードで扱い、影響する指標をnullとしました。管理した負荷条件での性能再測定と、その結果を含む最終確認が残っています。検出数は精度ではありません。

## 2026年11月26日の実施方針

利用者の指示により、新規の性能測定は一旦保留としました。静かなMacの準備や負荷監視を求めず、後日の再開指示があるまで測定を自動再開しません。過去の棄却記録は保持し、性能条件を達成済みには変更しません。全体精度のnullは全90件へ一般化できる数値を算出していない意味です。限定区間の数値や検出数とは区別します。性能測定を除く今回の取得・比較・修正・回帰試験・出力監査・必要なブラウザ確認と独立レビューは実施済みです。以下に残るPartialや未対応APIは製品の対応範囲・改善候補を示します。

## 精度評価の拡充作業を再開

2026年11月26日の追加指示により、上記の「性能測定以外は実施済み」という判断を訂正します。限定67区間の評価完了と、当初のコーパス全体に対する精度評価の充足を混同していました。正解データと精度評価の範囲拡充、製品改善候補の重複除去と件数・影響OSS数の集計を残作業として再開しています。性能測定と負荷監視の保留は維持します。以下の数値は拡充前の保存済み結果で、新しい精度評価の完了を示すものではありません。

## 精度拡充の現在の確認範囲

主評価90件から360箇所、比較用8件から32箇所を固定し、出力を見ない2名の注釈とソース・一次資料による裁定を保存しました。主評価では移行評価の範囲に入る126事実を62 OSSで、比較用では3事実を3 OSSで確認しました。これは検出結果との一致数ではありません。方式参照・操作・パラメータなどの事実を既存ルールへ対応付ける独立確認と採点が残っています。主評価の不明9事実と列挙未完了8箇所も保持しています。

出力側は主評価91,231件から固定した無作為300件と、比較用24件を別々に評価します。注釈の途中で、評価表示と製品出力に秘密値のマスク漏れを確認したため、安全な表示処理と製品修正の独立確認を先行しています。旧パケットと記録は削除せず、内容の再表示を止めています。新しい精度値はまだ確定していません。[精度拡充の証跡索引](../../../benchmark/evidence/accuracy-expansion-index.json) に現在の状態を記録しています。

## 対象と出典

対象台帳は [registry.json](../../../benchmark/registry.json)、実行入口は [entry.py](../../../benchmark/entry.py)、条件は [PROTOCOL-98.md](PROTOCOL-98.md) と [protocol.json](../../../benchmark/protocol.json) です。既存18件とv0.5由来30件を固定SHAで照合し、新規取得42件を加えました。主対象90件と比較用8件を別集計し、同一OSSの再試行を追加件数に含めていません。9グループ各10件です。JS/TSとC/C++を分けた11言語は、後掲の実ファイル拡張子別集計と区別します。

開始ソースは `cf3bfe2071228921f5a33d2a44e0f3b94de4d1de` です。作業は `codex/oss-crypto-corpus` に保存しています。v0.5対応の統合マージ `3cb487d` と元ソース `5ae84d5` を含み、言語対応を再実装していません。139ルールを維持し、パッケージの表示版は0.4.0です。統合済みであることはv0.5公開準備完了の判定ではありません。[ROADMAP-LINKS.md](../../../benchmark/ROADMAP-LINKS.md) にタスク076–087との関係を残しました。

ソース・大量出力は `/Users/waka/.cache/pqc-scan-oss98-20261125` に保存しています。URL、コミット、対象範囲、入力マニフェスト、実ファイルのハッシュは台帳から参照できます。ライセンスの事実確認98件は [license-evidence.json](../../../benchmark/license-evidence.json) に結び付けました。n8nのSustainable Use/Enterprise、GitLabの複数範囲、SustainsysとBitwardenの限定部分などを保持しています。全ツリーの統一ライセンス、OSS適格性、法的適合性を一括認定していません。

独立したパス・モード照合では475,994エントリを確認し、475,271件を入力に含めました。未展開gitlink27件、ライセンス限定範囲からの除外694件、当初の取得不全2件を別記しています。現在は利用者の明示指示で、Node.jsの `deps/v8/test/mjsunit/regress/regress-crbug-808192.js` とDiscourseの `spec/fixtures/images/fake.jpg` を評価範囲から除外しました。台帳の該当2件だけを変更し、他96件、入力バイト列、入力マニフェストは維持しました。元の全追跡ツリーを完全取得したという意味ではありません。保護機能を無効化・回避していません。[除外変更記録](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/fixture-exclusion-amendment.json) に変更前後の台帳と入力のハッシュがあります。各実行の前後で入力内容のハッシュを確認しました。ignore規則による除外、未対応・部分検査・Unknownは診断に残しています。外部OSSのビルド、スクリプト、依存解決は実行していません。

## 全体結果

主対象の検出数は90,638件から91,231件へ593件増加しました。比較用8件は12,642件で変わりません。増加はすべて `PQC_STANDARD_REFERENCE` です。既存IDの削除は0件、既存ID同士の相対順序は98件すべてで維持されました。既存2,673件の証拠の差は証明書の評価時刻によるもので、時刻を除いた差は0件でした。

正常終了は98件すべてで得られましたが、修正後の評価基盤による主対象の検査判定はComplete13件・Partial77件、比較用はComplete1件・Partial7件です。native診断は主対象でcomplete_within_scope14件・partial75件・unknown1件です。RustCrypto/KEMsはnativeで選択範囲内完了でも、未展開gitlinkがあるため入力側の判定はPartialになります。Tink Javaにはunknownの2ファイルがあります。全98件で `not_enumerated_population=true` が残るため、除外された全母集団まで検査したとは言えません。

## 修正と回帰確認

1. 標準PQC名のunderscore表記18種類の見逃しを修正しました。ML-KEMの3パラメータ、ML-DSAの3パラメータ、SLH-DSAのSHA2/SHAKE・128/192/256・s/fの12表記が対象です。ルールID、重大度、参照としての意味、既存スコープを維持しました。正例18件に加え、コメント、前後の識別子、未知パラメータなどの負例を確認しました。調整用Nobleでは該当表記の検出が0件から241件、18種類になりました。これは独立精度の改善値ではありません。241件にnativeの参照観察はありますが、対応するalgorithm assetは0件で、パラメータ分類の網羅性を示しません。
2. CBOMの文字列連結キーがコロンを含む異なる場所・一致値を同一視し、行を落とす問題を修正しました。構造化した組をキーにし、同じ行の列違いをまとめる既存動作を維持しました。実CLIの再現例は検出2件・CBOM1行から、同じ検出ID2件・CBOM2行になりました。修正前に失敗する回帰テストと、既存順序・同一行の併合・異なるルールを確認しています。
3. 評価基盤が列挙失敗を完了判定に含めない問題を修正しました。Synapseの列挙失敗4件、matrix-js-sdkの1件を、現行の比較・要約ではPartialと判定します。旧98件の生証跡は書き換えません。既知の旧評価器SHAに限って当時の判定を検証し、現行集計では診断から再判定します。新規実行は列挙失敗数を必須の非負整数として検証します。不正値、未知SHA、再開時の旧実行の混入を回帰確認しました。旧invocationのComplete15件を現行結果として使用しないでください。

4. HTMLのルール説明ダイアログで、末尾からのTabと先頭からのShift+Tabがダイアログ内を循環しない問題を修正しました。修正前の実ブラウザではBODYまたはブラウザ側への移動を確認しました。背景のレポート操作へフォーカスが漏れたという証拠ではありません。表示中の説明リンクと閉じるボタンだけを対象に端点を循環し、リンク0件では閉じるボタンに留めます。Escape、修飾キー付きTab、内部の通常移動を維持しました。実装と回帰テストの独立レビューは承認されています。[独立レビュー](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/focus-review-9df03425875c4ecd98445c83f1985171/REVIEW.md) は製品全体の完了承認ではありません。

測定ヘルパーでは、既存出力の上書き拒否、子プロセス群の終了処理、期限検証、ハッシュの分割読み込みも確認・修正しました。出力監査と要約の修正には、CBOMの正当な併合と不明なマスキング範囲の区別、不整合な集計の拒否、CSVの制御文字の拒否、生成途中の失敗時に既存ファイルを守る検証を含みます。

## 暗号・用途と正解の範囲

主対象の修正後nativeカテゴリは、CryptoAPI21,194件、JWT25,243件、KeyMaterial2,666件、Middleware45件、PKI22,535件、PQC5,254件、SSH4,193件、TLS10,101件です。カテゴリ、プロトコル、方式、用途を同一の単位として足し直していません。

過去の64観察は18プロジェクトのソース事実として固定SHA・ファイル・行のハッシュを再照合しました。実装12、設定12、テスト・例4、API利用32、依存4です。指定行区間に出力がある観察は16件から19件、同じファイルに出力がある観察は43件から46件でした。native観察は指定区間で22件から25件、algorithm assetを伴うものは8件で変わりません。これらは出力接触のある歴史的資料で、独立正解でもRecallの分母でもありません。

指定ソース区間の技術別の状況です。タグは重複します。区間内に出力がない観察をFNへ読み替えていません。

| 技術タグ | ソース事実数 | 指定区間に出力・前 | 指定区間に出力・後 |
|---|---:|---:|---:|
| DH | 1 | 0 | 0 |
| EC-unspecified | 1 | 1 | 1 |
| ECDH | 13 | 4 | 4 |
| ECDSA | 8 | 1 | 1 |
| JOSE | 2 | 0 | 0 |
| JWT | 8 | 1 | 1 |
| ML-DSA | 8 | 1 | 2 |
| ML-KEM | 19 | 7 | 9 |
| RSA | 12 | 5 | 5 |
| SSH | 13 | 3 | 3 |
| X.509 | 4 | 0 | 0 |
| crypto-library | 1 | 0 | 0 |
| hybrid-PQC | 10 | 4 | 6 |

SLH-DSAはこの過去64観察に含まれません。別の調整用18宣言で12種類のSLH-DSA表記をソース確認し、修正後のNobleの参照検出にはSHA2/SHAKE・128/192/256・s/fの全12表記が含まれました。ML-KEMは3表記、ML-DSAも3表記です。方式名の参照を数えた結果で、API操作・用途・標準適合の網羅性は未評価です。

RSA、ECDSA、ECDH、X.509、SSH、JWT、ML-KEM、ML-DSA、ハイブリッド参照について、ソースの役割と一次資料を [primary-references.json](../../../benchmark/evidence/primary-references.json) と [追加一次資料](../../../benchmark/evidence/primary-references-supplement.json) に結び付けました。SLH-DSAを含むNobleの18パラメータ宣言は [source facts](../../../benchmark/evidence/holdout-source-facts.json) にあります。このファイルの旧holdout名・当初の未裁定状態は歴史として残し、[split-amendment.json](../../../benchmark/evidence/split-amendment.json) で正式測定前にNobleを調整側へ移した記録を優先します。この旧分割の調整49件・未使用検証候補41件は歴史的計画です。後述の67ソース区間に対する調整20区間・holdout47区間とは別の単位です。標準の名称との対応は、実装の適合性検証や実行時利用の証明ではありません。ハイブリッドの組合せ参照も、実際の交渉、用途、標準化状態が確定したとは扱いません。

初期の数値採点は、RustCrypto/KEMsとRustCrypto/signaturesの重複しない12トークン区間を対象にしました。宣言の正例6、文書コメントの負例6について、前後ともTP6・FP0・FN0・TN6、Precision/Recall/F1は各1.0でした。採点器は既存の `tests/quality` を再利用しています。同一モデルのラベル担当、意図的な小標本、ローカルの時系列証拠という制約があり、別エージェントの一致だけで独立性を認定していません。採点外の同ルール327件と他ルール58件を除外した限定評価です。全ファイル・OSS・言語・全90件の精度はnullです。重複行を含んだv1正解案は不採用のまま保存し、v2だけを使用しています。

### 67ソース区間の追加評価

追加評価は16リポジトリの57選定ファイルと3補助ファイルに基づく67区間です。9グループ・11ソース言語を含み、マニフェストは別区分です。RSA、ECDSA、ECDH、X.509、SSH、JWT、ML-KEM、ML-DSA、SLH-DSA、ハイブリッドについて、呼び出し、宣言、実装、設定、コメント、依存を区別しました。方式名、役割、パラメータが確定できない箇所はUnknownを維持し、標準名の記載を実装適合や実行時利用の証明に置き換えていません。[採用時点の固定記録](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/truth-admission-fb794fd22a94498982e9ea8f03150ff7/admission-receipt.json) と [評価次元](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/adapter-independent-review-531db06b39f247b0bf54e148050176cd/truth-dimensions.json) に対象・根拠・分割を結び付けています。

調整20区間とholdout47区間の分割、元の採点対象49区間、適用ルール、区間を、新しい16リポジトリの前後実行に先立って固定しました。ソースを読む担当A・Bと裁定担当は今回の予測を見ずに判定しましたが、同じモデル系列です。判定の一致を統計的独立性の証明には使いません。妥当性の根拠はソースと一次資料です。親担当には既存プロジェクト出力への接触があり、完全な新規プロジェクト独立評価ではありません。

固定後の前後16組はすべてComparableで、32実行の出力監査が成功しました。比較8件の再実行も8組Comparable、16実行の監査成功、schemaエラー0でした。[前後16組の実行記録](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/prospective-scoring-58e8d1e3bcc94f8c82b01726ec53b5f0/prospective-execution.json) と [比較8件の監査集計](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/prospective-scoring-58e8d1e3bcc94f8c82b01726ec53b5f0/completion.json) を保存しています。これらは実行と形式の整合性の証拠であり、正解の適用不備を解消する証拠ではありません。

元の凍結正解による前後の結果は同じでした。holdoutの古典暗号・プロトコル層はTP10・FP0・FN1・TN9、依存層はTP0・FP0・FN1・TN1です。ただし、この2つのFNを製品の見逃しと認定することはできません。全67区間のアンカー契約を監査し、元の採点対象49区間のうち2区間に適用不備を確認しました。

- `expanded-source-026` の `padding.PKCS1v15` は列46からの呼び出しとして既に検出されています。凍結正解は末尾属性の列54からを指定し、検出位置を含みませんでした。
- `expanded-source-029` は `pyproject.toml` のoptionalな `cryptography` 宣言です。この入力形式は文書化された依存パーサーの対象外です。さらに、既知行の依存検出の列1は指定区間 `[6,18)` に含まれません。

元のソース事実、ラベル、区間、採点結果は書き換えません。製品側の検出位置移動や `pyproject.toml` 対応追加で採点へ合わせる変更も行いません。[全67区間の適用妥当性追補](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/fn-rootcause-31089a6b96444a628dcd6a8862f121e0/validity-supplement.json) は出力接触後の追補です。全67区間のソース契約を独立に再確認し、026・029を評価対象外とする追補を採用しました。元の2件のFNはそのまま保存し、検出改善や新たな独立holdoutの数値へ置き換えていません。影響する古典暗号・プロトコル層、依存層、これらを含むholdout全体の公式Precision・Recall・F1は `null` に留めます。残る47採点区間を用いる再計算を示す場合も、事後の感度分析であり、新規holdoutや検出改善の測定とは呼びません。

2区間を含まない層は、次の限定的な記述統計として保持できます。全90件の母集団精度への推定には使えません。

| 分割・層 | TP | FP | FN | TN | Precision | Recall | F1 |
|---|---:|---:|---:|---:|---:|---:|---:|
| holdout・PQCソース | 11 | 0 | 0 | 1 | 1.0 | 1.0 | 1.0 |
| 調整・古典暗号とプロトコル | 7 | 0 | 0 | 2 | 1.0 | 1.0 | 1.0 |
| 調整・ソース定義 | 5 | 0 | 0 | 1 | 1.0 | 1.0 | 1.0 |

[元の凍結採点結果](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/prospective-scoring-58e8d1e3bcc94f8c82b01726ec53b5f0/prospective-scores.json) は影響する元の2層も含めて保存しています。小標本の意図的な区間選定、同じモデル系列、既存出力への接触という制約は、数値1.0の層にも適用されます。[採用ガード](../../../benchmark/annotation_contracts.py) は、元のpositive・negative全49件のソース契約を要求し、不足・不一致を拒否します。[統合後の検証](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/integrated-admission-c86d08d94d7e4571aa3aacbff50614c8/receipt.json) で回帰15件が成功し、実67区間では026・029だけをnot-evaluableとして拒否しました。既存の除外18件と、契約に適合する47件を区別しています。手動の契約値自体の正しさや独立性を、このツールだけで認定するものではありません。[公開用の指標追補](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/final-scope-validity-review-67b1e759ba2a4047ae1c30e07b749f05/qualified-metrics-overlay.json) は影響する22集計をnullとし、元の採点結果を保持しています。

未検出APIの代表例はRbNaClのEd25519・X25519とXChaCha20-Poly1305、liboqs-pythonの `KeyEncapsulation` / `Signature` の構築・操作の役割です。名称参照の検出とAPIの意味の解析を分けています。現行の有限ルールが約束しないライブラリ対応を、既存保証の退行とは扱いません。次に優先するのは、これらの小さなAPI規則、動的セレクタのUnknown維持、underscore名のnative正規化の仕様・互換性検討、対称暗号と量子移行リスクの分類の区別です。libsodium本体、liboqs本体、liboqs-cppは今回の台帳外で未評価です。

## 出力形式とブラウザ

保存した両フェーズ各98件の出力について、native JSON、診断、inventory、SARIF、従来CBOM・依存SBOM、CycloneDX1.7の2形式の内容・集計・参照・schemaを監査し、全件通過しました。HTML本体、ルール説明HTML、Markdownも実際の出力集合に含めて比較しました。SARIFの位置情報省略、URIエンコード、従来CBOMの同一行併合は、形式固有の投影として区別しました。マスキングやbounded source factで元の同一性が失われる範囲は、確定した重複とはせず監査上の不確実性を記録しています。

正規化したreportの意味と全配列順序が同じなのは主対象84/90件、比較8/8件です。差のある主対象6件は追加593件の対象と一致します。診断の比較は全98件で同じでした。ルールカタログのdigestとnativeのrun・参照IDを保持する出力では、検出不変のOSSでもfingerprintが変わります。HTML・Markdown・inventory・CycloneDXの全98組が同一だったとは報告していません。すべての差をdigestだけで説明できたという主張もしていません。各出力の参照整合性と前後の比較結果をそれぞれ証跡として保持します。

最新ビルドと直前の統合ビルドの286製品ソースファイルを照合し、差が `compact.js` だけで、ルールは同一であることを確認しました。[ソース再利用の証拠](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/html-asset-derivation/source-reuse-proof.json) により、変更のない検出・機械出力経路について、入力・設定の結び付きを保った旧証拠を再利用します。全98件の修正後HTMLには新しいスクリプトを一度だけ差し替えた別ファイルを作成し、置換外の前後バイト列のハッシュ一致を確認しました。[98件の派生出力索引](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/derived-cohort-0e9943c2396848238ba209af5a66a9fe/index.json) は派生物です。98件すべてを最新CLIで再描画したものでも、native JSONから文脈を復元したものでもありません。他10形式は元出力へのハッシュ付き参照を保っています。補助ツールの実装とテストは [独立レビュー](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/adapter-independent-review-531db06b39f247b0bf54e148050176cd/review.json) で承認されています。後続の独立確認では、全98件の置換前後・ソース・入力・実行証跡を照合し、実装とテストの適切性を確認しました。これは管理条件付き性能評価や全作業の完了を承認するものではありません。

従来のChrome観察では、最大のBoringSSLレポート86,890,163バイト・14,283件の表示項目（nativeの14,380件から等価参照を97件集約）、空入力、英日、カスタムルールの翻訳未登録表示を確認しました。最大レポートのページ送り、検索、View・重大度・ルール・ファイルの複合条件、解除を操作しました。ML-KEMという検索語で0件になる例は検索対象がルール・説明・ファイルであることと区別し、`PQC_STANDARD_REFERENCE` では38件を確認しました。ユーザー操作のソートは提供されていないため、表示された重大度順のページを確認しました。BoringSSLのnativeとHTMLの差97件は、同じ場所・証拠・問題・リスクを持つJOSE等価参照の集約です。実出力のhashと投影キーを独立に照合し、不整合とは扱っていません。

通常1484×1129と390×844で描画を確認し、狭い画面の日本語ダーク表示で横方向のはみ出しはありませんでした。入力のラベル、Tabで検索から重大度への移動、説明ダイアログのEscapeとフォーカス復帰、ループバックHTTPサーバーを止めた後の検索を確認しました。これはWCAG全項目の適合認定ではありません。ファイル名・説明・コード・一致値に `</script>` と無害なDOM canaryを置き、展開後・ルール説明画面でも画像・イベントハンドラの注入とcanary変更は0でした。境界レポートは合成入力で、98件や性能結果に加算していません。

初回navigationとAX取得を合わせた自動化の往復は約11.6秒でした。純粋な描画時間としては扱いません。ページ全体のAX転送が64MiB上限に達し、一部のDOM操作にも自動化側の期限超過がありました。範囲を絞った確認と画面キャプチャでは表示と操作を観察できています。全ページのAX取得を完了したという主張はしません。後続の計測では、自動化の往復とブラウザ内の時間を分けました。

最新の製品バイナリから生成した実CLIの小さい英日レポート、空レポート、BoringSSLレポートで、修正後の端点循環をブラウザで観察しました。説明リンク0件・1件・52件の順方向・逆方向、Escapeによる閉鎖と呼び出し元への復帰を確認しています。[小規模・空の実CLI観察](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/browser-measurement/actual-cli-focus-checks.json) と [大規模の実CLI観察](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/browser-measurement/actual-large-focus-checks.json) は最新バイナリのハッシュに結び付いています。最新の大規模HTMLは86,890,712バイトです。旧HTMLの性能観察と同じバイト列ではありません。

従来HTMLを5回読み込んだブラウザ内Performance APIの観測では、First Contentful Paintは88〜308ms、中央値148msでした。loadEventEndは4204.7〜7505.7ms、中央値4553.4msです。検索や条件変更後の2回のrequestAnimationFrameまでの時間も採りましたが、物理的な画面表示の遅延そのものではありません。同一操作に由来するinputとchangeを独立試行として数えていません。OS負荷と埋め込み表示の条件を残し、改善率や因果的な速度差を主張しません。[ブラウザ内測定](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/browser-measurement/summary.json) に生観測と制約があります。従来の狭い画面、注入境界、オフライン確認は変更のない経路に限る証拠として維持し、WCAG全体の適合認定には使いません。

閲覧用の最新URLは [BoringSSL実CLIレポート](http://127.0.0.1:8769/actual-final-large/report.html#rule-49) です。[最新大規模レポートの日本語画面](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/browser-measurement/actual-large-focus-ja.jpg) を保存しています。

## 性能と検証状態

全98件の前後それぞれ1回のwall時間・CPU時間・子プロセスのピークRSSは実行証跡にあります。子プロセス群の合算メモリではなく `wait4` の高水位です。出力生成を含み、ビルドは時間外です。前後で処理する検出・CBOM行が変わる場合があるため、同じ仕事量の高速化とは言えません。

11代表・前後各5回の交互測定は [performance-design.json](../../../benchmark/evidence/performance-design.json) に固定し、実行ドライバーを独立レビューしました。結果は後掲に歴史的測定として残しています。修正後バイナリは最新のHTML修正前であり、OS側の負荷も残ります。最新製品の管理条件下での性能再評価の代わりには使いません。

最新の製品バイナリSHA-256は `c611e81306fbcde82091ba57690fac45ad798133e6e45809b089e045a9fb117b` です。[最新build receipt](../../../benchmark/evidence/final-accessible-build-receipt.json) に製品ソース・dirty差分・ルール・環境を結び付けています。Rust1.93.1でdefault987件、all-features1049件、Node30件のテストが成功しました。fmt、両構成のclippy、build、release build、JavaScript構文確認も終了コード0です。既存ignoredはdefault3件、all-features5件で、新規免除を加えていません。構成ごとにCargo出力を分離しました。[最新統合検証](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/final-accessible-validation-hi3k42ik/validation-receipt.json) にコマンド・ソース照合・結果があります。

最新バイナリを指定したPython品質テスト247件は成功しました。採用ガードを統合した後にも全262件が成功し、既存skip1件を維持しました。[Python品質テスト](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/final-python-ecbf6236c3334fb9b9bcb7a7c7516055/receipt.json) はバイナリと評価ソースのハッシュを記録しています。旧証跡を維持した検査状態の補正4行は [inspection-reassessment.json](../../../benchmark/evidence/inspection-reassessment.json) にあります。各修正の独立レビューと、拡張正解・派生98件・最終統合報告の採否は別の判断です。既存CIの品質ジョブは小さい合成入力を対象にし、大規模OSS評価は明示的なローカル実行として分けています。リモートCI、push、PR、merge、公開は行っていません。

管理条件付き性能ドライバーでは、準備時間が実測区間へ混ざる旧案と、macOSのCPU統計キャッシュにより短時間区間が歪む旧案を不採用としました。最新案は公開APIの `host_processor_info` を新しく読み、実行区間の境界を揃えます。著者テスト53件と独立テスト8件が成功し、[CPU読取の独立レビュー](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/fresh-cpu-review-b7218a7e01634764a697a1e972a055a5/review.md) は限定した実装・テストを承認しています。実測の受入れを承認したものではありません。

最新の運用条件は、開始前30秒の平均使用量0.5コア以下・最大1コア以下、thermal nominal、通常のメモリ状態、swap増加なしです。実行中は各観測窓で全体1.5コア以下・推定外部負荷0.5コア以下を条件とします。CPUカウンターの量子化と差し引き推定の制約があり、数値条件は干渉の誤差上限ではありません。ディスク負荷の隔離も未証明です。こちらの重い処理を終えた後に、監視器の実preflightは15点・エラー0で終了しました。その後900.01秒間、開始条件を待ちましたが成立しませんでした。事前確認を含む約914.81秒の監視全体では平均3.959コア、最大5.256コアでした。thermalはnominal、メモリはnormal、swap増加は0です。スキャナーの実測開始は0件、受入れペアは0/5で、待機上限による棄却を保存しました。[性能測定の開始条件と棄却記録](/Users/waka/.cache/pqc-scan-oss98-20261125/completion/5d611708ca344bfeac5174bd9001cd81/controlled-performance-admission-outcome.json) に生観測・測定器・terminalのハッシュを結び付けています。OSや保護サービスは停止していません。因果的な速度改善値は `null` です。

## 未完了の条件と再開

利用者が指定した2ファイルの除外後、変更後の取得範囲は揃いました。比較8件の前後再実行と形式監査も成功しています。当初の取得不全記録は保存し、原ツリーの完全取得へ読み替えていません。

67区間の適用妥当性追補、採用ガード、98件の派生出力と再利用根拠は独立確認を終えました。残る実施条件は、最新バイナリを使った管理条件下での反復性能測定です。このMacの負荷が開始条件を満たす状態になれば、検証済みドライバーで新規実行を開始し、同じ11代表の前後5組を測定できます。再開時にはスキャン・ビルド・監査・ブラウザ操作を重ねず、最新のafterバイナリとbuild receiptを指定します。今回の棄却済み実行や旧110サンプルを、受入れ済みの性能測定へ読み替えません。判断記録の最終確認も保存対象です。拡張正解の影響する層と全体の公式精度は `null` のままです。実装テストや実行成功だけで全評価を完了と判定しません。

## グループ別の検出数

| 主対象グループ | OSS数 | 修正前 | 修正後 |
|---|---:|---:|---:|
| C / C++ | 10 | 30,189 | 30,189 |
| C# | 10 | 10,627 | 10,627 |
| Go | 10 | 6,708 | 6,708 |
| Java | 10 | 17,684 | 17,877 |
| JavaScript / TypeScript | 10 | 2,281 | 2,530 |
| PHP | 10 | 1,892 | 1,892 |
| Python | 10 | 15,339 | 15,490 |
| Ruby | 10 | 3,306 | 3,306 |
| Rust | 10 | 2,612 | 2,612 |

## 実ファイルの言語別の検出数

ファイル名・拡張子による1検出1区分の帰属です。プロジェクトの宣言言語の重複加算はしません。`.h`、設定・マニフェスト、その他は別区分です。

| 区分 | 主対象・前 | 主対象・後 | 比較8件・前後同数 |
|---|---:|---:|---:|
| C | 5,530 | 5,530 | 1,439 |
| C++ | 1,047 | 1,047 | 97 |
| C# | 9,881 | 9,881 | 0 |
| Go | 4,806 | 4,806 | 1,854 |
| Java | 11,878 | 12,071 | 3,292 |
| JavaScript | 946 | 952 | 887 |
| PHP | 1,432 | 1,432 | 0 |
| Python | 3,287 | 3,438 | 59 |
| Ruby | 2,662 | 2,662 | 73 |
| Rust | 2,297 | 2,297 | 8 |
| TypeScript | 1,263 | 1,506 | 283 |
| C/C++ header unresolved | 786 | 786 | 958 |
| Manifest/configuration | 18,165 | 18,165 | 754 |
| Other/unattributed | 26,658 | 26,658 | 2,938 |

## OSS別の現在の状態

実行は前後ともSucceededです。検査判定は前後で同じで、現行評価器による再判定です。各行の固定SHA・入力制約・範囲・診断は台帳と機械要約から参照できます。表の件数を精度や実際の暗号利用数とは解釈しません。

| 母集団 | OSS | グループ | 検査判定 | 検出・前 | 検出・後 |
|---|---|---|---|---:|---:|
| 主対象 | bcgit/bc-java | Java | Partial | 10,329 | 10,494 |
| 主対象 | Keyfactor/ejbca-ce | Java | Partial | 614 | 617 |
| 主対象 | apache/mina-sshd | Java | Partial | 447 | 447 |
| 主対象 | mwiede/jsch | Java | Complete | 1,402 | 1,402 |
| 主対象 | apache/santuario-xml-security-java | Java | Partial | 279 | 279 |
| 主対象 | spring-projects/spring-security | Java | Partial | 1,149 | 1,149 |
| 主対象 | jwtk/jjwt | Java | Complete | 905 | 905 |
| 主対象 | tink-crypto/tink-java | Java | Partial | 2,283 | 2,308 |
| 主対象 | apache/kafka | Java | Partial | 242 | 242 |
| 主対象 | apache/pdfbox | Java | Partial | 34 | 34 |
| 主対象 | cloudflare/circl | Go | Partial | 269 | 269 |
| 主対象 | smallstep/certificates | Go | Complete | 1,472 | 1,472 |
| 主対象 | gravitational/teleport | Go | Partial | 798 | 798 |
| 主対象 | openbao/openbao | Go | Partial | 852 | 852 |
| 主対象 | FiloSottile/age | Go | Complete | 73 | 73 |
| 主対象 | golang/crypto | Go | Partial | 322 | 322 |
| 主対象 | cloudflare/cfssl | Go | Partial | 2,798 | 2,798 |
| 主対象 | quic-go/quic-go | Go | Complete | 11 | 11 |
| 主対象 | caddyserver/caddy | Go | Complete | 47 | 47 |
| 主対象 | ethereum/go-ethereum | Go | Partial | 66 | 66 |
| 主対象 | ruby/openssl | Ruby | Partial | 238 | 238 |
| 主対象 | net-ssh/net-ssh | Ruby | Partial | 688 | 688 |
| 主対象 | jwt/ruby-jwt | Ruby | Partial | 537 | 537 |
| 主対象 | onelogin/ruby-saml | Ruby | Partial | 31 | 31 |
| 主対象 | RubyCrypto/rbnacl | Ruby | Partial | 0 | 0 |
| 主対象 | cedarcode/webauthn-ruby | Ruby | Partial | 136 | 136 |
| 主対象 | unixcharles/acme-client | Ruby | Partial | 80 | 80 |
| 主対象 | puppetlabs/puppet | Ruby | Partial | 187 | 187 |
| 主対象 | gitlab-org/gitlab | Ruby | Partial | 1,246 | 1,246 |
| 主対象 | rubygems/rubygems | Ruby | Partial | 163 | 163 |
| 主対象 | pyca/cryptography | Python | Partial | 2,979 | 2,979 |
| 主対象 | paramiko/paramiko | Python | Complete | 263 | 263 |
| 主対象 | certbot/certbot | Python | Partial | 292 | 292 |
| 主対象 | Legrandin/pycryptodome | Python | Partial | 8,520 | 8,671 |
| 主対象 | jpadilla/pyjwt | Python | Complete | 344 | 344 |
| 主対象 | authlib/authlib | Python | Partial | 1,157 | 1,157 |
| 主対象 | ronf/asyncssh | Python | Partial | 330 | 330 |
| 主対象 | open-quantum-safe/liboqs-python | Python | Partial | 17 | 17 |
| 主対象 | MatthiasValvekens/pyHanko | Python | Partial | 1,354 | 1,354 |
| 主対象 | element-hq/synapse | Python | Partial | 83 | 83 |
| 主対象 | panva/jose | JavaScript / TypeScript | Partial | 973 | 973 |
| 主対象 | openpgpjs/openpgpjs | JavaScript / TypeScript | Partial | 29 | 37 |
| 主対象 | digitalbazaar/forge | JavaScript / TypeScript | Partial | 163 | 163 |
| 主対象 | paulmillr/noble-post-quantum | JavaScript / TypeScript | Partial | 47 | 288 |
| 主対象 | paulmillr/noble-curves | JavaScript / TypeScript | Partial | 72 | 72 |
| 主対象 | mscdex/ssh2 | JavaScript / TypeScript | Partial | 228 | 228 |
| 主対象 | auth0/node-jsonwebtoken | JavaScript / TypeScript | Partial | 506 | 506 |
| 主対象 | MasterKale/SimpleWebAuthn | JavaScript / TypeScript | Complete | 263 | 263 |
| 主対象 | ethers-io/ethers.js | JavaScript / TypeScript | Complete | 0 | 0 |
| 主対象 | matrix-org/matrix-js-sdk | JavaScript / TypeScript | Partial | 0 | 0 |
| 主対象 | rustls/rustls | Rust | Partial | 792 | 792 |
| 主対象 | Eugeny/russh | Rust | Complete | 93 | 93 |
| 主対象 | aws/aws-lc-rs | Rust | Partial | 994 | 994 |
| 主対象 | RustCrypto/KEMs | Rust | Partial | 167 | 167 |
| 主対象 | RustCrypto/signatures | Rust | Partial | 224 | 224 |
| 主対象 | briansmith/ring | Rust | Partial | 26 | 26 |
| 主対象 | sequoia-pgp/sequoia | Rust | Partial | 247 | 247 |
| 主対象 | rustls/rcgen | Rust | Complete | 53 | 53 |
| 主対象 | cloudflare/boringtun | Rust | Partial | 0 | 0 |
| 主対象 | sigstore/sigstore-rs | Rust | Complete | 16 | 16 |
| 主対象 | phpseclib/phpseclib | PHP | Partial | 195 | 195 |
| 主対象 | firebase/php-jwt | PHP | Partial | 289 | 289 |
| 主対象 | lcobucci/jwt | PHP | Partial | 78 | 78 |
| 主対象 | web-token/jwt-framework | PHP | Partial | 940 | 940 |
| 主対象 | robrichards/xmlseclibs | PHP | Partial | 27 | 27 |
| 主対象 | onelogin/php-saml | PHP | Partial | 45 | 45 |
| 主対象 | simplesamlphp/simplesamlphp | PHP | Partial | 8 | 8 |
| 主対象 | nextcloud/server | PHP | Partial | 305 | 305 |
| 主対象 | PHPMailer/PHPMailer | PHP | Partial | 4 | 4 |
| 主対象 | roundcube/roundcubemail | PHP | Partial | 1 | 1 |
| 主対象 | openssl/openssl | C / C++ | Partial | 6,031 | 6,031 |
| 主対象 | google/boringssl | C / C++ | Partial | 14,380 | 14,380 |
| 主対象 | wolfSSL/wolfssl | C / C++ | Partial | 3,304 | 3,304 |
| 主対象 | randombit/botan | C / C++ | Partial | 4,349 | 4,349 |
| 主対象 | openssh/openssh-portable | C / C++ | Partial | 463 | 463 |
| 主対象 | strongswan/strongswan | C / C++ | Partial | 573 | 573 |
| 主対象 | bitcoin/bitcoin | C / C++ | Partial | 69 | 69 |
| 主対象 | curl/curl | C / C++ | Partial | 169 | 169 |
| 主対象 | libssh2/libssh2 | C / C++ | Partial | 243 | 243 |
| 主対象 | lsh123/xmlsec | C / C++ | Partial | 608 | 608 |
| 主対象 | bcgit/bc-csharp | C# | Partial | 9,183 | 9,183 |
| 主対象 | AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | C# | Partial | 449 | 449 |
| 主対象 | openiddict/openiddict-core | C# | Partial | 109 | 109 |
| 主対象 | ITfoxtec/ITfoxtec.Identity.Saml2 | C# | Partial | 95 | 95 |
| 主対象 | Sustainsys/Saml2 | C# | Partial | 22 | 22 |
| 主対象 | jstedfast/MimeKit | C# | Partial | 162 | 162 |
| 主対象 | win-acme/win-acme | C# | Partial | 65 | 65 |
| 主対象 | OrchardCMS/OrchardCore | C# | Partial | 40 | 40 |
| 主対象 | bitwarden/server | C# | Partial | 195 | 195 |
| 主対象 | dotnet/aspnetcore | C# | Partial | 307 | 307 |
| 比較 | keycloak/keycloak | Java | Partial | 4,072 | 4,072 |
| 比較 | apache/airflow | Python | Partial | 56 | 56 |
| 比較 | kubernetes/kubernetes | Go | Partial | 1,989 | 1,989 |
| 比較 | nodejs/node | JavaScript / TypeScript | Partial | 5,978 | 5,978 |
| 比較 | n8n-io/n8n | JavaScript / TypeScript | Complete | 282 | 282 |
| 比較 | discourse/discourse | Ruby | Partial | 72 | 72 |
| 比較 | rails/rails | Ruby | Partial | 8 | 8 |
| 比較 | vectordotdev/vector | Rust | Partial | 185 | 185 |

## 歴史的な反復性能測定

以下は最新HTML修正前のバイナリを使った保存済みの測定です。管理条件付き再測定の結果ではありません。11代表を前後各5回、計110回実行し、失敗0件・55組すべてComparableでした。5巡で条件のハッシュは一致しました。ビルド・テスト・取得・出力監査・ブラウザ操作・プロファイラを測定と重ねず、1ワーカーで直列実行しました。測定前の入力ハッシュ確認によりソースキャッシュはwarmです。小さい入力を初回の時間を見た後で選んだ代表であり、90件の無作為標本ではありません。

12論理CPUのホストで、記録した1分load averageは3.40から8.56の範囲でした。OSやユーザー側の負荷まで隔離した専用ホスト測定ではありません。厳密な他負荷隔離の条件は未達として残します。以下は資源量の観測であり、優劣の因果判定はinconclusiveです。全11件で前後の時間範囲が重なり、速度改善・退行を確定していません。

| 代表OSS | wall秒・前 中央値［最小–最大］ | wall秒・後 中央値［最小–最大］ | CPU秒・前/後 中央値 | RSS MiB・前/後 中央値 |
|---|---:|---:|---:|---:|
| apache/mina-sshd | 2.969［2.926–3.519］ | 3.181［2.930–3.357］ | 2.494 / 2.492 | 52.31 / 52.09 |
| cloudflare/circl | 1.398［1.371–1.495］ | 1.398［1.378–1.564］ | 1.292 / 1.299 | 46.38 / 47.17 |
| jwt/ruby-jwt | 0.511［0.504–0.540］ | 0.513［0.503–0.532］ | 0.482 / 0.483 | 47.73 / 47.75 |
| paramiko/paramiko | 0.446［0.436–0.473］ | 0.442［0.435–0.468］ | 0.407 / 0.406 | 34.75 / 35.27 |
| panva/jose | 0.722［0.707–0.900］ | 0.718［0.710–0.750］ | 0.678 / 0.675 | 73.66 / 74.61 |
| digitalbazaar/forge | 0.918［0.916–0.949］ | 0.923［0.916–0.947］ | 0.887 / 0.890 | 37.42 / 37.55 |
| Eugeny/russh | 0.846［0.824–0.870］ | 0.826［0.821–0.854］ | 0.797 / 0.791 | 43.38 / 44.22 |
| firebase/php-jwt | 0.222［0.219–0.229］ | 0.220［0.217–0.236］ | 0.203 / 0.200 | 35.73 / 35.81 |
| libssh2/libssh2 | 0.834［0.825–0.850］ | 0.838［0.828–0.905］ | 0.804 / 0.809 | 42.78 / 43.50 |
| randombit/botan | 12.833［12.792–13.525］ | 13.040［12.725–13.166］ | 12.281 / 12.260 | 249.59 / 248.64 |
| Sustainsys/Saml2 | 0.536［0.533–0.585］ | 0.529［0.521–0.629］ | 0.491 / 0.486 | 26.00 / 26.06 |

全サンプルと各資源の範囲は [性能生集計](/Users/waka/.cache/pqc-scan-oss98-20261125/performance-invocations/a8e7e00ca5ea4c55a8be60a38ccf90d4/summary.json) にあります。Botanの別実行を3秒間sampleで観測し、測定110件から除外しました。CPU観測は90.8%で、1ワーカーのTree-sitter解析・ソース事実解析・正規表現と、初期列挙のファイルI/Oがスタックにありました。対応ソースは `tree_sitter_detector.rs` の `detect_source`、`source_facts/c_family.rs` の `inspect`、`walker.rs` の `collect_entries_with_scope` です。短い区間の観測なので全11件の単一律速を確定しません。性能改善を目的とした追加の製品変更は行っていません。[除外プロファイル](/Users/waka/.cache/pqc-scan-oss98-20261125/excluded-profiling/fe7e5ccf2f1946998b4ecf556cfa1d74/result.json) に実行・取得の成功とサンプルのハッシュを保存しました。
