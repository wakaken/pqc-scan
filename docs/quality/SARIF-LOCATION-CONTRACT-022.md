# SARIF location contract and verification — task 022

Date: 2026-11-10 (Asia/Tokyo). Delivery: one task, local commit only.
Base and validation working HEAD: `1335033fb3ffd96632f9f39a51bf8f2a115f2ead`.
Branch: `codex/task-022-sarif-location`. Initial worktree clean, fetched
origin/develop and branched from it. Validation covers the uncommitted code
digests below; the resulting commit contains this record. No push/PR/merge.

## Contract and scope

The existing SARIF 2.1.0 projection alone changes. Unknown line (native zero)
omits region, regardless of native column. Known line with unknown column keeps
startLine only. Empty file path omits locations; no synthetic line one is added.
The dependency detector already preserves unknown SBOM locations as 0:0, so no
detector change is needed. Native Finding/Location schemas, IDs, severity, rules,
recommendations, CLI and report filenames remain unchanged.

Paths are filesystem strings, not input URIs. Unreserved ASCII and slash remain;
all other UTF-8 bytes, including literal percent, colon, fragment/query delimiters,
controls and Unix literal backslash, are percent-encoded. No decoding, canonical
path resolution or filesystem reads occur. Relative `./` and `..` components are
preserved, not silently resolved. POSIX absolute paths get file://; on Windows,
native backslashes normalize to slash, drive paths get file:///C:/ and UNC paths
get file://server/share/. Drive-relative paths remain encoded relative references.
Non-UTF-8 filename bytes have already been converted lossily by the legacy model;
this task cannot recover their original identity.

Run columnKind is unicodeCodePoints: one-based Unicode code points, including
combining characters, emoji and tabs individually; never visual width or UTF-16.
Existing regex/key positions use this unit; dependencies and certificate framing
use column one or ASCII indentation. AST native columns combine byte-based node
starts with code-point match offsets. Changing them would also change native IDs.
Instead the SARIF writer emits AST columns only when the existing snippet proves
the entire prefix through the legacy offset is ASCII. Missing/masked/truncated
prefixes and non-ASCII prefixes produce line-only regions. The writer trusts the
provided ScanResult snapshot; it does not reopen paths. External callers must
provide code-point positions (and accurate snippets if requesting AST columns).
A fabricated or stale external snippet cannot establish source truth.

This is an intentional compatibility correction for previous invalid/unescaped
SARIF positions. ASCII locations are retained when provable; some masked or
missing-snippet ASCII AST positions lose column precision. No detector/native
position bug is concealed by changing native zeros or inventing source positions.

## Official schema and test design

[Vendored schema provenance](../../crates/pqc_scan_report/tests/schema/README.md)
records the OASIS 2.1.0 Errata 01 edition, upstream revision, published source,
license/IPR notice and byte digest. Published and pinned repository files were
compared byte-for-byte. Development-only jsonschema 0.29.1, default features off,
validates draft-04 with formats enabled; references are internal and no schema
retrieval is required. Cargo.lock pins transitive dependencies. Initial cargo
fetch populated the local cache, subsequent build/tests/lint ran offline.

The initial writer suite failed four cases on the old implementation: unknown
zero regions, unencoded URI paths, missing columnKind and official schema minimum
constraints. Its unchanged result-payload/determinism control passed. After the
projection correction all five pass. Fixtures cover line/column zero combinations,
empty paths, ordinary ASCII, Japanese, emoji, combining code points, tabs, reserved
characters, literal percent, absolute/relative paths and platform backslashes.
The schema test checks its digest and rejects zero-position negative controls.
An actual CLI test reads two files, detects exactly three controlled findings,
preserves unknown native SBOM 0:0 and asserts SARIF artifact-only/Unicode/AST
semantics plus official schema validity. The escaped JSON name makes unknown
source location real rather than a manually fabricated finding.

## CI expectations and safety

Before adjustment, 752 raw = 509 matched + 243 retained: the registry correctly
failed closed on changed AST result digests. The helper now accepts explicit
column:null expectations only for an absent column; present null/zero/negative
columns remain ineligible, even with matching complete hashes. Positive lines,
source SHA, full result SHA, rule, total count and safe literal path remain
mandatory. No URI decoding was added. Artifact-only, encoded, unsafe, aliased or
indirect paths remain retained. Existing production-versus-fixture CLI acceptance
continues to retain product error-level findings and preserve raw bytes.

