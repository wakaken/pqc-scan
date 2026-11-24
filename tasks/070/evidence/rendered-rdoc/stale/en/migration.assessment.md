# Migration decisions

Targets: 1 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>RSA</code> — Urgent

**Where:** <code>app&#46;txt&#58;1</code> · <code>algorithm</code>

**Declared system:** <code>system&#45;a</code>

**Response:** Investigate

**Why:** A stale due-deadline claim requires urgent investigation and refresh.

**Next check:**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

**Information:** Information missing

**Readiness:** Readiness unknown

**Missing inputs:**

- Exact parameter support
- Peer support
- Protocol support
- Provider support
- Runtime support

**Additional context:**

- The exact parameter encoding is unknown.
- A used claim is not currently active; refresh it.

[Evidence, claims and candidate details](#evidence-1)

## Evidence and scope

Ready means declared support, not a verified deployment. Candidates require separate parameter, encoding and interoperability validation. Monitor is limited to the stated role; outside scope is not a safety verdict.

<a id="evidence-1"></a>

### Evidence 1

Decision: <code>migration&#95;decision&#45;v1&#45;0b1785e6769b6c6a39742cff0e5f114b58b26928bf8367236aca7c712f916271</code>

Target: <code>migration&#95;target&#45;v1&#45;114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;23c219c47ed0ecbb5a97f9de325be65de36f34d1173e37ff617087912bc96ba7</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;793b515e0e86b0a08526b1e458d2f8ab42c950ed9af4651c3d586e9253318984</code>](#claim-2), [<code>migration&#95;claim&#45;v1&#45;d1d205efd9c9e4eda48cfd275a99a9ad88652a99765dd4f2aa24fd3af80da01b</code>](#claim-3), [<code>migration&#95;claim&#45;v1&#45;e418c8992ceac49483f1d17ef777a37dbddf9157ca5ebf20df0d29716ea2dc9b</code>](#claim-4)

Proposed candidates: [<code>migration&#95;candidate&#45;v1&#45;a4c9deae9c9a62462567334450d70afc3388ca15b342a78ceb06ddd3b8654a30</code>](#candidate-1)

Sources: [<code>F12</code>](#source-1), [<code>F13</code>](#source-2), [<code>F14</code>](#source-3), [<code>LOCAL026</code>](#source-4), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5), [<code>context&#58;org&#45;deadline</code>](#source-7)

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
      "stale_urgent_claim_requires_refresh",
      "unknown_parameter_encoding",
      "stale_claim"
    ]

## Applied claim provenance

Operator and external-review claims are unverified declarations. Freshness describes their stated interval, not independent confirmation.

<a id="claim-1"></a>

### Claim: Actual or planned use

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;active&quot;</code>

    {
      "id": "migration_claim-v1-23c219c47ed0ecbb5a97f9de325be65de36f34d1173e37ff617087912bc96ba7",
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

<a id="claim-2"></a>

### Claim: Applicability

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;applicable&quot;</code>

    {
      "id": "migration_claim-v1-793b515e0e86b0a08526b1e458d2f8ab42c950ed9af4651c3d586e9253318984",
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

<a id="claim-3"></a>

### Claim: Required purpose

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;signature&quot;</code>

    {
      "id": "migration_claim-v1-d1d205efd9c9e4eda48cfd275a99a9ad88652a99765dd4f2aa24fd3af80da01b",
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

<a id="claim-4"></a>

### Claim: Scoped deadline

External-review claim (unverified) · Expired declaration

Declared value: <code>&#123;&quot;at&quot;&#58;&quot;2026&#45;11&#45;16T00&#58;00&#58;00Z&quot;&#44;&quot;kind&quot;&#58;&quot;at&quot;&#44;&quot;reference&#95;id&quot;&#58;&quot;org&#45;deadline&quot;&#44;&quot;scope&#95;reason&quot;&#58;&quot;organization&#95;policy&quot;&#125;</code>

    {
      "id": "migration_claim-v1-e418c8992ceac49483f1d17ef777a37dbddf9157ca5ebf20df0d29716ea2dc9b",
      "context_pointer": "/bindings/0/claims/deadline",
      "field": "deadline",
      "state": "known",
      "value": {
        "at": "2026-11-16T00:00:00Z",
        "kind": "at",
        "reference_id": "org-deadline",
        "scope_reason": "organization_policy"
      },
      "provenance": {
        "source_kind": "external_review_claim",
        "actor_id": "owner",
        "asserted_at": "2026-11-16T00:00:00Z",
        "valid_until": "2026-11-17T00:00:00Z"
      },
      "freshness": "stale"
    }

## Proposed candidates and validation

<a id="candidate-1"></a>

### <code>ML&#45;DSA</code>

Sources: [<code>S204</code>](#source-6), [<code>LOCAL026</code>](#source-4)

- Runtime support: Unknown
- Provider support: Unknown
- Exact parameter support: Unknown
- Peer support: Unknown
- Protocol support: Unknown

**Validation checks:**

- Confirm actual use and the required cryptographic purpose.
- Verify exact parameters and protocol encoding before rollout.
- Test runtime, provider, peers and rollout.
- Validate the implementation and interoperability separately.

Exact candidate record:

    {
      "id": "migration_candidate-v1-a4c9deae9c9a62462567334450d70afc3388ca15b342a78ceb06ddd3b8654a30",
      "target_id": "migration_target-v1-114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad",
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

<a id="source-7"></a>

### <code>context&#58;org&#45;deadline</code>

    {
      "id": "context:org-deadline",
      "edition": "v1",
      "clause": "3.1",
      "uri": "https://synthetic.invalid/never-fetch",
      "local_source_id": "org-deadline",
      "scope_limit": "operator_declared_reference_not_independently_verified"
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
      "sha256": "645e8432263a0f4abd94036e77bc761186162f361d717ff542c11d307ef8d10e"
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
