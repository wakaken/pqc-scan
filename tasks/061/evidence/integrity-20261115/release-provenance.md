# Prepared artifact integrity and signature validation (task061)

Complete against Q6-14/Q6-15; implementation and test reviews accepted. No productionrelease
identity, realrelease signature, tag, publication or newtrustpolicy was created.

## Inputs, trust boundary and actual method

The readonly inputs are task059's exact localstatic/network archives, bound by
archiveSHA256 and all compiledsource/rules/Cargo inputs. They contain binary,
rules, notices, internalSHA256SUMS and caller-supplied-build provenance. Existing
packageverifier independently rejects changed or missingmembers. Internalhashes
alone do not establish authenticity:an attacker can recompute a selfconsistent
manifest. Receiver verification must anchor outside the downloadedbundle.

The measured preparation uses ephemeral Ed25519 testkeys inside a boundedoffline
nonrootcontainer. A fixturestatement binds preparationOnly=true, independently
expected builder/source/target/variant/name and archivehash. Receiver first
verifies signature using a separately selected trustedtestpublickey, then checks
exactpolicy fields/types, then actualsubjecthash, then the existingarchive
verifier. It never accepts a publickey supplied by the statement. The statement
is a localfixture type, not a claim of SLSA-conformant or GitHub-attested output.

The actualimage is the existing immutable localsha256:16a329d30bd8f2b8d12353b678777392203d3e3f567505709a8ca4a408f95bb1.
The tool reports OpenSSL3.0.22; its executablehash is retained. This pinnedoffline
fixturetool is not a selected productionrelease signer. DockerDesktop had to be
started before the localdaemon was available; no image/dependency was fetched.

Kernel-observed limits:512MiB memory,0 additional swap,1CPU quota,32pids,256MiB
scratchtmpfs,64MiB perfile,120CPU-seconds and240wall-seconds. Source/toolchain/root
are readonly; onlyscratch is writable; networknamespace has onlyloopback and
externalroute denial was tested. Environment is empty except five explicit safe
values. No credentials, hosthome, sockets or sharedservices enter the container.
Only parent-constructed boundedpublicJSONfacts were retained; allprivatekeys and
scratchfiles disappeared with removal. No privatekeybytes or authentication
state were logged. Ed25519 here is testauthenticity, not a quantumresistance claim.

## Actual results and controls

32 cases:both correctprepared signatures accepted;30 invalidcases rejected.
For eachvariant the checks cover binary/rules/notice corruption and omission,
variantbinary mixing, recomputed untrustedchecksums, signaturebit alteration,
untrustedroot and correctlysigned but unauthorized source/builder/target/variant
or preparationOnly numericspoofing. The recomputedmanifest case intentionally
passes hash-only verification but fails the complete signedsubject procedure.
Positivecontrols prevent an all-rejecting verifier from being counted as valid.

The publickeyfingerprint, actual statement/signature hashes,64-byte signatures,
archivehashes, exactcommands/exits, sourceinputhashes and observedboundaries are
retained in artifact-integrity.json and container-r2/{RUN,INPUTS,RESULT}.json.
Privatekeys were deleted explicitly and container-removal was confirmed.
The first30-case run remains unchanged; finalrun adds eachbinaryomission and
measurementhashes. No Java/Rust cryptoimplementation or artifactformat changed.

## Recipient preparation versus a real release

For this localfixture, rerun the hash-bound run-container.py.txt against the
same readonly059 inputs/image using a fresh outputlabel. It creates a newtestkey,
executes the complete receivepath and destroys the key/container; no persistent
keymaterial is required. Its controlleddata are not a productiontrustroot.

For an actualfuture release, a maintainer must independently establish the
trusted signer/builder/workflow and scope. Obtain the authenticbundle and trusted
roots separately, verify exactartifactsubject, expectedrepository/workflow and
sourcecommit, then verify memberchecksums before installation. Bind dirtysource
or candidate status accurately; a valid signature does not certify software
quality or clear the networknotice hold. Existing workflow attestations cover
static only and were not dispatched by this request.

The productionprocedure remains the existing paired DISTRIBUTION guide, including
its pinned source/workflow verification flags. Credentials, OIDC permissions,
realrelease signing and publication require separateauthorization. No fresh
hostedattestation or SLSAlevel is claimed by these localfixtures. Trustroot
freshness/revocation and actualbuilderidentity must be assessed when importing
real signedmaterial; they cannot be inferred from a checksum or localtestkey.

Primary references: [OpenSSL pkeyutl](https://docs.openssl.org/3.0/man1/openssl-pkeyutl/),
[SLSA provenance](https://slsa.dev/spec/v1.2/provenance),
[GitHub offline verification](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/verify-attestations-offline).
The script commands/receipts are actualexecution evidence; externalreferences
supply context rather than certification of this candidate.
