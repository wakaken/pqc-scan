import jwt
from Crypto.PublicKey import RSA
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding, rsa

ALGORITHM = "RS256"
KID = "legacy-rsa-key"


def issue_access_token(subject: str) -> str:
    private_key = rsa.generate_private_key(public_exponent=65537, key_size=2048)
    public_key = private_key.public_key()

    _ = RSA.generate(2048)

    message = f"{subject}:{ALGORITHM}".encode("utf-8")
    signature = private_key.sign(message, padding.PKCS1v15(), hashes.SHA256())
    public_key.verify(signature, message, padding.PKCS1v15(), hashes.SHA256())

    token = jwt.encode(
        {"sub": subject, "kid": KID, "id_token_signed_response_alg": "RS256"},
        "dev-secret",
        algorithm=ALGORITHM,
    )
    return token
