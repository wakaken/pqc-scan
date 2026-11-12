# 品質評価で参照する公式資料と適用範囲

資料の確認日: 2026-11-11（Asia/Tokyo）。公開ページ・概要と下記資料を確認した記録です。今回の推敲では再調査していません。
ISOの有料本文全体を読んだ、あるいは全規格条項に適合したとは主張しません。
既存[製品規格台帳](../product/STANDARDS-REGISTER.md)の分類方針を再利用します。
以下は評価方法の選択であり、製品のルール変更・規格認証ではありません。

## 評価フレームワークの比較

| 方法 / 公式資料 | 採用する部分 | 採用しない主張・不足 |
|---|---|---|
| [NIST SAMATE](https://www.nist.gov/programs-projects/software-assurance-metrics-and-tool-evaluation-samate)、[SATE](https://www.nist.gov/itl/csd/secure-systems-and-applications/samate/static-analysis-tool-exposition-sate)、[SATE VI報告](https://www.nist.gov/publications/sate-vi-report-bug-injection-and-collection) | 実物入力・既知事実・誤報/見逃しの手動分析、方法と限界の開示。既存023の固定した正解データに適用 | NIST認定やツールの単一順位付けではない |
| [OWASP Benchmark](https://owasp.org/projects/benchmark) | 正例・負例のラベル、TP/FP/FN/TN、測定結果の比較方式 | Webアプリ脆弱性用のJava/PythonケースをPQC能力の証拠にしない。現在のPQCコーパスを使う |
| [SARD](https://samate.nist.gov/SARD/)、[suite一覧](https://www.nist.gov/itl/csd/secure-systems-and-applications/sard-acknowledgments-and-test-suites-descriptions)、[Juliet C/C++ v1.2 guide](https://samate.nist.gov/SARD/downloads/documents/Juliet_Test_Suite_v1.2_for_C_Cpp_-_User_Guide.pdf) | 弱点を明記し、正負の例と制御・データフローの違いを組み合わせる設計。CWE-327/328等の古典暗号に関連する分類は存在 | 一覧・利用ガイドでは、ML-KEM等のPQC移行評価に直接使える正解データは確認できず。450,000件を超える全内容を精査したという意味ではない。C/C++は現在のAST対象外。Java例の再利用可能性は個別ライセンス/意味レビュー後 |
| [ISO/IEC 25010:2023](https://www.iso.org/standard/78176.html) | 2023-11、Edition 2、Published。ICT/ソフトウェア製品の9品質特性を参考に、独自Q1〜Q6で要求・測定・受入を整理 | 2011版と同一のモデルとみなさない。公開概要で9特性・適用範囲を確認。本文全条項の監査や認証は未実施 |
| [ISO/IEC 25023:2016](https://www.iso.org/standard/35747.html) | 2016-06、Edition 1、Published、2022確認、改訂中。製品品質測定の考え方と独自の分母/式 | 2016年の測定定義を2023モデル全体への自動適合としない。公式概要も一律の合否範囲は定めない。後継DISは最終規格と区別 |
| [OpenSSF Scorecard](https://scorecard.dev/)、[Best Practices基準](https://www.bestpractices.dev/en/criteria/0) | リポジトリ運用・CI/供給網の別評価。CLI/Actionは導入可能、Best Practicesは根拠付き自己申告 | 暗号検出精度ではない。今回Scorecard CLI未導入/未実行。公開APIは404でスコア不明。badge申請なし |

## 暗号・プロトコル・出力規格

| 公式資料 | 確認した版・公開状態 | 本評価への適用 |
|---|---|---|
| [FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) | Final、2024-08-13。2025-11-17 更新候補の注記あり | ML-KEMはKEM、512/768/1024。API参照を実装検証と同一視しない |
| [FIPS 204](https://csrc.nist.gov/pubs/fips/204/final) | Final、2024-08-13。2026-07-31 更新候補の注記あり | ML-DSA署名。旧Dilithium名との互換性を自動仮定しない |
| [FIPS 205](https://csrc.nist.gov/pubs/fips/205/final) | Final、2024-08-13 | SLH-DSA署名。標準原語と暗号実装側・プロトコル側の対応は別 |
| [NIST IR 8547](https://csrc.nist.gov/pubs/ir/8547/final) | Final、2026-10（履歴2026-10-27）。2024 draftとは区別 | 移行対象・用途・情報寿命の判断根拠。普遍的なOSS期限を作らない |
| [SP 800-227](https://csrc.nist.gov/pubs/sp/800/227/final) | Final、2025-09 | KEMの性質・安全な利用。ソースの検出パターンの精度を証明しない |
| [SP 800-131A Rev.2](https://csrc.nist.gov/pubs/sp/800/131/a/r2/final)、[Key Management刊行一覧](https://csrc.nist.gov/Projects/Key-Management/publications) | Rev.2 Final (2019)。一覧上Rev.3は2024-10-21 Draft、SP800-57 Rev.6もDraft | 古典的な強度と量子計算への脆弱性を分離。草案を現行の最終版として扱わない。Rev.2ページの取得はtimeoutとなり、公式検索・索引で確認 |
| [TLS 1.2 RFC5246](https://www.rfc-editor.org/info/rfc5246/)、[RFC8446](https://www.rfc-editor.org/info/rfc8446/)、[RFC9846](https://datatracker.ietf.org/doc/rfc9846/) | TLS1.2 2008、TLS1.3 2018。現在はRFC9846 (2026-07, Proposed Standard)が5246/8446等を置き換える | 旧台帳の8446参照を最新仕様への完全適合としない。9846との差分確認は残作業。TLS版・暗号スイート・鍵交換group・証明書は別の事実 |
| [RFC10024](https://www.rfc-editor.org/rfc/rfc10024.html) | 2026-08、Standards Track | TLS1.3の3 ML-KEM/ECDHE hybrid group。実際の交渉結果と設定文字列を区別 |
| [RFC4253](https://www.rfc-editor.org/info/rfc4253/)、[RFC10042](https://www.rfc-editor.org/rfc/rfc10042.html) | SSH transport 2006、後者2026-08 Informational | 提示方式・KEXの認証・ホスト鍵の事前照合を分離。ML-KEM SSH方式がRFC化されても認証のPQC化は証明されない |
| [CycloneDX概要](https://cyclonedx.org/specification/overview/)、[CBOM](https://cyclonedx.org/capabilities/cbom/) | current 1.7、2025-10-21。ECMA-424公開2025-12-10 | 既存tag1.7のオフラインSchemaと意味を確認する試験を再利用。従来の配列だけのCBOMはCycloneDXではない |
| [SARIF 2.1.0 + Errata 01](https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/sarif-v2.1.0-errata01-os-complete.html) | OASIS Standard incorporating Approved Errata、2023-08-28 | 既存の公式SchemaのhashとURI/位置/Unicodeの検証を再利用。実行時の接続診断には独立したSchemaを使う |

発効は公開日・版・管轄を区別します。FIPSの公示日や法的適用を含む準拠性判定は今回対象外です。
正誤表の全項目が実装に及ぼす影響、SP800全シリーズ、TLS最新差分全条項は未検査です。

## Rust検証手法の一次資料

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)のline/region測定、
nightlyが必要なbranch option、[cargo-mutantsの結果分類](https://mutants.rs/using-results.html)、
[cargo-fuzz/libFuzzer guide](https://rust-fuzz.github.io/book/cargo-fuzz/guide.html)、
[proptest](https://proptest-rs.github.io/proptest/)、[RustSec](https://rustsec.org/)、
[cargo-deny](https://embarkstudios.github.io/cargo-deny/)、[Miri](https://github.com/rust-lang/miri)
を確認しました。採用順序・隔離・コスト・未実行事項は[ロードマップ](QUALITY-ROADMAP.md)にまとめます。
ツール追加そのものを検証能力向上の証拠にはしません。
