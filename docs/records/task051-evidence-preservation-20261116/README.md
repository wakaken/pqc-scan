# Task051 historical evidence preservation — 2026-11-16

705 historical evidence files were preserved with complete SHA-256 and archive
restoration verification. This is preservation, not a new task, product change,
release, 054 completion or acceptance of unverified quality criteria.

## Privacy and preservation

Physical backup locations, machine-local checkout paths and private recovery
mapping are intentionally not published. The owner retains those details in
private records. This public record contains repository-relative original paths,
logical Git provenance, sizes, checksums, verification results and storage decisions.
It does not reveal the owner's local backup directory structure.

All 705 originals remain preserved outside Git worktrees and temporary storage,
both as individual files and in a whole-payload archive. SHA-256 comparison of
source originals, durable copies and archive-restored files matched 705/705;
restored modes matched as well. Standard shasum independently checked 705 originals
plus the archive. Source originals were not deleted or modified. This is a local
backup, not an off-device disaster-recovery copy.

Archive SHA-256:
`db4257406e56cb5d1edeadc4ccc773db077e63f0aa41156222dc9c1806e6eb1d`.
Original payload: 336622308 bytes (321.03 MiB).

## Git selection

683 originals are stored at their original `tasks/<ID>/evidence/...` paths:
historical results, conditions, commands, logs, case/oracle mappings, source
snapshots, rendered synthetic reports and visual evidence. Original languages,
whitespace and bytes are unchanged. Source snapshots are historical data, not
new active scripts, tests or build inputs.

22 originals remain in private backup: 6 compiled binaries, 15 opaque run/source/
package archives and one 11.36 MiB official raw JSON snapshot. They were not discarded
or assumed reproducible. `EXTERNAL-ONLY.tsv` identifies them by their original
repository-relative paths and hashes. The owner-held mapping is required to
locate the private backup; a checksum alone is not the original evidence.

`MANIFEST.json` lists all 705 original paths, sizes, SHA-256, Git modes and storage
decisions. `ORIGINAL-SHA256SUMS` covers the 705 originals by repository-relative
path; it is an integrity inventory, not a physical backup locator. `GIT-SHA256SUMS`
covers the 683 originals committed at those paths. `BACKUP-VERIFICATION.json` and
`INDEPENDENT-HASH-CHECKS.json` retain measured verification outcomes with physical
locations omitted. Original detailed receipts are retained privately.

## Provenance and limits

Source branch: `codex/task051-dependency-supply-chain-review`, based at f07d782.
Destination before preservation: `cc7f0b793431e2b7a67477f20d99bbfbf265988a` on
`codex/task053-ci-verification-20261116`. The earlier comparison checked reachable
Git history and committed ZIP/TAR/GZIP members, as well as current files.
`PRE-PRESERVATION-COMPARISON.json` is that earlier inventory. Its 705 unpreserved
count describes the starting point, not the outcome after preservation.

No task state, original criterion, frozen catalog label, previous measurement or
source expectation is changed. Current-target hash drift and self-scan applicability
still require 054 reconciliation. Adding evidence can change self-scan inputs; it
does not prove current safety or make a past whole-repository scan current.
Ignored target/build/cache outputs were not part of this 705-file backup. This
operation does not declare the old checkout safe to delete.

## Verification and recovery

The entire archive was restored into a separate empty verification directory.
All entries were checked against the 705 allowlisted relative file paths before
extraction, and all restored hashes and modes matched. No archived code ran.

From the repository root, verify the committed originals with
`shasum -a 256 -c docs/records/task051-evidence-preservation-20261116/GIT-SHA256SUMS`.
For private recovery, obtain the owner-held location and recovery mapping, restore
to an empty directory and verify every hash before copying into a working tree.
Do not overwrite newer task results or source files during recovery.

No new Rust build, hosted CI or implementation/test-review result is claimed for
this evidence/prose-only operation. Original raw whitespace is preserved and its
diagnostics recorded. Public-record privacy edits do not change evidence bytes.
