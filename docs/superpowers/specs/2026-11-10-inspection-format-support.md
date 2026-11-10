# Inspection format support

User requested corrections for all nine incomplete example files. User explicitly selected PKCS#12 support limited to unencrypted certificate extraction; encrypted contents remain unsupported with a reason. No private-key extraction, decryption, password CLI, remote services, source execution or package resolution.

Preserve CLI flags, report schemas, rule IDs, legacy identity algorithms and finite work/input bounds. Corrected evidence, supported-scope status and recovered occurrences may intentionally change results; record comparisons. Existing rule patterns/classification do not change. Task023 budgets and task states remain unchanged.

- Replace examples' malformed PEM with a public reviewed certificate; retain malformed input in negative tests. Replace placeholder P12 with a synthetically encoded certificate-only unencrypted PFX using public certificate DER, no private keys.
- Parse unencrypted PKCS#12 Data/AuthenticatedSafe/SafeContents cert bags using bounded ASN.1 library parsing. Route public DER to existing certificate attribute extraction. Keep binary source locations honest. MAC/trust verification is not claimed. Encrypted/unknown/key-bearing data remains explicitly unsupported/partial, never falsely complete. Resource limits and malformed structure preserve safe outcomes.
- Pin tree-sitter-typescript 0.23.2; use dedicated TS/TSX grammars. Preserve source-span occurrence filtering, comment policy and caps; do not broaden rules or add symbol resolution. Valid TS fixtures become complete; malformed inputs stay partial.
- Add bounded supported-subset parsing for go.mod, pom.xml, requirements.txt, Gemfile and Gemfile.lock. No evaluation of Ruby/build scripts, subprocess package managers, fetching referenced URLs or inventing resolved versions. Only fully consumed supported syntax may complete; dynamic/unsupported/malformed records remain explicit and retain usable hits.
- SBOM display labels and validated package identity from purl are distinct. Validate purl type/namespace/name/version; never fabricate or trust conflicting identities, never treat SBOM as installed/resolved evidence. Preserve independent source assertions and legacy field shapes.
- Update English/Japanese user docs, source/runtime dependency evidence, focused/frozen corpus comparisons and exact-source CI omission registry. Keep raw results available and new production observations retained.

References: https://docs.rs/tree-sitter-typescript/0.23.2/tree_sitter_typescript/ ; https://www.rfc-editor.org/rfc/rfc7292 ; RustCrypto pkcs12 syntax documentation. These are source/design references, not interoperability evidence.
