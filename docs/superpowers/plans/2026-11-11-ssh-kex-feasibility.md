# Task039: authenticated-host, no-userauth SSH KEX study

The maintainer explicitly selected the previously conditional use case: establish
PQC/hybrid SSH key exchange and verify the peer using prior trust, without user
authentication/login. This authorizes a separate feasibility harness, not a product
mode, cryptographic reimplementation or storage-schema change.

Main agent researches/implements/runs the study. One combined implementation
review and one separate final-test review; no delegated research or child agents.

1. Compare pinned russh0.65.1, ssh2 0.9.7/libssh2 and system OpenSSH using official
   documentation plus exact downloaded source. Separate remote advertisements,
   selected algorithms, completed NEWKEYS, signature verification and prior trust.
2. Build standalone tests/feasibility/ssh-kex, independent of the root workspace
   and lockfile. Use Rust1.93.1. No production dependency changes.
3. Docker network-none runtime with ephemeral local OpenSSH keys/config, no ambient SSH
   config/agent/credentials. Bounded one-attempt study cases; each starts a new
   fixture. Record only sanitized message types, outcomes, versions and hashes.
4. Prove trusted-pin hybrid success, wrong/unknown pin rejection, tampered server
   signature rejection, classical-only/no-overlap, bounded stalled-peer failure;
   server-side decrypted packet types must prove zero SERVICE_REQUEST(5),
   USERAUTH(50..79), GLOBAL_REQUEST(80), channel(90..100). No private key bytes,
   shared secrets, complete packet payloads or raw library debug logs retained.
5. Reject candidates failing essential no-auth/PQC/trust observability. Record
   supported API/provider/build limits and untested platforms; recommend a narrow
   future implementation, preserving current advertisement-only product behavior.
6. Review implementation, run frozen-source tests/build/format/root tests, review
   test adequacy, then update039 feasibility outcome honestly (including blockers).

No outside endpoint, push/publication, background SSH server, new product command
or implicit trust-on-first-use. Trust input must be separately provisioned; a key
learned from the same untrusted connection is not authentication.
