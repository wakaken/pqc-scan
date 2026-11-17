# Individual static quality completion

Status: requested individual evaluation, fixes and both reviews complete within the explicit scope below; not product acceptance or publication.
User-authorized scope: individually assess the 93 previously untested and five
one-sided rules and 136 static missing-outcome spans; add meaningful tests and
fixes as necessary; obtain both existing independent review stages.
Baseline: ee6661ab016c6e20796f674c50f348f87d950ed4.
Branch: codex/static-quality-completion-20261117. Previous release candidate,
measurements, truth, labels and reviewers remain preserved.

## Acceptance and method

- Reconcile all 98 rule IDs to existing reviewed examples and method limits.
  Reuse valid current-input evidence; add explicit positive and negative nominal
  cases where absent, with input facts and expected observations fixed before
  execution. Test actual default detectors, not a regex generated from the rule.
  New exposed/tuned cases are not independent holdout or universal accuracy.
- Retain the prior 93/5/25 classification as historical. New assessments identify
  exact cases, results, oracle limitations, source hashes and remaining gaps;
  unavailable or failed cases cannot be relabelled Passed.
- Assess all 136 exact historical spans individually. For each outcome, connect
  an observable invariant/error/limit to a named test, or document a source-based
  infeasibility proof and its prerequisite. Recollect bounded current LLVM
  coverage and distinguish observed counters, ambiguous counters and invariants.
  Never manufacture a 100% threshold or treat an unseen outcome as safe.
- Preserve rule predicates, cryptographic meaning, public schema, truth and
  numerical gates. Discoveries requiring compatibility/cryptographic changes
  follow the existing human review boundary; independent authorized work continues.
- Run required fmt/build/workspace/default/all-feature/Clippy and affected Python
  validation. Reuse /root/output_implementation_review, then
  /root/output_test_review under docs/agent/REVIEWER.md. Main agent performs all
  investigation, implementation and measurements. No additional identities.
- No merge, tag, publication or real release signing. No historical task states
  changed merely to record this work.

## Work sequence

1. Freeze individual inventories and reviewed input facts.
2. Add/reuse rule positive/negative checks and boundary/error oracles.
3. Run targeted checks, inspect failures, implement justified minimal fixes.
4. Implementation review; correct findings; required broader tests and bounded
   coverage; individual source/counter adjudication; test review and final checks.
5. Record every individual disposition and any concrete remaining blocker.

## Final individual evaluation

All 98 requested rule IDs have explicit primary positive/negative nominal
controls and individually retained method limits. The original 93 Not Tested /
5 Partial / 25 nominal classification remains historical and unchanged. New
controls are source-aware, not independent holdout. Certificate absence assertions
on malformed/unsupported objects are supplemental conformance checks, not true
negatives, and are not used to manufacture an accuracy score.

All 136 requested spans have individual current-source mappings and behavioral
oracles: 124 unchanged predicates have both outcomes observed, three stricter
replacement predicates have both outcomes observed, and nine remain unobserved
in one direction with explicit prerequisite/infeasibility reasoning. Those nine
are not relabelled as covered or used to claim 100% branch coverage. Test review
accepted the arguments after checking exact pinned parser boundaries. Raw src-path
percentages are not exact production-only or statement rates.

Three product defects were corrected: omitted Java RSA parameter constructors;
Maven rejection mutating an earlier valid declaration; and ASN.1 parameter casts
creating false type/OID/curve/key-size facts. Regression failures are preserved.
Review found budget interference in the first constructor patch. The final patch
preserves the old pass and spends only residual budgets on constructors. Tight
budget controls preserve all other detections' fields and order.

Final-source Rust 1.93.1: default 605 passed / 0 failed / 3 ignored; all features
664 / 0 / 5. Fmt, both builds, both Clippy configurations and static release build
exit 0. Fresh default LLVM export/profile/diagnostic exits 0 in the immutable
Linux container; host source bindings remain equal. Raw src-path lines are
21,066/22,555; regions 34,439/37,067; branches 4,305/5,538. Native macOS tests verify
observable contracts; these are not macOS-specific coverage percentages. All
seven mismatch entries concern the same locked memchr definition outside owned
src. Historical all-feature auxiliary dump failure is not retried or claimed fixed.

