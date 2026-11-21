//! Shared human terminology and typed display values; never policy evaluation.
use crate::ReportLanguage;
use pqc_scan_core::migration::{
    ActionKind, ClaimFreshness, Information, Readiness, ResolvedClaim, Target, Urgency,
};
pub(crate) struct Labels(pub(crate) ReportLanguage);
impl Labels {
    pub(crate) fn pick<'a>(&self, en: &'a str, ja: &'a str) -> &'a str {
        if self.0 == ReportLanguage::Ja {
            ja
        } else {
            en
        }
    }
    pub(crate) fn urgency(&self, v: Urgency) -> &str {
        match v {
            Urgency::Urgent => self.pick("Urgent", "早急に対応"),
            Urgency::Plan => self.pick("Plan a response", "計画して対応"),
            Urgency::Monitor => self.pick("Monitor this role", "対象の役割を継続確認"),
            Urgency::Undetermined => self.pick("Urgency undetermined", "緊急度を判断できない"),
            Urgency::NotApplicable => {
                self.pick("Outside declared scope", "申告された範囲では対象外")
            }
        }
    }
    pub(crate) fn information(&self, v: Information) -> &str {
        match v {
            Information::Sufficient => self.pick("Required information available", "必要情報あり"),
            Information::Incomplete => self.pick("Information missing", "不足情報あり"),
            Information::Conflicting => {
                self.pick("Claims conflict with known facts", "申告と既知の事実に矛盾")
            }
        }
    }
    pub(crate) fn readiness(&self, v: Readiness) -> &str {
        match v {
            Readiness::Ready => self.pick(
                "Prerequisites met by active declarations",
                "必要条件は有効な申告上で充足",
            ),
            Readiness::Blocked => self.pick("Known prerequisites not met", "未対応の条件あり"),
            Readiness::Unknown => self.pick("Readiness unknown", "準備状況は不明"),
            Readiness::NotApplicable => {
                self.pick("Migration readiness not applicable", "移行準備の判断対象外")
            }
        }
    }
    pub(crate) fn action(&self, v: ActionKind) -> &str {
        match v {
            ActionKind::Investigate => self.pick("Investigate", "調査"),
            ActionKind::Plan => self.pick("Prepare the migration plan", "移行計画を作る"),
            ActionKind::Unblock => {
                self.pick("Resolve blocking prerequisites", "阻害条件を解消する")
            }
            ActionKind::Migrate => self.pick("Migrate and validate", "移行を実施し検証する"),
            ActionKind::Monitor => self.pick(
                "Monitor changes to this role",
                "この役割の変更を継続確認する",
            ),
            ActionKind::None => self.pick(
                "No action within the declared scope",
                "申告された範囲では対応なし",
            ),
        }
    }
    pub(crate) fn code<'a>(&self, s: &'a str) -> &'a str {
        let (en, ja) = match s {
            "outside_declared_scope" => ("The active declaration excludes this target; known technical concerns remain.", "有効な申告では対象外です。既知の技術上の懸念は残ります。"),
            "known_classical_concern" => ("The saved facts show a relevant classical cryptographic concern.", "保存された事実に、用途に関係する従来暗号の懸念があります。"),
            "deadline_reached_or_lead_time_boundary" => ("The scoped deadline is within the declared migration lead time.", "適用範囲を定めた期限が、申告された移行所要期間の内側に入っています。"),
            "critical_use_declared" => ("Business-critical active or planned signing use has been declared.", "業務上重要な署名用途での利用中・利用予定が申告されています。"),
            "long_protection_or_historical_ciphertext" => ("Declared confidentiality obligations or historical ciphertext require an urgent response.", "申告された秘密保持の必要期間、または過去の暗号文の保護を踏まえ、早急な対応が必要です。"),
            "stale_urgent_claim_requires_refresh" => ("A stale due-deadline claim requires urgent investigation and refresh.", "期限到来の申告が失効しています。早急に調査し、申告を更新してください。"),
            "known_concern_plan" => ("Active scoped facts support planning a response to the known concern.", "有効な対象範囲と前提情報を踏まえ、既知の懸念への対応を計画します。"),
            "bounded_role_no_known_concern" => ("No known concern was established for this bounded role and its recognized facts.", "識別できた事実と、この役割の範囲では既知の懸念は確認されていません。"),
            "insufficient_information" => ("The available facts do not establish a supported urgency branch.", "現在の情報では、緊急度を裏付ける判断条件が揃いません。"),
            "reference_only" => ("A reference does not establish deployed use.", "参照の検出だけでは、実環境での利用は確認できません。"),
            "unauthenticated_advertisement" => ("These are unauthenticated offers, not a selected negotiation.", "これは認証していない提示方式であり、交渉で選ばれた方式ではありません。"),
            "collection_incomplete" => ("Collection is incomplete for this target.", "この対象の情報収集は未完了です。"),
            "verification_not_completed" => ("Verification was not completed.", "検証は完了していません。"),
            "unknown_primitive" => ("The primitive is unidentified.", "暗号方式を特定できていません。"),
            "unknown_parameter_encoding" => ("The exact parameter encoding is unknown.", "正確なパラメーターの符号化は不明です。"),
            "unknown_binding" => ("The business-system binding is unknown.", "業務システムとの対応付けは不明です。"),
            "unverified_combiner_no_resistance_inference" => ("Combiner unverified; no combined resistance conclusion", "結合処理は未検証。組み合わせ全体の耐性は未判定"),
            "standard_specified_implementation_unverified" => ("Combiner specified by the standard; implementation unverified", "標準で結合方法を規定。実装は未検証"),
            "combiner_implementation_unverified" => ("The hybrid combiner implementation has not been verified.", "ハイブリッド方式の結合処理の実装は未検証です。"),
            "stale_claim" => ("A used claim is not currently active; refresh it.", "判断に関係する申告が現在有効ではありません。更新が必要です。"),
            "purpose_mismatch" => ("The declared purpose conflicts with the known primitive purpose.", "申告された用途が、既知の暗号方式の用途と矛盾しています。"),
            "confirm_actual_use_and_required_purpose" => ("Confirm actual use and the required cryptographic purpose.", "実際の利用状況と必要な暗号用途を確認してください。"),
            "refresh_missing_or_stale_scoped_claims" => ("Obtain or refresh the missing, unknown or stale scoped claims.", "不足・不明・失効している対象範囲の申告を確認し、更新してください。"),
            "resolve_declared_blocking_prerequisites" => ("Resolve each declared blocking prerequisite below.", "下記の申告された阻害条件を解消してください。"),
            "validate_implementation_and_interoperability_separately" => ("Validate the implementation and interoperability separately.", "実装と相互運用性を別途検証してください。"),
            "verify_exact_parameters_and_protocol_encoding" => ("Verify exact parameters and protocol encoding before rollout.", "展開前に正確なパラメーターとプロトコルの符号化を確認してください。"),
            "test_runtime_provider_peer_and_rollout" => ("Test runtime, provider, peers and rollout.", "実行環境、プロバイダー、接続相手、展開手順を検証してください。"),
            "explicit_parameter_selection_required" => ("Choose an exact parameter set", "使用する設定を選ぶ必要があります"),
            "candidate_primitive_integration_unverified" => ("Candidate primitive; integration unverified", "方式の候補。組込みは未検証"),
            "migration_goal_month" => ("Migration goal month (planning only)", "移行の目標年月（計画情報）"),
            "migration_target" => ("Declared migration algorithm / configuration", "検討する方式・設定の申告"),
            "runtime" | "support.runtime" => ("Runtime support", "実行環境の対応"),
            "provider" | "support.provider" => ("Provider support", "プロバイダーの対応"),
            "parameters" | "support.parameters" => ("Exact parameter support", "正確なパラメーターへの対応"),
            "peer" | "support.peer" => ("Peer support", "接続相手の対応"),
            "protocol" | "support.protocol" => ("Protocol support", "プロトコルの対応"),
            "jose" | "support.jose" => ("JOSE integration", "JOSEの組込み"),
            "verifiers" | "support.verifiers" => ("Verifier rollout", "検証側への展開"),
            "pki" | "support.pki" => ("PKI support", "PKIの対応"),
            "context.binding" => ("Explicit system binding", "システムとの明示的な対応付け"),
            "applicability" => ("Applicability", "適用範囲"),
            "use_case" => ("Required purpose", "必要な用途"),
            "usage_state" => ("Actual or planned use", "利用中・利用予定の状況"),
            "business_critical" => ("Business criticality", "業務上の重要性"),
            "confidential" => ("Confidentiality requirement", "秘密保持の必要性"),
            "confidentiality_until" => ("Confidentiality lifetime", "秘密保持が必要な期間"),
            "signature_verification_until" => ("Signature-verification lifetime", "署名検証が必要な期間"),
            "historical_ciphertext" => ("Historical ciphertext obligation", "過去の暗号文の保護"),
            "migration_lead_seconds" => ("Migration lead time", "移行に必要な期間"),
            "deadline" => ("Scoped deadline", "対象範囲の期限"),
            "technical.identity" => ("Exact primitive identity", "正確な暗号方式の識別"),
            "technical.parameters" => ("Exact parameters", "正確なパラメーター"),
            "technical.encoding" => ("Parameter encoding", "パラメーターの符号化"),
            "technical.collision_sensitive_use" => ("Whether collision resistance is required", "衝突耐性が必要な用途かどうか"),
            "observation.selected_use" => ("Selected negotiation or actual use", "交渉で選ばれた方式または実際の利用"),
            "inventory_not_exhaustive" => ("Inventory does not establish exhaustive coverage.", "棚卸しの結果だけでは、網羅的に確認したとは判断できません。"),
            "one_saved_attempt_not_endpoint_coverage" => ("This is one saved attempt, not endpoint-wide coverage.", "これは保存された1回の試行であり、接続先全体の確認結果ではありません。"),
            "saved_content_not_collector_attestation" => ("The digest binds saved content, not collector authenticity.", "ダイジェストは保存内容の同一性を示し、収集者の真正性を証明しません。"),
            "no_offline_cryptographic_reverification" => ("Offline assessment does not repeat cryptographic verification.", "オフライン評価では暗号学的な再検証を行いません。"),
            "local_advisory_policy_not_practitioner_validation" => ("This local advisory policy has not established practitioner usefulness.", "このローカルな助言方針が実務に役立つかどうかは、まだ評価していません。"),
            _ => return s,
        };
        self.pick(en, ja)
    }
}

