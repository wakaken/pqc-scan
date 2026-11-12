# TASK-043: 誤報・見逃し・注釈不足の原因と修正候補

調査日: 2026-11-11。対象HEAD: `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。
ブランチは `codex/quality-assessment-model`。041・042の未コミット変更を保持しています。
042の受入記録・必須9成果物・128成果物hash・現行runtime/harnessの一致を確認し、042の残作業はないと判断しました。

043では、記録済みの観測50件を20の原因群へ整理し、修正仕様案・影響・正負/境界試験・承認要否を定義しました。
製品コード、規則、評価器、正解、過去の測定は変更していません。修正効果を測った作業ではありません。
各観測の原入力・位置・規則・正解版・hashと修正案の正本は[調査台帳](task043/TRIAGE.json)、コマンド・終了状態・検証は[実行記録](task043/VALIDATION.json)です。

## 調べた範囲と現在の計数

| 証拠 | 調査対象 | 扱い |
|---|---:|---|
| 既存57単位 | FP 3 + FN 5 = 8件 | 元の正解と採点を維持 |
| 追加156単位 | FP 10 + FN 24 = 34件 | 未帰属7件を含む固定注釈への不一致 |
| 既存244パターン対照例 | 望ましい負例と現patternが食い違う8件 | 製品精度の母数へ加えない |

旧・追加データを既存run.pyで各3回再測定しました。042から全split/familyのTP/FP/FN差分はゼロです。
旧データはTP25/FP3/FN5/TN17、追加データはTP36/FP10/FN24/TN55。意味・診断・順序・採点は反復間で安定していました。
この再実行は既知データの再検証です。新しい独立holdoutの評価ではありません。

追加データの未帰属7件は、次のように分かれました。元の7件やFPの計数は削除していません。

- 5件は注釈不足。RS256を含むJWT参照1件と、公開RSA3072証明書の正しい署名属性4件です。証明書の4件は同じ元証明書の形式違いで、独立した4事例ではありません。
- 2件は破損した.p12/.pfxに対する既存互換性のfilename signal。出力には `malformed_structure` とMAC・信頼の未検証が明記されています。解析に成功したPKCS12としての正例とは分ける必要があります。

したがって「製品が10件誤検出した」という結論にはなりません。注釈完全性はPartialのままです。
正解を修正する場合は別版で監査し、042-v1と初回結果を保持します。事後に修正した正解での成績を、検出器の改善や新しい独立評価とは呼びません。

## 見逃しの内訳から分かったこと

Node.jsの第三者由来コードでは、PQC方式名を列挙した有限loopの変数を引数にする9件を見逃しています。
現規則は呼出し内のliteralだけを認め、変数と配列を関連付けません。同じ上流の相関例であり、一般のRecallを推定する材料にはなりません。

依存関係の6件も一様ではありません。Gradleの同一行block・Kotlinの括弧構文・gemspecの任意block変数の3件は、依存一覧にもありません。
一方、CIRCL・CycloneDXのOpenSSL・SPDXのOpenSSLの3件は、依存一覧に存在します。PQC規則のlibrary/ecosystemの列挙に入っていないため、PQC所見がない状態です。
後者を依存抽出parserの欠陥として直すと、原因を取り違えます。

Javaプロパティも表記差があります。規則は行全体のkey=value専用で、元の正解はXML形のentryや.conf内のJava API断片まで含みます。
これらが実際にロードされる設定かはUnknownです。対応表記と「静的な設定意図」の評価範囲を先に定めます。
JSSEのnamedGroups自体は有効なグループの優先順を指定するものですが、このfixtureが設定として受理・適用されたことは検証していません。[Oracle JDK 25 JSSE](https://docs.oracle.com/en/java/javase/25/security/java-secure-socket-extension-jsse-reference-guide.html)

Pythonのimportだけの正例、Rustの未完了path、Rubyの表示文字列も、元のnominal正解と規則が測る性質が完全には一致しません。
規則を広げたり狭めたりする前に、参照在庫・API呼出し・実利用のどれを扱うかをそろえる必要があります。

## パターン一致と製品の所見を分けた追加確認

既存の8つの負例対照を、元のpattern targetのまま適切な拡張子の入力にしてCLIへ渡しました。
対象規則が所見を出したのは6件です。TS_RUBY_RSA_USAGEとTS_TLS_LEGACY_VERSION_LITERALの2件は、そのままでは出力されませんでした。
前者は非PQCのRuby call候補を除く既存処理、後者は識別子内部の部分一致を除く既存処理があるためです。

ただし、同じ内容をヘルプ文字列・バージョン文字列に置いた追加の2例では、両規則とも所見を出しました。
全10入力は結果を知った後の診断用データです。TP/FPの新しい独立精度には加えず、`CONTROL-OBSERVATIONS.json`に全所見と環境・入力hashを残しました。
「compiled patternで一致する」と「特定のソース入力でCLIが出力する」を区別した証拠です。

## 優先順位と修正仕様案

P1は誤った保証・誤報・測定の誤解を先に減らす候補、P2は検出範囲を広げる前の設計を要する候補です。
以下は実装タスクへ切り出すための候補IDであり、新しいタスク番号や修正済みの宣言ではありません。

| ID | 優先度 | 原因・対象 | 修正仕様案と主な影響 |
|---|---|---|---|
| C01 | P1 | Node alias・同名関数・shadowing、5件 | 明示importと局所scopeを関連付ける。import省略のnominal参照を一括で落とさない |
| C02 | P1 | Nodeコードを含む文字列、1件 | 呼出しnodeと実引数へ限定する。方式literal自体を削除しない |
| C03 | P1 | Rustのローカルmod、1件 | 明示的な同名moduleと外部crateを区別する |
| C04 | P1 | Ruby literalとAPI正解の相違、1件 | literal在庫を維持するかAPIへ限定するかを決める |
| C05 | P2 | Python import alias、1件 | 明示aliasだけを有界に追跡し、再代入を区別する |
| C06 | P2 | Python importと生成専用pattern、1件 | importを対象にするか、生成とは別の観測にするかを決める |
| C07 | P2 | 構文エラーを含む呼出し、3件 | 回復木の候補を調べ、残せる参照を限定する。詳細機構はPartial |
| C08 | P2 | Rustの未完了path、1件 | 未完了crate名を扱う保証と、完全symbolの意味を分ける |
| C09 | P2 | Java SLH-DSA参照、1件 | ML専用patternへ加える名称・variant・providerの留保を決める |
| C10 | P2 | 小文字kyber旧称、1件 | 認めるaliasと語境界を定義し、最終標準名と分ける |
| C11 | P1 | XML/Java API形のプロパティ、2件 | 入力表記と静的代入・適用不明を区別する |
| C12 | P2 | Node有限loop、9件 | scope・集合サイズ・再代入・候補数に上限を置く値伝播を設計する |
| C13 | P2 | RSA生成・hash名の署名、2件 | 生成のbranch追加と鍵の局所的な関連付けを別の修正にする |
| C14 | P2 | Gradle/gemspec文法、3件 | DSLを実行せず、採用する有限な宣言形だけを増やす |
| C15 | P2 | 抽出済み依存のPQC規則対象、3件 | library/ecosystem/版/来歴の根拠を定め、enabledと混同しない |
| C16 | P1 | 正しい未注釈所見、5件 | 別版の注釈監査。検出器から所見を消して解決しない |
| C17 | P1 | 破損PKCS12のfilename signal、2件 | 弱いsignalと解析済みの保証を区別する。互換性に注意 |
| C18 | P1 | TLS version prefix、3対照 | 旧版tokenを正確に区切り、modern versionを旧版にしない |
| C19 | P1 | 無効化されたTLS/cipher、4対照 | 追加・除去operandを解釈し、混在する有効な危険設定を保持する |
| C20 | P1 | 汎用Ruby JWTをRSAとするpattern、1対照 | algorithmの証拠と不明なJWT APIを分け、string/callの境界を確かめる |

C07は構文エラーのPartial診断と不検出までを実測しました。回復木がどの候補を落としたかの全dumpは未実施なので、詳細原因をVerifiedとはしていません。
追加のAST診断用実行ファイルの作成を試みましたが、ローカルC linkerがSDK内のarchitecture表記を読めず失敗しました。記録は `AST-INSPECTION.json` に残し、回復木の詳細調査はNot Testedとしています。環境のインストール・設定変更はしていません。
その他のVerifiedは、記載したソース上の仕組みと観測が一致したという意味です。修正の正しさや製品全体の合格ではありません。

## 各候補で実装前に用意する試験

20候補すべてに、台帳の `positiveTestProposal` / `negativeTestProposal` / `boundaryTestProposal` を付けています。
これは今後の修正に必要な受入試験案です。今回追加実装・実行した試験とは区別します。

- 束縛・構文: 有効な直接呼出しと明示aliasを保ち、同名の別関数・shadowing・表示文字列を確定的なcrypto APIにしない。同一行の複数所見・scope・再代入も試す。
- 設定: 旧版や危険operandを選択した正例を保ち、modern version・除外だけの負例を報告しない。正負が混在する一行を丸ごと無視しない。
- 依存: 対応する静的宣言を抽出し、動的補間や同名の無関係packageをPQC有効としない。不正purl・未知版・未解決版と、部分結果・資源上限を保つ。
- 注釈・container: 署名と公開鍵を別属性にし、未注釈の正しい所見、重複、破損、encrypted-only、MAC未検証を区別する。

Gradleの括弧式は公式のKotlin DSL例にもありますが、pqc-scanは現状で全DSLを解析していません。[Gradle宣言構文](https://docs.gradle.org/current/userguide/declaring_dependencies_basics.html)
RubyGemsのadd_dependencyも依存を宣言するAPIであり、block変数名は解析側の固定名と別の問題です。[RubyGems仕様](https://guides.rubygems.org/specification-reference/#add_dependency)
ApacheのSSLProtocolには+/-の構文があり、OpenSSL cipher listの!はoperandの除去を表します。これらを名前の出現だけで有効設定とは扱えません。[Apache mod_ssl](https://httpd.apache.org/docs/2.4/mod/mod_ssl.html#sslprotocol)、[OpenSSL 3.5 cipher list](https://docs.openssl.org/3.5/man1/openssl-ciphers/)

## 最初に実装へ進める候補と承認境界

検出器の修正を始めるなら、最初の候補はC18のTLSバージョン境界です。旧版を検出する目的が明確で、3規則の正負・境界を一つの小さい変更として確認できます。
並行する測定上の優先事項はC16の注釈監査です。042-v1を保持し、修正版を別版・事後調整用として扱うことが前提です。
Nodeの束縛と文字列はC01/C02を別の限定した仕様にします。有限loopの追跡C12へ一度に広げません。

規則の意味・暗号判断・正式対応範囲・出力保証を変える実装には、影響分析と維持者の承認が必要です。
20候補を一括で実装する許可とは解釈していません。個々の変更を1タスク・1PRで選べる根拠までを043の成果とします。
新規タスク登録、検出器・規則の修正、正解版の更新、公開・push・PR・mergeは行っていません。

## 検証と残る限界

今回実行したのはPython44件、規則metadata試験1件（119規則・244対照）、関係するRust detector試験42件、format check、旧・追加コーパス各3回、診断用10入力です。
成功した既存試験は現在の限定された動作を確認するもので、上の修正候補を検証済みにするものではありません。
hosted CI、全workspace再試験、Clippy再実行、性能ベンチマーク、TLS/SSHのネットワーク試験、独立した新holdoutは実施していません。
製品・規則・評価器に差分がないため、今回は関係する試験と証拠の再照合に限定しました。

新たな独立レビューや正解の再承認は行っていません。043は調査・文書だけなので、AGENTS.mdの実装用サブエージェント二段階は適用していません。
Q1の評価完全性はPartial、1.0リリース判定はUnknownです。
