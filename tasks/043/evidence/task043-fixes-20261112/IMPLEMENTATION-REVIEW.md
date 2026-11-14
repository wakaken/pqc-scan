# TASK-043 実装レビュー

判定: **変更要求（P2 4件）**。レビュー担当は `task043-implementation-review-a`。追加担当への委譲は行っていません。

開始点は `ENTRY.json` の HEAD `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf` と `before/source` です。041/042から存在した未コミット変更は今回の実装成果とは扱っていません。レビューした現在ソースの個別SHA-256は `IMPLEMENTATION-REVIEW-SOURCE.json` に保存しています。以下の独立再現は release binary `ee08fe668260e1b5aa4c78c2330fd943df82b34ea1dd19f0a4149a0b56cd1162` で行いました。

## 指摘

### IR-1 [P2] default/rest/typed parameter を未束縛の暗号名にしない

対象: `crates/pqc_scan_detectors/src/source_facts/context.rs:328` と各言語の parameter 登録。

`pattern_names` は通常の識別子だけを拾い、JS `assignment_pattern` / `rest_pattern`、Python `default_parameter` / `typed_parameter` などの束縛を拾いません。その結果、局所引数が明示的に名前を隠しているのに `canonical` の未束縛API名fallbackへ進みます。`function f(generateKeyPairSync = local) { generateKeyPairSync('ml-kem-768'); }` と rest 引数版はいずれも `PQC_NODE_API` を1件出力しました。`def f(mlkem=local): ...generate()` と型付き引数版も `PQC_PYTHON_API` を1件出力しました。いずれも未知の局所値を標準PQC APIの参照と確定するべきではありません。default値そのものを解決せず、引数名を不明な束縛として登録し、関数外の正しいimport/参照は維持してください。

### IR-2 [P2] Python の loop target と拡張代入で由来を失効させる

対象: `crates/pqc_scan_detectors/src/source_facts/python.rs:101`。

束縛収集は通常の `assignment` だけを扱います。`from ... import mlkem; for mlkem in unrelated: mlkem.MLKEM768PrivateKey.generate()` および `mlkem += unrelated` の後の生成参照を、元のimportに結び付けて陽性にします。独立再現では正しいimport陽性に加え、再束縛後のcallもそれぞれ陽性でした。局所loop target・拡張代入など明示的な名前の変更を保守的に不明扱いにし、元のimport参照自体は残す必要があります。未対応の束縛構文が外側の暗号由来を露出させないよう、with/exceptやdestructuringを含む同じ経路も確認してください。

### IR-3 [P2] gemspec の block 変数を block 終了後へ持ち越さない

対象: `crates/pqc_scan_detectors/src/dependency_detector/lexical.rs:74`。

現在は宣言位置以前で最後に見つけたblock変数名だけを使い、対応する `end` や再束縛を確認しません。`Gem::Specification.new do |s|\nend\ns = unrelated\ns.add_dependency 'openssl', '3.3.0'` を `DEP_PQC_CAPABLE_LIBRARY` と依存inventoryへ入れました。これは任意の `s` オブジェクトであり、Gem::Specificationの依存宣言とは確定できません。静的に確定できるblock内のreceiverだけへ範囲を限定し、不明なnested blockや再代入時は確定しないようにしてください。従来の `spec.add_dependency` 断片互換性をどう保持するかも明示してください。

### IR-4 [P2] Rust item の可視性を let の宣言順と混同しない

対象: `crates/pqc_scan_detectors/src/source_facts/context.rs:218`、`source_facts/rust.rs` のitem登録。

Rustのlookupは全束縛へ `declared_at <= reference` を適用します。しかしmodule/struct等のitemは同じscope内で後に書いても名前解決に影響します。`use ml_kem::MlKem768; mod ml_kem { pub struct MlKem768; }` を外部PQC crate importとして陽性にし、`use ml_kem::*; fn f(){ MlKem768::generate(&mut rng); } struct MlKem768;` も後置の局所型を無視して生成callをPQC扱いにしました。itemのscope全域の遮蔽とletの逐次束縛を区別してください。独立再現は `reviewer-rust-repro/` に保存しました。

## 独立再現と確認した検証

`reviewer-repro/reproduction.json` と `reviewer-rust-repro/reproduction.json` に、データとしてのみ扱った入力、実行CLI引数、binary hash、終了値を保存しました。いずれのscanも終了0です。原出力は各 `report.json` です。実行したのはscannerだけで、検体コード、依存DSL、鍵生成は実行していません。これらは具体的な疑義に対する限定的な再現です。workspace testの重複実行は行っていません。

