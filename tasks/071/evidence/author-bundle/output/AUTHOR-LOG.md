# Synthetic vignette author log

## Assignment and isolation

The author produced data only in `/tmp/pqc071-author-1_ik0eew/output/`. No product was run, repository file opened, code or product output reviewed, network accessed, or agent delegated. Temporary Python invocations serialized and checked authored data only; no implementation or product tests were created. Files owned by other agents were not changed.

The runtime exposed general system/developer instructions and the user's project-agent guidance in the conversation context, including the repository location. No task-specific repository implementation, product test cases, product output, measurement history, or main-thread investigation was supplied to or consulted by this author. The author did not open any AGENTS.md or project documentation file. The scoped assignment and the supplied source package were the authorship inputs.

## Model identity disclosure

Exact backend model identifier: **not exposed**. A developer instruction describes the assistant as Codex based on GPT-6. That is the full exposed model description; no specific GPT-6 variant, service tier, sampling configuration, or backend identity is inferred. No human case author or human operational validation participated.

## Supplied assignment prompt

```text
Create the24 synthetic vignettes/12 pairs specified by /tmp/pqc071-author-1_ik0eew/INPUT-SPEC.md. Your sole responsibility is case authorship. Read only that specification, SOURCES.json and primary-source files under /tmp/pqc071-author-1_ik0eew/sources/. You have no authorization to read repository files or implementation/tests/output/history, run the product, review code, or delegate. Own only /tmp/pqc071-author-1_ik0eew/output/; other agents' edits must be preserved. Supply source-grounded technical expectation hypotheses with uncertainties, no policy-output oracle. Keep family groups out of both splits. Save exact generated JSON and author log; record all accesses/corrections and exposed model identity honestly. Do not add any new implementation or tests.
```

## Input-specification text

The following is the exact specification read from the supplied file, copied again when writing this log:

```text
# Minimal synthetic-vignette input specification

You author data only, not code or policy. Supply24 original synthetic cases in12
one-factor pairs. Each pair has a distinct project/system/pattern lineage. Assign
4 complete pair families to tuning,8 complete families to final confirmation. No
algorithm/protocol/pattern family can cross splits (group related families together).
Use no existing test cases or product outputs. Use only supplied primary sources;
state explicitly when a fact is not verifiable. No human or real operational evidence.

Output JSON to output/cases.generated.json, schema:
{"author_disclosure":{...}, "families":[{"id":"F01","split":"tuning|final",
"lineage":"...","technical_family":"...","one_factor":"JSON path changed",
"rationale":"...", "cases":[{"id":"S01A","scenario":"English narrative",
"capture":{"kind":"primitive|tls|ssh_offers|ssh_kex","algorithm":"exact identifier",
"parameters":{"key_bits":"integer string","curve":"optional exact name"},
"namespace":"optional name|oid|draft", "role":"algorithm|public_key|certificate_subject|certificate_signature|key_establishment|advertised_kex|host_signature",
"outcome":"complete|partial", "assurance":"syntactic|parsed|verified_negotiation|unauthenticated_advertisement",
"tls_group_id":4588,"tls_version":772,"cipher_suite_id":4865,
"ssh_method":"optional exact name","notes":"only applicable fields supplied"},
"claims":{"applicability":"applicable|not_applicable|?", "use_case":"signature|jwt|pki|key_establishment|encryption|hashing|?",
"usage_state":"active|planned|test_only|unused|?","business_critical":true,
"confidential":false,"historical_ciphertext":false,
"confidentiality_until":"optional ISO instant|?","signature_verification_until":"optional ISO instant|?",
"migration_lead_seconds":86400,
"deadline":{"kind":"none"}|{"kind":"at","at":"ISO instant","scope_reason":"organization_policy","reference_id":"policy-local"},
"support":{"runtime":true,"provider":true,"parameters":true,"peer":false,"protocol":true,"jose":true,"verifiers":true,"pki":true}},
"provenance":{"actor_id":"synthetic-owner","asserted_at":"2026-11-01T00:00:00Z","valid_until":"2027-11-01T00:00:00Z"},
"technical_expectations":[{"property":"purpose/quantum/parameters/role/assurance/component or boundary assertion","expected":"source-grounded value","source_file":"sources/...","clause":"specific clause","rationale":"paraphrase; not a product result"}],
"unconfirmed":[] }]}]}

Use fixed evaluation instant2026-11-19T00:00:00Z and capture2026-11-01T00:00:00Z.
Omitted claims are unanswered; '?' is explicit unknown, false is a declaration.
Provenance is synthetic operator assertion, never independently verified. Expiry
may be omitted, then freshness is unknown. Do not invent quantum arrival dates.
Use quoted synthetic policy references only, no actual domains/keys/secrets.
The main author will construct adopted immutable saved envelopes and verify your
technical expectations separately against sources; you do not provide policy
expected outputs, numerical scores, superiority or usefulness judgments.

Keep source roles and assurance distinct; a program reference is not actual use;
parsed facts are not signature/trust verification; advertised SSH is not negotiated;
TLS cipher is not its key exchange. Exact identifiers from supplied standards can
be used, but do not invent protocol registry IDs or equate draft and final names.
Prefer simple ordinary primitive captures plus several protocol/scope-boundary
pairs when primary facts suffice. Avoid arbitrary unsupported input features.
A pair changes exactly one substantive fact/claim, not multiple hidden fields.
Design meaningful novel combinations, not renamed versions of familiar examples.
Do not change more than one algorithm component/parameter and call it one factor.

Read SOURCES.json and relevant supplied source texts. Do not inspect any path outside
this directory, no repository code, tests, plans, product outputs, shell environment
or main-thread history. Do not run products. Do not delegate. No networking is needed.
Record actual source files read, precise model identifier if exposed (otherwise say
not exposed; never guess), supplied prompt, output iterations and any self-corrections
in output/AUTHOR-LOG.md. You are not alone in the codebase; own only output/ here;
do not revert others' files. Respond with output paths and data limitations.
```

