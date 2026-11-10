const key: Promise<CryptoKeyPair> = subtle.generateKey({name: 'ML-DSA-65'}, false, ['sign']);
