# Safe source API excerpts

Status: complete. All four original criteria, final-source checks, four-report browser acceptance and both independent review stages accepted.

## Source and scope

Baseline commit31a463803446e42d7d4e989087feb3f11337b4f3 plus the10changed files bound by [source05 manifest](evidence/IMPLEMENTATION-SOURCE-05.json), SHA256ce308b4ab0086ad8ccad2f9748d11efc82868caeaf7af4f3d3230c98d575b568;334runtime/input bindings. [Combined patch](evidence/implementation-05.patch). This fix remains uncommitted. No new commit/push, version bump, release, rule, schema, privacy opt-out or cryptographic change was selected.

## Behavior and original criteria

1. Recognized PHP/C#/C/C++ calls now expose only a finite core-validated public API name and normalized one-line `API(/* arguments omitted */)` excerpt at the original location. Exact observation association precedes sanitization; restoration follows sanitization. Raw arguments, receiver identifiers and surrounding protected source are never copied. The transient display label is absent from native facts and excluded from CBOM algorithm identities. Existing library/configuration identities retain precedence.
2. Raw-source guards, bounded parser candidate selection, unknown/partial states and finding IDs remain unchanged. Arbitrary labels, suffixes, argument/receiver injection and algorithm-looking canaries are rejected. Default and inventory scan projections agree. Existing privacy and frozen882source controls pass. A pre-fix CBOM projection defect found during validation has its diagnosis and meaningful failing regression preserved; the final regression prevents display text becoming CBOM algorithm metadata.
3. Java/Go do not have the new-language unconditional source-context protection flag. They and other existing languages still share secret/credential/private-key redaction, which can conservatively mask context; a mask is not proof that actual private key material was present. Real scanner regressions confirm normal seven-language fixtures remain readable and Java/Go private-key-block and credential canaries remain absent from both default/native exports.
4. The four originally displayed pinned OSS scopes were rescanned and their report view copies refreshed. Original task085 artifacts remain unchanged. Actual Japanese/English browser observations verify the complained-about Bitcoin line, safe excerpts in each language, keyboard expansion and loaded-offline controls. Both required independent review stages accepted this final source and evidence; see the review record.

## Verification

All9checks in [CHECKS-05.json](evidence/CHECKS-05.json) exit0 against source05. Each command, duration, log and logSHA are recorded there. `CARGO_INCREMENTAL=0`; configured rust-toolchain retained.

- `cargo test --workspace --locked`:956passed,0failed,3ignored.
- `cargo test --workspace --all-features --locked --target-dir target/v05-feature`:1015passed,0failed,5ignored.
- `cargo fmt --all -- --check`; default and all-feature `cargo clippy --workspace --all-targets --locked -- -D warnings`; default workspace and release builds: exit0.
- All9Node test files:134passed,0failed,0skipped.
- `git diff --check`: exit0.
- Meaningful initial excerpt RED and subsequent GREEN, CBOM regression RED/GREEN and targeted08 evidence retained. Superseded/cancelled runs are historical, not substituted final-source passes.
- [Six-language consuming export smoke](evidence/EXPORT-PRIVACY-CONTROLS-05.json): final release CLI `scan --format all --inventory-json --diagnostics` exit0; HTML/Markdown/JSON/SARIF/CBOM/SBOM/native/diagnostics lack synthetic secret canaries. Includes UTF16LE C++; all four new-language call excerpts visible; CBOM/native contain no display-only excerpt text. Java/Go use conditional credential protection. This is synthetic consumer evidence, not external accuracy or runtime cryptography evidence.

## Four displayed measurements

[Rescan receipt](evidence/OSS-RESCAN-05.json) binds final binary/source, commands, original receipt/inventory/report/rule hashes, full input-tree validation and selection bindings before/after, scope/coverage equality and all findingID/rule/location/severity/risk signatures.

| Displayed scope | Findings before/after | Normalized public API excerpts before/after | Remaining masked matches |
|---|---:|---:|---:|
| XMLSecLibs PHP |27 /27|0 /1|13|
| ITfoxtec C# |95 /95|0 /34|13|
| curl C |169 /169|0 /2|126|
| Bitcoin C++ |69 /69|0 /69|0|

The excerpt column measures the changed call display, not all readable findings. Remaining masks include actual private-key fixtures and records without a validated call-display identity, such as type/import/constant/regex references; source context stays conservatively protected. No blanket raw-source unmasking was attempted. Thus this resolves the reported recognized-call presentation without claiming every source record is now verbatim. No new full30 benchmark, precision/recall estimate, runtime validation or migration-assessment rerun was selected.

[Complete CBOM comparison](evidence/CBOM-COMPATIBILITY-05.json) found identical PHP27/C#87/Bitcoin69rows; curl16of169rows recover the existing validated `C/C++ OpenSSL include reference` label instead of `[masked-sensitive-content]`, with every other row field unchanged. Retaining typed drafts in default mode makes its pre-existing safe library-label recovery agree with native mode. This is an explicit value improvement, not a format change or total CBOM byte-parity claim; call-display text is still excluded. The exploratory whole-equality assertion failure and actual differences are preserved in CBOM-COMPATIBILITY-DIAGNOSIS-05.json.

[Displayed report hashes/URLs](evidence/DISPLAYED-REPORTS-05.json); [browser observations and captures](evidence/BROWSER-05.json). Server remains bound only to127.0.0.1:57385, serving a dedicated view directory. The Japanese screenshot [Bitcoin219](evidence/browser-cpp-ja.jpg) shows public identity in summary/evidence/highlighted line. Viewport1232x1397 has no horizontal document overflow; Enter toggles the occurrence, and a loaded duplicate still paginates/opens embedded explanation/closes with Escape after its separate server is stopped. The user's four report tabs remain available.

## Review and completion

See [review record](TASK-086-REVIEWS.md). Implementation source05 accepted by retained implementation reviewer with no remaining actionable findings. Main owns all investigation, edits, testing, measurement and documentation. The retained test reviewer accepted all original criteria with no actionable findings. Main completed the final integrity gate; no required work remains for this bounded fix.
