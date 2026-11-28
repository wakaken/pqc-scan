# 選定35件の暗号OSS評価（2026-11-27）

既存ディスク内で主評価27件と比較8件の評価を進め、確認済み修正の回帰を確認するための報告です。35件すべてを実行し、主評価27件は成功、比較用は7件成功・Node.jsが600秒でTimedOutとなりました。Node.jsの確認済みEVP文言1箇所は未再評価で、評価全体の完了フラグはfalseを維持します。

以前の28件の成功は今回の件数へ加算していません。今回の小入力・代表実行の再利用は、同一最終ソース・同一試行への参照だけです。

対象は既存98件の部分集合です。新規取得や二重計上は行っていません。主対象は9言語グループ各3件で、JS/TSとC/C++はそれぞれ一つのグループです。Goはage、JS/TSはnoble-post-quantum、RustはRustCrypto/KEMsを選定しました。残る主対象63件は将来対象として台帳・ソース・証跡を保持し、今回の完了条件から除外しています。

## 実行条件と状態の意味

固定SHA・ファイルハッシュ・既存の入力範囲を維持し、1スレッド、1試行、600秒、ファイル上限2MiB、開始時空き4GiB、必須11形式で実行しました。collectPerformance=falseを使用し、OSS評価のCPU・RSS・経過時間の性能値、負荷・swap監視は再開していません。外部OSSのコード・ビルド・依存解決は実行していません。Node.jsとDiscourseの利用者承認済み2ファイル除外を維持し、保護機能は変更していません。

Succeededは解析処理と必須出力が成立した状態です。Complete/Partialは入力制限と解析範囲を含む別の検査状態であり、暗号的な安全性を意味しません。検出数はテスト・コメント・重複規則を含む生のfinding数です。TimedOutの出力・検出数・精度はnullです。

## 主評価27件

| 対象 | SHA (12) | Execution | Inspection | Findings | Output audit |
| --- | --- | --- | --- | --- | --- |
| bcgit/bc-java | bb81bfa75c9f | Succeeded | Partial | 10,494 | PASS |
| Keyfactor/ejbca-ce | a46c89e0c6b6 | Succeeded | Partial | 617 | PASS |
| apache/mina-sshd | f885e0fc9f68 | Succeeded | Partial | 447 | PASS |
| cloudflare/circl | 31be93db4772 | Succeeded | Partial | 269 | PASS |
| smallstep/certificates | 0d8a80647845 | Succeeded | Complete | 1,472 | PASS |
| FiloSottile/age | b74dce4cdbe3 | Succeeded | Complete | 73 | PASS |
| ruby/openssl | b5774711c1f0 | Succeeded | Partial | 238 | PASS |
| net-ssh/net-ssh | 348b78049bfe | Succeeded | Partial | 688 | PASS |
| jwt/ruby-jwt | c22701ef8e0e | Succeeded | Partial | 537 | PASS |
| pyca/cryptography | 0b3a52a7cb58 | Succeeded | Partial | 2,979 | PASS |
| paramiko/paramiko | 0e9ccb91e0f9 | Succeeded | Complete | 263 | PASS |
| certbot/certbot | 9d5c9d44e57d | Succeeded | Partial | 292 | PASS |
| panva/jose | d1e200f22d2b | Succeeded | Partial | 973 | PASS |
| openpgpjs/openpgpjs | ea041b3c70eb | Succeeded | Partial | 37 | PASS |
| paulmillr/noble-post-quantum | 48e493397e69 | Succeeded | Partial | 288 | PASS |
| rustls/rustls | 796b8d3b0848 | Succeeded | Partial | 792 | PASS |
| Eugeny/russh | 56a999871aa1 | Succeeded | Complete | 93 | PASS |
| RustCrypto/KEMs | b26db85a9b44 | Succeeded | Partial | 167 | PASS |
| phpseclib/phpseclib | 808f39f816c4 | Succeeded | Partial | 195 | PASS |
| firebase/php-jwt | 99efeb17d6de | Succeeded | Partial | 289 | PASS |
| lcobucci/jwt | 8bd5e392228c | Succeeded | Partial | 78 | PASS |
| openssl/openssl | 18d0baa70de7 | Succeeded | Partial | 6,031 | PASS |
| google/boringssl | 5ec099107553 | Succeeded | Partial | 14,380 | PASS |
| wolfSSL/wolfssl | f4f52d975ff1 | Succeeded | Partial | 3,304 | PASS |
| bcgit/bc-csharp | 4814a10bc95d | Succeeded | Partial | 5,788 | PASS |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | 0d96aca5a57a | Succeeded | Partial | 449 | PASS |
| openiddict/openiddict-core | 68269d28b439 | Succeeded | Partial | 109 | PASS |

