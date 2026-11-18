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
