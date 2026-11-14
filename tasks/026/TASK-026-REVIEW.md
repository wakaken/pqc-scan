# Task 026 independent review record

Final design/source/CI review: **APPROVED**. Base
`065646ed67a5211011b5e05b38b2ae5d0d4a6968`; reviewed local working-tree change,
not remote CI. Initial requested fixes below are historical and superseded by the
final re-review. Coordinator validation is recorded in
[TASK-026-LOCAL-ASSESSMENT.md](TASK-026-LOCAL-ASSESSMENT.md).

# task026 独立設計レビュー

日付: 2026-11-10。基準HEAD: `065646ed67a5211011b5e05b38b2ae5d0d4a6968`。
役割: Reviewer。設計・既存契約を読み取り、実装・index・task状態は変更していません。

判定: **設計方針は妥当です。以下の具体化を実装と受入テストに反映すれば実装へ進めます。** 実装承認・テスト通過の判定ではありません。

## 確認した資料

AGENTS.md、prompts/REVIEWER.md、README.md、README.ja.md、architecture、BEST-PRACTICES、DECISIONS、MISTAKES、PATTERNS、TASK-LIFECYCLE、RESTART-LOOP、QUALITY-GATES、task026 YAML、task026 design/plan、DOMAIN-MODEL、STANDARDS-REGISTER、roadmap task026、既存inventory.rs/observationモデルとnative-scan-v1 schema。
RESTART-LOOPの制約は同文書の記載通りrestart baseline/task014に限られ、task026の独立レビューを妨げるものではありません。

## 妥当な判断

- inventory v1は`assessment_status: not_evaluated`と空assessmentsを固定するため、既存schemaを変更せず別の明示opt-in assessment artifactを生成する判断は正しいです。
- `migration-review-v1`はローカル助言として限定され、規制適合・配備準備・普遍的締切を主張しません。旧Risk/severity/rules/RecommendedActionを保持します。
- quantum/classical、construction/standardization、用途/usage/assuranceを分離し、KEMを署名の代替にしない点、hybridのcombinerを未知とする点はdomain modelと整合します。
- 現在のnative_completenessはnot_exhaustiveです。legacy inspectionがcompleteでもaggregate Passは禁止すべきで、この設計はその点を明記しています。
- bounded strict context、固定vocabulary、raw match/snippetの非出力、既存inventoryとの参照整合テストは適切です。

## 実装で必須の具体化

1. **判定表と優先順位**: 明示NotApplicableはその理由・operator provenanceを保持し、applicability unknownはIndeterminateにします。applicableが確定した場合は、既知違反があればcoverage不足と併存してFail、違反なしでもunknown/incomplete/prerequisite未解決ならIndeterminateにします。空scanや未知kindだけのscanは「違反なし」からPassへ変換しません。bounded per-asset Passとscan aggregateの意味を別に明記してください。
2. **contextの適用範囲**: scan全体の単一purpose/JWT/PKI/support値を、異なる用途の全assetの観測事実として適用しないでください。operator assertionとscanner observationを区別し、boolean trueでもruntime/provider/peer/protocol検証済みという主張に昇格させません。用途不一致・未知用途はIndeterminateで、dependency presenceをsupportへ変換しません。
3. **certificateの取りこぼし防止**: certificate/public_key/usageをprimitive policyのNotApplicableにしても、関連algorithmの署名/subject-key評価とcertificate validityのclassical issueをaggregate判断から落とさないでください。未知PSS digest、未知parameters、未評価のtrustも明示します。新規catalogだけで既存inventory normalizationを拡張しません。
4. **symmetric/hashの正確な限界**: AESの鍵探索、hash preimage/collisionを別attack modelとし、query exponentを実装security bitsや総合Passへ変換しないでください。MACや用途・出力長不明のgeneric hashに同一数値を流用しません。MD5/SHA1のclassical collision問題とquantum説明を同時保持します。
5. **safe input/output**: contextをlimit+1 bytesまで読み、unknown enum/field、重複field、oversize、invalid UTF-8、誤型、読込失敗で固定エラーを返します。serdeエラーやパスをそのままstderrへ出さず、synthetic sensitive canaryで確認してください。invalid profile/contextは出力生成前に拒否します。assessment生成は同じrun/asset/observation IDに限定し、別scanとの混線を拒否します。
6. **provenanceと再現性**: profile/catalog version、固定source IDとedition/section、effective-date policyなし、CLI選択による明示opt-in、context省略/指定/applicabilityの由来を保持します。同じscan+contextで安定した順序と結果を生成し、現時刻をpure policy内で読みません。certificate時間評価が必要なら明示scan clockに固定します。

