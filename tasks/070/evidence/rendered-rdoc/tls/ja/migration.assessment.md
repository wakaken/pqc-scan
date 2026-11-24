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

- 正確なパラメーターの符号化は不明です。
- 業務システムとの対応付けは不明です。

[根拠・申告・候補の詳細](#evidence-3)

## 根拠と対象範囲

準備が整っているという表示は申告上の対応状況であり、配備を検証した結果ではありません。候補には、パラメーター、符号化、相互運用性の検証が別途必要です。継続確認は記載された役割の範囲に限り、対象外という判断も安全性の判定ではありません。

<a id="evidence-1"></a>

### 根拠 1

判断: <code>migration&#95;decision&#45;v1&#45;64f7e09c5f6cea5012a513bd3677cfeb67ddc0f58146050109125e0e508425a6</code>

対象: <code>migration&#95;target&#45;v1&#45;7f0437243eb67d95e9c589931213def7ffbdf88da00db46029f56c898e2a3578</code>

使った申告: [<code>migration&#95;claim&#45;v1&#45;a25b742095fb1c8131101a1b96212c85e4d4822dbd0487c72f681c3f08b40054</code>](#claim-1), [<code>migration&#95;claim&#45;v1&#45;a7727f48f7d9ff0faabcdbdf589846d40e87800a91cc6a35e36415c8ac467cde</code>](#claim-2)

検討候補: 

出典: [<code>F06</code>](#source-1), [<code>F07</code>](#source-2), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5)

保存された観測、技術上の事実、固定の理由コード：

    {
      "id": "migration_target-v1-7f0437243eb67d95e9c589931213def7ffbdf88da00db46029f56c898e2a3578",
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
        "collection_outcome": "completed"
      },
      "limitations": [
        "unknown_parameter_encoding",
        "combiner_implementation_unverified"
      ]
    }

    [
      "bounded_role_no_known_concern",
      "unknown_parameter_encoding",
      "combiner_implementation_unverified"
    ]

<a id="evidence-2"></a>

### 根拠 2

判断: <code>migration&#95;decision&#45;v1&#45;090adf97fe32da6be39c3ee92731e5323cd94a6ea948dd5298b828cb12391ce9</code>

対象: <code>migration&#95;target&#45;v1&#45;cfb6eff3499d8d9a3b29da39ae787362ed425ffaf6f34c7388b7f700d341fe00</code>

使った申告: 

検討候補: [<code>migration&#95;candidate&#45;v1&#45;698ea4faa6c84824726b9c036276aa8e410cd6e54e56db1eb82848b8233c216b</code>](#candidate-1)

出典: [<code>F08</code>](#source-3), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5), [<code>LOCAL026</code>](#source-6), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

保存された観測、技術上の事実、固定の理由コード：

    {
      "id": "migration_target-v1-cfb6eff3499d8d9a3b29da39ae787362ed425ffaf6f34c7388b7f700d341fe00",
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
        "collection_outcome": "completed"
      },
      "limitations": [
        "unknown_parameter_encoding"
      ]
    }

    [
      "insufficient_information",
      "unknown_parameter_encoding",
      "unknown_binding"
    ]

<a id="evidence-3"></a>

### 根拠 3

判断: <code>migration&#95;decision&#45;v1&#45;af716d0aed4c59b4a54d56412a8d4e6c2be1fdc11525b4248e718b67a207a6d6</code>

対象: <code>migration&#95;target&#45;v1&#45;24c6384f7d935a4495728e19b225cd92a65d68625b15d8413ad6c5dbbe53b99d</code>

使った申告: 

検討候補: 

出典: [<code>F08</code>](#source-3), [<code>F12</code>](#source-4), [<code>F14</code>](#source-5), [<code>LOCAL026</code>](#source-6), [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

保存された観測、技術上の事実、固定の理由コード：

    {
      "id": "migration_target-v1-24c6384f7d935a4495728e19b225cd92a65d68625b15d8413ad6c5dbbe53b99d",
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
        "collection_outcome": "completed"
      },
      "limitations": [
        "unknown_parameter_encoding"
      ]
    }

    [
      "insufficient_information",
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
      "id": "migration_claim-v1-a25b742095fb1c8131101a1b96212c85e4d4822dbd0487c72f681c3f08b40054",
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
      "id": "migration_claim-v1-a7727f48f7d9ff0faabcdbdf589846d40e87800a91cc6a35e36415c8ac467cde",
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
      "id": "migration_candidate-v1-698ea4faa6c84824726b9c036276aa8e410cd6e54e56db1eb82848b8233c216b",
      "target_id": "migration_target-v1-cfb6eff3499d8d9a3b29da39ae787362ed425ffaf6f34c7388b7f700d341fe00",
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
      "sha256": "d2460ea7e75165d9b0e0fc27c4b654c9b66a66c82c97baf8a0556ad4aa1f8f26",
      "schema_version": 1,
      "collection_time": "2026-11-17T00:00:00Z",
      "attempt_id": 1,
      "protocol": "tls"
    }

    {
      "format": "pqc-scan.migration-context",
      "version": 1,
      "sha256": "10bfaa0cda407d0b300cbeb265cb28cbd9498589ba9c6c6f847ab531ef328adc"
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
      "collection_outcome": "completed",
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
