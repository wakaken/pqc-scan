package auth

import (
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
)

type TokenSigner struct {
	Algorithm string
	method    string
}

func NewTokenSigner(algorithm string) *TokenSigner {
	return &TokenSigner{Algorithm: algorithm, method: "SigningMethodRS256"}
}

func (s *TokenSigner) Sign(subject string) ([]byte, error) {
	privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, err
	}

	digest := sha256.Sum256([]byte(subject + "." + s.Algorithm))
	signature, err := rsa.SignPKCS1v15(rand.Reader, privateKey, crypto.SHA256, digest[:])
	if err != nil {
		return nil, err
	}

	_ = rsa.VerifyPKCS1v15(&privateKey.PublicKey, crypto.SHA256, digest[:], signature)
	return signature, nil
}

func (s *TokenSigner) Method() string {
	return s.method
}
