# 移行判断

対象数: 2 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>RSA</code> — 早急に対応

**場所:** <code>system&#45;b&#47;auth&#46;rs&#58;1</code> · <code>algorithm</code>

**申告されたシステム:** <code>system&#45;a</code>

**対応:** 調査

**理由:** 業務上重要な署名用途での利用中・利用予定が申告されています。

**次の確認:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

**情報の状態:** 不足情報あり

**移行準備:** 準備状況は不明

**不足情報:**

- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応

**補足:**

- 正確なパラメーターの符号化は不明です。

[根拠・申告・候補の詳細](#evidence-1)

## 2. <code>RSA</code> — 計画して対応

**場所:** <code>system&#45;a&#47;auth&#46;rs&#58;1</code> · <code>algorithm</code>

**申告されたシステム:** <code>system&#45;b</code>

**対応:** 調査

**理由:** 有効な対象範囲と前提情報を踏まえ、既知の懸念への対応を計画します。

**次の確認:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

**情報の状態:** 不足情報あり

**移行準備:** 準備状況は不明

**不足情報:**

- 正確なパラメーターへの対応
- 接続相手の対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応

**補足:**

- 正確なパラメーターの符号化は不明です。

[根拠・申告・候補の詳細](#evidence-2)

## 根拠と対象範囲

準備が整っているという表示は申告上の対応状況であり、配備を検証した結果ではありません。候補には、パラメーター、符号化、相互運用性の検証が別途必要です。継続確認は記載された役割の範囲に限り、対象外という判断も安全性の判定ではありません。

<a id="evidence-1"></a>

### 根拠 1

判断: <code>migration&#95;decision&#45;v1&#45;a13a31e84c1cb6fbdaab7091ecce8931f2c0bdaebc583dc2e244bc5ee3bf15c5</code>

対象: <code>migration&#95;target&#45;v1&#45;384b866a8a9b7b9727239252d160bad0bd6fda711b26fe088c952e2e0ceeaf7d</code>

使った申告: [<code>migration&#95;claim&#45;v1&#45;084dc0d2d241642a0109dec9d764cf216f7f585b4f537abf777525b81ed8483b</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;74b10ebedafb1906e69f08d266b3ed4fb37763fd6c0e4c82fd0bff573a02f548</code>](#claim-5), [<code>migration&#95;claim&#45;v1&#45;95c3c16ae97b9c1d5149b80d3e883979db7ded5021a6244403f275eaa7f60517</code>](#claim-7), [<code>migration&#95;claim&#45;v1&#45;c319572a55500ade79c402969f7db3bc3c70e3adccdb3edc12a450baa42ade5e</code>](#claim-8)

検討候補: [<code>migration&#95;candidate&#45;v1&#45;2c53efdcac9c0b8036d19728ed16dbdb6e0f96efa880ff53c3e1e8b6cb210881</code>](#candidate-1)

出典: [<code>F12</code>](#source-1), [<code>F14</code>](#source-3), [<code>LOCAL026</code>](#source-4), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

保存された観測、技術上の事実、固定の理由コード：

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

### 根拠 2

判断: <code>migration&#95;decision&#45;v1&#45;742f70ed1829883b0b93c67ddd7eb8a92df0f9560a6d341c89c88438af0e96b7</code>

対象: <code>migration&#95;target&#45;v1&#45;ae577ba87f4660ddb0c2d95b7d14dea300b9050d2cc7e560c804421e854bf622</code>

使った申告: [<code>migration&#95;claim&#45;v1&#45;0a6d7bbd4fd099f812c42393e9fdd8fe8d876bae174f9fe29bc754e70d8eb3ed</code>](#claim-2), [<code>migration&#95;claim&#45;v1&#45;4dd8de68a4b95ad71a2778c20ef5d180467e9946cea5af4866e6f6e506d84fd2</code>](#claim-3), [<code>migration&#95;claim&#45;v1&#45;7391d9522b9b50e2aa00b378d6843bdd72263a70004e5603a658abf916ab15d5</code>](#claim-4), [<code>migration&#95;claim&#45;v1&#45;88ee3c5f8c8f8fd09a81ecfdc62f2a8c01ae71fb2b6d7121a8cf33fc8ff06865</code>](#claim-6), [<code>migration&#95;claim&#45;v1&#45;ff62d040f2d7a2b163baabf5fbad9d4d5dda74e67fd3141fadce68e69b8d51a1</code>](#claim-9)

検討候補: [<code>migration&#95;candidate&#45;v1&#45;1797317ec15d8f16fe14ccea8a5cf1ac8f0c0d7e01e3c892b972394bbe5859b6</code>](#candidate-2)

出典: [<code>F12</code>](#source-1), [<code>F13</code>](#source-2), [<code>F14</code>](#source-3), [<code>LOCAL026</code>](#source-4), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

保存された観測、技術上の事実、固定の理由コード：

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

## 使用した申告の由来

利用者や外部レビュー由来の申告は未検証です。有効性の表示は申告された期間を表し、独立した確認結果ではありません。

<a id="claim-1"></a>

### 申告: 利用中・利用予定の状況

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;active&quot;</code>

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

### 申告: 適用範囲

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;applicable&quot;</code>

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

### 申告: 必要な用途

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;signature&quot;</code>

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

### 申告: 利用中・利用予定の状況

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;active&quot;</code>

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

### 申告: 適用範囲

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;applicable&quot;</code>

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

### 申告: 業務上の重要性

利用者の申告（未検証） · 申告期間内

申告値: <code>false</code>

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

### 申告: 必要な用途

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;signature&quot;</code>

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

### 申告: 業務上の重要性

利用者の申告（未検証） · 申告期間内

申告値: <code>true</code>

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

### 申告: 対象範囲の期限

利用者の申告（未検証） · 申告期間内

申告値: <code>&#123;&quot;kind&quot;&#58;&quot;none&quot;&#125;</code>

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

## 検討候補と検証

<a id="candidate-1"></a>

### <code>ML&#45;DSA</code>

出典: [<code>S204</code>](#source-6), [<code>LOCAL026</code>](#source-4)

- 実行環境の対応: 不明
- プロバイダーの対応: 不明
- 正確なパラメーターへの対応: 不明
- 接続相手の対応: 不明
- プロトコルの対応: 不明

**検証事項:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 展開前に正確なパラメーターとプロトコルの符号化を確認してください。
- 実行環境、プロバイダー、接続相手、展開手順を検証してください。
- 実装と相互運用性を別途検証してください。

正確な候補の記録：

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

出典: [<code>S204</code>](#source-6), [<code>LOCAL026</code>](#source-4)

- 実行環境の対応: 不明
- プロバイダーの対応: 不明
- 正確なパラメーターへの対応: 不明
- 接続相手の対応: 不明
- プロトコルの対応: 不明

**検証事項:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 展開前に正確なパラメーターとプロトコルの符号化を確認してください。
- 実行環境、プロバイダー、接続相手、展開手順を検証してください。
- 実装と相互運用性を別途検証してください。

正確な候補の記録：

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

## 出典の版と適用範囲

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

## 入力・版・確認範囲の制約

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

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。
