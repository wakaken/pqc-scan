# Task060 rebuild comparison results

Complete against original Q6-13. Source revision is
`f07d782cd68cfac95d93585d5ace049f590160d4` plus the exact current worktree inputs
in [SOURCE.json](evidence/rebuild-20261115/native-r1/SOURCE.json).

Four actual `cargo build --release --locked --offline -p pqc_scan_cli` commands
exited 0; network adds `--features probe-tls,probe-ssh,probe-ssh-kex`.
Independent source, HOME, Cargo, temporary and initially absent target roots were
used, sharing only read-only dependency sources and installed tools. Neither
build reused the other's artifacts. Both scratch contexts were removed.

Static binaries match in all 16,119,024 bytes; network binaries match in all
22,196,960 bytes. SHA-256, versions and embedded Mach-O metadata also match.
This measures actual independently rebuilt binaries, not repackaged archives.
Both environments use the same physical macOS host, SDK, Rust 1.93.1 and literal
path remapping flags, with fixed SOURCE_DATE_EPOCH. No postprocessing was used.
The result establishes reproducibility under those recorded conditions, not
across other hosts, toolchains or unnormalized default build conditions.

[Comparison](evidence/rebuild-20261115/rebuild-comparison.json) is byte-identical
to the required target/quality-task-060/rebuild-comparison.json deliverable.
[Commands](evidence/rebuild-20261115/native-r1/COMMANDS.json),
[toolchain](evidence/rebuild-20261115/native-r1/NATIVE-TOOLCHAIN.json),
[reviews](TASK-060-REVIEWS.md) and
[final bindings](evidence/rebuild-20261115/EVIDENCE-FINAL.json) preserve the full
measurement, accepted review stages and final-source applicability. No product,
rule, packaging or production support policy changed. Task052 remains unfinished
and task053 interrupted.
