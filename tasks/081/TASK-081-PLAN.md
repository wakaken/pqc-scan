# Task081 bounded dependency formats and guidance

Prerequisite: accepted076-080, HEAD7e8cd91426fa21609a91c0511de44252eaa890d4 plus080manifest5fb2fe9db1ee0eb9c78ca0fcec7c64907ac0c1de22c4a4d7d859368a1f6b352a. Original081 criteria, roadmap4.2 and frozen25 dependency inputs/source editions remain unchanged. Main owns investigation/code/tests/docs; reuse only the retained two reviewers.

## Outcome and constraints

Implement all Composer, NuGet/MSBuild/SDK, vcpkg, Conan0.5 and literal CMake rows. Preserve separate source assertions, declared/reported/resolved/environment/conditional/development/locator distinctions and current six dependency fields. No package manager, compiler, MSBuild, Python, build script, graph resolver, registry query or input-code execution. No stored-field/schema/required-contract or evaluator changes. OQS projects keep distinct identities; dependency presence does not prove installed versions, effective providers or PQC readiness.

## Approach

Reuse DependencyItem, Build, bounded duplicate-rejecting JSON decoder, roxmltree with DTD disabled, current2MiB/64depth/50,000values/10,000records bounds and RepositoryContext.read_relative selected-file/no-link protections. New format modules sit beside existing parsers; old inputs retain their paths/semantics. CMake is always lexical/incomplete, including dynamic directives; Conan Python is not executed. Future NuGetlock3/Conan0.6 remain explicitly unsupported.

Add narrow ecosystem-aware Composer vendor/package and NuGet ID/constraint validation. NuGet canonical comparison is ASCII case-insensitive but accepted-record ordinals/source paths retain distinct assertions. Do not relax old ecosystems' validation to accept NuGet bracket ranges. Generate registry purls only from validated representable identities/eligible versions; Composer Git/path/custom archive records stay unresolved and URLs/content hashes are omitted. No vcpkg/Conan/CMake registry purl is invented.

Keep existing public DependencyOrigin variants intact for Rust API compatibility. If environment distinctions need additional provenance, use an additive finite typed context constructor/enum that changes only existing string provenance/capture fields, never arbitrary raw metadata. Platform requirements/emulation/disabled emulation, SDK and target-framework declarations are not installed software records.

For central packages, inspect only nearest selected in-root Directory.Packages.props/Directory.Build.props through bounded safe repository context. Literal declarations may inform a separately traceable project assertion; conditional/dynamic/duplicate/conflicting/imported/unselected data cannot supply effective resolution. No manifest-lock or project graph joining. Preserve central source assertions independently.

## Steps and acceptance checks

1. Reuse frozen25 cases and pinned official format snapshots; write failing supported/valid-empty/future/malformed/duplicate/provenance/privacy tests before implementation. Add exact expected six-field/purl/position assertions beyond descriptive frozen expectations without rewriting the freeze.
2. Extend classifier/routing and typed identity support minimally; implement bounded Composer/JSON, NuGet/XML+locks+SDK+central context and C-family manifests/text/lexical subsets. Missing/dynamic values remain unresolved and statuses never become false Complete empty results.
3. Exercise source-local identity, case distinctions, safe names/versions/URLs, XML entities, sibling failure, unsupported editions and zero/below/at/above budget boundaries. Cover excluded/symlink/out-of-root/dynamic central sources and no input execution.
4. Add evidence-bound PHP/.NET/C-family guidance and exact wrapper maintenance distinctions, avoiding blanket AES replacement or dependency-to-ready promotion. Update applicable rules/quality and paired user docs/provenance architecture.
5. Run targeted parser/core/guidance regressions/fmt/Clippy, inspect combined diff, obtain retained implementation review and fix findings. Complete default/all-feature workspace/build, real CLI/strict/package/old-seven+saved compatibility/privacy/EN-JA browser checks, then retained test review. Bind final source/helper/input/output hashes and actual commands/exits; preserve failures.

Only every original081 criterion and both accepted stages permit done.082-083 remain sequential future integration/final acceptance, not a destination for missing081 work. Primary source existence and synthetic conformance are not independent accuracy or runtime capability evidence. High-risk architecture, incompatible rules, unclear crypto logic and storage-format changes retain the AGENTS human boundary.
