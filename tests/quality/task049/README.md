# TASK-049 評価の再実行

元の計画と受入条件は`tasks/049-quality-protocol-matrix.yaml`、結果とprofile差分は
`docs/quality/TASK-049-RESULTS.md`と`TASK-049-TLS-PROFILE.md`を参照してください。

既存の`tests/protocol/prepare-image.py`で工具とlocked Cargo cacheを準備します。
対象実行はこの準備から分離し、既存の`container_runner.execute`を
`case="protocol049"`で呼びます。出力先は存在しない新しいディレクトリを指定します。
今回の呼出しと同じ形式です:

```python
import sys
from pathlib import Path
sys.path.insert(0, "tests/protocol")
from container_runner import execute, image_id
execute(
    image_id("target/quality-task-049/run-20261114/tools/IMAGE.json"),
    Path("target/quality-task-049/new-confined-run"),
    case="protocol049",
)
```

`entry.py`は隔離container内専用です。ホストから直接実行しないでください。
通常/全機能のtest・clippy・build、fmtと、既存OpenSSH offers/KEXを逐次実行します。
`HOST-RESULTS.json`がsource hash・工具image・実際の制限・container削除を記録し、
`VALIDATION.json`が各コマンドの終了状態を記録します。
`raw/`にcase別reportとpeerの受信記録、`ssh-kex/`に実CLI結果を保存します。
TLSの秘密鍵はメモリ内、OpenSSHの一時鍵はtmpfs内で使い、保存しません。

新規試験だけを開発中に確認する場合:

```sh
cargo test --locked --offline -p pqc_scan_probe_tls -p pqc_scan_probe_ssh --all-features 049
python3 -m unittest discover -s tests/quality -p test_protocol049.py
```

これだけでは隔離測定・OpenSSH・二段階レビューの代わりにはなりません。
`PQC049_EVIDENCE`を指定した新規Rust試験は公開情報のJSONを保存します。
TLS未知groupはアクセサー境界の単体対照であり、未知groupとの実接続成功ではありません。

測定後の生データから6成果物を再集約する場合:

```sh
python3 tests/quality/task049/assess.py \
  --raw target/quality-task-049/new-confined-run/raw \
  --out target/quality-task-049/new-assessment
```

固定peerの条件と実report、packet種別、通信量、後始末を照合し、不足ファイルや
観測と矛盾する値は失敗にします。集約のPassedだけで全環境・全入力の安全性や
プロトコルの全面適合を認定しません。CI protocolsの停止設定は維持します。
