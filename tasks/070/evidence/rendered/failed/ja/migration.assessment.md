# 移行判断

対象数: 3 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>unknown</code> — 緊急度を判断できない

**場所:** <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> · <code>negotiated&#95;key&#95;establishment</code>

**対応:** 調査

**理由:** 現在の情報では、緊急度を裏付ける判断条件が揃いません。

**次の確認:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

**情報の状態:** 不足情報あり

**移行準備:** 準備状況は不明

**不足情報:**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応
- 正確な暗号方式の識別
- 正確なパラメーター
- 利用中・利用予定の状況
- 必要な用途

**補足:**

- この対象の情報収集は未完了です。
- 検証は完了していません。
- 暗号方式を特定できていません。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

[根拠・申告・候補の詳細](#evidence-1)

## 2. <code>RSA&#45;MD5</code> — 早急に対応

**場所:** <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> · <code>certificate&#95;signature</code>

**対応:** 調査

**理由:** 保存された事実に、用途に関係する従来暗号の懸念があります。

**次の確認:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

**情報の状態:** 不足情報あり

**移行準備:** 準備状況は不明

**不足情報:**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- 接続相手の対応
- PKIの対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- パラメーターの符号化
- 利用中・利用予定の状況
- 必要な用途

**補足:**

- この対象の情報収集は未完了です。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

[根拠・申告・候補の詳細](#evidence-2)

## 3. <code>RSA</code> — 緊急度を判断できない

**場所:** <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> · <code>certificate&#95;subject&#95;key</code>

**対応:** 調査

**理由:** 現在の情報では、緊急度を裏付ける判断条件が揃いません。

**次の確認:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 不足・不明・失効している対象範囲の申告を確認し、更新してください。

**情報の状態:** 不足情報あり

**移行準備:** 準備状況は不明

**不足情報:**

- 適用範囲
- システムとの明示的な対応付け
- 正確なパラメーターへの対応
- プロバイダーの対応
- 実行環境の対応
- パラメーターの符号化
- 利用中・利用予定の状況
- 必要な用途

**補足:**

- この対象の情報収集は未完了です。
- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

[根拠・申告・候補の詳細](#evidence-3)

## 根拠と対象範囲

準備が整っているという表示は申告上の対応状況であり、配備を検証した結果ではありません。候補には、パラメーター、符号化、相互運用性の検証が別途必要です。継続確認は記載された役割の範囲に限り、対象外という判断も安全性の判定ではありません。

<a id="evidence-1"></a>

### 根拠 1

判断: <code>migration&#95;decision&#45;v1&#45;06dd1f8c8cd72f92d491fae6496d501cf5cea0a40444c0bb01a439b893f0dc60</code>

対象: <code>migration&#95;target&#45;v1&#45;7dc4de55d949e725fee8cca5d0130e03814c88fddf71b8b5f099a30e993bb2d3</code>

使った申告: 

検討候補: 

出典: [<code>F14</code>](#source-4), [<code>LOCAL026</code>](#source-5)

保存された観測、技術上の事実、固定の理由コード：

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

### 根拠 2

判断: <code>migration&#95;decision&#45;v1&#45;57af0a250c2273ad6a6333ac09de78dde50df961c5d5f3640b743b42dfb84a96</code>

対象: <code>migration&#95;target&#45;v1&#45;e872792348a59baca96dd26560dbbb7d0e9bb76943b882703904f1ee984c7604</code>

使った申告: 

検討候補: [<code>migration&#95;candidate&#45;v1&#45;e7ee1dfcfe4894544c7ed2b841c13d3d40e324c7a0acf51aa8f16afd45759028</code>](#candidate-1)

出典: [<code>F08</code>](#source-2), [<code>F12</code>](#source-3), [<code>F14</code>](#source-4), [<code>LOCAL026</code>](#source-5), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6), [<code>R6151</code>](#source-7)

保存された観測、技術上の事実、固定の理由コード：

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

### 根拠 3

判断: <code>migration&#95;decision&#45;v1&#45;fcda13859d2e6064f6a32e2bb3337103a31f4c0954c1f292f319db3aaaf83e8e</code>

対象: <code>migration&#95;target&#45;v1&#45;4696c757de0244243d537a84a174637883f59cf7f6c02ed30a8c991bee6cad83</code>

使った申告: 

検討候補: 

出典: [<code>F08</code>](#source-2), [<code>F12</code>](#source-3), [<code>F14</code>](#source-4), [<code>LOCAL026</code>](#source-5), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)

保存された観測、技術上の事実、固定の理由コード：

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

## 使用した申告の由来

利用者や外部レビュー由来の申告は未検証です。有効性の表示は申告された期間を表し、独立した確認結果ではありません。

## 検討候補と検証

<a id="candidate-1"></a>

### <code>ML&#45;DSA</code>

出典: [<code>S204</code>](#source-8), [<code>LOCAL026</code>](#source-5)

- 実行環境の対応: 不明
- プロバイダーの対応: 不明
- 正確なパラメーターへの対応: 不明
- 接続相手の対応: 不明
- プロトコルの対応: 不明
- PKIの対応: 不明

**検証事項:**

- 実際の利用状況と必要な暗号用途を確認してください。
- 展開前に正確なパラメーターとプロトコルの符号化を確認してください。
- 実行環境、プロバイダー、接続相手、展開手順を検証してください。
- 実装と相互運用性を別途検証してください。

正確な候補の記録：

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

## 出典の版と適用範囲

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

## 入力・版・確認範囲の制約

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

- これは保存された1回の試行であり、接続先全体の確認結果ではありません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。
