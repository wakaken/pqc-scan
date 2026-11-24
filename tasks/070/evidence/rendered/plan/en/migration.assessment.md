# Migration decisions

Targets: 1 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>RSA</code> — Plan a response

**Where:** <code>app&#46;txt&#58;1</code> · <code>algorithm</code>

**Declared system:** <code>system&#45;a</code>

**Response:** Prepare the migration plan

**Why:** Active scoped facts support planning a response to the known concern.

**Next check:**

- Verify exact parameters and protocol encoding before rollout.
- Test runtime, provider, peers and rollout.

**Information:** Required information available

**Readiness:** Prerequisites met by active declarations

**Additional context:**

- The exact parameter encoding is unknown.

[Evidence, claims and candidate details](#evidence-1)

## Evidence and scope

Ready means declared support, not a verified deployment. Candidates require separate parameter, encoding and interoperability validation. Monitor is limited to the stated role; outside scope is not a safety verdict.

<a id="evidence-1"></a>

### Evidence 1

Decision: <code>migration&#95;decision&#45;v1&#45;fd476c8f56f7b7264593f824c307f392231a1189b849cd5d9fee4e6f2da4f2e7</code>

Target: <code>migration&#95;target&#45;v1&#45;114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;0dafc0cc26195d6c15cb852db55dd2426645271554b4d37cfdb057a084d342a0</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;18c3be2eb70e3c6ffe9984ef4933e14911ac634aa2eba95aa61b94374f9740f3</code>](#claim-2), [<code>migration&#95;claim&#45;v1&#45;1c0be445f3f159223c0af22798301a8828b8c5525d90485ad7090819e725a013</code>](#claim-3), [<code>migration&#95;claim&#45;v1&#45;2da1ed872e36c77ef022605e4040a20d70aaed8fdc2898c6f32c00b968ca789a</code>](#claim-4), [<code>migration&#95;claim&#45;v1&#45;3e92a8778f14b28503008d623b446365c6abdfdef0368290b0c220a44d1c0768</code>](#claim-5), [<code>migration&#95;claim&#45;v1&#45;49e1656daca2d1933157978f10aa8a33f08f75367f7393329f890c4a349599fb</code>](#claim-6), [<code>migration&#95;claim&#45;v1&#45;5251de489fe70522f3da44c65fe15089305a11da9e7c82a8044d823ee33f9ddf</code>](#claim-7), [<code>migration&#95;claim&#45;v1&#45;d6c300a3f073c520c3901fb3f219023aa06b7c4a3e49dc443ace38b257bdbed6</code>](#claim-8), [<code>migration&#95;claim&#45;v1&#45;e956d653dc16e21ae40385a0ec72f19966c8db8ae8a444ad0a5201ec6aee94a0</code>](#claim-9), [<code>migration&#95;claim&#45;v1&#45;e98e600e3b77ca8936ba766224a8ceb0a07c379ccf3b36bcaa61794a7346fead</code>](#claim-10)

Proposed candidates: [<code>migration&#95;candidate&#45;v1&#45;678e6ce8d7c3d2af25fcd1cdbd2f2daf54046ff91ab664680ef3a47b2b818a8d</code>](#candidate-1)

Sources: [<code>F12</code>](#source-1), [<code>F13</code>](#source-2), [<code>F14</code>](#source-3), [<code>LOCAL026</code>](#source-4), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad",
      "source_ref": {
        "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7",
        "observation_id": "observation-v1-b75202b15a37ec57748d93b99e1538e2c70f9b0a81bb14f7b908c113e0bae108",
        "root_asset_id": "asset-v1-e30d0455518fa3502bcd153e2d7d70096c2ac56e9d275847e8c1488261964b33",
        "role": "algorithm",
        "path": "app.txt",
        "line": 1
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
          "NIST-PQ-FAQ"
        ],
        "encoding_assurance": "unknown",
        "protocol_ref": null,
        "role": "algorithm"
      },
      "observation": {
        "method": "ast_pattern",
        "assurance": "syntactic",
        "usage": "referenced",
        "collection_outcome": "complete"
      },
      "limitations": [
        "unknown_parameter_encoding"
      ]
    }

    [
      "known_concern_plan",
      "unknown_parameter_encoding"
    ]

## Applied claim provenance

Operator and external-review claims are unverified declarations. Freshness describes their stated interval, not independent confirmation.

<a id="claim-1"></a>

### Claim: Protocol support

Operator claim (unverified) · Active declared interval

Declared value: <code>true</code>

    {
      "id": "migration_claim-v1-0dafc0cc26195d6c15cb852db55dd2426645271554b4d37cfdb057a084d342a0",
      "context_pointer": "/bindings/0/claims/support/protocol",
      "field": "support.protocol",
      "state": "known",
      "value": true,
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-2"></a>

### Claim: Scoped deadline

Operator claim (unverified) · Active declared interval

Declared value: <code>&#123;&quot;kind&quot;&#58;&quot;none&quot;&#125;</code>

    {
      "id": "migration_claim-v1-18c3be2eb70e3c6ffe9984ef4933e14911ac634aa2eba95aa61b94374f9740f3",
      "context_pointer": "/bindings/0/claims/deadline",
      "field": "deadline",
      "state": "known",
      "value": {
        "kind": "none"
      },
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-3"></a>

### Claim: Peer support

Operator claim (unverified) · Active declared interval

Declared value: <code>true</code>

    {
      "id": "migration_claim-v1-1c0be445f3f159223c0af22798301a8828b8c5525d90485ad7090819e725a013",
      "context_pointer": "/bindings/0/claims/support/peer",
      "field": "support.peer",
      "state": "known",
      "value": true,
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-4"></a>

### Claim: Applicability

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;applicable&quot;</code>

    {
      "id": "migration_claim-v1-2da1ed872e36c77ef022605e4040a20d70aaed8fdc2898c6f32c00b968ca789a",
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

<a id="claim-5"></a>

### Claim: Runtime support

Operator claim (unverified) · Active declared interval

Declared value: <code>true</code>

    {
      "id": "migration_claim-v1-3e92a8778f14b28503008d623b446365c6abdfdef0368290b0c220a44d1c0768",
      "context_pointer": "/bindings/0/claims/support/runtime",
      "field": "support.runtime",
      "state": "known",
      "value": true,
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-6"></a>

### Claim: Provider support

Operator claim (unverified) · Active declared interval

Declared value: <code>true</code>

    {
      "id": "migration_claim-v1-49e1656daca2d1933157978f10aa8a33f08f75367f7393329f890c4a349599fb",
      "context_pointer": "/bindings/0/claims/support/provider",
      "field": "support.provider",
      "state": "known",
      "value": true,
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-7"></a>

### Claim: Actual or planned use

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;active&quot;</code>

    {
      "id": "migration_claim-v1-5251de489fe70522f3da44c65fe15089305a11da9e7c82a8044d823ee33f9ddf",
      "context_pointer": "/bindings/0/claims/usage_state",
      "field": "usage_state",
      "state": "known",
      "value": "active",
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-8"></a>

### Claim: Required purpose

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;signature&quot;</code>

    {
      "id": "migration_claim-v1-d6c300a3f073c520c3901fb3f219023aa06b7c4a3e49dc443ace38b257bdbed6",
      "context_pointer": "/bindings/0/claims/use_case",
      "field": "use_case",
      "state": "known",
      "value": "signature",
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-9"></a>

### Claim: Exact parameter support

Operator claim (unverified) · Active declared interval

Declared value: <code>true</code>

    {
      "id": "migration_claim-v1-e956d653dc16e21ae40385a0ec72f19966c8db8ae8a444ad0a5201ec6aee94a0",
      "context_pointer": "/bindings/0/claims/support/parameters",
      "field": "support.parameters",
      "state": "known",
      "value": true,
      "provenance": {
        "source_kind": "operator_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-17T00:00:00Z",
        "valid_until": "2027-11-17T00:00:00Z"
      },
      "freshness": "active"
    }

<a id="claim-10"></a>

### Claim: Business criticality

Operator claim (unverified) · Active declared interval

Declared value: <code>false</code>

    {
      "id": "migration_claim-v1-e98e600e3b77ca8936ba766224a8ceb0a07c379ccf3b36bcaa61794a7346fead",
      "context_pointer": "/bindings/0/claims/business_critical",
      "field": "business_critical",
      "state": "known",
      "value": false,
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

Sources: [<code>S204</code>](#source-6), [<code>LOCAL026</code>](#source-4)

- Runtime support: Declared supported
- Provider support: Declared supported
- Exact parameter support: Declared supported
- Peer support: Declared supported
- Protocol support: Declared supported

**Validation checks:**

- Confirm actual use and the required cryptographic purpose.
- Verify exact parameters and protocol encoding before rollout.
- Test runtime, provider, peers and rollout.
- Validate the implementation and interoperability separately.

Exact candidate record:

    {
      "id": "migration_candidate-v1-678e6ce8d7c3d2af25fcd1cdbd2f2daf54046ff91ab664680ef3a47b2b818a8d",
      "target_id": "migration_target-v1-114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad",
      "identifier": "ML-DSA",
      "purpose": "signature",
      "status": "candidate_primitive_integration_unverified",
      "parameter_selection": "explicit_parameter_selection_required",
      "prerequisites": [
        {
          "name": "runtime",
          "claimed_support": true,
          "applied_claim_ids": [
            "migration_claim-v1-3e92a8778f14b28503008d623b446365c6abdfdef0368290b0c220a44d1c0768"
          ]
        },
        {
          "name": "provider",
          "claimed_support": true,
          "applied_claim_ids": [
            "migration_claim-v1-49e1656daca2d1933157978f10aa8a33f08f75367f7393329f890c4a349599fb"
          ]
        },
        {
          "name": "parameters",
          "claimed_support": true,
          "applied_claim_ids": [
            "migration_claim-v1-e956d653dc16e21ae40385a0ec72f19966c8db8ae8a444ad0a5201ec6aee94a0"
          ]
        },
        {
          "name": "peer",
          "claimed_support": true,
          "applied_claim_ids": [
            "migration_claim-v1-1c0be445f3f159223c0af22798301a8828b8c5525d90485ad7090819e725a013"
          ]
        },
        {
          "name": "protocol",
          "claimed_support": true,
          "applied_claim_ids": [
            "migration_claim-v1-0dafc0cc26195d6c15cb852db55dd2426645271554b4d37cfdb057a084d342a0"
          ]
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

### <code>F12</code>

    {
      "id": "F12",
      "edition": "task066 adopted contract 2026-11-18",
      "clause": "technical dispatch and ingestion invariants; FACT-SOURCE-MAPPING F12",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "saved claims preserve assurance; digest does not attest collector truth"
    }

<a id="source-2"></a>

### <code>F13</code>

    {
      "id": "F13",
      "edition": "task066 adopted time contract 2026-11-18",
      "clause": "time and policy semantics; FACT-SOURCE-MAPPING F13",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "scoped declared lifetimes/deadlines; no quantum forecast or universal deadline"
    }

<a id="source-3"></a>

### <code>F14</code>

    {
      "id": "F14",
      "edition": "migration-decision-v1 1.0.0; adopted 2026-11-18",
      "clause": "P1-P8; readiness and action precedence",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "local advisory policy; conformance is not practitioner usefulness"
    }

<a id="source-4"></a>

### <code>LOCAL026</code>

    {
      "id": "LOCAL026",
      "edition": "local-assessment-1",
      "clause": "finite vocabulary and evidence limits",
      "uri": null,
      "local_source_id": "docs/architecture/LOCAL-ASSESSMENT-V1.md",
      "scope_limit": "advisory facts; no deployment or cryptographic verification"
    }

<a id="source-5"></a>

### <code>NIST&#45;PQ&#45;FAQ</code>

    {
      "id": "NIST-PQ-FAQ",
      "edition": "task056 pinned FAQ snapshot",
      "clause": "quantum public-key and generic-search models",
      "uri": "https://csrc.nist.gov/projects/post-quantum-cryptography/faqs",
      "local_source_id": null,
      "scope_limit": "not quantum arrival, probability or operational security bits"
    }

<a id="source-6"></a>

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
      "format": "inventory-v1",
      "sha256": "ea072032ff581bb287e010de83d5c9be0f72debdce0962925dfa3c5dbf29f81f",
      "schema_version": 1,
      "collection_time": "2026-11-17T00:00:00Z",
      "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7"
    }

    {
      "format": "pqc-scan.migration-context",
      "version": 1,
      "sha256": "4e142e398bb4586f4dc5a0cb91b4cab6f015113b27a09cc20b45b4eb5dec9bc7"
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
      "collection_outcome": "complete",
      "input_count": 1,
      "target_count": 1,
      "unknown_identity_or_parameters_targets": 0,
      "unresolved_target_count": 0,
      "limits": [
        "inventory_not_exhaustive"
      ]
    }

- Inventory does not establish exhaustive coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.