報告書を生成した27件の検出合計は51,342件です。主評価と比較の指標は別に集計しています。

## 比較・回帰用8件

| 対象 | SHA (12) | Execution | Inspection | Findings | Output audit |
| --- | --- | --- | --- | --- | --- |
| keycloak/keycloak | 18559075f50c | Succeeded | Partial | 4,072 | PASS |
| apache/airflow | c4e2d0054bd0 | Succeeded | Partial | 56 | PASS |
| kubernetes/kubernetes | a0594a28a435 | Succeeded | Partial | 1,989 | PASS |
| nodejs/node | 330e3ee67e84 | TimedOut | Unavailable | null | Unavailable |
| n8n-io/n8n | 9a84403d2fae | Succeeded | Complete | 282 | PASS |
| discourse/discourse | 6dfa59ea612b | Succeeded | Partial | 72 | PASS |
| rails/rails | ee9b18fdb0fc | Succeeded | Partial | 8 | PASS |
| vectordotdev/vector | 7e97ead1775c | Succeeded | Partial | 185 | PASS |

報告書を生成した7件の検出合計は6,664件です。主評価と比較の指標は別に集計しています。

## 言語グループ・検出箇所の言語

| 主評価グループ | Reports | Findings |
| --- | --- | --- |
| C / C++ | 3 | 23,715 |
| C# | 3 | 6,346 |
| Go | 3 | 1,814 |
| Java | 3 | 11,558 |
| JavaScript / TypeScript | 3 | 1,298 |
| PHP | 3 | 562 |
| Python | 3 | 3,534 |
| Ruby | 3 | 1,463 |
| Rust | 3 | 1,052 |

| 検出箇所の分類 | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| C | 4,416 | 0 |
| C++ | 981 | 0 |
| C# | 6,051 | 0 |
| Go | 2,260 | 1,854 |
| Java | 6,847 | 3,292 |
| JavaScript | 68 | 0 |
| PHP | 491 | 0 |
| Python | 1,081 | 56 |
| Ruby | 1,329 | 73 |
| Rust | 1,110 | 8 |
| TypeScript | 1,228 | 283 |
| C/C++ header unresolved | 534 | 0 |
| Manifest/configuration | 9,254 | 739 |
| Other/unattributed | 15,692 | 359 |

上表は既存集計器による検出箇所の拡張子分類です。リポジトリの宣言言語で全検出を振り分けていません。.hはC/C++未確定、設定ファイルとその他も別枠です。比較欄の0は成功7件で観測した0であり、Node.jsの検出なしを示しません。

## 暗号技術別の観測

| 出力のcategory | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| CryptoAPI | 12,473 | 1,784 |
| JWT | 10,936 | 3,900 |
| KeyMaterial | 1,501 | 167 |
| Middleware | 34 | 0 |
| PKI | 13,459 | 323 |
| PQC | 3,727 | 27 |
| SSH | 1,351 | 57 |
| TLS | 7,861 | 406 |

CryptoAPIなどの古典暗号API、TLS、SSH、PKI、JWT、PQCの静的な参照を観測しています。分類名は元の出力を保持しており、実運用・脆弱性・移行完了の判定ではありません。規則別件数は集計JSONに保持しています。

Nativeでは元の129事例から選定35件に該当する41事例だけを抽出し、正解を開く前に実出力からの抽出を封印しました。元のソース一次根拠と承認済み正解、既存のjudgment.compareをそのまま使用しています。

| Population | Cases | Selectors | Verified | No exact location | Not assessed |
| --- | --- | --- | --- | --- | --- |
| Primary | 38 | 11 | 2 | 9 | 27 |
| Comparison | 3 | 0 | 0 | 0 | 3 |

主評価の比較可能な属性11件のうち一致は2件で、9件はno_exact_locationです。比較用は採点対象0件で一致率nullです。これは有限のフィールド表現の照合で、分類精度ではありません。結合できない9件をFNへ変換していません。主評価のcompletenessはpartial10件・not_exhaustive1件を保っています。旧改修後出力を同じ41事例へ絞った結果と、今回の有限フィールド・結合状態は同じでした。時刻・実行条件を含む全体の因果比較はUnknownです。

