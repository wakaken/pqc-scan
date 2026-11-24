# Migration decisions

Targets: 1 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>RSA</code> — Outside declared scope

**Where:** <code>app&#46;txt&#58;1</code> · <code>algorithm</code>

**Declared system:** <code>system&#45;a</code>

**Response:** No action within the declared scope

**Why:** The active declaration excludes this target; known technical concerns remain.

**Next check:**

- Reassess when the facts, scope or claims change.

**Information:** Required information available

**Readiness:** Migration readiness not applicable

**Additional context:**

- The exact parameter encoding is unknown.

[Evidence, claims and candidate details](#evidence-1)

## Evidence and scope

Ready means declared support, not a verified deployment. Candidates require separate parameter, encoding and interoperability validation. Monitor is limited to the stated role; outside scope is not a safety verdict.

<a id="evidence-1"></a>

### Evidence 1

Decision: <code>migration&#95;decision&#45;v1&#45;067843b7b1e9bd1356eecac18b465326fba784da7ef23f871694b10f16ca62f8</code>

Target: <code>migration&#95;target&#45;v1&#45;114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad</code>

Applied claims: [<code>migration&#95;claim&#45;v1&#45;3b855b48ff4d804e2b443b56a4c5514afefb71746d3612436802f82bdd2afe9c</code>](#claim-1)

Proposed candidates: 

Sources: [<code>F12</code>](#source-1), [<code>F14</code>](#source-2), [<code>LOCAL026</code>](#source-3), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

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
      "outside_declared_scope",
      "unknown_parameter_encoding"
    ]

## Applied claim provenance

Operator and external-review claims are unverified declarations. Freshness describes their stated interval, not independent confirmation.

<a id="claim-1"></a>

### Claim: Applicability

Operator claim (unverified) · Active declared interval

Declared value: <code>&quot;not&#95;applicable&quot;</code>

    {
      "id": "migration_claim-v1-3b855b48ff4d804e2b443b56a4c5514afefb71746d3612436802f82bdd2afe9c",
      "context_pointer": "/bindings/0/claims/applicability",
      "field": "applicability",
      "state": "known",
      "value": "not_applicable",
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

### <code>NIST&#45;PQ&#45;FAQ</code>

    {
      "id": "NIST-PQ-FAQ",
      "edition": "task056 pinned FAQ snapshot",
      "clause": "quantum public-key and generic-search models",
      "uri": "https://csrc.nist.gov/projects/post-quantum-cryptography/faqs",
      "local_source_id": null,
      "scope_limit": "not quantum arrival, probability or operational security bits"
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
      "sha256": "8f0e48e1276a39dda35441c921a4784fe6ab131b9b8f9d2dfb1d563b25d0c8ac"
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
