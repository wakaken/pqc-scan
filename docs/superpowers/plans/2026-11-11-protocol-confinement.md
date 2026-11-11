# Protocol confinement plan

Scope: 品質文書の古い現在形を承認済み局所基準へ統一。メモリ・全書込み量・PIDのプロセス群制約、preview/OpenSSH生成を含むreadonly source/空環境/loopbackを実装。製品Rust・ルール・ID方式・014/016取消・023完了・凍結コーパス/測定は保持。公開/CI成功/署名を代行しない。

Implementation: Linux Docker cgroup v2、network none、非root、read-only root/source/cache、専用tmpfs scratchと小さなshmのみ書込。memory+swap/cpu/PID/time制限。イメージ/依存取得はtarget codeを実行しない準備工程として分離。source snapshotは許可ディレクトリと追跡対象だけから作り、symlink/secret env/git/home/socketを渡さない。生成物取出しはtar path/type/合計容量検査。

- [x] 失敗先行: aggregate disk/memory/process/read-only/network/env境界とworkflow経路のテスト。
- [x] 共通container host/inside runner、offline build/test、previewとSSH productハーネス適用。実資源拒否とtimeout cleanupを確認。
- [x] QUALITY-GATESの現在形修正、旧検証記録を履歴として保持、現仕様を別記録。
- [x] 実装レビュー1名。主担当が修正。
- [x] Python/Node/Rust必須検証とstrict container全feature/preview/OpenSSHを実施しsource/成果物のSHAを記録。
- [x] 別の1名が最終テストレビュー、検証記録と完了説明。
