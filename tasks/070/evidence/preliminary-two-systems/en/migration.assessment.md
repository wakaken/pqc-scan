# Migration decisions

Targets: 2 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>RSA</code> — Urgent

**Where:** <code>system&#45;b&#47;auth&#46;rs&#58;1</code> · <code>algorithm</code>

**Declared system:** <code>system&#45;a</code>

**Response:** Investigate

**Why:** Business-critical active or planned signing use has been declared.

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

[Evidence, claims and candidate details](#evidence-1)

## 2. <code>RSA</code> — Plan a response

**Where:** <code>system&#45;a&#47;auth&#46;rs&#58;1</code> · <code>algorithm</code>

**Declared system:** <code>system&#45;b</code>

**Response:** Investigate

**Why:** Active scoped facts support planning a response to the known concern.

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

[Evidence, claims and candidate details](#evidence-2)

## Evidence and scope

Ready means declared support, not a verified deployment. Candidates require separate parameter, encoding and interoperability validation. Monitor is limited to the stated role; outside scope is not a safety verdict.

<a id="evidence-1"></a>

### Evidence 1

Decision: <code>migration&#95;decision&#45;v1&#45;a13a31e84c1cb6fbdaab7091ecce8931f2c0bdaebc583dc2e244bc5ee3bf15c5</code>

Target: <code>migration&#95;target&#45;v1&#45;384b866a8a9b7b9727239252d160bad0bd6fda711b26fe088c952e2e0ceeaf7d</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;084dc0d2d241642a0109dec9d764cf216f7f585b4f537abf777525b81ed8483b</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;74b10ebedafb1906e69f08d266b3ed4fb37763fd6c0e4c82fd0bff573a02f548</code>](#claim-5), [<code>migration&#95;claim&#45;v1&#45;95c3c16ae97b9c1d5149b80d3e883979db7ded5021a6244403f275eaa7f60517</code>](#claim-7), [<code>migration&#95;claim&#45;v1&#45;c319572a55500ade79c402969f7db3bc3c70e3adccdb3edc12a450baa42ade5e</code>](#claim-8)

Proposed candidates: [<code>migration&#95;candidate&#45;v1&#45;2c53efdcac9c0b8036d19728ed16dbdb6e0f96efa880ff53c3e1e8b6cb210881</code>](#candidate-1)

Sources: [<code>F12</code>](#source-1), [<code>F14</code>](#source-3), [<code>LOCAL026</code>](#source-4), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-384b866a8a9b7b9727239252d160bad0bd6fda711b26fe088c952e2e0ceeaf7d",
      "source_ref": {
        "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7",
        "observation_id": "observation-v1-08264b952b8ae33a085393934b509d1ef2160387e28a2339c4c8ecff01550be8",
        "root_asset_id": "asset-v1-e30d0455518fa3502bcd153e2d7d70096c2ac56e9d275847e8c1488261964b33",
        "role": "algorithm",
        "path": "system-b/auth.rs",
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
      "critical_use_declared",
      "unknown_parameter_encoding"
    ]

<a id="evidence-2"></a>

### Evidence 2

Decision: <code>migration&#95;decision&#45;v1&#45;742f70ed1829883b0b93c67ddd7eb8a92df0f9560a6d341c89c88438af0e96b7</code>

Target: <code>migration&#95;target&#45;v1&#45;ae577ba87f4660ddb0c2d95b7d14dea300b9050d2cc7e560c804421e854bf622</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;0a6d7bbd4fd099f812c42393e9fdd8fe8d876bae174f9fe29bc754e70d8eb3ed</code>](#claim-2), [<code>migration&#95;claim&#45;v1&#45;4dd8de68a4b95ad71a2778c20ef5d180467e9946cea5af4866e6f6e506d84fd2</code>](#claim-3), [<code>migration&#95;claim&#45;v1&#45;7391d9522b9b50e2aa00b378d6843bdd72263a70004e5603a658abf916ab15d5</code>](#claim-4), [<code>migration&#95;claim&#45;v1&#45;88ee3c5f8c8f8fd09a81ecfdc62f2a8c01ae71fb2b6d7121a8cf33fc8ff06865</code>](#claim-6), [<code>migration&#95;claim&#45;v1&#45;ff62d040f2d7a2b163baabf5fbad9d4d5dda74e67fd3141fadce68e69b8d51a1</code>](#claim-9)

Proposed candidates: [<code>migration&#95;candidate&#45;v1&#45;1797317ec15d8f16fe14ccea8a5cf1ac8f0c0d7e01e3c892b972394bbe5859b6</code>](#candidate-2)

Sources: [<code>F12</code>](#source-1), [<code>F13</code>](#source-2), [<code>F14</code>](#source-3), [<code>LOCAL026</code>](#source-4), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-ae577ba87f4660ddb0c2d95b7d14dea300b9050d2cc7e560c804421e854bf622",
      "source_ref": {
        "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7",
        "observation_id": "observation-v1-b75202b15a37ec57748d93b99e1538e2c70f9b0a81bb14f7b908c113e0bae108",
        "root_asset_id": "asset-v1-e30d0455518fa3502bcd153e2d7d70096c2ac56e9d275847e8c1488261964b33",
        "role": "algorithm",
        "path": "system-a/auth.rs",
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

### Claim: Actual or planned use

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;active&quot;</code>

    {
      "id": "migration_claim-v1-084dc0d2d241642a0109dec9d764cf216f7f585b4f537abf777525b81ed8483b",
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
      "id": "migration_claim-v1-0a6d7bbd4fd099f812c42393e9fdd8fe8d876bae174f9fe29bc754e70d8eb3ed",
      "context_pointer": "/bindings/1/claims/applicability",
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
      "id": "migration_claim-v1-4dd8de68a4b95ad71a2778c20ef5d180467e9946cea5af4866e6f6e506d84fd2",
      "context_pointer": "/bindings/1/claims/use_case",
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

### Claim: Actual or planned use

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;active&quot;</code>

    {
      "id": "migration_claim-v1-7391d9522b9b50e2aa00b378d6843bdd72263a70004e5603a658abf916ab15d5",
      "context_pointer": "/bindings/1/claims/usage_state",
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

<a id="claim-5"></a>

### Claim: Applicability

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;applicable&quot;</code>

    {
      "id": "migration_claim-v1-74b10ebedafb1906e69f08d266b3ed4fb37763fd6c0e4c82fd0bff573a02f548",
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

<a id="claim-6"></a>

### Claim: Business criticality

Operator claim (unverified) · Active declared interval

Declared value: <code>false</code>

    {
      "id": "migration_claim-v1-88ee3c5f8c8f8fd09a81ecfdc62f2a8c01ae71fb2b6d7121a8cf33fc8ff06865",
      "context_pointer": "/bindings/1/claims/business_critical",
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

<a id="claim-7"></a>

### Claim: Required purpose

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;signature&quot;</code>

    {
      "id": "migration_claim-v1-95c3c16ae97b9c1d5149b80d3e883979db7ded5021a6244403f275eaa7f60517",
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

<a id="claim-8"></a>

### Claim: Business criticality

Operator claim (unverified) · Active declared interval

Declared value: <code>true</code>

    {
      "id": "migration_claim-v1-c319572a55500ade79c402969f7db3bc3c70e3adccdb3edc12a450baa42ade5e",
      "context_pointer": "/bindings/0/claims/business_critical",
      "field": "business_critical",
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

<a id="claim-9"></a>

### Claim: Scoped deadline

Operator claim (unverified) · Active declared interval

Declared value: <code>&#123;&quot;kind&quot;&#58;&quot;none&quot;&#125;</code>

    {
      "id": "migration_claim-v1-ff62d040f2d7a2b163baabf5fbad9d4d5dda74e67fd3141fadce68e69b8d51a1",
      "context_pointer": "/bindings/1/claims/deadline",
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
      "id": "migration_candidate-v1-2c53efdcac9c0b8036d19728ed16dbdb6e0f96efa880ff53c3e1e8b6cb210881",
      "target_id": "migration_target-v1-384b866a8a9b7b9727239252d160bad0bd6fda711b26fe088c952e2e0ceeaf7d",
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

<a id="candidate-2"></a>

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
      "id": "migration_candidate-v1-1797317ec15d8f16fe14ccea8a5cf1ac8f0c0d7e01e3c892b972394bbe5859b6",
      "target_id": "migration_target-v1-ae577ba87f4660ddb0c2d95b7d14dea300b9050d2cc7e560c804421e854bf622",
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

## Input, versions and coverage limits

    {
      "format": "inventory-v1",
      "sha256": "3fedf0e044766d249d6ed9632c1cd119c5c5f3a51ef8114466a1ebb10d70fcf5",
      "schema_version": 1,
      "collection_time": "2026-11-17T00:00:00Z",
      "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7"
    }

    {
      "format": "pqc-scan.migration-context",
      "version": 1,
      "sha256": "5c19886cf84206871143d0b3792befe0587f087614cdaede11a921ef3debe137"
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
      "target_count": 2,
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