Actual static-binary 023/042/043 corpora repeat stably: 52/205/216 findings with
unchanged rule/file/location multisets and finding IDs. Original same-truth 023
nonregression passes. Full field comparison retains 2/12/12 metadata differences:
incorrect empty parameters_oid on NULL is now absent; all other finding fields
are equal. No normalizer hides the difference. Maven/Java have separate regression
inputs; unchanged canonical counts alone do not prove their fixes.

Self-scan verifies all 106 product inputs: 485 raw = 475 reviewed references +
10 requiring review. Three append-only source hashes were rebound while retaining
every old byte prefix and allowance field. No new omission was learned. Node 53
checks pass. Prior hosted CI remains evidence for ee6661a, not this dirty source;
no current-hosted or publication claim is made.

See [rule assessments](quality-completion/RULE-ASSESSMENTS.json),
[branch assessments](quality-completion/BRANCH-ASSESSMENTS.json),
[oracles](quality-completion/ORACLES.md), and
[field comparison](quality-completion/evidence/COMPATIBILITY.json).
Implementation reviewer /root/output_implementation_review accepted R4 and
maintained the previous Java/Maven acceptance. Test reviewer
/root/output_test_review accepted this expanded source with no actionable finding;
the previous candidate review is preserved separately. No reviewer budget or historical task state is reset.
Product acceptance, merge, real signing/tagging and publication remain separate.

## Individual rule index

