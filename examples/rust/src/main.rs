mod auth;

use auth::legacy_token_profile;

fn main() {
    let profile = legacy_token_profile();
    let tls_profile = "TLS_RSA_WITH_AES_256_CBC_SHA";
    let cert_sig = "sha256WithRSAEncryption";

    println!("{}|{}|{}", profile.jwt_alg, tls_profile, cert_sig);
}
