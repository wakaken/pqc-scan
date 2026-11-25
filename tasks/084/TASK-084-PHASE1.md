# Phase 1 preparation results

Status: preparation substantially established; commercial-source scope decision and final review remain pending. The complete four-phase study is unfinished.

Baseline HEAD: `7e8cd91426fa21609a91c0511de44252eaa890d4` plus the frozen dirty source. All 80 accepted v0.5 hashes match `c660ef994798d3a3ec26aa43282fa91cad1cff10306c13640c383a67b26f9636`. The full source snapshot binds 10,128 files; sources and existing dev binary are preserved externally. A separate release/default build (`CARGO_INCREMENTAL=0 cargo build --workspace --release --locked`, exit 0) is frozen with binary SHA-256 `100052dd2549b18c71f0adc9edad09884b1ac5af96f1a01ae35a5ee207dffa0a`. Existing dev measurements are not pooled with release measurements.

## Upstream pins and licenses

All 30 default-branch tips were fetched once with Git hooks/external user filters disabled. Source roots remain outside this repository. No target build, installation, script execution, submodule recursion or LFS hydration occurred. The inventory contains 112,180 regular files and 1,792,976,250 bytes before scanner selection. Headers remain ambiguous and C/C++ are separate strata.

| Repository | Fixed commit | Repository license statement |
|---|---|---|
| phpseclib/phpseclib | `808f39f816c4c5e2b08dd2d0b28d1ddc3b59e779` | [MIT](https://github.com/phpseclib/phpseclib/blob/808f39f816c4c5e2b08dd2d0b28d1ddc3b59e779/LICENSE) |
| firebase/php-jwt | `99efeb17d6def8c0295777f2d45b07ff4fdb4e40` | [BSD-3-Clause](https://github.com/firebase/php-jwt/blob/99efeb17d6def8c0295777f2d45b07ff4fdb4e40/LICENSE) |
| lcobucci/jwt | `8bd5e392228c3499d53397fcb589b9c46163a97e` | [BSD-3-Clause](https://github.com/lcobucci/jwt/blob/8bd5e392228c3499d53397fcb589b9c46163a97e/LICENSE) |
| web-token/jwt-framework | `024031b36eb15e8fdfc02dffa257631a052c153c` | [MIT](https://github.com/web-token/jwt-framework/blob/024031b36eb15e8fdfc02dffa257631a052c153c/LICENSE) |
| robrichards/xmlseclibs | `a7e0ac7c1d96502ca85537d723ff219e2217dac9` | [BSD-3-Clause](https://github.com/robrichards/xmlseclibs/blob/a7e0ac7c1d96502ca85537d723ff219e2217dac9/LICENSE) |
| onelogin/php-saml | `1b63f065850e64692e8cb9f7c48979253d372e23` | [MIT](https://github.com/onelogin/php-saml/blob/1b63f065850e64692e8cb9f7c48979253d372e23/LICENSE) |
| simplesamlphp/simplesamlphp | `bf3b3090661167933aea43134dc2f00146bf57ba` | [LGPL-2.1](https://github.com/simplesamlphp/simplesamlphp/blob/bf3b3090661167933aea43134dc2f00146bf57ba/COPYING) |
| nextcloud/server | `8cbd95a46d3336dd48ef6cb2a6b82ca6b6623fbc` | [AGPL-3.0-or-later](https://github.com/nextcloud/server/blob/8cbd95a46d3336dd48ef6cb2a6b82ca6b6623fbc/COPYING-README) |
| PHPMailer/PHPMailer | `d6e82f548f2efe1abd10d03e08ae3fccdc6f1e10` | [LGPL-2.1](https://github.com/PHPMailer/PHPMailer/blob/d6e82f548f2efe1abd10d03e08ae3fccdc6f1e10/LICENSE) |
| roundcube/roundcubemail | `d2af8ed4cce2ac9883a18d569d4547ddf414d90f` | [GPL-3.0-or-later](https://github.com/roundcube/roundcubemail/blob/d2af8ed4cce2ac9883a18d569d4547ddf414d90f/composer.json) |
| openssl/openssl | `18d0baa70de7c59d0df5b899122335c93ccbe0e8` | [Apache-2.0](https://github.com/openssl/openssl/blob/18d0baa70de7c59d0df5b899122335c93ccbe0e8/LICENSE.txt) |
| google/boringssl | `5ec099107553142e9cb34dfe52f3c5bb42c9b7fb` | [Apache-2.0 plus retained component notices](https://github.com/google/boringssl/blob/5ec099107553142e9cb34dfe52f3c5bb42c9b7fb/LICENSE) |
| wolfSSL/wolfssl | `f4f52d975ff1b0c9eb0bd1e8b4563c1a6d6b2401` | [GPL-3.0 or commercial alternative](https://github.com/wolfSSL/wolfssl/blob/f4f52d975ff1b0c9eb0bd1e8b4563c1a6d6b2401/COPYING) |
| randombit/botan | `64aec977039f292ee2b2a5e1f67951eefa4e903d` | [BSD-2-Clause](https://github.com/randombit/botan/blob/64aec977039f292ee2b2a5e1f67951eefa4e903d/license.txt) |
| openssh/openssh-portable | `d1b85c67231443b6db150c56ec5aec0e88459edb` | [BSD-style component licenses](https://github.com/openssh/openssh-portable/blob/d1b85c67231443b6db150c56ec5aec0e88459edb/LICENCE) |
| strongswan/strongswan | `e15ee26e468b6785d9cd149a3fd62c686ddc9944` | [GPL-2.0-or-later with exceptions and component licenses](https://github.com/strongswan/strongswan/blob/e15ee26e468b6785d9cd149a3fd62c686ddc9944/LICENSE) |
| bitcoin/bitcoin | `dbe9eacd17ae1cc24581081613123a40b44a9f31` | [MIT plus bundled components](https://github.com/bitcoin/bitcoin/blob/dbe9eacd17ae1cc24581081613123a40b44a9f31/COPYING) |
| curl/curl | `34af4747858d806d58a793ec255814059039fa9d` | [curl](https://github.com/curl/curl/blob/34af4747858d806d58a793ec255814059039fa9d/COPYING) |
| libssh2/libssh2 | `dd1546700d0ed8ff7138bc1ccc288286fb3ab2b4` | [BSD-style](https://github.com/libssh2/libssh2/blob/dd1546700d0ed8ff7138bc1ccc288286fb3ab2b4/COPYING) |
| lsh123/xmlsec | `eb3cd78a4731aaff7b69818d212071a6fd1d75fd` | [MIT-style plus MPL-covered NSS portions](https://github.com/lsh123/xmlsec/blob/eb3cd78a4731aaff7b69818d212071a6fd1d75fd/Copyright) |
| bcgit/bc-csharp | `4814a10bc95da8757d8390c59b0b62d4cc00cb0d` | [MIT](https://github.com/bcgit/bc-csharp/blob/4814a10bc95da8757d8390c59b0b62d4cc00cb0d/LICENSE.md) |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | `0d96aca5a57ab4ab144f33c3603b53f82d4e1d18` | [MIT plus third-party notices](https://github.com/AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet/blob/0d96aca5a57ab4ab144f33c3603b53f82d4e1d18/LICENSE.txt) |
| openiddict/openiddict-core | `68269d28b43988e8fac1e7449990bdd1684e2fa6` | [Apache-2.0](https://github.com/openiddict/openiddict-core/blob/68269d28b43988e8fac1e7449990bdd1684e2fa6/LICENSE.md) |
| ITfoxtec/ITfoxtec.Identity.Saml2 | `50a5af70cc8979fd5b1b2de80bf6a46905486197` | [BSD-3-Clause](https://github.com/ITfoxtec/ITfoxtec.Identity.Saml2/blob/50a5af70cc8979fd5b1b2de80bf6a46905486197/LICENSE) |
| Sustainsys/Saml2 | `1adec7d9ef5ff66e00f51232e9c232fa81735430` | [MIT src/foss; commercial src/commercial](https://github.com/Sustainsys/Saml2/blob/1adec7d9ef5ff66e00f51232e9c232fa81735430/LICENSE.txt) |
| jstedfast/MimeKit | `66a7e069010bbee0f93f0b40867c9250568bd910` | [MIT](https://github.com/jstedfast/MimeKit/blob/66a7e069010bbee0f93f0b40867c9250568bd910/LICENSE) |
| win-acme/win-acme | `c213e6dccdae3bbb238ade96dcb99acc61fc455e` | [Apache-2.0](https://github.com/win-acme/win-acme/blob/c213e6dccdae3bbb238ade96dcb99acc61fc455e/LICENSE) |
| OrchardCMS/OrchardCore | `3fc465bbb4a8ffd7a95ca902c09ccd2c89c7c533` | [BSD-3-Clause](https://github.com/OrchardCMS/OrchardCore/blob/3fc465bbb4a8ffd7a95ca902c09ccd2c89c7c533/LICENSE) |
| bitwarden/server | `487bc1602e941c057c3984462288a27021bd45c6` | [AGPL-3.0 default; separate Bitwarden license under bitwarden_license](https://github.com/bitwarden/server/blob/487bc1602e941c057c3984462288a27021bd45c6/LICENSE.txt) |
| dotnet/aspnetcore | `a6110c23cfa4805ee95a6a00ee3812a1e9f1857f` | [MIT plus third-party notices](https://github.com/dotnet/aspnetcore/blob/a6110c23cfa4805ee95a6a00ee3812a1e9f1857f/LICENSE.txt) |

Repository statements do not establish a uniform license for bundled sources. Bitwarden and Sustainsys contain separate commercial directories; both full-root scans are pending the user’s scope decision. No commercial permission is assumed. Submodules are absent in Nextcloud (1), OpenSSL (12), win-acme (2) and ASP.NET Core (2); all 17 gitlinks are retained with commit IDs. No LFS pointers were found among tracked regular files.

## Existing infrastructure and adaptations

The current scanner has 137 built-in rules and bounded PHP/C/C++/C# name/value/receiver tracking with source-local Composer/NuGet/vcpkg/Conan/CMake declarations. Runtime use, installed graphs, compiler/preprocessor execution and arbitrary implementations remain outside its established support. Existing lexical OpenSSL findings can coexist with typed AST observations.

Reuse the unchanged `evaluate.py::score` and `run.py::measure`. The existing corpus validator requires MIT and local materialization; the truth loader requires two independent labels. Neither contract is weakened or applied fictitiously to these repositories. `external.py` validates source bytes and supplies only annotated intervals to the original scorer; unannotated predictions remain unattributed. Its source-language classification preserves `.C` and ambiguous headers. The older reporter is unchanged, preserving historical evidence bindings.

The initial 47 nominated API truth units cover 21 projects; 12 additional unscored domain questions retain all nine other projects and implementation/wrapper/hybrid/composite facts. These are main-authored, source-first exploratory annotations, not independent labels or exhaustive repository truth. They were frozen before any study scanner output. Further truth and source/official-spec verification remain required for Phase 2; partial metrics must remain bounded.

## Verification and remaining work

New external-adapter tests: 4 tests pass. The initial missing-feature run failed as expected; a macOS `/tmp` alias issue was reproduced and corrected without accepting symlinked descendants. The full existing quality suite reports an inherited applicability failure: old reviewed hashes of both COVERAGE guides no longer bind the v0.5 baseline. `PREEXISTING-APPLICABILITY-FAILURE.json` proves that both mismatches existed in `BASELINE-SOURCE.json` before this work. Passing tests are not reported as a fully passing suite. Historical hashes were not rewritten.

Next: decide OSS-only scope for the two mixed-license repositories; complete meaningful truth/coverage questions, run all permitted baseline scans, diagnose findings, add necessary generic improvements and regressions, perform both required independent review stages, then rescan all 30 at the same pins and reconcile the full original request.

## Evidence

- `evidence/BASELINE-SOURCE.json`, `BASELINE-ENVIRONMENT.json`, build log and baseline status.
- `evidence/UPSTREAMS.json`, per-project `acquisition/*` inventories/receipts/logs and `LICENSE-REVIEW.json`.
- `evidence/SOURCE-CHARACTERISTICS.json`: source-first lexical reconnaissance only; matching words, comments and zero counts are not ground truth or absence proofs.
- `evidence/TRUTH-INITIAL.json`, `DOMAIN-TRUTH-INITIAL.json`: immutable initial source-derived questions.
- `evidence/external-red.log`, `external-green.log`, quality suite logs and inherited-failure record.