| Native algorithm family | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| ECDH | 5 | 0 |
| ECDSA | 11 | 6 |
| Ed25519 | 6 | 2 |
| ML-DSA | 41 | 3 |
| ML-KEM | 37 | 1 |
| RSA | 23 | 5 |
| Unknown | 300 | 41 |
| X25519 | 3 | 0 |

この表はNativeのalgorithm資産数です。規則検出数や実利用数とは単位が異なります。既存の有限な名称表で認識できたものだけを分類し、それ以外はUnknownを維持しています。Node.jsの資産数はnullです。

採点前の参照照合は、元の実装が省略可能なbytes欄まで辞書全体で比較したため失敗しました。元実装・失敗ログを保持し、宣言された参照項目を検証する3条件だけを変更したv2で採点しました。元の採点器・正解・抽出結果は変更していません。旧assessment helperの参照は歴史メタデータであり、現在の同じパスを旧ハッシュの読取可能入力とは扱いません。現行helperは前回の最終Ruby epochへ結び付けています。独立レビューは索引に記録します。

## 精度を算出できる範囲

27件は暗号評価を重視した目的抽出で、無作為標本でも未使用holdoutでもありません。過去出力への接触と調整・裁定の記録を保持しました。同系統AIの別担当による正解作成を、独立した正解の保証とは扱っていません。旧392箇所・324主張を新しい分母へ流用していません。

選定範囲には既存の標本行140行があり、成功34件に属する136行を既存採点器で処理しました。正規採点に対応する正例区間は主評価の2区間だけで、比較用には0区間です。陰性区間は含まれていません。元のソースと一次根拠・文脈のハッシュ、ラベル、採点器を照合しています。

| 対象 | Historical split | TP | FP | FN | Precision | Recall |
| --- | --- | --- | --- | --- | --- | --- |
| RustCrypto/KEMs | holdout (now exposed) | 1 | 0 | 0 | 1.0 | 1.0 |
| jwt/ruby-jwt | tuning | 0 | 0 | 1 | null | 0.0 |

両区間とも既存の改修後出力と今回の採点は同じです。FP0はこの区間内だけの結果で、区間外の検出は未帰属です。tuningと過去holdoutを混ぜた汎化精度やF1は作成せず、全体および比較用のPrecision・Recall・F1はnullを保持しています。Rubyの採点対象区間の見逃しは残っています。

主評価の既知命題38件では、正規採点で検出を立証できたのは1件、正規区間の見逃しは1件、対応付け未解決は36件です。未解決の内訳は正規採点で表現できない9件・対応規則なし25件・入力非対応2件であり、36件をFNとは数えません。不明な事実3件・不完全な標本行3行も残ります。比較用の既知命題2件は対応規則なしで、FNへ変換していません。

## 確認済み修正の再評価

| 対象 | Current result |
| --- | --- |
| C# predefined types, bc-csharp | 15 historical hits removed |
| JWT kid wording, BoringSSL | 15 conservative wording checks passed |
| EVP wording, OpenSSL | 2 conservative wording checks passed |
| EVP wording, Node.js | 1 not reassessed; TimedOut; null |
| JWK private output fragments | 4 values / 12 fragments; 0 survivors |

32箇所は固定ソース・旧出力と今回の実出力を照合し、独立担当が限定PASSとしました。C#の対象はint12・string2・bool1で、他の検出や診断から対象ファイルの処理も確認しています。ただし6ファイルにはdetector_partialが残り、全FP除去や完全検査とは扱いません。文言17件では対象位置のlegacy finding IDを維持しています。

秘密値の検査はBoringSSL内の4値に由来する12断片の出力残存0という限定結果です。秘密値・断片そのものや値のハッシュは記録へ出していません。oth、印字RSA所有範囲、raw literal fallback、改行保存や公開値保持の普遍的保証には拡張していません。過去の関連FAILと後続の限定PASSを保存しています。

旧出力はrelease、今回のバイナリはdevで、features=[]は一致しています。入力・SHA・規則条件を照合しましたが、時刻、protocol、性能収集設定は異なります。非時間依存の位置・型・保守的文言と2正規区間だけを比較し、全体的な因果改善、時間依存のrisk・期限値の同等性はUnknownです。今回の製品340ファイルは前回の最終Ruby epochと同一で、修正を二重実装していません。