impl Labels {
    pub(crate) fn token<'a>(&self, s: &'a str) -> &'a str {
        let (en, ja) = match s {
            "algorithm" => ("Algorithm reference", "暗号方式への参照"),
            "public_key.algorithm" | "public-key.algorithm" => ("Public key", "公開鍵"),
            "certificate_signature" | "certificate.signature_algorithm" => {
                ("Certificate signature", "証明書の署名")
            }
            "certificate_subject_key" | "certificate.subject_key.algorithm" => {
                ("Certificate public key", "証明書の公開鍵")
            }
            "negotiated_key_establishment" => ("Connection key establishment", "通信の鍵確立"),
            "advertised_host_signature" => {
                ("Offered SSH host signature", "提示されたSSHホスト署名")
            }
            "advertised_key_establishment" => {
                ("Offered SSH key establishment", "提示されたSSH鍵確立")
            }
            "host_signature" => ("SSH host signature", "SSHホスト署名"),
            "reference-only" => ("Reference only", "参照のみ"),
            "signature" | "pki" => ("Signing / authentication", "署名・認証"),
            "jwt" => ("JWT signing", "JWT署名"),
            "key_establishment" => ("Key establishment", "鍵確立"),
            "encryption" => ("Encryption", "暗号化"),
            "hash" | "hashing" => ("Hashing", "ハッシュ"),
            "vulnerable" => ("Quantum exposure identified", "量子計算に対する懸念あり"),
            "designed_resistant" => (
                "Designed for quantum resistance; deployment unverified",
                "耐量子性を意図した方式・配備は未検証",
            ),
            "generic_search" => (
                "Generic quantum search effects; assess key and hash parameters separately",
                "汎用の量子探索による影響。鍵・ハッシュの条件は別途確認",
            ),
            "unverified_or_incomplete" => (
                "Verification not completed or collection incomplete",
                "検証未完了、または収集情報が不完全",
            ),
            "syntactic" => ("Syntactic reference", "構文上の参照"),
            "lexical" => ("Text match", "文字列の一致"),
            "parsed" => ("Parsed attributes", "解析した属性"),
            "collector_reported_verified_session" => (
                "Collector reports a verified session",
                "収集記録がセッションの検証成功を報告",
            ),
            "unauthenticated_advertisement" => ("Unauthenticated offers", "認証していない提示方式"),
            "active" => ("In use", "利用中"),
            "planned" => ("Planned use", "利用予定"),
            "unused" => ("Not in use", "未使用"),
            "test_only" => ("Test only", "試験用途のみ"),
            "applicable" => ("In scope", "対象"),
            "not_applicable" => ("Outside declared scope", "申告範囲では対象外"),
            "known" => ("Known", "確認済みの値"),
            "unknown" => ("Unknown", "不明"),
            "complete" | "completed" => (
                "Completed within recorded scope",
                "記録された範囲で収集完了",
            ),
            "partial" => ("Partially collected", "一部の収集が未完了"),
            "failed" => ("Collection failed", "収集失敗"),
            "public_key_below_local_minimum" => (
                "Public-key size below the local minimum",
                "公開鍵の長さがローカル基準未満",
            ),
            "signature_uses_weak_collision_digest" => (
                "Signature uses a weak collision digest",
                "署名に衝突耐性の弱いハッシュを使用",
            ),
            "weak_collision_resistance" => ("Collision-resistance concern", "衝突耐性の懸念"),
            _ => return self.code(s),
        };
        self.pick(en, ja)
    }
    pub(crate) fn support(&self, v: Option<bool>) -> &str {
        match v {
            Some(true) => self.pick("Declared supported", "申告上は対応"),
            Some(false) => self.pick("Declared unsupported", "申告上は未対応"),
            None => self.pick("Unknown", "不明"),
        }
    }
    pub(crate) fn freshness(&self, v: ClaimFreshness) -> &str {
        match v {
            ClaimFreshness::Active => self.pick("Active declared interval", "申告期間内"),
            ClaimFreshness::Stale => self.pick("Expired declaration", "申告は失効"),
            ClaimFreshness::NotYetEffective => self.pick("Not yet effective", "申告の開始前"),
            ClaimFreshness::Unknown => self.pick("Expiry unknown", "有効期限不明"),
        }
    }
    pub(crate) fn value(&self, c: &ResolvedClaim) -> String {
        let Some(value) = &c.value else {
            return self.pick("Unknown", "不明").into();
        };
        if let Some(v) = value.as_bool() {
            return if c.field.starts_with("support.") {
                self.support(Some(v)).into()
            } else {
                self.pick(
                    if v { "Yes" } else { "No" },
                    if v { "はい" } else { "いいえ" },
                )
                .into()
            };
        }
        if let Some(v) = value.as_str() {
            return self.token(v).into();
        }
        if let Some(v) = value.as_u64() {
            return format!("{v} {}", self.pick("seconds", "秒"));
        }
        if c.field == "migration_target" {
            return format!(
                "{} / {} · {}",
                value["algorithm"].as_str().unwrap_or("unknown"),
                value["parameters"].as_str().unwrap_or("unknown"),
                self.token(value["use_case"].as_str().unwrap_or("unknown"))
            );
        }
        if c.field == "deadline" {
            if value["kind"] == "none" {
                return self
                    .pick("No applicable deadline declared", "適用する期限なしと申告")
                    .into();
            }
            return format!(
                "{} · {}: {}",
                value["at"].as_str().unwrap_or("unknown"),
                self.pick("Reference", "参照資料"),
                value["reference_id"].as_str().unwrap_or("unknown")
            );
        }
        self.pick("Unknown", "不明").into()
    }
}
pub(crate) fn location(t: &Target) -> String {
    t.source_ref
        .path
        .as_ref()
        .map(|p| {
            t.source_ref
                .line
                .map(|n| format!("{p}:{n}"))
                .unwrap_or_else(|| p.clone())
        })
        .or_else(|| t.source_ref.detail_path.clone())
        .unwrap_or_else(|| t.source_ref.role.clone())
}
pub(crate) fn enum_token(v: &impl serde::Serialize) -> String {
    serde_json::to_value(v)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_else(|| "unknown".into())
}