主担当の `green-binding-limit.log` では対象20テストが成功しています。`NATIVE-CHECKS.json` ではfmt/workspace test/Clippy/Python各群/release buildが終了0であり、Nodeの初回失敗は別記されています。`native-node-final.log` の最終31件成功も確認しました。これらの成功は上記反例を否定せず、修正後ソースへの再検証が必要です。

## 確認した設計上の性質と残る受入作業

対象の組込pattern/scope/kindの完全一致で補助処理を限定し、変更したcustom ruleを一般matcherへ戻す仕組みは確認しました。新規依存、公開型/CLI/schema、規則kind/risk/severity/confidenceの変更は確認されません。候補/深さ/束縛/所見上限、Partial/Truncatedの保持、標準名の正確なvariant境界、別名import、有限loop、同期RSA鍵の局所関係、署名と検証の別source startという方針は妥当です。一般的な実行・安全性の証明として扱ってはいけません。

本時点では、両READMEへの新挙動の記載、44件の最終受入台帳、現行コーパスの凍結後測定、最終ソースに対する検証と第二段階の試験レビューは完了扱いにしていません。後続の同一レビュアーによる限定再確認を予定します。

## ソース注釈の独立性

実装・pattern・結果を見る前に、全206入力を30規則それぞれの定義と照合し、`tests/quality/task043-corpus/labels-a.json` を確定しました（SHA-256 `66c0933a6d01fd4b1a9f1bafe9581ae321ab3cb3cd0d012850dde4ffef88ec37`）。missingFacts/未解決定義はありません。246継承問は原ソース確認後に不変の `expanded-v2/truth.json` をoracleとして参照・継承したことを明示しています。新しいblind再注釈とは呼びません。9追加問はソースとプロトコルだけから独立判定しました。他担当の現行注釈は未見です。新しい043コーパスのscanner結果もこのレビューでは参照していません。

## 第1回限定再確認（初回記録は保持）

主担当の修正後ソースと `green-review-bindings.log`、`green-new-path-budgets.log`、`new-path-export-second.log` を確認しました。対象25件とCLI export1件は成功です。既存指摘に対する判定は以下です。

- IR-1: default/rest/typed parameterの名前を未知の局所束縛として登録する修正と、外側の正しい参照を保持する対照例を確認しました。指摘したケースは解消しています。
- IR-4: item/importを宣言順に依存しない束縛として登録し、letの逐次的遮蔽と分ける修正と正負テストを確認しました。指摘したケースは解消しています。
- IR-2: for targetと拡張代入の既知のケースは解消しています。ただし初回指摘で確認を求めた他の明示的binderは未対応です。`with local_context as mlkem: ...generate()`、`except Error as mlkem: ...generate()`、`import unrelated as mlkem` の名前は登録されず、未束縛fallbackからPQC参照になります。一般的な名前解決を実装する必要はありませんが、明示された未知/別moduleの束縛を外側APIとして推測しない処理が必要です。本部分はソース経路からの確認で、修正後binaryによる追加再現は行っていません。
- IR-3: Ruby既存grammarでblockの範囲・receiverと通常代入を確認する変更により、初回のblock終了後と再代入のケースは解消しています。ただし新しいAST上限で中断したとき `parse_gemspec` は空Vecを返し、`parse::heuristic` が中断理由を受け取れないためNotReportedとなります。100,000 node / 4,096 childの上限に達した場合は、契約どおりTruncatedを伝達する必要があります。4,096 childを超えるblockでは10,000 item上限へ到達する前に全候補を落とすため、無断で完全な結果と扱ってはいけません。

両READMEに今回の検出変更と実行/有効化の限界が追記されたことを確認しました。契約とrule metadataもcustom pattern/scope/kindの一般matcher互換性、分類・公開schema維持、局所解析の限界を説明しています。`SELF-SCAN-POLICY-UPDATE.json` の2つの現在hashは実ソースと許可表のhashに一致しており、記載された変更に新しい所見許可はありません。開始前の未コミット許可表拡張を今回の2件のhash更新と混同していません。

CLI exportテストは実際に読まれる一時入力から4つのRSA署名/検証参照を出し、Unicode列、native byte span、すべての生成exportとstdout/stderrのカナリア非露出を確認しています。初回のtarget除外で所見ゼロになった失敗ログは保持され、一時入力へ修正して成功したことも確認しました。最終native検証、44件台帳、試験妥当性レビューと上記IR-2/IR-3の追加修正待ちのため、最終承認はまだ出していません。