## 残る改善候補10テーマ

| Theme | Selected repositories | Future repositories |
| --- | --- | --- |
| API-RUBY-SODIUM-BINDINGS | 0 | 1 |
| API-PYTHON-OQS-ROLES | 0 | 1 |
| NATIVE-PQC-UNDERSCORE-ALIASES | 4 | 0 |
| INSPECT-SOURCE-GRAMMAR | 17 | 37 |
| INSPECT-DEPENDENCY-FAILURE | 3 | 4 |
| INSPECT-DEPENDENCY-SUBSET | 9 | 18 |
| INSPECT-DEPENDENCY-COMPLETENESS | 2 | 6 |
| INSPECT-CERTIFICATE-FORMAT | 14 | 22 |
| REPORT-DIAGNOSTIC-TRIAGE | 4 | 7 |
| JWT-RUBY-MEMBER-SYNTAX-SCOPE | 1 | 0 |

件数は既存の候補台帳と選定35件のリポジトリの共通部分で、欠陥数や新たな裁定ではありません。根拠となる候補ID・対象・旧sourceConfirmed/investigate判定を再評価JSONに保持しています。RbNaClとliboqs-pythonのAPI候補は今回対象外であり、解決済みに変更していません。下線付きPQC名のNative正規化、解析文法、依存・証明書範囲、診断整理、Rubyのkid:構文は未解決です。

## ディスク整理と保全

開始時の空きは501,297,152 bytes（約0.47GiB）でした。報告書作成時の観測値は5,212,106,752 bytes（約4.85GiB）で、観測時刻は2026-11-26T22:31:44.330192+00:00です。

| Operation | Files | Logical bytes | Observed free-space increase (bytes) |
| --- | --- | --- | --- |
| Delete regenerable .rlib/.rmeta/.o/.a | 5740 | 4,832,751,840 | 82,464,768 |
| Transparent historical-output compression | 933 | 6,613,521,895 | 5,869,359,104 |
| Transparent finished-primary-output compression | 249 | 831,632,638 | 743,354,368 |

フォルダー全体は削除していません。削除対象の正規パス・用途・参照・保全対象を確認し、5740ファイルを個別にハッシュ照合しました。最後の圧縮は286候補のうち249件を置換し、効果のない37件と対象外65件を維持しています。圧縮前後で元のパス、論理バイト列、サイズ、mode、mtimeを照合しました。inode・ctime・圧縮フラグは変更され得ます。

APFSの共有ブロックがあるため、論理容量と実際の空き増分を分けています。各工程前後の観測差には他のディスク活動の影響もあり、入れ子のdu値は合算していません。ソース、正解、裁定、標本、分割、生出力の内容、ログ、マニフェスト、レビュー、測定履歴、必要なバイナリ、将来63件を保全しました。既存出力の読み取りは同じパスで継続できます。

削除した個別パスの全一覧・ハッシュと、圧縮した全パスはSTORAGE-CLOSURE.jsonから参照できます。削除は共通証跡ルート内の次の5領域のうち、マニフェストに列挙した中間生成物だけに限定しました。

| Canonical build root |
| --- |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/product-export-privacy-fix/target-default |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/isolated-temporal-ci-f2054133015d4a69acb084711db37acb/target-ci |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/ruby-scalar-range-implementation-73c7e151b22c425b9b5a12e458b5857d/target-default |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/shipping-envelope-1ef73ce550084ea9bff61b199835d483/target-overlay-warm |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/ruby-epoch-final-validation-9976999cac6e47dfb0717fdb3e630bec/target-default |

圧縮を解除した別ファイルが必要な場合の方法は、ditto --noclone --nopreserveHFSCompression --nohfsCompression SRC NEWPATHです。先に記録した方法で復元照合を行い、置換前に元のSHA-256との一致を確認します。全体の大きな退避コピーは作成していません。ビルド複製のcp -cRは未対応・別ファイルシステムでは通常コピーへフォールバックし得ます。当初の保証し過ぎた表現は追記で訂正し、同一APFS上の実際の観測と区別しました。

## 必須検証・独立レビュー・実ブラウザ

作業worktreeは/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan、ブランチはcodex/oss-crypto-corpus、HEADはcf3bfe2071228921f5a33d2a44e0f3b94de4d1deです。未コミット・未追跡変更を保持しています。製品340ファイルはsourceEpoch oss98-final-ruby-dc58c84b6f8f4bc9873dadbaab5c3e4fと一致し、現行評価基盤111ファイルも固定ハッシュへ照合しました。Rustツールチェーンは1.93.1です。評価バイナリのSHA-256は8b1d0bbc11d7e08483c31d6d50913126957234ee3b796aac323cc7ca06b04a69です。

