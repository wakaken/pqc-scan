# Confined protocol verification

Prepare tools and the exact locked Cargo cache separately:

```sh
python3 tests/protocol/prepare-image.py --out-dir target/protocol-image
PQC_RUN_CONTAINER_TESTS=1 PQC_PROTOCOL_IMAGE_FILE=target/protocol-image/IMAGE.json \
  python3 -m unittest discover -s tests/protocol -p 'test_*.py' -v
python3 tests/protocol/run-isolated.py --out-dir target/protocol-ci \
  --image-file target/protocol-image/IMAGE.json --previews --ssh-kex
```

Preparation may access official package/tool sources. It copies only manifests
and empty stubs into its new image layer; Cargo fetch does not execute repository
build scripts/tests. OpenSSH 10.2p1 is checksum-pinned. A cached tool image can be
selected by immutable image ID using `prepare-image.py --tool-image sha256:...`.
No target code runs until the offline container starts.

Target compilation, all-feature tests/clippy, eight synthetic TLS/SSH report
previews and five product OpenSSH cases share the same nonroot, read-only source
and root, empty environment, no host HOME/agent/socket and loopback-only namespace.
Container cgroup v2 bounds all descendants to 6 GiB memory (no added swap), two CPU
quota and 128 PIDs. Incremental caches and debug symbols are disabled, while
test/debug assertions remain enabled. All build/temp/HOME/evidence data lives in a 4 GiB tmpfs plus
1 MiB shared memory. The host deadline is 1200 seconds. Every exit path removes
the container. The host exports at most 64 MiB of regular, relative artifacts.
Read HOST-RESULTS.json for actual kernel checks, immutable image and source hashes;
RESULTS.json binds exact executables and their logs. Passing logs do not replace
the separate resource-pressure and cleanup checks.

Set PQC_PROTOCOL_EVIDENCE_DIR to preserve individual resource-check evidence.
The tests trigger memory OOM across children, aggregate ENOSPC across files, PID
exhaustion and a timeout, then check removal. Missing Docker, cache, image, cgroup
limits or a required fixture fails the gate; there is no weaker fallback.

`run-native-smoke.py` and `isolation.py` retain macOS development smoke checks.
They do not enforce group memory or aggregate disk and are not the CI confinement
gate. Previous source-bound records remain historical; current strict-run evidence
belongs to its own output directory. This workflow does not publish or sign builds.