| Rule | Previous stratum | Current primary controls |
|---|---|---|
| `API_BC_RSA_ENGINE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_EVP_PKEY_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_GO_CRYPTO_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_JOSE_RSA_ALG` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_NODE_CRYPTO_SIGN_RSA` | Partial | Positive + interpreted negative verified; finite method limits retained |
| `API_OPENSSL_RSA_NEW` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_PY_RSA_MODULE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_RSA_KEYPAIR_GENERATOR` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `API_SIGNATURE_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_DSA_SIGNATURE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_EXPIRED` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_EXPIRY_2026_OR_BEFORE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_MD5_SIGNATURE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_ML_DSA_PUBLIC_KEY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_ML_DSA_SIGNATURE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_NOT_YET_VALID` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_OID_RSA_PSS` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_P12_CONTAINER` | Partial | Positive + interpreted negative verified; finite method limits retained |
| `CERT_PEM_CONTAINER` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_RSA_1024` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_RSA_2048` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_SHA1_SIGNATURE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_SLH_DSA_PUBLIC_KEY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `CERT_SLH_DSA_SIGNATURE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_BORINGSSL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_BOUNCYCASTLE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_GO_JOSE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_JAVA_BOUNCYCASTLE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_LIBSSH` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_OPENSSL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_PARAMIKO` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_PYCA_CRYPTOGRAPHY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_PYCRYPTODOME` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_RUBY_JWT` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_RUBY_OPENSSL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_RUST_OPENSSL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `DEP_RUST_RSA_CRATE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `ENVOY_CIPHER_SUITES_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `ENVOY_SIGNATURE_ALGORITHMS_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `ENVOY_TLS_MAX_VERSION_LEGACY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `GO_TLS_RSA_IMPORT` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `HTTPD_SSLCIPHERSUITE_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `HTTPD_SSLHONORCIPHERORDER_OFF` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `ISTIO_DESTINATIONRULE_RSA_CIPHERS` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `ISTIO_DESTINATIONRULE_TLSV1` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JAVA_KEYFACTORY_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWK_RSA_KEY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_ES256` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_KID_HEADER` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_LIB_JOSE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_LIB_JSONWEBTOKEN` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_PS256` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_PS384` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_PS512` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_RS384` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_RS512` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `JWT_SIGNING_METHOD_RS` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `K8S_GATEWAYAPI_TLSV1` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `K8S_INGRESS_SSL_PROTOCOLS_LEGACY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `NGINX_PROXY_SSL_PROTOCOLS_LEGACY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `NGINX_SSL_PREFER_SERVER_CIPHERS_OFF` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `NGINX_SSL_PROTOCOLS_LEGACY` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `NODE_TLS_MIN_VERSION` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `OIDC_ID_TOKEN_SIGALG` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `OPENSSL_RSA_API` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `PQC_DRAFT_ALGORITHM` | Partial | Positive + interpreted negative verified; finite method limits retained |
| `PQC_GO_TLS_DISABLED` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `PQC_RUBY_API` | Partial | Positive + interpreted negative verified; finite method limits retained |
| `PQC_RUBY_CLASSICAL_GROUPS` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `SSH_AUTHORIZED_KEYS_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `SSH_CONFIG_HOSTKEY_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `SSH_DH_GROUP1` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `SSH_RSA_ALGO` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `SSH_RSA_SHA2_SIGALG` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TLS_CERT_PIN_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TLS_ECDHE_RSA` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TLS_RSA_CIPHER_SUITE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TLS_RSA_KEY_EXCHANGE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TLS_VERSION_1_0` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TLS_VERSION_1_1` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_CERT_SIGALG_LITERAL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_CLASSICAL_EC_API` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_CLASSICAL_EDDSA_API` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_GO_RSA_USAGE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_JAVA_RSA_API` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_JWT_EDDSA_ALG` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_JWT_ES_ALG` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_JWT_RSA_ALGS` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_NODE_JOSE_USAGE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_NODE_RSA_USAGE` | Partial | Positive + interpreted negative verified; finite method limits retained |
| `TS_PY_RSA_USAGE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_RSA_KEY_SIZE_LITERAL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_RUBY_RSA_USAGE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_RUST_RSA_USAGE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_SSH_RSA_STRING` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_TLS_LEGACY_VERSION_LITERAL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `TS_TLS_RSA_CIPHER_LITERAL` | Not Tested | Positive + interpreted negative verified; finite method limits retained |
| `PQC_STANDARD_REFERENCE` | Not Tested | Positive + interpreted negative verified; finite method limits retained |

## Individual branch index

The detailed JSON retains original/current spans, outcome booleans, exact source
lines, tests and proofs. This shorter index does not replace it.

| Original source span | Current disposition | Observable contract / proof |
|---|---|---|
| `pipeline.rs:383:12` | ObservedBoth | `undecodable_wide_bom_is_diagnosed_but_pkcs12_is_left_to_its_binary_detector` |
| `pipeline.rs:385:30` | ObservedBoth | `undecodable_wide_bom_is_diagnosed_but_pkcs12_is_left_to_its_binary_detector` |
| `pipeline.rs:551:12` | ObservedBoth | `nameless_dependency_metadata_is_never_exported_as_a_named_component` |
| `pipeline.rs:690:8` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:707:8` | InfeasibleOutcomeUnderCurrentPreconditions | `snippet_nonempty` |
| `pipeline.rs:717:8` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:734:5` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:735:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:736:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:737:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:738:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:744:5` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:746:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:747:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:748:12` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:758:24` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `pipeline.rs:761:22` | ObservedBoth | `snippet_boundaries_and_each_private_marker_protect_displayed_source` |
| `privacy.rs:79:34` | InfeasibleOutcomeUnderCurrentPreconditions | `quoted_value` |
| `privacy.rs:81:42` | InfeasibleOutcomeUnderCurrentPreconditions | `quoted_value` |
| `privacy.rs:84:12` | ObservedBoth | `basic_and_bearer_credentials_are_redacted_without_erasing_public_scheme` |
| `privacy.rs:337:20` | ObservedBoth | `unknown_source_line_is_sensitive_if_any_private_or_credential_range_exists` |
| `certificate_detector/attributes.rs:71:8` | InfeasibleOutcomeUnderCurrentPreconditions | `spki_envelope` |
| `certificate_detector/attributes.rs:71:28` | InfeasibleOutcomeUnderCurrentPreconditions | `spki_envelope` |
| `certificate_detector/attributes.rs:82:8` | InfeasibleOutcomeUnderCurrentPreconditions | `certificate_spki_alias` |
| `certificate_detector/attributes.rs:88:12` | ObservedBoth | `signature_disagreement_and_nonzero_bit_padding_never_claim_complete` |
| `certificate_detector/attributes.rs:89:12` | ObservedBoth | `signature_disagreement_and_nonzero_bit_padding_never_claim_complete` |
| `certificate_detector/attributes.rs:116:12` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:118:12` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:153:8` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:159:8` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:162:8` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:165:12` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:170:8` | ObservedBoth | `sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields` |
| `certificate_detector/attributes.rs:183:12` | ObservedBoth | `rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status` |
| `certificate_detector/attributes.rs:217:24` | ObservedBoth | `non_oid_algorithm_parameters_are_reported_as_other_not_as_an_oid` |
| `certificate_detector/attributes.rs:235:17` | ObservedBoth | `rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status` |
| `certificate_detector/attributes.rs:237:17` | ObservedBoth | `rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status` |
| `certificate_detector/attributes.rs:240:16` | ObservedBoth | `rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status` |
| `certificate_detector/attributes.rs:242:24` | ObservedBoth | `rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status` |
| `certificate_detector/attributes.rs:320:20` | ObservedBoth | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `certificate_detector/attributes.rs:321:24` | ObservedBoth | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `certificate_detector/attributes.rs:325:33` | InfeasibleOutcomeUnderCurrentPreconditions | `complete_integer_field` |
| `certificate_detector/attributes.rs:326:40` | ObservedStricterReplacement | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `certificate_detector/attributes.rs:333:29` | ObservedBoth | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `certificate_detector/attributes.rs:334:36` | ObservedStricterReplacement | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `certificate_detector/attributes.rs:386:8` | ObservedStricterReplacement | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `certificate_detector/attributes.rs:392:12` | ObservedBoth | `dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer` |
| `dependency_detector/bounded.rs:5:22` | ObservedBoth | `quoted_comments_continuations_and_limits_preserve_explicit_incompleteness` |
| `dependency_detector/bounded.rs:21:32` | ObservedBoth | `quoted_comments_continuations_and_limits_preserve_explicit_incompleteness` |
| `dependency_detector/bounded.rs:24:17` | ObservedBoth | `quoted_comments_continuations_and_limits_preserve_explicit_incompleteness` |
| `dependency_detector/bounded.rs:50:8` | ObservedBoth | `quoted_comments_continuations_and_limits_preserve_explicit_incompleteness` |
| `dependency_detector/bounded.rs:70:40` | ObservedBoth | `quoted_comments_continuations_and_limits_preserve_explicit_incompleteness` |
| `dependency_detector/bounded.rs:82:12` | ObservedBoth | `malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:84:16` | ObservedBoth | `quoted_comments_continuations_and_limits_preserve_explicit_incompleteness` |
| `dependency_detector/bounded.rs:111:16` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:137:32` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:141:24` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:149:16` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:154:42` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:154:62` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:160:19` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:166:12` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:166:33` | ObservedBoth | `go_blocks_and_incomplete_grammar_never_become_complete_inventory` |
| `dependency_detector/bounded.rs:197:19` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:198:21` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:200:13` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:201:21` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:207:13` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:209:21` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:210:28` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:211:28` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:211:46` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:217:14` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:217:24` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:218:20` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:219:20` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:220:20` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:221:21` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:221:31` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:222:57` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:223:21` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:225:21` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:226:28` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:231:13` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:231:34` | ObservedBoth | `go_metadata_accepts_only_complete_declarations_without_evaluation` |
| `dependency_detector/bounded.rs:274:32` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:295:16` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:296:24` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:329:19` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:332:12` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:333:16` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:334:16` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:338:16` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:341:16` | InfeasibleOutcomeUnderCurrentPreconditions | `quoted_gem_capture` |
| `dependency_detector/bounded.rs:342:20` | InfeasibleOutcomeUnderCurrentPreconditions | `quoted_gem_capture` |
| `dependency_detector/bounded.rs:343:20` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:357:16` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:358:20` | ObservedBoth | `requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values` |
| `dependency_detector/bounded.rs:400:20` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:401:20` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:418:38` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:418:49` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:444:60` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:444:80` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:454:21` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:459:20` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:494:16` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:494:25` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:501:17` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:507:31` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:509:28` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:515:40` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:515:51` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:515:65` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:527:8` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:535:24` | ObservedBoth | `lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:557:38` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:595:15` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:596:20` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:637:12` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:637:49` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:696:13` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:702:16` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:705:30` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:706:16` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:708:35` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:709:16` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:722:16` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:723:20` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:731:27` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:748:24` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:756:16` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:756:40` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:767:24` | ObservedBoth | `maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component` |
| `dependency_detector/bounded.rs:791:16` | ObservedBoth | `maven_expansion_keeps_unresolved_cycles_and_bounds_each_intermediate; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |
| `dependency_detector/bounded.rs:803:12` | ObservedBoth | `maven_expansion_keeps_unresolved_cycles_and_bounds_each_intermediate; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution` |

