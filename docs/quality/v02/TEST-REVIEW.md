# v0.2 closeout 最終テストレビュー

日付: 2026-11-12。担当: v02_test_review（テストレビュー専任）。
対象: CURRENT-REQUEST-FILES.jsonのcloseout変更と既存変更の統合。

## 判定

ローカル検証の適切性を承認します。修正必須のテスト不足はありません。
この承認は、未実行のhosted CI、clean-source三変種配布、static attestationの成功を意味しません。これらはpush後の正確な候補コミットに対して実行し、同じ担当への限定follow-upで確認してください。

## assertionと対象の対応

- `validate_installed_scan`の成功例は承認済み3 rule IDを全て要求し、件数も3に固定します。missing/duplicate/extra/negative/wrong-line/unreadの6変異を`ValueError`で拒否するテストがあり、単に現実の件数へ期待値を緩めていません。全件example.py:1かつ2ファイル走査を要求するため、plain.pyへの誤検出や陰性入力の未読も失敗します。check-install.pyが実際にこのvalidatorを呼ぶことを確認しました。
- 既存のcommand-surfaceテストはstatic/combined/allの正例とmissing/extra protocolsの負例を保持。archive改ざん、重複、非regular、path escape、展開資源上限、license symlinkの負例も維持されています。static実archiveのmacOS sandbox deny-network clean-prefix受入はhelp/version/rules/scanを外部fixtureで実行し、0.2.0、3 findings、陰性、後片付けを記録しています。単体のmock package内に0.1.0 fixtureが残ることは実製品版検証の代替ではなく、別の実archive結果が0.2.0を検証しています。
- `shell/v02-self-scan.test.cjs`はledgerの実source SHAを検証し、raw不変、exact partition、retained inventoryの完全一致をassert。未知result、過剰な同一result、source変更時の保持を直接検証します。既存policy/歴史allowanceテストも同じ最終Node実行に含まれています。実最終raw SARIFを独立にledgerと多重集合比較し1357件完全一致、retained SARIFも40件完全一致、audit raw SHAも一致しました。fixtureだけを自己一致させて実scanを省略した証拠ではありません。
- 今回のcloseoutでは既存scanner production logicを変更していません。INITIAL-SOURCE.jsonの577 filesからの変更はcloseoutのversion/docs/package範囲に限定され、以前の実装修正を保持。既存semantic/context修正のassertionはpositive/negative、partial診断、位置、解決済み対宣言値、stable IDを検証し、最終workspace/all-feature実行に含まれています。
- native goldenの製品版更新はfinal native JSONテストとworkspace/all-featuresの対象です。inventory/SARIF等のschema版を製品版に置換していません。roadmap改名・公開範囲等は実装レビューで静的照合され、文書のために同型unit testを追加する必要はありません。

## 最終sourceと実行証拠

`TEST-SOURCE.json`の260 source bindingsを現ファイルと独立照合して不一致0件。`FINAL-TESTS.json`の全8ログをSHA256照合して一致。fmt/build/workspace475/all-features526（既存ignore1）/all-targets all-features clippy -D warnings/Node34/quality Python21/distribution11は全てexit0です。ログのテスト数だけではなく、上記のassertion内容と最終sourceへの結合を確認しました。

protocols/HOST-RESULTS.json/RESULTS.jsonはimmutable image27c6be...、source before/after一致、binary before/after一致、60 executable全exit0、preview8、実SSH KEX/OpenSSHを記録。kernel.logの11件は実子プロセスのmemory/PID/disk/timeout/cleanup/network/source境界を含み、単なるDocker設定値のassertではありません。6GiB/no swap/4GiB scratch/PID128/2CPU/1200秒の制限とcontainer removalが記録されています。

baseline/summary.jsonは0.2.0実binary、frozen corpus/truth、3-run semantic/diagnostic/order/score安定性を記録。TP/FP/FNの非回帰と2x調査基準を維持し、pathologicalの保持findings増加と約0.812秒/332MBの費用を調査対象として公開しています。旧14 OSS研究や既存browser観測を今回の再実行へ偽装していません。renderer無変更の履歴browser証拠と新しいCLI/schema検証は別の証拠として扱われています。

## 残るゲート

候補コミットをpushした後、exact-commit通常CIと手動配布workflowの成功、clean sourceの三変種offline installとcommand surface、staticのrepo/workflow/source digestを限定したattestation検証、公開notice flagsの保全を確認してください。develop merge後の最終HEADにもCI成功証拠が必要です。ローカルdirty archiveはこのhostedゲートを代替しません。

本担当はfull suiteの重複実行、実装変更、追加委譲を行っていません。提供済みログと実source/assertionを読み、軽量なhash・JSON証拠照合のみ実行しました。

## 限定follow-up: release notes同梱

配布文書から参照する両言語release notesをarchiveへ同梱する追加修正を承認します。追加の修正必須指摘はありません。

- collectorの変更はdocs/releases/*.mdを既存source root/no-symlink検査と同じasset経路へ追加するもの。test_packageのmockに両言語fixtureを作り、実際に生成したtarのmemberをassertしています。release-notes-red.logは未同梱を検出して失敗し、green.logは全11件成功。両ログSHAはpostSuiteAssetFollowupの値と一致しました。
- TEST-SOURCE-AFTER-ASSETS.jsonの260 bindingsを現sourceと照合し不一致0件。旧TEST-SOURCEとの差はcollectorとそのPythonテストの2件だけで、postSuiteAssetFollowupのbefore/afterとも一致。Rust/rules/CLI/protocol/Nodeは変更されておらず、既存full suiteの全面再実行は不要です。
- notes-static-first/secondは同一SHA256 `45fc0ad4f98555520b22fc698e8362aa40069615cbfeb32728c2ab2915e8e540`。実tar中の両言語notesは現sourceと完全一致。notes内validation参照は同梱されないquality文書への相対リンクからsource repository HTTPSへ変更されています。
- notes-static-install/INSTALL-RESULTS.jsonはこの同じarchive SHAを拘束し、deny-network sandboxでhelp/rules/scan/version全exit0、3 findings、clean-prefix削除を記録しています。新assetが実archiveへ入ることと、追加後も実offline導入が成立することを別々に確認しました。

旧候補41eaの進行中CIは追加修正後の候補成功の証拠になりません。次の最終候補コミットでhosted CI/配布/attestationを再実行する方針を確認しました。本follow-upでもテストsuiteの重複実行・実装変更・追加委譲は行っていません。
