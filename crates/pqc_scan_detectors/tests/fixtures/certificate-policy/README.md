# Task 021 public certificate policy corpus

The nine task 020 public fixtures are reused unchanged. `dsa-sha256.der` is an additional public-only 2048-bit DSA certificate generated with OpenSSL 3.6.3 for parser/classification tests. Its temporary key/parameters were outside the repository and deleted. It is distributed under the repository MIT license. Tests do not invoke OpenSSL, generate keys or access a network.

`manifest.json` pins its SHA-256 and independently inspected signature/SPKI identifiers. FIPS 186-5 does not approve DSA for new signing; this intentionally classical test fixture is not a deployment recommendation.

`cases.json` is a reviewed certificate-only truth table for task 021, not the broad task 023 accuracy corpus. Truth units are `(case, rule ID)` observations of encoded facts; lists were specified before classifier implementation using RFC 3279/4055/5758, RFC 5280 and task 020/OpenSSL public facts. The fixed assessment instant is 2026-11-09 00:00:00 UTC (1794182400).

Tests reconstruct certificate envelopes to change validity or declared signature identifiers. These parser-only mutations invalidate signatures; signature/chain validity is never claimed. Default ranges 2020-2040 avoid wall-clock boundary ambiguity in actual CLI comparison, while Rust tests inject exact seconds for inclusive-boundary cases. Unknown identifiers remain unknown. Malformed/unsupported inputs can produce container observations only. DSA subject-key semantic extraction remains Partial as in task 020; that does not prevent observing its explicit signature OID.

The first cases test subject type/size and exact signature families, including RSA-signed EC, PSS subject-versus-signature, weak digests, unknown/PQC IDs and misleading filenames. Remaining cases distinguish actual expiry, future validity, legacy cutoff, GeneralizedTime, reversed intervals and unparsed containers. No real private-key fixture is included.

The PSS signature case explicitly declares SHA-256, MGF1/SHA-256 and salt length 32. It does not use empty/default PSS parameters (which would imply SHA-1). PSS digest-parameter analysis is outside the supported weak-hash rule profile; no such omission is labeled as proof of digest safety.
