# Task080 retained review record

Use only `/root/implementation_review` and `/root/test_review` for this combined076-083 user request. Main owns all investigation, implementation, testing and docs; reviewers cannot delegate.

## Implementation review1: changes requested

Manifest21a4e6bdcdd7141d34c5960fc934fbd8118777bdf3ecd5db1f975f735f6affa8, all48 source hashes matched. Reviewer built the target and performed narrow scans only; raw inputs/outputs are preserved in evidence/review1-reproduction. Three P2 findings: DigestInit context/opaque key-output pointers retained old types; OQS SLH uppercase/lowercase mechanism spellings were coerced; C++ type parameters/using declarations could masquerade as real APIs.

Main reproduced all three before fixes (review1-red.log). Explicit EVP output roles now replace prior state or invalidate opaque pointer outputs. Known output buffers separately invalidate scalar selectors; object-family source references remain. Selector dialects separate EVP numeric IDs/OpenSSL names/OQS exact mechanism strings and constants. C++ owning template frames/scoped symbols reject type/non-type/default/template-template and using collisions while retaining unrelated-scope/global-qualified controls. No import/instantiation/crypto execution or storage/evaluator changes.

review1-adopted.log passes26 C-family tests, including all484 original frozen cases. review1-regression-3.log also passes C#19/PHP12/routing11. Clippy/fmt/quality134/290 pass. Earlier reds/intermediate errors remain historical: generic candidate boundary assumptions, known-shadow Partial, public RS256 rule-ID exception, a missing numeric annotation/LocationPartialEq assertion and redundant lookup triggeringAPI512. These were corrected rather than changing product limits.

Scoped implementation confirmation requested for5fb2fe9db1ee0eb9c78ca0fcec7c64907ac0c1de22c4a4d7d859368a1f6b352a. Source48 plus two source fixture hashes are bound. Initial broad suites passed the pre-fix version only; final broad runs and all CLI/package/compatibility/browser checks and test review remain required.

## Scoped implementation acceptance

Retained implementation reviewer accepted5fb2fe9db1ee0eb9c78ca0fcec7c64907ac0c1de22c4a4d7d859368a1f6b352a, all48 source/two input hashes matched. Three P2 findings resolved; no new findings. Normal output-role and unrelated-scope/global-qualified controls remain. Full suites were not independently rerun. Main must complete post-fix broad/CLI/package/browser evidence and retained test review before080closure.

## Final test acceptance

The retained test reviewer accepted5fb2fe9db1ee0eb9c78ca0fcec7c64907ac0c1de22c4a4d7d859368a1f6b352a:48source/twoexternalinput/151observation hashes matched, original076freeze unchanged.484focal cases/26tests, shared budgets, pointer/SLH/owner-scope/UTF16/privacy/legacy-ID controls, post-fix default/all-feature, CLI/adversarial headers/package/compatibility/134-rule quality and EN/JA browser evidence satisfy original080. No full suite was independently repeated. No080 test blocker remains;081-083, runtime crypto capability and whole-milestone completion are excluded.
