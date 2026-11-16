# Final-source hosted CI readiness — task053

Status: local corrections and both independent local reviews accepted; hosted
CI and artifact-byte verification remain required.053 is blocked,052 unfinished
and054 unstarted. No remote write has occurred.

## Concrete verification candidate

CI-CANDIDATE.tar.gz is a verification-only source snapshot. CI-CANDIDATE.json
lists every included path, mode, SHA-256, Git blob identity and the changes from
base commit f07d782cd68cfac95d93585d5ace049f590160d4. It contains current tracked
source, new source/tests/task definitions and the exact catalog/source/review
references required for offline aggregation. The archive contains no .git,
ignored target outputs, host HOME, credentials or private keys. Historical raw
files outside the selected catalog references remain in the original worktree;
the candidate is not a release or a replacement of that worktree.

The candidate includes existing uncommitted prerequisite source changes. Merely
pushing the053 Python diff would not reproduce this exact measured checkout.
Do not commit or publish other local files outside the manifest. Review the
manifest and CORRECTION.diff before authorizing remote publication.

## Proposed action requiring explicit authorization

Publish the reviewed verification candidate on a new branch named
codex/task053-ci-verification-20261116 in wakaken/pqc-scan, then manually run the
existing pqc-scan.yml workflow against that exact commit. No merge, tag, release,
actual release signing or repository setting change is proposed. No PR is needed.

The existing manual workflow runs quality, baseline and scan; protocols remains
disabled. This includes the existing repeated baseline measurements and Code
Scanning upload, in addition to Rust/Node/Python validation and integrated reports.
It consumes normal GitHub Actions resources. Validate any check failure within
053 scope; do not hide unrelated or prerequisite failures.

The prior task plan explicitly excluded commit/push and remote execution. The
current request authorizes repairs, but publication would also disclose the
included uncommitted prerequisite changes. Obtain authorization for this exact
candidate and enabled workflow jobs before remote writes.

## Completion checks after execution

1. Bind the run URL/ID/attempt, event, exact commit/tree and source manifest.
2. Check enabled job and required step conclusions; never treat skipped work as
   success. Confirm protocols remains disabled.
3. Download the integrated PR/manual and baseline artifacts, verify actual ZIP
   bytes/digests and inspect regular members with bounded/no-traversal extraction.
4. Check the uploaded catalog SHA, declared source/input/review bindings, all73
   requirement rows, machine/test/Markdown factual parity and preserved052/054
   states against the exact candidate. A remotely generated report may correctly
   retain Partial product states; that is different from a failed integration.
5. Add immutable real hosted receipts to the053 record, preserving previous local
   and remote data, and obtain scoped review confirmation if data meaning changes.
   Only then evaluate Q6-01 and the task's original completion conditions.

Old successful runs and local CI replay cannot satisfy these hosted checks.
