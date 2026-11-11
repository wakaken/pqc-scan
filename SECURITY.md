# Security policy

This project is a development-stage static scanner with optional explicitly
invoked network probes. The latest verified development snapshot receives
best-effort maintenance. No response SLA, old-version backport guarantee or 1.0
support commitment is declared. Unsupported inputs and incomplete observations
must not be interpreted as a clean security assessment.

## Reporting a vulnerability

As observed on 2026-11-10, GitHub private vulnerability reporting for this
repository is disabled. No verified private contact is currently documented.
Do not post exploit details, private source, credentials, keys or sensitive
reports in a public issue. Open an issue that only asks the maintainers for a
private reporting contact, then wait for that contact before sharing details.
This interim process does not promise a private channel or a response deadline.
Do not send vulnerability details to an unverified address.

If a verified private channel is established, include affected revision/features,
a minimal nonsecret reproduction, expected/actual behavior and impact. Use
synthetic keys and endpoints. Redact reports before sharing: source and endpoint
metadata can remain sensitive even when recognized key material is masked.

## Distribution and updates

No publication or signature is established by local packaging preparation.
SHA-256 manifests detect changes relative to supplied hashes; they do not prove
authenticity. Follow the [installation and update guide](docs/user/DISTRIBUTION.md)
or [日本語の導入・更新手順](docs/user/DISTRIBUTION.ja.md). Keep binary and rule
versions together and retain the previous verified bundle for rollback.

## 日本語での連絡

2026年11月10日の確認では、GitHubの非公開脆弱性報告は無効で、確認済みの非公開連絡先も
未掲載です。公開issueには詳細を書かず、非公開連絡先の案内だけを依頼してください。
案内を受けるまでは攻撃手順、非公開ソース、認証情報、鍵、機微なレポートを投稿しないでください。
回答期限や非公開窓口の提供は保証していません。再現には秘密を含まないfixtureを使います。