## Access record

All explicit input-file accesses were confined to the package. The following records the actual reads/searches in their execution order. Search output was sometimes truncated by aggregate tool output limits; no unread remainder is represented as having been fully inspected.

1. Read `INPUT-SPEC.md` in full with `cat`.
2. Read `SOURCES.json` in full with `cat`.
3. Searched `sources/sp800-131ar2.txt` for `AES|SHA-2|SHA-256|RSA|DSA|ECDSA|Diffie|HMAC|112|2048|128`; displayed result was part of a truncated combined output.
4. Requested `cat sources/rfc6151.txt`; its output was in that same truncated combined output, so later targeted reads were used for selected clauses.
5. Searched `sources/rfc10042.txt`, `sources/rfc10024.txt` and `sources/rfc9846.txt` for numbered headings and `X25519|SecP256|secp256|MLKEM|4588|4587|4589|Key Exchange|key exchange|cipher|unauthenticated|negotiate`; the combined output was truncated.
6. Read `sources/rfc10042.txt` in full with `cat` (about 8,157 tokens returned without a truncation warning).
7. Read lines 100–240 of `sources/rfc6151.txt`.
8. Read lines 1–250 of `sources/rfc10024.txt`.
9. Searched `sources/fips203.txt` for `ML-KEM-512|ML-KEM-768|ML-KEM-1024|quantum|shared secret|Parameter Sets`; some aggregate output was truncated.
10. Searched `sources/fips204.txt` for `ML-DSA-44|ML-DSA-65|ML-DSA-87|quantum|signature|parameter sets`; some aggregate output was truncated.
11. Searched `sources/fips205.txt` for `SLH-DSA-SHA2-128s|quantum|signature|parameter sets`; the individual result and aggregate output were truncated.
12. Read lines 267–375 of `sources/rfc10024.txt`, including final TLS group IDs and the distinction from obsolete draft Kyber group names.
13. Read lines 30–110 of `sources/fips204.txt`.
14. Read lines 645–680 of `sources/sp800-131ar2.txt`.
15. Read lines 553–642 of `sources/sp800-131ar2.txt`; a small part of aggregate output was truncated, but the AES lengths, modes paragraph and relevant table rows were visible.
16. Read lines 5945–6042 of `sources/rfc9846.txt`.
17. Read lines 30–115 of `sources/fips203.txt`.
18. Read lines 321–354 of `sources/fips205.txt`.
19. Read lines 2910–2988 of `sources/rfc9846.txt`.
20. Read lines 698–758 of `sources/sp800-131ar2.txt`.
21. Searched `sources/fips204.txt`, at most 12 matching lines, for `ML-DSA-65|ML-DSA-44|ML-DSA-87|Parameter Sets`.
22. Read lines 2225–2256 of `sources/fips205.txt`.
23. Read lines 3110–3169 of `sources/rfc9846.txt`.
24. Read lines 1050–1068 of `sources/fips204.txt`.
25. Read lines 1007–1024 of `sources/sp800-131ar2.txt`.
26. Read `SOURCES.json` again through Python to copy the supplied manifest into the author disclosure. Listed hashes were copied, not independently recomputed against source files.
27. Read lines 65–107 of `sources/rfc6151.txt` to confirm its introductory purpose distinctions.
28. Searched `sources/rfc9846.txt`, at most five matching lines, for `0x0304|772`; these lines support the TLS 1.3 version value, with 772 being the decimal conversion of 0x0304.
29. Read the newly authored `output/cases.generated.json` through Python for data-only consistency checks.
30. Read `INPUT-SPEC.md` again while composing this author log, and read the newly authored JSON bytes to compute its artifact hash.

