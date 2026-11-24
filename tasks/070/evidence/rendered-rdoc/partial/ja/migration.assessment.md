# 移行判断

対象数: 3 · 評価時刻: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## 1. <code>X25519MLKEM768</code> — 対象の役割を継続確認

**場所:** <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> · <code>negotiated&#95;key&#95;establishment</code>

**申告されたシステム:** <code>system&#45;a</code>

**対応:** この役割の変更を継続確認する

**理由:** 識別できた事実と、この役割の範囲では既知の懸念は確認されていません。

**次の確認:**

- 事実、対象範囲、申告が変わった際に再評価してください。

**情報の状態:** 必要情報あり

**移行準備:** 移行準備の判断対象外

**補足:**

- この対象の情報収集は未完了です。
- 正確なパラメーターの符号化は不明です。
- ハイブリッド方式の結合処理の実装は未検証です。

[根拠・申告・候補の詳細](#evidence-1)

## 2. <code>RSA&#45;SHA256</code> — 緊急度を判断できない

**場所:** <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> · <code>certificate&#95;signature</code>

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
- 接続相手の対応
- PKIの対応
- プロトコルの対応
- プロバイダーの対応
- 実行環境の対応
- パラメーターの符号化
- 正確なパラメーター
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

判断: <code>migration&#95;decision&#45;v1&#45;cbcaf81c18274dc1217560b530d4fca80e2ed8da336d94bf45fdafa8bfa23263</code>

対象: <code>migration&#95;target&#45;v1&#45;27f39fc851d2f3fba67b4cb0a064fb5f2ad98226a307188ca0c03c77811c0619</code>

使った申告: [<code>migration&#95;claim&#45;v1&#45;9fc56777a19b26959e1edba0dcdb7b7e24759770236d38e7c830ee53eede066d</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;b0bd01c15bc51625607ef97e70ecf85cb81b53b1ce7ff665e8608d9e7e93aadf</code>](#claim-2)

検討候補: 

出典: [<code>F06</code>](#source-1), [<code>F07</code>](#source-2), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5)

保存された観測、技術上の事実、固定の理由コード：

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

### 根拠 2

判断: <code>migration&#95;decision&#45;v1&#45;bec05938f3d6e94d356b7ec6bfe3eb51d004cc5c17f78ce4dd104e1316e185f9</code>

対象: <code>migration&#95;target&#45;v1&#45;5318468fd239f64171a661c5cb67c82749ff507c88500ef6d0923f48ce7ca2ec</code>

使った申告: 

検討候補: [<code>migration&#95;candidate&#45;v1&#45;9f87ebb42fbb18a79504a5d5fd9bae02ad91b4938b192e2aae2a63f7e39ae684</code>](#candidate-1)

出典: [<code>F08</code>](#source-3), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5), [<code>LOCAL026</code>](#source-6), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

保存された観測、技術上の事実、固定の理由コード：

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

### 根拠 3

判断: <code>migration&#95;decision&#45;v1&#45;50b1f217aef92b790f7a32dece63ecf150a722d1e5a8cf003444ad0577563529</code>

対象: <code>migration&#95;target&#45;v1&#45;7953901b317a79ffd73c18feceb154c8c9f12beb6671a307f535f5591473f18c</code>

使った申告: 

検討候補: 

出典: [<code>F08</code>](#source-3), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5), [<code>LOCAL026</code>](#source-6), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

保存された観測、技術上の事実、固定の理由コード：

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

## 使用した申告の由来

利用者や外部レビュー由来の申告は未検証です。有効性の表示は申告された期間を表し、独立した確認結果ではありません。

<a id="claim-1"></a>

### 申告: 適用範囲

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;applicable&quot;</code>

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

### 申告: 必要な用途

利用者の申告（未検証） · 申告期間内

申告値: <code>&quot;key&#95;establishment&quot;</code>

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

## 検討候補と検証

<a id="candidate-1"></a>

### <code>ML&#45;DSA</code>

出典: [<code>S204</code>](#source-9), [<code>LOCAL026</code>](#source-6)

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

## 出典の版と適用範囲

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

## 入力・版・確認範囲の制約

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

- これは保存された1回の試行であり、接続先全体の確認結果ではありません。
- ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。
- オフライン評価では暗号学的な再検証を行いません。
- このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。
