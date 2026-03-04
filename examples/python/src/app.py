from security.token_service import issue_access_token


class AppConfig:
    min_tls_version = "TLSv1.1"
    cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"
    cert_signature = "sha256WithRSAEncryption"


def boot() -> None:
    token = issue_access_token("alice")
    print(token[:16], AppConfig.min_tls_version, AppConfig.cipher_suite)


if __name__ == "__main__":
    boot()