Primary-source files actually consulted: `sp800-131ar2.txt`, `rfc6151.txt`, `rfc10042.txt`, `rfc10024.txt`, `rfc9846.txt`, `fips203.txt`, `fips204.txt`, and `fips205.txt`, all under `sources/`. No PDF binary, `pqc-faq.html`, unlisted source, external linked reference, shell environment, source directory inventory, repository file, or existing test/output file was read. Named external references within supplied publications were not fetched or independently inspected.

## Authorship iterations and self-corrections

- During unsaved design, considered grouping several different classical asymmetric algorithms in tuning and symmetric/hash primitives in another split. Rejected that allocation because ECDH and hash components also occur in the protocol hybrids, creating related-family leakage. The saved allocation keeps every RSA case in tuning and all other captured algorithm families and all protocol hybrids in final. The four tuning families have distinct project/pattern lineages while sharing the RSA technical group deliberately.
- During unsaved design, rejected an RSA certificate-role pair with a fixed 3072-bit key field: the certificate-signature identifier does not by itself supply the issuer's modulus length. The saved pair omits key parameters on both sides and records that uncertainty.
- Avoided changing a hybrid group as one factor, because that would change multiple cryptographic components or parameters. Both protocol pairs instead preserve exact identifiers and vary a single assurance/completeness fact.
- A possible HMAC-MD5 family was considered during unsaved exploration but not used; source searches/read ranges include its discussion and this is disclosed above.
- One JSON serialization was saved. There has been no saved JSON correction or source-driven revision after that serialization. This is the first saved author-log version.
- The only output artifacts are the exact generated JSON and this log. No source verification record, adopted evaluation envelope, policy expected result, scoring result, superiority judgment, or usefulness assessment was authored.

## Data-only checks and limits

A Python check of the saved JSON completed successfully (exit 0): JSON parses; there are 12 complete two-case families, 24 unique case IDs, four whole families in tuning and eight in final; each pair differs in exactly the named one-factor leaf across `capture`, `claims`, and `provenance`; each case has source-linked technical hypotheses. IDs, narratives and hypothesis wording are author metadata; substantive pair comparison used only capture/claim/provenance inputs. The check does not independently validate source interpretation, product acceptance, operational assertions or implementation behavior.

F01–F04 form the RSA tuning group. F05–F12 remain final, including the shared ECC, ML-KEM, AES and SHA components inside TLS/SSH methods. No paired family is split, no final capture uses RSA, and no tuning capture uses a final technical family. Generic schema labels such as `signature`, `active`, and `parsed` are not treated as algorithm or system lineages.

All scenarios, dates, support assertions, deployment statements, deadlines and provenance are synthetic. Missing claims remain unanswered. One family deliberately omits provenance expiry so freshness is unknown. The stronger TLS assurance is a stipulated synthetic capture property, not a real handshake observed by the author. A complete SSH advertisement still does not establish negotiation or host authentication. Source-publication statements are used as technical hypotheses with limitations, not as current policy-output labels. No quantum arrival date is estimated.

## Saved JSON identity

SHA-256 of `output/cases.generated.json`: `e8ee67e23fdeadee49a0dd9ae1de4f280e5e7ff405298ce257318e71b90f974a`.