## 第2回限定再確認

`green-context-gemspec-status.log` の27件成功と現在ソースを照合しました。Pythonのwith/except aliasと無関係module importを未知の束縛へ登録する修正、gemspecのnode/child/祖先上限から `lexical_truncated` → `parse::heuristic` → `Build(Truncated)` への伝達を確認しました。前回追記のこれらの反例は解消しています。

IR-2の同じscope処理には1点残っています。`Context::new` はPythonにも `for_statement` scopeを作るため、`from ... import mlkem\nfor other in values:\n mlkem = local\nmlkem.MLKEM768PrivateKey.generate()` のloop body内の代入がloop後へ効かず、外側importを再露出させます。Pythonのforはlexical scopeを作りません。現在のfor targetだけをparent scopeへ登録する補正ではbody代入を扱えないので、Pythonのfor自体にscopeを作らない処理と、このloop後の否定例が必要です。新たな一般resolverを求めるものではありません。他の今回の修正について追加の重大懸念は見つかっていません。

## 第3回限定再確認：実装部分を承認

**現在の実装部分の判定は承認です。IR-1〜IR-4の未解消指摘はありません。** この判定が初回および各中間判定を実装部分について更新します。過去の指摘・失敗・独立再現は上記と各原ログに保持しています。

Pythonのforに独立scopeを作らず、targetとbodyの代入を同じ言語scopeへ登録する修正を確認しました。`red-python-loop-scope.log` の元の反例失敗と、修正後 `green-python-loop-scope.log` の28件成功を照合し、loop後の生成callは報告せず、明示importの1参照を維持するassertionを確認しました。他の3指摘の修正と27件時点の再確認も有効です。

Gemspecで来歴を持たない裸の `spec.add_dependency` 断片を依存宣言として推測しない互換性上の限界、およびAST上限による中断をTruncatedへ伝達することが契約へ明記されていることを確認しました。一般的なRuby DSL解析や実行を保証する記述にはなっていません。

承認対象のソースhashは `IMPLEMENTATION-REVIEW-SOURCE-R3.json` に固定しました。実装レビューとして追加の必須修正は見つかっていません。ただしこの承認はTASK-043完了の認定ではありません。最終ソースへのnative検証、44件の受入台帳と凍結後測定、第二段階の試験妥当性レビューは後続証拠を確認してから全体判定します。

## 出典metadata修正版043-v2の注釈再結合

主担当が発見した末尾LF保存差について、別版 `tests/quality/task043-corpus-v2` と元043-v1を比較しました。206ファイルのmanifest、255質問、全source byte SHA、protocol本文は同一です。corpusの差分はversionとrevisionPurposeのみ、provenanceの差分は3出典の元応答hash/保存コピーhash・Raw URL・変換説明と補正履歴のみでした。全license bindingも維持されています。

3保存コピーのhash、および各コピーの末尾LFを1 byteだけ除いたhashが新metadataと一致しました。元応答相当長/コピー長はauth0 helper 3611/3612、README 17709/17710、BC 229855/229856 bytesです。今回新しい独立HTTP取得は行わず、保存済みコピーと補正値の整合を検証した範囲です。

自身の元255行と独立性履歴をそのまま維持し、新版の `labels-a.json` に時刻/corpus bindingを更新して保存しました。新版SHA-256は `8b146523c2a14a1562e4068383dd092fb42d7a2b326546ea5c95e1b55111ee72`、corpus SHA-256は `f8734fbadec003d6c74f5a0920422f675c0419fea6f6b795a857b7fb3c17cf07` です。元の自身labels SHA `66c0933a6d01fd4b1a9f1bafe9581ae321ab3cb3cd0d012850dde4ffef88ec37` は変わっていません。

これはmetadata-only replayであり、新しいblind注釈・独立試行ではありません。実装、歴史的エラー情報、限定再現結果を既に見た履歴を明示しています。新版測定はこの時点では未見で、他レビュアーの現在labelsも参照していません。新版で再凍結した同じ入力に対するbefore/after測定と最終受入証拠は後続確認です。

## 既存harnessのmetadataReplay対応レビュー

追加の `taxonomy.validate_metadata_replay` とreportの説明、および `test_metadata_replay.py` を確認しました。判定は**承認**です。対象3ファイルのhashは `IMPLEMENTATION-REVIEW-HARNESS-R4.json` に固定しました。

