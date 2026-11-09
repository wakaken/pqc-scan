# Public certificate fixtures for task 020

These nine DER files contain public certificates only, generated locally with OpenSSL 3.6.3 on 2026-11-09 for pqc-scan tests and distributed under the repository MIT license. Ephemeral RSA/EC/Ed25519 private keys were generated in a temporary directory and deleted; they are not fixtures. Tests do not generate keys, invoke OpenSSL, connect to a network or verify certificate signatures/trust.

`manifest.json` records independent OpenSSL-extracted algorithm/curve facts, DER/SPKI lengths and SHA-256 fingerprints. Expected semantic sizes in Rust tests are literal values, not values computed by the production parser.

| File | Subject public key | Certificate signature | Notes |
|---|---|---|---|
| rsa1024.der | RSA 1024 | SHA-256/RSA | Weak-key parsing fixture only |
| rsa2048.der | RSA 2048 | SHA-256/RSA | Main fixture |
| rsa3072.der | RSA 3072 | SHA-256/RSA | Sign-padding/length case |
| rsa2048-second.der | Same RSA 2048 SPKI | SHA-256/RSA | Different serial/name/DER identity |
| p256.der | P-256 | ECDSA/SHA-256 | Named parameters |
| p384.der | P-384 | ECDSA/SHA-256 | Named parameters |
| p521.der | P-521 | ECDSA/SHA-256 | 521 field bits, not 528 padded coordinate bits |
| ed25519.der | Ed25519 | Ed25519 | 32 encoded bytes, absent parameters |
| p256-rsa-signed.der | P-256 | SHA-256/RSA | Signature algorithm differs from subject key |

Generation used `openssl genpkey`, followed by `openssl req -new -x509 -outform DER -subj /CN=pqc-scan-public-test -days 3650 -set_serial <n>`. The mixed fixture used `openssl x509 -new -force_pubkey <public-EC-PEM> -key <temporary-RSA-key>`. New random keys/dates change the bytes; the checked-in DER and manifest pin the exact corpus.

For independent inspection (public information only):

```sh
openssl x509 -inform DER -in rsa2048.der -noout -text
openssl x509 -inform DER -in rsa2048.der -pubkey -noout | openssl pkey -pubin -outform DER | openssl dgst -sha256
openssl dgst -sha256 *.der
```

Rust tests also mutate OIDs and assemble small public SPKIs for malformed/unknown/non-byte-aligned cases. Mutated certificates are parser-only inputs with invalid signatures and are never claimed to be valid PQC certificates. A private-key marker test uses an explicit synthetic canary, not a key.