## Final review/source disposition

Implementation: /root/output_implementation_review accepted R4, retaining R3
Java/Maven acceptance and confirming the ASN.1 type-specification basis. Test:
/root/output_test_review accepted every individual row, the 9 prerequisite
arguments, the exact LLVM span/outcome mapping, source/input hashes, explicit
controls and canonical field differences. No actionable finding remains.
Reviewed baseline is ee6661ab016c6e20796f674c50f348f87d950ed4 plus the 25 files in
quality-completion/evidence/TEST-REVIEW-SOURCE.json, manifest SHA-256
7b9e9a56ee6cb967b003fc35960ecba0134bf3bf867bfd37bb0c11af609e16c7.
Subsequent edits only record review decisions and correct evidence references;
reviewed implementation/test bytes remain equal. This fulfills this requested
individual evaluation, not universal quality certification or release acceptance.
This reviewed work is authorized for commit/push and exact-commit hosted CI
verification. CI receipts will identify the tested commit separately; this
pre-CI record does not claim a hosted result or a new source-bound distribution
candidate. Old source-specific candidate artifacts and all historical
truth/labels/failures remain preserved.

## Independent audit reconciliation and release decision

The separate roadmap audit fixed its source at ee6661a, not this change. Its
report and unchanged affected inputs are bound in
quality-completion/evidence/AUDIT-RECONCILIATION.json. The audit independently
assessed 72/73 roadmap requirements Fulfilled and Q6-17 Unknown; that historical
judgment is preserved, not upgraded from the success of these individual tests.

