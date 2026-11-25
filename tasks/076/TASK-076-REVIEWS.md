# Task 076 independent reviews

Review budget belongs to the combined 076-083 maintainer request: one implementation identity `/root/implementation_review`, one test identity `/root/test_review`. Main work is not delegated. Reuse these identities; this prerequisite review does not cover future product changes.

## Implementation review 1

Source: HEAD `7e8cd91426fa21609a91c0511de44252eaa890d4` plus the dirty artifacts bound by `evidence/FREEZE-2.json` (then named FREEZE.json). Scope: task 076 prerequisite and initial checks only. Decision: changes requested; two P2 findings. Full suite not rerun by reviewer; reviewer relied on supplied evidence.

Original actionable findings (preserved quotation):

> **[P2] 必須操作の期待値が凍結コーパスに不足しています。**
> `openssl_pkey_derive` は汎用キーによるunknownのみで、必須DH/ECDH導出の既知キーに対する期待値がありません。また、`EVP_DigestVerifyInit` は全856ケースに存在せず、EVP署名も汎用 `EVP_DigestSignInit` のunknownのみです。既知のキー由来、汎用unknown、コメント・再代入などの否定境界をC/C++両方に追加してください。

> **[P2] liboqsのパラメーター表記が凍結契約内で矛盾しています。**
> `API-MATRIX.json` の12項目は末尾が小文字 `s/f` ですが、固定済み `sig.h:168–190` の実際のmechanism stringsは大文字 `S/F` です。コーパスと `INTERFACE-CONTRACT.md` は既に大文字を使用しています。入力の正確な文字列と正規化後のFIPS表記を区別し、表を整合させてください。

The reviewer found no additional major issues and accepted the distinctions between existing contracts, grammar parsing/crypto execution, author exposure, historical populations and unexecuted Windows/Linux assumptions. Acceptance remains pending the fixes and scoped confirmation, then a separate test review.

## Main-agent corrections

Verified the findings against the pinned PHP stub, OpenSSL EVP headers and OQS sig.h before changing expectations. `evidence/REVIEW1-AMENDMENT.json` preserves the original 856-case population, exact 26 additional case IDs, the source basis and the fact that no subject implementation/output exists yet. Added known DH/ECDH derive and known/unknown/comment/reassignment EVP sign/verify controls for C and C++; OQS input strings now match uppercase S/F and have separate canonical lowercase mappings. Earlier indices and baseline artifacts remain intact. New freeze, grammar parse/integrity checks and baseline-review1 observations bind the 882-case final prerequisite population.

Implementation scoped confirmation: accepted for FREEZE.json SHA-256 `b2e278c8c26fbcecf94362b95dffa22723263deefa80bb74aafd2db35a409436`; both P2 findings resolved, no further mandatory findings. Test review: pending with `/root/test_review`. No task completion is inferred from this correction record.


## Test review 1 and correction

Reviewer `/root/test_review` reviewed HEAD plus FREEZE SHA-256 `b2e278c8c26fbcecf94362b95dffa22723263deefa80bb74aafd2db35a409436` (retained as FREEZE-3.json). Decision: one P2 finding, acceptance withheld.

> **[P2] 再代入ケースの否定対象を凍結してください。** `php-reassignment-positive`、`cs-rebound-positive`、`cpp-rebound-positive` は、setup観測を許可しながら `rule=null`、`span=null`、`forbidden_algorithms=[]` です。この状態では、正当なconstructor観測と再代入後の誤ったidentity継承を区別する具体的なアサーションが定まりません。対象呼出しの `span` または `forbidden_span`、禁止identity、観測なし／unknownの期待を明示してください。

Other evidence was accepted within its limits: 13 syntax/ABI checks, 882 parse cases, 25 dependency inputs, old/saved controls and 16 measurements. Reviewer independently ran freeze validation (exit 0) and recalculated input/output hashes; no full suite rerun.

The main agent verified an earlier preparatory patch matched unsuffixed labels rather than actual `-positive` IDs. TEST-REVIEW1-AMENDMENT.json preserves before/after expectations and the correction. Three exact rebound call spans now prohibit inherited old identities and explicitly permit absence or unresolved references without those identities; constructor/setup spans remain separately valid. No scanned file bytes, binary, command or environment changed, so the same baseline-review1 measurement population remains applicable. Integrity and grammar checks were rerun for the amended index; no subject implementation/output exists yet.

Amended source: FREEZE SHA-256 `c207aaa30adb88d397357f2a21f6b37ef1ccd328d72bf2030b8262b2783ae319`. Test scoped confirmation and implementation applicability confirmation are pending from the same two reviewers.

## Final prerequisite acceptance

Both existing reviewers accepted FREEZE SHA-256 `c207aaa30adb88d397357f2a21f6b37ef1ccd328d72bf2030b8262b2783ae319`. Implementation applicability remained accepted after the test amendment; test review independently reran freeze validation (exit 0), verified exact focal spans and evidence reuse, and reported no unresolved findings. Scope remains 076 feasibility/contracts/corpus/protocol only. Main-agent completion applies every original prerequisite criterion; no later product task is covered by these reviews.