元のblind版をhashで束縛し、その原版自身を通常の `load_truth` で再検証しています。corpusVersion/revisionPurposeの2つを除く全corpus内容、入力、質問、規則集合、protocol本文、最終labels/adjudication、同一reviewer自身の全labelsと原label record hashの完全一致を要求しています。replay chainを拒否し、再freeze時刻が原freezeより前になることも拒否します。通常の非replay版では `scannerResultsSeen=false` の条件を維持しています。したがって結果接触済みであることを正直に記載した同じ注釈のmetadata再結合だけを許可し、変更した入力や正解を新しいblind評価として通すものではありません。reportも「Metadata-only replay; not a new independent trial」と明記します。scorerの一致条件や検出結果は変更していません。

`green-metadata-replay-second.log` の2件成功、`quality-replay-final.log` の52件成功を確認しました。正直な結果接触記録の受入と、入力hash・質問位置・全員で変更したlabel・protocol本文を改変した対照例の拒否を含みます。主担当の結果を確認し、full suiteの重複実行はしていません。

## 旧57単位のraw FP4/FN2の原ソース再確認

`audited-after-original-report/quality.json` の6記録と、元の不変corpus/原ソース/質問区間を照合しました。これは新版測定を見ずに再注釈したという主張ではなく、旧比較のエラー帰属をソースから確認したpost-observation reviewです。

| raw観測 | 原ソースからの確認 | 結論 |
|---|---|---|
| `approval_backend.py` のimport FPとu051 FN | line 3の`from cryptography...mldsa import (`は明示importで、未使用でも名目的参照。旧質問区間はcolumn 6〜53、新所見はstatement全体のcolumn 1から始まる | 1つの正しい参照の位置境界の差によるFP/FN対。方式の誤判定や参照の欠落ではない |
| `archive-policy.xml` のFPとu054 FN | line 6のentryがsecp384r1,secp521r1だけを明示。原質問はindent後のcolumn 3開始、新regexは行頭空白を含むcolumn 1開始 | 正しい明示設定の位置境界の差によるFP/FN対。ロード/実行を保証するものではない |
| `tuning/variants/alias.py` line 1の未帰属FP | cryptography.mlkemからMLKEM768PrivateKeyをKeyとして明示import。元unit u020はline 2のKey.generateだけを注釈 | 新しく拾った正しい独立import参照が元データで未注釈。無関係moduleや同名の誤認ではない |
| Ruby u022 FP | `puts 'ML-DSA-65'`はAPI実行ではなく標準名literalの表示。Ruby規則の既存purposeはalgorithm literalであり、このliteralは以前から検出対象。元57単位の質問はAPI useとしてnegative | 旧評価propertyと既存規則propertyの不一致。PQC実行成功として表示するべきではないが、新しく発生した製品欠陥ではない |

これら6記録から追加の必須製品修正は確認されません。参照開始位置・snippet・IDの変化は今回の検出修正として契約へ記載済みです。ただし旧57単位に対するraw `accuracyComparison=Failed` とFP4/FN2は、その固定質問区間と評価定義に対するスコアとして保持します。新255単位の成績で旧スコアをPassedへ上書きしたり、旧truthを都合よく変更したりしてはいけません。現在版における独立した完備監査と原受入44件の解消は別の証拠として扱います。

## 最終combinedレビュー：承認

**最終の実装・文書・受入台帳・測定証拠を承認します。実装レビュー側の未解消必須指摘はありません。** 対象は基点HEAD `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf` から保存された開始worktreeに対する043の差分です。最終バイナリSHA-256は `cfb12f4b8751915b9c2a5d23be5e83e872358704e20e0b19d3273236aa4c288f`、runtime digestは `5a337275b498233c1d45c67d83e7c3536bb692226c120fb835c6d8212e26edc6`、harness digestは `41474d4fba6ff7aea6371d77e8047ca78fcd679df305c2faf1511bed11407654` です。レビュー対象成果物のhashと確認項目は `IMPLEMENTATION-REVIEW-FINAL.json` に固定しました。

台帳の44件の集合を042の元supported-scope errorDetailsと直接照合し、完全一致を確認しました。各入力hashも原ソースと一致し、修正後の原summaryの実dispositionは元FP4件すべてTN、元FN40件すべてTPです。元truthを変更した行や対象を外した行はありません。旧証拠343ファイルのhashを実ファイルと照合し、変更がないことも確認しました。

