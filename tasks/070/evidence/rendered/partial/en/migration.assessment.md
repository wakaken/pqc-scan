# Migration decisions

Targets: 3 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>X25519MLKEM768</code> — Monitor this role

**Where:** <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> · <code>negotiated&#95;key&#95;establishment</code>

**Declared system:** <code>system&#45;a</code>

**Response:** Monitor changes to this role

**Why:** No known concern was established for this bounded role and its recognized facts.

**Next check:**

- Reassess when the facts, scope or claims change.

**Information:** Required information available

**Readiness:** Migration readiness not applicable

**Additional context:**

- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The hybrid combiner implementation has not been verified.

[Evidence, claims and candidate details](#evidence-1)

## 2. <code>RSA&#45;SHA256</code> — Urgency undetermined

**Where:** <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> · <code>certificate&#95;signature</code>

**Response:** Investigate

**Why:** The available facts do not establish a supported urgency branch.

**Next check:**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

**Information:** Information missing

**Readiness:** Readiness unknown

**Missing inputs:**

- Applicability
- Explicit system binding
- Exact parameter support
- Peer support
- PKI support
- Protocol support
- Provider support
- Runtime support
- Parameter encoding
- Exact parameters
- Actual or planned use
- Required purpose

**Additional context:**

- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

[Evidence, claims and candidate details](#evidence-2)

## 3. <code>RSA</code> — Urgency undetermined

**Where:** <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> · <code>certificate&#95;subject&#95;key</code>

**Response:** Investigate

**Why:** The available facts do not establish a supported urgency branch.

**Next check:**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

**Information:** Information missing

**Readiness:** Readiness unknown

**Missing inputs:**

- Applicability
- Explicit system binding
- Exact parameter support
- Provider support
- Runtime support
- Parameter encoding
- Actual or planned use
- Required purpose

**Additional context:**

- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

[Evidence, claims and candidate details](#evidence-3)

## Evidence and scope

Ready means declared support, not a verified deployment. Candidates require separate parameter, encoding and interoperability validation. Monitor is limited to the stated role; outside scope is not a safety verdict.

<a id="evidence-1"></a>

### Evidence 1

Decision: <code>migration&#95;decision&#45;v1&#45;cbcaf81c18274dc1217560b530d4fca80e2ed8da336d94bf45fdafa8bfa23263</code>

Target: <code>migration&#95;target&#45;v1&#45;27f39fc851d2f3fba67b4cb0a064fb5f2ad98226a307188ca0c03c77811c0619</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;9fc56777a19b26959e1edba0dcdb7b7e24759770236d38e7c830ee53eede066d</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;b0bd01c15bc51625607ef97e70ecf85cb81b53b1ce7ff665e8608d9e7e93aadf</code>](#claim-2)

Proposed candidates: 

Sources: [<code>F06</code>](#source-1), [<code>F07</code>](#source-2), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-27f39fc851d2f3fba67b4cb0a064fb5f2ad98226a307188ca0c03c77811c0619",
      "source_ref": {
        "role": "negotiated_key_establishment",
        "attempt_id": 1,
        "detail_path": "/details/negotiated/key_exchange_group",
        "protocol_version": {
          "id": 772,
          "name": "TLSv1_3"
        },
        "cipher_suite": {
          "id": 4865,
          "name": "TLS13_AES_128_GCM_SHA256"
        },
        "key_exchange_group": {
          "id": 4588,
          "name": "X25519MLKEM768"
        }
      },
      "technical": {
        "identifier": "X25519MLKEM768",
        "construction": "hybrid",
        "standardization": "rfc_standard",
        "quantum": "unknown",
        "purpose": "key_establishment",
        "parameters_known": true,
        "classical_concerns": [],
        "attack_models": [],
        "components": [
          {
            "role": "classical_component",
            "assessment": {
              "identifier": "X25519",
              "construction": "classical_public_key",
              "standardization": "classical_standard",
              "quantum": "vulnerable",
              "purpose": "key_establishment",
              "parameters_known": true,
              "classical_concerns": [],
              "attack_models": [],
              "components": [],
              "combiner_assessment": null,
              "source_ids": [
                "F12",
                "LOCAL026",
                "NIST-PQ-FAQ"
              ]
            }
          },
          {
            "role": "post_quantum_component",
            "assessment": {
              "identifier": "ML-KEM-768",
              "construction": "post_quantum",
              "standardization": "nist_final",
              "quantum": "designed_resistant",
              "purpose": "key_establishment",
              "parameters_known": true,
              "classical_concerns": [],
              "attack_models": [],
              "components": [],
              "combiner_assessment": null,
              "source_ids": [
                "F12",
                "LOCAL026",
                "S203"
              ]
            }
          }
        ],
        "combiner_assessment": "standard_specified_implementation_unverified",
        "source_ids": [
          "F06",
          "F07",
          "F12"
        ],
        "encoding_assurance": "unknown",
        "protocol_ref": "F07",
        "role": "negotiated_key_establishment"
      },
      "observation": {
        "method": "tls_saved_observation",
        "assurance": "collector_reported_verified_session",
        "usage": "negotiated",
        "collection_outcome": "partial"
      },
      "limitations": [
        "unknown_parameter_encoding",
        "collection_incomplete",
        "combiner_implementation_unverified"
      ]
    }

    [
      "bounded_role_no_known_concern",
      "collection_incomplete",
      "unknown_parameter_encoding",
      "combiner_implementation_unverified"
    ]

<a id="evidence-2"></a>

### Evidence 2

Decision: <code>migration&#95;decision&#45;v1&#45;bec05938f3d6e94d356b7ec6bfe3eb51d004cc5c17f78ce4dd104e1316e185f9</code>

Target: <code>migration&#95;target&#45;v1&#45;5318468fd239f64171a661c5cb67c82749ff507c88500ef6d0923f48ce7ca2ec</code>

Applied claims: 

Proposed candidates: [<code>migration&#95;candidate&#45;v1&#45;9f87ebb42fbb18a79504a5d5fd9bae02ad91b4938b192e2aae2a63f7e39ae684</code>](#candidate-1)

Sources: [<code>F08</code>](#source-3), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5), [<code>LOCAL026</code>](#source-6), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-5318468fd239f64171a661c5cb67c82749ff507c88500ef6d0923f48ce7ca2ec",
      "source_ref": {
        "role": "certificate_signature",
        "attempt_id": 1,
        "detail_path": "/details/peer_certificates/0/signature_oid",
        "certificate_sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "spki_sha256": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "not_before_unix": 1700000000,
        "not_after_unix": 1900000000
      },
      "technical": {
        "identifier": "RSA-SHA256",
        "construction": "classical_public_key",
        "standardization": "classical_standard",
        "quantum": "vulnerable",
        "purpose": "signature",
        "parameters_known": false,
        "classical_concerns": [],
        "attack_models": [],
        "components": [],
        "combiner_assessment": null,
        "source_ids": [
          "F12",
          "LOCAL026",
          "NIST-PQ-FAQ",
          "F08"
        ],
        "encoding_assurance": "unknown",
        "protocol_ref": "F08",
        "role": "certificate_signature"
      },
      "observation": {
        "method": "certificate_parse",
        "assurance": "parsed",
        "usage": "unknown",
        "collection_outcome": "partial"
      },
      "limitations": [
        "unknown_parameter_encoding",
        "collection_incomplete"
      ]
    }

    [
      "insufficient_information",
      "collection_incomplete",
      "unknown_parameter_encoding",
      "unknown_binding"
    ]

<a id="evidence-3"></a>

### Evidence 3

Decision: <code>migration&#95;decision&#45;v1&#45;50b1f217aef92b790f7a32dece63ecf150a722d1e5a8cf003444ad0577563529</code>

Target: <code>migration&#95;target&#45;v1&#45;7953901b317a79ffd73c18feceb154c8c9f12beb6671a307f535f5591473f18c</code>

Applied claims: 

Proposed candidates: 

Sources: [<code>F08</code>](#source-3), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5), [<code>LOCAL026</code>](#source-6), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-7953901b317a79ffd73c18feceb154c8c9f12beb6671a307f535f5591473f18c",
      "source_ref": {
        "role": "certificate_subject_key",
        "attempt_id": 1,
        "detail_path": "/details/peer_certificates/0/public_key_oid",
        "certificate_sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "spki_sha256": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "not_before_unix": 1700000000,
        "not_after_unix": 1900000000
      },
      "technical": {
        "identifier": "RSA",
        "construction": "classical_public_key",
        "standardization": "classical_standard",
        "quantum": "vulnerable",
        "purpose": "unknown",
        "parameters_known": true,
        "classical_concerns": [],
        "attack_models": [],
        "components": [],
        "combiner_assessment": null,
        "source_ids": [
          "F12",
          "LOCAL026",
          "NIST-PQ-FAQ",
          "F08"
        ],
        "encoding_assurance": "unknown",
        "protocol_ref": "F08",
        "role": "certificate_subject_key"
      },
      "observation": {
        "method": "certificate_parse",
        "assurance": "parsed",
        "usage": "unknown",
        "collection_outcome": "partial"
      },
      "limitations": [
        "unknown_parameter_encoding",
        "collection_incomplete"
      ]
    }

    [
      "insufficient_information",
      "collection_incomplete",
      "unknown_parameter_encoding",
      "unknown_binding"
    ]

## Applied claim provenance

Operator and external-review claims are unverified declarations. Freshness describes their stated interval, not independent confirmation.

<a id="claim-1"></a>

### Claim: Applicability

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;applicable&quot;</code>

    {
      "id": "migration_claim-v1-9fc56777a19b26959e1edba0dcdb7b7e24759770236d38e7c830ee53eede066d",
      "context_pointer": "/bindings/0/claims/applicability",
      "field": "applicability",
      "state": "known",
      "value": "applicable",
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-2"></a>

### Claim: Required purpose

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;key&#95;establishment&quot;</code>

    {
      "id": "migration_claim-v1-b0bd01c15bc51625607ef97e70ecf85cb81b53b1ce7ff665e8608d9e7e93aadf",
      "context_pointer": "/bindings/0/claims/use_case",
      "field": "use_case",
      "state": "known",
      "value": "key_establishment",
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

## Proposed candidates and validation

<a id="candidate-1"></a>

### <code>ML&#45;DSA</code>

Sources: [<code>S204</code>](#source-9), [<code>LOCAL026</code>](#source-6)

- Runtime support: Unknown
- Provider support: Unknown
- Exact parameter support: Unknown
- Peer support: Unknown
- Protocol support: Unknown
- PKI support: Unknown

**Validation checks:**

- Confirm actual use and the required cryptographic purpose.
- Verify exact parameters and protocol encoding before rollout.
- Test runtime, provider, peers and rollout.
- Validate the implementation and interoperability separately.

Exact candidate record:

    {
      "id": "migration_candidate-v1-9f87ebb42fbb18a79504a5d5fd9bae02ad91b4938b192e2aae2a63f7e39ae684",
      "target_id": "migration_target-v1-5318468fd239f64171a661c5cb67c82749ff507c88500ef6d0923f48ce7ca2ec",
      "identifier": "ML-DSA",
      "purpose": "signature",
      "status": "candidate_primitive_integration_unverified",
      "parameter_selection": "explicit_parameter_selection_required",
      "prerequisites": [
        {
          "name": "runtime",
          "claimed_support": null,
          "applied_claim_ids": []
        },
        {
          "name": "provider",
          "claimed_support": null,
          "applied_claim_ids": []
        },
        {
          "name": "parameters",
          "claimed_support": null,
          "applied_claim_ids": []
        },
        {
          "name": "peer",
          "claimed_support": null,
          "applied_claim_ids": []
        },
        {
          "name": "protocol",
          "claimed_support": null,
          "applied_claim_ids": []
        },
        {
          "name": "pki",
          "claimed_support": null,
          "applied_claim_ids": []
        }
      ],
      "validation_steps": [
        "confirm_actual_use_and_required_purpose",
        "verify_exact_parameters_and_protocol_encoding",
        "test_runtime_provider_peer_and_rollout",
        "validate_implementation_and_interoperability_separately"
      ],
      "source_ids": [
        "S204",
        "LOCAL026"
      ]
    }

## Source editions and scope

<a id="source-1"></a>

### <code>F06</code>

    {
      "id": "F06",
      "edition": "RFC10024 August2026; RFC9954 July2026",
      "clause": "10024:4.1-4.3,7.1-7.4; 9954:3.1-3.3,6",
      "uri": "https://www.rfc-editor.org/rfc/rfc10024.html",
      "local_source_id": null,
      "scope_limit": "specified components/combiner; implementation unverified; obsolete Kyber IDs are separate"
    }

<a id="source-2"></a>

### <code>F07</code>

    {
      "id": "F07",
      "edition": "RFC9846 July2026",
      "clause": "4.2; 4.3.7; 4.3.8; 4.5.1-4.5.3",
      "uri": "https://www.rfc-editor.org/rfc/rfc9846.html",
      "local_source_id": null,
      "scope_limit": "collector-reported full session only; suite is not KEX; CertificateVerify scheme absent"
    }

<a id="source-3"></a>

### <code>F08</code>

    {
      "id": "F08",
      "edition": "RFC5280 2008; RFC4055 2005",
      "clause": "5280:4.1.1.2,4.1.2.5,4.1.2.7; 4055:1.2,3",
      "uri": "https://www.rfc-editor.org/rfc/rfc5280.html",
      "local_source_id": null,
      "scope_limit": "parsed subject/signature/time facts; no offline trust, revocation or encoding re-verification"
    }

<a id="source-4"></a>

### <code>F12</code>

    {
      "id": "F12",
      "edition": "task066 adopted contract 2026-11-18",
      "clause": "technical dispatch and ingestion invariants; FACT-SOURCE-MAPPING F12",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "saved claims preserve assurance; digest does not attest collector truth"
    }

<a id="source-5"></a>

### <code>F14</code>

    {
      "id": "F14",
      "edition": "migration-decision-v1 1.0.0; adopted 2026-11-18",
      "clause": "P1-P8; readiness and action precedence",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "local advisory policy; conformance is not practitioner usefulness"
    }

<a id="source-6"></a>

### <code>LOCAL026</code>

    {
      "id": "LOCAL026",
      "edition": "local-assessment-1",
      "clause": "finite vocabulary and evidence limits",
      "uri": null,
      "local_source_id": "docs/architecture/LOCAL-ASSESSMENT-V1.md",
      "scope_limit": "advisory facts; no deployment or cryptographic verification"
    }

<a id="source-7"></a>

### <code>NIST&#45;PQ&#45;FAQ</code>

    {
      "id": "NIST-PQ-FAQ",
      "edition": "task056 pinned FAQ snapshot",
      "clause": "quantum public-key and generic-search models",
      "uri": "https://csrc.nist.gov/projects/post-quantum-cryptography/faqs",
      "local_source_id": null,
      "scope_limit": "not quantum arrival, probability or operational security bits"
    }

<a id="source-8"></a>

### <code>S203</code>

    {
      "id": "S203",
      "edition": "FIPS203 Final 2024-08-13",
      "clause": "1.1; 3.1; 7; 8 Table2",
      "uri": "https://csrc.nist.gov/pubs/fips/203/final",
      "local_source_id": null,
      "scope_limit": "KEM parameter names/purpose only"
    }

<a id="source-9"></a>

### <code>S204</code>

    {
      "id": "S204",
      "edition": "FIPS204 Final 2024-08-13",
      "clause": "1.1; 4 Table1",
      "uri": "https://csrc.nist.gov/pubs/fips/204/final",
      "local_source_id": null,
      "scope_limit": "pure signature names/purpose; no prehash equivalence"
    }

## Input, versions and coverage limits

    {
      "format": "tls-v1",
      "sha256": "7ef9154af5e9807dbeaa7106dc073c45ceaa0d5fba3e821c439d631036fee240",
      "schema_version": 1,
      "collection_time": "2026-11-17T00:00:00Z",
      "attempt_id": 1,
      "protocol": "tls"
    }

    {
      "format": "pqc-scan.migration-context",
      "version": 1,
      "sha256": "3d7739f819baf774a4519b09b2e6320f2334564dcbea193c0ebaef92169a20f6"
    }

    {
      "id": "migration-facts-v1",
      "version": "1.0.0"
    }

    {
      "id": "migration-decision-v1",
      "version": "1.0.0"
    }

    {
      "id": "decision_table",
      "version": "1.0.0"
    }

    {
      "collection_outcome": "partial",
      "input_count": 1,
      "target_count": 3,
      "unknown_identity_or_parameters_targets": 1,
      "unresolved_target_count": 0,
      "limits": [
        "one_saved_attempt_not_endpoint_coverage"
      ]
    }

- This is one saved attempt, not endpoint-wide coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.