109 prior reviewed test groups were re-reviewed and rebound solely for omitted
columns; all other result payloads must match their prior approvals. The appended
helper test's full source hash was refreshed. Six observations were explicitly
registered at the new writer integration test's Unicode RS256 input and lookup,
with no production registrations. The final partition is **752 raw = 624 reviewed
omissions + 128 retained**, 84 verified source files. The 128 retained payloads
match the previous retained population with only intended column omissions.
The full raw bytes and remaining result objects/run metadata are unchanged by
projection. Registry null means unknown, never a wildcard.

## Executed validation

All final commands below exited 0 on Rust 1.93.1, macOS host:

| Check | Command / evidence under ignored target/task022/ |
|---|---|
| Build | cargo build --workspace --locked --offline; build.log |
| Format | cargo fmt --all then cargo fmt --all -- --check; fmt.log |
| Workspace | cargo test --workspace --locked --offline; tests.log; 145 tests passed |
| Lint | cargo clippy --workspace --all-targets --locked --offline -- -D warnings; clippy.log |
| Node | node --test shell/*.test.cjs; node.log; 22 tests passed |
| Whitespace | git diff --check |
| Schema/semantics | Included in Rust writer and actual CLI tests; no network required |
| Compatibility/canary | python3 target/task022/compatibility.py; compatibility/summary.json |
| Self-scan | target/debug/pqc-scan scan . --format sarif --out-dir target/task022/current-self --rules-dir rules; current-self.log |
| Projection | node shell/prepare-code-scanning.cjs with that raw input and current registry; current-self/audit.json |
| Projection invariants | node target/task022/verify-projection.cjs; projection-summary.json |

Old/new binaries scanned the identical four-file temporary root with explicit
rules and one thread. All 13 findings/native locations/IDs and all other native
fields matched, excluding generated_at. CBOM/dependency-SBOM matched bytes;
HTML/Markdown matched after replacing only the exact generation timestamp in
JSON/Chrono display spellings. The initial comparison correctly reported the
un-normalized Chrono timestamp difference; after correcting that timestamp-only
normalization it passed. SARIF rules/non-location payloads/counts matched; four
URIs encoded, three unknown regions omitted, two unproven AST columns omitted.
A synthetic private-body canary was absent from every report and stdout/stderr.
Input bytes/digests and exact root are in compatibility/summary.json; temporary
fixture roots were actually read (four scanned files) and removed afterward.

## Independent review and limitations

Independent reviewer review_task022 used a separate context and reviewed the
whole change for correctness/security/compatibility, including the CI bindings.
The [independent review record](SARIF-LOCATION-REVIEW-022.md) is committed alongside
this report. Final approval found no actionable issues, including after the CLI
fixture portability correction. The task YAML records Evaluator completion. Initial pass found no
material issues and independently ran five writer tests, one CLI test and 19
helper tests. The coordinator ran the full workspace and all 22 Node tests.

Unexecuted: hosted Actions/Code Scanning upload, remote consumers, Windows runtime
and OS/architecture build matrix, performance/accuracy benchmark. Windows code
has conditional tests and source review but was not executed on this macOS host.
No browser acceptance is required because HTML implementation/behavior did not
change and the generated artifact comparison passed. No claim of universal secret
masking, complete scanner accuracy or runtime crypto reachability is made.

## Validated source bindings (SHA-256)

- `crates/pqc_scan_report/src/sarif_report.rs`: `07165114d1af5a250b3824fca2e583071c701e5ded93bd986ea749c1517e732c`
- `crates/pqc_scan_report/tests/sarif_contract.rs`: `6793d90098a2c1e5451d772618d74272c00a486850d92aeda24a96ece8d76c8a`
- `crates/pqc_scan_cli/tests/sarif_locations.rs`: `240337072dce8fa9e830ce2de7acf673db4ba52c95dfe6112e1b1492ccdb95da`
- `shell/prepare-code-scanning.cjs`: `2000ce8bd6bc43b28ddcb6b513a2c52841cc6b30ad296c9751b19b3e04fe0fe5`
- `shell/prepare-code-scanning.test.cjs`: `1127530392b675b4b14b0196fc96db3dfde4e456a8d2cac66337755ff360f327`
- `docs/quality/SELF-SCAN-EXPECTATIONS.json`: `bdac6c93e97660aa96fcff496bf73e585d5cf3b99b437936573b44ffdd013cab`
- `Cargo.lock`: `007f6c5a612b0c168157734a13dc17a9c82bea5cc7653448b7bdbda3ba0c1158`