同じ043-v2入力・質問・truth・harnessによるbefore/afterを照合しました。beforeのbinary/runtime digestは042凍結baselineと一致しています。afterの実行ファイルと132 runtime bindings、329 harness bindingsは現在ファイルと全件一致し、両digestの再計算も一致します。製品差分表の14ファイルの現在hashも一致しました。原版と現行ルールを混ぜた比較ではありません。

255単位の結果はbefore TP61/FP4/FN42/TN90、after TP103/FP0/FN0/TN94です。適格197、Unsupported53、Ambiguous5を保持し、未帰属FP・重複陽性予測・FNはありません。7問の新第三者由来部分はTP3/TN3/Ambiguous1で、継承調整用部分と分離されています。各3反復の意味・診断・生順序・成績は安定しています。metadata-only replayを新しい独立試行とはしていません。旧042-v2の未注釈verify2件、旧57単位のraw Failed/FP4/FN2も結果文書に保持され、先に行った独立のソース帰属と整合しています。

最終workspaceログのRust成功件数を再集計して385、品質Python54、配布10、Node訂正後31を確認しました。fmt/Clippy/releaseの終了0と、protocol 5成功・6skipも検証台帳と一致しています。Node初回のreport.py source hash不一致による終了1、訂正後の終了0は両方保持されています。新しい所見許可ではなく、既存の有限な許可表のsource hash3件のみの更新です。自己scanは1595件中1241件が既存expected、354件は残っており、ゼロ問題とは記載されていません。

metadataReplayの最終修正（chainのkey存在による拒否）と、個別reviewer reason改変・原本hash不一致・chain・結果接触済み原版の拒否対照、JSON/Markdown両方の再評価表示も確認しました。元blind証拠の不変性条件を緩める変更はありません。

日英README、規則参照、source-bound metadata、検出契約、結果文書、ロードマップとtask YAMLの受入範囲は整合しています。原FP4/FN40を別タスクへ送る扱いはありません。証拠index29件のhash/byte数を全件照合し、raw archive（520項目、原summary6件・log184件）とsource overlays（258項目）の存在も確認しました。未実施の性能・hosted CI・任意の言語/暗号形式・全OS・実通信は今回の承認範囲に広げていません。公開Schema、分類、新規依存、暗号実装の変更はありません。

この最終承認は第一段階の実装レビュアーとしての判定です。taskをdoneへ変更するEvaluatorは、別identityによる第二段階の最終試験妥当性レビューも承認済みであることを確認してください。私の確認時点の `TEST-REVIEW.md` はTR-3/TR-4の最終追記待ちであり、その未確定記録だけをもって二段階完了とはしていません。試験の全件重複実行や追加review identityの起動は行っていません。

## 二段階承認確認後の全体判定

**元の受入条件全体について承認します。IR-1〜IR-4、TR-1〜TR-4に未解消事項はありません。** 第二段階レビュアーの `TEST-REVIEW.md` 最終承認と `TEST-REVIEW-FINAL-SOURCE.json` / `TEST-REVIEW-FINAL-VERIFICATION.json` を確認しました。単なる成功ログの承認ではなく、原rawへの既存scorer再適用、原44件の一対一照合、現在source/binary、原343ファイルの不変性、正負・境界・上限・位置・秘匿のassertionを照合して承認していることを確認しました。これにより直前の「第二段階最終追記待ち」は解消しています。

追加されたroot Cargo/toolchain、規則metadata、自己scan許可表を含む最終source binding500件も現在ファイルと一致しました。最後のtask YAML成果物列挙と日本語推敲は今回の追加scopeに一致しており、製品コード/harness/入力/正解は最終測定以来変わっていません。taskは調査から実装へ拡張された元の44観測を全件満たしており、別タスクへ延期して完了する扱いはありません。今回の承認は043の限定された受入範囲で、リリースや全品質軸の承認ではありません。

承認の照合対象は `IMPLEMENTATION-REVIEW-COMBINED-APPROVAL.json` に保存しました。最後の成果物組立として、主担当は両review記録をdurable側へコピーした後に `EVIDENCE-INDEX.json` を再生成して全件照合してください。確認時のindexはFINAL-SOURCE.json/VALIDATION.jsonの最後のmetadata追記前の2hashを持っており、主担当へ通知済みです。これは新しい実装・試験の指摘ではなく、Evaluatorがdoneを記録する前に必要な最終packetの整合更新です。私自身はtask statusを変更していません。
