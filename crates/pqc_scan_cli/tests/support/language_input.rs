//! Parse-only synthetic inputs: no compiler, package manager or crypto execution.
use std::{fs, path::Path};
pub const CANARY: &str = "PRIVATE082_SYNTHETIC_ONLY";
pub fn write(root: &Path) {
    fs::create_dir_all(root).unwrap();
    for (name, text) in [
        ("kem.php", "<?php\nsodium_crypto_kem_mlkem768_keypair();\n"),
        ("signature.cs", "class Sample { void Run() { var k = System.Security.Cryptography.MLDsa.GenerateKey(System.Security.Cryptography.MLDsaAlgorithm.MLDsa65); } }\n"),
        ("signature.c", "void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_from_name(NULL,\"SLH-DSA-SHA2-128s\",NULL); }\n"),
        ("signature.cpp", "void sample() { oqs::Signature k(\"ML-DSA-65\"); }\n"),
        ("rsa.php", "<?php\nopenssl_pkey_new(['private_key_type' => OPENSSL_KEYTYPE_RSA, 'private_key_bits' => 2048]);\n"),
        ("unknown.c", "void sample(void) { EVP_DigestSignInit(ctx, NULL, NULL, NULL, key); const char *secret = \"PRIVATE082_SYNTHETIC_ONLY\"; }\n"),
        ("composer.json", "{\"require\":{\"php\":\"^8.6\"},\"require-dev\":{\"phpseclib/phpseclib\":\"^3.0\"},\"config\":{\"platform\":{\"ext-sodium\":false}}}"),
        ("packages.lock.json", "{\"version\":2,\"dependencies\":{\"net10.0\":{\"BouncyCastle.Cryptography\":{\"type\":\"Direct\",\"requested\":\"[2.6.0,)\",\"resolved\":\"2.6.2\"}}}}"),
        ("vcpkg.json", "{\"dependencies\":[\"openssl\"]}"),
    ] { fs::write(root.join(name), text).unwrap(); }
}
