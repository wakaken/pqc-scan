# Migration decisions

Targets: 3 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>unknown</code> — Urgency undetermined

**Where:** <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> · <code>negotiated&#95;key&#95;establishment</code>

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
- Exact primitive identity
- Exact parameters
- Actual or planned use
- Required purpose

**Additional context:**

- Collection is incomplete for this target.
- Verification was not completed.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

[Evidence, claims and candidate details](#evidence-1)

## 2. <code>RSA&#45;MD5</code> — Urgent

**Where:** <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> · <code>certificate&#95;signature</code>

**Response:** Investigate

**Why:** The saved facts show a relevant classical cryptographic concern.

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

Decision: <code>migration&#95;decision&#45;v1&#45;06dd1f8c8cd72f92d491fae6496d501cf5cea0a40444c0bb01a439b893f0dc60</code>

Target: <code>migration&#95;target&#45;v1&#45;7dc4de55d949e725fee8cca5d0130e03814c88fddf71b8b5f099a30e993bb2d3</code>

Applied claims: 

Proposed candidates: 

Sources: [<code>F14</code>](#source-4), [<code>LOCAL026</code>](#source-5)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-7dc4de55d949e725fee8cca5d0130e03814c88fddf71b8b5f099a30e993bb2d3",
      "source_ref": {
        "role": "negotiated_key_establishment",
        "attempt_id": 1,
        "detail_path": "/details/negotiated/key_exchange_group"
      },
      "technical": {
        "identifier": "unknown",
        "construction": "unknown",
        "standardization": "unknown",
        "quantum": "unknown",
        "purpose": "unknown",
        "parameters_known": false,
        "classical_concerns": [],
        "attack_models": [],
        "components": [],
        "combiner_assessment": null,
        "source_ids": [
          "LOCAL026"
        ],
        "encoding_assurance": "unknown",
        "protocol_ref": "F07",
        "role": "negotiated_key_establishment"
      },
      "observation": {
        "method": "tls_saved_observation",
        "assurance": "unverified_or_incomplete",
        "usage": "unknown",
        "collection_outcome": "failed"
      },
      "limitations": [
        "unknown_primitive",
        "unknown_parameter_encoding",
        "collection_incomplete",
        "verification_not_completed"
      ]
    }

    [
      "insufficient_information",
      "collection_incomplete",
      "verification_not_completed",
      "unknown_primitive",
      "unknown_parameter_encoding",
      "unknown_binding"
    ]

<a id="evidence-2"></a>

### Evidence 2

Decision: <code>migration&#95;decision&#45;v1&#45;57af0a250c2273ad6a6333ac09de78dde50df961c5d5f3640b743b42dfb84a96</code>

Target: <code>migration&#95;target&#45;v1&#45;e872792348a59baca96dd26560dbbb7d0e9bb76943b882703904f1ee984c7604</code>

Applied claims: 

Proposed candidates: [<code>migration&#95;candidate&#45;v1&#45;e7ee1dfcfe4894544c7ed2b841c13d3d40e324c7a0acf51aa8f16afd45759028</code>](#candidate-1)

Sources: [<code>F08</code>](#source-2), [<code>F12</code>](#source-3), [<code>F14</code>](#source-4), [<code>LOCAL026</code>](#source-5), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6), [<code>R6151</code>](#source-7)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-e872792348a59baca96dd26560dbbb7d0e9bb76943b882703904f1ee984c7604",
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
        "identifier": "RSA-MD5",
        "construction": "classical_public_key",
        "standardization": "classical_standard",
        "quantum": "vulnerable",
        "purpose": "signature",
        "parameters_known": false,
        "classical_concerns": [
          "signature_uses_weak_collision_digest"
        ],
        "attack_models": [],
        "components": [],
        "combiner_assessment": null,
        "source_ids": [
          "F12",
          "LOCAL026",
          "NIST-PQ-FAQ",
          "R6151",
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
        "collection_outcome": "failed"
      },
      "limitations": [
        "unknown_parameter_encoding",
        "collection_incomplete"
      ]
    }

    [
      "known_classical_concern",
      "collection_incomplete",
      "unknown_parameter_encoding",
      "unknown_binding"
    ]

<a id="evidence-3"></a>

### Evidence 3

Decision: <code>migration&#95;decision&#45;v1&#45;fcda13859d2e6064f6a32e2bb3337103a31f4c0954c1f292f319db3aaaf83e8e</code>

Target: <code>migration&#95;target&#45;v1&#45;4696c757de0244243d537a84a174637883f59cf7f6c02ed30a8c991bee6cad83</code>

Applied claims: 

Proposed candidates: 

Sources: [<code>F08</code>](#source-2), [<code>F12</code>](#source-3), [<code>F14</code>](#source-4), [<code>LOCAL026</code>](#source-5), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-4696c757de0244243d537a84a174637883f59cf7f6c02ed30a8c991bee6cad83",
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
        "collection_outcome": "failed"
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

## Proposed candidates and validation

<a id="candidate-1"></a>

### <code>ML&#45;DSA</code>

Sources: [<code>S204</code>](#source-8), [<code>LOCAL026</code>](#source-5)

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
      "id": "migration_candidate-v1-e7ee1dfcfe4894544c7ed2b841c13d3d40e324c7a0acf51aa8f16afd45759028",
      "target_id": "migration_target-v1-e872792348a59baca96dd26560dbbb7d0e9bb76943b882703904f1ee984c7604",
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

### <code>F07</code>

    {
      "id": "F07",
      "edition": "RFC9846 July2026",
      "clause": "4.2; 4.3.7; 4.3.8; 4.5.1-4.5.3",
      "uri": "https://www.rfc-editor.org/rfc/rfc9846.html",
      "local_source_id": null,
      "scope_limit": "collector-reported full session only; suite is not KEX; CertificateVerify scheme absent"
    }

<a id="source-2"></a>

### <code>F08</code>

    {
      "id": "F08",
      "edition": "RFC5280 2008; RFC4055 2005",
      "clause": "5280:4.1.1.2,4.1.2.5,4.1.2.7; 4055:1.2,3",
      "uri": "https://www.rfc-editor.org/rfc/rfc5280.html",
      "local_source_id": null,
      "scope_limit": "parsed subject/signature/time facts; no offline trust, revocation or encoding re-verification"
    }

<a id="source-3"></a>

### <code>F12</code>

    {
      "id": "F12",
      "edition": "task066 adopted contract 2026-11-18",
      "clause": "technical dispatch and ingestion invariants; FACT-SOURCE-MAPPING F12",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "saved claims preserve assurance; digest does not attest collector truth"
    }

<a id="source-4"></a>

### <code>F14</code>

    {
      "id": "F14",
      "edition": "migration-decision-v1 1.0.0; adopted 2026-11-18",
      "clause": "P1-P8; readiness and action precedence",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "local advisory policy; conformance is not practitioner usefulness"
    }

<a id="source-5"></a>

### <code>LOCAL026</code>

    {
      "id": "LOCAL026",
      "edition": "local-assessment-1",
      "clause": "finite vocabulary and evidence limits",
      "uri": null,
      "local_source_id": "docs/architecture/LOCAL-ASSESSMENT-V1.md",
      "scope_limit": "advisory facts; no deployment or cryptographic verification"
    }

<a id="source-6"></a>

### <code>NIST&#45;PQ&#45;FAQ</code>

    {
      "id": "NIST-PQ-FAQ",
      "edition": "task056 pinned FAQ snapshot",
      "clause": "quantum public-key and generic-search models",
      "uri": "https://csrc.nist.gov/projects/post-quantum-cryptography/faqs",
      "local_source_id": null,
      "scope_limit": "not quantum arrival, probability or operational security bits"
    }

<a id="source-7"></a>

### <code>R6151</code>

    {
      "id": "R6151",
      "edition": "RFC6151 2011",
      "clause": "2",
      "uri": "https://www.rfc-editor.org/rfc/rfc6151.html",
      "local_source_id": null,
      "scope_limit": "MD5 collision-sensitive use; not every hash use"
    }

<a id="source-8"></a>

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
      "sha256": "538e5773018c4079f6209d78ccad54416ee6c61098bd2120f2d2fc583ab05b4c",
      "schema_version": 1,
      "collection_time": "2026-11-17T00:00:00Z",
      "attempt_id": 1,
      "protocol": "tls"
    }

    null

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
      "collection_outcome": "failed",
      "input_count": 1,
      "target_count": 3,
      "unknown_identity_or_parameters_targets": 2,
      "unresolved_target_count": 0,
      "limits": [
        "one_saved_attempt_not_endpoint_coverage"
      ]
    }

- This is one saved attempt, not endpoint-wide coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.
