# Scorecard acquisition and execution notes

The official v5.6.0 darwin arm64 archive was newly downloaded (25,777,148 bytes).
Its SHA-256 matched the official GitHub release asset digest; the extracted
binary SHA-256 is retained in official/RECEIPTS-3.json. The tag resolves to
61e84743356275f6ffca23b1b2f2cb945e19eb96, matching the binary version and raw
result. Release SLSA provenance was not independently verified; no system-wide
installation or new credential grant occurred. Docker daemon was unavailable;
it was not started. A native macOS fallback was evaluated and used.

The unauthenticated initial attempt returned exit 0 with empty stdout and
SecPolicyCreateSSL error. Removing resource limits or the exec restriction did
not fix it. Removing the home-read restriction advanced to API access but the
unauthenticated request waited for rate limits; the 60-second diagnostic timed
out. Diagnostic timeout output was not retained by that controller, and no
success is inferred. The narrower experiment allowing home metadata,
.CFUserTextEncoding and Library/Preferences also advanced past TLS; it was
stopped at 15 seconds during the explicit rate-limit wait. These experiments
identify the broad home-read denial as the TLS initialization cause; they do
not isolate which of the newly allowed CoreFoundation reads was essential.
No TLS verification was disabled.

The first authenticated attempt had 18 result entries but exit 1. Its 64MiB
per-file cap truncated the remote archive (HEAD contains 119,176,309 source blob
bytes), so source-dependent checks were invalid. OSV also attempted to read the
parent local repository .gitignore because scratch was nested in the checkout.
Its overall 2.9 is retained as invalid-attempt evidence, not accepted as a score.

The final run moved scratch into a fresh OS temporary directory outside the
checkout, raised the archive file cap to 256MiB and scratch supervision to 1GiB,
and retained CPU 300s, wall 600s, RSS supervision 4GiB, nofile 256, nproc 64 and
GOMAXPROCS=2. All writes except scratch and /dev/null were denied; host home data
was denied except the exact binary and specified CoreFoundation preference reads.
Only the fixed Scorecard binary could exec. Network access was enabled. Existing
gh OAuth was passed only in memory, redacted before saving outputs and never
included in commands. Existing scopes are broader than necessary; no new grants
or minimum-scope-token claim is made. Endpoints were not firewall-allowlisted.
Native RSS/disk supervision is sampled and is not Linux aggregate cgroup control.

FINAL-RUN.json records exit 0, 18 checks, source commit and real elapsed/resource
observations; final.stderr is empty. Packaging remains unscored (-1), with the
tool's reason retained. It is not a tool execution failure or a zero score.
Scorecard cleaned its remote archive/tree before returning, so no archive digest
was recoverable. COMMITTED-TREE.txt and REMOTE-INPUTS.json bind the independently
identified committed reference, not unobserved tool-internal archive bytes.

The valid measurement is the remote repository at its reported commit plus live
GitHub/OSV/BadgeApp observations. It is not the dirty local task063 remediation.
A commit does not freeze mutable PR/protection/release metadata or the OSV service.
No repository code, tests, build scripts or diagnostic target endpoints ran in
the network-enabled Scorecard process. No paid job or hosted workflow was started.
