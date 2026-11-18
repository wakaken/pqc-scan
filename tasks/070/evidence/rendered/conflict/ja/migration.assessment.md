# 移行判断

対象数: 1 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>ML&#45;KEM&#45;768</code> — 緊急度を判断できない

**場所:** <code>app&#46;txt&#58;1</code> · <code>algorithm</code>

**申告されたシステム:** <code>system&#45;a</code>

**対応:** 調査

**理由:** 現在の情報では、緊急度を裏付ける判断条件が揃いません。

**次の確認:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

**情報の状態:** 申告と既知の事実に矛盾

**移行準備:** 準備状況は不明

**不足情報:**

- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応

**補足:**

- 正確なパラメーターの符号化は不明です。
- 申告された用途が、既知の暗号方式の用途と矛盾しています。

[根拠・申告・候補の詳細](#evidence-1)

## 根拠と対象範囲

準備が整っているという表示は申告上の対応状況であり、配備を検証した結果ではありません。候補には、パラメーター、符号化、相互運用性の検証が別途必要です。継続確認は記載された役割の範囲に限り、対象外という判断も安全性の判定ではありません。

<a id="evidence-1"></a>

### 根拠 1

判断: <code>migration&#95;decision&#45;v1&#45;248594f8817705ea0878e59a5726d9c50bcfebfe45581e400761a76f79785763</code>

対象: <code>migration&#95;target&#45;v1&#45;f51bff5a334ec2a95a3b654c2f3e32977a4a6b84e03e657a9fd6d22eb3f0a720</code>

使った申告: [<code>migration&#95;claim&#45;v1&#45;8c3381dfeed4df0e97ece774247da6b9b87e4d6287169d93554031e3991e02bf</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;93d293e1c38bf21dd7f9c5072c5436c7e8b7b4a88727657323307c208920d22a</code>](#claim-2), [<code>migration&#95;claim&#45;v1&#45;a50e967b78162ee1ba19e1e562fe1466472fde656a3d8b3844bd722cea562060</code>](#claim-3)

検討候補: 

出典: [<code>F12</code>](#source-1), [<code>F14</code>](#source-2), [<code>LOCAL026</code>](#source-3), [<code>S203</code>](#source-4)

保存された観測、技術上の事実、固定の理由コード：

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

## 使用した申告の由来

利用者や外部レビュー由来の申告は未検証です。有効性の表示は申告された期間を表し、独立した確認結果ではありません。

<a id="claim-1"></a>

### 申告: 必要な用途

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;signature&quot;</code>

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

### 申告: 適用範囲

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;applicable&quot;</code>

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

### 申告: 利用中・利用予定の状況

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;active&quot;</code>

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

## 検討候補と検証

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

## 入力・版・確認範囲の制約

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

- 棚卸しの結果だけでは、網羅的に確認したとは判断できません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。
