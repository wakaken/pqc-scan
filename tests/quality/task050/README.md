# TASK-050 性能・再現性の測定

既存`run.py`の明示opt-inです。製品の動作や旧baselineの既定値は変更しません。

```sh
cargo build --release --workspace --locked --offline
python3 tests/quality/run.py --performance-study --binary target/release/pqc-scan --build-profile release --features-label default --out-dir target/quality-task-050/new-host-study
```

出力先は新規の`target/`配下のみです。入力はその外の一時rootにコピーし、コードを実行しません。
入力はMITの本リポジトリ製品Rustソース、Cargo.toml/lockの1/4/8コピーです。
実製品の構造と複数MiBを使いますが、コピーの独立性や一般の製品群の代表性は主張しません。
1/2threads、起動・全出力生成を含む1初回+3warmを逐次測定します。入力files/bytes/hashを前後照合します。

全11出力の生bytes/意味/順序hash、所見/asset/occurrence/observationの個数・重複数・multiset/order hashを保存します。
生成日時と既存protocolに定義した証明書評価時刻以外は消しません。run/observation IDも保持するため、
実行固有IDの変化は安定との判定になりません。HTML/Markdownは生成日時だけ正規化した保守的な文字列比較です。
別の意味抽出器でテキストの並びを捨ててはいません。
全出力原本は各studyのsmall/t1/初回だけ圧縮保存します。他の標本の原本はhash/検査後に削除します。
全標本の出力原本を保存したという意味ではありません。測定・集約JSONと失敗ログ/保持結果は全て保存します。

Linuxは既存strict runnerのcase `performance050`です。工具準備済みのimmutable image recordを使い、
`container_runner.execute(image_id(record), new_output, case='performance050')`で実行します。
`entry.py`はcontainer内専用です。通常/全機能を別target-dirへrelease buildし、各2studyを逐次実行します。
6GiB・2CPU・128PID・4GiB tmpfs・1200秒、非root、外部networkなし、source/root読取専用は維持します。
生原本の保持を明示サンプリングすることで既存64MiB export境界を緩めません。

```sh
PQC050_BINARY="$PWD/target/release/pqc-scan" python3 -m unittest discover -s tests/quality -p 'test_*.py'
python3 tests/quality/task050/compare.py --runs target/quality-task-050/new-host-study target/quality-task-050/other-study --out target/quality-task-050/new-comparison
```

`compare.py`は新しい出力先だけを使います。環境/構成/入力の差を退行にせず、同条件の2x超過を調査信号として保存します。
歴史的な別日の結果も由来と差を示し、制御された日間変動やcold cacheとは呼びません。
`wait4` CPUはOSの子process会計、RSSはOS定義のhigh-waterでtree合計ではありません。
Linux RSSのKiBをbytesへ変換し、macOSは元がbytesです。取得不能はnullです。
小さいRSSではexec前launcher像の下限も含み得ます。wallは5ms polling分解能で、buildを含みません。
失敗（閾値終了・不完全検査・不正rules・deadline）は成功のthroughput/分布から除外し、資源と残った出力を併記します。
新しい閾値、CI、正式対応環境は追加しません。