**Static macOS arm64 v0.3.0 release decision: Hold.** The auditor's Medium F-01
(generic isolated suite writes into readonly source target) and Low F-02
(TLS 1.0 boundary consumption mislocates and misses adjacent references) affect
files unchanged by this work. Existing reproductions are reused; the three
fixes in this change do not close either finding. Current CI success would not
exercise the paused generic protocol job or disprove the TLS counterexample.

The requested 98 rule and 136 span assessments are complete within their stated
method limits. The audit's old untested-rule/static-span inventory is therefore
superseded for this finite follow-up, not converted to universal accuracy or
100% coverage. Nine infeasible outcomes stay unobserved, independent holdout
coverage remains limited, and the additional audit fuzz campaign remains Partial
after the auditor's time limit. No old failure is erased.

Before product acceptance: resolve F-01 under the original isolated suite and
boundary checks; resolve F-02 with human review of affected rule compatibility
and location/count/ID regression tests; reconcile all 73 current-candidate
requirements including Q6-17; bind final packaging, installation and provenance
to final source; and obtain the maintainer's explicit acceptance of remaining
limits. The older ee6661a archive/rebuild receipts do not establish reproducibility
of the changed product bytes. Network publication remains separately held.
No merge, tag, actual signing or publication is authorized by this verification.
