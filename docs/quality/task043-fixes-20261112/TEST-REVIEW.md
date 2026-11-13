# TASK-043 試験妥当性レビュー

担当: `task043-fix-test-review`。最終判定: **承認**（最終確認節を参照）。TR-1〜TR-4はすべて解消しました。以下の初回・中間Partial判定と失敗履歴は保存し、最終節が今回の試験妥当性判定を更新します。製品コードの編集、検体実行/import/鍵生成、委譲、全試験の重複実行は行っていません。

## 対象と独立性

`docs/quality/TASK-043-FIXES-PLAN.md` の元のFP4/FN40全44観測の修正を対象とし、041/042からの既存dirty差分は043実装として評価していません。基準HEADは `ENTRY.json` の `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。HEAD単独では作業版を識別できないため、最終ソースhashと実行バイナリ/rulesの対応を最終判定時に確認します。

実装・規則・現行ラベル・出力を見る前に全206入力を読み、全30指名特性について追加の正当な参照の欠落と定義を監査しました。`tests/quality/task043-corpus/labels-b.json`（SHA-256 `80036aaf4ed95b8fd02cd5ee5b9da547a1faa09cc7240abaebe624079091ef38`）を保存後に実装レビューへ移りました。全入力確認後に原 `expanded-v2/truth.json` を継承oracleとして参照したことを明示し、246継承問を新しいblind再注釈とは呼んでいません。9追加問は現行結果なしで判定しました。公開証明書は既知hash/OpenSSL読取専用属性表示、P12はOIDと公開非対応契約・未暗号化コンテナ中の公開DER一致で確認し、復号していません。

## 確認した試験の意味

`task043_detection_fixes.rs` は既定規則を直接使い、Node alias/名前隠蔽/文字列、有限loop、RSA鍵来歴、Python import、Rust生成・両receiver、Java/Python/Rust破損参照、SSHコメント、Java設定、SLH/旧称境界、Gradle/gemspec、ecosystemを正負対照で確認します。malformedのPartial、32/33所見、33要素配列、束縛overflowのTruncated、custom patternとscopeのfallbackも検証しています。検体は文字列としてパースされます。

`red-simple.log` は初期6試験の実際の期待値不一致、`red-source-assertions.log` は5試験の実際の期待値不一致を示します。`red-source.log` はprivate field参照によるコンパイル失敗であり、動作のred証拠には数えません。修正後のassertion red記録が別途あるため、この誤った初回試験は隠されていません。後続のboundary/recovery/parameter/scope/bindingのredも保存されています。

既存 `bounded_outcomes.rs`、`ast_occurrences.rs` は汎用matcher/上限/位置/重複、既存029のCLI/core/report試験は秘匿・schema・native連結と両言語同一性を確認します。ただしcustom TEST規則の試験は新しい組込source_facts経路を起動せず、新経路の十分性の代用にはなりません。

## 指摘と必要な解消証拠

- **TR-1 [P2] 新しいsource_facts上限経路の対照不足。** 対象は `crates/pqc_scan_detectors/src/source_facts/context.rs`、`javascript.rs` と `tests/task043_detection_fixes.rs`。alias深さ8の境界/超過を既定PQC/RSA規則で確認し、超過時はTruncatedかつ未解決値を古い暗号由来へfallbackしないことを検証してください。AST深さまたは候補走査の代表的上限も新経路を実際に起動して確認してください。既存の汎用上限試験だけでは新しい枝は検証されません。
- **TR-2 [P2] 新経路の位置・observation span・秘匿・Regex capの裏付け不足。** 対象は `crates/pqc_scan_detectors/src/regex_detector.rs`、`tree_sitter_detector.rs`。RSA4参照と同一行複数callの件数だけでは、1件の欠落と1重複が相殺します。factory/useの別start、Unicode列、native observation spanを意味のあるassertionで確認してください。`API_NODE_CRYPTO_SIGN_RSA`は新しいRegex専用枝なので32/33境界も必要です。近傍/引数に合成credentialを置き、実際にこの枝が陽性であることを先に確認し、最終export（全format/native/diagnostics/stdout/stderr）でcanary全体と中間断片が残らない対照が必要です。既存029CLIはcustom BODY_CONTROL/TECH_CONTROLのみでこの枝を起動しません。
- **TR-3 [P2] 最終ソースに対する実行・44件の受入証拠が未確定。** `NATIVE-CHECKS.json`（01:37〜01:40）のfmt/workspace/Clippy/Python3群/release終了0、およびNode初回終了1→`NATIVE-NODE-FINAL.json`終了0（31件）を確認しました。しかしIR-1〜4の修正中であり、これらは最終版の結果とは扱えません。最終hash、必要再試験、旧正解/適用範囲を変えない44件の一対一台帳、凍結後の新規データ測定、重複/余分なFP/FN、coverage/canary/repeatabilityを確認してから最終判定します。

IR-1〜4で示されたdefault/rest/typed引数、Python loop/拡張代入、gemspec block寿命、Rust後置itemは既存成功ログに含まれない実反例です。修正後は各指摘に対応する正負対照と、元の外側import/通常利用が保たれるassertionが必要です。同一実装レビュアーの確認結果も参照し、別の担当を増やしません。

## 現時点の測定解釈

自己scanは `self-scan-audit.json` に raw1441、有限expected1241、retained200と記録されています。ゼロ所見・全件正当・全品質合格という意味にはなりません。試験fixtureの有限例外は原所見を保持し、既存の製品/依存上の残件とは分離して説明する必要があります。最終変更後のsource/result/count bindingを追加確認します。

新しいsource-only注釈は限定されたコーパス内の監査証拠です。入力数×規則数というmanifest寸法やゼロ予測を根拠に完全性・母集団精度を認定していません。旧44件修正、旧データ回帰、新たなholdoutでの観測は別々に報告される必要があります。


## 第2回確認（最終測定前）

主担当の `green-python-loop-scope.log`（28試験、終了0）と `new-path-export-second.log`（CLI1試験、終了0）を対応ソースと照合しました。alias8/9、70段AST、4097呼出し、旧束縛overflow、新Regex32/33を既定規則で起動し、成功/Truncatedと誤ったfallbackの不在を確認するassertionがあり、**TR-1は解消**です。IR-1〜4に対応するdefault/rest/typed・with/except/unrelated import・Python loop後・gemspec寿命/再束縛・Rust item/let順の対照も追加されています。

TR-2は新RegexのUnicode列と32/33、4つのfactory/use startとnative byte_startについて解消しました。CLI試験は既定規則で実際に4参照を検出し、引数/近傍の合成credentialを含む全生成exportを読むため、新経路の秘匿を直接試験しています。ただしbyte_endが任意のstart後の文字境界でも通るassertionと、prefix/suffixだけのcanary否定は不十分です。4つの期待引数末尾との一致、canary中間固有断片の否定、期待する生成ファイル名集合を追加するよう依頼しました。最終版・最終結果で確認します。

### 出典metadataの訂正

主担当が、接続ツールの応答保存時に末尾LFが1個追加され、その保存copyのhashをupstreamSha256としていた不備を発見しました。元043-v1と履歴を残し、別 `tests/quality/task043-corpus-v2` へ訂正しています。私自身がv1/v2の差分を確認し、files/units/families・全206source hash・protocol bytes・license bytesが同一、差分はcorpusVersion/revisionPurposeと3件のupstream/copy/newline metadata・訂正履歴に限ることを確認しました。保存copyから末尾LFを1個除いたhashが新upstreamSha256と一致します。ただし私自身によるremote再取得はしておらず、ローカル再現だけを独立したremote出典検証とは呼びません。

自身のv2 `labels-b.json`（SHA-256 `d47d23d9c42f4f5a24f6fea48610f5232596cf9e28783a3d296c85d383761527`）は255行のlabel/reasonを原記録と同値のまま、新corpusへのmetadata-only replayとして保存しました。元のblind attestationへのhash参照を残し、凍結後に実装・red/green実所見・native要約・自己scan結果を閲覧した履歴を明記しています。scannerResultsSeenをtrueにしており、新しいblind再注釈や新しい独立試行を装っていません。他担当の現行labelsファイルは未見です。


## 第3回確認（metadata replay検証追加後）

`export-adequacy-final.log` 終了0を現行CLI試験と照合しました。4参照それぞれのfirstarg末尾byte_end完全一致、8個の期待export名集合、stdout/stderr/全export内の固有canary中間断片否定が加わり、**TR-2は解消**です。

出典訂正を扱うtaxonomyの新 `validate_metadata_replay` を確認しました。元版truth/corpusのhashをbindし、元版自体のload_truthを実行し、chainを拒否、version/説明以外のmanifest・protocol・final truth/adjudication・両reviewer全row同値を要求しています。結果接触をtrueとしたB記録を受け入れつつ、元blind証拠を保持する設計です。新しい独立試行とは扱えません。

`green-metadata-replay-second.log` 2試験終了0と `quality-replay-final.log` 品質Python52試験終了0を確認しました。正直な結果接触を持つreplay成功と、inputs/questions/final labels/protocolを書換えhashを再bindしても拒否する対照があります。ただしfinal truthを変えない片方のreviewer reason単独改変、元原本hash不一致、replay chain拒否は別の重要枝であり、局所対照を追加依頼しました。現行report.pyではJSON/comparisonにmetadata-only replayを表示しますが、Markdownにも明記する必要があります。これを**TR-4 [P2] replayの不変性・表示の追加対照**として追跡します。

`red-metadata-replay.log` と初回 `green-metadata-replay.log` はfixture一時パスのsymlink解決不一致で失敗しており、metadata replay挙動のredではありません。失敗履歴を保持し、動作redを確認したと遡及して記述しないよう主担当へ伝えました。新harness差分も同一の実装レビュアーAによる最終確認が必要です。


## 最終確認：試験妥当性を承認

**TR-1〜TR-4に未解消事項はありません。元のFP4件・FN40件を043内で直すという受入範囲について、試験と測定の十分性を承認します。** これは全品質軸・リリース・母集団精度の認定ではありません。レビュー担当は同一identityを継続し、追加担当やfull suiteの重複実行はしていません。

### 証拠と実際の照合

`docs/quality/task043-fixes-20261112/` の全件台帳・最終品質報告・検証台帳・ソース照合・保存原本を確認しました。単に台帳のPassedを信用せず、既存scorerを保存raw reportへ再適用して元の44 idを照合し、修正前FP4/FN40、修正後TN4/TP40を確認しました。各入力hash、44件の一意性、全件の正負/境界試験への対応も確認しました。元343ファイルのhash不変性、成果物indexの全hash/bytes、製品差分の全after hash、最終source bindingを現在ファイルと再照合し、全て一致しました。検証スクリプトの結果は `TEST-REVIEW-FINAL-VERIFICATION.json` に記録しました。

対象HEADは `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf` のdirty作業版です。修正後release binary SHA-256は `cfb12f4b8751915b9c2a5d23be5e83e872358704e20e0b19d3273236aa4c288f`、runtime digestは `5a337275b498233c1d45c67d83e7c3536bb692226c120fb835c6d8212e26edc6`、評価harness digestは `41474d4fba6ff7aea6371d77e8047ca78fcd679df305c2faf1511bed11407654` です。最終測定・現在binary・runtime132ファイル・harness329ファイルが一致します。変更前binary `ce604d8348a474125b9cd12747554136de7614c13ca83ab22ea56e9c9581930c` とruntimeは元042 baselineと一致し、同じ評価harness/corpus/truthで測定されています。製品ソースと規則だけが意図した比較差分です。

043-v2は修正前TP61/FP4/FN42/TN90から、修正後TP103/FP0/FN0/TN94になりました。判定対象197、Unsupported53、Ambiguous5を混ぜず保存しています。正例重複0・帰属不能予測0であり、追加された2つの正当なRSA検証参照を抑制して旧intervalへ合わせていません。before/after各3反復の終了0と意味・診断・生順序・成績の安定を確認しました。206ファイルは全てread_okですが、解析coverageはpartial32/unknown21/未列挙母数unknownを保持しており、全ファイル完全解析と取り違えていません。

新たな第三者由来7質問はTP3/TN3/Ambiguous1という極小で相関のある観測です。出典訂正はmetadata-only replayであり、新しい独立試行ではありません。元のsource-only凍結を保持し、現在の結果接触を偽っていません。最終JSON/Markdownの両方にその説明が実際に表示されています。

### TR-4と最終チェック

`test_metadata_replay.py` は、final truthを変えない個別review reasonの改変、元原本hash不一致、replay chain、元blind recordの結果接触true化も拒否します。各hashを再bindした対照であり、単なるstale-hash検知だけではありません。`test_report.py` はmachine/human両出力の再評価表示をassertします。これらの追加対照と実成果物を照合してTR-4を解消しました。最初のpath解決失敗はTDDの動作redではないと結果文書に明記されています。後から旧guardを再構成した対照も、遡及TDDとは区別されています。

最終 `VALIDATION.json` と対応ログで、`cargo fmt --all -- --check`、`cargo test --workspace --offline --locked`、`cargo clippy --workspace --all-targets --offline --locked -- -D warnings`、`cargo build --workspace --release --offline --locked` の終了0を確認しました。workspace385件に新28detector試験・新1CLI試験と既存schema/ID/秘匿/互換性試験が含まれます。品質Python54件・配布Python10件・Node31件も成功です。protocolは5成功/6skipをそのまま記録し、実行していないoptional runtime/containerを合格扱いにしていません。Node初回終了1はreport.pyの有限な許可source hashのstaleで、source hashのみ更新した後の31件終了0と原失敗の両方を保存しています。実装AのIR-1〜4および追加harnessの承認記録も確認しました。

旧57単位のraw比較Failedは保持されています。実装Aが原ソースへ帰属させた位置interval差・未注釈import・既存Ruby literal特性の不一致を参照し、旧成績をPassedへ書換えていないことを確認しました。旧042のraw FP2も未注釈の正当な検証参照として残っています。どちらも今回の44件を正解変更/除外/延期で消したものではありません。

自己scanの最終値はraw1595/有限expected1241/retained354です。ゼロ問題とは扱いません。今回は既存許可source hash3件だけを更新し、一般的な抑制拡張や問題の消去をしていません。性能benchmarks・hosted CI・全OS・実通信・全Q1〜Q6は今回約束された精度修正の受入外であり、その未実行を最終品質保証へ拡張していません。

以上の限定された受入範囲で、追加の必須試験はありません。最終レビュー時刻・照合対象hashは `TEST-REVIEW-FINAL-SOURCE.json` に保存しました。