## 最低限の受入テスト

- applicability applicable/unknown/not-applicable × violation有無 × complete/incompleteの表、空scan、未知kind、Fail+unknown混在。
- ML-KEM/ML-DSA/SLH-DSAのexact/generic/draft、RSA/ECのpurpose不明、hybrid両component+unknown combiner、AES/hash attack別結果。
- signing用途へのKEM拒否、JWT JOSE/encoding/verifier prerequisites、PKI issuer/certificate/trust-store prerequisites、dependency presenceのみの非Pass。
- RSA署名+EC subject key、同一SPKI別certificate、expired/not-yet-valid/unknown-clockとquantum結論の共存。
- strict context境界値とsensitive canary、dangling/mismatched inventory refs、unknown factsの安全なbounded結果。
- actual CLI opt-in/schema/golden、profile/contextエラー、既存exit precedence、default/inventoryの旧新shadow、旧schema assessment placeholders不変。

## 証拠の限界

設計と現行コード/schemaのローカルレビューです。公式sourceの新規オンライン再確認、Rust build/fmt/test/clippy、CLI実行、CI、実装レビューはこのレビューでは実行していません。STANDARDS-REGISTERの版・日付自体の独立オンライン検証を通過したとは主張しません。完了前に別途実装/評価証拠が必要です。

# task026 独立コードレビュー

日付: 2026-11-10。基準HEAD: `065646ed67a5211011b5e05b38b2ae5d0d4a6968`。
対象: 同HEADからの未コミット変更（新規assessment/policy、CLI、writer、schema、testsを含む）。
役割: Reviewer。実装・index・HEAD・task状態は変更していません。他者の変更を戻していません。

判定: **要修正（P2 1件）**。以下は初回レビュー時点の判定です。README/validation/evidenceは親担当で追記中のため最終文書承認を含みません。

## 指摘

### [P2] 正式PQC OIDをcatalogへ対応させ、KEMの署名候補への誤変換を防ぐ

対象: `crates/pqc_scan_core/src/assessment.rs:86`（assess_algorithmのOID正規化）、`crates/pqc_scan_core/src/policy.rs:181`（未知purposeからcontextにより候補を生成）。

OID正規化はclassical OIDしか含まず、既存certificate detectorが検出・inventoryへ保存するML-DSA/SLH-DSA正式OIDが全てidentifier/quantum/purpose unknownになります。ML-KEM768 public-key OID `2.16.840.1.101.3.4.4.2`もunknownになるため、PKI contextを与えるとそのKEM public_key/algorithmに署名候補ML-DSAを付けます。未知として保守的に判定するだけでなく、既知KEMの用途情報を消して候補を逆用途に変えてしまう点が問題です。

再現: 既存public fixture `rsa2048.der`を使用し、署名OIDの9 bytes `2a864886f70d01010b`を`608648016503040312`（ML-DSA65）へ、別fixtureのsubject-key OID `2a864886f70d010101`を`608648016503040402`（ML-KEM768）へ同長置換しました。署名検証を目的としないparser-only syntheticです。`target/debug/pqc-scan scan /tmp/pqc026-independent-review --format json --out-dir /tmp/pqc026-independent-out --rules-dir rules --inventory-json --policy-profile migration-review-v1 --assessment-context /tmp/pqc026-independent-review/context.json`（contextはversion1/use_case pki）の終了コードは0。生成inventoryは両OIDを保持しますが、assessmentは両方unknown、ML-KEM public_keyとalgorithmのcandidateはML-DSAになりました。