| Validation | Result |
| --- | --- |
| Default Rust tests | 1045 passed / 3 ignored; source-exact reuse |
| Default fmt / clippy / build | exit 0; source-exact reuse |
| All-features tests | 1107 passed / 5 ignored |
| All-features clippy / build | exit 0; isolated target |
| Rebuilt CLI subset | 132 passed; overlap with1107; 29 rebuilt executables |
| Python CI | 197 passed; installed outer wrapper passed |
| No-performance regressions | 60 passed |
| Selected scorer controls | 6 passed |
| Native existing regressions | 22 passed plus orchestration controls |
| Successful output audit | 34 passed (27 primary / 7 comparison) |

以前の1696バイトの不完全なテストファイルは原本を保持し、chmod・実行していません。全機能検証は別のビルド出力領域で完了しました。CLIのコピー済みテストにはコピー元の実行先が埋め込まれていたため、別領域で29実行ファイルを再生成し、132件を正しいCLIへ結び付けて再検査しました。132件を1107件へ加算していません。Python CIの900秒への変更は外側ラッパーの1定数だけで、197件の検査・終了判定は維持しました。OSS解析の期限は600秒のままです。

旧98件専用の105件準備検証と歴史時計再生は今回の明示35件・現在時計の経路では使用していません。旧未完記録と許可falseを保持し、105件成功とは記録していません。今回変更した経路の検証と実入力・最終ソース照合を使い、影響のない全検証を機械的に再実行していません。独立技術レビューでもこの適用範囲を確認しています。

重要な実装・テスト妥当性、今回の採点・集計・報告の技術レビューはcodex:gpt-6.1-sol@highです。作業記録監査はcodex:gpt-6.1-terra@highで分離しました。各範囲の判定と最終統合レビュー・監査は、報告書ハッシュを含め索引から参照できます。著者による自己承認や過去のレビュー担当の書き換えは行っていません。

Ruby JWTとRustCrypto/KEMsの実HTMLをブラウザで確認しました。英語・日本語、検索・重大度・ファイル・テスト表示、ダイアログのTab/Shift-Tab/Escapeとフォーカス復帰、狭幅での表示、読み込み後に専用ローカルサーバーを停止した状態の操作が成立しました。観測した狭幅は354×767で、横方向のはみ出しはありませんでした。OSのネットワークや保護設定は変更していません。

Ruby JWTの生JSON537件と画面355件は意図された表示統合で、537個のfinding IDは各言語データに保持されています。KEMsのスキャナ内Completeは選定された154ファイルの範囲で、未展開gitlinkを含む入力全体のPartialとは区別しています。画面写真と操作記録はBROWSER-OBSERVATIONS.jsonへ結び付けました。

Prove It Worksは生出力・ソース・実ブラウザの照合に、Sequence Work into Verifiable Unitsは小入力から代表4件、全35件への拡大に適用しました。Explain the Numberは母集団別の件数と限定した精度分母の区別に、Model the DomainはcollectPerformanceの明示的な真偽値と未測定nullの保持に適用しました。

## 証跡と再開条件

固定台帳、入力ハッシュ、全件状態、実行コマンド・終了結果、採点・監査・レビューの参照は[今回の索引](../../../benchmark/evidence/selected35-index.json)にあります。[実行条件](PROTOCOL-35.md)と[英語版](REPORT-35.md)も参照できます。旧98件の記録は履歴として保持しています。

Node.jsは固定600秒の期限へ達し、stdout/stderrは空でreportを生成していません。入力の整合性は維持していますが、停止段階は特定できていません。同条件での再実行は行っていません。再開には、性能計測を伴わない限定した進捗根拠による原因診断、または別途合意した実行上限・条件が必要です。成功後に11形式の監査、4標本行の採点、EVP1箇所の再確認を追加します。失敗した試行は保存し、条件を変えた結果は別試行として扱います。

将来63件と改善候補の全面実装、全体精度研究、性能測定、develop統合、v0.5公開は別作業です。push・PR・merge・タグ・公開・外部連絡・課金は行っていません。今回の縮小をリリース承認や必須検証の免除とは扱っていません。
