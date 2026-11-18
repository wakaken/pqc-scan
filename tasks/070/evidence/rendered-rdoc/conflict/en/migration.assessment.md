# Migration decisions

Targets: 1 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>ML&#45;KEM&#45;768</code> — Urgency undetermined

**Where:** <code>app&#46;txt&#58;1</code> · <code>algorithm</code>

**Declared system:** <code>system&#45;a</code>

**Response:** Investigate

**Why:** The available facts do not establish a supported urgency branch.

**Next check:**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

**Information:** Claims conflict with known facts

**Readiness:** Readiness unknown

**Missing inputs:**

- Exact parameter support
- Provider support
- Runtime support

**Additional context:**

- The exact parameter encoding is unknown.
- The declared purpose conflicts with the known primitive purpose.

[Evidence, claims and candidate details](#evidence-1)

## Evidence and scope

Ready means declared support, not a verified deployment. Candidates require separate parameter, encoding and interoperability validation. Monitor is limited to the stated role; outside scope is not a safety verdict.

<a id="evidence-1"></a>

### Evidence 1

Decision: <code>migration&#95;decision&#45;v1&#45;248594f8817705ea0878e59a5726d9c50bcfebfe45581e400761a76f79785763</code>

Target: <code>migration&#95;target&#45;v1&#45;f51bff5a334ec2a95a3b654c2f3e32977a4a6b84e03e657a9fd6d22eb3f0a720</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;8c3381dfeed4df0e97ece774247da6b9b87e4d6287169d93554031e3991e02bf</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;93d293e1c38bf21dd7f9c5072c5436c7e8b7b4a88727657323307c208920d22a</code>](#claim-2), [<code>migration&#95;claim&#45;v1&#45;a50e967b78162ee1ba19e1e562fe1466472fde656a3d8b3844bd722cea562060</code>](#claim-3)

Proposed candidates: 

Sources: [<code>F12</code>](#source-1), [<code>F14</code>](#source-2), [<code>LOCAL026</code>](#source-3), [<code>S203</code>](#source-4)

Saved observation, technical facts and fixed reason codes:

    {
      "id": "migration_target-v1-f51bff5a334ec2a95a3b654c2f3e32977a4a6b84e03e657a9fd6d22eb3f0a720",
      "source_ref": {
        "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7",
        "observation_id": "observation-v1-b75202b15a37ec57748d93b99e1538e2c70f9b0a81bb14f7b908c113e0bae108",
        "root_asset_id": "asset-v1-e2627b84b9da96375e00a8151dbb4faa63392fa08a348a78af6b3204156f7e42",
        "role": "algorithm",
        "path": "app.txt",
        "line": 1
      },
      "technical": {
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
      "insufficient_information",
      "unknown_parameter_encoding",
      "purpose_mismatch"
    ]

## Applied claim provenance

Operator and external-review claims are unverified declarations. Freshness describes their stated interval, not independent confirmation.

<a id="claim-1"></a>

### Claim: Required purpose

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;signature&quot;</code>

    {
      "id": "migration_claim-v1-8c3381dfeed4df0e97ece774247da6b9b87e4d6287169d93554031e3991e02bf",
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

<a id="claim-2"></a>

### Claim: Applicability

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;applicable&quot;</code>

    {
      "id": "migration_claim-v1-93d293e1c38bf21dd7f9c5072c5436c7e8b7b4a88727657323307c208920d22a",
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

### Claim: Actual or planned use

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;active&quot;</code>

    {
      "id": "migration_claim-v1-a50e967b78162ee1ba19e1e562fe1466472fde656a3d8b3844bd722cea562060",
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

## Proposed candidates and validation

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

### <code>F14</code>

    {
      "id": "F14",
      "edition": "migration-decision-v1 1.0.0; adopted 2026-11-18",
      "clause": "P1-P8; readiness and action precedence",
      "uri": null,
      "local_source_id": "docs/architecture/MIGRATION-ASSESSMENT-V1.md",
      "scope_limit": "local advisory policy; conformance is not practitioner usefulness"
    }

<a id="source-3"></a>

### <code>LOCAL026</code>

    {
      "id": "LOCAL026",
      "edition": "local-assessment-1",
      "clause": "finite vocabulary and evidence limits",
      "uri": null,
      "local_source_id": "docs/architecture/LOCAL-ASSESSMENT-V1.md",
      "scope_limit": "advisory facts; no deployment or cryptographic verification"
    }

<a id="source-4"></a>

### <code>S203</code>

    {
      "id": "S203",
      "edition": "FIPS203 Final 2024-08-13",
      "clause": "1.1; 3.1; 7; 8 Table2",
      "uri": "https://csrc.nist.gov/pubs/fips/203/final",
      "local_source_id": null,
      "scope_limit": "KEM parameter names/purpose only"
    }

## Input, versions and coverage limits

    {
      "format": "inventory-v1",
      "sha256": "4d1110b4d2902562feb682de7ed8ac675ab724045f5e11c6d5435da23685baef",
      "schema_version": 1,
      "collection_time": "2026-11-17T00:00:00Z",
      "run_id": "scan-v1-a9ff06a933b8f9ebe359562fad7005b81895235c5435bb91a62562566eb479b7"
    }

    {
      "format": "pqc-scan.migration-context",
      "version": 1,
      "sha256": "a0aeebe4df81e4f1ed7d0a68da2d23b6cdaa1c3491493579a6ad4e665e2d0aef"
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