修正案: assessmentの有限catalogで正式ML-KEM/ML-DSA/SLH-DSA OIDを既知のparameter-set/purposeに対応させ、未知/draft/HashML等は既存scopeに従い保守的に残してください。inventory normalizationやlegacy classificationを拡張する必要はありません。既存のnot_available parameter制約も保持してください。正式OIDの純粋catalog表、および実際のcertificate/public_keyからの用途別候補の回帰テストを追加してください。

一次根拠: [NIST CSOR Algorithm Registration](https://csrc.nist.gov/projects/computer-security-objects-register/algorithm-registration) をレビュー中にオンライン確認。pure ML-DSAはsigAlgs17..19、SLH-DSAは20..31、ML-KEMはkems1..3です。

## 妥当な点

- opt-in CLIにrequires/value_parserがあり、context失敗はscan/output開始前に固定エラーで返ります。16KiB+1読込、strict serde、重複field拒否、UTF-8拒否を確認しました。
- inventory v1のnot_evaluated/empty placeholdersを保持し、別artifact/schemaへ分けています。legacyRisk/severity/rules/recommendationの変更はありません。
- applicability NA/unknownが既知違反より先に評価され、適用確定後はFail+unknownでもFail、empty/incomplete/not-exhaustiveはaggregate Passになりません。
- operatorのbooleanはclaimed_supportとunverified provenanceとして保持され、dependencyがsupportへ昇格しません。JWT/PKI prerequisitesは署名purposeに限定されています。
- hybridのcombiner未知、AES鍵探索/hash preimage/collisionの別モデル、quantumとclassicalの併存、certificate signature/subject-keyの独立評価、固定scan時刻によるvalidity評価を確認しました。
- 新出力はallowlist vocabularyとvalidated IDで構成され、未知アルゴリズムやcontextの任意入力をechoしません。

## 検証と証拠の限界

実行: `cargo test --offline -p pqc_scan_core --test assessment_policy -p pqc_scan_cli --test policy_report`、終了コード0。core5件、CLI2件が成功しました。上記actual CLI syntheticによる不具合再現も実行しました。

読了: AGENTS、prompts/REVIEWER、task026、design/plan、DESIGN-REVIEW、両README、architecture、BEST-PRACTICES、DECISIONS、MISTAKES、PATTERNS、TASK-LIFECYCLE、RESTART-LOOP、関連inventory/observation/certificate parser、変更実装とschema/tests。RESTART-LOOPの制約は同文書の定義通りrestart baseline/task014に限定します。

未実行: 全workspace build/test/fmt/clippy、CI、parent追記予定docs/golden/shadowsの最終確認。これらの通過をこのレビューから主張しません。正式OID対応後の再レビューが必要です。

## 修正後の再レビュー（2026-11-10）

**コード判定: 承認。初回P2指摘は解消しました。新たな要修正事項は確認していません。** この判定は以下のローカル変更・検証範囲に限り、未完了の文書/evaluator/CIを通過したという意味ではありません。

正式pure PQC OIDの明示18件マッピング（ML-KEM3、ML-DSA3、SLH-DSA12）を確認しました。CSORで確認したpurpose/parameter-setと対応し、HashML-DSA等の別OIDはpure variantに誤同一化しません。未知identifierからcontextだけで候補を生成するfallbackは抑止されました。namespace不整合はunknown、opaque not_availableのparameters_known=falseは保持され、parameterless PQCへのunexpected parametersも既知として扱いません。

同一のparser-only syntheticを新CLIで再実行し、ML-DSA65署名OIDはsignature/designed_resistant/parameters_known=false/Indeterminate、ML-KEM768のalgorithmとpublic_keyはkey_establishment/designed_resistantとして出力され、PKI contextで両KEM assetの候補配列が空になることを確認しました。このML-KEM syntheticは元RSAのNULL parametersを保持するため、unexpected_parametersによりFailになります。署名や実装安全性を証明するfixtureではありません。再実行出力先: `/tmp/pqc026-independent-fixed`。aggregate Passでないこともassertしました。

追加の保守的変更も確認しました。assurance不明/観測なしではper-asset PassをIndeterminateへ下げ、hash用途ではpeer/protocolを一律要求せず、署名/KEMはpeer、JWTはJOSE/verifier、PKIはissuer/trust-storeを引き続き要求します。operator claimが検証済みtelemetryへ昇格することはありません。

再検証: `cargo test --offline -p pqc_scan_core --test assessment_policy -p pqc_scan_cli --test policy_report`、終了コード0。core8件、CLI3件が成功。formal OID用途、namespace不整合、opaque/unexpected parameters、unknown assurance、certificate両role/expired、空inventory、pure decision table、strict context、repeatable CLI golden等のテスト内容を確認しました。

未実行/対象外は初回記載のworkspace全チェック・CI・親担当の最終docs/golden/shadow受入を引き継ぎます。レビュー対象の実装・index・HEAD・task状態は変更していません。

## 最終source/docs/schema/golden確認（2026-11-10）

**最終レビュー判定: 承認。未解決のコード/文書指摘はありません。**

最新のassessment observation逆参照indexを確認しました。各observationのasset参照はBTreeSetで重複除去して1回だけindexへ追加し、observation外側ループの元順序を各assetのsupport内に保持します。以前の`contains`による全件filterと同じ所属・順序となり、assetごとの全observation再走査を避けています。先行のreference/ID validationも残っています。

最終文書のLOCAL-ASSESSMENT-V1、README英日、architecture/current inventory contract、STANDARDS-REGISTERの追加差分を読み、実装との整合性を確認しました。明示opt-in、別schema、既存inventory未評価欄維持、aggregate Pass禁止、per-asset Passの限定、scan-wide未検証申告、purpose/JWT/PKI制約、parameter/opaque/HashMLの未対応範囲、AES/hash数学モデルの限界が記載されています。英日READMEの利用者向け説明とCLI例も一致しています。

assessment schemaとchecked-in goldenを再確認しました。full CLI goldenはrun/assessment/observationのrun-local identityのみ同時正規化し、stable asset IDと内容は維持します。schema validation・inventory参照integrity・legacy JSON類の同値比較はCLIテストで継続して実行されます。既存inventory schema、rules/legacy classificationへの変更はありません。

最終局所再検証: `cargo test --offline -p pqc_scan_core --test assessment_policy -p pqc_scan_cli --test policy_report`（core8/CLI3成功、exit0）、`git diff --check`（exit0）。このReviewerは全workspace/CI/shadow/performance集計を新たに実行していません。親Evaluatorが保持する当該検証証拠と区別します。レビュー対象source/index/HEAD/task状態への書込は行っていません。

## CI期待値再bind・legacy shadowの独立追認（2026-11-10）

**追認判定: 承認。コードの最終承認を維持します。**

HEADのSELF-SCAN-EXPECTATIONS.jsonと現在値を独立Python検証で比較し、file/result件数、path、reason、rule、column、countが不変であることをassertしました。変更は既存2files（CLI main/report lib）の既存37 entriesのline/digest再bindとsource-file digest更新だけです。registry-line-map/rebindの各entryについてHEADの旧lineと現在の新lineのsource textが一致し、旧新result digestがregistry記録と一致することをassertしました。全99source-file SHA256も現在bytesと一致しました。新規除外entryはありません。対象行は既存cfg(test) fixtureであり、追加のproduction observationを登録していません。

saved raw SARIFに現expectationを使い`prepareCodeScanning`を独立再実行しました。CLI付与のraw-byte SHA256も別計算し、生成projectionとauditが保存済み`code-scanning.sarif`/`code-scanning-audit.json`へ全件deep-equalとなることを確認しました。分割はraw856 = expected660 + retained196、sourceChecks99件は全てverifiedです。新規source/tests由来の観測は既存660の除外へ追加されていません。

`legacy-shadow.json`の5profiles（旧binary default/inventory、新binary default/inventory/policy）の実際の保存出力を再読込しました。report/cbom/dependency-sbom/SARIF/diagnostics JSONはroot generated_atだけ除去して全5profiles同値、HTML/Markdownは生成timestampだけ正規化して全5profiles同値でした。3inventoryはrun/attempt/time/observation IDの全参照を同時正規化して同値で、asset IDとpayloadは維持されていました。旧binaryでも同じsourceに856件出るため、以前の814との差42件は検出器変更の差ではありません。

この追認はローカル保存raw/outputsを使った独立再計算・照合です。remote GitHub checks、進行中verification.json全体、frozen性能/accuracyの最終集計を代替しません。対象source/index/HEAD/taskへの変更は行っていません。

## RSA-PSS用途境界の追加修正レビュー（2026-11-10）

**判定: 承認。最終コード承認を維持します。** 親Evaluatorが発見したRSA-PSSの用途誤分類を確認し、修正差分を独立に再レビューしました。

RSA-PSS（文字名とOID `1.2.840.113549.1.1.10`）はgeneric RSA/ECのpurpose Unknown分岐から除かれ、Signatureになります。従ってkey_establishment contextのpurpose mismatchから候補配列が空になり、KEM候補へ変換されません。quantum exposure、既存parameter解釈の保守性、legacy severity/rulesは変更されません。R4055 source IDとprofile source edition、goldenのsource列、LOCAL-ASSESSMENT/standards-registerの用途説明も整合しています。

一次資料 [RFC4055](https://www.rfc-editor.org/rfc/rfc4055.html) sections1.2/3をオンラインで独立確認しました。RSASSA-PSS制限付きpublic-key OIDの用途は署名操作であり、RSAES-OAEP key transportとは区別されます。

再実行: `cargo test --offline -p pqc_scan_core --test assessment_policy -p pqc_scan_cli --test policy_report`、exit0。core9件（文字名/OID両方のRSA-PSS establishment拒否回帰を含む）、CLI3件成功。`git diff --check`もexit0。追加sourceのため親が再凍結中の全checks/shadow/performanceは、この局所レビューとは別のEvaluator証拠として扱います。

## 横断用途guardの最終追認（2026-11-10）

**判定: 承認。未解決指摘なし。** Generic RSA/ECの未知purposeに対するcontext推論がSignature/KeyEstablishmentに限定され、hashing/encryption contextで別種類のprimitive候補を作らないことを確認しました。既知purposeのmismatch時の空候補、RSA-PSS署名固定は維持されています。

Certificate signature slotの処理も確認しました。generic RSA/ECだけがroleから署名purposeを解決し、既知KEM等の別purposeは上書きしません。`certificate_signature_purpose_mismatch`を保持し、parameters_knownをfalse、certificate全体のquantum結論をUnknownにして、componentのpurpose/quantum事実を独立に残します。この不正用途をFailにするclassical concernも保持されます。通常の署名/subject-key/validity評価と既存出力には変更がありません。LOCAL-ASSESSMENT-V1の追記も実装と一致します。

独立再実行: `cargo test --offline -p pqc_scan_core --test assessment_policy -p pqc_scan_cli --test policy_report`、exit0（core9/CLI3全成功）。同じ既存table内に追加されたhash/encryption不適合候補拒否、KEM-in-signature-slot syntheticのcomponent purpose保持・mismatch concern・quantum Unknownを確認しました。`git diff --check`もexit0。これ以前のregistry/shadow追認は当時のsourceに対する証拠であり、親が今回のsourceで再凍結する最終checks/shadow/performanceを別途確認する前提です。source/index/HEAD/taskの変更は行っていません。
